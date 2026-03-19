# Hoja de ruta del frente final del lenguaje SV

## 1. Objetivo general

Cerrar el frente operativo del lenguaje SV con suficiente coherencia semántico-diagnóstica, documental y de evidencia como para habilitar, sin fractura, el paso posterior al backend soberano.

## 2. Orden de trabajo

### Bloque A — Contrato diagnóstico

Revisar la concordancia efectiva entre:

- IR canónica v0.2;
- catálogo canónico de errores;
- catálogo implementativo observable;
- emisión real del `validator` y del `runner`.

### Bloque B — Cadena de implementación

Revisar la continuidad real entre:

- `lexer` y `parser`;
- `lowering` e IR;
- `validator`;
- `serializer`;
- `CLI`;
- `playground`.

### Bloque C — Suite y evidencia

Determinar qué cubre realmente la suite y qué no cubre todavía, distinguiendo entre:

- conformidad;
- smoke;
- SEC-0;
- cobertura de errores observables;
- deuda de pruebas.

### Bloque D — Documentación pública

Alinear la documentación pública con el estado verificable del código y de la suite, evitando sobreatribuciones de cobertura o de cierre funcional.

### Bloque E — ABI semántico-diagnóstico

Dejar fijado el núcleo mínimo que deberá preservarse al pasar, cuando proceda, al backend soberano.

## 3. Regla de disciplina

Cada bloque debe cerrarse con evidencia suficiente antes de abrir el siguiente, salvo que una sorpresa técnica obligue a realimentación adversarial controlada.

## 4. Criterio de prioridad

La prioridad inmediata del frente es el **saneamiento del contrato diagnóstico**.

## 5. Lo que no debe confundirse

- saneamiento de frontend no es apertura de backend;
- documentación ajustada no equivale a cierre funcional nuevo;
- laboratorio Beta no es vía ordinaria de resolución de tensiones del núcleo.
