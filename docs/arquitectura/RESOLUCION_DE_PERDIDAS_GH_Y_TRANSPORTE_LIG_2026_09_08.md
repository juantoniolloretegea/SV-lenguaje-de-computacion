# Resolución por operación del retorno G/H · GH-LIG/0.1

**Entrada:** `d374e1cb373a3dcd141faf482f9501ceb47a7e0b`, PR #80 apilada sobre #79, ambas candidatas. **Retorno recibido:** SVperitus-dataset `54fe0d89c9e59065eae2bc8a38f5ec0832ece4b9`. **Fecha:** 08/09/2026. **Estado:** propuesta de resolución, fila 7 abierta.

## 1. Obligación y sede

La fila 7 de la adenda secuencial exige cambios justificados por G/H y una matriz de resolución de pérdidas, vinculada a operación, contrato, versión y evidencia (PT01/PT03/PT04/PT14). Este acto sucede a RETP-098; no modifica la tabla ni el retorno. El inventario conserva 15 requisitos G10, 44 formulaciones LSV, 81 enlaces, ocho pares documentales, doce comprobaciones integradas y la aplicabilidad de seis familias/18 consultas F-IF. Se publica en `tests/row7_gh/resolucion.json` y se contrasta con `inventario-gh.json`, extraído sin reformular los campos seleccionados del retorno exacto. Los localizadores, responsables y formulaciones originales permanecen consultables.

El alcance ofrecido sigue siendo representación y bienformación declarativa, ligaduras LIG/0.1 y transporte documental sintético. Ninguna operación clínica Q0 se ofrece. Las doce SP permanecen **sólo especificadas y no ejecutadas de extremo a extremo**; el estado de una parte local no cambia ese hecho. La resolución distingue trabajo material realizado, pérdida documental refutada o recuperada, capacidad excluida y condición concreta para reabrirla. Una exclusión es un límite comprobable de lo ofrecido, no una licencia para producir resultados incompletos.

## 2. Testigos previos de transporte

Se reutilizan literalmente los ocho GH-DOC y sus dieciséis estados. Las entradas y respuestas esperadas proceden de `testigos.json` del retorno, identificado por bytes, SHA-256 y blob Git; no se generan a partir de `sv_core`. El programa portador es el testigo sintético D/AG de LIG/0.1. Su parámetro P no representa ninguno de los 27 parámetros clínicos. Un uso, una instancia, sin destino celular y sin ternarizador consumido: no se constituye Q0 ni se fabrican células.

Para cada estado se construyen tres contratos de prueba:

- **F0:** procedencia con el paquete documental completo.
- **H:** procedencia con `schema`, `context` y `common`; se elimina únicamente `detail`. No existe S ni artefacto sobrante que conserve el detalle. Los dos estados del mismo par producen **el mismo contrato H completo**, incluida su huella; el identificador de estado pertenece al banco, no al contrato.
- **HS:** el mismo H más un artefacto S que contiene `detail`, referido expresamente por la operación bajo `BindingsWithSideInformation`.

La definición de operación procede de los campos `operation` del testigo recibido. La sonda pasa los tres contratos por `validate_bindings` y emite los bytes obtenidos desde el resultado validado. El observador externo aplica las rutas documentales declaradas en G/H y exige sus respuestas literales, incluidos tipos, orden y contenido. No se añade `query` al Lenguaje ni se interpreta clínicamente ningún campo. Las huellas de documentos se calculan desde estas entradas; la expectativa contractual de esta sonda se calcula durante su construcción y **no aporta una segunda prueba independiente de la codificación LIG**, ya cubierta por el testigo independiente de RETP-098.

Se exigen 48 transportes aceptados: 16 F0, 16 H y 16 HS. F0 y HS permiten 16 recuperaciones literales cada uno. H conserva 16 respuestas del control `context`, pero conserva también **ocho pérdidas demostradas**: igualdad de entrada H con respuestas originales distintas. No se atribuye la recuperación con S a H sola. Ningún estado de prueba produce Tri ni una salida terminal clínica.

