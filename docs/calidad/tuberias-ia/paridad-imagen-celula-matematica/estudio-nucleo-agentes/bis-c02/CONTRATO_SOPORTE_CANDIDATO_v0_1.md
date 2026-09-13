# Contrato candidato de recepción de tamaños de soporte

**Versión 0.1 · S22 · BIS-02/C02 · 13 de septiembre de 2026 · Juan Antonio Lloret Egea y Watson**

## 1. Estatuto y finalidad

Este contrato desarrolla BIS-C02 para preparar la decisión de sede de BIS-03. Es una propuesta experimental; no constituye un perfil productivo, un tipo canónico de IR, una primitiva ni una extensión de la gramática. Su objeto es separar la conformidad algebraica de una célula y la pertenencia de su dimensión al conjunto declarado por una versión de soporte. La posterior realización exige decisión de sede y pruebas, conforme al workflow.

La [acta de perfiles, contratos y ensamblaje](../../../../ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md), §§4, 6, 7 y 9, exige identidad, versión, fuente competente y evidencia. La fuente autorizada no se sustituye por una huella: ésta identifica bytes y no confiere autoridad. La Frontera normativa A.2 conserva N=b² y b≥3. El resultado nativo de BIS-C01 acredita esa ruta para sus entradas y deja separado el soporte.

## 2. Condición que se quiere comprobar

Para cada especificación celular de una unidad previamente validada, se exige N ∈ S(id, versión), siendo S un conjunto finito expresamente declarado. El juicio algebraico N=b² permanece en su sede. La versión seleccionada puede admitir varias dimensiones, sin adoptar nueve por defecto ni constituir un máximo universal del SV.

Una modificación de S crea una nueva versión declarada y exige revisión del impacto en semántica, IR, construcción, representación, operaciones y recursos. La revisión determina qué piezas deben modificarse y qué evidencia debe repetirse; añadir un tamaño no implica por sí mismo una nueva regla algebraica. La instancia recibida bajo una versión no cambia de dimensión ni hereda otra versión por esa actualización.

Se conservan tres juicios: geometría conforme, pertenencia al soporte de prueba y autorización operacional. Este ensayo sólo especifica los dos primeros y su enlace. La autorización operacional necesita además constitución de dominio, facultades del agente y requisitos de la operación; no se presume a partir de un tamaño.

## 3. Entradas y ligaduras del montaje candidato

El montaje recibe una unidad SVP y su perfil fuente explícito, una referencia exacta al soporte —identidad, versión y huella— y un registro de perfiles de prueba custodiado por separado. El registro no se toma del cuerpo de la petición. Sus bytes y los de cada perfil se comprometen antes del ensayo.

El núcleo recibe la unidad por su entrada pública de compilación. Las dimensiones que se comparan proceden de los objetos CellSpec de la IR aceptada, no de un N afirmado por el solicitante ni del número de parámetros inventariados. El control abarca todas las CellSpec presentes en la unidad del ensayo. La admisión parcial de unidades mixtas queda fuera de este contrato candidato y no puede habilitarse silenciosamente.

La referencia nominal de CellSpec sirve para identificar un objeto dentro de la unidad y no acredita identidad constitutiva de dominio. El recibo experimental deberá enlazar, como mínimo, identidad literal de fuente, versiones de gramática/IR/serializador, perfil exacto, especificación y dimensión comprobada. BIS-C03 debe resolver las ligaduras adicionales de constitución e instancia. Un recibo experimental no se denominará permiso ni célula productivamente admitida.

## 4. Orden y canales de comprobación

1. Validar la unidad SVP por la ruta vigente. Un rechazo previo se conserva y no se atribuye al soporte.
2. Resolver la referencia de soporte contra el registro independiente. Ausencia, versión no registrada y discrepancia de identidad se distinguen; no se selecciona la última versión ni se descarga un sustituto.
3. Comprobar el cuerpo correspondiente contra su identidad comprometida y verificar que la declaración contiene dimensiones explícitas, sin duplicados ni reparaciones. Una declaración vacía puede representar soporte de ninguna dimensión; nunca permiso universal.
4. Examinar cada CellSpec en el orden de la IR y comprobar pertenencia. Si una dimensión está fuera, no se emite recibo favorable para la unidad completa. No se convierte el resultado en Tri.U ni en E003.
5. Emitir la observación experimental, ligada a entradas y versión. El montaje conserva por separado los fallos instrumentales, los rechazos del Lenguaje y los resultados de soporte.

