# Sucesora experimental de calendario y contabilidad para A

**RETP-2026-134 · 11/09/2026. Candidata: IE004-ES-P2/3-COSTE/1.** Autoriza este trabajo Juan Antonio Lloret Egea: «Luz verde para: evaluar los significados antes de aplicar permisos». Estado de esta fijación: código compilable; controles todavía no ejecutados. La autorización incluye el seguimiento del coste de texto, perfiles lingüísticos, memo y buffers.

## 1. Herencia y cambio explícito

Se heredan léxico, G01–G25 y significado de §§1–5 de IE004-ES-P2/2, SHA-256 `10c29934f2f8f6bfbb40546f7cf0056d4b53d720c432cacfb9d6b70abefe3d5f`. El documento /2 permanece intacto. Esta sucesora sustituye para el experimento el calendario de barridos de §6.3 y precisa una contabilidad nueva del trabajo real de la realización. Los números de sus unidades no son comparables directamente con los intentos de /2 ni con MEMO/1. No se cambia un cupo después de observar resultados.

El identificador completo de interacción es `IE004-ES-P2/3-COSTE/1`; cada referencia Gxx debe incluirlo y la identidad de fuente. El receptor de esta prueba sólo habilita ese perfil, K-IE004/1 y P-IE004/1. /2 o SVP-EN no se reinterpretan como esta candidata por defecto. El perfil fuente del DSL, el idioma de interfaz y el perfil de interacción conservan las responsabilidades distintas de RETP-121. No se incorpora aquí interpretación de consultas inglesas ni traducción de datos.

Los commits de entrada son Lenguaje `52e1215b5416aff0c451694b1599c39c21424877` y laboratorio `992f68870e8b2a59e095330ceec9d8876feb2e08`. AGENTS y rectoras de Pilares, perfiles/ensamblaje, transición §§1–30 y arquitectura fueron leídas íntegramente en esta continuidad y sus blobs permanecen iguales en el corte. Se ha leído íntegra el acta de encaje lingüístico RETP-121, /2 y el siguiente objeto de RETP-133. Se conservan las fronteras de dominio, núcleo, agente y tecnología; no se modifica DSL/IR ni el catálogo de errores productivo.

## 2. Propiedad del texto y anchuras

La entrada es una vista UTF-8 válida de hasta 8.192 bytes mantenida inmutable por su propietario mientras se analiza. La realización conserva el original y sus offsets; crea un único buffer normalizado y una tabla de hasta 128 tokens. Cada conversión de mayúscula declarada tiene la misma anchura UTF-8 que su destino; el código lo comprueba. No borra tildes, normaliza Unicode general, divide palabras por semejanza ni corrige espacios desconocidos. Los huecos entre intervalos se reconstruyen desde el original y sólo son separadores admitidos.

Los cinco campos semánticos se codifican como conjuntos de identificadores tipados del repertorio cerrado. No almacenan una copia del nombre del parámetro, del sinónimo o de la frase en cada estado. Las longitudes y offsets de texto son u64 con conversión comprobada a usize. Los enlaces del bosque son identificadores u32 comprobados; no son direcciones que deban sobrevivir al crecimiento de un Vec.

Los conjuntos por campo usan u8 porque el repertorio actual tiene, respectivamente, 3 operaciones reconocidas, 2 objetos, 5 formas paramétricas (incluidas las dos exclusiones de hemoglobina), 2 momentos y 6 campos (incluido el excluido intervalo). El contexto admite sólo los cinco campos cerrados de IE-004, cada uno nulo o un valor habilitado. Estas anchuras son una representación versionada de este puesto; no fijan una capacidad universal para un dominio o idioma futuro. Ampliarlas exige verificar representación, cupos y compatibilidad antes de admitir la versión.

El operando de escritura es texto ASCII de 1–10 dígitos, nunca un valor numérico del registro. Su tipo copia esos diez bytes como máximo y conserva la evidencia de su token en el bosque. Una interpretación tiene hasta dos operandos positivos y dos patrones negativos: G02 contiene como máximo dos consultas y una negación; G18 sólo crea dos patrones en «no cambie nada». Si esas cotas derivadas de la gramática se contradicen, el código falla por constitución en lugar de crecer libremente. Los arrays fijos corresponden a estas cotas pequeñas; no se crea una arena grande en la pila.

