# Correspondencia de resolución, evidencia e inscripción aplicable — IE-004

**RETP-2026-142 · 11/09/2026. Responsable:** Watson. **Mandato:** Juan Antonio Lloret Egea, continuación del objeto único del workflow V2.

**Dictamen:** análisis acotado concluido. Hay evidencia recuperable de recepción y resolución, una cadena protegida R1 ya realizada en el núcleo y guardas de Frame comprobables. **No existe en el ejecutable IE-004 leído una llamada que una esos tres recorridos. La tubería integral hasta una transición exigible permanece NO_ACREDITADA.**

Este resultado localiza el trabajo restante. No propone una nueva célula ni cambia el significado de `U`, la gramática o el Frame.

## 1. Corte y pregunta comprobada

Lenguaje: `8880a099b0b9549aaa66c2c5d93e0b9c247834ff`. Laboratorio: `a45510f977af231295e794da9aec7243fb831855`. Se sigue el [workflow V2](WORKFLOW_ACOTADO_SUBORDINACION_IA_ES_V2_2026_09_11.md), §§3–4.

Pregunta concreta: **¿qué función y qué evidencia permiten pasar de la petición de IE-004 a su resolución y, si una operación constituida lo exige, a la inscripción y al Frame correspondientes?**

Se cotejaron 50 archivos públicos contra sus blobs Git. Las rectoras ya leídas en la revisión RETP-141 —AGENTS, Pilares, perfiles, transición y arquitectura— se conservan en el corte. Se revisaron específicamente el contrato R1-5, cierre R1, deuda vigente con sucesiones, fuentes de recepción, semántica, ligaduras y representación de Frame/TransitionData/Trajectory. El [inventario](ie004/trazabilidad-142/FUENTES_CORTE.json) identifica los archivos cotejados; no afirma que todo el repositorio se haya auditado ni que toda fuente inventariada se haya releído íntegramente.

Las actas privadas rectoras conservan su custodia. No se abrieron preguntas reservadas, sal ni oráculo P3. Las comprobaciones sólo usan el banco público RETP-137 y fuentes ya publicadas.

## 2. Recorrido encontrado

| Tramo y obligación | Realización localizada | Evidencia o testigo | Dictamen en este corte |
| --- | --- | --- | --- |
| Lote íntegro → solicitud individual conservada | `adaptar`, `solicitud_canonica`, `Decoder::lote` en el adaptador de lote; huella del lote, intervalo original, huella del caso y de la solicitud | Controles y capturas de RETP-140 conservados; lectura de fuente y contrato. No se reejecutó ese lote | Realizado en el banco; las identidades del sintético y la reserva siguen pendientes del tratamiento ya registrado |
| Solicitud → admisión y contexto | `solicitud`, `resolver_a`, `resolver_admitida` | C03: A01 recuperada, huella del transporte cotejada y reproducción literal de marco y traza | Comprobado para el testigo público elegido; no equivalencia con una solicitud de ejecución R1 |
| Pregunta → significado y resultado | `Motor::resolver`, `significados`, `permiso_y_dato` | Traza con original, normalizado, intervalos de tokens, significados finales, causas, llamadas de política y contadores; C03 conserva todos sus bytes | Reconstrucción demostrada para A01. La traza no es el pensamiento del modelo ni exporta cada microestado del análisis |
| Resultado → entrega de cuerpo | `cuerpo_a`, `marco_a`, `publicar_a`, modo `validar` | C04 acepta el marco completo; C05 rechaza truncación; C06 rechaza alteración del contenido sin actualizar huella | Integridad y rechazo comprobados. `SVAC0001` es un marco de transporte, no `sv_core::Frame` |
| Propuesta → comprobación V independiente del cuerpo | `ejecutar_v`, `verificar_propuesta`, `anexo_v` | C07 reproduce V02 válida; C08 reproduce V04 rechazada; cuerpo A ya fijado | Se comprueba una derivación sintáctica y su correlación. No unicidad semántica, autoridad de la IA ni autoría independiente |
| Archivos → evidencia recuperable | Cápsula y metadatos RETP-137; conductor `ejecutar.mjs` | C01 verifica 409 archivos; C02 coteja 36 recorridos históricos, hashes y orden A entregada antes de V | Custodia del banco comprobada. Recuperación de archivos no equivale a recuperación de continuidad autoritativa tras reinicio |
| Resolución IE-004 → decisión protegida R1 | El ejecutable incluye sus fuentes A/V; no incluye ni invoca `sv_core`, `decide_permit_traced` o `mediate_traced_permit` | Comparación del conjunto de `include!` de su `main.rs` con las entradas públicas R1 | Enlace no realizado. `llamadas_politica:1` y `vigente:true` pertenecen al montaje del banco; no producen `Permit` |
| Decisión protegida → mediación y ejercicio | `ProtectedDecisionContinuity`, `decide_permit_traced`, `mediate_traced_permit`, `execute_traced_mediated` | Contrato R1-5 y cierre R1; fuente liga `ExerciseRef` real con `DecisionTraceRef` antes de delegar al ejecutor | Realización R1 existente, de alcance intra-proceso. No se reejecutó R1 ni se declara integrado con IE-004 |
| Declaración de Frame → cierre estructural | `validate_frame_via_c03` llama a `Frame::from_candidate` | C09: 16 pruebas existentes, incluidos Frame válido, estado ajeno, duplicación y criticidad no producible | Constructor y guardas comprobados. Validar referencias declaradas no ejecuta una transición ni calcula el estado siguiente |
| Datos de transición y trayectoria | `IrObjectKind::TransitionData`, `IrObjectKind::Trajectory`, `transition_data_wellformed::validate_program` y guardas de contexto | La fuente comprueba horizonte, nodos, posiciones y duplicados; declara expresamente que no decide causalidad. DFL-003/004/006 y sucesiones delimitan pendientes | Representación y comprobaciones locales existentes; producción de la transición dependiente de IE-004 no acreditada |
| Operación → destino constituido | `OperationBindings`, `BindingDestination`, `ValidatedBindings`, LIG/0.1 | El destino es nodo/posición cuando la operación lo exige; fuente distingue `requires_destination` | Hay medios representacionales. IE-004 no aporta por sí solo constitución, instancia, operación y destino SV completos. No se deducen de sus cinco campos |

