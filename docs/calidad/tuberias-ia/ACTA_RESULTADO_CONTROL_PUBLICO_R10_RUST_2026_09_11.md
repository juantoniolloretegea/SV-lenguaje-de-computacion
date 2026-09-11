# Resultado del control público R10 en Rust

**RETP-2026-131 · 11/09/2026.** Adenda de resultados de RETP-130. Instrumento `IE004-R10-SINTAXIS/1`; perfil IE004-ES-P2/2 sin modificación. Reserva P3 cerrada.

## Decisión y alcance

**El recorrido instrumentado es insuficiente para el corpus público fijado.** Compila y se ejecuta en las cuatro configuraciones, pero sólo 69 de 72 controles coinciden con su resultado sintáctico/admisión esperado. Se conservan los tres agotamientos. No se habilita la captura reservada de Grok. La recepción humana de custodia sigue acreditada por declaración; este fallo no cambia ni invalida el compromiso.

Se ejecutó una vez por configuración la misma fuente publicada previamente en Lenguaje `f63d97c2d10fac93d471f731e763f20f904e0904` y laboratorio `7fd99bda2d21930fc1910d63437980be4dcdb453`. No hubo modificaciones de fuente/corpus, reintentos, aumento de cupos ni enumeración adicional tras observar los resultados. El [acta de preparación y recibo](ACTA_RECEPCION_COMPROMISO_P3_Y_CONTROL_PREVIO_RUST_2026_09_11.md) conserva su estado anterior a la ejecución.

El instrumento reconoce la proyección sintáctica de G01–G25 mediante barridos y cobra visitas y particiones. No realiza la semántica de §5, la decisión de permisos, la lectura del banco, el cuerpo DATO ni la comprobación V. La identidad entre sus cuatro ejecuciones no es paridad del corrector completo ni cualificación de P3/P4/P5.

## Observaciones

| Grupo fijado antes de ejecutar | Controles | Coinciden en cada configuración | Resultado |
| --- | ---: | ---: | --- |
| Preguntas históricas públicas | 48 | 48 | Reconocimiento terminado; no acredita 48 respuestas semánticas |
| Contrastes públicos C01–C18 | 18 | 17 | C02 agota la cuenta; C06/C07/C09 no derivan como estaba previsto |
| Fronteras de tokens 127/128/129 | 3 | 1 | 127/128 agotan la cuenta; 129 se rechaza en admisión |
| Fronteras de bytes 8191/8192/8193 | 3 | 3 | 8191/8192 derivan; 8193 se rechaza en admisión |
| **Total de controles de corpus** | **72** | **69** | **Tres fallos conservados** |

Las 21 comprobaciones locales de primitivas finalizan en cada configuración: suma/producto u64 sin wrap, conversión a usize, reserva fallible, límites de cuenta y rangos UTF-8. No son una prueba universal de memoria ni aislamiento A/V: el control de cuentas usa objetos locales distintos y no realiza el transporte de la propuesta. No se observó pánico en estas ejecuciones; todos los procesos terminaron con código 0. Ese código no convierte los tres fallos funcionales de la sonda en éxitos.

| Caso | Bytes | Tokens | Intentos cobrados | Barrido iniciado al detenerse | Última posición: longitud, inicio, regla, alternativa |
| --- | ---: | ---: | ---: | ---: | --- |
| C02 | 68 | 17 | 1.000.000 | 7 | 11, 5, G07, 1 |
| CP-T-127 | 883 | 127 | 1.000.000 | 1 | 11, 54, G11, 1 |
| CP-T-128 | 890 | 128 | 1.000.000 | 1 | 11, 49, G08, 1 |

C02 es «No escriba 7 en el valor de la IgG; consulte el valor actual de IgG.». Su reconocimiento esperado ya era público antes de la prueba. Los índices de longitud/inicio cuentan tokens desde cero; la regla/alternativa se numeran desde uno. La posición es el punto registrado de interrupción, no una atribución de la causa a esa producción aislada. La cuenta impide la operación siguiente sin superar su límite ni presentar un reconocimiento parcial como terminado.

Los registros completos por caso, incluidos intentos, barridos, estados de presencia sintáctica y bytes de la tabla, son **idénticos entre los cuatro destinos/configuraciones**. La cabecera distingue correctamente usize de 64 bits en nativo y 32 en WASI. El tamaño de tabla registrado no es RSS ni memoria total; los estados de presencia tampoco son el cupo de estados semánticos del corrector completo.

