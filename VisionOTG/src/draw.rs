use cairo::Context;
use ndarray::Axis;

use crate::model::SharedDetections;

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

// -----------------------------------------------------
// DRAW CALLBACK
// -----------------------------------------------------
pub fn draw_overlay(cr: &Context, detections: &SharedDetections) {
    let dets = detections.lock().expect("Failed to lock global detections placeholder");
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
                cr.stroke().expect("Failed to draw model detections");

                // Example HUD text
                cr.set_font_size(20.0);
                cr.move_to(xmin * width, ymin * height - 20.0);
                cr.show_text(&format!("{}: {:.2}", obj.class, score))
                    .expect("Failed to show text on overlay");
            }
        },
        Err(e) => {
            eprintln!("Failed to get clip extents: {}", e);
        }
    };
}
