# Diagnósticos de declaración y procedencia ES/EN — RETP-114

**09/09/2026. Estado previo: casos comprometidos; realización y ejecución pendientes.**

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
