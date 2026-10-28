# Instrucciones preliminares pre-núcleo

**Versión:** 0.1 · 28 de octubre de 2026.  
**Estado:** estudio arquitectónico y requisitos de comprobación; sin recepción de una implementación.  
**Ámbito:** preparación del entorno de ejecución del SV y protección material de sus fronteras.

## Finalidad y jerarquía

Las instrucciones preliminares pre-núcleo designan las operaciones deterministas que preparan y comprueban el entorno antes de admitir la ejecución del núcleo del SV. El término «pre-núcleo» se refiere al núcleo del SV; cuando una realización deba actuar antes del sistema operativo, deberá declarar también esa condición temporal. No designa una nueva gramática, una representación intermedia distinta ni un modelo de IA situado dentro del firmware.

El estudio desarrolla una posible realización del [objetivo del Agente Especializado en Ciberseguridad Inteligente](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/072b101459a94b1df139ce451dd116615847f679/agentes/ciberseguridad-inteligente/PROTECCION-DESDE-EL-ARRANQUE.md). Se subordina a la [frontera normativa del Lenguaje SV](../../FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md) y al [contrato SEC.0-X de ejecución material, arranque, atestación e independencia](CONTRATO_ABSTRACTO_DE_EJECUCION_MATERIAL_TCB_ARRANQUE_ATESTACION_INDEPENDENCIA_SEC0_X_2026_08_21.md). SEC.0-X ya distingue un candidato de arranque de un estado de ejecución admitido. Este documento no sustituye sus contratos ni acredita su realización.

Se conservan la terna (0, 1, U), el álgebra y las invariantes del SV. Un fallo de descarga, firma, observación o soporte se registra como incidencia técnica conforme al contrato aplicable; no se convierte automáticamente en U ni en una calificación del modelo. La falta de actualidad exigida para una atestación conserva el diagnóstico D-N previsto por SEC.0-X.

Los dominios, Universos y agentes mantienen sus requisitos de constitución y autorización. La disponibilidad del programa de preparación no los constituye, no activa modelos y no habilita un uso clínico o de ciberseguridad. La implementación propuesta de los componentes propios del SV será en Rust; los componentes de terceros conservan su identificación, licencia y revisión.

## Separación entre preparación, decisión y actuación

El firmware realiza las operaciones iniciales de la plataforma. La configuración conservada por el equipo no equivale a un lugar donde alojar el modelo de IA. Un programa cargado posteriormente no puede acreditar retrospectivamente la integridad de todas las etapas que lo preceden.

La propuesta distingue cuatro responsabilidades:

1. Una base inicial de confianza, establecida por un procedimiento autorizado, verifica el siguiente componente antes de ejecutarlo.
2. Un componente mínimo de preparación identifica la plataforma, comprueba el artefacto y sus condiciones de admisión y prepara una entrega verificable al entorno siguiente.
3. La mediación determinista conserva las facultades y restricciones del núcleo durante la ejecución. Los mecanismos del sistema operativo, hipervisor o dispositivo deben sostener materialmente esa separación para las amenazas declaradas.
4. La IA aporta entradas por las fronteras admitidas, sin acceso directo a memoria privada, reglas, claves o almacenamiento del núcleo. Esta restricción incluye al modelo que desempeñe el Perfil defensor (PD).

La raíz de confianza no puede obtener toda su legitimidad del mismo servidor o soporte que entrega aquello que debe verificar. Su establecimiento, rotación y recuperación deben satisfacer SEC.0-X. Una clave pública de verificación previamente admitida puede formar parte de una realización; la elección concreta y su custodia siguen pendientes.

## Medios de entrega y arranque

El SV podrá distribuir versiones identificadas a equipos e instituciones autorizados. La autorización de entrega, la integridad del contenido y la admisión local son comprobaciones distintas. Una dirección IP, una dirección de red física o el conocimiento de una ruta no demuestran por sí solos identidad ni permiso.

| Vía | Condición necesaria | Límite |
| --- | --- | --- |
| Arranque por red | Firmware y adaptador compatibles; dirección y servicio de arranque configurados. | Tener conexión a Internet no demuestra esa compatibilidad. |
| USB, memoria extraíble o disco | Medio y formato reconocidos por el equipo; artefacto autorizado y comprobado. | El soporte no se convierte por ello en raíz de confianza. |
| Partición de sistema EFI u otra ubicación local admitida | Cadena de arranque y plataforma compatibles. | La existencia de una partición no acredita protección ni autoriza modificarla. |
| Otros mecanismos de carga admitidos por la plataforma | Contrato, compatibilidad y comprobación específicos. | La lista de medios no es exhaustiva ni implica equivalencia de garantías. |

