# Comprobación de custodia e identidad del TTF contratado y de los PNG R01/R06

**Fecha:** 16 de septiembre de 2026. **Seguimiento:** S22. **Objeto:** resolver la precondición de disponibilidad e identidad anterior a la preparación de la cualificación conjunta de LEYENDA-CONTENIDO/4.

## 1. Sede fijada

La custodia se mantiene en `juantoniolloretegea/SV-matematica-semantica-cuaternaria`, corte inmutable `86441ad4d375e31737dfcead0b1fd9cd52161883`, paquete:

`laboratorio/tareas-watson/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-extension-perfiles-recursos-v0_1/EVIDENCIA_RASTER_CAPTOR.tar.gz`

El árbol del corte contiene el paquete con 4.308.172 bytes y blob Git `814619348801577afc28b8fb5be45f73e518d46c`. La comprobación de recepción del mismo corte dejó acreditada la extracción de 43 archivos idénticos, byte a byte, a los de la campaña. No se repiten las ocho sondas raster/captor.

## 2. Identidades fijadas

| Objeto | Identidad SHA-256 | Fundamento |
|---|---|---|
| TTF contratado, `DejaVuSans.ttf` | `ae7b7855e115a5966d8b1b3f80f254ccc117ec86f9965e202ee2940453837280` | Coincide con `HUELLA_TTF_CONTRATADA` de `src/plantillas.rs` en Peritus `6d73c376ca9e6a0d462bdb4483d92cd6e453e285`. |
| PNG R01, miembro `R01.png` | `3e9fae6d4844a07d0f1281d4974c5da1c5ce01c6dbc774be27b18f1735f7509d` | Cotejo receptor con el miembro del paquete y con `MUESTRA_RASTER_PRODUCIDA.png`. |
| PNG R06, miembro `R06.png` | `829286631401c3208554a9f1f81bc091fa5355201490bd92ba72f9b0bb254827` | Cotejo receptor respecto de la referencia de transporte conservada; no se atribuye al manifiesto histórico `IDENTIDADES.txt`. |

La ejecución receptora de identidad fue nueva, empleó Rust 1.98.0 y terminó con retorno cero. El TTF fue incluido en el cotejo adicional de recepción del laboratorio; su huella coincide con el contrato implementado, que vuelve a calcularla sobre los bytes antes de crear `fontdue::Font`.

## 3. Resultado

Queda comprobada la **disponibilidad bajo custodia** del paquete fijado y la **concordancia de identidad** del TTF contratado y de R01/R06 para preparar la cualificación. Esta conclusión se apoya en el árbol inmutable y en las comprobaciones receptoras ya conservadas; no constituye una nueva ejecución del reconocedor.

No se publican los contenidos reservados. No se han ejecutado Q1/Q2 ni E1–E16. Tampoco quedan acreditadas por este acto la independencia del códec, la igualdad de rasterización entre resvg y fontdue, la correspondencia geométrica de U+007C, la validez semántica de R06 ni la suficiencia del método.

## 4. Continuidad

Puede prepararse la cualificación conjunta sobre las 27 celdas congeladas, sin ampliar la retícula ni modificar parámetros. Antes de ejecutarla deben fijarse como entradas las tres huellas anteriores, las órdenes, los esperados de Q1/Q2 y la regla de parada. Un empate residual entre celdas con las cuatro claves de §C iguales es fracaso; `n_min` no desempata.

S22 y (p1+p3)-Bis permanecen en ejecución. El contrato y el reconocedor siguen siendo candidatos. S32 conserva su seguimiento separado. La fase R4 del entorno soberano, el contrato R2-0 de persistencia y el reparo R1 de leyenda mantienen los ámbitos definidos en Acta 001 §9.
