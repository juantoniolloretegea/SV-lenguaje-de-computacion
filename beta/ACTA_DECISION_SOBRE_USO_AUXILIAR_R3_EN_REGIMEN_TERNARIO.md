# ACTA DE DECISIÓN SOBRE EL USO AUXILIAR DE LA CARTA EN R³ EN EL RÉGIMEN TERNARIO DEL LENGUAJE SV

**Fecha:** 16/03/2026  
**Estado:** Vigente  
**Rango:** Decisión operativa subordinada en `beta/`  
**Ámbito:** Proyecto `SV-lenguaje-de-computacion` — régimen ternario canónico `(0,1,U)`

## 1. Objeto del acta

La presente acta fija la posición operativa del proyecto respecto de la línea de trabajo consistente en representar, con fines auxiliares, el polígono ternario del Sistema Vectorial SV mediante una carta espacial en `R³`, manteniendo la carta polar plana en `R²` como proyección canónica de referencia.

La decisión no reabre la doctrina del signo `U`, no modifica la célula ternaria canónica y no altera por sí sola la semántica nuclear del lenguaje SV.

## 2. Antecedente inmediato

Se ha estudiado una representación auxiliar en `R³` en la que los estados determinados permanecen en el plano `z = 0`, mientras que la aparición de `U` se modela mediante una elevación finita `z = h`, preservando el orden angular de los parámetros.

Ese estudio ha mostrado utilidad geométrica y analítica suficiente para justificar una admisión subordinada de la línea como herramienta de laboratorio, sin que ello autorice su promoción al núcleo del sistema.

## 3. Decisión

Se acuerda:

### 3.1. Admisión subordinada

Admitir la carta auxiliar en `R³` como línea válida de trabajo en el régimen ternario del proyecto, exclusivamente en calidad de representación auxiliar, instrumento de análisis y herramienta de laboratorio.

### 3.2. No promoción al núcleo

Declarar expresamente que esta admisión no comporta:

- modificación de la célula ternaria canónica `(0,1,U)`;
- reinterpretación ontológica del signo `U`;
- incorporación de la carta en `R³` a la semántica primaria del lenguaje;
- alteración automática de IR, gramática, validator, lowering, runner o tests nucleares.

### 3.3. Naturaleza del parámetro `h`

Fijar que el parámetro `h` pertenece a la carta geométrica auxiliar y no al estatuto semántico del signo `U`.

Por tanto, `h` no podrá invocarse como medida intrínseca de indeterminación ni como atributo propio del sistema ternario exacto.

## 4. Fundamento de la decisión

La admisión se apoya en que la carta auxiliar en `R³` permite hacer visible, sin alterar la ontología primaria del sistema, una ruptura local de coplanaridad asociada a la presencia de `U`, y permite definir magnitudes discretas útiles para análisis comparativo.

Entre esas magnitudes, el estudio previo ha dejado establecida de forma exacta la expresión:

`Ez(v,h) = k(v) · h²`

donde `k(v)` representa el número estructural de transiciones verticales inducidas por la presencia de `U` en la carta elegida.

Asimismo, se ha constatado estabilidad empírica útil de otras magnitudes, en particular `ΔL`, dentro de la batería ensayada, sin que ello equivalga todavía a un teorema general de invariancia.

## 5. Usos autorizados

Mientras esta línea permanezca en régimen subordinado, la carta auxiliar en `R³` podrá emplearse en los siguientes planos:

### 5.1. Tooling y análisis

- análisis geométrico auxiliar de células y vectores;
- cálculo de métricas discretas de transición;
- comparación entre configuraciones con distinta distribución de `U`;
- clasificación experimental de patrones.

### 5.2. Depuración y trazabilidad

- visualización de estados intermedios;
- apoyo a tareas de depuración semántica;
- trazabilidad de transformaciones sobre células ternarias;
- inspección de concentraciones, dispersión o transiciones asociadas a `U`.

### 5.3. Vigilancia metodológica

- control de configuraciones que, siendo formalmente ternarias, presenten complejidad geométrica relevante;
- apoyo auxiliar a futuras decisiones de laboratorio o de diseño de tooling.

## 6. Usos no autorizados

La presente acta excluye expresamente los siguientes usos:

- tomar la carta en `R³` como semántica primaria del sistema;
- presentar `U` como magnitud geométrica, numérica o métrica intrínseca;
- integrar sin decisión posterior expresa esta línea en el núcleo del lenguaje `.svp`;
- usar `ρ`, `z`, `h`, `Ez`, `ΔL` u otras magnitudes auxiliares como sustituto del significado exacto de `0`, `1` y `U`;
- convertir la representación auxiliar en criterio automático de verdad semántica.

## 7. Criterio de encaje en el proyecto

La ubicación correcta de esta línea es `beta/`, no el núcleo del lenguaje.

Se trata de una proposición de laboratorio útil para el régimen ternario vigente, pero de rango subordinado. Su función es ampliar la capacidad de análisis, no redefinir el sistema.

## 8. Consecuencia operativa para futuras actuaciones y agentes del proyecto

Toda actuación futura sobre esta materia deberá partir de las siguientes reglas:

1. la terna `(0,1,U)` sigue siendo el único eje canónico vigente;
2. la carta en `R³` solo es admisible como representación auxiliar;
3. cualquier integración en tooling del lenguaje deberá respetar la separación entre objeto exacto y carta geométrica;
4. toda propuesta de ascenso de rango requerirá decisión expresa posterior de la dirección del proyecto y no podrá inferirse de esta acta.

## 9. Cierre

La línea de elevación auxiliar del polígono ternario a una carta espacial en `R³` queda, por tanto, admitida como vía de trabajo práctica y legítima dentro del laboratorio subordinado del proyecto, con utilidad en análisis, depuración, visualización y control, pero sin promoción al núcleo semántico del lenguaje SV.

Su valor presente es instrumental y metodológico.  
Su eventual valor futuro como componente más fuerte del ecosistema SV queda expresamente diferido.
