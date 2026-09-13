# Contrato candidato de conservación de Tri.U y separación del fallo de representación

**Versión 0.1 · 13 de septiembre de 2026 · S22 · BIS-02/C08 · Juan Antonio Lloret Egea y Watson**

## 1. Objeto y fundamento

La representación recibe un estado matemático constituido y referenciado. Su elaboración o presentación no constituye una nueva ternarización, una resolución de U ni una reevaluación de la arquitectura. La IR 0.3 §2 establece la separación entre `Tri`, admisibilidad y fallo técnico, y §2.4 mantiene no habilitada la producción observación → Tri mediante `Ternarizer`.

C08 concreta la obligación BIS-O07 y la enlaza con BIS-O08: fidelidad del estado y evidencia de cada estatuto. Es un contrato candidato para BIS-03. No modifica gramática, IR, núcleo, dominio o perfiles. La letra U puede representarse legítimamente; la inexistencia o el fallo de una representación no origina una U.

## 2. Estatutos que deben conservarse

| Objeto o situación | Estatuto | Obligación |
| --- | --- | --- |
| `Tri.U` en estado admitido | Valor ternario válido | Conservar identidad, posición y símbolo; aplicar el convenio gráfico declarado. |
| `CaptureOutcome::Bottom` | Fallo técnico de captura | Mantenerlo fuera de Tri; no reutilizar el nombre como error universal de todo componente. |
| `NotAdmitted` | Estado de admisibilidad | Impedir la vía no admitida; no convertirlo en 0, 1 o U. |
| Representación no solicitada | Ausencia de solicitud según contrato de uso | No afirmar ejecución, materialización o fallo. |
| Representación aún no materializada | Falta de artefacto comprobado | No confundirla con inexistencia o invalidez del estado matemático. |
| Soporte indisponible | Situación técnica de disponibilidad | Registrar evidencia y efecto sobre el uso solicitado. |
| Artefacto rechazado | Resultado de comprobación contra contrato | Conservar causa y candidato; no presentarlo como representación verificada. |
| Fallo de producción o presentación | Incidencia técnica del componente que falla | Identificar etapa y evidencia; conservar el estado fuente. |
| Artefacto materializado y verificado | Resultado gráfico conforme al alcance probado | Mantener vínculo con estado, revisión y convenio; no presumir consumo o comprensión. |

Estas filas no definen un enum nuevo ni una partición exhaustiva de la IR. Algunas situaciones coexisten: un estado puede contener U y tener además un fallo técnico de representación. El resultado del proceso, la existencia del artefacto y su entrega se observan por separado. Ninguna etiqueta técnica constituye por sí sola la evidencia del hecho.

## 3. Regla de conservación y frontera de uso

Para un intento de representación del estado referenciado S se exige conservar exactamente su vector, constitución, instancia y revisión. Si el proceso falla, el resultado técnico debe expresar el fallo y su alcance. No se rellena, elimina, permuta ni cambia una coordenada para obtener una imagen completa. Un reintento es otra actividad técnica con evidencia propia; no reescribe el intento fallido ni cambia silenciosamente la revisión de origen.

La ausencia de imagen tiene efectos según el contrato de uso. Si la presentación es opcional, su ausencia no invalida el estado ni autoriza efectos adicionales. Si el uso exige representación comprobada, la falta de esa representación impide ese uso. Esta detención técnica no equivale a una clasificación U ni a la anulación del estado matemático. Una operación semántica posterior conserva su contrato y sus precondiciones; no se autoriza por disponer de imagen.

Los fragmentos de salida pueden conservarse como evidencia identificada del fallo. No se presentarán como artefacto completo ni como una versión válida de `frvis`. Una representación antigua puede conservar su identidad histórica; no se atribuirá a otra revisión. C03/C05 gobiernan identidad y consumo. Una huella concordante no demuestra autoridad, vigencia solicitada o fidelidad gráfica por sí sola.

## 4. Convenio gráfico y límites del testigo

