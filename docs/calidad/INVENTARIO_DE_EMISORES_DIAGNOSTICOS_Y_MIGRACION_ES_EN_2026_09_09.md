# Inventario de emisores y preparación de la migración diagnóstica ES/EN

**RETP-112 · 09/09/2026 · Inventario estático revisado; realización pendiente.**

## 1. Corte y alcance

Base integrada: `73c738348a9493a459744c62b8ade382ff595af5`, árbol `4827e8757ed2776e3f4f35f206ea3cb7bdb6e01e`. La PR #88 quedó integrada por la decisión de alcance registrada en RETP-111. Este inventario es el primer producto posterior y no declara implementada la localización.

Se han consultado los Pilares, el acta de perfiles completa, la transición secuencial desde OP-IMM-001 con su adenda y relevos hasta §30, el acta del español, RETP-105–110 y el [contrato diagnóstico](CONTRATO_DIAGNOSTICO_ESTRUCTURADO_Y_LOCALIZACION_ES_EN_2026_09_09.md). Las piezas rectoras conservan su texto. El inventario desarrolla §6 del contrato y no convierte mensajes actuales en doctrina ni códigos nuevos.

Se recorren las tres entradas de compilación de la biblioteca, la CLI de una unidad, las tres entradas de compilación de la ABI WebAssembly y su presentación en `app-b2.js`. Se incluyen los comprobadores llamados por esa compilación y sus adaptaciones. La serialización de IR y su construcción se revisan como puntos de conservación o pérdida de contexto. Los archivos y rangos exactos están fijados en [fuentes.json](evidencias/RETP-112/fuentes.json).

No es un censo de todas las funciones públicas del núcleo: los diagnósticos de ejercicio R1, permisos, ejecución, LIG y observadores externos requieren sus inventarios al incorporar esas entradas. Tampoco se atribuye cobertura a errores del compilador Rust, a la biblioteca estándar, a expansiones de macros, a excepciones implícitas del navegador o a fallos del sistema operativo. Los ejemplos de prueba, incluido `assembly_probe`, no se hacen pasar por la CLI productiva.

## 2. Resultado reproducible y significado del recuento

[puntos.json](evidencias/RETP-112/puntos.json) conserva **224 ubicaciones**: 176 expresiones de emisión, 19 adaptaciones, 20 salidas y nueve aserciones internas. Cada entrada lleva ruta, líneas, intervalo exacto de bytes, expresión íntegra, huella y variante escrita cuando existe. Los intervalos se refieren al código del realizador inventariado, no a ubicaciones ya disponibles de la fuente SVP del usuario.

**224 no significa 224 causas, códigos ni casos alcanzables.** La misma infracción puede atravesar varias ubicaciones; una ubicación puede agrupar causas. `LOC-...` es una referencia local de esta versión del inventario. La presencia de `E...` en un literal se distingue de un método `diagnostic_code()` y de una correspondencia pendiente. Un campo nulo indica ausencia de esa evidencia en la expresión, no inexistencia de un código en toda la doctrina.

El [localizador](evidencias/RETP-112/inventariar.mjs) comprueba las huellas antes de emitir resultados. Oculta comentarios y literales, excluye patrones `Err(...) =>`, distingue las salidas marcadas como error y respeta los rangos declarados fuera de pruebas. Es una ayuda de lectura léxica, no un parser Rust/JavaScript completo ni un análisis de alcanzabilidad. No interpreta mensajes de ejecución para fabricar diagnósticos. Requiere Node como auxiliar documental; no añade dependencias al producto.

Desde la raíz del repositorio:

```sh
node docs/calidad/evidencias/RETP-112/inventariar.mjs > /tmp/inventario-retp-112.json
cmp docs/calidad/evidencias/RETP-112/puntos.json /tmp/inventario-retp-112.json
```

