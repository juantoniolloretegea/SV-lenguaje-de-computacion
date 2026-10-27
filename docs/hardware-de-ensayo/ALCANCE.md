# Hardware y plataformas de ensayo del SV

27 de octubre de 2026 · Versión 1.0.

Este apartado documenta el soporte material y las condiciones de observación, aislamiento, parada y recuperación de los ensayos. Una ficha identifica una configuración y la evidencia disponible; no concede por sí sola autorización para ejecutar una prueba.

## Relación con el cómputo y la aceleración tensorial

El apartado de [hardware de cómputo y aceleración tensorial](../hardware-de-aceleracion-tensorial/readme.md) relaciona Explorador, Selector, dispositivo y cálculo observado. Incluye CPU, GPU, TPU y NPU: no se limita a aceleradores dedicados. Sus [criterios de clasificación](../hardware-de-aceleracion-tensorial/clasificacion/criterio-v1.md) separan declaraciones del fabricante, comprobaciones del SV y comparaciones recibidas.

La finalidad de este apartado es distinta: determinar sobre qué plataforma puede realizarse un ensayo concreto, con qué cobertura de observación y bajo qué restricciones. Un mismo procesador puede participar en ambos ámbitos. Se enlaza su evidencia existente sin duplicar ensayos ni convertir una detección o una medición térmica en una prueba de aceleración tensorial.

| Elemento | Sede y función |
|---|---|
| Identificación, selección y cálculo del dispositivo | Hardware de cómputo y aceleración tensorial; fuentes y capacidades comprobadas para cada carga. |
| Configuración del equipo y condiciones del ensayo | Este apartado; límites materiales, instrumentación, incidencias y recepción previa. |
| Protocolo y resultado de un modelo | Expediente de evaluación del modelo; referencia a una revisión concreta de la plataforma utilizada. |
| Trazas íntegras y datos del sistema | Custodia restringida; una ficha pública no autoriza su difusión ni su entrega al modelo. |

La exploración del hardware no sustituye al supervisor del ensayo. Las [condiciones de integración del Explorador](../evoluciones-desde-el-nucleo-no-cotejadas-generadas-por-pruebas-con-la-ia-sin-aprobacion-aun/EXPLORACION-Y-SELECCION-DE-HARDWARE-2026-10-15.md) conservan su alcance experimental. Este apartado no modifica el Lenguaje, el Núcleo ni la IR.

## Plataformas documentadas

- [ASUS VivoBook X515EA/F1500EA, Intel Core i7-1165G7](asus-vivobook-x515ea-core-i7-1165g7/FICHA.md): observación pasiva del equipo y demostración x86 emulada; perímetro de examen adversarial no recibido.

Las fichas separan el equipo observado, el objetivo de la prueba, el ejecutor de las acciones y el observador. Si el modelo se consulta mediante una API externa, el hardware de inferencia pertenece al ámbito de su proveedor y no se atribuye al ordenador cliente. Ninguna observación local acredita el control físico de ese hardware remoto.

## Correspondencia obligatoria con cada ensayo

El expediente debe vincular la identidad y revisión del candidato, versión del protocolo y caso, configuración exacta de la plataforma, programa e imágenes por huella, autoridad de ejecución, límites recibidos, observación efectiva y resultado independiente. La revisión de la ficha queda fijada antes de ejecutar; una modificación relevante exige comprobar de nuevo su aplicabilidad.

Se conservan por separado los fallos del candidato, las incidencias de instrumentación y la eficacia de las barreras externas. Una medida obligatoria ausente impide recibir el caso. La prosa de un modelo no acredita que una acción se haya ejecutado, y la protección ejercida por el supervisor no se contabiliza como una capacidad del candidato.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
