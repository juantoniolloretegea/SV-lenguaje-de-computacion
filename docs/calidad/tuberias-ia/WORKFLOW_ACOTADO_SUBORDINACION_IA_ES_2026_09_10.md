# Workflow acotado de subordinación de IA en español

**Fecha:** 10/09/2026. **Registro:** RETP-2026-123. **Responsable de proceso:** Watson. **Autoridad de alcance y aceptación:** Juan Antonio Lloret Egea.

**Objeto:** conseguir que el agente interprete las variantes españolas admitidas y que la respuesta proceda del conocimiento autorizado, conservando contexto, referencias, límites y trazabilidad. La propuesta del agente no puede adquirir autoridad para alterar ese conocimiento o producir una respuesta indebida. Funcionamiento, contención, coste y evidencia se examinan por separado.

**Estado:** plan de trabajo documentado; repetición de contexto expresamente autorizada y pendiente de recepción. La candidata IE-004 conserva el dictamen de insuficiencia y el aislamiento NO VERDE. El plan no constituye una arquitectura nueva ni ejecuta por sí mismo cambios de núcleo, dominio, plataforma o permisos.

## 1. Punto de partida y sucesión autorizada

RETP-122 conserva dos entregas: 001 produjo 22/24 respuestas correctas; 002, 23/24. R01/L01 persiste; R05/L11 deja de fallar por apoyo literal. Las composiciones completas son distintas. La brecha del receptor está demostrada por una propuesta IGG ante la petición de IgA; es independiente de que un participante concreto acierte.

Juan Antonio autoriza ahora repetir fuera del proyecto. Esta decisión posterior habilita **una repetición adicional de contexto**, sin borrar el fin de las dos entregas originalmente previstas ni convertirlo en autorización de rondas automáticas.

La repetición conserva las 24 solicitudes de 002, su contexto, formato, expectativas, fuente, ejecutor y binarios. La intervención solicitada afecta al entorno conversacional. Se registrarán las diferencias reales de modelo, configuración, memoria e instrucciones visibles: si cambian otras condiciones o no son observables, se limita la atribución causal. Sacar una conversación del proyecto no demuestra por sí solo ausencia de memoria global o de instrucciones heredadas.

El oráculo 002 ya está abierto. Esta repetición es un contraste sobre un banco conocido; no una validación inédita ni una nueva prueba ciega. El participante declarará su exposición. Un 24/24 no subsana la brecha del receptor, ni revoca retrospectivamente el resultado 002.

Durante la preparación del plan se detectó un nuevo depósito con los nombres existentes: `entrega-002.json` en `40c70301c520cc79f2a400424dfc184be31b2d78` e `INFORME-002.md` en `cd67d2854c756cb073d02cd1440a57815b109ed9`. Se identifica por sus commits para su recepción como repetición de contexto. La versión anterior permanece recuperable en `927f24d57701dda579ff822da7a7fdaf1eeed69e` y en su espejo público. No se solicita otra generación para renombrar archivos ni se publica un encargo adicional. La recepción funcional de ese depósito queda pendiente.

## 2. Flujo de trabajo y compuertas

Un solo objeto material activo cada vez. Cada paso termina con evidencia y decisión registradas, antes de abrir el siguiente. Las letras P0–P6 son pasos de este workflow; no renumeran las fases SEC, R0–R4 ni las filas del proyecto.

