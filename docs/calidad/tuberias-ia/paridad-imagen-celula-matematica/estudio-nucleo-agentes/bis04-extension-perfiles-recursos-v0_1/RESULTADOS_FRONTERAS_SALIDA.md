# Resultado de las fronteras conjuntas de salida

**14/09/2026 · S22 · RETP-2026-237 · W-S26-02.**

Las dos campañas Rust cumplen los doce oráculos por campaña: **24 invocaciones, cero discrepancias**. Cada campaña contiene seis entregas y seis rechazos. Las cuatro sensibilidades del observador se detectan en cada perfil; actúan sobre evidencia ya producida y no añaden invocaciones al sujeto.

## Banco anterior a la ejecución

[Oráculos y alcance previo](BANCO_FRONTERAS_SALIDA.md). Publicación previa en Lenguaje: `0452670b1d78c581592acca7740814c1c4cdef05`; laboratorio: `74c013fc4d0cef1aa6e0b09486e9709700324ae5`. El cotejo R08 de los árboles completos resultó conforme a las 14:46:49 UTC: 3022 entradas de archivo en Lenguaje y 2571 en laboratorio. Las campañas comenzaron después, a las 14:47:40 y 14:48:31 UTC respectivamente.

Las doce variantes corresponden a seis escenarios, cada uno con fuente EN y ES. Ningún esperado se corrigió después de ejecutar. Las 157 huellas comprometidas de las entradas se releen y coinciden al finalizar. Núcleo, admisor, semántica e IR permanecen íntegros.

## Resultado observado

| Escenario, en ambos idiomas | Resultado en ambos perfiles | Primera guarda y efecto |
| --- | --- | --- |
| FS01: descriptor 8191; recibo 2048 | Entrega íntegra; suma 10239 bytes | Sin rechazo; un despacho. |
| FS02: descriptor 8192; recibo 2048 | Entrega íntegra; suma 10240 bytes | Sin rechazo; un despacho. |
| FS03: descriptor 8192; recibo propuesto 2049 | Rechazo; salida 0 bytes | R01, `output receipt BYTES`; doce guardas previas, cero despachos. |
| FS04: descriptor 8193; recibo propuesto 2048 | Rechazo durante recepción | R01, `BYTES`; traza de admisión vacía, lectura del byte detector, cero despachos. |
| FS05: frontera exacta con consumidor UTF-8 «ñ» | Entrega íntegra; suma 10240 bytes | Sin rechazo; conteo por bytes. |
| FS06: recibo de 2049 bytes con UTF-8 «ñ» | Rechazo; salida 0 bytes | R01, `output receipt BYTES`; cero despachos. |

En las entregas se cotejan recibo completo, descriptor capturado, vector, longitudes, lecturas y estado posterior. En los rechazos se conservan causa, precedencia, ausencia de salida y ausencia de despacho. Las sensibilidades detectan recibo aumentado, descriptor truncado, suma declarada falsa y despacho declarado tras rechazo.

El exceso agregado no es alcanzable si se cumplen los dos límites individuales: `8192 + 2048 = 10240`. **No se atribuye ejecución a la rama de rechazo agregado.** La evidencia cubre la composición de las fronteras y los rechazos individuales que impiden la salida excesiva.

## Ejecución, fuentes y trazabilidad

Rust/cargo 1.98.0, núcleo completo mediante `cargo build --locked --offline`. Perfil debug y perfil optimizado con `opt-level=3`, `debug-assertions=no` y `overflow-checks=no`. Concurrencia de ensayo 1; sin red, reintentos ni Python durante las campañas. Retorno de ambos binarios: 0. Duraciones internas observadas del conductor: 55667530 ns y 19623945 ns; no son presupuestos ni garantías temporales.

[Resultados estructurados](RESULTADOS_FRONTERAS_SALIDA.json) conserva los 24 registros observados. [Evidencia completa](EVIDENCIA_FRONTERAS_SALIDA.tar.gz) conserva los canales, capturas, discrepancias vacías, resúmenes, versiones, órdenes, retornos, huellas e incidencias. [Binarios ejecutados](BINARIOS_FRONTERAS_SALIDA.tar.xz) conserva las dos realizaciones exactas. La evidencia incluye un manifiesto SHA-256 de sus archivos; las huellas externas de ambos paquetes figuran en [ARCHIVOS_FRONTERAS_SHA256.txt](ARCHIVOS_FRONTERAS_SHA256.txt).

Las fuentes del preparador, banco, registrador de comandos y receptor de Sucesos quedaron publicadas antes de la campaña. El consolidador y el preparador del cotejo de publicación son auxiliares documentales; no ejecutan el SV ni generan los esperados. El registrador conserva las órdenes que se le pasan, sus canales y retorno; no es una instrumentación general del sistema operativo ni demuestra resistencia a un anfitrión adversario.

## Incidencias conservadas

La instalación del entorno se recuperó desde el archivo oficial Rust 1.98.0, SHA-256 `ed8ee2df70909c88cbaf87a6cfa3920dac00b537de12a6abe6906641e0f5952f`, comprobado frente al fichero oficial. La biblioteca LLVM instalada apareció truncada y se repuso desde ese mismo archivo antes de verificar rustc. Se conservan los canales de instalación. Esta recuperación no instala rustup; para reproducir estas campañas se utilizan rutas explícitas a rustc/cargo.

Antes del precompromiso se corrigió una ambigüedad de tipo del preparador y una invocación Cargo que no localizaba rustc en PATH. En el cotejo de publicación, la primera exportación de rutas Git escapadas fue rechazada por R08 (`PUB_RUTA`); la exportación literal mediante `core.quotePath=false` superó el cotejo antes de ejecutar. No se desactivó ninguna guarda de R08.

Después de las campañas, el primer consolidador intentó leer el manifiesto completo con el lector JSON de cuota 16384 y fue rechazado (`BYTES`, retorno 101). Se conserva esa fuente y su diagnóstico. La versión aplicada lee por registro el perfil plano del manifiesto generado y mantiene la cuota de cada lectura JSON; no modifica el lector del admisor, las entradas ni los resultados. La consolidación posterior termina con retorno 0. Ninguna de estas incidencias se cuenta como prueba funcional del banco.

## Alcance recibido y relevo

Se recibe una cualificación local de estas fronteras de salida. No cierra Bis, los escenarios originales C04/C05 ni S26. No acredita imagen renderizada, lectura visual por IA, consulta histórica de Frame, autenticación externa, memoria física, cuotas de RAM/tiempo o continuidad del host. Las 202 filas originales C02–C12 y los 24 escenarios globales Bis conservan sus recuentos históricos: 2 ejecutados y 22 pendientes.

El siguiente paso queda concretado en [RELEVO_REPRESENTACION_CONSUMO.md](RELEVO_REPRESENTACION_CONSUMO.md). S24 mantiene Bis → catálogo y cierre de fase → GUI, con la indicación C#/.NET ya recibida para ese momento. S28 permanece finalizado; esta campaña no modifica su acta ni requiere nuevas copias excepcionales en agentes o dominios.
