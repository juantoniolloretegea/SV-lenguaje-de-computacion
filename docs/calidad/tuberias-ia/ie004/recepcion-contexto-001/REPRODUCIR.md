# Reproducir la recepción de contexto IE-004

Se reproduce el receptor sobre los bytes depositados por Grok fuera del proyecto, según la confirmación de Juan Antonio. No se vuelve a ejecutar Grok ni se acredita con estas órdenes su contexto conversacional.

El paquete público incluye entrada, oráculo ya abierto, compromiso, salida, observaciones, muestra de sustitución y programa de análisis. No necesita el repositorio privado ni una API de IA. Use una copia del repositorio público que contenga esta recepción; no mezcle archivos de otras versiones.

Desde `docs/calidad/tuberias-ia/ie004/fuentes`, con Rust 1.98.0, destino `wasm32-wasip1` y Node con WASI preview1:

```bash
mkdir -p ../bin-contexto-repro
rustc +1.98.0 --edition=2021 -O -C panic=abort --crate-name receptor_ie004 receptor.rs -o ../bin-contexto-repro/receptor
rustc +1.98.0 --edition=2021 -O -C panic=abort --target wasm32-wasip1 -C link-arg=--max-memory=67108864 --crate-name receptor_ie004 receptor.rs -o ../bin-contexto-repro/receptor.wasm
FUENTE_COMMIT=f8f0fa26da55663bbf24f944d121c05eef8070c9 EJECUTOR_COMMIT=3c4d4d54813c8335be9d4a504a2cc529de4e043d node ejecutar.mjs ../bin-contexto-repro ../contexto-repro ../recepcion-contexto-001/solicitudes-originales.json ../recepcion-contexto-001/entrega-original.json ../recepcion-contexto-001/ORACULO-ABIERTO.json ../recepcion-contexto-001/COMPROMISO-002.json
node ../recepcion-contexto-001/analizar-contexto.mjs ../contexto-repro ../recepcion-002
```

Use un directorio de salida nuevo para cada reproducción; no escriba sobre el paquete custodiado. La segunda orden Node sólo deriva relaciones y comparación documental: no añade procesos del receptor ni consultas al modelo.

La recepción custodiada debe dar 23/24 identificaciones y respuestas correctas, 17/18 consultas con dato y 6/6 diagnósticos/denegaciones debidos. Persiste R01. Paráfrasis: 1/2; contrastes: 5/5; nota externa: 1/1. Frente a 002 anterior: 24/24 rutas/diagnósticos iguales, 24/24 cuerpos iguales, 15/24 apoyos iguales y 23/24 casos correctos en ambas.

Las salidas nativa y WASI coinciden en 5 418 bytes, SHA-256 `a15e3db240bc606aae3a3efa15c86ab750a4398e30a181549ef70ee74774042d`. `LIMITE-SEMANTICO.json` debe seguir mostrando que la propuesta IGG ante la petición de IgA obtiene el dato incorrecto de IGG. El programa comprueba la existencia de esa brecha: su exit 0 no constituye aprobación del mecanismo.

Esta recepción utilizó los binarios originales del puesto `34501044665/1`, sin recompilación local. La reconstrucción pública puede cambiar hashes de binarios según plataforma y toolchain; compare fuente, entradas, criterios y salida. Las medidas de tiempo y entorno son observaciones locales, no valores que deba reproducir numéricamente. CPU/RSS quedan declaradas ausentes cuando falta GNU time. El coste de inferencia no se mide aquí.

Para revisar P1, `EXPEDIENTE-CAUSAL.json` conserva entradas y resultados relevantes. Los originales de L11 permanecen públicamente en `../recepcion-001/`; los originales 002 anteriores en `../recepcion-002/`. El contenido actual de las rutas de entrega de Grok no sustituye esas identidades históricas.

`MANIFIESTO.json` contiene bytes, SHA-256 y blob Git de los demás archivos de esta carpeta; excluye deliberadamente su propio hash. `PROCEDENCIA.json` fija fuente, ejecutor y binarios. Ninguna de estas huellas demuestra por sí sola corrección semántica o aislamiento.
