# Acta técnica de consolidación auditada de H2 restringido — 23/03/2026

## 1. Objeto

La presente acta consolida, en un único punto de control, la serie auditada de **H2 restringido** del Lenguaje SV, una vez verificados sus tres subfrentes mínimos de suficiencia estructural:

- **H2.1** — elasticidad de la IR para alojar futuras familias de propiedades preservadas;
- **H2.2** — no-rigidez del `validator` ante patrones de transición no uniformes;
- **H2.3** — suficiencia del plano ejecutable actual para no hipotecar evoluciones futuras.

No se declara por este acto que **H2** haya quedado verificado como hito. Se declara algo distinto y más sobrio: que la apertura restringida de H2 queda ahora **materialmente respaldada por auditoría suficiente** en sus tres subfrentes mínimos.

## 2. Base técnica considerada

La consolidación se apoya en la lectura fresca real del repositorio operativo y, en particular, en:

- `IR_CANONICA_BIENFORMACION_SV_v0_2.md`
- `src/svp_ir.py`
- `src/svp_ast.py`
- `src/svp_validator.py`
- `src/svp_main.py`
- `src/svp_serialize.py`
- `docs/arquitectura/MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`
- `docs/arquitectura/NOTA_DE_ARQUITECTURA_MINIMA_DEL_NUCLEO_ENGANCHABLE_DEL_LENGUAJE_SV.md`
- `docs/arquitectura/INFORME_DE_CONTINUIDAD_DEL_FRENTE_BASICO_Y_HABILITACION_DEL_BACKEND_SV.md`
- `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`
- `docs/calidad/DEUDA_VIVA_HITOS_LENGUAJE_SV.csv`
- `docs/calidad/MATRIZ_UCBC_HORIZONTES_LENGUAJE_SV.csv`

La base ejecutable permaneció íntegra durante toda la serie:

- `tests/run_conformance.py` → **31/31**
- `tests/run_cli_smoke.py` → **3/3**
- `tests/run_sec0_smoke.py` → **3/3**

## 3. Dictamen consolidado por subfrentes

### 3.1. H2.1 — IR

La IR actual **pasa** en régimen de elasticidad suficiente. No representa todavía familias de propiedades preservadas como capa positiva ya formalizada, pero tampoco aparece cerrada de forma que obligue a rehacer su esqueleto cuando esa necesidad deba abordarse. La deuda `D-01` permanece abierta, pero pierde gravedad estructural inmediata.

### 3.2. H2.2 — Validator

El `validator` actual **pasa** en no-rigidez suficiente. Impone disciplina estructural mínima y trazabilidad tipada, pero no introduce todavía una teoría cerrada de persistencia, umbral o transición. La deuda `D-02` permanece abierta, pero ya no debe leerse como riesgo inmediato de clausura semántica del cambio.

### 3.3. H2.3 — Plano ejecutable

El plano ejecutable actual **pasa** en suficiencia de no-hipoteca. Su pobreza actual impide todavía verificar un runner semánticamente rico, pero justamente por ello no ha cristalizado todavía decisiones irreversibles sobre continuidad uniforme, persistencia o equivalencia. La deuda `D-03` permanece abierta y correctamente situada en H3.

## 4. Alcance exacto de la consolidación

La presente consolidación autoriza a leer **H2 restringido** de la siguiente manera:

- H2 queda **abierto en régimen restringido**;
- su apertura ya no descansa solo en no-bloqueo abstracto, sino también en auditoría positiva suficiente de sus tres subfrentes mínimos;
- pero **H2 no queda verificado** por este acto;
- `Q-04` y `Q-05` permanecen en **Pendiente**;
- no se autoriza todavía semántica fuerte de invariancia, persistencia, umbral, clases de régimen ni backend soberano.

## 5. Objeción adversarial considerada

La objeción adversarial más fuerte sostiene que la serie H2.1–H2.3 podría invitar a tratar H2 como si ya estuviera prácticamente logrado.

No procede.

La serie auditada demuestra suficiencia estructural mínima para sostener la apertura restringida de H2, pero no aporta todavía formalización positiva de familias de propiedades preservadas, ni modelado semántico de persistencia/umbral/transición, ni runner rico, ni ABI semántico para clases locales de régimen. Por tanto, la decisión correcta es consolidar **H2 restringido**, no declarar **H2 verificado**.

## 6. Decisión

Se declara consolidada la serie auditada de **H2 restringido**. En consecuencia:

1. se mantiene la apertura restringida de H2;
2. se preserva el estatuto pendiente de `Q-04` y `Q-05`;
3. se reclasifica la gravedad inmediata de `D-01`, `D-02` y `D-03` como deuda futura no materializada, no como amenaza estructural inminente;
4. no se abre todavía H3;
5. no se abre todavía backend soberano.

## 7. Cierre

La utilidad de esta consolidación no es acelerar artificialmente el sistema, sino impedir dos errores simétricos:

- creer que H2 sigue apoyado solo en hipótesis negativas de no-bloqueo;
- o creer que la superación de H2.1–H2.3 equivale ya a una verificación positiva de H2.

Ninguno de esos extremos procede.

La lectura correcta, a partir de ahora, es ésta:

> **H2 restringido queda abierto y suficientemente respaldado por auditoría mínima, pero H2 sigue sin verificarse positivamente.**