| Archivo bajo el corte indicado | Ubicaciones | Contenido revisado y tratamiento requerido |
|---|---:|---|
| `rust/sv_core/src/lib.rs` | 13 | Doce adaptaciones a `InvalidProgram(String)` y rechazo del ensamblaje con menos de dos unidades. Conservar fase y contexto antes de la conversión. |
| `rust/sv_core/src/frontend.rs` | 43 | Léxico, sintaxis, superficies, valores y cinco aserciones internas. Conservar el token original y su intervalo antes de canonicalizar. |
| `rust/sv_core/src/grammar_conformance.rs` | 3 | Dominios cerrados de relación, patrón y régimen del grafo. Retener sujeto, valor y dominio esperado. |
| `rust/sv_core/src/wellformed.rs` | 77 | Referencias, objetos y operaciones; códigos literales E004/E115; adaptaciones de errores tipados y tres aserciones internas. |
| `rust/sv_core/src/transition_data_wellformed.rs` | 13 | E406 mediante constante: referencias, pertenencia, sucesos y destinos repetidos, posiciones. Algunas guardas pueden quedar precedidas por la validación general. |
| `rust/sv_core/src/context_wellformed.rs` | 20 | Relación entre arquitecturas, marcos, trayectorias, agentes y consultas; referencias y tipos. Sin código explícito en estas expresiones. |
| `rust/sv_core/src/nat.rs` | 1 | `InvalidNat`; conservar el dato recibido en el llamador. No se presupone alcanzable desde el token numérico ya filtrado. |
| `rust/sv_core/src/admissibility.rs` | 3 | Tres variantes de `InvalidAdmissibilitySpec`, con E110 declarado por el tipo. |
| `rust/sv_core/src/frame.rs` | 11 | Once variantes de `FrameClosureViolation`, con E308 declarado por el tipo. |
| `rust/sv_core/src/resolution.rs` | 5 | Cinco variantes de `UnsafeUResolution`, con E305 declarado por el tipo. |
| `rust/sv_native/src/main.rs` | 6 | Uso, perfil, lectura, nombre y salida de compilación. Rechazo de compilación retorna 1; fallo técnico, 2. |
| `rust/sv_wasm/src/lib.rs` | 14 | Perfil, UTF-8 de fuentes/nombres y compilación de una o dos unidades. Hoy comparten señal de error y cargas textuales. |
| `docs/candidata-integracion/app-b2.js` | 14 | Lanzamientos explícitos por tamaños, lectura, carga, integridad y guardas de interfaz. Los lugares de captura/presentación se revisan aparte en §5. |
| `rust/sv_core/src/ir.rs` | 0 | La lectura de construcción y acceso confirma la falta de intervalos por objeto; cero coincidencias no acredita ausencia universal de fallos. |
| `rust/sv_core/src/equivalence.rs` | 1 | Aserción de escritura en `String`; no es un diagnóstico SVP ordinario. |

## 3. Causas, parámetros y procedencia

El registro íntegro permite ir a cada expresión y a los parámetros hoy interpolados. La siguiente clasificación revisada fija qué información debe rescatarse en el emisor. No asigna códigos por parecido de prosa.