## 3. Refutadores y límites del observador

Antes de aceptar evidencia se atacarán inventario, identidad de operación/estado, bytes, orden, H con detalle oculto, S ausente o fuera de alcance, huellas, igualdad de contratos H y resultados de resolución. Los ataques a la capa semántica recalculan las huellas alteradas para evitar que una guarda de integridad corte antes; un control reserializado debe admitirse. Cada ataque exige su causa, no sólo retorno distinto de cero. El inventario esperado se obtiene de las fuentes recibidas y del presente contrato, nunca de los recuentos del informe ejecutado.

Las sondas de nativo, WASI y navegador deben emitir el mismo informe literal. Son destinos de una misma realización Rust, no semánticas independientes. Este ensayo prueba que LIG transporta los objetos que el observador documental consume; no prueba que el núcleo entienda sus bytes, que autentique la autoridad o que impida a cualquier consumidor externo leer información no declarada. El banco H no contiene esa información y el observador sólo consume la interfaz declarada.

## 4. Matriz de resolución propuesta

Cada requisito mantiene los responsables del retorno. Los tratamientos siguientes se enlazan explícitamente con los 15 G10 y las 44 LSV en `resolucion.json`; no se confunde agrupación de trabajo con fusión de requisitos.

| Tratamiento | Capacidad acotada | Límite / condición de reapertura | Sede |
|---|---|---|---|
| IDENTIDAD | Identidad y versión de contrato/programa/operación; referentes, propietario, usos y alcance de S comprobados. | No acredita identidad completa de una ejecución clínica, credenciales ni significado de documentos. Probar cada dependencia de la operación clínica con constitución y autoridad efectivas. | DFL-005, fila 7; aplicación Q0 condicionada a SP-01/02 |
| BYTES | Paso por LIG y recuperación documental literal a contrastar en esta candidata. | No interpreta unidades, atribución, ventanas, clínica ni historia autoritativa. Mantener fuentes/orden y declarar toda S; para Q0, constituir y ejecutar reglas propias. | DFL-005; SP-05 documental, fila 7 |
| CONTEXTO | Bienformación local de arquitecturas y referentes; CoverageReport contrasta sus tres nombres. | No prueba causalidad, append-only persistente, permisos, cobertura ni CQ1–CQ6. Resolver semántica N3/N4 y contratos de cobertura antes de ofrecer esas operaciones. | DFL-003/004/005; H06/H07 locales en fila 7 |
| CLINICA | Ninguna ejecución clínica ofrecida. Referentes LIG sólo preservan identidad. | No hay reglas productivas, partición, totalidad ni transducción observación→Tri acreditadas. Reglas clínicas constituidas y testigos completos previos a habilitar la operación. | K1-T; puerta algebraica fila 10 y retorno fila 11 según secuencia |
| AUTORIDAD | Identidad de declaración referida y transporte documental. | Una huella o nombre no autentica autoridad; no habilita intervención clínica ni saneamiento técnico. Acreditar legitimidad, ámbito, vigencia y sucesión en la realización receptora. | R1 integrado y soporte R2/R3/R4; frontera fila 13 |
| CRITICIDAD | Ningún productor Q0 ofrecido. | No se deriva criticidad de etiqueta, grupo, presencia de U o firma. Productor trazable y regla constituida, sin reemplazar pendientes por resultados nominales. | DFL-006; realización algebraica fila 10 y contraste fila 11 |
| RESUMEN | Falsación documental finita; orden e información lateral explícitos. | No ofrece resumen clínico reversible, veto ejecutivo, composición de seis frames ni 27 parámetros. Demostrar recuperación de cada respuesta ofrecida y no compensación con reglas/productores efectivos. | DFL-005/004; FFL-E; resolución por operación en fila 7, ejecución posterior según secuencia |
| SALIDA | Ninguna salida clínica ofrecida. Se conserva el codominio recibido. | No emite ni selecciona las cuatro salidas; no introduce quinta salida o consejo. Ejecutar tabla terminal exclusiva con premisas válidas y autoridad material. | SP-07; integración posterior a reglas y productor constituidos |
| FALLO | Errores de compilación/LIG son rechazos técnicos; no contienen resultado clínico. | No prueba interrupción de Q0 ni efectos inciertos; no convierte ausencia en Tri o salida clínica. Ensayar cortes de ejecución en cada fase y garantizar ausencia de salida clínica parcial. | SP-09; DFL-006 y soporte operacional fila 13/R2 |
| REPRODUCCION | Proyección declarativa y sondas del mismo Rust entre destinos, en los conjuntos identificados. | No prueba Q0, concurrencia completa ni independencia semántica. Python compilador permanece retirado. Identidad completa de ejecución y resultados normativos independientes para la operación ofrecida. | DFL-001/013; SP-08 integrado pendiente |
| SOPORTE | Sin capacidad material nueva ofrecida por este incremento. | No acredita almacenamiento autoritativo, recuperación, permisos del entorno, normas de producto ni garantías I/II. Contrato material y ensayos por sede; no seleccionar plataforma por la existencia del prototipo. | DFL-009 fila 9 tras CYB; frontera fila 13; R2/R3/R4 según contrato |
| CONSTITUCION | LIG admite operación sin destino y distingue instancias cuando el dominio las constituye. | Los 27 parámetros y grupos (6,1,3,2,6,9) no se convierten en células. U_NO_DECIDIDO se conserva. Constitución explícita del dominio; nunca geometría inferida por cardinalidad. | DFL-005; retorno acotado sólo si una operación justificada exige esa decisión |
| PERIMETRO | Se recibe la aplicabilidad de seis familias y 18 consultas sin extender Q0. | No abre Ciberseguridad ni otro universo; no introduce DICOM/citometría/médula. Revisión y recepción de candidata identificada con esta matriz; decisión propia del segundo falsador. | Fila 7 abierta; fila 8 a cargo de Ciberseguridad |

