# Calidad del frente operativo del Lenguaje SV

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

## 1. Función de esta carpeta

`docs/calidad/` reúne los documentos públicos de control técnico, trazabilidad, deuda viva, verificación, continuidad y registro del Lenguaje SV.

Esta carpeta no constituye autoridad doctrinal superior. Su función es documentar de forma revisable por terceros el estado técnico del repositorio y su relación con la matemática, la especificación, la implementación y la evidencia.

## 2. Continuidad vigente

La continuidad actual queda documentada, como mínimo, por:

1. `ACTA_TECNICA_DE_ORDENACION_DE_CONTINUIDAD_SEMANTICA_Y_ARQUITECTONICA_DEL_LENGUAJE_SV_2026_08_16.md`;
2. `ACTA_TECNICA_DE_REAPERTURA_DEL_LENGUAJE_SV_POR_RUTA_A_Y_RETORNO_A_FFL_A_2026_08_18.md`;
3. `ACTA_TECNICA_DE_CIERRE_GOBERNADO_DE_FFL_A_CONTRATO_DIAGNOSTICO_2026_08_18.md`;
4. `DECISION_FFL_B_GOBIERNO_DE_DIAGNOSTICOS_EFECTIVOS_BAJO_VIA_B_2026_08_18.md`;
5. `REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`;
6. `TABLERO_DE_BLOQUES_CERRABLES_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.csv`;
7. `REGISTRO_EVOLUCION_TECNICA_PROYECTO.md` y `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.

FFL-B continúa como único bloque técnico activo. FFL-C, FFL-D y FFL-E permanecen sin apertura hasta decisión expresa posterior.

## 3. Estado técnico reciente de FFL-B

Los cierres documentados mediante acta hasta E206/E207 se complementan con los siguientes cambios registrados:

- `RETP-2026-062` — retirada de `conflicts` de `graph_decl` y rechazo superficial mediante E001;
- `RETP-2026-064` — unicidad de `(target, position)` en régimen `Simple` mediante `E114 — SimpleRegimeConcurrency`;
- `RETP-2026-065` — correspondencia estructural entre cada constructor de `Supervisable` y el tipo de su contenido mediante E006.

La evidencia acumulada vigente acredita:

- conformidad: **55/55** — 9 casos válidos y 46 inválidos;
- pruebas rápidas de la interfaz de línea de órdenes: **3/3**;
- SEC-0: **3/3**.

La corrección de `graph_decl` no materializa `MissingConflictOperator`. E114 sólo protege el régimen `Simple`. El tipado de `Supervisable` no cierra la semántica completa de J3.3.

## 4. Contrato diagnóstico y correspondencia funcional

Los documentos principales son:

- `C1C_DECISION_REGULARIZACION_CONTRATO_DIAGNOSTICO.md`;
- `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`;
- `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`;
- `CROSSWALK_FUNCIONAL_DE_OBLIGACIONES_DIAGNOSTICAS_IR_V0_2_Y_FRONTEND_VIA_B_2026_08_18.md`;
- `DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md`.

Los nombres históricos de algunos archivos contienen términos ingleses. Se conservan sin modificación para mantener la trazabilidad de referencias publicadas.

## 5. Control de evolución y evidencia

### Registro maestro

- `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv` — numeración RETP y relación completa de asientos;
- `REGISTRO_EVOLUCION_TECNICA_PROYECTO.md` — lectura humana del tramo vigente;
- `historico/REGISTRO_EVOLUCION_TECNICA_PROYECTO_HASTA_RETP_2026_047.md` — preservación del historial detallado hasta RETP-047.

### Deuda y bloques

- `REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`;
- `TABLERO_DE_BLOQUES_CERRABLES_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.csv`;
- `REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`;
- `DEUDA_VIVA_HITOS_LENGUAJE_SV.csv`;
- `MATRIZ_UCBC_HORIZONTES_LENGUAJE_SV.csv`.

Los registros de hitos H1-H3 no requieren modificación por los cierres 062-065, porque ninguno de ellos verifica un hito nuevo ni abre H3.

## 6. Vigilancia doctrinal y continuidad

Permanecen como documentos de referencia, entre otros:

- `ESPEJO_DOCTRINAL_COLECCIONES_LENGUAJE_SV.md` y `.csv`;
- `MATRIZ_DE_VIGILANCIA_TEMPRANA_UCBC_INTERFACES_LENGUAJE_SV.md` y `.csv`;
- `PROTOCOLO_CORTO_DE_VIGILANCIA_CONCEPTUAL_ABSOLUTA_DEL_LENGUAJE_SV_ANTE_FRENTES_DOCTRINALES_ABIERTOS_2026_03_24.md`;
- `ACTA_TECNICA_DE_ALERTA_DE_GOBIERNO_SOBRE_COLECCION_I_CUSTODIA_ESTRUCTURAL_Y_FRENTE_NLP_2026_03_30.md`;
- `ACTA_TECNICA_DE_CIERRE_AUDITADO_Y_PRESERVACION_CONTROLADA_DE_SV_AUTH_A2_R2_Y_J6_2026_08_14.md`;
- `ACTA_TECNICA_DE_RECEPCION_DOCTRINAL_Y_PRESERVACION_DE_CONTINUIDAD_DEL_APRENDIZAJE_TRAZABLE_HACIA_IR_N3_N4_2026_08_15.md`;
- `ACTA_TECNICA_COMPLEMENTARIA_DE_FIJACION_DE_FUENTE_DOCTRINAL_Y_PRESERVACION_PRE_DSL_DEL_APRENDIZAJE_TRAZABLE_2026_08_16.md`;
- `ACTA_TECNICA_DE_RECEPCION_LATENTE_DE_NO_CLAUSURA_CERTIFICADA_2026_08_19.md`.

La existencia de una publicación, colección, rama experimental o realización aplicada no autoriza por sí misma una modificación de gramática, IR, validador, infraestructura de ejecución o bibliotecas.

## 7. Jerarquía de continuidad

El orden aplicable continúa siendo:

`doctrina y matemática del Sistema SV → especificación → implementación → diagnóstico → prueba → evidencia`.

Ningún documento de calidad puede convertir por sí solo una previsión futura en capacidad ejecutiva.
