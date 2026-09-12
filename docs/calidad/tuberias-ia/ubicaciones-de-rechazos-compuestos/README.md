# Ubicaciones de rechazos compuestos

**RETP-160 · 12 de septiembre de 2026 · Candidata experimental.**

Doce puntos del analizador conservan ahora la posición original del elemento que causa el rechazo, aunque se hayan leído otros elementos antes de emitirlo. Esto permite señalar el campo repetido o fuera de orden, la variante no admitida o la etiqueta de admisibilidad incorrecta. La causa de grafía ajena al perfil se conserva desde la clasificación léxica existente.

## Referencia y resultado

Este trabajo continúa el [Léame primero del frame](../frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md), el expediente de significado humano y fidelidad, su adenda visual y el documento de trazabilidad, auditoría y reproducción. Conservar la procedencia de un error ayuda a explicar qué ocurrió. La fidelidad exige además comprobar el conocimiento de referencia y lo efectivamente entregado al humano; esta campaña no la da por demostrada.

| Comprobación | Resultado |
|---|---|
| Fuentes nuevas ES/EN | 54: 2 válidas y 52 inválidas |
| Fuentes heredadas RETP-159 | 40, con sus esperados intactos |
| Total focal | 94 fuentes: 10 válidas y 84 inválidas |
| Ensamblajes derivados | 168; ambas posiciones con una unidad sana del otro perfil y nombre de archivo compartido |
| Repeticiones | Tres por modo: 564 observaciones monofuente y 1.008 comprobaciones de ensamblaje |
| Corpus canónico | 14 válidas y 106 inválidas; resultados heredados idénticos entre base y candidata en debug y release |
| Pruebas unitarias | 239/239 en cada modo |
| Focales RETP-158 | 29 en cada modo, mediante la sucesora explícita ya publicada en RETP-159 |
| Catálogo | Las mismas 13 causas y 26 plantillas ES/EN |
| Sensibilidad | Detectadas la ausencia de intervalo en la base y la posición desplazada deliberadamente |
| Fronteras de acceso | Cuatro clientes negativos conservan los seis errores previstos |

Los casos de precedencia comprueban que un error de separador o fin de fuente, emitido antes, mantiene prioridad sobre una variante que aún no se ha validado. Las fuentes incluyen CRLF, tabulación y texto acentuado. Los intervalos se expresan en bytes UTF-8 y se comprueban contra la fuente y su huella SHA-256. Los ensamblajes preservan el índice y perfil de la unidad responsable incluso al invertir el orden. Las repeticiones y los ensamblajes derivados no se cuentan como nuevas fuentes.

[Plan previo](PLAN_PREVIO.md), [compromiso anterior al cambio](COMPROMISO_PREVIO.json), [fuentes nuevas y esperados](CASOS_NUEVOS.json), [fuentes heredadas](CASOS_HEREDADOS_159.json), [resultado](evidencia/RESULTADO.json), [observaciones](evidencia/OBSERVACIONES.json), [procesos y salidas](evidencia/PROCESOS.json), [sensibilidad y catálogo](complemento/RESULTADO.json), [medidas](MEDIDAS.json).

## Cambio y compatibilidad

Se modifica únicamente [frontend.rs](cambios/rust/sv_core/src/frontend.rs) de la cápsula RETP-159. Los otros 76 archivos coinciden byte por byte. [Cambio incremental](CAMBIO_INCREMENTAL.patch) e [inventario antes/después](INVENTARIO_EMISORES.json).

Incluye ocho ramas de los campos opcionales de SemanticRelation y Pattern: repetición, orden, restricciones repetidas y campo desconocido. Incluye también la variante de supervisión, el contexto de consulta, la palabra protegida en el despacho de let y la etiqueta individual de admisibilidad. El emisor guarda el índice al leer el elemento; la validación permanece en su lugar original. No se interpreta la prosa del error ni un centinela para clasificarlo.

