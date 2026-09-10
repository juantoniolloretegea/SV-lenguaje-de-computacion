# INFORME-001 · consultas ES27

## Entrada
- Confirmación de entrega-001.json: `2f8d3dd5b92e88b62940ed41262282179be59e7c`
- URL de la entrega: https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/2f8d3dd5b92e88b62940ed41262282179be59e7c/laboratorio/tareas-grok/consultas-es27/entrega-001.json
- Fuente fijada del puesto (workflow): `9ca6036afe7387431b7591791ddbce74498ff9c9`

## Ejecución observada
- Flujo: Consultas ES27 - Rust nativo y WASM
- Run: https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/actions/runs/34476110251
- Número: 2; intento: 1; conclusión: success
- Trabajo: https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/actions/runs/34476110251/job/102867141879
- Artefacto: `consultas-es27-34476110251-1` (id 10151533878). El nombre `consultas-es27-RUN-INTENTO` del encargo no coincidió literalmente.
- Resultado leído por la línea `RESULTADO_ENTREGA_BASE64` del paso «Ejecutar consultas», decodificada como UTF-8. No se sustituye por la propuesta propia.
- `paridad` observada: true. `repeticiones_por_destino`: 3. `entrada_bytes`: 1173. `entrada_sha256`: `d0b46324e0a3f5039d028a39a739d7ec3311d67a9696d338802a30b2da84896e`.
- Resumen del banco en el mismo registro: `ENTREGAS_EJECUTADAS_PENDIENTES_DE_RECEPCION`; `casos`: 56; `paridad`: true; `entregas`: 1.

## Respuestas efectivamente observadas

| id | propuesta_ia | estado observado | respuesta observada |
| --- | --- | --- | --- |
| G01 | Madrid | DATO | Madrid |
| G02 | Madrid | SOLICITUD_NO_REPRESENTADA | (vacía) |
| G03 | Madrid | DATO | Madrid |
| G04 | Madrid | DATO | Madrid |
| G05 | Madrid | DATO | Madrid |
| G06 | SIM-007 | SOLICITUD_NO_REPRESENTADA | (vacía) |
| G07 | Madrid; SIM-007 | SOLICITUD_NO_REPRESENTADA | (vacía) |
| G08 | SIM-007; Madrid | SOLICITUD_NO_REPRESENTADA | (vacía) |
| G09 | Madrid | SOLICITUD_NO_REPRESENTADA | (vacía) |
| G10 | SIM-007 | SOLICITUD_NO_REPRESENTADA | (vacía) |
| G11 | Sin dato en la ficha | SOLICITUD_NO_REPRESENTADA | (vacía) |
| G12 | Fuera de cobertura | SOLICITUD_NO_REPRESENTADA | (vacía) |

## Coincidencias y discrepancias
- Coinciden propuesta y dato observado en G01, G03, G04 y G05: `Madrid`.
- G02 (reformulación «capital española») no fue representada por el puesto.
- G06 a G12 no devolvieron dato. El marcador SIM-007, los dos órdenes de registros, la pregunta sin ficha y la petición fuera de cobertura quedaron en `SOLICITUD_NO_REPRESENTADA`.
- No se declara aprobada la entrega ni validada la arquitectura. La recepción corresponde a Juan Antonio y Watson.

## Límites de sesión
- No se instaló compilador Rust en esta sesión.
- No se abrió el ZIP del artefacto en disco; el JSON de entrega se obtuvo por `RESULTADO_ENTREGA_BASE64`.
- No se modificaron fuentes, reglas, workflow ni otras carpetas.
