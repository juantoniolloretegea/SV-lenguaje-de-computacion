# Documentación del código Rust en español e inglés

**Versión 1 · (p1+p3)-Bis · S22 · 13 de septiembre de 2026**

## Dictamen y fundamento

Se adopta una convención de documentación bilingüe para el código Rust que construye y valida la DSL. Su objetivo es que terceras personas puedan comprender los contratos, las invariantes y los límites de la realización. No añade perfiles lingüísticos al SV ni modifica la semántica de los programas.

La [especificación de perfiles fuente, §§1, 6 y 7](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e7370d1ad3e75d829b5a032e156d7691aa5a94fd/ESPECIFICACION_NORMATIVA_PERFILES_FUENTE_SVP_ES_EN_v1_2026_08_29.md) limita la canonicalización a las formas constitutivas de la fuente SVP. Excluye comentarios, identificadores, cadenas y datos. El texto documental de los archivos .rs no atraviesa el selector SVP-ES/SVP-EN. Tampoco el idioma de la GUI determina el perfil de compilación.

| Material | Régimen |
| --- | --- |
| Comentario ordinario Rust, // o bloque | Texto de mantenimiento. Español e inglés en el mismo código. No constituye vocabulario SVP. |
| Documentación Rust, /// o //! | Documentación de elemento o módulo. Puede producir rustdoc y contener ejemplos comprobables. |
| Ejemplo de programa .svp dentro de documentación | Declara el perfil que corresponda. Sus palabras constitutivas, datos y bytes se conservan conforme a su contrato. |
| Diagnóstico, cadena emitida o texto mostrado al usuario | Dato de salida o presentación localizada. Requiere su régimen propio; no se modifica como si fuera un comentario. |
| Etiqueta, identificador canónico, campo IR o código de error | Identidad técnica existente. No se traduce ni duplica por esta política. |

Rust trata los comentarios ordinarios como espacio léxico. Los comentarios de documentación se expresan como atributos doc; no deben describirse simplemente como texto que todas las herramientas ignoran. Los bloques de ejemplo de rustdoc pueden compilarse y ejecutarse como pruebas. [Rust Reference: comentarios](https://doc.rust-lang.org/reference/comments.html) · [rustdoc: pruebas documentales](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html). Consulta: 13/09/2026.

## Convención de redacción

1. Para aclaraciones cortas, usar líneas contiguas `// ES:` y `// EN:`. Para módulos, tipos o funciones con contrato, agrupar párrafos `ES` y `EN` en documentación /// o //!.
2. Mantener una misma obligación en ambas lenguas. Revisar negaciones, condiciones, cuantificadores, unidades y límites en el mismo cambio. Ninguna traducción puede crear una garantía adicional.
3. Explicar finalidad, entradas, resultado, invariantes, errores, efectos y límites cuando sean pertinentes. Evitar comentarios que repitan literalmente la instrucción o atribuyan al contenedor la validación del contrato.
4. Conservar nombres de símbolos, variantes, campos, diagnósticos, perfiles y versiones. Los identificadores actúan como anclas comunes.
5. Referenciar la cláusula o prueba que respalda una obligación sustantiva. Distinguir una propiedad implementada de una deuda o propuesta.
6. Mantener los ejemplos de código una sola vez cuando sean comunes; acompañarlos de explicación en ambos idiomas. Si un ejemplo .svp depende del perfil, declararlo y comprobar la variante aplicable.
7. Actualizar simultáneamente ambas lenguas y sus referencias al modificar el código. La divergencia se corrige antes del cierre del incremento.

Ejemplo de estilo, basado en el acceso de lectura existente de IrProgram, sin constituir una API nueva:

```rust
// ES: La consulta expone una referencia compartida; no permite modificar
//     esta colección del programa admitido mediante esta vía.
// EN: The query exposes a shared reference; it does not allow this
//     collection of the admitted program to be modified through this path.
```

## Comprobación y alcance de los cambios

La revisión de comentarios verifica que las afirmaciones describan la fuente exacta. Examina especialmente las fronteras de construcción, las mutaciones, la admisión de entradas, la propagación de errores y la distinción entre estructuras de representación y resultados ejecutados.

Un cambio de prosa en .rs modifica la identidad de ese archivo. No se promete identidad de binarios, símbolos de depuración, números de línea, documentación generada o costes de compilación sólo porque la operación lógica permanezca igual. La revisión debe aislar los cambios de comentario respecto de código, atributos, macros y ejemplos.

Si se modifican comentarios rustdoc, se comprueban los enlaces y se ejecutan las pruebas documentales afectadas en la cadena Rust declarada. Los ejemplos hipotéticos se identifican como tales y no se presentan como API implementada. Las pruebas compile_fail comprueban también que el rechazo observado sea el pertinente; fallar por una importación ausente no prueba una restricción distinta.

Los comentarios dentro de una fuente .svp están excluidos de canonicalización, pero forman parte de sus bytes originales y de source_sha256. Por tanto, identidad de significado e identidad byte a byte se evalúan por separado. Los consumidores de documentación, incluidos asistentes IA, reciben material informativo y no autoridad para alterar contratos o permisos.

## Incorporación progresiva

La primera pasada prioriza frontend.rs, ir.rs, wellformed.rs y frame.rs por su relación con la constitución, los perfiles y el Frame vigente. Después se atienden los módulos de operaciones, proyección y frontera que el workflow determine afectados. Se mantiene un inventario de piezas revisadas y pendientes; no se afirma cobertura total por añadir una cabecera bilingüe.

Esta versión fija la política. No acredita que todo el código existente esté ya comentado en ambas lenguas, ni sustituye la revisión material de cada incremento.

## Contratos de los métodos de composición

La documentación ES/EN de cada operación describirá el receptor, participantes, compatibilidad de codominios e interpretaciones, roles, orientación, salida, errores y efectos. Distinguirá construcción, validación y evaluación. El nombre del método no sustituye la definición algebraica. Se indicará qué referencias normativas justifican la operación y qué casos comprueban su admisión y rechazo.
