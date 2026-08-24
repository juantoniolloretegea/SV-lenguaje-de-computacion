# Apertura de R1 — autoridad, mediación y decisiones protegidas

**Fecha:** 24 de agosto de 2026  
**Repositorio:** `SV-lenguaje-de-computacion`  
**Estado:** R1 abierto  
**R0:** cerrado  
**R2–R4:** no iniciados  
**Garantía I:** `NO_PROBADO`  
**Garantía II:** `NO_PROBADO`

## 1. Objeto

R1 materializa, dentro del proceso soberano, las condiciones de autoridad, mediación y fallo cerrado necesarias para que una decisión protegida sólo pueda producir un efecto cuando la forma, la autoridad, las obligaciones aplicables y sus ligaduras hayan sido acreditadas conforme a los contratos SEC.0.

La fase se abre después del cierre integral de R0 y no modifica el alcance ya cerrado de la semántica del Lenguaje SV. Su objeto es añadir la frontera de control que separa una representación semánticamente válida de la facultad legítima de producir un efecto protegido.

## 2. Base contractual

R1 se interpreta conjuntamente con:

- `ESPECIFICACION_ARQUITECTONICA_ENTORNO_EJECUCION_SOBERANO_SV_V0.md`;
- `docs/arquitectura/ADENDA_DE_ALCANCE_TEMPORAL_A_ESPECIFICACION_ENTORNO_SOBERANO_SV_V0_2026_08_22.md`;
- `docs/arquitectura/CONTRATO_ABSTRACTO_DE_AUTORIDAD_CONSTITUCION_Y_GENESIS_SEC0_A_V2_2026_08_21.md`;
- `docs/arquitectura/ADENDA_CORRECTIVA_SEC0_A_UNIDAD_DE_GENESIS_Y_CONTINUIDAD_AUTORITATIVA_2026_08_22.md`;
- `docs/arquitectura/CONTRATO_ABSTRACTO_DE_DIAGNOSTICO_Y_FALLO_CERRADO_SEC0_D_2026_08_21.md`;
- los contratos SEC.0-M, SEC.0-X y SEC.0-T en lo que delimiten propiedades que R1 no puede atribuirse por sí solo.

La secuencia arquitectónica vigente define R1 como la fase de «autoridad, mediación y decisiones protegidas»: formas constituidas, T-0 restringida a la génesis inicial de la continuidad autoritativa, autoridad aplicable, `Req`, fallo cerrado, ligaduras y trazas.

## 3. Punto de partida e identidad inicial del SUT

R1 parte del repositorio integrado en:

```text
main = 6f4bb0f3dbcdfe3dbb71f4e27ddf670db68bf99d
```

El SUT inicial de esta fase queda delimitado a:

```text
sv_core
+ fronteras intra-proceso necesarias para autoridad y decisión protegida
+ adaptadores nativo/WebAssembly en cuanto consumidores del mismo sv_core
```

Los adaptadores no adquieren autoridad por invocar al núcleo y no podrán constituir unilateralmente formas, autoridad, resultados de comprobación o permisos de efecto.

R1 no amplía la identidad probatoria de R0 a propiedades materiales que dependan de componentes externos al proceso.

## 4. Propiedades contractuales que se materializan

R1 deberá materializar conjuntamente las propiedades siguientes.

### 4.1. Distinciones de autoridad

Se conservarán como clases no intercambiables:

```text
información
≠ evidencia admitida
≠ hecho semántico constituido
≠ autoridad
≠ habilitación
≠ ejercicio
```

Ninguna conversión ordinaria podrá promover información, evidencia, capacidad técnica o ejecución material a autoridad.

### 4.2. Formas constituidas

Toda forma capaz de producir un efecto protegido deberá derivar de un descriptor previamente constituido que fije, al menos:

- clase T-*;
- familia de efectos;
- ligaduras de contexto;
- autoridad previa necesaria;
- regla de acumulación cuando corresponda.

La clase de una transición no podrá elegirse durante el acto por el ejecutor o por el componente interesado en su aceptación.

### 4.3. Autoridad y envolvente de efectos

La realización deberá representar una autoridad de forma que su ejercicio no pueda exceder la envolvente constituida `E_max(a | C)` ni el dominio gobernado `D_a` cuando éste sea exigible.

La habilitación podrá estrechar lo ejercitable, pero no ampliar la autoridad:

```text
Enabled(a, I) ⊆ E_max(a | C)
```

### 4.4. Génesis y transiciones de autoridad

Las únicas vías abstractas por las que puede llegar a existir autoridad conservarán la clasificación contractual:

```text
T-0 | T-C | T-G | T-R
```

T-I, T-V, T-H y T-E no constituirán autoridad.

T-0 quedará restringida al primer estado legítimo de autoridad de una continuidad autoritativa aún no habitada:

```text
continuidad ya habitada ⇒ T0_disponible = false
```

La identidad de proceso, réplica, reinicio, contenedor, máquina virtual o identificador técnico no podrá reabrir T-0.

R1 materializará esta regla en el estado lógico intra-proceso. La persistencia material, la selección de una continuidad vigente entre bifurcaciones y la recuperación durable corresponden a R2 o a fases posteriores.

### 4.5. Requisitos aplicables

Para toda forma sujeta a control, efecto `e` y contexto `C`, se conservará:

```text
Req(F,e | C) = N(F,e | C) ∪ S(F,e | C)
```

con:

```text
Req(F,e | C) = ∅ ⇒ forma inválida para producir el efecto
```

La aplicabilidad de una obligación no podrá eliminarse localmente durante la ejecución para obtener permiso.

### 4.6. Resultados técnicos de comprobación

Se materializarán exactamente los tres resultados abstractos:

