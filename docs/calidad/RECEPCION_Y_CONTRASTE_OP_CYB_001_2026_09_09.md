# Recepción y contraste acotado de OP-CYB-001

**Mandato:** continuación autorizada por Juan Antonio Lloret Egea el 09/09/2026, tras la lectura del relevo aprobado. **Sede:** Lenguaje, retorno de I/J y comienzo de la fila 9. Este expediente no constituye contenido de Ciberseguridad ni declara completado su contraste técnico por recibir documentos.

## 1. Cortes y recepción

| Objeto | Corte exacto |
|---|---|
| Lenguaje recibido | `66967a80a40f4e2781ef983bd725690db54c25c5` |
| Árbol del Lenguaje | `6de7e582b35b7e55d6bf0e5cbe8c150124436301` |
| Relevo CYB aprobado | `169af16d05ffc954454bb5528ec106e5752b7016` |
| Árbol del relevo | `9790596388554d87039c6ca450d4f9dbc088888c` |
| Constitución material recibida | `b3aa3f01c825bf6440e3f04aadc87cc7c7ee6d89` |

El emisor es `juantoniolloretegea/SVperitus-dataset`, rama `dominio-ciberseguridad-inteligente`, bajo `dominios/ciberseguridad-inteligente/dominio-04-09-26/`. Se recibe su [acta aprobada](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/169af16d05ffc954454bb5528ec106e5752b7016/dominios/ciberseguridad-inteligente/dominio-04-09-26/universos/OP-CYB-001/ACTA_CONTINUIDAD_Y_RELEVO_OP_CYB_001_AL_LENGUAJE_SV_2026_09_09.md), en particular §§5–10. Los originales científicos y curriculares conservan su aprobación; esta campaña no repite su constitución.

Se han leído `AGENTS.md`, Pilares RETP-073, acta de perfiles RETP-075, acta completa de transición con su tabla de catorce filas, RETP-105 y su sucesión de deuda, contrato LIG/0.1 y expediente CYB aprobado. LIG/0.1 y H06/H07 estructurales están integrados. Las expresiones históricas «candidata» de sus contratos se leen con la sucesión RETP-105.

La recepción comprende 32 definiciones, 17 controles, 18 elementos de estado de partida y nueve clases documentales de relación. El libro `catalogo-profesional/CATALOGO_CURRICULAR_CIBERSEGURIDAD_v0.10.xlsx` tiene 672096 bytes, 85 hojas y 90 tablas; SHA-256 `76166a77373e7df2c23bfe5a6fe17bf3cc305305cce629e43da720d656d0bd45`. No se modifica ni se deriva de su recuento una arquitectura.

Los once JSON en `tests/retorno_cyb/originales/` son extracciones literales, sin reserialización, de dos anexos del emisor:

| Anexo | SHA-256 |
|---|---|
| `ANEXO_PRESION_REGULADA_OP_CYB_001_v0.3.zip` | `7be66eb290dddb3ab2b9b1369b62a9cf0904cd93f8bf924e40fc15ea6b340c96` |
| `ANEXO_CONTINUIDAD_OP_CYB_001_v0.4.zip` | `67b14579d4dcbfae597414528ba2e3d61161ddc7a34d0e1219c0f046ceb39402` |

Las rutas, miembros, tamaños y huellas están en `tests/retorno_cyb/fuentes.json`. No se incorporan al Lenguaje los auxiliares Python de los anexos. Sus reglas documentales se conservan como antecedente identificado; una transcripción externa de prueba no será una segunda realización semántica SV.

## 2. Pregunta y alcance fijados antes de ejecutar

**Pregunta:** ¿el contrato LIG/0.1 integrado conserva y permite recuperar, con referencias exactas, los datos que consumen las reglas documentales de estos testigos, sin atribuir a una reducción técnica las distinciones eliminadas?

