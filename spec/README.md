# Especificación formal del lenguaje

Esta carpeta reúne piezas de especificación formal del lenguaje de computación del Sistema Vectorial SV.

Su función no es sustituir a **Fundamentos algebraico-semánticos del Sistema Vectorial SV** ni a la serie **Álgebra de composición intercelular del Sistema Vectorial SV**, sino traducir la doctrina canónica al plano técnico del lenguaje, preservando su semántica y sus restricciones de implementación.

## Alcance de esta carpeta

Las piezas alojadas en `spec/` pueden cumplir, entre otras, las siguientes funciones:

- fijar la traducción técnica mínima de nociones ya asentadas en la doctrina del SV;
- declarar restricciones de implementación que impidan empobrecimientos semánticos del lenguaje;
- articular el paso entre doctrina, gramática superficial, IR canónica y validación;
- ofrecer criterios operativos subordinados para el parser, el lowering, la auditoría y los futuros backends.

Toda especificación aquí contenida debe leerse como **subordinada** a la doctrina matemática y semántica del sistema.

## Pieza activa actual

En el momento actual, esta carpeta incorpora como pieza activa principal:

- [`u_en_svp_semantica_formal_y_regimen_de_resolucion.md`](./u_en_svp_semantica_formal_y_regimen_de_resolucion.md)

Esta pieza constituye una **derivación técnica subordinada** de la publicación canónica:

- *Origen doctrinal, definición y alcance de la U en el Sistema Vectorial SV*
- URL canónica: `https://www.itvia.online/pub/origen-doctrinal-definicion-y-alcance-de-la-u-en-el-sistema-vectorial-sv/release/1`

Su misión es impedir que la implementación del lenguaje degrade la U, la confunda con nulidad o la reduzca a estadística, inferencia opaca o comodidad de implementación.

## Criterio de jerarquía

La interpretación de cualquier archivo incluido en esta carpeta queda subordinada a los siguientes textos, por este orden:

1. **Fundamentos algebraico-semánticos del Sistema Vectorial SV**
2. **Documentos I–VI** de la serie **Álgebra de composición intercelular del Sistema Vectorial SV**
3. **La publicación canónica sobre la U** en `itvia.online`
4. **La Frontera normativa del lenguaje SV**
5. Las presentes especificaciones técnicas subordinadas

Ningún archivo de esta carpeta puede leerse como refundación doctrinal ni como corrección lateral del corpus ya publicado.

## Relación con el resto del repositorio

La carpeta `spec/` se articula con:

- `grammar/`, para la gramática superficial y el parser;
- `src/`, para la implementación de referencia;
- `tests/`, para la verificación de conformidad;
- y la raíz del repositorio, donde viven la Frontera normativa, la IR canónica y la Gramática superficial mínima.

Su función es dar soporte formal a esas capas sin sustituirlas.

## Regla editorial

Los documentos canónicos del sistema se publican en `itvia.online`.
Los repositorios contienen código, artefactos técnicos, implementaciones de referencia y reflejos subordinados.

Por ello, los archivos aquí incluidos no sustituyen la publicación canónica, sino que la acompañan dentro del entorno de trabajo y verificación del lenguaje.

## Estado

Carpeta activada con derivación técnica subordinada de la U para el lenguaje de computación del Sistema Vectorial SV.
