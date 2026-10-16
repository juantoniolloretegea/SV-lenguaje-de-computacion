# Identificación y selección de dos NVIDIA Tesla T4 en Kaggle

16 de octubre de 2026.

El Explorador Tensorial del SV identificó dos dispositivos del mismo modelo y el Selector del Árbitro - Director CPU/GPU/TPU/NPU dirigió operaciones a cada uno de ellos y a CPU. Se cotejaron 120 resultados exactos y se reprodujeron 30 decisiones. La aportación es comprobar la distinción y utilización individual de dos GPU equivalentes. No amplía la compatibilidad a otra familia de hardware ni acredita cálculo distribuido.

## Identificación comprobada

Kaggle proporcionó un entorno Linux x86-64 con la opción «GPU T4 x2». El Explorador obtuvo el inventario mediante consultas al sistema y NVML; los identificadores no se introdujeron manualmente. El adaptador contrastó su correspondencia con los contextos de cálculo CUDA. La numeración GPU 0/GPU 1 sólo abrevia los identificadores de esta sesión.

| Propiedad | GPU 0 | GPU 1 |
|---|---|---|
| Modelo comunicado | Tesla T4 | Tesla T4 |
| UUID | `GPU-d682bf82-f3aa-2add-3662-c848e87275a7` | `GPU-8218a387-45ce-abbb-2259-0de39d55e029` |
| Dirección PCI expuesta | `00000000:00:04.0` | `00000000:00:05.0` |
| Memoria total expuesta | 16 106 127 360 bytes | 16 106 127 360 bytes |
| Memoria libre inicial | 15 636 037 632 bytes | 15 636 037 632 bytes |
| Capacidad de cálculo CUDA | 7.5 | 7.5 |
| Controlador | 580.178.04 | 580.178.04 |
| Operaciones exactas | 37 | 42 |

La CPU se presentó como `Intel(R) Xeon(R) CPU @ 2.00GHz`, con cuatro procesadores lógicos; esa denominación no permite identificar una variante comercial más precisa. Se observaron 33 658 306 560 bytes de memoria principal y se realizaron 41 operaciones CPU. La exploración inicial requirió 41 957 µs; la inicialización de los contextos GPU, medida aparte, 656 259 µs.

La coincidencia entre UUID de NVML y CUDA, la comprobación del contexto antes de cada operación y los resultados recuperados sustentan la identificación y el destino utilizados. Ambas GPU tenían las mismas características relevantes para este ensayo, pero identidades y memorias separadas. Sus memorias no se sumaron.

## Operación y selección

Se utilizaron seis entradas de producto matricial entero: mínima, ceros, rectangular, signos, límites y carga. Las entradas completas acompañan a los resultados. No intervino un modelo de IA. El cálculo CPU y el cálculo GPU se contrastaron con una referencia que acumula en enteros de 64 bits.

Para cada entrada se efectuaron cinco operaciones en cada destino, rotando el orden: 90 operaciones de calibración. A continuación se cursaron cinco decisiones por entrada: CPU obligatoria, GPU 0 obligatoria, GPU 1 obligatoria, selección entre ambas GPU y selección entre CPU y ambas GPU. Estas decisiones produjeron otras 30 operaciones, precedidas de una nueva consulta del inventario.

La política experimental eligió la menor mediana de cinco medidas comparables. En GPU, el recorrido medido incluye asignación, transferencia, cálculo, sincronización, recuperación y liberación en una sesión ya inicializada. Observación, inicialización y cotejo se midieron separadamente.

| Entrada | CPU: mediana, µs | GPU 0: mediana, µs | GPU 1: mediana, µs | Selección entre los tres destinos |
|---|---:|---:|---:|---|
| Mínima | 1 | 439 | 357 | CPU |
| Ceros | 6 | 427 | 382 | CPU |
| Rectangular | 57 | 421 | 440 | CPU |
| Signos | 163 | 477 | 444 | CPU |
| Límites | 22 | 593 | 464 | CPU |
| Carga | 3722 | 632 | 627 | GPU 1 |

La entrada de carga multiplicó una matriz de 257 × 129 por otra de 129 × 257. La diferencia de 5 µs entre las medianas GPU no demuestra superioridad estable de una tarjeta. La selección automática posterior tardó 847 µs en GPU 1, mientras que las operaciones obligatorias anteriores tardaron 621 µs en GPU 0 y 670 µs en GPU 1. Se observó también una operación de límites de 12 271 µs en GPU 1. La elección fue conforme a la calibración; no se demostró que garantice el menor tiempo en cada ejecución posterior.

## Telemetría y cotejo

El registro instrumental abarcó 6800 ms y contiene 423 registros encadenados, con 343 844 bytes. La CPU acumuló 6450 ms de proceso entre las muestras extremas, incluidos cálculo, control y observación; ese tiempo no representa exclusivamente cálculo CPU. La memoria residente máxima muestreada del proceso fue de 250 331 136 bytes.

