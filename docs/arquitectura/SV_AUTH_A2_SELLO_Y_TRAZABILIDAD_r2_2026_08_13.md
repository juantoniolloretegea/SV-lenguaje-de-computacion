# SV-AUTH A.2 — Sello y trazabilidad de la realización r2

**Fecha:** 13/08/2026  
**Rama:** `sv-auth-v0.2`  
**Especificación base:** `SV_AUTH_A2_CLAUSURA_OPERACIONAL_v0_2_2.md`  
**Pruebas semánticas base:** `SV_AUTH_A2_PRUEBAS_v0_2_2.md`  
**Auditoría histórica:** `SV_AUTH_A2_AUDITORIA_SPEC_RUNTIME_2026_08_13.md`

## 1. Objeto del sello

Este documento cierra administrativamente A.2 después de la auditoría adversarial de la especificación v0.2.2 y de la realización de referencia r2. No modifica `main`, no declara integración J6/E6xx y no afirma que `SV-motor` publicado implemente SV-AUTH.

Los artefactos 36/36 y 74/74 se conservan como estados históricos intermedios. Ninguno de ellos es el artefacto sellado de A.2.

## 2. Artefacto r2 fijado

Artefacto: `SV_AUTH_A2_revised_reference_r2_20260813.zip`

SHA-256:

`7c18761cf5546c8fdd9ad962c0ea3e0a54a9ddd4a4bf6d43c0ab29c7e4cf794f`

Ejecución reproducida sobre el contenido exacto del ZIP:

- `78 passed, 0 failed`;
- `537` sentencias cubiertas por el informe de cobertura;
- `96 %` de cobertura global;
- `authority_runtime.py`: `94 %`;
- `authority_types.py`: `98 %`.

La batería contiene exactamente 78 funciones `test_*` en `tests/test_authority.py`.

## 3. Enmienda normativa de precondición de uso

Para evitar que la igualdad `subject = principal(token)` quede únicamente en la prosa de `UseContext`, este sello sustituye normativamente las premisas mínimas de las tres reglas de commit/resolución de la tabla 12.1 por las siguientes:

| Regla | Premisas mínimas selladas |
|---|---|
| `COMMIT_DET` | `DetToken` vivo; `Gamma_S`; **`subject = principal(token)`**; operación/scope/objeto/estado coincidentes; candidato/base/resolvedor ligados; base verificada cuando el grant la exige; `(v,g)` vigentes |
| `COMMIT_SOV_U` | `HumanToken` vivo; `Gamma_S`; **`subject = principal(token)`**; operación/scope/objeto/estado coincidentes; candidato `U`; certificado de no clausura verificado y ligado; `(v,g)` vigentes |
| `RESOLVE_SOV_U` | `HumanToken` vivo; `Gamma_S`; **`subject = principal(token)`**; operación/scope/objeto/estado coincidentes; `SovereignDecision(U)` histórica exactamente ligada por `prior_decision_ref`; nueva base verificada; `(v,g)` vigentes |

No existe camino válido en A.2 en el que `subject != principal(token)` y una de estas reglas concluya con éxito.

## 4. Matriz regla sellada ↔ batería r2

La matriz siguiente comprueba que cada una de las 15 reglas sensibles de la tabla 12.1 tiene ejercicio directo en la batería r2. Se listan tests representativos; algunos tests ejercen varias reglas.

