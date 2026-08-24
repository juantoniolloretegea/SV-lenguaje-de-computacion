# Arquitectura del frente operativo

**Fecha y Versión: V.1 del conjunto**  
**Fecha:** 24 de agosto de 2026  
**Versión del conjunto:** V.1 del conjunto  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Titularidad y autoría:** © Juan Antonio Lloret Egea, 2026. Este conjunto se distribuye con atribución explícita de autoría y bajo la licencia indicada, sin autorización para apropiación de la paternidad intelectual del Sistema Vectorial SV.  

---

Esta carpeta reúne los documentos de arquitectura y gobierno técnico del frente activo del Lenguaje SV.

## Piezas vigentes de referencia

- `ACTA_DE_APERTURA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV_2026_03_19.md`
- `HOJA_DE_RUTA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`
- `CRITERIOS_DE_CIERRE_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`
- `NOTA_DE_ARQUITECTURA_MINIMA_DEL_NUCLEO_ENGANCHABLE_DEL_LENGUAJE_SV.md`
- `CONTRATO_DE_ENGANCHE_DE_INTERFACES_FUTURAS_Y_ABI_SEMANTICO_DIAGNOSTICO_MINIMO.md`
- `MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`
- `INFORME_DE_CONTINUIDAD_DEL_FRENTE_BASICO_Y_HABILITACION_DEL_BACKEND_SV.md`
- `CONTRATO_ABSTRACTO_DE_AUTORIDAD_CONSTITUCION_Y_GENESIS_SEC0_A_V2_2026_08_21.md`
- `ADENDA_CORRECTIVA_SEC0_A_UNIDAD_DE_GENESIS_Y_CONTINUIDAD_AUTORITATIVA_2026_08_22.md`
- `CONTRATO_ABSTRACTO_DE_DIAGNOSTICO_Y_FALLO_CERRADO_SEC0_D_2026_08_21.md`
- `ADENDA_DE_ALCANCE_TEMPORAL_A_ESPECIFICACION_ENTORNO_SOBERANO_SV_V0_2026_08_22.md`
- `ACTA_TECNICA_DE_APERTURA_R1_AUTORIDAD_MEDIACION_Y_DECISIONES_PROTEGIDAS_2026_08_24.md`
- `CONTRATO_R1_0_TIPOS_CERRADOS_Y_FRONTERAS_DE_CONSTRUCCION_2026_08_24.md`
- `ADENDA_R1_0_ALCANCE_AUTORIDAD_CONTINUIDAD_Y_FRONTERA_DE_R1_2026_08_24.md`

## Regla de lectura

Las piezas históricas del frente básico conservan su valor dentro del alcance para el que fueron emitidas. La realización soberana se interpreta además conforme a la especificación del entorno de ejecución soberano y a sus contratos SEC.0 vigentes.

Para R1, el orden de lectura material es:

1. cierre integral de R0;
2. especificación arquitectónica del entorno soberano y su adenda de alcance temporal;
3. SEC.0-A y la corrección de T-0 por continuidad autoritativa;
4. SEC.0-D sobre `Req`, `D-A/D-R/D-N` y fallo cerrado;
5. acta técnica de apertura de R1;
6. contrato de R1-0 y su adenda de alcance;
7. contratos y registros de cada corte R1-n conforme se integren.

La sede doctrinal superior permanece fuera de esta carpeta y fuera de este repositorio.

## Estado de realización

```text
R0 = CERRADO
R1 = ABIERTO
R1-0 = CERRADO
R1-1 = EN DESARROLLO
R2 = NO INICIADO
R3 = NO INICIADO
R4 = NO INICIADO

Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```

R1-0 fija los tipos cerrados y las fronteras de construcción del control soberano. Su adenda de alcance precisa que la autoridad de R1 es intra-proceso, que la continuidad lógica no se identifica con una instancia técnica y que la mediación acreditable en R1 termina en la frontera de `sv_core`.

R1 queda limitado a autoridad, mediación y decisiones protegidas dentro del proceso soberano. La apertura no atribuye persistencia material, recuperación durable, aislamiento de plataforma, raíz de confianza, cadena de suministro ni resistencia adversarial integral del sistema completo.