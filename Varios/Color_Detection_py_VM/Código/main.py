#!/usr/bin/env python3
# main.py

import cv2
import numpy as np
import sys

# ---------- PARÁMETROS AJUSTABLES ----------
# Ruta por defecto en la imagen Yocto.
# Si quieres pasar otra ruta, usa:
# python3 detector_colores.py /ruta/al/video.mp4
VIDEO_PATH = sys.argv[1] if len(sys.argv) > 1 else "/usr/share/videos/video2.mp4"

RESIZE_WIDTH = 320    # ancho para k-means (velocidad). Ajusta si quieres más precisión.
K = 6                 # número de clusters (colores dominantes) por frame
S_MIN = 50            # saturación mínima para considerar píxel "no negro"
V_MIN = 50            # brillo mínimo para considerar píxel "no negro"
MIN_AREA = 500        # area mínima (en pixeles en resolución ORIGINAL) para dibujar la caja
BOX_COLOR = (0, 255, 255)  # amarillo en BGR para la caja/mascara

# Archivo de salida con el video procesado.
OUTPUT_PATH = "/tmp/video_procesado.avi"
# -------------------------------------------

# Paleta ampliada de nombres de colores (BGR). NO incluir negro.
NAMED_COLORS = {
    "rojo":        [0, 0, 255],
    "naranja":     [0, 165, 255],
    "amarillo":    [0, 255, 255],
    "verde":       [0, 255, 0],
    "verde lima":  [50, 205, 50],     # lime
    "verde oliva": [0, 128, 128],     # olive-ish
    "cian":        [255, 255, 0],
    "turquesa":    [208, 224, 64],
    "azul":        [255, 0, 0],
    "azul marino": [128, 0, 0],       # navy-ish
    "celeste":     [235, 206, 135],
    "índigo":      [130, 0, 75],
    "morado":      [128, 0, 128],
    "lila":        [250, 230, 230],
    "fucsia":      [255, 0, 255],
    "rosa":        [203, 192, 255],
    "salmón":      [114, 128, 250],
    "coral":       [80, 127, 255],
    "marrón":      [19, 69, 139],
    "chocolate":   [30, 105, 210],
    "beige":       [220, 245, 245],
    "crema":       [208, 253, 255],
    "gris":        [128, 128, 128],
    "blanco":      [255, 255, 255],
    "dorado":      [0, 215, 255],
    "plata":       [192, 192, 192],
    "ámbar":       [0, 191, 255],
    "melocotón":   [185, 218, 255],
    "menta":       [201, 252, 189],
    "teal":        [128, 128, 0],
    "turquesa claro":[204, 255, 229],
    "lavanda":     [250, 230, 255],
    "salvia":      [142, 199, 141],
    "ocre":        [0, 128, 255],
    "naranja claro": [0, 200, 255],
    "naranja oscuro": [0, 100, 200],
    "blanco hueso":[220, 245, 245],
}

def bgr_array_to_lab(arr_bgr_uint8):
    """
    Convierte un array BGR uint8 a Lab float32.
    """
    if arr_bgr_uint8.size == 0:
        return np.zeros((0, 3), dtype=np.float32)

    reshaped = arr_bgr_uint8.reshape((-1, 1, 3))
    lab = cv2.cvtColor(reshaped, cv2.COLOR_BGR2LAB)
    return lab.reshape((-1, 3)).astype(np.float32)

# Precompute named colors in Lab space
named_bgr = np.array(list(NAMED_COLORS.values()), dtype=np.uint8)
named_names = list(NAMED_COLORS.keys())
named_lab = bgr_array_to_lab(named_bgr)

def nearest_color_name(bgr_color):
    """Devuelve el nombre de la paleta más cercano al color BGR dado."""
    bgr_u8 = np.uint8(np.round(bgr_color)).reshape((1, 3))
    lab = bgr_array_to_lab(bgr_u8)[0]
    dists = np.linalg.norm(named_lab - lab, axis=1)
    idx = int(np.argmin(dists))
    return named_names[idx]

