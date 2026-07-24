mod camera;
mod draw;
mod model;

use clap::Parser;
use futures_util::stream::StreamExt;
use gstreamer::prelude::*;
use ndarray::Array3;
use std::error::Error;
use std::sync::{Arc, Mutex, mpsc};
use tokio::signal;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(
    version = "3.0",
    about = "VisionOTG",
    long_about = "FPV Camera with AI inference"
)]
struct Args {
    /// Pass either the camera index or v4l2src cameras
    #[arg(
        short,
        long,
        help = "Camera index or v4l2src camera to run",
        default_value = "/dev/video0"
    )]
    camera: String,

    /// The path to the YOLOv8 model
    #[arg(
        short,
        long,
        help = "The path to the YOLOv8 model",
        default_value = "yolov8n.onnx"
    )]
    model: String,

    /// Input normalization
    #[arg(short, long,
          help="Specify the model input normalization", 
          value_enum,
          default_value_t = model::Normalization::Unsigned)]
    norm: model::Normalization,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Initialize GStreamer
    gstreamer::init()?;

    // Initialize detections placeholder
    let detections: model::SharedDetections =
        Arc::new(Mutex::new(Array3::<f32>::zeros((1, 300, 6))));

    // Define GStreamer pipeline: capture -> process -> overlay -> display
    let (pipeline, overlay) = camera::create_pipeline(&args.camera)?;

    // Clone shared state into overlay callback
    let overlay_detections = detections.clone();

    overlay.connect("draw", false, move |args| {
        let cr: cairo::Context = args[1].get().expect("Failed to start Cairo context");
        draw::draw_overlay(&cr, &overlay_detections);
        None
    });

    let (frame_tx, frame_rx) = mpsc::sync_channel::<camera::Frame>(2);

    model::inference_handler(args.model, args.norm, frame_rx, detections.clone());
    camera::appsink_handler(&pipeline, frame_tx);

    // Start input pipeline
    match pipeline.set_state(gstreamer::State::Playing) {
        Ok(_) => {}
        Err(err) => {
            panic!(
                "Failed to start GStreamer pipeline. Check if the camera '{}' exists: {}",
                args.camera, err
            );
        }
    }

    // -------------------------
    // Main loop (bus)
    // -------------------------
    let bus = pipeline
        .bus()
        .expect("Pipeline was initialized without bus");
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
    camera::cleanup(&pipeline);
    Ok(())
}
