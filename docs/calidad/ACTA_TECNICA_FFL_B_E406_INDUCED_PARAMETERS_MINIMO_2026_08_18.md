# Acta técnica FFL-B — materialización mínima de `E406 — InsufficientTransitionData`

**Fecha:** 18/08/2026  
**Frente:** FFL-B — J4.3 / `TransitionData.induced_parameters`  
**Estado:** CERRADO  
**Rama de verificación:** `agent/ffl-b-e406-min-transition`  
**Base:** `daa7ac6980b6f16c27207442c50a4eace09603c9`  
**HEAD funcional auditado:** `a1d3d98b1971fe3cb7c1941dc1b3d9ce41b1e54c`  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ISSN:** 2695-6411

---

## 1. Objeto

Materializar de forma estrictamente mínima la obligación diagnóstica de `J4.3` según la cual un `TransitionData` debe especificar al menos un cambio en `induced_parameters`.

Este microbloque no modifica la doctrina del Suceso, no amplía la gramática, no altera la IR canónica, no introduce un nuevo nodo y no atribuye al frontend capacidad de reconstrucción ejecutiva que todavía no esté acreditada.

## 2. Fundamento canónico

La IR canónica v0.2 declara para `TransitionData` el campo:

`induced_parameters : [(NodeId, Nat, Tri)]`.

En `J4.3 — Transición bien formada` exige, entre otras condiciones, que:

- `induced_parameters` especifique al menos un cambio de parámetro;
- los datos sean suficientes para reconstruir el operador inducido a partir de `induced_parameters`.

La tabla canónica de errores asigna expresamente:

`E406 — InsufficientTransitionData — J4.3 — induced_parameters vacío`.

La presente materialización se limita a esta última correspondencia literal.

## 3. Microauditoría material previa

La lectura fresca del repositorio acreditó que la información necesaria ya existía en todos los niveles previos al validator:

1. la gramática superficial declara `induced_parameters` como lista de `induced_param_literal`;
2. el AST conserva `TransitionDataDecl.induced_parameters` como lista;
3. el parser admite una lista vacía mediante el mecanismo general de listas;
4. el lowering transporta la lista al objeto IR de `TransitionData` sin alterar su cardinalidad;
5. el validator comprobaba ya `horizon_ref` y la pertenencia de tipos de suceso, pero no comprobaba la no-vaciedad de `induced_parameters`.

Por tanto, la obligación era alcanzable y comprobable sin nueva sintaxis ni nueva semántica.

## 4. Adversarial de alcance

La no-vaciedad de `induced_parameters` es condición necesaria, pero no demuestra por sí sola la cláusula más fuerte de `J4.3` relativa a la suficiencia de los datos para reconstruir el operador inducido.

En consecuencia:

- **sí procede** materializar `induced_parameters == [] → E406`;
- **no procede** declarar cerrado todo `J4.3`;
- **no procede** inferir que una lista no vacía sea necesariamente suficiente para reconstruir el operador inducido;
- **no procede** introducir ahora un comprobador de reconstructibilidad no definido materialmente por la superficie vigente.

La parte no acreditada conserva su estatuto propio y no queda absorbida por este microcierre.

## 5. Decisión diagnóstica

No existía colisión del identificador `E406` en el catálogo implementativo efectivo anterior al parche.

Dado que el nombre, el identificador y la condición material coinciden exactamente con la IR v0.2, se incorpora el mismo diagnóstico:

`E406 — InsufficientTransitionData`.

No se crea alias efectivo ni se reutiliza otro código por semejanza.

## 6. Implementación acotada

El lote funcional quedó limitado a:

- alta de `E406` en `src/svp_errors.py` y en `ERRORS`;
- importación de `E406` en `src/svp_validator.py`;
- una única comprobación en `_validate_transition_data`:

  `if not node.induced_parameters: raise E406`;

- un caso adversarial específico `tests/conformance/invalid/transition_induced_parameters_vacios.svp`;
- una única entrada esperada `E406` en `tests/run_conformance.py`.

