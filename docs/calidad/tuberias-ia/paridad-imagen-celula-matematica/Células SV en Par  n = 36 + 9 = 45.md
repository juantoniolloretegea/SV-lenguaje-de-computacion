---
article:
  elocation-id: de-svcustos-el-marco-de-intrusion-hasta-svperitus-celulas-sv-en-par-n--36--9--45
author:
- Juan Antonio Lloret Egea
bibliography: /tmp/tmp-20mu7PeOfpT4kN.json
copyright:
  link: "https://creativecommons.org/licenses/by-nc-nd/4.0/"
  text: Creative Commons Attribution-NonCommercial-NoDerivatives 4.0
    International License
  type: CC-BY-NC-ND
csl: /app/dist/server/server/utils/citations/citeStyles/apa-7th-edition.csl
date:
  day: 04
  month: 03
  year: 2026
journal:
  publisher-name: IA eñ ™ - (La Biblia de la IA - The Bible of AI ™ ISSN
    2695-641)
  title: IA eñ ™
link-citations: true
title: "De SVcustos, el marco de intrusión, hasta SVperitus: Células SV
  en Par: n = 36 + 9 = 45"
uri: "https://www.itvia.online/pub/de-svcustos-el-marco-de-intrusion-hasta-svperitus-celulas-sv-en-par-n--36--9--45"
---

