# Contrato candidato y alcance — COMPILER-DIAGNOSTICS/1

## Una decisión, dos formas de entrega

`compiler_diagnostics::compile` y `compile_assembly` son entradas detalladas de la candidata. `compile_svp`, `compile_svp_profile` y `compile_svp_assembly` llaman al mismo recorrido y recuperan el error heredado. El parser sigue privado; ningún diagnóstico constituye autoridad ni autoriza ejecución. La IR conserva versión 0.3, estructura y serialización.

`Report` reúne una causa tipada, la etapa, el inventario de unidades originales y los lugares relacionados. Sus campos se construyen dentro del núcleo y se consultan por referencias inmutables. El inventario contiene índice de unidad, nombre, perfil, tamaño en bytes y SHA-256. Los lugares contienen índice, rol y rango opcional. Se distingue el inventario de entradas de las declaraciones efectivamente implicadas; un archivo del inventario no queda acusado por aparecer en él.

## Causas migradas

E004 distingue codominio vacío de miembros repetidos. E115 conserva celda, semántica y codominio cuando existen, y listas separadas de claves repetidas, ausentes y ajenas. La semántica autónoma con duplicados no recibe un codominio inventado. La colisión conserva ambas declaraciones; se mantiene la precedencia anterior, que registra objetos antes que operaciones. El orden de detección no se presenta como orden textual cuando difieren.

Los seis tipos de FrontendError se conservan mediante correspondencia entre variantes, nunca mediante lectura de Debug. Los validadores pendientes reciben CD.UNMIGRATED y su etapa; no se extrae un código SV del texto heredado. Los códigos canónicos acreditados aquí siguen siendo E004 y E115. Las doce claves CD.* pertenecen al contrato local, no amplían el catálogo canónico.

## Procedencia y presentación

Los rangos se obtienen del texto original durante el análisis. Son intervalos [inicio, fin), medidos en bytes desde cero; EOF es [len, len). Una declaración completa puede ser el rango relacionado sin que se publique su texto. No se reconstruye fuente desde IR. Dos unidades con igual nombre se distinguen por índice, perfil y huella.

Los mensajes son plantillas estáticas ES/EN. Una entrada monolingüe recibe su idioma; un ensamblaje mixto dispone de ambos. Con cero unidades no se inventa un perfil: messages() está vacío y el consumidor puede solicitar explícitamente una plantilla. La localización no modifica el juicio.

La presentación ordinaria y Debug de Report no incluyen las fuentes ni literales como SECRETO. Los parámetros tipados y nombres originales siguen disponibles para un receptor autorizado: cualquier interfaz que los represente como HTML/JSON deberá aplicar el escape del formato correspondiente. Esta candidata no añade un serializador público de diagnósticos. legacy() e into_legacy() son accesos explícitos compatibles y pueden contener texto original; no deben confundirse con una salida ya saneada para publicación.

## Qué queda abierto

Migración del resto de emisores y causas de perfiles; rangos sintácticos exactos distintos de EOF; serialización/presentación final de parámetros; recepción de bytes inválidos en adaptadores; integración y medición WASI/navegador. El contrato global DG01–DG14 y DFL-001/011 no se cierran por este subconjunto. Se mantiene DFL-005/006 para admisión profesional y verificadores. La reserva P3 permanece cerrada y P4/P5 conservan sus compuertas.
