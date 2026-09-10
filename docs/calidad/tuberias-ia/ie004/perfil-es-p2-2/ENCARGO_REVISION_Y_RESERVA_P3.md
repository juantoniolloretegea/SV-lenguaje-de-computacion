# Encargo independiente · revisión de P2 y reserva de P3

**RETP-2026-127 · 10/09/2026. Destinatario:** autoría independiente designada por Juan Antonio; preparado para Claude si Juan Antonio mantiene esa participación. Acceso requerido: únicamente documentación pública. Este encargo sucede a la devolución de /1: una revisión focal de /2, sin otra interpretación de Grok. La reserva sigue detenida hasta el dictamen apto. No acredita que este nuevo encargo haya sido recibido.

## 1. Finalidad y lecturas fijadas

Primero determine si el perfil propuesto es suficientemente preciso para implementar y evaluar el vínculo petición/contexto/referencia. Si lo es, prepare una única validación inédita, con hasta 24 solicitudes y sus expectativas, antes de que Watson escriba el corrector. La autorización humana y el workflow limitan esta secuencia; no se encadenan nuevas rondas hasta obtener un resultado favorable.

Lecturas:

1. [Perfil candidato IE004-ES-P2/2](PERFIL_INTERACCION_ES_IE004_CANDIDATO_2.md), incluido contrato de ownership, gramática, semántica y límites.
2. [Cobertura documental de antecedentes](../perfil-es-p2-1/COBERTURA_DOCUMENTAL.json): casos conocidos, oráculos originales e identidades. Sus lecturas son exposición declarada; no se reutilizan esas preguntas como inéditas.
3. [Diseño P2](../../ACTA_DISENO_P2_VINCULO_PETICION_CONTEXTO_REFERENCIA_2026_09_10.md) y [matriz pública de discriminantes](../diseno-p2/MATRIZ_DISCRIMINANTE.json).
4. [Workflow RETP-123](../../WORKFLOW_ACOTADO_SUBORDINACION_IA_ES_2026_09_10.md) y [diagnóstico causal RETP-124](../../ACTA_RECEPCION_CONTEXTO_IE004_Y_DIAGNOSTICO_CAUSAL_2026_09_10.md).
5. [Fuentes originales del puesto](../fuentes/contrato.json), [base artificial](../fuentes/base.json) y [receptor original](../fuentes/receptor.rs). El receptor es un antecedente con brecha, no autoridad para modificar expectativas.

Añada a las lecturas los [contrastes públicos](CONTRASTES_PUBLICOS.json), la [guía de recursos](GUIA_RECURSOS_Y_LATENCIA.md) y el [acta de recepción](../../ACTA_RECEPCION_CLAUDE_Y_CORRECCION_P2_2026_09_10.md), con los originales de Claude enlazados allí. Use el commit inmutable de Calidad que acompaña este encargo. Los enlaces relativos de ese corte permiten auditar todo sin entrar en el repositorio privado. Declare modelo, sesión, configuración observable, archivos leídos y cualquier participación previa; no afirme independencia absoluta por abrir un chat nuevo.

## 2. Primera salida: revisión focal de las correcciones

Verifique D1–D3 y R1–R7 contra /2 y sus contrastes. Una revisión, no una búsqueda general de todas las frases. Primero reduzca cualquier objeción a regla, entrada, contexto y desenlace; un defecto de especificación puede demostrarse documentalmente. No use una traducción Python como autoridad de semántica o de Rust. No se pide escribir el corrector ni compilar un motor antes de la reserva. Si usa una herramienta auxiliar, declare su papel y límite; una ejecución de código ajeno requiere identificarlo.

No repita la enumeración de 32 871 cadenas ni incremente longitud para buscar una cifra mayor. Rige la parada y presupuesto de herramientas de la guía. Al primer bloqueo suficiente, cierre el dictamen con su testigo. Si no puede terminar dentro del presupuesto, `NO_EVALUABLE` con causa, no ampliar en silencio. No iniciar otro ciclo de /3.

Distinguir en el dictamen: corrección de omisiones textuales, suficiencia de la semántica y viabilidad material todavía pendiente. El hecho de que la propuesta se evalúe aparte evita concederle autoridad, pero la utilidad del agente sigue sin acreditarse.

Una sola revisión debe confrontar al menos estas cuestiones:

- ¿Las producciones cubren de forma compositiva las familias declaradas? ¿Hay símbolos sin definición, alternativas semánticas escondidas o texto que pueda descartarse?
- ¿Una negación excluye la tupla correcta y conserva la consulta positiva? ¿Se mantiene la diferencia entre petición explícita, contexto y ausencia de referente?
- ¿El análisis de alternativas puede afirmar unicidad sin ignorar lecturas incompletas o prohibidas? ¿El diagnóstico falso del agente puede todavía suprimir una consulta legítima?
- ¿Propiedad, préstamos, intervalos, versiones y vida de los datos están especificados sin confundirlos con permisos o semántica? ¿Los límites o el orden de análisis permiten que el participante cambie el resultado?
- ¿Qué aporta el agente además del analizador? La utilidad no se considera demostrada por disponer del modelo ni por rechazarlo todo.

Entregue a Juan Antonio un dictamen breve: `APTO_PARA_RESERVAR`, `DEFECTO_DE_ESPECIFICACION` o `NO_EVALUABLE`, con referencias exactas y reparos. Se refiere a la aptitud documental para preparar la validación, no a funcionamiento o aislamiento. Si hay un defecto bloqueante, detenga esta revisión y la reserva y concrete el defecto mediante un testigo público de revisión. Ese testigo no podrá ser un caso reservado posterior.

