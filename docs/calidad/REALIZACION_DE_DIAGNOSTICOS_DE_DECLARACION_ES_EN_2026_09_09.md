# Diagnósticos de declaración y procedencia ES/EN — RETP-114

**09/09/2026. Estado actual: incremento de declaración verificado en candidata; localización completa y entrega productiva pendientes.**

Se continúa el contrato diagnóstico ES/EN, RETP-109–113, sobre `ba4c63c6140015b14eb7afd9b1e32ce8307cc705`. Se han leído Pilares, perfiles, transición completa y norma del español. El incremento conserva admisión, orden de guardas, IR y salida histórica; incorpora causa y procedencia donde hoy se pierden en cadenas.

## Alcance comprometido

Se tiparán los tres dominios cerrados de Gramática 0.2, la declaración duplicada (objetos y resultados), E004 vacío/repetición, base celular inferior a tres y E115 repetición aislada/cobertura relacional. No se inventará un código para guardas que carecen de correspondencia constituida. Las otras causas permanecerán explícitamente sin diagnóstico estructurado en este incremento.

El analizador conservará intervalos completos de declaraciones por su posición en los vectores de objetos y operaciones, separados de la IR. El intervalo comprende desde la palabra inicial hasta el último símbolo de la declaración; no se presenta como ubicación exacta de un campo. Una colisión conservará los dos índices de declaración, sin deducir el origen sólo por el nombre. E115 relacional conservará celda, semántica y codominio referenciados. El ensamblaje conservará perfiles, índices de unidad y huellas originales, incluidos nombres de archivo iguales. Las explicaciones de un ensamblaje mixto se ordenan ES, EN; los contextos sólo atribuyen declaraciones implicadas.

El tipo de error de compilación añadirá una variante estructurada de validación. Las guardas pendientes conservarán `InvalidProgram(String)`. `Debug` seguirá produciendo la envoltura y cadena históricas para ambas variantes; ningún componente reconstruirá causa a partir de esa cadena. La incorporación es diagnóstica, no una puerta de admisión de programas o referentes profesionales.

## Casos previos

[Casos fijados](../../tests/diagnostic_validation/cases.rs) (ruta desde la raíz: `tests/diagnostic_validation/cases.rs`). Ocho pruebas agrupan positivos y contraejemplos sobre E004, E115, referencias adelantadas/cruzadas, colisiones objeto/operación, nombres de archivo repetidos, UTF-8, CRLF, tabulaciones, aislamiento de la unidad sana y ausencia explícita de cobertura. Las pruebas existentes conservan sus esperados de admisión, IR y rechazo histórico; sólo se adaptará su lectura de la envoltura donde se emita la nueva variante.

No se consideran cerrados DG01–DG14, la migración del resto de emisores, CLI/ABI ni playground. La producción visible y el relevo profesional R1 permanecen posteriores a la realización completa y verificada. Este incremento no autoriza refactorización, nuevas dependencias, traducciones profesionales ni cambios en dominios.

## Realización candidata

Casos previos registrados en `f15d411d40a943e3976822ad7e87061cd061a9ac`. La candidata añade las causas y el mapa paralelo descritos arriba. `CompileError::Diagnostic` expone `program_diagnostic()`; `legacy_program_message()` conserva acceso explícito al rechazo histórico. Las causas no migradas siguen devolviendo `InvalidProgram(String)`. Los oráculos históricos de cuatro familias leen ese acceso sin modificar sus fuentes ni sus esperados. Se conserva una señal explícita `provenance_complete` si faltase una correspondencia interna; una ausencia no concede admisión.

Verificación de compilación, pruebas y conformidad: pendiente de CI en este corte. No se acredita compilación local; los toolchains disponibles en este entorno no proporcionan una biblioteca estándar utilizable. El testigo de medición del laboratorio no se convierte en medida de rendimiento de esta candidata.

**Corrección de adaptación del banco:** el corte `0f9eafad6d6d10cbc06f16914e3029406b9d0afb` compila y supera las ocho pruebas nuevas; dos pruebas históricas de `json_projection.rs` fallan porque aún exigen la variante anterior aunque reciben el rechazo E115 y el texto esperado. Se adapta ese quinto lector a `legacy_program_message()`, conservando fuentes y aserciones. No se considera conforme aquel corte. La repetición se realizará sobre el nuevo commit.

