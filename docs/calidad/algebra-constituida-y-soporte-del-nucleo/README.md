# Álgebra constituida y soporte real del núcleo SV

**RETP-2026-151 · 12 de septiembre de 2026 · Auditoría acotada, sin modificación del núcleo productivo.**

## Resultado

Las leyes generales de composición pertenecen al soporte algebraico del núcleo. No deben trasladarse en bloque a la fase de agentes. El código examinado representa operaciones y comprueba una parte de sus condiciones; **esa admisión no acredita todavía su ejecución algebraica**. Existen controles materiales de autoridad, admisibilidad, resolución y cierre de referencias: tampoco sería correcto describir todo el núcleo como meramente nominal.

El reparto concreto de una composición entre dominio y agente sigue abierto. El experto constituye su significado y las actuaciones admitidas; la realización técnica debe hacer exigibles las reglas aplicables. Esta auditoría no decide relaciones, conectores, prioridades ni composiciones de inmunología, ciberseguridad u otros dominios.

**Estado del trabajo:** contraste documental y caracterización terminados en el alcance siguiente. Ejecución algebraica completa, fidelidad de pantalla, seguridad integral y cierre del núcleo: no acreditados por esta pieza.

## 1. Cortes y fuentes

| Repositorio | Rama | Corte examinado |
|---|---|---|
| SV-lenguaje-de-computacion | main | `e611218a8c6c04ade022b867018b84a7f3e6700e` |
| SV-matematica-semantica | main | `b8fd32978292d25adf9b87cf71e409005dce642c` |
| SV-matematica-semantica-cuaternaria | lab/playground-sv-permanente | `3fbf00f5f046c2d98d096ff51ec6c162ac5ca1ae` |

Se recuperaron textos completos de Fundamentos y de la serie I–VI en el repositorio del autor. El PDF índice aportado no sustituye esos desarrollos. [FUENTES.json](FUENTES.json) identifica 88 archivos recuperados con commit, ruta, blob Git, SHA-256 y tamaño; es un inventario de recuperación, no una afirmación de auditoría exhaustiva de cada línea. Se contrastaron los pasajes identificados abajo, los documentos rectores y la representación, las guardas y las entradas públicas pertinentes de `sv_core`.

Fuentes rectoras: [Pilares](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e611218a8c6c04ade022b867018b84a7f3e6700e/docs/calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), [perfiles y ensamblaje, §§6–12](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e611218a8c6c04ade022b867018b84a7f3e6700e/docs/calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md), [secuencia de transición, §13, filas 9–14](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e611218a8c6c04ade022b867018b84a7f3e6700e/docs/dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md), [deuda viva](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e611218a8c6c04ade022b867018b84a7f3e6700e/docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md) e [IR vigente v0.3](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e611218a8c6c04ade022b867018b84a7f3e6700e/IR_CANONICA_BIENFORMACION_SV_v0_3.md).

## 2. Lo que debe conservarse

- SV no es un espacio vectorial. Se declara `b ≥ 3` y se deriva `n = b²`; cada posición toma un símbolo de `{0,1,U}`. El número de configuraciones es `3ⁿ`, no `bⁿ`. Un array es una realización de almacenamiento, no una redefinición del objeto.
- En el ejemplo SV(9,3), el vector ordenado `(0,1,U,0,U,1,0,1,U)` determina los nueve estados. La representación polar emplea posiciones identificadas y una codificación radial explícita. El radio de U no convierte U en número algebraico.
- Deben conservarse identidad, orden, estado, contenidos vinculados y procedencia exigidos por la operación. Una forma dibujada sin esos vínculos no acredita toda la célula ni su significado.
- Las leyes generales no imponen una misma composición a todos los dominios. La relación semántica se declara antes de elegir el patrón y la operación. La selección concreta y su reparto dominio/agente no se resuelven aquí.
- La salida profesional concierne al especialista de cada dominio. Los ejemplos históricos IMMUNO-1/2 son antecedentes visuales, no autorización para reconstruir sus parámetros o promover una interfaz actual.

## 3. Correspondencia entre doctrina y realización

