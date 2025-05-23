// implement here all the methods related to the face detection 

use opencv::core::{Mat, Vector};
use opencv::{imgproc, objdetect};
use opencv::hub_prelude::CascadeClassifierTrait;
use crate::video::Video;



pub fn detect_face(video: &mut Video, frame: &mut Mat) -> anyhow::Result<()> {
    let mut gray = Mat::default();
    imgproc::cvt_color(frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut faces = Vector::new();
    if let Some(ref mut detector) = video.face_detector {
        detector.detect_multi_scale(
            &gray,
            &mut faces,
            1.1,
            10,
            objdetect::CASCADE_SCALE_IMAGE,
            opencv::core::Size::new(30, 30),
            opencv::core::Size::new(0, 0),
        )?;
    }

    for face in faces.iter() {
        imgproc::rectangle(
            frame,
            face,
            opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
            2,
            imgproc::LINE_8,
            0,
        )?;
    }

    Ok(())
}