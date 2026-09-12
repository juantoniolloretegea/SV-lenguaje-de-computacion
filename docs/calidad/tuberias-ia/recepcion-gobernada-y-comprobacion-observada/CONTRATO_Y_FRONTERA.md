# Contrato candidato de recepción y frontera de confianza

RETP-154. Desarrollo técnico acotado del pendiente de RETP-153 y del contrato de consumo documental CYB §8.3. No constituye facultades institucionales ni sustituye la secuencia rectora.

## 1. Distribución de responsabilidades

| Responsable | Objeto que debe aportar o realizar | Comprobación exigida |
| --- | --- | --- |
| Humano experto y sede competente del dominio | Acto constituido, titular, facultades, alcance y reglas recibidas | Existencia y reconocimiento conforme al gobierno aplicable, no un «sí» generado por IA |
| Frontera competente de admisión | Evidencia de procedencia del acto y premisa externa exigida por T-0 | Procedimiento y productor identificados, evidencia recuperable y rechazo ante ausencia o invalidez |
| Instalador de confianza del ensamblaje | Plan exacto recibido y correspondencia del verificador con la obligación | Instala antes de procesar propuestas; no acepta una regla arbitraria elegida por la petición |
| Núcleo | Validar T-0, preservar ligaduras y ejecutar la comprobación técnica admitida | Tipos privados, comparación concreta, rechazo y evidencia conservada |
| Propuesta de IA | Datos sometidos al contrato recibido | No constituye autoridad, premisa, plan admitido ni resultado de verificación |

La primera fila recoge una decisión del autor ya recibida. Las otras filas son obligaciones técnicas, no preguntas que se devuelvan al autor para que diseñe Rust. El contrato no selecciona clave, proveedor de identidad, transporte, host o perfil de agente no constituido.

## 2. Realización candidata de la recepción

`PreparedGenesisReception::prepare` es interno al crate. Recibe y posee la premisa opaca **ya producida**, el `GenesisPlan` ya fijado, los bytes del acto, emisor declarado y versión. No crea `ExternalGenesisPremise`, no cambia la visibilidad de `from_core_id` y no interpreta el acto. El plan contiene formas, autoridades, efectos, contexto, requisitos y aplicabilidades conforme al R1 existente; recibe sus validaciones reales de T-0.

`receive` sólo admite acto, emisor declarado y versión. Comprueba límites y coincidencia exacta con la instalación antes de tomar el plan. Si difieren, devuelve error y permite que el original siga siendo recibido. Al llegar a T-0 el preparado queda terminal tanto en éxito como en rechazo constitutivo. Un error de T-0 no entrega continuidad, no permite reemplazar el plan y no se disfraza de U. Un éxito conserva los bytes y expone la continuidad únicamente por referencia compartida.

No existe recuperación tras reinicio ni registro durable de consumos. La política terminal local no prueba ejecución «exactamente una vez» en un sistema distribuido. El llamador custodio tiene que registrar también los errores devueltos si el contrato forense exige historia completa; este objeto no sustituye ese registro.

## 3. Productor observado de comprobación

`PreparedExactCheck::prepare` es interno al crate y exige descriptor y aplicabilidad pertenecientes a la **misma continuidad**, comprobados por referencia a los objetos almacenados. La igualdad de sus textos no basta. El préstamo Rust mantiene vivos e inmutables los objetos durante el uso. No se serializan direcciones ni se presenta la identidad de memoria como identidad institucional o durable.

El descriptor tiene que ser `Specific`. Además, el instalador competente debe haber recibido la regla de que esa obligación significa igualdad exacta de bytes con la referencia fijada. El tipo `Specific` por sí solo no demuestra esa regla: el contrato se conserva como bytes y la instalación semántica sigue siendo una obligación explícita del ensamblaje. El comparador no interpreta o valida una regla profesional. Las obligaciones `Core` se rechazan siempre en esta vía.

`run` ejecuta la igualdad exacta. No acepta `CheckResult` como entrada ni ejecuta una función arbitraria aportada por la IA. Devuelve un objeto que conserva la `RequirementCheck` producida internamente, contrato, referencia, observación y préstamo de continuidad. No permite extraer `RequirementCheck` ni fabricar un resultado resuelto para otra continuidad.

| Entrada observada | Resultado del comparador | Evidencia |
| --- | --- | --- |
| Bytes presentes iguales | `Accredited` | Referencia y observación exactas |
| Bytes presentes diferentes | `Refuted` | Se conservan ambas secuencias |
| Evidencia no disponible | `NotVerifiable` | Ausencia explícita (`None`) |
| Secuencia presente vacía | Se compara con la referencia, incluso si ésta también está vacía | `Some([])` se conserva distinto de `None` |
| Tamaño excedido | Error técnico `TooLarge` | No se produce comprobación |

Los estados son resultados técnicos de esta comparación. No son valores ternarios del dominio, veredictos profesionales ni autorizaciones. La procedencia física de los bytes observados no se deduce del contenido. La invocación de prueba proporciona los bytes directamente; no acredita captura remota ni verdad del emisor.

## 4. Límite material y cierre requerido

La premisa usada en los tests nace exclusivamente de `ExternalGenesisPremise::for_test`, dentro del banco sintético existente. No hay una raíz profesional instalada. La compilación ordinaria conserva la ausencia de ese constructor. Se prueba la recepción posterior a una admisión supuesta; **no se ha realizado todavía la admisión material de la raíz**. Tampoco se producen las comprobaciones nucleares obligatorias de autoridad, forma, aplicabilidad y no autoacreditación.

Para habilitar una operación concreta deben quedar identificados su acto material admitido, la evidencia que acredita su procedencia, el productor de premisa, los verificadores de todos sus requisitos y el enlace completo a permiso, mediación, ejercicio e inscripción exigibles. La prueba debe incluir ausencia, sustitución, repetición y pérdida de vigencia en la medida que el contrato recibido las requiera. Una firma sin emisor reconocido, un hash o una coincidencia de texto no satisfacen esas condiciones.

Este pendiente no invalida la recepción documental autorizada ni abre un agente. No se crea ahora una API genérica `aprobar(bool)` ni un registro universal de autoridades. Las leyes comunes del SV permanecen en el núcleo; las composiciones particulares y la presentación poligonal de cada especialista conservan sus sedes y fases.

## 5. Recursos, compatibilidad y límites del ensayo

Se fija 65536 bytes como máximo por campo de acto, emisor, versión, contrato, referencia y observación en este montaje. Acto, emisor, versión y contrato no pueden estar vacíos; referencia y observación sí. Se comprueba longitud antes de copiar. Esto acota las copias nuevas de esos campos, no el tamaño previo del plan, el número de instancias, la memoria residente, el tiempo total o un servicio. El agotamiento de memoria del asignador queda fuera de esta garantía local.

La recepción y cada resultado guardan copias propias de evidencia. El número de ejecuciones y su retención dependen del custodio; no se presume una cuota de servicio. Sólo se ha ensayado compilación nativa con Rust 1.98.0. No se añade dependencia externa, interpretación de prosa, gramática, IR, Frame, operador algebraico o diagnóstico canónico. La candidata RETP-153 es su antecedente técnico y continúa pendiente de integración.
