# SV-lenguaje-de-computacion

Repositorio operativo y técnico del lenguaje de computación del Sistema Vectorial SV.

**Función de la sede:** especificación operativa, implementación observable y gobierno técnico subordinado del lenguaje de computación del Sistema Vectorial SV.

## Ecosistema SV — ubicación de esta sede

### Distribución vigente de sedes

- **Doctrina, jerarquía y puerta general del ecosistema:** `SV-matematica-semantica`
- **Lenguaje, contrato técnico y sede operativa del manual SVP:** `SV-lenguaje-de-computacion`
- **Origen observacional, datasets e intrusión:** `SVcustos-dataset`
- **Agentes especializados, fases, artefactos y aplicaciones de uso:** `SVperitus-dataset`

### Regla de no sustitución

Este repositorio no altera ni sustituye la autoridad doctrinal superior. Su función es técnica y operativa, subordinada al corpus canónico.

### Remisión

Para una vista general del ecosistema SV y de la distribución entre sedes, consulte la puerta de entrada del ecosistema en `SV-matematica-semantica/docs/index.html`.

---

Este repositorio alberga la especificación operativa activa, la implementación de referencia del frontend, la documentación técnica de fase, la suite observable y el gobierno técnico subordinado del lenguaje. No sustituye a la sede doctrinal superior del ecosistema SV, que permanece en `SV-matematica-semantica`.

## Previsión estructural sobre agentes especializados e IA

Se constata ya, por maduración del ecosistema SV, la necesidad futura de una capa de consulta y simulación trazable por agente especializado, subordinada a la doctrina, al álgebra y al agente humano.

Esta previsión no introduce todavía:

- sintaxis nueva;
- gramática nueva;
- niveles IR nuevos;
- backend soberano nuevo;
- ni clausura automática de `U`.

Su función, en esta fase, es únicamente reservar contrato, límites y condiciones de compatibilidad futura para evitar una reingeniería tardía del lenguaje.

## Condiciones mínimas ya fijadas para esa futura capa

- separación estricta entre `REAL` y `SIM`;
- historia `append-only`, sin reescritura retrospectiva de la trayectoria real;
- primacía del SUCESO y del horizonte declarado frente al tiempo desnudo;
- respuesta trazable con justificación y metadatos de reconstrucción;
- prohibición de cierre automático de `U` por inferencia opaca, extrapolación continua o automatismo no legitimado;
- subordinación final al álgebra, a la doctrina y al agente humano.

**Documento de trabajo asociado:** `docs/arquitectura/NOTA_TECNICA_DE_PREVISION_DE_CAPA_IA_TRAZABLE_POR_AGENTE_ESPECIALIZADO.md`

## Custodia estructural del diseño y del DSL

La publicación vigente de la célula especializada de seguridad estructural ha hecho comparecer una exigencia adicional que afecta ya al lenguaje: la futura evolución de consultas, simulaciones, lowering, validación, serialización y artefactos de interfaz no podrá desarrollarse al margen de una custodia estructural del diseño, del DSL y de los laboratorios.

Esto significa, en esta fase, dos cosas:

1. el lenguaje debe reservar espacio conceptual y documental para convivir con esa custodia;
2. ninguna presión procedente de agentes, demos, laboratorios o capas IA puede doblar el tronco del lenguaje hasta introducir:
   - matemática por la puerta de atrás;
   - degradación silenciosa de la `U`;
   - mezcla ilegítima entre historia real y rama simulada;
   - o semántica nueva no autorizada por la cadena doctrinal.

La relación correcta no es de sometimiento del lenguaje a la célula de seguridad, sino de **convivencia obligatoria bajo vigilancia estructural visible**.

## Estado actual

- **IR canónica vigente:** `IR_CANONICA_BIENFORMACION_SV_v0_2.md`
- **Gramática superficial vigente:** `GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md`
- **Frontera normativa vigente:** `FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md`
- **Frontend de referencia activo:** parser + validator + lowering + serialización JSON canónica
- **Contrato de enganche mínimo:** `docs/arquitectura/CONTRATO_DE_ENGANCHE_DE_INTERFACES_FUTURAS_Y_ABI_SEMANTICO_DIAGNOSTICO_MINIMO.md`
- **Arquitectura mínima del núcleo enganchable:** `docs/arquitectura/NOTA_DE_ARQUITECTURA_MINIMA_DEL_NUCLEO_ENGANCHABLE_DEL_LENGUAJE_SV.md`
- **Marco de estabilidad, resiliencia y horizontes:** `docs/arquitectura/MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`
- **Nota de previsión de capa IA trazable:** `docs/arquitectura/NOTA_TECNICA_DE_PREVISION_DE_CAPA_IA_TRAZABLE_POR_AGENTE_ESPECIALIZADO.md`
- **Control activo de calidad y hitos:** `docs/calidad/README.md`
- **Trazabilidad adicional de lectura fresca, barridos secuenciales y partes por agente:** `docs/calidad/REGISTRO_BARRIDOS_DE_ACTIVIDAD_Y_LATENCIA_DEL_REPO.md` y `docs/calidad/REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.md`
- **Hito actual auditado:** `H1 — Base segura` verificado como base material suficiente del frente vigente, sin apertura automática de `H2`/`H3` ni cierre total de `FFL-A` a `FFL-E`; véase `docs/calidad/ACTA_TECNICA_DE_VERIFICACION_DEL_HITO_1_BASE_SEGURA_2026_03_23.md`.
- **Apertura vigente de fase:** `H2` autorizado en régimen restringido como frente arquitectónico-auditor de trabajo, sin verificación positiva todavía de `H2`, sin implementación fuerte y sin apertura automática del backend; véase `docs/calidad/ACTA_TECNICA_DE_AUTORIZACION_RESTRINGIDA_DE_APERTURA_DE_H2_2026_03_23.md`.
- **Estado de cierre:** la convergencia fuerte entre IR, catálogo, validator, suite y documentación pública sigue parcialmente abierta; la capa `N4/Uso` no debe leerse todavía como cierre total.

## Qué hace hoy el frontend de referencia

Toma un archivo `.svp` escrito en la Gramática superficial mínima v0.1, lo parsea, aplica la validación implementada vigente —subordinada a la IR canónica v0.2— y produce la representación IR serializada en JSON canónico.

## Qué no hace

No ejecuta programas. No calcula evaluaciones materiales (`T(n)`, clasificación o conteos). No resuelve `U`. No implementa todavía backend soberano. No sustituye a la semántica doctrinal del sistema. No incorpora aún la futura capa de interlocución IA por agente especializado.

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
- `docs/arquitectura/NOTA_TECNICA_DE_PREVISION_DE_CAPA_IA_TRAZABLE_POR_AGENTE_ESPECIALIZADO.md`
- `docs/calidad/README.md`
- `docs/calidad/ACTA_TECNICA_DE_VERIFICACION_DEL_HITO_1_BASE_SEGURA_2026_03_23.md`
- `docs/calidad/ACTA_TECNICA_DE_MICROAUDITORIA_H2_PRE_NO_BLOQUEO_2026_03_23.md`
- `docs/calidad/ACTA_TECNICA_DE_AUTORIZACION_RESTRINGIDA_DE_APERTURA_DE_H2_2026_03_23.md`
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