**Lectura de la tabla:** “representado” significa que hay un objeto o una operación identificable; “validado” significa que una guarda comprueba la condición indicada; “ejecutado” exige producir y comprobar el resultado de la ley. No son estados equivalentes.

| Familia y fuente | Soporte localizado | Límite y obligación pendiente |
|---|---|---|
| Célula y estados — Fundamentos §§3–5 | `frontend.rs`, `nat.rs`, `ir.rs`, `wellformed.rs`: b, n derivado, alfabeto, longitud y orden. Pruebas existentes `cell_geometry_native.rs`. | No confundir geometría y almacenamiento con ejecución de clasificación ni reconocimiento visual. |
| Evaluación local — Fundamentos §5; IR v0.2 J3.1 y §6.2 | `Evaluate`, proyecciones permitidas y comprobación de fuente evaluable. Caso 01. | Se emiten referencias a recuentos y umbral; no se producen los valores de `EvalResult`. Fila 10: realizar recuentos, umbral, clasificación e interpretación terminal tipada, conforme al contrato aplicable. |
| Conector y transmisión — I §§4–6 | Mapa completo, sin claves repetidas, codominio y posición compatibles. Casos 02–03. | Falta ejecutar el transporte desde una salida concreta, conservando productor, conector y receptor. No inventar el mapa. |
| Actualización por puente — I §4; IR J2.2 | `CoupledState` impide cambiar posiciones ajenas a `bridges`. Casos 10–11. | Un cambio dentro del puente se admite sin procedencia material. Debe enlazarse al resultado y al conector que lo produjeron. DFL-001. |
| Grafo simple — I §6, RS1–RS4 | Guardas de nodos, aristas, posiciones, compatibilidad, aciclicidad y unicidad del destino. Casos 06, 07 y 09. | Aceptar el grafo no demuestra evaluación topológica ni completitud ejecutiva de cada motor local. |
| Concurrencia general — I §§6 y 8 | Campo `General`; se mantienen varias guardas comunes al grafo. Caso 08. | No hay declaración completa ni imposición del operador de conflicto y prioridad exigidos por RG1. No es régimen general acreditado. Las familias de resolución descritas no autorizan a elegir una automáticamente. DFL-001. |
| Compuerta — II §9 | Tabla explícita, pertenencia, totalidad del producto y compatibilidad posicional; operación `Gate`. Casos 04–05. | No se calcula `GateResult.output`. Totalidad estructural no prueba pertinencia de la relación semántica. DFL-001/006. |
| Máximo y mínimo — Fundamentos §7; II §9.7; IR §6.1 | La especificación prevé reducción a tabla bajo orden total documentado. | No se localiza en la API algebraica examinada una ejecución de esa reducción. El orden de la lista de un codominio no acredita orden ordinal, compatibilidad de roles ni una ley universal. |
| Supervisión y criticidad — II §§10–13; IR J3.3–J3.4 | Tipos y referencias de supervisión; comprobaciones de rol y objetivo. | Faltan productores de criticidad y consecuencias ejecutivas del dictamen. `Veto` no equivale a anulación material demostrada. La criticidad global y parte del objeto supervisado siguen abiertas en la doctrina. DFL-006. |
| Constructor multinivel — II §11 | `Compose` conserva grafo, referencias a relaciones y patrones. Caso 12. | Constructor de grado C: no hay firma universal cerrada. Debe preservarse ese estatuto; ni admitir referencias ejecuta Comp ni su apertura permite inventar leyes. |
| Sucesos, frames y trayectorias — III | `frame.rs`, `transition_data_wellformed.rs`, `context_wellformed.rs`: cierre de referencias y controles de transición/contexto. | Referencias resueltas no son resultados algebraicos calculados. Reproducción de trayectoria exige datos suficientes y operador inducido constituido. No convertir cada consulta en suceso. DFL-004/006. |
| Transducción, uso y análisis — IV, V y VI | Captura/admisibilidad y ligaduras disponen de controles materiales parciales; `Ternarizer` conserva nombres; dominio/agente/consulta tienen representación y validación contextual. | IR v0.3 §2.4 no habilita ternarización ejecutable. Los nombres no prueban partición total y disjunta. V especializa sin alterar leyes; VI ofrece análisis discretos, no un intérprete ya materializado. DFL-003/005; no se auditan aquí todas sus proposiciones matemáticas. |

