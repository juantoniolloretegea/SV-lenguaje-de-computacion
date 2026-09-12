# Acta S14 — Base consumida y recuperación frente a reevaluación

**CONFORME dentro del contrato fijado.** 2026-09-12T18:49:37Z. Watson / W-S0. S14 / RETP-184. Doce casos en seis ejecuciones (tres debug y tres release): 72 observaciones normales; 69 capturas idénticas por ejecución. Diecisiete invocaciones y tres sensibilidades detectadas en los casos previstos. Sin reintento funcional ni cambio de fuente o esperado posterior a la fijación.

## Resultado material

La misma solicitud A01 produce el cuerpo positivo anterior con la base de vigencia original y el cuerpo negativo anterior al consumir la base nueva. Las bases difieren en un byte y se cargan desde la misma ruta. Ambos cuerpos se cotejan íntegramente con esperados previos y conservan una llamada de política. Las doce observaciones por ejecución incluyen cargas, rechazos y recuperaciones: sólo dos actos nuevos producen recibo G1 en cada ejecución normal; no son doce consultas nuevas.

La sustitución de archivo bajo referencia antigua se rechaza como BaseDistinta. La reevaluación recibe identidad propia y padre original; recuperar después el original devuelve sus mismos cuerpo y base. Presentar la base nueva como original, cambiar identidad o sustituir el cuerpo genera el rechazo correspondiente. El límite de dos actos y la lectura acotada conservan sus causas.

G1 y sus dependencias permanecen intactos. La base exterior de vigencia del experimento no se confunde con K-IE004/1 del recibo. No se ha relajado la fijación de lote/montaje de S2. Esta campaña tampoco vuelve a acreditar la cobertura documental S2/S11 o el transporte S3: reutiliza el custodio previo y añade una frontera local de carga y atribución.

## Sensibilidades

| Mecanismo alterado | Evidencia prevista y observada |
| --- | --- |
| Omitir identidad de carga | GJ02 acepta indebidamente la base sustituida; exit 1 |
| Pasar siempre vigente=true conservando la base nueva | GJ04 produce cuerpo positivo en vez del negativo fijado; exit 1 |
| Omitir sólo la base al atribuir original | GJ06 acepta la atribución de base nueva al cuerpo original; exit 1 |

El segundo control comprueba consumo efectivo: las etiquetas y los bytes conservados no bastan si el productor recibe otro valor. Las capturas de los tres mutantes se conservan, incluidos los cuerpos producidos y las aceptaciones indebidas. No se cuentan errores de compilación como detecciones. La compilación heredada G1 emitió en ambos modos la advertencia de importación `Write` no utilizada; stdout/stderr completos están preservados. No se modificó la biblioteca para quitarla.

## Evidencia y medición

[Contrato](CONTRATO_S14.md), [fijación](FIJACION_PREVIA.json), [custodia previa](CUSTODIA_PREVIA.json), [procedencia](PROCEDENCIA.json), [fuentes G1](FUENTES_S2.json), [código](codigo/bases.rs), [conductor](codigo/contraste.rs), [observaciones](OBSERVACIONES.json), [491 capturas recuperables](EVIDENCIAS_S14.json), [comandos](COMANDOS.json), [binarios](BINARIOS.json), [entorno](ENTORNO.json), [mediciones](MEDICIONES.json) y [reproductor](reproducir.py).

Pared total de campaña: 4.681910 s. Suma de pared de invocaciones: 4.579708 s. CPU usuario: 4.969620 s; sistema: 0.692691 s. Incluye compilación y mutantes; no mide preparación/publicación, coste de modelo o latencia profesional. RSS individual, tokens y coste no disponibles.

Aperturas verificadas antes de ejecutar: público `84ca4113d0505a1868feb65f6b388e7c14b7d0d0`; laboratorio `cf573d5149821019fe5afd5d75a402aa9b66b399`. Fijación SHA-256 `eb16c7ed46fe272aee3e0437fa5ab3ce493cb689f29cc0235a1b8237a0076e04`. Los rectores previamente leídos permanecen idénticos en el corte de entrada y constan en [RECTORES.json](RECTORES.json). Los artefactos fijados —incluido el README de apertura— conservan sus bytes; esta acta sucede aquel estado pendiente sin reescribirlo.

## Alcance y relevo

G/J recibe evidencia de **base documental de vigencia realmente consumida y separación de actos dentro de un proceso**. El [inventario A–L sucesor](MATRIZ_COBERTURA_A_L_RESULTADO_S14.json) conserva los criterios y resultados anteriores. Formato, capacidad y límite se recogen en [causas por etapa](CAUSAS_OBSERVADAS.json); no se convierten en códigos canónicos por esta campaña.

Las referencias y el conductor son confiables. La comparación de bytes no autentica por sí sola un origen; la captura previa del archivo no prueba ausencia de carrera hostil. No se acreditan sustitución de binario/regla/corpus arbitrarios, restauración/reinicio, persistencia adversaria, canales, pantalla, acto profesional, conducta de LLM o QueryResult/TransitionData ejecutivos del núcleo. Tampoco se prueba aquí la rama SolicitudDistinta ni todos los fallos de E/S que el código puede emitir.

**Siguiente paso:** recibir G/J en el alcance exacto anterior y delimitar la siguiente obligación todavía pendiente del montaje integrado. F conserva la distinción integral entre negativa del proveedor, esquema inválido, no admisión y comunicación fallida; B/E/K/L y las fronteras profesionales/materiales conservan sus productores y puertas. No se declara cerrada toda la integración ni se amplía el núcleo por analogía. La matriz de suficiencia S13 sigue gobernando toda futura promoción.

Se continúa recogiendo causas durante la integración para consolidar después el catálogo aplicable. S12 permanece vigente: la posición de agentes se valorará tras cerrar inmunología. No se decide la suficiencia general del primer universo CYB. P3/P4/P5/P6 conservan reservas y condiciones.
