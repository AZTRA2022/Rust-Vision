use opencv::{
    prelude::*,
    videoio,
    highgui,
    objdetect,
    imgproc,
    core::{self, Vector},
    Result as OpenCVResult,
};
use anyhow::{Result, Context};

pub enum CameraSource {
    Camera(i32),
    Online(String),
    Unavailable,
}

pub struct Video {
    pub source: CameraSource,
    cap: Option<videoio::VideoCapture>,
    pub dimension: (f64, f64),
    pub fps: f64,
    face_detector: Option<objdetect::CascadeClassifier>,
}

impl Video {
    pub fn new(source: CameraSource, dimension: (f64, f64), fps: f64) -> Self {
        Self {
            source,
            cap: None,
            dimension,
            fps,
            face_detector: None,
        }
    }

    fn configure_capture(&self, cap: &mut videoio::VideoCapture) -> Result<()> {
        cap.set(videoio::CAP_PROP_FRAME_WIDTH, self.dimension.0)?;
        cap.set(videoio::CAP_PROP_FRAME_HEIGHT, self.dimension.1)?;
        cap.set(videoio::CAP_PROP_FPS, self.fps)?;
        Ok(())
    }

    pub fn initialize(&mut self) -> Result<()> {
        let mut cap = match &self.source {
            CameraSource::Camera(id) => {
                videoio::VideoCapture::new(*id, videoio::CAP_ANY)?
            },
            CameraSource::Online(url) => {
                let cap = videoio::VideoCapture::from_file(&url, videoio::CAP_ANY)
                    .with_context(|| format!("Failed to connect to {}", url))?;
                if !cap.is_opened()? {
                    anyhow::bail!("Unable to open the flux : {}", url);
                }
                cap
            },
            CameraSource::Unavailable => {
                anyhow::bail!("Aucune caméra disponible")
            }
        };

        self.configure_capture(&mut cap)?;
        self.cap = Some(cap);

        // Load the face detector
        // if you don't have the opencv lib , go on github and download the lib and replace the path below
        let xml = "/usr/local/share/opencv4/haarcascades/haarcascade_frontalface_default.xml";
        self.face_detector = Some(objdetect::CascadeClassifier::new(xml)?);

        Ok(())
    }

    pub fn read_frame(&mut self) -> Result<Mat> {
        let cap = self.cap.as_mut()
            .ok_or_else(|| anyhow::anyhow!("Capture uninitialised"))?;

        let mut frame = Mat::default();
        cap.read(&mut frame)
            .with_context(|| "Error while reading frame")?;

        if frame.empty() {
            anyhow::bail!("Empty frame received");
        }

        Ok(frame)
    }

    pub fn detect_faces(&mut self, frame: &mut Mat) -> Result<()> {
        let mut gray = Mat::default();
        imgproc::cvt_color(frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

        let mut faces = Vector::new();
        if let Some(ref mut detector) = self.face_detector {
            detector.detect_multi_scale(
                &gray,
                &mut faces,
                1.1,
                10,
                objdetect::CASCADE_SCALE_IMAGE,
                core::Size::new(30, 30),
                core::Size::new(0, 0),
            )?;
        }

        for face in faces.iter() {
            imgproc::rectangle(
                frame,
                face,
                core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                2,
                imgproc::LINE_8,
                0,
            )?;
        }

        Ok(())
    }

    pub fn display(&mut self, title: &str) -> OpenCVResult<()> {
        println!("Connexion Established Press 'q' to Quit");

        loop {
            match self.read_frame() {
                Ok(mut frame) => {
                    self.detect_faces(&mut frame);
                    highgui::imshow(title, &frame)?;
                },
                Err(e) => {
                    eprintln!("Capture Error : {}", e);
                    break;
                }
            }

            let key = highgui::wait_key(30)?;
            if key == 'q' as i32 || key == 's' as i32 {
                println!("Program stopped by user");
                break;
            }
        }

        highgui::destroy_all_windows()?;
        Ok(())
    }
}

impl Drop for Video {
    fn drop(&mut self) {
        if let Some(cap) = self.cap.as_mut() {
            let _ = cap.release();
        }
    }
}