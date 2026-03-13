# OBJETIVO_RUST_0_BACKEND_SOBERANO.md

## 1. Objeto

Este documento fija, con alcance arquitectónico y no doctrinal, el objetivo principal de backend soberano del lenguaje SV.

Su finalidad es evitar deriva futura en decisiones de implementación, runtime y autonomía procesal, dejando constancia expresa de la brújula técnica adoptada.

No modifica la doctrina base del Sistema Vectorial SV, la Frontera normativa del lenguaje SV, la IR canónica v0.2 ni la Gramática superficial mínima v0.1.

## 2. Decisión

Se fija como objetivo principal del proyecto:

**backend soberano en Rust para la autonomía procesal del lenguaje SV.**

Se fija como objetivo secundario eventual:

**C como backend alternativo de interoperabilidad, exportación o soporte complementario, pero no como destino soberano principal.**

## 3. Alcance de la decisión

Esta decisión afecta únicamente a la dirección arquitectónica futura del compilador y del runtime del lenguaje SV.

No implica:

- reescritura inmediata del frontend de referencia;
- apertura inmediata de backend ejecutable;
- abandono del parser/lowering de referencia actual en Python;
- introducción de una máquina virtual propia;
- cambio de fase del proyecto fuera del orden ya consolidado.

## 4. Fundamento de la decisión

La decisión se adopta por convergencia de tres exigencias del proyecto:

### 4.1. Necesidad del modelo SV

El lenguaje SV no persigue ser un generalista sustitutivo de C, sino un lenguaje específico con garantías semánticas fuertes para el marco SV.

Esto exige que la riqueza semántica capturada en compilación no se degrade después en un runtime frágil, opaco o excesivamente pesado.

### 4.2. Autonomía procesal fuerte

El objetivo del proyecto no es depender de Python en destino ni simular autonomía mediante empaquetados opacos.

La autonomía procesal buscada exige, en su fase madura:

- compilación o ejecución sin Python presente;
- artefactos distribuibles autónomos;
- runtime mínimo y explícito;
- dependencia reducida de artefactos auxiliares pesados.

### 4.3. Robustez estructural del backend

Para el backend soberano del lenguaje SV se prioriza:

- seguridad de memoria;
- previsibilidad del comportamiento;
- control del runtime;
- sostenibilidad del mantenimiento;
- coherencia con la línea transversal SEC-0.

En ese equilibrio, Rust ofrece una convergencia más fuerte que C como objetivo principal.

## 5. Razón de preferencia de Rust frente a C como objetivo principal

Rust se elige como objetivo principal porque proporciona, en mejor equilibrio conjunto:

- binarios autónomos y ligeros;
- control fino de memoria sin GC obligatorio;
- menor exposición a fallos críticos de memoria en el backend;
- mayor compatibilidad con una arquitectura de runtime pequeño y explícito;
- mejor alineación con una estrategia de robustez progresiva del compilador.

C no queda descartado como tecnología válida, pero se considera más apropiado, por ahora, como:

- backend alternativo eventual;
- vía de exportación;
- soporte de interoperabilidad;
- opción táctica futura, no brújula principal.

## 6. Qué permanece igual tras esta decisión

Tras esta decisión, permanece vigente:

- Python como frontend de referencia actual;
- el parser/lowering de referencia ya consolidado;
- la suite de conformidad y de robustez ya existente;
- la Fase IV del proyecto tal como está descrita públicamente;
- la línea SEC-0 como baseline transversal del compilador.

## 7. Qué debe evitarse a partir de ahora

A partir de esta fijación arquitectónica debe evitarse:

- presentar empaquetados de Python como solución final de autonomía;
- abrir una VM propia de forma prematura;
- dispersar el proyecto en una estrategia dual Rust/C desde el primer momento;
- tomar decisiones de tooling que alejen estructuralmente al lenguaje de un backend soberano en Rust;
- confundir comodidad del frontend actual con arquitectura final del sistema.

## 8. Efectos prácticos inmediatos

Esta decisión no abre todavía un frente de backend ejecutable.

Su efecto inmediato es de orientación:

- toda propuesta futura de backend, runtime o distribución debe evaluarse preguntando si acerca o aleja al proyecto de una autonomía soberana en Rust;
- C podrá contemplarse como objetivo complementario futuro, pero no como centro de gravedad del proyecto.

## 9. Decisión diferida

Quedan diferidas a actos posteriores y separados:

- la especificación formal del runtime mínimo de SV;
- la definición completa del modelo de ejecución de programas SV;
- la hoja de ruta concreta de transición desde Python a backend soberano;
- la eventual política de interoperabilidad o exportación a C.

## 10. Fórmula de cierre

Se fija, por tanto, como objetivo de proyecto:

**backend soberano principal en Rust para la autonomía procesal futura del lenguaje SV, manteniendo C como opción secundaria eventual y manteniendo Python sólo como frontend de referencia mientras no se abra la fase de backend.**
