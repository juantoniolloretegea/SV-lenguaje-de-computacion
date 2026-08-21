# SEC.0 — modelo ejecutable de referencia y batería adversarial inicial

## 1. Objeto

Este directorio contiene una primera materialización ejecutable de obligaciones seleccionadas de los contratos SEC.0-A, SEC.0-D, SEC.0-M, SEC.0-X y SEC.0-T.

Su finalidad es proporcionar objetos mínimos, mutaciones y casos adversariales falsables que permitan comprobar que las reglas abstractas pueden traducirse a decisiones ejecutables sin modificar su significado.

No constituye un entorno de ejecución de producción, no modifica la gramática ni la IR v0.2, no introduce códigos diagnósticos y no certifica una plataforma material.

## 2. Archivos

- `reference_model.py`: modelo ejecutable mínimo de autoridad, comprobación, fallo cerrado, presupuesto de recursos, continuidad, raíz de confianza, atestación, presentación humana y contrato de pruebas.
- `../run_sec0_contracts.py`: ejecutor de la batería adversarial inicial.

El ejecutor histórico `tests/run_sec0_smoke.py` se conserva separado. Sus tres casos pertenecen a la línea previa de resistencia del compilador y no deben interpretarse como cobertura de los contratos SEC.0-A/D/M/X/T.

## 3. Propiedades materializadas en esta primera batería

La batería comprueba, entre otras, las siguientes propiedades:

- una comprobación no crea autoridad;
- una forma controlada con requisitos vacíos o incompletos queda bloqueada;
- `D-R` y `D-N` no se convierten en éxito;
- un verificador no puede acreditar por sí mismo la autoridad necesaria para su uso;
- una vista derivada no sustituye a una fuente autoritativa inaccesible;
- una forma repetible exige un presupuesto de recursos válido;
- la atención humana debe presupuestarse cuando una forma pueda consumirla de manera acumulada;
- un testigo clonable junto con el estado no acredita consumo único;
- la recuperación de una raíz comprometida exige una vía independiente frente al mismo fallo;
- una atestación antigua no satisface una obligación de actualidad;
- omitir de `TCB(G)` un componente capaz de falsear la garantía invalida la declaración;
- presentación y firma deben referirse a la misma revisión material;
- una comprobación de construcción sólo excluye una herramienta si es independiente frente al mismo fallo;
- la aplicabilidad de una clase de prueba deriva de las capacidades del `SUT`;
- un caso nominal sin mutación ejercida no constituye cobertura;
- una inyección que no alcanza el objetivo no constituye cobertura;
- `INCONCLUSO` no constituye cobertura;
- un observador que elimina el fallo no produce un `PASS` transferible a la realización ordinaria;
- la evidencia pública debe ser independiente frente al mismo fallo para el que se invoca.

## 4. Ejecución

Desde la raíz del repositorio:

```bash
python tests/run_sec0_contracts.py
```

El ejecutor no requiere dependencias externas a la biblioteca estándar de Python.

## 5. Interpretación de resultados

Un resultado íntegramente satisfactorio acredita únicamente que **este modelo ejecutable de referencia** conserva las propiedades ensayadas por los casos presentes.

No acredita todavía:

- conformidad completa de la implementación vigente del Lenguaje SV con todos los invariantes SEC.0;
- resistencia de una plataforma de producción;
- independencia física de testigos o componentes;
- persistencia material real;
- atestación real;
- aislamiento efectivo de CPU, memoria o almacenamiento;
- inexistencia de vías laterales no ensayadas.

Las propiedades no materializadas deben permanecer fuera de cualquier afirmación de conformidad completa hasta disponer de una realización y evidencia suficientes.

## 6. Regla de evolución

Cuando una prueba integral descubra una violación, el escenario original debe conservarse, el fallo debe reducirse a un caso mínimo, la corrección debe eliminar la causa y el caso reducido debe incorporarse como regresión permanente. La regresión no sustituye al escenario integral que descubrió el defecto.
