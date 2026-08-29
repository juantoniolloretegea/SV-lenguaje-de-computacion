# Acta técnica de encargo correctivo incidental por huecos heredados detectados durante Beta 2 y bloqueo de continuidad de R2

**Fecha:** 29 de agosto de 2026  
**Estado:** deuda heredada confirmada; continuidad de R2 suspendida  
**Corte de referencia:** `SV-lenguaje-de-computacion@f6b704e819e7d08589f00e9e3f9550deab21d0b1`

## 1. Objeto

Durante la verificación de Beta 2 se amplió el contraste de conformidad más allá de la batería comprometida y se localizaron tres huecos de cierre gramatical en la realización Rust, junto con dos incidencias históricas de documentación e inventario.

La comparación contra el corte de referencia demuestra que estos defectos ya estaban presentes antes de Beta 2. En consecuencia, se registran como **deuda heredada del núcleo descubierta durante Beta 2**, no como regresiones introducidas por esa fase.

El hallazgo origina un **encargo correctivo incidental** separado del alcance funcional de Beta 2. Su finalidad es restablecer la correspondencia entre gramática, realización Rust, corpus de conformidad y material histórico antes de continuar las fases que dependen de esa base.

Esta acta no introduce semántica nueva, no amplía la gramática y no autoriza promoción alguna a producción.

## 2. Evidencia de ausencia de regresión de Beta 2

Se construyeron, con Rust 1.98.0 y bajo la misma infraestructura de ejecución, dos realizaciones:

1. el corte productivo exacto `f6b704e819e7d08589f00e9e3f9550deab21d0b1`;
2. el candidato Beta 2 reconstruido desde ese mismo corte.

Sobre ambos se ejecutaron cuatro sondas negativas específicamente construidas para las producciones afectadas:

```text
semantic_relation_kind_foreign.svp: PROD=0 BETA2=0 SAME=True
pattern_kind_foreign.svp: PROD=0 BETA2=0 SAME=True
graph_regime_foreign.svp: PROD=0 BETA2=0 SAME=True
graph_regime_foreign_bypasses_simple_concurrency.svp: PROD=0 BETA2=0 SAME=True
ALL_SAME=True
```

En el ejecutable utilizado, el código de retorno `0` significa aceptación.

La misma campaña verificó además los archivos `.svp` ya rastreados en el corte productivo:

```text
archivos .svp rastreados = 90
coincidencia Python ↔ Rust Beta 2 = 90/90
```

La batería comprometida de conformidad también permaneció alineada. Los huecos sólo aparecieron al construir sondas negativas para literales cerrados que no estaban representados en el corpus existente.

Por tanto:

```text
hallazgo durante Beta 2
≠
regresión causada por Beta 2
```

## 3. DG-01 — cierre ausente de `SemanticRelation.kind` en Rust

La gramática vigente conserva de v0.1 la producción cerrada:

```ebnf
semantic_relation_kind ::= "DeclaredRelation" ;
```

La sonda:

```text
kind: ForeignRelation;
```

produce:

```text
Python = rechazo
Rust   = aceptación
```

La realización Rust admite, por tanto, una grafía fuera del lenguaje definido.

**Clasificación:** deuda heredada de conformidad gramatical del núcleo.

**Condición de cierre:** Rust debe rechazar todo valor distinto de `DeclaredRelation`, con caso negativo permanente en la batería de conformidad y regresión posterior sobre los destinos aplicables.

## 4. DG-02 — cierre ausente de `Pattern.kind` en Rust

La gramática vigente conserva de v0.1 la producción cerrada:

```ebnf
pattern_kind ::= "DeclaredPattern" ;
```

La sonda:

```text
kind: ForeignPattern;
```

produce:

```text
Python = rechazo
Rust   = aceptación
```

**Clasificación:** deuda heredada de conformidad gramatical del núcleo.

**Condición de cierre:** Rust debe rechazar todo valor distinto de `DeclaredPattern`, con caso negativo permanente en la batería de conformidad y regresión posterior sobre los destinos aplicables.

## 5. DG-03 — cierre ausente de `Graph.regime` y elusión de una restricción de `Simple`

La gramática vigente establece:

```ebnf
regime_literal ::= "Simple" | "General" ;
```

La sonda:

```text
regime: ForeignRegime;
```

produce:

```text
Python = rechazo
Rust   = aceptación
```