## 3. Comprobación técnica efectuada

El [plan previo](ie004/trazabilidad-142/PLAN_CONTRASTE.json) fijó nueve comprobaciones. Los números siguientes tienen denominadores distintos y no se suman como una nota de comprensión:

- **409 archivos:** recuperados de la cápsula pública, cotejando tamaños y SHA-256 de base64, gzip, contenido y cada archivo.
- **36 recorridos históricos:** nueve escenarios en cuatro configuraciones; cotejo de cuerpo, traza y orden de recepción, sin reejecutar esos 36 procesos. Se conserva el plazo agotado de V sin EOF como resultado del conductor.
- **Un caso A público:** reconstruido con Rust 1.98.0 nativo release; marco y traza idénticos a los capturados en RETP-137. Los paths de compilación cambian; no se exige ni se afirma identidad del binario reconstruido.
- **Tres comprobaciones de marco:** completo aceptado; truncado y alterado rechazados sin cuerpo.
- **Dos propuestas públicas V:** V02 devuelve `DERIVACION_SINTACTICA_COMPROBADA`; V04 devuelve `V_RECHAZADA / TRUNCADO`. Sus anexos coinciden literalmente con los históricos.
- **16 pruebas existentes de Frame:** ejecutadas mediante un arnés aislado que importa `nat.rs`, `frame.rs` y `frame_tests.rs` sin modificarlos. No es una ejecución del núcleo completo ni de una trayectoria.

El conductor documental utiliza Node para recuperar, cotejar y guardar artefactos; las comprobaciones funcionales indicadas se ejecutan en Rust. No se añade Node a la realización nuclear ni se ofrece este conductor como solución nativa productiva.

### Incidencias propias conservadas

El primer intento de mi observador supuso que todo `stderr` era JSON. WASI incorpora dos líneas de aviso de Node antes de la traza; C02 se detuvo antes de ejecutar Rust. La corrección reconoce exclusivamente ese prefijo observado, conserva el flujo completo y registra ambas partes. No corrige la mezcla de canales del banco.

En el segundo intento, mi esperado C08 decía `JSON_INVALIDO`. El control ya fijado en RETP-137 y su captura dicen `TRUNCADO`; el resultado nuevo era idéntico al antiguo. Rectifiqué el esperado del observador contra esas fuentes previas. A y V no se volvieron a ejecutar para corregir C08. C09 se ejecutó por primera vez después de esa rectificación.

Se conservan los dos resultados fallidos, los scripts respectivos, los originales y las [rectificaciones](ie004/trazabilidad-142/RECTIFICACION_ESPERADO_C08.md). El resultado final del contraste no borra esos errores ni los atribuye a Rust.

