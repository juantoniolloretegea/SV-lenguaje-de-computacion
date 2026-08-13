# SV-AUTH v0.2 — A.2 Clausura operacional de autoridad

## Estatuto

Documento de trabajo de la rama `sv-auth-v0.2`. No modifica retrospectivamente la Frontera normativa v0 ni la IR canónica v0.2 publicada. Su objeto es cerrar la semántica operacional mínima necesaria antes de cualquier implementación J6/E6xx o runtime AUTH.

La genealogía queda separada:

- Frontera normativa v0: Tri irreductible, Res trazable, prohibición de delegación automática de diseño, IA no soberana.
- Manifiesto V1: capas auxiliares producen observables/proxies, no decisiones soberanas; motor subordinado; no cierre automático de U.
- Nota técnica de capa IA trazable: primacía humana, REAL/SIM, append-only, no fabricación de certeza, no bypass de custodia.
- SV-AUTH v0.2: formalización nueva de estados de autoridad, envolvente, capacidades, efectos, no derivabilidad y extensión conservativa.

## 1. Configuración

Una configuración AUTH es

`K = (C^v, I, A^g, Tok, Hist)`

con índices ordinales de versión `v` y época de autoridad `g`.

- `C^v`: Constitución de autoridad.
- `I`: InformationState.
- `A^g`: AuthorityState persistente.
- `Tok`: capacidades efímeras.
- `Hist`: historia append-only con referencia semántica fija.

`Theta` no se usa como TokenState porque ya designa el contrato finito de resolución en la línea de interfaces heterogéneas.

## 2. Tipos nominales sensibles

Se consideran tipos sellados:

`AuthorityRole`, `Grant`, `DetToken`, `HumanToken`, `GovernanceToken`, `SovereignDecision`.

No existe coerción desde valores informativos hacia ninguno de ellos.

`ResolutionRecord` de la IR vigente no es `SovereignDecision` y no existe coerción entre ambos.

`SovereignU` no es un cuarto valor de la terna. Es abreviatura para un `SovereignDecision` cuyo valor es `Tri.U`.

## 3. Envolvente y habilitación

`AEnv(C^v, A^g)` es el conjunto de grants persistentes válidos bajo la Constitución y el AuthorityState vigentes.

`EnabledAuth(K)` es el subconjunto de operaciones autorizadas por grants preexistentes cuyas precondiciones informativas están satisfechas en `I`.

La información puede modificar `EnabledAuth`; no puede crear, ampliar, transferir ni sustituir la fuente persistente de autoridad representada por `AEnv`.

## 4. Firmas de efectos

Para cada regla `alpha` se declaran:

- `Read(alpha)`: componentes que puede leer.
- `Write(alpha)`: componentes que puede modificar.
- `Intro(alpha)`: tipos nominales que puede introducir.

Componentes posibles: `{C, I, A, Tok, Hist}`.

### 4.1. Clases de efecto

- Informational: `Write ⊆ {I, Hist}` y `Intro` no intersecta tipos sensibles.
- Capability: `Write ⊆ {Tok, Hist}` y sólo puede introducir tokens mediante los constructores sellados de esta especificación.
- Commit: `Write ⊆ {Tok, Hist}`; nunca modifica `A` ni `C`.
- Governance: puede escribir `A` y `Hist`; es la única clase que puede crear/revocar grants persistentes.
- Constitutional: puede escribir `C`, y cuando proceda `A`, además de `Hist`; debe preservar `WF_AUTH` y las invariantes globales del SV.
- Restore: inicialización confiable desde historia previamente comprometida; no es una transición de decisión y nunca restaura tokens vivos.

## 5. Tabla finita de reglas sensibles

