# Decisión FFL-B — gobierno de diagnósticos efectivos bajo Vía B

**Fecha:** 18/08/2026  
**Ámbito:** `FFL-B — cadena de implementación`  
**Estado:** vigente  
**Autoridad normativa superior:** `IR_CANONICA_BIENFORMACION_SV_v0_2.md`  
**Decisión diagnóstica previa:** `C1C_DECISION_REGULARIZACION_CONTRATO_DIAGNOSTICO.md`

## 1. Objeto

La reapertura por Ruta A y el cierre gobernado de FFL-A han permitido continuar la auditoría de la cadena `parser → validator → lowering`. En FFL-B aparecen obligaciones canónicas que la superficie actual puede representar, pero cuyos identificadores de error en la IR v0.2 ya están ocupados por significados distintos en el catálogo implementativo efectivo.

Esta decisión fija cómo tratar esos casos sin renumeración masiva, sin corregir silenciosamente la IR y sin convertir el catálogo efectivo en autoridad paralela.

## 2. Regla de materialización

Una obligación canónica todavía no implementada solo podrá materializarse en FFL-B cuando concurran simultáneamente estas condiciones:

1. la obligación conste materialmente en la IR vigente;
2. la gramática, AST e IR operativa actuales contengan ya los datos necesarios para comprobarla;
3. la comprobación no exija introducir semántica nueva, nueva sintaxis ni un objeto doctrinal todavía ausente;
4. exista una ruta diagnóstica no ambigua;
5. la modificación pueda probarse con un caso de conformidad o adversarial específico.

Si falta alguna condición, la obligación permanece como deuda localizada y no se simula su cierre.

## 3. Colisiones de identificador

Cuando el identificador canónico de una obligación ya tenga otro significado en el contrato efectivo, queda prohibido reutilizarlo silenciosamente.

En tal caso, si la obligación debe materializarse ahora, se empleará un **identificador efectivo nuevo y libre**, situado en la familia implementativa correspondiente, y se documentará expresamente el cruce:

`obligación/ID canónico → ID efectivo → sitio de emisión → prueba`.

Esto mantiene vigente la Vía B y evita dos errores:

- falsa convergencia por compartir número;
- falsa ausencia funcional porque el número efectivo sea distinto.

## 4. Obligaciones inmediatamente materializables

La auditoría actual identifica como materializables, por disponer ya de estructura suficiente, al menos:

- en `CoupledState`, la prohibición de modificar posiciones fuera de `BridgeSet`;
- en `CompositionGraph`, el tipado de nodos y referencias estructurales de aristas, ya corregidos mediante validación genérica;
- la compatibilidad comprobable entre arista, posición puente y conector, si se mantiene dentro de los datos ya declarados;
- en `TransitionData`, la pertenencia de los tipos de suceso al horizonte referenciado y la exigencia mínima de cambio inducido, si se confirma su correspondencia exacta con los juicios canónicos vigentes.

Cada punto deberá abrirse y cerrarse por separado. Esta lista no autoriza una implementación en bloque.

## 5. Obligaciones que no deben fingirse cerradas

No se materializarán por aproximación mientras la representación actual no aporte la información necesaria:

- procedencia completa de cada actualización de `CoupledState` desde un conector concreto;
- semántica completa de concurrencia en régimen general si no existe un `ConflictOperator` tipado y verificable en la superficie vigente;
- resultados ejecutivos de `EvalResult` o `GateResult` que el frontend actual no ejecuta;
- obligaciones de consulta, justificación o ABI que pertenezcan a FFL-E o a una capa ejecutiva posterior.

La ausencia de ejecución material no se convertirá en un error artificial de frontend.

## 6. Regla documental y de prueba

Todo nuevo diagnóstico efectivo creado bajo esta decisión deberá actualizar en el mismo bloque:

- `src/svp_errors.py`;
- sitio real de emisión;
- `tests/run_conformance.py` y caso específico;
- catálogo público efectivo;
- matriz de concordancia por identificador;
- crosswalk funcional IR ↔ frontend;
- deuda viva, si cambia su estatuto.

La IR v0.2 no se modificará como consecuencia automática de esta materialización.

## 7. Primer candidato

El primer candidato autorizado para tratamiento aislado es la parte verificable de J2.2:

> si `base_vector` y `updated_vector` difieren, toda posición modificada debe pertenecer al `BridgeSet` del `CoupledSpec`.

La cláusula adicional de procedencia desde un `Connector` no se dará por cerrada mientras esa procedencia no esté representada de forma verificable.

Por existir colisión con el `E202` efectivo actual (`GateInputNotEvalResult`), esta obligación no podrá usar `E202` sin una migración diagnóstica formal distinta.

## 8. Cierre

**FFL-B puede continuar con convergencia local y trazable bajo Vía B.**

No se autoriza renumeración masiva, expansión de gramática, nueva IR, backend ni cierre ficticio de obligaciones cuya evidencia no exista.