No se modificaron gramática, IR canónica, AST, parser, lowering, serializer, CLI, runner ejecutivo, Playground, stdlib, backend, Rust ni WASM.

## 7. Sincronización diagnóstica

En el mismo lote funcional se actualizaron:

- `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`;
- `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`;
- `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`;
- `docs/calidad/CROSSWALK_FUNCIONAL_DE_OBLIGACIONES_DIAGNOSTICAS_IR_V0_2_Y_FRONTEND_VIA_B_2026_08_18.md`;
- `docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`;
- esta acta.

La clasificación por identificador pasa de cuatro a cinco coincidencias semánticas, y `E406` deja de figurar como `solo_ir` para la cláusula materializada.

## 8. Evidencia dinámica y cierre

La rama `agent/ffl-b-e406-min-transition`, con `HEAD` funcional `a1d3d98b1971fe3cb7c1941dc1b3d9ce41b1e54c` y base exacta `daa7ac6980b6f16c27207442c50a4eace09603c9`, fue ejecutada en solo lectura por una unidad auditora independiente, sin commits ni parches de esa unidad.

La adversarial dinámica acreditó:

- `tests/run_conformance.py`: **45/45**, `rc=0`;
- `tests/run_cli_smoke.py`: **3/3**, `rc=0`;
- `tests/run_sec0_smoke.py`: **3/3**, `rc=0`;
- `transition_induced_parameters_vacios.svp`: emisión exacta `E406 — InsufficientTransitionData`, causada por `induced_parameters` vacío;
- `transition_data_events.svp`: válido, `rc=0`, con producción de IR;
- `transition_event_fuera_horizon.svp`: emisión exacta `E307 — UndeclaredHorizonEvent`.

La verificación confirmó además que en `_validate_transition_data` el juicio `E307` se evalúa antes de `E406`, de modo que un tipo de suceso fuera del horizonte no queda ocultado por la nueva condición de no-vaciedad.

El diff exacto `daa7ac6980b6f16c27207442c50a4eace09603c9..a1d3d98b1971fe3cb7c1941dc1b3d9ce41b1e54c` contiene un solo commit. El cambio funcional queda restringido a:

- `src/svp_errors.py`: alta de `E406` y su inclusión en `ERRORS`;
- `src/svp_validator.py`: importación de `E406` y comprobación de lista vacía;
- `tests/run_conformance.py`: una entrada esperada;
- un único fixture nuevo: `transition_induced_parameters_vacios.svp`.

El resto del lote corresponde a documentación y sincronización del mismo juicio. No se constató remaquetación amplia ni modificación de gramática, IR canónica, AST, parser, lowering, serializer, CLI, Playground, stdlib, backend, Rust o WASM.

La evidencia externa señaló observaciones cosméticas no bloqueantes que no alteran el juicio funcional ni se incorporan a este cierre como cambios adicionales. Modificarlas después de la auditoría habría alterado innecesariamente el `HEAD` funcional verificado.

Con esta evidencia, **el microbloque E406 queda cerrado** en el alcance exacto de la no-vaciedad de `TransitionData.induced_parameters`.

## 9. Límite del cierre y continuidad

Este cierre no demuestra ni implementa la cláusula adicional de `J4.3` relativa a la suficiencia de los datos para reconstruir el operador inducido.

En particular, no se acredita por este microbloque:

- que todo triple de `induced_parameters` describa un cambio efectivo respecto de un estado anterior;
- que cada `NodeId` o posición quede validado aquí contra una arquitectura concreta;
- que una lista no vacía permita por sí sola reconstruir el operador inducido;
- ni que todo `J4.3` pueda darse por clausurado.

La continuidad de FFL-B deberá volver a partir del repositorio fresco y de la deuda funcional realmente restante. El cierre de E406 no autoriza automáticamente otro microcierre ni un nuevo frente serio.

No se abre backend, Rust, WASM, IA productiva, stdlib ni `NL → SVP`.

---

*Documento técnico subordinado del Lenguaje SV.*
