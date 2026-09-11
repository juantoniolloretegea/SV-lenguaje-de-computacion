# Enlace público del adaptador de lote con el recibo G1

**RETP-2026-145 · 11/09/2026 · Responsable: Watson.** Mandato de Juan Antonio Lloret Egea: continuar la segunda necesidad, trazabilidad. **Estado: CONFORME_ENLACE_PUBLICO_INTRAPROCESO.**

## 1. Resultado y alcance

El adaptador público existente queda conectado a G1 mediante [Rust nativo](lib.rs). El enlace admite el lote y su montaje completos, verifica los productos del adaptador y entrega cada solicitud a su custodio. Conserva simultáneamente los 24 recibos y permite recuperar entrada, marco, cuerpo y traza de cada posición, junto con el lote y montaje originales.

**24/24 cuerpos coinciden con las huellas y los esperados públicos históricos en debug y release.** Los 12 testigos del enlace pasan en ambas compilaciones y las 101 capturas son idénticas entre ellas. No se necesitó corrección de esta candidata. El lote mantiene 24 posiciones que reutilizan diez preguntas distintas; ni las posiciones repetidas ni las dos compilaciones amplían esa cobertura semántica.

El recorrido ejecutado es: lote y montaje públicos fijados → adaptador Rust existente → comprobación de petición y procedencia → G1 → recibo recuperable en memoria. V queda explícitamente **no solicitada** en esta ruta; su anexado individual conserva el resultado RETP-144 y no se vuelve a cualificar aquí.

## 2. Identidad y montaje que se conservan

La identidad de enlace contiene versión, SHA-256 del lote, SHA-256 del montaje y posición; se liga al ámbito y ordinal emitidos por G1. Un nombre `P3-xx` aislado no selecciona ni autoriza el montaje. Esta candidata acepta exclusivamente el lote público y montaje fijados:

- Lote: `88bda419f98d75c10be7cf91acc849e852904495a9e6deb4403a8429a1c2990a`.
- Montaje: `92750c47b491139f3abe4c007dcad9298346449bf1be9cd1779007278bae9eff`.

Se rechazan otros bytes, incluso si conservan los mismos identificadores. P3-04, P3-14 y P3-24 mantienen `vigente:false`; las otras 21 posiciones mantienen `true`. La restricción a estas huellas públicas no resuelve por renombrado la colisión histórica con la reserva: conserva los originales y evita reutilizarlos fuera de este expediente. El renombrado y las sucesiones anteriores a la captura reservada siguen pendientes.

Antes de abrir A se comprueba el marco íntegro del adaptador, sus 24 solicitudes canónicas, los rangos del caso original y las huellas de caso, pregunta y solicitud en la traza. La reserialización canónica no se presenta como identidad byte a byte del objeto original: ambos quedan recuperables y enlazados. Ningún campo faltante se completa mediante inferencia.

Se emplean seis custodios G1, con sus cuatro slots existentes. No se amplía G1 ni se descartan recibos para admitir el resto del lote. Las posiciones se ejecutan en orden y una sola vez. Un lote con 23 posiciones ejecutadas permanece incompleto aunque esas 23 puedan recuperarse.

## 3. Contraste fijado y evidencia

[Plan previo](PLAN_FIJADO.json), [testigos Rust](tests.rs), [resultado](evidencia/RESULTADO.json), [fijación de 23 archivos](evidencia/FIJACION_PREVIA.json) y [cápsula recuperable](evidencia/CAPSULA.json). Fijación SHA-256: `4308866dd0e0f7b273f7f6eaab4e4749f73e33b3f196524b253addfc91070c48`.

| Testigos | Comprobación y resultado |
| --- | --- |
| LG1-01 | Las 24 posiciones terminan y permanecen recuperables a la vez; el observador compara cuerpos con los esperados RETP-140, sin calcular nuevos significados. |
| LG1-02/03 | Lote distinto con los mismos ids y montaje con vigencia alterada: rechazo antes de abrir A. |
| LG1-04/05/06 | Marco truncado, alteración sin hash válido, solicitud cambiada con hash recalculado y rango de traza cambiado: rechazo antes de A. |
| LG1-07/08 | Segunda ejecución, posición futura y posición fuera del lote: rechazo; se conserva el recibo anterior. |
| LG1-09 | Intercambio privado de manejadores entre custodios: rechazo de pertenencia al recuperar. |
| LG1-10 | 23 posiciones ejecutadas: lote incompleto y 23 cuerpos recuperables. |
| LG1-11 | Guarda agregada: límite exacto y exceso discriminados; suma sin desbordamiento. No se fuerza una asignación física de 96 MiB. |
| LG1-12 | Dos instancias del mismo lote: cuerpos iguales e identidades de invocación distintas. |