## Resultado y custodia

Corte corregido `b7636296f2c75715691464df83a030d006a2f054`, árbol `99e31a0aa19a27ce93e0da47465e33b433d533bd`. La fusión de prueba `049563bc815a7a133e4fb83ccc2382da26851ee2` tiene ese mismo árbol. Se consultan directamente los seis flujos terminados con éxito: 34402104854 (conformidad), 34402104877 (base nativa), 34402104881 (paridad), 34402105006 (Rust), 34402105070 (construcción aislada) y 34402105209 (CYB).

La suite suma **384 pruebas Rust correctas, ocho nuevas**, tanto con Rust 1.98.0 como con 1.98.1; también pasa en la construcción aislada sin intérpretes y sin red. La conformidad conserva 14/14 fuentes válidas y 106/106 inválidas; las 43 mutaciones dirigidas son detectadas, sin supervivientes ni mutaciones inválidas. Se conserva la paridad histórica nativo/WASI/navegador. **Las ocho pruebas nuevas ejercitan las entradas Rust nativas; no prueban todavía un transporte localizado en WASI ni navegador.**

Los [registros íntegros y metadatos](evidencias/RETP-114/custodia.json) y sus [huellas](evidencias/RETP-114/huellas.json) conservan el primer fallo del banco y la corrección posterior. Los casos previos no se han reescrito tras ejecutar. La procedencia identifica el intervalo completo de la declaración; todavía no individualiza dentro de ese intervalo el campo o la referencia textual responsables. No se conserva el contenido íntegro de la fuente en el nuevo diagnóstico ni se incorporan los textos semánticos a sus explicaciones.

| Ubicación del inventario RETP-112 | Causa preservada | Prueba de la nueva estructura |
|---|---|---|
| LOC-057/058/059 | Valor ajeno al dominio cerrado de relación, patrón o régimen | `base_y_dominios_cerrados_se_localizan_sin_crear_codigos` |
| LOC-060/061 | Declaración repetida de objeto o resultado | `colision_conserva_ambas_declaraciones_aunque_coincida_el_archivo`; `colision_objeto_operacion_y_operacion_operacion_conserva_precedencia` |
| LOC-066 | Claves de semántica de salida repetidas, sin celda | `semantica_distingue_repetidas_ausentes_y_ajenas_sin_revelar_textos` y oráculos N0-03 conservados |
| LOC-069/070 | Codominio vacío o con valores repetidos; E004 | `codominio_causa_codigo_prosa_y_control_en_ambos_perfiles` |
| LOC-071 | Base celular inferior a tres | `base_y_dominios_cerrados_se_localizan_sin_crear_codigos` y controles geométricos anteriores |
| LOC-073 | Repetidas, ausentes y ajenas en la relación celda/semántica/codominio; E115 | `semantica_distingue_repetidas_ausentes_y_ajenas_sin_revelar_textos`; `referencias_cruzadas_conservan_contextos_por_unidad_y_orden` |

Son diez emisores instrumentados y siete variantes de causa, no diez códigos ni una localización completa. Los controles previos de dominios cerrados conservan sus positivos ES/EN. Los casos de unidad sana, huella y causa pendiente delimitan la atribución y la cobertura. Añadir una variante pública a `CompileError` obliga a los consumidores con un `match` exhaustivo a contemplarla: la compatibilidad de la salida histórica no equivale a conservar todas las coincidencias de patrones Rust.

## Continuidad

El siguiente incremento debe continuar por las referencias, tipos y restantes guardas del inventario, aprovechando el mapa paralelo sin alterar las obligaciones. Después corresponde el transporte estructurado y la presentación ES/EN en CLI, ABI y playground, con la verificación completa DG01–DG14 y la entrega productiva observada. Los perfiles profesionales IMM/CYB siguen independientes del perfil fuente; no se crean facultades, células ni referencias profesionales por localización. El enlace protegido R1 continúa después de esta aceptación. PR #89 en borrador, fila 9 abierta.
