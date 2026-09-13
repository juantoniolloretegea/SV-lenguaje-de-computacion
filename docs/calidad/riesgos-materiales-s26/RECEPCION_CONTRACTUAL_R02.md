# S26 · Recepción contractual R02: custodia, lectura y confirmación

**RETP-2026-225 · 13/09/2026 · obligaciones documentadas; realización pendiente.**

## 1. Mandato, antecedentes y alcance

Se recibe la continuación autorizada por Juan Antonio Lloret Egea sobre el
eslabón material entre consulta, RAM, persistencia y futura GUI. El punto de
partida es Lenguaje/main `829cf7611bd5a543dbacf381a1e691d9b5fc4ce4` y
laboratorio/lab/playground-sv-permanente
`f0bc7f1d56a0d1c124fc031e9a1eb199e7b7295b`.

Se han cotejado las 26 fuentes de R01 y los 48 archivos de su publicación de
resultados. Los tres rectores leídos completos en esta continuidad —Pilares,
acta de perfiles/contratos/ensamblaje y transición secuencial con sus adendas—
conservan las huellas registradas en [FUENTES R01](r01/FUENTES.json).
Se consultan además las sedes enlazadas en §2. Las fuentes privadas permanecen
fuera de este expediente público.

Esta recepción desarrolla las obligaciones materiales de las sedes existentes.
Sus etiquetas C01–C08 y T01–T08 son locales al documento: no son tipos, estados,
códigos de error ni primitivas nuevas de SV. No cambia la representación de
Frame, la asignación de parámetros, las estructuras de almacenamiento del
núcleo ni los resultados históricos de los ensayos. Tampoco elige una BD.

## 2. Qué se reutiliza

| Sede | Obligación ya constituida que se recibe | Trabajo material todavía necesario |
| --- | --- | --- |
| [R2-0 §§4–7](../../arquitectura/CONTRATO_R2_0_PERSISTENCIA_CONTINUIDAD_Y_RECUPERACION_2026_08_25.md) | Separación de estados, AStore, vistas, cobertura, PDep y vigencia. | Especificar quién suministra la expectativa y cómo se obtiene una lectura coherente en la realización elegida. |
| R2-0 §§10–12 | Recuperación no circular, compromiso–efecto, indeterminación y consumo único. | Identificar frontera durable, política de confirmación, observadores y reinicio del soporte concreto. |
| [Continuidad y ligaduras por operación §§2–4](../../arquitectura/CONTRATO_DE_CONTINUIDAD_Y_LIGADURAS_POR_OPERACION_FILA_7_2026_09_08.md) y [LIG material](../../arquitectura/CONTRATO_MATERIAL_DE_LIGADURAS_DFL_005_2026_09_08.md) | Referentes exactos, instancia, posición, constitución, procedencia y alcance. | Resolver operacionalmente esos referentes sin sustituirlos por nombres, índices ni primera coincidencia. Se conserva la sucesión registrada de LIG, no el estado histórico de ausencia de realización de su antecedente. |
| [Nota de frame histórico y consulta presente](../NOTA_TECNICA_SOBRE_FRAME_HISTORICO_REAPERTURA_Y_CONSULTA_PRESENTE_2026_03_19.md) | Historia acreditada, reapertura, cobertura y consulta presente distintas. | Declarar qué corte y qué dependencias gobiernan cada respuesta y su presentación. |
| [Captura A/V §1–4](../tuberias-ia/ie004/reserva-p3/CONTRATO_SEPARACION_CAPTURA_A_V.md) | Original desde custodia, propuesta sin autoridad, comprobación sobre instantánea y cuerpos separados. | Reutilizar la separación de autoridad; sus formatos, cuotas y reserva P3 no se generalizan a todo SV. |
| [Inventario y sondas R01](INVENTARIO_MATERIAL_R01.md) | Evidencia de conservación del buffer, cotejo exacto y detección de captura alterada en el montaje. | Cubrir lo que esos cuatro contrastes no observan: adquisición, selección concurrente, commit, recuperación y consumidor efectivo. |

DFL-003/004/005/006 reciben el enlace por operación; no se abre otra deuda con
el mismo objeto ni se declara cerrada ninguna de ellas.

## 3. Obligaciones exigibles al recorrido que pretenda ofrecer la capacidad

### C01 · Referencia esperada y custodia

Antes de aceptar material propuesto, el consumidor debe disponer del encargo y
sus referentes esperados desde la custodia aplicable. Deben poder identificarse
su procedencia, alcance y mecanismo de protección frente al fallo ensayado.
Un hash incluido por el mismo proponente sólo permite comprobar consistencia
con esa declaración. No constituye por sí mismo el referente autorizado.

