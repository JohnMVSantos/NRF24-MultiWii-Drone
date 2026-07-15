use std::sync::{Arc, Mutex, mpsc};
use std::error::Error;
use tokio::signal;
use futures_util::stream::StreamExt;
use gstreamer::prelude::*;
use cairo::Context;

use onnxruntime::{
    environment::Environment, ndarray::{Axis, Array1, Array3, Array4}, 
    tensor::OrtOwnedTensor, 
    GraphOptimizationLevel,
    LoggingLevel,
};

// TODO: Should be made as part of command line arguments
const WEBCAM: &str = "0";
const NORM: f32 = 1.0 / 255.0;

struct Frame {
    width: usize,
    height: usize,
    pixels: Vec<u8>, // RGB pixel data
}

struct Object {
    class: &'static str,
    color: (f64, f64, f64),
}

const OBJECTS: [Object; 80] = [
    Object{ class: "person", color: (0.0, 1.0, 0.0) },
    Object{ class: "bicycle", color: (1.0, 0.0, 0.0) },
    Object{ class: "car", color: (0.0, 0.0, 1.0) },
    Object{ class: "motorcycle", color: (1.0, 1.0, 0.0) },
    Object{ class: "airplane", color: (1.0, 0.0, 1.0) },
    Object{ class: "bus", color: (0.0, 1.0, 1.0) },
    Object{ class: "train", color: (1.0, 0.5, 0.0) },
    Object{ class: "truck", color: (0.5, 0.0, 1.0) },
    Object{ class: "boat", color: (0.0, 0.5, 1.0) },
    Object{ class: "traffic light", color: (0.5, 1.0, 0.0) },
    Object{ class: "fire hydrant", color: (1.0, 0.3, 0.3) },
    Object{ class: "stop sign", color: (0.3, 1.0, 0.3) },
    Object{ class: "parking meter", color: (0.3, 0.3, 1.0) },
    Object{ class: "bench", color: (1.0, 0.7, 0.3) },
    Object{ class: "bird", color: (0.7, 0.3, 1.0) },
    Object{ class: "cat", color: (0.3, 1.0, 0.7) },
    Object{ class: "dog", color: (1.0, 0.3, 0.7) },
    Object{ class: "horse", color: (0.7, 1.0, 0.3) },
    Object{ class: "sheep", color: (0.3, 0.7, 1.0) },
    Object{ class: "cow", color: (1.0, 0.5, 0.5) },
    Object{ class: "elephant", color: (0.5, 1.0, 0.5) },
    Object{ class: "bear", color: (0.5, 0.5, 1.0) },
    Object{ class: "zebra", color: (1.0, 0.8, 0.5) },
    Object{ class: "giraffe", color: (0.8, 0.5, 1.0) },
    Object{ class: "backpack", color: (0.5, 1.0, 0.8) },
    Object{ class: "umbrella", color: (1.0, 0.5, 0.8) },
    Object{ class: "handbag", color: (0.8, 1.0, 0.5) },
    Object{ class: "tie", color: (0.5, 0.8, 1.0) },
    Object{ class: "suitcase", color: (1.0, 0.6, 0.6) },
    Object{ class: "frisbee", color: (0.6, 1.0, 0.6) },
    Object{ class: "skis", color: (0.6, 0.6, 1.0) },
    Object{ class: "snowboard", color: (1.0, 0.9, 0.6) },
    Object{ class: "sports ball", color: (0.9, 0.6, 1.0) },
    Object{ class: "kite", color: (0.6, 1.0, 0.9) },
    Object{ class: "baseball bat", color: (1.0, 0.6, 0.9) },
    Object{ class: "baseball glove", color: (0.9, 1.0, 0.6) },
    Object{ class: "skateboard", color: (0.6, 0.9, 1.0) },
    Object{ class: "surfboard", color: (1.0, 0.7, 0.7) },
    Object{ class: "tennis racket", color: (0.7, 1.0, 0.7) },
    Object{ class: "bottle", color: (0.7, 0.7, 1.0) },
    Object{ class: "wine glass", color: (1.0, 0.8, 0.7) },
    Object{ class: "cup", color: (0.8, 1.0, 0.7) },
    Object{ class: "fork", color: (0.7, 0.8, 1.0) },
    Object{ class: "knife", color: (1.0, 0.7, 0.8) },
    Object{ class: "spoon", color: (0.8, 1.0, 0.8) },
    Object{ class: "bowl", color: (0.8, 0.8, 1.0) },
    Object{ class: "banana", color: (1.0, 0.85, 0.6) },
    Object{ class: "apple", color: (0.85, 1.0, 0.6) },
    Object{ class: "sandwich", color: (0.6, 0.85, 1.0) },
    Object{ class: "orange", color: (1.0, 0.6, 0.85) },
    Object{ class: "broccoli", color: (0.85, 1.0, 0.6) },
    Object{ class: "carrot", color: (0.6, 1.0, 0.85) },
    Object{ class: "hot dog", color: (1.0, 0.6, 0.85) },
    Object{ class: "pizza", color: (0.85, 0.6, 1.0) },
    Object{ class: "donut", color: (0.6, 0.85, 1.0) },
    Object{ class: "cake", color: (1.0, 0.85, 0.6) },
    Object{ class: "chair", color: (0.4, 0.4, 0.4) },
    Object{ class: "couch", color: (0.7, 0.4, 0.2) },
    Object{ class: "potted plant", color: (0.2, 0.7, 0.4) },
    Object{ class: "bed", color: (0.4, 0.2, 0.7) },
    Object{ class: "dining table", color: (0.7, 0.7, 0.2) },
    Object{ class: "toilet", color: (0.2, 0.7, 0.7) },
    Object{ class: "tv", color: (0.7, 0.2, 0.7) },
    Object{ class: "laptop", color: (0.9, 0.4, 0.2) },
    Object{ class: "mouse", color: (0.2, 0.9, 0.4) },
    Object{ class: "remote", color: (0.4, 0.2, 0.9) },
    Object{ class: "keyboard", color: (0.9, 0.9, 0.2) },
    Object{ class: "cell phone", color: (0.2, 0.9, 0.9) },
    Object{ class: "microwave", color: (0.9, 0.2, 0.9) },
    Object{ class: "oven", color: (0.5, 0.5, 0.5) },
    Object{ class: "toaster", color: (0.3, 0.3, 0.3) },
    Object{ class: "sink", color: (0.6, 0.3, 0.1) },
    Object{ class: "refrigerator", color: (0.1, 0.6, 0.3) },
    Object{ class: "book", color: (0.3, 0.1, 0.6) },
    Object{ class: "clock", color: (0.6, 0.6, 0.1) },
    Object{ class: "vase", color: (0.1, 0.6, 0.6) },
    Object{ class: "scissors", color: (0.6, 0.1, 0.6) },
    Object{ class: "teddy bear", color: (0.8, 0.8, 0.8) },
    Object{ class: "hair drier", color: (0.5, 0.2, 0.8) },
    Object{ class: "toothbrush", color: (0.8, 0.5, 0.2) },
];