| Regla | Tests representativos del r2 | Nº de tests mapeados |
|---|---|---:|
| `INFO` | `test_ce01_information_cannot_create_grant`; `test_ce01b_information_cannot_inject_verified_certificate`; `test_ce05_informational_binding_cannot_change_aenv`; `test_la03_local_u_can_be_replaced_by_new_information_without_human_token`; `test_ta_ordinary_trace_preserves_aenv` | 5 |
| `VERIFY` | `test_ce01c_unknown_verifier_cannot_verify`; `test_ce01d_nonclosure_requires_nonclosure_verifier`; `test_ax07_verifier_boundary_is_explicit`; `test_b01_verifier_resolver_mismatch_rejected`; `test_b02_duplicate_verified_certificate_id_rejected`; `test_ax11_nonclosure_certificate_must_target_u` | 6 |
| `REQUEST_HUMAN` | `test_ce21_review_request_does_not_mint_human_token`; `test_la04_human_review_request_is_legitimate_and_non_authorizing` | 2 |
| `ADMIT_HUMAN_ACT` | `test_ce10_human_act_cannot_be_replayed_for_second_mint`; `test_ax01_human_act_principal_must_match_grant`; `test_ax02_human_act_basis_is_bound`; `test_b04_human_act_boundary_rejection`; `test_b05_human_act_stale_and_duplicate_admission_rejected`; `test_b06_human_act_missing_prior_for_resolution_rejected`; `test_b13_sovereign_u_authorization_requires_nonclosure_basis`; `test_ax12_human_u_act_requires_nonclosure_certificate_before_mint` | 8 |
| `MINT_DET` | `test_ce02_scope_cannot_exceed_grant`; `test_ce04_grant_candidate_policy_is_enforced`; `test_ce04b_forged_certificate_dataclass_is_not_verified_basis`; `test_b03_invalid_grant_is_outside_aenv_and_cannot_mint`; `test_b09_mint_kind_escalation_and_duplicate_token_id_rejected`; `test_ta_ordinary_trace_preserves_aenv` | 6 |
| `MINT_HUMAN` | `test_ce10_human_act_cannot_be_replayed_for_second_mint`; `test_ce13_det_grant_cannot_mint_human_token`; `test_la05_human_act_can_mint_scoped_token_and_commit_sovereign_u`; `test_la07_existing_sovereign_u_can_be_resolved_with_fresh_human_act`; `test_b09_mint_kind_escalation_and_duplicate_token_id_rejected`; `test_tb_no_human_act_no_sovereign_u_path` | 6 |
| `ADMIT_GOV_ACT` | `test_b07_governance_boundary_stale_replay_and_mismatch_rejected`; `test_b08_governance_act_outside_grant_rejected`; `test_b10_governance_mint_act_binding_mismatch_rejected` | 3 |
| `MINT_GOV` | `test_ax08_governance_token_requires_admitted_governance_act`; `test_b10_governance_mint_act_binding_mismatch_rejected`; `test_la06_governance_channel_can_bind_external_service_and_add_grant` | 3 |
| `COMMIT_DET` | `test_ce03_subject_cannot_present_anothers_token`; `test_ce06_unverified_commit_basis_rejected`; `test_ce07_token_candidate_cannot_be_reused_for_other_candidate`; `test_ce08a_constitution_version_stales_token`; `test_ce08b_authority_epoch_stales_token`; `test_ce11c_same_id_modified_token_is_forgery`; `test_ce17_scope_and_object_binding_cannot_move`; `test_ax03_use_scope_is_bound`; `test_ax09_execution_subject_requires_boundary_admission`; `test_b11_consumed_token_cannot_be_reused`; `test_b12_commit_det_requires_det_commit_operation`; `test_la02_external_data_can_determine_and_authorized_service_commit` | 12 |
| `COMMIT_SOV_U` | `test_ce15_det_token_cannot_commit_sovereign_u`; `test_la05_human_act_can_mint_scoped_token_and_commit_sovereign_u`; `test_b13_sovereign_u_authorization_requires_nonclosure_basis`; `test_ax12_human_u_act_requires_nonclosure_certificate_before_mint` | 4 |
| `RESOLVE_SOV_U` | `test_ce16_det_token_cannot_resolve_sovereign_u`; `test_ax04_fabricated_prior_sovereign_u_cannot_be_resolved`; `test_la07_existing_sovereign_u_can_be_resolved_with_fresh_human_act`; `test_b14_resolution_rejects_non_u_prior_and_wrong_reference_or_candidate` | 4 |
| `GOV_BIND` | `test_ce14b_governance_cannot_bind_external_as_human_sovereign`; `test_la06_governance_channel_can_bind_external_service_and_add_grant` | 2 |
| `GOV_GRANT` | `test_ce14_governance_cannot_make_det_sovereign_grant`; `test_ce14c_governance_operation_mismatch_cannot_add_grant`; `test_ax05_duplicate_grant_id_rejected`; `test_la06_governance_channel_can_bind_external_service_and_add_grant` | 4 |
| `CONSTITUTION_REVISION` | `test_ax06_constitution_cannot_remove_required_authority_role`; `test_la08_constitution_revision_preserves_invariants_and_stales_old_tokens`; `test_b15_constitution_revision_version_and_grant_compatibility_checked` | 3 |
| `RESTORE` | `test_ce12c_restore_rejects_missing_semantic_environment`; `test_restore_drops_live_tokens_and_preserves_history`; `test_ax10_restore_requires_trusted_snapshot_admission` | 3 |

