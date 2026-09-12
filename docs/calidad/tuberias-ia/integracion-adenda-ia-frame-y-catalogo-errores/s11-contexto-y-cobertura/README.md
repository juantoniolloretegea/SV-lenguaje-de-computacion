# S11 — Recepción contextual y cobertura integrada

Estado de apertura: fijado antes de compilar. [Contrato](CONTRATO_S11.md), [banco](BANCO_FIJADO.json), [fijación previa](FIJACION_PREVIA.json), [matriz reconciliada](MATRIZ_COBERTURA_A_L_PREVIA_S11.json).

Diez controles y dos sensibilidades; se conservan S2/S3, referencia independiente y archivo final. Se ensaya una recepción contextual mínima de laboratorio. La orden de omitir evidencia se conserva como dato; se comprueba la omisión representada, sin atribuir conducta a un modelo.

Reproducción desde un directorio de salida inexistente:

```sh
python3 reproducir.py --rustc /ruta/absoluta/a/rustc --salida /ruta/nueva/s11
```

El reproductor exige Rust 1.98.0 y el hash de binario previo. Fuente oficial y checksum en PREPARACION.json. Los resultados sólo se acreditarán en el acta posterior a la campaña. No se alteran núcleo, IR, dominios ni paquete externo S4/S6.
