# P2 · Perfil español, custodia del texto y preparación de reserva

**Fecha:** 10/09/2026. **Registro:** RETP-2026-126. **Responsable:** Watson. **Autoridad de alcance y aceptación:** Juan Antonio Lloret Egea.

**Resultado de esta actuación:** especificación del perfil candidato `IE004-ES-P2/1`, contrato de propiedad/préstamo del texto y encargo independiente de revisión/reserva preparados y espejados. **P2 sigue en revisión; P3 no iniciado; aislamiento NO VERDE.** No hay un nuevo resultado de ejecución ni otro encargo de interpretación para Grok.

## 1. El matiz incorporado

Juan Antonio autoriza continuar y propone distinguir quién posee una cadena, quién la usa y durante cuánto tiempo. Afecta al vínculo material de P2: el conductor conserva la petición original y el contexto fijado; el verificador recibe vistas de lectura cuya vida depende de esa custodia; la propuesta externa es otro objeto. Las citas se expresan como intervalos ligados a una solicitud y se vuelven a comprobar contra sus bytes. La traza persistente conserva originales, no referencias temporales a funciones que ya terminaron.

La propiedad de memoria no demuestra identidad semántica, permisos ni revocación de copias externas. Esta delimitación se apoya en documentación oficial de Rust consultada el 10/09/2026: [ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), [lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html), [préstamos](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) y [acceso comprobado a `str`](https://doc.rust-lang.org/std/primitive.str.html#method.get). El [perfil candidato](ie004/perfil-es-p2-1/PERFIL_INTERACCION_ES_IE004_CANDIDATO_1.md) precisa además el alcance del ejemplo aportado, sin atribuirle una compilación que no se ha realizado.

## 2. Qué queda especificado para revisar

El perfil define léxico, conversión de mayúsculas con trazabilidad de bytes, 23 producciones, reglas de contexto y papel sintáctico, alternativas, causas de diagnóstico y presupuestos de laboratorio. Contempla la longitud, los préstamos y los desbordamientos de contadores/intervalos. El análisis conserva la totalidad de la pregunta, incluidas negación, exclusiones y referencias no resueltas.

Dos precisiones respecto de RETP-125 evitan trasladar la brecha al nuevo diseño:

- **La negación conserva su alcance de tupla:** «No consulte IgG» excluye LEER/IGG; no excluye por separado toda operación LEER. Debe permitir la petición posterior de IGA.
- **La propuesta no gobierna el presupuesto del análisis:** no elige su orden canónico ni consume la cuota de recuperación. Una propuesta falsa, grande o tardía no debe transformar por ese solo motivo una petición representada en un diagnóstico semántico.

La [cobertura documental](ie004/perfil-es-p2-1/COBERTURA_DOCUMENTAL.json) relaciona los 48 casos de las dos campañas originales con reglas propuestas y sus expectativas históricas. Conserva procedencia y oráculos; no los recalcula para el diseño. La repetición de contexto reutilizó las mismas solicitudes 002 y no añade otros 24 casos inéditos. Esa relación es una previsión de cobertura, **no una prueba de reconocimiento** ni 48 aciertos del mecanismo.

Se mantienen artificiales la base y la terna almacenada. No se incorpora un universo, una célula, un Frame, un perfil de agente ni una nueva API productiva. La ventaja de usar el modelo, la suficiencia efectiva de la gramática, la paridad y los costes siguen pendientes de cualificación.

## 3. Continuación concreta y acotada

Queda disponible el [encargo de revisión y reserva independiente](ie004/perfil-es-p2-1/ENCARGO_REVISION_Y_RESERVA_P3.md), utilizable por Claude desde la parte pública. Pide una única revisión de la suficiencia del contrato. Si hay un defecto bloqueante, se registra el testigo y se detiene la reserva. Si el contrato permite evaluar, la autoría independiente prepara una sola validación de hasta 24 casos bajo el presupuesto ya aprobado.

El [workflow RETP-123](WORKFLOW_ACOTADO_SUBORDINACION_IA_ES_2026_09_10.md) exige congelar preguntas y expectativas **antes de corregir**, fuera del acceso de implementador y participante. Por eso se prepara el encargo completo, pero no se fabrica aquí una «reserva independiente» ni se escribe todavía el corrector. Juan Antonio debe custodiar el paquete y traer sólo dictamen y compromiso; la petición detallada y su secuencia de apertura constan en el encargo. No se afirma que esa custodia ya exista o que Claude haya aceptado el trabajo.

Una vez satisfechas revisión y reserva, procede el único ciclo de materialización/cualificación previsto, con los antecedentes como regresión y una sola captura inédita de Grok cuando corresponda. Un fallo crítico rechaza la versión. Si no puede justificarse el perfil o el aporte del agente, se decide el mecanismo o su alcance con esa evidencia, sin rondas automáticas.

## 4. Evidencia y límites

**Revisión realizada:** cotejo de ramas, rectoras, fuente/contrato y oráculos públicos; revisión documental de la semántica propuesta; relación de casos conocidos; inventario de longitudes e identidades; verificación de publicación y espejos. **Ejecuciones nuevas del corrector o receptor: 0. Consultas nuevas al modelo: 0.** Los cálculos de bytes del inventario no ejecutan gramática, núcleo ni juicios SV.

Los límites fijados son una candidata de laboratorio, no presupuestos productivos acreditados. Siguen pendientes la revisión de ambigüedades, completitud, límites y servicio bajo fallo del agente; I01–I05 y la frontera real; rendimiento comparable P5 y aceptación humana. Las garantías del compilador Rust no sustituyen esas pruebas.

**Cortes:** Lenguaje `55e354227518c77753578cfcf2d6da5617a76956`; laboratorio `fa21b92a39565a7f5a4582a9ca51c331c73eb10a`. AGENTS, Pilares, perfiles/ensamblaje, transición con adendas y arquitectura conservan las identidades cotejadas frente a las lecturas íntegras previas. Rigen Fase 004, RETP-121, workflow RETP-123, diagnóstico RETP-124 y diseño RETP-125. [Manifiesto del paquete](ie004/perfil-es-p2-1/MANIFIESTO.json).

Esta acta es la pieza sucesora del diseño experimental para la nueva precisión de custodia, gramática y presupuesto. Se actualizan RETP CSV/Markdown, índices y custodia. Los documentos, capturas, oráculos, fuentes y binarios anteriores permanecen conservados. Catálogo/localización y fila 9 mantienen el orden posterior.
