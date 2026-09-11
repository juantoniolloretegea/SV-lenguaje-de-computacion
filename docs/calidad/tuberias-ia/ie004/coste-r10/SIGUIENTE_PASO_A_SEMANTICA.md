# Siguiente objeto acotado: evaluación semántica de A

**RETP-2026-133 · 11/09/2026. Estado: preparado; implementación pendiente.** La sonda IE004-R10-MEMO/1 ha terminado su contraste público. Su resultado permite preparar el evaluador de significados sobre el bosque completo. No habilita todavía la captura reservada, la validación V ni una aprobación del sistema.

## Entrada y frontera normativa

Entrada de diseño: léxico y G01–G25 de IE004-ES-P2/2, semántica de su §5, contexto cerrado de IE-004, fuentes y resultados públicos de RETP-130–133. Mantener la pregunta original, los intervalos comprobados y las identidades de contexto, base y política. Ninguna propuesta del modelo interviene en esta entrada de A.

Antes de congelar una realización, redactar una sucesora explícita del calendario de /2 §6.3 y su contabilidad. El DAG y las repeticiones positivas permiten el recorrido por dependencias ensayado, pero éste no cumple la obligación de barridos de /2. La sucesora debe enumerar qué cambia en orden, trazas y cargos y qué se conserva en gramática y significado. No basta renombrar el contador. Toda operación relevante, incluida inicialización, deduplicación y materialización semántica, debe estar cubierta por la imputación declarada o por un presupuesto de preparación expresamente acotado.

El compromiso P3 permanece asociado a /2. La futura declaración de compatibilidad deberá identificar la sucesora y conservar preguntas/contextos, significado y esperados, y precisar el cambio de recursos/calendario. Se puede preparar sin ver la reserva; no se declara ya cumplida. Si cambia contenido semántico o alcance, corresponde resolverlo con el custodio antes de cualquier uso de aquella reserva.

## Producto que se debe entregar

Un evaluador Rust con representación tipada de restricciones positivas por campo, exclusiones parciales, demandas contextuales, operando textual y marcas. Cada familia sintáctica aporta sus interpretaciones; la disyunción reúne alternativas y la composición conjuntiva forma productos. Conservar conflictos, incompletitud y prohibiciones hasta reunir y deduplicar significados por igualdad estructural. Aplicar política sólo después de comprobar unicidad.

La representación canónica y su orden deben constar en un formato versionado, junto con código, fuente exacta del perfil, contabilidad, límites y pruebas públicas fijadas antes de ejecutarse. Los diagnósticos técnicos y semánticos del puesto quedan fuera de `Tri`; el estado 0/1/U sólo se devuelve como literal del campo ESTADO autorizado.

## Obligaciones de aceptación pública

| Obligación | Distinción que la prueba debe ejercer |
| --- | --- |
| S01 · disyunción antes de permisos | Los tres ejemplos públicos de /2 §5.3: IgG o IgM da PETICION_AMBIGUA; IgG o IgG converge; IgG de IgA da PETICION_CONTRADICTORIA, en el contexto allí fijado |
| S02 · exclusión parcial | Conservar la lectura positiva y el alcance exacto del objeto negado; no convertir toda negación en prohibición general de LEER |
| S03 · demandas y contexto | Elipsis resoluble con contexto constituido frente a demanda pendiente; ningún dato se completa desde la nota externa |
| S04 · campo y literal | VALOR, UNIDAD, FUENTE, ALCANCE, MOMENTO y ESTADO no se confunden; operando de escritura conserva texto |
| S05 · todas las causas | Mantener interpretaciones incompletas, contradictorias o prohibidas aunque otra permita dato; precedencia estable después de unicidad |
| S06 · cierre y recursos | Un testigo parcial no permite salida al agotarse la cuenta; overflow, conversión o reserva fallida producen resultado técnico sin dato |
| S07 · identidad y evidencia | Mismos significados con orden distinto se deduplican; causas distintas no; evidencia original separada de su clave de igualdad |
| S08 · memoria y destinos | Cuota total de estados y arena agregada de A, comprobación de tamaños y comportamiento nativo/WASI con debug/release |

La matriz ejecutable deberá vincular cada obligación a casos públicos concretos y esperados previos. El corpus histórico se usa respetando su versión de oráculo; si un contraste refleja una revisión del perfil, se explicita, sin corregir retroactivamente el resultado antiguo. Las 72 pruebas sintácticas existentes son regresión sintáctica, no 72 oráculos semánticos ya establecidos.

## Presupuesto de trabajo y salida

Una implementación candidata y una ejecución de su matriz pública fijada por cada una de las cuatro configuraciones, con los plazos de la guía vigente. Si aparece un defecto: conservar el fallo, corregir la causa una vez y verificar el contraejemplo y las obligaciones afectadas. Si el mismo impedimento persiste, cerrar la realización como insuficiente y elevar una decisión de diseño con su evidencia. Un defecto nuevo no autoriza una sucesión indefinida de rondas. No pedir otra tanda a Grok para suplir un corrector incompleto.

No elevar límites después de observar los resultados. No presentar una cuenta sintáctica de 62.906 unidades como coste total futuro de A: aún faltan fases. Aplicar [el criterio de memoria](MEMORIA_VEC_ARRAYS.md), medir capacidades y crecimientos por buffer; las reservas de sintaxis y semántica no duplican el cupo de 32 MiB. El techo de 16.384 estados cuenta significados parciales agregados, no nodos sintácticos ni estados por celda.

Salida suficiente de este siguiente objeto: todas las obligaciones públicas satisfechas con semántica, traza y cuerpo comparables en los cuatro destinos/configuraciones; versión de calendario y contabilidad explícita; ninguna causa técnica convertida en U; limitaciones registradas. Sólo entonces procede congelar corrector y esquema de captura, verificar compatibilidad/custodia y preparar la captura P3. P4 deberá acreditar aislamiento material A/V y P5 beneficio y coste del agente. Este documento no acredita ninguno de esos resultados.

El resto de frentes conserva su secuencia: aplicación del dominio mediante IA subordinada; catálogo/localización en parada técnica; retorno posterior a fila 9/Ciberseguridad. No se modifica la constitución del dominio ni se abre aprendizaje durante su aplicación.
