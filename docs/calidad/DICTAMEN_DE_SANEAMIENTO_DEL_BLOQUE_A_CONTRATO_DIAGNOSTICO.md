# Dictamen de saneamiento del Bloque A — Contrato diagnóstico del lenguaje SV

**Estado:** salida técnica sincronizada  
**Fecha:** 19/03/2026

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

## 7. Próximo paso técnico recomendado

El siguiente paso dentro del Bloque A debe continuar **por familias** y con el orden ya actualizado al estado materializado del repositorio:

1. `E301–E304` — capa de evolución
2. `E401–E507` — capa de uso / serialización / conformidad

La familia `E001–E010` queda ya suficientemente barrida para la fase actual y la familia `E101–E111` queda ya contrastada finamente en sus cuatro puntos residuales de mayor interés (`E102 / E104 / E106 / E111`). El siguiente barrido legítimo del Bloque A pasa, por tanto, a `E201–E211`.


## 8. Nota de avance sobre la familia E201–E211

Tras el primer barrido de la familia `E201–E211`, `E202`, `E204` y `E205` pasan a tener cobertura explícita de suite. `E210` y `E211` ya la tenían. Quedan como subtramos residuales `E201`, `E203`, `E206`, `E207`, `E208` y `E209`, cuyo contraste fino queda diferido por depender hoy de formas superficiales no directamente expuestas o no emitidas de modo específico en la gramática v0.1.