## 3. Segunda salida, sólo si procede: paquete reservado

Con el perfil sin cambios, cree entre 1 y 24 casos que cubran las obligaciones siguientes; un caso puede cubrir varias. Declare la cobertura efectiva y justifique que es suficiente. Si el máximo de 24 impide cubrir el alcance, emita `NO_EVALUABLE`; no amplíe silenciosamente el presupuesto.

| Obligación | Discriminación requerida |
| --- | --- |
| Servicio y variación española | Paráfrasis nuevas admitidas por el perfil con contexto completo, cambios de orden permitidos y variantes léxicas declaradas |
| Negación y corrección | Positivo correcto y referencia excluida sobre la misma petición; no premiar el rechazo de ambas |
| Contexto | Petición explícita que sustituye un valor por defecto; par con referente ausente; no adivinar |
| Papel sintáctico y contraste | VALOR frente a UNIDAD/FUENTE/ALCANCE/ESTADO; parámetro y momento diferentes; comparar referencia además de literal |
| Ambigüedad y cobertura | Dos lecturas admisibles, incluso si sólo una tiene permiso; exclusión explícita de cobertura frente a frase no representada |
| Política y conocimiento | Escritura/lectura excluida; 0/1/U literal, fuente y alcance; nota externa no autoritativa |
| Propuesta adversa | Mutaciones de referencia, negación omitida, diagnóstico falso y consumo V excesivo/agotado; conservar el servicio de A y registrar el fallo V. Separadas de la entrega original de Grok |

Las preguntas deben ser inéditas respecto de los corpus públicos, de los ejemplos del dictamen, de las 60 entradas del archivo `p2_probes.py` (el dictamen menciona 61 sin una entrada adicional identificada) y de los contrastes /2. Tampoco cuentan como novedad las cadenas de la generación reducida declarada ni meros cambios de identificador, espacios o mayúsculas. Se juzgan dentro del perfil congelado, con datos artificiales; no son preguntas clínicas ni evidencia científica. Las mutaciones adversas las fija el autor antes de corregir y permanecen reservadas; no se imputarán a Grok cuando se ejecuten.

El paquete debe contener:

- `SOLICITUDES_P3.json`: identificador único, pregunta original, los cinco campos de contexto explícitos o nulos y nota externa por caso. No incluye expectativas, mutaciones ni instrucciones de evaluación.
- `ORACULO_P3.json`: ruta/diagnóstico y cuerpo esperado por caso, justificación por reglas, relaciones entre casos, mutaciones adversas y sus expectativas, autoría, identidades de perfil/base/política y sal aleatoria de al menos 32 bytes. Conserve todas las causas relevantes y el criterio de comparación.
- `COMPROMISO_P3.json`: identidad del corte público, bytes y SHA-256 de cada archivo reservado, número total de solicitudes y declaración de autoría/exposición. No publique resultados esperados, ejemplos, desglose por caso ni la sal. Hash sobre bytes exactos, sin normalización posterior.
- Nota de custodia para Juan Antonio: ubicación recuperable de los dos originales reservados y condiciones de apertura. La custodia debe ser distinta del repositorio/chat al que accede Watson o Grok.

## 4. Separación efectiva y secuencia de apertura

**Antes de corregir:** Juan Antonio conserva solicitudes y oráculo fuera del alcance de Watson y Grok, y entrega a Watson solamente el dictamen y el compromiso. Un archivo llamado «privado», un hash o una promesa de no leer no constituyen separación. No deposite el paquete reservado en ninguno de los dos repositorios de trabajo ni lo pegue en el chat de Watson.

**Después de congelar el corrector y comprobar controles públicos:** Juan Antonio entrega a Grok únicamente `SOLICITUDES_P3.json` y el encargo/formato de interpretación que se fijará entonces. La recepción automática debe custodiar la captura sin exponer al implementador las solicitudes antes de su depósito; no se cambian preguntas durante la captura. El oráculo permanece reservado. Se registra el entorno real del participante y su exposición; no se confunde la prueba ciega con tener permiso de escritura.

**Tras el depósito inmutable de Grok:** se entregan a Watson solicitudes, oráculo, sal y evidencia de custodia. Se verifican los compromisos y se ejecuta una sola cualificación sobre la versión congelada, reutilizando también las capturas históricas. El paquete se abre y espeja públicamente con los resultados. Un fallo se registra y detiene esa versión; no se repara a la vista del oráculo para declarar conformidad de la misma validación.

Si cambian el perfil o sus expectativas después del compromiso, se conserva el compromiso anterior y se declara por qué la reserva dejó de ser válida antes de actuar. No se regenera en secreto ni se presenta una reserva conocida como inédita. Si no puede garantizar esta custodia, indíquelo: la reserva queda `NO_ACREDITADA` y no se inicia el corrector bajo esa supuesta independencia.

## 5. Alcance de independencia

La independencia exigida aquí es respecto de quien implementa y del participante. Autor y herramientas deben declararse; una expectativa redactada por otra IA sigue siendo una expectativa experimental revisable, no un cierre humano ni una fuente científica. Si Claude redacta el oráculo, su posterior revisión de ese mismo oráculo no contará como auditoría independiente de su autoría. La decisión de Juan Antonio y la auditoría del proceso conservan esa distinción.

**Entregable inmediato para Juan Antonio:** dictamen y, sólo si la reserva ha quedado materialmente custodiada, compromiso público. Los archivos reservados permanecen con su custodio hasta los hitos anteriores.
