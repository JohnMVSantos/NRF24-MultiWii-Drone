use gstreamer::prelude::*;
use std::sync::{Arc, Mutex, mpsc};
use std::error::Error;
use tokio::signal;
use futures_util::stream::StreamExt;
use cairo::Context;

use onnxruntime::{
    environment::Environment, ndarray::{ArrayView3, Axis, Array4}, tensor::OrtOwnedTensor, GraphOptimizationLevel,
    LoggingLevel,
};

use image::RgbImage;

// gst-launch-1.0 mfvideosrc device-index=1 ! videoconvert ! autovideosink

struct Frame {
    width: usize,
    height: usize,
    pixels: Vec<u8>, // RGB pixel data
}

// -------------------------
// Shared state for AI output
// -------------------------
#[derive(Clone, Default)]
struct Detection {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    cls: u8,
    score: f32,
}

type SharedDetections = Arc<Mutex<Vec<Detection>>>;

// Linux
//const WEBCAM: &str = "/dev/video0";

// Windows
const WEBCAM: &str = "0";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize GStreamer
    gstreamer::init()?;

    let detections: SharedDetections = Arc::new(Mutex::new(Vec::new()));

    // Define GStreamer pipeline: capture -> process -> overlay -> display
    let (pipeline, overlay) = create_pipeline()?;

    // Clone shared state into overlay callback
    let overlay_detections = detections.clone();

    overlay.connect("draw", false, move |args| {
        let cr: cairo::Context = args[1].get().unwrap();
        draw_overlay(&cr, &overlay_detections);

        None
    });

    let (frame_tx, frame_rx) = mpsc::sync_channel::<Frame>(2);

    inference_handler(frame_rx, detections.clone())?;
    appsink_handler(&pipeline, frame_tx)?;

    // Start input pipeline
    pipeline.set_state(gstreamer::State::Playing)?;

    // -------------------------
    // OPTIONAL: AI THREAD PLACEHOLDER
    // -------------------------
    // In real usage, your AI thread updates `detections`
    // {
    //     let detections = detections.clone();
    //     std::thread::spawn(move || loop {
    //         let mut dets = detections.lock().unwrap();

    //         // Fake moving box
    //         dets.clear();
    //         dets.push(Detection {
    //             x: 100.0,
    //             y: 100.0,
    //             w: 200.0,
    //             h: 150.0,
    //             cls: 1,
    //             score: 0.9,
    //         });

    //         drop(dets);
    //         std::thread::sleep(std::time::Duration::from_millis(33));
    //     });
    // }

    // -------------------------
    // Main loop (bus)
    // -------------------------
    let bus = pipeline.bus().unwrap();
    let mut bus_stream = bus.stream();

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Ctrl+C received");
            let _ = pipeline.send_event(gstreamer::event::Eos::new());
        }

        _ = async {
            while let Some(msg) = bus_stream.next().await {
                match msg.view() {
                    gstreamer::MessageView::Eos(_) => break,
                    gstreamer::MessageView::Error(err) => {
                        eprintln!("Error: {:?}", err);
                        break;
                    }
                    _ => {}
                }
            }
        } => {}
    }

    // Cleanup pipeline on exit
    cleanup(&pipeline);
    Ok(())
}

// -----------------------------------------------------
// PIPELINE: camera -> videoconvert -> tee -> overlay + AI
// -----------------------------------------------------
fn create_pipeline() -> Result<(gstreamer::Pipeline, gstreamer::Element), Box<dyn Error>> {
    // Choose camera source based on OS
    #[cfg(target_os = "linux")]
    let camera_src = "v4l2src";
    #[cfg(target_os = "macos")]
    let camera_src = "avfvideosrc";
    #[cfg(target_os = "windows")]
    let camera_src = "mfvideosrc";
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    let camera_src = "autovideosrc";

    let pipeline_str = format!(
        "{} device-index={} \
        ! videoconvert \
        ! tee name=t \
           t. ! queue ! cairooverlay name=overlay ! videoconvert ! autovideosink \
           t. ! queue ! videoscale ! videoconvert ! video/x-raw,width=640,height=640,format=RGB \
           ! appsink name=ai_sink emit-signals=true sync=false",
        camera_src, WEBCAM
    );

    let pipeline = gstreamer::parse::launch(&pipeline_str)?
        .dynamic_cast::<gstreamer::Pipeline>()
        .unwrap();

    let overlay = pipeline
        .by_name("overlay")
        .expect("cairooverlay not found");

    Ok((pipeline, overlay))
}

