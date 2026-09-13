# Estudio (p1+p3)-Bis: célula matemática, imagen y agentes

**Referencia vigente: versión 2 · S21 · 13 de septiembre de 2026 · RETP-2026-195**

[Historial y alcance de la revisión 2](REVISION_V2.md). La recepción inicial S20 / RETP-2026-193 se conserva como antecedente.

[Explicación para terceros — Markdown](../CELULA_IMAGEN_Y_AGENTES_EN_EL_SV_P1_P3_BIS_2026_09_13_V2.md) · [PDF](../CELULA_IMAGEN_Y_AGENTES_EN_EL_SV_P1_P3_BIS_2026_09_13_V2.pdf) · [Hallazgos](HALLAZGOS.md) · [Identidades de fuentes](FUENTES.json) · [Comprobaciones](VERIFICACION.json).

## Trabajo vigente: S22 · RETP-2026-214

**Workflow V2: definición cerrada; ejecución en curso.** [Markdown](WORKFLOW_P1_P3_BIS_v2.md) · [PDF](WORKFLOW_P1_P3_BIS_v2.pdf) · [Estado](ESTADO_WORKFLOW.json) · [Resultado BIS-01 e inicio BIS-02](RESULTADO_BIS_01_Y_APERTURA_BIS_02.md).

BIS-00 y BIS-01 finalizados en sus alcances documentales. BIS-02 en ejecución: [contrato candidato](CONTRATO_CANDIDATO_BIS_02_v0_1.md), [banco preliminar histórico de 24 escenarios](BANCO_PREVIO_BIS_02_v0_1.json) y [matriz de doce obligaciones](MATRIZ_BIS_01.json). S13 ya fijaba la denominación precisa de gramática 0.2, IR 0.3 y obligaciones semánticas en Frontera/IR/adendas; se corrige el pendiente nominal de localizar «semántica V0.2» independiente.

Se conservan [workflow V1](WORKFLOW_P1_P3_BIS_v1.md), [radiografía inicial](RADIOGRAFIA_INICIAL_BIS_01.md), [política Rust ES/EN](DOCUMENTACION_RUST_ES_EN_v1.md) y antecedentes. La revisión estática no acredita pruebas Rust. El [contraste nativo BIS-C01](bis-c01/RESULTADO.md) ejecuta dos escenarios en trece variantes: 13/13 conformes y 4/4 sensibilidades. Los otros 22 escenarios siguen pendientes. [BIS-C02](bis-c02/README.md) concreta contrato candidato y 14 variantes documentales, cero ejecutadas. [BIS-C03](bis-c03/README.md) añade contrato candidato y 16 variantes documentales, cero ejecutadas, y precisa el alcance ya integrado de LIG/0.1. [BIS-C04](bis-c04/README.md) incorpora contrato y 20 variantes previas sobre geometría simbólica exacta, cero ejecutadas. [BIS-C05](bis-c05/README.md) añade contrato y 16 especificaciones documentales, cero ejecutadas. El [criterio de aceptación Rust](bis-c05/CRITERIO_DE_ACEPTACION_RUST_v1.md) distingue preparación Python, compilación y recepción/validación ejecutadas en Rust. [BIS-C06](bis-c06/README.md) añade contrato y 18 escenarios especificados, cero ejecutados, y distingue instantáneas de custodia histórica. [BIS-C07](bis-c07/README.md) añade contrato, 18 fixtures SVP y oráculos para serie/compuerta entre 16/25 posiciones, cero ejecutados. [BIS-C08](bis-c08/README.md) concreta conservación de Tri.U y separación de fallos: veinte escenarios, seis fuentes SVP parciales, cero ejecutados. [BIS-C09](bis-c09/README.md) concreta contenido frente a autoridad, con veinte escenarios y once documentos sintéticos, cero ejecutados. [BIS-C10](bis-c10/README.md) concreta justificación y evidencia, con veinte escenarios y ocho archivos sintéticos, cero ejecutados. [BIS-C11](bis-c11/README.md) añade presupuesto sintético y veinte escenarios de recursos, cuatro entradas textuales, cero ejecutados. Continúa BIS-C12: perfiles ES/EN, documentación y construcción; antes de realizar código se completarán los contratos y se decidirán sus sedes conforme al workflow. La GUI queda para después del Bis, el catálogo y el cierre de fase.

## Concordancia de BIS-01 · S22 / RETP-2026-198–199

[Suceso, frame, representación y operaciones SV](CONCORDANCIA_SUCESO_FRAME_REPRESENTACION_Y_OPERACIONES_SV.md) · [Fuentes](FUENTES_CONCORDANCIA_SUCESO_FRAME.json). Contraste del Documento III, Documento V y piezas pertinentes. Se recibe expresamente: «el suceso es el hecho; la prosa, la matemática y la imagen son formas de representarlo». Se reciben las precisiones del autor y se delimitan las decisiones pendientes; no se identifican por nombre una actividad técnica, un suceso del horizonte y un frame.

## Ampliación de BIS-01 · S22 / RETP-2026-197

[Contraste de antecedentes y biblioteca base](CONTRASTE_ANTECEDENTES_COMPOSICION_Y_BIBLIOTECA_SV.md) · [Identidades](FUENTES_ANTECEDENTES_COMPOSICION.json). IMMUNO-2 histórico y SV-ADC se reciben exclusivamente como ejemplos de células y composición. La revisión distingue puente en serie, compuerta heterogénea y meta-supervisión, así como biblioteca base, primitivas y contratos particulares. No se importa un dominio ni se ejecuta el demostrador.

## Criterio rector y alcance

