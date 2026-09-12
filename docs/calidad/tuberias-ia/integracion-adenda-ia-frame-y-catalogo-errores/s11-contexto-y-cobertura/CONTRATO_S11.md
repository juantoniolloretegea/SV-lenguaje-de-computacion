# S11 — Documento externo, cobertura e identidad contextual

Versión S11-CONTEXTO-COBERTURA/1. Ensayo de laboratorio de la integración 1+3. Responsable: Watson / W-S0. Autorización: continuación expresa de Dirección tras S10. Corte público de entrada: `81cef96616fa2a005e1ce1730ccac2309de7baa4`; laboratorio: `310461affc7a047912ecb66226d095f97acca3d8`, rama `lab/playground-sv-permanente`.

## Obligación y suficiencia

La matriz A exige conservar una orden externa como dato y detectar la omisión de una dependencia requerida; H exige contexto íntegro y perteneciente a la invocación. El recorrido constituido aquí es documental: recibo negativo P3-11, referencia independiente del caso y de su vigencia, documento recibido y selección propuesta, comprobación y archivo final. Leer el recibo negativo no concede permiso ni ejecuta una actuación profesional.

Se reutilizan sin cambios la cápsula de fuentes S2, su `Referencia`, `Seleccion` y `Entrega`, y el conductor de archivo S3. La cobertura existente detecta omisión o sustitución de la vigencia, pero su contrato no incluye un documento externo adicional. Se ensaya sólo en laboratorio un envoltorio `Contexto` que toma prestados la referencia y los bytes de ese documento. Comprueba límite, identidad y fidelidad antes de llamar a la cobertura existente. El límite de 8192 bytes es presupuesto de este montaje, no umbral doctrinal ni constitución de un dominio.

No añade semántica, campos de IR, permisos, Frame ni SUCESO de dominio. El documento externo no interviene en la decisión ni modifica las referencias requeridas. El SUCESO S11 registra trabajo técnico. El código nuevo queda en calidad y laboratorio; no se promueve al núcleo.

## Recepción y observación

El conductor fija la referencia desde el enlace antes de construir las selecciones. Fija también el documento requerido, cuyo origen sintético está en `ESPECIMENES.json`. El sobre de identidad lo aporta el conductor confiable. Las selecciones adversarias son especímenes programados: ningún LLM recibe aquí el documento, y no se mide si un modelo obedece una instrucción indirecta.

Se guardan documento requerido y presentado, identidad, cuerpo propuesto, caso, presencia y bytes de vigencia, causa real y existencia del destino. La omisión se distingue de una cita vacía mediante `vigencia-presente`. El archivo aceptado contiene el recibo; el documento externo permanece en las capturas de auditoría. No se afirma que ese archivo contenga el contexto ni que un host adversario no pueda falsificar el sobre o el registro. La lectura final vuelve a comprobarse contra la referencia independiente.

## Casos fijados antes de compilar

| Caso | Variación | Resultado exigido |
| --- | --- | --- |
| AH01 | Documento neutro, contexto propio y citas completas | Recibo negativo recuperado idéntico |
| AH02 | Orden externa de omitir vigencia; citas completas | Mismo recibo; orden conservada como dato sin autoridad |
| AH03 | Misma orden; se omite la vigencia | `Cobertura(FaltaVigencia)`; destino ausente |
| AH04 | Documento neutro; misma omisión | Mismo rechazo; destino ausente |
| AH05 | Orden y contexto propios; vigencia de P3-01 | `Cobertura(VigenciaDistinta)`; destino ausente |
| AH06 | Mismos bytes y citas; identidad contextual de P3-01 | `IdentidadContexto`; destino ausente |
| AH07 | Se pierde el último byte del documento requerido | `DocumentoDistinto`; destino ausente |
| AH08 | Documento neutro sustituye al requerido bajo la misma identidad | `DocumentoDistinto`; destino ausente |
| AH09 | Se presentan 8193 bytes ante referencia de 8192 | `LimiteDocumento`; destino ausente |
| AH10 | Documento requerido y presentado de 8192 bytes | Recibo negativo recuperado idéntico |

AH01/AH02 y AH04/AH03 aíslan presencia de orden; AH02/AH03 aíslan omisión; AH02/AH06 aíslan identidad; AH10/AH09 aíslan un byte adicional. Se comprueba además que el constructor rechaza un documento requerido de 8193 bytes. El resultado negativo P3-11 se coteja con un esperado anterior de S2, no derivado de esta ejecución.

## Método y presupuesto cerrado

Rust 1.98.0, binario SHA-256 `3690cc576ede140504698405d5d8fa3826aaadbe71699c6c4ed0a565d6f493e2`, igual a S2/S3. Linux x86_64, edición 2021. Debug opt-level 0 y overflow-checks yes; release opt-level 3 y overflow-checks no. Tres ejecuciones por modo: diez casos, sesenta observaciones. Capturas idénticas entre las seis ejecuciones en el dominio determinista de este banco.

Dos controles de sensibilidad, cada uno con un único mecanismo desactivado: la obligación de aportar vigencia debe ser detectada en AH03; la comprobación de identidad contextual, en AH06. Los mutantes no son reparaciones ni candidatos de producción. En cada uno debe observarse exit 1 con el identificador fijado; un simple error de compilación no cuenta como sensibilidad.

Máximo 22 invocaciones: versión (1), compilaciones normales (8), ejecuciones normales (6), compilaciones del mutante de cobertura (4), su ejecución (1), compilación y ejecución del mutante contextual (2). Sesenta segundos por invocación, cero reintentos funcionales. Un fallo inesperado detiene la campaña y se conserva; no se cambian esperados para acomodarlo. Las preparaciones de archivos y recuperación de herramienta se declaran fuera de esas 22 invocaciones.

`reproducir.py` sólo verifica integridad, materializa bytes, orquesta compilación/ejecución, compara salidas y registra tiempos. La decisión y las comprobaciones del recorrido son Rust. Se registran argv, código de salida, stdout, stderr, tiempo de pared y CPU; RSS individual no disponible. No se atribuye tiempo de un modelo ni latencia profesional.

## Condiciones de cierre y límites

Se admite exclusivamente el resultado acotado si todos los casos cumplen la causa y efecto fijados, las capturas son idénticas y ambos mutantes se detectan. A/H reciben evidencia integrada en este montaje; no cierre universal. C/I se reutilizan, D llega al archivo según S3; B/E/F/G/J/K/L conservan sus obligaciones pendientes según la matriz reconciliada.

No se acredita comportamiento interno de modelos, pantalla o acto de revisión humana, autenticidad frente a un host hostil, historia durable, ausencia de canales encubiertos, cadena profesional, suficiencia completa del núcleo ni paridad global. No se modifican el paquete externo S4/S6, la reserva P3 ni sus condiciones de custodia. P4/P5/P6 y la secuencia de dominios y agentes permanecen vigentes. El catálogo ES/EN recibirá causas por etapa, sin convertir fallos técnicos en `U`.

Las piezas rectoras y sus blobs constan en `RECTORES.json`; se han leído completas en los cortes declarados. La suficiencia nuclear y cualquier promoción requerirán sus propios contrastes aplicables.
