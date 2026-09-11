# Resultado del contraste causal de coste R10

**RETP-2026-133 · 11/09/2026.** Se cierra el ensayo sintáctico acotado preparado en RETP-132. Estado: **CONTRASTE_SINTACTICO_SATISFECHO; EVALUADOR_SEMANTICO_PENDIENTE; CAPTURA_NO_HABILITADA; RESERVA_CERRADA; AISLAMIENTO_NO_VERDE**.

## Qué cambió y qué se ejecutó

La sonda de barridos anterior agotaba su millón de intentos en C02 (17 tokens) y en los controles de 127/128 tokens. La alternativa experimental IE004-R10-MEMO/1 evalúa por dependencias, memoriza relaciones completas y conserva todas las familias en un bosque de derivaciones. Léxico, G01–G25 y los 72 controles públicos permanecen idénticos. Fuentes publicadas antes de ejecutar: [Lenguaje, 638eba02](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/638eba02f55f528c94871548858ec967d0e9ef2a) y [laboratorio, 1d6f1183](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/commit/1d6f1183c9604817852844544e1855c8051e2f2e).

La [decisión causal](ie004/coste-r10/DECISION_COSTE_Y_SUCESION.md) documenta el argumento de equivalencia sintáctica y la diferencia expresa frente al calendario obligatorio de /2. No se ha modificado el perfil /2 ni se declara que este algoritmo cumpla aquel calendario. Sus unidades experimentales no son los intentos de la sonda anterior; comparar sus cifras como un factor de mejora sería incorrecto.

## Resultados observados en Rust

| Configuración, Rust 1.98.0 | Corpus público | Primitivas | Unidades experimentales máximas | Capacidad máxima contabilizada | Tiempo del proceso completo |
| --- | ---: | ---: | ---: | ---: | ---: |
| Nativo debug | 72/72 | 21/21 | 62.906 | 16.144.529 B | 100,306 ms |
| Nativo release | 72/72 | 21/21 | 62.906 | 16.144.529 B | 43,984 ms |
| WASI debug | 72/72 | 21/21 | 62.906 | 15.492.821 B | 179,428 ms |
| WASI release | 72/72 | 21/21 | 62.906 | 15.492.821 B | 105,678 ms |

Una ejecución por configuración, sin reintentos. Los bosques completos y los contadores de trabajo son idénticos entre las cuatro; las capacidades difieren con la anchura de usize. Los tiempos incluyen arranque y exportación del bosque. No acreditan rendimiento del servicio, del modelo o del futuro corrector, ni admiten comparación directa con el volumen de salida distinto de RETP-131. Compilación acumulada: 1,314 s; ejecución acumulada del corpus: 0,429 s. Preparación documental, compilación preparatoria y controles estructurales se registran fuera de esa suma.

C02 termina con 3.506 unidades; CP-T-127 con 62.225; CP-T-128 con 62.906. Se conservan 48/48 históricos, 18/18 contrastes, 3/3 fronteras de tokens y 3/3 fronteras de bytes. No son 72 comprobaciones de semántica: el esperado de esta sonda es derivabilidad o rechazo técnico/léxico del instrumento.

Los 72 casos no ejercían multiplicidad sintáctica. Para resolver ese riesgo se fijaron después cuatro [controles estructurales](ie004/coste-r10/CONTROLES_ESTRUCTURALES.md): dos alternativas idénticas retienen dos derivaciones; el producto retiene cuatro; una repetición anulable se rechaza; una primera raíz descubierta no autoriza aceptación cuando se agota la cuenta antes del cierre. **4/4 en cada configuración**, con salidas idénticas. Este pequeño contraste no se presenta como parte de la reserva ni como enumeración exhaustiva.

## Memoria: inciso humano incorporado

La [nota sobre Vec, arrays y slices](ie004/coste-r10/MEMORIA_VEC_ARRAYS.md) separa asignación, capacidad y recolocación. En CP-T-128, la tabla triangular ocupa 14.120.340 bytes: 87,46 % de la capacidad contabilizada nativa y 91,14 % de la de WASI. Se reserva una vez y tiene ocupadas por nodos el 0,3741 % de sus posiciones. Cambiar solamente el contenedor a un array de igual tamaño no eliminaría esa reserva.

Los demás vectores pueden crecer; sus realojos efectivos y copias físicas no se midieron. El pico solicitado observado fue 16.247.873 B en nativo y 15.596.165 B en WASI, sin equivaler al pico físico del asignador. La arena sintáctica no incluye toda la solicitud/runtime ni los estados semánticos aún pendientes. Por tanto, no acredita el cupo agregado de A. Se fija contabilizar esos componentes y crecimientos al construir el evaluador, conservando la separación A/V.

## Decisión y siguiente objeto

El contraste satisface su objetivo: los tres fallos públicos de coste quedan resueltos por un recorrido causal distinto, dentro de los límites experimentales declarados, conservando el conjunto de derivaciones en los controles ejercidos. Termina aquí esta ronda de coste.

Queda preparado el [siguiente paso de evaluación semántica de A](ie004/coste-r10/SIGUIENTE_PASO_A_SEMANTICA.md): sucesión explícita del calendario y contabilidad; restricciones, disyunciones, negación y demandas; igualdad de significado antes de permisos; arena y estados agregados. Su implementación y ejecución todavía no se han realizado. P3 exige después congelar corrector/captura y verificar compatibilidad con la reserva; P4 debe demostrar aislamiento material y P5 la aportación y coste del agente.

El compromiso conserva 3.193 bytes y SHA-256 `d37246a019332327f54dd45c641944d90e3ae440163d76d0607c5235df63f9f5`. Custodia recuperable y separada acreditada por declaración humana; no se han accedido ni solicitado las preguntas, entradas auxiliares, oráculo o nota reservados. No hay nueva tanda de Grok, cambio de universo/dominio, núcleo, DSL/IR, catálogo o fila 9.

## Evidencia recuperable

[Guía de reproducción](ie004/coste-r10/REPRODUCIR.md), [resultados](ie004/coste-r10/resultados/RESULTADO.json), [manifiesto del corpus](ie004/coste-r10/resultados/MANIFIESTO_RESULTADOS.json), [controles estructurales ejecutados](ie004/coste-r10/controles-resultados/RESULTADO_CONTROLES.json) y [manifiesto de cierre](ie004/coste-r10/MANIFIESTO_CIERRE.json). Se depositan capturas completas comprimidas sin pérdida, logs, fuentes, flags, entorno y huellas. Los bytes descomprimidos se han cotejado sin reejecutar el motor. No se ejecutó Python.
