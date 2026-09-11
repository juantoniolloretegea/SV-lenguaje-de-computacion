# G1 nativo: recepción custodiada comprobada en memoria

**RETP-2026-144 · 11/09/2026 · Responsable: Watson.** Continuación autorizada por Juan Antonio Lloret Egea. **Estado: CANDIDATA_G1_CONFORME_INTRAPROCESO_TRAS_CORRECCION.** La trazabilidad integral sigue sin acreditar.

## 1. Objeto realizado y corte

Se implementa el [contrato G1/1](CONTRATO_RECIBO_CUSTODIADO_IE004_G1_1.md) mediante una candidata Rust nativa que recibe la entrada, llama a A, conserva marco y traza, comprueba su correspondencia y permite recuperar los bytes. V se anexa después, con identidad de invocación y sin modificar A. El recibo completo acredita recepción y recuperación en el alcance declarado; puede contener un rechazo o fallo técnico correctamente documentado.

Cortes de partida: Lenguaje `aab69e561a06137a78cbe4c9700cb99c8ea376f7`; laboratorio `4a81191fba7e32ff297efbcebaa448fbde04d85e`. Se cotejaron las rectoras previamente leídas: AGENTS (`42221257270146fa94a25e8aca20a2c364c077b7`), Pilares (`bba81c4ab115899fc4ae812bc7214aab6f39a9bd`), perfiles y ensamblaje (`1df9b818f3f531f628fe0e89dd2c497b5da52d4d`), transición secuencial (`44f8fda87856ab16007195cab324d40a758e506a`) y arquitectura núcleo/frontera/host (`6c57f2e895045d4a26bacefc98c49ebac2b433f7`). Conservan sus blobs. Rigen el [workflow V2](../../WORKFLOW_ACOTADO_SUBORDINACION_IA_ES_V2_2026_09_11.md) y RETP-142/143.

La sede es el banco experimental: [lib.rs](candidata/lib.rs), [lector de traza](candidata/json_vista.rs), [llamadas a A/V](candidata/av.rs), [testigos Rust](candidata/tests.rs). Las fuentes originales A/V se reutilizan sin modificación. La recepción y comprobación son Rust; [ejecutar.mjs](candidata/ejecutar.mjs) compila, ejecuta y conserva evidencia del ensayo.

## 2. Qué permite comprobar

| Obligación | Realización y contraste |
| --- | --- |
| Pertenencia | Manejador opaco con ámbito y ordinal emitidos por el custodio; intercambio entre custodios rechazado. Construcción directa desde otra crate rechazada por el compilador. |
| Completitud A | Captura acotada de original; marco y traza íntegros, hashes, montaje y ligaduras comprobados antes del recibo. Ausencia, truncación y discordancia no producen recibo completo. |
| Recuperación | Conservación de bytes propios y devolución por referencia inmutable; cotejo de longitud y hash al recuperar. La pérdida posterior se informa sin reescribir el estado histórico. |
| Anexado V | Apertura posterior a A, cierre único, propuesta y anexo recuperables; vínculo con cuerpo y lote. Rechazo de propuesta completa conserva su evidencia. |
| Fallo y ausencia | Interrupción notificada por el supervisor, lectura incompleta y decisión explícita de no solicitar V son estados distinguibles. Un fallo técnico de A puede tener recibo completo. |

La identidad de realización —hashes de fuentes y binario— la aporta el conductor confiable y se conserva como declaración atribuida. No es autenticación independiente frente a un host comprometido. El lector valida el perfil canónico emitido por A/V y las ligaduras descritas; no repite un análisis semántico independiente de todos los pasos internos.

## 3. Fijación y resultado

[Perfil y plan](candidata/PERFIL_Y_PLAN.json) fijados antes de la primera ejecución: los 18 testigos RETP-143, dos positivos adicionales y siete pruebas de fronteras/recuperación. G1-19 recibe una petición completa rechazada por transporte; G1-20 recibe una petición pública de 129 tokens que produce `A_TOKENS_LIMITE`. En ambos casos hay cuerpo y traza completos; el fallo funcional no se transforma en `U` ni en dato.

| Etapa conservada | Debug | Release | Lectura |
| --- | --- | --- | --- |
| [Inicial](candidata/evidencia/inicial/RESULTADO.json) | 27/27 | 27/27 | Primera cobertura conforme; no cerraba los dos huecos descubiertos después. |
| [Adversarial](candidata/evidencia/adversarial/RESULTADO.json) | 27/29 | 27/29 | Dos fallos reproducidos antes de corregir. |
| [Corregida](candidata/evidencia/corregido/RESULTADO.json) | 29/29 | 29/29 | Una corrección causal del receptor y mismos contraejemplos. |

La [ampliación adversarial](candidata/AMPLIACION_ADVERSARIAL.json) fijó estos contraejemplos antes de aplicar la corrección:

