# SV-AUTH v0.2 — A.2 Pruebas semánticas

## Estatuto

Este documento demuestra los resultados de la clausura operacional candidata definida en `SV_AUTH_A2_CLAUSURA_OPERACIONAL_v0_2.md`. Las demostraciones son relativas al sistema de reglas selladas y a `WF_AUTH`; no afirman seguridad criptográfica, identidad física ni integridad frente a un runtime arbitrariamente comprometido.

## L1 — Separación de efectos

**Enunciado.** Para toda regla informativa `alpha`, `Write(alpha) subseteq {I,Hist}` y `Intro(alpha)` no contiene `AuthorityRole`, `Grant`, `DetToken`, `HumanToken`, `GovernanceToken` ni `SovereignDecision`. Para toda regla ordinaria que no sea gubernativa ni constitucional, `C` y `A` permanecen invariantes.

**Demostración.** Por inspección exhaustiva de la tabla sellada de reglas sensibles:

- `INFO`, `VERIFY` y `REQUEST_HUMAN` sólo escriben `I` y, opcionalmente, `Hist`; ninguno introduce tipos sensibles.
- `MINT_DET`, `MINT_HUMAN` y `MINT_GOV` escriben `Tok` y `Hist`, pero no `C` ni `A`.
- `COMMIT_DET`, `COMMIT_SOV_U` y `RESOLVE_SOV_U` escriben `Tok` y `Hist`, pero no `C` ni `A`.
- `ADMIT_HUMAN_ACT` registra una premisa admitida de frontera; no crea grants ni modifica Constitución.
- Sólo `GOV_TRANSITION` puede escribir `A`; sólo `CONSTITUTION_REVISION` puede escribir `C`.

No existe regla ordinaria `mint_grant` ni coerción informativa hacia tipos sensibles. Luego se cumplen ambas afirmaciones. □

## L2 — Procedencia y confinamiento de capacidades

**Enunciado.** Toda capacidad viva posee una cadena de procedencia hasta un grant vigente y sólo puede ejercerse dentro de la tupla autorizada por ese grant y bajo el mismo par `(v,g)`.

**Demostración.** Por inversión sobre el tipo de token.

- Si `tau : DetToken`, la única regla introductora es `MINT_DET`, que exige un grant `G` de clase determinista vigente, `Authorizes_G(...) = 1`, base verificada y binding completo.
- Si `tau : HumanToken`, la única regla introductora es `MINT_HUMAN`, que exige además un `HumanAuthorizationAct` admitido, fresco y coincidente con operación, alcance, objeto, candidato/base, estado y `(v,g)`.
- Si `tau : GovernanceToken`, la única regla introductora es `MINT_GOV`, que exige procedimiento gubernativo y grant correspondiente.

Los commits y transiciones verifican `subject = principal(tau)`, binding de operación/objeto/estado y vigencia exacta de `(v,g)`. El consumo es lineal. Una variación de cualquiera de esos campos invalida la premisa de uso. Por tanto el token especializa una autoridad preexistente pero no puede ampliarla ni transferirla. □

## L3 — Preservación semántica de la historia

**Enunciado.** Si `K0 -> ... -> Kn` es una traza `WF_AUTH`, entonces para `i < j`, `Hist_i` es prefijo de `Hist_j`, y todo registro `r` ya existente conserva su denotación bajo su `SemanticEnvironmentRef` original.

**Demostración.** Ninguna regla de la tabla modifica o elimina registros anteriores. Las reglas que afectan a `Hist` sólo añaden registros. Por inducción sobre la longitud de la traza, la relación de prefijo se conserva. Además, `payload(r)` y `SemanticEnvironmentRef(r)` son inmutables y no existe regla de rebinding semántico. Por definición, `Denote(r) = [[payload(r)]]_{SemEnv(r)}`; como ambos argumentos permanecen invariantes, la denotación histórica permanece invariada. `RESTORE` reconstituye una instantánea confiable, no reinterpreta registros ni restaura capacidades vivas. □

## P1 — Independencia entre equivalencia resolutiva y equivalencia de autoridad

Sea `equiv_R` la equivalencia respecto del contenido resolutivo observado por el mecanismo declarado y `equiv_A` la igualdad de perfiles de autoridad efectiva.