type SharedDetections = Arc<Mutex<Array3<f32>>>; // 3D array containing model detections

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize GStreamer
    gstreamer::init()?;

    // Initialize detections placeholder
    let detections: SharedDetections = Arc::new(Mutex::new(Array3::<f32>::zeros((1, 300, 6)))); 

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
    let (camera_src, camera_index) = {
        #[cfg(target_os = "linux")]
        {("v4l2src", format!("device=/dev/video{}", WEBCAM))}
        #[cfg(target_os = "macos")]
        {("avfvideosrc", WEBCAM.to_string())}
        #[cfg(target_os = "windows")]
        {("mfvideosrc", WEBCAM.to_string())}
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {("autovideosrc", WEBCAM.to_string())}
    };

    let pipeline_str = format!(
        "{} device-index={} \
        ! videoconvert \
        ! tee name=t \
           t. ! queue ! cairooverlay name=overlay ! videoconvert ! autovideosink \
           t. ! queue ! videoscale ! videoconvert ! video/x-raw,width=640,height=640,format=RGB \
           ! appsink name=ai_sink emit-signals=true sync=false",
        camera_src, camera_index
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

        // Check model input/output compatibility
        assert_eq!(
            input0_shape,
            vec![1, 3, 640, 640],
            "Unexpected model input shape. Expected [1, 3, 640, 640], got {:?}",
            input0_shape
        );

        assert_eq!(
            output0_shape,
            vec![1, 300, 6],
            "Unexpected model output shape. Expected [1, 300, 6], got {:?}",
            output0_shape
        );

        while let Ok(frame) = frame_rx.recv() {
            // YOLOv8n input takes CHW format.
            let mut data = Array4::<f32>::zeros(
                (input0_shape[0], input0_shape[1], 
                    input0_shape[2], input0_shape[3]));
            let hw = frame.width * frame.height;
            let out = data.as_slice_mut().unwrap();
            // Perform unsigned normalization
            for (i, rgb) in frame.pixels.chunks_exact(3).enumerate() {
                out[i] = rgb[0] as f32 * NORM;
                out[hw + i] = rgb[1] as f32 * NORM;
                out[2 * hw + i] = rgb[2] as f32 * NORM;
            }
            let input_tensor_values = vec![data];

            // Run model inference
            let outputs: Vec<OrtOwnedTensor<f32, _>> = session
                .run(input_tensor_values)
                .expect("Failed to run inference session");

            // Place outputs to the shared detections placeholder.
            let output = &outputs[0];
            let mut out = output.view().to_owned();
            
            // Normalize detections to [0, 1] for overlay drawing.
            let scale = Array1::from(vec![
                1.0 / frame.width as f32,
                1.0 / frame.height as f32,
                1.0 / frame.width as f32,
                1.0 / frame.height as f32,
                1.0,
                1.0,
            ]);
            out *= &scale;

            let mut shared = detections.lock().unwrap();
            shared.assign(&out);
            drop(shared);
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
    let dets_ = dets.index_axis(Axis(0), 0);

    match cr.clip_extents() {
        Ok((x1, y1, x2, y2)) => {
            let width = x2 - x1;
            let height = y2 - y1;

            cr.set_line_width(3.0);
            
            // Draw bounding boxes
            for d in dets_.outer_iter() {
                let xmin  = d[0] as f64;
                let ymin  = d[1] as f64;
                let xmax  = d[2] as f64;
                let ymax  = d[3] as f64;
                let score = d[4];
                let class = d[5];

                if score < 0.25 {
                    continue;
                }

                let obj = OBJECTS.get(class as usize).unwrap_or(
                    &Object { class: "unknown", color: (1.0, 1.0, 1.0) });

                cr.set_source_rgb(obj.color.0, obj.color.1, obj.color.2);
                cr.rectangle(xmin * width, 
                             ymin * height, 
                             (xmax - xmin) * width, 
                             (ymax - ymin) * height);
                cr.stroke().unwrap();

                // Example HUD text
                cr.set_font_size(20.0);
                cr.move_to(xmin * width, ymin * height - 20.0);
                cr.show_text(&format!("{}: {:.2}", obj.class, score)).unwrap();
            }
        },
        Err(e) => {
            eprintln!("Failed to get clip extents: {}", e);
        }
    };
}
