# Cierre técnico de R1 — autoridad, mediación y decisiones protegidas

**Fecha:** 25 de agosto de 2026  
**Repositorio:** `SV-lenguaje-de-computacion`  
**Estado:** R1 cerrado  
**R0:** cerrado  
**R2:** no abierto por este documento  
**R3–R4:** no iniciados  
**Garantía I:** `NO_PROBADO`  
**Garantía II:** `NO_PROBADO`

## 1. Objeto

Este documento registra el cierre de R1 como fase de realización intra-proceso de autoridad, mediación y decisiones protegidas.

R1 queda cerrado porque la realización integrada satisface conjuntamente los criterios establecidos en su apertura: no fabricación de autoridad, constitución gobernada, unicidad de T-0 en continuidad habitada, requisitos no vacíos para formas sujetas a control, fallo cerrado ante D-R y D-N, separación respecto de `Tri`, ligadura exacta del permiso, mediación intra-proceso, traza suficiente y ausencia de regresión respecto de R0.

## 2. Corte soberano

La última subunidad de R1 fue verificada sobre:

```text
7210843305f450f36e7544e9a36699216d1b93a8
```

e integrada en:

```text
main = 243bf4c2d6cdf93329185e2a628ee73aaf12a8e3
```

La integración conserva como evidencia reproducible del candidato exacto:

```text
Conformidad SVP                 #198 = success
R0 Rust                         #162 = success
R0-8 Baseline nativa            #113 = success
R0 WASM paridad de tres vías    #108 = success

sv_core   = 198/198
sv_wasm   = 2/2
doc-tests = 17/17
R0-7      = 11/11 casos válidos equivalentes
            + 61/61 casos inválidos rechazados
```

## 3. Propiedades cerradas en R1

R1 deja materializadas, dentro del perímetro intra-proceso declarado, las siguientes propiedades:

1. información, evidencia, resultado técnico, autoridad, habilitación y ejercicio permanecen separados;
2. T-I, T-V, T-H y T-E no constituyen autoridad;
3. T-0 sólo puede constituir la primera autoridad de una continuidad lógica no habitada;
4. una continuidad ya habitada rechaza una segunda T-0;
5. toda forma sujeta a control requiere `Req(F,e | C) ≠ ∅`;
6. cualquier D-R o D-N aplicable bloquea el efecto;
7. D-N y los fallos técnicos o estructurales permanecen fuera de `Tri`;
8. el permiso queda ligado a forma, autoridad, efecto, contexto y obligaciones aplicables;
9. la vía pública conforme de ejercicio protegido exige decisión trazada, mediación y ejecución gobernada;
10. la traza conserva las comprobaciones individuales, los resultados resueltos y cualificados, el agregado, la disposición de permiso o bloqueo y, cuando existe ejercicio, su relación causal con el `ExerciseRef` real.

La clase T-E es productiva únicamente como ejercicio gobernado. T-G, T-C y T-R permanecen no productivas.

## 4. Límites que R1 no acredita

El cierre de R1 no acredita:

- persistencia autoritativa durable;
- recuperación material tras reinicio;
- selección de continuidad vigente entre bifurcaciones;
- resistencia durable a retroceso, clonación o doble consumo;
- raíz material de confianza;
- correspondencia entre fuente, artefacto construido, distribuido y cargado;
- aislamiento del sistema operativo, hipervisor o hardware;
- mediación completa de todas las vías materiales del sistema;
- confidencialidad material o ausencia de exfiltración;
- Garantía I;
- Garantía II.

Estas propiedades permanecen fuera de R1 y deberán abordarse en las fases que les correspondan.

## 5. Continuidad arquitectónica gobernada

El núcleo se implementa como realización mínima garantista suficiente para sostener las propiedades exigibles del conjunto en el alcance actualmente constituido. Su diseño no clausura revisiones, sustituciones ni arquitecturas futuras que puedan demostrar una solución técnicamente superior, más segura o más adecuada.

Esa apertura evolutiva no constituye una vía excepcional ni una facultad de modificación libre. Toda alteración causalmente relevante deberá quedar previamente gobernada, acreditada y trazada conforme al **Pliego de Condiciones del Sistema Vectorial SV**, DOI `10.21428/39829d0b.bbcac925`, y a las reglas de autoridad, constitución y transición que resulten aplicables.

Por tanto:

```text
el diseño presente no clausura la evolución futura
∧
la evolución futura no crea una vía lateral de elusión
```

## 6. Estado resultante

```text
R0 = CERRADO
R1 = CERRADO
R2 = NO ABIERTO POR ESTE DOCUMENTO
R3 = NO INICIADO
R4 = NO INICIADO

T-E = PRODUCTIVA POR VÍA GOBERNADA
T-G / T-C / T-R = NO PRODUCTIVAS

BudgetΣ / IA-SEC = NO ABIERTOS
Garantía I / II = NO_PROBADO
```