## 5. Condición de continuidad

El verificador debe rechazar omisiones o duplicaciones de los 15/44 requisitos, los 81 enlaces recibidos, las ocho familias GH-DOC, las doce SP y las seis familias/18 consultas F-IF. La trazabilidad de los 27 parámetros y las cuatro salidas se conserva en el inventario; `REQ-IMM-SV-011` permanece `U_NO_DECIDIDO`. No se convierte ningún grupo, ni siquiera el de nueve, en célula. IF-IMM-02/03 mantienen su no aplicabilidad Q0; las demás conservan exactamente el límite recibido, no una adopción universal.

La revisión de la matriz decidirá si las capacidades ofrecidas bastan como candidata para el segundo falsador. Hasta esa recepción, #79/#80 y este incremento siguen sin promover y la fila 7 permanece abierta. Ciberseguridad constituye su propio universo en la fila 8; no recibe una obligación de completar Q0. Tampoco se solicita ahora un retorno de Inmunología. DFL-005 no se cierra globalmente: su representación candidata está materializada, mientras las operaciones que requieran interpretación, autoridad, transducción o resultados integrados conservan sus condiciones. DFL-011, el acta del español y RETP-092 no se alteran; DFL-012/013 conservan sus identidades. No se modifican README ni actas históricas.


## 6. Resultado y reproducción del corte material

RETP-099 registra la ejecución de `bcf25c5b566a6535ba1c3851346fbb355b98ae84`: 48 transportes, 16 recuperaciones F0, 16 HS, 16 controles H y ocho pérdidas H; ocho pruebas Rust; 16 ataques del observador rechazados por su causa. Nativo/WASI/navegador emiten informes idénticos. Los cuatro flujos son conformes y conservan la campaña previa de 43 mutantes de núcleo. PR #81 continúa candidata y no promovida.

