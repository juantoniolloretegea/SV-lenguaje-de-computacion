# Acta técnica de autorización restringida de apertura de H2

**Fecha:** 23/03/2026  
**Hora (Europe/Madrid):** 17:45:05  
**Agente:** Agente WLenguaje7 SV  
**Naturaleza:** acta operativa subordinada de autorización restringida de hito-fase  
**Estado:** H2 autorizado en régimen restringido; `Q-04` y `Q-05` permanecen pendientes  
**Registro técnico asociado:** véase `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv` como fuente maestra de numeración RETP

## 1. Objeto

Dejar constancia técnica de que el repositorio operativo `SV-lenguaje-de-computacion`, en su estado fresco auditado tras `H1` verificado y `H2-pre` formalizado como no bloqueo, dispone ya de base positiva suficiente para **autorizar la apertura restringida de `H2`** como frente arquitectónico-auditor de trabajo.

Esta autorización no equivale a verificar `H2` como hito alcanzado, no abre por sí sola el backend soberano y no autoriza implementaciones fuertes de invariancia, persistencia, umbral, transición ni clases formales de régimen.

## 2. Hechos constatados

Consta en el árbol real del repositorio, al menos, lo siguiente:

- `H1 — Base segura` ya ha quedado auditado y registrado en `docs/calidad/ACTA_TECNICA_DE_VERIFICACION_DEL_HITO_1_BASE_SEGURA_2026_03_23.md`;
- `H2-pre` ya ha quedado auditado y registrado en `docs/calidad/ACTA_TECNICA_DE_MICROAUDITORIA_H2_PRE_NO_BLOQUEO_2026_03_23.md`;
- el marco de resiliencia del lenguaje está publicado en `docs/arquitectura/MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`;
- el contrato mínimo de enganche y ABI semántico-diagnóstico está publicado en `docs/arquitectura/CONTRATO_DE_ENGANCHE_DE_INTERFACES_FUTURAS_Y_ABI_SEMANTICO_DIAGNOSTICO_MINIMO.md`;
- la arquitectura mínima del núcleo enganchable está publicada en `docs/arquitectura/NOTA_DE_ARQUITECTURA_MINIMA_DEL_NUCLEO_ENGANCHABLE_DEL_LENGUAJE_SV.md`;
- la matriz de horizontes y el registro de hitos mantienen `Q-04` y `Q-05` en `Pendiente`, pero con evidencia ya registrada de no bloqueo estructural.

Consta además, mediante verificación local del repo fresco auditado, que la base ejecutable observable permanece íntegra:

- `tests/run_conformance.py`: `31/31`;
- `tests/run_cli_smoke.py`: `3/3`;
- `tests/run_sec0_smoke.py`: `3/3`.

## 3. Juicio técnico

Sobre la base del estado fresco auditado, se estima concurrente la siguiente secuencia:

1. `H1` ya verifica una base material mínima segura del lenguaje.
2. `H2-pre` ya ha descartado la existencia de un estrechamiento ilegítimo del espacio estructural futuro de `H2`.
3. No se ha detectado una carencia positiva crítica que obligue a mantener `H2` totalmente cerrado como frente de trabajo.

Por tanto, **sí existe base positiva suficiente para autorizar la apertura restringida de `H2`** como frente de diseño, auditoría y preparación arquitectónica, sin convertir esa apertura en verificación del hito.

## 4. Alcance permitido de la apertura restringida

Quedan permitidas, dentro de `H2`, las actuaciones de carácter:

- arquitectónico;
- auditivo o adversarial;
- de evaluación de elasticidad de IR;
- de evaluación de no-rigidez del `validator`;
- de evaluación de no-hipoteca del plano ejecutable actual;
- y de preparación conceptual de familias de propiedades, persistencia, umbral, transición y equivalencias parciales **sin implementarlas todavía**.

## 5. Alcance expresamente no permitido

No quedan autorizadas por esta acta las siguientes actuaciones:

- declarar `H2` como verificado;
- introducir semántica cerrada de invariancia o persistencia;
- introducir clases formales de régimen;
- resemantizar `Trajectory`, `Frame`, `Horizon` o `Domain` por intuición;
- endurecer backend soberano o abrirlo por inercia;
- ampliar gramática o IR por entusiasmo de fase.

## 6. Objeción adversarial considerada

Se considera y rechaza expresamente el siguiente riesgo doble:

- sobreactuar `H2-pre` como si ya implicara `H2` logrado;
- o, en el extremo contrario, negar toda apertura de `H2` por la sola permanencia de `Q-04`, `Q-05`, `D-01` y `D-02` en régimen pendiente o de deuda viva aceptada.

No procede ninguno de los dos extremos. La posición correcta es una autorización restringida, gobernada y expresamente limitada.

## 7. Decisión

Se fija la siguiente lectura válida para el repositorio operativo del lenguaje:

1. **sí** queda autorizada la apertura restringida de `H2` como frente arquitectónico-auditor de trabajo;
2. **no** queda verificado `H2` como hito alcanzado;
3. **sí** permanecen `Q-04` y `Q-05` en `Pendiente`;
4. **no** se autoriza por ello apertura automática de `H3` o del backend soberano;
5. **sí** se reconoce que la base actual del lenguaje ya permite trabajar `H2` sin trauma estructural y sin necesidad de rehacer el sistema.

## 8. Artefactos documentales de este lote

- `README.md`
- `docs/README.md`
- `docs/arquitectura/README.md`
- `docs/calidad/README.md`
- `docs/index.html`
- `docs/calidad/ACTA_TECNICA_DE_AUTORIZACION_RESTRINGIDA_DE_APERTURA_DE_H2_2026_03_23.md`
- `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`
- `docs/calidad/REGISTRO_BARRIDOS_DE_ACTIVIDAD_Y_LATENCIA_DEL_REPO.csv`
- `docs/calidad/REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.csv`
- `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`
- `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`

## 9. Fórmula de cierre

`H2` queda autorizado **solo en régimen restringido**, como frente legítimo de preparación arquitectónica y auditoría, con deuda viva todavía reconocida y sin promoción automática a implementación fuerte o backend soberano.
