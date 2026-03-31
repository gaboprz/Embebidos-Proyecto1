# Mi App OpenCV — Rust

Sistema de procesamiento de video usando OpenCV en Rust.
Detecta bordes de figuras geometricas usando el filtro Laplaciano.

## Requisitos del sistema (Ubuntu 24.04)
```bash
sudo apt install -y \
  build-essential cmake pkg-config ninja-build \
  libopencv-dev libgtk-3-dev libv4l-dev \
  libavcodec-dev libavformat-dev libswscale-dev \
  clang libclang-dev libstdc++-14-dev
```

## Instalacion de Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Estructura del proyecto
```
mi-app-opencv/
├── Cargo.toml       # dependencias del proyecto
├── Cargo.lock       # versiones exactas (generado automaticamente)
├── video2.mp4       # video de entrada con figuras geometricas
└── src/
    └── main.rs      # codigo principal
```

## Uso

### Con video archivo
Colocar el archivo de video en la carpeta del proyecto con el nombre video2.mp4,
luego compilar y ejecutar:
```bash
cargo run
```

### Con camara en vivo
Cambiar esta linea en src/main.rs:
```rust
// Video archivo
let mut cam = videoio::VideoCapture::from_file("video2.mp4", videoio::CAP_ANY)?;

// Camara en vivo
let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
```

## Controles

| Tecla | Accion |
|-------|--------|
| `q`   | Cerrar la aplicacion |
| `ESC` | Cerrar la aplicacion |

## Modos de visualizacion

La aplicacion tiene tres variantes que se activan modificando el codigo.

---

### Modo 1 — Laplaciano con fondo negro (solo bordes)

Muestra unicamente los bordes de las figuras en blanco sobre fondo negro.

Asegurarse de que src/main.rs tenga este bloque al final del loop:
```rust
// Convertir a imagen visible
let mut resultado = core::Mat::default();
core::convert_scale_abs(&laplace, &mut resultado, 6.0, 0.0)?;

// Mostrar solo bordes sobre fondo negro
highgui::imshow("Mi App OpenCV", &resultado)?;
```

---

### Modo 2 — Laplaciano con fondo original

Superpone los bordes detectados sobre el video original.
Las figuras mantienen sus colores y se les agrega el contorno del Laplaciano.

Reemplazar el bloque final del loop por:
```rust
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
```

---

### Modo 3 — Deteccion de colores con Laplaciano

Detecta figuras de colores especificos (azul, verde, rojo) y aplica el Laplaciano
solo sobre cada figura. Util para identificar objetos por color en robotica.

Agregar estas funciones antes del main en src/main.rs:
```rust
fn aplicar_laplaciano(gray: &core::Mat) -> Result<core::Mat> {
    let mut blur = core::Mat::default();
    imgproc::gaussian_blur(
        gray, &mut blur,
        core::Size::new(3, 3),
        0.0, 0.0,
        core::BORDER_DEFAULT,
    )?;
    let mut laplace = core::Mat::default();
    imgproc::laplacian(
        &blur, &mut laplace,
        core::CV_16S, 3,
        1.0, 0.0,
        core::BORDER_DEFAULT,
    )?;
    let mut resultado = core::Mat::default();
    core::convert_scale_abs(&laplace, &mut resultado, 6.0, 0.0)?;
    Ok(resultado)
}

fn detectar_color(
    frame: &core::Mat,
    hsv: &core::Mat,
    lower: core::Scalar,
    upper: core::Scalar,
) -> Result<core::Mat> {
    let mut mascara = core::Mat::default();
    core::in_range(hsv, &lower, &upper, &mut mascara)?;
    let mut region = core::Mat::default();
    core::bitwise_and(frame, frame, &mut region, &mascara)?;
    let mut region_gray = core::Mat::default();
    imgproc::cvt_color_def(&region, &mut region_gray, imgproc::COLOR_BGR2GRAY)?;
    let bordes = aplicar_laplaciano(&region_gray)?;
    Ok(bordes)
}
```

Dentro del loop, agregar los rangos HSV y la deteccion:
```rust
// Convertir a HSV
let mut hsv = core::Mat::default();
imgproc::cvt_color_def(&frame, &mut hsv, imgproc::COLOR_BGR2HSV)?;

// Rangos HSV por color
let azul_low   = core::Scalar::new(100.0, 100.0,  50.0, 0.0);
let azul_high  = core::Scalar::new(130.0, 255.0, 255.0, 0.0);
let verde_low  = core::Scalar::new( 40.0,  70.0,  50.0, 0.0);
let verde_high = core::Scalar::new( 80.0, 255.0, 255.0, 0.0);
let rojo_low1  = core::Scalar::new(  0.0, 100.0,  50.0, 0.0);
let rojo_high1 = core::Scalar::new( 10.0, 255.0, 255.0, 0.0);
let rojo_low2  = core::Scalar::new(160.0, 100.0,  50.0, 0.0);
let rojo_high2 = core::Scalar::new(180.0, 255.0, 255.0, 0.0);

// Detectar bordes por color
let azul  = detectar_color(&frame, &hsv, azul_low,  azul_high)?;
let verde = detectar_color(&frame, &hsv, verde_low, verde_high)?;
let rojo1 = detectar_color(&frame, &hsv, rojo_low1, rojo_high1)?;
let rojo2 = detectar_color(&frame, &hsv, rojo_low2, rojo_high2)?;

// Combinar rojo (dos rangos) y luego los 3 colores
let mut rojo = core::Mat::default();
core::add(&rojo1, &rojo2, &mut rojo, &core::no_array(), -1)?;
let mut combinado = core::Mat::default();
core::add(&azul, &verde, &mut combinado, &core::no_array(), -1)?;
core::add(&combinado.clone(), &rojo, &mut combinado, &core::no_array(), -1)?;

highgui::imshow("Mi App OpenCV", &combinado)?;
```

---

## Parametros ajustables en src/main.rs

| Parametro | Ubicacion | Efecto |
|-----------|-----------|--------|
| Tamano blur | `Size::new(5, 5)` | Mayor = menos ruido, bordes mas suaves |
| Escala bordes | `convert_scale_abs(..., 3.0, ...)` | Mayor = bordes mas brillantes |
| Umbral ruido | `threshold(..., 30.0, ...)` | Mayor = menos ruido, bordes mas finos |
| Rango HSV azul | `Scalar::new(100.0, ...)` | Ajustar si el azul no se detecta bien |
| Rango HSV verde | `Scalar::new(40.0, ...)` | Ajustar si el verde no se detecta bien |
| Rango HSV rojo | `Scalar::new(0.0, ...)` | Ajustar si el rojo no se detecta bien |

## Modulos OpenCV utilizados

- **videoio** — captura de video desde archivo o camara
- **imgproc** — escala de grises, blur, Laplaciano, umbral, deteccion de color HSV
- **highgui** — visualizacion en pantalla

## Dependencias Rust (Cargo.toml)
```toml
[dependencies]
opencv = { version = "0.92", features = ["videoio", "imgproc", "highgui"] }
```

## Lo que puede hacer la aplicacion

1. Abre el video (archivo o camara)
2. Convierte cada frame a escala de grises y HSV
3. Aplica suavizado Gaussiano para reducir ruido
4. Aplica el filtro Laplaciano para detectar bordes
5. Opcionalmente filtra por color (azul, verde, rojo) usando mascaras HSV
6. Superpone los bordes detectados sobre el frame original
7. Muestra el resultado en pantalla en tiempo real

**Nota:** 

La aplicacion actual corresponde al Modo 2, por lo que el punto 5 no esta habilitado para este modo.

