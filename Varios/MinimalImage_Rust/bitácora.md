## 09/03/2026

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