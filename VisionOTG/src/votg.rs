mod camera;
mod model;
mod draw;

use std::sync::{Arc, Mutex, mpsc};
use std::error::Error;
use tokio::signal;
use futures_util::stream::StreamExt;
use gstreamer::prelude::*;
use ndarray::Array3;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize GStreamer
    gstreamer::init()?;

    // Initialize detections placeholder
    let detections: model::SharedDetections = Arc::new(Mutex::new(Array3::<f32>::zeros((1, 300, 6)))); 

    // Define GStreamer pipeline: capture -> process -> overlay -> display
    let (pipeline, overlay) = camera::create_pipeline()?;

    // Clone shared state into overlay callback
    let overlay_detections = detections.clone();

    overlay.connect("draw", false, move |args| {
        let cr: cairo::Context = args[1].get().expect("Failed to start Cairo context");
        draw::draw_overlay(&cr, &overlay_detections);
        None
    });

    let (frame_tx, frame_rx) = mpsc::sync_channel::<camera::Frame>(2);

    model::inference_handler(frame_rx, detections.clone());
    camera::appsink_handler(&pipeline, frame_tx);

    // Start input pipeline
    pipeline.set_state(gstreamer::State::Playing)?;

    // -------------------------
    // Main loop (bus)
    // -------------------------
    let bus = pipeline.bus().expect("Pipeline was initialized without bus");
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
