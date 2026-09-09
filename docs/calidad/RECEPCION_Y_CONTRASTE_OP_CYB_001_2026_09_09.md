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
