# Constructor del manual del lenguaje SVP

## 1. Naturaleza

Este constructor fija la arquitectura de trabajo del manual del lenguaje SVP (extensión `.svp`). Su finalidad es permitir un cierre progresivo por capítulos o tramos sin perder el eje del sistema ni dispersarse en caminos laterales.

No es todavía el manual final. Es la **máquina de construirlo**.

## 2. Regla general

Cada tramo del manual deberá ser:

- suficientemente nuclear para permitir detalle;
- suficientemente general para no romper el hilo;
- subordinado al pliego, a la Frontera normativa, a la IR y a la gramática vigente;
- y verificable mediante ejemplos, contraejemplos y relación explícita con la implementación.

## 3. Unidades de cierre recomendadas

Se recomienda cerrar el manual por **tramos nucleares** y no por lluvia de ideas.

### Tramo 0 — Estatuto del lenguaje
- qué es SVP;
- qué no es;
- qué lugar ocupa respecto de doctrina, IR y backend.

### Tramo 1 — Léxico mínimo y convenciones superficiales
- nombres;
- identificadores;
- forma de declaración;
- disciplina nominal;
- extensión `.svp`.

### Tramo 2 — Niveles N0–N4
- definición;
- estado;
- resultado;
- evolución;
- uso.

### Tramo 3 — Célula, evaluación y clasificación
- `CellSpec`;
- `CellState`;
- `evaluate`;
- umbral;
- criticidad.

### Tramo 4 — Composición y supervisión
- `gate`;
- `supervise`;
- `compose`;
- relación semántica previa.

### Tramo 5 — Resolución y régimen de la U
- `resolve`;
- `Context`;
- `Mechanism`;
- reapertura.

### Tramo 6 — Trayectoria, frame y datos de transición
- `Frame`;
- `TransitionData`;
- `Trajectory`;
- regla append-only.

### Tramo 7 — Consulta y uso
- `query`;
- contexto;
- respuesta, justificación y metadatos.

### Tramo 8 — Diagnósticos, errores y zona prohibida
- catálogo efectivo;
- relación con IR;
- errores estructurales;
- sintaxis inalcanzable.

### Tramo 9 — CLI, Playground, serialización y backend
- CLI actual;
- Playground;
- lowering;
- serialización;
- brújula Rust.


### Tramo 10 — Compatibilidad interlenguajes y adopción
- APIs y enganches;
- wrappers y módulos puente;
- FFI, runtime, CLI y serialización;
- prioridad por lenguajes anfitriones;
- preservación semántica del SV.

## 4. Ficha obligatoria por tramo

Todo tramo del manual deberá declarar:

1. ID del tramo
2. Título
3. Rango
4. Objeto
5. Dependencias
6. Qué fija
7. Qué no fija
8. Ejemplos `.svp`
9. Errores o contraejemplos
10. Relación con IR y docs públicos
11. Criterio de cierre
12. Estado

## 5. Criterio de cierre

Un tramo se considerará cerrable cuando reúna, como mínimo:

- exposición sobria del objeto;
- correspondencia visible con la IR o con la superficie vigente;
- uno o más ejemplos `.svp` suficientemente nucleares;
- delimitación expresa de lo que queda fuera;
- y ausencia de contradicción local suficiente con el pliego o con la Frontera normativa.

## 6. Relación con la compatibilidad interlenguajes

La compatibilidad interlenguajes del SV se tratará como tramo propio del manual y deberá coordinarse con la política y protocolo de compatibilidad interlenguajes del Lenguaje SV.

## 7. Regla de control

El constructor del manual no autoriza lluvias laterales. Toda idea nueva deberá pasar antes por el Wishlist IRQ del ecosistema SV.
