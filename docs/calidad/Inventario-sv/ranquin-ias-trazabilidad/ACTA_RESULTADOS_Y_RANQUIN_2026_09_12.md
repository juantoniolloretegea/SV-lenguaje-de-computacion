# Acta de resultados y ranquin documental de IA · SV

12 de septiembre de 2026 · Watson / W-S0 · S10 · RETP-2026-178.

Se consolidan cinco participantes, diez entregas y 120 casos. Grok y Claude alcanzan conformidad documental completa en su segunda entrega. Qwen conserva la aceptación provisional de la Dirección, con su entrega original técnicamente NO_CONFORME. DeepSeek conserva un incumplimiento acotado de fidelidad. Mistral queda descartado por la Dirección de la selección actual tras su segunda entrega NO_CONFORME.

La selección de una implantación propia todavía no se ha realizado. Este ranquin ordena **entregas documentales observadas**; no certifica todos los procesos internos, la identidad del modelo servido ni la aptitud universal de una empresa o familia de modelos.

## Evidencia y criterio del ranquin

Los originales, diagnósticos, ejecuciones del observador y rectificaciones permanecen en sus expedientes. [Evidencias de entrada](EVIDENCIAS_DE_ENTRADA.json) identifica cada archivo empleado, tamaño, SHA-256 recalculado y enlace a un corte inmutable del repositorio. Todos coinciden con los blobs de ese corte. [Resultados detallados](RESULTADOS_DETALLADOS.json) conserva los dictámenes y recepciones; [120 casos en CSV](RESULTADOS_POR_CASO.csv) facilita su consulta. El [ranquin CSV](RANQUIN.csv) y su [versión JSON](RANQUIN.json) contienen las mismas cinco filas.

