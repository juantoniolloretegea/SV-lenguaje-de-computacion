# Resultado del adaptador de lote y compatibilidad preparada

**RETP-2026-140 · 11/09/2026 · CONFORME_EN_ALCANCE_PUBLICO_TRAS_CORRECCION_CONDUCTOR.** Se cierra el objeto autorizado tras RETP-137. La reserva no se ha abierto.

## Qué se esperaba y qué se observó

El adaptador debía verificar identidad de bytes antes de decodificar, admitir exactamente los 24 casos ordenados del formato público P3, preservar pregunta/contexto y permitir el recorrido A sin recibir notas o V. Debía detener sobrelongitudes, esquemas incorrectos, duplicados incluso escapados, orden incorrecto y Unicode malformado. Los originales siguen recuperables: la solicitud reserializada no se presenta como el mismo archivo original.

| Configuración Rust 1.98.0 | Adaptador | Recorrido inicial A | Corrección acotada | Estado conjunto |
| --- | --- | --- | --- | --- |
| Nativo debug | 36/36 × 3 | 63/72 conformes | 9/9 | 24 posiciones × 3 conformes |
| Nativo release | 36/36 × 3 | 63/72 conformes | 9/9 | 24 posiciones × 3 conformes |
| WASI debug | 36/36 × 3 | 63/72 conformes | 9/9 | 24 posiciones × 3 conformes |
| WASI release | 36/36 × 3 | 63/72 conformes | 9/9 | 24 posiciones × 3 conformes |

Son 432 observaciones del adaptador; 288 observaciones iniciales de recorrido A y 36 repeticiones causales de las fallidas. **No son 468 controles del adaptador, ni 324 casos independientes.** Las 24 posiciones reutilizan diez entradas públicas: no es novedad lingüística ni una captura de modelo. Todos los cuerpos definitivos coinciden con los esperados públicos anteriores, conservados sin cambios.

## Fallo conservado y reparación causal

Omití en el conductor el argumento de revocación heredado de A04. P3-04/14/24 se ejecutaron con vigencia activa, y la comparación de cuerpos detectó esa diferencia en todas las reproducciones/configuraciones. La primera matriz permanece **FALLO**. La lectura directa de sus capturas verificó que `vigente` era el único campo diferente y que pregunta, contexto, significados y ambigüedad anterior a permisos se conservaron.

RETP-139 fijó la corrección antes de ejecutarla: restituir `revocado` desde las fichas públicas previas. Las 36 observaciones corregidas pasaron; no cambió ningún Rust, binario, pregunta, contexto o esperado. No se repitieron los controles del adaptador ni los recorridos ya conformes. La incidencia EISDIR del manifiesto —un directorio temporal del compilador tratado como archivo— se resolvió mediante inventario de archivos regulares, sin reejecutar la matriz. Véase [causa y sucesión](CORRECCION_CAUSAL_CONDUCTOR.md).

## Recursos observados de preparación del lote

| Control | Trabajo del lote | Reservas acumuladas nativo | Reservas acumuladas WASI |
| --- | ---: | ---: | ---: |
| L01: 24 posiciones públicas | 191565 | 5929926 B | 5927622 B |
| L04: archivo válido de 2 MiB | 8547017 | 5929926 B | 5927622 B |
| L25: primera pregunta de 8192 bytes | 336009 | 5970768 B | 5968464 B |
| L32: 24 preguntas de 8192 bytes | 3870862 | 6901676 B | 6899372 B |

Debug/release coinciden en trabajo y reservas de su destino. La cota de lote fijada fue 24 millones de unidades y 16 MiB de reservas acumuladas. El byte 2097153 se observó para detectar exceso sin acumularlo; una pregunta de 8193 bytes y otra de 129 tokens fueron rechazadas. Los cuerpos y el trabajo del adaptador coinciden entre destinos; la representación de estructuras produce la diferencia de reservas indicada. Estas cifras no son RSS, pico físico ni instrucciones CPU. Tampoco se suman o convierten a los intentos /2 o a las unidades del motor A. La preparación completa permanece visible como coste separado.

## Custodia comprobada

Se han cotejado 765 metadatos de procesos con sus stdout/stderr y las identidades de ocho binarios. [Resultado consolidado](RESULTADO_CONSOLIDADO.json) y [manifiesto](MANIFIESTO_CAPTURAS.json). El [archivo recuperable](ARCHIVO_CAPTURAS.json) conserva **2879 archivos**, incluida la matriz desfavorable íntegra, sus solicitudes, marcos y trazas, y las capturas de corrección. Se comprobó la extracción a una carpeta nueva sin ejecutar receptores. Los ocho ejecutables finales están identificados y son reconstruibles; los temporales de compilación quedan excluidos e identificados.

`recuperar-capturas.mjs <ruta-nueva>` verifica el paquete y cada archivo antes de escribirlo. La reproducción completa corregida para terceros es `reproducir-completa-corregida.mjs`, en una copia nueva. El programa original de RETP-138 permanece intacto para explicar y reproducir su fallo. No hace falta repetir la campaña para consultar o recuperar sus resultados.

## Compatibilidad y siguiente decisión

El formato público `IE004-P3-A/1` está cualificado en este alcance. **No se afirma que el lote reservado haya pasado:** sólo conocemos su compromiso y metadatos públicos. La [matriz de compatibilidad](MATRIZ_COMPATIBILIDAD_P3.md) y la [decisión preparada](DECISION_PREPARADA_CUSTODIO.json) están disponibles para Juan Antonio; no se han rellenado en su nombre.

El compromiso identifica /2 y la candidata ejecuta /3. Se mantiene la correspondencia semántica declarada, pero el calendario de barridos y las unidades contables cambiaron expresamente. Una ejecución /3 no acredita el §6.3 literal de /2 ni todas las expectativas paramétricas reservadas. Antes de abrir la captura debe quedar fijado el alcance de la asociación con el compromiso existente; cualquier cambio de alcance u oráculo necesita sucesión explícita anterior a su uso, conservando los originales. No se pide renovar la custodia ya confirmada.

El siguiente objeto es esa decisión de compatibilidad y la preparación del encargo/formato de captura bajo el alcance que se determine. La admisión y entrega de auxiliares al participante sigue fuera de A y no se ha implementado en este adaptador. No se inicia otra ronda de Grok, no se accede al oráculo y no se decide por Claude sobre su autoría. P4 (imposición material) y P5 (viabilidad/aportación) siguen pendientes. Catálogo/localización, fila 9 e integración profesional conservan su secuencia.
