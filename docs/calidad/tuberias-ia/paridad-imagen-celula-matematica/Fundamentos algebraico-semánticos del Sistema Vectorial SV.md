---
article:
  elocation-id: fundamentos-algebraico-semanticos-del-sistema-vectorial-sv
author:
- Juan Antonio Lloret Egea
bibliography: /tmp/tmp-20e4kjhaKkzdjh.json
copyright:
  link: "https://creativecommons.org/licenses/by-nc-nd/4.0/"
  text: Creative Commons Attribution-NonCommercial-NoDerivatives 4.0
    International License
  type: CC-BY-NC-ND
csl: /app/dist/server/server/utils/citations/citeStyles/apa-6th-edition.csl
date:
  day: 09
  month: 03
  year: 2026
journal:
  publisher-name: IA eñ ™ - (La Biblia de la IA - The Bible of AI ™ ISSN
    2695-641)
  title: IA eñ ™
link-citations: true
title: Fundamentos algebraico-semánticos del Sistema Vectorial SV
uri: "https://www.itvia.online/pub/fundamentos-algebraico-semanticos-del-sistema-vectorial-sv"
---

#  {#fundamentos-algebraico-semánticos-del-sistema-vectorial-sv}

![](https://assets.pubpub.org/c39829d0b-6cf2-4c57-8562-04aa66068f1d/pb0cf9a13-4186-4006-9eb4-7c86aceb3077/u8d2bc69d-73c6-4669-a7e7-fb07fdba4171/image-31773057797850.png){#ngjboq1mj9n}

### «Todo ser humano, si se lo propone, puede ser escultor de su propio cerebro.» (Santiago Ramón y Cajal). {#todo-ser-humano-si-se-lo-propone-puede-ser-escultor-de-su-propio-cerebro-santiago-ramn-y-cajal}

![](https://assets.pubpub.org/c39829d0b-6cf2-4c57-8562-04aa66068f1d/pb0cf9a13-4186-4006-9eb4-7c86aceb3077/u8d2bc69d-73c6-4669-a7e7-fb07fdba4171/instancia_x1_EQ_cajal-71773057940731.png){#ndhi1qnfuvd}

#  Fundamentos algebraico-semánticos del Sistema Vectorial SV

## Célula exacta, representación polar, indeterminación epistémica y composición tipada

**Autor:** Juan Antonio Lloret Egea \| **ORCID:**
[0000-0002-6634-3351](https://orcid.org/0000-0002-6634-3351) \|
**ISSN:** 2695-6411 \| **Licencia:** CC BY-NC-ND 4.0 \| **Institución:**
Instituto tecnológico virtual de la Inteligencia Artificial para el
español ™ (ITVIA) \| **Versión:** 1.0.0 **Lugar y fecha:** Madrid,
09/03/2026 \| **Estado:** Texto fundacional de referencia

------------------------------------------------------------------------

## Resumen

Este trabajo fija, con alcance fundacional, la formulación matemática y
arquitectónica del **S**istema **V**ectorial (**SV)**. Su objeto no es
describir una implementación concreta ni defender un modelo de
inteligencia artificial particular, sino establecer qué es el sistema,
qué no es, qué invariantes lo constituyen y bajo qué condiciones puede
ampliarse sin perder coherencia formal.

La tesis central es cuádruple. Primero, la unidad primaria del marco es
la célula SV exacta, entendida como un producto cartesiano finito de
estados ternarios, y no como un espacio vectorial sobre un cuerpo en su
ontología inicial. Segundo, su representación geométrica exacta es, en
general, una poligonal polar cerrada, por lo que no debe confundirse con
una función global de la forma y = f(x). Tercero, el símbolo U no
representa un número ni un infinito algebraico, sino un estado
epistémico de no determinación actual, susceptible de resolución
posterior por intervención humana o experta. Cuarto, la composición
entre células no se rige por una única ley universal: exige una álgebra
semántica tipada, con operadores distintos según la relación funcional,
jerárquica o supervisora entre las células implicadas.

Desde el punto de vista aplicado, el documento sitúa a la inteligencia
artificial en su lugar correcto: no como fundamento del sistema, ni como
autoridad que gobierne al ser humano, sino como capa auxiliar
estrictamente subordinada al motor normativo, a la geometría del
polígono y a la trazabilidad de las decisiones. El paso del parámetro a
la imagen cerrada persigue precisamente devolver el problema a una forma
que el cerebro humano pueda comprender, discutir y gobernar sobre la
misma representación visible que recorre la máquina. Dentro de este
marco, la estadística y la minería de datos no se aceptan como fuente
soberana de decisión ni como sustituto del juicio trazable: una salida
probabilística u opaca no puede reemplazar a la sentencia primaria del
sistema. El Sistema Vectorial SV sólo admite, como salidas primarias,
Apto, No_Apto o Indeterminado.

------------------------------------------------------------------------

## 1. Introducción y alcance

El Sistema Vectorial SV es un marco algebraico y representacional
destinado a evaluar situaciones complejas mediante una gramática
ternaria estable. En su forma más simple, cada parámetro relevante de
una situación se expresa mediante uno de tres estados: 0, 1 o U. A
partir de ese vector se obtiene una clasificación determinista, una
representación geométrica polar y, cuando procede, una capa de análisis
algorítmico o de revisión experta.\[R1\]\[R2\]

La secuencia vector → polígono cerrado no tiene aquí un propósito
meramente visual. Su finalidad es epistemológica y de gobierno: hacer
que el resultado del sistema adopte una forma visible, estable y
auditable sobre la que el ser humano pueda razonar directamente. La
imagen no se introduce para que la máquina sustituya al juicio humano,
sino para que la máquina quede obligada a recorrer la misma figura que
el ser humano puede inspeccionar, discutir y, en último término,
decidir.

El presente trabajo no pretende sustituir a la documentación doctrinal
publicada hasta la fecha. Su propósito es más preciso: cerrar el aparato
formal mínimo que hace inteligible, verificable y extensible el Sistema
Vectorial SV. En otras palabras, este artículo responde a cuatro
preguntas fundamentales:

-   ¿Qué es exactamente una célula SV en su nivel ontológico primario?

-   ¿Cómo debe representarse sin forzarla indebidamente a marcos
    matemáticos que no le pertenecen?

-   ¿Qué significa el símbolo U y qué consecuencias tiene para la
    decisión y la trazabilidad?

-   ¿Cómo se compone el sistema cuando intervienen varias células sin
    caer en una falsa ley universal única?

El texto se sitúa, por tanto, entre dos riesgos simétricos. El primero
consiste en rebajar el marco a una mera convención ingenieril, sin
aparato matemático suficiente. El segundo consiste en recubrirlo con una
sobreactuación simbólica que lo haga ilegible para cualquier lector que
no sea especialista en álgebra, geometría o teoría de sistemas. Este
artículo busca un tercer camino: máximo rigor con mínima opacidad.

En ese sentido, el alcance del paper es fundacional, no exhaustivo.
Quedan expresamente fuera de su cierre definitivo, por ahora, la
discusión sobre lenguajes concretos de implementación, los detalles de
despliegue por plataforma, y la elección de un stack técnico estable.
Esos puntos pertenecen a un desarrollo posterior.

------------------------------------------------------------------------

## 2. Convención semántica canónica

La convención semántica canónica del Sistema Vectorial SV es la
siguiente:\[R1\]\[R2\]

-   **0** significa **Apto**. En función del dominio, puede leerse
    también como normal, correcto o situación adecuada.

-   **1** significa **No_Apto**. En función del dominio, puede leerse
    también como intrusión, incorrecto, situación inadecuada o alto
    riesgo.

-   **U** significa **Indeterminado**. Expresa que la información
    disponible es insuficiente, ambigua o no resoluble todavía en el
    estado actual del análisis.

Esta convención tiene una importancia mayor de lo que podría parecer. No
es una simple ayuda de lectura: es la pieza que permite mantener la
misma gramática algebraica cuando cambia el dominio. En un contexto de
seguridad, 1 puede leerse como intrusión. En un contexto de evaluación
experta, 1 puede significar no aptitud. En ambos casos, la estructura
formal permanece intacta; lo que cambia es la semántica de dominio
documentada por el motor normativo.\[R1\]\[R2\]

La representación polar visible asociada a estos tres símbolos se fija,
en la implementación canónica, mediante la correspondencia:\[R1\]\[R2\]

-   0 → 1

-   1 → 2

-   U → 3

En términos geométricos, esto significa que el valor 0 se dibuja en el
anillo más interior, 1 en el anillo medio y U en el anillo más exterior.
El lector debe advertir que esta asignación es una codificación visible
y no una prueba de que U sea "más verdadero" o "más grave" en sentido
absoluto. Lo que expresa es otra cosa: la indeterminación se dibuja como
distancia máxima precisamente para no ocultar que el sistema, en ese
punto, no sabe todavía lo suficiente para decidir con honestidad.

------------------------------------------------------------------------

## 3. Ontología primaria de la célula SV

### 3.1. Definición exacta

Sea *b* un número natural con *b* ≥ 3. El número de parámetros de una
célula exacta se define por la restricción arquitectónica

> *n* = *b*²

La célula SV exacta de tamaño *n* se define entonces como

> 𝒮ₙ = {0, 1, U}ⁿ

En palabras sencillas, una célula SV exacta es un vector ordenado de
longitud *n* cuyos componentes pertenecen a un alfabeto ternario. El
hecho de que *n* sea un cuadrado perfecto no es ornamental. La
restricción se introduce para preservar una organización por capas
simétricas y una representación polar de reparto uniforme.\[R2\]\[R3\]

### 3.2. Proposición 1 --- Cardinalidad

El número de células SV exactas posibles de tamaño *n* es

> \|𝒮ₙ\| = 3ⁿ

**Demostración.** Cada una de las *n* posiciones puede tomar exactamente
uno de tres símbolos. Por el principio multiplicativo elemental, el
número total de configuraciones es 3 × 3 × ... × 3, con *n* factores; es
decir, 3ⁿ.

**Comentario explicativo.** Esta proposición parece obvia, pero es
fundacional. Significa que el crecimiento combinatorio del sistema no
depende de una heurística de programación ni de una moda de modelado,
sino de una estructura discreta muy nítida. Cuando *n* = 9, el espacio
exacto contiene 3⁹ configuraciones. Cuando *n* = 25, contiene 3²⁵. Esa
explosión combinatoria justifica que el marco necesite, en ciertos
niveles, herramientas de ayuda algorítmica para explorar patrones sin
alterar la gramática de base.

### 3.3. Proposición 2 --- La célula exacta no es inicialmente un espacio vectorial

En su nivel ontológico primario, 𝒮ₙ = {0, 1, U}ⁿ no debe identificarse
con un espacio vectorial sobre un cuerpo.

**Prueba conceptual.** Un espacio vectorial exige, al menos, la
existencia de una suma interna bien definida y de una multiplicación por
escalares sobre un cuerpo, además de la satisfacción de los axiomas
correspondientes. En la célula exacta del Sistema Vectorial SV no se
postulan inicialmente tales operaciones sobre los símbolos 0, 1 y U. Más
aún: U no es un número, sino un estado epistémico. Por tanto, la célula
exacta debe entenderse primero como producto cartesiano finito de
estados semánticos, y solo después, si conviene, podrá recibir una
codificación auxiliar sobre una estructura algebraica distinta.

**Comentario explicativo.** Esto no prohíbe algebraizar el sistema. Lo
que prohíbe es una confusión frecuente: tratar como "números" lo que, en
origen, son estados con semántica propia. Si en una fase posterior se
define una codificación en 𝔽₃ o en ℝ, esa codificación podrá ser útil;
pero será un modelo auxiliar, no la ontología primaria del sistema.

### 3.4. Dualidad exacta/auxiliar

La consecuencia inmediata es una dualidad metodológica que conviene
declarar desde el principio:

-   **Plano exacto:** la célula es un objeto ternario semántico.

-   **Plano auxiliar:** la célula puede codificarse numéricamente para
    ciertos fines algebraicos, geométricos o computacionales.

Esta dualidad permite trabajar con rigor sin cerrar caminos de
investigación. La investigación puede ampliar el marco; lo que no debe
hacer es borrar la diferencia entre el objeto y su representación.

------------------------------------------------------------------------

## 4. Representación geométrica: del vector a la poligonal polar

### 4.1. Definición de los vértices

Sea **v** = (v₁, ..., vₙ) una célula exacta y sea ρ la codificación
visible que asigna radios 1, 2 y 3 a 0, 1 y U, respectivamente. Se
define el ángulo del eje *i* por

> θᵢ = 2π(i − 1)/n, para i = 1, ..., n.

El vértice visible asociado al parámetro vᵢ es

> Vᵢ(**v**) = ( ρ(vᵢ) · cos θᵢ , ρ(vᵢ) · sin θᵢ )

La representación visible de la célula es la cadena poligonal cerrada
que une consecutivamente los puntos V₁(**v**), ..., Vₙ(**v**) y vuelve a
V₁(**v**).

**Explicación en prosa.** El sistema no dibuja puntos dispersos, sino
una figura cerrada. Cada parámetro ocupa siempre el mismo eje angular.
Lo que cambia no es la dirección, sino la distancia al centro. Esa
simple decisión fija una geometría estable: el lector puede comparar
polígonos distintos sin que cambie el orden de los
parámetros.\[R2\]\[R5\]

### 4.2. Proposición 3 --- La célula no es, en general, una función global y = f(x)

La representación exacta o visible de una célula SV no es, en general,
el gráfico de una función global cartesiana de la forma y = f(x).

**Demostración.** La figura asociada a la célula es una poligonal
cerrada. Una curva cerrada cualquiera no satisface, en general, el
criterio de unicidad vertical: puede haber un mismo valor de *x* con
varios valores de *y*. Por tanto, no existe garantía de que la
representación de una célula sea el gráfico de una única función global
de variable real. Bastan ejemplos simples ---como cualquier polígono
simétrico respecto del eje vertical--- para que una misma recta vertical
corte la figura en dos puntos distintos.\[R4\]\[R5\]

**Comentario explicativo.** Éste es un punto importante porque evita un
error frecuente. La célula sí puede estudiarse mediante funciones
locales por tramos, o mediante parametrización polar, pero no conviene
forzarla a ser algo que no es.

### 4.3. Corolario --- Representación paramétrica y estudio local

Aunque la figura no sea, en general, una función global y = f(x), sí
puede describirse de manera paramétrica o por tramos lineales. En
particular, cada lado de la poligonal visible es un segmento y, si no es
vertical, admite una ecuación local del tipo

> y = aᵢx + bᵢ

Además, la representación polar-paramétrica viene dada por

> x(θ) = r(θ) cos θ
>
> y(θ) = r(θ) sin θ

donde r(θ) es una interpolación conveniente de los radios
discretos.\[R4\]\[R5\]

**Explicación en prosa.** En otras palabras, la figura sí puede
analizarse con las herramientas habituales del cálculo, pero ese
análisis es local o paramétrico. El error estaría en presentar como
función global lo que, por su propia naturaleza geométrica, es una curva
cerrada.

### 4.4. Proposición 4 --- Área visible de la célula

Si rᵢ = ρ(vᵢ) y rₙ₊₁ = r₁, el área visible del polígono viene dada por

> A(**v**) = (1/2) · sin(2π/n) · Σᵢ₌₁ⁿ rᵢ rᵢ₊₁

**Idea de demostración.** La fórmula se obtiene al aplicar la fórmula
del área de Gauss a una sucesión de vértices con salto angular
constante. La constancia del ángulo permite simplificar los productos
cruzados y extraer el factor sin(2π/n).

**Comentario explicativo.** Este resultado es elegante porque traduce
una figura aparentemente compleja a una suma discreta muy manejable. En
la práctica significa que el área visible no depende de una intuición
visual imprecisa, sino de una expresión algebraica explícita. Dos
polígonos distintos pueden compararse por área sin abandonar la
gramática del sistema.

### 4.5. Proposición 5 --- Perímetro visible de la célula

Con la misma notación, y con rₙ₊₁ = r₁, el perímetro visible viene dado
por

> L(**v**) = Σᵢ₌₁ⁿ √( rᵢ² + rᵢ₊₁² − 2 rᵢ rᵢ₊₁ cos(2π/n) )

**Comentario explicativo.** Aquí reaparece una idea importante del
Sistema Vectorial SV: la geometría no es decorativa. El perímetro, el
área y la forma global constituyen variables geométricas derivadas que
pueden ser útiles para análisis comparativos, para detección de patrones
y para módulos algorítmicos auxiliares.

------------------------------------------------------------------------

## 5. Motor normativo y clasificación determinista

### 5.1. Conteos básicos

Para toda célula **v** se definen los conteos

> N₀(**v**) = número de componentes con valor 0
>
> N₁(**v**) = número de componentes con valor 1
>
> Nᵤ(v) = número de componentes con valor U

Naturalmente,

> N₀(**v**) + N₁(**v**) + Nᵤ(**v**) = n

### 5.2. Umbral canónico

El umbral canónico, adoptado normativamente desde \[R1\]\[R2\], se
define como

> T(n) = ⌊7n/9⌋ \[R1\]\[R2\]

A partir de él, la clasificación determinista de una célula exacta se
expresa así:

-   La célula es **No_Apta** si N₁(**v**) ≥ T(n).

-   La célula es **Apta** si N₀(**v**) ≥ T(n).

-   La célula es **Indeterminada** en los demás casos.

### 5.3. Proposición 6 --- Unicidad de la clasificación fuerte

Para toda célula con n = b² y b ≥ 3, no pueden darse simultáneamente las
desigualdades N₀(**v**) ≥ T(n) y N₁(**v**) ≥ T(n).

**Demostración.** Si ambas fueran ciertas, se tendría N₀(**v**) +
N₁(**v**) ≥ 2T(n). Pero para n ≥ 9, se cumple 2⌊7n/9⌋ \> n. En efecto,
⌊7n/9⌋ ≥ 7n/9 − 1, de donde 2⌊7n/9⌋ \> 14n/9 − 2, y para n ≥ 9 esto es
estrictamente mayor que n. Como N₀(**v**) + N₁(**v**) ≤ n, se obtiene
contradicción.

**Comentario explicativo.** Esta proposición asegura que la regla fuerte
no genera dos sentencias incompatibles a la vez. O bien domina el 0, o
bien domina el 1, o bien ninguno de los dos alcanza el umbral y entonces
el sistema se mantiene honestamente en U.

### 5.4. Sentido práctico del umbral

El umbral no es una superstición numérica. Funciona como una barrera de
prudencia. Exige que exista una mayoría cualificada de evidencia apta o
no apta antes de emitir una sentencia fuerte. Cuando esa mayoría no
existe, el sistema no finge saber: conserva la indeterminación.

------------------------------------------------------------------------

## 6. Estatuto epistémico de U

### 6.1. Definición

El símbolo U no representa un valor numérico ni un infinito algebraico.
En el Sistema Vectorial SV, U representa un estado epistémico de no
determinación actual.

Dicho de otra manera: cuando un parámetro vale U, el sistema está
diciendo con honestidad que, en el estado presente de la evidencia, no
sabe si debe sentenciar 0 o 1.

### 6.2. Resolución posterior

La intervención humana, experta o institucional puede producir una
resolución posterior de la forma

> U → 0   o   U → 1

Sin embargo, esa resolución no debe confundirse con una verdad
ontológica garantizada. Expresa una sentencia operativa vigente en ese
momento y bajo esas condiciones de evidencia. El sistema, por tanto,
distingue entre dos planos que conviene no mezclar:

-   el plano de la realidad efectiva de las cosas;

-   el plano de la decisión formal adoptada por el sistema o por el
    experto.

### 6.3. Función de resolución

Para modelar esta transición de forma abstracta puede definirse una
función de resolución

> Res : {U} × 𝒳 × ℛ → {0, 1, U}

donde 𝒳 representa el contexto de evidencia y ℛ el mecanismo de revisión
o resolución.

**Explicación en prosa.** La función anterior no pretende imponer una
técnica concreta. Solo deja constancia de una idea decisiva: la
indeterminación puede persistir, puede resolverse a favor de la aptitud
o puede resolverse a favor de la no aptitud. El sistema no confunde
nunca la suspensión de juicio con una ausencia de interés; la conserva
como un dato formal del proceso.

### 6.4. Consecuencia para la trazabilidad

La existencia de U obliga a una trazabilidad rica. El sistema debe poder
distinguir entre:

-   la salida automática inicial de la célula;

-   la intervención posterior del experto;

-   la salida recalculada después de dicha intervención.

Este desdoblamiento no es un lujo documental. Es la condición mínima
para que la revisión humana añada conocimiento sin borrar la historia
del proceso.\[R1\]\[R2\]

------------------------------------------------------------------------

## 7. Álgebra semántica y composición tipada

### 7.1. De la célula simple a la arquitectura multicelular

Hasta aquí el lector ha visto la célula como unidad aislada. Sin
embargo, una arquitectura real rara vez se agota en una sola célula.
Aparecen dependencias funcionales, relaciones jerárquicas, supervisiones
de segundo orden y ensamblajes compuestos. El paso de una célula a
varias exige una teoría explícita de la composición.

### 7.2. Definición canónica de célula composable

Una célula composable del Sistema Vectorial SV se representa por el
cuádruple

> Cᵢ = (vᵢ, Ωᵢ, κᵢ, ρᵢ)

Sus componentes tienen el siguiente significado:

-   **vᵢ:** vector ternario interno.

-   **Ωᵢ:** codominio de salida de la célula.

-   **κᵢ:** interpretación semántica del codominio.

-   **ρᵢ:** rol estructural de la célula en la arquitectura.

La distinción entre Ωᵢ, κᵢ y ρᵢ es decisiva. Dos células pueden
compartir el mismo codominio y, sin embargo, no ser composables por la
misma ley si difieren en su rol o en la semántica de su salida.

### 7.3. Función de evaluación

Toda célula posee una función de evaluación

> χᵢ : Cᵢ → Ωᵢ

En la formulación vigente, χᵢ factoriza a través del vector interno y
del motor normativo asociado. Esto significa que el valor de salida de
una célula depende de la información contenida en su vector y del
criterio normativo que la gobierna, pero no del nombre retórico que se
le quiera dar a posteriori.

### 7.4. Tesis central

No existe una única operación binaria universal válida para cualquier
par de células del Sistema Vectorial SV.

Esta tesis corrige una tentación natural, pero equivocada: pensar que la
composición entre células puede reducirse siempre a max() o a cualquier
otra regla fija. La composición correcta depende de la relación tipada
entre las células.

### 7.5. Familia básica de operadores

La familia básica actualmente justificada es

> ℱ_SV = { max, min, ⊗\_gate, σ\_{k,φ}, ▷, Γ, Comp }

La familia se organiza en tres niveles:

-   **Nivel 1 --- Operadores de composición relacional: max, min,
    ⊗\_gate, σ\_{k,φ}, ▷.**.

-   **Nivel 2 --- Operador de enriquecimiento:** Γ.

-   **Nivel 3 --- Constructor de sistema:** Comp.

### 7.6. Dominancia homogénea

La operación de dominancia homogénea tiene la forma

> χ_out = max( χᵢ(Cᵢ), χⱼ(Cⱼ) )

o, dualmente, min, cuando el orden del codominio está documentado.

Para que esta operación sea lícita deben cumplirse, al menos, las
siguientes condiciones:

-   compatibilidad de codominio;

-   compatibilidad ordinal entre las interpretaciones semánticas;

-   compatibilidad estructural de los roles de las células;

-   existencia de un orden explícito sobre el codominio compuesto.

**Explicación en prosa.** max() no queda abolido. Queda puesto en su
sitio. Es un operador válido cuando ambas células miden gravedad o
integridad sobre una misma escala ordinal, o sobre escalas isomorfas. En
ese contexto es limpio, útil y, en ciertos casos históricos,
perfectamente justificable.\[R3\]

### 7.7. Compuerta jerárquica

La compuerta jerárquica responde a la forma

> χ_out = χᵢ(Cᵢ) ⊗\_gate χⱼ(Cⱼ)

Aquí no se elige la salida "más alta", sino que una célula actúa como
condición de paso o de bloqueo de la otra. Su lógica es asimétrica: el
orden de los argumentos importa.

**Explicación en prosa.** Este operador es apropiado cuando una célula
no compite con la otra, sino que la habilita, la invalida o la
condiciona. En arquitectura de sistemas reales, este caso es más
frecuente de lo que parece.

### 7.8. Composición en serie

La composición en serie se expresa mediante un operador de sustitución

> σ\_{k,φ}( Cⱼ, χᵢ(Cᵢ) )

Su función consiste en transportar la salida de una célula hacia un
parámetro puente de otra mediante un conector φ.

**Explicación en prosa.** La lógica aquí no es comparar dos sentencias,
sino hacer que la salida de una célula alimente a otra. Es la forma
natural de modelar dependencias funcionales entre módulos sin deformar
la gramática ternaria.

### 7.9. Supervisión meta o veto

La supervisión meta se representa por

> M ▷ C_base

En este caso, una célula de segundo orden supervisa a una célula base.
El operador no equivale a una dominancia ordinaria: expresa control,
validación o veto de integridad.

### 7.10. Criticidad de la indeterminación

El operador Γ no genera una clasificación nueva. Enriquece el análisis
de la indeterminación. Si se define

> Nᵤ(**v**) = número de componentes con valor U

y δ₀(**v**) y δ₁(**v**) como las distancias mínimas, en número de
cambios elementales, para que la célula alcance respectivamente el
umbral de aptitud o de no aptitud, puede plantearse la forma

> Γ(v) = Nᵤ(v) − min( δ₀(v), δ₁(v) )

**Explicación en prosa.** Esta magnitud no sustituye a la clasificación
principal. Sirve para distinguir entre indeterminaciones cercanas a una
resolución y otras profundamente abiertas. Dicho de modo menos técnico:
no todas las U son iguales.

### 7.11. Compilación multicelular

El constructor Comp no es una operación binaria ordinaria. Se representa
como

> Comp( C₁, C₂, ..., Cₘ )

Su misión es organizar una arquitectura multicapa, con operadores de
distinto tipo en su interior. No compone únicamente salidas: compone
estructura.

### 7.12. Principio de cierre

La conclusión que debe retenerse es ésta: el Sistema Vectorial SV no
tiene una ley universal única de composición, sino una gramática de
composición tipada. La investigación futura podrá ampliar esta
gramática; lo que no debería hacer, salvo prueba formal superior, es
volver a reducirla indebidamente a una sola ley ciega para todos los
casos.

------------------------------------------------------------------------

## 8. Sistema Vectorial SV e inteligencia artificial

### 8.1. Tesis de subordinación

El Sistema Vectorial SV no necesita inteligencia artificial para
clasificar un vector una vez que éste ha sido parametrizado. El conteo
de 0, 1 y U, junto con la aplicación del umbral, es una operación
discreta elemental.\[R1\]\[R2\]

De ello se sigue una tesis mayor: la inteligencia artificial, dentro del
marco SV, no funda el sistema, no lo gobierna y no desplaza al ser
humano del centro decisional. Su lugar correcto es el de una capa
auxiliar subordinada a una gramática previa, explícita, auditable y
algebraicamente estable. El ser humano puede y debe entender, decidir y
gobernar la IA; el sistema no admite la inversión de esa
jerarquía.\[R2\]

**Principio de primacía humana e inteligibilidad.** En el Sistema
Vectorial SV, la imagen poligonal cerrada no constituye un ornamento ni
una mera salida para consumo algorítmico: constituye una interfaz de
inteligibilidad destinada a que el ser humano pueda comprender, auditar
y, en último término, gobernar la decisión. La inteligencia artificial
puede asistir, acelerar, contrastar o vigilar; no puede sustituir la
gramática del sistema, ni desplazar la soberanía interpretativa del ser
humano sobre la representación común.

### 8.2. Tres papeles legítimos de la IA

En el estado actual del marco, la IA tiene papeles legítimos, pero todos
ellos derivan de esa subordinación estructural y ninguno le confiere
soberanía sobre el sistema:

-   **Análisis geométrico auxiliar.** Puede aprender patrones espaciales
    del polígono visible que no se agotan en el simple conteo de
    clases.\[R1\]\[R2\]

-   **Transferencia al dominio visual real.** Puede enlazar la célula SV
    con imágenes, señales o fuentes perceptivas externas cuya extracción
    de rasgos sería inviable a mano.\[R1\]\[R2\]

-   **Contraste y vigilancia.** Puede actuar como revisor auxiliar de
    coherencia frente a grandes historiales de resoluciones previas, sin
    sustituir la decisión humana, el motor normativo ni la sentencia
    final documentada.\[R1\]\[R2\]

### 8.3. Agnosticismo del backbone

El marco SV debe permanecer agnóstico al backbone. Cambiar una CNN por
otra arquitectura, o incluso reemplazarla por un modelo no
convolucional, no altera la gramática del sistema mientras permanezcan
intactos el alfabeto ternario, la evaluación normativa, la
representación trazable y el principio de revisión.\[R2\]

### 8.4. Principios de diseño compatibles con un uso responsable de IA

Este planteamiento es coherente con la literatura contemporánea sobre IA
fiable y gobernanza del riesgo. El NIST AI RMF insiste en que la gestión
de riesgos de IA debe incorporar criterios de trazabilidad, gobernanza,
explicación y supervisión humana, y el Reglamento (UE) 2024/1689 insiste
en la necesidad de diseñar sistemas de IA seguros y dignos de
confianza.\[R6\]\[R7\]

**Explicación en prosa.** En el marco SV esa supervisión no es una
cláusula decorativa. La imagen poligonal existe precisamente para que el
ser humano pueda inspeccionar la misma forma sobre la que opera la
máquina y para que ninguna capa estadística sustituya la sentencia
ternaria primaria por un cocinado opaco del dato. El Sistema Vectorial
SV no reconoce como fundamento interno de decisión las salidas
probabilísticas, estadísticas o mineras que no estén ancladas en
parámetros explícitos, semánticamente definidos y auditables. Allí donde
el sistema no pueda afirmar 0 o 1 con base suficiente, su deber formal
es declarar U. Fingir certeza donde no la hay corrompería el marco;
ocultar la incertidumbre bajo un cálculo opaco lo degradaría. Por eso la
salida primaria sólo puede ser Apto, No_Apto o Indeterminado, y toda
ampliación algorítmica debe respetar esa gramática en lugar de
absorberla.\[R2\]

------------------------------------------------------------------------

## 9. Invariantes constitutivos y compatibilidad futura

Un marco fundacional solo merece ese nombre si también deja claros sus
invariantes constitutivos. En el estado actual del desarrollo, los
siguientes principios deben entenderse como constitutivos del Sistema
Vectorial SV:

-   Alfabeto ternario canónico: 0, 1, U.

-   Semántica primaria: 0 = Apto, 1 = No_Apto, U = Indeterminado.

-   Restricción arquitectónica de la célula simple: n = b², con b ≥ 3.

-   Distinción entre plano exacto y plano visible.

-   Representación polar cerrada con orden estable de parámetros.

-   Motor normativo determinista documentado.

-   Separación entre evaluación de la célula y composición entre
    células.

-   Álgebra semántica tipada para arquitecturas multicelulares.

-   Primacía decisional humana y subordinación de la IA al marco, no del
    marco ni del ser humano a la IA.

-   La imagen poligonal cerrada como interfaz de inteligibilidad
    compartida entre ser humano y máquina.

-   Exclusión de la probabilidad opaca como fundamento primario de
    decisión.

-   Trazabilidad de la intervención humana sobre la indeterminación.

Estos invariantes no deben leerse como un cierre caprichoso de la
investigación. Al contrario: son el conjunto mínimo de piezas sin las
cuales el Sistema Vectorial SV dejaría de ser reconocible como tal. Un
desarrollo futuro puede ampliar, generalizar o incluso reemplazar alguno
de estos puntos, pero para hacerlo con legitimidad formal deberá
declarar con precisión qué sustituye, por qué lo sustituye y qué gana o
pierde el sistema con esa sustitución.

------------------------------------------------------------------------

## 10. Revisión adversarial interna de cierre

La mejor manera de cerrar un marco no consiste en repetir que es sólido,
sino en intentar romperlo desde dentro. Esta sección resume la revisión
adversarial mínima que este texto ha debido resistir.

### 10.1. Objeción algebraica

**Objeción:** si la célula se escribe con símbolos discretos y se usan
fórmulas, ¿por qué no llamarla directamente espacio vectorial y
simplificar?

**Respuesta:** porque eso confundiría el objeto con una posible
codificación auxiliar. El sistema gana claridad y honestidad si reconoce
que su plano exacto es semántico-discreto, y que la algebraización
estricta es una capa derivada, no originaria.

### 10.2. Objeción representacional

**Objeción:** si la figura se dibuja en el plano, ¿por qué no tratarla
siempre como una función y = f(x)?

**Respuesta:** porque una poligonal cerrada no es, en general, el
gráfico de una función global. Forzarla a serlo introduciría una
simplificación falsa. El tratamiento correcto es local o paramétrico, no
una identificación impropia.

### 10.3. Objeción sobre U

**Objeción:** si U termina resolviéndose por un experto, ¿no bastaría
con eliminarlo y decidir siempre 0 o 1?

**Respuesta:** no. Eliminar U supondría obligar al sistema a fingir
conocimiento donde no lo tiene. La presencia de U no es un defecto del
marco; es su forma de honestidad operativa.

### 10.4. Objeción composicional

**Objeción:** si max() ya funcionó en ejemplos previos, ¿por qué no
elevarlo a ley universal?

**Respuesta:** porque una arquitectura real contiene relaciones de
naturaleza distinta: dependencia, supervisión, compuerta, serie,
ensamblaje multicapa. Reducir todas esas relaciones a una sola operación
haría el sistema más simple, sí, pero también matemáticamente menos
verdadero.

### 10.5. Objeción desde la IA

**Objeción:** si la IA aprende patrones útiles, ¿por qué no dejar que
ella misma determine el marco?

**Respuesta:** porque la IA puede aportar capacidad de generalización,
detección y contraste, pero no debe sustituir la gramática de decisión
ni la trazabilidad. Un sistema que delega su ontología en el modelo de
moda pierde estabilidad científica y auditabilidad institucional. En
este marco, el ser humano no queda relegado a una supervisión
decorativa: debe poder comprender la figura, gobernar el proceso y
sentenciar sobre la misma representación sobre la que opera la máquina.
La IA asiste; no manda. Y cuando la evidencia no alcance para afirmar 0
o 1, el sistema no se refugia en una probabilidad opaca: declara U.

------------------------------------------------------------------------

## 11. Conclusión

El Sistema Vectorial SV queda, tras este desarrollo, fijado en una
formulación que aspira a ser estable en el tiempo por una razón muy
concreta: no descansa en una moda tecnológica ni en una preferencia de
implementación, sino en una articulación sobria entre semántica
ternaria, representación geométrica, clasificación determinista y
composición tipada.

Su célula exacta es un producto cartesiano finito de estados; su figura
visible es una poligonal polar cerrada; su símbolo U expresa una
honestidad epistémica irrenunciable; su composición multicelular exige
una gramática más rica que una sola ley; y su relación con la IA es la
de un marco rector respecto de una capa auxiliar, nunca la de un sistema
sometido a una ontología estadística opaca o a un cocinado no trazable
del dato. La imagen existe para que el ser humano pueda decidir sobre la
misma figura que la máquina analiza. Ésa es la jerarquía del sistema, y
alterarla supondría salir del Sistema Vectorial SV.

Ésta es, en último término, la razón por la que el marco merece ser
tratado con disciplina. No porque prohíba caprichosamente, sino porque
distingue con cuidado entre lo que pertenece a la esencia del sistema y
lo que pertenece a sus desarrollos contingentes. El valor de un
documento fundacional no está en inmovilizar la investigación, sino en
impedir que el futuro avance a costa de olvidar aquello que le dio
coherencia.

------------------------------------------------------------------------

## Referencias

\[R1\] Juan Antonio Lloret Egea. *La guía práctica del conocimiento
profundo y la crítica de la razón pura.* ITVIA, Release 1, 07/03/2026.
https://www.itvia.online/pub/la-guia-practica-del-conocimiento-profundoy-la-critica-de-la-razon-pura/release/1

\[R2\] Juan Antonio Lloret Egea. *Compilador doctrinal y célula meta
SV(9,3)-IA.* ITVIA, Release 2, 2026.
https://www.itvia.online/pub/compilador-doctrinal-y-celula-meta-sv93-ia/release/2

\[R3\] Juan Antonio Lloret Egea. *De SVcustos, el marco de intrusión,
hasta SVperitus: Células SV en Par: n = 36 + 9 = 45.* ITVIA, Release 1,
2026.
<https://www.itvia.online/pub/de-svcustos-el-marco-de-intrusion-hasta-svperitus-celulas-sv-en-par-n--36--9--45/release/1>

\[R4\] OpenStax. *Calculus Volume 2,* sección 7.1, "Parametric
Equations". Rice University, 2016.
https://openstax.org/books/calculus-volume-2/pages/7-1-parametric-equations

\[R5\] OpenStax. *Calculus Volume 3,* sección 1.3, "Polar Coordinates".
Rice University, 2016.
https://openstax.org/books/calculus-volume-3/pages/1-3-polar-coordinates

\[R6\] National Institute of Standards and Technology (NIST).
*Artificial Intelligence Risk Management Framework (AI RMF 1.0).* NIST
AI 100-1, 2023. https://nvlpubs.nist.gov/nistpubs/ai/nist.ai.100-1.pdf

\[R7\] Unión Europea. *Reglamento (UE) 2024/1689 del Parlamento Europeo
y del Consejo, de 13 de junio de 2024, por el que se establecen normas
armonizadas en materia de inteligencia artificial.* Diario Oficial de la
Unión Europea. https://eur-lex.europa.eu/eli/reg/2024/1689/oj/eng\
\
#SistemaVectorial #SistemaVectorialSV #CélulaSV #LógicaTernaria
#PolígonoPolar #ComposiciónTipada #IndeterminaciónEpistémica
#InteligenciaArtificialAuditable #ÁlgebraSemántica #MarcosDecisionales
#IASubordinada #GramáticaTernaria #MotorNormativo #RepresentaciónPolar
#PoligonalCerrada #ProductoCartesiano #ClasificaciónDeterminista
#TrazabilidadDecisional #GobernanzaIA #SupervisiónHumana
#AgenteEspecializado #SVperitus #EvaluaciónExperta #SistemasAuditables
#HonestidadEpistémica #ComposiciónMulticelular #OperadoresTipados
#CriticidadGamma #FunciónDeCriticidad #DiseñoResponsableIA #IAExplicable
#NISTAIRM #RegulacionIA #ReglamentoIA2024 #DecisiónTrazable
#MatemáticaDiscreta #GeometríaPolar #CuadradoPerfecto #UmbralCanónico
#MayoríaCualificada #CombinatoriaFinita #EstadosEpistémicos
#FuncionesDeEvaluación #MarcoFundacional #InvariantesConstitutivos
#DualidadExactaAuxiliar #AgnosticismoDeBackbone #OpenAccess #CCBYNCND
