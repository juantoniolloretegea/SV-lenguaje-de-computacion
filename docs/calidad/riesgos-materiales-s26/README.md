# S26 · Riesgos materiales del frame y preparación de contrastes

**Estado: en ejecución. Referencia: RETP-2026-222.**

## Origen y objeto

Por instrucción expresa de Juan Antonio Lloret Egea se registra esta actividad:
«Sí podemos definir ahora los fallos que intentaremos provocar y qué evidencia
exigiremos para considerarlos detectados o impedidos».

La pregunta nació al examinar la futura GUI y retroceder hasta la identidad de
las consultas, la distinción entre observación y recepción, el reloj, la RAM,
los índices, la persistencia y las transacciones. Una presentación puede mostrar
una referencia distinta de la consultada, atribuir firmeza a un candidato o
autorizar una acción distinta de la revisada. El estudio abarca también CLI,
API, conectores, productores, cachés y recuperación.

El alta reconoce que la reflexión ya comenzó en la conversación; no atribuye a
esa reflexión una fecha retrospectiva ni una campaña ejecutada. Se inicia ahora
su seguimiento documental. Responsable: **Watson / W-S0**.

## De dónde venimos

Cortes de entrada cotejados al preparar el alta:

- Lenguaje/main: `2fc90a08575484ce7c2af9f3fc544c2249c90c03`.
- Laboratorio/lab/playground-sv-permanente:
  `500b283729a8930f8c23033df3fe369d5370759b`.

Se reutilizan Pilares, el acta de perfiles/contratos/ensamblaje y el acta de
transición secuencial, leídos completos en esta continuidad al corte indicado.
Las sedes específicas son R2-0, LIG, Frame y ejecución R1; no se constituye otro
almacén ni una identidad universal. La consulta del anexo privado SEC.5 en el
antecedente no autoriza copiar su contenido a este expediente público.

| Sede existente | Obligación recibida | Límite que se conserva |
| --- | --- | --- |
| [R2-0 §§4–7](../../arquitectura/CONTRATO_R2_0_PERSISTENCIA_CONTINUIDAD_Y_RECUPERACION_2026_08_25.md) | Separar proceso, persistencia, derivación, autoridad y continuidad; acreditar cobertura de vistas. | Contrato no demuestra una BD instalada, commit durable ni resistencia al host. |
| R2-0 §§10–15 | Recuperación no circular, dependencia decisión–efecto, reutilización, recursos y frontera temporal. | No se promete exactamente una vez, continuidad por fecha ni protección física desde Rust. |
| [LIG](../../../rust/sv_core/src/bindings.rs) y Pilares §1.4 | Expectativa exacta; instancia, posición, constitución y procedencia. | Una ligadura validada no constituye por sí sola una consulta histórica completa. |
| [Frame](../../../rust/sv_core/src/frame.rs) | Cierre y pertenencia de referencias; conservación del objeto admitido. | Frame.index no es identidad global; validez no exige cobertura de todos los nodos. |
| [R1](../../../rust/sv_core/src/execution.rs) | Traza del ejercicio y separación de compromiso, confirmación e indeterminación. | Varias entradas comparten ExerciseRef; estado conocido de cadena abierta no es entrada histórica fijada. |
| S22 / RETP-218–221 | Montaje experimental, identidad, captura, paridad y recursos acotados. | Bis global sigue abierto; los 202 casos originales C02–C12 no se recuentan como ejecutados. |
| S24 / RETP-204 | Bis → catálogo y cierre de fase → análisis e instalación de GUI. | S26 prepara obligaciones transversales; S24 sigue pendiente. |

## Qué se define ahora

Los siguientes identificadores son casos documentales locales de S26, no códigos
de error ni primitivas del Lenguaje. **Los doce están pendientes de ejecución.**
Cada fila incluye el control positivo que impide confundir un bloqueo universal
con una protección correcta. Los estímulos literales, puntos de inyección y
versiones del montaje se fijarán antes de ejecutar cada campaña.

