use opencv::core::Mat;
use crate::video::{CameraSource, Video};
pub mod video;
mod face;

fn main() {
    let mut video = Video::new(
        CameraSource::Online("http://192.168.1.10:4747/video/".to_string()),
        // use another CameraSource if your computer have a Webcam
        (640.0, 480.0),
        60.0,
    );

    if let Err(e) = video.initialize() {
        eprintln!("Error initializing video: {}", e);
        return;
    }

    if let Err(e) = video.display("Video Capture from Online Source") {
        eprintln!("Error during display: {}", e);
    }
}