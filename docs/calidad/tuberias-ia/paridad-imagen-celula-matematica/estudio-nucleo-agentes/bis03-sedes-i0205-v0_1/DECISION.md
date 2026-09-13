# Decisión de sedes para el montaje C02–C05

Juan Antonio Lloret Egea y Watson · S22 · RETP-2026-219 · 13 de septiembre de 2026.

## Corte y alcance

Lenguaje `3c4f5f87e2e052daacb25be4a4dfe7c1db0d946d`; laboratorio `e1779bdfc53ab815e7b3dc2c24914c3ecdafc3a2`, rama `lab/playground-sv-permanente`. [FUENTES.json](FUENTES.json) identifica rectores, contratos y código inspeccionado, con alcance de lectura y SHA256. Se revalidó la identidad de las lecturas rectoras anteriores conforme al workflow V2 §2. La lectura de código es estática; no se presenta como ejecución nueva.

**Se elige una composición experimental alrededor de la API pública de `sv_core`, con recepción, soporte, vínculo de revisión, geometría y observación locales.** Para este recorrido no se justifica ampliar gramática, IR, `Tri`, Frame ni permisos. No se ha implementado todavía esta composición. La decisión se limita al banco documental RETP-218: no fija la plataforma de usuario final ni acredita suficiencia integral.

El montaje se realizará en un workspace aislado de laboratorio. Un crate `sv_bis_i0205` dependerá por ruta de una copia de `sv_core` comprobada contra este corte; el manifiesto del montaje conservará los hashes de todos sus fuentes. El manifiesto Cargo se materializará con la ruta de esa copia, no con un `git main` flotante ni descargas durante la ejecución. El workspace productivo seguirá con sus tres miembros existentes. No se añadirá una dependencia de registro para esta primera realización: los auxiliares JSON y SHA se adaptarán desde antecedentes ya custodiados, bajo el complemento de pruebas aquí comprometido.

## Resolución por obligación

[DECISION_SEDES.json](DECISION_SEDES.json) contiene doce decisiones y sus enlaces a las doce obligaciones BIS. La cobertura de una obligación en esa matriz significa que tiene una sede o exclusión explícita, no que haya pasado una prueba.

| Pieza | Hecho comprobado en código | Decisión del montaje |
| --- | --- | --- |
| Compilación y perfil | `compile_svp_profile` aplica frontend, conformidad y validaciones antes de devolver `IrProgram`; `SourceProfile::from_tag` reconoce en/es. | Llamada pública directa con perfil explícito. La CLI ordinaria admite omitir perfil y lee con `fs::read_to_string`; no se toma como receptor acotado del banco nuevo. |
| Dimensiones | El frontend obtiene `n` mediante `square_nat`; `CellSpec` conserva b/n como `Nat`. Bienformación verifica b≥3 y longitud de estados. | Derivar todas las dimensiones desde `objects()`, incluidas CellSpec no seleccionadas. Compararlas como naturales exactos con el soporte versionado antes de convertir a `usize`. |
| Identidad | `IrProgram` e `IrObject` tienen campos privados y getters de lectura. La IR expone referencias de grafo, CoupledSpec y CoupledState. | Resolver los enlaces existentes y añadir fuera de IR el vínculo sintético de instancia/revisión. Comparar fuente, tipo de objeto y pertenencia efectiva, no sólo nombres de texto. |
| LIG | `validate_bindings` exige su BindingContract, Agent, Domain, operación y artefactos. Conserva programa y contrato en `ValidatedBindings`. | Mantener esa vía donde corresponda. El fixture integrado no tiene tales constituciones; un JSON de revisión no es un reemplazo de LIG. |
| Estado almacenado | La IR conserva `Vec<Tri>` bajo objetos sin acceso mutable público. `Tri::as_u8` vale 0/1/2. | Copia admitida en enum privado de arrays por tamaño; geometría usa radios explícitos 1/2/3, nunca el discriminante como radio. |
| Geometría | La API inspeccionada no constituye el descriptor ni la entrega exigidos por RETP-218. | Módulo experimental de representación derivada: fórmula y comparación posicional antes del SHA geométrico esperado. |
| Entrega | Compilar y serializar IR no constituye captura de entrega documental integrada. | Receptor local y captor del conductor, separados del sujeto; observador con reglas propias y oráculos comprometidos. |

La ausencia de implementación geométrica se delimita a las API y fuentes inspeccionadas y al contrato específico. No se afirma que el repositorio carezca de toda representación o recepción anterior.

## Almacenamiento y números

El primer candidato usará un `FixedVector` privado con variantes de arrays `[Tri;16]`, `[Tri;25]` y `[Tri;49]`. No se publica un constructor libre ni acceso mutable. La selección de variante requiere soporte admitido y longitud exacta; el soporte v1 sigue excluyendo 49 aunque exista una variante compilada capaz de almacenarlo. La IR recibida permanece intacta. El banco integrado sólo entrega una célula de 16; la declaración C7 de 49 ensaya soporte de la unidad completa, no una entrega de 49 posiciones.

Se prefieren estos arrays como hipótesis de realización concreta conforme al workflow §5. No se declara ventaja de rendimiento, ni ubicación garantizada en pila. Se medirá el coste de la enum, copias y almacenamiento antes de decidir su promoción o generalización. La API devuelve `&[Tri]` y carece de `push`, `resize`, setter de revisión o reemplazo de vector. La fabricación directa y el crecimiento deben fallar como pruebas de construcción/encapsulación en BIS-04.

