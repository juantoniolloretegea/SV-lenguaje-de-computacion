# S26 · Lista de comprobación de auxiliares y límites de confianza

**Instrucción vigente S29, 14/09/2026:** [Rust obligatorio, localización y provisión del entorno](../tuberias-ia/recuperacion-rust-s25/OBLIGACION_RUST_Y_PROVISION_DEL_ENTORNO_S29.md). Sustituye la formulación histórica de «primera opción»: Rust se exige en todos los procesos del trabajo SV; cada uso de Python requiere explicación y justificación previas. Se declararán ejecutables, dependencias, conectores y participación de la infraestructura. Los apartados siguientes conservan el estado de su recepción original.

**Estado:** instrucción operativa recibida; comprobaciones materiales pendientes donde se indican.  
**Origen:** indicaciones de Juan Antonio Lloret Egea en esta conversación sobre Python, shell, RAM y representatividad de los ensayos.  
**Alcance:** preparación, ejecución, observación, documentación, registro y publicación del trabajo del Lenguaje SV.  
**Entrada cotejada:** Lenguaje/main `65dddb810c90cd236f24cb23e0d1c1ceee078064`; laboratorio/lab/playground-sv-permanente `dd18b38ac1eac972c381354ee269cce1270e0681`.

## 1. Aplicación desde este incremento

Rust será la primera opción para nuevas herramientas duraderas del proyecto, especialmente cuando validen evidencia, gobiernen registros o decidan una publicación. Cada uso de Python se justificará antes de ejecutarlo, indicando por qué no se emplea Rust en esa tarea, qué puede afectar y cómo se comprobará su resultado. La comodidad o la denominación «auxiliar» no acreditan suficiencia.

La lista no certifica inocuidad universal. Permite documentar una utilización delimitada y los riesgos que siguen abiertos. Si falta evidencia para una propiedad necesaria, esa propiedad queda no acreditada y no se utiliza como fundamento de conformidad. Esto no prohíbe Python, no ordena una reescritura general ni convierte la elección de Rust en prueba de seguridad.

Se aplican los Pilares, el acta de perfiles/contratos/ensamblaje y el acta de transición secuencial, consultados en esta continuidad y cotejados por identidad de blob con el corte de entrada. Se conserva la retirada del compilador Python y la separación entre fallo técnico y Tri.U. Esta lista desarrolla el control del trabajo auxiliar solicitado por el responsable; no cambia contratos del núcleo, constituye un host, abre una fase ni cierra S26.

## 2. Comprobación previa por auxiliar y ejecución

| Comprobación | Información o evidencia exigida |
| --- | --- |
| Finalidad y alternativa | Operación exacta, necesidad, alternativa Rust considerada y razón concreta para la herramienta elegida. Registrar también cualquier impedimento real de acceso o dependencia. |
| Identidad del ejecutable | Fuente/revisión, ejecutable o intérprete, dependencias, configuración y argumentos efectivos. No basta con nombrar un lenguaje. |
| Autoridad de entrada y salida | Qué recibe, qué produce y quién valida antes de que el producto entre en registros o decisiones. Identificar si transporta, transforma, verifica o emite un dictamen. |
| Residencia y acceso | Procesos, buffers, memoria compartida, FFI, archivos, credenciales, red y permisos efectivos que intervienen. Identificar cada ruta de escritura y los objetos que puede alcanzar. |
| Mutación y custodia | Qué copia se verifica y cuál se consume; quién puede modificar ambas entre esas operaciones. Un hash inicial o un puntero conservado no resuelven por sí solos esa pregunta. |
| Recursos | Límites necesarios de tamaño, memoria, salida y duración; comportamiento al alcanzarlos. No atribuir cuotas a un montaje que no las impone. |
| Errores e interrupciones | Rechazo, excepción/pánico, terminación, salida parcial y efecto ya producido diferenciados. Un proceso sin informe no demuestra ausencia de efecto. |
| Escritura y recuperación | Frontera de confirmación, conjunto de archivos afectado, interrupción entre pasos y recuperación de la misma operación. No confundir escritura local, publicación remota y persistencia durable. |
| Guardas efectivas | Las condiciones necesarias para aceptar o publicar deben seguir ejecutándose en la configuración desplegada. No depender de comprobaciones eliminables por optimización. |
| Reintentos y concurrencia | Identidad de encargo/intento, estado previo, respuesta perdida, duplicación y trabajo concurrente. Un checkpoint anterior no sustituye la comprobación de su aplicabilidad actual. |
| Observador | Positivo pertinente y alteración conocida que prueben sensibilidad. Identificar independencia respecto al fallo ensayado y captura del consumidor o destino necesario. |
| Disposición | Uso justificado dentro de un alcance, uso pendiente de comprobación o uso rechazado para esa operación; responsable, evidencia y límites explícitos. No rellenar automáticamente con «inocuo». |

Una justificación puede cubrir una operación repetida sólo mientras permanezcan identificados e iguales su alcance, entradas autorizadas, versión, permisos y condiciones relevantes. Un cambio obliga a revisarla.