## 4. Carencias concretas y corrección mínima localizada

### G1 · Entrega de evidencia custodiada por invocación

El banco conserva los elementos en archivos y los enlaza mediante su conductor. A escribe traza en `stderr` y cuerpo en `stdout`; la validación del marco recibe sólo el cuerpo enmarcado. Por tanto, aceptar el marco no demuestra por sí solo que la traza completa haya quedado recibida, validada y recuperable.

**Testigo:** `V01` y `V02` de la misma configuración tienen el mismo cuerpo y el mismo identificador A01, pero distinto anexo V. La huella del cuerpo identifica contenido; no distingue esas invocaciones. Los nombres de ejecución y los registros del conductor sí permiten separarlas en esta campaña.

**Sede y mínimo siguiente:** contrato de recepción custodiada de IE-004, realizado por el lado receptor en Rust, fuera de Frame. Debe conservar una identidad de invocación en su ámbito, entrada exacta, montaje efectivo, versiones, cuerpo, traza y sus vínculos recuperables. El recibo base de A debe distinguirse del anexo posterior V; esperar a V no puede volver a bloquear la entrega A. Una evidencia ausente, truncada, duplicada o correspondiente a otra invocación no debe registrarse como recibo completo.

Esto exige un contrato explícito y testigos de esas pérdidas; no exige inventar otro valor de Tri, meter identidad de invocación en el cuerpo canónico de paráfrasis ni constituir una célula de nueve posiciones. Se preserva la distinción entre fallo del servicio y resultado de dominio.

### G2 · Correspondencia con una operación gobernada

R1 tiene una vía trazada; IE-004 utiliza una política artificial local. La respuesta `DATO` no produce ni puede convertirse automáticamente en una decisión R1.

**Testigo:** `permiso_y_dato` devuelve literales de `FICCION-IE004/1` tras comprobar campos y el booleano de vigencia; no recibe `FormRef`, `AuthorityRef`, `Req` ni un compromiso mediado. La entrada `execute_traced_mediated` requiere objetos sellados que esa función no construye.

**Sede y condición:** el Lenguaje debe delimitar el contrato de correspondencia con una operación constituida y la procedencia de sus referentes; dominio y agente aportarán lo que les corresponde en sus fases. Reutilizar R1 y LIG cuando se acredite el encaje; no añadir un constructor público para rellenar referentes ausentes. Se relaciona con DFL-005 y el límite de autoridad documentado en RETP-108, no con una nueva deuda ilimitada.

### G3 · Inscripción y Frame cuando realmente se deban producir

La consulta pública elegida es de lectura. No existe un dato en ese contrato que obligue a convertir su respuesta en una actualización celular. La evidencia de esa consulta debe cerrarse sin fabricar estado.

Para una operación que sí requiera actualización, se necesitan su constitución, el destino validado y el productor de la transición. El constructor Frame y el objeto IR TransitionData no sustituyen a ese productor.

**Testigo positivo existente:** Frame con referencias coherentes se acepta. **Contraejemplo existente:** una evaluación cuya fuente no está en el Frame se rechaza. Estos testigos discriminan cierre estructural, no la causalidad de una nueva transición desde IE-004.

**Sede y condición:** DFL-003/004/006, contratos de operación y puerta algebraica/productores correspondientes. Debe fijarse el requisito representacional necesario antes de consolidar el núcleo; la realización sigue su etapa. No se reabre C03 ni se modifica Frame por no disponer todavía de esa ejecución.

### G4 · Recuperación material de continuidad

Los archivos de esta campaña son recuperables. `ProtectedDecisionContinuity` y `ExecutionContinuity` mantienen objetos lógicos intra-proceso; su contrato excluye persistencia durable, recuperación tras reinicio y prueba física independiente del ejecutor.

**Sede y condición:** contrato material aplicable y R2, con las garantías posteriores de plataforma/integración que corresponda. Un archivo con hash no levanta esa limitación. Si el alcance exigiera recuperación autoritativa, seguirá bloqueado hasta realizarla; no se oculta como una anotación no bloqueante.

## 5. Dictamen adversarial del enlace

Las cuatro equivalencias siguientes quedan descartadas en este corte:

