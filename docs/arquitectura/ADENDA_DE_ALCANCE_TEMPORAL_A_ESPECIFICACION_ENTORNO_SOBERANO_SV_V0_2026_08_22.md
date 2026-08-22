# Adenda de alcance a la especificación arquitectónica del entorno soberano SV v0

**Fecha:** 22/08/2026  
**Estado:** aclaración arquitectónica  
**Ámbito:** `ESPECIFICACION_ARQUITECTONICA_ENTORNO_EJECUCION_SOBERANO_SV_V0.md`

## 1. Objeto

Esta adenda precisa el alcance temporal de la secuencia R0–R4 definida en la especificación arquitectónica v0 del entorno de ejecución soberano del Lenguaje SV.

Su finalidad es impedir que la enumeración de R0–R4 o la referencia a una «siguiente evolución» se interpreten como apertura automática de una fase de implementación Rust.

No modifica las propiedades arquitectónicas, las garantías SEC.0, la elección de Rust como objetivo soberano ni la estructura técnica de R0–R4.

## 2. Estatuto de R0–R4

R0–R4 describen el **orden interno previsto para una futura materialización soberana cuando ésta haya sido formalmente abierta**.

No constituyen por sí mismos:

- una orden de inicio;
- una autorización de implementación;
- una afirmación de que el siguiente cambio del repositorio deba ser R0;
- una sustitución de las decisiones previas necesarias para determinar el alcance materializable;
- una garantía de que todos los perfiles futuros deban realizar todas las etapas o capacidades descritas.

## 3. Apertura separada de la materialización

La implementación soberana deberá abrirse mediante una decisión separada que determine, al menos:

1. el alcance de la realización;
2. las propiedades contractuales que se materializan;
3. la identidad inicial del `SUT`;
4. las capacidades y dependencias incluidas;
5. el perfil de garantías pretendido;
6. los límites y propiedades que permanecerán `NO_PROBADO`.

Hasta que exista esa apertura, R0–R4 conservan estatuto arquitectónico y no ejecutivo.

## 4. Interpretación de la sección 24

La sección 24 de la especificación v0 debe interpretarse como una secuencia interna condicionada:

```text
apertura formal de materialización
→ R0
→ R1
→ R2
→ R3
→ R4
```

según el alcance y perfil que hayan sido aprobados.

La enumeración no establece que R0 sea necesariamente el siguiente acto del proyecto tras el cierre contractual de SEC.0.

## 5. Interpretación del cierre de v0

La frase final de la especificación v0:

> «La siguiente evolución deberá concretar las interfaces mínimas entre el núcleo Rust y las dependencias externas sin seleccionar prematuramente tecnologías de plataforma»

se entiende desde esta adenda en sentido condicional:

> **Cuando se abra formalmente la materialización soberana, su evolución deberá concretar las interfaces mínimas entre el núcleo Rust y las dependencias externas sin seleccionar prematuramente tecnologías de plataforma.**

La frase no constituye una apertura implícita de implementación.

## 6. Relación con el objetivo Rust

Se mantiene `OBJETIVO_RUST_0_BACKEND_SOBERANO.md`: Rust continúa siendo el objetivo principal del backend soberano futuro y Python conserva temporalmente su función de etapa frontal de referencia.

La elección arquitectónica del lenguaje de implementación no determina por sí sola el momento de inicio de la realización.

## 7. No efectos

Esta adenda no:

- modifica gramática, IR, validador ni catálogo diagnóstico;
- inicia código Rust;
- selecciona sistema operativo, almacenamiento, hipervisor, hardware, mecanismo criptográfico ni servicio de identidad;
- acredita Garantía I o Garantía II;
- cambia el estatuto `NO_PROBADO` de propiedades materiales no ejercidas;
- altera la identidad exacta exigida por SEC.0-T para transferir evidencia.

## 8. Cierre

La especificación arquitectónica v0 permanece vigente con esta precisión de alcance.

R0–R4 son una secuencia de realización futura y condicionada. Su existencia documental no abre la implementación soberana ni determina por sí sola el siguiente acto técnico del proyecto.