# Acta técnica de cierre gobernado de FFL-A — contrato diagnóstico

**Fecha:** 18/08/2026  
**Ámbito:** `SV-lenguaje-de-computacion`  
**Bloque:** `FFL-A — Contrato diagnóstico`  
**Naturaleza:** acta pública de cierre técnico con deuda reconocida  
**Estado:** vigente tras reapertura por Ruta A

## 1. Objeto

La presente acta resuelve el bloque `FFL-A`, reabierto el 18/08/2026 tras la superación de la compuerta doctrinal y matemática fijada por el acta de continuidad de 16/08/2026.

El cierre aquí declarado no equivale a convergencia total entre la IR canónica v0.2 y el catálogo implementativo. Se adopta el criterio ya vigente de **Vía B**: la IR conserva autoridad normativa superior, mientras el contrato efectivo del frontend se documenta de forma sincera, provisional y trazable hasta una convergencia futura formalmente decidida.

## 2. Criterio de cierre aplicado

`CRITERIOS_DE_CIERRE_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md` permite cerrar el contrato diagnóstico cuando exista concordancia suficiente o, en su defecto, cuando la deuda quede perfectamente localizada y gobernada.

No puede cerrarse si persisten contradicciones no gobernadas entre IR, catálogo, emisión observable y documentación pública.

La auditoría de reapertura se ha dirigido específicamente a ese umbral y no a completar por anticipado las obligaciones funcionales de `FFL-B`, `FFL-E` o de un backend futuro.

## 3. Evidencia material contrastada

El cierre se apoya en lectura fresca y cruzada de:

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

## 4. Resultado de la resincronización por identificador

El estado vigente queda fijado en:

- **38 códigos** en la IR v0.2;
- **37 códigos** en el catálogo implementativo efectivo;
- **4 coincidencias semánticas por mismo identificador:** `E102`, `E104`, `E106`, `E111`;
- **20 casos de mismo identificador con significado distinto**;
- **14 códigos solo IR**;
- **13 códigos solo implementación**.

La auditoría corrigió residuos documentales que habían quedado superados por el propio repositorio:

- `E102` y `E104` constan hoy como convergentes por identificador y obligación;
- `E101` y `E105` constan con cobertura explícita de suite;
- `E202`, `E204` y `E205` constan con cobertura explícita en la suite declarada;
- `E303` mantiene emisión observable y cobertura explícita bajo su significado implementativo vigente y no se reabre;
- `E008` permanece en el catálogo por trazabilidad, pero no se presenta ya como emisión directa ni como caso explícito de suite;
- `E507` queda registrado con fase efectiva `parse`, concordante con su sitio real de emisión.

Estas correcciones no renumeran el catálogo ni alteran la IR.

## 5. Separación entre concordancia numérica y cobertura funcional

La matriz por identificador no es suficiente para juzgar el estado material de las obligaciones canónicas. Por ello se ha fijado un crosswalk funcional independiente.

Ese crosswalk distingue:

- obligación convergente bajo el mismo ID;
- obligación protegida bajo otro ID;
- obligación precluida por la estructura o superficie actual;
- protección parcial;
- obligación no materializada.

Esto evita dos errores simétricos: declarar incumplimiento sólo porque el número no coincide, o declarar cumplimiento sólo porque el identificador coincide.

## 6. Deuda residual reconocida

Permanece deuda funcional y de ABI. Entre los puntos localizados figuran, sin pretensión de agotamiento de los bloques posteriores:

- comprobación canónica de actualización fuera de posiciones puente (`E202` canónico);
- obligación de operador de conflicto cuando concurra el supuesto correspondiente (`E204` canónico);
- compatibilidad del conector declarado en cada arista (`E206` canónico);
- juicios ejecutivos sobre conteos, umbral y precedencia de clasificación (`E301–E303` canónicos);
- pertenencia de cada tipo de suceso declarado en `TransitionData.events` al horizonte referenciado (`E403` canónico);
- suficiencia material de `TransitionData` (`E406` canónico);
- obligaciones parciales de ternarización, resolución, captura, admisibilidad y determinismo de compuerta;
- obligaciones posteriores de consulta, justificación y ABI todavía no materializadas.

Esta deuda **no se declara resuelta**. Queda localizada en `DFL-001` y en el crosswalk funcional y deberá ser absorbida únicamente por el bloque técnico al que corresponda.

## 7. Adversarial de cierre

Se han atacado expresamente las siguientes posibilidades de falso cierre:

1. **Falsa convergencia por compartir ID.** Queda impedida por la matriz vigente: 20 identificadores continúan declarados como semánticamente divergentes.
2. **Falsa ausencia por cambio de ID.** Queda impedida por el crosswalk funcional, que reconoce obligaciones canónicas materialmente protegidas bajo otro diagnóstico.
3. **Falsa cobertura de códigos catalogados.** `E008` queda expresamente identificado como no directamente emitido y sin caso explícito de suite.
4. **Falsa ejecutabilidad.** La protección estructural o parcial no se presenta como ejecución completa; en particular, la determinación de una tabla de compuerta no se confunde con ejecución material de `GateResult.output`.
5. **Traslado silencioso a backend.** No se abre backend ni se usa un futuro backend como destino ficticio de la deuda.
6. **Renumeración cosmética.** La Vía B se mantiene; no se renumeran códigos para producir apariencia de cierre.

Tras estos ataques no se identifica una contradicción diagnóstica **no gobernada** que impida cerrar `FFL-A` bajo el criterio vigente.

## 8. Límite de la evidencia

Esta acta no afirma una nueva ejecución local independiente de la suite completa en el momento del cierre. La evidencia aquí utilizada comprende los sitios de emisión observables en el código, el manifiesto de casos y códigos esperados de `tests/run_conformance.py`, las pruebas ya incorporadas al repositorio y la documentación de saneamiento previa.

La ejecución y suficiencia global de la batería pertenece al cierre de `FFL-C` y deberá acreditarse allí de forma específica.

## 9. Decisión

> **FFL-A — CERRADO BAJO VÍA B, CON DEUDA RESIDUAL RECONOCIDA Y GOBERNADA.**

El cierre significa:

- contrato efectivo públicamente identificable;
- relación con la IR v0.2 explícita;
- emisión y cobertura no sobreatribuidas en la documentación diagnóstica vigente;
- divergencias por ID localizadas;
- correspondencias funcionales separadas;
- deuda canónica parcial o no materializada identificada y trasladable a su bloque natural.

No significa:

- convergencia completa IR ↔ implementación;
- agotamiento del ABI;
- implementación de todos los errores canónicos;
- cierre de `FFL-B`, `FFL-C`, `FFL-D` o `FFL-E`;
- autorización de backend, Rust, WASM, IA productiva, NLP o `NL → SVP`.

## 10. Continuidad inmediata

El siguiente bloque secuencial es:

> **FFL-B — cadena de implementación: lexer ↔ parser ↔ lowering ↔ validator ↔ serializer ↔ CLI/Playground.**

Su apertura deberá comenzar de nuevo desde el repositorio fresco. Las deudas identificadas por el crosswalk solo se materializarán allí si pertenecen realmente a la cadena de implementación y si la IR y la frontera normativa exigen su cierre en la superficie vigente.

No se arrastrará automáticamente a `FFL-B` toda deuda de ABI o de superficies futuras.
