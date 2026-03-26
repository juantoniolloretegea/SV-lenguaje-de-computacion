# Acta técnica de cierre provisional de PC-HNA y condiciones de reapertura

**Fecha:** 26/03/2026  
**Naturaleza:** acta operativa subordinada de gobierno semántico  
**Frente:** Lenguaje SV / control de fundamento prebackend  
**Estado:** cierre provisional con condiciones expresas de reapertura  
**Registro técnico asociado:** véase `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv` como fuente maestra de numeración `RETP`

## 1. Objeto

Dejar constancia expresa de que, en el estado actual del Lenguaje SV, la condición de cierre provisional denominada `PC-HNA` queda suficientemente satisfecha para seguir avanzando sin trasladar todavía precedencia semántica fuerte a `IR`, `validator`, `runner` o backend.

La presente acta **no** constituye una nueva pieza doctrinal y **no** sustituye la demostración superior del frente VII. Su función es estrictamente operativa: sellar el suelo mínimo necesario antes de futuros desarrollos que puedan usar precedencia entre sucesos, familias de preservación, clases locales de régimen o células especializadas.

## 2. Hechos constatados

Consta materialmente en el repositorio técnico actual que:

1. `Trajectory` se gobierna como estructura `append-only` por tipo y por diagnóstico visible.
2. `Frame` permanece gobernado como estructura inmutable.
3. `TransitionData` exige hoy referencia a `Horizon` para no incurrir en incoherencia diagnóstica visible.
4. La aciclicidad estructural del grafo del lenguaje se impone como condición propia del modelo técnico actual y no depende, por sí sola, de una relectura tardía del frente VII.
5. No consta en la gramática superficial publicada una operación legítima de remoción, cancelación o reescritura retroactiva de eventos del `Horizon`.

## 3. Cierre provisional de PC-HNA

A efectos de gobierno técnico del Lenguaje SV, se declara provisionalmente satisfecha `PC-HNA` bajo estas dos premisas acumulativas:

1. el `Horizon` vigente del lenguaje se trata operativamente como superficie declarada que no admite remoción retroactiva de eventos por la gramática superficial actualmente publicada;
2. no procede, en el estado actual, modelar admisibilidad dependiente de la **ausencia** táctica de eventos del horizonte como si esa semántica estuviera ya cerrada.

Bajo esas dos premisas, la vigilancia del hueco HNA puede considerarse **provisionalmente contenida** para el estado actual del árbol.

## 4. Alcance exacto del cierre

El cierre provisional de `PC-HNA` **sí** autoriza:

- seguir avanzando en gobierno técnico sin reabrir por inercia gramática, `IR`, `validator`, `runner` o backend;
- registrar la cautela como suelo previo para próximos frentes;
- y exigir reapertura formal antes de trasladar precedencia semántica fuerte a capas ejecutivas futuras.

El cierre provisional de `PC-HNA` **no** autoriza:

- declarar resuelta de forma total la precedencia semántica entre sucesos;
- levantar por sí solo `D-01` o `D-03`;
- introducir relecturas fuertes de horizonte dinámico o revisable;
- ni tratar el runner futuro como si esta cuestión estuviera agotada.

## 5. Condiciones explícitas de reapertura

`PC-HNA` deberá reabrirse obligatoriamente si concurre cualquiera de estas situaciones:

1. se propone una forma legítima de remoción, cancelación o reordenación retroactiva de elementos del `Horizon`;
2. se quiere introducir precedencia semántica como objeto operativo fuerte del lenguaje;
3. se pretende levantar `D-01` o `D-03` con traducción ejecutiva de familias preservadas, enlaces o equivalencias;
4. se abre una célula especializada o un frente `NLP` que dependa materialmente de precedencia entre sucesos o de horizontes más ricos que el actual;
5. se propone una semántica de admisibilidad que dependa de ausencias explícitas en el `Horizon`.

## 6. Relación con deuda viva y horizontes

La presente acta no clausura deuda viva por sí sola. Su función es otra:

- complementar la vigilancia ya existente de `H-04` a `H-10`;
- impedir que una contención operativa tácita se confunda con fundamento sellado;
- y dejar trazabilidad de que el siguiente salto de complejidad no podrá apoyarse en este punto sin nueva verificación explícita.

## 7. Fórmula de cierre

`PC-HNA` queda **provisionalmente cerrada** para el estado actual del Lenguaje SV, con condiciones expresas de reapertura y sin autorización automática de implementación semántica fuerte.