def main():
    cap = cv2.VideoCapture(VIDEO_PATH)
    if not cap.isOpened():
        print(f"Error: no se pudo abrir '{VIDEO_PATH}'")
        sys.exit(1)

    fps = cap.get(cv2.CAP_PROP_FPS)
    delay = int(1000 / fps) if fps and fps > 0 else 30

    # El writer se crea cuando ya tengamos el primer frame,
    # para asegurar que conocemos el tamaño real del video.
    out = None
    orig_w = None
    orig_h = None

    while True:
        ret, frame = cap.read()
        if not ret:
            print("Fin del video o no se pudo leer el frame. Saliendo.")
            break

        if frame is None:
            continue

        # Inicializar writer con el tamaño real del video.
        if out is None:
            orig_h, orig_w = frame.shape[:2]
            out = cv2.VideoWriter(
                OUTPUT_PATH,
                cv2.VideoWriter_fourcc(*"MJPG"),
                fps if fps and fps > 0 else 30,
                (orig_w, orig_h)
            )

            if not out.isOpened():
                print(f"Error: no se pudo crear el archivo de salida '{OUTPUT_PATH}'")
                cap.release()
                sys.exit(1)

        h, w = frame.shape[:2]
        scale = RESIZE_WIDTH / float(w)
        small = cv2.resize(frame, (RESIZE_WIDTH, int(h * scale)), interpolation=cv2.INTER_AREA)

        hsv_small = cv2.cvtColor(small, cv2.COLOR_BGR2HSV)
        s_channel = hsv_small[:, :, 1]
        v_channel = hsv_small[:, :, 2]
        mask_nonblack = (s_channel > S_MIN) & (v_channel > V_MIN)

        samples = small[mask_nonblack]
        if samples.shape[0] < K:
            # Antes se mostraba aquí con cv2.imshow, pero en la imagen mínima
            # no hay entorno gráfico. Solo guardamos el frame sin cambios.
            out.write(frame)
            continue

        samples_f = np.float32(samples)
        criteria = (cv2.TERM_CRITERIA_EPS + cv2.TERM_CRITERIA_MAX_ITER, 10, 1.0)
        flags = cv2.KMEANS_PP_CENTERS
        compactness, labels, centers = cv2.kmeans(samples_f, K, None, criteria, 8, flags)

        labels_flat = -np.ones((small.shape[0] * small.shape[1]), dtype=np.int32)
        mask_idx = np.flatnonzero(mask_nonblack.flatten())
        labels_flat[mask_idx] = labels.flatten()
        labels_img = labels_flat.reshape((small.shape[0], small.shape[1]))

        for ci in range(centers.shape[0]):
            mask_cluster_small = (labels_img == ci).astype(np.uint8) * 255
            if cv2.countNonZero(mask_cluster_small) < 10:
                continue

            mask_cluster = cv2.resize(mask_cluster_small, (orig_w, orig_h), interpolation=cv2.INTER_NEAREST)
            kernel = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (5, 5))
            mask_cluster = cv2.morphologyEx(mask_cluster, cv2.MORPH_OPEN, kernel, iterations=1)
            mask_cluster = cv2.morphologyEx(mask_cluster, cv2.MORPH_CLOSE, kernel, iterations=1)

            contours, _ = cv2.findContours(mask_cluster, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
            if not contours:
                continue

            for cnt in contours:
                area = cv2.contourArea(cnt)
                if area < MIN_AREA:
                    continue

                x, y, wbox, hbox = cv2.boundingRect(cnt)
                color_name = nearest_color_name(centers[ci])

                # Dibujar rectángulo amarillo y texto sobre el frame procesado.
                cv2.rectangle(frame, (x, y), (x + wbox, y + hbox), BOX_COLOR, 3)
                text_y = y - 10 if (y - 10) > 10 else y + 20
                cv2.putText(frame, color_name, (x, text_y),
                            cv2.FONT_HERSHEY_SIMPLEX, 0.8, (255, 255, 255), 2)

        # Guardar el frame procesado.
        out.write(frame)

        # Líneas antiguas que ya no sirven en una imagen mínima sin GUI:
        # cv2.imshow("frame", frame)
        # if cv2.waitKey(delay) & 0xFF == ord("q"):
        #     break

    cap.release()
    if out is not None:
        out.release()

    # Sin GUI no hace falta destruir ventanas.
    # cv2.destroyAllWindows()

if __name__ == "__main__":
    main()