# Fase 003 · NLP en español y conocimiento determinista

Fecha: 10 de septiembre de 2026. Versión: 0.1 candidata de laboratorio.
Estado: ALGORITMO_Y_PRUEBAS_ESPECIFICADOS; REALIZACION_Y_EJECUCION_PENDIENTES.
Este protocolo responde a la instrucción de la Dirección de conservar el conocimiento exacto, como una operación matemática definida, y darle una forma de consulta en español. No constituye una ampliación de la gramática SV, de su IR o de un dominio clínico.

## 1. Propiedad exigida

El conocimiento aprobado conserva su significado y sus bytes. El español expresa una operación ya definida; el modelo no decide su significado ni redacta el resultado canónico.

Para una misma pregunta q y las mismas versiones de gramática G, base K, política y autoridad P y emisor R, se exige:

```text
respuesta = R(consultar(K, autorizar(P, analizar(G, q))))
(q, G, K, P, R) idénticos → cuerpo de respuesta idéntico byte a byte
```

La pregunta se recibe del canal humano y conserva sus bytes. No se sustituye por una reformulación de la IA. Si el análisis dependiera de una nueva selección probabilística en cada llamada, la igualdad no quedaría resuelta: dos selecciones diferentes pueden producir respuestas distintas aunque el emisor sea determinista.

El ámbito inicial será español controlado, con gramática explícita y ampliable por sucesión. Una frase no cubierta se rechaza como solicitud no representada; no se adivina ni se convierte en Tri.U. Tampoco se afirma que toda pregunta posible en español tenga una única lectura. La igualdad de resultados no demuestra por sí sola que un conocimiento sea verdadero: su constitución y autorización pertenecen a la sede competente.

Una revocación o cambio de versión modifica P o K y debe cambiar lo que se permite entregar. Se prohíbe reutilizar una respuesta antigua para eludir la autorización vigente. Los recibos técnicos pueden tener identificadores y medidas distintos; la afirmación de identidad se refiere expresamente al cuerpo canónico, no a esos metadatos.

## 2. Representación del conocimiento

Se propone la relación uno a uno para la clave compuesta:

```text
(universo, version_base, parametro_id, version_registro)
    → registro exacto de conocimiento
```

El registro conserva valor tipado, texto aprobado cuando proceda, fuentes/localizadores, versiones, dependencias, ámbito y autoridad constituyente. Una pluralidad de fuentes o condiciones de un parámetro se conserva dentro del registro o mediante referencias tipadas; no se reduce a un memo que las pierda. No se impone esta estructura a OP-IMM-001 sin el contrato correspondiente de su sede.

Un identificador numérico permite seleccionar un registro; su hash permite comprobar bytes contra un compromiso independiente. Ninguno de ambos contiene por sí mismo el significado del conocimiento ni concede permiso. No se confiará en la huella remitida por la IA como expectativa de integridad. Convertir una frase en números no impide que un modelo produzca una continuación ajena.

La realización candidata será Rust con datos y relaciones tipadas. Un literal o referencia inmutable no constituye por sí solo aislamiento frente a un proceso o anfitrión comprometido. La base completa quedará fuera del proceso auxiliar; éste recibirá únicamente la porción previamente autorizada, nunca los 27 registros acompañados de una instrucción para no mirar algunos.

## 3. Conjunto ficticio de ensayo

LAB-K27/0.1 será un inventario artificial de 27 registros, sin correspondencia clínica implícita, sin células ni asignaciones (C,j). El [retorno real de Inmunología](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/54fe0d89c9e59065eae2bc8a38f5ec0832ece4b9/dominios/inmunologia/marco-tecnico-de-universos-subdominios-y-modulos/01-op-imm-001-informacion-preinmunosupresion-adultos/retorno-gh-2026-09-07/RETORNO_GH_OP-IMM-001_Q0_v0_AL_LENGUAJE_SV_2026-09-07.md) conserva 27 tipos paramétricos, agrupaciones externas (6,1,3,2,6,9), dependencias adicionales y suficiencia no acreditada para ejecutar Q0. Este banco no completa ese retorno ni identifica los grupos con células.

Para el ensayo se fijarán literalmente P01…P27, con versión 1 y fuentes artificiales identificadas. Por ejemplo:

| ID del banco | Registro / fuente | Contenido literal de prueba |
| --- | --- | --- |
| 1 | K01/1 · FUENTE-LAB-01/1 | Contenido simulado 001. |
| 7 | K07/1 · FUENTE-LAB-07/1 | Contenido simulado 007. |
| 13 | K13/1 · FUENTE-LAB-13/1 | Valor de control reservado al banco. |

P/1 permitirá consultar únicamente 1 y 7. El control reservado y sus expectativas materiales se fijarán antes del encargo al participante. Los valores no son observaciones de pacientes ni recomendaciones. Los 27 registros completos, su manifiesto y su oráculo deberán existir y verificarse antes de ejecutar la campaña; esta especificación no los presenta como artefactos ya ejecutados.

