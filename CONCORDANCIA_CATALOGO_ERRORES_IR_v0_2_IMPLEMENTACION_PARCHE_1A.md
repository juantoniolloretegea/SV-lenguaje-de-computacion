# CONCORDANCIA_CATALOGO_ERRORES_IR_v0_2_IMPLEMENTACION_PARCHE_1A.md

## 1. Objeto y alcance

Este documento registra, con finalidad exclusivamente descriptiva y de trazabilidad técnica, las discrepancias materiales verificadas entre el catálogo de errores publicado en la representación intermedia canónica (IR) v0.2 y el catálogo de errores utilizado por la implementación de referencia tras el PARCHE 1A.

Su alcance se limita a documentar dichas discrepancias y a dejar constancia de su existencia para futuras decisiones formales. Este documento no tiene función normativa ni prescriptiva.

No altera la IR v0.2, que permanece como documento de referencia cerrado, ni implica revisión de su contenido sustantivo.

## 2. Fuentes comparadas

### 2.1. Fuente normativa publicada
- `IR_CANONICA_BIENFORMACION_SV_v0_2.md`

### 2.2. Fuente de implementación actual
- catálogo de errores de la implementación de referencia tras PARCHE 1A
- validaciones activas asociadas al frente de supervisión y endurecimiento de conformidad

## 3. Discrepancias materiales verificadas

| Código | Descripción en IR v0.2 | Descripción en implementación actual | Observación |
|--------|-------------------------|--------------------------------------|-------------|
| E205 | `UndeclaredRegime` (grafo sin régimen declarado) | `SuperviseOpaqueTarget` (invocación de `supervise` sin constructor explícito de `Supervisable`) | Reasignación material de significado entre documento normativo publicado e implementación actual |
| E211 | No consta en IR v0.2 | `SuperviseMetaNotSupervisor` (argumento de `supervise` no proveniente de una célula con rol `Supervisor`) | Código presente en implementación actual y ausente en la IR publicada |

## 4. Nota específica sobre E211

`E211` aparece en la implementación actual como resultado del endurecimiento introducido en PARCHE 1A para mejorar la precisión diagnóstica en el frente de supervisión.

La presencia de `E211` en la implementación no implica, por sí sola, modificación de la IR v0.2 ni debe interpretarse como actualización tácita del catálogo normativo publicado.

Su existencia se documenta aquí únicamente como hecho de concordancia pendiente de eventual tratamiento formal separado.

## 5. Efectos y no-efectos del documento

### 5.1. Efectos
- Proporciona trazabilidad documental entre especificación publicada e implementación actual.
- Permite identificar de forma explícita qué discrepancias materiales constan a la fecha de cierre de PARCHE 1A.
- Facilita una futura decisión formal sin necesidad de reinterpretar retroactivamente la IR v0.2.

### 5.2. No-efectos
- No modifica la IR v0.2.
- No modifica el parser, el validator, el runner de conformidad, la suite de tests ni el backend.
- No introduce autoridad normativa nueva.
- No resuelve por sí mismo la eventual revisión futura del catálogo formal de errores.

## 6. Decisión diferida

Cualquier decisión sobre:
- revisión formal de la IR v0.2,
- creación de una versión posterior de la IR,
- regularización definitiva de la concordancia entre catálogo normativo y catálogo implementado,

queda diferida a un acto separado, expresamente deliberado y formalizado.

Hasta entonces, la IR v0.2 conserva su estatus de documento normativo publicado, y este documento puente conserva un valor exclusivamente descriptivo y auxiliar de trazabilidad.
