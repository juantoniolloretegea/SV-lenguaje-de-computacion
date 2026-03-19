# Dictamen de saneamiento del Bloque A — Contrato diagnóstico del lenguaje SV

**Estado:** plantilla de trabajo

## 1. Objeto

Este dictamen recogerá la salida final de la microauditoría del Bloque A.

## 2. Resultado de la matriz

### 2.1. Coincidencias
_Pendiente de cumplimentación._

### 2.2. Divergencias por mismo identificador y significado distinto
_Pendiente de cumplimentación._

### 2.3. Errores presentes solo en IR
_Pendiente de cumplimentación._

### 2.4. Errores presentes solo en implementación
_Pendiente de cumplimentación._

## 3. Regularizaciones provisionales vigentes
_Pendiente de cumplimentación._

## 4. Tratamientos recomendados

### 4.1. Mantener como deuda gobernada
_Pendiente de cumplimentación._

### 4.2. Reconciliar semánticamente
_Pendiente de cumplimentación._

### 4.3. Documentar mejor
_Pendiente de cumplimentación._

### 4.4. Preparar migración posterior
_Pendiente de cumplimentación._

## 5. Conclusión adversarial
_Pendiente de cumplimentación._

# Dictamen de saneamiento del Bloque A — Contrato diagnóstico del lenguaje SV

**Estado:** primera salida técnica  
**Fecha:** 19/03/2026

## 1. Objeto

Este dictamen recoge la primera salida de la microauditoría del Bloque A y fija tratamiento recomendado por grupos para el contrato diagnóstico vigente del frontend de referencia.

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

## 3. Regularización provisional vigente

Sigue vigente la regularización por **Vía B** ya documentada en `docs/calidad/C1C_DECISION_REGULARIZACION_CONTRATO_DIAGNOSTICO.md`.

Eso implica:

- la **IR v0.2 conserva autoridad normativa superior**;
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

porque constituye la zona de mayor riesgo de confusión para lectores, testers y futuras unidades Watson.

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

El siguiente paso dentro del Bloque A debe ser una **segunda pasada por familias**, al menos en este orden:

1. `E001–E010` — capa de definición  
2. `E101–E111` — capa de estado  
3. `E201–E211` — capa de resultado  
4. `E301–E304` — capa de evolución  
5. `E401–E507` — capa de uso / serialización / conformidad

Sin abrir todavía Bloque B.
