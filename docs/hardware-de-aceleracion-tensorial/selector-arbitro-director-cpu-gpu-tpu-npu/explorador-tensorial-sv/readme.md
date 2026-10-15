# Explorador Tensorial del SV

**Realización experimental 0.1 · 15 de octubre de 2026.**

El Explorador Tensorial del SV identifica los recursos expuestos al proceso de ejecución y comunica sus observaciones al Selector del Árbitro - Director CPU/GPU/TPU/NPU. Distingue la identificación del dispositivo, el acceso a su interfaz y la capacidad comprobada para una operación. La detección no concede esa capacidad.

El Árbitro - Director conserva la admisión y el gobierno de la actuación; el Selector determina un destino admisible; el adaptador ejecuta y aporta evidencia del dispositivo utilizado. El Explorador no reemplaza ninguna de estas funciones.

## Aplicación

La exploración se realiza en el entorno que ejecutará la operación. Helsinki-NLP/opus-mt-en-es, como traductor administrado por el Gestor documental, corresponde al nodo 1, aunque su traducción acompañe una consulta dirigida a otro nodo. Una API externa no expone por ello el hardware de inferencia de su proveedor.

## Consultas implementadas

| Interfaz | Información recuperada | Límite |
| --- | --- | --- |
| Sistema operativo, mediante `sysinfo` | CPU visible, paralelismo disponible y memoria del entorno. | No identifica toda la topología física ni reserva memoria. |
| SetupAPI de Windows | Dispositivos presentes en las clases de pantalla y acelerador de cálculo; identidad, descripción y estado. | Un dispositivo de pantalla puede ser virtual. La clase genérica de aceleración no se convierte automáticamente en NPU. |
| PCI de Linux, mediante sysfs | Identidad PCI, clase y controlador asociado. | La enumeración PCI no comprende todos los dispositivos USB, de plataforma o de un anfitrión no expuesto a una máquina virtual. |
| NVML | GPU NVIDIA visibles, identidad, controlador y memoria consultable. | La imposibilidad de cargar NVML se registra como consulta no disponible; no demuestra ausencia de GPU. |
| Cliente PJRT de un adaptador | Plataforma TPU y dispositivos accesibles a ese cliente. | La consulta no abre por sí sola un cliente. Esta versión conserva identidad y acceso, pero deja sin determinar la memoria utilizable para selección. |

En Linux, los controladores `amdxdna` e `ivpu` permiten reconocer su clase NPU. La clase PCI de procesamiento genérico, sin identificación suficiente, permanece como acelerador sin clasificar. Esta regla tiene pruebas artificiales; no constituye una prueba física de NPU.

Cada consulta indica alcance, estado, incidencias y duración. Las identidades son propias de cada interfaz: una misma GPU observada mediante PCI y NVML puede aparecer dos veces. No se suman esas observaciones como si fueran dispositivos físicos diferentes.

## Relación con la selección

El inventario conserva un instante monotónico y pertenece a una sesión del Explorador. La interfaz pública permite consultarlo, pero no modificarlo ni reconstruirlo desde una solicitud JSON. Las sondas y los adaptadores que aportan capacidades forman parte del proceso de confianza.

Una capacidad recibida identifica la carga exacta, su revisión y dimensiones, el destino, la revisión del adaptador, la evidencia del contraste y su vigencia. El Explorador vincula esa capacidad a la identidad observada. La evidencia numérica y física procede del adaptador: un nombre de archivo o una referencia escrita no la demuestran por sí solos.

La consulta incompleta, la identidad discordante, una capacidad caducada, una clase desconocida o la cobertura parcial impiden incorporar el recurso. El Selector conserva sus comprobaciones de política, residencia y memoria. La estimación de rendimiento comprende el recorrido completo, sin preferencia automática por GPU, TPU o NPU.

Antes de invocar la ejecución se consulta de nuevo el entorno y se repite la selección. Si cambia la decisión, la interfaz devuelve un error y no invoca la operación. No introduce una sustitución silenciosa por CPU. La decisión se consume una vez mediante esta interfaz; esto no equivale a un permiso material del Árbitro - Director.

## Resultados y alcance

El [contraste físico en Windows](evidencias/contraste-windows.json) comprende 20 exploraciones sobre una CPU Intel Core i7-1165G7 y un dispositivo gráfico Intel Iris Xe. Sus nombres coincidieron con la consulta independiente CIM en las 20 observaciones. Se obtuvieron cero capacidades de ejecución por mera detección. Mediana: 3,2645 ms; mínimo: 2,619 ms; máximo, correspondiente a la primera consulta: 1.020,618 ms. Son tiempos de observación, no de cálculo del modelo.