`Nat` conserva una cadena decimal canónica sin cota semántica de palabra. El adaptador no aplicará `as usize` ni `as u64` a un natural recibido: primero pertenencia exacta al soporte y después conversión comprobada cuando se necesite indexar. Los coeficientes geométricos son enteros con signo. Los contadores y tamaños tienen tipos no negativos y conversión comprobada propia. Las sumas modulares sólo corresponden al algoritmo SHA, nunca a presupuestos.

## Recepción JSON y auxiliares existentes

`ie004/recepcion-av/json_estricto.rs` ya ofrece lectura acotada, rechazo de duplicados y Unicode, profundidad, contabilidad y reservas comprobadas. Sin embargo, su enum numérica sólo conserva u64 y tiene reglas especiales para claves `nodos` y `hijos`. **No es reutilizable sin adaptación para este montaje.** Se extraerá la recepción estructural, se retirarán esas reglas particulares del transporte anterior y se conservarán lexemas enteros exactos con signo antes del tipado por campo. Tampoco se trasladarán cuotas antiguas como si fueran el presupuesto competente del nuevo montaje.

El complemento [LIMITES_ADAPTADOR.json](LIMITES_ADAPTADOR.json) fija 32 contenedores anidados, 512 valores y 32 miembros por objeto para el decodificador experimental. Son límites de recepción del experimento, no máximos de células ni mediciones de RAM. Las cuotas de bytes RETP-218 siguen aplicándose por canal y agregado; 8192 es el techo del decodificador y los canales con cuota menor conservan esa cuota menor. Los 26 casos originales deben caber salvo el exceso deliberado de I0205-19. El verificador documental comprueba esta compatibilidad.

La recepción conservará la ausencia de un campo y `null` hasta su fase, sin inventar valores. En I0205-22, fuente inválida y soporte ausente, P02 debe anteceder al rechazo de soporte. Un deserializador que exija primero todos los campos alteraría el contrato. Los nombres de campos desconocidos, duplicados y formas JSON inválidas se rechazan como transporte; no se reparan ni se eliminan silenciosamente.

El antecedente `sha256.rs` incluye funciones SHA y un marco `SVAC0001`. Se reutilizará únicamente la función de huella y sus auxiliares necesarios; el marco y sus límites quedan fuera. El hash acredita identidad de bytes; no autoridad ni interpretación. El complemento compromete cinco entradas y huellas independientes, incluidos límites de bloque y relleno. No se exportará la función privada del frontend del núcleo para resolver una necesidad del laboratorio.

## Observador e inyecciones

El sujeto recibe los bytes de la solicitud, sus activos acotados, el contexto competente y los registros de referencia. No recibe el banco, los esperados ni los planes de inyección. El constructor del contexto pertenece al conductor; la solicitud no selecciona su propio contexto ni qué identidad se espera.

El sujeto devuelve un objeto de entrega validada con construcción privada, vector fijo y descriptor inmutable. El conductor lo lleva al receptor experimental y obtiene una captura final. Sólo el conductor aplica los planes de sustitución, ausencia y fallo. El observador compara captura, recibo, estado y primera guarda con los esperados; no llama al predicado de aceptación del sujeto para emitir su veredicto. Puede compartir los tipos de evidencia y la API soberana de compilación, pero no delega su juicio en `resultado=true` del receptor.

La independencia es de implementación y custodia dentro del laboratorio. No se presenta como aislamiento de procesos ni defensa frente a un host nativo comprometido. Un captor local admitido no es un tercero de confianza universal. Los cuatro mutantes OBS comprometidos deben discriminar sustituciones y afirmaciones sin captura antes de acreditar el observador.

Si falla antes de despachar, se exige observar que no hubo despacho. Si falla después, se conserva ese hecho y la incertidumbre del resultado. Ausencia de captura no prueba cero efectos. Una captura con bytes o contexto erróneos tampoco borra la entrega local ocurrida. No se convierte ninguna de esas situaciones en `Tri.U` ni se reintenta automáticamente.

## Paso acotado y pendientes

Se da por completada la **decisión documental de sede del montaje C02–C05** y se registra BIS-03 en ejecución en ese perímetro. BIS-02 permanece abierto para el conjunto del workflow. El contrato y banco de esta realización concreta ya están fijados en RETP-218; aquí se añade la interfaz y el complemento previo requerido para adaptar los auxiliares. Este paso acotado no cierra por arrastre las condiciones globales pendientes.

BIS-04 recibe una realización definida: crate externo, API pública de compilación, módulos de recepción/soporte/identidad/estado/geometría, receptor local y observador separado. Debe materializar el proyecto y documentar ES/EN el código nuevo, compilar con Rust/Cargo 1.98.0 y probar los auxiliares, construcción, 26 casos integrados y sensibilidades con evidencia propia. No se han ejecutado esas pruebas aquí. La elección de sedes no modifica la gramática 0.2, IR 0.3 o serializador 0.1.0.

Permanecen pendientes el alcance visual, ES/EN integrado, constituciones operativas, Frame inicial y reevaluación, composición, autoridad de agentes externos y costes de proceso. DFL-005 y las demás deudas no se cierran con esta decisión. El catálogo recibirá las causas tras realización y dictamen; S24 conserva la GUI después del cierre de fase.
