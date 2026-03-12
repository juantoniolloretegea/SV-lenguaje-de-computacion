# Frontera normativa del lenguaje de computación del Sistema Vectorial SV — v0

## Especificación de alcance, tipos, operadores y restricciones del núcleo semántico

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** [0000-0002-6634-3351](https://orcid.org/0000-0002-6634-3351)  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Fecha:** marzo 2026  
**Estado:** Documento normativo del lenguaje — v0  
**Repositorio:** [SV-lenguaje-de-computacion](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion)

© Juan Antonio Lloret Egea, marzo 2026. Todos los derechos reservados conforme a CC BY-NC-ND 4.0.

---

## Resumen

Este documento fija la frontera normativa del lenguaje de computación del Sistema Vectorial SV en su versión 0. Establece qué debe conocer el lenguaje, qué debe poder verificar, qué debe ejecutar de forma nativa y qué no debe permitir.

La frontera se organiza en cuatro bloques: el núcleo ejecutor mínimo (Bloque A), la capa formal tipada obligatoria (Bloque B), la interfaz analítica formal (Bloque C) y la zona prohibida o prematura (Bloque D). Los cuatro bloques se derivan del corpus doctrinal publicado del Sistema Vectorial SV, compuesto por los *Fundamentos algebraico-semánticos del Sistema Vectorial SV* y los Documentos I a VI de la serie de composición intercelular.

El lenguaje se concibe como DSL semántica restringida, no como lenguaje generalista. Su legitimidad no reside en la novedad técnica, sino en hacer estructuralmente más difícil la traición semántica del marco. Su validación se rige por conformidad cruzada, no por elegancia sintáctica.

---

## 1. Subordinación doctrinal

Este documento es doctrina derivada del corpus publicado del Sistema Vectorial SV. La autoridad normativa suprema reside en los *Fundamentos algebraico-semánticos del Sistema Vectorial SV* (Release 3). Todo lo que este documento establece debe ser coherente con dicho corpus. En caso de discrepancia, prevalecen los *Fundamentos*.

### Corpus de referencia

| Documento | Nivel | Contenido formal |
|-----------|-------|------------------|
| Fundamentos algebraico-semánticos del SV (R3) | Autoridad normativa suprema | Célula, alfabeto, umbral, evaluación, composición tipada, familia ℱ_SV, invariantes |
| Doc I — Transmisión en serie por parámetro puente (R4) | Composición, Grado A | Célula acoplable, conector, grafo DAG, regímenes RS/RG, conflicto, criticidad condicionada |
| Doc II — Gramática general de composición (R1) | Composición, Grados B/C | Relación semántica previa, compuerta, meta-supervisión, Comp, escala de madurez |
| Doc III — Horizonte de sucesos y reevaluación discreta (R1) | Reevaluación | Frame, dato de transición, operador inducido, trayectoria, cascada |
| Doc IV — Transducción al alfabeto ternario e interfaz paramétrica (R1) | Transducción | Cadena formal del mundo al sistema, sensor, admisibilidad, ternarización, U silenciosa |
| Doc V — Invariantes, agentes y operador de consulta (R2) | Uso y consulta | Dominio, agente, operador de consulta, teoremas de invariancia |
| Doc VI — Análisis discreto, representaciones y secuencias (R1) | Herramientas de análisis | Diferencias finitas, transformada Z, función generatriz, matrices de grafo, codificaciones |

El corpus completo comprende 6 proposiciones fundacionales y 30 proposiciones y teoremas en la serie I–VI, para un total de 36 piezas demostrativas que el lenguaje debe respetar.

---

## 2. Justificación del lenguaje

El Sistema Vectorial SV puede implementarse correctamente en lenguajes existentes, pero su semántica nuclear puede ser vulnerada por decisiones locales de ingeniería. El símbolo U puede degradarse internamente a entero, bandera, nulo o convención no auditable. Los operadores del marco pueden reescribirse como funciones semánticamente parecidas pero no idénticas. Las precondiciones constitutivas pueden quedarse en comentarios, tests o disciplina humana. La auditoría puede reducirse a texto libre en lugar de ser objeto semántico del sistema.

El lenguaje se justifica si, y solo si, convierte estas vulnerabilidades en errores formales detectables. Si no lo consigue, no merece existir.

---

## 3. Nomenclatura desambiguada

El corpus doctrinal contiene colisiones de nombre entre símbolos con significados distintos. El lenguaje las resuelve con la siguiente tabla de correspondencia.

| Nombre en el lenguaje | Símbolo del corpus | Significado | Origen |
|-----------------------|-------------------|-------------|--------|
| `Tri` | Σ = {0, 1, U} | Alfabeto ternario irreductible | Fundamentos §2 |
| `VisualCode` | ρ : Σ → {1,2,3} | Codificación visible para representación polar | Fundamentos §4.1 |
| `Role` | ρᵢ (componente de Cᵢ) | Rol estructural de la célula en la arquitectura | Fundamentos §7.2 |
| `Influence` | ρ(C, 𝒜) | Influencia estructural de una célula en el grafo | Doc VI §7.5 |
| `Codomain` | Ωᵢ / Kᵢ | Codominio tipado de salida de la célula | Fundamentos §7.2, Doc I §2.3 |
| `OutputSemantics` | κᵢ | Interpretación semántica del codominio de salida | Fundamentos §7.2 |
| `BridgeSet` | Bᵢ ⊆ {1, …, nᵢ} | Conjunto de posiciones declaradas como parámetro puente | Doc I §3.3 |
| `Connector` | φⱼ→ᵢ⁽ᵏ⁾ : Kⱼ → Σ | Conector de transmisión intercelular | Doc I §4.2 |
| `Ternarizer` | τⱼ : Oⱼ → Σ | Función de ternarización | Doc IV §4.5 |
| `AdmissibilityTable` | 𝒯 : Kᵢ × Kⱼ → K_comp | Tabla de admisibilidad de una compuerta | Doc II §9.2 |
| `InducedTransitionOp` | 𝒯_{νₙ} | Operador inducido por un dato de transición | Doc III §3.6 |

---

## 4. Bloque A — Núcleo ejecutor mínimo (v0)

El núcleo ejecutor es lo que el lenguaje debe implementar de forma nativa y ejecutable desde su primera versión. Estas son las piezas que no pueden delegarse a bibliotecas externas ni simularse por convención.

### A.1. Tipo ternario irreductible

`Tri = {Zero, One, U}`

Sin conversión implícita a `bool`, `int`, `float`, `null`, `None`, `NaN` ni equivalente. Toda conversión debe ser explícita, tipada y auditable.

### A.2. Restricción arquitectónica

Toda célula del sistema tiene tamaño `n = b²` con `b ∈ ℕ` y `b ≥ 3`. Esta restricción es constitutiva e incondicional. Valores válidos de n: 9, 16, 25, 36, 49, 64, etc. El lenguaje debe rechazar la construcción de una célula con n que no cumpla esta condición.

### A.3. Umbral canónico, conteos y clasificación

El umbral canónico es `T(n) = ⌊7n/9⌋`, sin excepciones ni parámetros ajustables.

Los conteos `N₀(v)`, `N₁(v)` y `Nᵤ(v)` satisfacen `N₀ + N₁ + Nᵤ = n`.

La clasificación determinista se evalúa con precedencia desfavorable: si `N₁(v) ≥ T(n)`, la célula es NO_APTA; si `N₀(v) ≥ T(n)`, la célula es APTA; en otro caso, INDETERMINADA. La unicidad de la clasificación fuerte está garantizada por la Proposición 6 de los *Fundamentos*.

### A.4. Función de criticidad

`Γ(v) = Nᵤ(v) − min(δ₀(v), δ₁(v))`

donde `δ₀(v) = T(n) − N₀(v)` y `δ₁(v) = T(n) − N₁(v)`.

Γ no clasifica. Enriquece. Mide cuánto importa la indeterminación.

### A.5. Función de resolución

`Res : {U} × Context × Mechanism → {0, 1, U}`

Toda resolución de U debe pasar por un objeto formal que registre el contexto de evidencia y el mecanismo de revisión. `Res` entra en el núcleo como tipo obligatorio que garantiza trazabilidad, no como función con implementación cerrada.

### A.6. Célula composable canónica

`Cᵢ = (vᵢ, Codomain_i, OutputSemantics_i, Role_i)`

donde `vᵢ ∈ Tri^n`, `Codomain_i` es el espacio de salidas posibles, `OutputSemantics_i` es la interpretación semántica del codominio y `Role_i` es el rol estructural en la arquitectura.

### A.7. Célula acoplable

`Ĉᵢ = (Cᵢ, BridgeSet_i)`

donde `BridgeSet_i ⊆ {1, …, nᵢ}` es el conjunto de posiciones declaradas como parámetros puente. Las posiciones puente son una propiedad de la célula, no un rol de la célula. Una célula con BridgeSet vacío es composable pero no participa en transmisión en serie.

### A.8. Función de evaluación

`χᵢ : Cᵢ → Codomain_i`

Factorizable como `evaluate` (lectura de conteos sobre el vector) seguido de `classify` (aplicación de T(n) y regla de clasificación). La factorización no es opcional: el lenguaje debe mantener ambas fases distinguibles para trazabilidad.

### A.9. Operadores de composición espacial

El núcleo reconoce los siguientes operadores con firma:

| Operador | Nombre | Firma simplificada | Grado |
|----------|--------|---------------------|-------|
| `σ_{k,φ}` | Serie por puente | Transmisión tipada entre células acopladas por conector | A |
| `⊗_gate` | Compuerta jerárquica | Combinación por tabla de admisibilidad | B |
| `▷` | Supervisión meta | Control de integridad de segundo orden | B |
| `Γ` | Criticidad | Enriquecimiento de la indeterminación | — |
| `Ψ_{i,k}` | Conflicto | Resolución de concurrencia en régimen general | A (serie) |

La dominancia homogénea (`max`, `min`) es azúcar semántico sobre `⊗_gate` con tabla trivial. El lenguaje puede ofrecer `max`/`min` como construcciones de conveniencia, pero internamente deben resolverse como instancias de compuerta, lo que garantiza que la relación semántica previa sea obligatoria incluso en el caso más simple.

### A.10. Constructor de arquitectura

`Comp` es reconocido como constructor organizativo de Nivel 4. No tiene firma cerrada al mismo nivel que los operadores de Nivel 3. Su axiomática fuerte sigue abierta. El lenguaje debe poder representar arquitecturas construidas con Comp, pero no debe tratarlo como si tuviera las mismas garantías formales que `σ_{k,φ}` o `⊗_gate`.

---

## 5. Bloque B — Capa formal tipada obligatoria

El lenguaje debe conocer estos tipos desde v0 y debe poder verificar sus invariantes, aunque la ejecución completa de la maquinaria asociada no sea obligatoria en la primera versión.

### B.1. Horizonte de sucesos

`ℋ(𝒜) = {ε₁, ε₂, …, εₘ}` — conjunto de tipos de suceso relevantes, declarados formalmente desde el dominio. El horizonte no se infiere por probabilidad ni por minería de datos.

### B.2. Frame canónico

`Sₙ = ℰ(𝒜, Pₙ)` — evaluación completa del sistema tras sucesos pertinentes. Invariante: una vez construido, un frame no puede ser modificado por operaciones posteriores (inmutabilidad).

### B.3. Dato de transición

`νₙ = {(εᵢ, τᵢ)}` con `τᵢ ∈ Tri` — registro de sucesos instanciados que motivan una reevaluación.

### B.4. Operador inducido

`InducedTransitionOp(νₙ)` — actualización del estado paramétrico del sistema a partir de un dato de transición suficientemente determinado.

### B.5. Trayectoria

`T = (S₁, ν₁, S₂, ν₂, …, Sₙ)` — secuencia ordenada de frames y datos de transición. Invariante: crecimiento por adición sin reescritura retrospectiva (append-only).

### B.6. Cadena de transducción

La entrada del mundo al sistema sigue la cadena formal:

`x ∈ Wⱼ → ϕⱼ(x) ∈ Oⱼ^⊥ → rⱼ ∈ R → [oⱼ ∈ Oⱼ] → τⱼ(oⱼ) ∈ Tri → Pⱼ → célula`

Tipos formales requeridos:
- `Wⱼ` — dominio observacional tipado.
- `ϕⱼ : Wⱼ → Oⱼ^⊥` — sensor o procedimiento de captura, con fallo explícito (⊥).
- `R = {ok, degradado, fallido, U}` — estados de admisibilidad observacional.
- `Ternarizer(τⱼ) : Oⱼ → Tri` — función de ternarización con partición documentada B₀/B₁/B_U.
- `E(C) ⊆ {1, …, n}` — máscara de exogeneidad.
- `ℐ(𝒜)` — interfaz paramétrica de la arquitectura.
- `U_s(𝒜)` — U silenciosa: lo que el sistema no parametriza, no conoce.

Invariante: si la captura falla o la admisibilidad no es suficiente, el resultado es U. La cadena no fabrica certeza.

### B.7. Dominio especializado, agente y consulta

- `𝔇 = (𝒫, ℐ, ℋ, Π_τ, Π_U, Π_C)` — dominio especializado sobre una arquitectura dada.
- `A_𝔇 = (𝒜, 𝔇, 𝒬)` — agente especializado tipado por dominio.
- `𝒬_ω(A_𝔇, sₙ) = (r, J, M)` — operador de consulta con firma `ω = (tipo, alcance, restricciones)`.

Condiciones de buena formación de la consulta: trazabilidad (CQ1), no opacidad (CQ2), no reescritura (CQ3), dependencia de cobertura (CQ4), consistencia con trayectoria (CQ5), gestión de U y criticidad (CQ6).

---

## 6. Bloque C — Interfaz analítica formal

Estas herramientas no entran en el núcleo ejecutor v0, pero el lenguaje debe proveer tipos e interfaces formales para ellas.

### C.1. Secuencias canónicas de trayectoria

Primitivas: `n₀(k)`, `n₁(k)`, `nU(k)`, `κ(k)`. Derivadas: `Γ(k)`, `Δn₀(k)`, `Δn₁(k)`, `ΔnU(k)`.

El análisis solo puede operar sobre secuencias extraídas canónicamente de una trayectoria, no sobre objetos semánticos sin mediación.

### C.2. Herramientas de análisis discreto

- Diferencia finita: `Δf(k) = f(k+1) − f(k)`.
- Transformada Z: `𝒵{f}(z) = Σₖ f(k)z⁻ᵏ`.
- Función generatriz: `G_C(x,y,z) = (x+y+z)ⁿ`.

### C.3. Herramientas de grafo

- Matriz de adyacencia `A_𝒜` del grafo de composición.
- Operador de propagación `P_𝒜 = Σₖ≥₀ A_𝒜ᵏ`.
- Profundidad efectiva `d_eff(𝒜)`.
- Influencia estructural `Influence(C, 𝒜) = Σᵢ (P_𝒜)ᵢⱼ`.

### C.4. Control de codificaciones

Toda codificación del vector ternario debe declarar si es inyectiva (sin pérdida) o con pérdida (proyección). Una codificación inyectiva preserva clasificación, criticidad y conteos. Una codificación con pérdida no puede tratarse como representación equivalente del sistema.

---

## 7. Bloque D — Zona prohibida o prematura

Lo que sigue está prohibido en el lenguaje v0, sea porque contradice los invariantes del corpus, sea porque el corpus no lo ha cerrado aún.

### D.1. Coerción implícita de U

Ninguna operación puede convertir U silenciosamente en 0, 1, false, null, NaN, -1 ni equivalente.

### D.2. Composición sin relación semántica previa

No se permite aplicar ningún operador de composición sin fijar antes la relación semántica entre los elementos implicados.

### D.3. Tratamiento de Comp como operador cerrado

`Comp` es constructor organizativo de Grado C. No tiene proposiciones fuertes ni firma cerrada. El lenguaje no puede prometer garantías formales que el corpus no ha demostrado.

### D.4. Garantías generales de convergencia de trayectorias

El corpus no define una norma general de convergencia o divergencia de trayectorias. El lenguaje no puede prometer esta propiedad.

### D.5. Ciclos de reevaluación controlados

El corpus prohíbe ciclos en el grafo de composición. Los ciclos de reevaluación temporal con garantía formal de terminación no están cerrados.

### D.6. Axiomática completa de transductores bien formados

El corpus fija la base algebraica de la transducción, pero no cierra su axiomática completa. El lenguaje no puede prometer completitud donde el corpus documenta apertura.

### D.7. Operador de entrenamiento en el núcleo

El operador de entrenamiento queda fuera del núcleo v0 como elemento abierto del corpus.

### D.8. Teoría multiagente plena

La composición entre agentes de dominios distintos se admite solo bajo relación semántica declarada e interfaces compatibles. Una teoría multiagente plena no está formalizada.

### D.9. Numerización con pérdida como equivalente semántico

Una codificación que colapse estados distintos (por ejemplo, tratar U como 0) no puede tratarse como representación legítima del sistema. Solo puede existir como proyección declarada.

### D.10. Delegación de decisiones de diseño a procesos automáticos

Ningún proceso automático (inteligencia artificial, optimizador, heurística) puede modificar la relación semántica previa R(𝒜), la topología del grafo de composición, los conectores, las tablas de admisibilidad, las políticas de indeterminación ni las políticas de transducción sin intervención humana declarada. El ser humano diseña; el sistema ejecuta.

---

## 8. Estrategia de compilación

El lenguaje se concibe como DSL semántica restringida con la siguiente ruta de compilación:

**Fase 1 — DSL canónica semántica.** Lenguaje declarativo, fuertemente restringido, que expresa los cuatro bloques de esta frontera normativa.

**Fase 2 — Representación intermedia canónica.** La DSL compila a una IR estable, auditable y suficiente para conformidad cruzada.

**Fase 3 — Backend principal en Rust.** Por su sistema de enums, exhaustividad de patrones y compilación a WASM.

**Fase 4 — Backend de referencia en Python.** Espejo ejecutable para verificación cruzada, no como núcleo doctrinal.

**Fase 5 — Salida opcional a WASM.** Solo cuando la semántica esté cerrada.

---

## 9. Criterio de legitimidad

La legitimidad del lenguaje no depende de su sintaxis ni de su backend. Depende de su capacidad para pasar la misma suite de conformidad cruzada que legitima cualquier implementación del Sistema Vectorial SV. Esta suite, derivada de la condición C8 del Consenso de Lenguajes, opera en cuatro niveles:

**Nivel A — Conformidad léxico-semántica.** El parser no acepta degradaciones ilícitas de Tri, objetos o operadores.

**Nivel B — Conformidad algebraica.** Casos canónicos para composición, compuerta, transducción, consulta, criticidad y reevaluación.

**Nivel C — Conformidad adversarial.** Intentos deliberados de romper invariantes: coerciones implícitas, mezcla de codominios, puentes incompatibles, cierre con U encubierta.

**Nivel D — Conformidad de trazabilidad.** Toda ejecución produce frame, evento, trayectoria y justificación coherentes entre backends.

---

## 10. Invariantes constitutivos que el lenguaje debe preservar

Estos invariantes se derivan directamente del corpus doctrinal y no pueden ser violados por ninguna construcción del lenguaje.

1. El alfabeto es {0, 1, U}. Nada más.
2. U es estado epistémico, no error.
3. n = b², b ≥ 3. Siempre.
4. T(n) = ⌊7n/9⌋. Sin excepciones.
5. La clasificación es determinista.
6. La composición depende de R: no se elige operador sin fijar relación.
7. Los operadores lícitos son los de ℱ_SV.
8. El polígono es inmutable una vez evaluado.
9. La trayectoria es append-only.
10. La inteligencia artificial no es soberana. El humano diseña, el sistema ejecuta.
11. Cero estadística, cero probabilidad opaca como fundamento primario de decisión.
12. Todo debe ser auditable.

---

## Referencias

[R1] Juan Antonio Lloret Egea. *Fundamentos algebraico-semánticos del Sistema Vectorial SV.* v1.0.0, Release 3. ITVIA, 2026. https://www.itvia.online/pub/fundamentos-algebraico-semanticos-del-sistema-vectorial-sv/release/3

[R2] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — I. Transmisión en serie por parámetro puente.* v1, Release 4. ITVIA, 2026. https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv/release/4

[R3] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — II. Gramática general de composición.* v1.0, Release 1. ITVIA, 2026. https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv--ii-gramatica-general-de-composicion/release/1

[R4] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — III. Horizonte de sucesos y reevaluación discreta.* v1, Release 1. ITVIA, 2026. https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv--iii-horizonte-de-sucesos-y-reevaluacion-discreta/release/1

[R5] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — IV. Transducción al alfabeto ternario e interfaz paramétrica del sistema.* v1, Release 1. ITVIA, 2026. https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv--iv-transduccion-al-alfabeto-ternario-e-interfaz-parametrica-del-sistema/release/1

[R6] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — V. Invariantes, agentes especializados y operador de consulta del sistema.* v2, Release 2. ITVIA, 2026. https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv--v-invariantes-agentes-especializados-y-operador-de-consulta-del-sistema/release/2

[R7] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — VI. Análisis discreto, representaciones y herramientas de secuencias del sistema.* v1, Release 1. ITVIA, 2026. https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv--vi-analisis-discreto-representaciones-y-herramientas-de-secuencias-del-sistema/release/1

---

*Documento normativo del lenguaje de computación del Sistema Vectorial SV. ISSN 2695-6411.*  
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0*