El efecto es material y no únicamente nominal. El caso canónico `tests/conformance/invalid/graph_simple_concurrencia_mismo_puente.svp` debe rechazarse bajo régimen `Simple` por concurrencia incompatible sobre un mismo destino y posición. Sustituyendo exclusivamente:

```text
regime: Simple;
```

por:

```text
regime: ForeignRegime;
```

se obtiene:

```text
Python = rechazo
Rust   = aceptación
```

Como control positivo, la sustitución por el literal válido `General` es aceptada por ambas realizaciones.

La realización Rust activa la restricción específica cuando el régimen es exactamente `Simple`; al no cerrar previamente el dominio del literal, una grafía ajena queda fuera de esa condición y puede eludir la restricción.

**Clasificación:** deuda heredada material de conformidad del núcleo, de prioridad alta.

**Condición de cierre:** el dominio de `Graph.regime` debe quedar cerrado a `Simple | General`; la sonda de literal ajeno y la sonda específica de elusión deben incorporarse como regresiones permanentes.

## 6. DD-01 — desincronización histórica de la EBNF en cierres internos

La EBNF histórica de `connector.mapping` y `admissibility_table.table` conserva un punto y coma después de la llave de cierre del bloque interno. Sin embargo:

- los casos canónicos vigentes omiten ese punto y coma;
- la realización Rust sigue la forma sin punto y coma;
- la realización Python admite también la forma con punto y coma;
- el historial técnico contiene una corrección del cierre de llaves en `connector` y `table` cuyos ejemplos pasaron de `};` a `}`.

La evidencia disponible indica una desincronización documental entre la producción EBNF conservada y la forma adoptada posteriormente por ejemplos, corpus y realización Rust.

**Clasificación:** deuda documental heredada.

**Condición de cierre:** reconciliar expresamente la producción normativa con la forma efectivamente adoptada, sin ampliar la realización Rust por mera inercia histórica.

## 7. VH-01 — vector adversarial histórico con estatuto de validez obsoleto

`tests/adversarial/deep_nested_query_valid.svp` conserva la forma histórica:

```text
states: {Ok, Degraded, Failed, U};
```

La Gramática 0.2 sustituyó ese conjunto por:

```text
{Ok, Degraded, NotAdmitted}
```

La realización Python vigente y Rust rechazan actualmente el vector. No existe divergencia entre ambas realizaciones en este punto.

El archivo es un vector adversarial histórico con procedencia identificable, pero su denominación `valid` ya no representa su estatuto respecto de la gramática vigente. Al encontrarse fuera de `tests/conformance/`, no formó parte de la cardinalidad de la batería de conformidad utilizada para el cierre de R0.

**Clasificación:** deuda heredada de inventario y trazabilidad de pruebas.

**Condición de cierre:** reclasificar, renombrar o reubicar el vector preservando su historia y evitando que pueda interpretarse como programa válido bajo la gramática vigente.

## 8. Estatuto de Python

La revisión de los archivos Python no ha identificado una dependencia del ejecutable Rust respecto de Python.

Los módulos Python conservan una función legítima como realización diferencial de referencia y como infraestructura de pruebas, comparación, metrología y caracterización. Su mera permanencia no constituye deuda de ejecución de Rust.

En consecuencia, este encargo no ordena una retirada general de Python. La exigencia es mantener clara su condición de referencia diferencial y evitar que se confunda con la realización soberana Rust.

## 9. Efecto sobre cierres previamente declarados

### 9.1. R0

El cierre integral de R0 acreditó la correspondencia entre la realización Rust y la referencia diferencial sobre el corpus de conformidad comprometido. La nueva evidencia demuestra que ese corpus no contenía sondas negativas para tres producciones cerradas y que Rust acepta formas que la gramática excluye.

Por ello, el cierre de R0 se **reabre de forma correctiva y estrictamente acotada** al perímetro de conformidad gramatical afectado por DG-01, DG-02 y DG-03.

Las pruebas y propiedades de R0 no afectadas por estos hallazgos conservan su valor probatorio. La reapertura no equivale a una anulación general de R0.

### 9.2. R1

No se ha demostrado un defecto propio de las propiedades de autoridad, mediación o decisión protegida materializadas en R1. Sin embargo, su acta de cierre utilizó como condición de base la ausencia de regresión respecto de R0 y asumió `R0 = CERRADO`.

