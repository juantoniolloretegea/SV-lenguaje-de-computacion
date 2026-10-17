# Clasificación de hardware del Sistema Vectorial SV

**SV-CLASIFICACION-HARDWARE-1 · 17 de octubre de 2026**

La [clasificación pública](https://administracion.itvia.online/benchmark/hardware) contrasta prestaciones declaradas y capacidades observadas. CPU, GPU, TPU y NPU designan clases de procesamiento, no niveles automáticos de superioridad. Una ficha describe una configuración, un programa, una carga y unos límites de observación concretos.

## Tres niveles de evidencia

1. **Declaración del fabricante.** Identificador, revisión, memoria, precisión, potencia y rendimiento anunciados, con fuente oficial y fecha de consulta. TOPS o TFLOPS con distinta precisión, densidad o número de dispositivos no son magnitudes intercambiables.
2. **Comprobación del SV.** Identidad observada, interfaces disponibles, ejecución física atribuible, resultado cotejado y medidas conservadas. Reconocer un dispositivo no acredita que haya calculado. Una interfaz no disponible tampoco demuestra ausencia física del componente.
3. **Comparación recibida.** Sólo una campaña común, fijada antes de ejecutar, permite asignar posiciones dentro de su grupo. Los antecedentes bajo tareas distintas no se convierten en competidores mediante normalizaciones posteriores.

Las fichas iniciales de MI300X VF, dos T4, TPU v5e y un equipo Android conservan resultados físicos delimitados y sus incidencias. No constituyen una clasificación universal de velocidad ni una evaluación de modelos de IA. Los informes originales permanecen en sus carpetas; cada ficha identifica la revisión y la huella del informe utilizado.

## Contrato previo de comparación

Antes de cualquier prueba se reciben finalidad, tarea y entradas por huella; referencia correcta y tolerancia; precisión y cuantización; configuración física o virtualizada; cantidad de dispositivos; concurrencia; programa, bibliotecas y controladores; etapas temporales; modalidad de arranque; repeticiones; instrumentación, permisos y conservación. Se fijan también límites y condiciones de detención. Las diferencias entre plataformas se declaran y no se ocultan mediante una puntuación agregada.

Cada grupo debe tener una métrica primaria común y sus criterios de calidad. Se distingue la primera ejecución de las posteriores. Las series conservan observaciones, mediana, dispersión y extremos. Una incidencia instrumental no se convierte en resultado de cálculo ni en ausencia de capacidad.

## Orden y límites

| Finalidad fijada | Orden dentro del mismo grupo | Condiciones |
|---|---|---|
| Una tarea fija | Menor mediana del tiempo completo | Mismo trabajo correcto, mismas fases y condiciones recibidas |
| Capacidad de trabajo por lotes | Mayor mediana de tareas terminadas por segundo | Misma calidad, concurrencia declarada y exigencia de latencia |
| Energía por tarea o rendimiento por vatio | Sólo clasificación separada recibida | Medición atribuible, intervalos y fronteras comunes; no potencia nominal |

No se mezclan estas clasificaciones. Una posición describe la magnitud recibida: no acredita por sí sola superioridad estadística ni universal. Los empates se conservan; no se desempata por nombre, fabricante, recursos aportados ni descuentos. Una serie incompleta, no recibida o sin una magnitud imprescindible no obtiene posiciones finales. Se mantienen visibles sus incidencias y la causa que impide clasificarla.

Para la separación de tiempo, capacidad de trabajo, calidad y configuración se toman como referencias [SPEC CPU](https://www.spec.org/cpu2017/Docs/overview.html), sus [reglas de ejecución](https://www.spec.org/cpu2017/Docs/runrules.html) y las [condiciones de MLPerf Inference](https://docs.mlcommons.org/inference/submission/). El criterio del SV es propio: no acredita ejecución oficial ni certificación SPEC o MLPerf.

## Árbitro–Director, Explorador y Selector

El Árbitro–Director recibe política, límites y destino permitido. El Explorador obtiene identidad, disponibilidad y memoria observables. El Selector comprueba compatibilidad y disponibilidad para la carga concreta y utiliza estimaciones comparables del recorrido completo cuando existen; si aplica otro criterio, debe identificarlo. No prioriza automáticamente aceleradores ni permite un reparto parcial silencioso.

El adaptador revalida el destino, ejecuta y conserva resultados. El cotejo une decisión, identidad física, cálculo y salida. Las bibliotecas nativas y controladores se declaran: que los componentes propios sean Rust no convierte toda la infraestructura en Rust. El hardware de una API de IA externa lo selecciona su proveedor; no se atribuye al SV control que no tiene.

Referencias: [manual del Árbitro–Director](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/arbitro-director) y [libro de hardware](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/hardware-aceleracion-tensorial).

## Solicitudes de fabricantes y proveedores

El [formulario bilingüe](https://administracion.itvia.online/benchmark/hardware/proveedores) solicita representante y capacidad de representación, entidad, correo, dispositivo, fabricante, plataforma, prestaciones con su fuente, motivo y recursos para la evaluación sin coste para el SV. No admite credenciales ni documentación reservada. Confirmar el correo acredita su control, no la representación legal.

Tras la confirmación se envía copia íntegra y la propuesta pasa a la bandeja privada. Sólo la administración o la función Responsable de evaluar peticiones de Proveedores puede responder o decidir. La trazabilidad conserva autoría, revisión, fecha y contenido. La petición no crea una cuenta, acepta recursos, asigna hardware ni inicia una prueba.

La aceptación para preparar un acuerdo es distinta del acuerdo, la recepción técnica y el lanzamiento. Los recursos y posibles conflictos se declaran antes de evaluar y en el expediente publicable. Ninguna aportación compra una posición o un resultado favorable. Las afirmaciones recibidas del proveedor no se importan como medidas del SV.

## Actualización y lectura

La página presenta las mismas fichas en prosa y en datos estructurados. Los datos personales de solicitudes nunca forman parte de esa proyección pública. La incorporación futura de resultados exige comprobar sus fuentes y su recepción, conservar las versiones anteriores y publicar únicamente el ámbito autorizado. Leer la página o descargar sus datos no ejecuta cargas ni examina el equipo del visitante.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
