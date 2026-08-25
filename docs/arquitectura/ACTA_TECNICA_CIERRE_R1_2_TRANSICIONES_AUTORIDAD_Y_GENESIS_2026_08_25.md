# Cierre de R1-2 — transiciones de autoridad, T-0 y continuidad lógica

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-2  
**Estado:** cerrado · integrado

## 1. Objeto

Este documento registra el cierre de R1-2 después de integrar la realización que materializa la clasificación de transiciones de autoridad y restringe T-0 a la génesis inicial de una continuidad autoritativa lógica no habitada.

El cierre no inicia R1-3, no modifica la semántica cerrada de R0 y no acredita propiedades materiales asignadas a R2–R4 o a las Garantías I y II.

## 2. Integración cerrada

La realización cerrada se integró sobre el candidato exacto:

```text
head auditado =
451fb8cee50bf2fa6a3eb8d70bd962133148a6fb

base =
65598846536561d578af9278e5194d0383cbb066

merge =
eb9f1c606d76245f2c9cde9c97326a93c09f8dcc

tree =
254eb3e4905efc7789d10dcbf43d777c1e0c81a2
```

El árbol del `merge` coincide con el árbol del candidato sometido a comprobación.

## 3. Propiedades materializadas

R1-2 deja establecidas, dentro del alcance intra-proceso de R1, las propiedades siguientes:

1. T-0 es la única vía productiva de autoridad del corte.
2. T-0 exige conjuntamente una premisa constituyente externa opaca no consumida y una continuidad lógica `Uninhabited`.
3. una T-0 completada constituye el estado inicial, ocupa la continuidad y consume la premisa;
4. una segunda T-0 sobre la misma continuidad lógica queda bloqueada;
5. una premisa consumida no puede reutilizarse en otra génesis dentro de esa continuidad;
6. una génesis rechazada no deja estado parcial, no ocupa la continuidad y no consume la premisa;
7. una `FormRef` o `AuthorityRef` duplicada dentro del estado inicial se rechaza antes de la constitución;
8. las autoridades propuestas deben conservar la coherencia estructural de contexto, `E_max` y `D_a`;
9. toda forma T-G, T-C o T-R debe declarar la autoridad previa requerida y dicha referencia debe existir en el estado inicial;
10. T-I, T-V, T-H y T-E permanecen no autorizantes;
11. T-G, T-C y T-R permanecen no productivas mientras no exista la frontera posterior de requisitos, decisión y mediación.

La clase T-* de una forma permanece fijada por el descriptor constituido; no puede reclasificarse durante el acto para obtener un régimen más permisivo.

## 4. Premisa constituyente externa

`ExternalGenesisPremise` es una capacidad opaca para la API ordinaria del núcleo. `sv_core` no ofrece un constructor público que permita acuñarla desde los adaptadores ordinarios.

Esta propiedad no debe interpretarse como una raíz material de confianza ni como resistencia frente a código privilegiado, `unsafe`, manipulación del proceso o compromiso de plataforma. La procedencia, identidad, integridad y legitimidad material de la premisa permanecen fuera del alcance de R1.

## 5. Evidencia reproducible

Sobre el candidato final se completaron correctamente:

```text
Conformidad SVP              #112 = SUCCESS
R0 Rust                       #84 = SUCCESS
R0-8 Baseline nativa          #36 = SUCCESS
R0 WASM paridad de tres vías  #32 = SUCCESS
```

La realización ejerció 93 pruebas unitarias de `sv_core` y cinco pruebas documentales sin fallos. Entre las comprobaciones negativas se incluyen:

- imposibilidad de construir la premisa constituyente mediante la API ordinaria;
- rechazo de una segunda T-0;
- rechazo de una premisa ya consumida;
- atomicidad ante rechazo de génesis;
- rechazo de referencias constitutivas duplicadas;
- clasificación no autorizante de T-I, T-V, T-H y T-E;
- bloqueo de T-G, T-C y T-R antes de la frontera posterior de requisitos.

La batería heredada mantiene 11/11 casos válidos equivalentes y 61/61 casos inválidos rechazados.

## 6. Límites expresos

R1-2 no acredita ni materializa:

- `Req(F,e | C)`;
- aplicabilidad de obligaciones;
- decisión agregada `D-A` / `D-R` / `D-N` sobre requisitos;
- `Permit`;
- mediación productiva de efectos protegidos;
- persistencia durable;
- continuidad material entre procesos;
- recuperación material;
- identidad externa, credenciales o firmas;
- raíz material de confianza;
- resistencia frente a código privilegiado;
- Garantía I;
- Garantía II.

El consumo intra-proceso de una premisa no equivale a un mecanismo de consumo único distribuido o resistente a restauración.

## 7. Estado resultante

```text
R0 = CERRADO
R1 = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = CERRADO · INTEGRADO
R1-3 = NO INICIADO
R2 = NO INICIADO
R3 = NO INICIADO
R4 = NO INICIADO

Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```

La apertura de R1-3 requerirá un acto separado y deberá fijar `Req`, aplicabilidad y los resultados `D-A`, `D-R` y `D-N` sin producir todavía por inferencia un permiso o un efecto protegido.