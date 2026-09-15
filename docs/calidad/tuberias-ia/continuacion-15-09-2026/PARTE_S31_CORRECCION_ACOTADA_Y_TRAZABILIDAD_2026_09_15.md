# Parte de trabajo S31: corrección acotada y trazabilidad documental

**Fecha de apertura:** 15 de septiembre de 2026.  
**Estado inicial:** pendiente.  
**Registro canónico:** [S31](../../Inventario-sv/sucesos/SUCESOS_SV.md#s31--corrección-acotada-de-recuentos-identidad-y-condiciones-de-reproducibilidad).  
**Corte de entrada del Lenguaje:** `465e3dfb18b8b0775a3ee2ffffda423acfef851b`.

## Objeto y antecedentes

Delimitar y seguir la precisión de los recuentos del mapa, la comprobación previa de identidad de los archivos utilizados, la recuperación acotada de dos archivos pendientes y la diferenciación documental de los lotes público y reservado P3. El alta precede a la ejecución de este parte. Los cotejos preliminares ya realizados constituyen antecedentes; no se atribuyen a una ejecución posterior ni se presentan como nuevos.

Rigen el registro Sucesos SV y la continuidad establecida por el Acta 001 de esta sede. El Acta 002 mantiene el contrato de leyenda R06 como candidato. Este parte no modifica la secuencia general ni acredita capacidades adicionales.

## Unidades de trabajo

| Unidad | Trabajo delimitado | Evidencia de terminación | Estado al alta |
| --- | --- | --- | --- |
| C01 | Precisar 761 referencias pendientes globales, 691 atribuidas a 32 nodos y 70 sin atribución; distinguir 101 enlaces al laboratorio entre 329 enlaces totales de cualquier medida de volumen de evidencia. | Nota de precisión con fuente y alcance de cada recuento; conservación del HTML y del paquete histórico. | pendiente |
| C02 | Cotejar en Rust 1.98.0 la identidad de cada archivo utilizado en esta intervención antes de su análisis: repositorio, corte, ruta, tamaño y blob esperado. | Relación de entradas y resultados; una copia discordante no se utiliza. Una huella SHA-256 calculada durante la intervención no se presenta como compromiso previo. | pendiente |
| C03 | Intentar recuperar exclusivamente los dos archivos pendientes identificados abajo por el acceso autorizado disponible. | Resultado por archivo: recuperado y cotejado, o recuperación no lograda con causa concreta y condición de continuación. No ejecutar sus contenidos ni repetir campañas. | pendiente |
| C04 | Contrastar exclusivamente la documentación pública sobre la coincidencia de identificadores del lote sintético y P3, y el sentido de los valores vigente:false. | Conclusión acotada: correspondencia con controles negativos o defecto documentado; condición previa al uso de P3. No consultar ni alterar material reservado. | pendiente |
| C05 | Consolidar resultados en este parte mediante una sección añadida y actualizar CSV, Markdown, historial y entrada de continuidad. | Referencias a cortes y evidencias, actuaciones concluidas, limitaciones restantes y relevo al contrato de leyenda R06. | pendiente |

## Archivos pendientes de recuperación

Repositorio de procedencia: `SV-matematica-semantica-cuaternaria`. Corte histórico: `86441ad4d375e31737dfcead0b1fd9cd52161883`. La presencia de los blobs en el corte se cotejará antes de utilizar los contenidos.

1. `laboratorio/tareas-watson/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/s6-trazabilidad-total/custodia/CONTROLES_CUALIFICACION.zip`; blob esperado `ffe5c4a9b86b26c2409abe4239e6355f7a3bef19`.
2. `laboratorio/tareas-watson/tuberias-ia/paridad-imagen-celula-matematica/soporte/v2/frame-original.png`; blob esperado `f123822692430d79b0b5b44d2aa4ffca4c238c94`.

La recuperación no equivale a cualificación del instrumento ni autoriza difundir contenidos privados. Si las vías disponibles no permiten obtener un archivo, se conserva el pendiente sin abrir una investigación de infraestructura.

## Exclusiones

No se modifica la presentación del HTML ni se regenera su paquete histórico. No se crean ramas ni se reorganizan directorios. No se renumeran actas, sucesos o registros técnicos. No se implementa el reconocedor de leyenda, se repiten campañas, se abren reservas, se renombran lotes, se modifican oráculos ni se implementan adaptadores P3. No se alteran los estados de S22 o S26 ni sus condiciones de cierre.

## Condiciones de suspensión

Una discrepancia real de identidad, una mezcla efectiva de material público y reservado o una evidencia que contradiga una aceptación vigente suspende la unidad afectada y exige documentar el hallazgo antes de ampliar la intervención. Una limitación de acceso ya conocida no amplía el alcance.

## Seguimiento y cierre

La apertura se registra como S31, revisión 0, en estado pendiente, con fechas de inicio y fin vacías. Antes de comenzar C01–C04 se añadirá la revisión de inicio, en ejecución, con su fecha efectiva. Toda actualización conservará las instantáneas anteriores del historial.

Al terminar se añadirá a este mismo parte una sección de cierre con una fila por C01–C05, evidencia, resultado y pendientes derivados. Se actualizará S31 a finalizado con fecha efectiva únicamente cuando haya concluido el alcance delimitado. Si un archivo no se recupera, la finalización del intento se distinguirá de la recuperación pendiente; no se atribuirá conformidad material.

Una interrupción antes de completar el alcance se registrará como pendiente, con motivo y siguiente acción. El cierre identificará las copias efectivamente actualizadas y las que conserven un corte anterior. Tras concluir esta intervención se retomará la subsanación del contrato candidato de leyenda R06. La numeración histórica se conserva íntegra.