Se reutiliza el portador sintético de `tests/row7_bindings/`, ya constituido para pruebas de LIG. D, AG, C1, sus numerales y capturas son exclusivamente objetos del banco. **No representan el dominio, el agente, las células ni los sensores de OP-CYB-001.** Los usos nuevos no llevan destino celular ni ternarizador. El requisito estructural actual de LIG de recibir un agente con grafo se satisface sólo en el portador; la asignación CYB sigue sin constituirse. Esto permite probar transporte y recuperación; no permite ofrecer una ligadura productiva de sus 32 parámetros.

No se ejecuta `query`, `Trajectory`, captura, admisión productiva, transducción ni un Frame CYB. El uso posterior de los datos lo realiza un observador documental externo, delimitado por los casos y reglas entregados. Sus resultados, incluidas cadenas como `NO_ADMISION`, describen ese procedimiento; no añaden variantes a `Tri` ni diagnósticos del núcleo.

### 2.1. Entradas, recuperadores y resultados esperados

| Familia | Datos transportados | Uso y resultado exigido |
|---|---|---|
| PR-001…046 | `parametro` y `entrada`, sin `esperado`, identificador del caso ni supuesto | Recuperar esos datos y aplicar externamente la regla documental de `ensayar.py` del anexo 0.3; comparar con el esperado original |
| CT-01…32 | `clase` y `datos`, sin `esperado`, identificador, título ni razón | Recuperar y aplicar externamente la regla documental de `contrastar.py` del anexo 0.4; comparar con el esperado original |
| SI01…10 | Expedientes `a` y `b`; reducción H = `technical` | F0 conserva el expediente; H pierde el campo `diferencia`; HS recupera ese campo desde S declarada. No se confunde el campo documental con un veredicto completo de legitimidad |
| PC-01…08 | Casos CT referidos; H = `proyeccion_tecnica_comun` | F0 permite el uso anterior; H es idéntica para las dos respuestas distintas; HS recupera los datos completos desde S declarada |

F0 es el documento completo pertinente a la operación. H es la reducción **elegida por el dominio para el testigo**, no una proyección que la IR imponga universalmente. S contiene el documento completo pertinente, disponible y versionado; no se pretende minimalidad. El identificador externo de fila permite cotejar la campaña, pero nunca entra en el contrato H, sus artefactos ni la operación de recuperación. Los esperados permanecen fuera del emisor y del consumidor: sólo el comprobador final los consulta.

Los JSON originales conservan sus bytes. La entrada derivada se codifica compactamente manteniendo orden de miembros, tipos, listas y literales; ambos lectores documentales la construyen por separado. Las magnitudes exactas del anexo 0.3 siguen como cadenas canónicas; los intervalos se comparan con enteros arbitrarios, sin paso por coma flotante. El rango restringido del lector JSON documental no limita `Nat` ni constituye el tipo universal de magnitudes de SV.

**Esperados previos:** 78 consumos documentales completos; 18 pares reducidos, con dos estados por par; F0 y HS deben preservar la distinción consumida, y H debe colisionar en el contrato completo. Son 186 transportes: 78 casos y 108 filas de pares (18 × 2 × 3). Los ocho pares PC no instancian los otros treinta parámetros; sólo P02/P06 son comunes. Los diez SI conservan su reducción propia de cuatro elementos. No se suman como estados celulares completos.

El banco nativo repetirá esos transportes en EN, ES y dos órdenes del ensamblaje multifuente existente. La prueba compara conservación de cargas entre perfiles; las identidades de programa pueden y deben diferir según sus fuentes. No reclama nueva paridad WASI/navegador; sus campañas anteriores conservan su corte. No hay cambio funcional del núcleo que promover mediante esta campaña.

### 2.2. Refutadores y controles

