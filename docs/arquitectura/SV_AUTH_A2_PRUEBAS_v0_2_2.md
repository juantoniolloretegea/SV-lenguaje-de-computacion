# SV-AUTH v0.2.2 — A.2 Pruebas semánticas revisadas

## Estatuto

Pruebas relativas a `SV_AUTH_A2_CLAUSURA_OPERACIONAL_v0_2_2.md`. No son una prueba de autenticación, criptografía, integridad de hardware/SO ni corrección absoluta de verificadores. Las premisas `Gamma_V`, `Gamma_H`, `Gamma_G`, `Gamma_S` y la admisión de snapshot pertenecen al límite confiable declarado.

## L1 — Separación de efectos

**Enunciado.** Toda regla informativa preserva `C` y `A`; toda regla Capability o Commit preserva `C` y `A`; sólo `GOV_BIND`, `GOV_GRANT` y `CONSTITUTION_REVISION` pueden modificar autoridad persistente o Constitución.

**Demostración.** Inspección exhaustiva de la tabla sellada:

- `INFO`, `VERIFY`, `REQUEST_HUMAN` escriben únicamente `I`;
- `ADMIT_HUMAN_ACT` y `ADMIT_GOV_ACT` añaden ledger histórico, no `C/A`;
- `MINT_DET`, `MINT_HUMAN`, `MINT_GOV` escriben `Tok` y `Hist`;
- `COMMIT_DET`, `COMMIT_SOV_U`, `RESOLVE_SOV_U` consumen `Tok` y añaden `Hist`;
- `GOV_BIND` y `GOV_GRANT` son los únicos escritores de `A`;
- `CONSTITUTION_REVISION` es el único escritor de `C`.

`RESTORE` es inicialización desde snapshot admitido, no paso operacional ordinario. No existe regla ordinaria `mint_grant`, coerción informativa a autoridad ni constructor ordinario de Constitución. □

## L2 — Procedencia y confinamiento de capacidades

**Enunciado.** Toda capacidad viva deriva de un `Grant` perteneciente a `AEnv(C^v,A^g)` y sólo puede usarse con el principal, operación, scope, objeto, candidato/base, estado, procedimiento, antecedente y par `(v,g)` que su constructor ligó.

**Demostración por inversión.**

1. Si `tau : DetToken`, el único introductor es `MINT_DET`. Su premisa contiene un grant determinista válido, `Authorizes_G=1`, base verificada y binding completo.
2. Si `tau : HumanToken`, el único introductor es `MINT_HUMAN`. Además de un grant humano válido, exige un `HumanAuthorizationAct` ya admitido, fresco y exactamente coincidente con grant/principal/operación/scope/objeto/candidato/base/estado/resolvedor/antecedente.
3. Si `tau : GovernanceToken`, el único introductor es `MINT_GOV`, que exige `GovernanceAuthorizationAct` admitido y fresco para la misma operación gubernativa.

El uso resuelve la capacidad viva almacenada, exige coincidencia de payload, `Gamma_S`, `subject=principal(tau)`, binding operacional exacto y vigencia de `(v,g)`. El consumo es lineal. Por tanto no existe ampliación, transferencia por simple presentación de un payload modificado ni supervivencia a cambio de versión/época. □

## L3 — Preservación semántica de historia

**Enunciado.** Si `K0 -> ... -> Kn` es una traza operacional `WF_AUTH`, entonces `Hist_i` es prefijo de `Hist_j` para `i<j`; cada registro previo conserva `payload` y `SemanticEnvironmentRef`, y por tanto su denotación original.

**Demostración.** Ninguna regla operacional elimina o reescribe registros. Todas las reglas que afectan `Hist` sólo añaden. Por inducción en la longitud de la traza se conserva la relación de prefijo. `Record` y su referencia semántica son inmutables y no existe regla de rebinding. Luego `Denote(r)=[[payload(r)]]_{SemEnv(r)}` permanece fija. `RESTORE` no pertenece a la traza ordinaria; al inicializar exige snapshot admitido, comprueba las referencias semánticas y elimina tokens sin modificar la historia admitida. □

## P1 — Independencia entre equivalencia resolutiva y equivalencia de autoridad

Sea `equiv_R` una equivalencia declarada sobre la firma observable del resolvedor y `equiv_A` igualdad de perfiles de autoridad efectiva.

**W1.** Actor externo `E` y servicio autorizado `S` producen la misma firma resolutiva `rho`. Entonces `E equiv_R S`. Si `E` carece de grants y `S` posee al menos un grant válido, `AProf(E)=emptyset` y `AProf(S)!=emptyset`; por tanto `E not equiv_A S`.

**W2.** Servicios `S1,S2` tienen idéntico perfil de grants válidos. Entonces `S1 equiv_A S2`. Si sus firmas resolutivas `rho_0,rho_1` pertenecen a clases distintas de `equiv_R`, entonces `S1 not equiv_R S2`.

Luego ninguna equivalencia implica la otra. La realización Python usa firmas de resolvedor/contexto como instancia mínima; no se identifica esa instancia con la equivalencia contextual completa de la línea IJGS. □

## TA — No escalada de autoridad bajo trazas ordinarias