Los [Fundamentos algebraico-semánticos del Sistema Vectorial SV.md](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/fa3eb727799c322090e4e9126f81238cd198b9cc/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/Fundamentos%20algebraico-sem%C3%A1nticos%20del%20Sistema%20Vectorial%20SV.md) rigen el contraste: objeto exacto ternario, n = b² con b ≥ 3, significado canónico, radios, evaluación determinista y composición tipada. La representación y las implementaciones deben responder a esos fundamentos. El código sintético no redefine el álgebra.

El autor ha precisado que las células de conocimiento nuclear del dominio y los parámetros singulares de decisión comparten molde de dibujo. Se recibe esta premisa expresamente. Su función, constitución y relaciones deben seguir identificadas. SVperitus es la sede natural de agentes; al Lenguaje corresponden las obligaciones que afecten a representación, conservación, validación o ejecución.

La recepción inicial entregó la explicación y el contraste de antecedentes. El estudio de suficiencia de las obligaciones semánticas y de IR 0.3 continúa en S22 según el estado vigente indicado arriba. No se declara cerrada la paridad integrada, no se entrena IA y no se modifica núcleo, IR, Rust, parámetros de dominio ni SVperitus. El orden posterior de agentes se decidirá tras el trabajo inmunológico correspondiente.

## Fuentes y cortes de la recepción inicial S20

| Fuente | Corte o identidad | Lectura efectuada |
| --- | --- | --- |
| Lenguaje: rectores y antecedentes iniciales | 19540321089dc48e4239e1f88324d1056c8caff4 | AGENTS, Pilares, perfiles, transición, entrada de lectura y registros; antecedente y SVG. |
| Nuevas aportaciones del autor | fa3eb727799c322090e4e9126f81238cd198b9cc | Fundamentos completos; secciones de definición, geometría, evaluación y competencia señaladas en hallazgos. La recepción de un archivo no equivale a revisión íntegra de todos sus apartados. |
| SVperitus-dataset | 47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c | Presentación, Documento 7 §§1.2, 2, 6 y 7, configuración, entrenamiento, evaluación, generador común y compositor IMMUNO-1/2. Revisión estática. |
| Laboratorio: corte de entrada | 59ff0937c3c7d50f390116f05baacb2f91a37b38 | Rama lab/playground-sv-permanente, para continuidad y espejo. |
| Impresión aportada | SHA-256 en FUENTES.json | Siete páginas de índice, resúmenes y enlaces de la colección. No contiene íntegros sus capítulos. |

El PDF de la colección se conserva bajo el nombre cargado por el autor y como copia del adjunto en antecedentes. Los capítulos Markdown añadidos después se conservan por separado; amplían la evidencia disponible. No se atribuye lectura de capítulos completos a la impresión del índice.

## Resultado del contraste inicial S20

Los radios canónicos quedan identificados: 0 → 1, 1 → 2 y U → 3. Se distingue el símbolo de estado, su radio y el índice numérico de una clase del modelo. Se identifican diferencias de orientación y discrepancias de redacción, significado, clasificación y código. Cada una queda documentada con fuente, alcance y consecuencia en HALLAZGOS.md.

La ejecución realizada se limita a un contraejemplo de tres índices del evaluador; no ejecuta PyTorch, entrenamiento ni inferencia. Los 3/3 y 1/3 del testigo no son métricas de una IA. El PDF explicativo se ha renderizado e inspeccionado visualmente.

## Continuación necesaria antes de derivar al catálogo

1. Aplicar los acuerdos de la versión 2 sobre frame tipado, dimensión fija y tamaños admitidos por versión; determinar ese conjunto desde las constituciones competentes. Fijar el contrato de paridad desde los fundamentos: célula, coordenadas, símbolo, radio, orden y transformación de representación; distinguir imagen geométrica de su rasterización.
2. Contrastar contra semántica V0.2 e IR 0.3 la preservación de identidad, constitución, evaluación, composición, evidencia y revisión humana. Una carencia exige testigo y sede del remedio antes de proponer cambio.
3. Resolver las discrepancias de los antecedentes que vayan a reutilizarse, manteniendo originales y versiones. No asumir una migración por parecido gráfico.
4. Preparar controles positivos y alteraciones discriminantes: posición permutada, estado cambiado, leyenda incorrecta, imagen de otra instancia, transformación no declarada y correspondencia de clases equivocada.
5. Determinar si NLP y visión compartirán realización; exigir evidencia de las capacidades y del canal realmente consumido. Mantener aislamiento de instrucciones externas y autoridad del motor normativo.
6. Derivar al catálogo únicamente causas delimitadas, con condiciones de emisión, evidencia y distinción entre fallo técnico, no admisión e indeterminación válida.

**Estado de S20:** finalizado sólo para recepción y explicación documental. La suficiencia integrada y la decisión sobre extensiones del núcleo no quedan acreditadas por este cierre.

## Revisión explicativa vigente y siguiente objeto

S21 cierra la actualización documental, con MD y PDF de once páginas y la entrada de relevo actualizada. El trabajo material continúa abierto: suficiencia de semántica V0.2 e IR 0.3, tamaños constituidos, contrato de paridad, banco de pruebas y capacidades efectivas de NLP/visión. SV(9,3) conserva su condición de mínimo y no se emplea como tamaño predeterminado ni como sustituto de cargas mayores.

Las diferencias entre antecedentes pueden proceder de convenios de distintas etapas. Se conservan las observaciones y los originales; antes de reutilizarlos debe declararse su relación con el convenio vigente.

[Revisión y versiones](REVISION_V2.md) · [Comprobaciones V2](VERIFICACION_V2.json) · [Manifiesto V2](MANIFIESTO_V2.json). El manifiesto original identifica la primera entrega, incluido el README de aquel corte; el historial Git permite recuperarlo.