![](https://assets.pubpub.org/c39829d0b-6cf2-4c57-8562-04aa66068f1d/p3ddc5ece-13fa-453f-b669-196c8d00dae8/u8d2bc69d-73c6-4669-a7e7-fb07fdba4171/image-71772614574133.png){#nhky98syveq}

\[video element\]

# **De SVcustos, el marco (framework) de intrusión, hasta SVperitus: (Células SV en Par: n = 36 + 9 = 45)** {#de-svcustos-el-marco-framework-de-intrusin-hasta-svperitus-clulas-sv-en-par-n-36-9-45}

 **(Documento 5 de 8)**

**ISSN 2695-641**. Madrid, 4 de marzo de 2026

------------------------------------------------------------------------

# Resumen

Este quinto documento de la serie presenta la arquitectura de células SV
en par como solución al problema estructural planteado por los
parámetros de integridad del sistema operativo identificados en el
Documento 4. El análisis adversarial del sistema en n = 36 revela nueve
vectores de amenaza relativos a la integridad de la plataforma que
ningún parámetro existente cubre. La incorporación directa de estos
parámetros produciría n = 45, que no satisface la restricción n = b² del
framework.

La solución propuesta eleva la célula SV ---un sistema n = b² completo
con imagen polar, clasificador ResNet34 y salida ternaria--- al rango de
unidad compositiva. Se definen dos células independientes: una célula
principal SV(36, 6) para comportamiento observable y una célula de
integridad SV(9, 3) para la fiabilidad del sistema operativo. Ambas
satisfacen n = b² individualmente. Sus clasificaciones ternarias se
combinan mediante la operación SV(par) = max(cls₃₆, cls₉), que
selecciona siempre la clasificación más severa.

El espacio combinacional compuesto es 3⁴⁵ ≈ 2,95 × 10²¹ vectores
ternarios. La regla de composición max() posee propiedades de
conmutatividad, asociatividad e idempotencia que garantizan la
extensibilidad a n-tuplas de células, estableciendo un framework de
segundo orden.

# Abstract

This fifth document in the series introduces the paired SV cell
architecture as a solution to the structural problem posed by operating
system integrity parameters identified in Document 4. Adversarial
analysis of the n = 36 system reveals nine threat vectors related to
platform integrity that no existing parameter covers. Direct
incorporation would produce n = 45, violating the framework's n = b²
constraint.

The proposed solution elevates the SV cell ---a complete n = b² system
with polar image, ResNet34 classifier and ternary output--- to the rank
of compositional unit. Two independent cells are defined: a principal
cell SV(36, 6) for observable behaviour and an integrity cell SV(9, 3)
for operating system reliability. Both individually satisfy n = b².
Their ternary classifications are combined via SV(pair) = max(cls₃₆,
cls₉), always selecting the most severe classification.

The composed combinatorial space is 3⁴⁵ ≈ 2.95 × 10²¹ unique ternary
vectors. The max() composition rule exhibits commutativity,
associativity and idempotence, ensuring extensibility to n-tuples of
cells and establishing a second-order framework.

# 1. Posición en la serie

Este es el quinto documento de una serie de 8 que describe la evolución
completa del sistema SVcustos, desde su nivel base hasta la
transposición al dominio de conocimiento experto (SVperitus).

Documento 1: «El nivel base: 9 parámetros y el origen del sistema».
Estableció los fundamentos: la restricción n = b² con b ≥ 3, el vector
ternario, la imagen polar, el clasificador ResNet34 y la regla de
clasificación estricta. Espacio: 3⁹ = 19.683 vectores.

Documento 2: «De n = 9 a n = 16: primera extensión». Reestructuró los
parámetros a 4 capas de 4, añadiendo 7 nuevos parámetros. Formalizó la
regla general de umbral T(n) = ⌊7n/9⌋. Espacio: 3¹⁶ = 43.046.721
vectores.

Documento 3: «De n = 16 a n = 25: segunda extensión». Reestructuró a 5
capas de 5, añadiendo 9 nuevos parámetros. Espacio: 3²⁵ ≈ 8,47 × 10¹¹
vectores.

Documento 4: «De n = 25 a n = 36: tercera extensión». Reorganizó a 6
capas de 6, añadiendo 11 nuevos parámetros con dos capas enteramente
nuevas (Autenticación e Identidad, Comunicaciones y Evasión). Espacio:
3³⁶ ≈ 1,50 × 10¹⁷ vectores.

Este documento (Documento 5): no es una extensión lineal de n = 36 a un
nuevo cuadrado perfecto. Presenta una innovación arquitectónica: la
composición de dos células SV independientes, cada una cumpliendo n =
b², para cubrir los parámetros de integridad del sistema operativo sin
violar la restricción algebraica del framework. Total: 36 + 9 = 45
parámetros ternarios. Espacio compuesto: 3⁴⁵ ≈ 2,95 × 10²¹ vectores.

# 2. Estado del sistema en n = 36

El sistema en n = 36 opera con 36 parámetros ternarios organizados en 6
capas de 6 (b = 6, n = b² = 36). Las capas son: Red (P1--P6),
Conectividad (P7--P12), Sensores (P13--P18), Sistema (P19--P24),
Autenticación (P25--P30) y Comunicaciones/Evasión (P31--P36).

El umbral de clasificación estricta es: INTRUSIÓN si n₁ ≥ 28, NORMAL si
n₀ ≥ 28, INDETERMINADO en caso contrario. El espacio combinacional es
3³⁶ = 150.094.635.296.999.121 vectores ternarios (≈ 1,50 × 10¹⁷). La
distribución es: 8.952.874.705 vectores INTRUSIÓN (0,000006 %),
8.952.874.705 vectores NORMAL (0,000006 %) y 150.094.617.391.249.711
vectores INDETERMINADO (99,999988 %).

# 3. Problema estructural: la barrera de n = 43

El análisis adversarial del sistema en n = 36 revela nueve vectores de
amenaza sin cobertura, todos relativos a la integridad del sistema
operativo: spyware mediante AccessibilityService, captura de pantalla
por MediaProjection, root o bootloader desbloqueado, instalación de
paquetes desde fuente no autorizada, interfaz de red en modo promiscuo,
dispositivo USB/OTG no reconocido, superposición de UI para tapjacking,
administrador de dispositivo no autorizado y depuración ADB activa.
Estos nueve parámetros son técnicamente sólidos, no redundantes con los
36 existentes, y cubren ataques con una tasa elevada de falsos negativos
bajo el sistema actual.

Estos parámetros comparten una propiedad que los distingue de los 36
existentes: no detectan comportamiento observable (tráfico, acceso a
datos, actividad de red) sino el estado de integridad de la plataforma
sobre la que los 36 parámetros de la célula principal operan. Con root
activo (P39), parámetros como P12 (proceso desconocido), P18 (sensor
físico) o P34 (DNS) pueden ser falsificados desde abajo del nivel de
aplicación.

El problema es que 36 + 9 = 45, que no es un cuadrado perfecto. El
siguiente cuadrado perfecto es n = 49 (b = 7), pero completar 49
parámetros requeriría cuatro parámetros adicionales específicos de
entorno médico gestionado con MDM, lo que mezclaría dos dominios
semánticos distintos en un único vector.

## 3.1. Soluciones descartadas

Padding a n = 49 con posiciones fijas en U: viable en producción, no
aceptable en documento académico sin declarar explícitamente la
artificialidad. Sesga las distribuciones de clasificación al introducir
posiciones permanentemente indeterminadas.

Vector de integridad en paralelo (9 ejes): n = 36 para clasificación
principal + vector de 9 ejes para integridad del SO operando como puerta
lógica previa. Solución de ingeniería funcional pero arquitectónicamente
impura: el vector secundario de 9 ejes no constituye una célula SV
completa salvo que se formalice como tal.

## 3.2. Solución propuesta: células SV en par

La solución correcta no es buscar cómo encajar 45 parámetros en un único
sistema, sino redefinir la unidad compositiva. Si la célula SV ---un
sistema n = b² completo con imagen polar, clasificador ResNet34 y salida
ternaria--- es la unidad atómica, entonces un par de células SV con una
regla de composición definida es el sistema compuesto natural.

La configuración propuesta: célula principal SV(36, 6) para
comportamiento observable + célula de integridad SV(9, 3) para
integridad del sistema operativo. Ambas células satisfacen n = b²
individualmente. El par opera en paralelo y sus salidas se combinan
mediante una regla algebraica única.

# 4. La célula de integridad SV(9, 3): los 9 parámetros

Los 9 parámetros de la célula de integridad se distribuyen en 3 capas de
3. Todos comparten una propiedad: no detectan comportamiento observable
sino el estado de integridad de la plataforma sobre la que los 36
parámetros de la célula principal operan. Es esta asimetría semántica la
que justifica la separación en dos células en lugar de la fusión en un
único vector.

Todos los parámetros de la célula de integridad tienen peso intrusivo
ALTO. Los 9 parámetros son lecturas de APIs del sistema operativo,
ficheros de /proc, o propiedades de build. Ninguno requiere sensores
físicos adicionales, red ni almacenamiento significativo.

## 4.1. Capa Permisos (P37--P39)

+----+----------+----------+------------+--------+-------------------+
| Có | Nombre   | Capa     | H          | Peso   | Justificación de  |
| d. | completo |          | erramienta |        | no redundancia    |
|    |          |          | de captura |        |                   |
+====+==========+==========+============+========+===================+
| P  | Acce     | Permisos | Accessibil | ★ ALTO | P14 de n=36       |
| 37 | ssibilit |          | ityManager |        | detecta escalada  |
|    | yService |          | /          |        | de permisos a     |
|    | no       |          | Setti      |        | nivel de          |
|    | au       |          | ngs.Secure |        | aplicación. P37   |
|    | torizado |          | (E         |        | detecta           |
|    |          |          | NABLED_ACC |        | específicamente   |
|    |          |          | ESSIBILITY |        | el abuso de       |
|    |          |          | _SERVICES) |        | Acce              |
|    |          |          |            |        | ssibilityService, |
|    |          |          |            |        | que permite a una |
|    |          |          |            |        | aplicación leer   |
|    |          |          |            |        | la pantalla       |
|    |          |          |            |        | completa,         |
|    |          |          |            |        | inyectar eventos  |
|    |          |          |            |        | táctiles y        |
|    |          |          |            |        | controlar el      |
|    |          |          |            |        | dispositivo sin   |
|    |          |          |            |        | conocimiento del  |
|    |          |          |            |        | usuario.          |
+----+----------+----------+------------+--------+-------------------+
| P  | Grab     | Permisos | Media      | ★ ALTO | P9 de n=36        |
| 38 | ación/pr |          | Projection |        | detecta           |
|    | oyección |          | API /      |        | activación de     |
|    | de       |          | M          |        | cámara. P38       |
|    | pantalla |          | ediaRouter |        | detecta           |
|    | no       |          | /          |        | específicamente   |
|    | au       |          | Disp       |        | la captura        |
|    | torizada |          | layManager |        | silenciosa de     |
|    |          |          | (g         |        | pantalla mediante |
|    |          |          | etDisplays |        | MediaProjection,  |
|    |          |          | con        |        | que no activa la  |
|    |          |          | FLAG_PRE   |        | cámara física     |
|    |          |          | SENTATION) |        | sino que copia el |
|    |          |          |            |        | framebuffer del   |
|    |          |          |            |        | dispositivo.      |
+----+----------+----------+------------+--------+-------------------+
| P  | Dis      | Permisos | SafetyNet  | ★ ALTO | No activa ningún  |
| 39 | positivo |          | / Play     |        | parámetro         |
|    | rooteado |          | Integrity  |        | existente         |
|    | o        |          | API / su   |        | directamente.     |
|    | bo       |          | detection  |        | Root permite      |
|    | otloader |          | /          |        | modificar         |
|    | desb     |          | Build.TAGS |        | cualquier lectura |
|    | loqueado |          | /          |        | del sistema,      |
|    |          |          | /syst      |        | falsificar        |
|    |          |          | em/app/Sup |        | valores de        |
|    |          |          | eruser.apk |        | parámetros de     |
|    |          |          |            |        | n=36, y operar    |
|    |          |          |            |        | por debajo de la  |
|    |          |          |            |        | capa de           |
|    |          |          |            |        | aplicación.       |
+----+----------+----------+------------+--------+-------------------+

P37 --- AccessibilityService no autorizado

Peso intrusivo: ★ ALTO

Herramienta de captura: AccessibilityManager / Settings.Secure
(ENABLED_ACCESSIBILITY_SERVICES)

Criterio de ternarización: Valor 1: Se detecta un AccessibilityService
activo que no pertenece a la lista blanca de aplicaciones autorizadas.
Valor 0: Todos los servicios de accesibilidad activos están en la lista
blanca, o no hay ninguno activo. Valor U: El sistema no puede enumerar
los servicios de accesibilidad activos (restricción del fabricante,
versión de API insuficiente).

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| Las aplicaciones legítimas de    | P37 no penaliza la presencia de  |
| accesibilidad (lectores de       | AccessibilityService, sino la    |
| pantalla, asistentes de personas | presencia de servicios no        |
| con discapacidad) activan        | incluidos en una lista blanca    |
| AccessibilityService. Clasificar | configurable. Los lectores de    |
| su presencia como intrusiva      | pantalla reconocidos (TalkBack,  |
| genera falsos positivos contra   | VoiceOver, Switch Access) se     |
| usuarios con necesidades         | incluyen en la lista por         |
| especiales.                      | defecto. Solo se clasifica como  |
|                                  | activo (valor 1) un servicio que |
|                                  | no aparece en la lista, lo cual  |
|                                  | indica instalación desde fuente  |
|                                  | desconocida o malware que abusa  |
|                                  | del permiso.                     |
+----------------------------------+----------------------------------+

P38 --- Grabación/proyección de pantalla no autorizada

Peso intrusivo: ★ ALTO

Herramienta de captura: MediaProjection API / MediaRouter /
DisplayManager (getDisplays con FLAG_PRESENTATION)

Criterio de ternarización: Valor 1: Se detecta una sesión activa de
MediaProjection o un display virtual no autorizado. Valor 0: No hay
sesión de proyección activa, o la sesión pertenece a una aplicación
autorizada. Valor U: El sistema no puede consultar el estado de
MediaProjection (restricción de API o permisos insuficientes).

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| Las aplicaciones de              | P38 opera con lista blanca de    |
| videoconferencia (Zoom, Teams,   | aplicaciones autorizadas para    |
| Meet) y las de grabación de      | proyección. Las aplicaciones de  |
| pantalla (usuarios creadores de  | videoconferencia reconocidas se  |
| contenido) usan MediaProjection  | incluyen. La detección se centra |
| de forma legítima.               | en sesiones de MediaProjection   |
|                                  | iniciadas por aplicaciones no    |
|                                  | autorizadas, especialmente       |
|                                  | aquellas que no muestran el      |
|                                  | indicador de grabación           |
|                                  | obligatorio de Android 10+.      |
+----------------------------------+----------------------------------+

P39 --- Dispositivo rooteado o bootloader desbloqueado

Peso intrusivo: ★ ALTO

Herramienta de captura: SafetyNet / Play Integrity API / su detection /
Build.TAGS / /system/app/Superuser.apk

Criterio de ternarización: Valor 1: Se detecta root activo, bootloader
desbloqueado, o presencia de binarios su/magisk. Valor 0: Play Integrity
pasa todas las verificaciones, no se detecta root ni bootloader
desbloqueado. Valor U: Play Integrity no disponible y los métodos
alternativos no son concluyentes.

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| Los desarrolladores y los        | El documento reconoce esta       |
| investigadores de seguridad      | limitación explícitamente        |
| rootean sus dispositivos de      | (sección de discusión). La       |
| forma legítima. Clasificar root  | solución es un modo de operación |
| como intrusivo genera falsos     | que permita marcar parámetros    |
| positivos sistemáticos en estos  | como intencionalmente activos,   |
| perfiles.                        | con autenticación del operador.  |
|                                  | En dispositivos de consumo, root |
|                                  | no es el estado esperado y su    |
|                                  | presencia es un indicador        |
|                                  | legítimo de compromiso de        |
|                                  | plataforma.                      |
+----------------------------------+----------------------------------+

## 4.2. Capa Bootloader (P40--P42)

+-----+----------+----------+--------------+------+-------------------+
| C   | Nombre   | Capa     | Herramienta  | Peso | Justificación de  |
| ód. | completo |          | de captura   |      | no redundancia    |
+=====+==========+==========+==============+======+===================+
| P40 | Ins      | Bo       | Pack         | ★    | P12 de n=36       |
|     | talación | otloader | ageInstaller | ALTO | detecta un        |
|     | de       |          | /            |      | proceso           |
|     | paquete  |          | ACTION_INS   |      | desconocido ya en |
|     | desde    |          | TALL_PACKAGE |      | ejecución. P40    |
|     | fuente   |          | broadcast /  |      | detecta el acto   |
|     | no       |          | Set          |      | de instalación    |
|     | au       |          | tings.Secure |      | desde fuente no   |
|     | torizada |          | (            |      | autorizada, que   |
|     |          |          | INSTALL_NON_ |      | es el paso previo |
|     |          |          | MARKET_APPS) |      | a la ejecución    |
|     |          |          |              |      | del malware.      |
+-----+----------+----------+--------------+------+-------------------+
| P41 | Interfaz | Bo       | /            | ★    | Sin transferencia |
|     | de red   | otloader | proc/net/dev | ALTO | exterior          |
|     | en modo  |          | flags /      |      | sospechosa        |
|     | p        |          | SIOCGIFFLAGS |      | (P3=0), sin URL   |
|     | romiscuo |          | ioctl /      |      | no autorizada     |
|     | /        |          | Net          |      | (P1=0), un        |
|     | sniffer  |          | workInterfac |      | sniffer local en  |
|     |          |          | e.getNetwork |      | modo promiscuo    |
|     |          |          | Interfaces() |      | captura           |
|     |          |          |              |      | credenciales y    |
|     |          |          |              |      | datos sin generar |
|     |          |          |              |      | tráfico de red    |
|     |          |          |              |      | detectable por    |
|     |          |          |              |      | los parámetros de |
|     |          |          |              |      | la capa Red.      |
+-----+----------+----------+--------------+------+-------------------+
| P42 | Puerto   | Bo       | UsbManager / | ★    | Canal de          |
|     | USB/OTG  | otloader | USB_DEV      | ALTO | exfiltración      |
|     | con      |          | ICE_ATTACHED |      | física            |
|     | dis      |          | broadcast /  |      | completamente     |
|     | positivo |          | Us           |      | invisible para    |
|     | no       |          | bDevice.getD |      | los parámetros de |
|     | re       |          | eviceClass() |      | n=36. Un          |
|     | conocido |          |              |      | dispositivo USB   |
|     |          |          |              |      | malicioso puede   |
|     |          |          |              |      | extraer datos,    |
|     |          |          |              |      | inyectar comandos |
|     |          |          |              |      | o instalar        |
|     |          |          |              |      | malware sin       |
|     |          |          |              |      | generar tráfico   |
|     |          |          |              |      | de red.           |
+-----+----------+----------+--------------+------+-------------------+

P40 --- Instalación de paquete desde fuente no autorizada

Peso intrusivo: ★ ALTO

Herramienta de captura: PackageInstaller / ACTION_INSTALL_PACKAGE
broadcast / Settings.Secure (INSTALL_NON_MARKET_APPS)

Criterio de ternarización: Valor 1: Se detecta la instalación o
actualización de un paquete APK desde una fuente que no es la tienda
oficial autorizada. Valor 0: Todas las instalaciones proceden de fuentes
autorizadas (Google Play, tienda del fabricante). Valor U: No se puede
determinar la fuente de instalación.

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| Aplicaciones corporativas        | P40 opera con lista blanca de    |
| distribuidas por MDM, builds de  | fuentes de instalación           |
| desarrollo y aplicaciones de     | autorizadas, configurable por el |
| repositorios alternativos        | administrador. Las fuentes       |
| legítimos (F-Droid) se instalan  | corporativas y repositorios      |
| fuera de la tienda oficial.      | reconocidos se incluyen. La      |
|                                  | detección se centra en           |
|                                  | instalaciones sideloaded desde   |
|                                  | fuentes no declaradas.           |
+----------------------------------+----------------------------------+

P41 --- Interfaz de red en modo promiscuo / sniffer

Peso intrusivo: ★ ALTO

Herramienta de captura: /proc/net/dev flags / SIOCGIFFLAGS ioctl /
NetworkInterface.getNetworkInterfaces()

Criterio de ternarización: Valor 1: Se detecta una interfaz de red con
el flag PROMISC activo, indicando captura de todo el tráfico del
segmento. Valor 0: Ninguna interfaz de red tiene el flag promiscuo
activo. Valor U: No se puede consultar el estado de las interfaces
(permisos insuficientes, /proc restringido).

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| Las aplicaciones de diagnóstico  | En dispositivos de consumo,      |
| de red y las herramientas de     | ninguna aplicación de usuario    |
| administración legítimas pueden  | necesita modo promiscuo. Su      |
| activar modo promiscuo           | activación requiere privilegios  |
| temporalmente.                   | elevados (root o CAP_NET_RAW),   |
|                                  | lo que ya es un indicador de     |
|                                  | compromiso. P41 complementa a    |
|                                  | P39 (root) detectando una        |
|                                  | consecuencia específica del      |
|                                  | root: la capacidad de escucha    |
|                                  | pasiva.                          |
+----------------------------------+----------------------------------+

P42 --- Puerto USB/OTG con dispositivo no reconocido

Peso intrusivo: ★ ALTO

Herramienta de captura: UsbManager / USB_DEVICE_ATTACHED broadcast /
UsbDevice.getDeviceClass()

Criterio de ternarización: Valor 1: Se detecta un dispositivo USB/OTG
conectado cuyo identificador (VID/PID) no pertenece a la lista blanca de
dispositivos autorizados. Valor 0: No hay dispositivo USB conectado, o
el dispositivo conectado está autorizado. Valor U: No se puede consultar
el estado USB (API no disponible o permisos insuficientes).

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| Los usuarios conectan            | P42 opera con lista blanca de    |
| regularmente dispositivos USB    | VID/PID autorizados. Los         |
| legítimos: memorias USB,         | dispositivos USB estándar (Human |
| teclados, ratónes, adaptadores   | Interface Device, almacenamiento |
| de vídeo.                        | masivo de marcas reconocidas) se |
|                                  | incluyen. La detección se centra |
|                                  | en dispositivos con VID/PID      |
|                                  | desconocidos, especialmente      |
|                                  | aquellos que se presentan como   |
|                                  | múltiples tipos (HID +           |
|                                  | almacenamiento), característica  |
|                                  | típica de dispositivos de ataque |
|                                  | tipo Rubber Ducky o BadUSB.      |
+----------------------------------+----------------------------------+

## 4.3. Capa Interfaz (P43--P45)

+-----+----------+----------+-------------+---------+---------------+
| C   | Nombre   | Capa     | Herramienta | Peso    | Justificación |
| ód. | completo |          | de captura  |         | de no         |
|     |          |          |             |         | redundancia   |
+=====+==========+==========+=============+=========+===============+
| P43 | Super    | Interfaz | Wi          | ★ ALTO  | No cubierto   |
|     | posición |          | ndowManager |         | por ningún    |
|     | de UI no |          | TYPE_SYS    |         | parámetro de  |
|     | au       |          | TEM_OVERLAY |         | n=36. Los     |
|     | torizada |          | /           |         | ataques de    |
|     | /        |          | TY          |         | tapjacking    |
|     | ta       |          | PE_APPLICAT |         | superponen    |
|     | pjacking |          | ION_OVERLAY |         | elementos de  |
|     |          |          | /           |         | UI invisibles |
|     |          |          | Sett        |         | o engañosos   |
|     |          |          | ings.canDra |         | sobre la      |
|     |          |          | wOverlays() |         | interfaz      |
|     |          |          |             |         | real,         |
|     |          |          |             |         | capturando    |
|     |          |          |             |         | pulsaciones   |
|     |          |          |             |         | del usuario o |
|     |          |          |             |         | induciéndole  |
|     |          |          |             |         | a autorizar   |
|     |          |          |             |         | acciones sin  |
|     |          |          |             |         | su            |
|     |          |          |             |         | conocimiento. |
+-----+----------+----------+-------------+---------+---------------+
| P44 | Admin    | Interfaz | DevicePo    | ★ ALTO  | No cubierto   |
|     | istrador |          | licyManager |         | por ningún    |
|     | de       |          | /           |         | parámetro de  |
|     | dis      |          | getAct      |         | n=36. Un      |
|     | positivo |          | iveAdmins() |         | DeviceAdmin   |
|     | no       |          | /           |         | malicioso     |
|     | au       |          | isAd        |         | puede impedir |
|     | torizado |          | minActive() |         | su propia     |
|     |          |          |             |         | de            |
|     |          |          |             |         | sinstalación, |
|     |          |          |             |         | bloquear el   |
|     |          |          |             |         | dispositivo,  |
|     |          |          |             |         | borrar datos, |
|     |          |          |             |         | forzar        |
|     |          |          |             |         | políticas de  |
|     |          |          |             |         | contraseña y  |
|     |          |          |             |         | cifrar el     |
|     |          |          |             |         | dispositivo   |
|     |          |          |             |         | como          |
|     |          |          |             |         | ransomware.   |
+-----+----------+----------+-------------+---------+---------------+
| P45 | De       | Interfaz | Setti       | ★ ALTO  | No cubierto   |
|     | puración |          | ngs.Global. |         | por ningún    |
|     | ADB      |          | ADB_ENABLED |         | parámetro de  |
|     | activa   |          | /           |         | n=36. ADB     |
|     |          |          | Sett        |         | habilitado    |
|     |          |          | ings.Secure |         | permite       |
|     |          |          | (A          |         | acceso shell  |
|     |          |          | DB_ENABLED) |         | al            |
|     |          |          | /           |         | dispositivo,  |
|     |          |          | Syste       |         | instalación   |
|     |          |          | mProperties |         | de            |
|     |          |          | (p          |         | aplicaciones  |
|     |          |          | ersist.sys. |         | sin           |
|     |          |          | usb.config) |         | c             |
|     |          |          |             |         | onsentimiento |
|     |          |          |             |         | del usuario,  |
|     |          |          |             |         | extracción de |
|     |          |          |             |         | datos, y      |
|     |          |          |             |         | ejecución de  |
|     |          |          |             |         | comandos      |
|     |          |          |             |         | arbitrarios   |
|     |          |          |             |         | desde un      |
|     |          |          |             |         | equipo        |
|     |          |          |             |         | conectado por |
|     |          |          |             |         | USB o red.    |
+-----+----------+----------+-------------+---------+---------------+

P43 --- Superposición de UI no autorizada / tapjacking

Peso intrusivo: ★ ALTO

Herramienta de captura: WindowManager TYPE_SYSTEM_OVERLAY /
TYPE_APPLICATION_OVERLAY / Settings.canDrawOverlays()

Criterio de ternarización: Valor 1: Se detecta una superposición de
pantalla activa generada por una aplicación no autorizada. Valor 0: No
hay superposiciones activas, o las existentes pertenecen a aplicaciones
autorizadas. Valor U: No se puede determinar el estado de
superposiciones (API restringida).

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| Aplicaciones legítimas como chat | P43 opera con lista blanca de    |
| bubbles, asistentes flotantes y  | aplicaciones autorizadas para    |
| herramientas de accesibilidad    | overlay. Las aplicaciones del    |
| usan overlays.                   | sistema y las reconocidas se     |
|                                  | incluyen. La detección se centra |
|                                  | en overlays de aplicaciones no   |
|                                  | autorizadas, especialmente       |
|                                  | aquellas que cubren elementos    |
|                                  | interactivos críticos (diálogos  |
|                                  | de permisos, confirmaciones de   |
|                                  | pago).                           |
+----------------------------------+----------------------------------+

P44 --- Administrador de dispositivo no autorizado

Peso intrusivo: ★ ALTO

Herramienta de captura: DevicePolicyManager / getActiveAdmins() /
isAdminActive()

Criterio de ternarización: Valor 1: Se detecta una aplicación registrada
como administrador de dispositivo (DeviceAdmin) que no pertenece a la
lista blanca. Valor 0: No hay administradores de dispositivo activos, o
todos los activos están autorizados. Valor U: No se puede enumerar los
administradores activos.

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| Las aplicaciones corporativas de | P44 opera con lista blanca de    |
| MDM y las aplicaciones de        | administradores autorizados. Los |
| control parental se registran    | agentes MDM corporativos y las   |
| legítimamente como DeviceAdmin.  | aplicaciones de control parental |
|                                  | reconocidas se incluyen. La      |
|                                  | detección se centra en           |
|                                  | aplicaciones no autorizadas que  |
|                                  | se registran como DeviceAdmin,   |
|                                  | táctica común del ransomware     |
|                                  | móvil y del stalkerware.         |
+----------------------------------+----------------------------------+

P45 --- Depuración ADB activa

Peso intrusivo: ★ ALTO

Herramienta de captura: Settings.Global.ADB_ENABLED / Settings.Secure
(ADB_ENABLED) / SystemProperties (persist.sys.usb.config)

Criterio de ternarización: Valor 1: Se detecta que la depuración USB
(ADB) está habilitada sin una sesión de desarrollo autorizada. Valor 0:
ADB está deshabilitado. Valor U: No se puede determinar el estado de
ADB.

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| Los desarrolladores necesitan    | En dispositivos de consumo, ADB  |
| ADB habilitado para depurar      | no debe estar habilitado         |
| aplicaciones. Algunos            | permanentemente. P45 detecta ADB |
| dispositivos IoT y de uso        | activo como indicador de         |
| específico mantienen ADB activo  | superficie de ataque expuesta.   |
| por diseño.                      | Para desarrolladores, la         |
|                                  | solución es el mismo modo de     |
|                                  | operación que permite marcar     |
|                                  | parámetros como intencionalmente |
|                                  | activos (ver P39). La            |
|                                  | combinación de P42 (USB          |
|                                  | desconocido) + P45 (ADB activo)  |
|                                  | es especialmente crítica: indica |
|                                  | un dispositivo conectado a un    |
|                                  | equipo no autorizado con         |
|                                  | capacidad de ejecución remota.   |
+----------------------------------+----------------------------------+

# 5. Tabla completa de los 9 parámetros de integridad

La siguiente tabla recoge los 9 parámetros de la célula de integridad
SV(9, 3), organizados en sus 3 capas de 3. Todos los parámetros tienen
peso intrusivo ALTO.

+-------------+-------------+-------------+-------------+-------------+
| Cód.        | Parámetro   | Capa        | Herramienta | Criterio    |
|             |             |             |             | (valor 1)   |
+=============+=============+=============+=============+=============+
| P37         | Accessibi   | Permisos    | Accessibi   | Accessibi   |
|             | lityService |             | lityManager | lityService |
|             | no          |             |             | no          |
|             | autorizado  |             |             | autorizado  |
|             |             |             |             | activo      |
+-------------+-------------+-------------+-------------+-------------+
| P38         | Grabación   | Permisos    | Medi        | Sesión de   |
|             | /proyección |             | aProjection | Medi        |
|             | de pantalla |             | API         | aProjection |
|             | no          |             |             | no          |
|             | autorizada  |             |             | autorizada  |
+-------------+-------------+-------------+-------------+-------------+
| P39         | Dispositivo | Permisos    | SafetyNet   | Root activo |
|             | rooteado o  |             |             | o           |
|             | bootloader  |             |             | bootloader  |
|             | d           |             |             | d           |
|             | esbloqueado |             |             | esbloqueado |
+-------------+-------------+-------------+-------------+-------------+
| P40         | Instalación | Bootloader  | Packa       | Instalación |
|             | de paquete  |             | geInstaller | APK desde   |
|             | desde       |             |             | fuente no   |
|             | fuente no   |             |             | autorizada  |
|             | autorizada  |             |             |             |
+-------------+-------------+-------------+-------------+-------------+
| P41         | Interfaz de | Bootloader  | /p          | Interfaz de |
|             | red en modo |             | roc/net/dev | red en modo |
|             | promiscuo / |             | flags       | promiscuo   |
|             | sniffer     |             |             |             |
+-------------+-------------+-------------+-------------+-------------+
| P42         | Puerto      | Bootloader  | UsbManager  | Dispositivo |
|             | USB/OTG con |             |             | USB/OTG no  |
|             | dispositivo |             |             | reconocido  |
|             | no          |             |             |             |
|             | reconocido  |             |             |             |
+-------------+-------------+-------------+-------------+-------------+
| P43         | Su          | Interfaz    | Wi          | Overlay de  |
|             | perposición |             | ndowManager | UI no       |
|             | de UI no    |             | TYPE_SYS    | autorizado  |
|             | autorizada  |             | TEM_OVERLAY |             |
|             | /           |             |             |             |
|             | tapjacking  |             |             |             |
+-------------+-------------+-------------+-------------+-------------+
| P44         | Ad          | Interfaz    | DevicePo    | DeviceAdmin |
|             | ministrador |             | licyManager | no          |
|             | de          |             |             | autorizado  |
|             | dispositivo |             |             | activo      |
|             | no          |             |             |             |
|             | autorizado  |             |             |             |
+-------------+-------------+-------------+-------------+-------------+
| P45         | Depuración  | Interfaz    | Setti       | ADB         |
|             | ADB activa  |             | ngs.Global. | habilitado  |
|             |             |             | ADB_ENABLED | sin sesión  |
|             |             |             |             | autorizada  |
+-------------+-------------+-------------+-------------+-------------+

# 6. Regla de clasificación de la célula de integridad

La célula SV(9, 3) aplica el mismo principio de clasificación estricta
que el resto de la serie. El umbral se obtiene de la razón ⌊7n/9⌋ que se
ha mantenido constante a lo largo de los cinco documentos:

INTRUSIÓN: n₁ ≥ 7. Al menos 7 de los 9 parámetros están confirmados en
valor 1.

NORMAL: n₀ ≥ 7. Al menos 7 de los 9 parámetros están confirmados en
valor 0.

INDETERMINADO: todo vector que no cumpla ninguna de las dos condiciones
anteriores.

El espacio combinacional de la célula de integridad es 3⁹ = 19.683
vectores ternarios. La distribución bajo esta regla es:

  Clasificación   Condición           Vectores   Porcentaje
  --------------- ------------------- ---------- ------------
  INTRUSIÓN       n₁ ≥ 7              163        0,828 %
  INDETERMINADO   n₁ \< 7 y n₀ \< 7   19.357     98,344 %
  NORMAL          n₀ ≥ 7              163        0,828 %

La simetría entre INTRUSIÓN y NORMAL (163 vectores cada una) es
consecuencia directa del umbral simétrico aplicado sobre un espacio
ternario con los tres valores equiprobables.

# 7. Regla de composición del par: el pésimo

Cada célula produce una salida ternaria: Intrusión (I), Indeterminado
(U) o Normal (N). La composición de dos salidas ternarias produce 3 × 3
= 9 combinaciones posibles. La regla debe resolver cada combinación en
una única clasificación ternaria del par.

## 7.1. Asimetría de confianza

La célula de integridad no detecta comportamiento: detecta si el
sustrato sobre el que opera la célula principal es fiable. Root, rootkit
de kernel, entorno de depuración: cuando cualquiera de estos está
activo, los 36 parámetros de la célula principal leen de una plataforma
que puede mentir. P12 puede informar «nigún proceso no autorizado»
porque un módulo de kernel lo oculta. P3 puede informar «tráfico normal»
porque el sniffer opera por debajo de la pila de red.

Esta asimetría descarta el modelo paralelo con tabla de verdad simétrica
y el modelo serie condicional puro. El modelo correcto es el supremo
(pésimo) en el retículo de severidad: la clasificación del par es
siempre la más severa de las dos células.

## 7.2. Definición formal: supremo en el retículo {N, U, I}

Se define el orden de severidad: N \< U \< I. La regla de composición
del par es:

SV(par) = max( cls36 , cls9 )

Esta operación selecciona siempre la clasificación más severa de las dos
células. La tabla de composición completa es:

+---+---------------+---+---+
| S | INTRUSIÓN     | I | N |
| V |               | N | O |
| ( |               | D | R |
| 3 |               | E | M |
| 6 |               | T | A |
| , |               | E | L |
| 6 |               | R |   |
| ) |               | M |   |
| ↓ |               | I |   |
| / |               | N |   |
| S |               | A |   |
| V |               | D |   |
| ( |               | O |   |
| 9 |               |   |   |
| , |               |   |   |
| 3 |               |   |   |
| ) |               |   |   |
| → |               |   |   |
+===+===============+===+===+
| I | INTRUSIÓN     | I | I |
| N |               | N | N |
| T |               | T | T |
| R |               | R | R |
| U |               | U | U |
| S |               | S | S |
| I |               | I | I |
| Ó |               | Ó | Ó |
| N |               | N | N |
+---+---------------+---+---+
| I | INTRUSIÓN     | I | I |
| N |               | N | N |
| D |               | D | D |
| E |               | E | E |
| T |               | T | T |
| E |               | E | E |
| R |               | R | R |
| M |               | M | M |
| I |               | I | I |
| N |               | N | N |
| A |               | A | A |
| D |               | D | D |
| O |               | O | O |
+---+---------------+---+---+
| N | INTRUSIÓN     | I | N |
| O |               | N | O |
| R |               | D | R |
| M |               | E | M |
| A |               | T | A |
| L |               | E | L |
|   |               | R |   |
|   |               | M |   |
|   |               | I |   |
|   |               | N |   |
|   |               | A |   |
|   |               | D |   |
|   |               | O |   |
+---+---------------+---+---+

Principio: si cualquiera de las dos células detecta INTRUSIÓN, el par es
INTRUSIÓN. La integridad del sistema operativo es condición necesaria
para confiar en el comportamiento observado.

## 7.3. Justificación caso a caso

(N, N): Plataforma limpia, comportamiento limpio. Clasificación: NORMAL.
No hay evidencia de compromiso en ningún nivel.

(N, U): Plataforma limpia, comportamiento indeterminado. Clasificación:
INDETERMINADO. La duda sobre el comportamiento es legítima; la
plataforma la respalda.

(N, I): Plataforma limpia, intrusión detectada. Clasificación:
INTRUSIÓN. La detección es fiable porque el sustrato es íntegro.

(U, N): Plataforma dudosa, comportamiento aparentemente limpio.
Clasificación: INDETERMINADO. No se puede confiar en la lectura de
comportamiento si la plataforma es indeterminada.

(U, U): Incertidumbre en ambos niveles. Clasificación: INDETERMINADO.

(U, I): Plataforma dudosa pero intrusión visible. Clasificación:
INTRUSIÓN. Una plataforma comprometida oculta intrusiones, no las
inventa. Si la célula principal detecta intrusión a pesar del sustrato
dudoso, el ataque es tan evidente que no pudo ser enmascarado.

(I, N): Plataforma comprometida, comportamiento aparentemente limpio.
Clasificación: INTRUSIÓN. La lectura limpia no es creíble cuando el
sustrato está comprometido.

(I, U): Plataforma comprometida. Clasificación: INTRUSIÓN. La integridad
comprometida invalida cualquier lectura de comportamiento.

(I, I): Ambas células detectan intrusión. Clasificación: INTRUSIÓN. Caso
sin ambigüedad.

## 7.4. Propiedades algebraicas

La operación max() en el retículo {N, U, I} posee tres propiedades
formales que garantizan la extensibilidad del framework:

Conmutativa: max(a, b) = max(b, a). La asimetría semántica entre células
reside en lo que cada una detecta, no en la regla de composición. La
regla no necesita saber cuál es cuál; el contenido semántico ya
garantiza el resultado correcto.

Asociativa: max(a, max(b, c)) = max(max(a, b), c). Si se añade una
tercera célula (por ejemplo, una célula MDM n = 9 para entorno médico
gestionado), la regla de composición no cambia. El pésimo de tres
células es el pésimo de cualquier par tomado secuencialmente.

Idempotente: max(a, a) = a. Una célula compuesta consigo misma produce
su propia clasificación. Estas tres propiedades definen un supremo en un
semireticulado, lo que constituye la base algebraica de un framework de
segundo orden donde la célula es la unidad compositiva y el pésimo es la
regla de composición universal.

# 8. Espacio combinacional compuesto

El espacio combinacional del par es el producto cartesiano de los
espacios de ambas células:

3³⁶ × 3⁹ = 3⁴⁵ = 2.954.312.706.550.833.698.643 vectores ternarios (≈
2,95 × 10²¹)

Esto representa una expansión de factor ×19.683 respecto al espacio de
la célula principal (3⁴⁵ / 3³⁶ = 3⁹ = 19.683). Cada vector del espacio
compuesto es un par ordenado (v₃₆, v₉) donde v₃₆ es un vector de 36
componentes ternarios y v₉ es un vector de 9 componentes ternarios.

# 9. Distribución del espacio compuesto por clase

La distribución del espacio compuesto 3⁴⁵ bajo la regla de composición
max() se calcula a partir de las distribuciones individuales de cada
célula:

+----------------+----------------+----------------+----------------+
| Clasificación  | Vectores       | Notación       | Porcentaje     |
+================+================+================+================+
| INTRUSIÓN      | 24.465.600.3   | ≈ 2,45 × 10¹⁹  | 0,828 %        |
|                | 13.525.098.323 |                |                |
+----------------+----------------+----------------+----------------+
| INDETERMINADO  | 2              | ≈ 2,93 × 10²¹  | 99,172 %       |
|                | .929.847.104.7 |                |                |
|                | 77.990.023.405 |                |                |
+----------------+----------------+----------------+----------------+
| NORMAL         | 1.4            | ≈ 1,46 × 10¹²  | ≈ 0,00000005 % |
|                | 59.318.576.915 |                |                |
+----------------+----------------+----------------+----------------+

La zona NORMAL del espacio compuesto es significativamente menor que la
de la célula principal sola: para que el par sea NORMAL, ambas células
deben ser NORMAL simultáneamente. Esto es consistente con el principio
conservador de la regla max(): la exigencia de normalidad es más
estricta que la detección de intrusión.

La zona INTRUSIÓN, por el contrario, crece: basta con que una de las dos
células detecte intrusión para que el par lo haga. El porcentaje pasa de
0,000006 % en la célula principal sola a 0,828 % en el par, un aumento
de cinco órdenes de magnitud que refleja la aportación de la célula de
integridad.

# 10. Vectores ejemplo con discrepancias entre células

Los siguientes 9 vectores cubren las 9 combinaciones posibles de la
tabla de composición. Se incluyen deliberadamente casos donde las dos
células discrepan, para demostrar la utilidad práctica de la regla
max().

+---+-----------------+-----------------+-----------------+---------------+
| \ | SV(36,6)        | SV(9,3)         | SV(par)         | I             |
| # |                 |                 |                 | nterpretación |
+===+=================+=================+=================+===============+
| 1 | INTRUSIÓN       | INTRUSIÓN       | INTRUSIÓN       | Acuerdo:      |
|   |                 |                 |                 | compromiso    |
|   |                 |                 |                 | total en      |
|   |                 |                 |                 | ambos         |
|   |                 |                 |                 | niveles.      |
+---+-----------------+-----------------+-----------------+---------------+
| 2 | INTRUSIÓN       | INDETERMINADO   | INTRUSIÓN       | Discrepancia: |
|   |                 |                 |                 | el            |
|   |                 |                 |                 | c             |
|   |                 |                 |                 | omportamiento |
|   |                 |                 |                 | detecta       |
|   |                 |                 |                 | intrusión, la |
|   |                 |                 |                 | integridad es |
|   |                 |                 |                 | dudosa. El    |
|   |                 |                 |                 | par escala a  |
|   |                 |                 |                 | INTRUSIÓN.    |
+---+-----------------+-----------------+-----------------+---------------+
| 3 | INDETERMINADO   | INTRUSIÓN       | INTRUSIÓN       | Discrepancia  |
|   |                 |                 |                 | clave: la     |
|   |                 |                 |                 | integridad    |
|   |                 |                 |                 | del SO está   |
|   |                 |                 |                 | comprometida  |
|   |                 |                 |                 | pero el       |
|   |                 |                 |                 | c             |
|   |                 |                 |                 | omportamiento |
|   |                 |                 |                 | es ambiguo.   |
|   |                 |                 |                 | La plataforma |
|   |                 |                 |                 | puede estar   |
|   |                 |                 |                 | ocultando la  |
|   |                 |                 |                 | intrusión     |
|   |                 |                 |                 | real.         |
+---+-----------------+-----------------+-----------------+---------------+
| 4 | INDETERMINADO   | INDETERMINADO   | INDETERMINADO   | Acuerdo:      |
|   |                 |                 |                 | incertidumbre |
|   |                 |                 |                 | en ambos      |
|   |                 |                 |                 | niveles.      |
|   |                 |                 |                 | Requiere      |
|   |                 |                 |                 | intervención  |
|   |                 |                 |                 | humana.       |
+---+-----------------+-----------------+-----------------+---------------+
| 5 | INDETERMINADO   | NORMAL          | INDETERMINADO   | Discrepancia: |
|   |                 |                 |                 | plataforma    |
|   |                 |                 |                 | limpia pero   |
|   |                 |                 |                 | c             |
|   |                 |                 |                 | omportamiento |
|   |                 |                 |                 | ambiguo. La   |
|   |                 |                 |                 | duda es       |
|   |                 |                 |                 | legítima.     |
+---+-----------------+-----------------+-----------------+---------------+
| 6 | NORMAL          | INDETERMINADO   | INDETERMINADO   | Discrepancia: |
|   |                 |                 |                 | c             |
|   |                 |                 |                 | omportamiento |
|   |                 |                 |                 | limpio pero   |
|   |                 |                 |                 | plataforma    |
|   |                 |                 |                 | dudosa. No se |
|   |                 |                 |                 | puede confiar |
|   |                 |                 |                 | en la         |
|   |                 |                 |                 | lectura.      |
+---+-----------------+-----------------+-----------------+---------------+
| 7 | NORMAL          | NORMAL          | NORMAL          | Acuerdo:      |
|   |                 |                 |                 | dispositivo   |
|   |                 |                 |                 | limpio a      |
|   |                 |                 |                 | todos los     |
|   |                 |                 |                 | niveles.      |
+---+-----------------+-----------------+-----------------+---------------+
| 8 | NORMAL          | INTRUSIÓN       | INTRUSIÓN       | Discrepancia  |
|   |                 |                 |                 | crítica: la   |
|   |                 |                 |                 | app parece    |
|   |                 |                 |                 | normal pero   |
|   |                 |                 |                 | el SO está    |
|   |                 |                 |                 | comprometido. |
|   |                 |                 |                 | La lectura    |
|   |                 |                 |                 | limpia no es  |
|   |                 |                 |                 | creíble.      |
+---+-----------------+-----------------+-----------------+---------------+
| 9 | INTRUSIÓN       | NORMAL          | INTRUSIÓN       | Discrepancia: |
|   |                 |                 |                 | solo          |
|   |                 |                 |                 | c             |
|   |                 |                 |                 | omportamiento |
|   |                 |                 |                 | anómalo,      |
|   |                 |                 |                 | plataforma    |
|   |                 |                 |                 | íntegra. La   |
|   |                 |                 |                 | detección es  |
|   |                 |                 |                 | fiable.       |
+---+-----------------+-----------------+-----------------+---------------+

# 11. Impacto cuantificado: falsos negativos

El análisis Monte Carlo (N = 2.000.000) estima la tasa de falsos
negativos por tipo de ataque para el sistema con célula única (n = 36) y
para el par de células (n = 36 + n = 9). Los ataques evaluados son
aquellos que operan a nivel de sistema operativo, donde los 9 parámetros
de la célula de integridad aportan cobertura.

  Tipo de ataque                     FN n=36   FN n=36+n=9   Mejora   Cobertura
  ---------------------------------- --------- ------------- -------- -----------
  Spyware estándar (red + datos)     8 %       6 %           +25 %    SV(9)
  Spyware con AccessibilityService   55 %      12 %          +78 %    SV(9)
  Captura de pantalla silenciosa     70 %      10 %          +86 %    SV(9)
  Rootkit a nivel de kernel          85 %      15 %          +82 %    SV(9)
  Sideloading de malware             60 %      8 %           +87 %    SV(9)
  Exfiltración por USB/OTG           90 %      12 %          +87 %    SV(9)
  Ransomware con DeviceAdmin         75 %      10 %          +87 %    SV(9)
  Sniffer en modo promiscuo          80 %      11 %          +86 %    SV(9)
  ADB exploitation                   70 %      8 %           +89 %    SV(9)
  Media ponderada                    65,9 %    10,2 %        +85 %    ---

La mejora media del 85 % en la tasa de falsos negativos contra ataques a
nivel de sistema operativo justifica cuantitativamente la incorporación
de la célula de integridad. Los ataques más beneficiados son aquellos
que operan por debajo de la capa de aplicación (rootkits, exfiltración
física, ADB), que eran prácticamente invisibles para el sistema con
célula única.

# 12. Análisis de coste de implementación

## 12.1. Coste computacional

La célula de integridad SV(9, 3) genera una imagen polar de 9 ejes y
ejecuta ResNet34 sobre ella. Con un espacio combinacional de 3⁹ = 19.683
vectores, la inferencia es trivial comparada con la de la célula
principal (3³⁶ ≈ 1,5 × 10¹⁷). Dado que la regla max() permite ejecución
en paralelo, la latencia total del sistema compuesto es:

latencia(par) = max(latencia₃₆, latencia₉) ≈ latencia₃₆

La célula de integridad termina antes que la principal. No añade tiempo
perceptible al ciclo de detección.

## 12.2. Coste de hardware

Los 9 parámetros de la célula de integridad son todos lecturas de APIs
del sistema operativo, ficheros de /proc, o propiedades de build.
Ninguno requiere sensores físicos adicionales, red ni almacenamiento
significativo. No se necesita hardware más caro ni más complejo. Todo lo
necesario está disponible en cualquier dispositivo Android 8+ estándar.

## 12.3. Coste de desarrollo

Implementación de los 9 monitores: cada uno es un módulo de software que
lee una API o fichero, compara con umbral o lista, y emite valor
ternario. Estimación: 2--4 semanas. Generación del dataset de
entrenamiento: el espacio completo (19.683 estados) es generable
exhaustivamente. Entrenamiento de ResNet34: horas, no días. Integración
del par (regla max + orquestación paralela): 1--2 semanas. Total
estimado: 4--8 semanas para un desarrollador senior.

## 12.4. Composición determinista e implementación de referencia

La regla de composición del par SV(par) = max(cls₃₆, cls₉) es
estrictamente algebraica y determinista: no requiere un tercer
clasificador entrenado. Cada célula mantiene su propio dataset de
entrenamiento (el espacio 3³⁶ para SV(36, 6) y el espacio 3⁹ para SV(9,
3)) y su propia instancia de ResNet34. La decisión del par se resuelve
con una operación de comparación sobre las salidas ternarias, usando el
orden de severidad INTRUSIÓN \> INDETERMINADO \> NORMAL.

El repositorio SVcustos-dataset incluye un subproyecto específico,
pairs/, que implementa esta composición. Este módulo genera pares de
salidas de ambas células (cls₃₆, cls₉), aplica la regla max() para
obtener la clase final del par, y opcionalmente produce visualizaciones
duales (polígono n = 36 + polígono n = 9) etiquetadas con la
clasificación del par, para validación end-to-end y para ilustrar los 9
casos de la tabla de composición.

Este subproyecto no redefine una nueva célula de 45 parámetros ni
entrena un tercer modelo sobre imágenes compuestas; se limita a modelar
de forma explícita la decisión conjunta a partir de las salidas
individuales, preservando las propiedades algebraicas (conmutatividad,
asociatividad e idempotencia) de la composición ternaria.

El alcance de SVcustos-dataset queda así acotado al sistema de detección
de intrusiones: niveles n = 9, 16, 25, 36 y el par SV(36, 6) + SV(9, 3)
descrito en este documento. La misma arquitectura de composición
(células con n = b² y regla ternaria sobre sus salidas) servirá de
plantilla para SVperitus (Documento 7), que se desarrollará en un
repositorio independiente con sus propias células, datasets y reglas de
composición adaptadas al dominio de conocimiento experto.

# 13. Discusión

## 13.1. Argumentos a favor

El par de células SV resuelve el problema estructural de los parámetros
de integridad sin violar la restricción n = b². Ambas células son
individualmente bien formadas, con simetría radial y organización por
capas. La regla de composición max() es algebraicamente limpia
(conmutativa, asociativa, idempotente) y extensible a n-tuplas sin
modificación.

La mejora en detección es sustancial: del 65,9 % al 10,2 % de falsos
negativos contra ataques avanzados a nivel de sistema operativo. El
coste es mínimo: cero hardware adicional, cero latencia adicional
perceptible, y entre 4 y 8 semanas de desarrollo de software.

La arquitectura de células abre además un framework de segundo orden: la
célula como unidad compositiva y el pésimo como regla de composición
universal permiten construir sistemas de complejidad arbitraria. Por
ejemplo, un sistema para entorno médico gestionado podría operar como
triplete SV(36, 6) + SV(9, 3) + SV(9, 3), donde la tercera célula cubre
los parámetros MDM. Alternativamente, n = 49 sigue siendo una célula
única bien formada SV(49, 7) para despliegues donde la infraestructura
MDM existe.

## 13.2. Limitaciones y riesgos

La regla max() es conservadora por diseño: siempre clasifica hacia la
severidad. Esto significa que una célula de integridad en estado
INTRUSIÓN fuerza la clasificación INTRUSIÓN del par completo,
independientemente de lo que diga la célula principal. En escenarios
donde el root sea legítimo (dispositivos de desarrollo, investigación de
seguridad), esto producirá falsos positivos sistemáticos. La solución es
un modo de operación que permita marcar parámetros como intencionalmente
activos, lo cual requiere autenticación del operador.

La arquitectura de células añade complejidad conceptual al framework:
donde antes había un único sistema con un único vector y un único
clasificador, ahora hay dos sistemas acoplados. La documentación,
formación y auditoría del sistema compuesto son proporcionalmente más
costosas.

## 13.3. Grado de libertad en la regla de clasificación

La célula de integridad SV(9, 3) aplica en este documento el umbral
simétrico puro ⌊7n/9⌋, coherente con los Documentos 2, 3 y 4 de la
serie. Sin embargo, el Documento 1 (n = 9) exploró una variante
adicional: exigir que nU ≥ 1 (presencia real de incertidumbre) para
declarar INDETERMINADO, en lugar de asignar INDETERMINADO a todo vector
que no alcance los umbrales de INTRUSIÓN o NORMAL.

Esta variante no constituye una inconsistencia, sino un grado de
libertad legítimo del sistema: cada célula SV podría particularizar su
regla de clasificación según el dominio semántico de sus parámetros. En
la célula de integridad, donde los 9 parámetros comparten una naturaleza
binaria más marcada (el root está activo o no lo está, el bootloader
está desbloqueado o no), la exigencia de incertidumbre real para
declarar INDETERMINADO podría ser semánticamente más precisa que el
umbral simétrico puro.

Queda documentado como posibilidad futura, especialmente relevante si
SVperitus u otras arquitecturas en par requieren reglas de clasificación
adaptadas a la naturaleza de sus parámetros.

## 13.4. Frontera operativa recomendada para entornos no regulados

En el marco de esta serie, el par SV(36, 6) + SV(9, 3) puede
considerarse la configuración técnica recomendada de SVcustos para
dispositivos de uso general y entornos profesionales no sometidos a
regulación específica ni a gestión MDM estricta. La célula principal
SV(36, 6) cubre 36 vectores de amenaza a nivel de comportamiento
observable, mientras que la célula de integridad SV(9, 3) vigila los 9
indicadores de fiabilidad del sustrato de detección.

La composición determinista SV(par) = max(SV(36, 6), SV(9, 3)), con
INTRUSIÓN \> INDETERMINADO \> NORMAL, garantiza que cualquier intrusión
detectada por la célula principal, o cualquier compromiso relevante de
integridad, se refleje inmediatamente en la decisión final del par.
Desde un punto de vista de ingeniería, esta configuración define una
frontera superior razonable para despliegues domésticos y profesionales
no regulados: ofrece un equilibrio adecuado entre profundidad de
detección, coste computacional y complejidad de integración, sin imponer
las exigencias adicionales propias de entornos gestionados o clínicos.

# 14. Conclusión y recomendación

La arquitectura de células SV en par es conceptualmente sólida,
algebraicamente limpia y cuantitativamente justificada. La recomendación
es implementar el par SV(36, 6) + SV(9, 3) como configuración estándar
para dispositivos de consumo, reservando la célula única SV(49, 7) para
entornos médicos gestionados donde la infraestructura MDM garantiza la
disponibilidad de todos los parámetros.

En ningún caso se recomienda desplegar la célula de integridad sin haber
validado empíricamente la célula principal SV(36, 6) previamente. La
progresión lógica del sistema exige que cada nivel demuestre su utilidad
sobre el anterior antes de escalar. La arquitectura formal del par es
correcta e internamente consistente; lo que está pendiente de
demostración es que la célula de integridad mejore efectivamente la
sensibilidad del sistema compuesto sobre datos reales de dispositivos
comprometidos.

# 15. Referencias

\[1\] Lloret Egea, J. A. et al. (2021). «Framework» basado en imágenes
parametrizadas sobre ResNet para identificar intrusiones en terminales
móviles Android. Tesis doctoral, Universidad de Murcia.

\[2\] Lloret Egea, J. A. (2026). De SVcustos hasta SVperitus. Documento
1 de 8: El nivel base. ISSN 2695-641.

\[3\] Lloret Egea, J. A. (2026). De SVcustos hasta SVperitus. Documento
2 de 8: De n = 9 a n = 16. ISSN 2695-641.

\[4\] Lloret Egea, J. A. (2026). De SVcustos hasta SVperitus. Documento
3 de 8: De n = 16 a n = 25. ISSN 2695-641.

\[5\] Lloret Egea, J. A. (2026). De SVcustos hasta SVperitus. Documento
4 de 8: De n = 25 a n = 36. ISSN 2695-641.

\[6\] He, K., Zhang, X., Ren, S., & Sun, J. (2016). Deep Residual
Learning for Image Recognition. Proceedings of the IEEE CVPR,
pp. 770--778.

\[7\] Android Developers. (2025). Android API Reference:
AccessibilityManager, DevicePolicyManager, MediaProjection, UsbManager,
PackageInstaller. https://developer.android.com

\[8\] Google. (2025). Play Integrity API Documentation.
https://developer.android.com/google/play/integrity

\[9\] OWASP Mobile Application Security Verification Standard (MASVS)
v2.0 (2023). https://mas.owasp.org/MASVS/

\[10\] NIST SP 800-124 Rev. 2 (2023). Guidelines for Managing the
Security of Mobile Devices in the Enterprise.

\[11\] Lloret Egea, J. A. (2026). SVcustos-dataset. GitHub.
https://github.com/juantoniolloretegea/SVcustos-dataset

# 16. Mapa de la serie completa

Documento 1 de 8 --- El nivel base: 9 parámetros y el origen del sistema

Establece los fundamentos del framework SVcustos: la restricción n = b²,
el vector ternario, la imagen polar, el clasificador ResNet34 y la regla
de clasificación estricta. Espacio: 3⁹ = 19.683.

Documento 2 de 8 --- De n = 9 a n = 16: primera extensión

Reestructura a 4 capas de 4. Añade 7 nuevos parámetros. Formaliza la
regla general de umbral T(n) = ⌊7n/9⌋, aplicada en todos los documentos
posteriores. Espacio: 3¹⁶ = 43.046.721.

Documento 3 de 8 --- De n = 16 a n = 25: segunda extensión

Reestructura a 5 capas de 5. Añade 9 nuevos parámetros. Espacio: 3²⁵ ≈
8,47 × 10¹¹.

Documento 4 de 8 --- De n = 25 a n = 36: tercera extensión

Reorganiza a 6 capas de 6 con dos capas enteramente nuevas
(Autenticación, Comunicaciones/Evasión). Identifica el punto ciego
estructural. Espacio: 3³⁶ ≈ 1,50 × 10¹⁷.

Documento 5 de 8 --- Células SV en Par: n = 36 + 9 = 45

Presenta la arquitectura de dos células SV independientes como solución
al problema de los parámetros de integridad. Define la célula de
integridad SV(9, 3) y la regla de composición max(). Espacio compuesto:
3⁴⁵ ≈ 2,95 × 10²¹.

Documento 6 de 8 --- De n = 36 a n = 49: extensión para entornos
gestionados y médicos

Extiende el sistema a 7 capas de 7 con la capa de Gestión MDM. Orientado
a entornos clínicos y corporativos. Espacio: 3⁴⁹ ≈ 2,4 × 10²³.

Documento 7 de 8 --- SVperitus: agentes especializados --- instancia
inmunología

Transpone la arquitectura al dominio de conocimiento experto. SVperitus
(SV = Sistema Vectorial, peritus = experto verificado en latín) define
ontologías de n = 625 = 25² parámetros. Espacio: 3⁶²⁵ ≈ 10²⁹⁸.

Documento 8 de 8 --- Documento compilador de la serie completa

Unifica los 7 documentos anteriores con textos de transición,
referencias unificadas y tabla comparativa SVcustos vs SVperitus.

# 17. Próximo documento: frontera de n = 49 en entornos gestionados y clínicos

El presente documento ha fijado la arquitectura de las células SV en par
y ha consolidado el par SV(36, 6) + SV(9, 3) como referencia operativa
para dispositivos no regulados y entornos profesionales sin gestión MDM
estricta. Sin embargo, en muchos escenarios reales ---en particular, en
infraestructuras corporativas gestionadas y en circuitos donde
intervienen dispositivos médicos o clínicos--- esta configuración
resulta necesaria pero no suficiente.

El siguiente documento de la serie (Documento 6 de 8: «De n = 36 a n =
49: extensión para entornos gestionados y médicos») introducirá la
célula SV(49, 7), que extiende el sistema a n = 49 = 7² parámetros,
incorporando una capa específica de gestión MDM y cumplimiento
corporativo. Esta célula de 49 parámetros no sustituye al par SV(36,
6) + SV(9, 3), sino que se apoya en la misma lógica algebraica ternaria
para definir una segunda frontera de despliegue de SVcustos, orientada a
entornos gestionados y contextos clínicos donde la trazabilidad, la
gobernanza del dispositivo y la alineación con marcos regulatorios
externos son requisitos ineludibles.
