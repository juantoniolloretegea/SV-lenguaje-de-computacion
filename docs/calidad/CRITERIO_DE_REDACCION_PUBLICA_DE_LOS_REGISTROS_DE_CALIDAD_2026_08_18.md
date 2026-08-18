# Criterio de redacción pública de los registros de calidad del Lenguaje SV

**Fecha:** 18/08/2026  
**Ámbito:** `docs/calidad/`  
**Estado:** vigente

## 1. Objeto

Este criterio fija la forma de redacción de las actas, registros, matrices, notas y decisiones públicas del bloque de calidad del Lenguaje SV.

Su finalidad es asegurar que la documentación pueda ser leída y revisada por terceros como registro técnico público, sin depender de conversaciones internas, del proceso de elaboración ni de una unidad concreta de trabajo.

## 2. Lengua y registro

La redacción ordinaria será en español técnico, formal, preciso y sobrio.

Se evitarán extranjerismos cuando exista una expresión española suficiente y no se perjudique la trazabilidad técnica. En particular, en la prosa pública se preferirán, según el contexto:

- **etapa frontal del compilador** frente a *frontend*;
- **infraestructura o etapa de ejecución** frente a *backend*;
- **entorno de ejecución** frente a *runtime*;
- **analizador sintáctico** frente a *parser*;
- **descenso o traducción a IR** frente a *lowering*;
- **validador** frente a *validator*;
- **serializador** frente a *serializer*;
- **batería de pruebas** frente a *suite*;
- **caso de prueba** frente a *fixture*;
- **comparación de cambios** frente a *diff*;
- **tabla de correspondencias funcionales** frente a *crosswalk*;
- **contraste crítico**, **revisión crítica** o **comprobación** frente a *adversarial*.

Los nombres propios de tecnologías, acrónimos normalizados, nombres de archivos, ramas, funciones, clases, diagnósticos y demás identificadores literales se conservarán cuando su traducción reduzca la precisión o rompa la trazabilidad. Esos elementos se señalarán preferentemente con formato de código.

## 3. Contenido admisible

Todo documento público de calidad deberá separar, cuando proceda:

- hecho constatado;
- fundamento doctrinal, matemático o técnico;
- decisión adoptada;
- evidencia;
- alcance e impacto;
- deuda o límite que permanece abierto;
- estado final del objeto tratado.

La evidencia de una verificación externa podrá identificarse por su función —por ejemplo, **unidad auditora independiente**— y por los resultados reproducibles obtenidos. No es necesario trasladar al documento público la conversación, el proveedor, el modelo empleado ni el desarrollo interno de la deliberación.

## 4. Contenido que debe excluirse

No se incorporarán a las actas públicas:

- conversaciones internas, instrucciones entre unidades o relatos del proceso de coordinación;
- expresiones coloquiales, despectivas, zafias, burdas o impropias de un registro técnico;
- metáforas de trabajo que no aporten precisión técnica;
- juicios de intención sobre personas o unidades;
- justificaciones basadas únicamente en memoria;
- afirmaciones de cierre sin evidencia suficiente.

Cuando una incidencia de proceso sea relevante para la trazabilidad, se formulará como hecho técnico verificable y con el mínimo detalle necesario.

## 5. Preservación histórica

Una corrección lingüística no podrá modificar silenciosamente el sentido técnico de un acta histórica ni alterar el estatuto de un cierre ya registrado.

Los nombres de archivos o identificadores históricos que contengan terminología anterior podrán conservarse para no romper referencias. Su contenido vivo deberá emplear, en adelante, la terminología pública fijada por este criterio.

## 6. Regla de revisión previa

Antes de incorporar o actualizar un documento público en `docs/calidad/` se comprobarán, como mínimo:

1. corrección ortográfica y gramatical;
2. necesidad real de cada extranjerismo;
3. ausencia de lenguaje coloquial o de proceso interno;
4. correspondencia entre hechos, evidencia y estado declarado;
5. conservación de los identificadores técnicos necesarios para la trazabilidad;
6. coherencia con la doctrina superior y con el repositorio fresco.

## 7. Vigencia

Este criterio rige la documentación pública de calidad desde su incorporación. Las piezas históricas sólo se revisarán lingüísticamente cuando sigan actuando como documentos vivos o cuando una incoherencia de redacción pueda inducir a error sobre el estado vigente.
