# Continuación · 15 de septiembre de 2026

**Recepción vigente — S22:** [Acta 002 §16](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/continuacion-15-09-2026/ACTA_002_RECEPCION_COTEJO_IDENTIDAD_Y_CONTRATO_LEYENDA_R06_2026_09_15.md#recepcion-leyenda-r1-corregido). Subsanación del reparo R1 de leyenda recibida en 6d73c376: comparación de todos los pares solapados; 18 pruebas y compilación release repetidas; contraejemplo histórico y control sin empate conformes en sus seis permutaciones. Reconocedor no cualificado. Verificar disponibilidad e identidad del TTF contratado y PNG R01/R06 en su custodia antes de preparar la cualificación conjunta sobre las 27 celdas congeladas. Mantener pendientes códec, correspondencia de píxeles y empate residual. S32 conserva la revisión de privacidad separada. La [regla de referencias de Acta 001 §9](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/continuacion-15-09-2026/ACTA_001_CONTINUIDAD_Y_RUMBO_2026_09_15.md#9-referencias-inequívocas-en-la-continuación) distingue fases, reparos y parámetros. Las entradas anteriores conservan su corte histórico.

## Estado vigente y accesos

**Privacidad y seguridad — S32, en ejecución:** [parte de trabajo y alcance](PARTE_ALCANCE_PRIVACIDAD_SEGURIDAD_Y_OP_CYB_001_2026_09_15.md). Correspondencia con OP-CYB-001; contratos y pruebas pendientes.

**Punto de continuación:** La disponibilidad e identidad del TTF contratado y de los PNG R01/R06 quedaron comprobadas en [Acta 002 §17](ACTA_002_RECEPCION_COTEJO_IDENTIDAD_Y_CONTRATO_LEYENDA_R06_2026_09_15.md#custodia-ttf-r01-r06) y en la [nota de comprobación](COMPROBACION_CUSTODIA_TTF_R01_R06_2026_09_16.md). Preparar el precompromiso de la cualificación conjunta sobre las 27 celdas congeladas; Q1/Q2 y E1–E16 siguen sin ejecutar. Mantener pendientes códec, correspondencia geométrica, E1 y empate residual. S32 conserva la revisión de privacidad separada.

| Materia | Estado y documento de consulta |
|---|---|
| Rumbo y dependencias | [Acta 001](ACTA_001_CONTINUIDAD_Y_RUMBO_2026_09_15.md). Su actualización de remisiones identifica las recuperaciones posteriores. |
| Leyenda R06 | [Acta 002](ACTA_002_RECEPCION_COTEJO_IDENTIDAD_Y_CONTRATO_LEYENDA_R06_2026_09_15.md). Recepciones sucesivas /2, /3 y /4; el §9 concluye la subsanación documental y precisa el alcance del cotejo; el §8 conserva el control documental. |
| Corrección S31 | [Parte y recepciones posteriores](PARTE_S31_CORRECCION_ACOTADA_Y_TRAZABILIDAD_2026_09_15.md). Finalizado; PNG y ZIP S6 recuperados e identificados. La exposición declarada de la referencia S6 condiciona una futura evaluación ciega. |
| Sucesos | [Estado vigente](../../Inventario-sv/sucesos/SUCESOS_SV.md), [CSV](../../Inventario-sv/sucesos/SUCESOS_SV.csv) e [historial](../../Inventario-sv/sucesos/HISTORIAL_SUCESOS_SV.csv). S22 y S26 en ejecución; S31 finalizado. |
| Continuidad general | [Léame primero](../frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md). Las entradas anteriores conservan su fecha y alcance histórico. |
| Mapa histórico | [HTML conservado](mapa/MAPA.html). Las precisiones y recuperaciones posteriores constan en el parte S31; no se regenera el mapa. |

El estado vigente se consulta en el registro y en la última recepción aplicable. Las menciones anteriores a archivos pendientes describen sus respectivos cortes; no sustituyen las recuperaciones posteriores.

## Evidencias recuperables

| Comprobación | Fuente | Entradas y resultado |
|---|---|---|
| S31: identidad y contraste documental | [COTEJO_S31.rs](COTEJO_S31.rs) | [Entradas públicas](ENTRADAS_S31.tsv) · [Salida](COTEJO_S31_SALIDA.txt) |
| S6: recepción del ZIP | [Fuente](COTEJO_S31_RECEPCION_ZIP_S6.rs) | [Registro conservado](COTEJO_S31_RECEPCION_ZIP_S6_SALIDA.txt) · procedencia privada en el parte S31 |
| S6: contenido y campos compartidos | [Fuente](COTEJO_S31_CONTENIDO_S6.rs) | [Resultados](COTEJO_S31_CONTENIDO_S6_SALIDA.txt) · entradas fijadas en el parte S31 |
| Recepción de leyenda /3 | [Fuente](COTEJO_RECEPCION_LEYENDA_03.rs) | [Registro conservado](COTEJO_RECEPCION_LEYENDA_03_SALIDA.txt) |
| Recepción de leyenda /4 | [Fuente](COTEJO_RECEPCION_LEYENDA_04.rs) | [Versión, comandos y salida](COTEJO_RECEPCION_LEYENDA_04_SALIDA.txt) |
| Recepción de la adenda /4 | [Fuente](COTEJO_RECEPCION_ADENDA_04.rs) | [Versión, comandos y salida](COTEJO_RECEPCION_ADENDA_04_SALIDA.txt) |
| Control de continuidad | [Fuente](CONTROL_CONTINUIDAD.rs) | [Entradas](ENTRADAS_CONTROL_CONTINUIDAD.tsv) · [Salida](CONTROL_CONTINUIDAD_SALIDA.txt) |

El [depósito de leyenda /4 y su adenda](https://github.com/juantoniolloretegea/SVperitus-dataset/tree/23fcae974badc52360493e62346dda21a5976a78/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06) conserva sus dieciséis archivos y sus versiones anteriores. Su manifiesto y las recepciones identifican cada evidencia; los testigos reservados mantienen su sede privada.

## Rust, reproducción y alcance de los registros

Las comprobaciones actuales se realizan en Rust 1.98.0. Cualquier excepción para Python requiere justificación previa y alcance inocuo, conforme a S29 y a la instrucción de la Dirección. La presencia de archivos Python en la custodia histórica S6 no acredita ni autoriza su ejecución actual.

Los cotejos S31 que descomprimen contenedores requieren flate2 1.1.9 y las dependencias fijadas del paquete embebido en el HTML. El parte S31 conserva sus instrucciones. Los cotejos receptores /3, /4 y el control de continuidad no requieren dependencias externas.

Los registros antiguos que contienen órdenes parametrizadas, retornos y resultados se consultan como tales: no acreditan por sí solos una transcripción literal completa del proceso. Las huellas observadas posteriormente no se presentan como compromisos previos. No se reconstruyen registros históricos.

Para repetir el control de continuidad, obtener los archivos del corte `146cb30e3cece036947081453d7e0982ca03a1cb`; adaptar únicamente las rutas locales de la primera columna de ENTRADAS_CONTROL_CONTINUIDAD.tsv, conservando ruta canónica, tamaño y blob. Generar la lista de rutas mediante `git ls-tree -r --name-only 146cb30e3cece036947081453d7e0982ca03a1cb`, dentro del repositorio correspondiente. Compilar CONTROL_CONTINUIDAD.rs con Rust 1.98.0 y ejecutar con cinco argumentos: manifiesto, lista de rutas, CSV, historial y Markdown del mismo corte. La comprobación de enlaces se limita a rutas relativas Markdown; no verifica anclas ni descarga destinos externos.

## Antecedente de apertura conservado

[Acta 001: continuidad y rumbo](ACTA_001_CONTINUIDAD_Y_RUMBO_2026_09_15.md) · [Mapa HTML](mapa/MAPA.html).

Las nuevas actas de Calidad se incorporan en esta carpeta. Las rutas anteriores se conservan. Para seguir la secuencia, rige el Acta 001 y su recepción en Sucesos S30; la secuencia automática de GUI anterior queda cancelada.

La [adenda recibida](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/23fcae974badc52360493e62346dda21a5976a78/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/ADENDA_CIERRE_DOCUMENTAL_ENTREGA_04.md) y su [manifiesto](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/23fcae974badc52360493e62346dda21a5976a78/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/MANIFIESTO_ADENDA_ENTREGA_04.tsv) conservan sus bytes de origen. Para el alcance de la igualdad declarada se aplica la precisión receptora de Acta 002 §9.3. No se afirma traslado material al laboratorio.
