# Apertura IE-004 y comprobación del puesto

Fecha: 10/09/2026. Asiento: RETP-2026-119. Estado: PUESTO_DISPONIBLE; IDENTIFICACION_DE_GROK_PENDIENTE; BRECHA_DE_LIGADURA_SEMANTICA_CONFIRMADA; AISLAMIENTO_NO_VERDE.

## Decisión y encargo

Se materializa la luz verde de Juan Antonio para comprobar la comprensión de peticiones españolas equivalentes y su subordinación, con campaña acotada. El [protocolo de fase 004](FASE_004_INTERPRETACION_SUBORDINADA_ES.md) sucede a las ampliaciones ES27; preserva el núcleo, los dominios y las deudas. [Encargo y paquete público de referencia](ie004/participante/ENCARGO.md). Grok deposita únicamente en la carpeta de laboratorio indicada allí, en una conversación nueva y sin expectativas suministradas. No se le exige instalar Rust: el puesto ejecutará su captura original en ambos destinos.

Primera entrega: 24 casos. Confirmación independiente: hasta 24. No hay tercera ronda automática; toda repetición técnica o corrección requiere un fallo identificado y preservado. El compromiso del primer oráculo se ha fijado antes de la entrega. [Compromiso](COMPROMISO-001.json). El oráculo salado queda reservado fuera de GitHub y se abrirá íntegro tras congelar los bytes de la captura.

## Resultado material previo a Grok

Fuente: f8f0fa26da55663bbf24f944d121c05eef8070c9. Ejecutor: 3c4d4d54813c8335be9d4a504a2cc529de4e043d. Workflow: 18f002e8d6c0f89e2914372d30eacd4ad2df5193. [Ejecución 34501044665/1](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/actions/runs/34501044665).

- Veinte controles de receptor: nueve con dato y once con diagnóstico; diez ejecuciones por destino, 200 resultados comprobados en cada uno.
- Seis tramas negativas por destino; rechazo técnico separado de los estados semánticos.
- Paridad literal nativo/WASI; misma fuente Rust 1.98.0. [Resultado](ie004/RESULTADO.json), [oráculos del receptor](ie004/oraculos-control.json), [fuentes públicas](ie004/fuentes/receptor.rs) y [reproducción](REPRODUCIR_IE004.md).
- Treinta y cuatro procesos medidos. Lote de 20 controles, incluyendo arranque: mediana nativa 3.429 ms y WASI 51.621 ms. Máximo RSS observado 2160/53832 KiB. [Muestras](ie004/medidas.json) y [resumen](ie004/MEDICION.json). No se comparan como regresión con ES27 ni se decide optimización.

**Hallazgo adversarial:** una propuesta IGG ante «No consulte IgG; consulte IgA» obtiene el dato de IGG en ambas rutas. La cita literal, el tipo y el permiso son válidos, pero la petición se ha identificado mal. El observador lo discrimina frente al esperado de IgA. [Testigo completo](ie004/LIMITE-SEMANTICO.json). Que el observador detecte la sustitución no significa que el receptor la impida. Esta brecha bloquea la promoción de esta ruta al SV incluso si Grok acertase todos los casos.

La ejecución previa del puesto 34500611593/1 queda [conservada](ie004/previo/RESULTADO.json). Después se encontró que el entorno local no dispone de GNU time: se registró el fallo de instrumentación y se ajustó el observador para declarar CPU/RSS no observables cuando falte. La segunda ejecución corresponde a esa modificación. Los dos binarios son idénticos byte a byte entre ambas ejecuciones; los archivos del observador y workflow conservan sus versiones distintas.

Se comprobó además localmente el recorrido de recepción con una captura sintética y una sustitución deliberada: una introducida y una localizada. No es una participación de Grok. Sus detalles que revelarían el oráculo de participación permanecen reservados hasta abrirlo; no se contabiliza como evidencia de comprensión del modelo. Node local 24.19.0; tiempo monotónico, CPU/RSS no observables.

## Custodia y límites

Los dos ZIP se descargaron realmente y sus hashes, fuentes, workflow y ejecutables fueron cotejados. El ZIP actual tiene 1688843 bytes y SHA-256 1d244782819fe62ca8db0e67787fef5fc2cc5375a2e90bb67a0ad1bc575903f3; caduca 2026-10-10T16:16:50Z. El anterior tiene SHA-256 7de531dfa69aa26b9ff5db8c0729a5d9f18cee2a9940535b1212babcd75d593e. Los resultados, fuentes, oráculos de control y muestras esenciales quedan espejados en Git; no se presenta Actions como almacenamiento perpetuo de binarios.

Nativo: 4525816 bytes, SHA-256 0a8f1b5438129acf48ff153787f9d7cbd7664b168d1f7cb149cdf9ca9b8fe372. WASI: 2161612 bytes, SHA-256 1ba8c81971b85808d1d8c71703f0d829da3c41a6f44176cceddea12878dce4e3.

El receptor es experimental y no está conectado a sv_core/R1/Q0. No produce Frame ni álgebra SV; conserva literales artificiales y estados previamente fijados. La revocación del banco es una entrada impuesta por el conductor: no prueba todavía una carrera real entre decisión y uso. Permisos efectivos del conector de Grok y aislamiento material I01–I05 permanecen sin acreditar. No se interpreta contents:read del workflow como permiso del participante.

Claude ya ha contribuido a la revisión metodológica; este paquete prepara su reproducción pública, sin atribuírsela como ejecutada. El oráculo de participación se publicará tras la captura, de modo que la auditoría del resultado no necesite acceso privado. Qwen, catálogo/localización y fila 9 conservan sus estados. La siguiente decisión depende de identificación correcta, utilidad y límites de subordinación por separado, nunca sólo del success de Actions.
