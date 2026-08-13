# SV-AUTH — Auditoría spec ↔ runtime de referencia

**Fecha:** 13/08/2026  
**Rama:** `sv-auth-v0.2`  
**Objeto:** contraste entre `SV_AUTH_A2_CLAUSURA_OPERACIONAL_v0_2.md`, `SV_AUTH_A2_PRUEBAS_v0_2.md` y el artefacto ejecutable `SV_AUTH_v0_2_A2_reference_execution_20260813.zip`.

## 1. Resultado ejecutivo

La primera realización de referencia fue reproducible y pasó 36/36 pruebas. Ese dato es válido respecto de aquella batería. Sin embargo, la auditoría cruzada posterior encontró divergencias reales entre la especificación A.2 y el runtime. Por tanto, el resultado 36/36 no debe usarse como certificado de conformidad completa spec ↔ runtime.

La corrección se ha realizado sin modificar `main` y sin retroproyectar AUTH sobre la IR v0.2 publicada.

Una segunda realización local, derivada de esta auditoría, pasa 74/74 pruebas con 96 % de cobertura conjunta de `authority_types` y `authority_runtime`. Ese resultado sigue siendo evidencia de referencia; no equivale todavía a integración J6/E6xx ni a `SV-motor implements SV-AUTH`.

## 2. Divergencias encontradas en la primera realización

### D1 — Proveniencia de `VerifiedCertificate`

La especificación exigía `VERIFY` mediante un verificador declarado. El prototipo permitía construir `VerifiedCertificate` a partir de un diccionario y, además, el tipo podía introducirse sin una barrera operacional suficientemente fuerte.

**Riesgo:** convertir una afirmación informativa en “base verificada” por mera construcción de datos.

**Corrección:** separación `RawCertificate` / `VerifiedCertificate`; constructor operacional único `VERIFY`; verificador declarado; admisión explícita de frontera del resultado del verificador; inyección directa de `VerifiedCertificate` por `INFO` prohibida.

### D2 — Acto humano no ligado al principal del grant

La primera `mint_human` comparaba operación, scope, objeto, candidato y estado, pero no exigía que `act.principal == grant.principal`.

**Riesgo:** acto de un humano A utilizado para acuñar capacidad bajo el grant de B.

**Corrección:** `HumanAuthorizationAct` queda ligado a `grant_id` y principal; la admisión verifica rol humano soberano y coincidencia exacta principal/grant.

### D3 — Acto humano no ligado a base/resolvedor

El acto humano no transportaba `certificate_id` ni resolvedor.

**Riesgo:** autorización humana reutilizada con una base distinta de la examinada.

**Corrección:** acto y token quedan ligados a certificado/base, resolvedor, estado y, para resolución soberana, al `prior_decision_ref`.

### D4 — `Authorizes_G` demasiado débil en runtime

El primer `Grant.authorizes` sólo comprobaba operación, scope, objeto y resolvedor.

**Riesgo:** el grant podía ser más permisivo que la premisa formal usada en L2.

**Corrección:** el grant declara política de candidatos y exigencia de base verificada; la referencia evalúa candidato, resolvedor y presencia de certificado verificado. El token sigue fijando además el estado concreto.

### D5 — Tokens no nominalmente separados y posible forgery por mismo `token_id`

La primera referencia usaba un único `Token(kind=...)`, y `_consume` resolvía el token vivo sólo por `token_id`.

**Riesgo:** presentar un objeto modificado con el mismo identificador que una capacidad viva.

**Corrección:** `DetToken`, `HumanToken` y `GovernanceToken` son tipos distintos; la capacidad presentada debe coincidir exactamente con la almacenada; deserialización operacional continúa prohibida.

### D6 — Scope de uso no contrastado y sujeto sólo implícitamente confiable

`UseContext` no contenía scope y la autenticación del `subject` quedaba implícita.

**Corrección:** `UseContext` incluye scope y un hecho de admisión de frontera. La teoría no prueba identidad física; exige que el límite de ejecución haya admitido el sujeto antes del uso.

### D7 — Resolución de `SovereignU` sin linaje histórico suficiente

La primera `resolve_sovereign_u` sólo exigía `prior.is_u`; no comprobaba que aquella U existiese en la historia ni que el token estuviera ligado a esa decisión concreta.

**Riesgo:** fabricar localmente una U previa y presentarla como antecedente.

**Corrección:** `prior_decision_ref` en acto/token, identidad de objeto y presencia exacta del `SovereignDecision(U)` en historia.