La cápsula contiene los comandos, códigos de salida, stdout/stderr, identidad de realización y 101 archivos por compilación: cuatro originales/productos de lote, 96 archivos individuales y un índice. Cada entrada incluye bytes originales en base64, tamaño y SHA-256. El observador coteja además contexto, pregunta, rangos, huellas, vigencia y ausencia explícita de V. No interpreta clínicamente los datos ficticios.

Rust observado: `1.98.0 (88d9e12ae 2026-08-18)`, x86_64 Linux. Debug usa comprobación de desbordamientos; release la desactiva. El enlace compila sin advertencias; la inclusión de fuentes históricas G1 conserva sus advertencias en los registros. La paridad de capturas no afirma identidad de binarios ni rendimiento equivalente.

## 4. Recursos y límites

| Cuenta declarada | Observado en ambas compilaciones | Tope previo |
| --- | ---: | ---: |
| Preparación del adaptador: trabajo | 183.791 | 24.000.000 |
| Preparación del adaptador: reservas acumuladas, B | 3.832.774 | 16.777.216 |
| Comprobación del enlace: trabajo | 165.033 | 24.000.000 |
| Comprobación del enlace: reservas acumuladas, B | 1.743.690 | 16.777.216 |
| Reservas acumuladas de los seis G1, B | 53.518.379 | 100.663.296 |
| Capturas del ensayo por compilación, B | 94.113 | 134.217.728 |

Antes de cada invocación se exige un margen de 20 MiB en la cuenta agregada de G1. Este margen cubre los límites de recepción/cierre de esta ruta A; no es una reserva universal para un servicio V o un perfil arbitrario. La cuenta conserva los cargos conservadores temporales de G1. A y V mantienen sus presupuestos propios; estos contadores no equivalen a instrucciones CPU, RSS, intentos /2 ni unidades /3.

La candidata trabaja con buffers completos ya entregados por el conductor y conserva bytes en memoria durante su vida. La escritura de archivos corresponde al arnés de evidencia. No acredita recepción desde un canal bloqueado, recuperación tras caída, almacenamiento durable, contención de host ni condiciones P4/P5. El plazo de 60 s es el límite externo del ensayo, no una primitiva SV. No se ensaya WASI en esta entrega.

## 5. Reproducción, rectoras y siguiente objeto

Desde una copia completa del repositorio con Node y Rust disponibles:

```sh
node docs/calidad/tuberias-ia/ie004/lote-g1/ejecutar.mjs /ruta/absoluta/rustc /ruta/absoluta/salida-nueva
```

El [conductor](ejecutar.mjs) exige salida nueva, coteja los tres archivos públicos de entrada/esperados, fija fuentes antes de compilar y comprueba que permanecen intactas al terminar. Compila G1 y el enlace en cada modo. No ejecuta el modelo ni abre la reserva. Para recuperar la cápsula se decodifica cada `base64` y se comprueba tamaño y SHA-256 antes de comparar con el archivo de su `ruta`.

Cortes de partida: Lenguaje `a31ec0f008b83eb9d3589228d0ff8cd18ff3591a`; laboratorio `3a6ab3ec9472fd3a3b7feb45f318767de711d922`. AGENTS, Pilares, perfiles/ensamblaje, transición y arquitectura se cotejaron contra sus lecturas completas anteriores: conservan las identidades declaradas en RETP-144. Se releen el adaptador y su contrato, la candidata G1, el workflow V2 y las carencias del expediente RETP-142. Se conservan los originales de esas piezas y las actas privadas rectoras. El cambio añade este enlace experimental, evidencia, navegación y RETP; no modifica las fuentes reutilizadas del adaptador, A/V, G1 o núcleo.

**Queda cerrado este enlace público de G1. El siguiente objeto es G2, ya localizado en [RETP-142](../../EXPEDIENTE_CORRESPONDENCIA_TRAZABILIDAD_IE004_2026_09_11.md): delimitar la correspondencia entre el recibo IE-004 y una operación gobernada existente, comprobando la procedencia de los referentes que exige R1.** Si faltan, se localiza exactamente el requisito; no se rellenan ni se habilitan constructores para eludirlo. Esa comprobación precede a cualquier afirmación de inscripción o Frame desde IE-004.

La consulta artificial de lectura no constituye por sí sola una transición celular. G3/G4 conservan sus operaciones, sedes y fases. La segunda necesidad integral permanece abierta; siguen el workflow V2, la reserva cerrada y el retorno posterior a catálogo/localización y continuación de fila 9. No se constituye dominio, agente, célula de examen, interfaz ni biblioteca general por este ensayo.
