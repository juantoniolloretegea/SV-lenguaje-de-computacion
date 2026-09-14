# Resultado del SVG material y su consumo instrumentado

**S22 · RETP-2026-239 · 14/09/2026 · W-S26-02. Conforme en el alcance experimental fijado.**

Las dos campañas normales cumplen **20/20 oráculos**. Los cuatro ensayos de mutantes producen las discrepancias funcionales previstas. Todo el candidato, conductor, captor, observador y preparador de pruebas está escrito y ejecutado en Rust; no se utilizó Python.

## Resultado material

Se produjo un SVG de 792 bytes desde un descriptor admitido por el montaje existente. El consumidor abrió el archivo realmente producido, interpretó sus dieciséis pares de coordenadas y calculó las diferencias cíclicas entre vértices. El SVG, los vértices y esas diferencias coinciden con los oráculos independientes publicados antes del ensayo.

[Muestra efectivamente producida](MUESTRA_SVG_PRODUCIDA.svg), SHA-256 `e5a9f9bc6df9b4f59a2a0cb908046239fda11ebbc247577b8e8c32b268a8c578`. Las fuentes EN/ES y las revisiones admitidas A/r1 y A/r2 produjeron los mismos bytes gráficos, conservando sus identidades y procedencias distintas en la envoltura.

| Casos, en ambos perfiles | Resultado observado |
| --- | --- |
| RC01–RC03: EN, ES y A/r2 autorizado | Tres consumos completos por perfil; 792 bytes leídos, SVG y operación concordantes. |
| RC04: coordenada sustituida | C04_BYTES; 792 bytes observados y cero operaciones completadas. |
| RC05–RC06: revisión o invocación ajena | C02_CONTEXTO; cero bytes leídos y cero consumos. |
| RC07: canal ausente | C01_AUSENCIA; no se acredita consumo. |
| RC08: 4097 bytes | C03_CUOTA; se observa el byte detector y se detiene la operación. |
| RC09: fallo E/S tras 64 bytes | C03_IO; captura parcial conservada, cero consumos. |
| RC10: script añadido al SVG | C05_PERFIL en el lector aislado; el script no se ejecuta. |

## Falsación de productor y consumidor

Los dos mutantes compilaron y ejecutaron RC01 en debug y optimizado. Cada banco mutado terminó con retorno 1 por discrepancia funcional, no por error de compilación o pánico.

| Mutante | Captura y metadatos | Resultado utilizado | Detección por perfil |
| --- | --- | --- | --- |
| `svg_permutado` | SVG con dos posiciones permutadas; su propia envoltura y hash son coherentes con esos bytes. | Vértices y diferencias no corresponden al estado original. | Siete discrepancias del observador; consumo completa. |
| `consumo_falso_svg` | SVG producido y capturado correctos; hash declarado correcto. | El consumidor calcula con otro SVG, manteniendo los metadatos de la captura buena. | Dos discrepancias: vértices utilizados y resultado de la operación. |

El segundo resultado confirma en este montaje que una captura y un hash correctos no bastan para acreditar qué dependencia utilizó una operación. La evidencia conserva precisamente ese contraste. No se extrapola a percepción visual de modelos o a un anfitrión arbitrariamente alterado.

## Custodia y ejecución

[Contrato](CONTRATO_SVG_CONSUMO.md) y [banco previo](PRECOMPROMISO_SVG_CONSUMO.md). Precompromisos: Lenguaje `c7d06a815979cecbcd9909ac053447667692efbd`; laboratorio `8cc94133a59786dd18cf31aef735485c35d150dc`. R08 cotejó ambos árboles completos a las 16:14:04 UTC, antes de la primera campaña a las 16:15:54 UTC. Fuentes rectoras, núcleo y admisor permanecen íntegros.

Rust/cargo 1.98.0; reconstrucción del núcleo mediante Cargo `--locked --offline`; compilación explícita del auxiliar y del banco. Perfiles debug y optimizado con `opt-level=3`, `debug-assertions=no` y `overflow-checks=no`. Las guardas aritméticas del candidato siguen activas. Sin red, reintentos ni concurrencia de campañas.

Se ejecutaron seis binarios: dos normales y cuatro mutados. Producen 24 observaciones: veinte normales y cuatro mutadas, con 22 entradas al montaje de admisión y dos controles del lector aislado. No son 24 escenarios globales Bis. Las 145 huellas de entrada se releen y coinciden; los oráculos y el estado original se conservan. No hubo discrepancias imprevistas ni correcciones del banco posteriores al precompromiso.

[Resultados estructurados](RESULTADOS_SVG_CONSUMO.json) · [evidencia, capturas y comandos](EVIDENCIA_SVG_CONSUMO.tar.gz) · [seis binarios ejecutados](BINARIOS_SVG_CONSUMO.tar.xz) · [huellas de los archivos](ARCHIVOS_SVG_SHA256.txt). La evidencia contiene stdout/stderr y códigos de retorno, incluidas advertencias de compilación y los fallos deliberados. El registro de comandos identifica los procesos lanzados; no constituye un monitor general del sistema operativo.

## Alcance y siguiente paso

Se acredita SVG material y consumo de coordenadas por esta operación Rust local. La proyección gráfica declara su aproximación: error menor que 3/1000000 por coordenada canónica para los radios ensayados, comprobado con aritmética entera. El estado ternario, el descriptor exacto y la cardinalidad no se redondean ni se reparan.

No se acredita rasterización, legibilidad, percepción por IA, autorización profesional o resistencia al host. La prueba RC10 corresponde únicamente al lector restringido. Las 202 filas originales C02–C12 y los escenarios globales Bis mantienen sus recuentos; S26/F01/F02 conserva su pendiente de interfaz material.

El próximo incremento debe constituir la frontera **SVG → artefacto rasterizado → captor**, con formato, presupuesto, pérdida, transformación y oráculos de píxeles explícitos. Deberá discriminar alteración de posiciones, recorte, leyenda/orientación y sustitución del artefacto realmente entregado. No corresponde repetir este banco por cambios administrativos. Bis y S26 siguen abiertos; catálogo y cierre de fase preceden a S24/C#/.NET.
