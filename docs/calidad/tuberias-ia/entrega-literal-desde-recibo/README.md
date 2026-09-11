# Entrega literal desde un recibo custodiado

**SV · RETP-2026-149 · 11 de septiembre de 2026**  
**Estado: candidata Rust comprobada en una frontera literal nativa. Sin aceptación productiva final.**

El contenido que se pretende entregar se compara con el cuerpo original conservado por el custodio. Recalcular correctamente la huella de un contenido alterado ya no basta para obtener una entrega comprobada en esta ruta. El dato cambiado y la negación eliminada se rechazan; los originales válidos se conservan.

## 1. Qué responde a las dos frases

**«Ahora debemos conocer qué evidencia».** Para este paso, la evidencia ya está acotada: solicitud original del montaje público, resolución y traza conservadas por el custodio, referencia opaca a esa invocación, propuesta concreta de entrega, causa de aceptación o rechazo y bytes efectivamente emitidos por el conductor de ensayo. Los resultados originales se cotejan además con los esperados fijados antes de esta candidata. La propuesta no escoge ni reemplaza el recibo contra el que se comprueba.

**«Pero no demuestra que sepamos identificarla de forma suficiente».** La nueva prueba sí demuestra suficiencia para una pregunta precisa: **¿conserva esta entrega literal todos los bytes del cuerpo de este recibo?** La respuesta se obtiene mediante comprobaciones Rust y controles positivos y negativos, sin pedir a otra IA que juzgue a la primera. No convierte este resultado en un criterio universal para decidir qué rasgos bastan para identificar cualquier objeto.

La secuencia del volcán aportó el ejemplo humano de pérdida relevante. Aquí se materializa un tramo de su obligación: no aceptar como conservación del original un dato sustituido o una negación eliminada. Pasar de esa conservación literal a una imagen requiere todavía comprobar la relación entre el contenido autorizado y lo que realmente se dibuja. La imagen, su significado y su fidelidad no quedan resueltos por esta comparación de bytes.

## 2. Autoridad y alcance recibidos

Se conserva la aclaración del autor: la autoridad corresponde al humano experto del dominio constituido; las actuaciones se delimitan en el agente consumidor. Diseñar y comprobar el mecanismo técnico corresponde a Watson. **No se reabre la pregunta sobre quién autoriza ni se pide al autor que diseñe el verificador.**

Esta candidata no constituye un dominio, un agente o un permiso profesional. Reutiliza el productor determinista y el custodio experimental ya existentes. Une parcialmente el punto 1 —subordinación e integridad de la IA— con el punto 3 —conservación hasta la entrega—. Las causas alimentan el punto 2 desde ahora, pero el catálogo y su localización se cerrarán después del alcance de pruebas correspondiente.

## 3. Cambio mínimo en Rust y proceso

1. El conductor confiable conserva el manejador de la invocación esperada. El custodio produce y conserva A mediante su ruta existente; el proponente no puede cerrar A por la API pública.
2. Se recibe un marco propuesto de hasta 4144 bytes. Antes de asignar memoria se aplica el cupo; la reserva se carga al presupuesto existente de G1, sin aumentarlo.
3. Se valida el marco y se compara su cuerpo con el obtenido del recibo del custodio. Una huella coherente sólo supera la primera comprobación.
4. Se registra un único intento por recibo. Se conservan estado, causa, tamaño propuesto y, cuando la captura acotada ha podido completarse, los bytes de la propuesta. Un exceso de tamaño conserva la causa y el tamaño observado, no el contenido ilimitado.
5. Sólo una comprobación satisfactoria permite obtener `EntregaLiteral`. Sus campos y constructor están protegidos; la vista toma el cuerpo del custodio, no el del proponente. Su vida está ligada al custodio y su identidad incluye ámbito y ordinal.
6. El conductor de entrega probado escribe exclusivamente desde esa vista. Un rechazo previo no emite cuerpo por stdout. Se conservan la evidencia de ensayo y los recibos originales.

El intento rechazado no se repara ni se sustituye con un segundo intento silencioso. Recuperar posteriormente una entrega vuelve a comprobar la disponibilidad de la custodia. Las operaciones de recuperación anteriores siguen disponibles para diagnóstico: esta candidata no demuestra que cualquier consumidor externo esté obligado materialmente a usar la nueva ruta.

La variante de lote liga el intento a la posición y al manejador ya fijados por el enlace público existente. No busca recibos por una etiqueta aportada por la propuesta. No se transforma automáticamente una entrega en SUCESO ni en `sv_core::Frame`.

[Fuente de la frontera](entrega.rs) · [Cambio completo de seis piezas](CAMBIO_CANDIDATO.patch) · [Reproducción](REPRODUCIR.md)

## 4. Resultado observado

Una matriz, dos configuraciones nativas —debug y release— con Rust 1.98.0. **31 procesos en total; ningún fallo inesperado ni corrección causal.** Los rechazos de compilación indicados abajo son controles negativos esperados.

