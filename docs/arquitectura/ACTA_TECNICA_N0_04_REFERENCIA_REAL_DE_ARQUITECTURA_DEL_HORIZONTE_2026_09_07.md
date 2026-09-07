# Acta técnica N0-04: referencia real de arquitectura del horizonte

**Fecha:** 7 de septiembre de 2026

**Registro:** RETP-2026-081

**Corte de entrada:** main `f9aa3ebada0db222bb9d196f94a9b42dac185f97`, integración de [PR #67 / N0-03](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/67).

**Expediente:** [historial de la candidata N0-04](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commits/n0-04-referencia-arquitectura-20260907), con su PR, controles y promoción.

**Estado:** fila 3 de K1; cierre limitado de N0-04 condicionado a la promoción del candidato exacto.

## 1. Fuentes recibidas y precisión normativa

Se leen completos AGENTS.md, los [Pilares RETP-073](../calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), el [acta de perfiles RETP-075](../calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md), la [transición con relevo §21](../dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md), la [radiografía N0](./N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md), el [acta N0-03](./ACTA_TECNICA_N0_03_UNICIDAD_DE_MIEMBROS_Y_ESTABILIDAD_DE_PROYECCION_JSON_2026_09_06.md) y el registro/deuda aplicables. Se cotejan IR v0.3, las definiciones heredadas de IR v0.2, el catálogo efectivo, los resolutores y las rutas de validación; perfiles fuente §§6–10 gobierna la conservación y el ensamblaje.

La radiografía describe el referente requerido como `CompositionGraph`. El texto literal de [IR v0.2, nivel 3](../../IR_CANONICA_BIENFORMACION_SV_v0_2.md) es `Horizon.architecture : ArchitectureId`; el nivel 4 sí declara `Agent.architecture : CompositionGraph`. Esta precisión evita atribuir al antecedente una grafía que no contiene. [IR v0.3 §6.4](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md#arquitectura-n0-04) concreta ahora J-H0: esa identidad debe resolver un grafo declarado y bien formado. No crea un nuevo campo ni un tipo de perfil tecnológico.

## 2. Defecto, decisión y alcance de realización

Python y Rust admitían un horizonte con arquitectura inexistente o de otro tipo. La igualdad nominal entre la arquitectura del agente y la de su horizonte tampoco acreditaba por sí sola un referente real. Los dos nuevos negativos mínimos reproducen admisión con retorno 0 en la base y rechazo con retorno 1, sin IR, en la candidata.

Se añade una comprobación complementaria de todos los horizontes al final de la validación global, después de los controles anteriores y N0-03. Reutiliza `_require_ref` en [Python](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/a09b9efef51f88de29048b9b35e7ac085dc0918f/src/svp_validator.py) y `expect_object` en [Rust](../../rust/sv_core/src/wellformed.rs), con tipo `GraphDecl`/`CompositionGraph`. Conserva la precedencia de los rechazos anteriores, las referencias adelantadas y la resolución entre unidades. No transforma ni completa la fuente o el programa.

La validación existente de `Agent–Domain–Horizon` se conserva. Para admitir el programa deben cumplirse tanto la igualdad de arquitectura del agente como J-H0; por ello ambas designan el mismo grafo real. No hace falta duplicar el resolutor dentro de Agent. Un grafo declarado sigue sometido a sus propias guardas. Un grafo homónimo de un horizonte, una operación o un objeto de otro tipo no satisface J-H0. Dos grafos estructuralmente iguales con identidades distintas no se sustituyen.

Se mantienen Gramática 0.2, esquema IR 0.3, serializador 0.1.0 y la implementación de los emisores. El nuevo rechazo usa E006 en Python; Rust conserva los mensajes de su resolutor común bajo `InvalidProgram`. El agente incompatible conserva E402 y su texto Rust. El catálogo conserva sus 51 entradas. No se añade un código ni se declara paridad diagnóstica estructurada. La diferencia conocida entre el nombre E006 y su uso para tipo incorrecto sigue en DFL-001.

## 3. Corpus y corrección explícita de un antecedente positivo

El corpus pasa de **88 a 91 programas: 14 válidos y 77 inválidos**. Se incorporan:

- [arquitectura ausente](../../tests/conformance/invalid/horizon_architecture_ausente.svp), E006 / referencia no declarada;
- [tipo incorrecto](../../tests/conformance/invalid/horizon_architecture_tipo_incorrecto.svp), E006 / se esperaba CompositionGraph;
- [agente con dos grafos reales distintos](../../tests/conformance/invalid/agent_arquitecturas_reales_distintas.svp), control de E402 y de la guarda Rust previa.

El positivo histórico [transition_data_events.svp en la base](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/f9aa3ebada0db222bb9d196f94a9b42dac185f97/tests/conformance/valid/transition_data_events.svp) omitía Arch1. Sobre sus bytes originales, la base admite y la candidata rechaza en ambos emisores. No se mantiene como positivo de la nueva obligación.

Su [fuente corregida](../../tests/conformance/valid/transition_data_events.svp) conserva íntegro el prefijo original y añade `CoupledSpec CCArch`, `SemanticRelation RArch` y `CompositionGraph Arch1` después del horizonte, protegiendo también referencias adelantadas. El [esperado corregido](../../tests/conformance/valid/transition_data_events.expected.json) conserva exactamente los cinco objetos originales y las cabeceras, añade los tres objetos declarados y actualiza sólo la huella de la fuente. Se redacta desde el esquema y las declaraciones; no se obtiene del compilador. No se presenta este cambio como conservación literal del esperado anterior. El historial Git conserva ambos cortes sin duplicar una segunda batería normativa.

| Artefacto | SHA-256 anterior | SHA-256 corregido |
|---|---|---|
| transition_data_events.svp | `4e99a15de36611c79984142f767813aaab7054b105bcf08a63fd286143d583b2` | `f5c23f25dcbfbd38ce26db9133e349f832566c1dcc7f3bd3e3cdc1a10c75f3db` |
| transition_data_events.expected.json | `ea1f90873e5c3c865ddfdbff148f3c0eb4654df730e6ceabff135ef16c8f3b32` | `e46e5e31a9b368cb8c21bf6eee349994d9a02832522ea8ddfb7143df7ba485ba` |

Los otros trece esperados y sus fuentes permanecen byte a byte iguales. Los 74 negativos anteriores conservan entradas, retornos, stdout y stderr en cada emisor respecto de la base. Algunos, incluido el antiguo agente incompatible, contienen también una arquitectura no declarada: conservar su primer rechazo no certifica esa referencia ni convierte el caso en control aislado de N0-04. Los nuevos negativos y el control con dos grafos reales eliminan esa ambigüedad de observación.

## 4. Pruebas y promoción

Las [cuatro pruebas Python](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/a09b9efef51f88de29048b9b35e7ac085dc0918f/tests/test_horizon_architecture.py) y las [cinco Rust](../../rust/sv_core/tests/horizon_architecture.rs) comprueban ausencia, tipo incorrecto, autorreferencia, referencia a operación, referente gráfico mal formado, referencias adelantadas, identidad del agente y ausencia de mutación. Rust ejerce ES/EN y ensamblaje mixto en ambos órdenes y con los papeles de los perfiles intercambiados. Python conserva su superficie EN. La secuencia de sucesos se conserva, incluidas repeticiones, como control del alcance previo; su significado normativo queda pendiente de su acto propio.

| Verificación local | Resultado |
|---|---|
| Conformidad y paridad R0-7 | 91/91: 14 positivos y 77 rechazos controlados |
| Python | 31/31: observador 19, N0-02 5, N0-03 3 y N0-04 4 |
| Rust | 210 internas; N0-01 3, N0-02 6, N0-03 5, N0-04 5; dominios cerrados 5; adaptador 2; documentales 17, todas correctas |
| CLI / SEC-0 / caracterización E006 | 3/3, 3/3 y 4/4 |
| Recepción de la base | 13 positivos y 74 negativos conservan literalmente sus salidas en cada emisor; el positivo histórico colgante pasa de retorno 0 a 1 |
| Sensibilidad v3 | Un control, dos rechazos E115 y dos divergencias CRLF abiertas detectadas; cinco fuentes conservadas |

Entorno local: Rust/cargo 1.98.0, mismo compilador para base y candidata, instalación comprobada contra la huella publicada del paquete. La higiene de toolchain no forma parte de este cambio; CI registra sus compiladores efectivos.

```bash
python -m unittest discover -s tests -p 'test*.py' -v
python tests/run_conformance.py
cargo test --manifest-path rust/Cargo.toml --workspace
cargo build --manifest-path rust/Cargo.toml -p sv_native
python tests/r0_7_equivalence.py --rust-bin rust/target/debug/sv-native
python tests/run_oracle_sensitivity.py --rust-bin rust/target/debug/sv-native --output-dir artifacts/oracle-sensitivity
python tests/run_cli_smoke.py
python tests/run_sec0_smoke.py
python tests/run_e006_characterization.py
```

La promoción exige los cuatro flujos correctos sobre la candidata exacta: Conformidad SVP, R0 Rust, R0-8 Baseline nativa y R0 WASM de tres vías. WASI y navegador reciben el corpus 14/77; las seis sondas DG previas conservan su alcance. Las variantes nuevas ES/EN y de ensamblaje se prueban en Rust nativo; no se atribuyen como campaña directa de esas variantes en navegador. Cabeza, árbol, base, ejecuciones e integración quedan en el expediente enlazado, sin transferir los controles de PR #67.

## 5. Perfiles, evidencia de laboratorio y relevo

La representación vigente basta para expresar la referencia y rechazar su ausencia. PT04 recibe el diagnóstico por vía; PT13 la paridad; PT14 el corpus y entorno; PT01/PT02 las identidades y la comparación literal con sus excepciones explícitas. No aplica conocimiento IMM/CYB a este invariante intrínseco ni se constituye cobertura o ejecución de un agente de dominio.

Se reutilizan los comprobadores y la distinción entre validez e identidad recibida del [registro experimental 016](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/016-VIGILANCIA_Y_COSTE_2026_09_06.md), conforme al acta de perfiles. El [registro 018](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/018-COMPARACION_DOTNET_FFI_WASM_2026_09_06.md) conserva sus 79 programas EN y condiciones .NET/FFI/WASM; no acredita retrospectivamente los 91 programas ni ensamblaje. No se promueve una nueva realización tecnológica. La prueba previa de las realizaciones tecnológicas y el enlace material conservan sus puertas; este cierre no justifica repetir la campaña de infraestructura.

El cierre limitado se hace efectivo al integrar el expediente con los cuatro flujos correctos. **Sigue la unicidad de `CoupledSpec.bridges` como representación de `BridgeSet`**, conforme a N0 §7 y transición §14; después corresponden las decisiones sobre `Horizon.events` y el mínimo estructural de Domain. N0-05 y N0-07 permanecen en K2; no son el paso inmediato por su numeración. DFL-001, DFL-008, los restantes frentes de K1/K1-T y los contratos pendientes siguen abiertos. No se acreditan otras relaciones causales de horizonte/frame, álgebra, consolidación nuclear, R2/R3/R4 ni un artefacto web nuevo.
