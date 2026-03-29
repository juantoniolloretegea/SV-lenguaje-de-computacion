# Acta técnica de verificación del Hito 1 — Base segura

**Fecha:** 23/03/2026  
**Naturaleza:** acta operativa subordinada de verificación de hito  
**Frente:** resiliencia pre-backend / control por hitos del Lenguaje SV  
**Estado:** H1 verificado; H2 y H3 permanecen pendientes  
**Registro técnico asociado:** véase `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv` como fuente maestra de numeración RETP

## 1. Objeto

Dejar constancia técnica de que el repositorio operativo `SV-lenguaje-de-computacion`, en su estado verificado, satisface materialmente las condiciones de `H1 — Base segura` definidas en `docs/arquitectura/MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`, sin convertir esa verificación en cierre total del frente final, en agotamiento de `N4/Uso` ni en apertura automática de `H2`, `H3` o del backend soberano.

## 2. Hechos constatados

Consta en el árbol real del repositorio, al menos, lo siguiente:

- gramática superficial mínima vigente publicada en `GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md`;
- IR canónica vigente publicada en `IR_CANONICA_BIENFORMACION_SV_v0_2.md`;
- frontera normativa vigente publicada en `FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md`;
- parser de referencia activo en `src/svp_parser.py`;
- validator de referencia activo en `src/svp_validator.py`;
- contrato de enganche y ABI semántico-diagnóstico mínimo publicado en `docs/arquitectura/CONTRATO_DE_ENGANCHE_DE_INTERFACES_FUTURAS_Y_ABI_SEMANTICO_DIAGNOSTICO_MINIMO.md`;
- arquitectura mínima del núcleo enganchable publicada en `docs/arquitectura/NOTA_DE_ARQUITECTURA_MINIMA_DEL_NUCLEO_ENGANCHABLE_DEL_LENGUAJE_SV.md`;
- informe de continuidad del frente básico y habilitación prudente del backend publicado en `docs/arquitectura/INFORME_DE_CONTINUIDAD_DEL_FRENTE_BASICO_Y_HABILITACION_DEL_BACKEND_SV.md`;
- matriz de horizontes y control por hitos publicada en `docs/calidad/MATRIZ_UCBC_HORIZONTES_LENGUAJE_SV.csv` y `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`.

Consta además, mediante verificación local del árbol del repositorio verificado auditado, que la base ejecutable observable permanece íntegra:

- `tests/run_conformance.py`: `31/31`;
- `tests/run_cli_smoke.py`: `3/3`;
- `tests/run_sec0_smoke.py`: `3/3`.

## 3. Juicio de verificación por controles de H1

### Q-01 — Gramática mínima estable

**Verificada.**

No se constata ampliación sintáctica nueva por presión semántica futura. La gramática superficial vigente permanece mínima, activa y subordinada a la IR y a la frontera normativa.

### Q-02 — IR no incompatible con relaciones y acumulaciones futuras

**Verificada con cautela fuerte suficiente para H1.**

La IR vigente ya dispone de estructuras (`SemanticRelation`, `Graph`, `Pattern`, `Horizon`, `Frame`, `TransitionData`, `Trajectory`, `Domain`, `QuerySpec`, `QueryContext`, `QueryResult`) que no obligan a un modelo reducido a pasos aislados ni acreditan incompatibilidad material con relaciones o acumulaciones futuras. La cautela fuerte de horizontes persiste, pero no bloquea `H1`.

### Q-03 — Validator no temporalista

**Verificada.**

No se acredita en el `validator` vigente una presuposición de tiempo fuerte, linealidad ontológica o uniformidad obligatoria de transición. El enforcement observable sigue siendo estructural y tipado, no temporalista.

## 4. Decisión

Se fija la siguiente lectura válida para el repositorio operativo del lenguaje:

1. **sí** queda verificado `H1 — Base segura` como hito materialmente alcanzado en el estado verificado del repo;
2. **no** se declara por ello cerrado el frente final `FFL-A` a `FFL-E`;
3. **no** se elimina por ello la deuda viva residual del frente ni la deuda viva de hitos posteriores;
4. **no** se autoriza por ello apertura automática de `H2`, `H3` o del backend soberano;
5. **sí** se reconoce que la base actual del lenguaje es suficientemente segura para seguir avanzando sin reabrir ahora gramática, IR o validator por entusiasmo.

## 5. Objeción adversarial considerada

Se considera y rechaza expresamente el siguiente riesgo doble:

- confundir la verificación material de `H1` con cierre total del frente final o con agotamiento de `N4/Uso`;
- o, en el extremo contrario, negar `H1` por la sola persistencia de deuda viva residual en `DFL-003`, `DFL-004`, `DFL-005` o en los hitos posteriores.

No procede ninguno de los dos extremos. `H1` verifica una base segura; no clausura el sistema.

## 6. Artefactos documentales de este lote

- `README.md`
- `docs/README.md`
- `docs/arquitectura/README.md`
- `docs/calidad/README.md`
- `docs/index.html`
- `docs/calidad/ACTA_TECNICA_DE_VERIFICACION_DEL_HITO_1_BASE_SEGURA_2026_03_23.md`
- `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`
- `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`
- `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`

## 7. Fórmula de cierre

El `H1 — Base segura` queda auditado y verificado como base suficiente del frente vigente, con deuda residual reconocida y sin promoción automática a hitos posteriores.