Resultado del control de matriz: **15/15 reglas selladas poseen ejercicio directo; 0 filas sin cobertura de prueba asociada.**

La matriz no convierte los tests en demostraciones. L1–L3, P1, TA, TB, C1/C2 e IS siguen siendo resultados de la semántica sellada; la batería aporta evidencia de conformidad de la realización r2 con esa semántica.

## 5. Trazabilidad 36/36 → 74/74 → 78/78

### 5.1. 36/36

La primera realización reproducible ejercía la arquitectura básica, pero la auditoría spec↔runtime detectó D1–D14: procedencia insuficiente de certificados verificados, binding humano incompleto, linaje soberano insuficiente, `AEnv` y habilitación demasiado débiles, tipos de token no suficientemente nominales, frontera gubernativa incompleta, restauración insuficientemente admitida, P1 ejecutable demasiado pobre y ausencia de `CONSTITUTION_REVISION`, entre otros.

Ese artefacto se conserva como fósil y **no** como conformidad A.2.

### 5.2. 74/74

La segunda realización local cerró D1–D14 e introdujo las barreras correspondientes. Fue un estado intermedio de endurecimiento y explica las cifras 74/74 que permanecen en documentos históricos anteriores.

### 5.3. 78/78 — r2 sellado

El r2 añadió cuatro ataques/regresiones finales:

- `test_ax11_nonclosure_certificate_must_target_u`;
- `test_ax12_human_u_act_requires_nonclosure_certificate_before_mint`;
- `test_ax13_ienabled_sov_u_requires_certified_nonclosure`;
- `test_ax14_wf_auth_rejects_history_cache_divergence`.

Estas cuatro pruebas fijan que la base de no clausura se refiere efectivamente a `U`, que debe existir antes de admitir el acto humano de constitución soberana, que la habilitación informativa de `commit_sov_u` exige esa base certificada y que el estado histórico/cacheado no puede divergir de `WF_AUTH`.

Por tanto, cuando un documento anterior de la rama cite 74/74 debe leerse como referencia al estado intermedio previo al r2; **el artefacto de cierre A.2 es exclusivamente el r2 de 78/78 identificado por el SHA-256 de §2**.

## 6. Alcance exacto del sello

Quedan sellados para A.2, **bajo el conjunto de reglas sensibles cerrado y las admisiones externas declaradas `Gamma_V`, `Gamma_H`, `Gamma_G`, `Gamma_S` y restauración confiable**:

- L1 — separación de efectos;
- L2 — procedencia y confinamiento de capacidades;
- L3 — preservación semántica de historia;
- P1 — independencia mínima entre firma resolutiva y perfil de autoridad, sin identificarla con la equivalencia contextual IJGS;
- TA — no escalada de `AEnv` bajo trazas ordinarias;
- TB — no derivabilidad de compromiso soberano sin acto humano admitido;
- C1 — conservación de `B_auth` dentro de AUTH;
- C2 — no interferencia de una transición AUTH pura sobre el estado basal opaco `S_B`;
- IS — `perfect informational substitution !=> authority substitution`.

Las pruebas son por enumeración/inversión sobre la **tabla sellada**. No son invariantes independientes de futuras extensiones arbitrarias.

No queda sellado todavía:

- handoff AUTH → estado basal integrado;
- conservación integrada de REAL/SIM, custodia o precedencia;
- J6/E6xx en parser/validator;
- implementación pública en `SV-motor`;
- autenticación humana, criptografía, SO/hardware o integridad frente a runtime hostil;
- equivalencia resolutiva contextual completa de la línea IJGS.

## 7. Norma de rama para J6 y posteriores

Esta norma es de ingeniería de integración, no un teorema A.2:

**Toda modificación o adición de una regla sensible de AUTH requiere, antes de merge:**

1. nueva versión explícita de la tabla de reglas;
2. reevaluación de `WF_AUTH`, IU y L1–L3/TA/TB/C1 en lo afectado;
3. reejecución completa de AES, P1 Witness Suite y LAS;
4. actualización de la matriz regla↔tests;
5. prohibición de merge si la batería no termina con cero fallos.

## 8. Decisión de cierre

Con la especificación v0.2.2, sus pruebas semánticas revisadas, la enmienda normativa de §3, la realización r2 fijada por hash y la matriz 15/15 de §4, **A.2 queda sellada para comenzar J6/E6xx en la rama `sv-auth-v0.2`**.

Este sello no autoriza ninguna afirmación de integración en `main` ni de implementación completa en el ecosistema publicado.
