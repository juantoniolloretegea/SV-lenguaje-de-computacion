# Calidad del ámbito operativo del Lenguaje SV

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Última actualización:** 5 de septiembre de 2026

## 1. Función de esta carpeta

`docs/calidad/` reúne los documentos públicos de control técnico, trazabilidad, deuda viva, verificación, continuidad y evolución del Lenguaje SV.

Su función es permitir que un tercero distinga con precisión entre especificación, realización, evidencia ejecutable, deuda técnica, estado de fases y despliegue público. Los documentos de Calidad no sustituyen a la gramática, la IR, el código ni las pruebas que fundamentan cada afirmación.

## 1.1. Pieza rectora de diseño

La entrada prioritaria para cualquier modificación de gramática, IR, validación, núcleo, frontera, host, dominio o agente es:

- [**Pilares y restricciones de diseño del Lenguaje de Computación SV**](./PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md).

Esta pieza fija una frontera de autoridad: la unidad competente de dominio constituye sus células y asignaciones; la unidad competente de agente declara su cobertura sobre una constitución recibida; el Lenguaje representa, valida, preserva y falla cerrado, pero no completa ni suplanta esas decisiones. También impide confundir una célula exacta —vector plano de longitud `n=b²`— con una matriz `b × b`, utilizar `U` como relleno o convertir una ausencia contractual en aceptación silenciosa.

Su relación con el reparto entre núcleo, frontera y host se documenta en el [acta técnica de arquitectura de software](./ACTA_TECNICA_DE_ARQUITECTURA_DE_SOFTWARE_NUCLEO_FRONTERA_Y_HOST_SV_2026_09_04.md).

## 2. Estado de continuidad vigente

Tras el cierre correctivo de 29/08/2026, el estado aplicable es:

```text
R0 = CERRADO, incluido el perímetro correctivo de DFL-007
R1 = CERRADO y revalidado sobre la base R0 corregida
R2 = ABIERTO; levantada la suspensión específica causada por DFL-007
R3 = NO INICIADO
R4 = NO INICIADO

Garantía I  = NO_PROBADO
Garantía II = NO_PROBADO
```

El levantamiento de la suspensión no constituye cierre de R2 ni acredita persistencia o continuidad material todavía no demostradas.

Los documentos principales de esta transición son:

- [`ACTA_TECNICA_DE_ENCARGO_CORRECTIVO_INCIDENTAL_HUECOS_HEREDADOS_BETA_2_Y_BLOQUEO_R2_2026_08_29.md`](./ACTA_TECNICA_DE_ENCARGO_CORRECTIVO_INCIDENTAL_HUECOS_HEREDADOS_BETA_2_Y_BLOQUEO_R2_2026_08_29.md) — apertura y condiciones del encargo DFL-007;
- [`ACTA_TECNICA_DE_CONFORMIDAD_CIERRE_CORRECTIVO_B2_Y_RESTAURACION_CONTINUIDAD_2026_08_29.md`](./ACTA_TECNICA_DE_CONFORMIDAD_CIERRE_CORRECTIVO_B2_Y_RESTAURACION_CONTINUIDAD_2026_08_29.md) — cierre del encargo, revalidación de R1 y levantamiento de la suspensión específica de R2;
- [`../arquitectura/ACTA_TECNICA_CIERRE_R1_2026_08_25.md`](../arquitectura/ACTA_TECNICA_CIERRE_R1_2026_08_25.md) — cierre técnico original de R1;
- [`../arquitectura/ACTA_TECNICA_APERTURA_R2_PERSISTENCIA_Y_CONTINUIDAD_MATERIAL_2026_08_25.md`](../arquitectura/ACTA_TECNICA_APERTURA_R2_PERSISTENCIA_Y_CONTINUIDAD_MATERIAL_2026_08_25.md) — apertura de R2.

## 3. Corte correctivo B2

La realización estable corregida queda integrada en:

```text
main de realización = c1acf943a7a44ce81080881e59283de8a2019606
```

WebAssembly publicado:

```text
bytes   = 378956
SHA-256 = 95c7d1e0313567ef099c6e426a7fcee8ff4a5ac8adb670265f859f1bf03caab3
```

Paquete utilizado para el despliegue manual:

```text
SV_LENGUAJE_PRODUCCION_B2_CLOUDFLARE_2026-08-29_FINAL_CONFORMIDAD.zip
bytes   = 167503
SHA-256 = 566200f97bfea86a0b7ce7c4919bac9d5367a67b8cba719eef1c573942d696f5
archivos = 39
```