| Comprobación | Resultado en ambas configuraciones |
|---|---|
| Marco original A01 | Se entrega exactamente el cuerpo anterior |
| Dato 8.40 → 9.40, sin actualizar huella | Rechazo por integridad; ningún cuerpo entregado |
| Dato 8.40 → 9.40, con huella recalculada | Rechazo por contenido distinto; ningún cuerpo entregado |
| «no acredita» → «acredita», con huella recalculada | Rechazo por contenido distinto; ningún cuerpo entregado |
| Marco de recibo revocado presentado frente al recibo original | Rechazo por contenido distinto |
| Recibo revocado conservado literalmente | Entrega de su negativa original, sin convertirla en aprobación |
| Manejador ajeno, falta de cierre A, reintento y límites | Rechazo con estado y causa definidos |
| Pérdida de traza después de comprobar | No se recupera una entrega como comprobada |
| Forjar `EntregaLiteral` desde otra crate | Rust rechaza el acceso a campos privados |
| Usar la vista después de destruir el custodio | Rust rechaza la vida insuficiente de la referencia |
| 24 posiciones del lote público | 24 entregas iguales a las huellas y cuerpos esperados de RETP-140 |

Por configuración pasan **29 pruebas G1 anteriores + 11 nuevas de entrega + 12 anteriores del lote**. Hay además cuatro contrastes del conductor y el recorrido de las 24 entregas. Se comprobó paridad de los 24 cuerpos y de las salidas de los cuatro contrastes entre debug y release. Las fuentes usadas se fijaron antes de compilar y se cotejaron al terminar.

[Resultado estructurado](RESULTADO.json) · [Evidencia con entradas, salidas y códigos](EVIDENCIA.json) · [Plan fijado](PLAN_FIJADO.json)

## 5. Qué queda fuera de lo acreditado

- **Verdad de origen:** conservar fielmente una resolución errónea no la vuelve verdadera. El productor A y la selección de la referencia por el conductor pertenecen a la base confiable de este ensayo. No se acredita su infalibilidad por esta prueba.
- **Interpretación visual:** la candidata exige identidad de bytes. No acepta traducción, resumen o cambio de representación como equivalentes. Esas relaciones deberán tener contratos y comprobaciones específicos; no se introducen reglas sobre volcanes en el núcleo.
- **Trazabilidad:** se conservan operaciones observables y artefactos, no el pensamiento interno de un modelo. La retención del intento es intraproceso. La cápsula de campaña es conservación documental, no persistencia durable del servicio.
- **Imposición material:** no se prueba un sistema operativo hostil, una pantalla modificada después de la entrega, una fuga por otro canal ni la imposibilidad de que un consumidor eluda esta API. La salida de la CLI puede fallar durante la escritura; no hay protocolo de entrega atómica al destinatario ni lectura confirmada.
- **Recursos:** se mantiene el límite de G1 y se añade una propuesta acotada por recibo. Los ensayos tienen plazo externo de 60 segundos por proceso. El cupo de salida del observador se coteja tras capturar; no es una medida de RSS ni garantía material global. Los conductores públicos deben acotar la lectura antes de proporcionar el slice a la API.
- **Identidad material:** los conductores nuevos usan ceros explícitos para los metadatos declarativos de realización, sin presentarlos como autenticación; la campaña conserva las huellas reales de fuentes y binarios. Los tests anteriores G1 y del lote reciben esas huellas reales; los nuevos tests unitarios de la frontera usan metadatos ficticios explícitos. No se confunde una declaración de huella con identidad profesional.
- **Generalización y repetición:** dos recibos diferentes con idéntico cuerpo pueden superar una comparación literal del cuerpo; la identidad devuelta es la del recibo seleccionado por el conductor. No se acredita una protección universal contra reproducción de mensajes.

No se han abierto reserva, modelos, P3 reservado, P4/P5, WebAssembly, R1 productivo, interfaz profesional ni nuevos perfiles de dominio. La compilación se limita a la candidata nativa; no se modifica el núcleo productivo.

## 6. Encaje, diagnóstico y continuación

Este documento es la pieza sucesora acotada de RETP-148 para la frontera literal. Conserva el workflow V2 y su secuencia. No convierte una biblioteca opcional en garantía nuclear: demuestra una operación mínima y sus límites antes de cualquier integración obligatoria o promoción. No declara terminado el recorrido completo.

Las variantes nuevas de fallo nacen en el comprobador Rust. [Causas y localización pendiente](CAUSAS_Y_LOCALIZACION_PENDIENTE.md) recoge el insumo del catálogo y la limitación diagnóstica de la CLI. No se declara cerrado el punto 2 por tener una lista de mensajes.

La siguiente obligación es la correspondencia entre esta referencia conservada y una transformación/presentación autorizada dentro del alcance existente, manteniendo las compuertas de trazabilidad, seguridad y viabilidad. Si la transformación no tiene regla constituida, no se acreditará equivalencia por una explicación convincente de la IA. La aceptación productiva final sigue siendo humana.

## 7. Cortes y fuentes consultadas

Lenguaje: `2e9dc3b216035725521a850b8157dc7d736fe57c`, rama `main`. Laboratorio: `4e681a9c88ebb66272cbb5c3ab9df2aed4dc3704`, rama `lab/playground-sv-permanente`.

Se cotejaron ocho fuentes de continuidad y 45 archivos existentes de la candidata contra los blobs del corte de Lenguaje. Se conservaron las lecturas completas de pilares, acta de perfiles y ensamblaje, y acta de conformidad IMM con sus adendas rectoras; no cambiaron sus bytes. También se aplican el workflow V2, RETP-142/144/145/147/148, la adenda CYB y el contrato de diagnóstico/localización ya incorporados al expediente. La asignación de autoridad humana vigente procede de la aclaración conversacional del autor.

[Identidades cotejadas](FUENTES.json) · [Fuentes reproducibles completas](FUENTES_REPRODUCIBLES.json) · [Procedencia del compilador](COMPILADOR.json) · [Huellas de los artefactos compilados](REALIZACIONES.json) · [Manifiesto de la entrega](MANIFIESTO.json)