1. **Marco binario = Frame SV:** el primero contiene longitud, cuerpo y hash; el segundo referencias y cierre estructural.
2. **Anexo V conforme = decisión soberana:** V comprueba una derivación sintáctica y declara que no tiene autoridad sobre el cuerpo.
3. **Traza R1 existente = IE-004 integrado:** las funciones y entradas existen en piezas distintas; no hay llamada que las una en el ejecutable leído.
4. **Representar una transición = ejecutarla:** los validadores comprueban relaciones declaradas; los productores y su evidencia siguen siendo obligaciones diferentes.

Tampoco todo evento técnico es un SUCESO de dominio ni exige una posición celular. La decisión de inscripción debe proceder de la operación constituida. Esto corrige el salto de «hay respuesta y traza» a «sólo falta elegirle una célula».

No se ha demostrado necesidad de cambiar Σ, T(n), gramática, IR o campos de Frame para resolver G1. Para G2/G3, una modificación sólo podrá justificarse con una operación y una pérdida concreta; la carencia de integración no constituye por sí sola una contradicción de las actas.

## 6. Cierre de este objeto y siguiente intervención única

**Termina el expediente de correspondencia exigido por RETP-141:** cada tramo tiene fuente, evidencia o ausencia localizada y sede de resolución. **No termina la necesidad integral de trazabilidad.** No se abre aún P4 ni se declara protegida la IA.

El siguiente objeto mínimo es **fijar el contrato del recibo custodiado G1 y sus testigos de pérdida**, con realización nativa prevista en Rust y conservación del cuerpo canónico. Debe comprobarse entrega completa de A y vinculación posterior V, manteniendo visibles los límites de G2–G4. Esa es una pieza concreta de recepción, no un nuevo examen, dominio o agente.

La corrección del recibo no cerrará por sí sola la integración R1, la causalidad SV ni la recuperación durable. Esas capacidades sólo se incorporarán al alcance conforme a su contrato y fase. No se resolverán rellenando referentes desde la conversación.

La reserva y la asociación /2–/3 conservan su estado pendiente. Las correcciones de identidad del lote y las sucesiones del oráculo mantienen el tratamiento previsto antes de P3, fuera de este contraste público.

## 7. Fuentes y reproducción

Fuentes decisivas del corte:

- [Contrato A/V](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/docs/calidad/tuberias-ia/ie004/recepcion-av/CONTRATO_RECEPCION_AV_1.md), [entradas y entrega A/V](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/docs/calidad/tuberias-ia/ie004/recepcion-av/main.rs), [recepción y traza A](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/docs/calidad/tuberias-ia/ie004/recepcion-av/recepcion_a.rs), [resolución y política del banco](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/docs/calidad/tuberias-ia/ie004/semantica-a/servicio.rs), [comprobador V](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/docs/calidad/tuberias-ia/ie004/recepcion-av/verificar_v.rs).
- [Adaptador de lote](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/docs/calidad/tuberias-ia/ie004/lote-p3/adaptador.rs), [conductor histórico](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/docs/calidad/tuberias-ia/ie004/recepcion-av/ejecutar.mjs) y [manifiesto de cápsula RETP-137](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/docs/calidad/tuberias-ia/ie004/recepcion-av/ARCHIVO_CAPTURAS.json).
- [Decisión trazada R1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/rust/sv_core/src/decision_trace.rs), [ejecución gobernada](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/rust/sv_core/src/execution.rs), [contrato R1-5](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/docs/arquitectura/CONTRATO_R1_5_LIGADURA_DECISION_EFECTO_Y_TRAZA_DETERMINISTA_2026_08_25.md) y [cierre R1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/docs/arquitectura/ACTA_TECNICA_CIERRE_R1_2026_08_25.md).
- [Frame](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/rust/sv_core/src/frame.rs), [sus pruebas existentes](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/rust/sv_core/src/frame_tests.rs), [bienformación](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/rust/sv_core/src/wellformed.rs), [TransitionData](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/rust/sv_core/src/transition_data_wellformed.rs), [LIG/0.1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/rust/sv_core/src/bindings.rs) y [deudas y sucesiones vigentes](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8880a099b0b9549aaa66c2c5d93e0b9c247834ff/docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md).

[Resultado del contraste](ie004/trazabilidad-142/RESULTADO_CONTRASTE.json), [evidencia recuperable](ie004/trazabilidad-142/EVIDENCIA_RECUPERABLE.json) y [procedimiento de reproducción](ie004/trazabilidad-142/REPRODUCIR.md). Los artefactos contienen tiempos y paths del instrumental cuando se registraron; no son nuevas primitivas ni una medición de P5. Se conservan ambos fallos del observador y la rectificación de su esperado. No se ejecutó ni corrigió código privado o reservado.

