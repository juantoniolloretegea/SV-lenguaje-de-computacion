# Decisión FFL-B — gobierno de diagnósticos efectivos bajo Vía B

**Fecha:** 18/08/2026  
**Ámbito:** `FFL-B — cadena de implementación`  
**Estado:** vigente  
**Autoridad normativa superior:** `IR_CANONICA_BIENFORMACION_SV_v0_2.md`  
**Decisión diagnóstica previa:** `C1C_DECISION_REGULARIZACION_CONTRATO_DIAGNOSTICO.md`

## 1. Objeto

La reapertura por Ruta A y el cierre gobernado de FFL-A permiten continuar la revisión de la cadena formada por análisis sintáctico, validación y descenso a IR.

En FFL-B aparecen obligaciones canónicas que la superficie vigente puede representar, pero algunos identificadores de error de la IR v0.2 están ocupados por significados distintos en el catálogo efectivo. Esta decisión fija su tratamiento sin renumeración masiva, sin corrección silenciosa de la IR y sin atribuir al catálogo efectivo autoridad normativa paralela.

## 2. Regla de materialización

Una obligación canónica todavía no implementada sólo podrá materializarse en FFL-B cuando concurran simultáneamente estas condiciones:

1. la obligación consta materialmente en la IR vigente;
2. la gramática, el AST y la IR operativa contienen los datos necesarios para comprobarla;
3. la comprobación no exige introducir semántica nueva, nueva sintaxis ni un objeto doctrinal todavía ausente;
4. existe una ruta diagnóstica inequívoca;
5. la modificación puede comprobarse mediante un caso de conformidad o un caso negativo específico.

Si falta alguna de estas condiciones, la obligación permanece como deuda localizada y no se atribuye un cierre que la evidencia no permita sostener.

## 3. Colisiones de identificador

Cuando el identificador canónico de una obligación tenga ya otro significado en el contrato efectivo, no podrá reutilizarse silenciosamente.

Si la obligación debe materializarse en ese estado, se empleará un **identificador efectivo nuevo y libre**, situado en la familia correspondiente, y se documentará expresamente la relación:

`obligación/ID canónico → ID efectivo → lugar de emisión → prueba`.

Esta regla mantiene la Vía B y evita tanto una convergencia aparente por compartir número como una falsa ausencia funcional cuando el diagnóstico efectivo utiliza otro identificador.

## 4. Obligaciones materializables en la superficie vigente

La revisión de FFL-B identificó como materializables, siempre mediante cierres separados y evidencia propia:

- en `CoupledState`, la prohibición de modificar posiciones fuera de `BridgeSet`;
- en `CompositionGraph`, el tipado de nodos y referencias estructurales de aristas ya representados;
- la compatibilidad comprobable entre arista, posición puente y conector dentro de los datos declarados;
- en `TransitionData`, la pertenencia de los tipos de suceso al horizonte referenciado y la exigencia mínima de cambio inducido cuando exista correspondencia exacta con los juicios canónicos.

La relación anterior identifica posibilidades de descenso técnico; no constituye una autorización de implementación conjunta.

## 5. Obligaciones cuyo cierre no procede atribuir

No se materializarán por aproximación mientras la representación vigente no aporte la información necesaria. Entre ellas figuran:

- la procedencia completa de cada actualización de `CoupledState` desde un conector concreto;
- la semántica general de concurrencia cuando no exista un `ConflictOperator` tipado y comprobable en la superficie vigente;
- los resultados ejecutivos de `EvalResult` o `GateResult` que la etapa frontal del compilador todavía no calcula;
- las obligaciones de consulta, justificación o ABI que correspondan a FFL-E o a una etapa ejecutiva posterior.

La ausencia de ejecución material no se convertirá artificialmente en un error de la etapa frontal.

## 6. Regla documental y de prueba

Todo nuevo diagnóstico efectivo creado bajo esta decisión deberá actualizar, en el mismo cierre técnico:

- `src/svp_errors.py`;
- el lugar real de emisión;
- `tests/run_conformance.py` y el caso específico correspondiente;
- el catálogo público efectivo;
- la matriz de concordancia por identificador;
- la tabla de correspondencias funcionales IR ↔ implementación;
- el registro de deuda viva cuando cambie su estatuto.

La IR v0.2 no se modificará como consecuencia automática de esta materialización.

## 7. Secuenciación

Cada obligación se tratará de forma aislada, con alcance expresamente delimitado. La modificación deberá ser proporcional al juicio comprobado y no incluir cambios ajenos a él.

La primera aplicación de esta regla fue la parte verificable de J2.2 relativa a posiciones puente. Los cierres posteriores deberán volver a comprobar el repositorio fresco antes de seleccionar una nueva obligación.

## 8. Decisión

**FFL-B puede continuar mediante convergencia local, trazable y subordinada bajo Vía B.**

No se autoriza renumeración masiva, expansión de gramática, nueva IR, infraestructura de ejecución ni atribución de cierre a obligaciones cuya evidencia material no exista.