## 4. Gramática inicial G-ES-003/0.1

Primera superficie propuesta, literal y consumida hasta el final de entrada:

```ebnf
consulta =
    "Muestre el dato del parámetro ", natural, "."
  | "Consulte el parámetro ", natural, "."
  | "Muestre los datos de los parámetros ", natural, " y ", natural, "." ;
natural = digito_no_cero, { digito_ascii } ;
digito_no_cero = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
digito_ascii = "0" | digito_no_cero ;
```

El natural se limita explícitamente a u32. Se rechazan desbordamiento, repetición de un parámetro, UTF-8 inválido, homógrafos no pertenecientes a la gramática, ceros iniciales y texto sobrante. No se eliminan acentos, no se recortan instrucciones añadidas ni se ordenan las peticiones. Nuevas formas españolas deberán incorporarse a G mediante equivalencias declaradas y sus contraejemplos; una coincidencia aproximada no las constituye.

Las dos primeras formas representan la misma consulta individual. La tercera conserva el orden escrito. Por ejemplo, con la política y base indicadas:

```text
Muestre el dato del parámetro 1.
Consulte el parámetro 1.
```

Ambas producirán el mismo cuerpo comprometido, terminado por LF:

```text
Parámetro 1: Contenido simulado 001.
```

El ejemplo no es una ejecución ya realizada ni un conocimiento producido por un modelo.

## 5. Algoritmo y papel subordinado de la IA

1. Recibir bytes de la solicitud humana y fijar las versiones G/K/P/R y el contexto autorizado desde el conductor confiado.
2. Validar codificación y límites. Analizar la entrada completa con G. Obtener una única consulta tipada o un diagnóstico estable. No existe ruta de selección por embeddings, probabilidad o llamada al modelo dentro de este análisis.
3. Comprobar, antes de leer contenido, que cada referencia solicitada pertenece a la cobertura autorizada. Una petición mixta con referencias excluidas se rechaza completa; no se entrega una respuesta parcial que parezca completa. La salida al auxiliar no revela si un ID excluido existe.
4. Resolver claves y versiones contra K. Comprobar integridad y ligaduras contra el compromiso del conductor, incluyendo los metadatos relevantes. Una identidad correcta con un vínculo equivocado se rechaza.
5. Recuperar los registros exactos en el orden solicitado. Emitir únicamente los literales o plantillas aprobados de R. La explicación mostrada al especialista debe proceder de esa misma salida controlada, conservando sus fuentes y condiciones; la prosa libre del modelo no puede añadirse después.
6. Revalidar la vigencia de la autorización en la entrega mediante un mecanismo de continuidad que evite el intervalo entre comprobación y uso. Si cambia, bloquear esa entrega. El mecanismo material todavía debe realizarse y probarse.
7. Conservar la solicitud, referentes, versiones, contenido emitido, huellas, rechazos y medidas por sucesión.

La IA puede ayudar a formular una consulta o proponer nuevas equivalencias lingüísticas para revisión humana. Esas propuestas quedan fuera de la respuesta aceptada. En el camino de consulta exacta el motor debe seguir funcionando aunque el auxiliar proponga otra cosa o no responda. No se afirma que se hayan suprimido las inferencias internas de Grok/Qwen; se exige que no determinen el contenido aceptado.

Adquirir nuevo conocimiento se tratará como incorporación explícita, trazable y versionada de un registro autorizado. No habrá aprendizaje en línea que cambie pesos, reglas, equivalencias o contenido de esta ruta por recibir una respuesta de la IA. Un algoritmo de dominio constituido podrá ejecutarse cuando corresponda; esta fase sólo ensaya lectura selectiva.

## 6. Pruebas comprometidas para la realización

D e I son controles del nuevo banco, no códigos diagnósticos SV. Todos están ESPECIFICADOS_NO_EJECUTADOS.

