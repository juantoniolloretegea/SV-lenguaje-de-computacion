# Adenda normativa del perfil léxico de la Gramática SVP 0.2

**Fecha:** 27 de agosto de 2026  
**Estado:** especificación técnica pública  
**Ámbito:** superficie léxica de archivos `.svp`

## 1. Objeto

La Gramática superficial mínima v0.1, heredada por la v0.2 salvo sustitución expresa, utiliza las primitivas `letter`, `digit` y `character` sin cerrar el dominio de `letter` y `digit`. Esta adenda completa esa frontera para eliminar dependencias accidentales de las bibliotecas Unicode de las realizaciones y asegurar una aceptación reproducible del mismo texto `.svp`.

No modifica `Tri`, la IR 0.3, la semántica de los operadores ni las palabras reservadas de la Gramática 0.2.

## 2. Codificación de la fuente

Un archivo `.svp` se interpreta como texto UTF-8 válido. La huella `source_sha256` se calcula sobre los bytes UTF-8 originales. No se aplica normalización, plegado de mayúsculas/minúsculas ni transliteración antes de calcular la huella o analizar la fuente.

## 3. Letras, dígitos e identificadores

El conjunto de letras admitidas en identificadores es cerrado:

```text
A-Z

a-z

Á É Í Ó Ú Ü Ñ

á é í ó ú ü ñ
```

Los dígitos admitidos son exclusivamente `0`–`9` del repertorio ASCII.

La producción queda completada como:

```ebnf
letter      ::= ascii_letter | spanish_letter ;
ascii_letter ::= "A" | ... | "Z" | "a" | ... | "z" ;
spanish_letter ::= "Á" | "É" | "Í" | "Ó" | "Ú" | "Ü" | "Ñ"
                 | "á" | "é" | "í" | "ó" | "ú" | "ü" | "ñ" ;
digit       ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
identifier  ::= letter { letter | digit | "_" } ;
nat         ::= digit { digit } ;
```

El guion bajo no puede iniciar un identificador. Las palabras reservadas de la Gramática 0.2 tampoco constituyen identificadores y no pueden reutilizarse como nombres.

## 4. Identidad y formas no admitidas

La identidad nominal es sensible a mayúsculas/minúsculas y a la secuencia exacta de puntos de código admitidos. Las formas con marcas combinantes no pertenecen al perfil de identificadores. Por ejemplo, una `ñ` precompuesta es admitida; `n` seguida de la marca combinante U+0303 (`COMBINING TILDE`) no constituye una grafía alternativa del mismo identificador y se rechaza léxicamente.

La restricción evita que dos realizaciones dependan de algoritmos o versiones diferentes de normalización Unicode. También quedan fuera del perfil las letras de otros alfabetos y las letras latinas no enumeradas. Los términos que requieran otras grafías pueden conservarse como texto de dominio cuando la gramática permita una cadena, sin ampliar por ello el espacio nominal del programa.

## 5. Cadenas, comentarios y espacios

Las restricciones de `identifier` no se aplican al contenido de los literales de cadena ni a los comentarios. Las cadenas continúan admitiendo texto UTF-8 conforme a la producción vigente y no introducen una forma alternativa de identificador.

El espacio léxico ignorado fuera de cadenas queda cerrado a:

```text
SPACE       U+0020
HORIZONTAL TAB U+0009
CARRIAGE RETURN U+000D
LINE FEED   U+000A
```

Otras clases Unicode de espacio no se asimilan implícitamente a estas cuatro.

## 6. Palabras reservadas

El perfil no crea alias ni traduce palabras reservadas. Las palabras reservadas de la Gramática 0.2 conservan su estatuto y su grafía canónica. Esta adenda sólo cierra la formación de identificadores y naturales.

## 7. Conformidad de las realizaciones

Toda realización que declare conformidad con la superficie SVP 0.2 deberá aceptar y rechazar las mismas secuencias para `identifier` y `nat` con independencia de las propiedades Unicode que ofrezca su entorno de ejecución.

Como mínimo, la batería de conformidad debe cubrir:

- identificadores con `ñ`, vocales acentuadas y `ü`;
- guion bajo únicamente en continuación;
- rechazo de palabras reservadas en posición de identificador;
- rechazo de marcas combinantes;
- rechazo de alfabetos distintos del perfil;
- rechazo de letras latinas no enumeradas;
- rechazo de dígitos no ASCII como `nat`.

## 8. Alcance

Esta adenda resuelve una frontera léxica. No introduce una segunda superficie en español, no modifica las palabras reservadas, no interpreta lenguaje natural y no cambia la semántica ni la representación intermedia del Lenguaje SV.