1. **V entrega `abc` y después falla la lectura.** El receptor notificaba error pero perdía ese fragmento. Ahora lo conserva con marca de captura incompleta y causa de E/S; A permanece intacta.
2. **Un contador numérico de la traza se sustituye por `null`.** La validación previa aceptaba esa forma. Ahora exige los tipos de los campos obligatorios y rechaza la traza con error de esquema.

Las cápsulas [inicial](candidata/evidencia/inicial/CAPSULA.json), [adversarial](candidata/evidencia/adversarial/CAPSULA.json) y [corregida](candidata/evidencia/corregido/CAPSULA.json) conservan capturas, comandos, códigos de salida, stdout/stderr y fijaciones. Los [antecedentes de fuentes](candidata/evidencia/ANTECEDENTES_FUENTES.json) conservan los bytes anteriores de `lib.rs` y `tests.rs`, cotejados contra sus fijaciones; el fallo no queda sustituido por el éxito final.

En la etapa corregida, los 20 testigos G1 pasan en ambas compilaciones. Se compararon **19 archivos capturados**, idénticos entre debug y release, correspondientes a G1-01, G1-10, G1-11, G1-19 y G1-20. Esa paridad no se extiende a los binarios ni a tiempos. La biblioteca y un uso externo válido compilan; la forja del manejador desde otra crate falla como se esperaba. Las advertencias de compilación permanecen en los registros.

## 4. Recursos y límite material

El perfil fija cuatro aperturas por custodio, 65.536 B de entrada A, 4.144 B de marco, 16.842.752 B de traza, 262.144 B de propuesta V y 8.192 B de anexo. La cuenta acumulada de reservas del custodio tiene techo de 64 MiB y no se reintegra. Cada cierre carga conservadoramente 2 MiB por la decodificación temporal del original. A/V conservan sus cuentas propias.

El contador G1 acota recorrido JSON y comprobaciones declaradas, con un máximo de 1.000.000.000 por cierre. No cuenta todas las instrucciones del proceso ni equivale a las unidades /2 o /3. En A01 se registraron 2.229.790 B en la cuenta de reservas del custodio y 49.226 unidades de esta validación. No son una medida de RSS ni una cota universal del servicio.

Se ensayaron las fronteras de captura A/V, reserva agregada, trabajo, slots y ordinal, además de JSON duplicado, cola, escapes, profundidad 96/97 y más de 8.192 valores. La frontera máxima de traza se prueba en la guarda de tamaño; no acredita una traza semántica válida de ese tamaño.

La conservación dura **la vida del custodio en memoria**. No hay recuperación tras caída del proceso. Un `Read` bloqueado requiere un supervisor del conductor; G1-12 inyecta su notificación de interrupción y no acredita un plazo real ni interrupción concurrente de una lectura bloqueada. Este ensayo es nativo x86_64 Linux, sin nueva paridad WASI.

## 5. Reproducción y continuidad

Desde una copia completa del repositorio, con Node y Rust disponibles, ejecutar:

```sh
node docs/calidad/tuberias-ia/ie004/recibo-g1/candidata/ejecutar.mjs /ruta/absoluta/rustc /ruta/absoluta/salida-nueva
```

El conductor exige un directorio de salida nuevo, restaura las 14 fixtures desde la cápsula pública RETP-143 y comprueba sus bytes y huellas. Fija fuentes antes de compilar y vuelve a cotejarlas al terminar. El entorno observado utilizó Rust `1.98.0 (88d9e12ae 2026-08-18)`, debug con comprobación de desbordamientos y release sin ella. El [manifiesto corregido](candidata/evidencia/corregido/FIJACION_PREVIA.json) tiene SHA-256 `25591d35c6b118201fad0408a4b04680224ab8f3c8d9cdec75daa9aaa211bb9f`. Los hashes de binarios y resultados están en la cápsula; una recompilación en otro entorno no promete identidad del binario.

Para reproducir los fallos anteriores se restauran los archivos de la etapa correspondiente de `ANTECEDENTES_FUENTES.json` en una copia separada y se coteja su fijación. La ampliación adversarial no formaba parte de la fijación inicial. Las capturas originales permanecen como evidencia primaria de cada etapa.

**Siguiente objeto único:** conectar el adaptador público del lote existente con este receptor y comprobar que conserva la pertenencia de petición y montaje hasta el recibo recuperable. La candidata individual todavía no sustituye ese conductor. Es el enlace inmediato de G1; no abre otra ronda de modelos ni la reserva.

G2 —operación gobernada—, G3 —inscripción y Frame exigibles— y G4 —recuperación material— conservan su sede y condiciones. Esta publicación no modifica gramática, IR, núcleo, `Σ`, `U`, Frame, constitución de dominio/agente, oráculos o compromiso. No constituye una célula de preguntas ni convierte cada consulta en SUCESO. La asociación /2–/3, sus sucesiones pendientes, P3 reservado, P4 y P5 continúan sin acreditar por esta entrega. El retorno posterior sigue siendo catálogo/localización y continuación de fila 9.