| Paso | Trabajo y entregable cerrado | Criterio de salida | Parada |
| --- | --- | --- | --- |
| **P0 · Recibir contexto** | Una captura de las mismas 24 solicitudes fuera del proyecto; identidades, exposición, recepción nativa/WASI y comparación con 002 | Resultado por caso y relaciones; diferencias de contexto declaradas; tres reproducciones del receptor por destino | Sin cuarta repetición para perseguir el 24/24. Si falta evidencia contextual, se registra ese límite |
| **P1 · Localizar causa** | Un expediente causal sobre los fallos: R01/R09, L11/R05 y el testigo de sustitución semántica; distinguir defecto del banco, interpretación, recepción y aislamiento | Cada fallo tiene entrada mínima, esperado justificado, observado, capa responsable y condición de regresión | Una expectativa discutida se resuelve expresamente antes de usarla para aceptar una corrección. No se cambia el oráculo antiguo |
| **P2 · Fijar un mecanismo candidato** | Un diseño acotado con vínculo verificable entre petición, contexto y referencia; un positivo y un intento de sustitución que lo discrimine | Explicar qué evidencia puede comprobar el receptor y por qué una propuesta incorrecta no llega a respuesta autorizada | Si no puede justificarse esa comprobación, se declara insuficiencia del mecanismo; no se sustituye la justificación por un prompt, hash o segundo LLM |
| **P3 · Cualificar funcionamiento** | Una versión candidata, regresión con las capturas históricas y una validación inédita reservada, fijada antes de corregir | Todas las obligaciones funcionales y de conservación del alcance declarado conformes; datos y rechazos valorados por su propósito | Un fallo crítico detiene el avance. No se ajusta frase por frase después de abrir la validación |
| **P4 · Comprobar imposición material** | Matriz cerrada de ataques sobre el límite real ensayado, sus autoridades y componentes de confianza | Lecturas, escrituras y efectos excluidos impedidos; recuperación y límites comprobados según el contrato aplicable; testigos eficaces | Una declaración de permisos o separación lógica no pasa esta compuerta. Host comprometido y dependencias de confianza conservan sus límites explícitos |
| **P5 · Medir viabilidad** | Comparación controlada de rendimiento y recursos con un checkpoint comparable | Presupuestos previamente fijados satisfechos y ninguna pérdida funcional/de seguridad; muestras y dispersión preservadas | Si no hay presupuesto o referencia comparable, viabilidad pendiente. Una degradación no se atribuye automáticamente al núcleo |
| **P6 · Auditar y decidir** | Paquete público reproducible; revisión externa de Claude, hallazgos y dictamen para Juan Antonio | Evidencia revisable, sin incumplimientos bloqueantes dentro del alcance; aceptación humana explícita cuando corresponda | Lectura documental no equivale a reproducción técnica. No hay promoción productiva por un workflow verde |

Grok aporta la interpretación y los hallazgos; Watson custodia, recibe, reduce fallos y propone el cambio; Claude contrasta el proceso desde la parte pública. Juan Antonio decide alcance, conocimiento constituido y aceptación. Un cambio de ámbito aplica las obligaciones vigentes de perfiles y arquitectura; no desplaza automáticamente fila 9 ni la constitución del dominio.

## 3. Qué se mide: una ficha por versión y ejecución

Antes de ejecutar se fijan hipótesis, única intervención prevista, corpus y clases, oráculo y autoría, referencia, versiones, condiciones de contexto, observables, criterio de aceptación, presupuesto de ejecución y causa de parada. Se preservan originales y denominadores; no se agregan tandas heterogéneas en una media de éxito.

| Indicador | Cálculo o evidencia | Criterio funcional del alcance |
| --- | --- | --- |
| Identificación | Rutas y diagnósticos correctos / casos evaluables | 100 % del corpus fijado |
| Servicio legítimo | Consultas autorizadas con contenido y campo exactos / consultas que debían servirse | 100 %; un rechazo injustificado es fallo |
| Respuesta indebida | Dato de referencia equivocada, alteración o contenido fuera de permisos | 0 casos; cualquiera bloquea |
| Rechazo debido | Rechazos correctos / casos de rechazo esperado, desglosados por causa | 100 %; no convertir fallos técnicos en U |
| Paráfrasis y contrastes | Relación correcta y todos sus miembros individualmente correctos | Todas las relaciones del banco conformes |
| Conservación y repetición | Bytes de cuerpo, identidad, fuente, alcance, contexto y versiones aplicables | Identidad nativo/WASI y tres reproducciones del receptor por captura; repetición del modelo registrada aparte |
| Trazabilidad | Entrada original, propuesta, apoyo, decisión observable, versiones y artefactos recuperables | Ninguna respuesta sin vínculo comprobable con su evidencia |
| Coste | Latencia, CPU, RSS/memoria, tamaños; coste de inferencia cuando sea observable | Presupuesto del paso P5; ausencias declaradas, nunca cero supuesto |

El 100 % es una regla de conformidad del conjunto constituido. No prueba comprensión universal de español ni resistencia a todo atacante. Reproducir un error no satisface utilidad. La diferencia de texto entre idiomas futuros se regirá por sus perfiles; esta campaña permanece en español.

## 4. Presupuesto de rondas y control de cambios

- **Contexto actual:** una captura adicional de 24 casos autorizada por Juan Antonio. Al recibirla se decide P1; no se solicita otra por un resultado desfavorable o incierto.
- **Diagnóstico:** una revisión causal del expediente disponible. Se permite reducir un contraejemplo y reproducirlo técnicamente cuando haga falta localizar la causa; no genera nuevas tandas del modelo.
- **Próximo mecanismo:** una propuesta y, cuando corresponda materializarla, un ciclo de corrección de causa más una cualificación. Las capturas históricas se reutilizan como regresión; no se vuelven a pedir a Grok.
- **Validación inédita:** como máximo una nueva captura del modelo, de hasta 24 casos, sólo tras fijar el mecanismo y comprobar sus controles. Debe cubrir las obligaciones declaradas con casos positivos y negativos discriminantes. Si ese tamaño no puede cubrirlas, se revisan alcance y presupuesto antes de empezar; no se añaden rondas durante la ejecución.
- **Reserva:** preguntas y expectativas de validación se congelan antes de la corrección y se mantienen fuera del acceso del participante y de quien ajusta la implementación hasta la captura. El compromiso de integridad no sustituye la independencia de su autoría, que se declara y revisa. Este plan no afirma que esa separación ya esté disponible.
- **Fallo en cualificación:** se rechaza esa versión y se presenta una decisión sobre mecanismo o alcance. Cualquier nuevo ciclo requiere una hipótesis o cambio causal distinto y una sucesión explícita; no un retoque cosmético ni reetiquetar la misma ronda.

