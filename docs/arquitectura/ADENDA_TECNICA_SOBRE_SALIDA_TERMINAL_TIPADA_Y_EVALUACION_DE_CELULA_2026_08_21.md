# Adenda técnica sobre salida terminal tipada y evaluación de célula

**Fecha:** 21/08/2026  
**Estado:** especificación técnica de FFL-E  
**Ámbito:** evaluación de célula, codominio de salida e interfaces tipadas

## 1. Objeto

Esta adenda corrige una especialización excesiva detectada entre la estructura general del Lenguaje SV y la formulación vigente de `EvalResult` en la IR v0.2.

La célula declara un codominio de salida finito y documentado, mientras que la IR v0.2 describe el campo terminal de `EvalResult` mediante una enumeración fija asociada a las etiquetas `APTO`, `NO_APTO` e `INDETERMINADO`.

El objetivo es preservar la semántica de evaluación ya fijada y, al mismo tiempo, impedir que un conjunto concreto de etiquetas de dominio se convierta indebidamente en tipo universal del Lenguaje.

## 2. Base matemática

Para una célula `C_i`, la función de evaluación tiene la forma

`chi_i : C_i -> K_i`,

donde `K_i` es el codominio tipado de salida de la célula.

`K_i` es finito, explícito y semánticamente documentado. Las etiquetas utilizadas por una familia de dominio no obligan a que todos los dominios empleen las mismas etiquetas.

La transmisión intercelular mantiene además una separación de tipos:

`phi : K_i -> Tri`.

Por tanto, la salida terminal de una célula pertenece primero a `K_i`; sólo una transducción posterior y declarada puede producir un valor de `Tri` para una posición receptora.

## 3. Desajuste de la IR v0.2

La IR v0.2 declara de forma general:

`CellSpec.codomain : Codomain`,

pero describe `EvalResult.classification` mediante un tipo fijo equivalente a:

`APTO | NO_APTO | INDETERMINADO`.

Las dos decisiones no tienen el mismo alcance. Si `CellSpec.codomain` es genérico, la salida terminal de `evaluate` no puede quedar tipada universalmente por tres nombres pertenecientes a una familia concreta de codominios.

La implementación de referencia no ejecuta actualmente `evaluate`; por ello el desajuste no constituye un fallo de ejecución observado. Es una limitación de la especificación que debe resolverse antes de atribuir semántica ejecutiva general a la evaluación.

## 4. Separación entre resultado estructural del umbral y etiqueta de dominio

Para las células que utilizan el umbral vigente `T(n) = floor(7n/9)`, se distinguen dos niveles.

### 4.1. Resultado estructural del umbral

Se define conceptualmente:

`ThresholdOutcome = { ZeroStrong, OneStrong, NoStrong }`.

Para un vector `v`:

- `OneStrong` si `N1(v) >= T(n)`;
- `ZeroStrong` si `N0(v) >= T(n)` y no se cumple la condición anterior;
- `NoStrong` en cualquier otro caso.

Esta enumeración expresa el resultado estructural del motor de umbral. No pertenece a `Tri` y no fija las etiquetas visibles del dominio.

### 4.2. Interpretación terminal tipada

Cada célula que utilice este evaluador deberá declarar una interpretación terminal equivalente a:

`TerminalInterpretation = { codomain, zero_strong, one_strong, no_strong }`.

Los tres valores designados deben pertenecer al `Codomain` declarado por la célula.

La salida terminal se obtiene mediante una aplicación

`mu_i : ThresholdOutcome -> K_i`.

De este modo, para esta familia de evaluadores:

`chi_i = mu_i ∘ threshold_outcome ∘ counts`.

La regla de umbral permanece única y determinista; lo que varía legítimamente entre dominios es la etiqueta tipada con la que cada resultado estructural se expresa en `K_i`.

## 5. Ejemplos de interpretación

Una célula puede declarar:

`ZeroStrong -> APTO`

`OneStrong -> NO_APTO`

`NoStrong -> INDETERMINADO`.

Otra célula puede declarar:

`ZeroStrong -> NORMAL`

`OneStrong -> INTRUSION`

`NoStrong -> INDETERMINATE`.

Ambas pueden compartir la misma regla estructural de umbral sin compartir el mismo codominio nominal.

Estas correspondencias son ejemplos de tipado de salida; no convierten las etiquetas terminales en valores de `Tri`.

## 6. Consecuencia para `EvalResult`

