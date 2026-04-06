## **Fecha: 18/03/26** - **Participante: Katherine**

En base a los requerimientos del proyecto, se hizo un breve resumen de criterios para selección de repositorio/modelo de referencia.

- __1. Compatibilidad con Rust:__
El repositorio debe estar escrito en **Rust** o ser fácilmente adaptable, utilizando preferentemente el crate `opencv` para acceder a las funciones nativas de visión por computadora.

- __2. Tríada Funcional Obligatoria:__
El modelo seleccionado debe permitir la integración de tres capacidades clave:
- **Captura** de imágenes o video
- **Procesamiento** de imágenes o video
- **Visualización** de imágenes o video

- __3. Desempeño en Tiempo Casi Real:__
El código debe ser lo suficientemente eficiente para ejecutarse en **tiempo casi real** dentro de una máquina virtual x86 en VirtualBox.

- __4. Modularidad de OpenCV:__
Se recomienda que el ejemplo utilice módulos específicos de OpenCV:
- `imgproc`
- `videoio`
- `dnn`

- __5. Claridad para Documentar Casos de Uso:__
El repositorio debe servir como base para definir:
- El **concepto de operaciones**
- Los **casos de uso** del proyecto


__6. Facilidad de Identificación de Dependencias:__
El código debe permitir identificar claramente qué bibliotecas del sistema operativo son necesarias, para poder:
- Consolidar correctamente las **recetas de Yocto**
- Integrar **Cargo** en el flujo de construcción

---

## **Fecha: 19/03/26** - **Participante: Katherine**

