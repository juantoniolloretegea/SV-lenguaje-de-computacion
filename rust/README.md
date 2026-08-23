# Realización Rust de R0

Esta carpeta contiene el inicio de la realización Rust del núcleo semántico del Lenguaje SV.

## Alcance del corte inicial

```text
sv_core
  ├─ Tri = {0, 1, U}
  ├─ representación Rust: Zero = 0, One = 1, U = 2
  └─ versiones canónicas: Gramática 0.2 / IR 0.3 / serializador 0.1.0

sv_wasm
  └─ adaptador WebAssembly mínimo que delega la semántica ternaria en sv_core
```

Los nombres `Zero` y `One` son identificadores internos de Rust. La representación textual canónica del Lenguaje SV permanece `0`, `1`, `U`.

El adaptador WebAssembly no constituye un segundo motor semántico. Su función es exponer el mismo núcleo a un destino de ejecución distinto.

## Fronteras

Este corte no contiene todavía:

- analizador léxico o sintáctico Rust completo;
- transformación completa a IR 0.3;
- serialización canónica completa;
- sustitución del Playground Python/Pyodide;
- garantías materiales del sistema completo.

La invalidez técnica de la interfaz binaria WebAssembly permanece fuera de `Tri`; no se transforma en `U`.

## Comprobación

La integración continua comprueba:

1. las pruebas nativas del espacio de trabajo Rust;
2. la compilación del mismo `sv_core` para `wasm32-unknown-unknown`;
3. la compilación de `sv_wasm` para ese mismo destino.

La ausencia de una segunda semántica no se deduce únicamente de que ambas compilaciones sean correctas. En este corte se conserva estructuralmente porque `sv_wasm` depende de `sv_core` y delega en él la validación y representación ternarias, sin declarar otro tipo `Tri` ni otra tabla de correspondencia semántica.

La ampliación posterior deberá conservar una única fuente semántica compartida por los destinos nativo y WebAssembly.
