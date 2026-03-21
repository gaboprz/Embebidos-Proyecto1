## 16/03/2026
- Se quiere crear una imagen que se pueda correr en la VM, en la cual se pueda correr un código de python en el que se abra un video y en este se haga una detección de colores, usando OpenCV.
- Para esto se crea el código fuente y se prueba en la computadora nativa. Luego de confirmar que funciona correctamente, se buscan generar las recetas del código y del video para que estas puedan usarse en la VM. El código se coloca en la dirección `meta-proyecto1/recipes-apps/reconocimiento-colores-py/files` y el `.bb` justo afuera de la última carpeta. Este último se ve como:

```bash
    SUMMARY = "Script de Python para reconocimiento de colores"
    DESCRIPTION = "Instala un script Python personalizado dentro de la imagen"
    LICENSE = "CLOSED"

    SRC_URI = "file://detector_colores.py"

    # Carpeta donde bitbake coloca los archivos traídos con file://
    S = "${WORKDIR}"

    # No depende de arquitectura (solo copia un script)
    inherit allarch

    do_install() {
        # Crear directorio /usr/bin en la imagen
        install -d ${D}${bindir}

        # Copiar el script Python y hacerlo ejecutable
        install -m 0755 ${WORKDIR}/detector_colores.py ${D}${bindir}/detector_colores.py
    }

    # Declarar que este archivo pertenece al paquete
    FILES:${PN} += "${bindir}/detector_colores.py"

    # Dependencias necesarias para ejecutar Python
    RDEPENDS:${PN} += "python3"
```
- El cocinar esta receta se hizo con el comando `bitbake reconocimiento-colores-py`. Este inicialmente dio problemas dado que se estaban usando guiones bajos como separador de palabras, al cambiarlos por guiones normales acabaron los problemas.
- Mientras que para el video se sigue una metodología similar a la del primer video añadido.
- Para correr el video se necesita acceso a algunas bibliotecas, como numpy y opencv. Para esto se agrega la receta `python3-numpy` al archivo `local.conf`. Ahora, el opencv no está disponible en las layers que se tienen actualmente. Es por esto que se siguen los siguientes comandos para tener disponible esta biblioteca en forma de receta.

```bash
    git clone -b kirkstone https://github.com/openembedded/meta-openembedded.git
    bitbake-layers add-layer ../meta-openembedded/meta-oe
```

- Ahora, dentro del archivo `local.conf` se agrega la receta `opencv`. Dicho archivo queda como:

```bash
IMAGE_INSTALL:append = " \
        python3 \
        opencv \
        python3-numpy \
        git \
        vim \
        gstreamer1.0 \
        gstreamer1.0-plugins-base \
        gstreamer1.0-plugins-good \
        gstreamer1.0-plugins-bad \
        gstreamer1.0-libav \
        gstreamer1.0-plugins-ugly \
        video2 \
        reconocimiento-colores-py \
    "
```

### Errores / Problemas:
- Guiones bajos no permitidos para nombres en poky.
- Inicialmente se clonó el meta-openembedded de otra versión de poky, lo cual se corrigió instalando el de la versión de kirkstone.


## 20/03/2026
- Ya con las recetas nuevas creadas y cocinadas, se cocina nuevamente la imagen mínima. Esta logra ejecutarse correctamente en la VM. Al correr el código dentro de esta se ve un error con una línea de código, esto dado que estas requieren de una interfaz gráfica para funcionar.

```python
    cv2.imshow("frame", frame)
    if cv2.waitKey(delay) & 0xFF == ord("q"):
        break
```
- Una idea que se tiene es que el video no se muestre "en vivo" con la ejecución del código, sino que este se ejecute y cree un nuevo video que tenga el efecto del código de python. Con este nuevo video creado de manera independiente, con Gstreamer se puede abrir y verificar su correcto resultado. Hubo una dificultad para ejecutar el comando que reproduce el video, pero se logra identificar que era que se estaba escribiendo `videocovert` en lugar de `videoconvert`.
- Finalmente, el código se ejecuta sin mostrar el video, pero genera un archivo de video con el resultado, esto en la ruta `/tmp/videoprocesado.avi`. Evidencia de este video se muestra en seguida:

<figure style="text-align: center; margin: 20px auto;">
  <img src="Imágenes/Detector_Colores_py_VM.png" alt="Placeholder" 
       style="width: 700px; height: auto; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
  <figcaption style="font-style: italic; color: #666;">Resultado del código de python en la VM</figcaption>
</figure>

### Errores / Problemas:
- Apertura de video dentro de la VM. En el código original este requiere de una interfaz gráfica para funcionar.
- Error al reproducir un video en la VM, con el comando que ya antes había funcionado, `gst-launch-1.0 filesrc location=/usr/share/videos/video.mp4 ! decodebin ! videoconvert ! fbdevsink`. Dice que no hay un elemento llamado `videoconvert`.