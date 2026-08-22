# SEC.0 — vectores adversariales independientes de la implementación

## 1. Objeto

Este directorio conserva la traducción de obligaciones seleccionadas de SEC.0-A, SEC.0-D, SEC.0-M, SEC.0-X y SEC.0-T a **vectores adversariales independientes de la implementación**.

Su finalidad es fijar qué condición debe atacarse, qué evidencia permite acreditar que el ataque alcanzó su objetivo y qué resultado exige el contrato. Los vectores podrán materializarse posteriormente contra el backend soberano y, cuando corresponda, contra el sistema completo.

Este directorio no contiene un backend, un entorno de ejecución ni una realización material de seguridad.

## 2. Referencia vigente

- `VECTORES_ADVERSARIALES_SEC0_V1.md`: catálogo inicial de vectores derivados de los contratos SEC.0 e independientes de la implementación.

El ejecutor histórico `tests/run_sec0_smoke.py` pertenece a la línea previa de resistencia del compilador. Sus tres casos no deben interpretarse como cobertura de SEC.0-A/D/M/X/T.

## 3. Estatuto de las realizaciones experimentales anteriores

Las realizaciones ejecutables de SEC.0 desarrolladas en Python se conservan en el historial del repositorio como evidencia de exploración y reducción de escenarios, pero no constituyen la referencia vigente de seguridad ni una realización soberana.

No se mantiene en el árbol actual una segunda implementación de autoridad, persistencia, continuidad, raíz de confianza o aislamiento destinada a anticipar la realización soberana.

El valor reutilizable se conserva en los vectores, criterios esperados y regresiones independientes del lenguaje de implementación.

## 4. Condición de ejecución futura

Un vector sólo deberá adquirir forma ejecutable cuando exista un sistema sometido a prueba identificable y pueda conservarse, conforme a SEC.0-T, evidencia suficiente de:

```text
TestRun
SUT
TestCase
Targets
ThreatModel
InitialState
InjectedFaults
ReachedFaults
Oracle
Observer
Expected
Observed
Verdict
Artifacts
```

`ReachedFaults` acredita el alcance efectivo del fallo o mutación sobre el objetivo. `Oracle` debe ser no circular frente a la misma clase de fallo y `Verdict` debe derivarse de `Expected` y `Observed`.

Las propiedades cuya verdad dependa de almacenamiento, recuperación, administración, raíces de confianza, aislamiento, comunicaciones u otras dependencias externas al proceso no pueden acreditarse únicamente mediante una prueba local del backend.

## 5. Relación con la doble garantía

Las pruebas derivadas de estos vectores contribuyen a la comprobación de conformidad, pero no constituyen por sí solas el cierre final.

La acreditación de una realización soberana exige conjuntamente:

1. construcción conforme a los contratos aplicables;
2. comprobación adversarial integral del sistema completo dentro del alcance y modelo de fallos declarados.

## 6. Regla de evolución

Todo fallo confirmado que revele una nueva vía causal debe conservarse como vector o regresión reutilizable. Cuando un escenario integral descubra una interacción, el caso reducido no sustituye al escenario integral original.

No debe añadirse una materialización ejecutable si su única finalidad es simular en Python una garantía que deberá imponerse posteriormente por la realización soberana o por infraestructura material externa.