Cambiar coherentemente fuente, contexto, registro y sus hashes puede producir
otro conjunto localmente consistente. El consumidor del encargo anterior debe
rechazar su sustitución. Admitir el nuevo conjunto bajo un encargo nuevo,
legítimamente constituido, debe seguir siendo posible. Si el montaje permite
que el adversario reemplace también la expectativa confiable y toda su prueba,
la resistencia a ese adversario no queda acreditada por el comparador local.

### C02 · Consulta identificada y corte coherente

El contrato de consulta debe precisar si solicita un contenido histórico fijado,
una proyección al corte declarado o el estado actualmente acreditable. Debe
fijar las identidades y dependencias necesarias para esa operación: programa,
constitución/arquitectura, Frame o historia pertinente, nodo/instancia, ligadura,
posición y campo. No se impone una clave universal que todos deban almacenar.

Px/Cy/Fz abreviados sólo son suficientes si esas dependencias se resuelven sin
ambigüedad. El consumidor no elegirá por nombre, posición de pantalla, fecha,
hash de valor, puntero RAM ni último elemento recibido. La ausencia de un campo
en un Frame parcial no se rellena con el valor de otro Frame ni con U.

Todas las piezas necesarias se comprueban contra el mismo conjunto de
referencias fijado. Pueden provenir de lecturas físicas sucesivas si se acredita
esa coherencia; no se exige simultaneidad física universal. La implementación
deberá declarar cómo inmoviliza o verifica los objetos consumidos y cómo evita
que una sustitución posterior a la comprobación cambie el objeto usado.

Para una consulta histórica exacta, el contenido recuperado satisfactoriamente
debe permanecer igual bajo las mismas dependencias inmutables. Eso no garantiza
que todas las llamadas respondan: pérdida de acceso, corrupción o dependencia
no disponible pueden producir un fallo explícito. Una proyección abierta puede
evolucionar; su resultado debe indicar el corte al que corresponde. La autoridad
presente para actuar no se hereda del contenido histórico consultado.

### C03 · Residencia y mutación

Cada realización debe describir el recorrido real desde entrada hasta consumidor:
buffers propios o prestados, estado candidato, objeto admitido, copias,
serialización, cachés, archivos/BD, recuperación y permisos de escritura.
También debe identificar quién puede sustituir la selección aunque no pueda
mutar el objeto anterior. Un puntero es un mecanismo de acceso del proceso; no
se empleará como identidad durable.

El objeto validado, el evaluado y el entregado deben conservar una ligadura
comprobable. Una modificación legítima del contenido histórico se representa
según su contrato mediante una nueva ocurrencia/revisión y su relación explícita;
no reescribe silenciosamente el antecedente. Append-only lógico requiere una
realización que lo sostenga frente a los fallos incluidos en su perfil.

### C04 · Validación y confirmación material

Se debe declarar por separado qué prueba cada evidencia del recorrido:

| Evidencia | Alcance admisible | Condición adicional para una afirmación superior |
| --- | --- | --- |
| Admisión de objeto en RAM | Cumplimiento de las comprobaciones del admisor sobre ese objeto. | Política material y evidencia de persistencia para afirmar que sobrevive al fallo contemplado. |
| Escritura aceptada | Aceptación por la API usada, dentro de su contrato. | Confirmación del backend bajo su política declarada de durabilidad y modelo de fallo. |
| Confirmación durable | Operación y dependencias conservadas conforme al perfil acreditado. | Evidencia de continuidad para atribuir vigencia; captura del consumidor para atribuir presentación. |
| Captura de entrega | El captor observó ese contenido en la frontera declarada. | Observación de pantalla si se afirma lo mostrado, y del destino si se afirma efecto externo. |
| `Confirmed` lógico de R1 | Respuesta técnica favorable del adaptador en el recorrido inspeccionado. | Protocolo y evidencia material aplicables para afirmar persistencia o efecto externo. |

Antes de una realización durable deben fijarse unidad de transacción,
dependencias incluidas, concurrencia admisible, confirmación observable,
almacenamiento/configuración y fallos cubiertos. RAM puede contener un candidato
sin confirmar; ese candidato no se publicará como confirmado. También puede
existir una operación ya comprometida cuya respuesta se perdió. No se inventa
rollback ni se repite el efecto para resolver esa falta de conocimiento.

