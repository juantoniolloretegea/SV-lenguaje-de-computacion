# Acta técnica FFL-B — materialización mínima de `E406 — InsufficientTransitionData`

**Fecha:** 18/08/2026  
**Frente:** FFL-B — J4.3 / `TransitionData.induced_parameters`  
**Estado:** CERRADO  
**Rama de verificación:** `agent/ffl-b-e406-min-transition`  
**Base:** `daa7ac6980b6f16c27207442c50a4eace09603c9`  
**Revisión funcional auditada:** `a1d3d98b1971fe3cb7c1941dc1b3d9ce41b1e54c`  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ISSN:** 2695-6411

## 1. Objeto

Esta acta registra la materialización mínima de la obligación diagnóstica de `J4.3` según la cual un `TransitionData` debe especificar al menos un cambio en `induced_parameters`.

El cierre no modifica la doctrina del Suceso, no amplía la gramática, no altera la IR canónica y no atribuye a la etapa frontal del compilador una capacidad de reconstrucción ejecutiva que todavía no esté acreditada.

## 2. Fundamento canónico

La IR canónica v0.2 declara para `TransitionData`:

`induced_parameters : [(NodeId, Nat, Tri)]`.

En `J4.3 — Transición bien formada` exige, entre otras condiciones, que:

- `induced_parameters` especifique al menos un cambio de parámetro;
- los datos sean suficientes para reconstruir el operador inducido a partir de `induced_parameters`.

La tabla canónica de errores asigna expresamente:

`E406 — InsufficientTransitionData — J4.3 — induced_parameters vacío`.

Este cierre se limita a esa correspondencia literal.

## 3. Comprobación material previa

La lectura del repositorio fresco acreditó que la información necesaria estaba ya representada antes del validador:

1. la gramática superficial declara `induced_parameters` como lista de `induced_param_literal`;
2. el AST conserva `TransitionDataDecl.induced_parameters` como lista;
3. el analizador sintáctico admite una lista vacía mediante el mecanismo general de listas;
4. el descenso a IR transporta la lista sin alterar su cardinalidad;
5. el validador comprobaba ya `horizon_ref` y la pertenencia de tipos de suceso, pero no la no-vaciedad de `induced_parameters`.

La obligación era, por tanto, directamente comprobable sin nueva sintaxis ni nueva semántica.

## 4. Contraste crítico del alcance

La no-vaciedad de `induced_parameters` es una condición necesaria, pero no demuestra por sí sola la cláusula más fuerte de `J4.3` relativa a la suficiencia de los datos para reconstruir el operador inducido.

En consecuencia:

- procede materializar `induced_parameters == [] → E406`;
- no procede declarar cerrado todo `J4.3`;
- una lista no vacía no se presume suficiente para reconstruir el operador inducido;
- no se introduce un comprobador de reconstructibilidad que la superficie vigente no defina materialmente.

## 5. Decisión diagnóstica

No existía colisión del identificador `E406` en el catálogo efectivo anterior a este cierre.

Como el nombre, el identificador y la condición material coinciden exactamente con la IR v0.2, se incorpora el mismo diagnóstico:

`E406 — InsufficientTransitionData`.

No se crea un alias ni se reutiliza otro código.

## 6. Implementación acotada

La modificación funcional se limita a:

- alta de `E406` en `src/svp_errors.py` y en `ERRORS`;
- importación de `E406` en `src/svp_validator.py`;
- una única comprobación en `_validate_transition_data`:

  `if not node.induced_parameters: raise E406`;

- un caso negativo específico: `tests/conformance/invalid/transition_induced_parameters_vacios.svp`;
- una entrada esperada `E406` en `tests/run_conformance.py`.

No se modifican gramática, IR canónica, AST, analizador sintáctico, descenso a IR, serialización, interfaz de línea de órdenes, `Playground`, biblioteca estándar, infraestructura de ejecución, Rust ni WASM.

## 7. Sincronización diagnóstica

En el mismo cierre se actualizan:

- `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`;
- `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`;
- `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`;
- la tabla de correspondencias funcionales vigente;
- `docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`;
- esta acta.

Para la cláusula materializada, `E406` deja de ser una obligación sólo presente en la IR y pasa a constituir una coincidencia exacta de identificador y significado.

## 8. Evidencia de cierre

La rama `agent/ffl-b-e406-min-transition`, con base `daa7ac6980b6f16c27207442c50a4eace09603c9` y revisión funcional `a1d3d98b1971fe3cb7c1941dc1b3d9ce41b1e54c`, fue sometida a verificación externa en modo de solo lectura.

Resultados acreditados:

- `tests/run_conformance.py`: **45/45**;
- `tests/run_cli_smoke.py`: **3/3**;
- `tests/run_sec0_smoke.py`: **3/3**;
- `transition_induced_parameters_vacios.svp`: emisión exacta de `E406 — InsufficientTransitionData`, causada por `induced_parameters` vacío;
- `transition_data_events.svp`: válido, con producción de IR;
- `transition_event_fuera_horizon.svp`: emisión exacta de `E307 — UndeclaredHorizonEvent`.

La comprobación confirmó que `E307` se evalúa antes que `E406`, por lo que un tipo de suceso ajeno al horizonte no queda ocultado por la nueva condición de no-vaciedad.

La comparación de cambios entre la base y la revisión funcional contiene una única confirmación funcional y un alcance estrictamente acotado: alta de `E406`, comprobación de lista vacía, una entrada de conformidad y un caso de prueba específico, además de la documentación correspondiente.

Las observaciones de formato detectadas durante la verificación no afectan al juicio funcional certificado y quedan fuera del alcance de esta acta.

## 9. Límite del cierre

**E406 queda cerrado exclusivamente para `induced_parameters == []`.**

Este cierre no acredita:

- que todo triple de `induced_parameters` describa un cambio efectivo respecto de un estado anterior;
- que cada `NodeId` o posición quede validado aquí contra una arquitectura concreta;
- que una lista no vacía permita por sí sola reconstruir el operador inducido;
- ni que todo `J4.3` pueda darse por cerrado.

La continuidad de FFL-B deberá volver a partir del repositorio fresco y de la deuda funcional realmente restante. Este cierre no autoriza automáticamente una nueva unidad técnica ni un frente distinto.

No se abre infraestructura de ejecución, Rust, WASM, IA productiva, biblioteca estándar ni `NL → SVP`.
