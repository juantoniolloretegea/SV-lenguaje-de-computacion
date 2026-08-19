# Acta técnica de recepción latente de «No clausura certificada»

**Fecha:** 19/08/2026  
**Hora (Europe/Madrid):** 08:20:00  
**Naturaleza:** acta de gobierno técnico y recepción doctrinal latente  
**Frente:** Lenguaje SV / fundamentos / no clausura certificada  
**Estado:** cerrada para recepción; continuidad técnica latente  
**Sede doctrinal de origen:** `SV-matematica-semantica/documentos/fundamentos/`  
**Sede técnica receptora:** `SV-lenguaje-de-computacion`  
**Estatuto:** `LATENTE_LEGITIMO`

## 1. Objeto

La presente acta deja constancia de la recepción, en la sede operativa del Lenguaje SV, de la publicación de fundamentos titulada **«No clausura certificada en sistemas finitos de resolución: certificados operativos, morfismos conservativos y complejidad de revisión»**.

El propósito no es integrar esa publicación en el lenguaje vigente. Es impedir dos errores simétricos:

1. que una unidad posterior ignore la publicación al continuar la IR, la gramática, `U`, `resolve` o la revisión de estados;
2. que la publicación se interprete como semántica ya vigente del DSL, como cuarto valor de `Σ`, como operador nuevo o como autorización de infraestructura de ejecución.

## 2. Jerarquía aplicable

Se mantiene la fijación de sedes vigente:

- `SV-matematica-semantica` es la sede superior doctrinal y normativa;
- `SV-lenguaje-de-computacion` es la sede operativa y técnica;
- el movimiento descendente legítimo es `doctrina → especificación → lenguaje → pruebas → evidencia`;
- ninguna sede técnica modifica por silencio la doctrina superior.

La presencia de la publicación en la sede de fundamentos no equivale a integración técnica automática.

## 3. Fuente doctrinal recibida

La fuente queda identificada por:

- sede: `SV-matematica-semantica/documentos/fundamentos/no-clausura-certificada-en-sistemas-finitos-de-resolucion/`;
- título: **«No clausura certificada en sistemas finitos de resolución: certificados operativos, morfismos conservativos y complejidad de revisión»**;
- autor: Juan Antonio Lloret Egea;
- DOI: **`10.21428/39829d0b.f0892864`**;
- localizador: `https://doi.org/10.21428/39829d0b.f0892864`;
- fecha del preprint español: 08/08/2026;
- estatuto editorial a esta fecha: preprint no revisado por pares.

Esta acta no sustituye el texto de la publicación ni resume sus demostraciones. Recibe únicamente el estatuto que el Lenguaje debe conservar.

## 4. Hecho doctrinal que el Lenguaje debe conservar

La publicación formaliza, para sistemas finitos de resolución, la separación entre:

- un proceso de resolución todavía no concluido;
- un episodio ya completado cuyo perfil de clausura certificado no es unitario.

El alfabeto visible permanece `Σ = {0, 1, U}`. En la semántica finita general, `U` denota un episodio completado con perfil no unitario certificado. El trabajo pendiente es un estado del proceso, no un cuarto valor del marco completado.

La publicación establece, además, que un certificado exacto de alcanzabilidad es el medio de verificar ese perfil, y que los morfismos conservativos y la memoria mínima de revisión pertenecen al aparato matemático de la no clausura. En la realización SV, un perfil no unitario verificado es necesario pero no suficiente para un registro soberano de `U`: la autorización humana es una condición adicional de instancia, exterior a los teoremas finitos generales.

Ninguno de estos objetos se declara aquí como parte del Lenguaje SV vigente.

## 5. Estatuto de recepción

La publicación queda clasificada como:

**`LATENTE_LEGITIMO`**

Ese estatuto significa:

- el certificado de no clausura **no** es trabajo pendiente;
- el certificado de no clausura **no** es un cuarto valor;
- no altera `Σ = {0, 1, U}`;
- no crea operador nuevo;
- no obliga cambio de DSL, gramática, AST, analizador sintáctico, validador, descenso a IR ni serialización;
- no abre AUTH, aprendizaje trazable ni etapa frontal adicional;
- no autoriza ejecutar `χ`, `resolve` como solucionador automático ni introducir tipos `Context`/`Mechanism` materiales.

## 6. Relación con piezas ya recibidas

Esta recepción es independiente de:

- la preservación controlada de SV-AUTH A.2 r2 y J6, de 14/08/2026;
- la recepción y la fijación de fuente del aprendizaje trazable, de 15/08/2026 y 16/08/2026.

Las tres piezas permanecen latentes y no se fusionan. En particular, `COMMIT_SOV_U` y la capa de autoridad humana no quedan absorbidas por el certificado de no clausura, ni el certificado queda absorbido por AUTH.

## 7. Lo que esta acta no autoriza

Queda expresamente no autorizado:

- modificar `IR_CANONICA_BIENFORMACION_SV_v0_2.md`;
- modificar `GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md`;
- añadir palabras reservadas, nodos IR o códigos de error;
- alterar analizador sintáctico, validador, descenso, serializador, interfaz de línea de órdenes, Playground o bibliotecas;
- abrir infraestructura de ejecución, Rust productivo, WASM, FFL-C/D/E, NLP o Beta / 0-1;
- introducir un objeto de certificado en el DSL;
- tratar `U` como marca de búsqueda incompleta;
- autocerrar `U` para simular un veredicto.

## 8. Condición de reapertura implementativa

Antes del primer cambio material que pretenda traducir la no clausura certificada al Lenguaje SV deberá existir un acta arquitectónica previa que, como mínimo, fije:

- la versión doctrinal exacta tomada como fuente;
- el objeto de certificado, si procede, y su nivel ontológico;
- la relación con `resolve`, `ResSpec`, `U` y, en su caso, AUTH;
- la prueba discriminante que separe trabajo pendiente de episodio certificado;
- la confirmación de que `Σ` permanece ternario.

Mientras no exista esa acta, la publicación permanece recibida y no implementada.

## 9. Decisión

Se recibe la publicación con estatuto `LATENTE_LEGITIMO`.  
Se abre el asiento `RETP-2026-060` en el registro maestro.  
No se modifica código, gramática ni IR.

**Estado:** cerrado para recepción; continuidad técnica latente.