La publicación al consumidor y el commit del backend no se presuponen una única
transacción. Su enlace, los cortes intermedios y la reconciliación deben quedar
ensayados. Un identificador estable de la operación ayuda a reconciliarla; no
prueba por sí solo que el efecto ocurrió una sola vez.

### C05 · Índices y búsquedas

Se conserva R2-0 §5: un índice puede localizar, pero su ausencia no acredita
inexistencia sin cobertura suficiente. El índice debe enlazar referentes
comprobables y declarar su cobertura/corte. Se debe poder contrastar la respuesta
con la fuente correspondiente, incluida la reconstrucción tras fallo.

Array, Vec, BTreeMap o HashMap no deciden autoridad ni durabilidad. Una clave
duplicada se tratará conforme al contrato del registro: relectura/idempotencia
acreditada, conflicto explícito o nueva ocurrencia legítima. No se sobrescribirá
el antecedente protegido para ocultar el conflicto. Igual valor en dos
adquisiciones no autoriza deduplicarlas; dos recepciones de la misma adquisición
no acreditan dos hechos del mundo. Si el productor no aporta la distinción,
el consumidor debe conservar esa insuficiencia.

### C06 · Consumidor, selección y acto revisado

La entrega debe conservar su correlación con el encargo, corte y objeto que la
originaron. Una respuesta atrasada de A no puede mostrarse como B ni activar
una acción revisada para B. Las acciones deben ligarse al objeto realmente
revisado y a la autoridad pertinente al despacho. Releer silenciosamente
«lo último» entre revisión y acción vulneraría esa ligadura.

Esta obligación vale para CLI, API, IA y futura GUI. Un ensayo síncrono de
`Capture` en el mismo proceso no cubre concurrencia de selección ni demuestra
lo que aparece en pantalla. El perfil de representación deberá distinguir dato,
estado de operación y fallo técnico sin hacer pasar uno por otro.

### C07 · Observador y recuperación

Los testigos deben corresponder a las operaciones realmente observadas. Dos
copias iniciales no cuentan como lectura anterior y lectura posterior a una
mutación. Para cada negativo deben conservarse el punto de inyección, evidencia
de que el estímulo lo alcanzó, respuesta y observación posterior pertinente,
además del control positivo.

La independencia se define respecto del fallo: otra función del mismo proceso
puede detectar discrepancias lógicas sin resistir un proceso comprometido. Una
copia restaurada no se acredita vigente sólo con sus propios hashes. La
protección contra el host, fallo físico o sustitución de toda la custodia exige
las dependencias materiales que correspondan a R3; permanece no acreditada
si faltan. No se deduce causalidad o identidad de la señal CLK ni de una fecha.

### C08 · Recursos, errores y efectos inciertos

El perfil de cada ensayo debe acotar entrada, acumulación, salida, captura y
concurrencia pertinentes, e identificar el mecanismo que impone sus límites.
Si usa plazos técnicos, debe declarar qué observan y cómo se ensayan; no los
convierte en primitiva semántica universal. Cuotas del vector no son cuotas
del proceso ni del recorrido completo.

Error de lectura, permiso, integridad, confirmación o captura mantiene su causa
y frontera. No se transforma en U, cero, ausencia demostrada o éxito. La
indeterminación del efecto se conserva hasta reconciliación constituida. Las
obligaciones se aplican a la operación dependiente; no bloquean por defecto
lecturas experimentales que declaren y respeten un alcance menor.

## 4. Discriminadores previos y evidencia que se exigirá

Esta tabla especifica contrastes; **ninguno de T01–T08 se declara ejecutado**.
Antes de ejecutarlos se congelarán fixtures literales, código, soporte,
instrumentación, oráculos y cuotas de la campaña correspondiente. R01 conserva
su propio banco, sus cuatro sondas y sus límites.

