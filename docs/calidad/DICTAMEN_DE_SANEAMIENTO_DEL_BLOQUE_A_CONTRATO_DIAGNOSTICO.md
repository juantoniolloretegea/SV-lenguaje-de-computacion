# Dictamen de saneamiento del Bloque A — Contrato diagnóstico del lenguaje SV

**Estado:** salida técnica sincronizada con notas posteriores de resincronización visible  
**Fecha base:** 19/03/2026  
**Actualización posterior:** 24/03/2026  
**Resincronización vigente:** 18/08/2026


## Nota de lectura posterior

Las referencias específicas a `E102`, `E104`, `E208`, `E209` y `E303` contenidas en este documento deben leerse conforme a las **notas posteriores de sincronización visible** incorporadas al final. Esas notas no reabren la doctrina ni reescriben la historia del validator: corrigen la descripción documental del estado ya observable del árbol del repositorio verificado.

## 1. Objeto

Este dictamen recoge la salida técnica de la microauditoría del Bloque A y fija tratamiento recomendado por grupos para el contrato diagnóstico observable del frontend de referencia.

## 2. Resultado de la matriz en la fecha base

### 2.1. Coincidencias
En la clasificación de partida constaban **2 códigos plenamente coincidentes** entre IR y contrato efectivo:

- `E106 — MissingSemanticRelation`
- `E111 — UnorderedCodomain`

### 2.2. Divergencias por mismo identificador y significado distinto
En la clasificación de partida constaban **22 códigos** con mismo ID formal y semántica divergente entre IR y contrato efectivo.

### 2.3. Errores presentes solo en IR
Constaban **14 códigos** definidos en la IR v0.2 y ausentes del contrato implementativo efectivo.

### 2.4. Errores presentes solo en implementación
Constaban **13 códigos** presentes en el contrato efectivo del frontend y ausentes de la IR v0.2.

## 2.5. Contraste fino histórico de la familia E101–E111

La salida de marzo incorporó el contraste fino público de `E102`, `E104`, `E106` y `E111` mediante sondas documentadas de verificabilidad externa.

En aquel estado se dejó fijado:

- `E102`: el subcaso superficial visible de `OutputSemantics` ausente caía en `E006`;
- `E104`: el subcaso superficial visible de codominio de conector inválido caía en `E008`;
- `E106`: coincidencia formal entre IR y contrato efectivo, con la ausencia de `relation` absorbida entonces por `E006`;
- `E111`: coincidencia formal sin sonda superficial propia en v0.1.

Estas observaciones tienen valor histórico y deben leerse junto a las notas de resincronización de las secciones 10 y 11.

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
La reconciliación semántica real entre IR y contrato efectivo no debe hacerse por impulso general, sino en una fase posterior gobernada y por familias de errores, una vez fijado mejor el ABI semántico-diagnóstico.

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

El Bloque A quedó, sobre el repositorio materializado entonces, en el siguiente estado de secuencia:

1. `E001–E010` — barrido suficiente para aquella fase.
2. `E101–E111` — contraste fino ejecutado en los cuatro puntos residuales de mayor interés (`E102 / E104 / E106 / E111`).
3. `E201–E211` — apertura parcial ejecutada, con cobertura explícita de `E202`, `E204` y `E205`, además de la cobertura ya existente de `E210` y `E211`.
4. `E401–E403` — endurecimiento inicial materializado fuera del Bloque A estricto, dentro del Bloque E de ABI semántico-diagnóstico y N4/Uso.

En consecuencia, el siguiente contraste fino pendiente **dentro del Bloque A** pasó entonces a ser `E301–E304`, sin presuponer una apertura funcional completa.

## 8. Estado fino histórico de la familia E301–E304

### 8.1. `E304`

`E304` disponía ya de materialidad verificable suficiente:

- emisión observable;
- cobertura explícita de suite;
- y estatuto público reconocible en el catálogo implementativo.

### 8.2. `E301` y `E302`

`E301` y `E302` no se presentaban, en la superficie publicada v0.1, como cierres funcionales homogéneos ya listos para batería adversarial equivalente.

La razón no era ausencia doctrinal, sino que la inmutabilidad de `Frame` y el carácter `append-only` de `Trajectory` aparecían en gran medida como invariantes de tipo y no como operaciones superficiales abiertas al usuario en la gramática actual.

### 8.3. `E303`

En la redacción base se dejó constancia de que `E303` todavía no tenía acreditación autónoma. Esa afirmación quedó posteriormente superada y se conserva aquí sólo como historia del saneamiento.

## 9. Nota de cierre histórico del saneamiento

La regularización de aquel punto no consistía en inflar artificialmente la capa de evolución, sino en dejar el Bloque A en un estado documental honesto y decidir por radio corto qué requería materialización posterior.

## 10. Nota posterior de sincronización visible (24/03/2026)

La base de verificación del repo real, contrastada contra `src/svp_errors.py`, `tests/run_conformance.py` y los casos inválidos vigentes, obligó a dejar constancia de la siguiente resincronización documental:

- `E102` constaba ya con **emisión observable** y **cobertura explícita** mediante `output_semantics_no_declarada.svp`.
- `E104` constaba ya con **emisión observable** y **cobertura explícita** mediante `conector_target_no_ternario.svp`.
- `E208` y `E209` constaban ya con **emisión observable** y **cobertura explícita** mediante `compose_relations_vacias.svp` y `compose_patterns_vacios.svp`.
- `E303` constaba ya con **emisión observable** y **cobertura explícita** mediante `transition_data_horizon_no_declarado.svp`.

En consecuencia, quedaron **materialmente superadas** las formulaciones anteriores que hacían caer a `E102` en `E006`, a `E104` en `E008` o que negaban acreditación autónoma a `E303`.

## 11. Resincronización vigente tras la reapertura por Ruta A (18/08/2026)

La reapertura del Lenguaje permitió contrastar de nuevo, contra el árbol fresco, la IR v0.2, `src/svp_errors.py`, `src/svp_parser.py`, `src/svp_validator.py` y `tests/run_conformance.py`.

Ese contraste obliga a precisar además:

1. `E102 — MissingOutputSemantics` no debe seguir clasificado como divergencia vigente. El identificador, el nombre y la obligación material coinciden hoy entre IR y contrato efectivo, y el frontend dispone de emisión y cobertura explícitas.
2. `E104 — InvalidConnectorCodomain` tampoco debe seguir clasificado como divergencia vigente. El frontend actual emite `E104` para el destino no ternario del conector y existe cobertura explícita propia.
3. `E008 — ConnectorTargetNotTri` permanece en el catálogo implementativo por trazabilidad, pero **no dispone hoy de sitio de emisión directo ni de caso explícito de suite**. El subcaso que históricamente absorbía se emite ahora como `E104`.
4. La cobertura explícita de `E101` y `E105` consta en la suite vigente y debe reflejarse como tal.
5. El metadato de fase de `E507 — UCoercionDetected` queda alineado con su emisión real en parser: `parse`.

Con estas correcciones, el balance vigente por mismo identificador pasa a ser:

- **4 coincidencias semánticas:** `E102`, `E104`, `E106`, `E111`;
- **20 divergencias con mismo ID**;
- **14 códigos solo IR**;
- **13 códigos solo implementación**.

Esta actualización no adopta la Vía A, no renumera el catálogo y no reduce la autoridad de la IR v0.2. Su efecto es únicamente eliminar contradicciones documentales que habían quedado obsoletas dentro de la propia Vía B.
