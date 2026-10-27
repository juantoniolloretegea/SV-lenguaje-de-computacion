# ASUS VivoBook X515EA/F1500EA con Intel Core i7-1165G7

27 de octubre de 2026 · Versión 1.1 · Identificador documental: `SV-PLATAFORMA-X515EA-I7-1165G7-20261027`.

**Estado: observación instrumental parcialmente recibida; plataforma no habilitada para el examen adversarial.** La instalación comparte funciones de puesto de trabajo y servidor del SV. Esta ficha no autoriza ataques contra el servicio en funcionamiento.

Se aplica el [criterio de suficiencia del ensayo](../SUFICIENCIA-DEL-ENSAYO.md): una carencia necesaria debe resolverse o cubrirse con otro soporte competente. No se elimina del examen ni se considera superada por la intervención de una protección externa. Esta revisión recibe dos mejoras instrumentales concretas; no declara resueltas las demás condiciones.

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
| Intel VTune 2026.6.0 | Segunda adquisición mediante recuento: informe de 5,003 s finalizado, sin aviso por límite de volumen; contadores y métricas agregadas conservados. | Multiplexación declarada 0,856; cuatro eventos OFFCORE descartados por seguridad del sistema y un evento adicional no válido. No se recibe cobertura completa de caché y memoria. |
| Bochs 3.1 | Ocho instrucciones x86 en modo real de 16 bits sobre datos sintéticos; registros y accesos a memoria emulada comprobados. Cierre ordinario automatizado recibido en 2,269 s. | Es una demostración instrumental; no mide el bus físico del portátil ni constituye un examen del candidato. |

La observación pasiva comprende 23 muestras JSON en un objetivo temporal de 45 s; 22 tenían sensores vigentes. La captura nativa comprende 88.558 sucesos en 46 s y declara cero sucesos perdidos. Son ventanas relacionadas, pero no duraciones idénticas. La CPU registró entre 67 y 93 °C durante actividad ordinaria concurrente. Este máximo requiere revisar el margen térmico antes de cargas; no demuestra daño ni fija un umbral universal.

VTune no pudo realizar el análisis de acceso a memoria solicitado por falta de los sucesos uncore necesarios en este entorno virtualizado. La calibración que introduciría una carga de DRAM se había desactivado. Se conservaron las protecciones del sistema. Un contador configurado no equivale a una medida válida.

La primera captura de VTune alcanzó 32 MB y su informe no concluyó en 45 s; ambos hechos se conservan. La segunda cambió el método a recuento agregado, con finalización completa y sin carga de calibración. La adquisición comunicó 5,003 s, 46.795.453.410 ciclos, 25.935.261.600 instrucciones retiradas y CPI 1,804. Son cifras comunicadas por VTune sobre actividad concurrente, con multiplexación; no consumo exclusivo del SV ni una traza de cada instrucción. El proceso completo de adquisición y elaboración del informe duró 36,577 s. La terminación correcta del proceso no recibe los eventos descartados ni las métricas dependientes de ellos. No se han desactivado VBS ni las restricciones OFFCORE.

La segunda comprobación de Bochs conserva la imagen y las órdenes del ejemplo anterior, pero cierra la entrada de consola para permitir la salida tras la orden de terminación. El programa finalizó con código cero, sin cierre forzado; se cotejaron ocho instrucciones, los registros AX/BX y los dos bytes de memoria esperados. Una prueba del verificador rechazó cuatro alteraciones o ausencias; Clippy no produjo avisos propios. Se recibe este cierre concreto, no un supervisor general de procesos hostiles.

Los [datos agregados](MEDICIONES.json) y su [manifiesto](MANIFIESTO.json) distinguen mediciones, incidencias y ausencia de ejecución del candidato. Los originales permanecen en custodia restringida; esta publicación no contiene procesos particulares, destinos de red, rutas personales, credenciales ni volcados de memoria. La cadena de huellas local no garantiza por sí sola integridad frente a un administrador que pueda sustituirla junto con los registros.

## Vinculación con el examen de ciberseguridad

Responsable de las condiciones de ejecución: **ingeniero responsable de la prueba**. La autorización es específica de caso, soporte y configuración. Esta ficha no califica a un modelo ni demuestra que su inferencia se ejecute en el procesador observado.

| Familia de casos prevista | Relación con el soporte | Estado y condición de continuación |
|---|---|---|
| C13. Canales temporales o de caché | La arquitectura emulada permite preparar ejemplos sintéticos; una caché física compartida exige un soporte independiente que exponga el fenómeno y cuya utilización esté permitida. | Sin ejecución adversarial; recuentos agregados disponibles, recepción de la señal, contadores necesarios y aislamiento pendientes. |
| C14. Revocación, parada y pérdida de observación | El supervisor debe permanecer fuera de la autoridad del candidato y verificar procesos descendientes, comunicaciones y recuperación. | Observador disponible; supervisor del examen y plazos de parada aún sin recepción. |
| C15. Código binario e instrucciones x86 | Registros, mnemónicos y memoria de programas sintéticos con arquitectura declarada. | Demostración emulada recibida; no equivale al examen del modelo ni a observar todas las instrucciones del anfitrión. |
| C16. Cliente, WebAssembly y servidor | Réplica del recorrido completo, con versiones y solicitudes correlacionadas. | Réplicas y aislamiento sin recepción; el servidor en servicio queda fuera de las pruebas adversariales. |
| C17. Aplicación PHP representativa | Objetivo sintético separado, con dependencias, identidades y efectos comprobables. | Pendiente de construcción y recepción; no se incorpora PHP al núcleo o al servidor público del SV. |

Esta tabla vincula requisitos de soporte; no sustituye el protocolo completo ni autoriza su ejecución. Los escenarios de escape, controladores, firmware, agotamiento o caché física no se reciben sobre el equipo que sostiene el servicio. Una máquina virtual comparte recursos físicos, y un disco USB conectado no resuelve esa dependencia.

Cada ejecución deberá enlazar esta ficha por revisión inmutable con el caso, sus límites, la autorización, las trazas y el resultado independiente. El resultado del modelo, la protección material y los fallos de instrumentación tendrán conclusiones separadas. El hardware de un proveedor de IA remoto queda fuera de la observación local.

## Límites de recepción

No están acreditados: perímetro físico del examen; aislamiento de las réplicas; cancelación integral del candidato; cobertura de los eventos descartados, latencia y ancho de banda de memoria; observación eléctrica de buses; cobertura completa de DMA, firmware o memoria física. Tampoco se ha realizado aquí una comparación de rendimiento tensorial ni una evaluación del modelo. Los recuentos recibidos no permiten cerrar esas condiciones.

AMD Developer Cloud y UpCloud se estudian como soportes complementarios, manteniendo el portátil. Su elegibilidad económica, hardware efectivo, permisos y contadores deben comprobarse por separado. Una instancia remota no hereda una recepción física por estar fuera del portátil, ni una imagen denominada Kimi acredita la identidad de la versión concreta exigida para el examen.

La vigilancia descrita fue temporal. Las herramientas y controladores instalados no acreditan una monitorización permanente. La ficha debe revisarse al cambiar el soporte, el sistema, las protecciones, los controladores o la finalidad del caso.

[Volver al alcance de hardware de ensayo](../ALCANCE.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
