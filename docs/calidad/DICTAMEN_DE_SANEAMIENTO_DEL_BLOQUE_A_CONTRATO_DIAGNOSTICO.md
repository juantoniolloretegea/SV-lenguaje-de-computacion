# Dictamen de saneamiento del Bloque A — Contrato diagnóstico del lenguaje SV

**Estado:** salida técnica sincronizada con nota posterior de resincronización visible  
**Fecha base:** 19/03/2026  
**Actualización posterior:** 24/03/2026


## Nota de lectura posterior

Las referencias específicas a `E102`, `E104`, `E208`, `E209` y `E303` contenidas en este documento deben leerse conforme a la **nota posterior de sincronización visible** incorporada al final. Esa nota no reabre la doctrina ni el validator: corrige la descripción documental del estado ya observable del repo fresco.

## 1. Objeto

Este dictamen recoge la salida técnica vigente de la microauditoría del Bloque A y fija tratamiento recomendado por grupos para el contrato diagnóstico observable del frontend de referencia.

## 2. Resultado de la matriz

### 2.1. Coincidencias
Constan **2 códigos plenamente coincidentes** entre IR y contrato efectivo:

- `E106 — MissingSemanticRelation`
- `E111 — UnorderedCodomain`

### 2.2. Divergencias por mismo identificador y significado distinto
Constan **22 códigos** con mismo ID formal y semántica divergente entre IR y contrato efectivo actual.

### 2.3. Errores presentes solo en IR
Constan **14 códigos** definidos en la IR v0.2 y ausentes del contrato implementativo efectivo actual.

### 2.4. Errores presentes solo en implementación
Constan **13 códigos** presentes en el contrato efectivo del frontend y ausentes de la IR v0.2.

## 2.5. Contraste fino de la familia E101–E111

La salida vigente incorpora ya el contraste fino público de `E102`, `E104`, `E106` y `E111` mediante sondas documentadas de verificabilidad externa.

Ese contraste deja fijado lo siguiente:

- `E102` mantiene divergencia: el subcaso superficial visible de `OutputSemantics` ausente cae hoy en `E006`.
- `E104` mantiene divergencia: el subcaso superficial visible de codominio de conector inválido cae hoy en `E008`.
- `E106` mantiene coincidencia formal entre IR y contrato efectivo, pero la ausencia de `relation` declarada cae hoy en `E006` en la superficie actual.
- `E111` mantiene coincidencia formal entre IR y contrato efectivo, pero no dispone todavía de sonda superficial propia en v0.1 porque el lowering que exige orden total explícito no está expuesto en la gramática publicada.

## 3. Regularización provisional vigente

Sigue vigente la regularización por **Vía B** ya documentada en `docs/calidad/C1C_DECISION_REGULARIZACION_CONTRATO_DIAGNOSTICO.md`.

Eso implica:

- la **IR v0.2** conserva autoridad normativa superior;
- el **catálogo público efectivo** describe el contrato diagnóstico realmente utilizado hoy por el frontend;
- y toda convergencia futura deberá tratarse como acto formal separado, no como absorción silenciosa.

## 4. Tratamientos recomendados

### 4.1. Mantener como deuda gobernada
Debe mantenerse como deuda gobernada, de forma explícita y sin maquillaje, todo el grupo de:

- `solo_implementacion`
- `solo_ir`

mientras no exista decisión formal de convergencia o de descarte.

### 4.2. Regularizar documentalmente de inmediato
Debe regularizarse documentalmente de inmediato el grupo de:

- `mismo_id_significado_distinto`

porque constituye la zona de mayor riesgo de confusión para lectores, testers y futuras revisiones técnicas.

La regularización mínima exigible es:

- visibilidad de la divergencia;
- no sobreatribución de concordancia por compartir identificador;
- y remisión expresa a la Vía B cuando proceda.

### 4.3. Reconciliar semánticamente en fase posterior gobernada
La reconciliación semántica real entre IR y contrato efectivo no debe hacerse ahora por impulso general, sino en una fase posterior gobernada y por familias de errores, una vez fijado mejor el ABI semántico-diagnóstico.

### 4.4. Preparar migración posterior
Los códigos `solo_ir` deben tratarse como **candidatos de ABI futuro**, no como fallos de implementación por sí mismos.  
Los códigos `solo_implementacion` deben tratarse como **contrato efectivo provisional**, no como doctrina consolidada.

## 5. Decisiones de grupo

### Grupo A — Coincidencias
**Decisión:** mantener vigentes y ampliar cobertura de suite cuando sea posible.

### Grupo B — Mismo ID / significado distinto
**Decisión:** mantener bajo Vía B, explicitar divergencia y no fingir concordancia.

### Grupo C — Solo implementación
**Decisión:** mantener como deuda gobernada del contrato efectivo actual.

### Grupo D — Solo IR
**Decisión:** reservar para ABI semántico-diagnóstico y convergencia futura.

## 6. Conclusión adversarial