Mientras la reapertura correctiva de R0 permanezca abierta, el cierre de R1 queda **suspendido como habilitación para continuar a fases posteriores**. Sus resultados técnicos se conservan, pero deberán volver a contrastarse contra la base R0 corregida antes de restablecer su eficacia de continuidad.

### 9.3. R2

R2 fue abierto sobre la premisa:

```text
R0 = CERRADO
R1 = CERRADO
```

Esa premisa ha dejado de estar plenamente acreditada hasta el cierre correctivo descrito en esta acta.

Por tanto:

```text
R0 = REABIERTO_CORRECTIVAMENTE_EN_CONFORMIDAD
R1 = CIERRE_SUSPENDIDO_POR_DEPENDENCIA
R2 = SUSPENDIDO
R3 = NO HABILITADO POR ESTA ACTA
R4 = NO HABILITADO POR ESTA ACTA
```

No deberá continuar la realización ni el cierre de R2 mientras permanezcan abiertos los requisitos de este encargo.

## 10. Requisitos para levantar el bloqueo

El bloqueo sólo podrá levantarse cuando exista evidencia conjunta de que:

1. DG-01, DG-02 y DG-03 han sido corregidos en la realización Rust;
2. las cuatro sondas negativas decisivas forman parte de una batería permanente de regresión o conformidad;
3. la batería de conformidad previa continúa íntegramente satisfactoria;
4. la correspondencia diferencial Python ↔ Rust continúa satisfactoria en el alcance aplicable;
5. los destinos nativo y WebAssembly que dependan de `sv_core` continúan sin divergencias relevantes;
6. DD-01 ha sido reconciliada documentalmente;
7. VH-01 ha quedado reclasificado con trazabilidad histórica conservada;
8. una verificación correctiva vuelve a acreditar el perímetro reabierto de R0;
9. R1 ha sido revalidado frente a la base corregida y su cierre vuelve a quedar habilitado para continuidad;
10. un documento posterior levanta expresamente la suspensión de R2.

Ningún resultado parcial satisface por sí solo el conjunto anterior.

## 11. Trazabilidad hacia Beta 2 y una futura Beta 3

Beta 2 conserva su clasificación de **no regresiva** respecto de estos hallazgos: los tres huecos principales ya existían en el corte del que parte.

El encargo correctivo queda separado de la evolución funcional de Beta 2. No deberá resolverse mediante una modificación lateral que mezcle perfiles, entorno, interfaz o ensamblaje multifuente con una corrección del contrato del núcleo.

Una futura Beta 3 deberá partir de una base en la que este encargo esté cerrado o deberá declarar expresamente la deuda todavía abierta. Las nuevas sondas negativas formarán parte de su regresión heredada para impedir la reaparición de los mismos huecos.

## 12. Custodia de evidencia

La evidencia detallada de la auditoría queda preservada en el laboratorio del proyecto. Los paquetes principales están identificados por las siguientes huellas SHA-256:

```text
09dbdc37151d60d2ea3da6b1df18388223bbaaca3478c641bf76d06319c6a1f7
4ec08a82a903f74483303b374b4fd4a77840fde95594184f0bb0cd55523d10da
a51945eb1d0ebd64dca444426717277076423d2cc1399e4d6ed75aa1db4d8163
```

La reproducción esencial no depende de esos paquetes: las sondas, el corte de referencia y los resultados necesarios para reconstruir la comprobación quedan descritos en esta acta.

## 13. Decisión de continuidad

Los defectos DG-01, DG-02 y DG-03 se consideran **bloqueos estructurales**, no mejoras opcionales ni elementos propios de una lista de deseos. DD-01 y VH-01 forman parte del mismo encargo correctivo porque afectan a la concordancia documental y a la trazabilidad de la prueba.

Hasta su cierre acreditado:

- R0 permanece reabierto únicamente en el perímetro indicado;
- R1 conserva sus resultados, pero su cierre no habilita continuidad hacia R2;
- R2 permanece suspendido;
- Beta 2 puede completar sus comprobaciones dentro de su alcance propio, pero no habilita por sí sola ninguna promoción;
- no se autoriza promoción a producción.

Una eventual decisión de promoción sólo podrá considerarse después de que este encargo y los restantes frentes aplicables se encuentren cerrados con evidencia satisfactoria. Esa decisión deberá adoptarse de forma separada y expresa.