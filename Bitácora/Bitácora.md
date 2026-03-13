### **Fecha:** 02/03/2026 - **Participante:** Gabriel Pérez

- Se instala Yocto Project, versión Kirkstone 4.0.33, usando la guía oficial de la página [Yocto](https://docs.yoctoproject.org/kirkstone/brief-yoctoprojectqs/index.html) . Además, se genera una imagen mínima usando el comando `core-image-minimal`, la cual luego se corre con `runqemu qemux86-64`.

### **Fecha:** 04/03/2026  - **Participante:** Gabriel Pérez

- Con el fin de entender cómo usar opencv, se sigue un ejemplo, [Video](https://www.youtube.com/watch?v=aFNDh5k3SjU&list=PLb49csYFtO2Hpfn8eLnaD9tJ0xYcMVcWe), en el cual se implementa un código de detección de colores usando la cámara de la computadora, esto usando python. 
- En este lo primero que se hace es crear un entorno virtual, en el que luego se van a instalar algunas librerías necesarias y ejecutar el proyecto. Para esto se corren los siguientes comandos en la terminal.

  ```bash
    # Instala paquetes para poder crear el entorno virtual  
    sudo apt install python3-full python3-venv 

    # Se navega hasta estar en el directorio deseado. Luego se crea el entorno
    python3 -m venv venv

    # Se activa el entorno
    source venv/bin/activate

    # Ahora se instalan los requerimientos
    pip install -r requirements.txt
  ```

- Luego, se toman los códigos presentes en el repositorio del autor del video, [Repo](https://github.com/computervisioneng/color-detection-opencv/blob/master/main.py), y se corre dicho código.


- También se recrea el código que se muestra en [Video](https://www.youtube.com/watch?v=zcfixnuJFXg), en el cual se hace una aplicación sencilla usando Rust y OpenCV. Para esta lo primero que se hizo fue instalar Rust desde la página oficial. Luego de esto se siguió el video hasta tener el código listo. Al tratar de correrlo con `cargo run`, la compilación daba error. Esto es dado que no tenía instaladas las dependencias de OpenCV, por lo que se ejecuta `sudo apt install -y build-essential cmake pkg-config ninja-build libopencv-dev libgtk-3-dev libv4l-dev libavcodec-dev libavformat-dev libswscale-dev`. Luego de esto, el programa funciona correctamente, mostrando la cámara en vivo de la laptop.

### **Fecha:** 08/03/2026 - **Participante:** Gabriel Pérez
- Se decide crear una imagen mínima y ejecutarla en una máquina virtual. Para esto se sigue, entre otros recursos, el tutorial que se muestra en [Página](https://gmacario.github.io/posts/2015-11-14-running-yocto-image-inside-virtualbox). Lo primero que se hace es instalar la aplicación Oracle VirtualBox, esto desde la página oficial. Una vez se tenía lista, se pone a cocinar la imagen mínima, pero antes de eso, en el archivo `/home/gabo/poky/build/conf/local.conf` se añade la línea:
   ```bash
    #Image Format
    IMAGE_FSTYPES += "wic.vmdk wic iso"
  ```
- Esto hace que se genere la imagen con un formato que es posible ejecutar en el VirtualBox. Luego de generada, en VirtualBox se elige el crear una nueva máquina. En esta se especifica un nombre, se coloca el sistema operativo como Linux, la versión como Other Linux y que sea de 64 bits. Luego de esto, se eligen 1024 MB de memoria base y 4 núcleos de procesamiento. Luego, en la selección del disco virtual, se elige la opción de usar un disco duro virtual existente, en donde se elige el archivo `/home/gabo/poky/build/tmp/deploy/images/genericx86-64/core-image-minimal-genericx86-64.wic.vmdk`. Finalmente, se abre la máquina virtual y se verifica que esta ejecuta correctamente.

<figure style="text-align: center; margin: 20px auto;">
  <img src="Imágenes/VirtualBox_Prueba1.png" alt="Placeholder" 
       style="width: 600px; height: auto; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
  <figcaption style="font-style: italic; color: #666;">Máquina Virtual de imagen mínima</figcaption>
</figure>

### **Fecha:** 09/03/2026 - **Participante:** Gabriel Pérez
- Se quiere ver el procedimiento para poder generar una imagen que ya no sea vanilla, es decir, que se pueda poner algo en las recetas. En este caso, en primera instancia se busca cocinar una imagen mínima, la cual tenga posibilidad de usar los paquetes de git, python y rust. Con estos debidamente configurados, se quiere confirmar que realmente se puedan ejecutar códigos dentro de la imagen.
- Para empezar se está tomando información de una serie de videos de YouTube, específicamente de [Playlist](https://www.youtube.com/playlist?list=PLwqS94HTEwpQmgL1UsSwNk_2tQdzq3eVJ).
- Una de las cosas que menciona el autor en [Video](https://www.youtube.com/watch?v=naszh2WoHAM&list=PLwqS94HTEwpQmgL1UsSwNk_2tQdzq3eVJ&index=6), es que hay que confirmar si los paquetes que se quieren agregar a la receta están disponibles de antemano. Para ver esto se usa el código
```bash
source oe-init-build-env 
# Mostrar todas las recetas
bitbake-layers show-recipes
# Mostrar receta específica
bitbake-layers show-recipes python3
```
- El resultado de esto se ve a continuación:
```bash
NOTE: Starting bitbake server...
Loading cache: 100% |############################################ Time: 0:00:00
Loaded 1644 entries from dependency cache.
=== Matching recipes: ===
git:
meta                 2.35.7
```
- Entonces, lo que hay que hacer es añadir el layer "meta" al archivo `/home/gabo/poky/build/conf/bblayers.conf`. Sin embargo, esto ya se encuentra ahí, como se puede ver en seguida:
```bash
BBLAYERS ?= " \
  /home/gabo/poky/meta \
  /home/gabo/poky/meta-poky \
  /home/gabo/poky/meta-yocto-bsp \
  "
```
- De no haber estado, se puede añadir con el comando `bitbake-layers add-layer meta`.

- Luego de esto, se deben añadir los paquetes específicos que se necesitan dentro del archivo `/home/gabo/poky/build/conf/local.conf`. En la parte final de este se coloca:

```bash
IMAGE_INSTALL:append = " \
    python3 \
    git \
    rust-hello-world \
"
```

- Ya con las configuraciones ejecutadas, se procede a cocinar la imagen. Esta luego se prueba usando el comando `runeqemu genericx86-64`. En esta se comprueba que están disponibles los programas deseados, además de que se prueba el python y el rust, tal como se muestra a continuación:

<figure style="text-align: center; margin: 20px auto;">
  <img src="Imágenes/Comprobacion.png" alt="Placeholder" 
       style="width: 600px; height: auto; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
  <figcaption style="font-style: italic; color: #666;">Comprobación de imagen mínima con Rust y Python</figcaption>
</figure>

- Finalmente se prueba la imagen desde VirtualBox, y se comprueba su correcto funcionamiento.
<figure style="text-align: center; margin: 20px auto;">
  <img src="Imágenes/Comprobacion_VM.png" alt="Placeholder" 
       style="width: 600px; height: auto; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
  <figcaption style="font-style: italic; color: #666;">Comprobación de imagen mínima con Rust y Python, en VirtualBox</figcaption>
</figure>

### **Fecha:** - **Participante:** 