1. Alteración de bytes sin actualizar su referencia: `ArtifactIntegrity`, llegando a esa guarda tras recalcular sólo la expectativa del contrato.
2. Versión equivocada o referente ausente: rechazo de referencia exacta o referencia ausente; sin sustitución automática.
3. H usada con S no declarada, o HS sin S: rechazo `SideInformation`.
4. Alteración simétrica de huellas de filas H: rechazo del recálculo externo de cada contrato, aunque sean iguales entre sí.
5. Contaminación del contrato con un identificador de estado: rechazo de la construcción esperada completa; no basta comparar sólo las cargas.
6. Salida con claves duplicadas, sustitutos Unicode aislados, decimales numéricos o enteros fuera del subconjunto documental: rechazo antes de interpretar. Toda entrada JSON del comprobador usa el mismo lector estricto.
7. Datos alterados y huellas recalculadas coherentemente: rechazo contra el testigo fijado. Una nueva huella no autoriza cambiar los esperados.
8. Reglas documentales: conservar los 22 ataques dirigidos de los dos anexos como sensibilidad de la transcripción externa. No son mutaciones de `sv_core` ni puntuación general.

La prueba nativa compara tipo y sujeto del error y vuelve a admitir el control intacto. El observador externo reconstruye el contrato esperado y recalcula su codificación LIG/0.1; la igualdad de dos huellas no sustituye ese recálculo. Ninguna salida del emisor fija su propio esperado.

## 3. Correspondencia completa y último nivel que se pretende contrastar

La tabla recibe REQ-CYB-001…009 y RS01…12; sus solapamientos no crean 21 capacidades distintas. `RECUPERACION_DOCUMENTAL` es un objetivo de prueba en esta versión previa, no un resultado ejecutado.

| RS | REQ relacionados | Referentes y prueba pertinente | Capacidad posterior que no acredita el transporte; sede |
|---|---|---|---|
| RS01 | 001 | P25; PR-001…004; SI01 y atribución de sus expedientes | Identidad real de sesión/principal y responsabilidad; dominio/institución y soporte |
| RS02 | 007 | P26/P27; PR-005…015; SI02/SI03 | Competencia, delegación y autorización material; contrato de dominio y frontera |
| RS03 | 004, 008 | P28–30; intervalos exactos en PR; SI04–06 | Semántica temporal operacional, vigencia y observación del efecto; DFL-003/004 y soporte |
| RS04 | 001 | SI: original, custodia, destinatario y vista | Control efectivo de acceso/difusión; frontera, R3/R4 |
| RS05 | 007 | P32; casos PR de plan y permiso; SI | Regla profesional de excepción, consecuencias y decisión humana; dominio |
| RS06 | 001, 003, 005, 008 | No admisión, no aplicabilidad, U y fallo en PR; CT-08/09/21/24 | Capturadores y productor observación→Tri; K1-T. No inferir admisión de la mera integridad |
| RS07 | 002, 006 | SI de delegación y vista; CT-19…28 | Exhaustividad real, permisos, flujos y conjunto vacío; constitución de cobertura y frontera |
| RS08 | 007, 009 | Resultado técnico separado de campos de legitimidad SI; respuestas CT | Consejo, consecuencias, Frame y CQ1–CQ6; DFL-003/006 y arquitectura de dominio |
| RS09 | 001, 004, 006, 008 | EP01–04; C17; CT-01…05; PC-01 | Estado observado no prueba génesis ni referencia aprobada; historial real y continuidad, DFL-004/R2 |
| RS10 | 004, 009 | EP10/11; CT-06…11; PC-02/03 | Interpretación operacional de relaciones y causalidad; Lenguaje, DFL-004/006. Referencia recíproca no equivale a ciclo de precedencia |
| RS11 | 007, 009 | EP07/08/10; CT-12…18; PC-04/05 | Custodia, designación y deber no se transfieren por referencia; constitución de obligación y autoridad, frontera |
| RS12 | 002, 004, 005, 006, 008 | EP05/06/09/12–14/18; CT-19…32; PC-06…08 | Conciliación por identidad, cobertura real y reevaluación selectiva; Lenguaje más captura/materialidad externa |

REQ-002 exige además probar dos instancias del portador con evidencias distintas de una misma definición sin unificar sus usos. REQ-005 exige la sensibilidad del criterio externo; REQ-008 exige comparación exacta de los intervalos. Ninguna queda absorbida por tener referencias en la tabla.

## 4. Dictamen y continuidad