## Entorno y coste observado

Rust 1.98.0, commit del compilador `88d9e12ae178fab0fb5cc050a94da85685d449ea`, LLVM 22.1.8. Nativo x86_64-unknown-linux-gnu; WebAssembly wasm32-wasip1 ejecutado con Node v24.19.0. Debug: opt-level 0, overflow-checks yes. Release: opt-level 3, overflow-checks no. Ambos usan panic abort y operaciones checked explícitas. El módulo WASI declara máximo de memoria lineal de 64 MiB y se ejecuta sin preopens. Esto no acredita aislamiento productivo del host ni consumo máximo real.

| Configuración | Compilación | Proceso de 72 controles + primitivas |
| --- | ---: | ---: |
| Nativo debug | 0,159 s | 1,912 s |
| Nativo release | 0,263 s | 0,239 s |
| WASI debug | 0,155 s | 2,182 s |
| WASI release | 0,276 s | 0,512 s |

Una muestra por configuración, con arranque y escritura de salida incluidos; WASI añade carga y compilación del módulo en el motor. Las cifras no estiman latencia del modelo, percentiles, tiempo real ni rendimiento del corrector. Compilación acumulada: 0,852 s; ejecución acumulada: 4,846 s. Los cuatro procesos quedaron dentro del límite previo de 30 s. La instalación del compilador y el trabajo documental no están incluidos en estas medidas; no existe una medida integral de preparación y no se declara acreditado su presupuesto global por estos tiempos. No se compara este coste con los minutos de elaboración de un dictamen por otra IA.

Node sólo lanza, captura y compara. La gramática y las primitivas se ejecutan en los binarios Rust. No se ejecutó Python. Las fuentes Rust se conservaron byte a byte desde la preparación.

## Interpretación y siguiente objeto técnico

El resultado aporta un testigo de insuficiencia de **esta materialización y su contabilidad**: ocurre incluso con 17 tokens, antes del techo de 128. La repetición del mismo agotamiento en release impide atribuirlo sólo a las comprobaciones automáticas de overflow de debug; el límite lógico y el trabajo cobrado son los mismos.

La búsqueda ingenua puede explorar particiones que otro recorrido evitaría, y la tabla simplificada omite estados semánticos. Por ello no se deduce una cota inferior universal, una imposibilidad de /2 ni que todo corrector conforme vaya a fallar en C02. La sonda localiza una realización que no debe usarse como base suficiente sin corregir su coste. Tampoco corresponde celebrar el agotamiento por haberlo contenido: para C02/127/128 se necesitaba terminar el reconocimiento.

El siguiente objeto es una decisión causal sobre recorrido e imputación: justificar un calendario acotado que preserve las derivaciones y la unicidad semántica exigida, y contrastarlo con estos mismos testigos públicos. Si exige cambiar una obligación o cota normativa de /2, deberá registrarse como revisión explícita y revisar la compatibilidad del compromiso antes de cualquier captura. No se modifica la reserva ni se autoriza una nueva ronda de modelo mediante esta adenda. El corrector completo, la trama A/V, los controles materiales y la viabilidad permanecen pendientes.

Los slices aportan acceso prestado y comprobado al original durante el análisis. Las evidencias persistentes conservan bytes/identidades/intervalos, no punteros vivos. Su utilidad local queda ensayada; no suplen autorización semántica, identidad de fuente ni seguridad entre procesos. Este punto no requiere ampliar el perfil ni abrir otro frente.

## Evidencia reproducible

- [Resultado por caso y configuración](ie004/recepcion-compromiso-p3/sonda-rust/resultados/RESULTADO.json).
- [Entorno y cortes previos](ie004/recepcion-compromiso-p3/sonda-rust/resultados/ENTORNO.json).
- [Manifiesto de logs y binarios](ie004/recepcion-compromiso-p3/sonda-rust/resultados/MANIFIESTO_RESULTADOS.json).
- [Fuentes y alcance del instrumento](ie004/recepcion-compromiso-p3/sonda-rust/README.md).
- [Reproducción](ie004/recepcion-compromiso-p3/sonda-rust/REPRODUCIR.md).

**Estado:** RECEPCION_HUMANA_DECLARADA_CONFIRMADA; CONTROL_PUBLICO_R10_CON_INSUFICIENCIA_DEL_RECORRIDO; CORRECTOR_COMPLETO_PENDIENTE; CAPTURA_NO_HABILITADA; RESERVA_CERRADA; AISLAMIENTO_NO_VERDE.