| Caso | Fallo que intentaremos provocar | Control positivo | Evidencia exigida y criterio adversarial |
| --- | --- | --- | --- |
| S26-F01 | Sustituir una referencia exacta por otra con igual nombre, índice, valor o polígono. | Dos ocurrencias legítimas con igual valor siguen siendo consultables por sus referencias. | Petición fijada, dependencias y respuesta recuperada; sustitución rechazada o identificada, sin sobrescribir la primera. Un hash del valor solo es insuficiente. |
| S26-F02 | Resolver Px por primera coincidencia, cambiando instancia, posición o constitución de Cy/Fz. | Consulta con instancia y posición inequívocas devuelve el componente fijado. | Constitución y ligaduras exactas frente a componente entregado; ambigüedad o ausencia explícita, sin seleccionar ni fabricar U. |
| S26-F03 | Confundir dos entregas de una observación con dos adquisiciones, o fusionar adquisiciones distintas de igual valor. | Relectura de la misma observación y adquisición nueva identificada conservan su relación declarada. | Identidades de origen/adquisición y recepción capturadas por separado. Si el origen no permite distinguirlas, resultado no acreditado; un ID de recepción nuevo no basta. |
| S26-F04 | Hacer que caché, reloj, orden de llegada o proyección abierta alteren una consulta histórica fijada. | Historia fijada permanece igual; consulta explícita al estado conocido puede evolucionar. | Mismo encargo/corte y dependencias, respuestas comparadas; ninguna selección silenciosa de «último». Variar reloj sólo en montaje aislado y declarado. |
| S26-F05 | Publicar B provisional de RAM como confirmado, o mezclar datos/dependencias de A y B durante una lectura. | Candidato etiquetado como tal y lectura coherente del estado confirmado. | Identidad del objeto validado y publicado, frontera de confirmación constituida y captura del lector; detener el recorrido en puntos declarados. Igualdad de punteros o un println no prueban confirmación. |
| S26-F06 | Interrumpir escritura, agotar espacio o perder la respuesta de commit y asumir éxito o fracaso sin evidencia. | Confirmación y recuperación conforme a la política durable declarada. | Operaciones del backend, fallo inyectado, lectura tras reinicio y reconciliación de la misma operación. Respuesta perdida no prueba rollback; write aceptado no prueba durabilidad. |
| S26-F07 | Usar índice atrasado/reconstruido para afirmar ausencia o servir una referencia distinta. | Índice con cobertura acreditada localiza el objeto correcto. | Fuente autoritativa, corte/cobertura del índice y resultado contrastados; ausencia sin cobertura no se convierte en inexistencia ni U. |
| S26-F08 | Restaurar copia antigua íntegra y presentarla como vigente, reviviendo permiso consumido/revocado. | Recuperación bajo continuidad acreditada preserva consumo y revocación. | Dependencias y testigo de continuidad suficiente frente al fallo. Si toda evidencia procede de la misma copia restaurada, vigencia no acreditada. |
| S26-F09 | Llegar tarde una respuesta A tras seleccionar B y mostrarla/usar sus acciones como B. | Respuesta correspondiente a la selección activa se muestra vinculada a ella. | Secuencia controlada de solicitudes/respuestas y captura efectiva del consumidor; A descartada o claramente separada y sin acciones atribuidas a B. Un log del productor no prueba lo mostrado. |
| S26-F10 | Cambiar bytes, posiciones, negación, leyenda, precisión o unidad entre dato y representación. | Correspondencia íntegra para el perfil y dispositivo declarados. | Estado de referencia independiente, descriptor entregado y captura final; diferencias detectadas y uso dependiente contenido. Sin confundir equivalencia semántica con igualdad de píxeles. |
| S26-F11 | Doble clic, reconexión o carrera entre revisión y despacho repiten efecto o aplican otra versión/autoridad. | Intento gobernado sobre el objeto revisado; reutilización sólo según contrato. | Objeto revisado, compromiso, ejercicio y observación del destino; un reintento de transporte no crea autoridad. Confirmed/Indeterminate se conservan; no se prueba ausencia de efecto sólo con error de interfaz. |
| S26-F12 | Fallo técnico, desconexión o agotamiento de recursos se presenta como U, cero, éxito o dato fresco. | U legítimo se conserva y fallo técnico permanece identificable. | Causa inyectada, salida completa o truncamiento declarado, señal efectiva al consumidor y estado del destino. Recursos y cobertura de captura acotados previamente; sin cuotas, no se acredita resistencia al agotamiento. |

