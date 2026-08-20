# Acta técnica de cierre de FFL-E — interfaz semántico-diagnóstica

**Fecha:** 21/08/2026  
**Estado:** cerrado  
**Ámbito:** Lenguaje SV — FFL-E

## 1. Objeto del cierre

FFL-E se abrió para identificar y publicar el contrato mínimo que debe preservar una futura infraestructura de ejecución cuando operaciones, consultas o interfaces utilicen representaciones que pueden haber perdido información del estado de origen.

El bloque puede cerrarse cuando el contrato distinga de forma inequívoca valor ternario, salida terminal tipada, representación, recuperabilidad por operación, contenido transmitido por interfaces y condición diagnóstica de representación insuficiente.

## 2. Resultado alcanzado

Quedan fijados los siguientes documentos técnicos:

- `ACTA_TECNICA_DE_APERTURA_DE_FFL_E_INTERFAZ_SEMANTICO_DIAGNOSTICA_2026_08_21.md`;
- `CONTRATO_MINIMO_DE_SUFIENCIA_REPRESENTACIONAL_POR_OPERACION_PARA_EL_LENGUAJE_SV_2026_08_21.md`;
- `ADENDA_TECNICA_SOBRE_SALIDA_TERMINAL_TIPADA_Y_EVALUACION_DE_CELULA_2026_08_21.md`;
- `MATRIZ_DE_IMPACTO_DE_LA_SUFIENCIA_REPRESENTACIONAL_EN_LA_ESPECIFICACION_Y_LA_IMPLEMENTACION_2026_08_21.md`.

## 3. Decisiones semánticas fijadas

FFL-E establece:

1. `Tri = {Zero, One, U}` permanece inalterado.
2. La insuficiencia de una representación para una operación no es `U`.
3. La salida terminal de una célula pertenece a su codominio tipado `K_i`; no pertenece por defecto a `Tri`.
4. Una transducción `K_i -> Tri` es una operación separada y explícita.
5. Una representación se declara como aplicación `F_j : X_D -> R_j` sobre el espacio de estados realizables del dominio.
6. Las reducciones `r_j : R_j -> R_(j+1)` pertenecen a una cadena de representaciones y deben satisfacer `F_(j+1)=r_j∘F_j`.
7. Una representación con pérdida puede ser suficiente para una operación e insuficiente para otra.
8. La recuperabilidad exacta de `Q` desde `F_j` exige una aplicación `q_j` con `Q=q_j∘F_j`.
9. La pérdida en el nivel siguiente puede acreditarse mediante dos estados realizables con igual representación y distinta salida de `Q`.
10. Los niveles admisibles de una operación en una cadena certificada forman el segmento inicial determinado por el índice acreditado; no son una lista libre.
11. Una interfaz sólo puede atribuirse la información que transmite. Si existe información adicional, debe declararse como dependencia adicional.
12. La identidad de parámetros y las agrupaciones semánticas necesarias para una representación deben declararse; no se deducen únicamente de `n=b²`.
13. El caso de una partición en capas no se convierte en estructura universal obligatoria para todos los dominios.

## 4. Saneamiento de la salida terminal

La revisión de FFL-E detectó que la IR v0.2 combina un `CellSpec.codomain` genérico con un `EvalResult.classification` especializado en tres etiquetas de una familia de dominio.

La adenda de salida terminal fija el criterio para la revisión posterior:

- el resultado estructural del umbral se mantiene separado de la etiqueta terminal;
- la etiqueta terminal debe pertenecer al codominio de la célula;
- `OutputSemantics` documenta significados, pero no sustituye una interpretación terminal ejecutable;
- los conectores continúan operando desde el codominio tipado hacia `Tri`.

La implementación actual no ejecuta materialmente `evaluate`; por ello este saneamiento es una obligación de la próxima especificación, no una corrección de un resultado de ejecución ya emitido.

## 5. Diagnóstico reservado

Se identifica la condición semántica `RepresentationInsufficientForOperation` para el caso en que la representación disponible no conserva las distinciones necesarias para una operación.

No se le asigna código numérico porque todavía no existe un punto de emisión implementado. Tampoco se identifica con `UndeclaredLossyEncoding`, con `StrongConclusionUnderInsufficientCoverage` ni con `U`.

## 6. Alcance que queda para la siguiente fase

El cierre de FFL-E no declara implementados los objetos identificados. Queda para una fase versionada posterior:

- definir de forma material `ParameterInstance` y la estructura de agrupación de dominio que corresponda;
- incorporar la interpretación terminal tipada a la especificación de evaluación;
- definir `RepresentationSpec`, `RepresentationChain`, `RepresentationRequirement` y `RepresentationFrontierCertificate` en la IR;
- decidir la superficie sintáctica mínima necesaria;
- actualizar AST, descenso a IR y validador;
- fijar diagnósticos observables;
- crear pruebas de conformidad;
- y abordar ejecución sólo cuando la semántica de cada operación esté cerrada.

Estas tareas son consecuencia del contrato ya fijado y no constituyen deuda de FFL-E.

## 7. Exclusiones preservadas

El cierre no autoriza:

- ampliar `Tri`;
- reactivar la línea Beta;
- inferir estratificación clínica a partir del tamaño de la célula;
- introducir reglas materiales de dominios especializados en el núcleo del lenguaje;
- modificar el significado de `resolve` o `ResolutionRecord`;
- asignar códigos diagnósticos sin ruta observable;
- declarar ejecución de consultas, evaluación o compuertas que la implementación actual no realiza;
- abrir FFL-D.

## 8. Cierre

El criterio de FFL-E —identificación y publicación del contrato mínimo semántico-diagnóstico preservable hacia una infraestructura futura— queda satisfecho.

FFL-E se declara cerrado el 21/08/2026.

FFL-A, FFL-B, FFL-C y FFL-E quedan cerrados. FFL-D permanece pendiente y requerirá decisión expresa para su apertura.
