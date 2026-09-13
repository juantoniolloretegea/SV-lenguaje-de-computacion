# S26 R07 · Resultados y recepción

**RETP-2026-234; S26 permanece en ejecución.**

Se ha ejecutado una campaña de ocho casos después de publicar el banco en Lenguaje `a243bd096502580d91fe5166ceb1ebb478f10681` y laboratorio `f8c7e21817ca4842ce0ad3904e023962419c9b3c`. Los dos árboles y sus referencias se cotejaron antes de lanzar el conductor. El banco, los auxiliares y el ejecutable conservaron sus SHA-256 después de la ejecución.

Entorno observado: Rust 1.98.0; Python 3.12.14. Ejecutable conductor `/tmp/s26-r07-observar`; campaña nueva `/workspace/scratch/cdd3907241dd/s26-r07-campana-01`. El compilador no se encontró mediante PATH en este lanzador sin inicio de sesión; sí respondió la instalación existente en `/opt/sv-rust-1.98.0/bin/rustc`, que se utilizó por ruta absoluta. No se reinstaló Rust ni se sustituyó el conductor por Python.

## Resultado material

| Caso | Observación | Interpretación |
| --- | --- | --- |
| P01 | Publicador normal: éxito y una llamada update_ref al doble local. | Control positivo satisfecho. |
| P02 | Base incompatible sin optimización: AssertionError; sin checkpoint ni llamada de escritura. | Guarda activa en este modo. |
| P03 | Mismo conflicto con `DEBUG=False`: update_ref local y PUBLICACION_VERIFICADA. | Elusión de guarda reproducida bajo `-O`. No hubo publicación GitHub real. |
| P04 | Checkpoint verified ajeno: devuelve el commit «anterior» sin consultar el doble ni exigir la entrada inexistente. | Reutilización sin nueva comprobación reproducida. |
| P05 | Asiento correcto: actualiza Sucesos CSV/MD, historial y RETP CSV/MD; revisión 1. | Control positivo satisfecho. |
| P06 | Revisión pedida 99 cuando corresponde 1, modo normal: AssertionError y archivos intactos. | Guarda activa en este modo. |
| P07 | Mismo asiento con `-O`: éxito y salida «S1 1 2»; registra revisión calculada 1. | Revisión solicitada incompatible aceptada; segunda elusión de guarda reproducida. |
| P08 | Markdown de Sucesos ausente: FileNotFoundError tras actualizar Sucesos CSV e historial; RETP CSV/MD intactos. | Estado parcial local reproducido ante error de archivo. |

**Ocho oráculos cumplidos: dos controles positivos, dos controles de rechazo y cuatro reproducciones de debilidades. Cero reparaciones acreditadas.** El código cero del conductor acredita únicamente esa concordancia. No acredita seguridad del publicador, del registrador ni del SV.

## Evidencia conservada

- [RESULTADOS.tsv](RESULTADOS.tsv): respuesta resumida del conductor Rust.
- [EVIDENCIA.tar.gz](EVIDENCIA.tar.gz): directorio completo de la campaña, con fuentes copiadas, fixtures, salidas, errores, terminación, llamadas al doble, checkpoints y registros posteriores. El Markdown ausente en P08 continúa ausente.
- [PRECOMPROMISO.json](PRECOMPROMISO.json), [banco previo](README.md) y [conductor](observar.rs): conservados sin editar tras ejecutar.

SHA-256 de EVIDENCIA.tar.gz: `531de46bfe752d6a7f1c1b8996d34afed711875a66cb3aa2ce63afcdf5174782`.

SHA-256 de RESULTADOS.tsv: `89e57f083228ae5effbcaa3181070f00ed5202a6f372c48ef6759852d4c154a9`.

## Alcance de la conclusión y relevo

Las debilidades dejan de ser únicamente observaciones estáticas: están reproducidas en las copias identificadas, con los estímulos y límites del banco. No se afirma que ocurrieran en campañas históricas, ni que un fallo del auxiliar pruebe un fallo de la semántica SV. No se han ensayado corrupción física de RAM, corte eléctrico, transacción remota, compromiso del host ni los ocho discriminadores R06.

El transporte de publicación es un doble local. El error y las escrituras de P08 ocurren realmente en archivos locales; su comportamiento no constituye prueba de durabilidad o recuperación después de reinicio. Las cuotas y el observador conservan los límites declarados antes de la campaña.

La recepción de resultados se realiza mediante ediciones explícitas y conector GitHub directo, sin utilizar los auxiliares examinados para administrar o publicar este expediente. Python ha servido como objeto de prueba bajo la justificación previa. El conductor y los oráculos son Rust; la coordinación de herramientas y el cotejo documental de publicación son JavaScript del entorno. Esa coordinación no se presenta como protección material del núcleo.

Siguiente trabajo: preparar una reparación acotada, con Rust como primera opción para control duradero, que preserve estos contraejemplos y sus positivos. La sustitución debe tratar guardas, aplicabilidad de checkpoints y consistencia del conjunto escrito; traducir `assert` no resuelve la escritura parcial. No se reutilizarán los auxiliares afectados como fundamento de una garantía que estas sondas contradicen. R06 mantiene su continuación; S26 no cierra casos globales ni altera Bis, S24, dominios o selección tecnológica.