En la confirmación que fija este contrato, la campaña está **PENDIENTE_DE_EJECUCION**. Sus resultados se incorporarán por sucesión, identificando confirmación, árbol, comandos, entorno, salidas y huellas. No se rellenan resultados anticipados.

El dictamen distinguirá pérdida de H demostrada en el alcance del par, suficiencia documental de F0/HS y contraste no concluyente respecto de una operación SV productiva. No declarará que todo OP-CYB-001 es representable o imposible por una campaña finita. Una carencia de uso operacional requerirá definir su consumidor y contrato; una necesidad celular concreta vuelve al dominio sin fabricar arquitectura.

DFL-009 tiene ahora su oportunidad de evaluación en fila 9 (servicio nativo, identidad, aislamiento, recursos, fallos y coste), pero no desplaza este contraste ni selecciona plataforma. Q0 IMM, sus doce SP, K1-T, DFL-003/004/006, DFL-001 general y R2/R3/R4 conservan sus límites. DFL-011 sigue siendo el español; DFL-012 el nombre externo `cell_ref`; DFL-013 la diversidad independiente ausente. Los 17 auxiliares Python y el observador externo Node continúan pendientes según RETP-105. Esta campaña no los incorpora a una entrega productiva ni acredita su retirada global.

Ciberseguridad permanece en pausa controlada. El siguiente objeto del Lenguaje será la decisión sobre los consumidores operacionales que hagan falta tras este contraste, con dato, regla, diagnóstico, efecto y sede. No se abre otro universo ni se solicita de nuevo la aprobación ya otorgada.

## 5. Resultado ejecutado y dictamen receptor RETP-106