La primera prueba S4 usó una rúbrica de 100 puntos y dejó reservas explícitas por etiquetas públicas ambiguas. Se conservan sus puntuaciones literales y sus reservas; no se convierten en notas definitivas ni se utilizan para ordenar participantes. La segunda prueba, SV-TRAZABILIDAD-2/1, fijó previamente contrato, banco, referencia y verificador: todas las obligaciones debían satisfacerse, sin compensación entre ellas. Su [paquete público fijado](https://github.com/juantoniolloretegea/SVcustos-dataset/tree/fccde9cf524a0d62b2dd1a2ee05170d6f4358074/pruebas-externas/s6-trazabilidad-total) permanece intacto. La cualificación del instrumento registró 482 controles; ello acredita ese alcance, no una prueba universal de toda IA.

El orden descriptivo adoptado en esta acta es posterior a la recepción: primero, conformidad íntegra del original S6; después, entre los originales no conformes, corrección de decisiones y relaciones causales y, a continuación, fidelidad de la cadena documental. No se impone retroactivamente otra condición de aprobado. Por eso DeepSeek precede a Mistral aunque tenga menos citas exactas: sus discrepancias quedan limitadas a sangría, mientras Mistral también altera una regla y responde causas y fundamento incorrectos. Los números de errores no son una escala común de gravedad.

## Ranquin y modelos disponibles al lado de cada participante

| Puesto documental | IA y versión declarada en S6 | Resultado original S6 | Fuentes exactas | Modelos con pesos disponibles del mismo editor |
| --- | --- | --- | --- | --- |
| 1, empate documental | Grok 4.6 · xAI | CONFORME_DOCUMENTAL | 32/32 | [Grok-1](https://github.com/xai-org/grok-1), Apache-2.0; [Grok-2](https://huggingface.co/xai-org/grok-2), licencia comunitaria xAI. Son versiones distintas de la ensayada. |
| 1, empate documental, exposición alta | Claude · configuración declarada `claude-opus-5` · Anthropic | CONFORME_DOCUMENTAL | 32/32 | No se identificaron pesos descargables de Claude en el [catálogo oficial revisado](https://platform.claude.com/docs/en/models/overview). |
| 3 | Qwen3.8 · Alibaba / Qwen | NO_CONFORME; bloque diagnóstico conforme | 32/32 en el bloque | [Qwen3.8-27B](https://huggingface.co/Qwen/Qwen3.8-27B), Apache-2.0; [Qwen3.8-2.4T-A95B](https://huggingface.co/Qwen/Qwen3.8-2.4T-A95B), licencia Qwen3.8-Max. Variante servida no acreditada. |
| 4 | DeepSeek V3 · DeepSeek | NO_CONFORME | 6/32 | [V4.1-Flash](https://huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash) y [V4-Pro-0813](https://huggingface.co/deepseek-ai/DeepSeek-V4-Pro-0813), MIT. También [R1](https://github.com/deepseek-ai/DeepSeek-R1) y [V3.2](https://huggingface.co/deepseek-ai/DeepSeek-V3.2). No equivalen a V3 ensayado. |
| 5 | Mistral Medium 3.5 · Mistral AI | NO_CONFORME; descartado por Dirección | 24/32 | [Medium-3.5-128B](https://huggingface.co/mistralai/Mistral-Medium-3.5-128B), MIT modificada; [Small 4](https://mistral.ai/news/mistral-small-4/), [Large 3 y Ministral 3](https://mistral.ai/news/mistral-3/), Apache-2.0. |

Claude declaró exposición alta y participación previa en el expediente. Su empate expresa igualdad de conformidad de la entrega, **no igualdad demostrada de capacidad en condiciones independientes**. La versión declarada tampoco certifica el despliegue servido. Los demás participantes conocían o podían conocer el banco público; no se presenta la segunda ronda como prueba ciega ni se presume ausencia de exposición por falta de declaración.

El aprobado de un servicio comercial no se hereda por otro conjunto de pesos. Incluso cuando coincide el nombre, cambiar pesos, cuantización, instrucciones, herramientas y entorno exige una validación de esa implantación. Pesos disponibles y licencia permisiva tampoco acreditan por sí solos auditoría completa del entrenamiento o de todos los procesos internos.

## Conservación de la primera ronda

| IA | Resultado /48 | Traza /36 | Procedimiento /8 | Entrega /8 | Total literal /100 | Puntos reservados | Incidencia independiente de las etiquetas |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Grok | 39 | 35 | 8 | 8 | 90 | 8 | E05: `CasoDistinto` en lugar de `VigenciaDistinta`, sostenido en la justificación. |
| Claude | 38 | 35 | 8 | 8 | 89 | 9 | E05: conversión de recibo negativo a DATO no sustentada por el caso. |
| Qwen | 33 | 23 | 8 | 8 | 72 | 3 | Causas técnicas no preservadas, límites por caso ausentes y traza insuficiente en E11/E12. |
| DeepSeek | 44 | 35 | 8 | 7 | 94 | 3 | E10: `FaltaCaso` en lugar de `FaltaVigencia` y justificación contradictoria. |
| Mistral | 42 | 32 | 6 | 8 | 88 | 4 | E10: causa incorrecta; E07: consecuencia incompleta y traza 1/3, inferior al mínimo 2/3. |

Estas cifras se transcriben de las evaluaciones históricas, enlazadas individualmente en el CSV. Las reservas no se suman como puntos ganados. Mistral S4 procede de la transcripción del JSON pegado por la Dirección, identificada como tal; no se atribuye a ese archivo la condición de original binario del proveedor. Su repetición posterior en el chat no se cuenta como una tercera prueba ni como un fallo nuevo.

## Segunda ronda: hallazgos y decisiones

**Grok:** doce casos y las 32 citas requeridas conformes; ninguna discrepancia del cotejo documental. No se adjuntó un registro de ejecución externa que permita certificar herramientas o procesos internos.

**Claude:** doce casos y 32 citas conformes. Su declaración de exposición se conserva íntegra en la recepción. Sirve como cotejo documental con esa exposición; no como participante independiente sin conocimiento previo.

**Qwen:** los doce casos y 32 citas del bloque extraído son conformes, pero el archivo original añadió siete apartados después del JSON y afirmó que no había texto fuera de él. El original sigue NO_CONFORME. La aceptación provisional expresada por la Dirección no cambia ese resultado. Las explicaciones posteriores mezclaron S4 y S6 y propusieron una causa interna no demostrada; su rectificación se conserva. No hubo texto introductorio antes del JSON en S6 y no debe imputársele. La promesa de ajustarse al formato todavía no es una nueva entrega verificada.

**DeepSeek:** decisiones, causas, reglas, fundamentos, consecuencias y límites correctos. Veintiséis de 32 citas difieren sólo en sangría, según el diagnóstico conservado. Es un defecto de fidelidad identificable; no acredita por sí mismo ocultación de procesos. La propuesta de transportar las fuentes canónicas sin reescribirlas no consta validada en otra entrega.

**Mistral:** el adjunto correcto de la segunda prueba es distinto del S4 repetido en el chat. El cotejo encuentra doce discrepancias: ocho citas F06/F07 con doble escape de saltos de línea, R08 alterada en E09, causa E09 nula en lugar de `ContenidoDistinto`, y causa/fundamento E10 incorrectos (`FaltaCaso`/B10 en lugar de `FaltaVigencia`/B04). E01–E06 carecen de discrepancias; los doce rótulos de decisión coinciden, pero eso no subsana las otras obligaciones. La Dirección decide descartarlo de la selección actual. El expediente demuestra estos incumplimientos; no determina qué componente los originó ni convierte esa decisión de selección en una conclusión universal sobre todas las IA del editor.

Ninguna explicación posterior sustituye los originales ni constituye automáticamente un nuevo intento. Gemini fue una consulta informativa, Watson es el observador y Open Interpreter es una herramienta de ejecución: no se les asignan puestos como participantes ensayados.

## Tiempos, esfuerzo y trazabilidad de actividad

| Medida | Evidencia recibida | Uso en esta acta |
| --- | --- | --- |
| Duración total y de herramientas | No hay registros comparables verificados de inicio y fin | Sin clasificación de rapidez; orden de llegada no equivale a duración |
| Llamadas de herramientas S4 | DeepSeek: recuento no disponible; Qwen: 6 declaradas; Claude: 9 declaradas; Mistral: 1 declarada como medida sin registro; Grok: 8 declaradas | Se conserva el dato y su procedencia; no se certifica la ejecución a partir del nombre de herramienta |
| Tokens y coste | No disponibles o no verificados en el expediente | Valores ausentes; no se imputan ceros |
| Esfuerzo humano | No hay cronometraje homogéneo de preparación, asistencia y corrección | Se registran dos entregas por participante y exposición; no se deduce eficiencia |
| Actividad externa S6 | No se aportan registros completos de ejecución de los participantes | La conformidad es documental; la actividad externa sigue sin certificarse |
| Trabajo de Watson | Cotejos, scripts y huellas conservados en los expedientes | Son medidas y actividad del observador, no del participante |

Las declaraciones originales y las ausencias se conservan en `RESULTADOS_DETALLADOS.json`. Si una futura ejecución emplea scripts, código o conectores, la admisión de esa actividad requerirá sus artefactos y registros de entradas, salidas y operaciones. Una explicación posterior del modelo no reemplaza esos registros.

## Publicación, actualización y abandono de los modelos disponibles

Consulta de fuentes oficiales: 12/09/2026. Inventario de opciones relevantes, no lista exhaustiva de todas las variantes o cuantizaciones. Se distingue publicación de pesos, anuncio, actualización de código/configuración y entrenamiento nuevo.

| Editor | Publicación comprobada | Continuidad observada | ¿Anuncio de abandono? |
| --- | --- | --- | --- |
| xAI | Grok-1: 17/03/2024 en [noticias oficiales](https://x.ai/news). Grok-2: subida 22/08/2025 y ficha 23/08/2025 en [historial](https://huggingface.co/xai-org/grok-2/commits/main). | Grok-2 muestra actualización de licencia el 05/11/2025. No acredita renovación posterior de sus pesos; ambos permanecen disponibles en el [editor oficial](https://huggingface.co/xai-org). | No encontrado en las páginas revisadas. Disponibilidad histórica no equivale a compromiso de mantenimiento. |
| Anthropic | [Opus 5](https://www.anthropic.com/news/claude-opus-5), 24/07/2026, es publicación de servicio; no publicación de pesos identificada. | Opus 5 aparece activo en la [política de retirada](https://platform.claude.com/docs/en/about-claude/model-deprecations). | No aplicable a una rama abierta no identificada. Retirar versiones anteriores no equivale a abandonar Claude. |
| Alibaba / Qwen | 3.8-2.4T-A95B: 12/08/2026; 3.8-27B: 14/08/2026, según [anuncios del repositorio](https://github.com/QwenLM/Qwen3.8). | Lanzamientos recientes y [actividad de la ficha 27B](https://huggingface.co/Qwen/Qwen3.8-27B/commits/main). La actividad del repositorio no acredita nuevos pesos en cada cambio. | No encontrado en las fuentes revisadas; la familia presenta actividad reciente. |
| DeepSeek | V4.1-Flash: septiembre de 2026; V4-Pro-0813: variante de agosto de 2026. No se fija un día de publicación que el expediente no haya confirmado. [Editor](https://huggingface.co/deepseek-ai). | El [historial V4.1-Flash](https://huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash/commits/main) muestra una corrección de codificación dos días antes de la consulta. | No encontrado en las fuentes revisadas; hay nuevas versiones. Esto no implica actualización de V3 ensayado. |
| Mistral AI | Medium 3.5: repositorio desde 29/04/2026 y [artículo fechado 22/05/2026](https://mistral.ai/news/vibe-remote-agents-mistral-medium-3-5/); Small 4: 16/03/2026; Large 3 y Ministral 3: 02/12/2025. | [Historial Medium 3.5](https://huggingface.co/mistralai/Mistral-Medium-3.5-128B/commits/main): configuración actualizada el 15/07/2026. | No encontrado en las fuentes revisadas; la sustitución de una versión en una plataforma no demuestra abandono de todos sus pesos. |

Las fechas relativas del historial de DeepSeek se conservan como tales; no se convierten en una fecha absoluta exacta. “No encontrado” es el resultado de las fuentes revisadas, no una garantía de inexistencia de cualquier anuncio. El [registro de fuentes](FUENTES_OFICIALES.json) identifica las URL y los límites de la consulta.

Para la futura revisión de licencias destacan tres diferencias: [Grok-2](https://huggingface.co/xai-org/grok-2/blob/main/LICENSE) tiene licencia comunitaria propia; [Qwen3.8-2.4T-A95B](https://huggingface.co/Qwen/Qwen3.8-2.4T-A95B/blob/main/LICENSE) emplea Qwen3.8-Max con condiciones específicas, distintas del Apache-2.0 de 27B; [Mistral Medium 3.5](https://huggingface.co/mistralai/Mistral-Medium-3.5-128B/blob/main/LICENSE) usa MIT modificada con condición de ingresos y posible licencia comercial separada. No se equiparan a MIT o Apache sin condiciones adicionales. Las variantes destiladas de R1 deben revisar también la licencia de su modelo base. No se ha emitido todavía un dictamen de compatibilidad para la implantación del SV.

## Relevo a la integración de los puntos 1 y 3

**Sí, se puede retomar la tarea principal.** La recepción comparativa tiene ya un resultado documentado y no exige otro torneo antes de continuar. **La integración no está cerrada.** El punto 1 es integridad/trazabilidad IA y el punto 3 reconstrucción, recibo y frame; el catálogo/localización ES/EN es el punto 2 y conserva su consolidación posterior.

Se reutilizan S1 (cobertura independiente C/I), S2 (vigencia causal con una llamada de política en ambos estados) y S3 (presentación, conservación y pérdida de negación), dentro de sus alcances sintéticos nativos. S4–S9 añaden el contraste externo documental. No acreditan pantalla, revisión humana efectiva, imposición frente al host, persistencia material, actuación profesional ni cierre completo A–L.

El siguiente paso es reconciliar la matriz A–L con esta evidencia y fijar el siguiente hueco del recorrido integrado, conservando un objeto presentado, su invocación, las fuentes exigidas y un comprobador con referencia independiente. Se debe fijar montaje, controles, ataques, esperados y presupuesto antes de ejecutar. La cobertura parcial existente se reutiliza, no se vuelve a contar como evidencia nueva.

Se mantiene la reserva P3 y la custodia humana ya recibida en RETP-130; no se solicita otra vez. P4, P5, P6, las obligaciones profesionales y la aceptación humana conservan sus condiciones. Se recogen causas para el catálogo durante la integración; se consolida después el catálogo y ES/EN. No hay promoción productiva ni modificación del núcleo en esta acta.

## Custodia de esta consolidación

Publicación en la carpeta de ranquin solicitada, espejo en el laboratorio, actualización de Sucesos SV y RETP-178, y relevo añadido al README de integración. Se retira únicamente el marcador `inicio.md` de esta carpeta. Se conservan las actas anteriores, incluidas las rectificaciones y los dictámenes no conformes. El manifiesto identifica las huellas de los archivos de esta consolidación.
