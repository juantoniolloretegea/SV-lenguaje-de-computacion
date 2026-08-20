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
5. `ACTA_TECNICA_DE_APERTURA_DE_FFL_C_PRUEBAS_Y_EVIDENCIA_2026_08_20.md`;
6. `ACTA_TECNICA_DE_CIERRE_DE_FFL_C_PRUEBAS_Y_EVIDENCIA_2026_08_20.md`;
7. `../arquitectura/ACTA_TECNICA_DE_APERTURA_DE_FFL_E_INTERFAZ_SEMANTICO_DIAGNOSTICA_2026_08_21.md`;
8. `REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`;
9. `TABLERO_DE_BLOQUES_CERRABLES_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.csv`;
10. `REGISTRO_EVOLUCION_TECNICA_PROYECTO.md` y `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.

FFL-A, FFL-B y FFL-C están cerrados. FFL-E está abierto desde el 21/08/2026. FFL-D permanece pendiente.

## 3. Cierre técnico de FFL-B

FFL-B quedó cerrado tras completar las comprobaciones estructurales representables sin ampliar la gramática, la IR ni la capacidad de ejecución.

Los cambios finales registrados son:

- `RETP-2026-062` — retirada de `conflicts` de `graph_decl` y rechazo superficial mediante E001;
- `RETP-2026-064` — unicidad de `(target, position)` en régimen `Simple` mediante `E114 — SimpleRegimeConcurrency`;
- `RETP-2026-065` — correspondencia estructural entre cada constructor de `Supervisable` y el tipo de su contenido mediante E006;
- `RETP-2026-067` — correspondencia entre la secuencia de entradas de `gate` y `AdmissibilityTable.input_codomains` mediante `E215 — GateTableSignatureMismatch`;
- `RETP-2026-068` — cierre de FFL-B con deuda técnica explícita y sin apertura automática de los bloques posteriores.

La evidencia de cierre de FFL-B fue:

- conformidad: **57/57** — 9 casos válidos y 48 inválidos;
- pruebas rápidas de la interfaz de línea de órdenes: **3/3**;
- SEC-0: **3/3**.

E215 comprueba únicamente número y codominio por posición. No ejecuta la compuerta ni calcula `GateResult.output`.

## 4. Cierre técnico de FFL-C

FFL-C se abrió el 20/08/2026 con escritura funcional limitada a `tests/` y modo de solo lectura sobre `src/`, gramática, AST, IR, validador, catálogo diagnóstico y manual.

Durante el bloque se incorporaron:

- el caso válido permanente `SystemTarget(CompositionGraph)`;
- una caracterización específica de la doble emisión observable de E006;
- un inventario de cobertura que separa caso persistido, emisión observable y propiedad estructural.

Una verificación independiente en modo de solo lectura sobre `3d48c422915b0e0bed65ba2e7ce8b807d7a94c33` acreditó:

- conformidad: **58/58** — 10 casos válidos y 48 inválidos;
- pruebas rápidas de la interfaz de línea de órdenes: **3/3**;
- SEC-0: **3/3**;
- caracterización de E006: **4/4**.

Los cuatro ejecutores terminaron con código de retorno 0 y el árbol permaneció sin cambios antes y después de la ejecución.

Los 48 casos inválidos cubren explícitamente 37 de los 47 códigos efectivos. Los diez restantes están clasificados por su alcanzabilidad real, por rutas diagnósticas alternativas o por preservación estructural; no se amplía el lenguaje para producir casos artificiales.

FFL-C queda cerrado. Su cierre no se altera por la apertura posterior de FFL-E.

## 4.1. Apertura técnica de FFL-E

FFL-E se abrió el 21/08/2026 para fijar el contrato semántico-diagnóstico de suficiencia representacional por operación en dominios, agentes, consultas e interfaces.

La apertura establece desde el inicio que:

- `Tri = {Zero, One, U}` permanece inalterado;
- el codominio terminal de una evaluación conserva su tipo declarado y no se identifica por defecto con `Tri`;
- una representación con pérdida puede ser suficiente para una operación e insuficiente para otra;
- la insuficiencia representacional no produce `U`;
- las capas o estratos son estructura declarada del dominio y no se deducen únicamente de `n = b²`;
- una interfaz sólo puede atribuirse la información que transmite de forma declarada.

El contrato técnico de FFL-E se documenta en `docs/arquitectura/CONTRATO_MINIMO_DE_SUFIENCIA_REPRESENTACIONAL_POR_OPERACION_PARA_EL_LENGUAJE_SV_2026_08_21.md`.

La apertura no modifica todavía `src/`, gramática v0.1, IR v0.2, validador, catálogo diagnóstico ni pruebas. FFL-D continúa pendiente.

## 5. Contrato diagnóstico y correspondencia funcional

Los documentos principales son:

- `C1C_DECISION_REGULARIZACION_CONTRATO_DIAGNOSTICO.md`;
- `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`;
- `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`;
- `CROSSWALK_FUNCIONAL_DE_OBLIGACIONES_DIAGNOSTICAS_IR_V0_2_Y_FRONTEND_VIA_B_2026_08_18.md`;
- `DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md`.

Los nombres históricos de algunos archivos contienen términos ingleses. Se conservan sin modificación para mantener la trazabilidad de referencias publicadas.

## 6. Estado diagnóstico vigente

Tras E215 constan:

- **38 códigos** definidos por la IR v0.2;
- **47 códigos** en el catálogo efectivo;
- **5 coincidencias semánticas por mismo identificador**;
- **20 identificadores compartidos con significado distinto**;
- **13 códigos** presentes sólo en la IR v0.2;
- **22 códigos** presentes sólo en la implementación efectiva.

La coincidencia numérica no implica equivalencia material. Las rutas alternativas se documentan en la matriz y en la tabla de correspondencias funcionales.

FFL-C no modifica estas cifras: amplía y caracteriza la evidencia de prueba, no el catálogo diagnóstico. FFL-E reserva la clase semántica `RepresentationInsufficientForOperation`, pero no le asigna todavía código numérico ni capacidad de emisión; por tanto, las cifras anteriores permanecen vigentes.

## 7. Deuda técnica que no bloquea los cierres alcanzados

Permanecen expresamente fuera de FFL-B y FFL-C:

- la concurrencia en régimen `General` que requiera `ConflictOperator`;
- la procedencia completa de una actualización de `CoupledState` desde un `Connector` concreto;
- la suficiencia reconstructiva completa de `TransitionData`;
- la producción y validación material de `Frame.criticalities`;
- la ejecución de `GateResult.output`;
- el determinismo material de `SupervisionResult.verdict` y el efecto de `Veto`;
- la revisión futura del alcance de E006 cuando una referencia existe pero su tipo es incompatible.

La forma válida `SystemTarget(CompositionGraph)` ya dispone de un caso permanente de conformidad y deja de ser deuda de cobertura positiva.

Estas limitaciones quedan registradas como deuda o como capacidades no representadas. No invalidan los cierres de FFL-B ni FFL-C.

FFL-E añade un ámbito nuevo de especificación: identidad y estratificación de parámetros, cadenas de representación, requisitos de representación de operaciones, certificados de recuperabilidad y diagnóstico de representación insuficiente. Estas capacidades no se atribuyen todavía a la implementación.

## 8. Control de evolución y evidencia

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

Los registros de hitos H1-H3 no cambian por E215 ni por los cierres de FFL-B y FFL-C. La apertura de FFL-E tampoco verifica por sí misma un hito nuevo.

## 9. Regla de continuidad

El orden aplicable continúa siendo:

`doctrina y matemática del Sistema SV → especificación → implementación → diagnóstico → prueba → evidencia`.

Ningún documento de calidad puede convertir por sí solo una previsión futura en capacidad ejecutiva. La apertura de FFL-E establece el contrato que deberá gobernar una ampliación posterior; no declara implementadas las capacidades que describe.
