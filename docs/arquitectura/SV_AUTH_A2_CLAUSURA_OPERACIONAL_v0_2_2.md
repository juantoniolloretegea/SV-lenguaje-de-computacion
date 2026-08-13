# SV-AUTH v0.2.2 — A.2 Clausura operacional revisada

## Estatuto

Documento de trabajo de la rama `sv-auth-v0.2`. Revisa la primera clausura A.2 después de la auditoría spec ↔ runtime del 13/08/2026. No modifica retrospectivamente la Frontera normativa v0, la IR canónica v0.2 ni el motor publicado.

La primera clausura y el artefacto 36/36 se conservan por trazabilidad. Esta revisión corrige las divergencias documentadas en `SV_AUTH_A2_AUDITORIA_SPEC_RUNTIME_2026_08_13.md`.

## 1. Genealogía y frontera de no contaminación

SV-AUTH no funda la subordinación de IA. Formaliza una frontera anterior distribuida entre:

- Frontera normativa v0: `Tri` irreductible, `Res` trazable, D.10, IA no soberana;
- Manifiesto V1: capas auxiliares producen observables/proxies y no decisiones soberanas;
- Nota de capa IA trazable: primacía humana, REAL/SIM, append-only, no fabricación de certeza y no bypass de custodia.

AUTH añade tipos, estados, capacidades y teoremas de autoridad. No retroproyecta estos objetos sobre documentos anteriores.

## 2. Estado extendido

La configuración de autoridad es:

`K = (C^v, I, A^g, Tok, Hist)`.

- `C^v`: Constitución de autoridad, roles y verificadores declarados.
- `I`: información ordinaria, certificados crudos y certificados verificados.
- `A^g`: principals/bindings/grants persistentes de la época `g`.
- `Tok`: capacidades efímeras y lineales.
- `Hist`: ledger append-only y registro de entornos semánticos.

Para razonar sobre conservación de la arquitectura anterior se usa, cuando haga falta, un estado basal opaco `S_B` del SV y el estado producto:

`S* = (S_B, K)`.

Las reglas AUTH puras no escriben `S_B`; producen autoridad/decisiones en su propia capa. La aplicación posterior de una decisión al estado basal pertenece a las reglas ya existentes del SV y queda fuera de AUTH A.2.

## 3. Tipos y roles

Tipos nominalmente distintos:

- `AuthorityRole`;
- `PrincipalBinding`;
- `Grant`;
- `DetToken`;
- `HumanToken`;
- `GovernanceToken`;
- `HumanAuthorizationAct`;
- `GovernanceAuthorizationAct`;
- `SovereignDecision`.

Clases mínimas de principal de la realización de referencia:

- `Human`;
- `Service`;
- `External`;
- `Governance`.

Bindings bien formados:

- `human-sovereign` sólo puede ligarse a principal `Human`;
- `governance` sólo a principal `Governance`;
- `det-service` a principal `Service` o `External` explícitamente admitido por gobierno.

`ResolutionRecord` de la IR v0.2 no es `SovereignDecision` y no existe coerción entre ellos.

`SovereignU(d)` abrevia `d : SovereignDecision` y `d.value = U`; no añade un cuarto valor a `Σ={0,1,U}`.

## 4. AEnv, habilitación informativa y ejercicio

### 4.1. Envolvente persistente

`AEnv(C^v,A^g)` es el conjunto de `Grant` que satisfacen `ValidGrant(C^v,A^g,G)`.

`ValidGrant` exige, al menos:

1. grant único y existente;
2. principal ligado a un `AuthorityRole` compatible con `AuthorityKind`;
3. binding bien formado;
4. operación/scope/objeto dentro del grant;
5. ninguna operación soberana (`commit_sov_u`, `resolve_sov_u`) en grant no humano;
6. si el grant exige base verificada, resolvedor y verificador declarados compatibles.

### 4.2. Habilitación por información

`IEnabledAuth(K,c,zeta)` contiene grants/operaciones de `AEnv` cuyas precondiciones informativas son verdaderas para candidato `c` y contexto `zeta`.

La información puede cambiar `IEnabledAuth`; no puede crear ni ampliar `AEnv`.

La pertenencia a `IEnabledAuth` no equivale a disponer de una capacidad viva ni a poder ejecutar un commit protegido.