La especificación [UEFI, capítulo de protocolos de red](https://uefi.org/specs/UEFI/2.11/24_Network_Protocols_SNP_PXE_BIS.html), contempla arranque desde una dirección de recurso y describe escenarios corporativos y domésticos. El servicio de distribución puede estar en la red institucional o en Internet. PXE y HTTP Boot tienen dependencias diferentes; no se presumirá que una configuración sirve indistintamente para ambos.

La realización propuesta preferirá reutilizar las capacidades de arranque y transporte seguro disponibles. Deberá verificar firma, identidad del artefacto, compatibilidad, versión admitida y autorización antes de ejecutarlo. El canal cifrado no sustituye la comprobación del artefacto. Tampoco se supondrá que todo firmware admite autenticación de cliente, certificados o interfaces de identificación equivalentes a las de un navegador.

La secuencia propuesta es: identificar el destino autorizado; obtener una versión concreta; comprobarla con una base de confianza previamente establecida; preparar su carga; acreditar las condiciones exigidas; y entregar el control con evidencia. La ruta no debe seleccionar sin control «la última versión». Deben preverse revocación, protección frente a versiones antiguas, límites de descarga, interrupción de la transferencia y sustitución entre comprobación y uso.

La falta de red no obligará por diseño a una descarga indefinida. Se estudiará una copia local admitida y una vía de recuperación independiente. Su uso sin conexión requerirá una política explícita de vigencia y revocación: no puede afirmarse conocimiento inmediato de una revocación remota. Si las condiciones necesarias no se acreditan, la operación protegida no se habilita; ello no equivale a ordenar automáticamente el apagado de todo el equipo o la interrupción de otros servicios.

## Función de WebAssembly

[WebAssembly](https://webassembly.org/docs/security/) permite ejecutar módulos en un entorno delimitado. Distintos lenguajes pueden producir esos módulos si disponen de las herramientas y dependencias compatibles; no ejecuta directamente cualquier programa o binario de cualquier lenguaje. [WASI](https://wasi.dev/security) permite expresar capacidades que el anfitrión concede explícitamente.

Esta separación ofrece una referencia útil para delimitar interfaces y recursos. El entorno de ejecución, las funciones que expone y el sistema que lo sostiene pertenecen a la base de confianza de las garantías que puedan alterar. Un módulo WebAssembly no constituye por sí solo un cargador de firmware, un controlador de dispositivos o una protección frente a un anfitrión comprometido. Las comprobaciones ya documentadas de `sv_wasm` tampoco acreditan estas nuevas funciones.

Se estudiará WebAssembly únicamente donde aporte una frontera comprobable. Incorporarlo en el arranque requeriría justificar y verificar también su entorno de ejecución. El componente inicial mínimo no necesita adoptar esa dependencia por analogía.

## Implementación y soporte material

| Elemento | Evaluación inicial | Comprobación pendiente |
| --- | --- | --- |
| Aplicación UEFI en Rust | Rust dispone de destinos UEFI; es una vía para un programa inicial acotado. | Compilación identificada, firma cuando proceda, carga y rechazo en plataformas concretas. |
| Controladores | Reutilizar primero los controladores admitidos de firmware y sistema operativo. Un controlador propio sólo se justifica por una capacidad necesaria no cubierta. | Privilegios, firma, compatibilidad, mantenimiento y separación de fallos. |
| Secure Boot, arranque medido y TPM | Funciones distintas: admisión de imágenes, medición y apoyo a evidencia o custodia según la plataforma. | Configuración efectiva, interpretación de medidas, actualidad y recuperación. Ninguna concede autoridad al agente. |
| Protección de memoria y acceso directo de dispositivos | La unidad de gestión de memoria de entrada/salida (IOMMU), el firmware y el sistema operativo pueden intervenir en la protección DMA. | Cobertura de dispositivos y continuidad de la protección entre etapas. |
| Red de capa 2 | Requiere una mediación efectiva sobre los recursos de enlace que se declaren protegidos. | Interfaces, controladores y vías alternativas; una descarga HTTP no acredita control de capa 2. |
| Chip o dispositivo dedicado | Alternativa por estudiar si una garantía no puede sostenerse con el soporte disponible. | Necesidad, independencia, cadena de suministro, recuperación y autorización económica específica. |

La [documentación de Rust para UEFI](https://doc.rust-lang.org/rustc/platform-support/unknown-uefi.html) distingue aplicaciones, controladores de servicios de arranque y controladores de ejecución. La elección de uno de ellos no acredita sus garantías. Los [servicios UEFI de arranque](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html) dejan de estar disponibles tras `ExitBootServices`; permanecer protegido después exige un relevo explícito al mecanismo que deba imponer la política.

Los [requisitos de protección DMA de Microsoft](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/oem-kernel-dma-protection) ilustran que la protección temprana depende conjuntamente de firmware, hardware y sistema operativo. Estas referencias orientan el estudio; no certifican un equipo del SV.

## Comprobación progresiva y revisión adversarial

La primera prueba propuesta utilizará una máquina virtual o emulador, imagen propia, recursos limitados y ausencia de acceso a discos físicos. La virtualización no es una capa del modelo OSI: puede representar procesador, firmware, almacenamiento y enlaces de red. Permite examinar un procedimiento de carga; no demuestra resistencia del firmware físico, aislamiento eléctrico ni protección real frente a acceso directo de periféricos.

La prueba por medio local y la prueba por red conservarán sus condiciones propias. La primera comprobación de red, si se realiza, deberá quedar en una red de ensayo delimitada, sin convertir el servicio público del SV en servidor de arranque. La elección de herramientas, instalación y ejecución se recibirán por separado. Esta edición no declara esas pruebas realizadas.

| Omisión o fallo a examinar | Consecuencia que debe evitarse | Control y evidencia necesarios |
| --- | --- | --- |
| Servidor, medio o artefacto sustituido | Ejecución de un componente no admitido. | Verificación desde una raíz independiente; rechazo observado del contenido alterado. |
| Destino no autorizado o permiso retirado | Entrega o habilitación fuera del alcance concedido. | Identidad, permiso vigente y prueba de denegación; no confiar sólo en IP. |
| Versión antigua, réplica o registro de admisión reutilizado | Restauración indebida de facultades o pérdida de continuidad. | Versión y actualidad gobernadas, testigo suficiente y pruebas de repetición. |
| Caída de red o transferencia incompleta | Dependencia permanente del servidor o ejecución parcial. | Límites, recuperación admitida y ausencia comprobada de habilitación indebida. |
| Pérdida de protección al iniciar el sistema operativo | Intervalo de actuación sin mediación. | Evidencia del relevo y comprobación posterior de los controles exigidos. |
| IA o interfaz intenta modificar la base de confianza | Sustitución de reglas, claves o decisiones protegidas. | Separación material y denegación por mecanismos externos al modelo. |
| Observador o recuperación comparte el fallo examinado | Falsa conformidad o imposibilidad de recuperar. | Declaración de dependencias e independencia suficiente para esa amenaza. |

Las evidencias identificarán fuentes, construcción, huellas, plataforma, firmware, medio, versión, condiciones, tiempos, resultado y límites, sin incluir secretos. Una prueba funcional de carga no recibirá la denominación de ensayo integral de seguridad. La emulación será seguida, cuando corresponda y esté autorizada, por la comprobación en el dispositivo menos vital que satisfaga las necesidades físicas del ensayo.

## Ubicación documental y continuación

Esta es la sede técnica del estudio. El libro del Lenguaje de Computación explica su lugar respecto del núcleo; el libro de dominios delimita su relación con Ciberseguridad Inteligente; la Enciclopedia ofrece una definición breve y remite a ambos. Los libros de observabilidad y evaluación conservan sus contratos y referencias comunes, sin duplicar esta especificación.

La continuación depende de los contratos del núcleo, los dominios constituidos y una propuesta de realización recibida. El estudio no modifica parámetros, reglas, tiempos ni calificaciones de convocatorias en curso. La disponibilidad técnica, el coste y la suficiencia de una implantación deberán medirse; reutilizar mecanismos conocidos reduce trabajo potencial, pero no acredita por sí solo una solución completa, económica o universal.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