Desde la raíz del repositorio, con Rust/Cargo 1.98.0 y Node disponibles:

```sh
python tests/row7_gh/generar_entradas.py --check
node tests/row7_gh/verificar.mjs --inventario
cargo +1.98.0 run --manifest-path rust/Cargo.toml -p sv_core --example gh_binding_probe > gh-observaciones.json
node tests/row7_gh/verificar.mjs gh-observaciones.json --autoprueba
cargo +1.98.0 test --manifest-path rust/Cargo.toml -p sv_core --test gh_bindings
```

El flujo `r0-wasm-parity.yml` conserva las órdenes exactas de construcción WASI y ejecución en navegador real, sus informes y artefactos. El modo `--inventario` sólo acredita el inventario; exige un informe real para comprobar transporte. `CONFORME_EN_TRANSPORTE_DOCUMENTAL` no significa suficiencia Q0, autenticación clínica ni cierre de fila 7.


## 7. Rectificación del observador contractual · RETP-100

**Corte examinado:** `b58b4c88d30c1de3548cac9b4ac16ebbf7250717`. La auditoría externa aportada por el Director identifica una omisión real: el observador comprueba formato de `contract_sha256` e igualdad de las dos H, pero no el valor de la huella. Una reproducción sintética del observador anterior admite las 48 huellas a cero. No se presenta esa reproducción como ejecución de Rust. Las ocho pérdidas documentales permanecen probadas por sus bytes y fuentes; el recálculo independiente de la huella contractual queda pendiente de esta corrección.

**Obligación previa:** GH-LIG-OBSERVACIONES/0.2 debe exponer el contrato completo obtenido de `ValidatedBindings`, con todos sus campos y todos los bytes de artefactos, además de la proyección de programa referida. Un codificador JavaScript externo calculará SHA-256 sobre la codificación LIG/0.1 §2 —prefijo, longitudes u64 big-endian, opciones, naturales decimales y orden explícito— sin llamar a Rust ni leer `binding_contract_sha256` como esperado. Se contrastará también con el testigo independiente fijo ya constituido en RETP-098; ninguna huella correcta se fijará copiando la salida nueva de la sonda.

Cada contrato emitido debe corresponder exactamente a la construcción GH-LIG declarada en §2: portador D/AG, una instancia I1/P, un uso U1 sin destino ni alias, referencias y artefactos íntegros, definición de operación G/H, F0/H/HS y S sólo donde corresponde. La plantilla de prueba LIG y los testigos G/H se fijan por huella. Comparar únicamente diferencias entre huellas no satisface esta obligación. Se rechazarán F0 o HS falsificadas, las dos H falsificadas por igual, las 48 huellas a cero y alteraciones de contrato con su huella recalculada.

**Identidad de programa:** el observador coteja el hash de la fuente portadora literal y calcula el de los bytes de proyección publicados; exige sus metadatos de fuente y el mismo enlace en el contrato. Eso comprueba integridad y enlace, no vuelve a implementar ni a demostrar independientemente la semántica del compilador. El informe lo declarará expresamente. Un informe público y coherente no acredita por sí solo que su emisor haya ejecutado Rust; la evidencia de ejecución conserva su sede en los trabajos y artefactos identificados.

**Alcance de la pila:** la #81 no modifica `rust/sv_core/src`; las #79/#80 añaden el validador de ligaduras y comprobaciones de contexto invocadas durante compilación. Las inserciones puras no prueban conservación del comportamiento: las nuevas guardas rechazan entradas antes admitidas. Debe revisarse y describirse ese cambio al promover. La prosa de los tratamientos de la matriz requiere revisión humana; ni la no vaciedad ni una huella demuestran su verdad o suficiencia. No se presenta este refuerzo como cierre de fila 7, Q0, DFL-005 global o independencia semántica.