### 4.3. Ejercicio

El ejercicio requiere una capacidad tipada viva, ligada a principal y contexto. Para operaciones humanas o gubernativas se exige además que la capacidad proceda de un acto de frontera admitido.

## 5. Base certificada

Se distinguen:

- `RawCertificate`;
- `VerifiedCertificate`.

`VerifiedCertificate` sólo puede introducirse por `VERIFY` bajo un `VerifierSpec` declarado y un resultado admitido por el límite de verificación del dominio.

`INFO` no puede inyectar directamente un `VerifiedCertificate`.

La teoría no demuestra la corrección física o algorítmica del verificador. Demuestra que el resto de AUTH no puede sustituir el juicio de verificación por construcción ordinaria de datos.

## 6. Fronteras externas primitivas

AUTH modela cuatro admisiones externas sin pretender autenticarlas internamente:

1. `Gamma_V |- q : VerifierAdmission` — resultado admitido del verificador declarado.
2. `Gamma_H |- a : HumanAuthorizationAct` — acto humano soberano admitido.
3. `Gamma_G |- g : GovernanceAuthorizationAct` — intervención gubernativa/humana declarada para cambios de autoridad/diseño.
4. `Gamma_S |- subject` — sujeto del `UseContext` admitido por el límite de ejecución.

Además, `RESTORE` exige una instantánea admitida por un límite de restauración confiable.

Ninguna regla computacional ordinaria produce estas premisas.

## 7. HumanAuthorizationAct

Un acto humano queda ligado, como mínimo, a:

`(act_id, grant_id, principal, operation, scope, object, candidate, certificate_ref, state_ref, resolver, prior_decision_ref?, v, g)`.

La admisión exige:

- principal ligado a `human-sovereign`;
- principal del acto = principal del grant;
- grant humano vigente;
- operación/scope/objeto/candidato dentro del grant;
- base verificada coincidente cuando el grant la exige;
- `(v,g)` vigentes;
- `act_id` fresco.

Un acto admite como máximo una acuñación de `HumanToken`.

`HumanReviewRequest` no es acto humano y no se convierte en token.

## 8. GovernanceAuthorizationAct

D.10 exige intervención humana declarada antes de modificar diseño/políticas. Por ello el grant gubernativo persistente no basta por sí solo para acuñar `GovernanceToken`.

Un acto gubernativo queda ligado a:

`(act_id, grant_id, principal, operation, scope, object, state_ref, v, g)`.

Debe ser admitido por `Gamma_G`, coincidir con un grant gubernativo vigente y sólo puede acuñar una capacidad.

## 9. Tokens

Tipos nominales:

- `DetToken`;
- `HumanToken`;
- `GovernanceToken`.

Toda capacidad liga:

`(token_id, grant_id, principal, operation, scope, object, candidate?, certificate_ref?, state_ref, resolver?, prior_decision_ref?, v, g)`.

Propiedades:

- sólo constructores sellados;
- un solo uso;
- no amplificación respecto del grant;
- principal/scope/objeto/candidato/base/estado/procedimiento ligados;
- `prior_decision_ref` obligatorio para resolución soberana;
- obsolescencia exacta si cambia `v` o `g`;
- la capacidad presentada debe coincidir con la capacidad viva almacenada;
- no existe constructor operacional por deserialización.

## 10. UseContext

`UseContext = (subject, executor, operation, scope, object, state, subject_admitted)`.

Se exige:

- `subject_admitted = true` bajo `Gamma_S`;
- `subject = principal(token)`;
- operación/scope/objeto/estado coinciden con el token.

No se exige `executor = principal(token)`. Un runtime confiable puede ejecutar materialmente una acción autorizada por otro principal sin adquirir su autoridad.

## 11. Historia semántica

Cada registro histórico posee `SemanticEnvironmentRef` inmutable.

`Denote(r) = [[payload(r)]]_{SemEnv(r)}`.

No existe regla que modifique `payload(r)` o `SemanticEnvironmentRef(r)` de un registro pasado.

`RESTORE`:

- requiere snapshot admitido por frontera confiable;
- verifica referencias semánticas;
- no introduce decisiones nuevas;
- elimina todos los tokens efímeros.

