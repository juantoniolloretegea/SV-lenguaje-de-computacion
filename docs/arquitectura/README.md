# Arquitectura del frente operativo

**Fecha y Versión: V.1 del conjunto**  
**Fecha:** 4 de abril de 2026  
**Versión del conjunto:** V.1 del conjunto  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Titularidad y autoría:** © Juan Antonio Lloret Egea, 2026. Este conjunto se distribuye con atribución explícita de autoría y bajo la licencia indicada, sin autorización para apropiación de la paternidad intelectual del Sistema Vectorial SV.  

---


Esta carpeta reúne los documentos de arquitectura y gobierno técnico del frente activo del lenguaje SV.

## Piezas vigentes de referencia

- `ACTA_DE_APERTURA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV_2026_03_19.md`
- `HOJA_DE_RUTA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`
- `CRITERIOS_DE_CIERRE_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`
- `NOTA_DE_ARQUITECTURA_MINIMA_DEL_NUCLEO_ENGANCHABLE_DEL_LENGUAJE_SV.md`
- `CONTRATO_DE_ENGANCHE_DE_INTERFACES_FUTURAS_Y_ABI_SEMANTICO_DIAGNOSTICO_MINIMO.md`
- `MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`
- `INFORME_DE_CONTINUIDAD_DEL_FRENTE_BASICO_Y_HABILITACION_DEL_BACKEND_SV.md`

## Regla de lectura

Estas piezas deben leerse de forma acumulativa y subordinada:

1. apertura del frente y hoja de ruta;
2. criterios de cierre del frente final;
3. nota de arquitectura mínima del núcleo enganchable;
4. contrato mínimo de enganche y ABI semántico-diagnóstico;
5. marco de estabilidad, resiliencia y horizontes del lenguaje;
6. informe de continuidad del frente básico y habilitación prudente del backend.

La sede doctrinal superior permanece fuera de esta carpeta y fuera de este repositorio.

## Estado de hito actualmente acreditado

La arquitectura vigente permite sostener ya, en régimen auditado y subordinado, `H1 — Base segura`, siempre sin convertir esa verificación en cierre total del frente ni en habilitación automática de `H2`, `H3` o del backend. La formalización de esa verificación se registra en `docs/calidad/ACTA_TECNICA_DE_VERIFICACION_DEL_HITO_1_BASE_SEGURA_2026_03_23.md`. Además, y sobre la base de `H2-pre` como dictamen de no bloqueo, el estado actual autoriza ya la **apertura restringida de `H2`** como frente arquitectónico-auditor de trabajo, sin verificación positiva todavía del hito ni habilitación automática de implementación fuerte o backend. Véase `docs/calidad/ACTA_TECNICA_DE_AUTORIZACION_RESTRINGIDA_DE_APERTURA_DE_H2_2026_03_23.md`.