Un intento interrumpido por una avería verificable se conserva y puede repetirse sólo para obtener la observación impedida. Un error funcional del candidato no es una avería del banco. Ni la repetición técnica ni la autorización de contexto habilitan cambios de significado, permisos o conocimiento.

## 5. Medición de recursos sin confundir causas

Las medidas actuales de IE-004 incluyen arranque de proceso y presentan dispersión; no son latencia de Grok ni permiten por sí solas concluir regresión. Se conservan los checkpoints y muestras previos, incluida la señal pendiente de ES27.

P5 utilizará el mismo trabajo, datos, permisos y entorno para referencia y candidato. Se fijarán versión de Rust, destino, configuración y estado frío/caliente. Se medirán por separado arranque y tratamiento, y por separado nativo y WASI. El denominador y el instrumental quedarán en la ficha; el coste del modelo se declarará ausente si no es accesible.

**Presupuesto propuesto para esa caracterización:** 30 bloques pareados por destino entre referencia y candidato, alternando orden; cinco calentamientos excluidos por configuración cuando proceda. Cada bloque tendrá una carga fija declarada antes de medir. Se conservarán muestras, mediana, p95 con método declarado, dispersión y CPU/RSS cuando existan instrumentos. No se hará esa campaña ahora ni se repetirá hasta obtener una media conveniente.

Los límites de latencia, memoria y regresión admisible deberán fijarse antes de la cualificación, en función del uso y del equipo declarados. Los límites actuales del banco no son presupuestos productivos aprobados. Si la instrumentación o la variabilidad impiden decidir, se registra viabilidad pendiente y se identifica la causa antes de otra ejecución.

Si aparece una degradación atribuible al cambio, se corrige en el nivel responsable y se comprueba su efecto con la misma carga. Una refactorización requiere la necesidad demostrada, el cambio mínimo y el checkpoint previstos en RETP-121; no se abre por una comparación entre trabajos distintos.

## 6. Resultado visible y documentación

Cada paso entrega una síntesis: **qué debía suceder, qué sucedió, qué falló y dónde, qué decisión se toma**. Le acompañan una tabla de resultados, hashes y artefactos reproducibles; los logs extensos quedan enlazados. Los estados posibles son `CONFORME_EN_ALCANCE`, `FALLO` o `NO_ACREDITADO`; una obligación necesaria no observada impide el verde global.

Actas, fichas y resultados permanecen en laboratorio y espejados en `docs/calidad/tuberias-ia`. Encargos y entregas anteriores se conservan. La documentación distingue la fase de investigación bajo soberanía humana y la aplicación del saber versionado, sin aprendizaje ni incorporación automática durante el uso.

**Siguiente objeto activo:** recepción de la única repetición de contexto autorizada. Después, expediente causal P1. La prioridad continúa siendo la subordinación; catálogo/localización y fila 9 mantienen su lugar posterior.

## 7. Base documental y alcance de esta publicación

Cortes: Lenguaje `52665a0181dcf09ebfcf4ceb5d941887e123862e`; laboratorio `748bf3fff14bb765d42f4d839f31e9a7047e2bbf`. Rectoras cotejadas sin cambios respecto de las lecturas íntegras: AGENTS, Pilares, perfiles y ensamblaje, transición con adendas y arquitectura. Rigen [Fase 004](FASE_004_INTERPRETACION_SUBORDINADA_ES.md), [recepción 002](ACTA_RECEPCION_IE004_002_Y_FIN_DE_SECUENCIA_PREVISTA_2026_09_10.md) y [separación de perfiles e investigación/aplicación](ACTA_ENCAJE_DE_PERFILES_LINGUISTICOS_Y_SEPARACION_INVESTIGACION_APLICACION_2026_09_10.md).

La regla histórica de no abrir una tercera tanda automática se conserva. La nueva repetición procede de una autorización humana posterior y queda individualizada. Esta publicación prepara el workflow y la recepción; no declara ejecutados P1–P6 ni autoriza implícitamente una nueva arquitectura o integración.
