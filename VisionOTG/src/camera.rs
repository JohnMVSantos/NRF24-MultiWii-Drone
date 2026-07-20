use std::error::Error;
use std::sync::mpsc;
use gstreamer::prelude::*;

// TODO: Should be made as part of command line arguments
const WEBCAM: &str = "8";

pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>, // RGB pixel data
}

// -----------------------------------------------------
// PIPELINE: camera -> videoconvert -> tee -> overlay + AI
// -----------------------------------------------------
pub fn create_pipeline() -> Result<(gstreamer::Pipeline, gstreamer::Element), Box<dyn Error>> {
    // Choose camera source based on OS
    let (camera_src, camera_index) = {
        #[cfg(target_os = "linux")]
        {("v4l2src", format!("device=/dev/video{}", WEBCAM.to_string()))}
        #[cfg(target_os = "macos")]
        {("avfvideosrc", format!("device-index={}", WEBCAM.to_string()))}
        #[cfg(target_os = "windows")]
        {("mfvideosrc",  format!("device-index={}", WEBCAM.to_string()))}
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {("autovideosrc",  format!("device-index={}", WEBCAM.to_string()))}
    };

    let pipeline_str = format!(
        "{} {} \
        ! videoconvert \
        ! tee name=t \
           t. ! queue ! cairooverlay name=overlay ! videoconvert ! autovideosink \
           t. ! queue ! videoscale ! videoconvert ! video/x-raw,width=640,height=640,format=RGB \
           ! appsink name=ai_sink emit-signals=true sync=false",
        camera_src, camera_index
    );

    let pipeline = gstreamer::parse::launch(&pipeline_str)?
        .dynamic_cast::<gstreamer::Pipeline>()
        .expect("Failed to launch GStreamer pipeline");

    let overlay = pipeline
        .by_name("overlay")
        .expect("Cairo overlay element not found");

    Ok((pipeline, overlay))
}

// Spawn a thread to handle appsink samples
pub fn appsink_handler(pipeline: &gstreamer::Pipeline, frame_tx: mpsc::SyncSender<Frame>) {
    // Use for AI detections: Extract the appsink elements by its string name
    let appsink = pipeline
        .by_name("ai_sink")
        .expect("GStreamer element ai_sink was not found")
        .dynamic_cast::<gstreamer_app::AppSink>()
        .expect("Failed to cast pipeline to AppSink");

    // Spawn a background thread to safely block and pull samples
    let camera_thread = std::thread::Builder::new().name("camera_thread".into());
    camera_thread.spawn(move || {
        loop {
            // pull_sample() blocks until a sample is ready or EOS occurs
            match appsink.pull_sample() {
                Ok(sample) => {
                    // Get video metadata
                    let caps = sample.caps().expect("Failed to get sample caps from the appsink");
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
    }).expect("Failed to spawn camera thread");
}

// Shutdown the pipeline on exit
pub fn cleanup(pipeline: &gstreamer::Pipeline) {
    pipeline.set_state(gstreamer::State::Null).expect("Failed to set pipeline state to Null");
}
