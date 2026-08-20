# Matriz de impacto de la suficiencia representacional en la especificación y la implementación

**Fecha:** 21/08/2026  
**Estado:** evaluación técnica de FFL-E

## 1. Objeto

Esta matriz identifica qué objetos del Lenguaje SV quedan afectados por el contrato de suficiencia representacional por operación y por la separación entre salida terminal tipada y alfabeto ternario.

La matriz distingue modificación necesaria, preservación sin cambios y decisión que debe quedar diferida hasta una versión posterior de la especificación.

## 2. Matriz

| Objeto | Estado vigente | Obligación derivada | Tratamiento |
|---|---|---|---|
| `Tri` | `{Zero, One, U}` | La insuficiencia de una representación no puede convertirse en un valor ternario | **Sin cambios** |
| `Codomain` | Conjunto finito y explícito | Debe seguir tipando la salida propia de cada célula | **Sin cambios estructurales** |
| `OutputSemantics` | Correspondencia entre valores del codominio y descripciones | No basta para determinar qué valor expresa cada resultado estructural del umbral | **Se conserva; no se reutiliza como regla ejecutiva implícita** |
| `CellSpec` | Declara `b`, `codomain`, `semantics`, `role` | Una célula que use el evaluador de umbral deberá quedar vinculada a una interpretación terminal tipada | **Modificación de especificación posterior** |
| `EvalResult` | `classification` está especializado en `APTO/NO_APTO/INDETERMINADO` | Debe separar resultado estructural del umbral y salida perteneciente al codominio de origen | **Modificación necesaria en la próxima IR** |
| `Connector` | Aplicación desde el codominio transmisor a `Tri` | Debe continuar recibiendo una salida tipada y realizar una transducción explícita | **Sin cambio de principio ni de firma** |
| `ParameterInstance` | Referenciado por la IR v0.2, no materializado en la superficie implementada | La identidad y posición de parámetros deben poder preservarse cuando una operación dependa de ellas | **Definición pendiente en la próxima especificación** |
| Estratificación de dominio | No existe como objeto material de la implementación | Las agrupaciones semánticas necesarias para una representación deben declararse; no se deducen de `n=b²` | **Nuevo objeto previsto; no universalizar una partición concreta** |
| `RepresentationSpec` | No existe | Debe declarar una aplicación `F_j : X_D -> R_j` y si es inyectiva o con pérdida sobre `X_D` | **Nuevo objeto previsto** |
| `RepresentationChain` | No existe | Debe declarar niveles y reducciones `r_j` con `F_(j+1)=r_j∘F_j` | **Nuevo objeto previsto** |
| `RepresentationFrontierCertificate` | No existe | Debe vincular operación, cadena, recuperación y testigo realizable cuando proceda | **Nuevo objeto previsto** |
| `RepresentationRequirement` | No existe | Debe vincular una operación o consulta con el certificado que determina los niveles admisibles | **Nuevo objeto previsto** |
| `AnalyticView.encoding` | Distingue `Injective` y `Lossy` | La pérdida global y la suficiencia para una operación concreta deben seguir siendo propiedades distintas | **Se conserva; posible enlace posterior con certificados** |
| `QuerySpec` | Declara tipo, alcance y restricciones | Una consulta que dependa de una representación deberá poder declarar su requisito de representación | **Ampliación posterior** |
| `QueryContext` | Cinco variantes tipadas | Debe poder acreditarse qué representación e información adicional recibe realmente una consulta | **Forma superficial pendiente; no añadir variante hasta fijar el objeto de representación** |
| `QueryResult` | Respuesta, justificación y metadatos | La justificación podrá registrar el certificado de representación utilizado cuando exista ejecución | **Posible ampliación de metadatos; no necesaria en FFL-E** |
| `Agent` | Arquitectura, dominio y motor de consulta | No debe ejecutar una consulta fuerte con una representación que no satisfaga su requisito | **Regla futura de validación o ejecución; sin cambio inmediato de forma** |
| Catálogo diagnóstico | 47 códigos efectivos | Debe distinguir insuficiencia representacional de `U`, cobertura insuficiente y codificación con pérdida no declarada | **Clase semántica reservada; sin código hasta existir punto de emisión** |
| Gramática v0.1 | No expresa estos objetos | No debe ampliarse hasta fijar la siguiente IR y las reglas de bienformación | **Sin cambios en FFL-E** |
| AST / descenso a IR / validador | Implementan v0.1/v0.2 sin ejecución material | No deben anticipar una forma aún no versionada | **Sin cambios en FFL-E** |
| Batería de pruebas | 58/58 de conformidad del alcance vigente | Las pruebas nuevas deberán nacer de la especificación versionada posterior | **Sin cambios en FFL-E** |

## 3. Alcance de la estratificación

La necesidad de preservar etiquetas y agrupaciones no autoriza a imponer una única geometría de estratos a todos los dominios.

Una realización puede usar una partición ordenada de parámetros. Otra futura realización podrá requerir una estructura distinta, siempre que la representación declare de forma suficiente qué agrupación utiliza y qué información conserva.

Por tanto, la próxima especificación deberá permitir declarar la estructura necesaria para la representación sin convertir el caso de capas cuadradas en requisito universal del lenguaje.

## 4. Inyectividad relativa al dominio

La propiedad `Injective` o `Lossy` de una `RepresentationSpec` se evalúa sobre el espacio de estados realizables `X_D` declarado, no necesariamente sobre todo `Tri^n`.

Dos vectores del espacio ambiente que no puedan constituirse en el dominio no bastan para demostrar pérdida relevante dentro de `X_D`.

## 5. Salida terminal

La siguiente IR deberá resolver el desajuste entre `CellSpec.codomain` genérico y `EvalResult.classification` especializado.

Para la familia actual de evaluadores basados en `T(n)`, el modelo fijado por FFL-E separa:

1. conteos ternarios;
2. resultado estructural del umbral;
3. interpretación terminal en el codominio de la célula;
4. transducción posterior a `Tri` cuando exista un conector.

No se amplía `Tri` y no se permite identificar automáticamente una etiqueta terminal con un literal ternario.

## 6. Orden de ejecución posterior

El orden mínimo resultante es:

1. versionar la IR con los tipos y juicios necesarios;
2. definir la superficie sintáctica que corresponda;
3. actualizar AST, descenso a IR y validación estructural;
4. asignar diagnósticos sólo cuando exista una ruta observable;
5. añadir conformidad positiva y negativa;
6. abordar ejecución material únicamente para las operaciones cuya semántica esté completamente especificada.

Esta secuencia evita que la implementación se adelante a la matemática y evita también que una capacidad nueva quede reducida a documentación sin traducción posterior al lenguaje.