| Regla | Clase | Lee | Escribe | Introduce | Premisas mínimas |
|---|---|---|---|---|---|
| `INFO` | Informational | I | I, Hist? | ninguno sensible | entrada tipada/admitida |
| `VERIFY` | Informational | I | I, Hist? | `VerifiedCertificate` | verificador declarado |
| `REQUEST_HUMAN` | Informational | I | I, Hist? | `HumanReviewRequest` | base declarada |
| `MINT_DET` | Capability | C,A,I | Tok,Hist | `DetToken` | grant Det vigente; `Authorizes_G`; certificado verificado; binding completo |
| `ADMIT_HUMAN_ACT` | frontera | C,A,Hist | Hist? | `HumanAuthorizationAct` admitido | juicio externo `Gamma_H |- a : HumanAuthorizationAct`; principal/grant/versión coherentes |
| `MINT_HUMAN` | Capability | C,A,I,Hist | Tok,Hist | `HumanToken` | acto humano admitido y fresco; grant Human vigente; binding completo |
| `MINT_GOV` | Capability | C,A,Hist | Tok,Hist | `GovernanceToken` | procedimiento gubernativo admitido; binding completo |
| `COMMIT_DET` | Commit | C,A,I,Tok,Hist | Tok,Hist | `CommittedDecision` | token Det vivo; subject=principal(token); binding exacto; base verificada |
| `COMMIT_SOV_U` | Commit | C,A,I,Tok,Hist | Tok,Hist | `SovereignDecision(U)` | HumanToken vivo para `commit_sov_u`; certificado de no clausura; binding exacto |
| `RESOLVE_SOV_U` | Commit | C,A,I,Tok,Hist | Tok,Hist | `SovereignDecision(0|1)` | SovereignDecision(U) previo; nuevo HumanToken para `resolve_sov_u`; nueva base verificada |
| `GOV_TRANSITION` | Governance | C,A,Tok,Hist | A,Tok,Hist | `Grant`/binding gubernativo | GovernanceToken válido; resultado `WF_AUTH` |
| `CONSTITUTION_REVISION` | Constitutional | C,A,Tok,Hist | C,A?,Tok,Hist | constitución/ligaduras válidas | procedimiento constitucional; resultado `WF_AUTH` |
| `RESTORE` | Restore | C,A,Hist persistido | I?,A?,Hist,Tok | ninguno nuevo | snapshot confiable; historia semánticamente verificable; `Tok := empty` |

No existe otra regla cuyo `Intro` contenga `Grant`, `DetToken`, `HumanToken`, `GovernanceToken` o `SovereignDecision`.

Toda futura extensión que introduzca uno de esos tipos debe modificar explícitamente esta tabla y volver a demostrar `WF_AUTH`, no escalada y extensión conservativa.

## 6. Binding de capacidad y contexto de uso

Una capacidad queda ligada, como mínimo, a:

`(grant_id, principal, operation, scope, object, candidate/basis, certificate_ref, state_ref, resolver/procedure, v, g)`.

El contexto de uso distingue:

`UseContext = (subject, executor, operation, object, state)`.

Debe cumplirse `subject = principal(token)`.

No se exige en general `executor = principal(token)`: un componente confiable puede ejecutar materialmente una transición autorizada por otro principal. El executor no adquiere por ello la autoridad ejercida.

La acuñación exige que el grant autorice la tupla completa relevante mediante un predicado `Authorizes_G(...)`. El token especializa un grant; nunca lo amplía.

## 7. Frontera humana

`Gamma_H |- a : HumanAuthorizationAct` es un juicio primitivo de frontera. SV-AUTH no demuestra que unos bits procedan físicamente de un humano.

Ninguna regla computacional interna introduce ese juicio.

El sistema puede verificar que un acto ya admitido:

- corresponde a un principal reconocido;
- cae dentro de un grant humano vigente;
- coincide con operación, alcance, objeto, candidato/base y estado;
- pertenece a `(v,g)` vigentes;
- no ha sido consumido antes para acuñar otra capacidad.

`HumanReviewRequest` no produce `HumanAuthorizationAct` ni `HumanToken`.

## 8. Historia y semántica

Cada registro histórico relevante contiene `SemanticEnvironmentRef` inmutable.

Su denotación es siempre:

`Denote(r) = [[payload(r)]]_{SemEnv(r)}`

y nunca se reinterpreta bajo el entorno corriente futuro.

Ninguna regla puede:

- modificar el payload de un registro pasado;
- cambiar su `SemanticEnvironmentRef`;
- usar un `AuthorizationRecord` o `TokenMintRecord` como capacidad viva.

`RESTORE` reconstituye historia legítima; no crea nuevos actos soberanos y elimina toda capacidad efímera viva.

## 9. WF_AUTH e invariantes

`WF_AUTH(K)` es metateoría de bienformación, no un campo configurable de la Constitución.

### IU1 — constitución soberana de U

No existe derivación bien formada desde autoridad puramente determinista hacia `SovereignDecision(U)`.

### IU2 — resolución soberana de U

No existe derivación bien formada desde `SovereignDecision(U)` y autoridad puramente determinista hacia `SovereignDecision(0|1)` que sustituya soberanamente aquella U.

### IU3 — no derogación

Ninguna transición gubernativa ni revisión constitucional puede producir una configuración en la que IU1 o IU2 dejen de cumplirse.

### IU4 — no escritura informativa de autoridad

