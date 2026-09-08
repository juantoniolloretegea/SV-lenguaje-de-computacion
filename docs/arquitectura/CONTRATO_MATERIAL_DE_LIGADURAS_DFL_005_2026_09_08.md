# Ligaduras por operación · contrato material LIG/0.1

**Corte de entrada:** `eeb7cf47bfbbbc6b617107b5976cde358a460377`, PR #79 no integrada. **Fecha:** 08/09/2026. **Estado:** candidata; fila 7 abierta.

## 1. Sede y capacidad concreta

Este contrato sucede al §4 del contrato de continuidad de fila 7. Se apoya en F-SV/0.1 §3.1, G/H SP-01/02/05 y GH-DOC-07. Se escoge una entrada Rust tipada, pública y versionada en `sv_core`, distinta de la gramática SVP y de la proyección 0.1.0. El núcleo comprueba el contrato recibido y devuelve una representación inmutable de los usos de **una operación solicitada expresamente**. Los adaptadores no repiten la validación.

La capacidad ofrecida es comprobar y recuperar las ligaduras declaradas. El identificador de operación designa al consumidor externo del contrato; no añade una operación SV ni ejecuta Q0, `query`, admisión, transducción, criticidad o reglas clínicas. La compilación ordinaria continúa produciendo IR declarativa; por sí sola no acredita ligaduras. Toda entrada que ofrezca LIG/0.1 debe pasar por `validate_bindings`.

No se modifica la API de producción del navegador ni su serializador. El mismo validador se ejercitará en nativo, WASI y navegador mediante una sonda de pruebas que se compila aparte y no se distribuye como adaptador productivo.

## 2. Identidad y referencias exactas

El contrato contiene esquema `LIG/0.1`, identificador y versión propios; fuente, SHA-256 de fuente y SHA-256 de la proyección completa recibida; dominio y agente explícitos; constitución y declaración de autoridad; instancias, operaciones y artefactos referidos. El solicitante aporta una expectativa independiente de identificador, versión y SHA-256 del contrato, más identificador y versión exactos de operación. No se elige la última versión ni se deduce una de otra.

La huella del contrato usa una codificación binaria definida: prefijo ASCII `SV-LIG-0.1` seguido de cero; cada texto y cada bloque de bytes llevan longitud `u64` big-endian; cada lista lleva número de elementos `u64` y conserva su orden; las opciones llevan marca 0/1; los naturales llevan su representación decimal canónica como texto. Los campos siguen el orden de declaración de los tipos públicos; las variantes enumeradas se codifican mediante su nombre estable como texto. La huella no se incluye a sí misma.

Cada artefacto declara identificador, versión, SHA-256, clase y bytes. El núcleo exige identidad única por identificador en este corte, versión exacta, clase requerida y correspondencia de bytes con SHA-256. Las clases son constitución, declaración de autoridad, definición de captura, definición de admisión, definición de ternarización, procedencia, regla de compartición, definición de operación e información lateral. Los identificadores y versiones no pueden estar vacíos ni contener controles; no se normalizan.

La comprobación de bytes **no interpreta el documento referenciado ni autentica a quien lo constituyó**. Estas referencias conservan procedencia y versión; no sustituyen las obligaciones de ejecución o autoridad de su sede. El resultado no se denomina contrato clínico comprobado ni certificado de suficiencia de Q0.

## 3. Instancias y cadena declarada

Cada instancia tiene identidad propia, propietario explícito igual al dominio del contrato, parámetro perteneciente a `Domain.parameters`, numeral natural declarado, referencia a captura, admisión y eventual ternarizador, y procedencia no vacía. Captura y admisión deben resolver objetos del tipo correcto, incluidos en las listas del dominio, y compartir el numeral declarado. Sus definiciones enlazan por nombre con `mapping` y `rule`, respectivamente. El ternarizador, si comparece, pertenece al dominio, enlaza su `mapping` y tiene el mismo espacio de observación que la captura. Su presencia no ejecuta una función ni produce `Tri`.

Dos instancias pueden compartir tipo, numeral y especificación; conservan identificadores distintos. La pertenencia se comprueba por referente, nunca por igualdad de plantilla. El programa exacto impide sustituir una declaración conservando el nombre sin cambiar el enlace del contrato.

## 4. Operación, usos y multiplicidad

Una operación contiene identificador y versión, definición exacta, usos ordenados, exigencia explícita de destino celular, declaraciones de compartición y alcance de información lateral. Cada uso tiene identificador único, referencia de instancia, destino opcional `(NodeId, Nat)` y alias opcional.

Si la operación exige posición, todo uso debe declarar destino. Si no la exige, la ausencia de destino es admisible. Todo destino presente resuelve un `CoupledSpec` del grafo del agente y una posición `[1,n]` de su `CellSpec`, sin estrechar `Nat`. Ninguna lista de parámetros constituye una célula.

