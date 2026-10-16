# AMD Instinct MI300X VF: exploración, selección CPU/GPU y recuperación de resultados

**Ensayo del 16 de octubre de 2026 · AMD Developer Cloud.**

El Explorador Tensorial del SV identificó el acelerador y contrastó su identidad entre HIP, PCI y el contexto de cálculo. El Selector del Árbitro - Director CPU/GPU/TPU/NPU encaminó correctamente las operaciones CPU obligatoria, GPU obligatoria y selección automática. Las dos ejecuciones produjeron **168 resultados exactos: 82 CPU y 86 GPU**, con 36 decisiones y 24 rechazos comprobados. Se observó una demora reproducida en la recuperación de un resultado GPU. El cálculo fue correcto; la estabilidad de su latencia no queda acreditada.

## Configuración examinada

| Elemento | Observación |
| --- | --- |
| Acelerador | AMD Instinct MI300X **VF**, presentación virtualizada; una GPU visible mediante HIP |
| Arquitectura | CDNA 3; `gfx942:sramecc+:xnack-`; grupo de ejecución de 64 |
| Identidad PCI | `0000:83:00.0`; fabricante y dispositivo `1002:74b5`; controlador `amdgpu` |
| Memoria GPU comunicada | 205.822.885.888 bytes totales; 205.474.758.656 bytes libres en la consulta inicial |
| CPU anfitriona | [Intel Xeon Platinum 8568Y+](../../intel/xeon-platinum-8568y-plus/readme.md), 20 procesadores lógicos disponibles; RAM observada: 253.347.037.184 bytes |
| Plataforma | AMD Developer Cloud, ATL1; máquina virtual Ubuntu 24.04, GLIBC 2.39; ROCm 10.1; revisión HIP comunicada: 7.16.26385 |
| Realización | Rust 1.98.1; CubeCL y `cubecl-hip` 0.11.0; `cubecl-hip-sys` 7.14.6085001; Tracel LLVM 23.1.0-3 |

El adaptador gráfico virtual Virtio se identificó separadamente y no recibió capacidad de cálculo. La presentación VF no acredita acceso exclusivo a todos los recursos físicos del servidor. La identificación, las versiones y el inventario se conservan en los registros del ensayo.

## Operación y método

Se calculó un producto matricial de enteros `i32`, con seis conjuntos de datos y referencia independiente acumulada en `i64`. Las dimensiones y los valores están acotados para impedir desbordamiento. Cada ejecución conservó la primera operación GPU, cinco mediciones de calibración por dispositivo y caso, y las tres modalidades de selección. CPU y GPU alternaron su orden durante la calibración. Se verificaron entradas, salidas completas, identidad, lanzamientos y decisiones. El destino no permitido y la memoria imposible se rechazaron sin cálculo.

| Caso | Dimensiones m × k × n | Mediana CPU, ejecuciones 1 / 2 (µs) | Mediana GPU completa, ejecuciones 1 / 2 (µs) | Selección automática |
| --- | --- | ---: | ---: | --- |
| Mínimo | 1 × 1 × 1 | <1 / <1 | 203 / 245 | CPU |
| Ceros | 16 × 16 × 16 | 2 / 3 | 197 / 260 | CPU |
| Rectangular | 31 × 47 × 65 | 25 / 27 | 250 / 235 | CPU |
| Signos | 127 × 65 × 33 | 81 / 102 | 310 / 306 | CPU |
| Límites | 3 × 512 × 5 | 9 / 11 | 278 / 299 | CPU |
| Carga | 257 × 129 × 257 | 1.581 / 2.020 | 472 / 522 | GPU |

La duración GPU completa comprende revalidación de identidad, preparación y transferencias de entrada, cálculo sincronizado y recuperación. No comprende la creación inicial del contexto, la exploración completa y decisión previas, la escritura de registros ni el cierre del muestreo. La realización CPU incluye validación y producto; es la realización concreta comparada, sin afirmar explotación simultánea de los 20 procesadores lógicos. El cero entero de una medición expresa resolución inferior a un microsegundo.

