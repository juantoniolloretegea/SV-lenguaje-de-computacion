# SV-lenguaje-de-computacion

Repositorio operativo y técnico del lenguaje de computación del Sistema Vectorial SV.

Este repositorio alberga la especificación operativa activa, la implementación de referencia del frontend, la documentación técnica de fase, la suite observable y el gobierno técnico subordinado del lenguaje. No sustituye a la sede doctrinal superior del ecosistema SV, que permanece en `SV-matematica-semantica`.

## Estado actual

- **IR canónica vigente:** `IR_CANONICA_BIENFORMACION_SV_v0_2.md`
- **Gramática superficial vigente:** `GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md`
- **Frontera normativa vigente:** `FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md`
- **Frontend de referencia activo:** parser + validator + lowering + serialización JSON canónica
- **Contrato de enganche mínimo:** `docs/arquitectura/CONTRATO_DE_ENGANCHE_DE_INTERFACES_FUTURAS_Y_ABI_SEMANTICO_DIAGNOSTICO_MINIMO.md`
- **Arquitectura mínima del núcleo enganchable:** `docs/arquitectura/NOTA_DE_ARQUITECTURA_MINIMA_DEL_NUCLEO_ENGANCHABLE_DEL_LENGUAJE_SV.md`
- **Marco de estabilidad, resiliencia y horizontes:** `docs/arquitectura/MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`
- **Control activo de calidad y hitos:** `docs/calidad/README.md`
- **Trazabilidad adicional de lectura fresca, barridos secuenciales y partes por agente:** `docs/calidad/REGISTRO_BARRIDOS_DE_ACTIVIDAD_Y_LATENCIA_DEL_REPO.md` y `docs/calidad/REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.md`
- **Hito actual auditado:** `H1 — Base segura` verificado como base material suficiente del frente vigente, sin apertura automática de `H2`/`H3` ni cierre total de `FFL-A` a `FFL-E`; véase `docs/calidad/ACTA_TECNICA_DE_VERIFICACION_DEL_HITO_1_BASE_SEGURA_2026_03_23.md`.
- **Estado de cierre:** la convergencia fuerte entre IR, catálogo, validator, suite y documentación pública sigue parcialmente abierta; la capa `N4/Uso` no debe leerse todavía como cierre total.

## Qué hace hoy el frontend de referencia

Toma un archivo `.svp` escrito en la Gramática superficial mínima v0.1, lo parsea, aplica la validación implementada vigente —subordinada a la IR canónica v0.2— y produce la representación IR serializada en JSON canónico.

## Qué no hace

No ejecuta programas. No calcula evaluaciones materiales (`T(n)`, clasificación o conteos). No resuelve `U`. No implementa todavía backend soberano. No sustituye a la semántica doctrinal del sistema.

## Uso rápido desde la raíz del repositorio

```bash
python src/svp_main.py examples/consulta_framecomparison.svp
python src/svp_main.py examples/consulta_framecomparison.svp -o salida.ir.json
```

## Verificación mínima observable

```bash
python tests/run_conformance.py
python tests/run_sec0_smoke.py
python tests/run_cli_smoke.py
```

## Estructura mínima orientativa

| Ruta | Función |
|---|---|
| `src/` | Implementación de referencia del frontend (`lexer`, `parser`, `validator`, `IR`, `serializer`, `CLI`) |
| `tests/` | Suite observable de conformidad, smoke y adversariales |
| `examples/` | Ejemplos mínimos de uso superficial |
| `docs/referencia/` | Catálogo público, referencia y piezas de lectura técnica |
| `docs/arquitectura/` | Actas, hoja de ruta, contrato mínimo de enganche, arquitectura mínima y marco de resiliencia |
| `docs/calidad/` | Registro técnico, deuda viva, vigilancia UCBC, control por hitos y calidad activa |
| `docs/manual_svp/` | Manual operativo local del lenguaje SVP |
| `beta/` | Régimen Beta subordinado y expresamente no normativo |

## Documentos de entrada recomendada

- `docs/README.md`
- `docs/arquitectura/MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`
- `docs/arquitectura/CONTRATO_DE_ENGANCHE_DE_INTERFACES_FUTURAS_Y_ABI_SEMANTICO_DIAGNOSTICO_MINIMO.md`
- `docs/calidad/README.md`
- `docs/calidad/ACTA_TECNICA_DE_VERIFICACION_DEL_HITO_1_BASE_SEGURA_2026_03_23.md`
- `docs/calidad/ACTA_TECNICA_DE_MICROAUDITORIA_H2_PRE_NO_BLOQUEO_2026_03_23.md`
- `docs/calidad/REGISTRO_BARRIDOS_DE_ACTIVIDAD_Y_LATENCIA_DEL_REPO.md`
- `docs/calidad/REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.md`
- `docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`
- `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`

## Nota de verificabilidad superficial

El literal superficial `FrameComparison` se usa hoy tanto como constructor de `QueryContext` como para `query_type` homónimo. El parser de referencia acepta ambos usos bajo un mismo literal superficial y el lexer evita ya la duplicidad silenciosa de clave que podía oscurecer esa lectura en revisiones externas.

## Subordinación

Sede doctrinal superior → `SV-matematica-semantica`  
Especificación operativa + implementación de referencia + gobierno técnico → `SV-lenguaje-de-computacion`

La cadena de lectura correcta sigue siendo:

**Gramática superficial mínima v0.1 → IR canónica v0.2 → Frontera normativa v0 → implementación observable → suite y documentación pública**.

---

*Lenguaje de computación del Sistema Vectorial SV.*  
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0 | ISSN 2695-6411*
