# F-IF: seis testigos sintéticos de interfaz y relevo G/H

**Identidad:** F-IF/1 · **Fecha:** 07/09/2026 · **Registro:** RETP-2026-090.

**Entrada:** Lenguaje `main@7dd6ef5ea39fcd978d485d1e2db1f3caa8c4bca3`, árbol `efd63a37120575ae0390e4fe7c0211a15c408d45`, PR #75 integrada. **Producto:** fila 5, campaña documental sintética y matriz de suficiencia/pérdida; candidata para G/H, fila 6.

## 1. Recepción y límite de autoridad

Se han leído completos AGENTS, [Pilares](../calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), [perfiles](../calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md), [transición hasta §29](../dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md#contrato-f-relevo-20260907), [F-SV/0.1-candidata](./CONTRATO_CANDIDATO_F_DOMINIO_REPRESENTACION_Y_SUFIENCIA_POR_OPERACION_2026_09_07.md) y [FFL-E](./CONTRATO_MINIMO_DE_SUFIENCIA_REPRESENTACIONAL_POR_OPERACION_PARA_EL_LENGUAJE_SV_2026_08_21.md); se cotejan los campos de [ir.rs](../../rust/sv_core/src/ir.rs) y los asientos directamente aplicables. El corte IMM recibido sigue siendo `SVperitus-dataset/dominio-inmunologia@3bea6b714be3bd1330e6ca6bbbc228b0eb9c065d`.

La campaña constituye exclusivamente **paquetes documentales artificiales** para las seis familias ya ordenadas por transición §15.1. Las operaciones recuperan datos declarados, con firma y esperado explícitos. No calculan un fenotipo, riesgo, aptitud, indicación, dosis ni resultado de OP-IMM-001. Sus estados son realizables por la constitución finita del banco; no se afirma que sean estados clínicos realizables. Las etiquetas de unidad, criterio, regla, procedimiento y tiempo son identificadores artificiales sin significado médico añadido.

Los programas del banco son observadores externos de evidencia. No analizan SV, no crean un intérprete alternativo, no incorporan JavaScript al núcleo ni autorizan Rust ajeno a la DSL. No se modifica Gramática 0.2, IR 0.3, proyección 0.1.0, SVP-ES/SVP-EN, el compilador Rust o el Playground. El compilador Python permanece retirado.

## 2. Paquete reproducible y contrato de los seis espacios

| Pieza | Función y autoridad |
|---|---|
| [cases.json](../../tests/f_if/cases.json) | Identidades, campos, dos alternativas por eje, firma de 18 consultas, respuestas esperadas y frontera candidata. Es la constitución sintética revisada; no es un esquema de intercambio clínico. |
| [run.mjs](../../tests/f_if/run.mjs) | Enumera el producto completo, verifica recuperación, busca colisiones exactas y comprueba transporte, captura, vínculo lateral y fronteras. Usa sólo módulos estándar de Node; no consulta servicios. |
| [sensitivity.mjs](../../tests/f_if/sensitivity.mjs) | Doce pruebas del observador, incluidos controles que lo deben refutar. |
| [evidence.json](../../tests/f_if/evidence.json) | Matriz completa de 54 filas, 18 certificados, 48 identidades de entrada y pares concretos de pérdida con bytes transmitidos y respuestas distintas. Incluye hashes de constitución y observador. |

Los seis paquetes comparten el contrato siguiente, aplicado mediante su ID `IF-IMM-01…06` y versión `/packet/1`. No se deja implícito ningún bloque de F §2:

| Bloque | Declaración aplicable a cada familia |
|---|---|
| Autoridad y perímetro | Unidad Lenguaje, encargo técnico F-IF autorizado. ID de espacio `IF-IMM-xx/1`; corte de constitución: blob de cases.json en el commit del expediente. Sólo las alternativas allí declaradas. No sustituye el dominio IMM ni asigna una versión a Q0. |
| Representación y fuente | Paquete externo JSON UTF-8, codificado por `encode`: compacto, orden de miembros conservado, sin salto final; tipos y orden de arrays conservados. cases.json usa sangría de dos espacios y salto final y se verifica contra esa forma. Una unidad por estado; no hay ensamblaje SV ni traducción ES/EN en estas entradas. Las versiones SV citadas en §1 identifican el destino cuya carencia se coteja, no un parser de estos paquetes. |
| Constitución y X_D | Producto de tres ejes binarios independientes `context × a × b`, exactamente ocho estados por familia. `fixed` pertenece también al paquete. La independencia es sólo documental: escenarios alternativos de registros artificiales, sin pretensión de compatibilidad clínica. No hay células, μ_C ni instancias (C,j) en estas consultas de documentos; no consumen parámetros celulares. DFL-005 sigue bloqueante para operaciones que sí los consuman. |
| W_j, captura φ_j y observación | j identifica aquí la entrada de interfaz, no una posición celular. W_j: secuencias finitas de bytes del banco. φ_j: decodificación UTF-8 y JSON exacta bajo `CAPTURE-FIF/1`; una secuencia ilegible o ajena a la codificación produce `CaptureFailure`, el fallo O_j^⊥ del banco, sin observación ni respuesta Q. Configuración ausente/otra versión produce error técnico separado antes de capturar. |
| Admisibilidad r_j | Observación igual, incluidos campos, tipos y orden, a uno de los ocho paquetes constituidos → `Ok`; otra observación bien codificada → `NotAdmitted`. No se reparan campos. `Degraded` no tiene clase constituida en este espacio; no se le atribuye prueba. Un null explícito dentro de una observación admitida, por ejemplo falta de constancia en IF04, es dato documental, no fallo de captura. |
| Transducción τ_j | No aplicable: Q devuelve tuplas documentales, no Tri ni estado celular. K1-T conserva producción observación→Tri no habilitada. No hay sustitución Bottom/NotAdmitted/fallo/insuficiencia→U. |
| Operaciones | Cada `operations` fija ID/versión implícita en F-IF/1, rutas ordenadas, dos respuestas esperadas y eje del que depende. Firma `Q:X_D→Y_Q`, con Y_Q exactamente las dos tuplas enumeradas. Regla: recuperar las rutas declaradas, sin convertir valores ni inferir significado. Precondición: paquete constituido y representación suficiente. Todas son de lectura y carecen de efectos. |
| Agente y permisos | No se constituye agente. El observador tiene el encargo de leer este corpus sintético y comparar sus resultados; no concede permiso a un agente clínico o material. |
| Soporte | Realización externa `F-IF-observador/1`, identificada por su blob/hash, cases.json, configuración CAPTURE-FIF/1 y Node informado en la ejecución. Módulos estándar; sin paquetes npm, red, reloj de dominio o fichero externo mutable. Requiere conservar bytes y orden y terminar la enumeración; no ofrece aislamiento frente al host ni presupuestos productivos. |
| Evidencia y fallo | Los pares y recuperaciones se cotejan sobre el dominio completo. Fallo de proceso/transporte o configuración invalida el ensayo; no emite juicio semántico alternativo. Los juicios del banco no son diagnósticos emitidos por SV. Su evidencia se recibe con el alcance y responsables de §§5–7. |

Los esperados están comprometidos separadamente de la función de recuperación. No se regeneran a partir de la salida observada. Constituciones, selección de consultas y oráculos son de esta misma autoría técnica: la independencia del código de comparación no equivale a revisión clínica o auditoría externa independiente.

## 3. Cadena, recuperación y evidencia lateral

Para cada familia, F0 conserva el paquete completo; r0 elimina el bloque `a` y obtiene F1; r1 elimina `b` y obtiene F2. Se comprueban ambas igualdades de reducción sobre los ocho estados. La interfaz ψ transmite exactamente `encode(Fj)`; H_j son esos bytes. El receptor sólo recibe H_j. Las coordenadas del producto y el ID/hash del paquete original permanecen en el expediente forense, **no viajan ocultos en H_j**.

Las tres operaciones por familia dependen, respectivamente, de `a`, `b` o `context`. q_j decodifica H_j y selecciona sus rutas: no recibe el estado original, su índice, los esperados ni una tabla de búsqueda. Se compara la tupla recuperada con el esperado comprometido en todos los estados. La igualdad del testigo negativo es literal entre bytes H; la desigualdad de Q conserva tipos, orden y null. No se confunde igualdad normalizada con igualdad literal.

| Consulta de cada familia | F0 | F1 | F2 | Certificado |
|---|---|---|---|---|
| Dependiente de a | Suficiente | Pérdida | Pérdida | Frontera 0, recuperación en F0 y par de pérdida en F1 |
| Dependiente de b | Suficiente | Suficiente | Pérdida | Frontera 1, recuperación en F1 y par en F2 |
| Dependiente de context | Suficiente | Suficiente | Suficiente | Frontera 2, último nivel |

Así F1 y F2 pierden información, pero conservan otras operaciones. La frontera es relativa a Q y a esta cadena; no corresponde a N0/N1/N2 de la IR. El segmento de niveles aceptados se contrasta contra los resultados, no se amplía libremente.

La recuperación adicional usa **(H2, S, P)**: S declara versión y aporta a/b completos con digest de fuente; P es ese digest fijado por el manifiesto del certificado recibido. `restore` comprueba versión, coincidencia con P y digest del paquete reconstruido. El nuevo certificado depende explícitamente de esos tres componentes. Intercambiar S de otro estado con igual H2 se rechaza por P, incluso si S tiene digest internamente correcto. P no autoriza una tabla de recuperación oculta ni acredita autenticidad frente a quien pueda sustituir todo el manifiesto. La prueba acredita consistencia en este corpus confiado; custodia material y autenticación permanecen pendientes. No vuelve suficiente H2 sola.

Ejemplo IF01: los estados `IF-IMM-01-000` y `IF-IMM-01-010` conservan idéntica F1 y devuelven unidades/criterios distintos a Q01-unidad. Los estados `000` y `001` conservan idéntica F2 y distinta Q01-correccion. El expediente conserva las dos fuentes y los bytes comunes; no adjudica a esos identificadores artificiales una equivalencia clínica.

## 4. Resultado de las seis familias

| Familia | Q con frontera 0: se pierde en F1 | Q con frontera 1: se pierde en F2 | Q con frontera 2: conservada | Conclusión técnica |
|---|---|---|---|---|
| IF-IMM-01, analítica | Q01-unidad: observación, valor textual, unidad y criterio | Q01-correccion: informe, estado y referente sustituido | Q01-peticion | Igual petición/muestra no elimina unidad ni relación de corrección. El banco no decide rangos ni conversiones. |
| IF-IMM-02, citometría | Q02-configuracion: panel, configuración, controles y procedencia de derivación | Q02-informe: interpretación declarada, autor y regla | Q02-muestra | Un mismo fichero instrumental no identifica la derivación ni el informe. No se calcula interpretación citométrica. |
| IF-IMM-03, médula ósea | Q03-informe: interpretación declarada y autor | Q03-procedimiento: sitio, procedimiento y procesamiento | Q03-muestra | Incluso conservando idénticos bytes de imagen, se pierden informe y procedimiento. Su hash tampoco resolvería esa pérdida. No es una prueba de inversión de hash: hashes distintos pueden distinguir estados en un conjunto finito. La recuperación de anexos generales por hash no está acreditada ni se presenta como imposibilidad demostrada. |
| IF-IMM-04, terapia | Q04-constancia: administración registrada y resultado documental | Q04-cambio: modificación/suspensión y motivo | Q04-orden | Orden y pauta no acreditan constancia de administración. `not-recorded` expresa falta de constancia, no que nunca ocurrió. Suspensión puede ser posterior al registro; el producto no decide cronología terapéutica. |
| IF-IMM-05, ingreso/día | Q05-episodio: episodio, encuentro e intervalo | Q05-organizacion: servicio, organización y modalidad | Q05-ubicacion | Misma ubicación no identifica episodio/encuentro ni su contexto organizativo. T1…T4 son etiquetas ordenadas explícitas, no reloj externo. |
| IF-IMM-06, historia | Q06-historia: orden causal, estados, enmienda, procedencia y episodios | Q06-vigencia: intervalo y base de validez | Q06-expediente | Una misma vista vigente pierde historia; el orden de recepción se conserva separado y no puede reemplazar al causal. No demuestra persistencia append-only frente a un host adverso. |

**Resultado finito:** 6 familias, 48 estados, 18 operaciones y 54 filas. **36 filas suficientes**, acreditadas mediante **288 recuperaciones exactas**; **18 filas con pérdida**, cada una con par realizable explícito. **144 recuperaciones adicionales** sobre (H2,S,P), **24 controles técnicos** —cuatro por familia— y **12 pruebas de sensibilidad**. No son 48 programas SV ni nuevos casos clínicos; el corpus de conformidad continúa en 100.

## 5. Insuficiencias localizadas y decisión de recepción

| Necesidad y operación afectada | Representación comprobada / carencia | Propietario y cambio propuesto |
|---|---|---|
| Las doce Q de detalle (`a`/`b`) | Pérdida probada en los niveles indicados, aunque transporte correcto y datos presentes en origen. | Interfaz: conservar los bloques necesarios o aportar S y P identificados con nuevo certificado; excluir Q cuando falten. No recomponer una respuesta por aproximación. |
| Todas las Q que se pretendan ejecutar como SV | El banco externo representa sus contratos. La IR actual conserva CaptureSpec/AdmissibilitySpec/Ternarizer nominales; Domain y QuerySpec no contienen cadena, recuperación y certificado FFL-E materializados. `query_type` o `restrictions` como cadenas no hacen comprobable este contrato. | Lenguaje después de G/H e I/J: resolver representación y versión de obligaciones realmente requeridas antes de ofrecer esas Q. Ninguna de las 18 se incorpora aquí a las operaciones admitidas de SV. No obliga a meter los formatos clínicos en IR. |
| Dependencias de OP-IMM-001 | No se ha fijado qué consultas del banco consume Q0 ni qué capturas cumplen los 27 parámetros. No existe ligadura clínica por semejanza de nombres. | G/H, dominio IMM: recibir la candidata y determinar aplicación/no aplicación, cobertura, reglas, procedencia y ausencia por requisito 001…015 y solicitudes 001…044 existentes. DFL-005 conserva (C,j), mínimo y ligaduras bloqueantes. |
| Observación → Tri, causas de U y criticidad | Esta campaña no consume transducción ni produce CellState/CriticalityResult. | K1-T/puerta algebraica y DFL-006 conservan sus pruebas pendientes; si G/H precisa estas funciones, devolver su necesidad sin habilitarlas por un formato documental. |
| Corrección, vigencia y anexo disponible | Bytes, referencias y orden del corte sintético verificables. Persistencia autoritativa, autenticidad, disponibilidad de anexos y orden material no demostrados. | Dominio constituye significado; frontera conserva referencias; R2/R3/R4 imponen garantías aplicables. No sustituir historia por vista ni confundir hash con contenido disponible. |
| Información lateral y procedencia | P y S explícitos bastan para el cálculo finito; su entrega y custodia confiables no están demostradas en plataforma productiva. | Contrato operacional/K2 según objeto; no ofrecer la operación cuando la dependencia imprescindible no esté acreditada. |

No hay refutación del criterio FFL-E en estos casos: las pérdidas refutan las representaciones reducidas para Q específicas. Tampoco se proclama suficiencia clínica de F. Las carencias de realización permanecen **suficiencia no acreditada**, diferenciadas de los pares que sí demuestran pérdida. La campaña no introduce diagnósticos de compilador ni convierte estos juicios en U.

## 6. Perfiles, soporte y aprovechamiento del laboratorio

Se reciben los registros 012/016/018 y matriz 020 mediante F §7 y transición §15.2, con sus cortes ya identificados: recuperación SQLite sintética, separación validez/identidad y comparación FFI/WASM de 79 programas EN. Se aprovecha especialmente la distinción validez/identidad: H puede ser válido y haber perdido lo que Q necesita. Esos ensayos no se repiten ni reciben retrospectivamente 48 entradas o 18 consultas nuevas.

| PT | Aplicación y límite en F-IF |
|---|---|
| PT01/PT02/PT04 | Fuentes, versión, bytes, tipos, orden y distinción entre juicio de representación y fallo. El perfil JSON externo no es SVP-ES/EN; no acredita ensamblaje del banco en SV. |
| PT03 | Constitución sintética explícita y cobertura total del producto; autoridad de IMM y agentes preservada. |
| PT07/PT10 | Se transportan constancia de cambio, enmienda, vigencia e historia. No se ejecuta cancelación ni revocación; lectura de instantáneas no prueba persistencia material. |
| PT12/PT13/PT14 | Dependencias de la recuperación conjunta explícitas; identidad de artefactos y controles para detectar cambios. No se certifica ensamblaje tecnológico productivo. |
| PT05/PT06 | No aplicables a estas Q de lectura sin efectos ni reentrega de ejercicios. Las constancias ficticias de administración no son efectos ejecutados. |
| PT08/PT09/PT11 | Observador y entorno identificados, sin dependencias npm ni acceso de red en el ensayo. No se afirman aislamiento, límites de disponibilidad, coste productivo o independencia del proveedor de CI. |

DFL-009 permanece diferida a fila 9 tras el primer universo CYB. Esta compuerta no abre servicio nativo, Cloudflare, Workers ni otras plataformas. Los programas SV conservan la custodia Rust subordinada a su DSL. La paridad nativo/WASI/navegador debe comprobarse en cada cambio ejecutable pertinente; aquí no hay una implementación SV nueva de estas consultas que pueda compararse entre destinos. PR #74 conserva su evidencia de paridad sobre su propio corte, sin atribuirla como prueba de F-IF.

## 7. Verificación reproducible y relevo

Desde la raíz del repositorio:

```sh
node --version
node --test tests/f_if/sensitivity.mjs
node tests/f_if/run.mjs
```

Ejecución local: Node **v24.19.0**, Linux x86_64; sin conexión externa del observador. La salida de `run.mjs` informa cuentas y SHA-256 de evidence.json, cuyo contenido incluye hashes de entradas. La evidencia no incorpora mediciones de rendimiento: duración de tests no equivale a coste del núcleo. `run.mjs` coteja bytes con evidence.json y falla ante deriva; no dispone de una opción para aceptar automáticamente nuevos esperados.

Las doce pruebas del observador comprueban dominio completo y rechazan vacío/parcial/ajeno; oráculo incorrecto o coerción; frontera ampliada o consulta artificialmente constante; fuente/hash ocultos; recuperación sin información; S intercambiada, corrupta o de otra versión; homónimos JSON, precisión perdida, normalización y UTF-8 inválido. Además conservan los contraejemplos específicos de fichero/imagen, orden/administración, ubicación/episodio e historia/recepción. Son controles acotados; no una prueba de ausencia de todos los defectos del observador.

La promoción exige **Conformidad SVP sobre la candidata exacta**, que ejecuta esta campaña y su sensibilidad, además de la realización nativa y los 100 casos SV existentes. El workflow registra su versión efectiva de Node y Rust. Las identidades de base, cabeza, árbol, commit de prueba, ejecución y resultado se conservan en el expediente de PR e integración. No se atribuye a esta exigencia un resultado todavía no ejecutado.

**Decisión:** F-IF completada en los seis espacios sintéticos al integrar la candidata verificada. **Relevo:** fila 6/G-H, retorno acotado a Inmunología. Su entrega deberá asignar aplicación/no aplicación a las pérdidas y dependencias, cotejar la candidata con OP-IMM-001 y devolver aceptación o refutación fundada con los requisitos reconciliados. Mantiene los 27 parámetros, sus agrupaciones externas `(6,1,3,2,6,9)`, M-MODIFIER-001 candidata y las exclusiones de Q0. El Lenguaje no decide por IMM ni exige cerrar sus demás universos. Tras ese dictamen corresponde fila 7; I/J, álgebra, K2 y materialidad siguen su secuencia.