Un `SovereignDecision(U)` usado como antecedente de resolución debe existir exactamente en `Hist` y coincidir con `prior_decision_ref` y objeto del token.

## 12. Firmas de efectos

Para cada regla `alpha` se declaran `Read(alpha)`, `Write(alpha)` e `Intro(alpha)`.

### 12.1. Reglas sensibles selladas

| Regla | Clase | Lee | Escribe | Introduce / efecto | Premisas mínimas |
|---|---|---|---|---|---|
| `INFO` | Informational | I | I | dato ordinario | no tipo sensible ni certificado verificado |
| `VERIFY` | Informational | C,I | I | `VerifiedCertificate` | verificador declarado + `Gamma_V` |
| `REQUEST_HUMAN` | Informational | I | I | `HumanReviewRequest` | entrada tipada |
| `ADMIT_HUMAN_ACT` | Boundary | C,A,I,Hist | Hist | acto humano admitido | `Gamma_H`; grant/principal/base/(v,g) coherentes |
| `MINT_DET` | Capability | C,A,I,Tok,Hist | Tok,Hist | `DetToken` | grant Det vigente + base verificada + binding completo |
| `MINT_HUMAN` | Capability | C,A,I,Tok,Hist | Tok,Hist | `HumanToken` | acto humano admitido, fresco y coincidente |
| `ADMIT_GOV_ACT` | Boundary | C,A,Hist | Hist | acto gubernativo admitido | `Gamma_G`; grant/principal/(v,g) coherentes |
| `MINT_GOV` | Capability | C,A,Tok,Hist | Tok,Hist | `GovernanceToken` | acto gubernativo admitido y fresco |
| `COMMIT_DET` | Commit | C,A,I,Tok,Hist | Tok,Hist | `CommittedDecision` | DetToken vivo + `Gamma_S` + base verificada |
| `COMMIT_SOV_U` | Commit | C,A,I,Tok,Hist | Tok,Hist | `SovereignDecision(U)` | HumanToken vivo + certificado de no clausura + `Gamma_S` |
| `RESOLVE_SOV_U` | Commit | C,A,I,Tok,Hist | Tok,Hist | `SovereignDecision(0|1)` | U histórica ligada + nuevo HumanToken + nueva base + `Gamma_S` |
| `GOV_BIND` | Governance | C,A,Tok,Hist | A,Tok,Hist | binding nuevo | GovernanceToken `bind_principal`; resultado `WF_AUTH` |
| `GOV_GRANT` | Governance | C,A,Tok,Hist | A,Tok,Hist | grant nuevo | GovernanceToken `grant_add`; grant único y `WF_AUTH` |
| `CONSTITUTION_REVISION` | Constitutional | C,A,Tok,Hist | C,Tok,Hist | nueva versión C | GovernanceToken dedicado; resultado `WF_AUTH` |
| `RESTORE` | Initialization | C,I,A,Hist snapshot | C,I,A,Hist,Tok | reconstitución; `Tok:=empty` | snapshot admitido y semánticamente válido |

No existe otro introductor de `VerifiedCertificate`, tokens o `SovereignDecision`.

Una futura regla sensible exige nueva versión de esta tabla y nueva prueba de `WF_AUTH`, TA, TB y conservación.

## 13. WF_AUTH e invariantes

### IU1 — Sovereign-U constructor isolation

Ninguna autoridad puramente determinista introduce `SovereignDecision(U)`.

### IU2 — Sovereign-U resolution isolation

Ninguna autoridad puramente determinista resuelve soberanamente una U existente.

### IU3 — Non-derogation

Gobierno o revisión constitucional no pueden producir un estado donde IU1/IU2 fallen.

### IU4 — Informational non-write

Reglas informativas preservan `C` y `A` y no introducen tipos de autoridad.

### IU5 — Linear epoch-bound capabilities

Tokens de un solo uso, ligados al par exacto `(v,g)` y a la capacidad viva almacenada.

### IU6 — Semantic history preservation

Historia por prefijo y denotación histórica estable.

### IU7 — Certified-basis provenance

`VerifiedCertificate` sólo por `VERIFY` + verificador declarado + `Gamma_V`.

### IU8 — Governance intervention

No existe `GovernanceToken` sin `GovernanceAuthorizationAct` admitido y fresco.

### IU9 — Subject binding

