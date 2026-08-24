# Cierre integral de R0 — primera realización soberana del Lenguaje de Computación SV

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Fecha:** 24 de agosto de 2026  
**Estado:** R0 cerrado  
**Repositorio:** `SV-lenguaje-de-computacion`

## 1. Objeto

Este documento registra el cierre integral de R0 como primera realización soberana del núcleo semántico del Lenguaje de Computación SV, dentro del alcance fijado para esta fase.

R0 materializa la semántica del lenguaje en Rust, establece su correspondencia con la referencia diferencial y acredita la ejecución del mismo núcleo mediante destino nativo y WebAssembly de navegador. El cierre de R0 no constituye una certificación del sistema completo ni acredita las garantías materiales reservadas para fases posteriores.

El corte técnico sometido al cierre parte de `main = 46ec676dbffb71465c2439255d2ddceea48fd931` y de las integraciones que materializaron R0-0…R0-8, la comprobación WebAssembly, el entorno público y la reconciliación normativa de C01–C03.

## 2. Realización cerrada

La realización de R0 mantiene una única fuente semántica en `sv_core`:

```text
texto .svp
   ↓
sv_core::compile_svp
   ├── Rust nativo
   └── WebAssembly de navegador
```

El adaptador WebAssembly no incorpora una segunda gramática, un segundo analizador sintáctico ni reglas semánticas independientes. La representación soberana Rust conserva la estructura de IR 0.3 y aplica las reglas de bienformación antes de exponer un programa constituido.

La implementación Python permanece como referencia diferencial y conserva el serializador canónico y el catálogo diagnóstico de referencia; no constituye una dependencia del ejecutable Rust.

## 3. Propiedades constitutivas materializadas

Dentro del alcance de R0 quedan materializadas:

- el tipo ternario irreductible `Tri = {0,1,U}` y el rechazo de valores ajenos;
- C01: separación entre fallo técnico, admisibilidad y `Tri.U`;
- C02: resolución dirigida a una ocurrencia constituida e identificable de `U`, con separación entre revisión y clausura;
- C03: coherencia estructural y causal de `Frame` mediante una única frontera de constitución;
- la autoridad semántica de `sv_core` para constituir la representación intermedia soberana;
- la correspondencia de IR 0.3 con la representación Rust;
- la equivalencia ejercida sobre el corpus de conformidad comprometido;
- la ejecución nativa y WebAssembly sobre el mismo núcleo;
- una medición basal reproducible anterior a cualquier optimización dirigida por rendimiento;
- la separación entre distribución web y autoridad semántica.

La Frontera Normativa v0 se conserva como antecedente. En las cláusulas de admisibilidad, resolución y `Frame` materialmente superadas, su lectura vigente queda determinada por la adenda técnica de C01–C03, la Gramática 0.2 y la IR 0.3.

## 4. Conformidad y paridad

El corpus comprometido contiene:

```text
11 casos válidos
61 casos inválidos
72 casos en total
```

La comprobación ejercida acredita:

```text
mismo texto .svp
+ aceptación/rechazo alineado en Python · Rust nativo · WebAssembly
+ observable textual de proyección idéntico Rust nativo ↔ WebAssembly
```

No se acredita identidad textual bit a bit entre la salida Python y `equivalence_json`.

Tampoco se acredita paridad diagnóstica exacta de códigos `E***` ni igualdad textual de mensajes de error.

La referencia Python coincide con los oráculos canónicos comprometidos en los casos válidos. Rust nativo y WebAssembly reciben el mismo texto `.svp` y no consumen como entrada una IR o un JSON preconstituidos por Python.

## 5. WebAssembly y entorno público

El destino WebAssembly de navegador utiliza el mismo `sv_core` que la realización nativa.

El artefacto desplegado queda identificado por:

```text
sv_wasm.wasm
bytes   = 337366
SHA-256 = 7b49228624f101dc8d863a2b4d631b7ed8eacb4ee4a29c2459d32f6b63aff5dc
```

