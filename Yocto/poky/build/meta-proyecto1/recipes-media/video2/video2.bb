SUMMARY = "Instala un video dentro de la imagen"
LICENSE = "CLOSED"
LIC_FILES_CHKSUM = "file://LICENSE;md5=092cecf55e2bc9a2a5e8378656d2d161"

SRC_URI = "file://video2.mp4 \
           file://LICENSE \
"

S = "${WORKDIR}"

inherit allarch

do_install() {
    install -d ${D}${datadir}/videos
    install -m 0644 ${S}/video2.mp4 ${D}${datadir}/videos/video2.mp4
    install -m 0644 ${S}/LICENSE ${D}${datadir}/videos/LICENSE
}

FILES:${PN} += "${datadir}/videos ${datadir}/videos/*"