No existe uso de token sin sujeto admitido y coincidente con su principal.

### IU10 — Sovereign lineage

`RESOLVE_SOV_U` sólo actúa sobre una `SovereignDecision(U)` histórica exactamente ligada por referencia y objeto.

## 14. Conservación de la frontera anterior

Sea `B_auth`:

1. IA auxiliar no soberana;
2. no cierre soberano automático de U;
3. no mutación automática de autoridad/diseño sin intervención declarada;
4. historia append-only.

Estas cláusulas están representadas directamente por IU1–IU10 y son objeto de prueba formal A.2.

Sea `B_orth`:

- separación REAL/SIM;
- no bypass de custodia estructural;
- precedencia de capas del SV.

AUTH A.2 no internaliza esos estados. En el producto `S*=(S_B,K)`, toda transición AUTH pura preserva `S_B`. Por tanto no reescribe por sí misma `B_orth`. La integración J6/runtime tendrá una obligación adicional de refinamiento: ningún handoff AUTH → SV basal podrá saltar las reglas existentes de custodia, REAL/SIM o precedencia.

No se declarará conservación del sistema integrado hasta comprobar esa obligación.

## 15. Resultados nucleares

### L1 — Effect Separation

Información no escribe `C/A`; capacidades y commits no escriben `C/A`; sólo reglas gubernativas/constitucionales autorizadas pueden hacerlo.

### L2 — Capability Provenance and Confinement

Toda capacidad viva deriva de un grant válido y no puede ejercerse fuera de su binding, época ni principal admitido.

### L3 — Semantic History Preservation

Toda traza bien formada preserva prefijo y denotación histórica.

### P1 — Resolver/Authority Independence

Equivalencia resolutiva declarada y equivalencia de perfiles de autoridad son independientes. La realización de referencia sólo proporciona una instancia por firmas; no sustituye la equivalencia contextual IJGS completa.

### TA — Authority Non-Escalation Under Ordinary Traces

Para toda traza operacional bien formada sin `GOV_BIND`, `GOV_GRANT` ni `CONSTITUTION_REVISION`:

`AEnv(K_n) = AEnv(K_0)`.

Puede cambiar `IEnabledAuth`.

### TB — Sovereign Non-Derivability Without Human Admission

Sin el `HumanAuthorizationAct` admitido correspondiente no existe derivación interna hacia `SovereignDecision(U)` ni hacia resolución soberana de una U histórica.

## 16. Objeción RBAC/ABAC/capabilities

AUTH no reclama novedad para roles, grants, control de acceso o capacidades en sí mismos.

La cuestión a contrastar externamente es si existe ya una construcción que combine de la misma manera:

- separación de influencia informativa y fuente persistente de autoridad;
- habilitación informativa sin escalada de `AEnv`;
- independencia de equivalencia resolutiva y equivalencia de autoridad;
- no derivabilidad de commits soberanos bajo trazas;
- conservación de una frontera previa de subordinación de IA.

No se formulará novedad hasta completar el estado del arte RBAC/ABAC/object-capability/usage-control/autorización formal.

## 17. Modelo de amenazas

El adversario puede controlar salidas informativas de LLM/CNN/sensores, producir consenso, igualar la respuesta humana e intentar replay, confused-deputy, import o deserialización.

No se prueba:

- identidad física;
- autenticación/criptografía;
- integridad de SO/hardware;
- seguridad frente a modificación arbitraria del runtime confiable;
- corrección absoluta del verificador;
- honestidad o competencia humanas.

Estos elementos comparecen como fronteras admitidas.

## 18. Criterio de conformidad

Una realización no es conforme sólo porque rechace ataques.

Debe superar:

- AES — exclusiones;
- P1 Witness Suite — independencia;
- LAS — automatización legítima;
- pruebas de historia, frontera, gobierno y restauración.

Un sistema reject-all falla LAS.

## 19. Estado

Esta revisión v0.2.2 incorpora los defectos encontrados tras el artefacto 36/36. Existe una realización local correspondiente con 74/74 pruebas y 96 % de cobertura, pero todavía debe someterse a auditoría externa y no está integrada en parser/validator ni en `SV-motor` público.

J6/E6xx permanece bloqueado hasta cerrar correspondencia spec v0.2.2 ↔ realización revisada.
