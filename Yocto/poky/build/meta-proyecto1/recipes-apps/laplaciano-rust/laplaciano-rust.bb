SUMMARY = "Detección de bordes en video con Rust y OpenCV"
LICENSE = "CLOSED"

SRC_URI = "file://Cargo.toml \
           file://Cargo.lock \
           file://src/main.rs \
"

S = "${WORKDIR}"

inherit cargo_bin

do_compile[network] = "1"

DEPENDS += "opencv clang-native llvm-native"
