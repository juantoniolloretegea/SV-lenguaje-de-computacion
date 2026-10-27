# ASUS VivoBook X515EA/F1500EA con Intel Core i7-1165G7

27 de octubre de 2026 · Versión 1.0 · Identificador documental: `SV-PLATAFORMA-X515EA-I7-1165G7-20261027`.

**Estado: observación instrumental parcialmente recibida; plataforma no habilitada para el examen adversarial.** La instalación comparte funciones de puesto de trabajo y servidor del SV. Esta ficha no autoriza ataques contra el servicio en funcionamiento.

## Identidad y antecedente

| Elemento | Identificación observada |
|---|---|
| Equipo | ASUS VivoBook X515EA/F1500EA |
| CPU | Intel Core i7-1165G7; cuatro núcleos y ocho procesadores lógicos |
| Memoria | Aproximadamente 16 GB de RAM |
| Dispositivo gráfico | Intel Iris Xe integrada; no se le atribuye ejecución de inferencia en estas comprobaciones |
| Sistema | Windows 11; hipervisor presente y seguridad basada en virtualización activa |
| Almacenamiento auxiliar | Unidad USB de trabajo; comparte el anfitrión y no constituye aislamiento físico |

El [Explorador Tensorial](../../hardware-de-aceleracion-tensorial/selector-arbitro-director-cpu-gpu-tpu-npu/explorador-tensorial-sv/readme.md) ya documenta esta combinación de CPU y GPU. Su [contraste en Windows](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/5820ebaaf4058fdda5a4f545eb5b73da6f412898/docs/hardware-de-aceleracion-tensorial/selector-arbitro-director-cpu-gpu-tpu-npu/explorador-tensorial-sv/evidencias/contraste-windows.json) contiene veinte observaciones concordantes con CIM y cero capacidades de ejecución concedidas por detección. Es un antecedente del 15 de octubre, no otra prueba de cálculo ni una medición actual del estado del equipo.

La identificación del modelo comercial no demuestra identidad física atestada de una unidad ni permite trasladar sus resultados a todos los equipos de esa familia. Los identificadores privados de la instalación no se publican.

## Instrumentación y evidencia disponible

| Medio | Comprobación recibida | Límite |
|---|---|---|
| LibreHardwareMonitor 0.9.6 y PawnIO | Lectura de los sensores expuestos por el equipo. | No se recibió RPM de ventilador ni temperatura de GPU o del almacenamiento interno. |
| Observador del SV en Rust 0.1.0 | Muestras JSON, vigencia, límites de volumen y cadena de integridad; tres pruebas satisfactorias. | No constituye un ejecutor aislado. No está acreditado un plazo estricto de cancelación de todas las llamadas WMI síncronas. |
| ETW de Windows | Procesos, hilos, carga de imágenes y sucesos TCP/UDP de los proveedores seleccionados. | Cobertura acotada a los proveedores y a la sesión; no registra todas las operaciones del sistema. |
| Intel VTune 2026.6.0 | Captura administrativa de contadores conservada. | Alcanzó el límite de 32 MB. El informe no terminó en el plazo de 45 s; no se reciben métricas detalladas de caché, IPC ni latencia. |
| Bochs 3.1 | Ocho instrucciones x86 en modo real de 16 bits sobre datos sintéticos; registros y accesos a memoria emulada comprobados. | No mide el bus físico del portátil. La salida ordinaria automatizada del emulador sigue pendiente. |

La observación pasiva comprende 23 muestras JSON en un objetivo temporal de 45 s; 22 tenían sensores vigentes. La captura nativa comprende 88.558 sucesos en 46 s y declara cero sucesos perdidos. Son ventanas relacionadas, pero no duraciones idénticas. La CPU registró entre 67 y 93 °C durante actividad ordinaria concurrente. Este máximo requiere revisar el margen térmico antes de cargas; no demuestra daño ni fija un umbral universal.

VTune no pudo realizar el análisis de acceso a memoria solicitado por falta de los sucesos uncore necesarios en este entorno virtualizado. La calibración que introduciría una carga de DRAM se había desactivado. Se conservaron las protecciones del sistema. Un contador configurado no equivale a una medida válida.

Los [datos agregados](MEDICIONES.json) y su [manifiesto](MANIFIESTO.json) distinguen mediciones, incidencias y ausencia de ejecución del candidato. Los originales permanecen en custodia restringida; esta publicación no contiene procesos particulares, destinos de red, rutas personales, credenciales ni volcados de memoria. La cadena de huellas local no garantiza por sí sola integridad frente a un administrador que pueda sustituirla junto con los registros.

## Vinculación con el examen de ciberseguridad

Responsable de las condiciones de ejecución: **ingeniero responsable de la prueba**. La autorización es específica de caso, soporte y configuración. Esta ficha no califica a un modelo ni demuestra que su inferencia se ejecute en el procesador observado.

| Familia de casos prevista | Relación con el soporte | Estado y condición de continuación |
|---|---|---|
| C13. Canales temporales o de caché | La arquitectura emulada permite comprobar ejemplos sintéticos; una caché física compartida exige otro soporte material independiente. | Sin ejecución adversarial; contadores detallados y aislamiento físico pendientes. |
| C14. Revocación, parada y pérdida de observación | El supervisor debe permanecer fuera de la autoridad del candidato y verificar procesos descendientes, comunicaciones y recuperación. | Observador disponible; supervisor del examen y plazos de parada aún sin recepción. |
| C15. Código binario e instrucciones x86 | Registros, mnemónicos y memoria de programas sintéticos con arquitectura declarada. | Demostración emulada recibida; no equivale al examen del modelo ni a observar todas las instrucciones del anfitrión. |
| C16. Cliente, WebAssembly y servidor | Réplica del recorrido completo, con versiones y solicitudes correlacionadas. | Réplicas y aislamiento sin recepción; el servidor en servicio queda fuera de las pruebas adversariales. |
| C17. Aplicación PHP representativa | Objetivo sintético separado, con dependencias, identidades y efectos comprobables. | Pendiente de construcción y recepción; no se incorpora PHP al núcleo o al servidor público del SV. |

Esta tabla vincula requisitos de soporte; no sustituye el protocolo completo ni autoriza su ejecución. Los escenarios de escape, controladores, firmware, agotamiento o caché física no se reciben sobre el equipo que sostiene el servicio. Una máquina virtual comparte recursos físicos, y un disco USB conectado no resuelve esa dependencia.

Cada ejecución deberá enlazar esta ficha por revisión inmutable con el caso, sus límites, la autorización, las trazas y el resultado independiente. El resultado del modelo, la protección material y los fallos de instrumentación tendrán conclusiones separadas. El hardware de un proveedor de IA remoto queda fuera de la observación local.

## Límites de recepción

No están acreditados: perímetro físico del examen; aislamiento de las réplicas; cancelación integral del candidato; métricas detalladas de caché y memoria; observación eléctrica de buses; cobertura completa de DMA, firmware o memoria física. Tampoco se ha realizado aquí una comparación de rendimiento tensorial ni una evaluación del modelo.

La vigilancia descrita fue temporal. Las herramientas y controladores instalados no acreditan una monitorización permanente. La ficha debe revisarse al cambiar el soporte, el sistema, las protecciones, los controladores o la finalidad del caso.

[Volver al alcance de hardware de ensayo](../ALCANCE.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
