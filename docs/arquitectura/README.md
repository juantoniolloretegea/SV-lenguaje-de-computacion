# Arquitectura del frente operativo

**Fecha y Versión: V.1 del conjunto**  
**Fecha:** 25 de agosto de 2026  
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
- `CONTRATO_R1_1_FORMAS_AUTORIDAD_ENVOLVENTE_Y_DOMINIO_GOBERNADO_2026_08_24.md`
- `CONTRATO_R1_2_TRANSICIONES_Y_GENESIS_2026_08_24.md`
- `ACTA_TECNICA_CIERRE_R1_2_TRANSICIONES_AUTORIDAD_Y_GENESIS_2026_08_25.md`
- `CONTRATO_R1_3_REQ_APLICABILIDAD_Y_RESULTADOS_2026_08_25.md`
- `ACTA_TECNICA_APERTURA_R1_3_REQ_APLICABILIDAD_Y_RESULTADOS_2026_08_25.md`

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
R1-1 = CERRADO
R1-2 = CERRADO · INTEGRADO
R1-3 = ABIERTO
R1-4 = NO INICIADO
R2 = NO INICIADO
R3 = NO INICIADO
R4 = NO INICIADO

Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```

R1-0 fija los tipos cerrados y las fronteras de construcción del control soberano. Su adenda de alcance precisa que la autoridad de R1 es intra-proceso, que la continuidad lógica no se identifica con una instancia técnica y que la mediación acreditable en R1 termina en la frontera de `sv_core`.

R1-1 fija la estructura de las formas, la autoridad acotada, `E_max` y `D_a`. La envolvente conserva la descripción completa del efecto y el dominio gobernado usa pertenencia exacta. La constitución productiva quedó reservada a las transiciones autorizantes del corte siguiente.

R1-2 queda cerrado e integrado. Materializa T-0 como única vía productiva de autoridad de su corte y exige simultáneamente una premisa constituyente externa opaca no consumida y una continuidad lógica no habitada. Una génesis válida ocupa la continuidad y consume la premisa; una génesis rechazada no produce estado parcial ni consume la premisa. Las referencias de formas y autoridades son únicas dentro del estado inicial. T-I, T-V, T-H y T-E permanecen no autorizantes. T-G, T-C y T-R pueden existir como clases constituidas, pero no aplican cambios de autoridad mientras no se materialicen los requisitos, resultados de comprobación y la mediación posterior. La representación no acredita continuidad material entre procesos ni permite deducir una nueva génesis de un reinicio técnico.

El campo de estado conservado en el contrato de R1-2 corresponde al candidato previo a la integración; el estado vigente del corte queda fijado por su acta técnica de cierre de 25 de agosto de 2026 y por este índice.

R1-3 está abierto sobre el contrato de requisitos, aplicabilidad y resultados de comprobación. `N(F,e|C)` y `Applicable(V,q,C)` deberán derivar de estado previamente constituido y no podrán suministrarse como decisiones libres del acto de comprobación. El corte termina en `CheckResult` y su agregación técnica: no produce `Permit`, no ejecuta efectos y no hace productivas T-G, T-C o T-R. R1-4 permanece no iniciado.

R1 queda limitado a autoridad, mediación y decisiones protegidas dentro del proceso soberano. La apertura no atribuye persistencia material, recuperación durable, aislamiento de plataforma, raíz de confianza, cadena de suministro ni resistencia adversarial integral del sistema completo.