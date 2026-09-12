# Relevo al catálogo desde la recepción S19

## Decisión de alcance

Se puede avanzar al inventario de puntos de emisión y al contrato diagnóstico acotado del recorrido S18. Se dispone de evidencia conjunta de contexto, base consumida, recepción, cobertura y entrega. No se declara cerrada la integración profesional completa de los puntos 1 y 3 ni la suficiencia general de Semántica 0.2 e IR 0.3.

S19 no introduce códigos E…, tipos de IR, primitivas, versiones de gramática ni comportamiento nuevo. Los identificadores S19-OBS son referencias de inventario. Las causas locales S17/S18 permanecen locales hasta que exista una correspondencia constituida; el parecido de nombres no la establece.

## Contrato que gobierna el siguiente objeto

Se recibe el contrato existente `CONTRATO_DIAGNOSTICO_ESTRUCTURADO_Y_LOCALIZACION_ES_EN_2026_09_09.md`, junto con el catálogo efectivo `ERRORES_CANONICOS_SV_v0_3.md`. Sus copias exactas, corte, blob, SHA-256 y URL están en `FUENTES_RECIBIDAS.json`. La concordancia de IR v0.2/Parche 1A se conserva como antecedente histórico y no sustituye el catálogo efectivo vigente.

El catálogo declara 51 códigos; esa cifra no demuestra 51 puntos de emisión ni cobertura ejecutable completa. El documento mantiene divergencias históricas y exige sincronizar código, punto de emisión, especificación y caso representable. No se corrigen retrospectivamente E101/E004 o E204, ni se asigna un E… a una causa del receptor por semejanza.

La recepción de S18 evidencia una causa tipada (`Causa`) y etapa dentro de Rust, así como identificadores y rótulos ES/EN fijados en S17. Sin embargo, `Diagnostico::registro` serializa el detalle mediante `{:?}`; los rótulos no constituyen por sí solos una presentación completa, versionada y localizada de parámetros y procedencia. S19 ordena esos registros como evidencia histórica; no analiza `Debug` para fabricar una causa técnica o decidir por el SV.

La siguiente unidad de trabajo deberá:

1. Inventariar los puntos de emisión del recorrido afectado, incluidos errores propagados desde G1 y el host, con función, variante, fase, entrada que los hace alcanzables y evidencia existente o ausencia explícita.
2. Separar origen SV, realización de tubería, compilador y E/S del host. `E0451` conserva origen rustc. Distinguir fallo técnico, negativa, recibo de política y éxito documental; ninguno constituye `Tri.U` ni permiso profesional.
3. Fijar un contrato versionado de diagnóstico con causa y parámetros tipados emitidos en el punto de comprobación, procedencia y localización cuando estén disponibles. Declarar la ausencia cuando no existan; no inventar coordenadas de fuente, cobertura ni una equivalencia canónica.
4. Fijar plantillas ES/EN deterministas, independientes del juicio. La presentación no cambia resultado ni permisos; el texto externo es dato. No traducir mediante un LLM en la ruta de decisión ni reconstruir causas a partir de la prosa.
5. Precisar compatibilidad y migración con los registros S17/S18 existentes. Conservar originales; cualquier nuevo formato tendrá identidad propia y comparación explícita. No promover el prototipo al núcleo para resolver una conveniencia del transporte.
6. Antes de cambios funcionales, fijar casos positivos/negativos y oráculos independientes para el alcance elegido, incluidos fallos de presentación y precedencias. Reutilizar evidencia válida, sin confundir enumeración en código con ejercicio de una rama.

Este es el siguiente objeto acotado de catálogo. No autoriza una expansión general del frontend o del host, ni altera ahora el núcleo. La necesidad de una extensión deberá demostrar primero qué obligación no puede satisfacer el mecanismo existente.

## Ramas que no quedan cualificadas por las 24 obligaciones S18

Lectura estática de `codigo/recorrido.rs`, recibido byte a byte. Esta lista orienta el inventario siguiente y **no se presenta como inventario exhaustivo de emisores**:

| Grupo | Ausencia de ejercicio específico en S18 |
|---|---|
| Carga de base | FormatoBase, LimiteBase y variantes de E/S de apertura/lectura distintas del caso BaseDistinta observado |
| Producción / entrega G1 | Todas las variantes propagables de G1 y EntregaG1; éxito de G1 no cualifica cada error interno |
| Reevaluación / referencia | Capacidad, BaseSinCambio y rechazo de solicitud distinta al reevaluar; no equivalen a los negativos de cobertura I06–I09 |
| Esquema | Cabecera, Tipo, Sobrante y Bandera no se ejercitan como casos independientes de S18; Truncado sí |
| Contexto propuesto | LimiteContexto en fase contexto, diferente del límite de instalación I20 |
| Presentación | Otras variantes del comprobador distintas de ContenidoDistinto |
| Archivo | Error de flush; E/S y límite de recuperación; demás variantes de apertura/escritura |
| Recursos / host | Agotamiento de memoria, caída del proceso, durabilidad, host comprometido y transporte real |

No todas estas ausencias exigen añadir una rama ni un caso a la siguiente campaña: primero se determinará cuáles son alcanzables y pertenecen a su alcance. Los antecedentes S15 no se borran ni se cuentan como nuevos ensayos integrados S18.

## Fronteras conservadas

B/E/K/L y puertas P3/P4/P5/P6 permanecen abiertas en sus términos. S16 conserva la separación entre fallo de recepción, permiso no otorgado y resultado indeterminado después de `DispatchCommitted`. I22 deja un prefijo de siete bytes: no se transforma ese fallo en ausencia de efecto. I19 detecta una sustitución posterior: no la impide.

La secuencia principal sigue siendo integración 1+3 → catálogo → retorno por la fila 9 a las obligaciones de inmunología y del Lenguaje. El primer universo IMM y el primero CYB tuvieron cierres acotados; no se decide por ello la suficiencia de los universos pendientes. El orden o necesidad de agentes se resolverá después de cerrar inmunología, conforme a la rectificación S12, sin adelantarlo por comodidad de la implementación.