Rust admite UTF-8 y su sintaxis de programación no selecciona el idioma de una petición. `Vec::with_capacity(n)` reserva capacidad inicial y no limita la longitud futura; `push` puede crecer cuando ésta se agota. Esta realización comprueba el máximo lógico y la capacidad antes de cada inserción. [Rust: texto UTF-8](https://doc.rust-lang.org/book/ch08-02-strings.html), [Rust: capacidad de Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity). Consulta 11/09/2026, alcance: almacenamiento UTF-8 y garantías de capacidad; no se deduce un coeficiente universal de expansión para el español. La página vigente de std mostraba 1.98.1; la ejecución se fija en Rust 1.98.0.

## 3. Recorrido y representación

El compilador local comprueba el DAG de referencias de la gramática y el mínimo positivo de los cuerpos repetidos. Usa arrays fijos de hasta 512 expresiones. La gramática es inmutable, se prepara antes de las solicitudes y su tamaño fijo se publica por separado.

El recorrido sintáctico por dependencias conserva todas las familias y extremos de cada expresión/posición. La tabla `(expresión, inicio)` tiene listas de extremos presentes. Se sustituye la anterior tabla triangular densa por esta representación porque inicializar y reservar millones de posiciones vacías competiría con el presupuesto conjunto que ahora incluye semántica. Se cobran también búsquedas fallidas, duplicadas e inicialización. No se supone gratis la representación dispersa ni se afirma que sea universalmente más rápida.

El significado se evalúa sobre todas las familias alcanzables de la raíz completa. Se comprueban las marcas de cierre de cada nodo. Una unión G10 reúne significados; una composición conjuntiva forma productos conservando sus conflictos. Negar crea patrones parciales conjuntivos, sin convertirlos en prohibiciones independientes. Las demandas explícitas usan el contexto constituido, permanecen pendientes si falta, y no son anuladas por otra referencia explícita. Sólo después de reunir las cláusulas positivas se completa lo admisible desde contexto.

La igualdad de significado compara estructuralmente restricciones, operandos, exclusiones, demandas y marcas, independientemente de los offsets de evidencia. Los significados finales se ordenan lexicográficamente por esos tipos; la evidencia de cada derivación permanece en el bosque y en las asociaciones nodo/estado. El orden de un modelo, de una tabla hash o de permisos no interviene. Si quedan dos significados, el resultado es PETICION_AMBIGUA. Sólo un significado completo sin impedimentos llega a la función de política. La decisión publica el número de llamadas a esa función.

G10 y el primer recorte negativo de G02 reciben anotaciones expresas en el compilador de gramática: la primera cambia composición semántica a unión, y la segunda transforma sólo el primer recorte en exclusión. Esas anotaciones no alteran qué secuencias derivan. No hay una tabla de preguntas completas o de identificadores de prueba dentro del algoritmo de resolución.

## 4. Recursos de A

Se conservan como techos candidatos: 8.192 bytes, 128 tokens, 16.384 estados semánticos almacenados en total y 32 MiB de capacidades de buffers agregadas de A. La cuenta es única para normalización, tokens, memo, bosque, asociaciones semánticas y significados finales. Se exige que sintaxis y semántica coexistan dentro de la misma arena. No se conceden 32 MiB a cada fase ni 16.384 estados a cada celda de memo.

El máximo de trabajo de esta candidata es 1.000.000 de unidades nuevas, debitadas antes de la acción. Categorías en el orden de la salida:

| Índice | Cargo |
| --- | --- |
| 0 | Bytes examinados/normalizados y cotejos de terminal; incluye barridos repetidos del léxico |
| 1 | Consulta al memo sintáctico, también cuando ya está resuelto |
| 2 | Nodo examinado al buscar un extremo, coincida o no |
| 3 | Combinación sintáctica de hijos examinada |
| 4 | Familia comparada al buscar duplicados |
| 5 | Pasos de evaluación y transformación semántica, bytes de terminal/operando y sizeof(Significado) por composición/compleción |
| 6 | sizeof(Significado) por comparación de igualdad u orden semántico |
| 7 | Elemento de buffer inicializado o insertado |
| 8 | Solicitud al asignador |
| 9 | sizeof(Significado) por examen de causas y cinco campos antes de la llamada a política |

Son cargos algorítmicos declarados, no instrucciones de CPU, bytes físicamente copiados ni combustible WASM. La admisión de bytes UTF-8/versiones/contexto está acotada por 8.192 bytes y campos cerrados antes de A; la preparación fija de gramática, pila, runtime, original custodiado y exportación de evidencia se registran fuera de las capacidades de buffers. La normalización y sus tokens sí pagan A. El cupo de estados cuenta todos los significados parciales almacenados y los finales, sin borrar conflictos para caber. Un fallo de cierre descarta toda salida semántica parcial.

Cada reserva usa suma/multiplicación comprobadas, conversión comprobada y asignación fallible. Se comprueba también la coexistencia de la capacidad vieja y la nueva solicitada; se registra la capacidad real devuelta. No se confía en overflow-checks de debug. Se cuentan reservas y crecimientos por ocho clases: normalización, tokens, tabla de memo, nodos, familias, índice semántico, estados semánticos, raíces finales. Los contadores de solicitudes no prueban que el asignador haya cambiado la dirección o copiado datos. Un asignador puede devolver más capacidad que la solicitada; la comprobación posterior y el fallo técnico no prueban un límite físico de RSS. Esa contención pertenece a P4.

El máximo de profundidad explícito es 512 llamadas de evaluación sintáctica y semántica, con rechazo antes de superar la cota. La prueba publica profundidades y tamaños de tipos. No presenta esa cota como medición de bytes de pila ni como ausencia universal de stack overflow.

## 5. Contraste acotado y decisión

Se fijan 61 controles semánticos públicos: los 16 de /2 conservando sus esperados y 45 discriminantes para S01–S08. Además se repiten las 72 regresiones sintácticas públicas porque cambia el índice de memo. Se ejercen primitivas de overflow, UTF-8, conversión, cuota antes de push, repetición anulable y retención de dos/cuatro derivaciones. Los casos con cupos reducidos son inyecciones explícitas de fallo, no ampliaciones o ajustes del cupo ordinario.

Una ejecución por nativo/WASI y debug/release, plazos de 120 s por compilación y 30 s por proceso de prueba. Node sólo compila, captura, comprime y compara evidencia. Las afirmaciones y decisiones de los casos se ejercen en Rust compilado. No se ejecuta Python. Se fijan relaciones entre paráfrasis y entre permisos/vigencia antes de ejecutar. Un éxito no acredita comprensión de español abierto ni de otro idioma.

Si hay un fallo, conservar primero el resultado y corregir su causa una vez; verificar después la matriz afectada. Si persiste el impedimento, cerrar la realización como insuficiente. No abrir más consultas a Grok para eludir un defecto del analizador. Los tiempos de una ejecución con exportación extensa de trazas no son un benchmark del modelo, p95 ni latencia clínica.

## 6. Puertas que conserva la reserva

La reserva y COMPROMISO_P3.json conservan identidad /2 y su SHA-256 `d37246a019332327f54dd45c641944d90e3ae440163d76d0607c5235df63f9f5`. El presente experimento no declara compatibilidad de la reserva con esta sucesora, ni la abre. Se requiere cotejar la revisión normativa y el formato de captura antes de congelar y usarla. La custodia separada sigue acreditada por declaración humana.

Esta pieza implementa A sobre entradas públicas constituidas; no implementa aún la recepción A/V separada, el verificador V, la contención material P4 ni la medición de aportación del agente P5. El resultado del dato sigue siendo literal de la base artificial con su fuente y alcance. Un error técnico, un perfil no habilitado o una solicitud no representada no se convierten en U. La continuación hacia el catálogo/localización y fila 9/Ciberseguridad conserva su secuencia.