## 3. RAM, recolector y fronteras

La lista exigirá describir la memoria realmente accesible al auxiliar. Que un intérprete gestione sus propios objetos no demuestra que gestione la memoria del núcleo; tampoco demuestra que carezca de capacidad para alterarlo mediante memoria compartida, FFI, permisos del sistema o archivos que el núcleo consumirá después.

La separación entre procesos se anotará como característica del montaje. Su suficiencia frente a un proceso comprometido, un host privilegiado o un fallo físico requiere pruebas y un perfil propios. No se acreditará aislamiento por el solo nombre del lenguaje ni por declarar que no hay punteros compartidos.

Gestión automática de memoria, manejo de errores, aislamiento, corrección lógica y durabilidad se comprobarán como propiedades distintas. La selección de Rust no exime de demostrar las fronteras de escritura, el tratamiento de fallos ni la identidad del contenido consumido.

## 4. Hallazgos estáticos recibidos y contraste pendiente

Los siguientes hallazgos proceden de lectura del código existente. No son una campaña de fallos ejecutada ni una invalidación retrospectiva de R01–R05.

| Auxiliar existente | Hallazgo | Evidencia requerida al corregir |
| --- | --- | --- |
| `soporte/publicar_archivos.py` | Usa `assert` para condiciones necesarias de base, blobs y árbol final. | Positivo y entradas incompatibles; guardas efectivas bajo todas las configuraciones admitidas. Ninguna publicación autorizada por omitir una comprobación. |
| `soporte/registrar_continuacion.py` | Usa `assert` y escribe sucesivamente varios archivos sin una transacción conjunta implementada en el auxiliar. | Interrupciones entre escrituras; identificación del estado parcial y recuperación sin pérdida, duplicación ni falsa declaración de conjunto completo. |
| `soporte/publicar_archivos.py` | Un checkpoint con `verified` provoca retorno anticipado sin un cotejo nuevo de entradas o referencia remota. | Checkpoint reutilizado con encargo, archivos o base diferentes: rechazo o nueva comprobación suficiente; positivo legítimo conservado. |
| `r05/reproducir.py` y `r05/conservar.py` | Preparan material y también comprueban condiciones o resultados. «Sólo orquesta» no describe todo su alcance. | Separar transporte, preparación y juicio; identificar qué comprobación sustenta cada afirmación y cómo responde al material inválido. |

La [documentación oficial de Python sobre assert](https://docs.python.org/3/reference/simple_stmts.html#the-assert-statement) especifica su eliminación cuando se solicita optimización. No se afirma que las ejecuciones históricas utilizaran esa opción. La revisión tampoco acredita que se produjeran escrituras parciales en ellas.

Los archivos históricos conservan su contenido y resultados. Una reparación tendrá versión y evidencia propias. Traducir literalmente el código a Rust sin cambiar un protocolo defectuoso no satisface los contrastes de esta tabla.

## 5. Herramientas utilizadas para esta incorporación

No se ejecutan los auxiliares Python revisados. Las lecturas locales utilizan programas de lectura y Git mediante el lanzador de comandos, sin lógica de conformidad escrita en shell. La edición utiliza la herramienta de parches. Se utiliza el conector GitHub directamente para lectura y publicación; en esta intervención el adaptador Python no es necesario.

Las llamadas y cotejos documentales se coordinan mediante la herramienta de ejecución JavaScript del entorno. Este hecho queda explícito: no constituye una nueva utilidad Rust ni evidencia de aislamiento material. El objeto comprobado es el cambio documental de dos archivos, con conservación del resto del árbol y cotejo de los bytes publicados. No se invoca compilador ni se ejecutan pruebas de comportamiento del SV para este incremento documental.

## 6. Continuación

La lista acompaña a la [recepción R06](r06/README.md) y a sus ocho discriminadores pendientes. Antes de reutilizar un auxiliar afectado para acreditar una propiedad necesaria, se resolverá su defecto o se empleará una vía alternativa comprobada en ese alcance. Se priorizarán las herramientas capaces de modificar registros o declarar conformidad, mediante cambios acotados y regresiones pertinentes.

Se mantienen la fase S26 abierta, los resultados anteriores y la secuencia Bis → catálogo/cierre de fase → S24. Esta incorporación no añade sondas ejecutadas, no selecciona BD/GUI/host y no convierte ensayos de proceso en pruebas de resistencia física de RAM o almacenamiento.

## 7. Recepción de reparación R08

RETP-2026-235: [candidata y resultados Rust](r08/RESULTADOS.md). Quince casos cumplidos en debug y optimizado; preparación separada y comprobación sobre cinco registros reales. Este auxiliar ofrece la vía alternativa para las correspondencias y escrituras de su perfil. Se conserva el control sobre autoridad, frescura del adaptador, concurrencia, recursos y durabilidad descrito en el contrato R08. La recepción no rehabilita automáticamente los auxiliares heredados ni convierte sus checkpoints en evidencia suficiente. R06 continúa pendiente.
