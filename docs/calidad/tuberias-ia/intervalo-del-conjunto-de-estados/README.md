# Intervalo del conjunto escrito de estados

**RETP-161 · 12 de septiembre de 2026 · Candidata experimental.**

Cuando el analizador rechaza un número de estados distinto de tres, el diagnóstico señala ahora la colección escrita completa: desde la llave de apertura hasta la de cierre. Conserva los espacios y comentarios interiores y excluye el separador y los comentarios posteriores. Las etiquetas individuales incorrectas siguen señalándose por separado.

Se continúa en `lab/playground-sv-permanente`, con el espejo habitual en Calidad de `main` del Lenguaje. Este incremento no abre ramas ni reorganiza o renumera registros. Continúa el [Léame primero del frame](../frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md): conservar la procedencia del rechazo contribuye a su explicación, pero la fidelidad del conocimiento efectivamente presentado requiere su comprobación propia.

## Contrato del intervalo

La condición existente `states.len() != 3` cuenta etiquetas escritas, incluidas repeticiones. No calcula la cardinalidad de un conjunto de valores distintos. Se preserva esa condición y el orden de validación. Un conjunto vacío o una coma final siguen produciendo su rechazo sintáctico previo. La validez y unicidad de tres estados no se sustituyen por esta comprobación de longitud.

El emisor guarda los índices de las dos llaves antes de consumirlas. El registro interno de ubicación admite primero y último token, inclusivos; un error de un solo elemento usa el mismo índice en ambos extremos. Los bytes se obtienen de los intervalos léxicos originales. No se busca una llave en el texto mediante coincidencias posteriores, ni se interpreta la prosa del error. Esto permite conservar comentarios que contienen llaves sin confundirlas con delimitadores sintácticos.

El rechazo de longitud permanece después de comprobar la llave de cierre y el punto y coma, y antes del campo `rule`. Los errores anteriores de etiqueta, separador y fin de fuente mantienen su prioridad. Los rechazos de tokenización conservan su precedencia sobre el análisis sintáctico.

Es una extensión observable de cobertura de `COMPILER-DIAGNOSTICS/2`, sin nuevas causas, variantes públicas ni plantillas. [Cambio incremental](CAMBIO_INCREMENTAL.patch) de un único [archivo Rust](cambios/rust/sv_core/src/frontend.rs); los otros 76 archivos de la cápsula son idénticos. No se promueve la candidata al núcleo productivo.

## Casos y resultados

| Comprobación | Resultado |
|---|---|
| Fuentes nuevas ES/EN | 48: 12 válidas y 36 inválidas |
| Fuentes heredadas RETP-160 | 94; dos intervalos ausentes sucedidos expresamente, otros 92 esperados intactos |
| Total focal | 142 fuentes: 22 válidas y 120 inválidas |
| Ensamblajes derivados | 240; unidad sana del otro perfil en ambas posiciones, con igual nombre de archivo |
| Repeticiones | Tres por modo: 852 observaciones monofuente y 1.440 comprobaciones de ensamblaje |
| Corpus canónico | 14 válidas y 106 inválidas; resultados heredados idénticos entre base y candidata en debug y release |
| Pruebas unitarias | 239/239 por modo |
| Focales y relación anteriores | 29 focales RETP-158 mediante su sucesora ya publicada, más comprobación relacional |
| Catálogo | Las mismas 13 causas y 26 plantillas ES/EN |
| Sensibilidad | Detectados intervalo ausente, llave de cierre excluida y rechazo adelantado al separador |
| Fronteras de acceso | Cuatro clientes negativos mantienen seis errores previstos |

Los nuevos casos incluyen longitudes 1, 2, 4 y 6; las seis permutaciones válidas en cada perfil; comentarios interiores y exteriores con llaves, CRLF, tabulación y acentos; y errores concurrentes para comprobar precedencia. Los intervalos, huellas SHA-256, índices y perfiles se contrastan con las fuentes originales. Los ensamblajes y repeticiones no aumentan el número de fuentes distintas.

[Plan previo](PLAN_PREVIO.md), [compromiso anterior al cambio](COMPROMISO_PREVIO.json), [casos nuevos](CASOS_NUEVOS.json), [documento heredado intacto](CASOS_HEREDADOS_160.json), [sucesión expresa de dos esperados](SUCESION_ESPERADOS.json), [resultado principal](evidencia/RESULTADO.json), [procesos](evidencia/PROCESOS.json), [observaciones](evidencia/OBSERVACIONES.json) y [sensibilidad y catálogo](complemento/RESULTADO.json).

