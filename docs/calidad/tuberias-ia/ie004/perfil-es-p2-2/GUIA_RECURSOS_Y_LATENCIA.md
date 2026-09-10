# Guía candidata de recursos y latencia · IE-004 · RETP-127

**10/09/2026. Alcance:** guía de laboratorio previa a P3/P5. Es una decisión de presupuesto candidato, no una medición, una selección de modelo o una aptitud productiva aprobada. Desarrolla el mandato de Juan Antonio de controlar el coste ahora sin abrir una comparación general de proveedores, .NET o FFI.

## 1. Qué se compara y qué no

Los aproximadamente 30 minutos comunicados para la revisión de Claude incluyen elaboración del informe y herramientas; no tenemos una descomposición verificada de ese tiempo. No son una muestra de latencia de una consulta inmunológica ni tiempo de ejecución del futuro receptor Rust. Tampoco acreditan recursos insuficientes, agotamiento del modelo o un defecto de Python. La configuración de modelo/esfuerzo es declarada por el usuario o proveedor; no un recurso de cómputo homologado.

La unidad de comparación futura es la misma solicitud, contexto, versiones, respuesta completa, carga y entorno. Se mantienen separados preparación/descarga, compilación, arranque, tratamiento, verificación de propuesta, espera del proveedor y publicación de evidencia. Un hash identifica bytes; no mide coste ni comprueba semántica.

## 2. Actions como techo del laboratorio

La documentación oficial consultada el 10/09/2026 fija **6 horas máximas por trabajo en runners alojados por GitHub**. El valor por defecto de `jobs.<job_id>.timeout-minutes` es 360; puede declararse un límite inferior. Son límites del servicio de CI, no tiempos medios ni objetivos de respuesta humana. No se trasladan al dominio ni a `Tri`.

