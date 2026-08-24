# Núcleo Rust del Lenguaje SV

Esta carpeta contiene la realización compartida en Rust del núcleo semántico del Lenguaje SV para destinos nativo y WebAssembly.

La implementación mantiene una sola fuente semántica en `sv_core`. `sv_wasm` es un adaptador material del mismo núcleo y no contiene una realización alternativa de `Tri`, `Nat`, `Frame`, C01, C02 o C03.

## Estado material

El núcleo contiene actualmente:

```text
Tri = {0, 1, U}
Nat de precisión decimal no limitada por el tamaño de palabra
Frame con cierre estructural y causal J-F0…J-F5
C01: separación de captura/admisibilidad respecto de Tri
C02: revisión identificada de U sin clausura automática
C03: cierre relacional y causal de Frame
```

Las versiones observables integradas permanecen:

```text
Gramática   = 0.2
IR          = 0.3
serializador = 0.1.0
```

Los identificadores Rust `Zero` y `One` son internos. La representación textual canónica del Lenguaje permanece `0`, `1`, `U`.

## Naturales

La gramática define `nat` como una secuencia decimal sin cota semántica y la IR declara `Frame.index : Nat`. Por ello, `sv_core` no representa ese índice mediante `u64`, `usize` ni otro entero limitado por la plataforma.

`Nat` conserva el valor mediante una representación decimal canónica de precisión arbitraria. En el alcance actual sólo se requieren identidad y conservación exacta; no se exponen aritmética ni relación de orden sobre naturales. Los ceros iniciales se normalizan y el material no decimal se rechaza antes de constituir el valor.

## `Frame` y C03

C03 es el cierre relacional y causal de `Frame`; no existe un segundo constructor ni una segunda semántica para esta propiedad. La constitución de `Frame` aplica J-F0…J-F5 sobre relaciones previamente resueltas:

- cada `CoupledState` pertenece a un nodo de `Frame.architecture`;
- no se repite una referencia de estado ni existe más de un estado por nodo de arquitectura;
- la identidad de nodo se determina por `CoupledSpec`, no por el `CellSpec` subyacente;
- cada `EvalResult` procede de un estado incluido y no se duplica una misma fuente material;
- cada `GateResult` depende exclusivamente de evaluaciones incluidas;
- cada `SupervisionResult` mantiene `meta_eval` y objetivo dentro del mismo cierre;
- `SystemTarget` coincide con `Frame.architecture`;
- `criticalities` permanece vacío mientras no exista un productor superficial constituido de `CriticalityResult`.

La comprobación exige coherencia de lo declarado y **no impone exhaustividad**.

Las estructuras auxiliares `Resolved*` son internas a `sv_core`. No forman parte de la interfaz pública y no conceden a los adaptadores autoridad para declarar por sí mismos que una relación ha sido resuelta.

Toda violación del cierre materializado se identifica mediante `E308` (`FrameClosureViolation`).

La batería específica de C03 revalida el cierre completo después de la incorporación de C01 y C02 sin introducir nueva lógica de producción: acepta un subconjunto declarado coherente y rechaza escapes causales en evaluaciones, compuertas, supervisión y criticidades no producibles.

## C01 — captura y admisibilidad

La captura y la admisibilidad técnicas permanecen separadas de la semántica ternaria:

```text
CaptureOutcome::Bottom ↛ Tri
NotAdmitted            ↛ Tri
fallo técnico          ↛ Tri.U
```

`AdmissibilityState` es un conjunto cerrado representado por:

```text
Ok
Degraded
NotAdmitted
```

`Ok` y `Degraded` son estados positivamente admitidos; `NotAdmitted` no. Esta clasificación no produce por sí misma ningún valor de `Tri`.

Las etiquetas heredadas `Failed` y `U` no pertenecen a `AdmissibilityState`. Las infracciones materializadas de `AdmissibilitySpec` se identifican mediante `E110` (`InvalidAdmissibilitySpec`).

Una observación admitida sólo puede alcanzar legítimamente `Tri.U` por una ternarización semántica cuya `partition_u` corresponda; no existe conversión automática desde la admisibilidad.

## C02 — revisión identificada de `U`

La revisión se dirige a una ocurrencia constituida e identificable:

```text
ResolutionTarget = (EvaluableStateRef, position)
```

La posición es uno-basada. El objetivo debe ser un `CellState` o `CoupledState` evaluable y el valor efectivo de la posición debe ser exactamente `Tri.U`. En `CoupledState` se utiliza el vector actualizado como vector efectivo.

`ResSpec` conserva las identidades de contexto y mecanismo. Mientras no exista una relación ampliada expresamente constituida, la instancia de revisión debe coincidir exactamente con ambas identidades. Las violaciones se identifican mediante `E305` (`UnsafeUResolution`).

`ResolutionRecord` separa:

```text
previous
reviewed_to
resolved_to
```

El material de revisión no reescribe el estado objetivo ni constituye clausura positiva. En el circuito actualmente materializado:

```text
previous    = U
resolved_to = U
```

incluso cuando `reviewed_to` proponga `0` o `1`. Por tanto, permanece representable `U → revisión → U`.

La constitución de `ResolutionRecord` permanece dentro de `sv_core`; los adaptadores no disponen de una construcción pública que permita fabricar una clausura positiva.

## Fronteras actuales

Este núcleo no acredita todavía:

- analizador léxico o sintáctico Rust completo;
- resolución general de símbolos enlazada;
- transformación completa de la IR 0.3 a representación soberana;
- serialización canónica completa;
- autoridad externa de clausura positiva;
- realización completa de `Ternarizer`;
- sustitución del Playground Python/Pyodide;
- Garantía I;
- Garantía II.

La invalidez técnica de una interfaz o plataforma permanece fuera de `Tri` y no se transforma en `U`.

## Comprobación

La integración continua comprueba:

1. pruebas nativas del espacio de trabajo Rust;
2. pruebas de documentación vigentes;
3. compilación del mismo `sv_core` para `wasm32-unknown-unknown`;
4. compilación de `sv_wasm` para ese destino;
5. conservación independiente de la conformidad del frontend Python.

La unicidad semántica no se deduce únicamente de una compilación correcta: se conserva estructuralmente porque los adaptadores dependen de `sv_core` y no contienen una implementación paralela de las propiedades constitutivas.
