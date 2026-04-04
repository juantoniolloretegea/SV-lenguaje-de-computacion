# Manifiesto de arquitectura, derechos, obligaciones, garantías y fundamentos del Sistema Vectorial SV

**Fecha y Versión: V.1 del conjunto**  
**Fecha:** 4 de abril de 2026  
**Versión del conjunto:** V.1 del conjunto  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Titularidad y autoría:** © Juan Antonio Lloret Egea, 2026. Este conjunto se distribuye con atribución explícita de autoría y bajo la licencia indicada, sin autorización para apropiación de la paternidad intelectual del Sistema Vectorial SV.  

---


**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Fecha:** 4 de abril de 2026  
**Estado:** documento de arquitectura y garantías para lectura transversal del ecosistema SV.

---

## 1. Estatuto y límites



Este documento fija un marco transversal de arquitectura, garantías, derechos y obligaciones para el ecosistema del Sistema Vectorial SV. No sustituye al corpus doctrinal, no introduce una nueva ontología del sistema y no desplaza al Lenguaje SV ni a sus validadores. Su función es ordenar, hacer explícitas las garantías que el ecosistema ya exige y disciplinar la entrada de capas estadísticas, extractores, modelos de visión, actores conversacionales y servicios auxiliares.

El documento no autoriza producción clínica, ni producción decisoria soberana, ni cierre automático de `U`. Su alcance es de gobierno arquitectónico, garantía de integridad y trazabilidad de capas auxiliares.

---

## 1.bis. Apoyo metodológico declarado

El presente manifiesto reconoce como apoyo metodológico explícito la tradición del **método** expuesta por **René Descartes**, especialmente la exigencia de claridad, distinción, descomposición analítica, orden de exposición y rechazo del asentimiento precipitado. Este apoyo es **metodológico** y no soberano: no desplaza la doctrina algebraico-semántica del SV ni introduce por sí mismo definiciones nuevas del sistema. Su función aquí es disciplinar la exposición, la prueba, la resistencia a la confusión de planos y la obligación de justificar paso a paso toda afirmación relevante.

## 2. Título algebraico mínimo del sistema

El ecosistema SV se rige por estos invariantes:

- alfabeto ternario canónico: `Σ = {0, 1, U}`;
- tamaño de célula: `n = b²`, `b ≥ 3`;
- umbral fuerte de clasificación: `T(n) = ⌊7n/9⌋`;
- clasificación en `K₃ = {APTO, INDETERMINADO, NO_APTO}`;
- compuerta conservadora posición a posición: `gate_value : Σ × Σ → Σ`;
- compuerta vectorial: `gate_vector : Σⁿ × Σⁿ → Σⁿ`;
- preservación estricta de `U` como indeterminación honesta;
- prohibición de fabricar certeza por plausibilidad, por coste, por convergencia estadística o por cierre favorable de conveniencia.

La clasificación fuerte de una célula `v ∈ Σⁿ` se resume así:

- `NO_APTO` si `N₁(v) ≥ T(n)`;
- `APTO` si `N₀(v) ≥ T(n)`;
- `INDETERMINADO` en cualquier otro caso.

En ningún punto del ecosistema se admite reemplazar este régimen por probabilidad, heurística no declarada, minería de datos, tiempo soberano o inferencia opaca.

---

## 3. Arquitectura de capas y subordinación

La cadena de prevalencia obligatoria del ecosistema es:

**SV soberano → doctrina → álgebra → lenguaje de computación → compuertas de seguridad → agentes especializados → motor después.**

Por tanto:

- `SV-matematica-semantica` conserva la autoridad doctrinal superior;
- `SV-lenguaje-de-computacion` conserva el contrato técnico del lenguaje, la IR y la bienformación;
- `SVcustos-dataset` y `SVperitus-dataset` son sedes subordinadas de observación, intrusión, agentes y artefactos;
- `SV-banco-de-idiomas` es infraestructura lingüística auxiliar;
- `SV-motor` es un frente de desarrollo local y aplicado, nunca una sede soberana del sistema.

---

## 4. Derechos y obligaciones

### 4.1 Derechos del experto humano

El experto humano tiene derecho a:

1. ver la estructura, no solo la salida final;
2. conocer cuándo una posición permanece en `U` y por qué;
3. rechazar la clausura fingida;
4. corregir soberanamente el sistema y conservar esa corrección como nuevo suceso;
5. operar bajo perfiles diferenciados de permisos.