| Observación periódica | CPU | GPU 0 | GPU 1 |
|---|---:|---:|---:|
| Muestras | 27 | 55 | 55 |
| Intervalo previsto, ms | 250 | 100 | 100 |
| Intervalo máximo observado, ms | 260 | 141 | 141 |
| Energía global del dispositivo entre muestras extremas, mJ | No medida | 179 607 | 174 844 |
| Potencia máxima muestreada, mW | No medida | 28 510 | 28 919 |
| Temperatura máxima muestreada, °C | No medida | 52 | 53 |

No se registraron errores de medición. Los intervalos efectivos excedieron los previstos; el muestreo puede omitir máximos. La energía de NVML pertenece al dispositivo completo y no equivale a energía exclusiva del programa. La presencia de memoria o del proceso en una GPU tampoco demuestra cálculo: se conserva, por operación, el contexto comprobado, el lanzamiento, los eventos CUDA sincronizados y la salida exacta. No se obtuvo una traza independiente de instrucciones del dispositivo.

El verificador Rust recuperó las 120 referencias y reprodujo las 30 decisiones. Doce retiradas lógicas del destino impidieron invocar la operación; no fueron desconexiones físicas. Siete alteraciones separadas de copias —huella, salida, identidad, destino, calibración, rechazo y truncamiento del registro— fueron rechazadas con el diagnóstico correspondiente. Estas comprobaciones no constituyen una firma externa ni una prueba exhaustiva contra falsificación de todo el expediente.

## Fuentes y reproducción

El programa propio, el cálculo y el cotejo están escritos en Rust. El programa GPU se compiló a PTX desde Rust y se cargó mediante el controlador CUDA. No se utilizó un motor numérico C++ ni se compiló cálculo CUDA C++; CUDA y NVML siguen siendo dependencias nativas externas de NVIDIA. La opción `nvrtc` de la biblioteca `cudarc` aporta tipos usados para cargar PTX; este recorrido no invoca compilación NVRTC.

- [Evidencias originales](dos-t4-20261016-evidencias.tar.gz): inventarios, entradas, salidas, calibraciones, decisiones, observaciones y manifiesto SHA-256.
- [Fuentes del ensayo](dos-t4-20261016-fuentes.tar.gz): adaptador, Explorador, Selector, cálculo y verificadores, con versiones fijadas. Se incluyen las fuentes y licencias de la dependencia opcional PJRT requerida para resolver la estructura de dependencias; no se activó ni se utilizó en este ensayo GPU.
- [Cotejo y comprobaciones adversariales](dos-t4-20261016-cotejo.json): resultados originales, contraste de fuentes recompiladas y diagnósticos de las siete alteraciones.

El ejecutable utilizado tuvo 1 830 696 bytes y SHA-256 `65f5b5594b11a03a7281788e4b6d519d7b529cfb702fa7c6a0ab444f5e1e8b94`. El PTX utilizado tiene SHA-256 `6d8ee141bdba91819c207175c5ce91cb1cbf269090986c9b1c037a3aede38e3c`. Se generó con Rust `1.101.0-nightly`, revisión `080cc2714`, como PTX 7.0 para `sm_70`; el dispositivo ensayado admite capacidad 7.5. El programa anfitrión se compiló con Rust 1.98.1; utiliza `cudarc 0.19.10` y `nvml-wrapper 0.12.1`.

Las fuentes empaquetadas se extrajeron y compilaron de nuevo; sus cuatro pruebas locales y el cotejo de las evidencias originales fueron conformes. Desde la raíz extraída, la reproducción del cotejo se realiza con:

```sh
cargo build --locked --release --manifest-path desarrollo/selector-arbitro-director-cpu-gpu-tpu-npu/ensayo-dos-t4/Cargo.toml --bins
desarrollo/selector-arbitro-director-cpu-gpu-tpu-npu/ensayo-dos-t4/target/release/sv-explorador-selector-dos-t4 verificar /ruta/a/evidencias
```

El segundo comando verifica registros conservados; no inicia una sesión GPU. El archivo de evidencias tiene SHA-256 `c410e21d54a4b542bad0a6728d757df6e11dbf5fe283ba7bd3dc1098e49db714`; el de fuentes, `622b360b2e54f53ebb75042a02cbadd75a95f14e119f0700efd4beea6c1adc1e`.

El resultado acredita selección individual en esta configuración y para estas seis cargas. No demuestra inferencia, concurrencia, reparto de una operación entre GPU, saturación prolongada, sustitución ante una avería física ni optimalidad general. El ensayo anterior con Qwen conserva su alcance propio; este contraste no modifica sus resultados.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
