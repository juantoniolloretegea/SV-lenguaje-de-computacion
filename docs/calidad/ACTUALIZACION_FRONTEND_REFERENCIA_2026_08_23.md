# Actualización técnica del frontend de referencia — 23/08/2026

## 1. Objeto

Este registro público describe tres correcciones materializadas en la etapa frontal del Lenguaje SV y la evidencia reproducible que delimita su alcance.

Las correcciones afectan a:

1. admisibilidad técnica y producción de `Tri`;
2. identificación del objetivo de `resolve` y separación entre revisión y clausura;
3. cierre estructural y causal de las colecciones derivadas de `Frame`.

No modifica la doctrina matemática superior del SV ni convierte la etapa frontal en una infraestructura soberana de ejecución.

---

## 2. Versiones observables

La etapa frontal produce actualmente:

```text
grammar_version    = 0.2
ir_version         = 0.3
serializer_version = 0.1.0
```

Documentos públicos asociados:

- `GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md`;
- `IR_CANONICA_BIENFORMACION_SV_v0_3.md`;
- `docs/referencia/ERRORES_CANONICOS_SV_v0_3.md`.

Las versiones anteriores se conservan como antecedentes y no se reescriben retrospectivamente.

---

## 3. Admisibilidad

El frontend rechaza el conjunto histórico:

```text
{Ok, Degraded, Failed, U}
```

y exige exactamente:

```text
{Ok, Degraded, NotAdmitted}
```

El orden de esos tres identificadores no es significativo.

La corrección evita que `Failed` o `U` sean tratados como estados técnicos de admisibilidad. El fallo de captura continúa representado por `Bottom`; una observación no admitida no produce por defecto un nuevo valor de `Tri`.

Diagnóstico específico:

```text
E110 — InvalidAdmissibilitySpec
```

---

## 4. `resolve`

La forma superficial ya no acepta el literal abstracto `U` como objetivo suficiente.

Forma vigente:

```svp
let RR1 = resolve((S1, 3),
                  with: RS1,
                  context: ContextoClinico,
                  mechanism: RevisionExperto);
```

La validación exige:

- estado evaluable;
- posición uno-basada dentro del vector;
- valor efectivo `U` en esa posición;
- `ResSpec` declarado;
- compatibilidad exacta de contexto y mecanismo en la relación por defecto.

Diagnóstico específico:

```text
E305 — UnsafeUResolution
```

La operación de revisión no adquiere por su mera ejecución autoridad para producir una clausura positiva.

---

## 5. `Frame`

Se refuerza la coherencia relacional de:

```text
cell_states
eval_results
gate_results
supervision
criticalities
```

La validación impide referencias externas al cierre del `Frame`, duplicación material de estado/evaluación y criticidades que la superficie vigente no puede producir.

Se conserva como caso válido que nodos distintos de arquitectura compartan un mismo `CellSpec` mediante `CoupledSpec` distintos.

Diagnóstico específico:

```text
E308 — FrameClosureViolation
```

La obligación es de coherencia, no de exhaustividad.

---

## 6. Batería de conformidad

El ejecutor `tests/run_conformance.py` contiene:

```text
casos válidos   = 11
casos inválidos = 61
total           = 72
```

El resultado verificado para el radio funcional previo a esta actualización documental es:

```text
pasados  = 72
fallidos = 0
```

La integración continua ejecuta dos controles distintos:

1. conformidad contra los JSON canónicos ya comprometidos para los casos válidos;
2. comprobación de que la ejecución no modifica esos oráculos.

Por tanto, la batería no se valida regenerando en el mismo flujo los resultados contra los que después se compara.

---

## 7. Contraejemplos incorporados

La batería contiene casos específicos contra, entre otros:

### Admisibilidad

```text
admissibility_spec_estados_legacy.svp
admissibility_spec_failed_legacy.svp
admissibility_spec_u_legacy.svp
```

### Resolución

```text
resolve_alias_estado_no_u.svp
resolve_instancia_incompatible.svp
resolve_missing_context.svp
resolve_missing_mechanism.svp
resolve_target_fuera_rango.svp
resolve_target_no_u.svp
```

### `Frame`

```text
frame_criticality_no_producible.svp
frame_estado_arquitectura_ajena.svp
frame_eval_duplicado.svp
frame_eval_externo.svp
frame_gate_input_externo.svp
frame_supervision_externa.svp
```

Casos positivos relacionados:

```text
admissibility_spec_states_permutados.svp
resolve_projection.svp
frame_cell_spec_compartida_valida.svp
```

---

## 8. Deuda que no se confunde con estas correcciones

La actualización no resuelve lateralmente la cobertura de `ConflictOperator` en régimen `General`.

Permanece:

```text
J2.3 / ConflictOperator = obligación normativa
E204 canónico = MissingConflictOperator
E204 efectivo = QueryMissingContext
cobertura superficial General = incompleta
```

La divergencia del identificador E204 se conserva explícita en el catálogo v0.3.

---

## 9. Alcance probatorio

El resultado 72/72 acredita la conformidad observable de la etapa frontal frente a la batería y a los oráculos declarados en este radio.

No acredita por sí solo:

- una realización soberana de ejecución;
- correspondencia completa entre especificación, cadena de construcción y artefacto material;
- resistencia adversarial de un sistema completo;
- perfiles materiales o forenses no implementados.

---

## 10. Estado técnico

Con esta actualización, la documentación pública y el frontend comparten las mismas versiones observables y las mismas restricciones para admisibilidad, `resolve` y `Frame`.

Las deudas no resueltas permanecen visibles y separadas de las propiedades efectivamente materializadas.