### 4.2 Obligaciones del experto humano

El experto humano debe:

1. declarar el dominio y el alcance de la sesión cuando proceda;
2. no tratar una capa estadística como si fuera sede de verdad;
3. no exigir producción donde el propio ecosistema la prohíbe;
4. mantener separadas la prueba de laboratorio y la intervención de producción.

### 4.3 Obligaciones de toda capa auxiliar de IA

Toda capa auxiliar de IA, LLM, CNN o extractor debe:

1. producir observables o proxies declarados, no decisiones soberanas;
2. respetar el dominio y el codominio declarados;
3. emitir `U` cuando falte base suficiente;
4. no asignar por sí sola `κ₃`, ni prescribir, ni suplantar al agente especializado o al experto;
5. exponer su actor, su licencia, su alcance y sus límites de uso.

---

## 5. Actores, permisos y no producción

El ecosistema puede declarar actores auxiliares de extracción o de traducción, pero todos ellos quedan sometidos a las compuertas del sistema. Ningún actor estadístico entra en producción soberana.

Se fijan tres perfiles mínimos de permisos, compatibles con la casa del SV:

- **Administrador**: modifica parámetros, contratos, actores, umbrales auxiliares y configuración estructural;
- **Privilegios medios**: ejecuta laboratorios, revisa salidas, serializa pruebas, pero no altera la arquitectura base;
- **Uso estándar**: consulta, ejecuta recorridos permitidos y ve salidas legibles, sin alterar contratos ni parámetros.

Mientras el bloque documental pendiente no cierre completamente el álgebra y la semántica necesarias para blindar desviaciones, doblegamiento del SV o basura semántica por la puerta trasera, todo el frente motor IA queda en **régimen exclusivo de laboratorio**. Esto implica:

- ejecución completa y prueba amplia de laboratorio: sí;
- despliegue de producción soberana: no;
- cierre clínico autónomo: no;
- sustitución del experto humano: no.

---

## 6. Laboratorios vinculados a este manifiesto

La presente pieza se acompaña de laboratorios de verificación independientes, organizados para comprobar cuatro frentes:

1. invariantes algebraicos del motor y del alfabeto ternario;
2. ternarización explicitable de dominios inmunológicos ya existentes;
3. composición intercelular y transmisión honesta de incertidumbre;
4. contratos mínimos de actores y extractores para no fabricar certeza ni corromper el codominio del SV.

Los laboratorios no sustituyen al corpus. Sirven para hacer verificable, en lenguaje ejecutable, qué sí puede afirmarse y qué no.

---

## 7. Mapa mínimo de repositorios y función

- `SV-matematica-semantica`: sede doctrinal superior y base de prevalencia.
- `SV-lenguaje-de-computacion`: sede operativa y técnica del Lenguaje SV.
- `SVcustos-dataset`: origen observacional, datasets e intrusión.
- `SVperitus-dataset`: agentes especializados, fases y artefactos.
- `SV-banco-de-idiomas`: infraestructura lingüística auxiliar.
- `SV-motor`: frente motor local, reproducible y subordinado.

La arquitectura del sistema exige que cada sede exponga su función propia sin suplantar la de otra. Por eso este manifiesto debe quedar visible en todas las sedes, pero **no** sustituye el README propio de ninguna: lo complementa, lo disciplina y fija el marco común de garantías.

---

## 8. Referencias seleccionadas

Referencias de contexto comparado y crítica arquitectónica citadas o asumidas como soporte de prudencia:

- Schmidt C. *MD Anderson Breaks With IBM Watson*. J Natl Cancer Inst. 2017;109(5):djx113.
- Wong A, et al. *External Validation of a Widely Implemented Proprietary Sepsis Prediction Model in Hospitalized Patients*. JAMA Intern Med. 2021;181(8):1065–1070.
- Obermeyer Z, et al. *Dissecting racial bias in an algorithm used to manage the health of populations*. Science. 2019;366(6464):447–453.
- Cabitza F, Rasoini R, Gensini GF. *Unintended Consequences of Machine Learning in Medicine*. JAMA. 2017;318(6):517–518.
- Rudin C. *Stop explaining black box machine learning models for high stakes decisions and use interpretable models instead*. Nat Mach Intell. 2019;1:206–215.

Estas referencias no desplazan la semántica propia del SV. Se usan para delimitar riesgos reales del ecosistema de IA y justificar, por contraste, la disciplina fuerte del marco SV.
