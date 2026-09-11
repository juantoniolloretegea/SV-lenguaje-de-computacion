# Correspondencia con /2 y estado de la reserva

**RETP-2026-136 · 11/09/2026.** Corte: Calidad `72db23fbe2bf0ee746c32287c5f0d66d6ebf63dd`; laboratorio `b8c22b897cc1e5eb46db6d6dcf8b824c0c7f388b`. Se cotejaron las identidades de AGENTS, Pilares, perfiles/ensamblaje, transición §§1–30 y arquitectura con sus lecturas íntegras anteriores: se conservan. Se leyeron el objeto RETP-135, workflow, Fase 004, /2, su sucesora de coste, contrato A/V y compromiso público. No se consultaron los archivos reservados.

## 1. Correspondencia explícita

| Obligación de IE004-ES-P2/2 | Realización /3 y recepción A/V |
| --- | --- |
| §1: referencia acotada, conocimiento artificial y ausencia de autoridad del modelo | `servicio.rs` conserva política y lectura literal; la recepción no modifica esas funciones. El montaje se fija fuera de V |
| §2: propiedad del original, préstamos, intervalos y evidencia durable | El JSON A posee sus cadenas; `Solicitud` toma préstamos inmutables y el motor termina antes de destruirlas. Los productos poseen bytes. V recibe otra copia después de la entrega |
| §3: conversión enumerada, UTF-8, separadores y cobertura completa | `Lexico::nueva`, `minuscula`, `separador`, `signo` sin cambios. Escapar JSON no añade normalización lingüística. Siguen fuera la grafía combinante y NBSP |
| §4: G01–G25, todas las alternativas, enteros textuales | `gramatica.rs` intacto; `Gramatica::compilar` y tabla de expresiones fijan su codificación binaria. El verificador V coteja cada transición y cobertura completa |
| §5.1: patrones negativos conjuntivos, operandos textuales y alcance | `Restriccion::unir`, `Significado::negar`, `agregar_negacion`, `regla_sem`, anotación `Negar` de G02; sin modificación |
| §5.2: conflictos conservados y compleción condicionada al contexto | `demanda`, `completar`, `contradictoria`, `causas`; el transporte admite sólo los cinco campos cerrados, nunca una nota externa como contexto |
| §5.3: unión G10 y producto conjuntivo; significados completos antes de permisos | `Op::Disy`, `sem_evaluar`, `significados` y orden estable de `resolver`; sin modificación ni filtrado por V |
| §5.3: diagnóstico, referencia, literal, fuente, alcance y versiones | El nuevo cuerpo conserva esos campos y añade versión de formato, montaje y vigencia explícita. El anexo V permanece separado |
| §6.1: A concluye antes de V, sin transferir sus cuentas | Procesos, inputs, propietarios y contadores separados. El conductor valida y deposita el marco A antes de abrir V; la traza registra el orden |

Esta tabla declara correspondencia de diseño y permite localizar el código. No es una demostración formal de equivalencia para todas las cadenas. La evidencia pública anterior y la nueva campaña conservan ese límite.

Los cinco archivos heredados `gramatica.rs`, `recursos.rs`, `sintaxis.rs`, `semantica.rs` y `servicio.rs` no se modifican. Sus huellas se fijan en el manifiesto previo. No se reconstruye el núcleo ni se reabre el catálogo/localización. Las comprobaciones semánticas de esta ronda reutilizan esperados públicos seleccionados; no sustituyen ni vuelven a calificar los 61 casos de RETP-135 como una captura inédita.

## 2. Diferencias expresas

La sucesora RETP-134 ya sustituyó los barridos de §6.3 por recorrido de dependencias y cambió la unidad contable. Sus 621.146 unidades no se convierten a intentos /2. Se conservan aquellos cupos; esta recepción añade límites explícitos para transporte, decodificación y exportación, con contadores separados del motor. Ninguna suma de memoria solicitada se presenta como RSS.

El certificado /1 de esta recepción prueba una derivación sintáctica completa. No afirma certificar por sí solo el conjunto de significados, una ruta autorizada, la utilidad de una IA o un teorema de seguridad. La resolución completa sigue siendo A. No hay semántica dictada por el orden de nodos, un hash o un segundo LLM.

El sobre individual A es un formato operativo nuevo. El lote reservado `IE004-P3-A/1` sigue siendo formato de custodia, con 24 objetos y sus identidades originales. Esta ronda no cambia sus bytes ni los deserializa. Para la futura captura se fija un máximo documental de 2 MiB para cada lote confiable y su auxiliar, previa comprobación de compromiso sobre bytes. La admisión estricta del lote completo y su extracción uno a uno, sin normalización ni claves añadidas al original, deberán cualificarse antes de utilizar P3. No se presenta ese adaptador como ejecutado aquí.

## 3. Compatibilidad y decisión pendiente de P3

El compromiso público conserva perfil `IE004-ES-P2/2`, base `K-IE004/1`, política `P-IE004/1` y transporte `IE004-P3-TRANSPORTE/1`; SHA-256 del compromiso: `d37246a019332327f54dd45c641944d90e3ae440163d76d0607c5235df63f9f5`. La recepción separada y recuperable fue confirmada por Juan Antonio; el estado histórico del archivo no anula esa declaración posterior.

**Compatibilidad semántica declarada por herencia de §§1–5; compatibilidad operativa integral con P3 todavía NO ACREDITADA.** No son idénticos el identificador de perfil, el calendario, las unidades, el cuerpo serializado ni la codificación del certificado. Sin leer el oráculo, no se puede afirmar que sus comprobaciones paramétricas cubran automáticamente esas diferencias. No se reetiqueta /2 como /3 ni se recodifica a posteriori un esperado reservado.

Antes de abrir la captura hay que completar y cualificar el adaptador de lote, y registrar la decisión del custodio sobre la asociación de este candidato con el compromiso existente. Si una obligación reservada dependiera del calendario/cupo /2 o se alterara significado, alcance u oráculo, deberá resolverse expresamente antes de usarla. Claude conserva la autoría del oráculo y su incompatibilidad con auditar independientemente esa misma autoría.

La reserva queda cerrada. Este incremento termina con la correspondencia, la interfaz individual, la codificación V y sus controles públicos. P4 debe acreditar imposición material; P5, aportación y coste del participante. No se solicita otra entrega de Grok ni se atribuyen a un modelo las derivaciones extraídas de la evidencia pública previa.
