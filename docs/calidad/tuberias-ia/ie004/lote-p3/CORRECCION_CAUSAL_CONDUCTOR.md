# Corrección causal del conductor · RETP-2026-139

**11/09/2026. Preparación de una corrección, anterior a su ejecución.** La matriz fijada en RETP-138 se completó en sus cuatro configuraciones. Los 36 controles del adaptador, repetidos tres veces, fueron conformes en todas: 432 observaciones. El recorrido de A produjo 63/72 conformes por configuración, con nueve fallos. La versión del conductor queda **FALLO**, sin borrar ni reetiquetar sus resultados.

## Causa y testigo

P3-04, P3-14 y P3-24 reutilizan A04 de RETP-137. Su ficha Rust fijaba `vigente:false`, y sus cuerpos esperados conservan ese valor. El nuevo conductor invocó siempre `a`, que selecciona vigencia activa, omitiendo `revocado`. Son tres posiciones, tres reproducciones y cuatro configuraciones: 36 comparaciones fallidas. La comparación detectó exactamente esa diferencia de montaje. Pregunta, contexto, estado ambiguo, ruta, causas y cero llamadas a política coinciden; no hay evidencia de modificación semántica por el adaptador.

La [comprobación documental de capturas](COMPROBACION_RONDA_1.json), sin nuevas ejecuciones del receptor, identifica como única diferencia `vigente` en los 36 cuerpos. También verifica los 288 originales conservados en las trazas. En el primer resumen `original_conservado:false` significaba que la aserción de cuerpo se había interrumpido antes de comprobarlo: no se toma como prueba de pérdida ni se modifica retroactivamente ese archivo.

La referencia correcta ya existía antes de esta campaña en `recepcion-av/casos_fijados.rs`, su `CONTROLES_PUBLICOS.json` y los cuerpos históricos. `MONTAJES_PUBLICOS.json` extrae esa configuración explícita; no se decide vigencia desde el lote, el modelo o el resultado observado. Un primer script de cotejo se detuvo porque el JSON no repetía `vigente` en todas las fichas; se acudió a la declaración explícita del Rust fijado. Ese cotejo no ejecutó consultas ni cambió datos.

## Intervención única y alcance del contraste

Se restituye el argumento `revocado` para esas posiciones del conductor. **Ningún byte Rust, pregunta, contexto, esperado ni binario cambia.** Se reutilizan los ocho binarios cotejados y las solicitudes extraídas ya capturadas. `CONTROLES_CORRECCION.json` fija exclusivamente las 36 observaciones fallidas y sus huellas originales. Se ejecutarán con el montaje correcto; un fallo persistente impedirá la conformidad. No se repetirán los 432 controles del adaptador ni los 252 recorridos ya conformes.

La campaña corregida se guarda en `resultados-correccion-1`. La primera queda en `resultados-1`. El estado consolidado distinguirá evidencia inicial y corrección; no presentará 468 controles del adaptador ni 324 solicitudes independientes. `reproducir-completa-corregida.mjs` permite a un tercero una reproducción completa nueva, con montaje explícito, sin ejecutarla ahora.

## Incidencia de archivado

Después de guardar `RESULTADO.json`, el inventariador intentó leer como archivo `rustct4dkEt`, un directorio temporal del compilador. Node devolvió `EISDIR`. Ninguna observación de la matriz se perdió; falló la creación del manifiesto final. Se corrige filtrando archivos regulares. Los directorios temporales de compilación se identifican y se excluyen de las capturas; los binarios completos se conservan identificados y reconstruibles. La recuperación de los resultados no requiere repetir ninguna ejecución.

La reserva y su oráculo siguen cerrados. El problema de montaje no autoriza modificar el perfil ni el compromiso. La matriz de compatibilidad integral /2–/3 conserva su decisión pendiente.
