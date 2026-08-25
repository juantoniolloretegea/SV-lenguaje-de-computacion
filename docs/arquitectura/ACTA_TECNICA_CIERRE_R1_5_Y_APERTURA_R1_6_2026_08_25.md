# Cierre de R1-5 y apertura de R1-6 — regresión integral y cierre de fase

**Fecha:** 25 de agosto de 2026  
**Repositorio:** `SV-lenguaje-de-computacion`  
**Estado:** R1-5 cerrado e integrado · R1-6 abierto  
**R1:** abierto  
**R2–R4:** no iniciados  
**Garantía I:** `NO_PROBADO`  
**Garantía II:** `NO_PROBADO`

## 1. Objeto

Este documento registra el cierre de R1-5, dedicado a la ligadura decisión–efecto y a la traza determinista, y abre R1-6 como último corte interno de R1.

R1-6 no introduce una semántica nueva ni amplía la productividad de las clases de transición. Su función es someter la realización integrada de R1-0 a R1-5 a regresión conjunta, contraste negativo y comprobación de los criterios de cierre establecidos al abrir la fase R1.

## 2. Corte integrado de R1-5

La realización cerrada de R1-5 fue verificada sobre el candidato:

```text
1bdb24f205c21ed61898d72b4444553c48156103
```

que quedó integrado en:

```text
main = 95a67b3f5ee3056477d5e34a17bcad201aaab9e5
```

El corte conserva como superficie pública conforme la secuencia:

```text
decide_permit_traced
→ mediate_traced_permit
→ execute_traced_mediated
```

La decisión protegida queda ligada al `EffectDescriptor` completo, a la forma, la autoridad, el contexto y el conjunto de obligaciones aplicables. La traza conserva las comprobaciones individuales, el resultado resuelto de cada obligación, su cualificación por cobertura, el resultado agregado y la disposición de permiso o bloqueo.

Cuando existe ejecución, la relación causal utiliza el `ExerciseRef` real constituido por la vía gobernada de R1-4 y mantiene la secuencia:

```text
DispatchCommitted
→ Confirmed | Indeterminate
```

sin convertir la traza en autoridad, permiso ni capacidad de ejecución.

## 3. Evidencia reproducible de R1-5

El candidato integrado superó:

```text
Conformidad SVP              #194 = success
R0 Rust                      #159 = success
R0-8 Baseline nativa         #111 = success
R0 WASM paridad tres vías    #107 = success
```

con los recuentos:

```text
sv_core   = 198/198
sv_wasm   = 2/2
doc-tests = 17/17
R0-7      = 11/11 casos válidos equivalentes
            + 61/61 casos inválidos rechazados
```

La batería incluye pruebas integradas desde una génesis T-0 real para:

- decisión D-A, mediación y ejercicio confirmado;
- bloqueo por D-R sin mediación ni ejercicio;
- degradación de una acreditación resuelta D-A a D-N cuando la cobertura es insuficiente;
- conservación de resultados individuales incompatibles antes de una resolución D-N;
- inaccesibilidad pública de las vías productivas sin traza;
- separación entre traza, autoridad, permiso, capacidad de ejecución y `Tri`.

## 4. Deuda no bloqueante heredada

R1-5 deja dos endurecimientos defensivos que no abren una vía productiva ordinaria:

1. `append_decision` puede comprobar una eventual colisión de `DecisionTraceRef` antes de insertar. La referencia productiva procede de un ordinal privado, monotónico y no acotado al ancho de palabra de máquina, por lo que no existe una vía pública reproducible para provocar la colisión en el estado integrado.
2. `TraceLinkConflict` conserva una defensa frente a una asociación incompatible de un mismo `ExerciseRef`. La unicidad de ejercicio cerrada en R1-4 impide que esta condición sea una vía ordinaria de producción de efecto.

Estas observaciones podrán endurecerse si resulta conveniente durante R1-6, pero no alteran el cierre de R1-5.

## 5. Fundamento de R1-6

La apertura original de R1 descompuso la fase del modo siguiente:

```text
R1-0  contrato de realización y tipos cerrados de control
R1-1  formas constituidas, autoridad, E_max y D_a
R1-2  transiciones T-* y restricción de T-0 por continuidad
R1-3  Req, aplicabilidad y resultados D-A/D-R/D-N
R1-4  fallo cerrado y mediación del efecto protegido
R1-5  ligadura decisión–efecto y traza determinista
R1-6  regresión, contraste adversarial y cierre de fase
```

Por tanto, R1-6 es un corte de integración y clausura. No debe utilizarse para trasladar a R1 propiedades reservadas a R2 o a fases posteriores.

## 6. Matriz obligatoria de cierre de R1

R1-6 deberá aportar evidencia reproducible para los diez criterios de cierre ya establecidos para R1.

### 6.1. No fabricación de autoridad

Debe mantenerse que una vía ordinaria no puede fabricar autoridad desde:

```text
información
+ evidencia
+ resultado técnico
+ traza
+ capacidad de adaptación o ejecución
```

### 6.2. Constitución de autoridad

La autoridad nueva sólo podrá aparecer por una transición autorizante válida dentro del modelo representado. T-I, T-V, T-H y T-E no constituirán autoridad.

Durante R1-6 se mantiene:

```text
T-G / T-C / T-R = NO PRODUCTIVAS
```

Su productividad no es condición de cierre de R1.

### 6.3. Unicidad de T-0 por continuidad

Una continuidad lógica ya habitada deberá seguir rechazando una segunda T-0, sin que identidad de proceso, reinicio técnico o identificador externo reabran la génesis dentro del modelo intra-proceso de R1.

### 6.4. Req no vacío en formas sujetas a control

Deberá conservarse:

```text
Req(F,e | C) = ∅
⇒ forma inválida para producir el efecto
```

### 6.5. Fallo cerrado por D-R y D-N

Deberá mantenerse:

```text
algún D-R aplicable ⇒ efecto bloqueado
algún D-N aplicable ⇒ efecto bloqueado
```

sin promoción por mayoría, selección favorable, ausencia de cobertura o reutilización nominal de resultados.

### 6.6. Separación respecto de Tri

Deberá conservarse:

```text
D-N ≠ Tri.U
fallo técnico ≠ Tri.U
fallo estructural ≠ Tri.U
```

### 6.7. Ligadura exacta del permiso

Un permiso deberá seguir ligado al objeto material de la decisión y no podrá reutilizarse fuera de sus ligaduras de forma, autoridad, efecto, contexto y obligaciones aplicables.

### 6.8. Mediación intra-proceso

El efecto protegido representado por R1 no deberá disponer de una vía pública ordinaria que eluda el punto de decisión y la mediación gobernados dentro de `sv_core`.

Esta comprobación no se extenderá a vías administrativas, de plataforma, depuración, almacenamiento o hardware externas al perímetro de R1.

### 6.9. Trazabilidad de resultados

La traza deberá seguir distinguiendo:

```text
D-A
D-R
D-N
```

con las comprobaciones individuales y las ligaduras necesarias para reconstruir el resultado gobernado sin inferencia heurística.

### 6.10. Ausencia de regresión respecto de R0

La integración completa de R1 no deberá introducir una regresión semántica atribuible a R1 en:

- batería de `sv_core`;
- frontera `sv_wasm`;
- pruebas de documentación y cierres de tipos;
- equivalencia R0-7;
- baseline nativa R0-8;
- paridad nativa/WASI/navegador WebAssembly;
- gramática e IR ya cerradas.

## 7. Batería mínima de R1-6

La regresión integral deberá cubrir, como mínimo, estas familias:

1. constitución inicial válida y rechazo atómico de génesis inválidas;
2. segunda T-0 imposible en continuidad habitada;
3. alcance exacto de autoridad, `E_max` y `D_a`;
4. formas controladas sin `Req` rechazadas;
5. aplicabilidad de verificadores, conflicto y cobertura;
6. reutilización ligada y rechazo por cambio material;
7. D-A completo como única vía de permiso positivo;
8. bloqueo por D-R y D-N;
9. mediación exacta del efecto permitido;
10. contratos de acumulación y ejercicio de R1-4;
11. cadena trazada de R1-5 y bloqueo de vías crudas públicas;
12. T-G, T-C y T-R no productivas;
13. `D-N`, fallos técnicos y fallos estructurales fuera de `Tri.U`;
14. ausencia de reloj ambiental, orden temporal implícito y dependencias semánticas externas;
15. conservación de las pruebas R0 en nativo y WebAssembly.

R1-6 podrá añadir pruebas de regresión o endurecimientos defensivos cuando descubra una vía real o una laguna de cobertura. No deberá introducir cambios de arquitectura para obtener un cierre meramente documental.

## 8. Criterio de adversarial final de R1

Antes del cierre de R1 se requerirá un contraste adversarial sobre el candidato exacto de R1-6 que intente romper conjuntamente los diez criterios de la sección 6.

Una incidencia sólo será bloqueante si existe una ruptura reproducible de una propiedad incluida en R1. Las propiedades reservadas expresamente a R2 o a fases posteriores no podrán utilizarse para atribuir falsamente una carencia a R1.

## 9. Límites de la fase

R1-6 no acredita ni abre:

- persistencia autoritativa durable;
- recuperación después de reinicio;
- unicidad global entre réplicas o bifurcaciones;
- consumo único resistente a restauración o carrera concurrente;
- límites materiales de recursos;
- aislamiento de sistema operativo, hipervisor o hardware;
- raíz material de confianza;
- cadena de construcción y suministro completa;
- firma o identidad externa;
- mediación completa de todas las vías materiales;
- forensia independiente completa;
- `BudgetΣ`;
- IA-SEC;
- Garantía I;
- Garantía II.

Estas propiedades permanecen fuera de R1 y no se presumirán por el cierre futuro de la fase.

## 10. Estado resultante

```text
R0   = CERRADO
R1   = ABIERTO
R1-5 = CERRADO · INTEGRADO
R1-6 = ABIERTO

T-E = PRODUCTIVA POR VÍA GOBERNADA
T-G / T-C / T-R = NO PRODUCTIVAS

R2 / R3 / R4 = NO INICIADOS
BudgetΣ / IA-SEC = NO ABIERTOS
Garantía I / II = NO_PROBADO
```

R1 sólo podrá cerrarse después de que R1-6 demuestre conjuntamente los diez criterios de cierre y supere la regresión nativa/WebAssembly y el contraste adversarial del candidato exacto.
