# Política y protocolo de compatibilidad interlenguajes del Lenguaje SV

## 1. Naturaleza

Este documento fija la política mínima y el protocolo de fase para que el Lenguaje de Programación Vectorial Poligonal, SV, pueda disponer en el futuro de APIs, enganches, módulos de compatibilidad y puentes de adopción hacia otros lenguajes sin degradar su semántica propia ni abrir frentes prematuros.

## 2. Tesis de fase

La compatibilidad interlenguajes es una necesidad estratégica de adopción y ecosistema, pero no constituye en esta fase un bloqueante del cierre nuclear del lenguaje. Debe quedar **ya en vena** como política y como constructor documental, dejando la implementación efectiva por lenguajes para fases posteriores gobernadas por evidencia, prioridad e infraestructura real.

## 3. Lenguajes prioritarios de primera vigilancia

La primera prioridad estratégica de compatibilidad queda fijada, en esta fase, sobre:

1. Python
2. Rust
3. C
4. C++
5. Kotlin

Sin perjuicio de ello, se mantendrá una vigilancia expresa sobre lenguajes de legado empresarial aún materialmente vivos, en particular Cobol.

## 4. Regla de compatibilidad

Toda compatibilidad interlenguajes del SV deberá respetar estas cuatro condiciones:

1. no alterar la semántica nuclear del lenguaje SV;
2. no convertir el lenguaje huésped en autoridad doctrinal ni semántica;
3. preservar trazabilidad entre superficie huésped, lowering o puente y significado SV;
4. declarar con precisión si la compatibilidad es API, FFI, wrapper, serialización, runtime, transpilación, embedding o herramienta de interoperabilidad.

## 5. Bloques de trabajo

### Bloque A — Cartografía
- clasificar tipos de compatibilidad posibles;
- distinguir adopción de ejecución;
- separar SDK, runtime, binding, wrapper, FFI y CLI.

### Bloque B — Prioridad
- priorizar primero Python, Rust, C, C++, Kotlin;
- mantener Cobol en vigilancia de legado, no como prioridad inicial de implementación.

### Bloque C — Contrato semántico
- fijar qué puede exponerse fuera del SV sin pérdida;
- fijar qué no debe cruzar fronteras si degradaría la `U`, la IR o la estratificación N0–N4.

### Bloque D — Ruta de implementación
- abrir por lenguaje solo cuando exista fase, necesidad y evidencia;
- exigir por cada lenguaje una ficha mínima de alcance, riesgos, modelo de compatibilidad y criterio de cierre.

## 6. Protocolo de activación de un lenguaje concreto

Ningún lenguaje concreto se abrirá por entusiasmo genérico. Para activarlo deberán cumplirse, como mínimo, estos pasos:

1. entrada o reevaluación en la lista de deseos IRQ;
2. consulta previa del CSV técnico disponible;
3. asignación Beta de IRQ y ruta;
4. ficha de alcance del lenguaje huésped;
5. decisión explícita de fase;
6. apertura de tramo específico en el manual SVP;
7. creación, si procede, de subfrente técnico o experimental con pruebas y criterio de cierre.

## 7. Relación con el manual del lenguaje SVP

La compatibilidad interlenguajes deberá disponerse también como tramo específico del manual del lenguaje SVP, para que el ecosistema no dependa de memoria oral ni de lluvia de ideas inconexas.

## 8. Lo que este protocolo no autoriza

Este documento no autoriza todavía:

- implementar bindings reales;
- prometer APIs públicas cerradas;
- abrir soporte oficial completo por lenguaje;
- ni degradar el objetivo principal de cierre del lenguaje SV.

## 9. Estado de fase

Estado actual: **política activa, protocolo activo, implementación diferida por prioridades y evidencia**.
