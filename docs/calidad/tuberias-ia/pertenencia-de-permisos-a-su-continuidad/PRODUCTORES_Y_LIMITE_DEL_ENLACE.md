# Productores reales y límite del enlace profesional

RETP-153. La lectura y su pertenencia RETP-152 conservan su cierre documental. El experto humano de cada dominio concede la autoridad; las actuaciones proceden de su constitución y de la cobertura del agente. Esa decisión humana ya se recibió. Esta tabla localiza el trabajo de realización pendiente y evita volver a preguntarle al autor quién autoriza.

| Pieza | Productor o vía presente en Rust | Condición pendiente |
| --- | --- | --- |
| Identificadores de control | `ControlId::new`; referencias protegidas mediante `from_core_id` interno al crate | Un identificador no prueba constitución, admisión ni legitimidad |
| Premisa constituyente | `ExternalGenesisPremise`, consumida por `AuthorityContinuity::apply_genesis` | No tiene productor público; sólo aparece `for_test` bajo `cfg(test)`. Falta admisión externa material del acto aprobado |
| Forma, efecto y autoridad | `apply_genesis` comprueba el plan y construye `FormDescriptor`, `EffectDescriptor` y `ConstitutedAuthority` | El productor interno existe. Necesita premisa admitida y propuestas con referencias constituidas; no sustituirlas por campos del recibo |
| Continuidad protegida | `ProtectedDecisionContinuity::from_authority` | Puede envolver una continuidad vacía; eso no concede autoridad. Necesita la constitución anterior para la vía gobernada |
| Comprobación individual | `RequirementCheck` con ligaduras exactas | Su único constructor localizado es `constitute_for_test`, condicionado a pruebas. Falta productor operativo bajo verificador y aplicabilidad constituidos |
| Resultado resuelto | `resolve_requirement_result` | Existe y conserva ligaduras/observaciones, pero consume comprobaciones; no las inventa ni acredita por sí mismo su ejecución |
| Permiso, mediación y ejecución | `decide_permit_traced`, `mediate_traced_permit`, `execute_traced_mediated` | Exigen los objetos anteriores. RETP-153 corrige en candidata el intercambio entre instancias con datos iguales |

Fuentes: `rust/sv_core/src/control.rs`, `authority/transitions.rs`, `requirements.rs`, `requirements_bridge.rs` y `decision_trace.rs`, identificadas en CORTES_Y_FUENTES.json. La afirmación sobre productores se limita a este crate y corte; no presume capacidades de un servicio externo no inspeccionado.

## Contrato de entrada que debe concretarse a continuación

El siguiente objeto es una admisión acotada que reciba el acto constituido, conserve sus bytes/versiones y ligue su emisor reconocido, dominio, formas, efectos, contexto y obligaciones a la continuidad que lo custodia. La propuesta de la IA nunca constituye ese acto ni elige su fuente de confianza. El emisor reconocido y la facultad de admitirlo deben proceder del gobierno ya aprobado: un hash, firma sin emisor reconocido o un booleano no los sustituyen.

En el mismo enlace deben identificarse el verificador operativo admitido, la evidencia exacta de su ejecución y la regla que convierte su resultado observado en `RequirementCheck`. Recuperar un informe o elegir `Accredited` no demuestra esa ejecución. El fallo de admisión o de comprobación conserva causa técnica; no rellena U ni concede permiso.

Éste es trabajo de contrato e implementación pendiente. No se introducen ahora un registro universal de autoridades, claves, transporte, emisor criptográfico o nueva semántica del dominio. Suponerlos para obtener una prueba positiva volvería circular el ensayo. El procedimiento debe localizar primero el acto constituido y su sede gobernada aplicable, respetando la pausa de los dominios. Si la evidencia necesaria no está disponible, se mantiene inhabilitada esa actuación concreta y se continúa la vía documental autorizada.

La corrección RETP-153 no cierra la procedencia de comprobaciones, la identidad de todos los referentes de dominio o la legitimidad de la génesis. Tampoco resuelve continuidad durable ni host comprometido. No reabre al autor la definición de frame ni transfiere al agente las leyes algebraicas generales del núcleo.
