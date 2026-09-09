# Contrato de consumo documental CYB y evaluación de soporte

**RETP-107 · 09/09/2026 · candidata del Lenguaje, fila 9.** Continuación autorizada por Juan Antonio Lloret Egea. Base examinada: `60bfdf44b2391d2a8bf85a4b37791a6dabd300c8`, árbol `fdea9fca8bb6c41064d2c5c90a0e6441c3859688`, PR #87 sin integrar. Se conservan el relevo CYB `169af16d05ffc954454bb5528ec106e5752b7016`, su constitución `b3aa3f01c825bf6440e3f04aadc87cc7c7ee6d89` y los once originales de RETP-106. Se han consultado AGENTS, Pilares, perfiles, tabla rectora, RETP-105/106, LIG y los registros de laboratorio 018/020 en su corte identificado.

## 1. Problema que se resuelve

RETP-106 enlaza los datos mediante LIG, pero la definición `OP` es un literal sintético: no identifica la regla realmente aplicada por el observador. Esta candidata reemplaza ese literal por una definición comprobable de consumo documental y liga los bytes exactos del consumidor y de sus reglas. El banco D/AG sigue siendo sintético: este cambio no constituye el dominio, el agente ni una operación productiva OP-CYB-001.

La sede es **contrato externo comprobado por el banco**, con integridad y referencias LIG verificadas por el núcleo existente. La selección y aplicación de la regla se comprueban fuera del núcleo. No se añade una palabra de la DSL, tipo de IR, evaluador profesional en `sv_core`, permiso R1, transductor ni Frame. La ejecución externa queda identificada como tal y no se presenta como segunda semántica SV.

## 2. Contrato previo a la ejecución

`tests/retorno_cyb/consumidores.json` fijará identidad, versión, fuente, módulo de reglas y módulo consumidor por SHA-256, así como un inventario cerrado de selectores: P25…P32, las seis clases CT y los campos discriminantes SI. No contendrá identificadores de estados, respuestas esperadas por caso ni código recibido de una fuente viva. Los selectores se derivan de los datos de entrada y de la consulta del par, nunca de su respuesta esperada.

Cada artefacto `OperationDefinition` declarará el selector, la versión exacta, las referencias de los dos módulos y la variante de entrada. Sus bytes se conservarán en `ValidatedBindings`. Los módulos serán artefactos de prueba identificados; LIG custodia sus bytes sin ejecutarlos ni autenticar su autoridad.

El consumidor externo deberá:

1. Recibir una definición idéntica a la fijada por el contrato y comprobar los bytes de ambos módulos antes de cargar el módulo de reglas. Sólo se ejecutarán esos bytes capturados; no se evaluará una ruta o texto arbitrarios recibidos del informe.
2. Obtener la carga desde las referencias exactas del contrato validado, comprobando que la recuperación publicada coincide. F0 consume procedencia; HS consume exclusivamente S declarada. No buscará datos por identificador de caso en los originales.
3. Rechazar H para el consumo que exige la distinción suprimida con `INFORMACION_INSUFICIENTE`, antes de aplicar la regla. Éste es un resultado técnico del banco, no `Tri.U` ni un resultado de dominio.
4. Comprobar que `parametro` o `clase` coincide con el selector fijado, o que el campo SI consultado existe. La extracción SI sólo recupera ese campo; no interpreta el expediente entero ni certifica legitimidad.
5. Aplicar las reglas externas ya recibidas en RETP-106, sin cambiar sus decisiones. Sus cadenas de resultado mantienen el estatuto documental de los anexos; no son constructores de `Tri`, `CheckResult`, `Permit` ni consejo.

Toda entrada JSON usa el lector documental estricto, incluidos manifiesto, definición de operación, carga recuperada e informe. Las magnitudes exactas conservan sus cadenas y comparación sin coma flotante. El manifiesto y los módulos quedan fijados antes de utilizar la salida de Rust como evidencia. La huella prueba identidad de bytes; no prueba corrección científica de la prosa, autoridad institucional ni autenticidad remota del emisor.

## 3. Refutadores fijados y aceptación

