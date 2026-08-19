# Calidad del ámbito operativo del Lenguaje SV

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

## 1. Función de esta carpeta

`docs/calidad/` reúne los documentos públicos de control técnico, trazabilidad, deuda viva, verificación, continuidad y registro del Lenguaje SV.

Su función es documentar de forma revisable por terceros el estado técnico del repositorio y su relación con la matemática, la especificación, la implementación y la evidencia.

## 2. Continuidad vigente

La continuidad actual queda documentada, como mínimo, por:

1. `ACTA_TECNICA_DE_ORDENACION_DE_CONTINUIDAD_SEMANTICA_Y_ARQUITECTONICA_DEL_LENGUAJE_SV_2026_08_16.md`;
2. `ACTA_TECNICA_DE_REAPERTURA_DEL_LENGUAJE_SV_POR_RUTA_A_Y_RETORNO_A_FFL_A_2026_08_18.md`;
3. `ACTA_TECNICA_DE_CIERRE_GOBERNADO_DE_FFL_A_CONTRATO_DIAGNOSTICO_2026_08_18.md`;
4. `DECISION_FFL_B_GOBIERNO_DE_DIAGNOSTICOS_EFECTIVOS_BAJO_VIA_B_2026_08_18.md`;
5. `REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`;
6. `TABLERO_DE_BLOQUES_CERRABLES_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.csv`;
7. `REGISTRO_EVOLUCION_TECNICA_PROYECTO.md` y `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.

FFL-A y FFL-B están cerrados. FFL-C, FFL-D y FFL-E permanecen pendientes hasta decisión expresa posterior.

## 3. Cierre técnico de FFL-B

FFL-B queda cerrado tras completar las comprobaciones estructurales representables sin ampliar la gramática, la IR ni la capacidad de ejecución.

Los cambios finales registrados son:

- `RETP-2026-062` — retirada de `conflicts` de `graph_decl` y rechazo superficial mediante E001;
- `RETP-2026-064` — unicidad de `(target, position)` en régimen `Simple` mediante `E114 — SimpleRegimeConcurrency`;
- `RETP-2026-065` — correspondencia estructural entre cada constructor de `Supervisable` y el tipo de su contenido mediante E006;
- `RETP-2026-067` — correspondencia entre la secuencia de entradas de `gate` y `AdmissibilityTable.input_codomains` mediante `E215 — GateTableSignatureMismatch`;
- `RETP-2026-068` — cierre de FFL-B con deuda técnica explícita y sin apertura automática de los bloques posteriores.

La evidencia acumulada vigente acredita:

- conformidad: **57/57** — 9 casos válidos y 48 inválidos;
- pruebas rápidas de la interfaz de línea de órdenes: **3/3**;
- SEC-0: **3/3**.

E215 comprueba únicamente número y codominio por posición. No ejecuta la compuerta ni calcula `GateResult.output`.

## 4. Contrato diagnóstico y correspondencia funcional

Los documentos principales son:

- `C1C_DECISION_REGULARIZACION_CONTRATO_DIAGNOSTICO.md`;
- `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`;
- `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`;
- `CROSSWALK_FUNCIONAL_DE_OBLIGACIONES_DIAGNOSTICAS_IR_V0_2_Y_FRONTEND_VIA_B_2026_08_18.md`;
- `DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md`.

Los nombres históricos de algunos archivos contienen términos ingleses. Se conservan sin modificación para mantener la trazabilidad de referencias publicadas.

## 5. Estado diagnóstico vigente

Tras E215 constan:

- **38 códigos** definidos por la IR v0.2;
- **47 códigos** en el catálogo efectivo;
- **5 coincidencias semánticas por mismo identificador**;
- **20 identificadores compartidos con significado distinto**;
- **13 códigos** presentes sólo en la IR v0.2;
- **22 códigos** presentes sólo en la implementación efectiva.

La coincidencia numérica no implica equivalencia material. Las rutas alternativas se documentan en la matriz y en la tabla de correspondencias funcionales.

## 6. Deuda técnica que no bloquea el cierre

Permanecen expresamente fuera de FFL-B:

- la concurrencia en régimen `General` que requiera `ConflictOperator`;
- la procedencia completa de una actualización de `CoupledState` desde un `Connector` concreto;
- la suficiencia reconstructiva completa de `TransitionData`;
- la producción y validación material de `Frame.criticalities`;
- la ejecución de `GateResult.output`;
- el determinismo material de `SupervisionResult.verdict` y el efecto de `Veto`;
- la revisión futura del alcance de E006 cuando una referencia existe pero su tipo es incompatible.

Estas limitaciones quedan registradas como deuda o como capacidades no representadas. No justifican mantener FFL-B abierto.

## 7. Control de evolución y evidencia

### Registro de evolución

- `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv` — numeración RETP y relación completa de asientos;
- `REGISTRO_EVOLUCION_TECNICA_PROYECTO.md` — lectura humana del tramo vigente;
- `historico/REGISTRO_EVOLUCION_TECNICA_PROYECTO_HASTA_RETP_2026_047.md` — preservación del historial detallado hasta RETP-047.

### Deuda y bloques

- `REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`;
- `TABLERO_DE_BLOQUES_CERRABLES_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.csv`;
- `REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`;
- `DEUDA_VIVA_HITOS_LENGUAJE_SV.csv`;
- `MATRIZ_UCBC_HORIZONTES_LENGUAJE_SV.csv`.

Los registros de hitos H1-H3 no cambian por E215 ni por el cierre de FFL-B: ninguno de estos actos verifica por sí mismo un hito nuevo ni abre H3.

## 8. Regla de continuidad

El orden aplicable continúa siendo:

`doctrina y matemática del Sistema SV → especificación → implementación → diagnóstico → prueba → evidencia`.

Ningún documento de calidad puede convertir por sí solo una previsión futura en capacidad ejecutiva. Cualquier reapertura de FFL-B o apertura de FFL-C, FFL-D o FFL-E requiere decisión expresa y fundamento técnico identificable.
