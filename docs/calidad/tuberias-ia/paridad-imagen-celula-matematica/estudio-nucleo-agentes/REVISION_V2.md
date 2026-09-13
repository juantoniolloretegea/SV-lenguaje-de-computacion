# Revisión 2: frame tipado, dimensión fija y alcance por versión

**(p1+p3)-Bis · S21 · RETP-2026-195 · 13 de septiembre de 2026**

Referencia explicativa vigente: [Markdown V2](../CELULA_IMAGEN_Y_AGENTES_EN_EL_SV_P1_P3_BIS_2026_09_13_V2.md) · [PDF V2](../CELULA_IMAGEN_Y_AGENTES_EN_EL_SV_P1_P3_BIS_2026_09_13_V2.pdf). Antecedente conservado: [Markdown de la primera edición](../CELULA_IMAGEN_Y_AGENTES_EN_EL_SV_P1_P3_BIS_2026_09_13.md) · [PDF de la primera edición](../CELULA_IMAGEN_Y_AGENTES_EN_EL_SV_P1_P3_BIS_2026_09_13.pdf).

## Objeto y procedencia

La revisión incorpora las precisiones expresadas por Juan Antonio Lloret Egea en la conversación posterior a la primera edición y su instrucción de trasladarlas a ambos formatos y a la entrada de lectura. Distingue esas decisiones de diseño de las propiedades documentadas de Rust y de las garantías cuya realización aún debe comprobarse. La explicación mantiene la primacía de los fundamentos algebraico-semánticos y conserva los antecedentes históricos.

## Cambios incorporados

| Apartado | Precisión |
| --- | --- |
| 2.1 | Los convenios históricos se identifican por versión y contexto. Las diferencias se conservan para el contraste y no redefinen silenciosamente la convención canónica. |
| 2.2 | Frame constituido y tipado, conceptualmente `(frmat, frvis)`, con validez conjunta y correspondencia de instancia, estado, constitución y contrato. Materialización de la imagen sujeta a una modalidad declarada. |
| 2.3 | Dimensión fija por célula admitida; conjunto finito de tamaños soportados por versión, cuyo contenido aún debe constituirse. Extensiones con revisión de impacto y pruebas; cambios de semántica o IR cuando resulten necesarios. |
| 2.4 | Arrays y Vec en Rust; encapsulación y construcción válida. SV(9,3) como mínimo sin valor predeterminado. Previsión de uso escaso diferenciada de frecuencia medida. Cobertura de dimensiones mayores, composiciones y recursos totales. |
| 7 y 8 | Estado documental y fuentes actualizados, con referencia a Rust y a esta revisión. |

La dimensión no atribuye autoridad a una célula de guarda. Esa autoridad procede del rol y del contrato. Los tamaños de los ejemplos no se convierten en una lista normativa cerrada. La familia matemática conserva `n = b²`, con `b ≥ 3`, sin fijar aquí un máximo universal.

## Continuidad y alcance del cierre

S20 conserva el cierre de la primera recepción documental. S21 finaliza la revisión explicativa y su incorporación a las entradas de continuidad. Este cierre no acredita un tipo Frame implementado, suficiencia de semántica V0.2 e IR 0.3, paridad integrada, rendimiento ni aptitud de una IA. No se han modificado Rust, el núcleo, la IR, las constituciones de dominio o los agentes.

La siguiente unidad debe leer [Léame primero](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md), el [estado de Sucesos SV](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv), su [historial](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/HISTORIAL_SUCESOS_SV.csv) y el [expediente](README.md). El registro determina actividad, estado y evidencia; la entrada de lectura determina la ruta documental. Las referencias antiguas mantienen su corte histórico.

El siguiente objeto material consiste en contrastar las obligaciones de constitución y paridad con semántica V0.2 e IR 0.3, identificar los tamaños desde las constituciones competentes y fijar el contrato y el banco de comprobación. Toda insuficiencia requiere un caso discriminante y la identificación de la sede del remedio. Después podrán derivarse al catálogo las causas delimitadas. La elección de una realización conjunta o separada para NLP y visión conserva su necesidad de evidencia. La secuencia posterior de agentes se decidirá tras el trabajo inmunológico correspondiente.

## Cortes y conservación

| Repositorio | Corte de entrada de S21 |
| --- | --- |
| Lenguaje, main | `d17851d8da1279ace6184d3efcc14b120ac6275d` |
| Laboratorio, lab/playground-sv-permanente | `fbb8dc8648bac12f6f733dadfb4eb960c84df544` |

Los fundamentos recibidos en `fa3eb727799c322090e4e9126f81238cd198b9cc` y los antecedentes de SVperitus en `47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c` conservan las identidades consignadas en [FUENTES.json](FUENTES.json). La apertura S21 se publicó en Lenguaje `e9b749a732d9fb37caf7ac0e7b6517143d5d5e69` y laboratorio `bad4d56753ee6938b0a4362133952333b11d9c55`.

La primera edición, los originales y sus comprobaciones permanecen conservados. [MANIFIESTO.json](MANIFIESTO.json) describe el corte de la primera entrega; el README se actualiza con historial Git. [MANIFIESTO_V2.json](MANIFIESTO_V2.json) identifica esta revisión y [VERIFICACION_V2.json](VERIFICACION_V2.json) delimita las comprobaciones documentales realizadas. Los resultados del testigo de índices de S20 no se presentan como una ejecución nueva ni como métricas de una IA.