El cálculo está definido en Rust/CubeCL y se compila mediante LLVM AMDGPU. La representación conservada contiene `amdgpu_kernel` y `amdgcn-amd-amdhsa`. HIP carga, transfiere y ejecuta el código. No se utilizó rocBLAS ni hipBLAS; la modalidad de generación HIP C++ se rechazó expresamente. **LLVM/LLD, sus adaptaciones nativas y el controlador HIP siguen siendo dependencias externas**: la evidencia no permite describir toda la cadena como exclusivamente Rust.

## Diagnóstico de la demora

En las dos ejecuciones, el retraso se concentra en `carga-gpu`, después de la calibración y antes de `carga-automatico`. El resultado contiene 66.049 enteros, equivalentes a **264.196 bytes**.

| Medida | Ejecución 1 | Ejecución 2 |
| --- | ---: | ---: |
| Cálculo mediante eventos GPU | 44,219 µs | 51,357 µs |
| Recuperación del resultado | 22.736 µs | 25.300 µs |
| Duración completa de `carga-gpu` | 23.140 µs | 25.792 µs |
| Proporción de la recuperación | 98,25 % | 98,09 % |
| Recuperación del caso automático inmediatamente posterior | 36 µs | 36 µs |
| Recuperación de las otras 42 operaciones GPU, intervalo observado | 19–37 µs | 19–44 µs |
| Incremento de memoria residente del proceso durante la operación lenta | 205.295.616 bytes | 205.295.616 bytes |
| Incremento de memoria GPU global durante la operación lenta | 186.044.416 bytes | 186.044.416 bytes |

La operación GPU obligatoria tardó 15,41 y 14,69 veces lo que tardó su operación CPU obligatoria respectiva. Esta comparación describe esas observaciones; no estima el rendimiento general del acelerador. Las medianas de calibración favorables a la GPU no predijeron esa demora posterior.

**Localización comprobada.** En `calculo.rs`, el intervalo `recuperacion_us` empieza antes de `client.read_one(c)` y termina después de convertir los bytes en un vector de enteros mediante `to_vec()`. El producto ya ha finalizado según sus eventos. La comparación numérica, la escritura JSON y la espera final del muestreo quedan fuera de ese intervalo. No se observó una ejecución numérica adicional ni un cambio de destino.

**Recorrido examinado.** En las versiones fijadas de CubeCL, `read_one` entrega la solicitud al servicio de ejecución; éste reserva memoria anfitriona, ordena la copia y espera un evento. Si necesita ampliar la memoria fijada, utiliza `hipHostMalloc` y `hipStreamSynchronize`; la copia contigua emplea `hipMemcpyDtoHAsync`, y la espera utiliza `hipEventSynchronize`. El intervalo también puede incluir espera de acceso al servicio y, finalmente, la copia a memoria Rust. Las fuentes examinadas se identifican por revisión y SHA-256 en [FUENTES.json](codigo/FUENTES.json).

**Indicio principal, no causa demostrada.** Los incrementos de memoria coinciden exactamente entre ejecuciones y son muy superiores al tamaño del resultado. Resultan compatibles con una reserva o inicialización diferida en el recorrido CubeCL/HIP. Los contadores abarcan toda la operación: no permiten adjudicar ese incremento a una llamada concreta, y la memoria GPU es una observación global. Tampoco existen trazas de llamadas HIP, del planificador del sistema o del hipervisor que permitan separar esas contribuciones. No procede atribuir un defecto al acelerador o al proveedor a partir de estos registros.

El periodo de muestreo de 25 ms no constituye una explicación suficiente. En la primera operación lenta sólo se registraron las muestras inicial y final, separadas 24.343 µs; no hubo consulta intermedia de telemetría. Además, el cierre del muestreo se ejecuta después del intervalo de recuperación. No se excluye todo coste de instrumentación o planificación, pero sí la afirmación de que el código imponga una espera fija de 25 ms antes de devolver cada resultado.

## Correcciones justificadas

