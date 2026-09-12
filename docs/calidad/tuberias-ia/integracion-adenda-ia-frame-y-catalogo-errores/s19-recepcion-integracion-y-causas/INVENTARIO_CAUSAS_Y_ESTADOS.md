# Inventario recibido de causas y estados S18

20 tuplas distintas de etapa, código, detalle textual y rótulos: 3 estados conformes, 1 negativa y 16 variantes de fallo/rechazo por etapa. Son registros locales del banco; no se asignan códigos canónicos E… por semejanza. Se conserva el detalle literal, sin interpretarlo como una estructura técnica.

| Referencia documental | Etapa | Código local | Detalle emitido | ES / EN | Casos |
|---|---|---|---|---|---|
| S19-OBS-01 | cobertura | CONFORME | Conforme | Conforme / Conforming | I01, I02, I14, I15, I18, I19, I22, I23, I24 |
| S19-OBS-02 | recuperacion | CONFORME | Conforme | Conforme / Conforming | I01, I02, I14, I15, I23, I24 |
| S19-OBS-03 | archivo | CONFORME | Conforme | Conforme / Conforming | I01, I02, I14, I15, I19, I23, I24 |
| S19-OBS-04 | carga_base | BASE_DISTINTA | BaseDistinta | Base distinta / Different base | I03 |
| S19-OBS-05 | contexto | DOCUMENTO_DISTINTO | DocumentoDistinto | Documento distinto / Different document | I04 |
| S19-OBS-06 | contexto | IDENTIDAD | Identidad | Identidad distinta / Identity mismatch | I05 |
| S19-OBS-07 | cobertura | FALTA_SOLICITUD | FaltaSolicitud | Falta solicitud / Missing request | I06 |
| S19-OBS-08 | cobertura | SOLICITUD_DISTINTA | SolicitudDistinta | Solicitud distinta / Different request | I07 |
| S19-OBS-09 | cobertura | FALTA_BASE | FaltaBase | Falta base / Missing base | I08 |
| S19-OBS-10 | cobertura | BASE_DISTINTA | BaseDistinta | Base distinta / Different base | I09 |
| S19-OBS-11 | cuerpo | CONTENIDO_DISTINTO | Presentacion(ContenidoDistinto) | Contenido distinto / Different content | I10, I16 |
| S19-OBS-12 | recepcion | COMUNICACION | Comunicacion(ConnectionReset) | Fallo de comunicación / Communication failure | I11 |
| S19-OBS-13 | protocolo | NEGATIVA_PROVEEDOR | Negativa | Negativa del proveedor / Provider refusal | I12 |
| S19-OBS-14 | protocolo | ESQUEMA_INVALIDO | Esquema(Truncado) | Esquema inválido / Invalid schema | I13 |
| S19-OBS-15 | atribucion | IDENTIDAD | Identidad | Identidad distinta / Identity mismatch | I17 |
| S19-OBS-16 | apertura_archivo | IO | Io(AlreadyExists) | Fallo de entrada/salida / Input/output failure | I18 |
| S19-OBS-17 | recuperacion | CONTENIDO_DISTINTO | Presentacion(ContenidoDistinto) | Contenido distinto / Different content | I19 |
| S19-OBS-18 | instalacion_contexto | LIMITE_CONTEXTO | LimiteContexto | Límite de contexto / Context limit | I20 |
| S19-OBS-19 | recepcion | LIMITE_RECEPCION | LimiteRecepcion | Límite de recepción / Reception limit | I21 |
| S19-OBS-20 | escritura_archivo | IO | Io(WriteZero) | Fallo de entrada/salida / Input/output failure | I22 |

Los identificadores S19-OBS son filas documentales, no nuevos códigos del Lenguaje. Cada fila JSON enlaza capturas originales con bytes, SHA-256 y base64. Admission/escritura/resultado pueden repetir un diagnóstico: no representan pruebas adicionales. `CONFORME` en cobertura no convierte un fallo posterior en éxito global. `E0451` pertenece a rustc. La negativa del proveedor I12 no equivale al recibo documental `PERMISO_REVOCADO` correctamente entregado en I14.

El inventario S15 se conserva íntegro dentro del JSON. No se homologa su nomenclatura retrospectivamente.
