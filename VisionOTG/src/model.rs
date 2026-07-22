use std::error::Error;
use std::sync::{Arc, Mutex, mpsc};
use clap::ValueEnum;
use serde_json;
use ndarray::{Array, Array1, Array3};
use ort::{
	inputs,
	session::{builder::GraphOptimizationLevel, Session},
	value::TensorRef
};

use crate::camera::Frame;

pub type SharedDetections = Arc<Mutex<Array3<f32>>>; // 3D array containing model detections

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Normalization {
    Unsigned,
    Signed,
    Raw,
}

impl Normalization {
    // Returns a function pointer that takes an f32 and returns an f32
    fn get_lambda(&self) -> fn(f32) -> f32 {
        match self {
            Normalization::Unsigned => |x| x / 255.0,
            Normalization::Signed => |x| (x / 127.5) - 1.0,
            Normalization::Raw => |x| x,
        }
    }
}

pub fn initialize_model<'a>(model_path: &str) -> Result<Session, Box<dyn Error>> {
    let session = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(4)?
        .commit_from_file(model_path)?;
    Ok(session)
}

// Spawn a thread for model inference
pub fn inference_handler(model_path: String, norm: Normalization, frame_rx: mpsc::Receiver<Frame>, detections: SharedDetections) {
    let model_thread = std::thread::Builder::new().name("model_thread".into());
    model_thread.spawn(move || {
        let mut session = initialize_model(&model_path).expect("Failed to initialize YOLOv8 ONNX model");
        let normalize = norm.get_lambda();

        let (input0_shape, channels) = {
            let meta = session
                .metadata()
                .expect("Failed to extract model metadata");

            let input0_shape: Vec<i32> = serde_json::from_str(
                &meta.custom("imgsz")
                    .expect("Model metadata is missing required 'imgsz' entry")
            )
            .expect("Failed to parse imgsz metadata as JSON");

            let channels: i32 = meta
                .custom("channels")
                .expect("Model metadata is missing required 'channels' entry")
                .parse()
                .expect("Model metadata 'channels' must be a valid integer");

            (input0_shape, channels)
        };

        let height = input0_shape[0];
        let width = input0_shape[1];

        // Check model input/output compatibility
        assert_eq!(
            [1, channels, height, width],
            [1, 3, 640, 640],
            "Unexpected model input shape. Expected [1, 3, 640, 640], got {:?}",
            input0_shape
        );

        while let Ok(frame) = frame_rx.recv() {
            // YOLOv8n input takes CHW format.
            let mut data = Array::zeros((1usize,
                channels as usize,
                height as usize,
                width as usize
            ));
            let hw = frame.width * frame.height;
            let out = data.as_slice_mut().expect("Failed to get mutable slice");
            // Perform unsigned normalization
            for (i, rgb) in frame.pixels.chunks_exact(3).enumerate() {
                out[i] = normalize(rgb[0] as f32);
                out[hw + i] = normalize(rgb[1] as f32);
                out[2 * hw + i] = normalize(rgb[2] as f32);
            }
            let input_tensor = TensorRef::from_array_view(&data)
                .expect("Failed to convert input ndarray into ONNX TensorRef");
            let outputs = session
                .run(inputs!["images" => input_tensor])
                .expect("ONNX model inference execution failed");

            let mut out = outputs["output0"]
                .try_extract_array::<f32>()
                .expect("Failed to read model output 'output0' as an f32 ndarray")
                .index_axis(ndarray::Axis(0), 0)
                .to_owned();
            
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

            let mut shared = detections.lock().expect("Failed to lock global detections placeholder");
            shared.assign(&out);
            drop(shared);
        }
    }).expect("Failed to spawn model inference thread");
}
