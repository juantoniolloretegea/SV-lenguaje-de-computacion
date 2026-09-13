# Contrato candidato de justificación y respaldo de evidencia

**Versión 0.1 · 13 de septiembre de 2026 · S22 · BIS-02/C10 · Juan Antonio Lloret Egea y Watson**

## 1. Objeto y estatuto

C10 concreta BIS-O12 y BIS-O06: la afirmación aceptada debe disponer de una justificación reconstruible y conservar las relaciones que la hacen válida en su contexto. El ejemplo humano de los tiques distingue la corrección del resultado de la corrección de los parciales y de su respaldo. El contrato y los escenarios son preparación de BIS-02; no acreditan una implementación productiva de consulta.

El suceso es el hecho; el registro, la prosa, la matemática y la imagen son representaciones. La existencia de un registro no demuestra por sí sola que ocurrió el hecho representado. Una operación de recuperación puede dejar trazabilidad sin requerir un frame. Cuando una decisión del SV exige respaldo algebraico, una narración o una imagen no sustituye sus requisitos. Este expediente documental no fabrica frames ni decide por dominios o agentes.

## 2. Relaciones exigidas

Cada conclusión debe identificar su contexto y alcance, las premisas utilizadas, las fuentes y revisiones correspondientes, los pasajes o campos que respaldan cada premisa y la regla de derivación aplicable. Debe conservarse también la evidencia adversa relevante, la cobertura que falta y el límite de la conclusión. Una lista de enlaces, una huella válida o una suma correcta no satisface por sí sola estas relaciones.

| Relación | Obligación comprobable |
| --- | --- |
| Afirmación y fuente | El contenido citado respalda la afirmación concreta, incluida su negación y sus límites. |
| Fuente y contexto | La identidad, revisión y pertenencia corresponden al encargo; no se eligen por conveniencia. |
| Premisas y conclusión | La regla declarada permite la conclusión en ese alcance; no aparecen pasos opacos. |
| Evidencia y cobertura | Las omisiones, contradicciones y dependencias relevantes permanecen visibles. |
| Justificación y entrega | La respuesta efectivamente entregada conserva el resultado, contexto y límites justificados. |
| Consulta y trayectoria | La consulta no altera retroactivamente los estados consultados ni sus evidencias. |

Citar una fuente para dos afirmaciones que respalda es admisible. Contabilizar dos veces un mismo hecho o presentar dos copias como evidencia independiente no lo es. La suficiencia depende del contrato, no del número bruto de citas.

## 3. Correspondencia con Documento V y la IR

Documento V §6 define Qω(A_D,s_n)=(r,J,M): respuesta tipada, justificación y metadatos. CQ1 exige reconstrucción; CQ2 excluye pasos opacos; CQ3 conserva la trayectoria; CQ4 explicita cobertura de interfaz; CQ5 exige consistencia o regla explícita de reconciliación; CQ6 conserva el efecto de criticidad y U sobre el cierre. Estas obligaciones rigen la evaluación de suficiencia, junto con las restricciones rectoras del Lenguaje. No se permite introducir inferencia opaca de un LLM en la cadena soberana.

La IR describe QuerySpec, QueryContext y QueryResult y sus obligaciones de bienformación. En Rust, la operación Query declara especificación, agente y contexto; result_type utiliza la etiqueta QueryResult. validate_query comprueba referencias y compatibilidades de contexto. Esos controles estructurales no constituyen por sí mismos la producción y comprobación integral de r, J y M. La deuda viva conserva los límites de H06/H07 y de la consulta productiva; C10 no los declara resueltos.

CQ6 no autoriza convertir una descarga fallida, un tique ausente o un error del proveedor en Tri.U. Si una consulta formal incorpora U, debe preservar su estatuto y la política constituida de dominio. K1-T sigue sin habilitar una conversión productiva observación → Tri. No se añaden tipos de IR, primitivas temporales ni códigos de diagnóstico.

## 4. Respaldo verificable y límites del observador

S2 aporta un antecedente acotado: su comprobador de cobertura coteja referencias del caso y de la configuración de vigencia con la lectura comprometida. Esa evidencia histórica no es un verificador universal de implicación semántica. C10 reutiliza la obligación de conservar las relaciones sin importar el caso médico ni atribuir nuevos resultados a S2.

Para un cálculo sintético puede fijarse un procedimiento exacto e independiente. Para una afirmación científica o profesional, la suficiencia exige el criterio y la revisión competente establecidos por su contrato. Comprobar sintaxis, tipos, huellas o palabras presentes no demuestra por sí solo respaldo semántico. El comprobador no puede tomar como única referencia la selección o la explicación del componente evaluado.

Se exigen evidencias externas de las operaciones, entradas, fuentes, reglas y entrega. Una explicación retrospectiva plausible no sustituye ese respaldo. El contrato no exige acceder al proceso interno privado de razonamiento del modelo y no presenta una justificación generada como transcripción garantizada de ese proceso.

## 5. Testigo documental y banco previo

El ejemplo usa importes enteros en céntimos y tiques estrictamente sintéticos. HIST_A_v1 compromete 4000+3500+2500=10000. REV_A_v2 sustituye T02-v1 por T02-v2 dentro de ese nuevo contexto: 4000+3000+2500=9500; quedan 500 sin justificar. Esto no significa necesariamente que esos 500 no se hayan gastado. El contexto histórico conserva su selección y sus bytes.

Se incluyen cinco tiques, una copia idéntica de T03, una nota de alcance y un resumen circular: ocho archivos de evidencia. La referencia independiente fija identidades, revisiones, pertenencia, regla y resultados esperados antes de recibir la propuesta. Los JSON son datos de preparación; no son tipos canónicos, permisos o QueryResult. El orden de presentación de sumandos no altera su suma; esto no modifica la obligación de preservar el orden posicional de una célula SV.

Veinte escenarios, cuatro positivos y dieciséis negativos, contrastan cobertura completa, presentación, reutilización legítima de citas, revisión con límite, duplicaciones, ausencias, pertenencia ajena, compensación de parciales falsos, extralimitación, revisiones incompatibles, opacidad, circularidad, evidencia adversa ocultada, contexto indefinido, historia alterada, confusión entre integridad y verdad, fallo técnico, invención y entrega incongruente. Cada escenario mantiene resultado observado nulo.

## 6. Condiciones de futura ejecución y criterio de salida

BIS-03 asignará sedes; BIS-04 materializará lo justificado. Antes de ejecutar se fijarán receptor, versiones, contexto, entradas, propuesta adversa o positiva concreta, observador independiente, límites de recursos y capturas. El banco actual especifica estímulos y resultados esperados; no incluye un productor completo de respuestas ni un ejecutor de todos los mutantes.

La campaña distinguirá comprobación documental, compilación Rust, ejecución de guardas y alcance semántico probado. La captura identificará la guarda alcanzada y las obligaciones posteriores no ejercidas. Una prueba de tipos no demuestra cobertura; un rechazo temprano no demuestra todas las guardas; un cálculo correcto no prueba una consulta formal ni una decisión profesional. La sensibilidad del comprobador deberá contrastarse con alteraciones deliberadas de referencias, parciales, revisiones y entrega.

C10-P/N originales continúan pendientes. Esta entrega acredita preparación e integridad documental, con cero escenarios C10 ejecutados en Rust y sin consultas a modelos. Los dos escenarios originales ejecutados y los veintidós pendientes conservan su estatuto. Continúa C11: presupuestos y límites de recursos. S24 conserva Bis → catálogo y cierre de fase → análisis e instalación de la GUI.
