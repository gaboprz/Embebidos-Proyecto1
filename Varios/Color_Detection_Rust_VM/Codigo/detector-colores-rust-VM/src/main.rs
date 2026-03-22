use opencv::{
    core::{self, Mat, Scalar, Vector, Point, Size},
    imgproc,
    videoio,
    prelude::*,
    Result,
};
use std::env;

fn main() -> Result<()> {
    // Ruta del vídeo de entrada (se puede pasar como argumento)
    let args: Vec<String> = env::args().collect();
    let video_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "video2.mp4".to_string()
    };

    // Ruta de salida fija
    let output_path = "/tmp/video_procesado.avi";

    // Abrir el vídeo de entrada
    let mut cap = videoio::VideoCapture::from_file(&video_path, videoio::CAP_ANY)?;
    if !cap.is_opened()? {
        eprintln!("Error: no se pudo abrir el vídeo '{}'", video_path);
        return Ok(());
    }

    // Obtener propiedades del vídeo
    let fps = cap.get(videoio::CAP_PROP_FPS)?;
    let fps = if fps > 0.0 { fps } else { 30.0 };
    let width = cap.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;

    // Crear el escritor de vídeo (codec MJPG, .avi)
    let fourcc = videoio::VideoWriter::fourcc('M', 'J', 'P', 'G')?;
    let mut writer = videoio::VideoWriter::new(
        output_path,
        fourcc,
        fps,
        Size::new(width, height),
        true, // isColor
    )?;

    if !writer.is_opened()? {
        eprintln!("Error: no se pudo crear el archivo de salida '{}'", output_path);
        return Ok(());
    }

    // Bucle principal
    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;
        if frame.empty() {
            break; // Fin del vídeo
        }

        // Convertir a HSV
        let mut hsv = Mat::default();
        imgproc::cvt_color(&frame, &mut hsv, imgproc::COLOR_BGR2HSV, 0)?;

        // Definir rangos para el color rojo en HSV
        let lower_red1 = Scalar::new(0.0, 50.0, 50.0, 0.0);
        let upper_red1 = Scalar::new(10.0, 255.0, 255.0, 0.0);
        let lower_red2 = Scalar::new(160.0, 50.0, 50.0, 0.0);
        let upper_red2 = Scalar::new(180.0, 255.0, 255.0, 0.0);

        // Crear máscaras para cada rango
        let mut mask1 = Mat::default();
        let mut mask2 = Mat::default();
        core::in_range(&hsv, &lower_red1, &upper_red1, &mut mask1)?;
        core::in_range(&hsv, &lower_red2, &upper_red2, &mut mask2)?;

        // Combinar ambas máscaras
        let mut mask = Mat::default();
        core::bitwise_or(&mask1, &mask2, &mut mask, &Mat::default())?;

        // Aplicar operaciones morfológicas: apertura y cierre
        let kernel = imgproc::get_structuring_element(
            imgproc::MORPH_ELLIPSE,
            Size::new(5, 5),
            Point::new(-1, -1),
        )?;

        let mut mask_opened = Mat::default();
        imgproc::morphology_ex(
            &mask,
            &mut mask_opened,
            imgproc::MORPH_OPEN,
            &kernel,
            Point::new(-1, -1),
            1,
            0,
            Scalar::all(0.0),
        )?;

        let mut mask_closed = Mat::default();
        imgproc::morphology_ex(
            &mask_opened,
            &mut mask_closed,
            imgproc::MORPH_CLOSE,
            &kernel,
            Point::new(-1, -1),
            1,
            0,
            Scalar::all(0.0),
        )?;

        mask = mask_closed;

        // Encontrar contornos
        let mut contours = Vector::<Vector<Point>>::new();
        imgproc::find_contours(
            &mask,
            &mut contours,
            imgproc::RETR_EXTERNAL,
            imgproc::CHAIN_APPROX_SIMPLE,
            Point::new(0, 0),
        )?;

        // Dibujar rectángulos sobre los contornos de área suficiente
        let min_area = 500.0;
        for i in 0..contours.len() {
            let contour = contours.get(i)?;
            let area = imgproc::contour_area(&contour, false)?;
            if area > min_area {
                let rect = imgproc::bounding_rect(&contour)?;
                // Rectángulo amarillo
                imgproc::rectangle(
                    &mut frame,
                    rect,
                    Scalar::new(0.0, 255.0, 255.0, 0.0),
                    3,
                    imgproc::LINE_8,
                    0,
                )?;
                // Etiqueta "rojo"
                let text = "rojo";
                let point = Point::new(rect.x, rect.y - 10);
                imgproc::put_text(
                    &mut frame,
                    text,
                    point,
                    imgproc::FONT_HERSHEY_SIMPLEX,
                    0.8,
                    Scalar::new(255.0, 255.0, 255.0, 0.0),
                    2,
                    imgproc::LINE_8,
                    false,
                )?;
            }
        }

        // Escribir el frame procesado en el vídeo de salida
        writer.write(&frame)?;
    }

    println!("Procesamiento completado. Vídeo guardado en: {}", output_path);
    Ok(())
}
