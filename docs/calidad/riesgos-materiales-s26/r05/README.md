# S26 R05 · salidas y sustitución de fuente

RETP-231: banco previo. Continuación de R04 después de incorporación documental S27 / RETP-230. S26 permanece en ejecución.

## Necesidad y realización

La observación de bytes y el resultado de entrega tienen alcances diferentes. Un retorno normal rechazado puede permitir relectura; un pánico puede impedir el informe final. Abrir una ruta y leer el objeto abierto tampoco demuestra que la ruta siga designando ese objeto.

Se reutilizan conductor, observador, fixtures y ensamblaje fijados de R04. Sólo la copia de recepción añade un hook entre apertura y lectura; en el recorrido ordinario ese hook está vacío. El hook posterior a operación ya existía. Los hooks no son mecanismos de protección. Código íntegro en `probe.rs`, `recepcion.rs`, `conductor.rs` y `observer.rs`; diferencia exacta en `recepcion.patch`. No se modifica código productivo.

Herramientas: shell/Python para copias, huellas, registros y conservación; Rust/Cargo 1.98.0 offline para ejecutar las aserciones; conector GitHub mediante `soporte/github_io.py` y publicador existente para precompromiso y resultados. Sin delegación ni conexión clínica. Cada orden de campaña conserva argv, salida, error y código de salida. `catch_unwind` sólo pertenece al arnés.

## Banco previo

| Caso | Estímulo | Resultado exigido |
|---|---|---|
| P01 | Control intacto | Concordancia observada; un intento; recibo exacto. |
| P02 | JSON de petición incompleto | Rechazo de admisión, cero intentos; relectura posterior real y concordante; sin recibo. |
| P03 | Plan sin captura | D01 / NO_ACREDITADO, un intento; relectura concordante; sin recibo. Igualdad de bytes no acredita entrega. |
| P04 | Ausencia de contexto después de observación inicial | Pánico del loader; no retorna MaterialRun. El arnés captura unwind y lee estado por separado; no inventa número de intentos. |
| P05 | Pánico en hook posterior a operación y mutación de archivo | Hook alcanzado; estado externo alterado; no retorna MaterialRun ni relectura propia. No se atribuyen cero efectos, rollback o recibo reconstruido. |
| P06 | Sustitución de ruta tras open, antes de read, con bytes diferentes | Primer lector conserva bytes del objeto abierto; operación abre sustituto y no entrega; relectura posterior detecta alteración; inode de ruta cambió. |
| P07 | Sustitución de ruta tras open con bytes iguales | Concordancia y recibo exacto, pese al cambio de inode. Contraejemplo a deducir identidad del archivo a partir de igualdad de bytes. |

P04 y P05 son contraejemplos esperados: el éxito de la aserción acredita el límite observado, no la prevención del pánico. P07 tampoco acredita una protección frente a sustitución. Ningún contador desconocido se convierte en cero.

## Reproducción y límites

Desde la raíz del checkout: `python docs/calidad/riesgos-materiales-s26/r05/reproducir.py /ruta/nueva/campana`. Directorio nuevo obligatorio. Verifica PRECOMPROMISO.json y FUENTES.json, prepara copias y ejecuta Cargo con --locked --offline. Se fija el banco antes de compilar. Los resultados posteriores no alteran sus oráculos.

Linux local; sin aborto de proceso, pérdida de alimentación, escritura a mitad de lectura, transacción durable, base de datos, GUI o resistencia a un host que falsee observaciones. La lectura separada del arnés tras pánico no sustituye el informe que el conductor no emitió. No se promueve ninguna capacidad productiva. Los doce casos globales S26 siguen abiertos; S22 y Bis → catálogo/cierre → S24 conservan su orden. La cobertura de rutas queda situada por S27 en el dominio y su agente.
