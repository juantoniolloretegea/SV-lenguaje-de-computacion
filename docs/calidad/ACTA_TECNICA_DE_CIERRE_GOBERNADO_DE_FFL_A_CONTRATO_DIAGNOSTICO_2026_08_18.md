# Acta técnica de cierre gobernado de FFL-A — contrato diagnóstico

**Fecha:** 18/08/2026  
**Ámbito:** `SV-lenguaje-de-computacion`  
**Bloque:** `FFL-A — Contrato diagnóstico`  
**Naturaleza:** acta pública de cierre técnico con deuda reconocida  
**Estado:** vigente tras reapertura por Ruta A

## 1. Objeto

La presente acta resuelve el bloque `FFL-A`, reabierto el 18/08/2026 tras la superación de la compuerta doctrinal y matemática fijada por el acta de continuidad de 16/08/2026.

El cierre no equivale a convergencia total entre la IR canónica v0.2 y el catálogo efectivo de la implementación. Se mantiene la **Vía B**: la IR conserva autoridad normativa superior y el contrato diagnóstico efectivo permanece subordinado, documentado y trazable hasta que una decisión posterior disponga otra forma de convergencia.

## 2. Criterio de cierre

`CRITERIOS_DE_CIERRE_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md` permite cerrar el contrato diagnóstico cuando exista concordancia suficiente o cuando la deuda restante quede localizada y gobernada.

No procede el cierre si persisten contradicciones no gobernadas entre IR, catálogo, emisión observable y documentación pública.

La revisión se limita a ese umbral. No anticipa obligaciones funcionales de `FFL-B`, `FFL-E` ni de una futura infraestructura de ejecución.

## 3. Evidencia material

El cierre se apoya en lectura cruzada del repositorio fresco, incluida la IR canónica, el catálogo público de errores, los módulos de análisis y validación, la batería de conformidad, la matriz de concordancia, el dictamen de saneamiento y la tabla de correspondencias funcionales vigente.

Entre los artefactos principales se encuentran:

- `IR_CANONICA_BIENFORMACION_SV_v0_2.md`;
- `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`;
- `src/svp_errors.py`;
- `src/svp_parser.py`;
- `src/svp_validator.py`;
- `src/svp_ir.py`;
- `tests/run_conformance.py`;
- `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`;
- `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`;
- `docs/calidad/DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md`;
- `docs/calidad/C1C_DECISION_REGULARIZACION_CONTRATO_DIAGNOSTICO.md`;
- `docs/calidad/CROSSWALK_FUNCIONAL_DE_OBLIGACIONES_DIAGNOSTICAS_IR_V0_2_Y_FRONTEND_VIA_B_2026_08_18.md`.

El último nombre se conserva como identificador histórico de archivo. Su función es la de **tabla de correspondencias funcionales**.

## 4. Estado por identificador en el momento del cierre

En el punto temporal de este cierre quedaron registrados:

- **38 códigos** en la IR v0.2;
- **37 códigos** en el catálogo efectivo;
- **4 coincidencias semánticas por el mismo identificador:** `E102`, `E104`, `E106`, `E111`;
- **20 identificadores coincidentes con significado distinto**;
- **14 códigos sólo presentes en la IR**;
- **13 códigos sólo presentes en la implementación**.

Estas cifras describen el estado de FFL-A en el momento de su cierre. Los microcierres posteriores de FFL-B pueden modificar el inventario vivo sin alterar retroactivamente esta evidencia histórica.

La revisión dejó además correctamente diferenciados los casos con emisión directa, cobertura explícita, protección estructural o ausencia de materialización.

## 5. Concordancia numérica y cobertura funcional

La coincidencia de identificadores no basta para afirmar que dos diagnósticos protegen la misma obligación. Del mismo modo, una obligación puede estar protegida bajo un identificador distinto.

Por ello, la matriz por identificador se complementa con una tabla de correspondencias funcionales que distingue:

- misma obligación y mismo identificador;
- obligación protegida mediante otro identificador;
- obligación impedida por la propia estructura de la superficie vigente;
- protección parcial;
- obligación no materializada.

Esta separación evita atribuir cumplimiento por mera coincidencia numérica o incumplimiento por mera divergencia de numeración.

## 6. Deuda residual reconocida

Permanece deuda funcional y de ABI. En el momento de este cierre quedaron localizados, entre otros, los siguientes asuntos:

- actualización de `CoupledState` fuera de posiciones puente;
- operador de conflicto cuando concurra el supuesto canónico;
- compatibilidad entre arista y conector;
- juicios ejecutivos sobre conteos, umbral y precedencia de clasificación;
- pertenencia de los tipos de suceso de `TransitionData` al horizonte referenciado;
- suficiencia material de `TransitionData`;
- obligaciones parciales de ternarización, resolución, captura, admisibilidad y determinismo de compuerta;
- obligaciones posteriores de consulta, justificación y ABI.

Esta relación no declara esas cuestiones resueltas. Varias de ellas fueron tratadas posteriormente en FFL-B mediante cierres técnicos independientes y con su propia evidencia.

## 7. Contraste crítico del cierre

Se comprobaron expresamente los siguientes riesgos:

1. atribuir convergencia semántica sólo por compartir identificador;
2. atribuir ausencia funcional cuando una obligación está protegida por otro diagnóstico;
3. atribuir cobertura a un código catalogado sin prueba o emisión acreditada;
4. confundir protección estructural o parcial con ejecución material completa;
5. trasladar deuda a una futura infraestructura de ejecución sin haberla gobernado;
6. renumerar diagnósticos únicamente para obtener apariencia de concordancia.

Tras estas comprobaciones no se identificó una contradicción diagnóstica no gobernada que impidiera cerrar `FFL-A` bajo la Vía B.

## 8. Límite de la evidencia

Esta acta no atribuye una nueva ejecución completa de la batería de pruebas en el instante del cierre. La evidencia empleada comprende los lugares de emisión observables en el código, el manifiesto de casos y resultados esperados, las pruebas incorporadas al repositorio y la documentación de saneamiento previa.

La comprobación global de la batería corresponde a `FFL-C` y deberá registrarse allí de forma específica.

## 9. Decisión

> **FFL-A — CERRADO BAJO VÍA B, CON DEUDA RESIDUAL RECONOCIDA Y GOBERNADA.**

El cierre acredita un contrato efectivo públicamente identificable, su relación explícita con la IR v0.2 y una deuda residual localizada. No acredita convergencia total, agotamiento del ABI, implementación de todos los errores canónicos ni cierre de `FFL-B`, `FFL-C`, `FFL-D` o `FFL-E`.

Tampoco autoriza infraestructura de ejecución, Rust, WASM, IA productiva, NLP ni `NL → SVP`.

## 10. Continuidad

El bloque secuencial posterior es `FFL-B — cadena de implementación`.

Su trabajo debe partir del repositorio fresco. Una deuda identificada por la tabla de correspondencias sólo podrá materializarse cuando pertenezca realmente a la superficie vigente, cuente con fundamento suficiente y pueda descender a una regla comprobable y a una prueba trazable.
