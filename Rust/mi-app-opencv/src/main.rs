use opencv::{
    core,
    highgui,
    imgproc,
    prelude::*,
    videoio,
    Result,
};

fn main() -> Result<()> {
    let mut cam = videoio::VideoCapture::from_file("video2.mp4", videoio::CAP_ANY)?;
    if !cam.is_opened()? {
        panic!("No se pudo abrir el video");
    }

    highgui::named_window("Mi App OpenCV", highgui::WINDOW_AUTOSIZE)?;

    loop {
        let mut frame = core::Mat::default();
        cam.read(&mut frame)?;
        if frame.empty() {
            break;
        }

        // Convertir a grises
        let mut gray = core::Mat::default();
        imgproc::cvt_color_def(&frame, &mut gray, imgproc::COLOR_BGR2GRAY)?;

        // Suavizado para reducir ruido
        let mut blur = core::Mat::default();
        imgproc::gaussian_blur(
            &gray, &mut blur,
            core::Size::new(5, 5),
            0.0, 0.0,
            core::BORDER_DEFAULT,
        )?;

        // Laplaciano
        let mut laplace = core::Mat::default();
        imgproc::laplacian(
            &blur, &mut laplace,
            core::CV_16S, 3,
            1.0, 0.0,
            core::BORDER_DEFAULT,
        )?;

        // Convertir Laplaciano a 8 bits
        let mut bordes = core::Mat::default();
        core::convert_scale_abs(&laplace, &mut bordes, 3.0, 0.0)?;

        // Umbral para eliminar ruido
        let mut bordes_limpios = core::Mat::default();
        imgproc::threshold(
            &bordes,
            &mut bordes_limpios,
            30.0,
            255.0,
            imgproc::THRESH_BINARY,
        )?;

        // Convertir bordes a BGR para combinar con frame original
        let mut bordes_bgr = core::Mat::default();
        imgproc::cvt_color_def(&bordes_limpios, &mut bordes_bgr, imgproc::COLOR_GRAY2BGR)?;

        // Combinar frame original con los bordes
        let mut resultado = core::Mat::default();
        core::add(&frame, &bordes_bgr, &mut resultado, &core::no_array(), -1)?;

        // Mostrar
        highgui::imshow("Mi App OpenCV", &resultado)?;

        // Salir con q o ESC
        let key = highgui::wait_key(30)?;
        if key == 'q' as i32 || key == 27 {
            break;
        }
    }

    Ok(())
}