**Enunciado.** Para toda traza `WF_AUTH`

`K0 -> K1 -> ... -> Kn`

que no contenga `GOV_BIND`, `GOV_GRANT` ni `CONSTITUTION_REVISION`,

`AEnv(Kn)=AEnv(K0)`.

`IEnabledAuth` puede variar.

**Demostración.** Por L1, cada paso admitido de la traza preserva `C` y `A`. `AEnv` depende exclusivamente de `ValidGrant(C,A)`, por lo que `AEnv(K_{i+1})=AEnv(K_i)` para cada paso. La igualdad entre extremos se obtiene por inducción.

En cambio `IEnabledAuth` añade precondiciones informativas. `VERIFY` puede introducir una base verificada que satisfaga la precondición de un grant ya perteneciente a `AEnv`; por ello `IEnabledAuth` puede crecer sin creación o ampliación de autoridad persistente. □

## TB — No derivabilidad de compromiso soberano sin admisión humana

**Enunciado.** En una traza `WF_AUTH` que no contiene el `HumanAuthorizationAct` admitido correspondiente, ninguna secuencia de reglas informativas, deterministas, verificadores o actores externos puede introducir `SovereignDecision(U)` ni resolver soberanamente una U histórica.

**Demostración por inversión.**

La única regla que introduce `SovereignDecision(U)` es `COMMIT_SOV_U`. Requiere `HumanToken`. Por L2, el único introductor de `HumanToken` es `MINT_HUMAN`, cuya premisa exige un `HumanAuthorizationAct` admitido bajo `Gamma_H`. Sin esa premisa no existe el token y el commit no es derivable.

Para resolver una U histórica, el único introductor soberano de `SovereignDecision(0|1)` con ese linaje es `RESOLVE_SOV_U`. Requiere una U existente en `Hist`, `prior_decision_ref` coincidente y un nuevo `HumanToken` ligado a `resolve_sov_u`; por L2 vuelve a ser necesario un acto humano admitido fresco.

`INFO`, `VERIFY`, consenso, igualdad exacta de salidas, `HumanReviewRequest`, registros previos y deserialización no introducen `HumanToken`. IU3 impide que gobierno o Constitución añadan un bypass bien formado. □

## C1 — Conservación formal de la frontera de autoridad previa

Sea `B_auth` el subconjunto de la frontera anterior representado directamente por A.2:

1. IA auxiliar no soberana;
2. no cierre soberano automático de U;
3. no mutación automática de autoridad/diseño sin intervención declarada;
4. historia append-only.

**Enunciado.** Toda transición `WF_AUTH` preserva `B_auth`.

**Demostración.**

- Por IU4 y TA, información auxiliar no crea/amplía `AEnv`; por TB tampoco deriva commits soberanos.
- TB prueba la segunda cláusula.
- Sólo reglas gubernativas/constitucionales pueden escribir `A/C`; IU8 exige para ellas un `GovernanceAuthorizationAct` admitido que representa la intervención declarada exigida por la frontera previa. IU3 impide derogar IU1/IU2.
- L3 prueba append-only y estabilidad semántica.

Luego las operaciones nuevas de AUTH no habilitan un canal que `B_auth` prohibía. □

## C2 — No interferencia con restricciones basales ortogonales

Sea `B_orth` el conjunto de propiedades previas no internalizadas por A.2, como REAL/SIM, custodia estructural y precedencia de capas. Considérese `S*=(S_B,K)`.

**Enunciado de capa aislada.** Para toda transición AUTH pura `S* -> S*'`, `S_B'=S_B`.

**Demostración.** Por definición de la firma de efectos AUTH, ninguna regla A.2 posee permiso de escritura sobre el estado basal opaco `S_B`; sus efectos se restringen a `K`. Luego todo predicado que dependa únicamente de `S_B` conserva su valor a través de una transición AUTH pura. □

**Límite.** Esto no certifica todavía el handoff de una decisión AUTH al lenguaje/motor integrado. J6 y el runtime estable deberán demostrar una obligación adicional de refinamiento: el handoff no puede saltar custodia, REAL/SIM ni precedencia. Hasta entonces no se afirmará conservación completa del sistema integrado.

## Corolario IS — Sustitución informativa perfecta no implica sustitución de autoridad

Sean dos ejecuciones que difieren únicamente en el productor de información y produzcan la misma firma resolutiva, el mismo candidato y la misma base verificada, sin transición gubernativa o constitucional.

Por TA, `AEnv` permanece invariante. Por P1, igualdad resolutiva no implica igualdad de perfiles de autoridad. En particular, si el actor externo no posee grant válido, la igualdad perfecta de su salida con la de un principal autorizado no le confiere autoridad.

Por tanto:

`perfect informational substitution !=> authority substitution`.

Este corolario no depende de que la IA sea incorrecta, insegura o incierta; incluso una salida informativamente idéntica no altera por sí sola la fuente de autoridad. □

## Alcance probatorio

Los resultados son propiedades de la semántica sellada A.2.2. Una implementación conforme debe ejercer, además, AES, P1 Witness Suite y LAS. El testeo de una implementación aporta evidencia de conformidad; no sustituye estas demostraciones.