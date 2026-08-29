# Especificación normativa de perfiles fuente del Lenguaje SV — SVP-ES y SVP-EN

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Fecha:** 29 de agosto de 2026  
**Estado:** Especificación técnica pública — v1

## 1. Objeto

Este documento constituye los perfiles fuente `SVP-ES` y `SVP-EN` como capa explícita de representación de la superficie del Lenguaje SV.

Los perfiles fuente permiten escribir las formas constitutivas del lenguaje en español o en inglés sin crear dos gramáticas, dos representaciones intermedias ni dos semánticas. Ambos perfiles convergen en una misma identidad canónica antes de que las producciones de la Gramática 0.2 sean interpretadas por el analizador sintáctico común.

La arquitectura normativa queda fijada como:

```text
bytes UTF-8 de la unidad fuente
        ↓
perfil léxico común de identificadores
        ↓
perfil fuente explícito SVP-ES o SVP-EN
        ↓
identidad canónica de forma constitutiva
        ↓
Gramática canónica 0.2
        ↓
IR canónica 0.3
        ↓
semántica única del Lenguaje SV
```

## 2. Separación respecto del perfil léxico

El perfil fuente no es el perfil léxico de identificadores constituido por la adenda de 27 de agosto de 2026.

El identificador interno:

```text
svp-grammar-0.2-lex-es-1
```

nombra el repertorio cerrado de caracteres admitidos en `identifier` y `nat`. Su objeto comprende, entre otros extremos, letras ASCII, las letras españolas precompuestas `ÁÉÍÓÚÜÑ` y sus minúsculas, dígitos ASCII y reglas de espacio léxico.

Ese perfil léxico:

- no traduce palabras reservadas;
- no crea alias;
- no constituye `SVP-ES`;
- no modifica la IR ni la semántica.

La afirmación de la adenda léxica según la cual dicha adenda «no introduce una segunda superficie en español» conserva por tanto su sentido propio: la adenda no la introduce. Los perfiles fuente se constituyen separadamente mediante este documento.

## 3. Perfiles fuente cerrados

Los perfiles fuente vigentes son exactamente:

```text
SVP-EN
SVP-ES
```

Su selección es explícita. No existe autodetección, inferencia por contenido, caída silenciosa de un perfil al otro ni aceptación de etiquetas alternativas.

En la interfaz binaria vigente:

```text
SVP-EN = 0
SVP-ES = 1
```

Las etiquetas textuales admitidas son exactamente:

```text
en
es
```

La función de compilación sin selector explícito conserva `SVP-EN` como perfil de compatibilidad con la superficie histórica de Gramática 0.2.

## 4. Identidades canónicas

La realización vigente contiene 154 identidades canónicas de formas constitutivas. Cada identidad posee una forma `SVP-EN` y una forma `SVP-ES`.

El catálogo cumple las siguientes condiciones:

1. cada forma admitida por un perfil corresponde a una única identidad canónica;
2. dentro de un mismo perfil no existen dos identidades distintas con la misma grafía;
3. una grafía exclusiva de un perfil no se interpreta como la identidad constitutiva correspondiente bajo el otro perfil;
4. las formas compartidas por ambos perfiles conservan una única identidad;
5. la canonicalización no depende del idioma de la interfaz gráfica.

En la realización B2 existen 11 formas compartidas y 297 grafías distintas para las 154 identidades. Las coincidencias compartidas no constituyen colisiones interperfil.

## 5. Formas protegidas y formas contextuales

Las identidades de superficie se dividen en formas protegidas y formas contextuales.

Una forma protegida no puede reutilizarse como identificador de usuario bajo el perfil en el que constituye lenguaje. Una forma contextual adquiere identidad constitutiva únicamente cuando la producción gramatical espera ese campo o literal; en posición nominal puede conservarse como identificador de usuario conforme al perfil léxico común.

Esta dualidad no autoriza traducción de identificadores y no altera la identidad nominal del programa.

