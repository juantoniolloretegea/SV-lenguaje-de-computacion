# SEC.0 — modelo ejecutable de referencia y batería adversarial inicial

## 1. Objeto

Este directorio contiene una primera materialización ejecutable de obligaciones seleccionadas de los contratos SEC.0-A, SEC.0-D, SEC.0-M, SEC.0-X y SEC.0-T.

Su finalidad es proporcionar objetos mínimos, mutaciones y casos adversariales falsables que permitan comprobar que determinadas reglas abstractas pueden traducirse a decisiones ejecutables sin modificar su significado.

No constituye un entorno de ejecución de producción, no modifica la gramática ni la IR v0.2, no introduce códigos diagnósticos y no certifica una plataforma material.

## 2. Archivos

- `reference_model.py`: modelo mínimo de autoridad constituida, formas protegidas, comprobación, fallo cerrado, presupuestos acumulativos, consumo único lógico, almacenamiento autoritativo frente a vista derivada, raíz de confianza, atestación, ligadura presentación-firma, aplicabilidad de clases y ejecución de mutaciones.
- `../run_sec0_contracts.py`: ejecutor de la batería adversarial inicial.

El ejecutor histórico `tests/run_sec0_smoke.py` se conserva separado. Sus tres casos pertenecen a la línea previa de resistencia del compilador y no deben interpretarse como cobertura de los contratos SEC.0-A/D/M/X/T.

## 3. Endurecimientos exigidos por los contratos

La realización de referencia conserva expresamente las separaciones siguientes:

1. **representación de autoridad ≠ autoridad constituida**: construir directamente un objeto `Authority` no basta para que el motor lo acepte como autoridad;
2. **clase declarada ≠ forma constituida**: una forma protegida necesita un descriptor constituido y no puede obtener validez por asignar localmente una clase T-*;
3. **presupuesto por acto ≠ presupuesto acumulativo**: los recursos consumidos se acumulan en `ExecutionLedger` y una sucesión de actos individualmente pequeños puede agotar el presupuesto;
4. **presencia de atención humana ≠ atención ilimitada**: cuando una forma consume atención humana, ese consumo debe declararse y se acumula como cualquier otro recurso gobernado;
5. **testigo declarado ≠ consumo único**: una autorización consumible necesita un registro lógico de consumo y una premisa externa de independencia aplicable al mismo fallo; una segunda utilización queda bloqueada;
6. **bandera de mutación ≠ falsabilidad**: `exercise_mutation()` ejecuta materialmente una alteración sobre el objeto de prueba y comprueba por observación si alcanzó el objetivo y si el criterio distingue el cambio;
7. **independencia representada ≠ independencia física demostrada**: `IndependencePremise` representa una premisa externa admitida. El modelo no puede demostrar desde Python que dos componentes sean físicamente independientes frente a un mismo fallo.

Estas separaciones evitan que el modelo utilice como evidencia precisamente aquello que los contratos SEC.0 obligan a demostrar fuera del modelo.

## 4. Propiedades materializadas en la batería

La batería comprueba, entre otras, las siguientes propiedades:

- una autoridad fabricada por construcción ordinaria no se acepta como constituida;
- una forma con descriptor no constituido queda bloqueada;
- una comprobación no crea autoridad;
- una forma controlada con requisitos vacíos o incompletos queda bloqueada;
- `D-R` y `D-N` no se convierten en éxito;
- un verificador no puede acreditarse a sí mismo mediante el mismo acto;
- una vista derivada no sustituye a una fuente autoritativa inaccesible;
- una forma repetible exige presupuesto y consumo declarados;
- los límites de recursos se aplican a la acumulación, no sólo a cada llamada aislada;
- la atención humana debe presupuestarse y su consumo acumulado puede bloquear nuevos actos;
- un testigo cuya premisa de independencia corresponde a otro fallo no acredita consumo único;
- una autorización de un solo uso queda consumida tras el primer compromiso lógico y un replay posterior se rechaza;
- una raíz comprometida no se legitima a sí misma para rotar;
- una atestación antigua no satisface una obligación de actualidad;
- omitir de `TCB(G)` un componente capaz de falsear la garantía invalida la declaración dentro del modelo;
- presentación y firma deben referirse a la misma revisión material;
- una comprobación de construcción sólo puede apoyarse condicionalmente en una premisa de independencia respecto del mismo fallo;
- la aplicabilidad de una clase de prueba deriva de las capacidades del `SUT`;
- una mutación que no alcanza el objetivo no constituye cobertura;
- un cambio de etiqueta que no altera el comportamiento observado no cubre una mutación semántica;
- `INCONCLUSO` no constituye cobertura;
- un observador que elimina el fallo no produce un `PASS` transferible a la realización ordinaria;
- la evidencia pública sólo puede considerarse independiente de forma condicional a una premisa externa que corresponda al mismo fallo.

## 5. Ejecución

Desde la raíz del repositorio:

```bash
python tests/run_sec0_contracts.py
```

El ejecutor no requiere dependencias externas a la biblioteca estándar de Python.

La cantidad de casos puede aumentar a medida que se incorporen regresiones. El número de casos superados sólo acredita los casos realmente presentes en la revisión ejecutada.

## 6. Interpretación de resultados

Un resultado íntegramente satisfactorio acredita únicamente que **este modelo ejecutable de referencia** conserva las propiedades efectivamente ensayadas por los casos presentes.

No acredita todavía:

- conformidad completa de la implementación vigente del Lenguaje SV con todos los invariantes SEC.0;
- que los constructores lógicos de este modelo sean mecanismos de seguridad frente a código Python hostil;
- autenticidad criptográfica real de autoridades, firmas o artefactos;
- independencia física de testigos, raíces, observadores o componentes;
- persistencia material real;
- consumo único frente a clonación real de procesos, máquinas o almacenamiento;
- atestación real;
- aislamiento efectivo de CPU, memoria o almacenamiento;
- suficiencia del modelo de recursos para una plataforma concreta;
- inexistencia de vías laterales no ensayadas.

En particular, una `IndependencePremise` no debe reinterpretarse como demostración de independencia material. Su función es hacer explícito el punto en el que la futura realización deberá aportar evidencia externa al propio sistema sometido al fallo.

`Verification` representa asimismo un resultado de comprobación precomputado: este modelo prueba qué debe ocurrir con `D-A`, `D-R` o `D-N`, pero no autentica por sí mismo el verificador real ni la evidencia que originó ese resultado.

`signature_matches_presentation()` comprueba únicamente la ligadura de objeto, revisión y resumen representado; no implementa una firma criptográfica ni acredita identidad humana.

## 7. Regla de evolución

Cuando una prueba integral descubra una violación, el escenario original debe conservarse, el fallo debe reducirse a un caso mínimo, la corrección debe eliminar la causa y el caso reducido debe incorporarse como regresión permanente. La regresión no sustituye al escenario integral que descubrió el defecto.

Una ampliación del modelo sólo es admisible cuando representa una obligación contractual ya cerrada o una premisa externa explícita. No debe introducir por conveniencia una nueva semántica del Lenguaje, una plataforma concreta ni una garantía material que no pueda acreditarse en este nivel.