Toda regla Informational preserva `C` y `A` y no introduce tipos sensibles.

### IU5 — capacidad lineal y ligada a época

Toda capacidad es de un solo uso y sólo es válida bajo el mismo par `(v,g)` en que fue acuñada.

### IU6 — historia semánticamente estable

Toda traza bien formada preserva por prefijo los registros históricos y su denotación original.

## 10. Extensión conservativa de la frontera IA preexistente

Sea `B0` el conjunto de restricciones arquitectónicas anteriores relevantes:

1. IA auxiliar no soberana.
2. No cierre automático de U por plausibilidad/inferencia opaca.
3. No modificación automática ilegítima de diseño/políticas.
4. Separación REAL/SIM.
5. Historia append-only.
6. No bypass de custodia estructural.
7. Capas auxiliares subordinadas a álgebra, lenguaje y humano competente.

Una configuración/regla AUTH sólo es admisible si:

`WF_AUTH(K) => Preserves(B0)`.

En particular, ninguna regla AUTH puede conferir a una capa externa un camino de autoridad que la arquitectura anterior prohibía.

## 11. Resultados a demostrar tras esta clausura

### L1 — Effect Separation

Para toda regla Informational, `Write(alpha) ⊆ {I,Hist}` y `Intro(alpha)` no contiene tipos sensibles. Para toda regla ordinaria no gubernativa ni constitucional, `A` y `C` permanecen invariantes.

### L2 — Token Provenance and Confinement

Toda capacidad viva posee una cadena de procedencia hasta un grant vigente y no puede ejercerse fuera de la tupla autorizada por dicho grant ni fuera de `(v,g)`.

### L3 — Semantic History Preservation

Para toda traza bien formada, la historia crece por prefijo y todo registro previo conserva su denotación bajo su `SemanticEnvironmentRef` original.

### P1 — Resolver/Authority Independence

La equivalencia de resultado/resolución y la equivalencia de autoridad son independientes. Deben existir testigos constructivos en ambas direcciones.

### TA — Authority Non-Escalation Under Ordinary Traces

Para toda traza ordinaria bien formada sin transición gubernativa ni constitucional:

`AEnv(K_n) = AEnv(K_0)`.

La información puede cambiar `EnabledAuth` sin crear ni ampliar `AEnv`.

### TB — Sovereign Non-Derivability Without an Admitted Human Act

En una traza que no contiene el juicio humano admitido correspondiente, ninguna secuencia de pasos informativos, deterministas o de actores externos puede introducir `SovereignDecision(U)` ni producir la resolución soberana de una `SovereignDecision(U)` existente.

## 12. Objeción RBAC/ABAC/capabilities

SV-AUTH no reclama novedad para roles, grants, tokens, capacidades lineales o control de acceso en sí mismos.

La cuestión formal es distinta: distinguir la influencia informativa que puede determinar completamente el contenido de una decisión de la autoridad persistente que permite comprometer el estado, y demostrar que sustituciones informativas admitidas y trazas ordinarias no escalan esa autoridad.

La comparación con RBAC, ABAC, object-capability, usage control y lógicas de autorización debe hacerse en el estado del arte antes de cualquier afirmación de novedad.

## 13. Criterio de no trivialidad y conformidad

La conformidad no se acredita con una suite de rechazo solamente.

Debe incluir:

- AES: trazas prohibidas que deben ser rechazadas.
- P1 Witness Suite: dos testigos de independencia resolutiva/autoridad.
- LAS: automatizaciones legítimas que deben ser admitidas.

Un sistema que rechaza todas las operaciones falla LAS y no es conforme.

## 14. Amenazas y límites del modelo

SV-AUTH no pretende probar:

- identidad física del humano;
- seguridad criptográfica;
- integridad de SO/hardware;
- resistencia a modificación arbitraria del runtime confiable;
- honestidad o competencia humana;
- verdad externa absoluta de una evidencia.

El resultado se formula bajo un límite de ejecución confiable declarado y actos externos admitidos por procedimientos del dominio.

## 15. Estado de A.2

Este documento cierra la especificación operacional candidata, pero no certifica aún los resultados L1–L3, P1, TA o TB.

Cierre efectivo de A.2 requiere:

1. auditoría interna contra la tabla de reglas;
2. ejecución de AES/P1/LAS sobre una realización de referencia;
3. ausencia de contraejemplos no cubiertos;
4. corrección de la teoría si algún caso falla.

Sólo entonces podrá declararse A.2 cerrada y comenzar la integración estable J6/E6xx.