Es una extensión observable de cobertura de `COMPILER-DIAGNOSTICS/2`, sin cambios de variantes públicas ni mensajes. Los emisores incluidos pasan de intervalo ausente a intervalo original; los rechazos de grafía constitutiva extranjera pasan a `CD.FOREIGN_SURFACE`. Los resultados de la API heredada, el orden de rechazo, los códigos SV y la IR se conservan en las comprobaciones ejecutadas. Los antiguos controles de RETP-159 que exigían ausencia de intervalo en campos opcionales quedan sucedidos por esta cobertura; su expediente histórico permanece intacto. Los 40 esperados focales de RETP-159 no se modifican.

## Reproducción

Descargue esta carpeta completa o [PAQUETE_REPRODUCIBLE.zip](PAQUETE_REPRODUCIBLE.zip). Con Linux, Python 3.10 o posterior y rustc con biblioteca estándar nativa:

```sh
python reproducir.py /ruta/absoluta/rustc /ruta/a/directorio/nuevo
```

Se ejecutó la reproducción completa desde la carpeta de entrega en un directorio nuevo: los 77 archivos por versión y ambos documentos de resultado coinciden exactamente con la cualificación inicial. Se conservan sus [registros separados](recepcion_portatil/), sin sumarlos como nuevas fuentes.

La reproducción valida el manifiesto SHA-256, recupera los 77 archivos por versión y ejecuta la cualificación y sus controles sin red. Se utilizó rustc 1.98.0 para x86_64-unknown-linux-gnu. El compilador y las decisiones SV se ejecutan en Rust; Python recupera y observa. Los instrumentos preservan 120 segundos por proceso y 2 MiB por flujo.

La cualificación inicial comprende 41 procesos principales y 7 complementarios, con comandos, binarios identificados, códigos y bytes de salida conservados. Los dos fallos de sensibilidad son los rechazos previstos del comprobador y están separados de los resultados de la candidata. No hubo fallos inesperados. Tiempo, CPU y máximo acumulado de memoria de hijos incluyen compilación: no acreditan consumo por caso, mejora de rendimiento, coste incremental, dinero ni inferencia.

## Límites y relevo

Cortes de entrada: Lenguaje `76722442d2899300c69d89ab932890a4d7d74c2f`; laboratorio `afcbd5046b2eec24cb058ce0fedcf580ca4790e8`, rama `lab/playground-sv-permanente`. Se verificó el expediente RETP-159 completo en ambos árboles. Se cotejaron las rectoras leídas: AGENTS, Pilares completos, perfiles y ensamblaje, transición completa y relevos; se consultaron en esta continuidad el contrato diagnóstico RETP-109/110, workflow V2 y plan RETP-147.

La cardinalidad incorrecta de los estados de admisibilidad mantiene rango ausente, comprobado en ambos perfiles: el error afecta al conjunto y no se atribuye artificialmente a una etiqueta. También permanecen sin campaña las conversiones defensivas de texto UTF-8 y del cuadrado natural señaladas en el inventario. Los demás validadores, sus subcausas, la serialización y la presentación final siguen pendientes; este incremento no demuestra cobertura global de todos los caminos.

El siguiente objeto acotado es constituir y comprobar la ubicación del conjunto de estados cuando falla su cardinalidad, incluida la precedencia y la distinción entre etiqueta y conjunto. Después siguen los emisores y validadores pendientes del mismo paso 6. El avance no cierra P6 del workflow ni DG01–DG14 global. DFL-001/011 y las restantes deudas siguen abiertas; la fila 9 y las prioridades del relevo vigente se conservan. La reserva P3 permanece cerrada; P4/P5 mantienen sus compuertas.

El código productivo sigue sin promover esta candidata. No se acreditan WASI, navegador, recepción profesional ni ejecuciones de proveedores externos. El encargo externo común continúa pendiente; el paquete ofrece reproducción pública y no constituye una prueba ciega. La visibilidad del laboratorio se conserva.

**Estado: CONFORME_EN_ALCANCE_NATIVO; CIERRE_GLOBAL_PENDIENTE.**
