use crate::video::{CameraSource, Video};
pub mod video;

fn main() {
    let mut video = Video::new(
        CameraSource::Online("http://192.168.1.10:4747/video/".to_string()),
        // use another CameraSource if your setup has a Webcam
        (640.0, 480.0),
        60.0,
    ); 
    //let mut temp = &video;
    
    /*
     // Puis passer video et frame à detect_face
        if let Err(e) = detect_face(&mut video, &mut frame) {
            eprintln!("Erreur lors de la détection de visage: {}", e);
        }
    */
    // Initialiser d'abord la vidéo
    if let Err(e) = video.initialize() {
        eprintln!("Erreur d'initialisation de la vidéo: {}", e);
        return;
    }

    // Lire d'abord la frame
    if let Ok(mut frame) = video.read_frame() {
       
    }
    if let Err(e) = video.display("Video Capture from Online Source") {
        eprintln!("Error during display: {}", e);
    }
}