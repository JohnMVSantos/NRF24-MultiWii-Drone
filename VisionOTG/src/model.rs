use std::error::Error;
use std::sync::{Arc, Mutex, mpsc};

use onnxruntime::{
    environment::Environment, ndarray::{Array1, Array3, Array4}, 
    tensor::OrtOwnedTensor, 
    GraphOptimizationLevel,
    LoggingLevel,
};

use crate::camera::Frame;

pub type SharedDetections = Arc<Mutex<Array3<f32>>>; // 3D array containing model detections

const NORM: f32 = 1.0 / 255.0;


pub fn initialize_model<'a>(environment: &'a Environment) -> Result<onnxruntime::session::Session<'a>, Box<dyn Error>> {
    let session = environment
        .new_session_builder()?
        .with_optimization_level(GraphOptimizationLevel::Basic)?
        .with_number_threads(1)?
        .with_model_from_file("yolov8n.onnx")?;
    Ok(session)
}

// Spawn a thread for model inference
pub fn inference_handler(frame_rx: mpsc::Receiver<Frame>, detections: SharedDetections) -> Result<(), Box<dyn Error>> {
    let environment = Environment::builder()
        .with_name("test")
        .with_log_level(LoggingLevel::Info)
        .build()?;
    
    std::thread::spawn(move || {
        let mut session = initialize_model(&environment).expect("Failed to initialize YOLOv8 ONNX model");

        let input0_shape: Vec<usize> = session.inputs[0].dimensions().map(|d| d.expect("Failed to get model input dimension")).collect();
        let output0_shape: Vec<usize> = session.outputs[0]
            .dimensions()
            .map(|d| d.expect("Failed to get model output dimension"))
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
            let out = data.as_slice_mut().expect("Failed to get mutable slice");
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

            let mut shared = detections.lock().expect("Failed to lock global detections placeholder");
            shared.assign(&out);
            drop(shared);
        }
    });
    Ok(())
}