| Contraste local / S26 | Negativo y control positivo | Evidencia de aceptación exigida | Dependencia |
| --- | --- | --- | --- |
| T01 / F01 | Sustituir toda la propuesta B conservando el encargo confiable A. Positivo: A; y B con encargo nuevo autorizado. | Encargo reservado antes de recibir propuesta; huellas y respuestas; B bajo A rechazado. Sustitución también del custodio se informa fuera del perímetro local. | Adaptar el montaje de custodia existente, sin fingir raíz externa. |
| T02 / F02/F05 | Leer fuente A y dependencia B, o sustituir el objeto entre cotejo y consumo. Positivo: conjunto coherente según referencias fijadas. | Barreras de ejecución reproducibles, lecturas efectivas, referencias y bytes consumidos; mezcla rechazada antes del uso protegido. | Puntos de intervención declarados; comparación con oráculo ajeno al resultado bajo prueba. |
| T03 / F04/F09 | A responde después de seleccionar B. Positivo: respuesta B aplicada a B. | Orden controlado, selección, correlación y captura del consumidor; A separada o descartada sin acción atribuida a B. | Consumidor concurrente; ensayo previo con consumidor de prueba identificado no acredita GUI. |
| T04 / F06/F11 | Cortar antes/durante/después de commit y perder su respuesta. Positivo: confirmación y recuperación ordinarias. | Estado antes, fallo inyectado, nuevo proceso, consulta por operación y efecto observado; ninguna confirmación falsa ni reintento no gobernado. | Backend y política durable definidos; no sustituibles por mapa en RAM. |
| T05 / F01/F03/F07 | Índice atrasado, clave en conflicto, dos adquisiciones iguales y doble recepción. Positivo: referentes distintos y relectura legítima conservados. | Fuente, cobertura, relaciones de origen/recepción y respuestas; no pérdida del antecedente ni falsa ausencia. | Fuente de identidad de adquisición; almacén/índice reales para la parte persistente. |
| T06 / F08 | Restaurar historia íntegra anterior al consumo/revocación. Positivo: recuperación bajo continuidad acreditada. | Estado restaurado y testigo suficiente fuera del mismo fallo; no revivir facultades. | Perfil de continuidad y recuperación R2/R3. |
| T07 / F05/F10 | Mutar tras el supuesto `before`; reutilizar copia inicial como `after`. Positivo: lectura efectiva posterior ligada al objeto esperado. | Instrumentación de lectura antes/después y bytes observados; el observador distingue estado alterado de testigo copiado. | Conductor y observador del montaje, sin afirmar BD ni aislamiento externo. |
| T08 / F10/F12 | Truncar captura o agotar la cuota del tramo. Positivo: contenido completo dentro de cuota, incluido U legítimo cuando proceda. | Bytes/operaciones contados, corte de fallo y salida al consumidor; no éxito ante observación incompleta. | Cuotas y mecanismo del tramo; la pantalla necesita captura efectiva posterior. |

La calificación sigue [S26: qué cuenta como evidencia](README.md): detección,
prevención en alcance, no acreditado o fallo observado. Para prevención debe
observarse la frontera del efecto prohibido; un diagnóstico por sí solo no basta.

## 5. Relevo concreto

1. **Siguiente incremento acotado:** preparar T01, T02 y T07 sobre una copia
   identificada del admisor/conductor existente. Comprometer sus oráculos antes
   de correrlos. Permiten atacar custodia, mezcla de referencias y testigos
   aparentes sin inventar una BD ni una GUI.
2. **R2:** recibir T04–T06 en el perfil material de la operación que necesite
   persistencia. Declarar backend, protección de custodia, transacción,
   cobertura y recuperación antes de atribuir durabilidad. Este documento
   no escoge entre implementaciones ni promete que el perfil ya existe.
3. **Bis/S22 y representación:** recibir T03/T08 cuando el consumidor y sus
   límites estén fijados; conservar las fronteras pendientes del montaje.
4. **S24:** mantener Bis → catálogo/cierre de fase → análisis e instalación
   de GUI. Su entrada deberá declarar qué evidencia previa es reutilizable y
   qué debe probarse sobre el consumidor visual real.

**Resultado de esta recepción:** ocho obligaciones y ocho discriminadores
ligados a sedes existentes. Cero nuevas pruebas de comportamiento ejecutadas,
cero casos globales S26 cerrados. R01 mantiene cuatro sondas conformes en una
campaña. La conformidad administrativa de este documento no acredita el
comportamiento futuro descrito ni cierra R2/R3/R4, DFL o Bis.

## 6. Trazabilidad de esta incorporación

`r02/registrar.py` conserva las ediciones administrativas y verifica preservación
de antecedentes, registros y estado Bis. `r02/VERIFICACION.json` registra ese
contraste documental y las huellas previas/posteriores de los archivos afectados.
`r02/publicar.py` declara archivos y destinos; reutiliza los auxiliares GitHub
de S26 para publicar sin force y cotejar el árbol completo y las copias.
Se utilizan Python y operaciones de lectura/escritura local para estos archivos,
y el conector GitHub para publicación. No se invoca otra IA, servicio de
inferencia, nueva instalación, compilación, BD o ejecución SV en este incremento.
