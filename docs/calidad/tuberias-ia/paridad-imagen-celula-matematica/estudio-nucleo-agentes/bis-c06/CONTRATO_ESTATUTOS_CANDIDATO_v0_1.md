# Contrato candidato de lectura, inicialización y reevaluación

**Versión 0.1 · S22 · BIS-02/C06 · 13 de septiembre de 2026 · Juan Antonio Lloret Egea y Watson**

## 1. Finalidad

Delimitar el estatuto de la actividad técnica registrada, la consulta de un estado, la evaluación inicial y la reevaluación formal. El suceso es el hecho; la prosa, la matemática y la imagen son representaciones. El registro de una actividad no la convierte por sí mismo en dato de transición del horizonte ni acredita la verdad de lo afirmado.

El contrato concreta las premisas del autor y prepara la decisión de sede de BIS-03. No modifica gramática, IR, núcleo, constituciones de dominio ni versiones doctrinales. Sus controles son especificaciones previas: no se ofrecen como operaciones nuevas ejecutadas en Rust.

## 2. Estatutos y resultados exigibles

| Actividad | Resultado y evidencia exigibles | Relación con Frame |
| --- | --- | --- |
| Lectura técnica | Fuente exacta, entrada y salida recuperables, invocación y alcance de lectura | Puede registrarse sin producir Frame ni TransitionData |
| Recuperación original | Mismo objeto histórico y vínculo original; acto de recuperación identificable | No produce una reevaluación por el mero hecho de recuperar |
| Emisión de un consejo documental | Ocurrencia de la afirmación, contenido y respaldo declarado | No acredita verdad, autoridad o decisión formal SV por sí sola |
| Consulta formal del Documento V | Respuesta, justificación reconstruible y metadatos bajo firma; CQ1–CQ6 | Lee o compara lo constituido; no reescribe trayectoria |
| Evaluación inicial | Arquitectura y parámetros constituidos, evaluación aplicable y resultado vinculado a su acto | No exige inventar un frame ni una transición precedentes |
| Reevaluación formal | Horizonte competente, dato suficiente, operador inducido aplicable, evaluación y enlace a su antecedente | Produce el resultado pertinente sin reescribir frames anteriores |

La correspondencia entre frame celular y frame arquitectónico conserva el alcance C03/C04. Un archivo de imagen es una representación; su presencia o ausencia no concede estatuto de evaluación. Una bifurcación de control de un programa no se convierte automáticamente en decisión algebraica SV.

## 3. Inicialización y alcance del constructor existente

El Documento III parte de un frame inicial S₁ para reconstruir la trayectoria; no exige un S₀ ficticio. El contrato de inicialización debe identificar la arquitectura, estado paramétrico, constitución, operación de evaluación y productor efectivo del resultado. La causalidad y trazabilidad de ese acto no se satisfacen inventando un dato de transición anterior. El índice es ordinal; no constituye una primitiva temporal.

En la realización vigente, `Frame::from_candidate` es interno a `sv_core` y comprueba cierre estructural y causal J-F0…J-F5 sobre relaciones resueltas. No dispone de un campo de predecesor. Conserva referencias y no expone mutadores posteriores; exige `criticalities` vacío mientras falte su productor. IR 0.3 §4.7 declara que el cierre no exige exhaustividad.

Por tanto, aceptar una declaración `Frame` no acredita automáticamente evaluación completa de arquitectura, criticidad producida ni una operación ejecutiva inicial. La obligación de vincular el frame con el hecho de evaluación se conserva como contrato pendiente de sede e imposición. No se debilita la validación existente ni se le atribuye una garantía que no ofrece.

## 4. Relevancia y reevaluación

La pertenencia al horizonte procede de su declaración competente. Una lectura de archivo puede aportar evidencia para un suceso formal si satisface además el contrato de captura, admisión y transducción que corresponda; su etiqueta de lectura no realiza esa admisión. K1-T mantiene inhabilitada la producción observación→Tri hasta cerrar sus obligaciones. Tampoco se convierte una ausencia técnica en Tri.U ni en ausencia verificada de un suceso del dominio.