```text
D-A = ACREDITADO
D-R = REFUTADO
D-N = NO_VERIFICABLE
```

con las separaciones obligatorias:

```text
D-N ≠ Tri.U
D-N ≠ D-R
D-N ≠ D-A
fallo técnico ≠ Tri.U
```

### 4.7. Fallo cerrado

Una forma sujeta a control sólo podrá producir su efecto cuando todas las obligaciones aplicables estén acreditadas:

```text
Permit(F,e | C) ⇔
    Req(F,e | C) ≠ ∅
    ∧ ∀q ∈ Req(F,e | C), Check(q) = D-A
```

Por tanto:

```text
algún D-R ⇒ efecto bloqueado
algún D-N ⇒ efecto bloqueado
```

La diferencia entre `D-R` y `D-N` deberá conservarse en el resultado y en la traza.

### 4.8. Ligadura decisión–efecto

El permiso deberá quedar ligado al menos a la forma, al efecto, al contexto, a la autoridad y al conjunto de obligaciones cuya comprobación lo sustenta.

Un permiso obtenido para un objeto, efecto o contexto no podrá reutilizarse como autorización genérica para otro distinto.

La persistencia temporal de permisos, la revocación durable y la continuidad después de reinicio pertenecen a R2; R1 sólo debe impedir la reutilización indebida dentro de la identidad intra-proceso ejercida.

### 4.9. Mediación intra-proceso

Dentro del perímetro de `sv_core`, el efecto protegido materializado para R1 no deberá disponer de una vía ordinaria que eluda el punto de decisión gobernado.

Esta propiedad es deliberadamente intra-proceso. No constituye mediación completa del sistema material ni excluye por sí sola vías administrativas, de plataforma, depuración, carga o almacenamiento externas al proceso.

### 4.10. Traza suficiente

Cada decisión protegida deberá conservar una traza determinista suficiente para distinguir, al menos:

- forma;
- efecto pretendido;
- contexto relevante;
- autoridad invocada;
- obligaciones aplicables;
- resultado individual de cada comprobación;
- resultado agregado;
- permiso o bloqueo;
- efecto comprometido, cuando exista.

La traza producida por el propio SUT no se presenta como evidencia independiente de Garantía I o II.

## 5. Capacidades y dependencias incluidas

R1 incluye:

- tipos y constructores internos necesarios para formas y autoridad;
- clasificación T-* necesaria para el control intra-proceso;
- representación lógica de continuidad no habitada/habitada para restringir T-0;
- definición y ligadura de `Req`;
- resultados `D-A/D-R/D-N`;
- agregación de comprobaciones;
- emisión de permiso sólo bajo fallo cerrado;
- mediación del efecto protegido dentro de `sv_core`;
- traza determinista de decisión y efecto;
- pruebas negativas que ataquen fabricación de autoridad, segunda génesis, requisitos vacíos, omisión de obligaciones, `D-N` permisivo y reutilización fuera de ligadura.

## 6. Propiedades excluidas de R1

R1 no materializa ni acredita:

- almacenamiento autoritativo durable;
- recuperación material después de reinicio;
- unicidad global de continuidad entre réplicas o bifurcaciones;
- consumo único resistente a restauración o carrera concurrente;
- colas, límites materiales de recursos o disponibilidad;
- aislamiento de sistema operativo, hipervisor o hardware;
- raíz material de confianza;
- cadena de construcción y suministro completa;
- identidad externa, criptografía o firma humana;
- mediación completa de todas las vías materiales del sistema;
- forensia independiente completa;
- Garantía I;
- Garantía II;
- apertura de R2, R3 o R4.

Estas propiedades permanecen `NO_PROBADO` dentro de sus ámbitos correspondientes.

## 7. Descomposición de R1

La fase se ordena en los siguientes cortes internos:

```text
R1-0  contrato de realización y tipos cerrados de control
R1-1  formas constituidas, autoridad, E_max y D_a
R1-2  transiciones T-* y restricción de T-0 por continuidad
R1-3  Req, aplicabilidad y resultados D-A/D-R/D-N
R1-4  fallo cerrado y mediación del efecto protegido
R1-5  ligadura decisión–efecto y traza determinista
R1-6  regresión, contraste adversarial y cierre de fase
```

La numeración ordena el trabajo; no crea versiones nuevas de Gramática o IR ni autoriza a cerrar un corte por la mera existencia del siguiente.

## 8. Criterios de cierre de R1

R1 sólo podrá cerrarse si existe evidencia reproducible de que:

1. una vía ordinaria no puede fabricar autoridad desde información, evidencia o capacidad técnica;
2. la autoridad nueva sólo puede aparecer por una transición autorizante válida dentro del modelo representado;
3. una continuidad lógica ya habitada no admite una segunda T-0;
4. una forma sujeta a control con `Req = ∅` no puede producir efecto;
5. cualquier `D-R` o `D-N` aplicable bloquea el efecto;
6. `D-N` y los fallos técnicos permanecen fuera de `Tri`;
7. el permiso queda ligado al objeto de decisión y no es reutilizable fuera de sus ligaduras;
8. el efecto protegido intra-proceso no dispone de una vía ordinaria que eluda la mediación;
9. la traza distingue acreditación, refutación y no verificabilidad;
10. las pruebas de R0 y sus destinos nativo/WebAssembly no sufren una regresión semántica atribuible a R1.

El cierre de R1 no acreditará por sí mismo Garantía I o Garantía II.

## 9. Estado resultante de la apertura

```text
Sec.6 = ABIERTA

R0 = CERRADO
R1 = ABIERTO
R2 = NO INICIADO
R3 = NO INICIADO
R4 = NO INICIADO

Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```

La primera unidad material de R1 será R1-0. Ningún estado posterior se presume por esta apertura.