La comprobación material posterior al despliegue confirmó que los historiales Beta español e inglés se representan como páginas HTML y no como código fuente.

## 4. Evidencia de conformidad

Las ejecuciones asociadas a la base corregida acreditan:

```text
conformidad R0-7              = 79/79
  válidos                     = 12/12
  inválidos                   = 67/67
sv_core                       = 210/210
dominios cerrados Rust        = 5/5
sondas DG-01/02/03 navegador  = 6/6
sv_wasm                       = 2/2
doc-tests sv_core             = 17/17
```

Evidencia reproducible:

- [PR #55](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/55);
- [Conformidad SVP — 33271992372](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/33271992372);
- [R0 Rust — 33271992363](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/33271992363);
- [R0-8 — 33271992371](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/33271992371);
- [R0 WebAssembly y navegador — 33271992457](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/33271992457).

Las seis sondas de navegador ejercen DG-01, DG-02 y DG-03 en `SVP-ES` y `SVP-EN`. Las regresiones permanecen incorporadas al repositorio.

## 5. Separación de perfiles y gramática

El estado normativo vigente distingue:

```text
perfil léxico de identificadores
≠
perfil fuente SVP-ES / SVP-EN
```

Los perfiles fuente se constituyen en `ESPECIFICACION_NORMATIVA_PERFILES_FUENTE_SVP_ES_EN_v1_2026_08_29.md` y convergen sobre una única Gramática canónica 0.2, una única IR 0.3 y una única semántica.

La realización vigente contiene 154 identidades canónicas, 297 grafías distintas y 11 formas compartidas. La selección es explícita y no existe autodetección de perfil.

## 6. DFL-007

DFL-007 se considera cerrada porque:

- `SemanticRelation.kind`, `Pattern.kind` y `Graph.regime` están nuevamente restringidos a sus dominios normativos;
- existen regresiones permanentes para ES y EN;
- el corpus de conformidad y la paridad nativa/WebAssembly permanecen verdes;
- la Gramática 0.2 reconcilia los cierres internos heredados de `connector.mapping` y `admissibility_table.table`;
- el vector `deep_nested_query_valid.svp` ha sido reclasificado como vector histórico de Gramática 0.1;
- R1 fue revalidado sobre la base corregida.

La publicación estable de B2 y el cierre de DFL-007 son hechos distintos. La publicación no cerró por sí sola la deuda ni habilitó automáticamente R2.

## 7. Deuda que permanece abierta

El cierre de DFL-007 no elimina otras deudas vivas. En particular, sigue pendiente la materialización de `ConflictOperator` y la comprobación completa de J2.3 para concurrencia en régimen `General`.

También conservan su estatuto propio, según el registro de deuda:

- la concordancia diagnóstica no agotada entre IR y catálogo efectivo;
- la procedencia completa de determinadas actualizaciones de `CoupledState`;
- la suficiencia reconstructiva completa de `TransitionData`;
- la producción material de `CriticalityResult`;
- la ejecución de `GateResult.output`;
- la semántica ejecutiva completa de supervisión;
- las obligaciones de FFL-D y las capacidades materiales de fases posteriores.

Ninguna de estas deudas se considera resuelta por el cierre correctivo B2.

## 8. Registros vivos

- [`REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`](./REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md) — deuda técnica vigente y deuda cerrada relevante para continuidad;
- [`REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`](./REGISTRO_EVOLUCION_TECNICA_PROYECTO.md) — lectura humana del tramo registral vigente;
- `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv` — numeración maestra RETP;
- [`HISTORIAL_VERSIONES_LENGUAJE_SV.md`](./HISTORIAL_VERSIONES_LENGUAJE_SV.md) — evolución de versiones, realizaciones y entornos;
- `TABLERO_DE_BLOQUES_CERRABLES_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.csv` — bloques técnicos y deuda asociada.

El detalle mecánico de cada modificación permanece en Git. Los registros vivos concentran hitos materiales y no crean un asiento independiente para cada corrección menor de una misma operación de integración y cierre.

## 9. Verificación independiente pendiente

El presente cierre acredita la conformidad interna del corte corregido y su despliegue material. La verificación externa independiente del mismo corte se documentará mediante un acta separada cuando se complete.

Hasta entonces no se atribuye al cierre de conformidad el estatuto de verificación independiente.