Se conservan los 186 contratos, los 78 casos completos, los 18 pares y sus esperados originales. Se requieren 150 respuestas documentales coincidentes (78 más 36 F0 y 36 HS), 36 rechazos explícitos de H y conservación de la igualdad de los dos contratos H de cada par. Las cuatro entradas nativas conservan su prueba; esta campaña no se atribuirá a WASI/navegador.

Además de las guardas anteriores, deberán rechazarse con causa diferenciada: definición de operación distinta con sus huellas recalculadas; regla distinta con referencias coherentemente actualizadas; consumidor distinto; selector incompatible con la carga; definición sin versión; H ofrecida como suficiente; HS sin información lateral; petición de una regla no constituida. Cada ataque tendrá un control intacto que vuelva a admitirse. Los ataques a la semántica del enlace deben superar la integridad criptográfica y llegar al comprobador pertinente.

Un módulo de reglas alternativo debe ser rechazado **antes de ejecutarse**; la prueba utilizará un efecto centinela local inocuo para distinguir rechazo temprano de ejecución seguida de error. La repetición de los 22 mutantes documentales previos conserva su alcance. No se declarará cobertura exhaustiva por contar mutantes.

## 4. Consumidores profesionales: qué queda decidido y qué falta

| Necesidad recibida | Último consumo disponible con este contrato | Sede y obligación que continúa antes de ofrecer la operación profesional |
|---|---|---|
| RS01/02/05: atribución, competencia, alcance y plan | P25/P26/P27/P31/P32 y consultas SI, sobre documentos fijados | Dominio/institución aportan raíz, facultades, delegación y cobertura efectivas. El Lenguaje debe enlazar esos referentes con la operación; no convertir `AuthorityDeclaration` en autoridad R1 por nombre. |
| RS03: tiempo, revocación y antecedentes | P28/P29/P30, con intervalos exactos y reglas del anexo | Captura constituye tiempo, incertidumbre y vigencia. Lenguaje recibe su representación y consumidor; DFL-003/004 y soporte conservan causalidad, continuidad y revocación material. |
| RS04/07: vistas y cobertura | Recuperación del campo SI y clase CT `perimetro` | Leer el nombre de una vista no impone acceso, finalidad, retención ni cobertura de red. Contrato de agente y frontera deben declararlos e imponerlos; DFL-009/R3/R4. |
| RS06: admisión, indeterminación y fallo | Respuestas documentales originales preservadas | K1-T y DFL-006 permanecen: faltan captura, admisión y transducción productivas. Ningún resultado del auxiliar se convierte en un valor SV. |
| RS08: consejo y resultado técnico | Distinciones SI recuperables | Productores de consejo/criticidad y ejecución algebraica siguen pendientes. Si se requiere Frame, G6 debe constituir la arquitectura; 32 parámetros no determinan células. |
| RS09/10: partida y relaciones | Clases CT `partida` y `linaje` | Los tipos CYB no se universalizan. DFL-003/004/006 y R2 conservan historia efectiva, causalidad y persistencia. Las referencias circulares no se equiparan a precedencia estricta. |
| RS11: asignación, aceptación y obligación | Clase CT `asignacion` | No se acredita designación institucional ni se libera al titular anterior. Las fuentes de autoridad y la custodia permanecen externas; no se inventa un adaptador R1. |
| RS12: conciliación y reevaluación | Clases CT `conciliacion`, `perimetro` y `vigencia` | Capturadores aportan identidades, cobertura y hechos; Lenguaje/motor conservan dependencias, efectos inciertos y continuidad. No se sustituye identidad por cardinalidad. |

La vía pública R1 existente (`decide_permit_traced`, `mediate_traced_permit`, `execute_traced_mediated`, bajo `ProtectedDecisionContinuity`) conserva su cierre intraproceso. Este expediente no ha constituido la conversión del catálogo CYB a sus autoridades, requisitos, reglas de resolución y efectos. Utilizar cadenas parecidas o un `CheckResult` fabricado sería una ligadura falsa. La futura correspondencia debe recibir esos objetos de sus sedes competentes y probar su recorrido completo.