La precedencia anterior es una propuesta explícita del montaje de prueba, no una nueva precedencia universal del catálogo. En una petición con SVP inválido y soporte ausente, se espera primero el rechazo SVP. Su eventual adopción productiva deberá contrastarse con la frontera que se constituya.

## 5. Banco independiente

La versión sintética 1 declara S={16,25}; la versión sintética 2 declara S={16,25,49}. La segunda existe para demostrar selección explícita y ausencia de ampliación silenciosa. Ninguna de las dos define las dimensiones de inmunología, ciberseguridad o neumología, ni equivale a una versión aprobada del soporte SV.

El [banco previo](BANCO_PREVIO_v0_1.json) concreta catorce variantes de los escenarios C02-P/N, con entradas SVP copiadas sin cambios de BIS-C01 y sus huellas; las salidas esperadas se fijan documentalmente antes de cualquier realización. Incluye admisión de 16/25, rechazo de 9/36/49 bajo v1, admisión de 49 bajo v2, regreso explícito a v1, versión ausente, falta de soporte, dos discrepancias de identidad, conservación de ES/EN y dos rechazos SVP previos.

Las etiquetas de resultado pertenecen exclusivamente al banco. No son códigos nuevos del catálogo. La comprobación de huellas contra el registro no acredita protección frente a un anfitrión capaz de sustituir simultáneamente registro y observador.

## 6. Decisión de sede que recibe BIS-03

| Pieza | Evidencia o necesidad | Propuesta que debe decidirse |
| --- | --- | --- |
| Álgebra y longitud | BIS-C01 y ruta de compilación vigente | Conservar la comprobación nuclear existente. |
| Identidad y conjunto S | No forman parte de las entradas actuales de compile_svp/compile_svp_profile ni de los campos de IrProgram inspeccionados | Ensayar un manifiesto externo tipado y enlazado como candidato; comparar con otras sedes si una obligación no sobrevive. |
| Imposición del soporte | Un documento o una casilla JSON no bloquean una operación | La futura vía operacional debe atravesar la comprobación y conservar su evidencia. Un adaptador optativo no basta para acreditar imposición nuclear. |
| Dimensión después de admisión | La representación actual conserva colecciones encapsuladas; el tipo interno no prueba por sí solo la totalidad del contrato | Comparar arrays y almacenamiento encapsulado al disponer de la sede y los tamaños del soporte elegido. |
| Versiones y capacidades | La pertenencia no prueba coste, GUI, IA, composición o entorno seguro | Mantener esos contratos y sus bancos propios. |

No se fija aún una API productiva ni se introduce un parámetro de soporte en la gramática. El análisis de BIS-03 deberá decidir cómo se preserva la obligación sin depender de que el programador invoque voluntariamente un control. Si el enlace externo resulta insuficiente, se documentará el caso que exige ampliar la representación o la frontera antes de implementar.

## 7. Criterios de terminación y pendientes

C02 sólo podrá declararse ejecutado cuando exista una realización identificada, un observador que contraste las ligaduras completas, los catorce resultados conservados y controles de sensibilidad capaces de detectar omisión de pertenencia, selección automática de versión y sustitución de perfiles. La mera validación documental de este banco no cumple ese criterio.

No se modifica ahora el núcleo ni se adelanta BIS-04. Continúan pendientes la decisión de sede, la realización, el ensayo, la lista productiva de tamaños y los presupuestos necesarios. BIS-C03 será el siguiente desarrollo contractual: identidad de constitución, instancia y revisión, incluida la diferencia entre vectores iguales pertenecientes a instancias distintas.