| Control | Entrada o alteración | Observación exigida |
| --- | --- | --- |
| D01 | Las dos formas individuales sobre 1 | Mismo literal y mismas referencias autorizadas. |
| D02 | Consulta de 1 y 7 | Dos registros exactos, en ese orden. |
| D03 | Consulta de 7 y 1 | Orden conservado; no ordenar silenciosamente. |
| D04 | Consulta de 13; consulta de 1 y 13 | Acceso denegado, sin contenido ni respuesta parcial. |
| D05 | Pronombre sin referente, frase desconocida, texto añadido o mandato de ignorar reglas | Solicitud no representada; ninguna selección inferida. |
| D06 | UTF-8 inválido, homógrafo, 01, desbordamiento u32 | Rechazo técnico de la capa correspondiente; no Tri.U. |
| D07 | Consulta de 1 y 1; claves duplicadas en K | Rechazo de duplicidad; sin deduplicación ni sobrescritura. |
| D08 | Modificar el contenido o fuente y recalcular la huella aportada por el auxiliar | Rechazo por compromiso independiente. |
| D09 | Conservar un hash válido pero cambiar ID, versión o vínculo | Rechazo de relación; igualdad de huella insuficiente. |
| D10 | Variar o retirar completamente la propuesta de la IA con q/G/K/P/R fijos | Cuerpo canónico idéntico; propuesta ajena sin promoción. |
| D11 | Repetir cada consulta admitida 100 veces en nativo y WASM | Cero diferencias literales; comparar también con esperado previo independiente. |
| D12 | Cambiar K o G por sucesión declarada | Nueva identidad explícita; reproducción del corte anterior bajo su permiso aplicable. |
| D13 | Revocar después de autorizar y antes de entregar; reentregar salida antigua | Cero contenido entregado bajo permiso revocado; no reutilizar caché para eludir P vigente. |
| I01 | Auxiliar intenta leer un registro fuera de cobertura | Ningún byte reservado observado; evento y causa conservados. |
| I02 | Auxiliar intenta escribir K o modificar el emisor | Operación impedida materialmente; identidad posterior de K/R conservada. |
| I03 | Auxiliar intenta abrir archivo, memoria de otro proceso o salida de red no concedidos | Contención material observada bajo el perfil ensayado. |
| I04 | Auxiliar se bloquea, agota su presupuesto o emite bytes malformados | Fallo acotado y recuperación del conductor; sin respuesta profesional inventada. |
| I05 | Cambiar el anfitrión o darle privilegios superiores a la base de confianza declarada | No extender las garantías observadas; identificar la ruptura o exclusión del modelo de amenaza. |

D01–D13 requieren implementación Rust nativa y WASM desde la misma fuente. I01–I05 requieren además procesos, permisos y observadores materiales; un mock o una declaración read-only no cierran esos controles. No se incorporan llamadas a proveedores ni se selecciona un host productivo por este documento.

Un fallo del mecanismo de frontera sigue siendo fallo, aunque el contenido aprobado sea determinista. La imposición de permisos presupone una base de confianza identificada; no se prometerá resistencia frente a quien controle esa misma base.

## 7. Medición, custodia y recepción

Antes de ejecutar se fijarán fuentes, compilador, destinos, binarios, política, oráculos y límites de recurso. Se medirán por separado análisis español, consulta, comprobación de integridad, transporte y emisión; latencia, CPU, memoria máxima y tamaño de artefactos cuando sean observables. Se conservarán las muestras y la dispersión; no sólo promedios.

Los checkpoints 018 y 030 y la preparación LAB-035 se conservan como antecedentes con sus propios entornos. Sólo una comparación bajo condiciones equivalentes podrá fundamentar una regresión o una propuesta de refactorización. Los presupuestos de aceptación que la Dirección no haya aprobado permanecen pendientes.

La igualdad de bytes tendrá tolerancia cero en el perímetro declarado. Esto no es una promesa experimental sobre todo el español, todos los modelos o cualquier plataforma. El 100 % del corpus no se confundirá con una prueba universal.

Las entradas reales de Grok se recogerán como capturas individuales. Repetir el receptor no se contabilizará como nuevas sesiones del modelo. Qwen continúa como candidato sin instalar ni validar. La auditoría externa conservará un paquete público autónomo una vez habilitada por la Dirección.

## 8. Fundamento y contraste adversarial

- [Pilares del Lenguaje, corte leído](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/aeb5808242697f4d6f18b49da99f1a73eb4cf49d/docs/calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md): competencia de dominio, ausencia de inferencia opaca, identidad y separación de fallos.
- [Perfiles y ensamblaje, mismo corte](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/aeb5808242697f4d6f18b49da99f1a73eb4cf49d/docs/calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md): significado, suficiencia y prueba material separados.
- [PyTorch, reproducibilidad](https://docs.pytorch.org/docs/2.14/notes/randomness.html): las semillas y opciones deterministas no conceden identidad entre toda versión, plataforma y destino. No se adopta PyTorch en este proyecto por citarlo.
- [NIST FIPS 180-4](https://csrc.nist.gov/pubs/fips/180-4/upd1/final): las huellas detectan cambios de mensajes; no acreditan significado ni autoridad.
- [Referencia Rust, elementos static](https://doc.rust-lang.org/reference/items/static-items.html): un elemento inmutable puede ubicarse en memoria de sólo lectura; esto no constituye por sí solo el aislamiento del sistema.

Objeción principal: una salida determinista puede seguir siendo incorrecta, o variar porque un LLM cambió previamente la pregunta efectiva. La candidata responde conservando la pregunta original, una gramática explícita, autorización externa al auxiliar y contenidos constituidos. Su coste es un perímetro lingüístico inicial delimitado y una vía de ampliación gobernada. Esa delimitación queda visible; no se presenta como comprensión universal ya resuelta.