Esta sucesión sustituye únicamente el estado pendiente de ejecución del §4. Las expectativas se publicaron primero en `1a668710e410c63b57426c46617c273553dc5566`, árbol `554efbddaa338e7ee95a0cde6f7c6ec7f909475a`, idéntico al corte local previo `840c8625…`. La implementación de prueba se publicó después en **`219d6a374c38d1a14a7e191592032f70f89ee402`**, árbol **`6997c987d51a9ef5d06b334a6f2dd04bd00e624f`**, [PR #87](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/87). La cola documental posterior conserva código, originales y esperados de ese árbol.

**Precisión de correspondencia:** en la tabla previa, RS02 citaba PR-005…015: la porción P26/P27 termina en PR-014; PR-015 corresponde a P28/RS03. SI03 es el antecedente temporal de RS03; SI06 presiona la delegación de RS02. Las referencias exactas de cada SI se conservan en `originales/v03/requisitos_lenguaje.json`, sin renumeración. Esta precisión no cambia entradas, reglas, esperados ni ejecución. Los 46 PR ensayan P25…P32; la campaña no acredita evaluación de las 32 definiciones ni repite los 64 casos de la revisión 0.2.

### 5.1. Resultados observados

| Comprobación | Resultado y límite |
|---|---|
| Originales y expectativas | 11/11 JSON literales verificados contra el manifiesto fijado |
| Pruebas Rust de recepción | 11/11: seis pruebas receptoras y cinco pruebas reutilizadas del lector documental |
| Transporte por cuatro entradas | 186 por EN, ES y ambos órdenes de ensamblaje: 744 comprobaciones de conservación, además de los controles de pares |
| Rechazos de LIG | Seis ataques por cuatro entradas, con tipo/sujeto exactos y control reparado admitido: 24/24 |
| Instancias independientes | Dos instancias sintéticas de la misma definición preservan evidencias distintas; no constituye instancias paramétricas CYB |
| Recálculo externo | 186/186 contratos completos reconstruidos y huellas LIG recalculadas; informe EN |
| Uso externo de casos completos | 78/78 respuestas coinciden con los esperados originales |
| Pares | 18/18: F0 conserva 36 respuestas; H colisiona en 18 pares completos; HS recupera las 36 respuestas con S explícita |
| Sensibilidad de reglas externas | 22/22 mutantes dirigidos detectados; transcripción de reglas del dominio, no mutación del núcleo |
| Sensibilidad del observador | 12/12 ataques: ocho a la API y cuatro a bytes a través del lector; además, dos ataques a la CLI real repetidos localmente |
| Repetición | Dos ejecuciones en Actions y una local del binario descargado producen bytes idénticos; dos ejecuciones del observador en Node 22.23.2 y 24.19.0 producen el mismo informe |
| Paquete Rust aislado | 359/359 pruebas, sin Python/Node en el contenedor, sin red y Cargo offline. La nueva sonda nativa utiliza `sha256sum` de GNU Coreutils; no es una dependencia productiva añadida a `sv_core` |

La campaña nueva es nativa. El flujo de WASI/navegador conserva el corpus y las sondas anteriores; su resultado verde **no se atribuye a los nuevos 186 transportes CYB**. La sonda no se distribuye como capacidad productiva de esos destinos.

### 5.2. Evidencia y reproducción

| Flujo de la cabeza material | Ejecución conforme |
|---|---|
| Retorno CYB documental | [34342683917](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34342683917) |
| R0 Rust | [34342683914](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34342683914) |
| Conformidad SVP | [34342683846](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34342683846) |
| R0-8 nativo | [34342683918](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34342683918) |
| Paridad del corpus anterior | [34342683797](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34342683797) |
| Paquete sin intérpretes | [34342683882](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34342683882) |

El flujo receptor comprueba la cabeza exacta, Rust 1.98.0, Cargo 1.98.0, Ubuntu 24.04.4, imagen `20260831.293.1`, Node 22.23.2 y GNU Coreutils 9.4. La repetición local usa el **mismo binario descargado**, no una segunda compilación ni una segunda realización SV.

El artefacto [10100396197](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34342683917/artifacts/10100396197), `retorno-cyb-documental`, fue descargado y verificado: 560171 bytes, ZIP SHA-256 `980a251d81fb033690822f6872211eb316137764ae2534358c50897654c76dd4`. Una primera URL temporal devolvió HTTP 403; una nueva descarga completó la custodia y la comprobación. No se reutiliza ese fallo técnico como resultado del dominio.

| Objeto | SHA-256 |
|---|---|
| Emisor nativo | `b02cdf1c87de1b54354fa5851a95f94cb641bbece8bf2f9c9439bffc4736198e` |
| Transporte y repeticiones | `0b9d0d69636490df56913e684ac17f8e306bb84775eafec4d032068eae9214c0` |
| Informe externo | `b84d04b007fd2212fb29db3004609b15787fdedd794c37ec29640675ff5cd357` |
| Paquete de fuentes declarado por el flujo aislado | `ab7df6856744ed684553ef9aa27f5a9d81600a6b8b94bcc4a0f19412c752a76b` |

El último hash procede del registro del flujo; no se confunde con el ZIP receptor descargado. El paquete comprende 242 archivos Git y dos de inventario. La custodia de Actions tiene retención de 90 días; los originales, esperados, fuentes del banco y el [informe completo](evidencias/RETP-106/verificacion-219d6a37.json) quedan además en Git. Sus herramientas y huellas están en la misma carpeta. No se presume conservación indefinida de una URL.

Reproducción desde la raíz del corte material, con las herramientas indicadas:

```sh
node tests/retorno_cyb/verificar.mjs --fuentes
cargo +1.98.0 test --manifest-path rust/Cargo.toml -p sv_core --test cyb_reception --offline
cargo +1.98.0 run --manifest-path rust/Cargo.toml -p sv_core --example cyb_reception_probe --release --offline > transporte.json
node tests/retorno_cyb/verificar.mjs transporte.json --autoprueba > verificacion.json
sha256sum transporte.json verificacion.json
```

El portador EN y su proyección de referencia ya existían. La huella de proyección fijada por el observador se cotejó con el ejecutable custodiado del corte RETP-104/105; es referencia de regresión de la misma realización, no oráculo doctrinal independiente. El SHA de cada contrato sí se recalcula mediante el codificador externo LIG ya contrastado con el vector independiente.

### 5.3. Dictamen por capacidad

| Capacidad examinada | Dictamen | Consecuencia |
|---|---|---|
| Recuperar cargas completas o con S mediante LIG y el portador | **Suficiencia acreditada en el alcance documental ejecutado** | No hay pérdida impuesta por LIG en estas cargas y usos; no se justifica ampliar la IR para corregir una pérdida no encontrada |
| Recuperar de H la distinción que eliminó | **Pérdida demostrada para 18 pares y consultas fijadas** | Exigir F0 o la S declarada cuando se consuma esa distinción; ninguna suficiencia de H por igualdad del resultado técnico |
| Aplicar las reglas documentales recibidas tras recuperar datos | **Concordancia externa acreditada para 78 casos y consultas de pares** | La regla vive en el observador de prueba; la aceptación LIG no equivale a su aplicación ni a autoridad |
| Ejecutar OP-CYB-001, Frame, consejo y transducción productivos | **No acreditado por esta campaña** | Continúan fuera de la oferta, con dependencias concretas. No se sustituye esa falta con el resultado del observador |
| Autenticidad, permisos, acceso, custodia y cobertura reales | **No acreditado** | Requieren constitución, observación y soporte material correspondientes; R2/R3/R4 no aprobados |

No se ha reproducido un defecto nuevo del núcleo en este perímetro. La pérdida demostrada está en la reducción H elegida para el caso. Tampoco se deduce que toda necesidad profesional esté ya realizada: la prueba demuestra conservación y uso documental externo, mientras la interpretación operacional y la imposición material tienen sedes diferentes.

Los artefactos `ConstitucionD`, `AutorD`, `Phi`, `Regla` y `OP` siguen siendo los literales sintéticos del portador anterior. Las reglas profesionales del observador están fijadas por su código y los anexos identificados; **no se ha constituido una ligadura productiva entre esas reglas y una operación CYB mediante aquellos literales**. El transporte recupera documentos que el comprobador externo consume bajo su regla fija. Éste es precisamente el límite que exige concretar el consumidor operacional siguiente y no atribuir a `OP` una ejecución profesional que no realiza.

### 5.4. Trabajo que permanece y punto exacto de continuación

1. **Consumidores operacionales del Lenguaje:** concretar qué operación SV debe usar atribución/autoridad/tiempo (RS01–03/05), vistas/alcance (RS04/07) y continuidad/obligaciones (RS09–12). Para cada una, fijar entradas, regla recibida, resultado y última capacidad disponible; decidir la sede formal sin convertir toda la carga en una cadena opaca ni elevar los nombres de relación del dominio a invariantes universales. Hasta materializar el consumidor, no ofrecer su ejecución.
2. **Identidad paramétrica y arquitectura CYB:** LIG mantiene dependencias de captura/admisión nominal y agente/grafo. Si un consumidor exige posiciones, Frame o cobertura celular, devolver esa necesidad constitutiva precisa; no asignar los 32 parámetros desde el Lenguaje. El contrato externo de documentos ya contrastado puede continuar sin esa atribución.
3. **K1-T y productores:** identificar las operaciones que requieran observación→Tri, criticidad o consejo ejecutado; conservar DFL-006 y la puerta algebraica de la secuencia. Los resultados de los auxiliares no los sustituyen.
4. **Soporte y DFL-009:** evaluar en fila 9 servicio nativo, identidad efectiva, límites, aislamiento, fallo, repetición y coste frente a estas necesidades. Reutilizar la evidencia del laboratorio y sus límites; no seleccionar plataforma por este expediente ni declarar seguridad material a partir de transporte correcto.

No se requiere reabrir la aprobación del universo ni otro universo para continuar. La recepción técnica y este primer contraste quedan realizados **en candidata**; la fila 9 y el núcleo permanecen abiertos. La integración registral se distinguirá de la existencia de la PR y de sus pruebas. Ciberseguridad conserva su pausa; ningún pendiente sin realización se presenta como capacidad autorizada para pasar.