Referencias de implementación: [IR Rust](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e611218a8c6c04ade022b867018b84a7f3e6700e/rust/sv_core/src/ir.rs), [bienformación](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e611218a8c6c04ade022b867018b84a7f3e6700e/rust/sv_core/src/wellformed.rs), [entrada pública](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e611218a8c6c04ade022b867018b84a7f3e6700e/rust/sv_core/src/lib.rs), [cierre de Frame](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e611218a8c6c04ade022b867018b84a7f3e6700e/rust/sv_core/src/frame.rs). Los controles de ejecución de efectos de R1 cumplen otra función y no deben contarse como evaluador de estas leyes.

Fuentes algebraicas: [Fundamentos](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/fundamentos/README.md), [índice I–VI](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/composicion/README.md), [I](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/composicion/I_transmision_serie_por_puente.md), [II](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/composicion/II_gramatica_general_composicion.md), [III](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/composicion/III_horizonte_sucesos_reevaluacion_discreta.md), [IV](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/composicion/IV_transduccion_alfabeto_ternario_interfaz_parametrica.md), [V](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/composicion/V_invariantes_agentes_operador_consulta.md), [VI](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/composicion/VI_analisis_discreto_representaciones_secuencias.md).

## 4. Testigos reproducibles y revisión adversarial

El [plan fijado](PLAN_FIJADO.json) contiene doce casos sintéticos, sin contenido clínico ni constitución de un dominio. Se compiló **el `sv_core` existente, sin modificar sus fuentes**, con Rust 1.98.0, y se invocó su API pública desde [sonda.rs](sonda.rs).

El ensayo final registra **12/12 coincidencias con la caracterización prevista**, siete admisiones y cinco rechazos semánticos exactos. Son resultados de caracterización: las admisiones de 08 y 10 documentan insuficiencias; no son conformidad con el álgebra. [Resultados](evidencia/RESULTADOS.json), [comandos y capturas](evidencia/COMANDOS.json), [identidad de realización](evidencia/REALIZACION.json).

Un primer montaje empleó erróneamente `0/1` en posiciones donde la superficie EN exige `Zero/One`. Falló antes de alcanzar las guardas pretendidas. Se conserva en [antecedente-sintactico](antecedente-sintactico/COMANDOS.json); no se contabilizan sus rechazos como evidencia semántica. Se corrigieron sólo los testigos y se exigió el rechazo textual preciso en cada negativo. Dos montajes de 15 procesos, 30 en total; un solo modo nativo de compilación. No es una nueva campaña exhaustiva nativo/WASI/navegador ni una prueba de independencia semántica.

Hallazgos y controles de interpretación:

1. **Falso cierre por etiqueta.** El caso 01 emite `result_type: EvalResult` y operaciones de proyección. No emite recuentos calculados ni el umbral. En este vector hay siete ceros, un uno y una U; para n=9, T=7. Esa comprobación aritmética externa ilustra qué dato falta, sin adjudicar significado a las etiquetas sintéticas A/B.
2. **Concurrencia sin ley de resolución.** 06 admite el control simple; 07 rechaza la segunda entrada al mismo puente; 08 admite exactamente esa concurrencia al cambiar a General, sin operador ni prioridad. Corrobora DFL-001, no demuestra una resolución implícita.
3. **Posición sin procedencia.** 10 admite cambiar U a One dentro del puente incluso sin conector declarado; 11 rechaza el cambio fuera de él. La guarda posicional existe y la cadena productora no queda acreditada.
4. **Tabla completa sin resultado.** 04 admite tabla y llamada, 05 rechaza una fila ausente. No se fabrica una salida para llenar el hueco ejecutivo.
5. **Reconciliación de versiones.** El tratamiento de admisibilidad y fallo del IV histórico se lee con las rectificaciones vigentes: IR v0.3 separa fallo de captura y U. No copiar al código una conversión histórica automática de fallo a U.
6. **Límite del corpus.** La serie II distingue grado A, grado B y grado C. El [Pliego §6.6](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/docs/gobierno/PLIEGO_DE_CONDICIONES_DEL_SISTEMA_VECTORIAL_SV.md) también excluye una firma universal cerrada de Comp. Esta lectura no certifica exhaustividad ni consistencia de todas las proposiciones del corpus.

