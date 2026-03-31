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
