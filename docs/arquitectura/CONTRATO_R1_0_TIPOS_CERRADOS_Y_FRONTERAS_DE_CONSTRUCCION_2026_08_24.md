# R1-0 — Tipos cerrados y fronteras de construcción para el control soberano

**Fecha:** 24 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-0

## 1. Objeto

R1-0 fija los tipos mínimos sobre los que se materializarán los cortes posteriores de R1. Su función es impedir que categorías contractualmente distintas queden representadas por un mismo tipo libre o que un adaptador externo pueda fabricar por construcción directa una referencia que implique admisión, constitución o autoridad.

Este corte no concede autoridad, no decide permisos y no ejecuta efectos protegidos.

## 2. Tipos cerrados incorporados

`sv_core` incorpora un módulo de control con las siguientes categorías.

### 2.1. Clases de transición

`TransitionClass` representa exactamente las ocho clases abstractas de SEC.0-A:

```text
T-I  información
T-V  verificación y admisión
T-H  habilitación
T-E  ejercicio
T-G  gobierno
T-C  constitución
T-0  génesis
T-R  recuperación
```

La posesión de una etiqueta `TransitionClass` no constituye una forma válida ni confiere autoridad. La clasificación efectiva de una forma deberá quedar ligada a una constitución válida en los cortes posteriores de R1.

### 2.2. Resultados técnicos de comprobación

`CheckResult` queda cerrado a:

```text
D-A  ACREDITADO
D-R  REFUTADO
D-N  NO_VERIFICABLE
```

`CheckResult` es un tipo distinto de `Tri`. No existe conversión pública de `D-A`, `D-R` o `D-N` a `Tri`, y la documentación ejecutable incorpora una comprobación negativa de compilación para `D-N → Tri`.

La mera construcción de un valor `CheckResult` tampoco acredita una obligación. R1-3 deberá gobernar la procedencia y aplicabilidad de las comprobaciones que puedan intervenir en una decisión protegida.

### 2.3. Ocupación lógica de continuidad

`ContinuityOccupancy` distingue únicamente:

```text
Uninhabited
Inhabited
```

R1-0 no implementa todavía la transición entre ambos estados ni decide la disponibilidad de T-0. El tipo se introduce para evitar que esa condición se represente mediante cadenas o indicadores abiertos cuando R1-2 materialice la restricción de génesis.

### 2.4. Identidad de control

`ControlId` proporciona una identidad nominal no vacía. Identificar un objeto no acredita que esté constituido, admitido, autorizado o habilitado.

Quedan separados los tipos:

```text
InformationRef
AdmittedEvidenceRef
ConstitutedFactRef
AuthorityRef
EnablementRef
ExerciseRef
```

La igualdad del identificador subyacente no convierte una categoría en otra.

## 3. Frontera de construcción

Las referencias protegidas conservan el identificador asociado, pero su constructor no forma parte de la API pública de `sv_core`.

En particular, un consumidor externo no puede transformar directamente:

```text
ControlId → AuthorityRef
ControlId → AdmittedEvidenceRef
ControlId → ConstitutedFactRef
ControlId → EnablementRef
ControlId → ExerciseRef
```

La frontera de construcción permanece dentro del núcleo para que los cortes posteriores puedan asociarla a las condiciones de constitución, gobierno, habilitación o ejercicio que correspondan.

Esta propiedad no prueba todavía que toda autoridad futura sea legítima: únicamente elimina una vía pública de fabricación por construcción directa.

## 4. Separación respecto de la semántica ternaria

R1-0 no modifica:

```text
Tri = {0, 1, U}
grammar_version = 0.2
ir_version = 0.3
serializer_version = 0.1.0
```

Los resultados técnicos de comprobación permanecen fuera del dominio ternario:

```text
D-N ≠ U
D-R ≠ 0
D-A ≠ 1
```

No se introduce una codificación numérica que permita identificar accidentalmente ambos dominios.

## 5. Pruebas incorporadas

Las regresiones de R1-0 comprueban:

- rechazo de identificadores de control vacíos;
- correspondencia exacta de las ocho etiquetas T-*;
- correspondencia exacta de `D-A`, `D-R` y `D-N`;
- distinción de tipo entre `CheckResult` y `Tri`;
- distinción nominal entre información, evidencia admitida, hecho constituido, autoridad, habilitación y ejercicio;
- existencia diferenciada de los estados lógico-intra-proceso de continuidad no habitada y habitada;
- imposibilidad de construir públicamente una referencia protegida desde un `ControlId` mediante documentación de compilación negativa;
- imposibilidad de convertir públicamente `CheckResult::NotVerifiable` en `Tri` mediante documentación de compilación negativa.

## 6. Límites

R1-0 no materializa:

- descriptores completos de formas;
- `E_max` o `D_a`;
- vías autorizantes T-0/T-C/T-G/T-R;
- transición de continuidad no habitada a habitada;
- `Req(F,e | C)`;
- aplicabilidad de verificadores;
- agregación de `D-A/D-R/D-N`;
- `Permit`;
- mediación o ejecución de efectos;
- ligadura decisión–efecto;
- persistencia o recuperación material;
- autoridad externa, criptografía o identidad de implantación;
- Garantía I o Garantía II.

Estas materias conservan los cortes R1-1…R1-6 o las fases posteriores que les correspondan.

## 7. Criterio de cierre de R1-0

R1-0 puede considerarse cerrado cuando:

1. los tipos anteriores compilan en el mismo `sv_core` utilizado por R0;
2. las pruebas y la documentación de compilación negativa concluyen correctamente;
3. las baterías heredadas de R0 no presentan regresión;
4. no se introduce una segunda semántica, una conversión técnica hacia `Tri` ni una vía pública de fabricación de referencias protegidas.

El cierre de R1-0 no abre automáticamente R1-1 ni modifica el estado de R2–R4.