# Tiques técnicos del Sistema Vectorial SV

El registro reúne investigaciones técnicas delimitadas, vinculadas a los sucesos del proyecto. Se constituye a petición de la Dirección el 22 de septiembre de 2026, sobre la carpeta creada en el corte `62780b71c7c7c545b1b163ab9a8c0f33eade760e` de `main`.

## Consulta

- [Índice CSV de tiques](TIQUES_TECNICOS.csv).
- [TT-0001 · Identidad del modelo instalado y diferencia respecto de Qwen3.8-Max](TT-0001.md).
- [TT-0002 · Duración de las peticiones y procesamiento del historial](TT-0002.md).
- [TT-0003 · Fidelidad semántica, español y continuidad temática](TT-0003.md).
- [TT-0004 · Identificación de titularidad y licencias en la interfaz y las exportaciones](TT-0004.md).
- [TT-0005 · Correspondencia entre medidas, terminación y validación del contenido](TT-0005.md).
- [TT-0006 · Disponibilidad, autenticación y continuidad después de una ausencia](TT-0006.md).

## Relación entre registros

| Registro | Objeto y función | Cuándo se actualiza |
|---|---|---|
| Suceso del proyecto, por ejemplo S39 | Actividad, alcance, estado, dependencias e hitos. | Cuando cambia un hito o una condición relevante de la actividad. |
| Tique técnico | Una cuestión verificable, sus condiciones, pruebas, resultado, límites y criterio de cierre. | Al recibir evidencia, delimitar la causa o comprobar una corrección. |
| Registro de ejecución JSON o JSONL | Hechos y salidas de una sesión o petición concreta. | Lo produce la ejecución instrumentada; el análisis no reescribe sus hechos. |
| Acta | Decisiones y recepción formal de resultados con su alcance. | Cuando existe una decisión que exige ese documento. |

Un suceso puede reunir varios tiques. Cada tique tiene un suceso principal existente; las relaciones con otros sucesos se explican en la ficha y no alteran sus estados. Una ejecución puede aportar evidencia a varios tiques sin que se copie íntegramente en cada uno. La serie TT es independiente de las series S y RETP. Su numeración no expresa prioridad ni orden obligatorio de ejecución.

## Organización material

`TIQUES_TECNICOS.csv` es el índice canónico del estado y las fechas. Cada `TT-nnnn.md` contiene la pregunta, las condiciones, las observaciones, su interpretación, las limitaciones y la condición de cierre. La carpeta `json/` contiene datos estructurados de las comprobaciones cuando aportan valor; no se impone un JSON vacío por tique. Los resultados existentes se enlazan en sus revisiones inmutables.

El CSV utiliza UTF-8, coma como separador, una cabecera y campos entre comillas dobles. La columna `ficha` conserva la URL completa como texto, sin fórmulas. Las fechas son UTC en ISO 8601. Una fecha de cierre desconocida queda vacía. No se copia el estado vigente al JSON de evidencia, para evitar dos fuentes de estado independientes.

## Estados y cierre

Se reutilizan los valores del registro de Sucesos: **pendiente**, **en ejecución** y **finalizado**. Un diagnóstico en ejecución no significa que exista un proceso remoto activo. Finalizado significa que la cuestión delimitada ha concluido con evidencia; el resultado puede ser adverso. La publicación del código, la compilación, el despliegue y la aceptación de una corrección se distinguen expresamente.

El cierre debe identificar el criterio satisfecho, la prueba recibida y los límites que permanecen. La ausencia de prueba no se convierte en conformidad. Si reaparece el mismo defecto, puede reabrirse el tique con motivo documentado y nueva fecha de actualización; Git conserva los cierres anteriores. Una cuestión de alcance diferente recibe otro identificador y enlaza su antecedente. Los identificadores no se reutilizan.

## Alta, actualización y trazabilidad

1. Consultar el índice y los tiques pertinentes. Reutilizar una cuestión ya abierta antes de dar otra de alta.
2. Asignar el entero libre siguiente, con cuatro cifras como anchura mínima, y enlazar un suceso existente. Identificar la unidad que realiza el seguimiento.
3. Distinguir una observación recibida de una comprobación propia. Identificar configuración, versiones, presupuesto, ámbito de medidas y condiciones de aceptación antes de una nueva prueba.
4. Publicar conjuntamente la ficha y su fila vigente. Conservar las evidencias originales y la procedencia de cualquier extracción. Un nuevo resultado recibe una referencia o revisión identificable.
5. Actualizar el suceso mediante una síntesis cuando cambie su hito o siguiente acción, sin trasladarle cada muestra ni convertir cada tique en una nueva acta.

Git conserva las revisiones del índice y las fichas; no se crea otro historial manual de tiques. Antes de publicar se coteja el corte de la rama para evitar sobrescribir cambios ajenos. Un JSON legible o una huella coincidente no acreditan por sí solos veracidad, autenticidad externa ni suficiencia profesional.

## Perímetro de la recepción inicial

Los seis tiques iniciales pertenecen a S39, investigación lateral de (p1+P3)-Bis procedente de la adenda de integridad del consejo de OP-CYB-001. Reciben hechos de la vía nativa Qwen/Candle y permiten localizar necesidades para su devolución a la sede competente del Lenguaje. La gramática 0.2, la IR 0.3 y el núcleo conservan sus estados. S40 y las fases R2–R4 no se abren por este registro. Tampoco se ejecuta aquí la reorganización de fichas de sucesos prevista en S41.

Se han consultado AGENTS.md, las reglas de Sucesos, S39 y S40 en el corte indicado y el Acta 004 como antecedente de finalidad y secuencia. Esta recepción organiza evidencia experimental; no introduce una decisión doctrinal de dominio o de arquitectura nuclear.

Los archivos públicos contienen únicamente los datos necesarios para justificar la revisión. Las conversaciones completas, expedientes privados, credenciales y registros de seguridad personales conservan su acceso restringido.

## Referencias

- [Reglas de Sucesos](../sucesos/README.md).
- [S39 y estado del proyecto](../sucesos/SUCESOS_SV.md#s39).
- [Acta 004: finalidad, alcance y continuidad EIO](../../tuberias-ia/continuacion-15-09-2026/ACTA_004_FINALIDAD_ALCANCE_Y_CONTINUIDAD_EIO_2026_09_22.md).
