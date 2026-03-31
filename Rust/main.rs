use opencv::{
    core,
    highgui,
    imgproc,
    prelude::*,
    videoio,
    Result,
};

fn main() -> Result<()> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    if !cam.is_opened()? {
        panic!("No se pudo abrir la camara");
    }
    println!("Camara abierta correctamente!");

    highgui::named_window("Mi App OpenCV", highgui::WINDOW_AUTOSIZE)?;

    loop {
        let mut frame = core::Mat::default();
        cam.read(&mut frame)?;
        if frame.empty() {
            break;
        }

        let mut gray = core::Mat::default();
        imgproc::cvt_color_def(&frame, &mut gray, imgproc::COLOR_BGR2GRAY)?;

        highgui::imshow("Mi App OpenCV", &gray)?;

        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}