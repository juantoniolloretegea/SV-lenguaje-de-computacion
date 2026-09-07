# Contrato candidato F de dominio, representación y suficiencia por operación

**Identidad documental:** `F-SV/0.1-candidata` · **Fecha:** 07/09/2026 · **Registro:** RETP-2026-089.

**Entrada:** Lenguaje `main@4d5f93b612003352a63c8c01bf99e1000a6d0125`, PR #74 integrada.

**Estatuto:** producto de fila 4/F, preparado para el contraste F-IF de fila 5. Especifica obligaciones candidatas; no constituye nuevos tipos, sintaxis, diagnósticos emitidos ni operaciones ejecutables.

## 1. Fundamento, autoridad y cortes

Se reciben completos los [Pilares](../calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), el [acta de perfiles](../calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md) y la [transición hasta §28](../dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md#k1-t-relevo-20260907). El [contrato FFL-E](./CONTRATO_MINIMO_DE_SUFIENCIA_REPRESENTACIONAL_POR_OPERACION_PARA_EL_LENGUAJE_SV_2026_08_21.md), §§2–16, proporciona el criterio de suficiencia; F lo aplica a la recepción de constituciones y a su frontera tecnológica. Su [matriz de impacto](./MATRIZ_DE_IMPACTO_DE_LA_SUFIENCIA_REPRESENTACIONAL_EN_LA_ESPECIFICACION_Y_LA_IMPLEMENTACION_2026_08_21.md) conserva el corte histórico y no prueba materialización posterior.

El expediente IMM recibido es la [valoración corregida, §§5–11](../dominios/inmunologia/VALORACION_TECNICA_Y_ENCAJE_DE_OP-IMM-001_CON_EL_LENGUAJE_SV_2026_09_03.md), su [adversarial](../dominios/inmunologia/ADVERSARIAL_DE_CONTINUIDAD_Y_CONFORMIDAD_DE_LA_VALORACION_OP-IMM-001_2026_09_03.md) y su [sincronización, §§4–8](../dominios/inmunologia/INFORME_DE_SINCRONIZACION_OPERATIVA_ENTRE_LENGUAJE_SV_E_INMUNOLOGIA_OP-IMM-001_2026_09_03.md). Se recibe su reconciliación sobre `SVperitus-dataset/dominio-inmunologia@3bea6b714be3bd1330e6ca6bbbc228b0eb9c065d`; este acto no actualiza ni reconstituye ese dominio. La correspondencia `15 ↔ 44` sigue en su sede, sin una tercera familia de requisitos clínicos. §6 precisa qué afirmaciones de realización han cambiado desde aquel corte.

La recepción K1 se apoya en [N0 §§17–19](./N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md#domain-parameters-20260907), [IR 0.3 §2.4](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md#ternarizer-k1-t), sus juicios y los campos/operaciones de [ir.rs](../../rust/sv_core/src/ir.rs). La instancia `(C,j)` y el contrato de transducción se reciben con las fuentes doctrinales identificadas allí; prevalece la separación de fallo y Tri vigente en IR 0.3.

El dominio decide su contenido, células, tamaños, orden, asignación, reglas y límites. El agente recibe ese dominio y declara su cobertura, operaciones y permisos. El Lenguaje debe poder preservar y comprobar lo que su semántica necesite; el motor y la frontera materializan los enlaces y recursos de su contrato. Una referencia externa es válida como referencia identificada; no demuestra el contenido, la ejecución ni una garantía por llevar un nombre o un hash.

## 2. Unidad de contrato y enlace de la terna

La unidad de evaluación es **una operación identificada sobre una versión y un perímetro de dominio, con una representación y un soporte declarados**. No se declara suficiente un dominio entero por un resultado parcial. Cada entrega reutilizará los objetos existentes de su sede y aportará estos datos o un localizador inequívoco a ellos:

| Bloque | Contenido que debe quedar resuelto | Condición comprobable |
|---|---|---|
| Identidad y autoridad | ID/versión del contrato y dominio, corte de constitución, emisor competente, finalidad, inclusiones y exclusiones | Referentes accesibles en el expediente; el hash liga contenido, no concede autoridad. |
| Representación fuente | IDs/versiones de perfiles fuente por unidad, Gramática, IR y proyección utilizadas, bytes y orden de unidades | ES/EN convergen en identidades canónicas dentro del contrato existente; cambiar el idioma no traduce reglas ni permisos. |
| Constitución | Inventario semántico, instancias, células constituidas cuando existan, asignaciones, cadenas de entrada, reglas y sus versiones | Cada dependencia de la operación resuelve sin inferir correspondencias por tamaño, posición de lista o parecido nominal (§3). |
| Operación | ID/versión, finalidad, firma, precondiciones, alcance realizable `X_D`, resultado `Y`, regla determinista, cobertura necesaria y efectos si existen | Una consulta nombrada no basta: se puede decidir qué distinciones exige y qué resultado espera. |
| Representaciones | `F_j`, tipos, reducciones, pérdidas, interfaz transmitida e información lateral | Las aplicaciones y el criterio de igualdad tienen definición explícita; certificados relativos a la operación (§4). |
| Agente consumidor | ID/versión y constitución recibida, cobertura, capacidades y permisos aplicables | Si no interviene agente se justifica; declarar una capacidad no otorga permiso. Ninguna unión de perfiles constituye un superagente. |
| Soporte | ID/versión de realización y contrato, artefacto/configuración, dependencias, requisitos por operación y evidencia | Se distinguen lo requerido, lo ofrecido y lo probado; un dato ausente no equivale a «no aplicable» (§7). |
| Evidencia y decisión | Fuente normativa, representación disponible, prueba positiva/negativa, pérdida, límite, responsable y siguiente sede | Se emite el juicio de §5 con su alcance; no un «conforme» global. |

`F-SV/0.1-candidata` identifica este contrato documental, no un nuevo perfil fuente ni un tipo de IR. Los futuros IDs de dominio, agente y soporte los declara su sede; F no asigna aquí versiones clínicas ni eleva `PT-SV-LOCAL/0.1-candidata` a tipo. La implementación concreta del paquete —manifiesto enlazado, tipos de IR o combinación— se decidirá sobre las pérdidas observadas en F-IF y retornos IMM/CYB. Mientras esa ligadura no sea verificable en la realización, el paquete documental no admite operaciones en ella.

## 3. Recepción bloqueante de DFL-005 y K1-T

### 3.1. Identidad, mínimo y multiplicidad

Para una célula constituida, la instancia es `(C,j)`, con posición uno-basada, `1 ≤ j ≤ n_C`, `n_C=b_C²`, `b_C≥3`. Su identidad en un expediente incluye el dominio y versión que resuelven `C`. Se conserva el vector plano y ordenado; no se reconstruye una célula a partir de un inventario nominal.

El contrato de cada operación declara **qué instancias o parámetros requiere**, y el paquete debe resolver todas ellas. Este mínimo es relativo a la operación:

- Una operación que necesita una instancia no satisface su contrato con `parameters=[]` ni con un nombre sin referente. Una declaración estructural vacía todavía aceptada por el compilador no demuestra suficiencia.
- No se fija una cardinalidad universal del inventario de dominio. Una operación que declare no depender de parámetros deberá justificarlo mediante su firma y regla; un conjunto vacío de estados realizables no sirve como evidencia positiva de utilidad o ejecución.
- Cuando la operación consume un estado celular completo, se exige la constitución y asignación de todas sus posiciones y un vector válido completo. Captura fallida o no admitida no completa posiciones con U ni autoriza un estado parcial.
- Se recibe la asignación explícita `μ_C:{1,…,n_C}→P_C` de los Pilares, con el régimen declarado de repetición/compartición. No se presume inyectividad, sobreyectividad ni biyectividad. Tampoco se permite duplicar o mezclar parámetros para completar geometría.

El enlace candidato debe poder resolver: **ID semántico → instancia destinataria → captura/admisibilidad/transducción aplicables**, con sus versiones, procedencia y condiciones de selección. La dirección escrita describe la trazabilidad exigida, no impone una función uno-a-uno desde nombres a instancias. Alias, un parámetro compartido y capturas alternativas necesitan declaración propia y una regla determinista que evite selección ambigua. Dos numerales `1` pueden denotar `(C1,1)` y `(C2,1)`; dos nombres distintos pueden carecer de referente o ser alias. Ni la igualdad de conjuntos de IDs ni la igualdad de cardinalidades decide esos casos.

Si concurren varias capturas para una misma instancia, la sede competente debe constituir cuál se aplica y bajo qué condiciones. Si eso falta, F devuelve la ligadura y no autoriza «primera», «última», fusión ni elección estadística. El contrato deberá conservar también exogeneidad, interfaz y las políticas de `Domain` que la operación utilice. Sus cadenas opacas actuales no prueban interpretación.

### 3.2. Captura, admisibilidad y transducción

Para cada entrada exógena utilizada deben resolverse `W_j`, captura `φ_j`, observación `O_j` o fallo `O_j^⊥`, regla de admisibilidad `r_j` y, cuando proceda, transducción `τ_j`. Las fuentes, reglas, configuraciones y transformaciones causalmente relevantes forman parte de la identidad. Cualquier dato externo variable necesario para reproducir la operación debe recibirse capturado e identificado; una consulta a una fuente viva no se presenta como reproducción de una entrada congelada.

La futura producción mediante Ternarizer exige espacio aplicable, pertenencia, cobertura y disjunción de `B₀/B₁/Bᵤ`, función total y determinista, significado previo de las salidas e independencia del resultado celular. Deben constar bordes y regla de admisibilidad que delimita la entrada efectiva. Tres nombres distintos no prueban partición; tres nombres iguales no prueban por sí solos su invalidez material.

`Bottom`, `NotAdmitted`, regla/configuración ausente, insuficiencia representacional y fallo técnico no se convierten en Tri. Una observación positivamente admitida (`Ok` o `Degraded`) sólo podría producir U mediante pertenencia demostrada a `B_U` y la ruta productiva legítima. **En el corte recibido esa ruta permanece no habilitada para los tres valores**, conforme a K1-T. Los literales Tri vigentes conservan su legitimidad; no acreditan haber transducido una observación ni ejecutado Q0.

## 4. Suficiencia, pérdida e interfaz

Se aplican las definiciones de FFL-E: `Q:X_D→Y`, representaciones `F_j:X_D→R_j` y reducciones tipadas con `F_(j+1)=r_j∘F_j`. El subíndice pertenece a la cadena concreta, no a una escala universal ni a los niveles N0–N4 de la IR.

La suficiencia exacta requiere recuperación explícita `Q=q_j∘F_j` o evidencia equivalente para todo el alcance que se afirme. Un corpus finito sólo demuestra sus casos, salvo que enumere exhaustivamente el espacio finito declarado. «Sin contraejemplos encontrados» no equivale a recuperación demostrada. Un certificado de frontera identifica el mayor nivel acreditado, recuperación y, salvo último nivel, un testigo realizable en el siguiente. Los niveles admitidos se derivan del certificado y las reducciones; no son una lista ampliable libremente.

La pérdida se acredita con `x,y∈X_D`, `F_(j+1)(x)=F_(j+1)(y)` y `Q(x)≠Q(y)`. Si no se acredita realizabilidad, el par queda como sonda pendiente. La ausencia de certificado o de representación verificable impide afirmar suficiencia, pero no constituye automáticamente un teorema de imposibilidad.

Si la interfaz entrega `H=ψ∘F_j`, el juicio recae sobre `H`. Una recuperación que necesita información lateral `S` se declara `Q=q(H,S)`, con procedencia, versión y disponibilidad de S; requiere un nuevo certificado sobre esa entrada conjunta. No se atribuye la recuperación a H sola. La identidad de un fichero, la observación derivada, su interpretación y el informe conservan tipos y relaciones distintos. El codominio terminal de la célula no se identifica automáticamente con Tri.

### 4.1. Control finito del criterio, sin contenido de dominio clínico

Sea el espacio sintético **externo a la geometría celular** `X={(a,a),(a,b),(b,a),(b,b)}`. Cada estado contiene dos etiquetas ordenadas; los cuatro son realizables por esta definición. `F₀` conserva el par; `F₁` cuenta sus etiquetas `a`; `Q_izquierda` devuelve la primera etiqueta y `Q_cuenta` devuelve el número de `a`.

| Estado | F₁ | Q_izquierda | Q_cuenta |
|---|---:|---|---:|
| `(a,a)` | 2 | a | 2 |
| `(a,b)` | 1 | a | 1 |
| `(b,a)` | 1 | b | 1 |
| `(b,b)` | 0 | b | 0 |

F₁ pierde información y, sin embargo, conserva exactamente Q_cuenta mediante `q(t)=t`. El par `(a,b)/(b,a)` refuta recuperar Q_izquierda desde F₁. F₀ sí la conserva mediante selección de la primera etiqueta. Añadir esa etiqueta como S restaura Q_izquierda desde `(F₁,S)`, sin hacer suficiente F₁ sola. El cálculo exhaustivo de estos cuatro estados es un control del contrato; no es ejecución SV, prueba de una célula de dos posiciones, testigo IMM ni campaña F-IF realizada.

## 5. Juicio por operación y sede de resolución

El expediente distinguirá estos resultados documentales. No son nuevas variantes IR ni códigos del compilador:

| Juicio | Evidencia mínima | Consecuencia |
|---|---|---|
| Suficiencia demostrada en alcance declarado | Constitución y dependencias resueltas; recuperación y prueba válida sobre ese alcance | Permite afirmar sólo esa suficiencia. La ejecución requiere además realización, permisos y soporte acreditados. |
| Pérdida demostrada para Q | Testigo realizable con igualdad de representación y desigualdad de resultado | Esa representación no admite ejecución exacta de Q. Corregir la transmisión, aportar S con nuevo certificado o excluir Q. |
| Suficiencia no acreditada | Falta localizable: constitución, ligadura, definición, certificado, realizabilidad o representación comprobable | Bloquea admitir Q en ese alcance; no equivale a U ni a pérdida demostrada. |
| Fuera de perímetro | Exclusión motivada por la constitución o contrato de operación | Conservar exclusión; no ampliar el dominio desde un adaptador o caso sintético. |

Cada insuficiencia debe indicar el objeto afectado, versión, operación, representación disponible, distinción necesaria, evidencia y responsable. Se conserva `RepresentationInsufficientForOperation` como clase semántica prevista por FFL-E, sin asignarle un número o afirmar emisión actual. El fallo de transporte o ejecución tiene su registro técnico separado, con fase e identidad; no se usa para justificar una salida semántica alternativa. Toda ejecución válida con identidad completa idéntica debe conservar los bytes de la salida canónica que su contrato defina; esa salida no se confunde con `equivalence_json`.

| Cuestión devuelta | Sede competente y puerta |
|---|---|
| Significado, realizabilidad, cobertura o regla clínica no constituida | IMM conforme a transición §9; sin inventar contenido ni abrir todo Inmunología. |
| Información perdida por interfaz, perfil externo o motor | Reparación explícita y nuevo testigo/certificado. Si afecta una obligación nuclear, no basta archivarla fuera de IR. |
| Obligación semántica que la representación del Lenguaje no expresa | F-IF y retornos IMM/CYB localizan la necesidad; decisión de representación y versión antes de ofrecer la operación dependiente. No ocultarla en cadenas. |
| Identidad, nombres, orden o procedencia transversales | K2 recibe su cierre, pero una ligadura imprescindible bloquea ya la operación en F. |
| Álgebra o producción observación→Tri | F recibe representación; realización en puerta algebraica con K1-T previo para producción. |
| Garantía material | Contrato operacional y R2/R3/R4 según su dependencia; excluir el alcance que la necesite hasta acreditarla. |

## 6. Recepción del caso director y actualización del encaje

La matriz histórica de la valoración conserva las quince entradas G10, las 44 solicitudes LSV y sus once ampliaciones sin padre exclusivo. La tabla siguiente localiza su recepción en F; no afirma 59 capacidades ni 59 requisitos independientes ni que hayan sido ejecutados:

| Requisito G10 `REQ-IMM-SV-…` | Recepción F | Pendiente que conserva |
|---|---|---|
| 001 · estado por parámetro | §§2–3, ligadura e identidad | Resultado integrado por parámetro y transducción productiva. |
| 002 · composición ordenada | §§2–4, referencias y pérdida | No deducir composición de agentes del ensamblaje ES/EN. |
| 003 · supervisión humana | §§2, 5 y 7, autoridad y efecto | Flujo OP integrado y evidencia material. |
| 004 · Frame/arquitectura | §§2 y 6.1 | J-H0 no prueba toda derivación ni causalidad de Frame. |
| 005 · procedencia | §§2–4 y 7 | Tupla completa y comprobación de contenido/referentes. |
| 006 · causas de U | §§3.2 y 5 | Causa separada del valor y resultado OP tipado. |
| 007 · fallo técnico | §§3.2, 5 y 7 | Registro integrado separado; durabilidad cuando aplique. |
| 008 · adjudicación | §§2, 5 y 7 | Acto nuevo atribuible; no saneamiento retrospectivo. |
| 009 · veto previo no compensable | §§2 y 5, regla y precondición de Q | Comprobar orden/veto antes del resumen; no inferirlo de R1 o criticidad. |
| 010 · productor de criticidad | §§5 y 6.1 | DFL-006; no fabricar CriticalityResult. |
| 011 · seis agrupaciones | §3.1 y este apartado | `U_NO_DECIDIDO` conservado; no equivalencia agrupación–célula. |
| 012 · reversibilidad del resumen | §4, operación y testigo realizable | Certificado por consulta; portar un resumen no prueba reversibilidad. |
| 013 · configuración cerrada | §§2–3 | Manifiesto del dominio/motor y validación de versión/contenido. |
| 014 · cuatro salidas exclusivas | §§2, 3.2 y 5 | Catálogo del dominio y salida OP no ejecutables hoy. |
| 015 · igualdad literal | §§2, 4 y 5 | Objeto canónico y tupla causal completos; paridad no basta. |

Las ampliaciones `REQ-IMM-LSV-010/020/021` conservan R2; `023/024`, la responsabilidad de frontera (§7); `030`, DICOM no aplicable al corte OP; `032/036`, evidencia y entorno sin conformidad de producto; `042/043/044`, decisiones y secuencia (§§5 y 9). Para las demás correspondencias se reutiliza valoración §5.1, sin eliminar enlaces múltiples.

Las cardinalidades `(6,1,3,2,6,9)` y los 27 parámetros recibidos son agrupaciones externas. `M-MODIFIER-001` sigue candidata a posible célula, pendiente de constitución; el contrato no asigna células, no completa nueve ni interpreta «seis frames» como seis Frames IR válidos. IF-IMM-03 puede probar un anexo sintético sin convertir DICOM en requisito de OP-IMM-001. Q0 v0 conserva su finalidad informativa acotada, sin salida asistencial habilitada.

### 6.1. Sucesión expresa de afirmaciones de realización

| Antecedente de valoración | Estado recibido en main@4d5f93b | Límite vigente |
|---|---|---|
| Compilador/comparador Python (§§6.4, 6.10 y 7.9) | Retirado por RETP-082. La custodia de implementación es Rust, subordinada a la DSL. | Esos pasos históricos no se ejecutan como condición de F. Los auxiliares de prueba no constituyen SV. |
| Unicidad de Codomain y totalidad de OutputSemantics | K1, RETP-077/079/080; juicios representables cerrados. | No prueban selección algebraica de salida, resultado clínico ni serializador completo. |
| Referencia arquitectónica de Horizon | J-H0, RETP-081; tipos únicos J-H1, RETP-084. | No completa trayectorias ni todo contrato de Domain/Agent. |
| Domain.parameters/cadenas de entrada | J-D0, RETP-086; unicidad nominal y listas conservadas. | DFL-005: mínimo por operación, identidad de instancia y ligaduras de §3 pendientes de realización. |
| Ternarizer declarado | K1-T, RETP-088: conservación nominal y producción no habilitada. | Nombrar mapping no ejecuta transducción en el núcleo ni en un host. |
| Proyección y oráculos | RETP-078/080/087/088: controles reparados y 100 casos en el corte recibido. | No son serialización clínica, certificado de suficiencia ni emisión diagnóstica estructurada completa. |
| Criticidad, cobertura de agente y composición | DFL-006, DFL-005 y K2 conservan sus objetos. | Frame.criticalities sigue vacío en la superficie; compose no compone Agent/Domain. |

## 7. Obligaciones tecnológicas y evidencia aprovechada

Se recibe la [matriz PT01–PT14 del laboratorio](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/020-matriz-obligaciones.csv). Su numeración se conserva. La [transición §15.2](../dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md#152-qué-se-reutiliza-y-cuándo) identifica los registros 012/016/018 y CS01–CS09 con sus límites.

| Obligación | Qué exige la operación y qué debe conservar el Lenguaje | Qué entrega ya el laboratorio / brecha material |
|---|---|---|
| PT01/PT03 | Enlace de versiones, constitución, cobertura y permisos (§§2–3). | Manifiestos 016/018 como experiencia; CS03 espera contrato constituido. No existe ligadura de extremo a extremo acreditada. |
| PT08/PT09 | Actor frente al que exige aislamiento, recursos necesarios y condiciones de fallo; restricciones identificadas, sin inventar presupuestos. | 016/018 y cuotas del prototipo aportan ataques/medidas acotados. No demuestran contención de host comprometido ni SLO universales. |
| PT10 | Objetos que Q necesita conservar, vigencia, procedencia y recuperación, diferenciando vista y autoridad. | 012: recuperación y efecto SQLite sintéticos. Persistencia autoritativa, retroceso, clonación y recuperación gobernada conservan R2. |
| PT11/PT12 | Dependencias de construcción/ejecución separadas; compatibilidad de interfaces y comportamiento conjunto ante fallo. | Inventarios y FFI/WASM de 018; no prueba conjunta del contrato F ni autonomía integral de proveedor. Ensamblar no suma permisos o garantías. |
| PT02/PT04/PT13/PT14 | Fuentes y perfiles, integridad de representaciones, diagnóstico, custodia semántica, cambios y destinos aplicables. | 018 conserva 79 programas EN monofuente de compilación/proyección. La evidencia K1 del Lenguaje conserva sus propios cortes ES/EN/ensamblaje; no amplía retrospectivamente 018. |
| PT05/PT06/PT07, si Q incluye efectos, repetición o cancelación | Procedencia del permiso, distinción request_id/ExerciseRef, reentrega/nuevo ejercicio, cancelación/revocación y su autoridad. | CS01/02/04/05 especificados y ensayos sintéticos previos; enlace R1 aún pendiente. Timeout no constituye cancelación, ni reentrega otro permiso. |

Cada contrato particular debe declarar aplicabilidad o exclusión justificada. Una garantía imprescindible no acreditada bloquea ofrecer esa operación, aunque se haya asignado a R2/R3/R4. F no elige ABI, proceso, proveedor o umbrales. El contrato operacional se cierra en fila 13 con los ensayos pertinentes; DFL-009 se valora en fila 9, después del primer universo CYB, incluyendo entonces Cloudflare/Workers u otros según necesidad.

## 8. Adversarial de la candidata

Esta revisión comprueba las consecuencias del contrato y su cotejo con la representación existente. Salvo el cálculo finito de §4.1, las filas son argumentos y exigencias de prueba posterior, no nuevos rechazos ejecutados por el compilador:

| Ataque | Respuesta exigida y localizador | Estado de evidencia |
|---|---|---|
| Dos parámetros distintos permutados producen el mismo resumen | §4.1: mismo F₁, distinta Q_izquierda; conservar posiciones o declarar pérdida. | Contraejemplo exhaustivo en espacio sintético de cuatro estados. |
| Rechazar toda representación Lossy para toda operación | §4.1: Q_cuenta sí se recupera exactamente. | Control positivo sobre los mismos cuatro estados. |
| Reconstrucción mediante dato lateral oculto | §4: declarar `(H,S)` y nuevo certificado. | §4.1 comprueba el caso con S explícita; H sola sigue insuficiente. |
| Nombres o cardinalidades coinciden, pero no hay ligadura `(C,j)` | §3.1: suficiencia no acreditada, con referente faltante. | Carencia material localizada en Domain/CaptureSpec; DFL-005. |
| Repetir numeral en células distintas; o dos alias para la misma | §3.1: resolver identidad y régimen, sin imponer unicidad numérica universal. | Criterio candidato; representación completa pendiente. |
| Declarar conjunto realizable vacío o corpus parcial como prueba total | §§3.1 y 4: exigir positivo realizable y acotar el alcance. | Guarda contractual; no se certifica Q0. |
| Tres etiquetas de partición autorizan producir U | §3.2: K1-T conserva producción no habilitada; fallo no entra en Tri. | Exclusión material ya recibida, PR #74; no nueva ejecución aquí. |
| Igualdad nativo/WASM acredita semántica aunque ambos omitan datos | §§4–5: primero oráculo normativo y suficiencia, después paridad de observables pertinentes. | Obligación de campaña; paridad sola rechazada como argumento. |
| Fichero disponible equivale a observación, informe o interpretación | §4 y F-IF: conservar relación derivada y regla consumidora. | Pendiente de los seis testigos de interfaz. |
| R1 o un ensayo SQLite legitima efecto durable y reintento | §§5 y 7: permiso, efecto y continuidad conservan condiciones propias. | Alcance histórico limitado; enlace y R2 pendientes. |
| Las seis familias IF amplían OP-IMM-001 | §6: conservar constitución congelada y exclusiones. | Rechazo documental de la ampliación automática. |

**Dictamen:** candidata coherente para someter a F-IF; no acredita implementación de DFL-005, E107/J1.5 ni suficiencia clínica. Las obligaciones están localizadas, con criterio de comprobación o insuficiencia expresa. La falsación por dominios permanece pendiente.

## 9. Paquete que debe contrastar F-IF

El siguiente acto es **fila 5/F-IF**, dentro de F. Recibe esta identidad documental y el corte integrado. Debe constituir paquetes sintéticos externos y operaciones consumidoras explícitas, con positivo, negativo, realizabilidad, cadena, recuperación o testigo de pérdida. Usará las seis familias ya fijadas, sin convertir sus campos en nuevas producciones SV:

| Testigo | Distinción que debe conservar y sonda negativa a preparar |
|---|---|
| IF-IMM-01 | Petición, muestra, observación, unidad, rango/criterio, estado, informe y corrección. Perder unidad o vínculo de corrección no puede pasar como mismo dato suficiente para una Q que los requiera. |
| IF-IMM-02 | Fichero instrumental, panel/configuración, controles, observación derivada, informe e interpretación. Cambiar configuración con fichero igual debe seguir distinguible cuando Q dependa de ella. |
| IF-IMM-03 | Muestra, lugar/procedimiento, procesamiento, informe, anexos e imagen cuando exista. Disponer del hash de un anexo no acredita su contenido ni interpretarlo. |
| IF-IMM-04 | Intención, orden, pauta, administración efectiva, modificación/suspensión y resultado. Una orden no se transforma en constancia de administración. |
| IF-IMM-05 | Episodio, encuentro, ubicación, servicio, inicio/fin y variación organizativa. Reutilizar ubicación no identifica episodio o encuentro. |
| IF-IMM-06 | Orden causal, validez temporal, estados, enmiendas, procedencia y relación entre episodios. Reordenar por recepción o sobrescribir un antecedente no conserva necesariamente Q. |

Cada paquete contendrá los datos de §2 y los de transición §15.1 (`W_j`, `φ_j`, `O_j/O_j^⊥`, `r_j`, `τ_j`, Q y oráculo). Un componente sin uso en la operación se justificará; una transducción conceptual no se ofrecerá como ruta SV ejecutable. Si la prueba necesita una decisión constitutiva real de IMM, se devuelve esa pregunta conforme a §9 de la transición. No se requiere hospital ni datos reales para este contraste técnico.

F-IF entregará una matriz por operación y nivel, con evidencia conservada, pérdida, insuficiencia pendiente, propietario y cambio propuesto. Una corrección cambiará identidad y revalidará sólo lo afectado y las puertas exigibles. El retorno G/H a IMM sigue en fila 6; luego Lenguaje e I/J, sin concurrencia. F sólo quedará contrastado en esos alcances cuando se reciban sus resultados.

## 10. Verificación y relevo de esta entrega

La revisión documental coteja §§2–9 con las piezas de §1, la recepción íntegra de los 15 identificadores G10 y las once ampliaciones LSV por referencia, los seis testigos existentes y PT01–PT14. Se comprueban el cálculo finito de §4.1, enlaces locales, correspondencia RETP CSV/Markdown y sucesión del relevo. No se atribuye al cotejo una nueva lectura editorial del DOI del Documento III.

La candidata conserva los blobs de código, gramática, IR, perfiles, corpus, esperados, workflows y Playground de `main@4d5f93b`. Se exige Conformidad SVP en su nueva cabeza. La paridad nativo/WASI/navegador de [PR #74](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/74/checks) acredita aquel corte ejecutable y no se contabiliza como nueva prueba del contrato F. Una realización futura deberá demostrar conformidad con la DSL y paridad pertinente sobre sus propios observables y candidata, incluidos los negativos; no basta compilar código Rust.

**Salida:** contrato F formulado y delimitado, efectivo como candidata documental al integrar el expediente verificado. **Siguiente:** F-IF. Permanecen las condiciones de DFL-001/005/006, K1-T, K2 y puertas algebraica/materiales; este contrato no declara el núcleo consolidado ni OP-IMM-001 ejecutable.


**Recepción posterior RETP-090:** [F-IF/1](./F_IF_SEIS_TESTIGOS_SINTETICOS_Y_RELEVO_G_H_2026_09_07.md) ejecuta las seis familias en espacios documentales sintéticos completos, con matriz de pérdida por operación y límites de realización. Su integración verificada da paso a G/H conforme a transición §30. La campaña no convierte este contrato candidato en semántica/IR ejecutable ni acredita fidelidad clínica; esos contrastes siguen su secuencia.
