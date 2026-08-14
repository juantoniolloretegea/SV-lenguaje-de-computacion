# Acta técnica de cierre auditado y preservación controlada de SV-AUTH A.2 r2 y J6

**Fecha:** 14/08/2026  
**Hora (Europe/Madrid):** 18:31:00  
**Agente:** Agente Watson SV-AUTH  
**Base de verificación:** `VERIFICACION_INTEGRAL`  
**Rama auditada:** `sv-auth-v0.2`  
**Rama de gobierno registral:** `main`

## 1. Objeto

Cerrar registralmente y preservar de forma controlada el macrolote SV-AUTH desarrollado en la rama `sv-auth-v0.2`, sin convertir su existencia, su conformidad de referencia ni su utilidad futura en una autorización tácita de integración dentro del Lenguaje SV.

La finalidad de esta acta es doble:

1. impedir la pérdida de un trabajo técnico materialmente aprovechable y ya sometido a auditoría fuerte;
2. impedir que una futura unidad interprete la existencia de A.2 r2 o de J6 como permiso para modificar `main`, la gramática, la IR, el parser, el validator, el runner, el Playground o el motor sin una decisión arquitectónica previa, explícita y trazable.

## 2. Estado del árbol en el punto de auditoría

En el inicio de este cierre:

- `main` estaba fijada en `18a561eb81bda793db857381d1a1d4cda320a13c`;
- `sv-auth-v0.2` estaba fijada en `12874332ba1dfa28d2a00ca265f2ece8164871a0`;
- la rama AUTH estaba 9 commits por delante y 0 por detrás de `main`;
- la diferencia contra `main` se limitaba a ocho archivos añadidos:
  - `docs/arquitectura/SV_AUTH_A2_AUDITORIA_SPEC_RUNTIME_2026_08_13.md`;
  - `docs/arquitectura/SV_AUTH_A2_CLAUSURA_OPERACIONAL_v0_2.md`;
  - `docs/arquitectura/SV_AUTH_A2_CLAUSURA_OPERACIONAL_v0_2_2.md`;
  - `docs/arquitectura/SV_AUTH_A2_PRUEBAS_v0_2.md`;
  - `docs/arquitectura/SV_AUTH_A2_PRUEBAS_v0_2_2.md`;
  - `docs/arquitectura/SV_AUTH_A2_SELLO_Y_TRAZABILIDAD_r2_2026_08_13.md`;
  - `src/svp_authority_static.py`;
  - `tests/conformance/test_j6_authority_static.py`.

No constaba modificación AUTH del parser, validator, IR canónica, gramática, runner de conformidad, `svp_main`, API del Playground ni backend.

La posterior incorporación de esta acta y de sus asientos de calidad a `main` puede hacer que la rama AUTH aparezca administrativamente por detrás de `main` en esos commits registrales. Ese hecho no constituye pérdida ni divergencia funcional de AUTH: la rama técnica queda deliberadamente inmóvil como punto de restauración.

## 3. Estado final de A.2

La versión normativa final de trabajo del macrolote es:

- `SV_AUTH_A2_CLAUSURA_OPERACIONAL_v0_2_2.md`;
- `SV_AUTH_A2_PRUEBAS_v0_2_2.md`;
- `SV_AUTH_A2_SELLO_Y_TRAZABILIDAD_r2_2026_08_13.md`.

Las versiones `v0_2` y los resultados 36/36 y 74/74 se conservan como historia de la auditoría y de la corrección, pero no gobiernan el cierre final.

El artefacto de referencia A.2 r2 queda identificado por SHA-256:

`7c18761cf5546c8fdd9ad962c0ea3e0a54a9ddd4a4bf6d43c0ab29c7e4cf794f`

El barrido de cierre confirma la evidencia ya sellada:

- 78/78 pruebas de conformidad;
- 537 sentencias medidas;
- 24 sentencias no cubiertas;
- 96 % de cobertura total;
- 94 % en `authority_runtime.py`;
- 98 % en `authority_types.py`;
- matriz de trazabilidad 15/15 entre reglas sensibles selladas y pruebas.

Estas cifras constituyen evidencia de conformidad de la realización de referencia con la semántica sellada. No sustituyen las demostraciones L1-L3, P1, TA, TB, C1, C2 e IS ni se interpretan como prueba universal de seguridad, autenticación, criptografía, integridad de hardware/SO o corrección de un runtime hostil.

## 4. Estado final de J6

`src/svp_authority_static.py` queda clasificado como **capa estática J6 de referencia**. Su propia cabecera establece que no ejecuta autoridad ni sustituye al runtime; valida declaraciones de autoridad, la tabla sellada de efectos y el lowering canónico de la especificación AUTH.

La fuente final en la cabeza de `sv-auth-v0.2` queda identificada por blob Git:

`af826dbfd66c3694a7541bc11f713c93a46d1279`

La batería `tests/conformance/test_j6_authority_static.py` queda identificada por blob Git:

`3d8b0d9e83a522cf66d4866b2144a55760da51fd`

El barrido confirma:

- 21/21 pruebas J6;
- 97 % de cobertura del módulo estático de referencia.

Se conserva asimismo el ZIP congelado de referencia con SHA-256:

`cfba4d7799f0af1bb57efa68066cd2a7600b5e3580b6934ce7e42657f6e98243`

La comparación realizada durante el cierre muestra que el `svp_authority_static.py` del ZIP y la fuente final de GitHub no son byte a byte idénticos por la presencia/ausencia de comentarios descriptivos no ejecutables. Eliminados esos comentarios, la fuente produce exactamente el blob Git final citado. No se ha encontrado divergencia funcional. Por ello la cabeza `12874332ba1dfa28d2a00ca265f2ece8164871a0` de `sv-auth-v0.2` gobierna como fuente técnica de restauración; el ZIP se conserva como realización reproducible semánticamente equivalente.

## 5. Límite de lo demostrado

El cierre mantiene expresamente la separación entre:

- propiedades de la semántica sellada A.2;
- realización ejecutable de referencia;
- comprobador estático J6 de referencia;
- eventual integración futura en el Lenguaje SV y su motor.

En particular, C2 demuestra no interferencia de una transición AUTH pura sobre el estado basal opaco `S_B`; no certifica todavía el handoff de una decisión AUTH hacia el parser, validator, motor integrado, REAL/SIM, custodia estructural o precedencia de capas.

No queda certificado por esta acta:

- parser o sintaxis superficial de AUTH;
- nodos AST/IR integrados;
- integración J6 en `svp_validator.py`;
- integración del catálogo E6xx en el catálogo diagnóstico general del lenguaje;
- ejecución AUTH en `svp_main.py` o en el motor;
- autenticación humana real;
- garantías criptográficas;
- host adversarial o runtime productivo;
- handoff integrado que preserve REAL/SIM, custodia y precedencia.

## 6. Clasificación de continuidad

El estado del macrolote se clasifica como:

**`LATENTE_LEGITIMO`**

No se clasifica como deuda viva ni como frente abandonado. La latencia es deliberada: la formalización y las realizaciones de referencia alcanzaron un punto útil, pero la continuación hacia el lenguaje integrado fue detenida para evitar un avance arquitectónico precipitado y para exigir antes una autorización transparente de cambio.

## 7. Decisión

Se acuerda:

1. preservar `sv-auth-v0.2` en `12874332ba1dfa28d2a00ca265f2ece8164871a0` como punto técnico de restauración y procedencia;
2. no fusionar por esta acta `sv-auth-v0.2` en `main`;
3. no trasladar por inercia A.2 o J6 al parser, validator, IR, gramática, runner, Playground o motor;
4. no abrir deuda viva sólo por la existencia de este trabajo latente;
5. registrar el cierre mediante `RETP-2026-046` y `BARR-2026-004`;
6. mantener las publicaciones y sus transductores fuera de esta regularización de Calidad, pues se gestionan por sus inventarios específicos;
7. exigir un acto previo de reapertura/habilitación arquitectónica antes de cualquier continuación implementativa de AUTH.

## 8. Condición obligatoria de reapertura

Antes del primer commit que pretenda integrar o extender materialmente AUTH en el Lenguaje SV deberá existir un acta previa y aprobada que, como mínimo, declare:

- objeto exacto del cambio;
- relación con A.2 r2 y J6 preservados;
- superficie que se pretende tocar —gramática, AST, IR, parser, validator, catálogo diagnóstico, runtime, motor o handoff—;
- frontera entre información, autoridad persistente, autoridad habilitada y compromiso;
- tratamiento de REAL/SIM, custodia estructural y precedencia de capas cuando haya handoff;
- suite positiva, negativa y adversarial exigible;
- compatibilidad y plan de reversión;
- límites de seguridad que siguen siendo premisas externas o fronteras confiables;
- criterio de cierre y evidencia necesaria para autorizar una integración posterior en `main`.

La reapertura podrá decidir conservar, revisar o descartar partes de J6. Lo que no podrá hacer es retroproyectar silenciosamente AUTH sobre la IR v0.2 o presentar una realización de referencia como si ya hubiese sido una capacidad productiva del Lenguaje SV.

## 9. Objeción adversarial principal

El riesgo principal no es la pérdida del trabajo, pues la rama y los artefactos están preservados, sino la interpretación retrospectiva incorrecta de su estatuto. Dos errores simétricos quedan prohibidos:

- tratar AUTH como experimento desechable por no estar en `main`;
- tratar AUTH como función ya integrada por estar formalizada, probada y acompañada de J6.

El estado correcto es intermedio y explícito: **sustrato técnico preservado, auditado y reutilizable, con integración futura bloqueada hasta decisión arquitectónica previa**.

## 10. Cierre

**Dictamen:** APTO PARA CIERRE REGISTRAL Y PRESERVACIÓN CONTROLADA.  
**Integración automática en `main`:** NO AUTORIZADA.  
**Pérdida o descarte del trabajo:** NO.  
**Reapertura futura:** condicionada a acta arquitectónica previa.