Una revisión futura de la IR deberá evitar que `EvalResult` presente como universal una enumeración de etiquetas perteneciente a un dominio concreto.

Para las células que usan el evaluador de umbral descrito, la forma mínima prevista es conceptualmente:

`EvalResult = {`

`  source_state : CellStateRef,`

`  counts : (Nat, Nat, Nat),`

`  threshold : Nat,`

`  threshold_outcome : ThresholdOutcome,`

`  output : CodomainValue,`

`  criticality : Int,`

`  deltas : (Int, Int)`

`}`.

`output` deberá pertenecer al codominio de la `CellSpec` de origen y resultar de la interpretación terminal declarada.

El nombre y la forma definitiva de estos campos se fijarán al versionar la IR. Esta adenda fija la separación semántica que esa revisión deberá preservar.

## 7. Alcance de `TerminalInterpretation`

`TerminalInterpretation` resuelve la relación entre el evaluador de umbral vigente y codominios terminales con etiquetas distintas.

No se afirma que toda función futura `chi_i : C_i -> K_i` deba factorizar necesariamente por `ThresholdOutcome`. Si un dominio futuro necesita una función de evaluación materialmente distinta o un codominio cuya semántica no pueda expresarse mediante estos tres resultados estructurales, deberá existir una especificación de evaluación propia antes de incorporarla al Lenguaje.

No se generaliza por anticipación una familia de evaluadores que la matemática no haya fijado.

## 8. Consecuencia para conectores

La forma del conector permanece:

`Connector : K_i -> Tri`.

El conector recibe el valor `output` ya tipado en `K_i` y aplica su correspondencia declarada hacia `Tri` cuando la salida de una célula deba ocupar una posición de otra.

No se permite omitir este paso tratando `APTO`, `NORMAL`, `INTRUSION`, `INDETERMINADO` u otras etiquetas terminales como literales de `Tri`.

## 9. Consecuencia para representaciones

Una representación terminal `F_m` que conserve únicamente la salida de la célula tiene tipo

`F_m : X_D -> K_D`.

Su tipo de salida es el codominio terminal declarado por el dominio o la célula correspondiente.

Si posteriormente una interfaz aplica una transducción `phi : K_D -> Tri`, el objeto transmitido pasa a ser `phi ∘ F_m`; esa composición no convierte retroactivamente `F_m` en una representación de tipo `Tri`.

Esta separación es obligatoria para evaluar correctamente la suficiencia representacional de operaciones e interfaces.

## 10. Consecuencias diagnósticas futuras

Antes de una futura ejecución de `evaluate`, deberán quedar representables al menos las siguientes condiciones de fallo:

- la interpretación terminal referencia un valor ausente del codominio de la célula;
- la evaluación produce un valor que no pertenece al codominio declarado;
- se intenta usar una etiqueta terminal como `Tri` sin transducción explícita;
- la `EvalResult.output` no corresponde al estado, conteos, umbral e interpretación terminal declarados.

Esta adenda no asigna códigos numéricos a dichas condiciones. Los códigos se fijarán cuando se determine su punto de emisión y exista una ruta observable en la especificación o implementación correspondiente.

## 11. Compatibilidad con el estado actual

La gramática v0.1, el AST, la IR v0.2 publicada y la implementación de referencia permanecen sin cambios en esta fase.

El `Codomain` y `OutputSemantics` ya existentes proporcionan parte de la estructura necesaria, pero `OutputSemantics` sólo documenta el significado de los valores y no declara por sí mismo qué valor corresponde a cada resultado estructural del umbral.

Por tanto, no procede reutilizar silenciosamente `OutputSemantics` como si fuese una interpretación terminal ejecutable.

## 12. Decisión de FFL-E

FFL-E fija las siguientes obligaciones para cualquier revisión posterior de la evaluación:

1. preservar `Tri` sin ampliación;
2. preservar `T(n)` y la regla de clasificación vigente para las células que pertenecen a esta familia de evaluación;
3. separar el resultado estructural del umbral de la etiqueta terminal del dominio;
4. tipar la salida de `EvalResult` por el codominio declarado de su célula de origen;
5. mantener toda transducción `K_i -> Tri` como operación explícita y separada;
6. no atribuir a la implementación actual una evaluación que todavía no ejecuta.

La resolución de este desajuste es condición previa para cerrar FFL-E y para cualquier fase posterior que pretenda ejecutar de manera general la evaluación de células con codominios especializados.
