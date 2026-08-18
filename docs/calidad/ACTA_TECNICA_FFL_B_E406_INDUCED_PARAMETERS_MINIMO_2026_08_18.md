# Acta técnica FFL-B — materialización mínima de `E406 — InsufficientTransitionData`

**Fecha:** 18/08/2026  
**Frente:** FFL-B — J4.3 / `TransitionData.induced_parameters`  
**Estado:** APLICADO EN RAMA / PENDIENTE DE EVIDENCIA DINÁMICA  
**Rama:** `agent/ffl-b-e406-min-transition`  
**Base:** `daa7ac6980b6f16c27207442c50a4eace09603c9`  
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

La lectura fresca del repositorio acredita que la información necesaria ya existe en todos los niveles previos al validator:

1. la gramática superficial declara `induced_parameters` como lista de `induced_param_literal`;
2. el AST conserva `TransitionDataDecl.induced_parameters` como lista;
3. el parser admite una lista vacía mediante el mecanismo general de listas;
4. el lowering transporta la lista al objeto IR de `TransitionData` sin alterar su cardinalidad;
5. el validator vigente comprueba ya `horizon_ref` y la pertenencia de tipos de suceso, pero no comprobaba la no-vaciedad de `induced_parameters`.

Por tanto, la obligación es alcanzable y comprobable sin nueva sintaxis ni nueva semántica.

## 4. Adversarial de alcance

La no-vaciedad de `induced_parameters` es condición necesaria, pero no demuestra por sí sola la cláusula más fuerte de `J4.3` relativa a la suficiencia de los datos para reconstruir el operador inducido.

En consecuencia:

- **sí procede** materializar `induced_parameters == [] → E406`;
- **no procede** declarar cerrado todo `J4.3`;
- **no procede** inferir que una lista no vacía sea necesariamente suficiente para reconstruir el operador inducido;
- **no procede** introducir ahora un comprobador de reconstructibilidad no definido materialmente por la superficie vigente.

La parte no acreditada conserva su estatuto propio y no queda absorbida por este microcierre.

## 5. Decisión diagnóstica

No existe colisión del identificador `E406` en el catálogo implementativo efectivo anterior al parche.

Dado que el nombre, el identificador y la condición material coinciden exactamente con la IR v0.2, se incorpora el mismo diagnóstico:

`E406 — InsufficientTransitionData`.

No se crea alias efectivo ni se reutiliza otro código por semejanza.

## 6. Implementación acotada

El lote funcional queda limitado a:

- alta de `E406` en `src/svp_errors.py` y en `ERRORS`;
- importación de `E406` en `src/svp_validator.py`;
- una única comprobación en `_validate_transition_data`:

  `if not node.induced_parameters: raise E406`;

- un caso adversarial específico `tests/conformance/invalid/transition_induced_parameters_vacios.svp`;
- una única entrada esperada `E406` en `tests/run_conformance.py`.

No se modifican gramática, AST, parser, lowering, serializer, CLI, runner ejecutivo, Playground, stdlib, backend, Rust ni WASM.

## 7. Sincronización diagnóstica

En el mismo lote se actualizan:

- `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`;
- `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`;
- `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`;
- `docs/calidad/CROSSWALK_FUNCIONAL_DE_OBLIGACIONES_DIAGNOSTICAS_IR_V0_2_Y_FRONTEND_VIA_B_2026_08_18.md`;
- `docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`;
- esta acta.

La clasificación por identificador pasa, en el estado aplicado del lote, de cuatro a cinco coincidencias semánticas, y `E406` deja de figurar como `solo_ir` para la cláusula materializada.

## 8. Criterio de cierre dinámico

El microbloque no se declarará cerrado por la mera aplicación del commit. Requiere como mínimo:

1. inspección del diff frente a `daa7ac6980b6f16c27207442c50a4eace09603c9` y confirmación de radio estrictamente corto;
2. `tests/run_conformance.py` en **45/45**;
3. `tests/run_cli_smoke.py` en **3/3**;
4. `tests/run_sec0_smoke.py` en **3/3**;
5. `transition_induced_parameters_vacios.svp` fallando exactamente con `E406 — InsufficientTransitionData`;
6. `tests/conformance/valid/transition_data_events.svp` continuando válido y produciendo IR;
7. `transition_event_fuera_horizon.svp` continuando fallando exactamente con `E307`, para comprobar que la nueva condición no altera el juicio previo de pertenencia al horizonte;
8. ausencia de cambios en gramática, IR canónica, AST, parser y lowering.

Hasta recibir esa evidencia, el estado es **aplicado en rama / pendiente de cierre** y `main` no debe moverse.

## 9. Continuidad

Si la evidencia dinámica satisface el criterio, podrá cerrarse documental y registralmente este microbloque y considerarse su integración por fast-forward limpio.

Ese cierre no autoriza por sí mismo un nuevo frente serio ni determina automáticamente cuál sea el siguiente microcierre de FFL-B. La continuidad deberá volver a partir del repositorio fresco y de la deuda funcional realmente restante.

No se abre backend, Rust, WASM, IA productiva, stdlib ni `NL → SVP`.

---

*Documento técnico subordinado del Lenguaje SV.*