La reevaluación formal requiere distinguir antecedente, dato de transición, operador inducido y resultado de la evaluación. Las comprobaciones locales de `TransitionData` ya presentes no ejecutan ese operador ni prueban su suficiencia causal. La declaración de pares no vacíos, de un horizonte y de posiciones legales es necesaria en su alcance, pero no basta para afirmar que se produjo el nuevo frame.

El régimen estricto del Documento III §6.1 exige cambio constitutivo suficiente para justificar crecimiento. Su §6.2 permite reevaluaciones de observación sin cambio estructural en régimen auditable pleno, marcadas expresamente. No se impone como ley universal que dos frames deban tener vectores diferentes. Tampoco se elude el requisito vigente de `induced_parameters` no vacío mediante un relleno ficticio: la representación y ejecución de una reevaluación sin cambio deben justificarse con su contrato antes de habilitar ese uso.

Las referencias de revisión de C03 no prueban sucesión temporal, ejecución ni cambio causal. Una revisión documental, una consulta presente y una reapertura semántica a U conservan estatutos distintos; DFL-004 sigue abierta. Recuperar un original permite inspeccionarlo sin atribuirle la base nueva de otra evaluación.

## 5. Trayectoria e instantáneas serializadas

El Documento III representa T=(S₁,ν₁,S₂,…,Sₙ). La IR 0.2, heredada en este punto, agrupa cada frame con su transición saliente opcional; la última entrada carece de ella. `wellformed.rs` comprueba no vaciedad, alternancia y referencias. `context_wellformed.rs` comprueba coherencia arquitectónica. Estas guardas se conservan.

La [nota de continuidad](CONTINUIDAD_E_INSTANTANEAS.md) fija una distinción para BIS-03: una instantánea con una entrada terminal y otra instantánea extendida no son por sí solas un registro material append-only. Completar en la nueva instantánea el enlace saliente del anterior último frame requiere declarar cómo se conserva el original y cómo se acredita la extensión. No se permite presentar una reescritura de una evidencia previamente fijada como simple adición.

No se diagnostica aquí una vulnerabilidad ejecutada ni se decide otra estructura de IR. El banco separa conformidad local, relación entre instantáneas y realización durable. La mutación de frames previos se rechaza bajo cualquier representación. Persistencia, recuperación tras reinicio y host adversario conservan sus fronteras de prueba.

## 6. Banco previo y condiciones para ejecutar

El [banco](BANCO_PREVIO_v0_1.json) contiene dieciocho escenarios detallados: seis positivos contractuales y doce negativos. Cada uno identifica precondiciones a materializar, estímulo, evidencia esperada y observador requerido. No contiene resultados nativos fabricados, permisos sintéticos presentados como reales ni instancias productivas de dominios.

Los positivos de inicialización, consulta o reevaluación que necesiten un productor no realizado no se habilitan mediante un booleano, una etiqueta o un constructor de test. Permanecen especificados hasta que la etapa de realización satisfaga el contrato. Mientras tanto, la lectura documental autorizada puede mantener su propio alcance sin adoptar aquellas capacidades.

Antes del ensayo se fijarán fixtures literales y sus huellas, realización y punto de entrada, presupuesto, oráculos independientes y controles para alcanzar cada guarda. La sensibilidad debe detectar promoción ilegítima de estatuto, invención de antecedente, mutación retrospectiva y sustitución de la base original. El [criterio Rust vigente](../bis-c05/CRITERIO_DE_ACEPTACION_RUST_v1.md) exige compilación, recepción real de datos y contraste funcional diferenciados de la preparación auxiliar Python.

Los escenarios originales C06-P/N siguen pendientes. Los nombres de resultados del banco expresan expectativas contractuales; no son diagnósticos canónicos, Tri ni permisos. No se cierran DFL-003/004/006, K1-T o los contratos de transición mediante este incremento.

## 7. Relevo

BIS-03 recibirá las capacidades existentes y las pérdidas delimitadas en la matriz. Las funciones, métodos y tipos Rust deberán imponer el contrato que les corresponda; una elección de notación no constituye una primitiva. Las observaciones se aplican transversalmente, sin incorporar semántica médica ni anticipar un agente.

Continúa BIS-C07: composición entre células de dimensiones diferentes, dirección, puentes y codominios bajo relación declarada. S22 y BIS-02 siguen en ejecución. S24 conserva el orden Bis → catálogo y cierre de fase → análisis e instalación de la GUI.