El entorno público principal se encuentra en:

```text
https://lenguaje-sv.itvia.online/
```

La distribución web entrega los activos necesarios para ejecutar localmente el módulo WebAssembly en el navegador. No constituye una fuente semántica alternativa ni altera el estatuto de `sv_core`.

El Playground anterior basado en Python/Pyodide se conserva como antecedente histórico y referencia de trazabilidad.

## 6. Medición basal

R0-8 fijó una referencia reproducible del camino nativo `.svp → sv_native → sv_core → salida observable`, incluyendo tiempo extremo a extremo del proceso, memoria residente máxima, CPU por lotes, tamaño y huella del binario, estabilidad de salida y ejes de escala controlados.

Las cifras permanecen ligadas al binario, perfil de ejecución, plataforma, herramienta de compilación, protocolo y corpus registrados. No se interpretan como leyes asintóticas ni como magnitudes universales del Lenguaje SV.

Tras la centralización posterior de la proyección diferencial, el binario nativo aumentó 872 bytes respecto de la referencia histórica de R0-8. La batería de R0-8 se volvió a ejecutar sin observar una regresión material.

## 7. Límites expresos del cierre

El cierre de R0 no acredita:

- Garantía I;
- Garantía II;
- seguridad integral de implantación;
- autonomía completa de distribución;
- cadena de suministro completa;
- persistencia, recuperación o continuidad finales;
- mediación final de identidad y autoridad de implantación;
- forensia completa;
- perfiles críticos;
- compatibilidad universal con todos los motores de navegador;
- paridad diagnóstica exacta `E***`;
- serializador canónico Rust completo;
- API Rust pública de alto nivel;
- realización de `ConflictOperator` / J2.3;
- cierre de `FFL-D`.

Permanecen además registradas las deudas relativas a la divergencia histórica de `E204`, `RG1`, `C-1.C` y las demás limitaciones expresamente documentadas en la calidad del proyecto.

Ninguna de estas deudas se interpreta como satisfecha por R0 ni se convierte en un cuarto estado del Lenguaje o en `Tri.U`.

## 8. Estado resultante

El cierre integral produce el siguiente estado:

```text
R0  = CERRADO
R1  = NO INICIADO
R2  = NO INICIADO
R3  = NO INICIADO
R4  = NO INICIADO

Garantía I  = NO_PROBADO
Garantía II = NO_PROBADO

Sec.6 = ABIERTA
```

El cierre de R0 no abre automáticamente ninguna fase posterior. La apertura de R1 requiere una decisión separada, con objeto, alcance y condiciones de cierre propios.

## 9. Trazabilidad principal

La realización y el cierre se apoyan, entre otras, en las siguientes integraciones públicas:

- PR #7 — materialización de C01–C03 en la etapa frontal de referencia;
- PR #9 — inicio de la realización Rust con destino WebAssembly compartido;
- PR #13–#17 — materialización incremental de `Frame`, C01–C03 y correspondencia IR 0.3;
- PR #20 — equivalencia soberana del corpus canónico;
- PR #21 — medición basal nativa reproducible;
- PR #22 — WebAssembly de navegador y paridad ejecutada de tres vías;
- PR #23 — entorno público Rust/WebAssembly y conservación histórica del Playground Python;
- PR #24 — reconciliación normativa de C01–C03 frente a la Frontera Normativa v0.

La evidencia detallada de cada hito permanece en las respectivas solicitudes de incorporación, flujos de integración continua, registros de calidad y artefactos reproducibles del repositorio.

## 10. Decisión de cierre

No queda identificada una obligación constitutiva de R0 que haya sido desplazada indebidamente a R1–R4. Las propiedades cuya ausencia afectaría a la identidad semántica de la primera realización soberana han sido materializadas dentro del radio de R0; las obligaciones restantes corresponden a fases materiales posteriores o a deuda explícitamente registrada.

Por tanto, con fecha 24 de agosto de 2026, **R0 queda cerrado** dentro del alcance definido en este documento.
