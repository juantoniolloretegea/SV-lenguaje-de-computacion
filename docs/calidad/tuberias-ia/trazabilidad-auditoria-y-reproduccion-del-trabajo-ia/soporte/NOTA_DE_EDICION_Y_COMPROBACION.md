# Edición y comprobación del expediente

## Artefactos y reproducción

- `../TRAZABILIDAD_AUDITORIA_Y_REPRODUCCION_DEL_TRABAJO_IA_2026_09_12.md`: fuente editorial del PDF y de los tres fragmentos de Rust.
- `generar_pdf.py`: generador conservado; requiere Python, ReportLab, Pillow y las fuentes DejaVu Sans, Sans Bold y Sans Mono en `/usr/share/fonts/truetype/dejavu`.
- `verificar_ejemplos.py`: extrae los tres bloques Rust, los envuelve individualmente en `main`, compila y ejecuta cada uno. Recibe la ruta del compilador como argumento.
- `RESULTADOS_EJEMPLOS.json`: identidad y versión del compilador, fuente exacta de cada programa, SHA-256, invocaciones, códigos de salida, stdout, stderr y duraciones observadas. La ruta temporal variable se representa mediante un marcador explícito.

Desde esta carpeta:

```sh
python generar_pdf.py
python verificar_ejemplos.py /ruta/al/compilador/rustc
```

La segunda orden genera un nuevo resultado local; para preservar una campaña anterior, conserve su archivo antes de repetirla. La generación del PDF conserva contenido y presentación, pero sus metadatos pueden variar entre ejecuciones; el manifiesto identifica esta edición concreta.

## Resultado observado

Tres compilaciones y tres ejecuciones correctas con Rust 1.98.0. Cada salida fue exactamente `[4, 8, 12]` seguida de salto de línea. Se conserva una única campaña de estos ejemplos; no se ensayó el núcleo ni se realizó un benchmark. Los tiempos corresponden a los procesos observados, no al coste total de la conversación.

El PDF tiene ocho páginas. Se comprobó su texto y se revisaron visualmente las ocho páginas renderizadas: títulos, tablas, fragmentos y anexos sin recortes. Las cinco imágenes coinciden byte a byte con los originales aportados. La resolución limitada de algunas capturas se compensa mediante la transcripción legible y no mediante retoque.

## Incidencias editoriales conservadas

1. El primer intento de generación del PDF solicitaba además `DejaVuSans-Oblique.ttf`, inexistente en el entorno, y terminó con `TTFError: Can't open file`. No produjo PDF. Se retiró esa dependencia no utilizada; la familia usa la variante regular como alternativa para cursiva. No cambió el texto.
2. En la primera renderización de revisión, `pagina-7.png` quedó truncada (90112 bytes) y la composición de miniaturas falló con `OSError: image file is truncated`. Se volvieron a renderizar las ocho páginas a 90 dpi y la inspección pudo completarse. El documento PDF no se modificó para resolver esa incidencia.
3. Tras comprobar Rust, se añadió al texto la declaración acotada del resultado y se regeneró la edición final. No se alteraron los tres fragmentos ejecutados.

La corrección de la firma `es_par` está visible en el documento y las capturas originales permanecen intactas. No se corrige silenciosamente el material del autor.