## Incidencia conservada y reproducción

La primera ejecución se detuvo al compilar el comprobador del corpus contra la biblioteca optimizada de la base RETP-160: Rust devolvió `E0786`, metadatos corruptos. La compilación inmediatamente anterior había terminado con código cero. Se preservan el [registro del intento](intento-01/evidencia/PROCESOS.json), su [identidad y alcance](intento-01/INCIDENCIA.json) y el [artefacto exacto comprimido](intento-01/ARTEFACTO_FALLIDO.rlib.gz).

La repetición completa con las mismas fuentes, instrumento y opciones pasó. No se modificaron la candidata ni los esperados para superar la incidencia. La causa raíz del fallo de metadatos no está establecida; el expediente no lo presenta como una propiedad corregida del compilador. Los resultados conformes corresponden a la segunda ejecución completa, y el primer intento permanece separado.

Descargue la carpeta completa o [PAQUETE_REPRODUCIBLE.zip](PAQUETE_REPRODUCIBLE.zip). Con Linux, Python 3.10 o posterior y rustc con biblioteca estándar nativa:

```sh
python reproducir.py /ruta/absoluta/rustc /ruta/a/directorio/nuevo
```

Se ejecutó la reproducción completa desde la carpeta de entrega en un directorio nuevo: los 77 archivos por versión y ambos documentos de resultado coinciden exactamente con la cualificación inicial. Se conservan sus [registros separados](recepcion_portatil/), sin sumarlos como nuevas fuentes.

La reproducción comprueba el manifiesto SHA-256 y recupera los 77 archivos por versión, sin red. La referencia ejecutada es rustc 1.98.0, x86_64-unknown-linux-gnu. Rust ejecuta el compilador y las decisiones SV; Python recupera y observa. El artefacto fallido se conserva como evidencia del incidente; la reproducción vuelve a compilar desde las fuentes y no lo utiliza como biblioteca.

La cualificación conforme comprende 41 procesos principales y 10 complementarios. Se conservan además los procesos del intento interrumpido. Los tres fallos de sensibilidad son los resultados previstos del comprobador y están separados del incidente de metadatos. Los límites son 120 segundos por proceso y 2 MiB por flujo. [Las medidas](MEDIDAS.json) incluyen comandos, binarios identificados, códigos y bytes de salida, tiempo, CPU y máximo acumulado de memoria de hijos. Este máximo incluye compilación y no acredita memoria por caso, incremento atribuible al cambio, rendimiento relativo, dinero ni inferencia.

## Continuidad

Cortes de entrada: Lenguaje `0ef4330b9bb18b09f5a386784d7d3baa559624b2`; laboratorio `605de268874621538489554a3f1be0b83f1f4a1f`, rama `lab/playground-sv-permanente`. Se verificó el expediente RETP-160 en ambos árboles y se cotejaron las rectoras leídas: AGENTS, Pilares, perfiles y ensamblaje, transición completa y relevos. Continúan el contrato diagnóstico RETP-109/110, workflow V2 y plan RETP-147, junto con los documentos de frame, adenda visual y trazabilidad, auditoría y reproducción.

Queda cubierto el rechazo de longitud de esta colección en el alcance probado. [El inventario](INVENTARIO_EMISORES.json) conserva pendientes las conversiones defensivas de UTF-8 y del cuadrado natural, su alcanzabilidad y la conversión genérica de errores sin procedencia. El siguiente objeto acotado es determinar la alcanzabilidad de esas conversiones sobre sus entradas permitidas y el tratamiento que corresponde, antes de asignarles ubicaciones o declararlas cubiertas. Después continúan los validadores y subcausas pendientes del mismo paso 6.

No se cierra P6, DG01–DG14 global, DFL-001/011 ni las demás deudas. Se mantienen fila 9 y prioridades vigentes; reserva P3 cerrada y P4/P5 con sus compuertas. Presentación/serialización final, WASI/navegador, recepción profesional y encargo externo común siguen pendientes. No se ejecutan proveedores externos ni se acredita fidelidad final del conocimiento presentado.

**Estado: CONFORME_EN_ALCANCE_NATIVO; INCIDENCIA_INSTRUMENTAL_CONSERVADA; CIERRE_GLOBAL_PENDIENTE.**
