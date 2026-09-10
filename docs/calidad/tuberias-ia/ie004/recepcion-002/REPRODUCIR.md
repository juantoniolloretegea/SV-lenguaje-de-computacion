# Reproducir la recepción 002 de IE-004

Paquete público. Se reproduce el receptor sobre la entrega depositada de Grok, sin acceso privado ni API de IA. Estas órdenes no vuelven a ejecutar el modelo ni prueban independencia de su sesión.

Use Rust 1.98.0 con destino `wasm32-wasip1` y Node con WASI preview1. Desde `docs/calidad/tuberias-ia/ie004/fuentes` de una copia de la publicación:

```bash
mkdir -p ../bin-recepcion-002-repro
rustc +1.98.0 --edition=2021 -O -C panic=abort --crate-name receptor_ie004 receptor.rs -o ../bin-recepcion-002-repro/receptor
rustc +1.98.0 --edition=2021 -O -C panic=abort --target wasm32-wasip1 -C link-arg=--max-memory=67108864 --crate-name receptor_ie004 receptor.rs -o ../bin-recepcion-002-repro/receptor.wasm
node ejecutar.mjs ../bin-recepcion-002-repro ../recepcion-002-repro ../recepcion-002/solicitudes-originales.json ../recepcion-002/entrega-original.json ../recepcion-002/ORACULO-ABIERTO.json ../recepcion-002/COMPROMISO-002.json
node ../recepcion-002/analizar-recepcion.mjs ../recepcion-002-repro ../recepcion-001
```

Se conserva el receptor y el observador ya publicados. El segundo programa calcula la lectura funcional y compara los ocho repetidos con la recepción 001; no cambia los criterios congelados ni ejecuta más consultas.

Resultado funcional custodiado: 23/24 identificaciones y respuestas; 17/18 consultas legítimas con dato y 6/6 diagnósticos/denegaciones. R01 es el único fallo. Paráfrasis 1/2, contrastes 5/5, nota externa 1/1. La comparación da rutas/diagnósticos iguales 8/8, cuerpos iguales 7/8 y resultados correctos en ambas 6/8.

Las dos capturas deben coincidir por bytes: 5 418 bytes y SHA-256 `a15e3db240bc606aae3a3efa15c86ab750a4398e30a181549ef70ee74774042d`. Debe conservarse el testigo de brecha semántica en `LIMITE-SEMANTICO.json`. Un exit 0 no significa aprobación del candidato.

La recepción publicada utilizó los binarios originales del puesto `34501044665/1`, sin recompilación local. Una reconstrucción con otra plataforma puede cambiar sus hashes; no se exige identidad de binarios entre plataformas o toolchains distintas. Tiempos y metadatos de entorno tampoco serán necesariamente iguales. CPU/RSS se declaran ausentes cuando falta GNU time. Las medidas no corresponden a la inferencia de Grok.

`MANIFIESTO.json` permite comprobar los bytes depositados de cada archivo de esta carpeta salvo el propio manifiesto; no contiene una huella autorreferente. Las fuentes reutilizadas y los binarios originales se identifican en `PROCEDENCIA.json`.