- [Límites de GitHub Actions](https://docs.github.com/en/actions/reference/limits): lectura de la tabla de duración y ejecución de trabajos; no auditoría de la cuota o plan efectivos de esta cuenta.
- [Sintaxis de timeout por trabajo](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#jobsjob_idtimeout-minutes): lectura del comportamiento de cancelación; no configuración material aplicada por este documento.

**Presupuesto candidato de CI para la realización P3:** máximo 20 minutos por trabajo/destino, con límites internos de 8 minutos para preparar/compilar, 5 para controles y 2 para exportar evidencia; 5 restantes para orquestación y terminación. Con dos destinos separados, el máximo reservado es 40 minutos de trabajo, aunque el tiempo de pared sea distinto. La cola de GitHub se registra aparte. La comparación de rendimiento P5 usa su propia ficha acotada, conservando los 30 bloques pareados propuestos en RETP-123; no se fuerza a caber ocultando muestras.

En un timeout se preserva el punto alcanzado y no se declara éxito. No hay reintento automático por fallo funcional ni ampliación automática del plazo. El ejecutor local debe imponer límites equivalentes; la cualificación no puede depender exclusivamente de que GitHub mate el trabajo. No se modifica ni se lanza un workflow en esta recepción documental.

## 3. Envolvente de consulta candidata

Estos valores son **una hipótesis de servicio interactivo para el laboratorio**, propuesta por Watson antes de medir. No proceden de GitHub ni de un estudio clínico y no se anuncian como alcanzados. Se evalúan sobre datos artificiales IE-004 y equipo identificado; un cambio exige versión y causa antes de nuevas mediciones.

| Magnitud | Objetivo candidato | Medición y límite |
| --- | --- | --- |
| Respuesta canónica completa, desde admisión local de solicitud | Media ≤ 2 s y p95 ≤ 5 s; plazo material máximo 10 s | Incluye todo componente que esté realmente en el recorrido de respuesta; anotar colas y latencia del transporte exterior por separado |
| Tratamiento confiable A, proceso caliente | p95 ≤ 100 ms; plazo material por solicitud de 1 s | Medir en el Rust compilado de cada destino; separar lectura/serialización si el instrumental permite y conservar el total |
| Proveedor de interpretación cuando se ensaye en el recorrido | Plazo máximo 5 s por solicitud; una propuesta, sin reintentos automáticos | Si no hay API/instrumentación autorizada, se declara no medido; no se inventa un tiempo por dividir la duración de un chat entre sus casos |
| Comprobación V, fuera de la ruta que entrega el cuerpo en /2 | Plazo máximo 1 s y cupo de trabajo V del perfil | Su timeout no cambia el cuerpo ni consume plazo/cupo A; registrar también su coste total |
| Concurrencia inicial | Una solicitud activa por instancia del banco | Define la carga inicial, no representa carga productiva ni acredita resistencia a ataques concurrentes |

El arranque frío se mide y publica separado; no se oculta dentro de una media de tratamiento caliente. Los plazos materiales requieren imposición por supervisor confiable: Rust no impone un deadline por existir ownership y WASM no lo impone por estar compilado. La realización concreta, su cancelación efectiva y sus dependencias quedan para P4.

En /2 la resolución A no espera al modelo; alcanzar estos tiempos demostraría el coste de esa ruta, **no la latencia ni utilidad de colaboración con IA**. La futura prueba de aportación deberá identificar qué trabajo útil realiza el modelo y si introduce espera, consumo o una pérdida de servicio. Si no aporta una mejora verificable bajo el contrato, esa colaboración no queda acreditada. No se cambia ahora a otro normalizador ni se elige Qwen, GPT, Claude o Grok como dependencia productiva.

## 4. Ficha de medida antes de ejecutar

Fijar commit/fuentes, compilador y flags, dependencias, hashes de binarios, destino, runtime WASM, CPU/RAM/SO, carga, condición fría/caliente, instrumental y límites. Identificar claramente qué partes se pudieron observar. Los límites A/V del perfil se cuentan separadamente; CPU/RSS y memoria lineal se miden cuando el instrumental lo permita, sin sustituir ausencias por cero.

Conservar todas las muestras, número de solicitudes, éxitos debidos, rechazos, fallos, timeouts y cancelaciones. Media y p95 se calculan sobre observaciones completadas, declarando cuántas quedaron censuradas; los timeouts siguen siendo incumplimientos y no se eliminan para mejorar las métricas. Para p95 usar el valor de orden `ceil(0,95*n)` de las muestras ordenadas y declarar n; con 30 bloques su precisión es limitada. No sumar muestras heterogéneas ni atribuir a Rust el tiempo del lanzador Python/Node, aunque sí conservar el coste total del conjunto.

P5 compara referencia y candidata bajo la misma carga/entorno, con el diseño pareado de RETP-123. Las capturas viejas y sus binarios se conservan como checkpoints; no demuestran regresión por sí solos ante un trabajo nuevo. La tolerancia relativa de regresión se fijará con una referencia comparable antes de medir; hoy no está acreditada esa comparabilidad. No se promueve viabilidad por cumplir sólo el tope de un job.

## 5. Parada acotada de esta revisión

Una corrección documental sucesora /2, una revisión focal de D1–D3 y los reparos resueltos, y reserva sólo si el contrato resulta suficiente. Detener la búsqueda general de cadenas al disponer de un defecto bloqueante documentable. Las comprobaciones de herramientas de esa revisión tendrán límite explícito, proponiéndose 5 minutos por proceso y 10 acumulados; las comprobaciones omitidas se declaran. Estos topes no afirman controlar desde aquí el tiempo de razonamiento de un chat de otro proveedor.

Un nuevo bloqueo devuelve causa y contraejemplo; no desencadena otra colección de sinónimos ni una nueva ronda Grok. La cualificación P3 sigue siendo una captura independiente de hasta 24 casos tras reserva y congelación. P4/P5 y la auditoría P6 conservan sus puertas. Refactorización sólo por necesidad causal demostrada y con el checkpoint previsto; no por el nombre o la velocidad nominal de un modelo.
