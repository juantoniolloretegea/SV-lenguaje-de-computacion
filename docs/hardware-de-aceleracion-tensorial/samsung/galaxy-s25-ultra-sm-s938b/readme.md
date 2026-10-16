# Samsung Galaxy S25 Ultra — SM-S938B

**Reconocimiento del hardware y selección CPU en Android · 16 de octubre de 2026.**

El Explorador Tensorial del SV identificó la CPU del sistema en chip Qualcomm SM8750 y la GPU integrada Adreno 830. El Selector del Árbitro - Director CPU/GPU/TPU/NPU autorizó cuatro productos matriciales en CPU, cuyos resultados coincidieron exactamente con una referencia independiente. Rechazó doce solicitudes a clases de acelerador sin capacidad de ejecución habilitada.

## Dispositivo y entorno

| Elemento | Identificación observada |
| --- | --- |
| Equipo | Samsung Galaxy S25 Ultra; modelo SM-S938B |
| Plataforma de ensayo | Samsung Remote Test Lab |
| Sistema | Android 16; SDK 36; ABI arm64-v8a |
| Sistema en chip | Fabricante QTI; modelo SM8750; plataforma sun |
| CPU | Ocho procesadores expuestos por Android, numerados de 0 a 7 |
| GPU | Adreno (TM) 830; tipo INTEGRATED_GPU en Vulkan |
| Interfaz neuronal | NNAPI: nnapi-reference, clasificado por la interfaz como CPU |

Samsung identifica al fabricante del equipo y al proveedor del laboratorio. El sistema en chip y la GPU corresponden a Qualcomm. Esta ficha representa una configuración concreta; no caracteriza todos los teléfonos Samsung ni todos los procesadores Qualcomm.

## Qué reconoció el Explorador

La observación combinó propiedades de Android y memoria del sistema, enumeración de dispositivos mediante Vulkan y consulta de NNAPI. Las identidades inicial y final coincidieron, al igual que las reconsultadas antes de cada cálculo.

Vulkan expuso una Adreno 830 consultable. NNAPI expuso únicamente nnapi-reference como dispositivo CPU: no proporcionó una NPU accesible por esa interfaz en esta configuración. Esa observación no demuestra la ausencia física de una NPU ni determina qué podrían exponer otras interfaces.

El inventario inicial tardó 83,161 ms: 0,267 ms para CPU, 70,875 ms para Vulkan y 11,991 ms para NNAPI. Son medidas de una consulta, incluida la inicialización que corresponda, no valores estables de rendimiento. El campo de memoria de Vulkan suma los bloques declarados por la interfaz; no debe interpretarse como memoria libre, memoria dedicada o memoria física adicional a la del sistema.

## Selección y cálculo contrastados

Para cada caso se preparó un producto matricial entero de 32 bits y una referencia con acumulación de 64 bits. Tras un cálculo previo de contraste, el Explorador vinculó la capacidad CPU a la operación, sus dimensiones y la revisión del cálculo. El Selector recibió una política de clase obligatoria y el único destino CPU habilitado. La identidad y la disponibilidad se reconsultaron antes de ejecutar.

| Caso | Dimensiones m × k × n | Resultado CPU | Tiempo del cálculo medido por reloj, ns | CPU usuario, µs | CPU sistema, µs |
| --- | --- | --- | ---: | ---: | ---: |
| Mínimo | 1 × 1 × 1 | Exacto | 938 | 1 | 2 |
| Rectangular | 2 × 3 × 2 | Exacto | 781 | 1 | 2 |
| Valores con signo | 31 × 47 × 65 | Exacto | 46 667 | 0 | 34 |
| Ceros | 16 × 16 × 16 | Exacto | 6 562 | 2 | 6 |

Los cuatro casos también se solicitaron con GPU, TPU y NPU como clases obligatorias. Las doce solicitudes fueron rechazadas sin ejecutar cálculo ni sustituir el destino por CPU. El resultado comprueba el cierre ante destinos no habilitados; no compara el rendimiento entre chips ni demuestra que el Selector pueda ejecutar en la GPU o la NPU de este equipo.

## Telemetría y límites

El tiempo de cálculo se midió con el reloj de Rust; el consumo de CPU del proceso, con getrusage/RUSAGE_SELF. Ambos instrumentos delimitan intervalos distintos y tienen distinta resolución. Con una sola observación por caso y duraciones tan breves, estos números no permiten estimar velocidad sostenida, variabilidad o ventaja frente a otro dispositivo. El cero de tiempo de usuario es un valor medido con resolución de microsegundos, no prueba de ausencia de cálculo.

Los registros separan tiempos de CPU, tiempos de dispositivo y llamadas propias. Las llamadas propias de cálculo GPU y NPU fueron cero; sus tiempos de dispositivo permanecen sin medición. No se midió la ocupación global de esos aceleradores. Se conservaron también la memoria del proceso y los inventarios revalidados.

El cálculo, el Explorador, el Selector y el cotejo están escritos en Rust. Las consultas utilizan las interfaces y bibliotecas del sistema Android; ello no atribuye a Rust los controladores del fabricante. No se ejecutó un modelo de IA ni cálculo acelerado GPU/NPU en este ensayo.

## Fuentes y registros

- [Registros íntegros](registros/): identidad, inventarios inicial y final, entradas, resultados, decisiones, rechazos, mediciones y manifiesto de integridad.
- [Cotejo en Rust](registros/COTEJO-RUST.json): integridad de los cuatro archivos del manifiesto, identidad estable, cuatro resultados exactos y doce rechazos. Es una comprobación local de los registros, no una atestación externa del dispositivo.
- [Fuentes Rust](codigo/fuentes-rust.tar.gz) y [relación de huellas](codigo/FUENTES.json): Selector, Explorador, ensayo Android, cálculo y comprobador. Se incluyen las dependencias locales opcionales necesarias para resolver el proyecto, con sus licencias; PJRT no se activó para Android.
- [Manifiesto del expediente](MANIFIESTO.json): tamaño y SHA-256 de los archivos publicados.

El ejecutable ensayado tiene SHA-256 `eea1d8c89bfcaff8e33146df15f58dc2a7414b29a0df31fb7d05b22298de802c`. El registro de identidad conserva además las huellas del programa del ensayo y del cálculo, cotejadas con las fuentes adjuntas. Las restantes huellas identifican la copia de fuentes conservada, sin afirmar una reproducción binaria determinista.

Para reproducir el ensayo se requiere Rust con el destino aarch64-linux-android y Android NDK; la edición utilizada fue Rust 1.98.1 y NDK r30. Desde selector/explorador-tensorial-sv se compila el ejemplo ensayo_android con ese destino y las dependencias fijadas en Cargo.lock, sin activar las opciones pjrt o hip. El ejecutable recibe un directorio nuevo como único argumento. El ejemplo cotejar_android recibe el directorio de resultados recuperado y el ejecutable físico; comprueba su huella y los registros. Las condiciones de acceso y exposición de interfaces deben verificarse en cada dispositivo.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
