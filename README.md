# SV-lenguaje-de-computacion

## Lenguaje de computación del Sistema Vectorial SV

Lenguaje núcleo semántico del Sistema Vectorial SV: pequeño, tipado, auditable y compilable a un backend robusto.

---

### Qué es este proyecto

Este repositorio contiene la especificación, la gramática y la implementación del lenguaje de computación propio del Sistema Vectorial SV. Su propósito no es competir con lenguajes generalistas, sino preservar de forma nativa lo que otros lenguajes solo preservan por disciplina externa.

### Por qué un lenguaje propio

El Sistema Vectorial SV impone invariantes que los lenguajes convencionales no garantizan por construcción:

- El alfabeto {0, 1, U} no son enteros ni booleanos: son estados semánticos.
- U no es null, no es None, no es NaN: es honestidad epistémica.
- Una célula SV(n, b) no es una lista: es una entidad algebraica con restricciones (n = b², b ≥ 3).
- Los operadores lícitos están tipados por la relación semántica previa, no son funciones arbitrarias.
- La trayectoria es inmutable y auditable por construcción.

Un lenguaje propio convierte estas exigencias en **errores de compilación**, no en convenciones de estilo.

### Principio rector

> El SV no necesita un lenguaje propio para existir, pero sí puede justificar un lenguaje propio cuando éste preserve de forma nativa lo que otros lenguajes solo preservan por disciplina externa.

---

### Estructura del repositorio

```
SV-lenguaje-de-computacion/
│
├── README.md                ← Este archivo
├── LICENSE                  ← CC BY-NC-ND 4.0
│
├── spec/                    ← Especificación formal del lenguaje
│   └── README.md
│
├── grammar/                 ← Gramática y parser
│   └── README.md
│
├── src/                     ← Código del transpilador
│   └── README.md
│
├── stdlib/                  ← Biblioteca estándar (cell, gamma, compose...)
│   └── README.md
│
├── examples/                ← Programas de ejemplo
│   └── README.md
│
├── tests/                   ← Suite de conformidad
│   └── README.md
│
└── docs/                    ← Documentación del lenguaje
    └── README.md
```

---

### Fases de desarrollo

| Fase | Objetivo | Estado |
|------|----------|--------|
| **I** | Especificación formal: tipos, álgebra, restricciones, auditoría | En curso |
| **II** | DSL canónica mínima: cell, classify, gamma, compose, query, frame | Pendiente |
| **III** | Transpilador a Rust (+ Python referencia + WASM) | Pendiente |
| **IV** | Suite de conformidad cruzada | Pendiente |
| **V** | Extensión gradual: dominios, consulta avanzada, trayectorias | Pendiente |

---

### Invariantes que el lenguaje garantiza por construcción

1. El alfabeto es {0, 1, U}. Nada más.
2. U es estado epistémico, no error ni nulidad.
3. n = b², b ≥ 3. Siempre. Violarlo es error de compilación.
4. T(n) = ⌊7n/9⌋. Sin excepciones.
5. La clasificación es determinista.
6. La composición depende de la relación semántica previa R(𝒜).
7. Los operadores lícitos son los de ℱ_SV.
8. El polígono es inmutable una vez evaluado.
9. La trayectoria es append-only.
10. Cero estadística, cero probabilidad opaca.
11. Todo es auditable.
12. La IA no es soberana. El humano diseña, el sistema ejecuta.

---

### Tipos primitivos previstos

| Tipo | Significado |
|------|------------|
| `Tri` | Estado ternario: {Zero, One, U} |
| `Cell(n, b)` | Célula SV con restricción n = b² |
| `Vector(n)` | Vector ternario de longitud n |
| `Class` | Resultado de clasificación: {Apto, NoApto, Indeterminado} |
| `Connector` | Conector tipado φ : K → Σ |
| `Gate` | Tabla de admisibilidad 𝒯 |
| `Frame` | Evaluación completa inmutable |
| `Event` | Factor ν (exógeno/endógeno) |
| `Trajectory` | Secuencia append-only de frames |

---

### Errores de compilación (no de ejecución)

- Declarar una célula con n que no sea cuadrado perfecto
- Componer arquitecturas con relación semántica no declarada
- Definir un conector cuyo codominio no sea ternario
- Cerrar un sistema con U relevante sin política explícita de resolución
- Degradar U a entero, booleano o nulidad

---

### Relación con los demás repositorios

| Repositorio | Función | Relación |
|-------------|---------|----------|
| [SV-matematica-semantica](https://github.com/juantoniolloretegea/SV-matematica-semantica) | Doctrina matemática y semántica | Padre doctrinal |
| **SV-lenguaje-de-computacion** (este) | Lenguaje de computación del SV | Subordinado a la doctrina |
| [SVcustos-dataset](https://github.com/juantoniolloretegea/SVcustos-dataset) | Marco de intrusión | Subordinado |
| [SVperitus-dataset](https://github.com/juantoniolloretegea/SVperitus-dataset) | Agentes especializados | Subordinado |

La doctrina reside en SV-matematica-semantica. Este repositorio implementa esa doctrina como lenguaje ejecutable. En caso de discrepancia, prevalece la doctrina.

---

### Datos del proyecto

| Campo | Valor |
|-------|-------|
| **Autor** | Juan Antonio Lloret Egea |
| **ORCID** | [0000-0002-6634-3351](https://orcid.org/0000-0002-6634-3351) |
| **ISSN** | 2695-6411 |
| **Licencia** | CC BY-NC-ND 4.0 |

---

*Lenguaje de computación del Sistema Vectorial SV.*
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0 | ISSN 2695-6411*