| Familia y ubicaciones | Causas que no deben confundirse | Parámetros y procedencia que debe conservar la migración |
|---|---|---|
| Ensamblaje, LOC-009 | Menos de dos unidades | Cardinalidad recibida y unidades efectivas; no inventar un objeto responsable. |
| Frontal, LOC-014–055, excluidas aserciones | EOF, token inesperado, superficie no admitida, natural, estado de admisibilidad y Tri inválidos; campos opcionales repetidos/fuera de orden; palabra protegida en posición de identificador | Token/grafía original, esperado, perfil explícito, unidad e intervalo; EOF permite intervalo vacío. Las seis variantes actuales no distinguen por sí solas todos esos motivos. |
| Dominios gramaticales, LOC-057–059 | `kind` de relación, `kind` de patrón, régimen del grafo | Tipo canónico, objeto, valor y alternativas constituidas; nunca deducir IMM/CYB del nombre. |
| Nombres/referencias, LOC-060–065 y LOC-136 | Declaración repetida, referencia ausente, objeto en lugar de resultado y viceversa, tipo incorrecto | Identidad de ambas declaraciones en la colisión; referencia y declaración en incompatibilidad; sujeto comprobado y tipo esperado. |
| E004, LOC-069–070 | Codominio vacío o valores repetidos | Codominio, conjunto de repeticiones y declaraciones implicadas. |
| E115, LOC-066 y LOC-073 | Claves repetidas, ausentes, ajenas; el segundo emisor puede incluir varias | `CellSpec`, `OutputSemantics`, `Codomain` y cada colección diferenciada. No reducir el conjunto a una etiqueta única elegida por prioridad de presentación. |
| Objetos/operaciones generales, LOC-067–135 restantes | Geometría y vectores; puentes/conectores; tablas; horizonte/trayectoria; constitución nominal; fuente, entradas, contexto o destino incompatibles | Sujeto y referencias de la comprobación, posición/longitud exactas, esperado/recibido y conjunto afectado. Varias cadenas sólo conservan hoy el nombre: se debe instrumentar la guarda, no adivinar lo perdido. |
| Admisibilidad, LOC-171–173; adaptaciones LOC-021/080 | Etiqueta inválida, parámetro no positivo, regla ausente | Etiqueta original, especificación y parámetro; E110 del tipo no llega como campo por el `Debug` de la adaptación. |
| Cierre de marco, LOC-174–184; adaptación LOC-114 | Criticidades no producibles; estado duplicado/ajeno; varios estados por nodo; evaluación ajena/fuente repetida; entrada de puerta ajena; metaevaluación o destino de supervisión ajenos; sistema distinto | Los campos tipados ya incluyen nombres de marco/arquitectura, estado, nodo, evaluación, puerta, supervisión y destino según la variante. La adaptación debe preservar la variante y relacionarlos con sus declaraciones originales. |
| Resolución U, LOC-185–189; adaptación LOC-126 | Fuente no evaluable, posición fuera de rango, valor distinto de U, contexto distinto, mecanismo distinto | Estado/tipo, `Nat` exacto, longitud, `Tri`, especificación y esperado/recibido. El transporte no estrechará naturales ni confundirá fallo técnico con U. |
| E406, LOC-137–149 | Horizonte/grafo/nodo/célula ausente o de otro tipo; suceso repetido, nodo ajeno, posición inválida, destino repetido | `TransitionData`, horizonte, arquitectura, nodo/célula, suceso, posición/límite y par repetido. La precedencia de la validación general conserva su juicio. |
| Contexto relacional, LOC-150–169 | Referencias/tipos; trayectoria vacía; arquitecturas distintas; consultas ajenas; interfaz o política `silent_u` distintas | Consulta, agente, dominio, marco, trayectoria y referencias efectivas en cada comparación. La palabra `Domain` no acredita una constitución profesional recibida. |
| CLI/ABI/interfaz, LOC-190–223 | Argumentos/perfil, lectura/UTF-8, carga/integridad, capacidad técnica y rechazo de compilación | Origen técnico, unidad cuando exista, causa de sistema delimitada; separar indisponibilidad técnica de rechazo SVP y del estado de presentación. |

**Alcanzabilidad pendiente por ubicación.** La lista estática es un límite superior de sitios candidatos, no una campaña ejecutada. Por ejemplo, la rama UTF-8 de `frontend.rs:858` opera dentro de una entrada `&str` y sobre límites delimitados por comillas ASCII: no se aporta una fuente que la alcance. Las conversiones `Nat` desde dígitos y desde el cuadrado calculado tienen invariantes previos. En E406 varias referencias ya se validan antes; en contexto una trayectoria vacía puede haber sido rechazada por bienformación general. No se fabricarán fuentes imposibles para contabilizar esos sitios como casos públicos.

Durante la migración, cada sitio recibirá un caso por la puerta pública o una justificación revisada de que una guarda anterior lo excluye, con vínculo a esa guarda y su caso. Los nueve `unwrap`/`expect` se mantienen separados como aserciones: su inventario no prueba que puedan provocar un fallo ni permite convertir un pánico en rechazo semántico. La caracterización ejecutada de RETP-109 conserva sus 13 casos y su corte; este inventario no aumenta aquel recuento.

## 4. Procedencia y adaptación en las entradas públicas

| Entrada o tránsito | Dato disponible al entrar | Pérdida o límite observado por lectura |
|---|---|---|
| `compile_svp` | Texto/nombre; perfil EN de compatibilidad | `CompileError` sólo conserva `Frontend(...)` o `InvalidProgram(String)`. |
| `compile_svp_profile` | Texto/nombre y perfil explícito | El perfil selecciona superficie; no viaja como contexto en el error. |
| `compile_svp_assembly` | Unidades ordenadas, nombre, perfil y texto separado | El `?` de análisis no añade índice de unidad. Después se reúnen objetos/operaciones; la identidad agregada del ensamblaje no sitúa una colisión en sus dos declaraciones. |
| `tokenize` / `Parser` | Bytes de fuente e índice de recorrido; perfil y tokens | `Token` carece de intervalos originales. `canonical_word` puede producir el centinela de superficie extranjera que después aflora en `Unsupported`. |
| Validación general y contextual | Objetos/referencias y valores canónicos | No se dispone de intervalos por declaración/campo para devolver la fuente original. Varias comparaciones entregan sólo una cadena. |
| CLI | Perfil, ruta y resultado de lectura | `Debug` de compilación y texto de sistema se imprimen directamente. Las categorías se distinguen por retorno, no por un catálogo localizado. |
| ABI WebAssembly | Perfiles, búferes y distinción A/B | Las causas técnicas y SVP se empaquetan como bytes con el mismo indicador de error. El nombre de la función que termina en `_json` no garantiza JSON para el rechazo actual. |