La existencia de una grafía que sea palabra constitutiva en `SVP-ES` y, al mismo tiempo, identificador léxicamente posible bajo `SVP-EN` no constituye ambigüedad entre perfiles: el perfil fuente está seleccionado antes del análisis y forma parte de la frontera de la unidad fuente.

## 6. Frontera de no traducción

La canonicalización de perfiles fuente afecta exclusivamente a las formas constitutivas incluidas en el catálogo de identidades.

No se canonicalizan ni traducen:

- identificadores definidos por el usuario;
- contenido de cadenas;
- comentarios;
- datos de dominio;
- nombres de archivo;
- secuencias Unicode ajenas al catálogo.

No se aplica normalización Unicode, plegado de mayúsculas/minúsculas ni transliteración como consecuencia del perfil fuente.

La huella `source_sha256` de una unidad se calcula sobre sus bytes UTF-8 originales. La canonicalización no reescribe el archivo fuente ni sustituye esa identidad probatoria.

## 7. Gramática canónica y versionado

`grammar_version = 0.2` identifica la versión de la gramática canónica común aplicada después de resolver las formas constitutivas del perfil fuente a sus identidades canónicas.

No significa que un archivo escrito con grafías `SVP-ES` sea literalmente una secuencia de terminales ingleses del texto EBNF histórico. Significa que, una vez constituida su identidad de superficie bajo el perfil explícito, se somete a las mismas producciones, restricciones y dominios cerrados de Gramática 0.2.

Por tanto:

```text
SVP-ES ≠ nueva gramática
SVP-EN ≠ nueva gramática
SVP-ES y SVP-EN → Gramática canónica 0.2
```

La IR 0.3 permanece independiente del idioma de fuente. El perfil fuente pertenece a la procedencia de compilación de la unidad y no introduce un campo semántico adicional en la IR canónica.

## 8. Dominios cerrados heredados

La canonicalización de perfiles no amplía las enumeraciones cerradas de Gramática 0.2 heredadas de v0.1.

En particular:

```ebnf
semantic_relation_kind ::= "DeclaredRelation" ;
pattern_kind           ::= "DeclaredPattern" ;
regime_literal         ::= "Simple" | "General" ;
```

Las formas equivalentes de `SVP-ES` convergen primero en esas identidades canónicas. Cualquier valor ajeno debe rechazarse con independencia del perfil fuente.

No se admite que un identificador léxicamente válido se convierta, por ello, en un literal válido de una enumeración cerrada.

## 9. Ensamblaje multifuente

Cada unidad de un ensamblaje conserva individualmente:

- sus bytes fuente;
- su nombre de archivo;
- su perfil fuente explícito.

Cada unidad se analiza hasta su propia frontera de fin de archivo. No se concatenan textos ni secuencias léxicas entre unidades.

Las unidades convergen a la representación canónica y sólo entonces se reúnen sus objetos y operaciones para la validación global del programa ensamblado.

El perfil de una unidad no puede rescatar ni reinterpretar una grafía perteneciente al perfil de otra unidad.

## 10. Extensión futura

La existencia de `SVP-ES` y `SVP-EN` no habilita automáticamente perfiles adicionales.

Todo perfil futuro deberá constituirse expresamente y demostrar, como mínimo:

- correspondencia total con las identidades canónicas que declare cubrir;
- unicidad e inyectividad de sus grafías;
- aislamiento respecto de los demás perfiles;
- conservación de identificadores, cadenas, comentarios y datos;
- ausencia de autodetección;
- conservación de la misma Gramática, IR y semántica, o declaración normativa expresa de cualquier cambio que exceda esa condición.

## 11. Estado resultante

Con esta especificación queda establecida una sola gramática canónica y dos perfiles fuente vigentes:

```text
Gramática canónica = 0.2
IR canónica        = 0.3
Perfiles fuente    = {SVP-ES, SVP-EN}
Perfil léxico      = frontera independiente de identificadores
Semántica          = única
```

La pluralidad de grafías fuente no constituye pluralidad de autoridad semántica.