## Qué cuenta como evidencia

Cada ejecución deberá conservar, con identidad propia: caso/revisión, corte de
código y configuración, soporte y límites, entrada literal, referencia esperada,
control positivo, punto de inyección, orden de operaciones, comandos/scripts y
su contenido, servicios/conectores/herramientas, salidas, estados y captura del
consumidor o destino pertinente. Las fechas son metadatos de la campaña, no una
prueba de identidad, vigencia ni causalidad por sí solas.

- **Detectado:** el fallo alcanzó el punto previsto, un observador cualificado
  registró una discrepancia identificable y se conserva qué efectos ya ocurrieron.
- **Impedido:** además se acredita que el efecto prohibido no cruzó la frontera
  declarada, con observación suficiente del destino y del intervalo de ensayo.
  Debe comprobarse que el estímulo sí alcanzó la guarda; no ejecutarlo no es impedirlo.
- **No acreditado:** falta cobertura, el observador falla o no puede determinarse
  el efecto. No se fuerza un resultado favorable ni se transforma en Tri.U.
- **Fallo observado:** se produce la conducta prohibida o una supuesta guarda no
  distingue su testigo negativo. Se conserva la campaña; una reparación exige
  variante/revisión nueva y repetición identificada, sin reescribir el resultado.

Cada observador deberá demostrar sensibilidad a una alteración conocida y admitir
el positivo pertinente. Su independencia se evaluará respecto del fallo estudiado:
otra función del mismo proceso no acredita resistencia al proceso comprometido.
La prueba de ausencia requiere límites explícitos de cobertura; una ventana finita
no acredita ausencia universal de efectos tardíos.

## Adónde vamos y condiciones de relevo

1. **Ahora, S26:** inventariar para cada recorrido dónde residen los bytes, quién
   puede mutarlos, qué copias existen, qué autoridad tienen y qué ocurre entre
   validación, publicación, confirmación y recuperación. Relacionar cada brecha
   con su sede existente y con los casos anteriores; no elegir BD/GUI por anticipado.
2. **Antes de nuevas realizaciones afectadas:** fijar el contrato material que
   falte en la sede competente, los fixtures y el oráculo previo. Implementar y
   ensayar en laboratorio únicamente el alcance justificado. Retener evidencias
   suficientes para distinguir supuesto, declaración, comprobación y pendiente.
3. **Antes de activar S24:** entregar la matriz de riesgos con disposición de
   cada caso: demostrado en un alcance, pendiente con dependencia, o no aplicable
   con justificación. Las obligaciones materiales necesarias para el uso elegido
   deben estar satisfechas; un pendiente no puede presentarse como garantía.
4. **Con GUI candidata y tras el relevo secuenciado:** concretar y ejecutar los
   contrastes de representación/acción, concurrencia y fallos en esa realización.
   Un visor experimental limitado debe declarar sus límites; no hereda autoridad
   ni persistencia productiva de la apariencia visual.

La matriz inicial está preparada; la campaña material y el inventario de recorridos
permanecen pendientes. S26 no cierra R2/R3/R4, DFL ni Bis; tampoco reinicia S24.
La auditoría local antecedente de identidad mantiene su alcance propio y no se
recuenta como ejecución de estas doce filas.

## Trazabilidad administrativa