La sede inicial continúa siendo un diagnóstico tipado y un mapa de procedencia paralelo al análisis, según §6 del contrato; no se ha demostrado necesidad de cambiar la IR canónica. Un mapa de nombres sin identidad de declaración no resuelve colisiones ni fallos anteriores a un objeto. Añadir textos en la web no recupera esa procedencia perdida.

## 5. Presentación y entrega observable

La lectura manual complementa el localizador de expresiones. En `app-b2.js`, `readPacked` decodifica bytes con UTF-8 estricto; una excepción de memoria/decodificación no equivale a una fuente inválida. `runSource` intenta leer JSON, conserva el texto si no lo consigue y usa `textContent`. Su `catch` general muestra también el rótulo de rechazo aunque la causa sea técnica. `runAssembly` añade unidades sólo si la carga se analiza como JSON; el texto de error antiguo no recibe ese contexto. Su `catch` también debe distinguir fallo técnico. Los rechazos de las consultas individuales de `Promise.allSettled` necesitan tratamiento explícito cuando se materialice la presentación conjunta.

La función `t` usa el idioma de navegación y un retorno alternativo al español o a la clave. Esta conducta de rótulos no cumple por sí sola la política diagnóstica por perfil ni el estado explícito de presentación incompleta. Las capturas de lectura de archivo y de arranque general también muestran `String(error)`; no se atribuye a esos textos un código del núcleo. Estos puntos de captura, retorno alternativo y presentación no se cuentan como nuevos emisores de causa en las 224 ubicaciones.

No se ha demostrado una elusión de permisos mediante estas carencias. La futura prueba de presentación debe conservar la señal de fallo aunque una plantilla diga otra cosa, no exista o falle. Código/tipo canónicos, datos originales y decisión no cambian de idioma. El ensamblaje ES+EN ofrecerá ambas explicaciones con atribución exacta; la unidad sana no se convierte en origen del defecto.

Los perfiles IMM/CYB mantienen independencia respecto de ES/EN. No se traducen automáticamente identificadores, unidades, reglas ni constituciones profesionales. Los referentes protegidos y la atribución profesional siguen pendientes de recepción comprobada. La observación en producción exigida en RETP-110 permanece posterior al despliegue de la realización verificada; el inventario y la fusión de #88 no satisfacen esa entrega.

## 6. Siguiente incremento y límite de terminación

1. Concretar causas tipadas y parámetros desde los emisores de §3, con correspondencia constituida o ausencia explícita de código. Resolver la alcanzabilidad por entrada y comprometer controles positivos/negativos antes de la realización de cada grupo.
2. Conservar unidad, perfil y posiciones en bytes desde el análisis; ligar declaraciones y referencias también en el ensamblaje. Probar EOF, colisiones, orden, nombres de archivo repetidos y UTF-8 con los controles DG04–DG08.
3. Implementar plantillas ES/EN revisadas y diagnóstico estructurado, separando juicio, procedencia y presentación. Fijar la versión del transporte antes de cambiar CLI/ABI. Migrar los oráculos por causa; conservar las obligaciones de las 106 fuentes inválidas sin copiar esperados de la implementación.
4. Aplicar la presentación a CLI, WASI y navegador; contrastar los mismos casos en cada destino ofrecido. Verificar candidato y, tras la publicación admitida, los artefactos realmente servidos por producción. Completar la matriz IMM/CYB × ES/EN sólo donde existan ligaduras comprobables.

Este incremento termina con el inventario, su reproducción y la continuidad documental. **DG01–DG14, localización funcional y entrega visible siguen pendientes.** No se proponen nuevas APIs, dependencias productivas, gramática, IR ni permisos. No se ejecuta ni modifica una operación de dominio. El enlace profesional R1 queda después del trabajo diagnóstico por mandato vigente; fila 9 y núcleo siguen abiertos.
