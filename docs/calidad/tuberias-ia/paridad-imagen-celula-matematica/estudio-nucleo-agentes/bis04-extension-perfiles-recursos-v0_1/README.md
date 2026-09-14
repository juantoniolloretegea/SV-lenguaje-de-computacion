# Extensión experimental: perfiles, recursos y captura

**Incremento SVG · RETP-238:** [contrato y banco previo](PRECOMPROMISO_SVG_CONSUMO.md), nueve casos integrados y un lector aislado, con mutantes de posición y consumo. Preparado y compilado; aún sin ejecución del banco. Se conservan la evidencia de fronteras y los antecedentes siguientes.

**Continuidad 14/09/2026 · RETP-237:** [fronteras conjuntas de descriptor y recibo](RESULTADOS_FRONTERAS_SALIDA.md), doce variantes EN/ES conformes en cada uno de dos perfiles Rust; 24 invocaciones, cero discrepancias y cuatro sensibilidades detectadas por campaña. [Banco previo](BANCO_FRONTERAS_SALIDA.md) conservado; [relevo de representación/consumo](RELEVO_REPRESENTACION_CONSUMO.md) preparado. Las secciones siguientes conservan el informe histórico RETP-221 y sus pendientes a aquel corte; las fronteras reciben ahora el alcance acotado de RETP-237.

Estado: **extensión ensayada en Rust/Cargo 1.98.0; RETP-2026-221**. Continuación de RETP-220; no cierre global del Bis.

## Resultado y hallazgo

| Comprobación | Campaña inicial | Campaña R01 |
|---|---|---|
| 26 casos integrados | 26 conformes: 9 entregas y 17 rechazos | 26 conformes: 9 entregas y 17 rechazos |
| Paridad de fuentes integradas | 2 comparaciones conformes | 2 comparaciones conformes |
| Pares con operación de evaluación | 3 impedidos por defecto del fixture | 3 variantes nuevas conformes: 2 igualdades y 1 desigualdad |
| Salida del binario | 1; campaña no conforme | 0; campaña conforme |

El compilador rechaza el nombre `Celda` de los fixtures C12 porque pertenece al vocabulario protegido. Ese resultado no indica desigualdad de dos IR: ninguna llegó a producirse. La [adenda R01](ADENDA_R01_PREVIA.json) fija tres pares nuevos con `CeldaPrueba`, manteniendo datos, dimensiones, orden, operación y expectativas. Los archivos originales y el fracaso observado permanecen conservados. El hallazgo se traslada al retorno de documentación/errores. El núcleo y el código de admisión no se modificaron.

Se ejecutaron 52 invocaciones integradas en dos campañas, correspondientes a 26 variantes únicas. No se suman a las 202 filas originales como si fueran su ejecución. Cada campaña incluye además cinco pares de compilaciones; la primera tiene tres comparaciones impedidas, la segunda contiene sus tres variantes corregidas. Los dos pares integrados conservan su identidad.

[RESULTADOS.json](RESULTADOS.json) resume resultados por caso; [EVIDENCIA_CAMPANA_01.tar.gz](EVIDENCIA_CAMPANA_01.tar.gz) y [EVIDENCIA_CAMPANA_R01.tar.gz](EVIDENCIA_CAMPANA_R01.tar.gz) conservan órdenes, diagnósticos, capturas literales, estados, trazas y mediciones. Sus archivos de huellas permiten comprobar los contenidos. La reproducción indicada ejecuta R01; `soporte/reproducir-campana-01.py` conserva la reproducción inicial fallida. Precompromisos históricos no son manifiestos del expediente final: README y reproductor se amplían; los bancos originales permanecen intactos. `MANIFIESTO.json` identifica el expediente final.

## Alcance fijado antes de ejecutar

26 casos nuevos: 8 de selección/perfil ES/EN, 12 de recursos y 6 de captura. Cinco comparaciones adicionales de IR canónica: dos sobre fuentes integradas y tres sobre fixtures C12 con una operación `evaluate`/`evaluar`. Se exige igualdad de todos los objetos y operaciones en cuatro pares y desigualdad al cambiar un dato textual en el quinto. No se traduce identificadores, datos, comentarios ni nombres de archivo. La procedencia literal (archivo y SHA-256) se comprueba separadamente y debe conservarse.

