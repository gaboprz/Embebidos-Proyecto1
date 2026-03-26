SUMMARY = "Comando simple para reproducir video con GStreamer"
LICENSE = "CLOSED"

SRC_URI = "file://reproducir_video.sh"

S = "${WORKDIR}"

inherit allarch

do_install() {
    install -d ${D}${bindir}
    install -m 0755 ${S}/reproducir_video.sh ${D}${bindir}/reproducir_video
}

FILES:${PN} += "${bindir}/reproducir_video"
