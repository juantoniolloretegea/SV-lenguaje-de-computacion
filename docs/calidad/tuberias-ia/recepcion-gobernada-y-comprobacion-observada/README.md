# Recepción gobernada y comprobación observada

**RETP-2026-154 · 12/09/2026 · Candidata aislada conforme en recepción técnica y comparación exacta. Admisión profesional NO ACREDITADA.**

Se han realizado dos mecanismos en Rust: recepción del acto y plan previamente fijados bajo la premisa opaca existente, y producción de una comprobación individual desde una comparación de bytes ejecutada. La entrada de la IA no instala la confianza, no elige el plan de génesis y no aporta el resultado técnico a acreditar.

**231/231 pruebas unitarias pasan en depuración y optimización:** las 217 de la candidata RETP-153 más 14 nuevas. Se conserva íntegra su corrección de pertenencia entre continuidades. Son pruebas nativas del crate, no toda la conformidad del repositorio. La biblioteca ordinaria compila sin `cfg(test)`; un cliente público compila y se ejecuta. Tres clientes adversariales se rechazan por cuatro errores esperados: dos E0624, uno E0451 y uno E0616.

## Qué debía ocurrir y qué ocurrió

| Obligación | Resultado observado | Alcance |
| --- | --- | --- |
| Recibir exclusivamente el acto fijado | T-0 constituye el plan; acto, emisor declarado y versión se conservan | Premisa y plan sintéticos del banco |
| Rechazar alteraciones antes de T-0 | Acto, emisor declarado o versión distintos se rechazan; el original sigue siendo recibible | Igualdad con la instalación; no autenticación del emisor físico |
| Impedir una segunda continuidad desde el mismo preparado | Segundo intento rechazado | Objeto intraproceso, sin restauración durable |
| Preservar errores constitutivos | Plan vacío devuelve `Genesis(EmptyInitialForms)`; no se entrega continuidad | Intento T-0 fallido terminal, sin reparación |
| Calcular la comprobación | Igualdad → Accredited; diferencia → Refuted; ausencia → NotVerifiable | Obligación específica de igualdad exacta, nunca verdad profesional |
| Conservar evidencia y pertenencia | Se conservan contrato, referencia y bytes observados, unidos a la continuidad prestada | La comprobación interna no puede extraerse por la API pública |
| Evitar mezcla entre constituciones idénticas | Descriptor y aplicabilidad de otra continuidad se rechazan aunque sean iguales por contenido | Identidad de los objetos prestados en Rust seguro |
| Impedir que igualdad acredite autoridad | El comparador rechaza obligaciones `Core` | No resuelve los verificadores nucleares pendientes |
| Acotar copias de entrada | Vacíos prohibidos y tamaños excesivos se rechazan; el límite exacto se admite | 65536 bytes por campo de este montaje |

Los accesores y el campo de resultado nominal siguen siendo observables; no son una conversión a `Permit`. La representación de fallo continúa separada de `Tri.U`.

## Decisión sobre el enlace

La candidata concreta **cómo recibir sin sustituir** una autorización ya admitida y **cómo obtener una comprobación desde una operación ejecutada** para un caso técnico cerrado. No completa la admisión externa: `ExternalGenesisPremise` continúa sin productor profesional disponible en la superficie examinada. Su constructor de prueba no se publica. Tampoco se ha instalado un verificador profesional que produzca los requisitos obligatorios de R1.

Un emisor declarado igual al fijado no demuestra quién presentó los bytes. Un documento que diga «autorizado» no produce la premisa. Una comparación exacta no demuestra competencia, vigencia ni corrección científica. El instalador de confianza debe fijar la correspondencia entre acto, plan y operación; esta candidata conserva esa correspondencia recibida, pero no deriva ni valida su significado desde prosa. El [contrato](CONTRATO_Y_FRONTERA.md) sitúa esas obligaciones y sus condiciones de aceptación.

Por ello la candidata **no expone una salida desligada hacia permisos**. La vía documental RETP-152 permanece disponible y la vía profesional permanece inhabilitada. No se declara cerrado el recorrido profesional ni la segunda necesidad completa. Esta es una realización parcial identificada, no otra autoridad ni un segundo núcleo.

## Siguiente objeto único

Completar el contrato de recepción de la premisa externa y de los verificadores obligatorios para el enlace profesional acotado: identificar su evidencia material admitida y el productor competente que la entrega al núcleo, sin inventar una raíz ni asignar esa decisión a la IA. El humano experto ya está identificado como autoridad de dominio; no hay que volver a preguntarle quién concede autoridad ni pedirle que diseñe la comprobación.

Antes de ese enlace no procede añadir otra conversión de etiquetas, promover este montaje como productivo ni abrir pruebas reservadas. Si el objeto material pertenece a una constitución de agente aún no realizada, se registra esa dependencia en su sede y se conserva inhabilitada la actuación concreta. No se adelanta la fase de agentes para hacer pasar el banco. La presente entrega termina la recepción técnica ensayada; no convierte la dependencia material en una garantía disponible.

La aceptación final productiva sigue reservada al humano. No se ha modificado `rust/sv_core` en producción. Los nuevos errores son tipos candidatos locales; su eventual integración requerirá el tratamiento del catálogo y la localización, sin inventar claves canónicas. DFL-005/006 conservan su alcance abierto. P3 reservado, asociación /2–/3, P4/P5 y constitución de dominios/agentes mantienen sus compuertas.

## Custodia y reproducción

Cortes leídos: Lenguaje `dba4d655280ae36172dd601377367a0f89b69010`; laboratorio `7cc917eef089e865af5325318bae911633daa029`. Se cotejaron las rectoras ya leídas —AGENTS, Pilares, perfiles, transición y workflow V2— contra sus blobs actuales. Se leyó el contrato CYB, especialmente §§8.1–8.4, y las superficies de génesis, requisitos y aplicabilidad. Sus huellas figuran en [FUENTES.json](FUENTES.json).

La cápsula [CANDIDATA_FUENTES.json](CANDIDATA_FUENTES.json) contiene las fuentes y los ejemplos incluidos por los tests, completos, con rutas y SHA-256. [CAMBIO_INCREMENTAL.patch](CAMBIO_INCREMENTAL.patch) muestra sólo el incremento respecto de RETP-153; no debe confundirse con un parche contra producción. Los resultados originales están en [RESULTADOS.json](RESULTADOS.json) y [FRONTERA.json](FRONTERA.json). No hubo fallos previos de compilación o aserciones en estos diez procesos; las advertencias del compilador se conservan.

Reproducir con `python reproducir.py /ruta/rustc /ruta/nueva-de-trabajo`. El compilador utilizado se identifica en [COMPILADOR.json](COMPILADOR.json). Python sólo prepara fuentes, ejecuta Rust y recoge resultados; no evalúa SV ni constituye autoridad.
