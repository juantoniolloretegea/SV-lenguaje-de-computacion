# Suficiencia del soporte y validez del ensayo

27 de octubre de 2026 · Versión 1.0 · Preparación experimental; no acredita un examen realizado.

Un ensayo debe permitir que aparezca la conducta que pretende estudiar y que el candidato la reconozca y la corrija materialmente. La protección del equipo y la validez de la prueba son obligaciones simultáneas. Si una barrera elimina de antemano el fenómeno, el resultado sólo demuestra el efecto de esa barrera: no demuestra capacidad del candidato. Si el soporte no permite observar un requisito necesario, debe resolverse esa carencia o cambiarse de soporte; no se suprime el requisito para obtener un dictamen favorable.

## Contrato previo y comprobación independiente

Antes del examen se fijan la versión del caso, sus requisitos necesarios, el efecto que se observará, el escenario inicial, las operaciones legítimas que deben conservarse, la instrumentación, las condiciones de parada y el criterio de recepción. Se separan tres resultados: actuación del candidato, funcionamiento del banco y eficacia de las protecciones exteriores. Ninguno sustituye a los demás.

El banco debe superar estas comprobaciones antes de recibir respuestas del candidato:

1. **Caso defectuoso reproducible.** El defecto produce el efecto esperado en un objetivo sintético propio. La barrera exterior protege al resto del sistema sin corregir silenciosamente ese defecto.
2. **Solución de referencia.** Una corrección conocida elimina el efecto improcedente y conserva las operaciones legítimas. Cerrar todos los servicios o denegar todas las entradas no satisface ese requisito.
3. **Variantes reservadas.** El corrector distingue casos defectuosos y corregidos que el candidato no ha visto. El conjunto y sus huellas quedan fijados; no se ajustan después de conocer la respuesta.
4. **Efecto comprobado.** Se conserva la propuesta y la modificación realmente aplicada, con resultados anteriores y posteriores. El relato del candidato no se acepta como prueba de ejecución.
5. **Observación suficiente.** Se comprueban vigencia, duración, pérdidas, resolución, multiplexación y errores de cada medida necesaria. La cantidad de herramientas instaladas no sustituye esta recepción.
6. **Parada y recuperación.** La autoridad externa puede detener el recinto del ensayo y comprobar sus procesos, conexiones y escrituras. Si interviene antes de terminar una obligación necesaria, se registra la interrupción; no se concede al candidato el resultado conseguido por el supervisor.

La falta de una condición del banco deja el intento sin dictamen válido sobre esa obligación. El incumplimiento del candidato, cuando el banco sí es válido, se evalúa conforme al contrato. No se ocultan casos pendientes dentro de una media ni se anuncia aptitud general a partir de los que resultaron posibles. Una edición posterior con otro alcance necesita identificación y autorización propias; no modifica retrospectivamente el examen.

## Distribución material sin pérdida de alcance

| Necesidad | Soporte que debe acreditarla | Condición de recepción |
|---|---|---|
| Desarrollo, revisión, custodia y observación del servicio actual | Portátil y almacenamiento de trabajo recibido | Continuidad del SV y de los datos; instrumentación acotada. El servicio público no se convierte en objetivo de ataque. |
| Instrucciones y memoria de un programa sintético | Emulador o instrumentación del proceso, con arquitectura declarada | Instrucciones, registros y accesos cotejados con un resultado independiente. No se atribuyen al bus físico del anfitrión. |
| Efectos de solicitudes, permisos, concurrencia y correcciones | Réplicas propias cliente–servidor, WebAssembly o PHP | Efectos reales dentro de la réplica, operaciones legítimas conservadas y ausencia de acceso al SV en servicio. |
| Caché física y canal temporal compartido | Soporte que exponga el recurso físico pertinente y cuya utilización esté permitida | Señal sintética, referencia temporal, resultado antes/después, ruido y repetición de variantes. Una simulación permite preparar el caso, pero no recibe esta obligación física. |
| CPU, memoria y acelerador remotos | Instancia concreta y herramientas compatibles con su arquitectura | Inventario efectivo, permisos, contadores realmente expuestos y procedencia. El nombre comercial de una oferta no acredita estas medidas. |
| Señales eléctricas de buses, DMA o firmware no expuestas | Instrumentación material o trazas competentes específicas | Resolución, puntos observados y cobertura demostradas. Ni una máquina virtual ni un inventario de software equivalen a una medición eléctrica. |
| Inferencia del candidato alojada por un proveedor | Evidencia del entorno que ejecuta esa inferencia | La medición de un cliente o de una réplica remota no permite atribuir contadores internos al proveedor del modelo. |