Se conserva C04: `Zero` corresponde a radio 1, `One` a radio 2 y `U` a radio 3. El discriminante Rust de U es 2; ni ese entero ni el radio 3 sustituyen al símbolo o establecen gravedad universal. La representación mantiene orden y correspondencia paramétrica. El testigo de 16 posiciones se copia literalmente del banco C04 y contiene cinco U. Su tamaño pertenece al ensayo, no constituye un nuevo dominio ni un catálogo universal de dimensiones.

El oráculo exige el mismo vector antes y después del intento. Las cinco posiciones U y la geometría exacta de C04 son referencias previas. No se han producido píxeles ni probado legibilidad, presentación o consumo. Comparar descriptores no prueba la imagen final. La admisión Rust del testigo y la ligadura de identidad siguen siendo precondiciones que deberá acreditar el montaje.

## 5. Capacidades Rust constatadas por inspección

`Tri` es un enum cerrado con Zero, One y U. `TryFrom<u8>` devuelve error para valores ajenos a 0, 1 y 2. El analizador `take_tri` admite las etiquetas canónicas y rechaza otras. `CellState` comprueba longitud frente a n. `AdmissibilityState` y `CaptureOutcome<T>` son tipos distintos; el código incluye ejemplos `compile_fail` contra su conversión automática a Tri. Esta lectura estática no equivale a ejecutar esos ejemplos en este incremento.

Esas restricciones evitan determinadas confusiones de tipos. No prueban que un adaptador externo no escriba explícitamente `Tri::U` después de capturar un error; ese literal sería un valor válido desde el punto de vista del enum. C08 exige comprobar además procedencia, conservación y contrato. Una prueba que sólo rechaza `Bottom` en un vector no acredita detección del mutante que lo reemplaza por U antes de entregarlo al núcleo.

La inspección de los tipos de IR y la concordancia previa no acredita un productor gráfico ni un canal completo de resultado de representación. BIS-03 decidirá dónde imponer estas obligaciones y cómo reutilizar las ligaduras existentes. Como criterio de diseño, los resultados técnicos y los datos matemáticos necesitan tipos y accesos que preserven la distinción; no se fija ahora una firma Rust, una biblioteca gráfica o una GUI. Los comentarios del código que se incorpore seguirán la política ES/EN; los perfiles fuente conservan su cometido propio.

## 6. Banco, observador y evidencia exigida

El banco contiene veinte escenarios: seis positivos y catorce negativos contractuales. Todos permanecen especificados y no ejecutados. Se aportan seis fuentes SVP: un estado válido previsto, cuatro valores ajenos a Tri y un vector incompleto. Son entradas parciales para la futura recepción; no constituyen un montaje de renderizado ni una campaña Rust ejecutada.

Los negativos incluyen fallo convertido en U, relleno, Bottom/NotAdmitted/null/NaN como coordenadas, éxito sin artefacto, salida parcial, revisión equivocada, U tratado como fallo, uso obligatorio sin imagen, indisponibilidad convertida en resultado, consumo no probado y U forzado a Zero. El observador deberá recibir el contrato y el vector esperado desde el conductor independiente, comprobar identidad y bytes efectivos y registrar la inyección por una vía que no dependa exclusivamente del componente sometido a prueba.

Se conservarán comandos, versiones, salidas, diagnósticos y orden real de rechazos conforme al criterio Rust C05. Rechazo de compilación, rechazo de datos y detección funcional son evidencias distintas. Un fallo previo no acredita una guarda posterior. No se añaden códigos al catálogo ni se declara operativo E003. El cotejo auxiliar Python de este expediente verifica integridad y consistencia de datos; no sustituye compilación ni validación en Rust.

## 7. Relevo

C08-P/N originales permanecen pendientes. BIS-02 continúa con C09: contenido documental frente a autoridad y órdenes incrustadas, conservando que un conductor sintético no demuestra el comportamiento de un LLM. BIS-03 decidirá sedes y BIS-04 realizará los cambios justificados. S24 mantiene la secuencia Bis → catálogo y cierre de fase → análisis e instalación de la GUI.