- Se inicia la primera búsqueda de repositorios en [OpenCV-examples](https://docs.opencv.org/4.x/examples.html),
tomando en cuenta los criterios de selección definidos, con el objetivo de descartar opciones que 
podrían añadir complejidad innecesaria al proyecto y quedarse con aquellas que realmente son viables.

- Se visitan los primeros 5 repositorios de la lista de _examples_ en [OpenCV-examples](https://docs.opencv.org/4.x/examples.html).

---

## **Fecha: 21/03/26** - **Participante: Katherine**

- Se continuó con la búsqueda de repositorios en [OpenCV-examples](https://docs.opencv.org/4.x/examples.html) y finalmente se determinaron 2 ejemplos que son útiles para el estudio de la app.

- También se encontró un repositorio en GitHub, [OpenCV-Rust](https://github.com/twistedfall/opencv-rust), que contiene ejemplos escritos en Rust; se seleccionó uno que se puede usar como base para la app.

Este ejemplo contiene lo fundamental solicitado para el proyecto.

### Análisis Comparativo de Ejemplos OpenCV

**Selección de ejemplo de referencia para el proyecto** | Repositorio: `opencv/opencv - samples.cpp` | `twistedfall/opencv-rust (GitHub)`


| Ejemplo | Captura Cámara | Modulo videoio | Modulo imgproc | Sin Archivos Externos | Tiempo Real | Funciones Clave Usadas | Veredicto |
| :--- | :---: | :---: | :---: | :---: | :---: | :--- | :--- |
| **laplace.cpp** | **SI** | **SI** | **SI** | **SI** | **SI** | VideoCapture, GaussianBlur, Laplacian, convertScaleAbs, imshow | **SELECCIONADO** (Cumple los 3 criterios) |
| **edge.cpp (Canny tutorial)** | NO | NO | **SI** | **SI** | NO | blur, Canny, cvtColor, copyTo, imshow, createTrackbar | **REFERENCIA** Conceptual — sin video |
| **video_to_gray.rs** | **SI** | **SI** | **SI** | **SI** | **SI** | VideoCapture::new(0), cvt_color_def, imshow | **BASE DE CODIGO** Esqueleto en Rust |
| **lkdemo.cpp** | **SI** | **SI** | **SI** | **SI** | **SI** | VideoCapture, calcOpticalFlowPyrLK, goodFeaturesToTrack | **DESCARTADO** Demasiado complejo |
| **camshiftdemo.cpp** | **SI** | **SI** | **SI** | **SI** | **SI** | VideoCapture, CamShift, calcBackProject, calcHist | **DESCARTADO** Requiere mouse |
| **video-write.cpp** | PARCIAL | **SI** | NO | **SI** | NO | VideoCapture(archivo), VideoWriter, split, merge | **DESCARTADO** Sin cámara |
| **facedetect.cpp** | **SI** | **SI** | **SI** | NO | **SI** | VideoCapture, CascadeClassifier, detectMultiScale | **DESCARTADO** Requiere .xml externo |

---

### Leyenda:
* **SI**: Criterio cumplido
* **PARCIAL**: Cumplido con limitaciones
* **NO**: Criterio no cumplido


---

## **Fecha: 23/03/26** - **Participante: Katherine**

Se da inicio a la creación de la app.

__Actividades realizadas:__

- __1. Instalación del entorno de desarrollo:__

Se instalaron las herramientas necesarias para el desarrollo de la aplicación en Ubuntu 24.04:

* Rust `1.94.0` vía rustup  
* OpenCV `4.6.0` vía `apt`  
* Dependencias del sistema: `build-essential`, `cmake`, `pkg-config`, `libopencv-dev`, `libgtk-3-dev`, `libv4l-dev`, `clang`, `libclang-dev`, `libstdc++-14-dev`

- __2. Creación del proyecto base en Rust:__

Se creó el proyecto `mi-app-opencv` con `cargo new`. Se configuró el `Cargo.toml` con el crate `opencv = "0.92"` y los features `videoio`, `imgproc` y `highgui`.

- __3. Implementación y prueba de la aplicación base:__

Se implementó el código base en `src/main.rs` con la tríada funcional completa:

* Captura de video desde cámara (`VideoCapture::new(0)`)
* Procesamiento: conversión a escala de grises (`cvt_color_def`)
* Visualización en tiempo real (`imshow`)

La aplicación compiló y ejecutó correctamente, mostrando la cámara en escala de grises en tiempo real.

### Problemas encontrados y soluciones

| Problema | Solución |
| :--- | :--- |
| Error `fatal error: 'memory' file not found` al compilar | Instalar `libstdc++-14-dev` |
| Terminal interpreta `!` como comando especial al usar `cat <<` | Usar `gedit` para editar archivos |

### Próximos pasos

* Agregar el filtro Laplaciano basado en `laplace.cpp`

---

## **Fecha: 30/03/26** - **Participante: Katherine**

### Actividades

1. **Implementación del Laplaciano** 

- Se modificó el código de `main.rs`, donde se implementó en *Rust* el Laplaciano. Se hicieron varias pruebas para verificar su funcionamiento, tales como:
    - Escala a grises con identificación de bordes aplicando el Laplaciano usando la cámara en vivo de la laptop.
    - Identificación de colores usando el Laplaciano con cámara en vivo.

El objetivo de las pruebas fue familiarizarse con el código, el filtro y las máscaras, además de hacer ajustes en los bordes y buscar un valor en los parámetros para disminuir el ruido, puesto que el Laplaciano es sensible al ruido por valores muy bajos y se quiere evitar ver este ruido en la aplicación.
    

2. **Modificación del Laplaciano para cargar video**

- Se modificó el código para cargar el video que usaba Gabriel.
- Se aplicó el Laplaciano en el video.
- Se hicieron pruebas usando el Laplaciano sobre el video, como:
    - Video en negro donde solamente se identifican los bordes de las figuras con el Laplaciano.
    - Video original aplicando el Laplaciano.

### Problemas encontrados

A pesar de haber hecho pruebas para evitar ver ruido, se detectó ruido en las figuras; las figuras con tonos suaves se observaban con mayor ruido.

### Solución

Se usó:

- 1. Suavizado gaussiano (antes del Laplaciano)
```rust
imgproc::gaussian_blur(
    &gray, &mut blur,
    core::Size::new(5, 5),  // kernel 5x5
    0.0, 0.0,
    core::BORDER_DEFAULT,
)?;
```

- 2. Umbral binario (después del Laplaciano)
```rust
imgproc::threshold(
    &bordes, &mut bordes_limpios,
    30.0,   // valor umbral
    255.0,
    imgproc::THRESH_BINARY,
)?;
```

En resumen, el flujo sería:

Imagen → Gaussian Blur (elimina ruido ANTES)  
       → Laplaciano  
       → Threshold (elimina ruido DESPUÉS)  
       → Bordes limpios
       
__Evidencia:__

- 1. En [Ruido](https://github.com/gaboprz/Embebidos-Proyecto1/blob/main/Bit%C3%A1cora/Im%C3%A1genes/Ruido.png) se pueden observar las figuras aplicando el Laplaciano y se observa alrededor del borde el ruido.

- 2. En [Sin-ruido](https://github.com/gaboprz/Embebidos-Proyecto1/blob/main/Bit%C3%A1cora/Im%C3%A1genes/Sin-ruido.png) se puede observar que la solución encontrada para eliminar el ruido en el Laplaciano tuvo éxito.
 
---

## **Fecha: 31/03/26** - **Participante: Katherine**

Se inicia la prueba del tutorial en Yocto.

---

## **Fecha: 01/04/26** - **Participante: Katherine**

- Se continúa con el tutorial  
- Se termina el tutorial  
- Se inicia con la documentación de Rust  

---

## **Fecha: 02/04/26** - **Participante: Katherine**  

Se termina la documentación