Los [casos de contrato](codigo/selector/explorador-tensorial-sv/src/tests.rs) prueban selección, revalidación y rechazo ante desaparición, cambio de identidad observable, memoria insuficiente, observaciones inválidas y capacidades incompatibles. Los dispositivos de esos casos son simulados y se identifican expresamente como tales.

En [Linux sobre WSL2](evidencias/contraste-linux.json) se realizaron otras 20 exploraciones, con coincidencia de la identificación de CPU respecto de `/proc/cpuinfo` y cero capacidades concedidas por detección. Mediana: 0,6405 ms; intervalo: 0,379–12,384 ms. El entorno expone un dispositivo gráfico PCI `1414:008e`; esta observación no se equipara a la GPU física del anfitrión. No se mide cobertura general de aceleradores a partir de estos dos entornos.

Los [resultados de comprobación](evidencias/pruebas.json) distinguen los 23 casos del Explorador de los 18 casos conservados del Selector. La compilación de la consulta PJRT no constituye una ejecución ni una observación física de TPU.

La integración del nuevo inventario con una inferencia acelerada, la atribución física por petición y el gobierno integral del Árbitro - Director permanecen sin comprobar conjuntamente. Los resultados de otros adaptadores no acreditan esa integración.

La reconsulta no reserva el recurso ni impide un cambio posterior. Tampoco acredita aislamiento frente a código malicioso del propio proceso, identidad física atestada, cancelación de una llamada de controlador bloqueada ni coordinación entre varios procesos. La memoria compartida de una GPU integrada no se inventa cuando la interfaz no la proporciona. Estas condiciones delimitan el uso de la versión 0.1.

## Código y reproducción

El [código](codigo/selector/explorador-tensorial-sv/) conserva la biblioteca, las sondas, el enlace con el Selector, las pruebas y el programa de contraste físico. El [Selector conservado](codigo/selector/src/lib.rs) mantiene la fuente ya ensayada, con SHA-256 `2b6cd91da365f15ab314ea69ef6eeec1b12826ddf0de01fcca58191ef5afff05`. El [manifiesto](codigo/MANIFIESTO.json) identifica los archivos mediante SHA-256.

Desde `codigo/selector/explorador-tensorial-sv`:

```text
cargo test --locked
cargo run --locked
```

El segundo comando emite el inventario en JSON. No carga modelos ni asigna capacidades. Para la consulta del cliente TPU se compila la característica `pjrt`; su construcción requiere las herramientas de cabeceras y protocolos del enlace PJRT. El adaptador suministra su cliente ya abierto mediante `sondas::pjrt::Cliente`; el inventario se obtiene con `sondas::Tpu`. La memoria desconocida impide por ahora promover esa observación TPU a destino seleccionable.

La realización propia está escrita en Rust. `sysinfo` y `windows-sys` consultan el sistema; `nvml-wrapper` consulta la biblioteca NVIDIA; `pjrt-rs` enlaza con la interfaz C de PJRT. Las bibliotecas del fabricante y los controladores conservan su naturaleza y sus licencias. Este componente identifica hardware: no es un motor de inferencia ni una demostración de que todas sus dependencias estén escritas en Rust.

## Referencias

- [SetupDiGetClassDevsW, Microsoft](https://learn.microsoft.com/en-us/windows/desktop/api/setupapi/nf-setupapi-setupdigetclassdevsw).
- [PCI mediante sysfs, Linux](https://www.kernel.org/doc/html/latest/PCI/sysfs-pci.html).
- [Controlador AMD NPU, Linux](https://github.com/torvalds/linux/blob/master/Documentation/accel/amdxdna/amdnpu.rst).
- [Controlador Intel NPU, Linux](https://github.com/torvalds/linux/blob/master/drivers/accel/ivpu/ivpu_drv.c).
- [PJRT para Rust](https://github.com/rai-explorers/pjrt-rs). La copia incluida procede del [expediente Helsinki](https://github.com/juantoniolloretegea/SV-motor/tree/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/helsinki-nlp/codigo/contraste-cpu-tpu/dependencias/pjrt-rs), con sus licencias MIT y Apache-2.0.
- [Condiciones de integración](../../../evoluciones-desde-el-nucleo-no-cotejadas-generadas-por-pruebas-con-la-ia-sin-aprobacion-aun/EXPLORACION-Y-SELECCION-DE-HARDWARE-2026-10-15.md).

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