1. **Descomponer la medición propia.** Registrar por separado la solicitud de lectura, la reserva de memoria, la copia HIP, la espera y la copia final a `Vec<i32>`. Medir también el recorrido desde la recepción de la petición hasta la entrega, diferenciándolo del tiempo de la operación. Los datos actuales no proporcionan ese desglose.
2. **Contrastar la gestión de memoria antes de cambiarla.** Conservar tamaños y sucesos de reserva, reutilización y liberación; identificar si la demora aparece en `hipHostMalloc`, en la copia o en la espera. Si se confirma una reserva tardía, evaluar memoria reutilizable o preparación explícita, conservando sus costes y comprobando las garantías de sincronización. Eliminar esperas sin esa comprobación sería incorrecto.
3. **Ampliar la estimación de rendimiento.** Mantener la mediana, pero añadir dispersión, máximos observados y actualización con las solicitudes posteriores. Cinco calibraciones no permiten garantizar latencia por petición ni estimar con solidez sucesos poco frecuentes. Una solicitud GPU obligatoria debe conservar su destino; la demora no autoriza una sustitución silenciosa por CPU.
4. **Distinguir ejecución y diagnóstico.** Las correcciones de medición están especificadas; no se presentan como una reparación física ya verificada. Los registros actuales localizan el problema, pero no permiten decidir si la causa última pertenece a CubeCL, HIP, al sistema operativo o a la virtualización.

## Evidencias y reproducción

Los dos archivos de resultados conservan entradas, salidas, inventarios, decisiones, controles negativos, tiempos, muestras, representación LLVM, bibliotecas observadas y manifiestos originales. Su comprobador Rust verifica 109 archivos por ejecución y **43.206.021 bytes** en conjunto. Los manifiestos originales se conservan adicionalmente dentro de cada archivo. Las **346 muestras** no presentan campos ausentes; el intervalo máximo fue 31,202 ms para un objetivo de 25 ms. La ocupación GPU es global y no atribuye consumo exclusivo al proceso; los contadores CPU conservan la resolución del sistema operativo. No se midió energía.

- [Resultados íntegros: ejecución 1](registros/ensayo-01.tar.gz) y [ejecución 2](registros/ensayo-02.tar.gz).
- [Cotejo de la ejecución 1](registros/COTEJO-01.json), [cotejo de la ejecución 2](registros/COTEJO-02.json) y [análisis retrospectivo de las 168 operaciones](registros/ANALISIS-DEMORA.json).
- [Inventario inicial](registros/INVENTARIO-AMD.json) y [registros de compilación y ejecución](registros/compilacion-y-ejecucion.tar.gz).
- [Fuentes utilizadas en el ensayo](codigo/fuentes-ensayo.tar.gz), [comprobador numérico y de integridad](codigo/cotejar.rs) y [analizador de la demora](codigo/analizar_registros.rs). Los dos últimos se ejecutan posteriormente sobre los registros; no forman parte del ejecutable físico.
- [Identidad de fuentes y ejecutable](codigo/FUENTES.json) y [manifiesto de los archivos publicados](MANIFIESTO.json).

El archivo de fuentes conserva la estructura de directorios, `Cargo.lock`, el Explorador, el Selector, el contrato numérico y las modificaciones acotadas de compilación. Para reconstruir el ejecutable físico se utiliza Ubuntu 24.04, Rust 1.98.1 y `SV_HIP_BINDINGS_PATCH=60850`; la variable selecciona las definiciones publicadas de interfaz, sin alterar la revisión que comunica HIP. El distribuidor de LLVM admite `SV_LLVM_CACHE_DIR` para fijar su directorio. Se conservan las licencias de terceros. La incompatibilidad inicial de GLIBC y la selección explícita de definiciones HIP son condiciones de reproducción, no pruebas de una demora durante el cálculo.

Para el cotejo, incorporar `cotejar.rs` y `analizar_registros.rs` al directorio `ensayo-amd/src/bin/` de las fuentes extraídas. El primero recibe un directorio `resultado-01` o `resultado-02`; el segundo recibe ambos, en ese orden. No necesitan acceso al acelerador para examinar los resultados. El ejecutable físico tiene SHA-256 `8994e997b5678ede51189a25ed8fef05f4a3c0c3b3b0d141795063cbfcb1abcd` y tamaño 123.096.896 bytes.

**Alcance:** reconocimiento, selección, cálculo exacto y observación diferenciada CPU/GPU sobre esta configuración. No acredita inferencia de un modelo, rendimiento máximo, uso de instrucciones matriciales especializadas, otras GPU AMD, Radeon 8060S ni NPU. La ficha de la CPU corresponde al mismo experimento y no añade otra ejecución.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