**Testigo W1.** Un actor externo `E` y un servicio autorizado `S` producen exactamente el mismo candidato `c`. Entonces `E equiv_R S`. Sin embargo `AProf(E)=emptyset` y `AProf(S)` contiene al menos el commit autorizado; luego `E not equiv_A S`.

**Testigo W2.** Dos servicios `S1,S2` poseen exactamente el mismo perfil de grants y, por tanto, `S1 equiv_A S2`. Sean sus candidatos `c0,c1` distinguibles por la semántica del resolvedor. Entonces `S1 not equiv_R S2`.

Luego ninguna equivalencia implica la otra. □

## TA — No escalada de autoridad bajo trazas ordinarias

**Enunciado.** Sea `K0 -> K1 -> ... -> Kn` una traza `WF_AUTH` formada exclusivamente por reglas no gubernativas ni constitucionales. Entonces

`AEnv(Kn) = AEnv(K0)`.

La información puede modificar `EnabledAuth` sin modificar `AEnv`.

**Demostración.** Por L1, cada paso ordinario preserva `C` y `A`. Como `AEnv` es función exclusivamente de `(C,A)`, cada paso satisface `AEnv(K_{i+1}) = AEnv(K_i)`. Por inducción se obtiene la igualdad entre extremos. En cambio `EnabledAuth` depende además de precondiciones evaluadas sobre `I`; una transición `VERIFY` puede hacer verdadera una precondición de un grant ya existente. Por ello `EnabledAuth` puede variar sin que exista creación o ampliación de autoridad persistente. □

## TB — No derivabilidad de compromiso soberano sin acto humano admitido

**Enunciado.** En una traza `WF_AUTH` que no contiene el juicio de frontera humano admitido correspondiente, ninguna secuencia de reglas informativas, deterministas o de actores externos puede introducir `SovereignDecision(U)` ni producir la resolución soberana de una `SovereignDecision(U)` existente.

**Demostración por inversión.** La única regla sellada cuyo tipo introducido contiene `SovereignDecision(U)` es `COMMIT_SOV_U`. Esta regla exige un `HumanToken`. Por L2, el único introductor de `HumanToken` es `MINT_HUMAN`, que exige un `HumanAuthorizationAct` admitido por el juicio primitivo de frontera. En ausencia de esa premisa no puede construirse el token requerido y, por tanto, tampoco `SovereignDecision(U)`.

Para la resolución soberana de una U existente, la única regla introductora de `SovereignDecision(0|1)` con ese linaje es `RESOLVE_SOV_U`, que exige un nuevo `HumanToken` ligado a `resolve_sov_u`. El mismo argumento de inversión vuelve a exigir un acto humano admitido fresco.

`INFO`, `VERIFY`, consenso de actores, igualdad exacta de salidas, `HumanReviewRequest`, registros históricos y deserialización ordinaria no introducen `HumanToken` ni `SovereignDecision`. IU3 impide añadir mediante gobierno una regla válida que derogue esta propiedad. □

## C1 — Extensión conservativa de la frontera IA preexistente

**Enunciado.** Toda regla/configuración `WF_AUTH` preserva el conjunto `B0` de restricciones anteriores relevantes: IA auxiliar no soberana, no cierre automático de U, no modificación automática ilegítima de diseño/políticas, separación REAL/SIM, historia append-only, no bypass de custodia y subordinación de capas auxiliares.

**Demostración.** L1 impide que reglas informativas de IA escriban `A` o `C`; TB impide que su salida derive compromisos soberanos; L3 conserva historia; las transiciones gubernativas y constitucionales están sometidas a `WF_AUTH` e IU3 y no pueden derogar IU1/IU2. Por construcción, las nuevas capacidades especializan grants preexistentes y no crean un canal externo que sustituya la frontera humana. Por tanto AUTH refina el régimen anterior sin habilitar una operación que `B0` prohibía. □

## Alcance probatorio

Los resultados anteriores son propiedades de la semántica cerrada A.2. La evidencia ejecutable debe comprobar una realización concreta mediante exclusiones adversariales y admisiones legítimas; el paso de esta prueba semántica a una implementación se considera conformidad, no fundamento de la demostración.