Los esperados están en [BANCO_PREVIO.json](BANCO_PREVIO.json), [PARIDAD_PREVIA.json](PARIDAD_PREVIA.json) y los `oracle.json` materializados. Todos mantienen `PENDIENTE`/null como instantánea previa; la evidencia se conserva en archivos separados. El anclaje matemático positivo es el vector y descriptor literal comprometidos en I0205, no una salida generada por el sujeto bajo prueba. La comparación canónica usa Rust `PartialEq` sobre todos los objetos y operaciones, con control de no vacuidad para los fixtures que incluyen evaluación.

Se ensayan límites exactos y excesos de recepción: metadatos, fuente, estado, soporte y total agregado; y recibo de salida 2048/2049 bytes. Los contadores observan bytes realmente devueltos por cada canal, incluido el byte detector de exceso. R01 puede rechazar al recibir (traza vacía) o al acotar el recibo antes de entregar (12 guardas superadas): cada traza se fija expresamente. Se alteran consumidor, canal, transformación, operación y bytes efectivamente recibidos, manteniendo la solicitud admitida.

## Custodia y reproducibilidad

El crate `sv_bis_extension` conduce el experimento y observa su evidencia. [ADMISION_VERIFICADA.tar.gz](ADMISION_VERIFICADA.tar.gz) conserva exactamente el crate de RETP-220, y `sv_core-verificado.tar.gz` conserva su núcleo. No se modifica ninguno. Cada caso contiene su propio registro y contexto de custodia, derivados explícitamente del banco anterior; la solicitud no elige esas autoridades. Los hashes y longitudes de fuentes con espacios o comentario añadido se actualizan en todos sus vínculos antes del ensayo. Variar el cuerpo de soporte genera una nueva referencia exacta por hash, sin ampliar sus tamaños admitidos.

Python prepara, archiva y lanza procesos; no decide admisión ni equivalencia SVP. No hay servicios externos en la ejecución. Herramientas previstas: Cargo/rustc 1.98.0, compilación `--locked --offline`, binario Linux; medición del PID mediante wait4 y reloj monotónico. Código completo en `proyecto/` y administración en `soporte/`. Núcleo y admisión se verifican por SHA-256 antes de compilar.

```sh
. /opt/sv-rust-1.98.0/env.sh
python soporte/reproducir.py /ruta/a/directorio-nuevo
```

## Límites y continuidad

Ejemplos y evidencia acotada de laboratorio. No acreditan garantías generales, consumo visual de IA, bus/host productivo, autoridad externa autenticada, constitución de dominio ni suficiencia total C12. La paridad canónica separa procedencia; no exige igualdad de recibos con perfiles/hashes diferentes. No se ejecutan aquí las 202 filas originales C02–C12 como banco propio ni se alteran sus recuentos. Quedan pendientes límites geométricos/salida agregada, agotamiento de memoria, presupuestos de tiempo/RAM constituidos y el resto del workflow. Bis → catálogo y cierre de fase → análisis e instalación de GUI.

[Fuentes rectoras y corte](FUENTES.json). Los tres rectores de AGENTS.md y workflow V2 mantienen su identidad. Esta extensión no constituye nuevas reglas del lenguaje.

## Costes observados y próximo paso

Campaña inicial: 11904 KiB de RSS máximo y 104090136 ns externos. Campaña R01: 11904 KiB y 113452806 ns externos. Son mediciones del binario completo con conductor, receptor y observador; no cotas del núcleo ni garantías temporales.

Siguiente paso: completar fronteras del descriptor y de salida agregada; preparar después el relevo hacia representación/consumo dentro del workflow. La GUI continúa diferida.

Precompromiso inicial: laboratorio `71d44d12183f51a4fd2b39d3f0c2925c3f4cce0d`, público `fc9cfdd220841582e7ad454aef236dd3e7cf737c`. Adenda R01 previa a repetición: laboratorio `5ff25e3300a1f432f00dda11fa95a73c511778b6`, público `ddcbced9c51bc2a557e8be71692611b1ae96f3e3`.
