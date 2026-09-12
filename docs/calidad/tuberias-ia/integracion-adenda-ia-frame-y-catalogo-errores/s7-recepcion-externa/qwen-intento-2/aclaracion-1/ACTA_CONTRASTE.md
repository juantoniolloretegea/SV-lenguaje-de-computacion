# Qwen · Contraste de la explicación posterior

S7 · RETP-2026-173 · 2026-09-12T15:43:12Z · Watson / W-S0.

Se recibe una explicación atribuida a Qwen, pegada por el usuario. Se conserva como declaración posterior, no como respuesta corregida ni nuevo intento. [Texto aportado](DECLARACION_APORTADA.md), [procedencia](RECEPCION.json) y [contraste factual](CONTRASTE.json).

## Hallazgos

| Afirmación | Contraste con el expediente |
| --- | --- |
| Añadió una introducción antes del JSON del segundo intento. | No consta en el archivo recibido: comienza directamente por el bloque JSON, sin bytes anteriores. El párrafo citado aparece en el primer intento. La explicación mezcla hechos de dos entregas. |
| Añadió siete apartados después del JSON. | Confirmado en la segunda entrega. Incumple el formato del contrato S6. |
| Afirmó que no había texto fuera del bloque. | Confirmado y contradicho por el mismo archivo. Este hallazgo procede de lectura del observador, no de un detector de veracidad del cotejador. |
| S4 contenía una cláusula similar que prohibía texto adicional. | El encargo común S4 fijado admite JSON o tabla con todos los campos. No contiene esa prohibición. La afirmación no queda respaldada por ese documento. |
| La causa raíz fue arrastre de un patrón y una cortesía al observador. | Es la explicación retrospectiva del participante. La repetición de texto externo es observable; el mecanismo interno alegado no queda demostrado. |

Las tres conclusiones técnicas se mantienen: el bloque del segundo intento satisface las doce obligaciones por caso, el archivo completo es NO_CONFORME por texto posterior y su declaración de ausencia de texto externo es inexacta. No se añade un incumplimiento por introducción ausente, ni se convierte una admisión de culpa en evidencia de hechos no presentes.

El primer encargo se consulta en su [versión fijada](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c/pruebas-externas/s4-recorrido-documental/PRUEBA_COMUN.md). Huella SHA-256: eac67c8b4ff393501b0dea2106db77d90ba5d2fd64128e024227d8a3a42f26a8. No se extiende la conclusión a posibles mensajes externos que no se han aportado.

## Alcance y decisión vigente

La explicación reconoce dos hechos comprobados, pero añade una atribución al intento equivocado y una cláusula no localizada en S4. Por ello no constituye por sí misma un informe acreditado de causa raíz. No se deducen intención, engaño deliberado ni opacidad interna del modelo.

Se conserva la [aceptación provisional por la dirección](../DECISION_DIRECCION.md). Esta nueva declaración no modifica el contrato, el original, los resultados del cotejo ni la evidencia de trazabilidad presente. La aceptación no convierte en correctas las nuevas afirmaciones inexactas.

S7 continúa en ejecución: Qwen y DeepSeek recibidos en segundo intento; Claude y Grok pendientes. Esta aclaración no sustituye ninguna entrega. El responsable sigue la misma secuencia, registra las causas y conserva la consolidación del catálogo para el final.

Cortes leídos: Lenguaje main 20e66f973427a124ba96efa2b6956a49087bd2d0; laboratorio lab/playground-sv-permanente d243e3ed5a26162bdd639ac9204fa39deee22002. [Rectores sin cambios desde su lectura](RECTORES.json). Verificación documental sin nueva ejecución semántica ni modificación del instrumento.