fn initialize_model<'a>(environment: &'a Environment) -> Result<onnxruntime::session::Session<'a>, Box<dyn Error>> {
    let session = environment
        .new_session_builder()?
        .with_optimization_level(GraphOptimizationLevel::Basic)?
        .with_number_threads(1)?
        .with_model_from_file("yolov8n.onnx")?;

    Ok(session)
}

// Shutdown the pipeline on exit
fn cleanup(pipeline: &gstreamer::Pipeline) {
    pipeline.set_state(gstreamer::State::Null).unwrap();
}

// Spawn a thread for model inference
fn inference_handler(frame_rx: mpsc::Receiver<Frame>, detections: SharedDetections) -> Result<(), Box<dyn Error>> {
    let environment = Environment::builder()
        .with_name("test")
        .with_log_level(LoggingLevel::Info)
        .build()?;
    
    std::thread::spawn(move || {
        let mut session = initialize_model(&environment).expect("Failed to initialize model");

        let input0_shape: Vec<usize> = session.inputs[0].dimensions().map(|d| d.unwrap()).collect();
        let output0_shape: Vec<usize> = session.outputs[0]
            .dimensions()
            .map(|d| d.unwrap())
            .collect();

        // TODO: Handle this check better
        assert_eq!(input0_shape, [1, 3, 640, 640]);
        assert_eq!(output0_shape, [1, 300, 6]);

        while let Ok(frame) = frame_rx.recv() {

            // let img = RgbImage::from_raw(
            //     frame.width as u32,
            //     frame.height as u32,
            //     frame.pixels.clone()
            // ).unwrap();

            // img.save("camera_debug.png").unwrap();

            // let p = &frame.pixels[0..3];

            // println!(
            //     "First pixel bytes: R={} G={} B={}",
            //     p[0], p[1], p[2]
            // );
            // println!("{}x{}", frame.width, frame.height);
            // println!("bytes={}", frame.pixels.len());

            let mut data = Array4::<f32>::zeros((1,3,640,640));

            let pixels = &frame.pixels;

            for y in 0..frame.height {
                for x in 0..frame.width {
                    let idx = (y * 640 + x) * 3;
                    data[[0,0,y,x]] =
                        pixels[idx] as f32 / 255.0;

                    data[[0,1,y,x]] =
                        pixels[idx+1] as f32 / 255.0;

                    data[[0,2,y,x]] =
                        pixels[idx+2] as f32 / 255.0;
                }
            }

            // // Convert frame pixels to ndarray
            // let rgb = ArrayView3::from_shape(
            //     (frame.height, frame.width, 3), 
            //     &frame.pixels
            // ).unwrap().to_owned();
            // // Cast to f32 and normalize
            // let rgb_f32 = rgb.mapv(|x| x as f32);
            // let rgb_normalized = rgb_f32.mapv(|x| x / 255.0);
            // // Rearrange to CHW format and insert batch dimension
            // let chw = rgb_normalized.permuted_axes([2, 0, 1]);
            // let data = chw.insert_axis(Axis(0));

            // println!(
            //     "min={} max={}",
            //     data.fold(f32::MAX, |a,&b| a.min(b)),
            //     data.fold(f32::MIN, |a,&b| a.max(b))
            // );

            let input_tensor_values = vec![data];

            // Run model inference
            let outputs: Vec<OrtOwnedTensor<f32, _>> = session
                .run(input_tensor_values)
                .expect("Failed to run inference session");
            
            // println!("Inference output shape: {:?}", outputs[0].shape());
            // println!("type outputs: {}", std::any::type_name_of_val(&outputs[0]));

            let output = &outputs[0];
            let dets = output.index_axis(Axis(0), 0);

            let mut xmin = 0.0;
            let mut ymin = 0.0;
            let mut xmax = 0.0;
            let mut ymax = 0.0;
            let mut score = 0.0;
            let mut class = 0.0;

            // for i in 0..output.shape()[0] {
            //     let output_view = output.view();
            //     let row_view = output_view.index_axis(Axis(0), i);
            //     let row = row_view.as_slice().unwrap();

            //     xmin = row[0];
            //     ymin = row[1];
            //     xmax = row[2];
            //     ymax = row[3];
            //     score = row[4];
            //     class = row[5];

            //     println!(
            //         "row {i}: xmin={xmin}, ymin={ymin}, xmax={xmax}, ymax={ymax}, score={score}, class={class}"
            //     );
            // }

            for det in dets.outer_iter() {
                let xmin  = det[0] / (frame.width as f32);
                let ymin  = det[1] / (frame.height as f32);
                let xmax  = det[2] / (frame.width as f32);
                let ymax  = det[3] / (frame.height as f32);
                let score = det[4];
                let class = det[5];

                if score < 0.25 {
                    continue;
                }

                println!(
                    "{xmin} {ymin} {xmax} {ymax} score={score} class={class}"
                );

                let mut shared = detections.lock().unwrap();

                shared.clear();

                shared.push(Detection {
                    x: xmin,
                    y: ymin,
                    w: xmax-xmin,
                    h: ymax-ymin,
                    cls: class as u8,
                    score,
                });
                drop(shared);

            }

            // TODO: update detections

            // let mut dets = detections.lock().unwrap();

            // // Fake moving box
            // dets.clear();
            // dets.push(Detection {
            //     x: xmin as f64,
            //     y: ymin as f64,
            //     w: (xmax - xmin) as f64,
            //     h: (ymax - ymin) as f64,
            //     cls: class as u8,
            //     score: score as f32,
            // });

            // drop(dets);
            // std::thread::sleep(std::time::Duration::from_millis(33));
        }
    });
    Ok(())
}