La fractura diagnóstica principal del lenguaje SV **no está en la ausencia de estructura**, sino en la coexistencia de dos planos diagnósticos todavía no reconciliados:

- la **IR v0.2** como norma diagnóstica superior;
- y el **catálogo efectivo** del frontend de referencia como contrato operativo vigente.

La salida correcta no es abrir más frentes ni rehacer doctrina, sino:

1. **sincerar** la divergencia;
2. **gobernarla** documentalmente;
3. y **reservar** la convergencia real para una fase posterior del frente final del lenguaje SV, por familias de errores y con control de ABI.

## 7. Estado de secuencia del Bloque A tras RETP-2026-027 y RETP-2026-030

El Bloque A queda hoy, sobre el repositorio realmente materializado, en el siguiente estado de secuencia:

1. `E001–E010` — barrido suficiente para la fase actual.
2. `E101–E111` — contraste fino ya ejecutado en los cuatro puntos residuales de mayor interés (`E102 / E104 / E106 / E111`).
3. `E201–E211` — apertura parcial ya ejecutada, con cobertura explícita de `E202`, `E204` y `E205`, además de la cobertura ya existente de `E210` y `E211`.
4. `E401–E403` — endurecimiento inicial ya materializado fuera del Bloque A estricto, dentro del Bloque E de ABI semántico-diagnóstico y N4/Uso.

En consecuencia, el siguiente contraste fino pendiente **dentro del Bloque A** ya no pasa por reabrir `E201–E211` como si siguiera intacta ni por prometer cierre homogéneo de toda la capa de evolución.

La familia pendiente correcta del Bloque A pasa a ser `E301–E304`, pero **solo** como contraste fino gobernado y sin presuponer todavía una apertura funcional completa.

## 8. Estado fino de la familia E301–E304

### 8.1. `E304`

`E304` dispone ya de materialidad verificable suficiente:

- emisión observable;
- cobertura explícita de suite;
- y estatuto público reconocible en el catálogo implementativo.

### 8.2. `E301` y `E302`

`E301` y `E302` no se presentan hoy, en la superficie publicada v0.1, como cierres funcionales homogéneos ya listos para batería adversarial equivalente.

La razón no es ausencia doctrinal, sino que la inmutabilidad de `Frame` y el carácter `append-only` de `Trajectory` aparecen en gran medida como invariantes de tipo y no como operaciones superficiales abiertas al usuario en la gramática actual.

Por ello, cualquier apertura posterior sobre `E301` y `E302` deberá decidirse con cautela, distinguiendo entre:

- contraste fino documental;
- alcanzabilidad superficial real;
- y eventual ajuste funcional mínimo, solo si estuviera justificado.

### 8.3. `E303`

`E303` conserva presencia en el catálogo, pero no queda todavía acreditada una **emisión específica autónoma** de `E303` como error observable separado.

La validación de `TransitionData` exige hoy `horizon_ref` por forma, y los fallos de referencia incorrecta tienden a quedar absorbidos por errores estructurales o de referencia previa ya existentes.

En consecuencia, no procede presentar `E303` como cierre funcional específico ya materializado mientras esa emisión autónoma no conste con evidencia propia.

## 9. Nota de cierre del saneamiento

La regularización correcta de este punto no consiste en inflar artificialmente la capa de evolución, sino en dejar el Bloque A en un estado documental honesto:

- `E304` sí consta como materialidad verificable.
- `E301` y `E302` requieren decisión posterior sobre su verdadero régimen de alcanzabilidad superficial.
- `E303` permanece sin emisión específica autónoma acreditada.

Por ello, el siguiente paso legítimo tras este dictamen no es prometer cierre homogéneo inmediato de `E301–E304`, sino decidir, en bloque aparte y con radio corto, si procede:

1. contraste fino puramente documental de `E301–E304`, o
2. ajuste funcional mínimo específicamente justificado.


## 10. Nota posterior de sincronización visible (24/03/2026)

La lectura fresca del repo real, contrastada contra `src/svp_errors.py`, `tests/run_conformance.py` y los casos inválidos vigentes, obliga a dejar constancia de la siguiente resincronización documental:

- `E102` consta ya con **emisión observable** y **cobertura explícita** mediante `output_semantics_no_declarada.svp`.
- `E104` consta ya con **emisión observable** y **cobertura explícita** mediante `conector_target_no_ternario.svp`.
- `E208` y `E209` constan ya con **emisión observable** y **cobertura explícita** mediante `compose_relations_vacias.svp` y `compose_patterns_vacios.svp`.
- `E303` consta ya con **emisión observable** y **cobertura explícita** mediante `transition_data_horizon_no_declarado.svp`.

En consecuencia, quedan **materialmente superadas** las formulaciones anteriores de este dictamen que hacían caer hoy a `E102` en `E006`, a `E104` en `E008` o que negaban acreditación autónoma a `E303`.

La actualización correcta no consiste en reescribir silenciosamente la historia del Bloque A, sino en **anotar explícitamente** que el estado documental vigente debe leerse ya con esta sincronización visible.
