use opencv::{
    core::{self, Mat, Scalar, Vector, Point, Size},
    highgui,
    imgproc,
    videoio,
    prelude::*,
    Result,
};

fn main() -> Result<()> {
    // Ruta del vídeo (cámbiala por la tuya)
    let video_path = "video2.mp4";

    // Abrir el vídeo
    let mut cap = videoio::VideoCapture::from_file(video_path, videoio::CAP_ANY)?;
    if !cap.is_opened()? {
        eprintln!("Error: no se pudo abrir el vídeo '{}'", video_path);
        return Ok(());
    }

    // Crear una ventana para mostrar el vídeo
    highgui::named_window("Detector de rojo", highgui::WINDOW_NORMAL)?;

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

        // Combinar ambas máscaras (con máscara vacía como último argumento)
        let mut mask = Mat::default();
        core::bitwise_or(&mask1, &mask2, &mut mask, &Mat::default())?;

        // Aplicar operaciones morfológicas: apertura y cierre
        let kernel = imgproc::get_structuring_element(
            imgproc::MORPH_ELLIPSE,
            Size::new(5, 5),
            Point::new(-1, -1),
        )?;

        // Primero apertura (OPEN) para eliminar ruido
        let mut mask_opened = Mat::default();
        imgproc::morphology_ex(
            &mask,                      // entrada
            &mut mask_opened,           // salida
            imgproc::MORPH_OPEN,
            &kernel,
            Point::new(-1, -1),
            1,
            0,
            Scalar::all(0.0),
        )?;

        // Luego cierre (CLOSE) para unir regiones
        let mut mask_closed = Mat::default();
        imgproc::morphology_ex(
            &mask_opened,               // entrada
            &mut mask_closed,           // salida
            imgproc::MORPH_CLOSE,
            &kernel,
            Point::new(-1, -1),
            1,
            0,
            Scalar::all(0.0),
        )?;

        // Asignar la máscara final
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

        // Mostrar el frame procesado
        highgui::imshow("Detector de rojo", &frame)?;
        let key = highgui::wait_key(30)?;
        if key == 113 {
            break;
        }
    }

    Ok(())
}