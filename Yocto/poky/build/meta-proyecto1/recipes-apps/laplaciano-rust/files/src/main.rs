use opencv::{
    core::{self, Mat, Size},
    imgproc,
    prelude::*,
    videoio,
    Result,
};
use std::env;

fn main() -> Result<()> {
    // Ruta del video de entrada.
    // Se puede pasar como argumento:
    // cargo run -- /usr/share/videos/video2.mp4
    let args: Vec<String> = env::args().collect();
    let video_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "/usr/share/videos/video2.mp4".to_string()
    };

    // Ruta del video de salida.
    let output_path = "/tmp/video_procesado.avi";

    // Abrir el video de entrada.
    let mut cap = videoio::VideoCapture::from_file(&video_path, videoio::CAP_ANY)?;
    if !cap.is_opened()? {
        eprintln!("Error: no se pudo abrir el video '{}'", video_path);
        return Ok(());
    }

    // Obtener propiedades del video.
    let fps = cap.get(videoio::CAP_PROP_FPS)?;
    let fps = if fps > 0.0 { fps } else { 30.0 };
    let width = cap.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;

    // Crear el escritor de video de salida.
    // MJPG + AVI suele funcionar bien en entornos simples.
    let fourcc = videoio::VideoWriter::fourcc('M', 'J', 'P', 'G')?;
    let mut writer = videoio::VideoWriter::new(
        output_path,
        fourcc,
        fps,
        Size::new(width, height),
        true,
    )?;

    if !writer.is_opened()? {
        eprintln!(
            "Error: no se pudo crear el archivo de salida '{}'",
            output_path
        );
        return Ok(());
    }

    // Bucle principal: leer, procesar y escribir cada frame.
    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;

        // Si no hay más frames, terminar.
        if frame.empty() {
            break;
        }

        // 1) Convertir a escala de grises.
        let mut gray = Mat::default();
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

        // 2) Suavizar para reducir ruido.
        let mut blur = Mat::default();
        imgproc::gaussian_blur(
            &gray,
            &mut blur,
            Size::new(5, 5),
            0.0,
            0.0,
            core::BORDER_DEFAULT,
        )?;

        // 3) Aplicar Laplaciano para resaltar bordes.
        let mut laplace = Mat::default();
        imgproc::laplacian(
            &blur,
            &mut laplace,
            core::CV_16S,
            3,
            1.0,
            0.0,
            core::BORDER_DEFAULT,
        )?;

        // 4) Convertir a 8 bits para poder visualizarlo mejor.
        let mut bordes = Mat::default();
        core::convert_scale_abs(&laplace, &mut bordes, 3.0, 0.0)?;

        // 5) Umbral para eliminar ruido pequeño.
        let mut bordes_limpios = Mat::default();
        imgproc::threshold(
            &bordes,
            &mut bordes_limpios,
            30.0,
            255.0,
            imgproc::THRESH_BINARY,
        )?;

        // 6) Convertir bordes a BGR para poder sumarlos al frame original.
        let mut bordes_bgr = Mat::default();
        imgproc::cvt_color(
            &bordes_limpios,
            &mut bordes_bgr,
            imgproc::COLOR_GRAY2BGR,
            0,
        )?;

        // 7) Combinar frame original + bordes.
        let mut resultado = Mat::default();
        core::add(&frame, &bordes_bgr, &mut resultado, &core::no_array(), -1)?;

        // 8) Escribir el frame procesado al video de salida.
        writer.write(&resultado)?;
    }

    println!("Procesamiento completado. Video guardado en: {}", output_path);
    Ok(())
}