El portátil continúa siendo parte del trabajo. Añadir otro soporte permite asignar a cada uno operaciones adecuadas, pero no convierte automáticamente una nube compartida en un laboratorio físico exclusivo. Deben conocerse el hipervisor, el grado de dedicación, los permisos y los recursos realmente compartidos. No se desactivan las protecciones del portátil para forzar una equivalencia que no existe.

## Instrumentación y perturbación experimental

Una prueba instrumental breve sirve para determinar qué eventos se reciben y cuánto volumen generan; no concede aptitud al candidato. El tamaño del registro y la duración del examen se dimensionan después a partir de esa evidencia, del alcance necesario y del espacio disponible. Alcanzar un límite no autoriza a conservar sólo la parte favorable. Se conserva el intento incompleto y se corrige la causa antes de repetir el caso.

Se distingue recuento agregado, muestreo, traza de eventos e instrumentación de instrucciones. Un informe de contadores con multiplexación no es una secuencia completa de instrucciones. Los eventos descartados por el sistema y las métricas que dependan de ellos quedan sin recibir. Si el análisis requiere un conjunto simultáneo de eventos y el equipo no lo admite, debe cambiarse el diseño de adquisición o el soporte; no se presentan ventanas distintas como una observación simultánea.

La propia instrumentación puede alterar el fenómeno. Se deben cotejar ejecuciones equivalentes con observación mínima y con la instrumentación prevista, conservando configuración, entradas y criterio de comparación. Si la herramienta altera la concurrencia que se examina, no basta su informe para recibir ese comportamiento. Por ejemplo, [ROCm Compute Profiler](https://rocm.docs.amd.com/projects/rocprofiler-compute/en/develop/what-is-rocprof-compute.html) documenta repetición y serialización de operaciones para recoger determinados contadores; sus resultados no se presentan como la concurrencia ordinaria del programa.

## Soportes remotos y condiciones de utilización

AMD Developer Cloud y UpCloud son alternativas por evaluar para operaciones concretas. No se publican saldos, identificadores de cuenta o accesos en este documento. El expediente restringido conserva la comprobación económica vigente, el vencimiento, las prestaciones elegibles y los recursos ya existentes. No se crea una instancia sólo para reservar crédito mientras se prepara el banco.

En AMD, la [documentación de virtualización de uProf](https://docs.amd.com/r/en-US/57368-uProf-user-guide/Virtualization-Support) condiciona las capacidades de observación a los contadores expuestos por el hipervisor. La [oferta de Developer Cloud](https://www.amd.com/en/developer/resources/cloud-access/amd-developer-cloud.html) no constituye recepción de esos contadores. Sus condiciones indican además que apagar una instancia no detiene su consumo y que el crédito promocional no cubre todos los servicios auxiliares; la retirada se prepara después de recuperar los resultados necesarios.

Las condiciones de uso también delimitan las pruebas posibles. La [FAQ de DigitalOcean](https://www.digitalocean.com/trust/faq) admite determinadas pruebas sobre recursos propios sin perjuicio a terceros; su [política de uso](https://www.digitalocean.com/legal/acceptable-use-policy), revisada el 20 de marzo de 2026, exige consentimiento escrito previo para investigación de seguridad. Esa diferencia impide presumir autorización para un examen adversarial por el mero acceso a una instancia. Las [condiciones de UpCloud](https://upcloud.com/terms-of-service/) prohíben accesos no autorizados, interferencias y elusión de límites; tampoco acreditan por sí mismas aislamiento físico o acceso a contadores.

La recepción de un soporte requiere conciliar el caso concreto con las condiciones aplicables. Este estudio no envía comunicaciones, no contrata recursos ni autoriza pruebas sobre infraestructura del proveedor o de terceros. La proximidad de la caducidad no modifica los requisitos científicos ni concede permisos de ejecución.

## Recepción y retorno

La [ficha del portátil](asus-vivobook-x515ea-core-i7-1165g7/FICHA.md) registra las correcciones instrumentales realmente comprobadas y las carencias que permanecen. La selección de otra plataforma se hará por su capacidad efectiva para resolver esas carencias. Todo resultado debe identificar versión de soporte, caso, evidencia, omisiones y consecuencias, conforme al [criterio obligatorio del SV](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/estandar-comparativo-o-benchmark/CONSECUENCIAS-Y-FUENTES.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
