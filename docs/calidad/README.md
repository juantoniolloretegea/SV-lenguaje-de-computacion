# Calidad del frente operativo

Esta carpeta reúne instrumentos activos de control, verificación, deuda viva, barridos secuenciales y trazabilidad de trabajo por agente del Lenguaje SV.

## Instrumentos activos

### Registro maestro y control del frente final

- `PROCEDIMIENTO_AUDITORIA_TECNICA_SV.md`
- `REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`
- `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`
- `REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`
- `TABLERO_DE_BLOQUES_CERRABLES_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.csv`
- `DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md`
- `C1C_DECISION_REGULARIZACION_CONTRATO_DIAGNOSTICO.md`
- `ACTA_TECNICA_DE_CIERRE_AUDITADO_DEL_PARCHE_CORRECTIVO_DEL_FRONTEND_2026_03_23.md`

### Vigilancia UCBC e integración de alertas tempranas

- `ACTA_TECNICA_DE_VIGILANCIA_UCBC_SOBRE_INTERFACES_Y_ALERTA_TEMPRANA_AL_LENGUAJE_SV_2026_03_21.md`
- `ACTA_TECNICA_DE_VIGILANCIA_UCBC_SOBRE_NOTA_DE_PRECISION_DE_SUCESO_LOCAL_Y_HORIZONTE_DECLARADO_2026_03_22.md`
- `MATRIZ_DE_VIGILANCIA_TEMPRANA_UCBC_INTERFACES_LENGUAJE_SV.md`
- `MATRIZ_DE_VIGILANCIA_TEMPRANA_UCBC_INTERFACES_LENGUAJE_SV.csv`


### Gobierno semántico ante frentes doctrinales abiertos

- `ACTA_TECNICA_DE_CONSOLIDACION_DE_LA_FAMILIA_VII_Y_ALERTA_DE_GOBIERNO_SEMANTICO_PARA_EL_LENGUAJE_SV_2026_03_24.md`
- `PROTOCOLO_CORTO_DE_VIGILANCIA_CONCEPTUAL_ABSOLUTA_DEL_LENGUAJE_SV_ANTE_FRENTES_DOCTRINALES_ABIERTOS_2026_03_24.md`
- `ESPEJO_DOCTRINAL_COLECCIONES_LENGUAJE_SV.md`
- `ESPEJO_DOCTRINAL_COLECCIONES_LENGUAJE_SV.csv`

### Resiliencia, horizontes y control por hitos

- `MATRIZ_UCBC_HORIZONTES_LENGUAJE_SV.csv`
- `REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`
- `DEUDA_VIVA_HITOS_LENGUAJE_SV.csv`
- `ACTA_TECNICA_DE_VERIFICACION_DEL_HITO_1_BASE_SEGURA_2026_03_23.md`
- `ACTA_TECNICA_DE_MICROAUDITORIA_H2_PRE_NO_BLOQUEO_2026_03_23.md`
- `ACTA_TECNICA_DE_AUTORIZACION_RESTRINGIDA_DE_APERTURA_DE_H2_2026_03_23.md`
- `ACTA_TECNICA_DE_CONSOLIDACION_AUDITADA_DE_H2_RESTRINGIDO_2026_03_23.md`

### Barridos secuenciales de actividad y latencia del repo

- `REGISTRO_BARRIDOS_DE_ACTIVIDAD_Y_LATENCIA_DEL_REPO.md`
- `REGISTRO_BARRIDOS_DE_ACTIVIDAD_Y_LATENCIA_DEL_REPO.csv`
- `ACTA_DE_ACTIVACION_DEL_CONTROL_DE_BARRIDOS_Y_PARTES_POR_AGENTE_2026_03_23.md`

### Partes de trabajo por agente

- `REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.md`
- `REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.csv`

## Regla de uso para lectura fresca

Los registros de barrido y de partes por agente introducen una taxonomía obligatoria de lectura del repo fresco:

- `INFERIDA`
- `DE_MEMORIA`
- `PARCIAL`
- `AJUSTADA_AL_OBJETO`
- `100_REAL`

### Regla operativa

- Las afirmaciones globales sobre el estado del repositorio y cualquier propuesta estructural, registral o de fase deben apoyarse en lectura `100_REAL` del repo fresco.
- La lectura `AJUSTADA_AL_OBJETO` solo puede legitimar actuaciones locales y explícitamente delimitadas.
- Las lecturas `INFERIDA` o `DE_MEMORIA` no bastan para introducir cambios estructurales ni para declarar estados globales del árbol.

## Regla de identificación de agente

Cuando deba identificarse una unidad de trabajo, se usará siempre la forma **Agente X**, donde **X** es el nombre operativo completo de la unidad activada.

## Regla de barrido secuencial

Cada **5 microparches** o **1 macrolote equivalente**, lo que ocurra antes, deberá ejecutarse un barrido secuencial del repo fresco. Ese barrido no debe basarse en la fecha de creación o de modificación del ZIP extraído como señal bruta suficiente. Debe valorar:

1. actividad estructural del fichero;
2. uso funcional efectivo;
3. latencia legítima o sospechosa;
4. necesidad o no de análisis adicional.

## Nota de prudencia

Este bloque combina registro maestro, deuda viva, vigilancia UCBC, control de horizontes, verificación por hitos, barridos secuenciales y partes de trabajo por agente. Ninguno de estos instrumentos autoriza por sí mismo cambios de gramática, IR, validator, runner o backend.

La superación auditada de `H1 — Base segura` no elimina por sí sola la deuda viva del frente final ni la deuda viva de hitos posteriores; su función es dejar constancia de que la base actual del lenguaje ya resulta suficientemente segura para seguir avanzando sin reabrir gramática, IR o validator por entusiasmo.

La microauditoría `H2-pre` tampoco verifica `H2`; solo deja asentado que el estado actual no muestra un estrechamiento ilegítimo del espacio estructural futuro de ese hito.

La autorización restringida de apertura de `H2` se apoya en esa ausencia de bloqueo y en la base segura ya acreditada por `H1`, pero no convierte `H2` en hito verificado: `Q-04` y `Q-05` permanecen en `Pendiente` hasta futura verificación positiva.


## Nota de consolidación de H2 restringido

La serie mínima de suficiencia estructural de **H2 restringido** queda ya consolidada por lectura auditada de sus tres subfrentes:

- `H2.1` — elasticidad de la IR para alojar futuras familias de propiedades preservadas;
- `H2.2` — no-rigidez del `validator` ante patrones de transición no uniformes;
- `H2.3` — suficiencia del plano ejecutable actual para no hipotecar evoluciones futuras.

Esta consolidación **no** verifica `H2` como hito. Su función es otra: dejar asentado que la apertura restringida de `H2` ya no descansa solo en un dictamen de no-bloqueo, sino también en una auditoría positiva mínima de suficiencia estructural.


## Nota adicional sobre diferenciación estructural

La consolidación doctrinal de la familia VII obliga a no tratar todos los soportes del sistema con igual rigidez. El Lenguaje SV debe distinguir entre estructura ósea ya consolidada, soportes funcionales de formalización y zonas de articulación con frentes emergentes, evitando la igualación de dureza que cerraría prematuramente el espacio semántico futuro.
