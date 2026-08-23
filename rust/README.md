# Realización Rust de R0

Esta carpeta contiene la realización progresiva en Rust del núcleo semántico del Lenguaje SV, compartida por los destinos nativo y WebAssembly.

## Estado material

```text
R0-1
  sv_core
    ├─ Tri = {0, 1, U}
    ├─ representación Rust: Zero = 0, One = 1, U = 2
    └─ versiones canónicas: Gramática 0.2 / IR 0.3 / serializador 0.1.0

R0-2
  sv_core
    ├─ Nat sin cota dependiente del tamaño de palabra
    └─ Frame + cierre relacional mínimo J-F0…J-F5

sv_wasm
  └─ adaptador WebAssembly del mismo sv_core
```

Los nombres `Zero` y `One` son identificadores internos de Rust. La representación textual canónica del Lenguaje SV permanece `0`, `1`, `U`.

El adaptador WebAssembly no constituye un segundo motor semántico. Su función es exponer el mismo núcleo a un destino de ejecución distinto.

## Naturales en R0-2

La gramática define `nat` como una secuencia decimal sin cota semántica y la IR declara `Frame.index : Nat`. Por ello, `sv_core` no representa ese índice mediante `u64`, `usize` ni otro entero limitado por la plataforma.

`Nat` conserva el valor mediante una representación decimal canónica de precisión arbitraria. En R0-2 sólo se requieren identidad y conservación exacta del índice ordinal; no se introducen aritmética ni relación de orden sobre naturales. Los ceros iniciales se normalizan y el material que no sea decimal se rechaza antes de constituir el valor.

## `Frame` en R0-2

R0-2 materializa `Frame` como objeto constituible únicamente cuando las relaciones ya resueltas satisfacen el cierre estructural y causal de la IR 0.3:

- cada `CoupledState` pertenece a un nodo de `Frame.architecture`;
- no se repite una referencia de estado ni existe más de un estado por nodo de arquitectura;
- cada `EvalResult` procede de un estado incluido y no se duplica una misma fuente material;
- cada `GateResult` depende exclusivamente de evaluaciones incluidas;
- cada `SupervisionResult` mantiene `meta_eval` y objetivo dentro del mismo cierre;
- `SystemTarget` coincide con `Frame.architecture`;
- `criticalities` permanece vacío mientras no exista un productor superficial constituido de `CriticalityResult`.

La comprobación no impone exhaustividad: un `Frame` puede declarar sólo una parte coherente de los estados y resultados de su arquitectura.

Las estructuras auxiliares `Resolved*` son proyecciones internas de relaciones ya resueltas necesarias para comprobar este cierre. No crean nuevos tipos de la gramática ni de la IR canónica, no forman parte de la interfaz pública de `sv_core` y no conceden a los adaptadores autoridad para declarar por sí mismos que una relación ha sido resuelta. El análisis sintáctico, la resolución general de símbolos y el descenso completo a IR permanecen fuera de R0-2.

Toda violación de este cierre se identifica en el núcleo mediante el código canónico `E308` (`FrameClosureViolation`).

## Fronteras

Este corte no contiene todavía:

- analizador léxico o sintáctico Rust completo;
- resolución general de símbolos en Rust;
- transformación completa a IR 0.3;
- serialización canónica completa;
- materialización de C01–C03 como operaciones Rust;
- `resolve` soberano en Rust;
- sustitución del Playground Python/Pyodide;
- Garantía I o Garantía II.

La invalidez técnica de la interfaz binaria WebAssembly permanece fuera de `Tri`; no se transforma en `U`.

## Comprobación

La integración continua comprueba:

1. las pruebas nativas del espacio de trabajo Rust;
2. la compilación del mismo `sv_core` para `wasm32-unknown-unknown`;
3. la compilación de `sv_wasm` para ese mismo destino.

La unicidad semántica no se deduce únicamente de una compilación correcta. Se conserva estructuralmente porque `sv_wasm` depende de `sv_core` y no contiene una realización alternativa de `Tri`, `Nat` ni `Frame`.
