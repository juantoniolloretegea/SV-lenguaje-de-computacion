# Acta técnica de reparación de oráculos: identidad, pares JSON y rechazo controlado

**Fecha:** 6 de septiembre de 2026  
**Registro:** RETP-2026-078  
**Corte de entrada:** `91dc5a3c3b2298ef3fd1b2eefe607f379643e076`, integración de PR #61 / N0-01  
**Expediente:** [historial de la candidata](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commits/reparacion-oraculos-20260906), con su PR y controles asociados  
**Frente:** fila 2 de la secuencia rectora  
**Naturaleza:** corrección de comprobadores y conservación de evidencia; sin cambio semántico  
**Estado:** reparación delimitada; promoción condicionada a los controles del candidato exacto

## 1. Objeto y fuentes

Los comprobadores anteriores leían JSON como mapas, ordenaban sus claves y
reserializaban antes de comparar. Esa transformación podía ocultar miembros
homónimos y variaciones del orden. La lectura de procesos en modo texto también
podía normalizar saltos de línea antes de calcular huellas. En las comparaciones
negativas de R0-7 y WASI bastaba un retorno no nulo; el navegador sólo comprobaba
la marca de error.

Se corrige el observador para que una afirmación de identidad, paridad o rechazo
sea falsable en el alcance que declara. No se modifica automáticamente ningún
resultado esperado a partir de la salida actual del compilador.