Reproducción: recupérese `rust/sv_core` del commit indicado, verifíquense sus fuentes contra el manifiesto y ejecute `python ejecutar.py RUTA_RUSTC RUTA_SV_CORE DIRECTORIO_RESULTADOS`. Python sólo conduce procesos y compara resultados; no constituye una segunda semántica SV. Los binarios se regeneran; se conservan sus huellas. La ruta local de la fuente aparece en la proyección, por lo que una reproducción en otra carpeta cambia ese metadato sin alterar el programa.

## 5. Respuesta acotada a las dos frases sobre evidencia

**“Ahora debemos conocer qué evidencia.”** La conversación ya aportó evidencia concreta de cinco cambios entre seis imágenes: se retiraba contexto y, en el último paso descrito, también lava y cráter. Los relatos recuperados permiten reconstruir esa secuencia documental. No es necesario volver a pedir al autor que invente un ejemplo o el comprobador técnico.

**“No demuestra que sepamos identificarla de forma suficiente.”** La suficiencia debe vincularse a una operación y sus condiciones. En el ejemplo, conservar la identificación a través de una secuencia documentada y describir qué muestra la última imagen aislada son preguntas distintas. La desaparición de rasgos no demuestra por sí sola que el objeto de origen dejase de ser volcán, ni permite atribuir una causa de ataque a la IA. El sistema debe conservar esa distinción y el origen de la afirmación. En SV, la obligación concreta es comprobar la correspondencia entre estados, operaciones y representación admitida, sin recuperar por invención lo omitido. Los ensayos RETP-149/150 cubren custodia literal y formato; esta auditoría localiza qué soporte algebraico falta. Ninguno acredita por sí solo verdad de origen ni suficiencia universal para todos los dominios.

## 6. Continuidad en seis pasos, sin trasladar la deuda a los agentes

1. **Fijar fuentes y alcance:** completado en este corte; cada ley recibe fuente y estatuto.
2. **Distinguir ley común de configuración de dominio:** completado como frontera de trabajo; el reparto particular dominio/agente continúa pendiente.
3. **Contrastar Rust y caracterizar guardas:** completado para los doce testigos y familias descritas; no equivale a ejecución algebraica completa.
4. **Recoger carencias sin cambiar doctrina:** completado como recepción en DFL-001/003/004/005/006/013; no se cierran esas deudas ni se multiplican identificadores.
5. **Retomar la secuencia activa:** conservar la campaña acotada de tuberías de IA y su workflow V2; incorporar sus causas al catálogo/localización al cerrar ese tramo, continuar fila 9 y llegar a la puerta algebraica de fila 10. Esta auditoría autorizada anticipa el inventario necesario, no reordena por sí sola la secuencia rectora ni abre reserva o P4/P5.
6. **En la puerta algebraica:** comenzar por evaluación local y resultados tipados; después conectar resultados con transmisión y tablas explícitas, con testigos de resultado y de pérdida de procedencia. Antes de ejecutar General, exigir representación y resolución de concurrencia; si no existe, la operación dependiente no podrá acreditarse como ejecutable. Eliminar esa capacidad del alcance exigiría una exclusión expresa y su consecuencia para el cierre. Comp y extensiones abiertas conservan sus reservas. La comprobación independiente exigida por DFL-013 debe acompañar el subconjunto admitido.

**Siguiente entrega técnica:** correspondencia acotada con operación gobernada y procedencia de sus referentes dentro del workflow vigente, usando lo ya construido. El inventario aquí fijado se conserva como entrada obligatoria de la puerta algebraica. La aceptación final humana permanece donde estaba; no se solicita de nuevo para esta auditoría ni se da por concedida para cerrar capacidades todavía no probadas.
