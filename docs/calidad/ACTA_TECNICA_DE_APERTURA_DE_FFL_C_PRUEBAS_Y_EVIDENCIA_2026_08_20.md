# Acta técnica de apertura de FFL-C — Pruebas y evidencia

**Fecha:** 20/08/2026  
**Hora (Europe/Madrid):** 07:21  
**Estado:** ABIERTO  
**Ámbito:** Lenguaje SV / FFL-C / pruebas y evidencia

## 1. Hecho

FFL-B consta cerrado con deuda técnica delimitada y evidencia acumulada de conformidad de 57/57 casos, pruebas rápidas de la interfaz de línea de órdenes de 3/3 y SEC-0 de 3/3.

Se abre FFL-C como bloque dedicado exclusivamente a comprobar si la evidencia reproducible disponible sostiene las afirmaciones técnicas ya publicadas y a persistir las comprobaciones que falten para justificar ese estado.

FFL-D y FFL-E permanecen pendientes y no se abren mediante esta acta.

## 2. Objeto

FFL-C comprende la relación entre:

- conformidad;
- pruebas rápidas;
- SEC-0;
- cobertura observable;
- correspondencia entre las afirmaciones técnicas publicadas y las pruebas que las sustentan.

La sede funcional de escritura es `tests/`.

## 3. Límite de escritura

Durante FFL-C permanecen en modo de solo lectura, salvo decisión expresa posterior:

- `src/`;
- la gramática superficial mínima;
- el AST;
- la IR v0.2;
- el validador;
- el catálogo diagnóstico;
- `docs/manual_svp/`;
- la infraestructura de ejecución futura.

FFL-C no introduce semántica nueva, no modifica diagnósticos y no amplía el lenguaje.

## 4. Comprobaciones iniciales autorizadas

La apertura comprende las siguientes comprobaciones:

1. inventariar la cobertura observable de las afirmaciones técnicas vigentes;
2. incorporar como caso válido permanente `SystemTarget(CompositionGraph)`, ya comprobado previamente de forma no persistente;
3. caracterizar mediante pruebas la doble utilización observable de `E006`: referencia inexistente y referencia existente de tipo incompatible, sin modificar el código, el nombre, el mensaje ni el validador;
4. comprobar la correspondencia entre los ejecutores de pruebas, sus denominaciones históricas y las afirmaciones públicas de cobertura;
5. conservar las denominaciones históricas de archivos y ejecutores; en nueva documentación se empleará «pruebas rápidas» cuando corresponda.

## 5. Exclusiones

No forman parte de FFL-C:

- `ConflictOperator` y las obligaciones no representables del régimen `General`;
- la ejecución de `gate` o el cálculo de `GateResult.output`;
- la semántica ejecutiva completa de `SupervisionResult`, `verdict` o `Veto`;
- la producción de `CriticalityResult`;
- la ampliación de la procedencia de `CoupledState`;
- el desarrollo del manual;
- las bibliotecas del sistema;
- Rust;
- AUTH;
- IA propia;
- NL→SVP.

## 6. Criterio de cierre

FFL-C podrá cerrarse cuando exista evidencia reproducible suficiente para sostener el estado técnico publicado, las ausencias relevantes de cobertura hayan quedado identificadas y las comprobaciones necesarias se encuentren persistidas o expresamente delimitadas.

El cierre documental y registral se realizará en un lote separado de las modificaciones de pruebas.

## 7. Estado

FFL-C queda abierto. FFL-D y FFL-E permanecen pendientes. FFL-B no se reabre por esta decisión.
