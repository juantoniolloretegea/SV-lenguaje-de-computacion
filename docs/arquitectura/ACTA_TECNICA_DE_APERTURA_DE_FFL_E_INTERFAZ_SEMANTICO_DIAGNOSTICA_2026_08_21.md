# Acta técnica de apertura de FFL-E — interfaz semántico-diagnóstica

**Fecha:** 21/08/2026  
**Estado:** abierto  
**Ámbito:** Lenguaje SV — FFL-E

## 1. Objeto

FFL-E se abre para fijar el contrato semántico-diagnóstico mínimo que debe regir la relación entre representaciones agregadas, operaciones, consultas e interfaces del Lenguaje SV.

El objeto no es ampliar el alfabeto ternario ni habilitar una infraestructura de ejecución. El objeto es impedir que una operación se considere legítimamente ejecutable cuando la representación disponible ya ha eliminado alguna distinción necesaria para obtener su resultado exacto.

## 2. Base formal

Sea `X` un espacio de estados realizables de un dominio declarado y sea

`F_0, F_1, ..., F_m`

una cadena finita de representaciones de `X` tal que, para cada `j < m`, existe una reducción determinista `r_j` con

`F_(j+1) = r_j ∘ F_j`.

Para una operación `Q : X -> Y`, se dice que `Q` es exactamente recuperable desde el nivel `F_j` cuando existe una aplicación `q_j` tal que

`Q = q_j ∘ F_j`.

Si existen dos estados realizables `x, y in X` con

`F_(j+1)(x) = F_(j+1)(y)`

pero

`Q(x) != Q(y)`,

la representación `F_(j+1)` no conserva información suficiente para recuperar exactamente `Q`.

Esta condición es relativa a la operación, al espacio de estados realizables y a la cadena declarada. No establece una jerarquía universal de representaciones.

## 3. Invariantes semánticos preservados

La apertura de FFL-E fija desde el inicio las siguientes restricciones:

1. `Tri = {Zero, One, U}` permanece inalterado.
2. El codominio terminal de una evaluación de célula no se identifica por defecto con `Tri`; conserva su tipo declarado.
3. La insuficiencia de una representación para una operación no es un valor `U`, no crea un subtipo de `U` y no autoriza una ampliación del alfabeto ternario.
4. La identidad de los parámetros y su pertenencia a una capa o estrato pertenecen a la estructura declarada del dominio. No se deducen únicamente de `n = b²`.
5. Una interfaz sólo puede atribuirse las distinciones que realmente transmite. Si transmite `H = phi ∘ F_j`, una recuperación exacta aguas abajo debe factorizar por `H`, salvo que se declare información adicional `S`; en ese caso la afirmación correcta adopta la forma `Q = q(H, S)`.
6. Una distinción eliminada por agregación no puede reconstruirse como resultado fuerte mediante inferencia no declarada.
7. Los testigos utilizados para negar recuperabilidad deben pertenecer al espacio de estados realizables del dominio; la mera pertenencia sintáctica a `Tri^n` no basta cuando el dominio impone restricciones de constitución.

## 4. Consecuencia para la arquitectura vigente

La IR v0.2 y la gramática v0.1 ya separan estado ternario, codominio de salida, dominio, agente, consulta e interfaz. Esa separación es compatible con el contrato anterior.

Sin embargo, la implementación vigente no materializa todavía como objetos completos del lenguaje:

- la identidad semántica de cada parámetro junto con su pertenencia declarada a un estrato;
- una cadena de representaciones declarada y tipada;
- el requisito de representación de una operación o consulta;
- un certificado de recuperabilidad exacta;
- un testigo de pérdida de recuperabilidad sobre estados realizables;
- una condición diagnóstica específica para representación insuficiente.

Estas ausencias constituyen alcance pendiente de FFL-E. No son contradicciones del núcleo ternario vigente.

## 5. Alcance de FFL-E

FFL-E deberá determinar, como mínimo:

1. la representación mínima de identidad de parámetro y estratificación de dominio;
2. la forma tipada de una representación y de una cadena finita de representaciones;
3. la vinculación entre una operación o consulta y la representación desde la que puede ejecutarse exactamente;
4. la forma de acreditar recuperabilidad y pérdida de recuperabilidad sin confundirlas con clausura ternaria;
5. el contrato de transmisión de información de una interfaz;
6. la condición diagnóstica observable cuando una operación exige distinciones que la representación recibida no conserva;
7. la ubicación de estos objetos en la IR y su relación con `Domain`, `Agent`, `QuerySpec`, `QueryContext`, `QueryResult` y las vistas analíticas.

## 6. Exclusiones

La apertura de FFL-E no autoriza por sí sola:

- cambios en `Tri`;
- reapertura de la línea Beta;
- incorporación automática de nuevas palabras reservadas a la gramática v0.1;
- modificación inmediata de AST, IR, validador o catálogo diagnóstico;
- ejecución de consultas o de compuertas no implementadas;
- infraestructura de ejecución nueva;
- incorporación de reglas materiales de inmunología, neumología u otros dominios especializados al núcleo del lenguaje;
- apertura de FFL-D.

## 7. Criterio de cierre

FFL-E podrá cerrarse cuando quede publicado un contrato mínimo que determine de forma inequívoca:

- qué información debe declarar una representación;
- cómo se relaciona una representación con una operación;
- qué constituye una afirmación de recuperabilidad exacta;
- cómo se acredita su pérdida;
- cómo se expresa una insuficiencia representacional sin producir `U`;
- qué información compromete una interfaz;
- y qué modificaciones posteriores de especificación o implementación quedan justificadas, diferenciándolas de las que siguen siendo innecesarias.

## 8. Estado

FFL-E queda abierto.

FFL-A, FFL-B y FFL-C permanecen cerrados. FFL-D continúa pendiente. La apertura documentada aquí no modifica todavía la gramática v0.1, la IR v0.2, la implementación de referencia ni las baterías de prueba.