## 5. Cotejo IMM/CYB y DFL-009 en fila 9

PT03/PT04/PT12/PT14 se aplican sin sumar permisos ni garantías. IMM conserva Q0, cuatro salidas y doce SP integradas pendientes; CYB conserva sus resultados y el régimen documental aprobado. Ambos exigen identidad, regla recibida, información lateral declarada y separación entre fallo y resultado. Esa coincidencia justifica reutilizar LIG y el método de prueba; no justifica una regla clínica o profesional común. F0/HS suficientes para recuperación no habilitan ambos consumidores productivos.

La evaluación DFL-009 recibe el [registro 018](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/e97fed715ff5e3ae19bbaccaf2852e9cd3288377/laboratorio-de-infraestructura-SV/registros/018-COMPARACION_DOTNET_FFI_WASM_2026_09_06.md) y el [registro 020](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/e97fed715ff5e3ae19bbaccaf2852e9cd3288377/laboratorio-de-infraestructura-SV/registros/020-PERFIL_TECNOLOGICO_Y_CONTINUIDAD_2026_09_06.md). Se han leído sus registros; no se ha repetido la campaña ni descargado y revalidado sus paquetes brutos en este incremento. Se mantiene su custodia y acceso originales.

| Opción o obligación | Evidencia recibida | Decisión acotada de evaluación |
|---|---|---|
| Servicio con ejecutable nativo identificado | Binario autónomo y repetición de RETP-106; 018 enlaza Rust nativo | Viabilidad del ejecutable local observada. Servicio remoto, protocolo, autenticación, saturación y recuperación no ensayados: no ofrecidos. La instalación de Rust en el visitante no es requisito de un servicio, pero aquí no se ha publicado tal servicio. |
| FFI en proceso | 018 observa menor coste y caída del anfitrión ante aborto sintético | No satisface por sí sola contención del proceso. No se selecciona como frontera de seguridad. |
| WASM ensayado desde .NET | 018 contiene ciertos fallos con combustible/memoria; anfitrión altera centinela | No protege contra ese anfitrión. Corpus EN y núcleo antiguo de 018 no certifican contratos CYB nuevos. |
| Proceso nativo separado | 018/020 lo identifican como comparación pendiente | Requiere ensayo en laboratorio de coste, recuperación y límites cuando el contrato los exija; no se atribuye al binario autónomo una prueba de aislamiento. |
| Cloudflare/Workers u otro servicio | No se ha ejecutado en este expediente una realización correspondiente | Sin veredicto de aptitud ni elección. La futura evaluación identificará si el destino es nativo o WASM y los componentes reales. |
| Identidad, repetición y efectos | R1 intraproceso; CS01–CS09 especificados, enlace material pendiente según 020 | Reutilizar esos criterios, sin crear otra batería equivalente ni equiparar identificador de solicitud y derecho a ejercer un efecto. |
| Presupuesto y dependencias | Medidas de 018 y herramientas identificadas del banco | No se inventan umbrales, SLO ni garantías con medianas. Python/Node externos y construcción/publicación siguen separados; no se declara retirada global. |

**Resultado de evaluación:** no hay soporte remoto admitido ni evidencia que obligue a elegir plataforma para este consumo documental. DFL-009 sigue abierta por servicio y garantías materiales; deja de estar meramente diferida: sus alternativas, evidencia reutilizable y brechas quedan examinadas. Los ensayos nuevos, si proceden, se realizarán y documentarán en el laboratorio antes de promover soporte. La fila 9 no se cierra mediante este dictamen.

## 6. Continuidad y estado previo

Estado de esta primera confirmación: **CONTRATO_FIJADO · IMPLEMENTACION_Y_PRUEBAS_PENDIENTES**. El siguiente cambio material implementará exclusivamente el enlace de consumo definido aquí y registrará sus resultados sobre la cabeza exacta. No fusiona #87, no constituye otro universo, no modifica README, actas históricas ni el español. DFL-011/012/013 mantienen identidad; Ciberseguridad sigue en pausa. El cierre de este enlace documental no cerrará los consumidores profesionales enumerados en §4, la fila 9 ni el núcleo.