Se han leído íntegramente los [Pilares, RETP-073](./PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md),
el [acta de perfiles, RETP-075](./ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md)
y el [acta de transición, incluida la secuencia y recepción §§12–18](../dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md).
Se reciben asimismo la [radiografía N0, §§5–8](../arquitectura/N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md),
el [acta N0-01, §8](../arquitectura/ACTA_TECNICA_N0_01_UNICIDAD_DE_CODOMAIN_2026_09_04.md#recepcion-20260906),
RETP-074/076/077 y el [registro de deuda viva](./REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md).

## 2. Contrato efectivo de comparación

| Observable | Comprobación vigente | Límite expreso |
|---|---|---|
| Python frente a los doce JSON esperados; Rust frente a Python | Pares JSON ordenados, rechazo de miembros repetidos a cualquier profundidad, arrays diferenciados de objetos, booleanos diferenciados de números y naturales sin conversión a coma flotante | Se permite distinta disposición de espacios y escapes JSON equivalentes. No se afirma igualdad de bytes entre estos emisores |
| Dos ejecuciones consecutivas de cada emisor en R0-7 | Igualdad directa de los bytes de stdout, con ambos procesos admitidos | Determinismo observado sobre los doce positivos; no universalidad |
| Rust nativo frente a WASI | Igualdad directa de stdout en positivos y de stderr en negativos controlados | Mismo programa y realizaciones identificadas; no cambio del serializador ni del núcleo |
| Rust nativo frente a WASM de navegador | Igualdad de los bytes del documento o diagnóstico con la respuesta del módulo | Se retira exclusivamente el LF que añade la CLI al documento y el envoltorio de diagnóstico de esa CLI; se elimina el recorte general `trimEnd` |
| Captura y huellas de procesos | stdout y stderr capturados como bytes, sin conversión de saltos de línea | El informe WASI v2 conserva ambos flujos en Base64, sus huellas, comandos y retornos |
| Corpus negativo | Coincidencia de los archivos con los catálogos de expectativas de Python y Rust | Una entrada sin expectativa, un catálogo desfasado o un corpus vacío impiden conformidad |

La comparación estructural conserva también la escritura de los números:
`9007199254740993` no se convierte en `9007199254740992`. No introduce una
normalización numérica alternativa. El módulo compartido es
[`tests/oracle_support.py`](../../tests/oracle_support.py); no forma parte del
ejecutable ni produce IR.

Nueve de los doce archivos esperados tienen espacios o saltos de presentación
distintos de los emitidos por Python en el corte de entrada. Se conservan sus
bytes y se compara su estructura ordenada. Su identidad histórica se comprueba
por Git; no se regeneran para hacer coincidir una afirmación literal improcedente.

## 3. Rechazo esperado y límite diagnóstico

Un negativo de proceso debe devolver **1**, no emitir IR y satisfacer su
expectativa diagnóstica. Retorno 2, pánico 101, terminación por señal, agotamiento
del límite del ejecutor, fallo de carga o error interno no acreditan rechazo
semántico. La espera máxima de 60 segundos pertenece al ejecutor de pruebas;
no añade tiempo como primitiva del Lenguaje.

Python conserva los códigos de `EXPECTED_INVALID_CODES`. Se comprueba el código
de la cabecera diagnóstica, sin confundirlo con identificadores como `E999` que
puedan aparecer en el detalle. Rust exige el envoltorio controlado de
`CompileError` y una identidad textual por caso, contrastada con
[`frontend.rs`](../../rust/sv_core/src/frontend.rs),
[`wellformed.rs`](../../rust/sv_core/src/wellformed.rs) y sus juicios invocados.

La tabla textual Rust caracteriza la realización; **no constituye concordancia
completa con los códigos Python ni un diagnóstico estructurado nuevo**. Por
ejemplo, `admissibility_table_output_fuera_codominio.svp` produce E011 en Python,
pero Rust rechaza antes por el cierre interno legado. El caso no acredita en
Rust la condición semántica que anuncia su nombre. La reparación conserva y hace
visible esa diferencia bajo DFL-001; no altera el caso para ocultarla.

WASI debe conservar literalmente el diagnóstico de la CLI nativa. En navegador
se compara el contenido de `CompileError` con el obtenido por la CLI nativa ya
comprobada. Una marca de error genérica, una excepción o una trampa de WASM no
sustituyen esa comprobación. La ejecución WASI usa `node --no-warnings` para
suprimir avisos del entorno experimental; los errores del anfitrión y del
programa conservan stderr y su retorno de fallo.

## 4. Controles de sensibilidad y deuda conservada

[`test_oracle_support.py`](../../tests/test_oracle_support.py) contiene **16
pruebas**, con controles conformes y contraejemplos para orden, multiplicidad,
tipos, precisión numérica, CRLF, codificación, JSON inválido, identidad literal,
cabecera diagnóstica, salida indebida y fallos de proceso.

[`run_oracle_sensitivity.py`](../../tests/run_oracle_sensitivity.py) reproduce una
base sintética y tres transformaciones explícitas. Conserva las entradas en
bytes, stdout, stderr, comandos, retornos, huellas de entrada y binario, identidad
del corte y huellas de los comprobadores. Sus resultados pertenecen a un banco de
detección separado de la conformidad:

| Caso | Resultado observado en el corte de entrada | Obligación pendiente |
|---|---|---|
| `control_valid` | Python y Rust admiten y coinciden estructuralmente; identidad de fuente correcta | Control válido del detector |
| `crlf` | Ambos admiten; la lectura Python cambia CRLF a LF antes de calcular la identidad | Preservación de bytes en la entrada Python, DFL-008 |
| `string_crlf` | Además de la identidad, cambia el contenido de una cadena | Preservación del literal; no reparación silenciosa, DFL-008 |
| `semantics_duplicate` | Ambos admiten; Python pierde un miembro y Rust emite miembros homónimos | Rechazo de la relación inválida en N0-02 y estabilidad de proyección en N0-03 |

El éxito del banco significa **un control conforme y tres divergencias
detectadas; cero divergencias reparadas**. No se agregan estos casos a los 80
programas conformes ni se certifica la admisibilidad de las entradas divergentes.
Una corrección posterior requiere actualizar explícitamente este expediente;
un cambio de comportamiento o un fallo interno no se aceptan automáticamente
como nueva salida esperada.

## 5. Aplicación de perfiles y evidencia reutilizada

PT02 recibe la conservación de bytes y estructura; PT04, la separación entre
rechazo y fallo técnico; PT13, la paridad pertinente entre destinos. PT01/PT14
conservan las identidades de fuentes, binarios, corpus y entorno. No se requiere
contenido IMM/CYB para reparar estos observadores.

La Gramática 0.2, IR 0.3 y serializador 0.1.0 conservan su realización. El corpus
común sigue en su superficie EN; los controles existentes del núcleo y las seis
sondas ES/EN conservan su alcance. No se atribuye a Python ensamblaje bilingüe ni
se amplía la cobertura de los contratos de dominio o agente.

Se recibe del [registro experimental 016](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/016-VIGILANCIA_Y_COSTE_2026_09_06.md)
la distinción entre validez e identidad. Los ensayos .NET/FFI/WASM del registro
018 mantienen sus 79 programas y sus límites; no se reejecutan ni se les atribuye
retrospectivamente este comprobador. No se promueve un perfil tecnológico ni se
selecciona una plataforma.

El ejecutor SEC-0 se reconcilia con la retirada ya documentada por DFL-007 del
caso `deep_nested_query_valid.svp`. Utiliza el positivo vigente
`query_context_all_variants.svp`; el antecedente de Gramática 0.1 sigue archivado.

## 6. Verificación, promoción y relevo

La comprobación local acredita conformidad **80/80**, R0-7 **12 positivos y 68
negativos**, CLI **3/3**, SEC-0 **3/3**, las **16** pruebas del observador y la
detección **3/3** con su control válido. El módulo WASM compilado con Rust 1.98.0
ejecuta el comprobador JavaScript bajo Node con **12/68/6**; esto no sustituye la
prueba en navegador real. Las realizaciones `src/` y `rust/`, la gramática, la IR
y los doce JSON esperados conservan su identidad respecto del corte de entrada.

Antes de integrar se exigen **Conformidad SVP, R0 Rust, R0-8 Baseline nativa y R0
WASM paridad de tres vías** sobre la candidata exacta. El último incluye WASI y
navegador real. Los filtros de los flujos reciben las dependencias del observador
para que futuras modificaciones no eludan esas comprobaciones. Las cadenas de
herramientas conservan su régimen anterior; su unificación sigue siendo una
deuda de infraestructura separada.

La promoción es efectiva cuando el expediente de la candidata identificado en esta acta conste
integrado con esos controles correctos. La continuación es **fila 3, K1 desde
N0-02**, conservando N0-03, DFL-001 y DFL-008 en sus sedes. Detectar una carencia
no la cierra. No se acredita cierre nuclear, ejecución algebraica adicional ni
R2/R3/R4.

## 7. Sucesión N0-02 del banco de sensibilidad · 06/09/2026

El [acta N0-02, RETP-079](../arquitectura/ACTA_TECNICA_N0_02_TOTALIDAD_Y_UNICIDAD_DE_OUTPUT_SEMANTICS_2026_09_06.md) modifica explícitamente la expectativa de `semantics_duplicate`: después de resolver la relación constituida se exige E115 en ambos emisores. Conserva el control y las dos sondas CRLF, y añade la misma semántica duplicada sin `CellSpec` como testigo de la deuda N0-03. El esquema del informe pasa a `sv-oracle-sensitivity-v2`; su resultado es un control, un rechazo relacional y tres divergencias abiertas detectadas. Los resultados de §§4–6 permanecen ligados a su corte histórico. N0-02 no cierra DFL-008 ni la protección global de la proyección JSON.

## 8. Sucesión N0-03: recorrido JSON y segundo rechazo E115 · 06/09/2026

El [acta N0-03, RETP-080](../arquitectura/ACTA_TECNICA_N0_03_UNICIDAD_DE_MIEMBROS_Y_ESTABILIDAD_DE_PROYECCION_JSON_2026_09_06.md) añade el recorrido de lectura/escritura del valor JSON y tres pruebas del observador (19 en total). El banco de sensibilidad v3 conserva exactamente las cinco fuentes de v2: exige un control, dos rechazos E115 y dos divergencias CRLF aún abiertas. La sonda no enlazada se conserva como regresión de cierre; no se retira ni se cuenta como una divergencia pendiente. Los resultados de las secciones anteriores conservan su identidad histórica.
