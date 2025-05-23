// implement here all the methods related to the video or the camera

use opencv::{
    prelude::*,
    videoio,
    highgui,
    objdetect,
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
    pub face_detector: Option<objdetect::CascadeClassifier>,
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
                let cap = loop{
                    let res = videoio::VideoCapture::from_file(&url, videoio::CAP_ANY)
                        .with_context(|| format!("Failed to connect to {}", url))?;
                    // I think that is a security for prevent bug or connection timeout
                    if res.is_opened()? {
                        break res;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                };
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

    pub fn display(&mut self, title: &str) -> OpenCVResult<()> {
        println!("Connexion Established Press 'q' to Quit");

        loop {
            match self.read_frame() {
                Ok(mut frame) => {
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