### D8 — `AEnv` ejecutable no correspondía al formal

La primera referencia reducía `AEnv` a identificadores de grants presentes en `AuthorityState`; no evaluaba Constitución ni bindings.

**Corrección:** `AEnv` es ahora conjunto de grants válidos bajo `(C^v,A^g)`, con `PrincipalBinding`, `AuthorityRole`, clase de principal y restricciones IU.

### D9 — `EnabledAuth` no estaba realmente ejecutado

LA01 sólo mostraba que añadir información no cambiaba grants.

**Corrección:** existe consulta ejecutable de autoridad informativamente habilitada. Se comprueba que una base verificada puede habilitar un grant ya existente sin alterar `AEnv`.

### D10 — Gobierno sin acto gubernativo admitido

El primer `MINT_GOV` podía acuñarse desde un grant gubernativo sin representar la intervención humana declarada exigida por la Frontera D.10.

**Riesgo:** AUTH habría debilitado la frontera preexistente que pretendía conservar.

**Corrección:** `GovernanceAuthorizationAct` admitido por frontera, no reutilizable, y ligado a grant/operación/scope/objeto/estado. Sin ese acto no existe `GovernanceToken`.

### D11 — Restauración confiable sólo declarada, no exigida

`RESTORE` recibía una configuración y eliminaba tokens, pero no exigía admisión del snapshot ni validaba referencias semánticas.

**Corrección:** admisión explícita de snapshot confiable, `SemanticEnvironment` registrado, verificación de referencias y eliminación de capacidades efímeras.

### D12 — P1 ejecutable demasiado débil

`resolver_equivalent` era igualdad de strings de candidato.

**Corrección:** la referencia usa una firma de resolvedor/contexto. Sigue siendo una instancia mínima y no debe presentarse como implementación completa de la equivalencia contextual IJGS.

### D13 — `CONSTITUTION_REVISION` estaba en la spec pero no en el runtime

**Corrección:** operación de referencia añadida, con incremento exacto de versión, preservación de roles necesarios, compatibilidad de grants y obsolescencia de capacidades anteriores por `(v,g)`.

### D14 — C1 sobre `B0` necesitaba distinguir lo modelado de lo ortogonal

El primer texto presentaba como un solo teorema la conservación de cláusulas directamente representadas en AUTH y otras como REAL/SIM o custodia estructural que no son estado interno de AUTH.

**Corrección conceptual:** separar:

- restricciones `B_auth` formalmente representadas: no soberanía IA, no cierre soberano automático de U, no mutación automática de autoridad/diseño, historia append-only;
- restricciones ortogonales del SV (`REAL/SIM`, custodia, precedencia de capas), preservadas por no escritura de AUTH sobre el estado basal y sujetas a obligación de refinamiento en la integración.

La afirmación fuerte sobre el sistema integrado no se certificará hasta J6/runtime estable.

## 3. Resultado de la segunda realización local

Batería local posterior a las correcciones anteriores:

- **74 passed, 0 failed**;
- **525 statements**;
- **96 % de cobertura total**;
- `authority_runtime`: 95 %;
- `authority_types`: 98 %.

La batería incluye AES original, ataques adicionales derivados de D1–D14, testigos P1, LAS ampliada, TA/TB y comprobaciones de historia/restauración.

## 4. Estado científico tras la auditoría

- El artefacto 36/36 permanece como fósil reproducible de la primera realización.
- No se usa como conformidad completa de A.2.
- A.2 debe revisarse a una versión de trabajo posterior que incorpore D1–D14.
- J6/E6xx permanece bloqueado hasta que la spec revisada y la segunda realización sean auditadas como correspondientes.
- `main` de lenguaje y motor permanece fuera de estas modificaciones.

## 5. Regla para TAI

No se afirmará:

- que `SV-motor` implementa AUTH antes de integración;
- que una batería de tests demuestra por sí sola los teoremas;
- que la equivalencia resolutiva de referencia es ya la equivalencia contextual IJGS completa;
- ni que AUTH prueba autenticación humana, seguridad criptográfica o integridad de un runtime hostil.

La formulación admisible, cuando spec ↔ runtime vuelva a cerrar, será distinguir explícitamente:

1. propiedades semánticas demostradas bajo reglas selladas;
2. realización de referencia conforme a esas reglas;
3. evidencia ejecutable AES/P1/LAS;
4. integración posterior en lenguaje y motor.