Un uso independiente no puede ocupar un destino ya ocupado en esta operación: LIG/0.1 no define agregación de instancias sobre un mismo destino. Un alias refiere un uso **anterior**, de la misma instancia y destino; no crea una nueva instancia ni puede autorreferirse. Usos repetidos de una instancia, incluidos alias, exigen una regla de compartición y una lista exacta de los identificadores de uso en el orden declarado. Una declaración sobrante, incompleta o reordenada se rechaza. Dos nodos distintos que comparten `CellSpec` siguen siendo destinos distintos.

La operación sin S exige lista lateral vacía; la operación con S exige al menos un referente exacto de información lateral. El resultado conserva expresamente el alcance conjunto. No acredita por ello recuperación de una respuesta clínica ni factoración universal. Las exigencias de destino y de S se aplican a la operación solicitada; no se impone a otra operación una distinción que no consume.

## 5. Testigos previos y criterio de aceptación

Base sintética: dominio D, agente AG, grafo G1 con CC1 y CCCompartido sobre C1; CCFuera fuera de G1. Dos instancias I1/I2 del parámetro P y numeral 1; captura Cap/Phi, admisión Adm/Regla y ternarizador Ter/Tau, ya declarados. Operación OP versión 1: usos U1→I1→(CC1,3), U2→I2→(CCCompartido,3), sin S. Los artefactos tienen bytes literales fijados por el testigo; sus huellas son derivadas de esos bytes, nunca de la salida del validador.

| Testigo | Cambio aislado | Obligación y resultado exigido |
|---|---|---|
| L01 | Cambiar versión del contrato manteniendo expectativa | Rechazo de identidad contractual. |
| L02 | Cambiar orden de usos manteniendo expectativa | Rechazo de huella; con nueva expectativa constituida, orden distinto preservado. |
| L03 | Cambiar programa manteniendo su enlace | Rechazo de identidad del programa. |
| L04 | Alterar bytes de artefacto sin su huella | Rechazo de integridad del artefacto. |
| L05 | Repetir identificador de artefacto con otra versión | Rechazo de ambigüedad; ninguna selección de última versión. |
| L06 | Referencia a versión ausente de una definición | Rechazo de referencia exacta. |
| L07 | Referencia a artefacto de clase distinta | Rechazo de clase. |
| L08 | Repetir identidad de instancia | Rechazo de identidad duplicada. |
| L09 | Propietario o parámetro ajeno | Rechazo causal respectivo. |
| L10 | Captura/admisión ajena o del tipo incorrecto | Rechazo de pertenencia/tipo respectivo. |
| L11 | Numeral distinto de captura o admisión | Rechazo de identidad paramétrica. |
| L12 | Definición no correspondiente a mapping/rule | Rechazo de enlace de regla. |
| L13 | Ternarizador ajeno o espacio distinto | Rechazo de pertenencia/espacio; ninguna ejecución Tri. |
| L14 | Instancia sin procedencia | Rechazo de fundamento ausente. |
| L15 | Operación o versión no declarada | Rechazo; ninguna alternativa automática. |
| L16 | Uso refiere instancia inexistente | Rechazo referencial. |
| L17 | Destino obligatorio ausente | Insuficiencia tipada; no célula ni U implícitas. |
| L18 | Destino CellSpec, inexistente o nodo ajeno | Rechazo causal de tipo/referencia/pertenencia. |
| L19 | Posición 0, 10 o natural mayor que u64, sobre n=9 | Rechazo de rango; posiciones 1 y 9 admitidas. |
| L20 | Dos usos independientes ocupan el mismo destino | Rechazo de colisión. |
| L21 | Repetir instancia sin compartición declarada | Rechazo; con regla y lista exacta, admisión sin deduplicar. |
| L22 | Alias a uso posterior o a otra instancia/destino | Rechazo de alias; alias anterior concordante admitido con compartición. |
| L23 | Lista de compartición incompleta o permutada | Rechazo de multiplicidad/orden. |
| L24 | S presente bajo alcance sin S, o alcance conjunto sin S | Rechazo de alcance; conjunto explícito conservado. |

Los negativos deben alcanzar la guarda semántica: salvo L01/L02, se recalcula la huella del contrato alterado, pero no las huellas de artefactos que el testigo declara intactas. Una expectativa intacta no puede cortar antes la prueba de una regla interna. La reparación puntual devuelve el control a admisión. Las mutaciones retirarán guardas una a una; deben compilar y ser detectadas por sus testigos. No se deduce cobertura exhaustiva ni independencia de realizaciones.

## 6. Continuidad

Esta candidata materializa la representación y validación local de las ligaduras ofrecidas por LIG/0.1. La matriz final de pérdidas de fila 7 y la suficiencia operacional G/H siguen pendientes. K1-T, DFL-006, autoridad ejecutiva, recuperación de respuestas con S, persistencia y capacidades N4 no se cierran mediante referencias a documentos. PR #79 conserva su contraste externo pendiente; la nueva candidata se apila sobre su cabeza exacta. DFL-011, DFL-012 y DFL-013 conservan identidad y sede. No se modifica el dominio inmunológico ni se abre CYB.