// Spawn a thread to handle appsink samples
fn appsink_handler(pipeline: &gstreamer::Pipeline, frame_tx: mpsc::SyncSender<Frame>) -> Result<(), Box<dyn Error>> {
    // Use for AI detections: Extract the appsink elements by its string name
    let appsink = pipeline
        .by_name("ai_sink")
        .unwrap()
        .dynamic_cast::<gstreamer_app::AppSink>()
        .unwrap();

    // Spawn a background thread to safely block and pull samples
    std::thread::spawn(move || {
        loop {
            // pull_sample() blocks until a sample is ready or EOS occurs
            match appsink.pull_sample() {
                Ok(sample) => {
                    
                    // Get video metadata
                    let caps = sample.caps().unwrap();
                    let info = gstreamer_video::VideoInfo::from_caps(caps)
                        .expect("Failed to parse VideoInfo");
                    let width = info.width() as usize;
                    let height = info.height() as usize;
                    // println!("stride={}", info.stride()[0]);
                    // let has_alpha = info.has_alpha();
                    // println!("Height and width: {:?}x{:?}", height, width);
                    // println!("Has alpha channel: {:?}", has_alpha);
                    // println!("type info: {}", std::any::type_name_of_val(&info));

                    // Extract the buffer payload from the pulled sample
                    if let Some(buffer) = sample.buffer() {
                        // Map the buffer memory for reading
                        if let Ok(map) = buffer.map_readable() {

                            let frame = Frame {
                                width,
                                height,
                                pixels: map.as_slice().to_vec(),
                            };
                            let _ = frame_tx.try_send(frame);

                            // let data: &[u8] = &map.to_vec();
                            // println!("Data Shape: {:?}", data.shape());
                            // println!("Data[1] Shape: {:?}", data[0].len());
                            // println!("Data[3] Shape: {:?}", data[1].len());
                            // let data: &[u8] = map.as_slice();

                            // println!("type data: {}", std::any::type_name_of_val(&data));
                            // println!("Successfully pulled a sample of size: {} bytes", data.len());
                        }
                    }
                }
                Err(_eos) => {
                    println!("Error occurred: {}", _eos);
                    println!("Reached End of Stream (EOS) or appsink stopped.");
                    break;
                }
            }
        }
    });
    Ok(())
}

// -----------------------------------------------------
// DRAW CALLBACK
// -----------------------------------------------------
fn draw_overlay(cr: &Context, detections: &SharedDetections) {
    let dets = detections.lock().unwrap();

    match cr.clip_extents() {
        Ok((x1, y1, x2, y2)) => {
            let width = x2 - x1;
            let height = y2 - y1;

            println!("Overlay surface size: {}x{}", width, height);

            // Draw bounding boxes
            for d in dets.iter() {
                cr.set_source_rgb(1.0, 0.0, 0.0);
                cr.set_line_width(3.0);

                cr.rectangle((d.x as f64 * width), 
                             (d.y as f64 * height), 
                             (d.w as f64 * width), 
                             (d.h as f64 * height));
                cr.stroke().unwrap();
            }

            // Example HUD text
            cr.set_source_rgb(0.0, 1.0, 0.0);
            cr.set_font_size(20.0);
            cr.move_to(20.0, 30.0);
            cr.show_text("Cairo Overlay Active").unwrap();
        },
        Err(e) => {
            eprintln!("Failed to get clip extents: {}", e);
        }
    };
}
