# RETP-113 — Procedencia y explicación ES/EN de la etapa frontal

Fecha: 9 de septiembre de 2026. Base: `39203889e2896389907e9a2ddffec0f60dc259e5`, PR #89 en borrador.

Se recibe la continuación autorizada de la localización después del registro del punto de control de rendimiento en el laboratorio. Lecturas rectoras: AGENTS, Pilares completos, acta de perfiles completa, transición y relevos hasta §30, acta del español y contrato diagnóstico RETP-109–112. Este incremento realiza la primera etapa del contrato; no cambia el alcance necesario para aprobar la candidata ni enlazar con R1.

## Compromiso previo de casos y sede

Los [casos fijados](../../tests/diagnostic_frontend/cases.rs) se incorporan antes del cambio funcional. Comprueban EOF ES/EN, UTF-8, CRLF/LF/tabulaciones, carácter léxico no admitido, grafía extranjera, atribución del fallo en ambas posiciones de un ensamblaje mixto con nombres de archivo iguales, huellas de bytes originales, identificadores contextuales admitidos, causas separadas de campos opcionales y ausencia de literales privados en la explicación nueva. Son fuentes sintéticas y no constituyen IMM ni CYB.

La procedencia se conserva durante la tokenización y el análisis, sin reconstruirla desde prosa o IR. Los intervalos son bytes `[inicio, fin)` del original; EOF está en la longitud total. La explicación del ensamblaje mixto se ofrece en ES y EN en ese orden fijo, con un único contexto cuando el fallo es local. Los parámetros se reciben de la guarda y la causa es cerrada; la ausencia de código E catalogado se declara como tal. No se deduce un código del texto previo.

## Compatibilidad y migración

Se constituye la representación interna `diagnostico-frontal/1`. `FrontendError` conservará su presentación `Debug` histórica como adaptador de compatibilidad durante este incremento. Su representación Rust pasa de variantes públicas a un valor de construcción interna con acceso de lectura a diagnóstico, causa y procedencia. `CompileError::Frontend` se conserva. El cambio de representación Rust se declara expresamente; no se publica una segunda entrada de compilación ni se altera el juicio.

La explicación nueva se consulta desde el error devuelto por las mismas entradas públicas. CLI, ABI y playground conservarán de momento su formato anterior: su migración conjunta requiere completar los emisores de validación y los fallos técnicos. Por tanto este incremento no elimina todavía el centinela de la CLI histórica, no localiza todos los rechazos y no acredita la entrega productiva. La vista nueva no utiliza el texto de compatibilidad para construir identidad, causa, parámetros o explicación.

Se preservan las reglas de aceptación, la gramática, la IR canónica y las fuentes de conformidad existentes. No se cambian `BindingContract`, `ValidatedBindings` ni R1. No se añaden dependencias, servicios de traducción ni capacidades de dominio. Este cambio corrige una pérdida de procedencia medida; no es una refactorización de rendimiento.

## Condición de verificación y continuidad

Se ejecutarán los casos comprometidos, el espacio de trabajo Rust y la conformidad vigente. La evidencia distinguirá ejecución local y flujos de CI, y no atribuirá cobertura en WASI o navegador a una prueba nativa. DG01–DG14 se conservan como obligaciones completas; un control parcial no cierra su grupo.

El incremento siguiente recibe los errores de bienformación, referencias y ensamblaje global, con sus contextos relacionados y causas discriminantes, antes de migrar oráculos y salidas CLI/ABI/playground. DFL-001/011, entrega visible y enlace profesional R1 continúan pendientes. Fila 9 permanece abierta.


**Realización candidata RETP-113:** causas frontales emitidas en las guardas, parámetros separados de la cadena histórica, intervalos originales durante tokenización y referencias a índices de unidad durante ensamblaje. Plantillas ES/EN cerradas sin servicios externos. Los casos previos están en el commit 5d6d15b0f9f189d31eede4c28e32391f91dd795a; las [huellas del material a verificar](evidencias/RETP-113/fuentes.json) identifican cinco fuentes. La compilación local no ha podido ejecutarse con las instalaciones temporales: Rust 1.98.0 informa un fallo de metadatos de su biblioteca estándar y la instalación alternativa no carga su biblioteca LLVM. Esto es una limitación de verificación local, no evidencia de un fallo del SV. Se exige la ejecución de CI antes de dar el incremento por comprobado. Salidas CLI/ABI y playground todavía históricas; emisores posteriores, DG01–DG14 y entrega productiva pendientes.


**Verificación recibida RETP-113:** realización 5627416505b119192423722e94a71a3137f1a17a; checkout de CI e5854e72ed0159c140dbd289a5645c143ebd0867. Ambos tienen árbol 488e588a16db189d3c681bd9b1b44226cd4d1518, igualdad cotejada. Los seis flujos terminaron conformes. Rust 1.98.0 de referencia y 1.98.1 adicional: 376 pruebas aprobadas, incluidas las siete pruebas nuevas de la etapa frontal. Conformidad vigente: 14/14 válidos, 106/106 inválidos; mutaciones dirigidas anteriores: 43/43 detectadas. Se conservan [evidencia y límites](evidencias/RETP-113/verificacion.json), logs de los trabajos y huellas. La paridad anterior nativo/WASI/navegador está conservada; no se presenta como prueba del nuevo transporte estructurado, todavía pendiente. La ejecución nueva es de CI y no sustituye con una afirmación de éxito los intentos locales fallidos.

El primer incremento frontal queda realizado y verificado en ese alcance. DFL-001/011, cobertura completa por emisor, contextos de bienformación y relaciones globales, migración de oráculos/salidas, DG01–DG14 y entrega visible permanecen abiertos. PR #89 sigue en borrador. Este asiento es documental y no cambia las fuentes verificadas.