El alta y este documento se incorporan a Sucesos SV (CSV, Markdown e historial),
RETP (CSV y Markdown), Léame primero y al enlace transversal del estado Bis.
El script `soporte/registrar.py` conserva el contenido exacto de las ediciones;
`soporte/publicar.py` y sus auxiliares declaran la publicación por GitHub y el
cotejo de árboles. El espejo de laboratorio conserva los mismos bytes aplicables.
El commit publicado identifica cada incorporación; no se anticipan hashes futuros.

No se ejecuta una nueva prueba Rust para esta alta documental. Las comprobaciones
administrativas verifican conservación de registros, correspondencia de formatos
y enlaces; no validan el comportamiento futuro descrito en la matriz.

## Continuación R01 / RETP-2026-223

Inventario R01 de nueve tramos materiales y disposición de los doce casos S26. Cuatro sondas parciales comprometidas sobre el montaje Rust existente y fixture I0205-01 literal. Cero sondas nuevas ejecutadas. [Inventario material](INVENTARIO_MATERIAL_R01.md).

## Continuación R01 / RETP-2026-224

Cuatro sondas R01 conformes en Rust/Cargo 1.98.0: positivo documental, archivo alterado después de recepción con nueva lectura rechazada I02, captura alterada detectada D06 y ausencia/fallo posterior D01/D07. Núcleo y admisor existentes sin cambios. [Inventario material](INVENTARIO_MATERIAL_R01.md).

## Recepción R02 / RETP-2026-225

Recibidas ocho obligaciones materiales y ocho discriminadores previos en S26, con relevo a R2-0, consulta histórica y DFL-003/004/005/006. Se precisan custodia, lectura coherente, residencia, confirmación, índices, consumidor, observador y recursos. Incremento documental: cero nuevas pruebas de comportamiento; cuatro sondas R01 anteriores conservadas; cero casos globales S26 cerrados. Sin BD, GUI, nueva primitiva o garantía material acreditadas. S22 activo y S24 pendiente conservan su secuencia.

Preparar T01/T02/T07 sobre copia del montaje existente: sustitución de propuesta bajo expectativa fija, mezcla de dependencias y lectura posterior efectiva. Publicar fixtures, código y oráculos antes de ejecutarlos.

[Recepción contractual](RECEPCION_CONTRACTUAL_R02.md).

## Continuación R03 / RETP-2026-226

Banco R03 previo: tres sondas parciales T01/T02/T07, con controles de sustitución bajo custodia fija, lectura mezclada en barrera explícita y testigo posterior copiado frente a releído. [Inventario material](r03/README.md).

## Continuación R03 / RETP-2026-227

R03: tres sondas conformes a sus oráculos. T01 rechaza B bajo A y admite B bajo su custodia declarada; T02 rechaza fuente mezclada I02; T07 reproduce conformidad con copia inicial pese a archivo alterado y detecta alteración con relectura real. [Inventario material](r03/README.md).

## Continuación R04 / RETP-2026-228

Variante experimental R04 de recepción del testigo: lectura inicial y posterior reales con cuota; entrega y preservación separadas. Ocho sondas previas P01-P08 con controles y fallos de lectura. [Inventario material](r04/README.md).

## Continuación R04 / RETP-2026-229

R04: ocho sondas conformes. Relectura posterior detecta cambio literal y del vector; ausencia, error de lectura y exceso se distinguen sin borrar entrega previa. Fallo o cambio inicial detienen el recorrido; el observador heredado conserva su límite ante copias falsamente posteriores. [Inventario material](r04/README.md).

## S26 R05 · RETP-231

R05: siete sondas previas fijadas para retorno rechazado, ausencia de captura, pánico y sustitución de ruta tras apertura. Copia local instrumentada; sin cambio productivo. Ejecutar siete sondas R05; conservar lecturas reales, informes ausentes y sustitución de objeto con bytes iguales sin promover garantías. [Banco y evidencia](r05/README.md).
