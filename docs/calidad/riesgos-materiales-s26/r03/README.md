# S26 R03 · Custodia, lectura mezclada y testigo posterior

**Banco previo: RETP-226. Estado inicial: tres sondas pendientes.**

Se ejecuta el relevo de [R02](../RECEPCION_CONTRACTUAL_R02.md), T01/T02/T07,
desde Lenguaje `b0f4f5f0093ad59324fcb6e00ed65cc15e3ca8f9` y laboratorio
`e167aeee5d58c745704000e40395a635e1f25d60`. Los rectores y sedes de R02
conservan su alcance; `FUENTES.json` fija las 26 piezas del montaje y lectura.
La verificación de los bytes publicados de R02 precede a este incremento.

Se reutilizan sin modificar el núcleo archivado y el admisor/observador de
Bis I0205. A contiene los quince archivos de R01 literales. B conserva su
contenido salvo un LF añadido a la fuente y las huellas/tamaño de fuente
concordantes en contexto, solicitud, registro y oráculo. B no representa otra
adquisición del mundo ni un dominio nuevo. Su selección como custodia en el
control es explícita dentro del laboratorio; no acredita autoridad institucional.

| Sonda | Contraste previo | Criterio y límite |
| --- | --- | --- |
| T01 | A/A y B/B frente a B/A, propuesta/custodia. | Los dos positivos deben admitir y certificar recibos completos distintos en identidad de fuente. B bajo la custodia A debe rechazar I02 por `full identity/custody mismatch`. La aceptación B/B demuestra dependencia de la expectativa suministrada, no su autenticación externa. |
| T02 | Sustituir el archivo fuente A por B al final de la lectura de metadatos A. | `Read` instrumentado registra los bytes efectivamente leídos de fuente. Mezcla debe rechazar I02 por `source identity`; sin sustitución debe admitir. Se ensaya una intercalación secuencial precisa, no una carrera multihilo ni una alteración posterior a admisión. |
| T07 | Alterar archivo testigo y suministrar al observador histórico un `after` copiado al inicio o realmente releído. | Se espera conformidad con la copia inicial pese a la alteración del archivo: contraejemplo a una interpretación material fuerte del observador. Con relectura efectiva se espera exactamente `preservacion real del estado`. El estado intacto realmente leído debe pasar. |

T07 caracteriza un límite ya inventariado; una aserción conforme al oráculo
no convierte ese límite en protección. El archivo testigo es una copia local
de ensayo, no una BD SV ni la RAM interna de `AdmittedDelivery`. La sonda no
altera el núcleo ni falsifica los informes históricos. Conserva ambos tipos
de evidencia para hacer visible la diferencia.

`preparar.py` conserva la transformación literal A→B; `probe.rs` contiene los
estímulos y oráculos Rust. `reproducir.py` sólo orquesta: verifica huellas,
crea un directorio nuevo, copia el montaje y compila/ejecuta tres funciones
con Cargo `--locked --offline`, sin dependencias de red. Rust/Cargo deben ser
los 1.98.0 instalados; se registra su versión antes de la prueba. Se mantienen
las cuotas del admisor. Cada orden tiene un límite técnico de 120 segundos;
esto no introduce tiempo en la semántica SV ni acredita rendimiento.

`PRECOMPROMISO.json` congela banco, fixtures, fuentes y scripts antes de la
campaña. Los resultados se añadirán después con referencia al commit previo,
stdout/stderr, órdenes y los archivos efectivamente escritos por Rust.
Python administra archivos y registros; no interpreta semántica SV. La
publicación usa el conector GitHub y los auxiliares S26 identificados en
`publicar.py`; no se invoca otro modelo o proveedor de inferencia.

Los tres ensayos son parciales; no cierran T01/T02/T07 en todos sus alcances
ni los doce casos globales S26. Quedan pendientes persistencia, raíz de
custodia, concurrencia general, adquisición, GUI y protección frente al host.
S22 y S24 conservan su secuencia.
