# SV-lenguaje-de-computacion

## Lenguaje de computación del Sistema Vectorial SV

Lenguaje núcleo semántico del Sistema Vectorial SV: pequeño, tipado, auditable y compilable a un backend robusto.

> [!NOTE]
> ### Acceso al emulador online de SVP
> **SVP Playground — Sistema Vectorial SV**
>
> Herramienta online pública del lenguaje SV para escribir código `.svp` directamente en el navegador, parsearlo y obtener la **IR canónica v0.2 en JSON** o el **error correspondiente con su código específico**.
>
> Su función actual es servir como entorno inmediato de prueba y validación del lenguaje: permite comprobar el lowering `.svp → IR v0.2` sin instalación local, con el parser/lowering de referencia cargado desde el propio repositorio y servido mediante GitHub Pages.
>
> No ejecuta todavía el backend: **parsea, valida y serializa**.
>
> **Acceso directo:**  
> https://juantoniolloretegea.github.io/SV-lenguaje-de-computacion/

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

### Documentos normativos y técnicos vigentes

Estos son los tres documentos que definen el lenguaje en su estado actual. Los tres están publicados en la raíz del repositorio y forman una cadena de subordinación descendente:

| Documento | Archivo | Función |
|-----------|---------|---------|
| **Frontera normativa v0** | [`FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md`](FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md) | Establece qué debe conocer, ejecutar, verificar y prohibir el lenguaje. Cuatro bloques: núcleo ejecutor (A), capa formal tipada (B), interfaz analítica (C), zona prohibida (D). |
| **IR canónica v0.2** | [`IR_CANONICA_BIENFORMACION_SV_v0_2.md`](IR_CANONICA_BIENFORMACION_SV_v0_2.md) | Representación intermedia normalizada, serializable y auditable. Cinco niveles ontológicos (N0–N4), juicios de bienformación, 39 errores no silenciables, lowering disciplinado. |
| **Gramática superficial mínima v0.1** | [`GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md`](GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md) | Superficie declarativa del DSL canónico. Declaraciones nominales, seis operadores con lowering unívoco a la IR, enumeraciones cerradas, prohibiciones sintácticas. |

La cadena de compilación que estos documentos definen es:

```
Archivo .svp → Parser → IR canónica v0.2 (JSON) → Backend Rust + Backend Python (referencia) → WASM (futuro)
```

Los tres documentos están subordinados a los *Fundamentos algebraico-semánticos del Sistema Vectorial SV* (Release 3), que residen en el repositorio padre [SV-matematica-semantica](https://github.com/juantoniolloretegea/SV-matematica-semantica). En caso de discrepancia, prevalecen los *Fundamentos*.

### Extensión de archivo

Los programas escritos en el lenguaje SV usan la extensión **`.svp`** (Sistema Vectorial Poligonal). El nombre remite a la imagen poligonal cerrada, invariante constitutivo del Sistema Vectorial SV: la interfaz de inteligibilidad compartida entre ser humano y máquina.

La IR serializada se emite en formato **JSON** canónico (UTF-8, claves ordenadas, salida determinista).

---

### Estructura del repositorio

```
SV-lenguaje-de-computacion/
│
├── README.md                                      ← Este archivo
├── LICENSE                                        ← CC BY-NC-ND 4.0
│
├── FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md           ← Frontera normativa del lenguaje
├── IR_CANONICA_BIENFORMACION_SV_v0_2.md           ← IR canónica y sistema de bienformación
├── GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md        ← Gramática superficial mínima
│
├── spec/                    ← Especificación formal del lenguaje
│   └── README.md
│
├── grammar/                 ← Gramática y parser
│   └── README.md
│
├── src/                     ← Código del parser/lowering de referencia
│   └── README.md
│
├── stdlib/                  ← Biblioteca estándar (cell, gamma, compose...)
│   └── README.md
│
├── examples/                ← Programas de ejemplo .svp
│   └── README.md
│
├── tests/                   ← Suite de conformidad DSL → IR
│   └── README.md
│
└── docs/                    ← Documentación del lenguaje y SVP Playground
    └── index.html
```

---

### Fases de desarrollo

| Fase | Objetivo | Estado |
|------|----------|--------|
| **I** | Especificación formal: frontera normativa, tipos, álgebra, restricciones, auditoría | Cerrada (Frontera normativa v0) |
| **II** | Representación intermedia canónica: niveles ontológicos, bienformación, errores, lowering | Cerrada (IR canónica v0.2) |
| **III** | Gramática superficial mínima: DSL declarativa con lowering unívoco a la IR | Cerrada (Gramática v0.1) |
| **IV** | Parser/lowering de referencia y suite de conformidad DSL → IR | En curso — parser Python, JSON canónico, 15 tests (7 válidos + 8 inválidos) y SVP Playground |
| **V** | Backend Rust + backend Python de referencia | Pendiente |
| **VI** | Compilación a WASM | Pendiente |
| **VII** | Extensión gradual: dominios, consulta avanzada, azúcar derivado | Pendiente |

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
10. Cero estadística, cero minería de datos, cero probabilidad opaca.
11. Todo es auditable.
12. La IA no es soberana. El humano diseña, el sistema ejecuta.

---

### Tipos nucleares del lenguaje

| Tipo | Significado |
|------|-------------|
| `Tri` | Estado ternario: {Zero, One, U} |
| `CellSpec` | Especificación de célula SV con restricción n = b², b ≥ 3 |
| `CoupledSpec` | Célula acoplable con posiciones puente declaradas |
| `Codomain` | Codominio tipado de salida |
| `Connector` | Conector tipado φ : K → Σ |
| `AdmissibilityTable` | Tabla de admisibilidad 𝒯 para compuerta |
| `EvalResult` | Resultado de evaluación (conteos, clasificación, criticidad) |
| `ResolutionRecord` | Registro de resolución de U con contexto y mecanismo |
| `Frame` | Evaluación completa inmutable, indexada por posición ordinal |
| `Trajectory` | Secuencia append-only de entradas (frames y datos de transición) |
| `CompositionGraph` | Grafo DAG de células con aristas enriquecidas |

La nomenclatura completa, con desambiguación de colisiones del corpus, se encuentra en la Frontera normativa v0 (§3) y en la Gramática superficial mínima v0.1 (§4).

---

### Errores de compilación (no de ejecución)

- Declarar una célula con b < 3.
- Componer arquitecturas sin relación semántica declarada.
- Invocar `gate` sin tabla de admisibilidad nombrada.
- Invocar `query` sin constructor explícito de contexto.
- Invocar `resolve` sin contexto de evidencia ni mecanismo de revisión.
- Definir un conector cuyo codominio no sea ternario.
- Degradar U a entero, booleano o nulidad.
- Construir literales de resultado (`EvalResult`, `GateResult`, etc.) en lugar de obtenerlos por operador.

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
