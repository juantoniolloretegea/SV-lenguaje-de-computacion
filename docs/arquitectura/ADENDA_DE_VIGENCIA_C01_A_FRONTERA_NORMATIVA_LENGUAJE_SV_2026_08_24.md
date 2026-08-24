# Adenda de vigencia de C01 a la Frontera normativa del Lenguaje SV

**Fecha:** 24/08/2026  
**Ámbito:** `SV-lenguaje-de-computacion`  
**Estado:** adenda técnica de vigencia  
**Objeto:** reconciliar la cadena de transducción de `FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md` con Gramática 0.2, IR 0.3 y la corrección C01.

## 1. Alcance y precedencia

Esta adenda no reescribe retrospectivamente la Frontera normativa v0. Corrige únicamente la lectura vigente de su apartado B.6 y de cualquier formulación histórica equivalente que proyecte un fallo de captura, una inadmisibilidad o la ausencia de una vía de constitución directamente sobre `Tri.U`.

Para el Lenguaje vigente prevalecen Gramática 0.2, IR 0.3 y C01 en el radio afectado.

No se modifica:

```text
Tri = {0,1,U}
```

ni se introduce un cuarto valor ternario.

## 2. Cadena de transducción vigente

La cadena conceptual permanece:

```text
mundo
→ captura
→ admisibilidad
→ observación positivamente admitida
→ Ternarizer
→ Tri
→ posición/célula
```

La separación obligatoria es:

```text
CaptureSpec.Bottom   ≠ Tri.U
NotAdmitted          ≠ Tri.U
fallo técnico        ≠ Tri.U
trabajo pendiente    ≠ Tri.U
```

`AdmissibilityState` contiene exactamente:

```text
{Ok, Degraded, NotAdmitted}
```

`Degraded` sólo puede alcanzar la ternarización cuando conserva estatuto de observación positivamente admitida conforme a la regla aplicable.

## 3. Producción legítima de `Tri.U`

`Tri.U` no representa indisponibilidad técnica ni insuficiencia de captura.

Una observación admitida puede producir legítimamente `Tri.U` cuando el `Ternarizer` aplicable la sitúa en la partición semántica `B_U` de su espacio observacional.

Por tanto:

```text
captura fallida                  ↛ Tri
admisibilidad insuficiente       ↛ Tri
NotAdmitted                      ↛ Tri
observación admitida en B_U      → Tri.U
```

La ausencia de un nuevo valor ternario no se rellena con `U` y no reescribe un estado ternario anterior.

## 4. Actores estadísticos y salidas auxiliares

La existencia de una salida estadística, probabilística o producida por una IA no altera esta separación.

No son reglas vigentes del Lenguaje:

```text
solo camino estadístico → Tri.U

divergencia entre camino determinista y actor estadístico → Tri.U
```

Una discrepancia externa puede originar una incidencia, una revisión o una restricción de uso cuando exista una regla previamente constituida que lo establezca. No puede, por sí sola, reescribir un valor ternario ya constituido ni fabricar una `U`.

## 5. `Ternarizer` no legitima por sí solo una inferencia opaca

El `Ternarizer` conserva su función de transducción semántica declarada. Su presencia no convierte automáticamente en fundamento SV una inferencia estadística previa.

En particular, no se deduce:

```text
score / etiqueta / ranking / vector latente
→ función determinista
→ Tri

por tanto

fundamento no estadístico
```

La entrada al `Ternarizer` debe satisfacer las condiciones de observación y admisibilidad del dominio. Cuando la corrección de un candidato externo sea material para el efecto reclamado, la admisión deberá poder justificarse con evidencia suficiente para ese alcance.

## 6. Tiempo y fallos externos

La recencia, el tiempo de espera agotado, la memoria de un servicio, una marca temporal de proveedor o un error de transporte no constituyen por sí mismos semántica ternaria.

Pueden formar parte de diagnóstico técnico, evidencia externa, forensia o reglas explícitas de un dominio cuando proceda. No existe una proyección universal:

```text
timeout → U
más reciente → vigente
fallo de servicio → U
```

## 7. Efectos sobre documentación histórica

Las formulaciones históricas conservadas en el README o en documentos anteriores siguen siendo útiles como registro de evolución, pero no gobiernan el comportamiento actual cuando contradicen esta adenda, C01, Gramática 0.2 o IR 0.3.

En particular, quedan desplazadas como reglas vigentes:

```text
R = {ok, degradado, fallido, U}
fallo/admisibilidad insuficiente → U
solo camino estadístico → U
divergencia estadística → U
```

## 8. Estado

Esta adenda fija una regla normativa de lectura. No acredita por sí sola una realización material, un actor de IA concreto, una garantía de seguridad ni una primera versión estable.

La correspondencia entre esta regla, la implementación y los artefactos ejecutados deberá demostrarse en las fases y perfiles que correspondan.