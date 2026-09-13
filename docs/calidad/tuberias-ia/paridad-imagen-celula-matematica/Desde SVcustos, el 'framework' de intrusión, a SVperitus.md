---
article:
  elocation-id: desde-el-framework-de-introsusion-por-mapeo-vectorial-ternario-a-svperitus
author:
- Juan Antonio Lloret Egea
bibliography: /tmp/tmp-21tqJ54va5n1Qo.json
copyright:
  link: "https://creativecommons.org/licenses/by-nc-nd/4.0/"
  text: Creative Commons Attribution-NonCommercial-NoDerivatives 4.0
    International License
  type: CC-BY-NC-ND
csl: /app/dist/server/server/utils/citations/citeStyles/apa-7th-edition.csl
date:
  day: 01
  month: 03
  year: 2026
journal:
  publisher-name: IA eñ ™ - (La Biblia de la IA - The Bible of AI ™ ISSN
    2695-6411)
  title: IA eñ ™
link-citations: true
title: Desde SVcustos, el 'framework' de intrusión, a SVperitus
uri: "https://www.itvia.online/pub/desde-el-framework-de-introsusion-por-mapeo-vectorial-ternario-a-svperitus"
---

------------------------------------------------------------------------

# APARTADO 1: génesis del marco, 9 parámetros {#apartado-1-gnesis-del-marco-9-parmetros}

------------------------------------------------------------------------

#### Esta publicación **parte del fundamento que la sustenta**, y que comenzaba así en su abstracto: {#esta-publicacin-parte-del-fundamento-que-la-sustenta-y-que-comenzaba-as-en-su-abstracto}

<https://doi.org/10.21428/39829d0b.981b7276>

"Se ha definido un *framework*[^1]* *conceptual y algebraicamente,
inexistente hasta ahora en su morfología, y pionero en aplicación en el
campo de la Inteligencia Artificial (IA) de forma conjunta; e
implementado en laboratorio, en sus aspectos más estructurales, como un
modelo completamente operacional. Su mayor aportación a la IA a nivel
cualitativo es aplicar la conversión o transducción de parámetros
obtenidos con lógica ternaria[@eswiki:135730214] (sistemas
multivaluados)[^2] y asociarlos a una imagen, que es analizada mediante
una red residual artificial
ResNet34[@enwiki:1019286662]^,^[@nivqpd17x9a]^\ ^para que nos advierta
de una intrusión. El campo de aplicación de este *framework* va desde
*smartwaches*, *tablets* y PC\'s hasta la domótica basada en el estándar
KNX[@eswiki:134702414]".

------------------------------------------------------------------------

> **SVcustos**: '**SV'** porque es un Sistema Vectorial, '**custos'**
> porque vigila sin intervenir y alerta cuando algo cambia, y latín
> porque esa figura del centinela que observa sin descanso y nunca
> abandona su puesto es exactamente lo que el término ya significaba
> hace dos mil años.

------------------------------------------------------------------------

### **SVcustos: el nivel base: n=9 y el origen del sistema**

El sistema SVcustos (denominación **actual** del primer 'framework' o
marco) nace de una observación técnica concreta: los estados de
seguridad de un dispositivo inteligente pueden describirse como un
vector de eventos discretos, cada uno de los cuales es o bien activo, o
bien inactivo, o bien no determinable. Esa triplicidad ---que el sistema
formaliza como valor 1, valor 0 y valor U respectivamente--- es la
célula mínima de toda la arquitectura posterior.

En su nivel base, el sistema opera con n=9 parámetros organizados en
tres capas de tres: red (tráfico hacia URLs no autorizadas, comunicación
sin cifrar, transferencia exterior), conectividad (BSSID cambiante,
Bluetooth activo, GPS activo) y sistema (parámetros físicos anómalos,
activación de cámara sin permiso, activación de micrófono sin permiso).
Estos nueve parámetros generan un espacio combinacional de 2⁹ = 512
vectores posibles en lectura binaria.

El mecanismo de clasificación transforma el vector en una imagen polar
de nueve ejes equiespaciados a 40° entre sí. Los valores 0, 1 y U se
mapean a radios 1, 2 y 3 respectivamente, produciendo un polígono
cerrado y reproducible que una red neuronal convolucional ResNet34
clasifica en tres estados: intrusión (≥7 de los 9 parámetros activos),
indeterminado (5 o 6 activos) y normal (≤4 activos). Con esos umbrales,
el espacio se distribuye aproximadamente así: 0,8% de los vectores caen
en zona de intrusión, 22,3% en zona indeterminada y 77% en zona normal.

La implementación parcial del nivel base existe: el repositorio incluye
reglas Snort para la captura de parámetros de red, esquema de base de
datos para la persistencia de eventos, y el entrenamiento inicial de
ResNet34. Lo que no existe en el nivel base es el pipeline completo de
extremo a extremo, desde la captura de los nueve parámetros hasta la
generación automatizada de la imagen polar y su clasificación en tiempo
real. Esa brecha de implementación es precisamente el punto de partida
de los documentos de extensión que siguen.\
La restricción algebraica fundamental ---n debe ser un cuadrado perfecto
b², con b≥3--- no es una decisión de diseño arbitraria. Es la condición
que garantiza que el polígono polar tenga simetría radial uniforme y que
los parámetros se organicen en b capas temáticas de b parámetros cada
una, haciendo el sistema auditablemente legible por capas. Esta
restricción es la que define toda la progresión de la serie: n=9 (b=3,
base), n=16 (b=4), n=25 (b=5), n=36 (b=6), n=49 (b=7), y más adelante,
en un dominio completamente distinto, n=625 (b=25).

\[file element\]

[^3]~\ Tabla\ combinacional\ de\ los\ 512\ vectores\ posibles\ usando\ 9\ parámetros~

------------------------------------------------------------------------

\[video element\]

------------------------------------------------------------------------

> **TODO SER HUMANO, SI SE LO PROPONE, PUEDE SER ESCULTOR DE SU PROPIO
> CEREBRO . **[Santiago Ramón y
> Cajal](https://youtu.be/RM1k9rxGkYg?si=K1zPgYfY1AAZAOj9 "null") 

------------------------------------------------------------------------

### **Introducción: un sistema, dos dominios, una arquitectura** {#introduccin-un-sistema-dos-dominios-una-arquitectura}

Este documento compila cinco trabajos (aquí denominados apartados) que
forman una unidad conceptual progresiva. **El primero** describe un
sistema de detección de intrusiones en dispositivos inteligentes. **El
último** describe un sistema para construir agentes de inteligencia
artificial especialistas. Entre medias hay tres extensiones que amplían
el primero sin cambiar su naturaleza. La pregunta que justifica la
compilación es: **¿qué tienen en común un *****smartwatch***** vigilado
por posibles intrusiones y un agente de IA que responde consultas de
inmunología clínica?**

**La respuesta está en la arquitectura**. Ambos sistemas representan el
estado de un objeto ---un dispositivo, un agente--- como un vector de n
parámetros ternarios organizados en b capas de b elementos, con n=b².
Ambos transforman ese vector en una imagen polar de n ejes. Ambos usan
ResNet34 para clasificar esa imagen en tres estados. La restricción n=b²
garantiza en ambos casos la simetría geométrica del polígono y la
legibilidad por capas de su morfología. El mecanismo es invariante; lo
que cambia es el significado de cada parámetro.

**En SVcustos** el parámetro es un evento de seguridad observable: una
URL no autorizada, un proceso desconocido, un certificado TLS inválido.
Su valor 1 significa que ese evento ha ocurrido. La clasificación de la
imagen polar determina si el dispositivo está siendo atacado.

**En SVperitus** el parámetro es un área de competencia del
especialista: el conocimiento de la recombinación V(D)J, la
interpretación de anticuerpos ANCA, la comprensión del mecanismo de
acción del eculizumab. Su valor 1 significa que el corpus de
conocimiento del agente cubre esa área con suficiencia verificada. La
clasificación de la imagen polar determina si el agente está listo para
operar en su dominio.

La compilación se lee en este orden. Los cuatro primeros capítulos
recorren la progresión de SVcustos desde n=9 hasta n=49, extendiendo el
sistema por capas con justificación técnica y análisis adversarial en
cada paso. El quinto capítulo es SVperitus: el documento fundacional del
sistema de vectores de competencia, que toma la arquitectura demostrada
en los cuatro anteriores y la redirige hacia un problema completamente
diferente.

> La transición entre los dos sistemas no es una ruptura; es la
> demostración de que la arquitectura es genuinamente agnóstica al
> dominio de sus parámetros.

El lector especializado en ciberseguridad puede leer los cuatro primeros
capítulos como una obra autónoma. El lector interesado en agentes de IA
puede leer el quinto de la misma manera. El lector que lea los cinco
encontrará en la arquitectura compartida algo más interesante que
cualquiera de los dos sistemas por separado: un lenguaje formal para
representar, clasificar y auditar el estado de sistemas complejos
heterogéneos mediante vectores paramétricos ternarios e imágenes
polares.

**La cita de Cajal no es decorativa**. Cajal demostró que el cerebro
adulto puede reorganizarse si se le da el estímulo adecuado y el tiempo
necesario. SVperitus propone exactamente eso para un agente de IA: una
arquitectura de aprendizaje continuo que actualiza su vector de
competencia a medida que ingiere nueva literatura, y que no se declara
operativo hasta que la evidencia de su cobertura supera el umbral que el
sistema le exige. La escultura del cerebro, en este caso, la realiza el
corpus académico.

------------------------------------------------------------------------

# APARTADO 2: ** **extensión del espacio de parámetros de n=9 a n=16 {#apartado-2-extensin-del-espacio-de-parmetros-de-n9-a-n16}

------------------------------------------------------------------------

### Reestructuración, nuevos parámetros y regla de clasificación {#reestructuracin-nuevos-parmetros-y-regla-de-clasificacin}

### 1.  Introducción y posición en la serie {#introduccin-y-posicin-en-la-serie}

El marco (*framework) *define un sistema de detección de intrusiones que
convierte un vector de parámetros binarios en una imagen polar
clasificada mediante la red neuronal ResNet34. El nivel base del sistema
opera con n=9 parámetros organizados en el espacio combinacional de 2⁹ =
512 vectores posibles. Este documento describe el primer escalón de
extensión formal: el paso de n=9 a n=16, que eleva el espacio a 2¹⁶ =
65.536 vectores.

La restricción algebraica del marco* *establece que n debe ser un
cuadrado perfecto de b, con b≥3, de modo que n = b². Esta condición
garantiza que el polígono polar posea simetría radial uniforme y que los
parámetros se organicen en b capas temáticas de b parámetros cada una.
La progresión formal completa de la serie es: n=9 (b=3, nivel base
implementado), n=16 (b=4, este documento), n=25 (b=5, documento hermano)
y n=36 (b=6, segundo documento hermano).

Este documento es el de entrada a la serie. Su propósito es justificar
la reestructuración de los 9 parámetros originales en cuatro capas y
describir los 7 parámetros nuevos que completan el vector de 16,
incluyendo su justificación técnica, herramienta de captura y análisis
de los argumentos contrarios. Los documentos que le continúan siguen la
misma estructura y dan por conocidos los contenidos de este.

 

### 2.  Reestructuración de los 9 parámetros originales en 4 capas {#reestructuracin-de-los-9-parmetros-originales-en-4-capas}

Antes de introducir los 7 nuevos parámetros, los 9 originales se
redistribuyen en cuatro capas funcionales de cuatro parámetros cada una.
Esta reorganización es condición necesaria para que el salto a n=16
tenga coherencia arquitectónica: cada capa agrupa parámetros del mismo
dominio de observación, lo que facilita tanto la interpretabilidad del
polígono polar resultante como el diseño de las capas adicionales en
extensiones posteriores.

Los 9 parámetros originales no pierden ninguno de sus valores ni se
redefinen. Únicamente se renumeran para encajar en la arquitectura de 4
capas, dejando posiciones libres en cada capa que serán ocupadas por los
parámetros nuevos.

 

+----------------+----------------+----------------+----------------+
| **Capa Red     | **Capa         | **Capa         | **Capa Sistema |
| (P1--P4)**     | Conectividad   | Sensores (P9-- | (P13--P16)**   |
|                | (P5--P8)**     | P12)**         |                |
+================+================+================+================+
| P1 URL no      | P5 BSSID (ex   | P9 Cámara (ex  | P13 Físicos    |
| autorizada P2  | P4)            | P7) P10        | (ex P9)        |
| No cifrada     |                | Micrófono (ex  |                |
|                | P6 Bluetooth   | P8) **P11      | **P14 Permisos |
| P3 Trans.      | (ex P5) P7 GPS | Salud          | **★**nuevo P15 |
| exterior       | (ex P6)        | **★**nuevo P12 | Datos pers.    |
|                |                | Proceso        | **★**nuevo P16 |
| **P4 TLS       | **P8 NFC       | **★**nuevo**   | Celular        |
| inválido**     | **★**nuevo**   |                | **★**nuevo**   |
|                |                |                |                |
| ★**nuevo**     |                |                |                |
+----------------+----------------+----------------+----------------+

 

### 3.  Los 7 parámetros nuevos: P4, P8, P11, P12, P14, P15, P16 {#los-7-parmetros-nuevos-p4-p8-p11-p12-p14-p15-p16}

Para cada nuevo parámetro se describe qué vector de amenaza cubre, cómo
obtener el valor de forma binaria/ternaria, y se expone el argumento
técnico contrario más sólido junto con la réplica correspondiente. Este
formato adversarial es el mismo empleado en los documentos de extensión
n=25 y n=36 de la serie.

###### 3.1   P4 --- Certificado TLS inválido o autofirmado \[Capa Red\] {#p4-certificado-tls-invlido-o-autofirmado-capa-red}

**Cobertura: **cubre el flanco que P2 no cubre. P2 detecta HTTP en
claro; P4 detecta HTTPS con certificado manipulado, el escenario clásico
de ataque de intermediario (MITM) sobre canal cifrado. Sin P4, un
atacante que use un certificado autofirmado evita P2 por completo.

**Obtención: **implementar un TrustManager personalizado que intercepte
la cadena de certificados antes de aceptar la conexión. Si el
certificado no pertenece a una CA de confianza del sistema o a una lista
de certificados fijados (certificate pinning), el valor es 1. En
Android: X509TrustManager con checkServerTrusted(). En Python/Windows:
ssl.SSLContext con verify_mode=ssl.CERT_REQUIRED y comparación contra
almacén local. Binarización: certificado válido → 0, inválido → 1,
conexión no TLS → U.

**Peso intrusivo: **★ ALTO. Un certificado inválido para un dominio
aparentemente autorizado es

indicador casi inequívoco de MITM.

 

  **Parámetro**   **Argumento contrario**                                                                                                                                                   **Réplica técnica**
  --------------- ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  **P4 · TLS**    El certificate pinning es invasivo: rompe actualizaciones legítimas de app cuando el servidor rota su certificado. Las CDNs usan wildcard certs que no son sospechosos.   El parámetro no valida el certificado del servidor en abstracto, sino contra la lista blanca de dominios autorizados del dispositivo. Un certificado válido para un dominio no autorizado sigue disparando P1. P4 solo actúa cuando la URL es aparentemente autorizada pero el certificado no cuadra: exactamente el escenario MITM.

 

 

 

###### 3.2   P8 --- NFC no autorizado \[Capa Conectividad\]

**Cobertura: **comunicación NFC con dispositivo o lector no incluido en
lista blanca. El documento base menciona NFC en la sección de domótica y
en la tabla de hardware de varios modelos (Samsung Gear S2, Moto 360,
Sony SmartWatch 3, Apple Watch Series 6), pero ninguno de los 9
parámetros originales lo capturaba.

**Obtención: **en Android, NfcAdapter.getDefaultAdapter() +
NfcAdapter.setReaderCallback() para monitorizar sesiones NFC activas. Se
registra el identificador del dispositivo remoto y se compara con la
lista blanca. En Python: librería nfcpy. Binarización: sesión NFC con
dispositivo no autorizado → 1, sin sesión o autorizada → 0, hardware sin
NFC → U.

**Peso intrusivo: **★ ALTO.

 

  **Parámetro**   **Argumento contrario**                                                                                                                                                                                 **Réplica técnica**
  --------------- ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  **P8 · NFC**    La lista blanca de lectores NFC es prácticamente imposible de mantener para un usuario doméstico. Cualquier terminal de pago en un comercio nunca estará en esa lista: cada pago legítimo dispara P8.   El parámetro no mide si se realiza un pago sino si se establece una sesión NFC de intercambio de datos fuera de la app de pago autorizada. NfcAdapter.setReaderCallback() distingue sesiones de pago (protocolo ISO- DEP/EMV) de sesiones de lectura de datos (NDEF). Las primeras se excluyen; las segundas son sospechosas.

\

 

###### 3.3   P11 --- Sensores de salud no autorizados \[Capa Sensores\]

**Cobertura: **acceso no autorizado a los sensores de salud del
dispositivo. El framework original almacena frecuencia cardíaca,
saturación de oxígeno, nivel de estrés, sueño y actividad física en la
tabla TSalud de la base de datos, pero ninguno de los 9 parámetros
originales vigilaba si una app accedía a esos datos sin permiso
explícito del usuario.

**Obtención: **en Android, auditando qué aplicaciones registran
listeners para Sensor.TYPE_HEART_RATE, TYPE_STEP_COUNTER o

TYPE_LOW_LATENCY_OFFBODY_DETECT y comparando contra apps con permiso
BODY_SENSORS concedido. En WearOS/Tizen: APIs equivalentes. En watchOS:
HKHealthStore.authorizationStatus(for:). Binarización: app sin permiso
leyendo sensor de salud → 1, todas con permiso → 0, no determinable → U.

**Peso intrusivo: **★ ALTO.

 

  **Parámetro**     **Argumento contrario**                                                                                                                                                                                                                       **Réplica técnica**
  ----------------- --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  **P11 · Salud**   En versiones anteriores a Android 10, el permiso BODY_SENSORS es único para todos los sensores y no distingue por tipo. Muchos smartwatches del documento original corren versiones antiguas de Wear OS donde esta auditoría no es posible.   Argumento válido para dispositivos pre-2019. El framework se define como prospectivo, y los dispositivos actuales (Samsung Galaxy Watch con One UI Watch, Apple Watch Series 6 con watchOS 7+) sí ofrecen esta granularidad. El parámetro aplica a instancias modernas, no a los modelos de 2014.

 

 

 

###### 3.4   P12 --- Proceso desconocido en ejecución \[Capa Sensores\] {#p12-proceso-desconocido-en-ejecucin-capa-sensores}

**Cobertura: **el indicador de intrusión más universal a nivel de
sistema. Cualquier malware, spyware o rootkit que se ejecuta en el
dispositivo aparece en la lista de procesos. Si el proceso no está en la
lista blanca de procesos autorizados, la probabilidad de intrusión es
alta independientemente de si produce señal en algún otro parámetro.

**Obtención: **en Windows/Linux,
psutil.process_iter(\[\'name\',\'pid\',\'exe\'\]) ofrece la lista
completa en tiempo real y comparación contra lista blanca almacenada en
base de datos. En Android, ActivityManager.getRunningAppProcesses() o,
para acceso más profundo, adb shell ps sobre el dispositivo emparejado
en modo desarrollador. Binarización: proceso desconocido activo → 1,
todos reconocidos → 0, lista inaccesible (sin privilegios root) → U.

**Peso intrusivo: **★ ALTO.

 

+-------------------+-----------------------+-----------------------+
| **Parámetro**     | **Argumento           | **Réplica técnica**   |
|                   | contrario**           |                       |
+===================+=======================+=======================+
| **P12 · Proceso** | Los procesos de       | La lista blanca se    |
|                   | sistema del           | construye en la       |
|                   | fabricante (Samsung   | primera ejecución del |
|                   | Health Service, Bixby | framework en modo de  |
|                   | Background Service,   | laboratorio (estado   |
|                   | etc.) son opacos y    | limpio certificado    |
|                   | numerosos. La lista   | del dispositivo). Las |
|                   | blanca inicial sería  | actualizaciones de    |
|                   | imposible de definir  | sistema se gestionan  |
|                   | sin conocer el        | con una versión de    |
|                   | firmware exacto del   | lista blanca          |
|                   | dispositivo.          | vinculada a la        |
|                   |                       | versión de firmware.  |
|                   | Cualquier             | Es un problema de     |
|                   | actualización de      | mantenimiento         |
|                   | sistema la invalida.  | conocido en EDR       |
|                   |                       | (Endpoint Detection   |
|                   |                       | and Response) con     |
|                   |                       | soluciones            |
|                   |                       | establecidas en la    |
|                   |                       | industria.            |
+-------------------+-----------------------+-----------------------+

\

###### 3.5   P14 --- Escalada de permisos de aplicación \[Capa Sistema\] {#p14-escalada-de-permisos-de-aplicacin-capa-sistema}

**Cobertura: **una app instalada con permisos mínimos que, sin acción
del usuario, adquiere permisos adicionales. Vector clásico de spyware
progresivo: la app se instala como inofensiva y escala sus privilegios
en actualizaciones silenciosas.

**Obtención: **en Android, PackageManager.getPackageInfo(packageName,
GET_PERMISSIONS) devuelve los permisos concedidos. Se almacena el estado
en la base de datos en el momento de instalación (baseline) y se compara
en cada ciclo de captura. Un delta positivo en permisos sensibles
(RECORD_AUDIO, CAMERA, ACCESS_FINE_LOCATION, READ_CONTACTS) sin acción
del usuario activa el parámetro. Binarización: delta de permiso sensible
sin interacción de

usuario → 1, sin cambios o cambios autorizados → 0, no determinable → U.

**Peso intrusivo: **★ ALTO.

 

+---------------+-------------------------+-------------------------+
| **Parámetro** | **Argumento contrario** | **Réplica técnica**     |
+===============+=========================+=========================+
| **P14 ·**     | Una escalada de         | La base de datos del    |
|               | permisos que ocurrió    | framework almacena el   |
| **Permisos**  | entre dos capturas y    | historial completo. La  |
|               | luego fue revertida no  | comparación no es       |
|               | se detecta. El          | captura-a-captura sino  |
|               | parámetro mide estado   | captura-contra-baseline |
|               | actual, no historia.    | de instalación, que     |
|               |                         | persiste entre ciclos y |
|               |                         | hace auditable          |
|               |                         | cualquier cambio        |
|               |                         | anterior, aunque el     |
|               |                         | estado actual sea       |
|               |                         | idéntico al original.   |
+---------------+-------------------------+-------------------------+

 

 

 

###### 3.6   P15 --- Acceso no autorizado a datos personales almacenados \[Capa Sistema\]

**Cobertura: **lectura de contactos, calendario, mensajes o historial de
llamadas por apps sin permiso explícito declarado. Cubre el escenario en
que el malware ya está instalado y extrae datos localmente antes de
enviarlos: P1, P2 y P3 solo detectarían la exfiltración en el momento
del envío por red, no la extracción local previa.

**Obtención: **en Android, AppOpsManager.getOpsForPackage() devuelve el
historial de operaciones por app desde Android 11. Comparación contra
permisos declarados: si la app accede a Contacts.CONTENT_URI sin
READ_CONTACTS en PackageInfo.requestedPermissions, se activa.
Binarización limpia: acceso sin permiso → 1, todos los accesos
autorizados → 0, no determinable → U.

**Peso intrusivo: **★ ALTO.

 

  **Parámetro**     **Argumento contrario**                                                                                                                                               **Réplica técnica**
  ----------------- --------------------------------------------------------------------------------------------------------------------------------------------------------------------- -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  **P15 · Datos**   AppOpsManager requiere el permiso GET_APP_OPS_STATS, que es de nivel firma o sistema. Una app de terceros no puede acceder a estos logs sin privilegios de sistema.   El framework se concibe con una app con permisos de gestión del sistema o en el contexto del sistema operativo del dispositivo. En ese contexto los privilegios son alcanzables. La implementación requiere que la app del framework esté firmada con el certificado del fabricante, lo que es una barrera de entrada real pero no un obstáculo conceptual.

###### 3.7   P16 --- Conexión celular no autorizada \[Capa Sistema\] {#p16-conexin-celular-no-autorizada-capa-sistema}

**Cobertura: **los dispositivos con LTE o 3G (Apple Watch Series 6,
Samsung Gear S, entre otros) pueden exfiltrar datos completamente al
margen de los parámetros de red WiFi/Bluetooth. Si la conexión celular
se activa fuera del APN autorizado o con un operador no reconocido, es
un indicador de intrusión que P1--P15 no detectarían.

**Obtención: **en Android, TelephonyManager.getDataState() +
getNetworkOperatorName() + getSimOperatorName(). Comparación del
operador activo contra lista blanca de operadores autorizados. En
iOS/watchOS: CTTelephonyNetworkInfo. Binarización: conexión celular a
operador no autorizado o APN desconocido → 1, sin conexión o autorizada
→ 0, dispositivo sin SIM → U.

**Peso intrusivo: **★ ALTO.

 

  **Parámetro**       **Argumento contrario**                                                                                                                                                                                                                                               **Réplica técnica**
  ------------------- --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  **P16 · Celular**   Solo aplica al subconjunto de smartwatches con conectividad celular independiente. En la Figura 2 del documento original, Samsung Gear 2, LG G Watch, Sony SmartWatch 3 y Asus ZenWatch no tienen SIM. P16 sería U permanente para la mayoría de esos dispositivos.   El documento es explícitamente prospectivo y menciona la democratización del 5G como tendencia. Apple Watch Series 6, presente en la misma figura, ya tiene LTE. El valor U para dispositivos sin SIM es el comportamiento correcto: indeterminado, no falso negativo. La cobertura será universal en el siguiente ciclo tecnológico.

### 4.  Tabla resumen de los 16 parámetros {#tabla-resumen-de-los-16-parmetros}

**  **

+----------+----------+----------+----------+----------+----------+
| **Cód.** | **Par    | **Capa** | **Herra  | **Peso** | **Proce  |
|          | ámetro** |          | mienta** |          | dencia** |
+==========+==========+==========+==========+==========+==========+
| **P1**   | URL no   | Red      | Snort    | ★        | Original |
|          | au       |          |          |          | n=9      |
|          | torizada |          |          | **ALTO** |          |
+----------+----------+----------+----------+----------+----------+
| **P2**   | Comu     | Red      | Snort    | ★        | Original |
|          | nicación |          |          |          | n=9      |
|          | no       |          |          | **ALTO** |          |
|          | cifrada  |          |          |          |          |
+----------+----------+----------+----------+----------+----------+
| **P3**   | Trans    | Red      | Snort    | ★        | Original |
|          | ferencia |          |          |          | n=9      |
|          | al       |          |          | **ALTO** |          |
|          | exterior |          |          |          |          |
+----------+----------+----------+----------+----------+----------+
| **P4**   | Cer      | Red      | Trus     | ★        | Nuevo    |
|          | tificado |          | tManager |          | n=16     |
|          | TLS      |          | / ssl    | **ALTO** |          |
|          | inválido |          |          |          |          |
+----------+----------+----------+----------+----------+----------+
| **P5**   | BSSID no | Cone     | Kismet   | ★        | Original |
|          | au       | ctividad |          |          | n=9      |
|          | torizado |          |          | **ALTO** | (ren     |
|          |          |          |          |          | umerado) |
+----------+----------+----------+----------+----------+----------+
| **P6**   | B        | Cone     | Kismet / | **bajo** | Original |
|          | luetooth | ctividad | PyBluez  |          | n=9      |
|          | no       |          |          |          | (ren     |
|          | au       |          |          |          | umerado) |
|          | torizado |          |          |          |          |
+----------+----------+----------+----------+----------+----------+
| **P7**   | GPS no   | Cone     | Kismet / | ★        | Original |
|          | au       | ctividad | Gn       |          | n=9      |
|          | torizado |          | ssStatus | **ALTO** | (ren     |
|          |          |          |          |          | umerado) |
+----------+----------+----------+----------+----------+----------+
| **P8**   | NFC no   | Cone     | Nf       | ★        | Nuevo    |
|          | au       | ctividad | cAdapter |          | n=16     |
|          | torizado |          | / nfcpy  | **ALTO** |          |
+----------+----------+----------+----------+----------+----------+
| **P9**   | Cámara   | Sensores | cv2 /    | ★        | Original |
|          | no       |          | camera2  |          | n=9      |
|          | au       |          |          | **ALTO** | (ren     |
|          | torizada |          |          |          | umerado) |
+----------+----------+----------+----------+----------+----------+
| **P10**  | M        | Sensores | PyAudio  | ★        | Original |
|          | icrófono |          | /        |          | n=9      |
|          | no       |          | Audi     | **ALTO** | (ren     |
|          | au       |          | oManager |          | umerado) |
|          | torizado |          |          |          |          |
+----------+----------+----------+----------+----------+----------+

+----------+----------+----------+----------+----------+----------+---+
| **Cód.** | **Par    | **Capa** | **Herra  | **Peso** | **Proce  |   |
|          | ámetro** |          | mienta** |          | dencia** |   |
+==========+==========+==========+==========+==========+==========+===+
| **P11**  | Sensores | Sensores | Senso    | ★        | Nuevo    |   |
|          | de salud |          | rManager |          | n=16     |   |
|          | no       |          | /        | **ALTO** |          |   |
|          | aut      |          | H        |          |          |   |
|          | orizados |          | ealthKit |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P12**  | Proceso  | Sensores | psutil / | ★        | Nuevo    |   |
|          | des      |          | Activit  |          | n=16     |   |
|          | conocido |          | yManager | **ALTO** |          |   |
|          | en       |          |          |          |          |   |
|          | e        |          |          |          |          |   |
|          | jecución |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P13**  | Pa       | Sistema  | psutil   | **bajo** | Original |   |
|          | rámetros |          |          |          | n=9      |   |
|          | físicos  |          |          |          | (ren     |   |
|          |          |          |          |          | umerado) |   |
|          | a        |          |          |          |          |   |
|          | normales |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P14**  | Escalada | Sistema  | Packag   | ★        | Nuevo    |   |
|          | de       |          | eManager |          | n=16     |   |
|          | permisos |          | /        | **ALTO** |          |   |
|          |          |          | AppOp    |          |          |   |
|          |          |          | sManager |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P15**  | Acceso a | Sistema  | Content  | ★        | Nuevo    |   |
|          | datos    |          | Resolver |          | n=16     |   |
|          | pe       |          | /        | **ALTO** |          |   |
|          | rsonales |          | AppOp    |          |          |   |
|          | sin      |          | sManager |          |          |   |
|          | permiso  |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P16**  | Conexión | Sistema  | Telephon | ★        | Nuevo    |   |
|          | celular  |          | yManager |          | n=16     |   |
|          | no       |          |          | **ALTO** |          |   |
|          | au       |          | /        |          |          |   |
|          | torizada |          | CTT      |          |          |   |
|          |          |          | elephony |          |          |   |
+----------+----------+----------+----------+----------+----------+---+

** **

**Distribución: **14 parámetros de alto peso intrusivo (★), 2 de bajo
peso (P6 Bluetooth, P13 parámetros físicos). Total original conservados:
9. Nuevos incorporados: 7 (P4, P8, P11, P12, P14, P15, P16).

 

### 5.  Del vector de 16 parámetros al polígono polar {#del-vector-de-16-parmetros-al-polgono-polar}

El vector ternario de 16 componentes se transforma en un polígono en
coordenadas polares. Cada parámetro ocupa un eje de los 16
equiespaciados a 22,5° entre sí. Los valores lógicos se mapean a radios:
0 → radio 1 (apagado), 1 → radio 2 (activo), U → radio 3
(indeterminado). El estado completo del dispositivo en un instante queda
así representado como una figura cerrada y reproducible.

Esta representación tiene dos propiedades relevantes para el sistema. La
primera es que permite el uso directo como entrada de una red neuronal
convolucional (ResNet34), que clasifica la imagen del polígono en lugar
del vector numérico. La segunda es que permite la inspección y auditoría
humana: un operador puede leer la forma del polígono e identificar de un
vistazo qué capa de parámetros presenta activación anómala.

 

 

 

*Figura 1. Representación polar de un vector de instancia del framework*

* *

* *

* *

### 6.  Regla de clasificación propuesta para n=16 {#regla-de-clasificacin-propuesta-para-n16}

La regla de clasificación opera sobre el conteo de parámetros en valor 1
del vector. Los valores 0 y U se consideran no intrusión confirmada a
efectos de conteo: 0 indica estado compatible con normalidad, U indica
información no disponible o no medible. La regla se calibra para
mantener la misma filosofía estricta del nivel base n=9: se declara
intrusión únicamente cuando la proporción de indicadores activos es muy
alta.

En n=9, el umbral de intrusión se fijó en 7 de 9 parámetros activos,
equivalente a un 78% de activación. Al pasar a 16 parámetros con la
misma filosofía proporcional, el umbral se eleva al 81% para evitar un
aumento de falsos positivos conforme crece el vector.

+--------------------+--------------------+--------------------+---+
| **Clasificación**  | **Condición (n**₁  | **Distribución     |   |
|                    | **= parámetros en  | estimada**         |   |
|                    | valor 1)**         |                    |   |
+====================+====================+====================+===+
| **1 ---            | n₁ ≥ 13 (≥ 81% de  | \~9% del espacio   |   |
| INTRUSIÓN**        | parámetros         | combinacional      |   |
|                    | activos)           |                    |   |
+--------------------+--------------------+--------------------+---+
| **U ---            | 9 ≤ n₁ ≤ 12        | \~41% del espacio  |   |
| INDETERMINADO**    | (56%--75% de       | combinacional      |   |
|                    |                    |                    |   |
|                    | parámetros         |                    |   |
|                    | activos)           |                    |   |
+--------------------+--------------------+--------------------+---+
| **0 --- NORMAL**   | n₁ ≤ 8 (≤ 50% de   | \~50% del espacio  |   |
|                    | parámetros         | combinacional      |   |
|                    | activos)           |                    |   |
+--------------------+--------------------+--------------------+---+

 

Esta regla es de diseño, no de ajuste estadístico sobre datos
experimentales. Su justificación es la coherencia con el nivel base n=9
y la garantía de que el umbral de intrusión exige una evidencia
acumulada muy alta antes de emitir una alerta positiva. La calibración
definitiva requiere validación sobre un dataset etiquetado de trazas
reales de dispositivos comprometidos.

 

 

### 7.  Ilustración con tres ejemplos de vectores {#ilustracin-con-tres-ejemplos-de-vectores}

**Ejemplo 1 --- Intrusión: **vector con 14 parámetros en valor 1 y 2 en
0. n₁ = 14 ≥ 13 → clasificación INTRUSIÓN. El polígono polar resultante
ocupa casi todo el anillo externo de radio 2 en los 14 ejes activos.

**Ejemplo 2 --- Normal: **vector con 3 parámetros en 1, 9 en 0 y 4 en U.
n₁ = 3 ≤ 8 → clasificación NORMAL. El polígono es bajo (radios 1 y 3
dominantes), con solo tres picos en radio 2.

**Ejemplo 3 --- Indeterminado: **vector con 10 parámetros en 1, 4 en 0 y
2 en U. n₁ = 10, que queda en el rango 9--12 → clasificación
INDETERMINADO. El polígono presenta varias secciones elevadas pero no
tan dominantes como en el caso de intrusión.

 

\

*Figura 3. Ejemplo de polígono polar: morfología del vector como
lenguaje auditable*

\

* *

### 8.  Apertura a otros dominios de aplicación {#apertura-a-otros-dominios-de-aplicacin}

Aunque el ejemplo de partida proviene de la ciberseguridad en
dispositivos inteligentes, el esquema vector → polígono polar → regla de
clasificación es independiente del dominio de los parámetros. Los 16
parámetros pueden redefinirse según el contexto sin modificar la
arquitectura del clasificador:

•       Conductas robóticas: parámetros = comportamientos observables de
un sistema autónomo.

•       Evaluación de competencias: parámetros = indicadores de progreso
o habilidades observables.

•       Perfiles profesionales: parámetros = actitudes o comportamientos
deseables en un puesto de trabajo.

•       Monitorización de sistemas industriales: parámetros =
indicadores de estado de actuadores y sensores.

 

En cada caso la regla de umbral ---por ejemplo, exigir ≥13 parámetros en
estado activo para considerar un perfil como de alto riesgo--- ofrece un
mecanismo claro, auditable y ajustable de clasificación. La posibilidad
de revisar los umbrales en función de datos empíricos y de inspeccionar
visualmente el polígono resultante son las dos propiedades que
distinguen este enfoque de los clasificadores de caja negra.

 

 

### 9.  Relación con los documentos que continúan la serie {#relacin-con-los-documentos-que-continan-la-serie}

Este documento establece la arquitectura de referencia para los dos
documentos siguientes de la serie. Ambos dan por conocidos los 16
parámetros aquí definidos y los toman como punto de partida, sin repetir
las justificaciones técnicas ya recogidas en las secciones 3 y 4.

 

+--------------------+-----------------------+-----------------------+
| **Documento**      | **Extensión**         | **Contenido           |
|                    |                       | principal**           |
+====================+=======================+=======================+
| **Este documento** | n=9 → n=16 (2¹⁶ =     | Reestructuración en 4 |
|                    | 65.536                | capas. 7 nuevos       |
|                    |                       |                       |
|                    | vectores)             | parámetros (P4, P8,   |
|                    |                       | P11, P12, P14, P15,   |
|                    |                       | P16).                 |
|                    |                       |                       |
|                    |                       | Regla de              |
|                    |                       | clasificación base.   |
|                    |                       | Análisis adversarial  |
|                    |                       | por parámetro.        |
+--------------------+-----------------------+-----------------------+
| **Documento n=25** | n=16 → n=25 (2²⁵ ≈    | 9 nuevos parámetros   |
|                    | 33,5M vectores)       | (P17--P25). Capa      |
|                    |                       | Almacenamiento y Capa |
|                    |                       | Contexto. Umbral      |
|                    |                       | ≥14/21 para intrusión |
|                    |                       | (\~9.5%). Valoración  |
|                    |                       | de implementación.    |
+--------------------+-----------------------+-----------------------+
| **Documento n=36** | n=25 → n=36 (2³⁶ ≈    | 11 nuevos parámetros  |
|                    | 68.700M vectores)     | (P26--P36). Capa      |
|                    |                       |                       |
|                    |                       | Autenticación y Capa  |
|                    |                       | Evasión forense.      |
|                    |                       | Umbral                |
|                    |                       |                       |
|                    |                       | ≥21/31 para intrusión |
|                    |                       | (\~3.6%).             |
|                    |                       | Recomendación de      |
|                    |                       |                       |
|                    |                       | implementación en dos |
|                    |                       | fases.                |
+--------------------+-----------------------+-----------------------+

 

La lectura recomendada de la serie es secuencial: este documento
primero, seguido de la extensión n=25 y posteriormente la extensión
n=36. Cada documento asume que el lector conoce el anterior y se centra
en las novedades de su nivel, evitando repeticiones.

\

 

 

# *Framework* avanzado

#### Extensión del espacio de parámetros {#extensin-del-espacio-de-parmetros}

####   {#r2458601294}

**De n=16 a n=25**: justificación, descripción y valoración

* *

     
  -- --
     

\

 

### 1.  Introducción y contexto {#introduccin-y-contexto}

El framework define un sistema de detección de intrusiones basado en la
conversión de vectores de parámetros binarios a imágenes polares
clasificadas mediante la red neuronal ResNet34. Su restricción
algebraica central establece que el número de parámetros n debe ser un
cuadrado perfecto de b, con b mayor o igual a 3, de forma que n = b².
Esta restricción no es arbitraria: garantiza que el polígono polar
generado posea simetría radial uniforme y que los parámetros puedan
organizarse en b capas temáticas de b parámetros cada una, facilitando
tanto la interpretabilidad del sistema como su escalado sistemático.

La progresión formal queda definida como: n=9 (b=3, nivel base,
implementado), n=16 (b=4, primera extensión propuesta), n=25 (b=5,
segunda extensión, objeto de este documento), n=36 (b=6, tercera
extensión, documentada en el documento hermano). Este documento se ocupa
específicamente del salto de n=16 a n=25, justificando por qué los 9
parámetros añadidos aportan valor genuino y no producen redundancia con
los 16 anteriores.

 

 

### 2.  Resumen del estado en n=16

Antes de introducir los nuevos parámetros, se recoge el estado completo
del sistema en su configuración de 16 parámetros, organizados en 4 capas
de 4. Esta tabla sirve como línea base para evaluar si los nuevos
parámetros de n=25 añaden cobertura genuina o solapan con la existente.

 

+----------+----------+----------+----------+----------+----------+
| **       | **Par    | **Capa** | **Herra  | **Peso** | **Estado |
| Código** | ámetro** |          | mienta** |          | en       |
|          |          |          |          |          | n=16**   |
+==========+==========+==========+==========+==========+==========+
| **P1**   | URL no   | Red      | Snort    | ★        | Original |
|          | au       |          |          |          | n=9      |
|          | torizada |          |          | **ALTO** |          |
+----------+----------+----------+----------+----------+----------+
| **P2**   | Comu     | Red      | Snort    | ★        | Original |
|          | nicación |          |          |          | n=9      |
|          | no       |          |          | **ALTO** |          |
|          | cifrada  |          |          |          |          |
+----------+----------+----------+----------+----------+----------+
| **P3**   | Trans    | Red      | Snort    | ★        | Original |
|          | ferencia |          |          |          | n=9      |
|          | al       |          |          | **ALTO** |          |
|          | exterior |          |          |          |          |
+----------+----------+----------+----------+----------+----------+
| **P4**   | Cer      | Red      | TrustMan | ★        | Nuevo en |
|          | tificado |          | ager/ssl |          | n=16     |
|          | TLS      |          |          | **ALTO** |          |
|          | inválido |          |          |          |          |
+----------+----------+----------+----------+----------+----------+
| **P5**   | BSSID no | Cone     | Kismet   | ★        | Original |
|          | au       | ctividad |          |          | n=9      |
|          | torizado |          |          | **ALTO** |          |
+----------+----------+----------+----------+----------+----------+
| **P6**   | B        | Cone     | Kismet   | **bajo** | Original |
|          | luetooth | ctividad | /PyBluez |          | n=9      |
|          | no       |          |          |          |          |
|          | au       |          |          |          |          |
|          | torizado |          |          |          |          |
+----------+----------+----------+----------+----------+----------+

\

 

+----------+----------+----------+----------+----------+----------+---+
| **       | **Par    | **Capa** | **Herra  | **Peso** | **Estado |   |
| Código** | ámetro** |          | mienta** |          | en       |   |
|          |          |          |          |          | n=16**   |   |
+==========+==========+==========+==========+==========+==========+===+
| **P7**   | GPS no   | Cone     | K        | ★        | Original |   |
|          | au       | ctividad | ismet/Gn |          | n=9      |   |
|          | torizado |          | ssStatus | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P8**   | NFC no   | Cone     | NfcAdapt | ★        | Nuevo en |   |
|          | au       | ctividad | er/nfcpy |          | n=16     |   |
|          | torizado |          |          | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P9**   | Cámara   | Sensores | cv2      | ★        | Original |   |
|          | no       |          | /camera2 |          | n=9      |   |
|          | au       |          |          | **ALTO** |          |   |
|          | torizada |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P10**  | M        | Sensores | PyAu     | ★        | Original |   |
|          | icrófono |          | dio/Audi |          | n=9      |   |
|          | no       |          | oManager | **ALTO** |          |   |
|          | au       |          |          |          |          |   |
|          | torizado |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P11**  | Sensores | Sensores | SensorM  | ★        | Nuevo en |   |
|          | de salud |          | anager/H |          | n=16     |   |
|          | no       |          | ealthKit | **ALTO** |          |   |
|          | aut      |          |          |          |          |   |
|          | orizados |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P12**  | Proceso  | Sensores | psutil   | ★        | Nuevo en |   |
|          | des      |          | /Activit |          | n=16     |   |
|          | conocido |          | yManager | **ALTO** |          |   |
|          | en       |          |          |          |          |   |
|          | e        |          |          |          |          |   |
|          | jecución |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P13**  | Pa       | Sistema  | psutil   | **bajo** | Original |   |
|          | rámetros |          |          |          | n=9      |   |
|          | físicos  |          |          |          |          |   |
|          | a        |          |          |          |          |   |
|          | normales |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P14**  | Escalada | Sistema  | Packa    | ★        | Nuevo en |   |
|          | de       |          | geManage |          | n=16     |   |
|          | permisos |          | r/AppOps | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P15**  | Acceso a | Sistema  | Conten   | ★        | Nuevo en |   |
|          | datos    |          | tResolve |          | n=16     |   |
|          | pe       |          | r/AppOps | **ALTO** |          |   |
|          | rsonales |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P16**  | Conexión | Sistema  | Telephon | ★        | Nuevo en |   |
|          | celular  |          | yManager |          | n=16     |   |
|          | no       |          |          | **ALTO** |          |   |
|          | au       |          |          |          |          |   |
|          | torizada |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+

 

**Distribución en n=16: **14 parámetros de alto peso intrusivo (★), 2 de
bajo peso (P6 Bluetooth, P13 parámetros físicos). Umbral propuesto de
clasificación: ≥10 de 14 parámetros de alto peso activos → Intrusión;
7--9 → Indeterminado; ≤6 → Normal.

 

 

### 3.  Justificación del salto de n=16 a n=25 {#justificacin-del-salto-de-n16-a-n25}

El salto a n=25 (b=5) añade una quinta capa temática de 5 parámetros y
amplía la cuarta capa de Sistema de 4 a 5 parámetros. La pregunta que
debe responderse antes de aceptar la extensión es doble: ¿existen
vectores de comportamiento de un smartwatch que los 16 parámetros
actuales no detectan? Y si es así, ¿son esos vectores relevantes para el
modelo de amenaza que el framework pretende cubrir?

La respuesta es afirmativa en ambos casos. El análisis de las
superficies de ataque no cubiertas por n=16 revela dos dominios
completos sin representación: el almacenamiento local y la modificación
del sistema operativo, y el contexto operativo del dispositivo. Un
proceso malicioso puede acceder al almacenamiento externo, modificar
archivos de sistema, usar la comunicación entre procesos para inyectarse
en otras apps, o activar el modo avión para cortar la telemetría antes
de exfiltrar. Ninguno de estos comportamientos produce señal en ninguno
de los 16 parámetros existentes.

El espacio combinacional sube de 2¹⁶ = 65.536 a 2²⁵ = 33.554.432
vectores. Esto no solo aumenta la granularidad de detección sino que
justifica con mayor solidez el uso de IA frente a métodos clásicos: a
partir de n\>9 cualquier algoritmo de clasificación basado en reglas
empieza a ser subóptimo, y a n=25 el espacio es tan amplio que la red
neuronal es el único clasificador práctico.

\

 

### 4.  Los 9 nuevos parámetros: P17 a P25 {#los-9-nuevos-parmetros-p17-a-p25}

Los 9 nuevos parámetros se distribuyen en dos capas nuevas de la
arquitectura n=25: la Capa de Almacenamiento (P17--P20) y la Capa de
Contexto (P21--P25). Cada parámetro ha sido verificado para confirmar
ausencia de redundancia con los 16 existentes.

 

 

###### 4.1   Capa de Almacenamiento (P17--P20)

Esta capa cubre el acceso no autorizado al almacenamiento del
dispositivo y la modificación de la capa de sistema operativo. Son los
vectores de persistencia del malware más frecuentes en dispositivos
Android y no producen señal en P1--P16 salvo que vayan acompañados de
exfiltración de red.

 

+----------+----------+----------+----------+----------+----------+
| **Códi   | **Nombre | **Capa** | **Her    | **Pes    | **Justi  |
| go**     | co       |          | ramienta | o**      | ficación |
|          | mpleto** |          | de       |          | de no    |
|          |          |          | c        |          | redun    |
|          |          |          | aptura** |          | dancia** |
+==========+==========+==========+==========+==========+==========+
| **P17**  | Acceso   | Al       | Storag   | ★ **ALT  | P3       |
|          | al       | macenami | eManager | O**      | detecta  |
|          | al       | ento     | /        |          | trans    |
|          | macenami |          | Android  |          | ferencia |
|          | ento     |          | Storage  |          | de red.  |
|          | externo  |          | Access   |          | P17      |
|          | no       |          | F        |          | detecta  |
|          | au       |          | ramework |          | l        |
|          | torizado |          |          |          | ectura/e |
|          |          |          |          |          | scritura |
|          |          |          |          |          | local en |
|          |          |          |          |          | tarjeta  |
|          |          |          |          |          | SD o     |
|          |          |          |          |          | almace   |
|          |          |          |          |          | namiento |
|          |          |          |          |          | co       |
|          |          |          |          |          | mpartido |
|          |          |          |          |          | sin      |
|          |          |          |          |          | permiso  |
|          |          |          |          |          | READ_EXT |
|          |          |          |          |          | ERNAL_ST |
|          |          |          |          |          |          |
|          |          |          |          |          | ORAGE    |
|          |          |          |          |          | co       |
|          |          |          |          |          | ncedido. |
|          |          |          |          |          | Es       |
|          |          |          |          |          | cenario: |
|          |          |          |          |          | malware  |
|          |          |          |          |          | que      |
|          |          |          |          |          | acopia   |
|          |          |          |          |          | datos    |
|          |          |          |          |          | lo       |
|          |          |          |          |          | calmente |
|          |          |          |          |          | antes de |
|          |          |          |          |          | ex       |
|          |          |          |          |          | filtrar. |
+----------+----------+----------+----------+----------+----------+
| **P18**  | Modi     | Al       | ino      | ★ **ALT  | Ningún   |
|          | ficación | macenami | tifywait | O**      | p        |
|          | de       | ento     | /        |          | arámetro |
|          | archivos |          | File     |          | de n=16  |
|          | de       |          | Observer |          | cubre la |
|          | sistema  |          | API      |          | in       |
|          |          |          |          |          | tegridad |
|          |          |          |          |          | del      |
|          |          |          |          |          | sistema  |
|          |          |          |          |          | de       |
|          |          |          |          |          | f        |
|          |          |          |          |          | icheros. |
|          |          |          |          |          | P18      |
|          |          |          |          |          | detecta  |
|          |          |          |          |          | e        |
|          |          |          |          |          | scritura |
|          |          |          |          |          | en       |
|          |          |          |          |          |          |
|          |          |          |          |          | /system, |
|          |          |          |          |          | /vendor  |
|          |          |          |          |          | o        |
|          |          |          |          |          | modi     |
|          |          |          |          |          | ficación |
|          |          |          |          |          | de       |
|          |          |          |          |          | binarios |
|          |          |          |          |          | del      |
|          |          |          |          |          | sistema. |
|          |          |          |          |          | I        |
|          |          |          |          |          | ndicador |
|          |          |          |          |          | de       |
|          |          |          |          |          | rootkit  |
|          |          |          |          |          | o        |
|          |          |          |          |          | pers     |
|          |          |          |          |          | istencia |
|          |          |          |          |          | de       |
|          |          |          |          |          | malware. |
+----------+----------+----------+----------+----------+----------+
| **P19**  | Com      | Al       | Binder   | ★ **ALT  | P12      |
|          | unicació | macenami | monitor  | O**      | detecta  |
|          | n entre  | ento     | / strace |          | proceso  |
|          | procesos |          | /        |          | des      |
|          | anómala  |          | Activit  |          | conocido |
|          | (IPC)    |          | yManager |          | activo.  |
|          |          |          |          |          | P19      |
|          |          |          |          |          | detecta  |
|          |          |          |          |          | que ese  |
|          |          |          |          |          | proceso  |
|          |          |          |          |          | se       |
|          |          |          |          |          | inyecta  |
|          |          |          |          |          | en otros |
|          |          |          |          |          | mediante |
|          |          |          |          |          | IPC      |
|          |          |          |          |          | (Binder  |
|          |          |          |          |          | en       |
|          |          |          |          |          | A        |
|          |          |          |          |          | ndroid). |
|          |          |          |          |          | Son      |
|          |          |          |          |          | compleme |
|          |          |          |          |          | ntarios: |
|          |          |          |          |          | P12      |
|          |          |          |          |          | alerta   |
|          |          |          |          |          | de       |
|          |          |          |          |          | pr       |
|          |          |          |          |          | esencia, |
|          |          |          |          |          | P19      |
|          |          |          |          |          | alerta   |
|          |          |          |          |          | de       |
|          |          |          |          |          | compor   |
|          |          |          |          |          | tamiento |
|          |          |          |          |          | o        |
|          |          |          |          |          | fensivo. |
+----------+----------+----------+----------+----------+----------+
| **P20**  | Acceso   | Al       | Cl       | ★ **ALT  | No       |
|          | al       | macenami | ipboardM | O**      | cubierto |
|          | port     | ento     | anager.O |          | en n=16. |
|          | apapeles |          | nPrimary |          | El       |
|          | no       |          | ClipChan |          | port     |
|          | au       |          | ged      |          | apapeles |
|          | torizado |          | Listener |          | es       |
|          |          |          |          |          | vector   |
|          |          |          |          |          | de robo  |
|          |          |          |          |          | de       |
|          |          |          |          |          | con      |
|          |          |          |          |          | traseñas |
|          |          |          |          |          | y        |
|          |          |          |          |          | tokens.  |
|          |          |          |          |          | Una app  |
|          |          |          |          |          | que lee  |
|          |          |          |          |          | el       |
|          |          |          |          |          | port     |
|          |          |          |          |          | apapeles |
|          |          |          |          |          | sin      |
|          |          |          |          |          | int      |
|          |          |          |          |          | eracción |
|          |          |          |          |          | del      |
|          |          |          |          |          | usuario  |
|          |          |          |          |          | (fuera   |
|          |          |          |          |          | de foco) |
|          |          |          |          |          | activa   |
|          |          |          |          |          | P20.     |
|          |          |          |          |          | Di       |
|          |          |          |          |          | sponible |
|          |          |          |          |          | como API |
|          |          |          |          |          | pública  |
|          |          |          |          |          | desde    |
|          |          |          |          |          | Android  |
|          |          |          |          |          | 10.      |
+----------+----------+----------+----------+----------+----------+

\

 

###### 4.2   Capa de Contexto (P21--P25)

Esta capa introduce parámetros de nivel de configuración y
comportamiento contextual del dispositivo. No cubren un tipo de dato
concreto sino el estado operativo del sistema en su conjunto. Incluye
dos parámetros de bajo peso (P22 y P25) cuya activación puede deberse a
uso legítimo frecuente.

 

+----------+----------+----------+----------+----------+----------+
| **       | **Nombre | **Capa** | **Her    | **Peso** | **Justi  |
| Código** | co       |          | ramienta |          | ficación |
|          | mpleto** |          | de       |          | de no    |
|          |          |          | c        |          | redun    |
|          |          |          | aptura** |          | dancia** |
+==========+==========+==========+==========+==========+==========+
| **P21**  | Confi    | Contexto | Con      | ★        | P5       |
|          | guración |          | nectivit |          | detecta  |
|          | de red   |          | yManager | **ALTO** | BSSID    |
|          | mo       |          | /        |          | desc     |
|          | dificada |          | Netw     |          | onocido. |
|          | no       |          | orkPolic |          | P21      |
|          | au       |          | yManager |          | detecta  |
|          | torizada |          |          |          | que el   |
|          |          |          |          |          | propio   |
|          |          |          |          |          | dis      |
|          |          |          |          |          | positivo |
|          |          |          |          |          | modifica |
|          |          |          |          |          | su       |
|          |          |          |          |          | confi    |
|          |          |          |          |          | guración |
|          |          |          |          |          | de red   |
|          |          |          |          |          | (proxy,  |
|          |          |          |          |          | DNS, VPN |
|          |          |          |          |          | sil      |
|          |          |          |          |          | enciosa) |
|          |          |          |          |          | sin      |
|          |          |          |          |          | acción   |
|          |          |          |          |          | del      |
|          |          |          |          |          | usuario. |
|          |          |          |          |          | Vector   |
|          |          |          |          |          | de       |
|          |          |          |          |          | re       |
|          |          |          |          |          | direccio |
|          |          |          |          |          | namiento |
|          |          |          |          |          | de       |
|          |          |          |          |          | tráfico. |
+----------+----------+----------+----------+----------+----------+
| **P22**  | Sensores | Contexto | Senso    | **bajo** | Ac       |
|          | de       |          | rManager |          | tivación |
|          | mo       |          | TY       |          | f        |
|          | vimiento |          | PE_ACCEL |          | recuente |
|          | a        |          | EROMETER |          | en uso   |
|          | ctivados |          | /        |          | d        |
|          | sin      |          | TYPE_G   |          | eportivo |
|          | int      |          | YROSCOPE |          | l        |
|          | eracción |          |          |          | egítimo. |
|          |          |          |          |          | Se       |
|          |          |          |          |          | incluye  |
|          |          |          |          |          | como     |
|          |          |          |          |          | bajo     |
|          |          |          |          |          | peso     |
|          |          |          |          |          | porque   |
|          |          |          |          |          | el       |
|          |          |          |          |          | patrón   |
|          |          |          |          |          | es       |
|          |          |          |          |          | pecífico |
|          |          |          |          |          | de       |
|          |          |          |          |          | ac       |
|          |          |          |          |          | tivación |
|          |          |          |          |          | continua |
|          |          |          |          |          | sin      |
|          |          |          |          |          | a        |
|          |          |          |          |          | ctividad |
|          |          |          |          |          | de       |
|          |          |          |          |          | pantalla |
|          |          |          |          |          | puede    |
|          |          |          |          |          | corre    |
|          |          |          |          |          | lacionar |
|          |          |          |          |          | con      |
|          |          |          |          |          | exfi     |
|          |          |          |          |          | ltración |
|          |          |          |          |          | de       |
|          |          |          |          |          | u        |
|          |          |          |          |          | bicación |
|          |          |          |          |          | inferida |
|          |          |          |          |          | por      |
|          |          |          |          |          | mov      |
|          |          |          |          |          | imiento. |
+----------+----------+----------+----------+----------+----------+
| **P23**  | Modo     | Contexto | Settings | ★        | Táctica  |
|          | avión    |          | .Global. |          | de       |
|          | activado |          | AIRPLANE | **ALTO** | evasión: |
|          | de forma |          | _MODE_ON |          | el       |
|          | i        |          | observer |          | malware  |
|          | rregular |          |          |          | activa   |
|          |          |          |          |          | el modo  |
|          |          |          |          |          | avión    |
|          |          |          |          |          | para     |
|          |          |          |          |          | int      |
|          |          |          |          |          | errumpir |
|          |          |          |          |          | la       |
|          |          |          |          |          | tel      |
|          |          |          |          |          | emetría, |
|          |          |          |          |          | realiza  |
|          |          |          |          |          | ope      |
|          |          |          |          |          | raciones |
|          |          |          |          |          | locales  |
|          |          |          |          |          | y lo     |
|          |          |          |          |          | d        |
|          |          |          |          |          | esactiva |
|          |          |          |          |          | para     |
|          |          |          |          |          | ex       |
|          |          |          |          |          | filtrar. |
|          |          |          |          |          | No       |
|          |          |          |          |          | cubierto |
|          |          |          |          |          | en       |
|          |          |          |          |          | ningún   |
|          |          |          |          |          | p        |
|          |          |          |          |          | arámetro |
|          |          |          |          |          | de n=16. |
+----------+----------+----------+----------+----------+----------+
| **P24**  | Sincro   | Contexto | Content  | ★        | P3       |
|          | nización |          | Resolver |          | detecta  |
|          | externa  |          | sync /   | **ALTO** | trans    |
|          | no       |          | Syn      |          | ferencia |
|          | au       |          | cManager |          | al       |
|          | torizada |          |          |          | exterior |
|          |          |          |          |          | a nivel  |
|          |          |          |          |          | de       |
|          |          |          |          |          | paquetes |
|          |          |          |          |          | de red.  |
|          |          |          |          |          | P24      |
|          |          |          |          |          | detecta  |
|          |          |          |          |          | sincro   |
|          |          |          |          |          | nización |
|          |          |          |          |          | pr       |
|          |          |          |          |          | ogramada |
|          |          |          |          |          | con      |
|          |          |          |          |          | s        |
|          |          |          |          |          | ervicios |
|          |          |          |          |          | externos |
|          |          |          |          |          | (Google  |
|          |          |          |          |          | Drive,   |
|          |          |          |          |          | Dropbox  |
|          |          |          |          |          | u otros) |
|          |          |          |          |          | sin que  |
|          |          |          |          |          | el       |
|          |          |          |          |          | usuario  |
|          |          |          |          |          | la haya  |
|          |          |          |          |          | i        |
|          |          |          |          |          | niciado. |
|          |          |          |          |          |          |
|          |          |          |          |          | Complem  |
|          |          |          |          |          | entario, |
|          |          |          |          |          | no       |
|          |          |          |          |          | red      |
|          |          |          |          |          | undante. |
+----------+----------+----------+----------+----------+----------+
| **P25**  | Push     | Contexto | Firebase | **bajo** | Los push |
|          | noti     |          | FCM      |          | l        |
|          | fication |          | token    |          | egítimos |
|          | anómala  |          | monitor  |          | son      |
|          |          |          | /        |          | fre      |
|          |          |          | Not      |          | cuentes. |
|          |          |          | ificatio |          | Se       |
|          |          |          | nListene |          | c        |
|          |          |          | rService |          | lasifica |
|          |          |          |          |          | como     |
|          |          |          |          |          | bajo     |
|          |          |          |          |          | peso     |
|          |          |          |          |          | porque   |
|          |          |          |          |          | solo es  |
+----------+----------+----------+----------+----------+----------+

\

 

  **Código**   **Nombre**       **Capa**   **Herramienta de captura**   **Peso**   **Justificación de no**    
  ------------ ---------------- ---------- ---------------------------- ---------- ------------------------- ---
               **completo**                                                        **redundancia**           
               desde servidor                                                      indicador relevante       
               no reconocido                                                       cuando el servidor        
                                                                                   emisor no está en         
                                                                                   lista blanca y el         
                                                                                   contenido del             
                                                                                   payload es binario,       
                                                                                   no textual.               

 

### 5.  Regla de clasificación propuesta para n=25 {#regla-de-clasificacin-propuesta-para-n25}

En n=25 existen 21 parámetros de alto peso intrusivo (P1--P5, P7--P12,
P14--P21, P23--P24) y 4 de bajo peso (P6, P13, P22, P25). La regla de
clasificación se calibra para mantener la distribución estadística del
nivel base n=9: aproximadamente 9% de intrusiones, 41% de indeterminados
y 50% de normales sobre el espacio combinacional binario.

 

+----------------------+----------------------+----------------------+
| **Clasificación**    | **Condición sobre    | **Distribución       |
|                      | alto peso activos**  | estimada**           |
+======================+======================+======================+
| **1 --- INTRUSIÓN**  | ≥ 14 de 21           | \~9.5% del espacio   |
|                      | parámetros de alto   | combinacional        |
|                      |                      |                      |
|                      | peso activos         |                      |
+----------------------+----------------------+----------------------+
| **U ---              | 11 -- 13 de 21       | \~40.5% del espacio  |
| INDETERMINADO**      | parámetros de alto   | combinacional        |
|                      | peso activos         |                      |
+----------------------+----------------------+----------------------+
| **0 --- NORMAL**     | ≤ 10 de 21           | \~50% del espacio    |
|                      | parámetros de alto   | combinacional        |
|                      |                      |                      |
|                      | peso activos         |                      |
+----------------------+----------------------+----------------------+

 

### 6.  Valoración: ¿es recomendable la extensión a n=25? {#valoracin-es-recomendable-la-extensin-a-n25}

###### 6.1   Argumentos a favor

Los 9 parámetros nuevos cubren dos dominios de amenaza completamente
ausentes en n=16: la persistencia de malware mediante acceso al
almacenamiento y la evasión operativa mediante manipulación del contexto
del dispositivo. No hay solapamiento semántico con los parámetros
existentes. Cada nuevo parámetro tiene una herramienta de captura
identificada y disponible en las plataformas objetivo.

El espacio de 33.5 millones de vectores hace ineludible el uso de IA
como clasificador: ningún árbol de decisión o sistema de reglas clásico
es competitivo a esta escala. El framework obtiene así una justificación
técnica más sólida para el uso de ResNet34 que en n=9.

La organización en 5 capas temáticas de 5 parámetros mantiene la
interpretabilidad del sistema: un operador puede leer el polígono polar
e identificar de un vistazo qué capa presenta anomalías.

 

###### 6.2   Limitaciones y riesgos

P22 (sensores de movimiento) y P25 (push notifications) son de bajo peso
por razón bien fundada: se activan frecuentemente en uso legítimo. Su
inclusión añade ruido que el clasificador deberá aprender a ignorar, lo
que requiere un dataset de entrenamiento más amplio y equilibrado del
que existe actualmente.

\

 

P19 (IPC anómala) y P20 (portapapeles) requieren privilegios de nivel
sistema para ser monitorizados de forma fiable en Android. La app del
framework necesitaría estar firmada con el certificado del fabricante
del dispositivo, lo que limita su despliegue a entornos gestionados.

Ninguno de los 9 parámetros nuevos ha sido capturado en un dataset real
de intrusiones en smartwatches. La regla de clasificación y los umbrales
propuestos son estimaciones analíticas, no resultados empíricos. La
validación experimental es condición necesaria antes de cualquier uso
operativo.

 

###### 6.3   Conclusión {#conclusin}

La extensión a n=25 es conceptualmente recomendable: los nuevos
parámetros aportan cobertura genuina, la arquitectura en capas se
mantiene coherente y el espacio combinacional justifica el uso de IA con
mayor solidez que n=16. Sin embargo, la recomendación de implementación
está condicionada a: disponibilidad de un dataset etiquetado real,
resolución del requisito de privilegios de sistema para P19 y P20, y
validación de los umbrales de clasificación sobre trazas de
comportamiento real.

\

 

 

# *Framework* avanzado {#framework-avanzado}

#### Extensión del espacio de parámetros {#extensin-del-espacio-de-parmetros}

**De n=25 a n=36**: justificación, descripción y valoración

* *

     
  -- --
     

\

* *

### 1.  Introducción y contexto {#introduccin-y-contexto}

Este documento es el segundo de una serie de dos trabajos alineados que
documentan la progresión formal del framework a lo largo de la secuencia
n=b², con b≥3. El documento anterior cubrió el salto de n=16 a n=25.
Este documento cubre el siguiente escalón: el paso de n=25 a n=36 (b=6).

La restricción n=b² no es una convención estética. Determina que el
polígono polar tenga simetría radial uniforme y que los parámetros
puedan organizarse en b capas temáticas de b parámetros cada una. En
n=36, esto produce 6 capas de 6 parámetros, lo que coincide con la
arquitectura de la red ResNet34: 6 es también el número de bloques
residuales en su configuración base, aunque esta coincidencia no implica
relación causal.

La pregunta central que debe responderse antes de aceptar la extensión
de n=25 a n=36 es la misma que en el documento anterior: ¿los 11
parámetros nuevos cubren vectores de amenaza genuinamente ausentes en
los 25 existentes, o simplemente engordan el sistema sin aportar
cobertura adicional?

 

 

### 2.  Resumen del estado en n=25

El sistema en n=25 queda organizado en 5 capas de 5 parámetros: Red
(P1--P5), Conectividad (P6--P10 en terminología de 5 capas, reajustada
de la distribución 4+4+4+4 original), Sensores (P9--P12), Almacenamiento
(P17--P20) y Contexto (P21--P25). Cuenta con 21 parámetros de alto peso
intrusivo y 4 de bajo peso.

 

+----------+----------+----------+----------+----------+----------+
| **       | **Par    | **Capa** | **Herra  | **Peso** | **       |
| Código** | ámetro** |          | mienta** |          | Estado** |
+==========+==========+==========+==========+==========+==========+
| **P1**   | URL no   | Red      | Snort    | ★        | Original |
|          | au       |          |          |          | n=9      |
|          | torizada |          |          | **ALTO** |          |
+----------+----------+----------+----------+----------+----------+
| **P2**   | Comu     | Red      | Snort    | ★        | Original |
|          | nicación |          |          |          | n=9      |
|          | no       |          |          | **ALTO** |          |
|          | cifrada  |          |          |          |          |
+----------+----------+----------+----------+----------+----------+
| **P3**   | Trans    | Red      | Snort    | ★        | Original |
|          | ferencia |          |          |          | n=9      |
|          | al       |          |          | **ALTO** |          |
|          | exterior |          |          |          |          |
+----------+----------+----------+----------+----------+----------+
| **P4**   | Cer      | Red      | TrustMan | ★        | Nuevo en |
|          | tificado |          | ager/ssl |          | n=16     |
|          | TLS      |          |          | **ALTO** |          |
|          | inválido |          |          |          |          |
+----------+----------+----------+----------+----------+----------+

\

 

+----------+----------+----------+----------+----------+----------+---+
| **       | **Par    | **Capa** | **Herra  | **Peso** | **       |   |
| Código** | ámetro** |          | mienta** |          | Estado** |   |
+==========+==========+==========+==========+==========+==========+===+
| **P5**   | BSSID no | Cone     | Kismet   | ★        | Original |   |
|          | au       | ctividad |          |          | n=9      |   |
|          | torizado |          |          | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P6**   | B        | Cone     | Kismet   | **bajo** | Original |   |
|          | luetooth | ctividad | /PyBluez |          | n=9      |   |
|          | no       |          |          |          |          |   |
|          | au       |          |          |          |          |   |
|          | torizado |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P7**   | GPS no   | Cone     | K        | ★        | Original |   |
|          | au       | ctividad | ismet/Gn |          | n=9      |   |
|          | torizado |          | ssStatus | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P8**   | NFC no   | Cone     | NfcAdapt | ★        | Nuevo en |   |
|          | au       | ctividad | er/nfcpy |          | n=16     |   |
|          | torizado |          |          | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P9**   | Cámara   | Sensores | cv2      | ★        | Original |   |
|          | no       |          | /camera2 |          | n=9      |   |
|          | au       |          |          | **ALTO** |          |   |
|          | torizada |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P10**  | M        | Sensores | PyAu     | ★        | Original |   |
|          | icrófono |          | dio/Audi |          | n=9      |   |
|          | no       |          | oManager | **ALTO** |          |   |
|          | au       |          |          |          |          |   |
|          | torizado |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P11**  | Sensores | Sensores | SensorM  | ★        | Nuevo en |   |
|          | de salud |          | anager/H |          | n=16     |   |
|          |          |          | ealthKit | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P12**  | Proceso  | Sensores | psutil   | ★        | Nuevo en |   |
|          | des      |          | /Activit |          | n=16     |   |
|          | conocido |          | yManager | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P13**  | Pa       | Sistema  | psutil   | **bajo** | Original |   |
|          | rámetros |          |          |          | n=9      |   |
|          | físicos  |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P14**  | Escalada | Sistema  | Packa    | ★        | Nuevo en |   |
|          | de       |          | geManage |          | n=16     |   |
|          | permisos |          | r/AppOps | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P15**  | Acceso   | Sistema  | Conten   | ★        | Nuevo en |   |
|          | datos    |          | tResolve |          | n=16     |   |
|          | pe       |          | r/AppOps | **ALTO** |          |   |
|          | rsonales |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P16**  | Conexión | Sistema  | Telephon | ★        | Nuevo en |   |
|          | celular  |          | yManager |          | n=16     |   |
|          |          |          |          | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P17**  | Almace   | Almace   | Storag   | ★        | Nuevo en |   |
|          | namiento | namiento | eManager |          | n=25     |   |
|          | externo  |          | / SAF    | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P18**  | Modi     | Almace   | ino      | ★        | Nuevo en |   |
|          | ficación | namiento | tifywait |          | n=25     |   |
|          | de       |          | /        | **ALTO** |          |   |
|          | sistema  |          | File     |          |          |   |
|          |          |          | Observer |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P19**  | IPC      | Almace   | Binder   | ★        | Nuevo en |   |
|          | anómala  | namiento | monitor  |          | n=25     |   |
|          |          |          | / strace | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P20**  | Port     | Almace   | Clipboar | ★        | Nuevo en |   |
|          | apapeles | namiento | dManager |          | n=25     |   |
|          | no       |          |          | **ALTO** |          |   |
|          | au       |          |          |          |          |   |
|          | torizado |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P21**  | Config.  | Contexto | Con      | ★        | Nuevo en |   |
|          | red      |          | nectivit |          | n=25     |   |
|          | mo       |          | yManager | **ALTO** |          |   |
|          | dificada |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P22**  | Sensores | Contexto | Senso    | **bajo** | Nuevo en |   |
|          | de       |          | rManager |          | n=25     |   |
|          | mo       |          | TYPE_ACC |          |          |   |
|          | vimiento |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P23**  | Modo     | Contexto | Setting  | ★        | Nuevo en |   |
|          | avión    |          | s.Global |          | n=25     |   |
|          | i        |          | observer | **ALTO** |          |   |
|          | rregular |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P24**  | Sincro   | Contexto | Content  | ★        | Nuevo en |   |
|          | nización |          | Resolver |          | n=25     |   |
|          | externa  |          | / Sync   | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P25**  | Push     | Contexto | FCM /    | **bajo** | Nuevo en |   |
|          | noti     |          | Noti     |          | n=25     |   |
|          | fication |          | fication |          |          |   |
|          | anómala  |          | Listener |          |          |   |
+----------+----------+----------+----------+----------+----------+---+

 

**Umbral de clasificación en n=25: **≥14/21 parámetros de alto peso →
Intrusión (\~9.5%); 11--13 → Indeterminado (\~40.5%); ≤10 → Normal
(\~50%).

\

 

### 3.  Justificación del salto de n=25 a n=36 {#justificacin-del-salto-de-n25-a-n36}

El análisis adversarial de los 25 parámetros existentes revela dos
dominios de amenaza sin cobertura. El primero es el de autenticación e
identidad: ningún parámetro de n=25 detecta ataques sobre el mecanismo
de autenticación del dispositivo, ya sea por fuerza bruta biométrica,
manipulación de credenciales o acceso al almacén de claves del sistema.
El segundo es el de comunicaciones activas y evasión forense: el sistema
actual detecta tráfico pasivo (escuchas, transferencias) pero no
comunicaciones iniciadas activamente por el malware ---SMS, llamadas---
ni intentos de borrar el rastro de su actividad mediante eliminación de
logs.

Estos dos dominios son cualitativamente distintos de los cubiertos en
n=25 y corresponden a fases del ciclo de ataque no representadas: la
fase de establecimiento de persistencia mediante control de identidad y
la fase de post-explotación mediante evasión forense. Un sistema que
detecta exfiltración (P3, P17) pero no el borrado de las evidencias de
esa exfiltración (P34) tiene una cobertura incompleta del ciclo de
amenaza.

El espacio combinacional asciende de 2²⁵ (33.5 millones) a 2³⁶ (68.700
millones) de vectores. A esta escala, la justificación del uso de IA no
es ya solo de eficiencia sino de viabilidad: no existe alternativa
computacional práctica al clasificador neuronal para inspeccionar este
espacio.

 

 

### 4.  Los 11 nuevos parámetros: P26 a P36 {#los-11-nuevos-parmetros-p26-a-p36}

Los 11 nuevos parámetros se distribuyen en dos capas nuevas: Capa de
Autenticación (P26--P31) y Capa de Comunicaciones activas y Evasión
(P32--P36, con P36 como sexto parámetro de la capa Contexto ampliada).
La sexta capa completa de 6 parámetros incluye P26--P31. Los parámetros
P32--P36 amplían la capa de Contexto/Evasión a su cuota de 6.

 

 

###### 4.1   Capa de Autenticación (P26--P31) {#capa-de-autenticacin-p26p31}

Esta capa cubre el vector de identidad y control de acceso del
dispositivo. Es el dominio más directamente relacionado con el robo de
credenciales y el takeover de cuenta, que constituyen el objetivo final
de gran parte del malware de vigilancia en dispositivos móviles.

 

+----------+----------+----------+----------+----------+----------+
| **       | **Nombre | **Capa** | **Her    | **Peso** | **Justi  |
| Código** | co       |          | ramienta |          | ficación |
|          | mpleto** |          | de       |          | de no    |
|          |          |          | c        |          | redun    |
|          |          |          | aptura** |          | dancia** |
+==========+==========+==========+==========+==========+==========+
| **P26**  | Cambio   | Auten    | Accoun   | ★        | P14      |
|          | de       | ticación | tManager |          | detecta  |
|          | cred     |          | /        | **ALTO** | escalada |
|          | enciales |          | Auth     |          | de       |
|          | no       |          | enticato |          | permisos |
|          | au       |          | rService |          | en la    |
|          | torizado |          |          |          | capa del |
|          |          |          |          |          | sistema  |
|          |          |          |          |          | op       |
|          |          |          |          |          | erativo. |
|          |          |          |          |          | P26      |
|          |          |          |          |          | detecta  |
|          |          |          |          |          | modi     |
|          |          |          |          |          | ficación |
|          |          |          |          |          | directa  |
|          |          |          |          |          | de       |
|          |          |          |          |          | cred     |
|          |          |          |          |          | enciales |
|          |          |          |          |          | de       |
|          |          |          |          |          | cuenta   |
|          |          |          |          |          | (cont    |
|          |          |          |          |          | raseñas, |
|          |          |          |          |          | PINs,    |
|          |          |          |          |          | p        |
|          |          |          |          |          | atrones) |
|          |          |          |          |          | sin      |
|          |          |          |          |          | int      |
|          |          |          |          |          | eracción |
|          |          |          |          |          | del      |
|          |          |          |          |          | usuario. |
|          |          |          |          |          | Son      |
|          |          |          |          |          | compleme |
|          |          |          |          |          | ntarios: |
|          |          |          |          |          | P14 es   |
|          |          |          |          |          | c        |
|          |          |          |          |          | ondición |
|          |          |          |          |          | n        |
|          |          |          |          |          | ecesaria |
|          |          |          |          |          | pero no  |
|          |          |          |          |          | su       |
|          |          |          |          |          | ficiente |
|          |          |          |          |          | para     |
|          |          |          |          |          | P26.     |
+----------+----------+----------+----------+----------+----------+
| **P27**  | Auten    | Auten    | Biometri | ★        | No       |
|          | ticación | ticación | cManager |          | cubierto |
|          | bi       |          | /        | **ALTO** | en n=25. |
|          | ométrica |          | Fi       |          | N        |
|          | fallida  |          | ngerprin |          | intentos |
|          | de forma |          | tManager |          | de       |
|          | r        |          |          |          | auten    |
|          | eiterada |          |          |          | ticación |
|          |          |          |          |          | bi       |
|          |          |          |          |          | ométrica |
|          |          |          |          |          | fallidos |
|          |          |          |          |          | en       |
|          |          |          |          |          | ventana  |
|          |          |          |          |          | temporal |
|          |          |          |          |          | son      |
|          |          |          |          |          | i        |
|          |          |          |          |          | ndicador |
|          |          |          |          |          | clásico  |
|          |          |          |          |          | de       |
|          |          |          |          |          | ataque   |
|          |          |          |          |          | de       |
|          |          |          |          |          | fuerza   |
|          |          |          |          |          | bruta    |
|          |          |          |          |          | física.  |
|          |          |          |          |          | El       |
|          |          |          |          |          | umbral   |
|          |          |          |          |          | de       |
|          |          |          |          |          | ac       |
|          |          |          |          |          | tivación |
|          |          |          |          |          | debe     |
|          |          |          |          |          | ca       |
|          |          |          |          |          | librarse |
|          |          |          |          |          | sobre    |
|          |          |          |          |          | datos    |
|          |          |          |          |          | reales   |
|          |          |          |          |          | de       |
|          |          |          |          |          | usuario. |
+----------+----------+----------+----------+----------+----------+
| **P28**  | Acceso   | Auten    | Android  | ★        | P15      |
|          | al       | ticación | Keystore |          | detecta  |
|          | keystore |          | System / | **ALTO** | acceso a |
|          | del      |          | KeyChain |          | datos    |
|          | sistema  |          | API      |          | pe       |
|          |          |          |          |          | rsonales |
|          |          |          |          |          | en       |
|          |          |          |          |          | ContentP |
|          |          |          |          |          | rovider. |
|          |          |          |          |          | P28      |
|          |          |          |          |          | detecta  |
|          |          |          |          |          | acceso   |
|          |          |          |          |          | al       |
|          |          |          |          |          | almacén  |
|          |          |          |          |          | de       |
|          |          |          |          |          | claves   |
|          |          |          |          |          | cripto   |
|          |          |          |          |          | gráficas |
|          |          |          |          |          | del      |
|          |          |          |          |          | sistema  |
+----------+----------+----------+----------+----------+----------+

\

 

+----------+----------+----------+----------+----------+----------+---+
| **       | **Nombre | **Capa** | **Her    | **Peso** | **Justi  |   |
| Código** | co       |          | ramienta |          | ficación |   |
|          | mpleto** |          | de       |          | de no    |   |
|          |          |          | c        |          | redun    |   |
|          |          |          | aptura** |          | dancia** |   |
+==========+==========+==========+==========+==========+==========+===+
|          |          |          |          |          | op       |   |
|          |          |          |          |          | erativo, |   |
|          |          |          |          |          | que      |   |
|          |          |          |          |          | contiene |   |
|          |          |          |          |          | certi    |   |
|          |          |          |          |          | ficados, |   |
|          |          |          |          |          | claves   |   |
|          |          |          |          |          | privadas |   |
|          |          |          |          |          | y tokens |   |
|          |          |          |          |          | de       |   |
|          |          |          |          |          | autent   |   |
|          |          |          |          |          | icación. |   |
|          |          |          |          |          | Dominio  |   |
|          |          |          |          |          | distinto |   |
|          |          |          |          |          | e        |   |
|          |          |          |          |          | indepe   |   |
|          |          |          |          |          | ndiente. |   |
+----------+----------+----------+----------+----------+----------+---+
| **P29**  | Token de | Auten    | OAuth    | ★        | P4       |   |
|          | sesión   | ticación | token    |          | detecta  |   |
|          | ma       |          | i        | **ALTO** | cer      |   |
|          | nipulado |          | nspector |          | tificado |   |
|          | o        |          | / JWT    |          | TLS      |   |
|          | inte     |          | decode   |          | inválido |   |
|          | rceptado |          | monitor  |          | a nivel  |   |
|          |          |          |          |          | de       |   |
|          |          |          |          |          | tra      |   |
|          |          |          |          |          | nsporte. |   |
|          |          |          |          |          | P29      |   |
|          |          |          |          |          | detecta  |   |
|          |          |          |          |          | mani     |   |
|          |          |          |          |          | pulación |   |
|          |          |          |          |          | del      |   |
|          |          |          |          |          | token de |   |
|          |          |          |          |          | auto     |   |
|          |          |          |          |          | rización |   |
|          |          |          |          |          | a nivel  |   |
|          |          |          |          |          | de       |   |
|          |          |          |          |          | apl      |   |
|          |          |          |          |          | icación. |   |
|          |          |          |          |          | Un       |   |
|          |          |          |          |          | atacante |   |
|          |          |          |          |          | puede    |   |
|          |          |          |          |          | tener un |   |
|          |          |          |          |          | TLS      |   |
|          |          |          |          |          | válido y |   |
|          |          |          |          |          | aun así  |   |
|          |          |          |          |          | p        |   |
|          |          |          |          |          | resentar |   |
|          |          |          |          |          | un token |   |
|          |          |          |          |          | robado o |   |
|          |          |          |          |          | forjado. |   |
+----------+----------+----------+----------+----------+----------+---+
| **P30**  | SMS o    | Auten    | Sm       | ★        | P3       |   |
|          | MMS      | ticación | sManager |          | detecta  |   |
|          |          |          | send     | **ALTO** | trans    |   |
|          | enviado  |          | listener |          | ferencia |   |
|          | sin      |          | /        |          | de datos |   |
|          | int      |          | SEND_SMS |          | por red  |   |
|          | eracción |          |          |          | IP. P30  |   |
|          | del      |          | pe       |          | detecta  |   |
|          | usuario  |          | rmission |          | exfi     |   |
|          |          |          | monitor  |          | ltración |   |
|          |          |          |          |          | por      |   |
|          |          |          |          |          | canal    |   |
|          |          |          |          |          | SMS, que |   |
|          |          |          |          |          | usa la   |   |
|          |          |          |          |          | red      |   |
|          |          |          |          |          | celular  |   |
|          |          |          |          |          | de       |   |
|          |          |          |          |          | v        |   |
|          |          |          |          |          | oz/datos |   |
|          |          |          |          |          | separada |   |
|          |          |          |          |          | de la    |   |
|          |          |          |          |          | red IP.  |   |
|          |          |          |          |          | Clásico  |   |
|          |          |          |          |          | de       |   |
|          |          |          |          |          | malware  |   |
|          |          |          |          |          | que      |   |
|          |          |          |          |          | exfiltra |   |
|          |          |          |          |          | por      |   |
|          |          |          |          |          | canal    |   |
|          |          |          |          |          | alt      |   |
|          |          |          |          |          | ernativo |   |
|          |          |          |          |          | cuando   |   |
|          |          |          |          |          | el       |   |
|          |          |          |          |          | tráfico  |   |
|          |          |          |          |          | IP está  |   |
|          |          |          |          |          | monit    |   |
|          |          |          |          |          | orizado. |   |
+----------+----------+----------+----------+----------+----------+---+
| **P31**  | Llamada  | Auten    | Teleco   | ★        | No       |   |
|          | te       | ticación | mManager |          | cubierto |   |
|          | lefónica |          | /        | **ALTO** | en n=25. |   |
|          | iniciada |          | CA       |          | I        |   |
|          | sin      |          | LL_PHONE |          | ndicador |   |
|          | int      |          |          |          | de       |   |
|          | eracción |          | pe       |          | spyware  |   |
|          | del      |          | rmission |          | que usa  |   |
|          | usuario  |          | monitor  |          | el       |   |
|          |          |          |          |          | sm       |   |
|          |          |          |          |          | artwatch |   |
|          |          |          |          |          | como     |   |
|          |          |          |          |          | dis      |   |
|          |          |          |          |          | positivo |   |
|          |          |          |          |          | de       |   |
|          |          |          |          |          | escucha  |   |
|          |          |          |          |          | remota   |   |
|          |          |          |          |          | a        |   |
|          |          |          |          |          | ctivando |   |
|          |          |          |          |          | llamadas |   |
|          |          |          |          |          | sile     |   |
|          |          |          |          |          | nciosas. |   |
|          |          |          |          |          | El canal |   |
|          |          |          |          |          | de voz   |   |
|          |          |          |          |          | no       |   |
|          |          |          |          |          | produce  |   |
|          |          |          |          |          | señal en |   |
|          |          |          |          |          | ningún   |   |
|          |          |          |          |          | p        |   |
|          |          |          |          |          | arámetro |   |
|          |          |          |          |          | de red   |   |
|          |          |          |          |          | de los   |   |
|          |          |          |          |          | exi      |   |
|          |          |          |          |          | stentes. |   |
+----------+----------+----------+----------+----------+----------+---+

 

 

###### 4.2   Capa de Comunicaciones activas y Evasión forense (P32--P36) {#capa-de-comunicaciones-activas-y-evasin-forense-p32p36}

Esta capa cierra el ciclo de amenaza cubriendo la fase de
post-explotación: las acciones que el malware realiza para ocultar su
presencia y para usar canales de comunicación alternativos que eviten
los monitores de red existentes.

 

+----------+----------+----------+----------+----------+----------+
| **       | **Nombre | **Capa** | **Her    | **Peso** | **Justi  |
| Código** | co       |          | ramienta |          | ficación |
|          | mpleto** |          | de       |          | de no    |
|          |          |          | c        |          | redun    |
|          |          |          | aptura** |          | dancia** |
+==========+==========+==========+==========+==========+==========+
| **P32**  | Acceso a | Com.     | Paymen   | ★        | P8       |
|          | API de   | /Evasión | tManager |          | detecta  |
|          | pago no  |          |          | **ALTO** | NFC      |
|          | au       |          | / NFC    |          | activo.  |
|          | torizada |          | HCE      |          | P32      |
|          |          |          |          |          | detecta  |
|          |          |          | monitor  |          | específ  |
|          |          |          |          |          | icamente |
|          |          |          |          |          | el uso   |
|          |          |          |          |          | del      |
|          |          |          |          |          | stack    |
|          |          |          |          |          | NFC en   |
|          |          |          |          |          | modo     |
|          |          |          |          |          | Host     |
|          |          |          |          |          | Card     |
|          |          |          |          |          | E        |
|          |          |          |          |          | mulation |
|          |          |          |          |          | para     |
|          |          |          |          |          | trans    |
|          |          |          |          |          | acciones |
|          |          |          |          |          | de pago  |
|          |          |          |          |          | no       |
|          |          |          |          |          | i        |
|          |          |          |          |          | niciadas |
|          |          |          |          |          | por el   |
|          |          |          |          |          | usuario. |
|          |          |          |          |          | Son      |
|          |          |          |          |          | semánt   |
|          |          |          |          |          | icamente |
|          |          |          |          |          | di       |
|          |          |          |          |          | stintos: |
|          |          |          |          |          | P8 es    |
|          |          |          |          |          | pr       |
|          |          |          |          |          | esencia, |
|          |          |          |          |          | P32 es   |
|          |          |          |          |          | tra      |
|          |          |          |          |          | nsacción |
|          |          |          |          |          | fin      |
|          |          |          |          |          | anciera. |
+----------+----------+----------+----------+----------+----------+
| **P33**  | C        | Com.     | DNS      | ★        | P1       |
|          | onsultas | /Evasión | query    |          | detecta  |
|          | DNS a    |          | sniffer  | **ALTO** | URL no   |
|          | dominios |          |          |          | au       |
|          | no       |          | / dnstap |          | torizada |
|          | categ    |          |          |          | a nivel  |
|          | orizados |          |          |          | HTT      |
|          |          |          |          |          | P/HTTPS. |
|          |          |          |          |          | P33      |
|          |          |          |          |          | detecta  |
|          |          |          |          |          | la       |
|          |          |          |          |          | consulta |
|          |          |          |          |          | DNS      |
|          |          |          |          |          | previa a |
|          |          |          |          |          | c        |
|          |          |          |          |          | ualquier |
|          |          |          |          |          | c        |
|          |          |          |          |          | onexión, |
|          |          |          |          |          | que      |
|          |          |          |          |          | puede    |
|          |          |          |          |          | revelar  |
|          |          |          |          |          | comu     |
|          |          |          |          |          | nicación |
|          |          |          |          |          | con      |
|          |          |          |          |          | infraes  |
|          |          |          |          |          | tructura |
|          |          |          |          |          | C2       |
|          |          |          |          |          | incluso  |
|          |          |          |          |          | si la    |
|          |          |          |          |          | URL      |
|          |          |          |          |          | p        |
|          |          |          |          |          | osterior |
|          |          |          |          |          | no       |
|          |          |          |          |          | genera   |
|          |          |          |          |          | alerta   |
|          |          |          |          |          | en P1.   |
|          |          |          |          |          | Son      |
|          |          |          |          |          | etapas   |
|          |          |          |          |          | d        |
|          |          |          |          |          | istintas |
|          |          |          |          |          | del      |
|          |          |          |          |          | mismo    |
|          |          |          |          |          | flujo.   |
+----------+----------+----------+----------+----------+----------+
| **P34**  | Borrado  | Com.     | auditd / | ★        | P18      |
|          | o        | /Evasión | ino      |          | detecta  |
|          | modi     |          | tifywait | **ALTO** | modi     |
|          | ficación |          | sobre    |          | ficación |
|          | de logs  |          |          |          | de       |
|          | del      |          | /        |          | archivos |
|          | sistema  |          | var/log, |          | de       |
|          |          |          | /        |          | sistema  |
|          |          |          | data/log |          | op       |
|          |          |          |          |          | erativo. |
|          |          |          |          |          | P34      |
|          |          |          |          |          | detecta  |
|          |          |          |          |          | específ  |
|          |          |          |          |          | icamente |
|          |          |          |          |          | la       |
+----------+----------+----------+----------+----------+----------+

\

 

+----------+----------+----------+----------+----------+----------+---+
| **       | **Nombre | **Capa** | **Her    | **Peso** | **Justi  |   |
| Código** | co       |          | ramienta |          | ficación |   |
|          | mpleto** |          | de       |          | de no    |   |
|          |          |          | c        |          | redun    |   |
|          |          |          | aptura** |          | dancia** |   |
+==========+==========+==========+==========+==========+==========+===+
|          |          |          |          |          | eli      |   |
|          |          |          |          |          | minación |   |
|          |          |          |          |          | o        |   |
|          |          |          |          |          | al       |   |
|          |          |          |          |          | teración |   |
|          |          |          |          |          | de       |   |
|          |          |          |          |          | r        |   |
|          |          |          |          |          | egistros |   |
|          |          |          |          |          | de       |   |
|          |          |          |          |          | au       |   |
|          |          |          |          |          | ditoría. |   |
|          |          |          |          |          | Son      |   |
|          |          |          |          |          | compleme |   |
|          |          |          |          |          | ntarios: |   |
|          |          |          |          |          | P18      |   |
|          |          |          |          |          | cubre    |   |
|          |          |          |          |          | persi    |   |
|          |          |          |          |          | stencia, |   |
|          |          |          |          |          | P34      |   |
|          |          |          |          |          | cubre    |   |
|          |          |          |          |          | evasión  |   |
|          |          |          |          |          | forense  |   |
|          |          |          |          |          | po       |   |
|          |          |          |          |          | sterior. |   |
+----------+----------+----------+----------+----------+----------+---+
| **P35**  | Compor   | Com.     | Perfil   | **bajo** | P        |   |
|          | tamiento | /Evasión | est      |          | arámetro |   |
|          | fuera de |          | adístico |          | est      |   |
|          | ventana  |          | de       |          | adístico |   |
|          | temporal |          | a        |          | que      |   |
|          | habitual |          | ctividad |          | requiere |   |
|          |          |          | horaria  |          | un       |   |
|          |          |          | del      |          | baseline |   |
|          |          |          | usuario  |          | de       |   |
|          |          |          |          |          | compor   |   |
|          |          |          |          |          | tamiento |   |
|          |          |          |          |          | del      |   |
|          |          |          |          |          | usuario. |   |
|          |          |          |          |          | Se       |   |
|          |          |          |          |          | c        |   |
|          |          |          |          |          | lasifica |   |
|          |          |          |          |          | como     |   |
|          |          |          |          |          | bajo     |   |
|          |          |          |          |          | peso     |   |
|          |          |          |          |          | porque   |   |
|          |          |          |          |          | la       |   |
|          |          |          |          |          | ventana  |   |
|          |          |          |          |          | temporal |   |
|          |          |          |          |          | varía    |   |
|          |          |          |          |          | eno      |   |
|          |          |          |          |          | rmemente |   |
|          |          |          |          |          | entre    |   |
|          |          |          |          |          | usuarios |   |
|          |          |          |          |          | y su     |   |
|          |          |          |          |          | tasa de  |   |
|          |          |          |          |          | falsos   |   |
|          |          |          |          |          | p        |   |
|          |          |          |          |          | ositivos |   |
|          |          |          |          |          | inicial  |   |
|          |          |          |          |          | es alta. |   |
|          |          |          |          |          | Aporta   |   |
|          |          |          |          |          | valor a  |   |
|          |          |          |          |          | medio    |   |
|          |          |          |          |          | plazo    |   |
|          |          |          |          |          | cuando   |   |
|          |          |          |          |          | el       |   |
|          |          |          |          |          | sistema  |   |
|          |          |          |          |          | ha       |   |
|          |          |          |          |          | a        |   |
|          |          |          |          |          | prendido |   |
|          |          |          |          |          | el       |   |
|          |          |          |          |          | perfil   |   |
|          |          |          |          |          | ind      |   |
|          |          |          |          |          | ividual. |   |
+----------+----------+----------+----------+----------+----------+---+
| **P36**  | Tráfico  | Com.     | NetStat  | ★        | P2       |   |
|          | cifrado  | /Evasión | / VPN    |          | detecta  |   |
|          | a        |          | API      | **ALTO** | comu     |   |
|          | dir      |          | monitor  |          | nicación |   |
|          | ecciones |          | / eBPF   |          | no       |   |
|          | IP no    |          | network  |          | cifrada. |   |
|          | categ    |          | probe    |          | P36      |   |
|          | orizadas |          |          |          | detecta  |   |
|          |          |          |          |          | el caso  |   |
|          |          |          |          |          | complem  |   |
|          |          |          |          |          | entario: |   |
|          |          |          |          |          | tráfico  |   |
|          |          |          |          |          | cifrado  |   |
|          |          |          |          |          | (que     |   |
|          |          |          |          |          | evita    |   |
|          |          |          |          |          | P2)      |   |
|          |          |          |          |          | dirigido |   |
|          |          |          |          |          | a IPs    |   |
|          |          |          |          |          | fuera de |   |
|          |          |          |          |          | lista    |   |
|          |          |          |          |          | blanca   |   |
|          |          |          |          |          | c        |   |
|          |          |          |          |          | onocida. |   |
|          |          |          |          |          | Cubre el |   |
|          |          |          |          |          | e        |   |
|          |          |          |          |          | scenario |   |
|          |          |          |          |          | en que   |   |
|          |          |          |          |          | el       |   |
|          |          |          |          |          | atacante |   |
|          |          |          |          |          | usa      |   |
|          |          |          |          |          | cifrado  |   |
|          |          |          |          |          | prec     |   |
|          |          |          |          |          | isamente |   |
|          |          |          |          |          | para     |   |
|          |          |          |          |          | evadir   |   |
|          |          |          |          |          | los      |   |
|          |          |          |          |          | m        |   |
|          |          |          |          |          | onitores |   |
|          |          |          |          |          | de       |   |
|          |          |          |          |          | tráfico  |   |
|          |          |          |          |          | no       |   |
|          |          |          |          |          | cifrado. |   |
+----------+----------+----------+----------+----------+----------+---+

 

### 5.  Regla de clasificación propuesta para n=36 {#regla-de-clasificacin-propuesta-para-n36}

En n=36 existen 31 parámetros de alto peso intrusivo y 5 de bajo peso
(P6, P13, P22, P25, P35). El umbral de clasificación se calibra
siguiendo el mismo criterio proporcional que en n=9 y n=25, buscando la
mayor aproximación posible a la distribución de referencia. La
concentración de la distribución binomial en torno a la media conforme
crece n hace que el porcentaje de intrusión sea estructuralmente menor
que en n=9, como se indica en la tabla.

 

+----------------------+----------------------+----------------------+
| **Clasificación**    | **Condición sobre    | **Distribución       |
|                      | alto peso activos**  | estimada**           |
+======================+======================+======================+
| **1 --- INTRUSIÓN**  | ≥ 21 de 31           | \~3.6% del espacio   |
|                      | parámetros de alto   | combinacional        |
|                      |                      |                      |
|                      | peso activos         |                      |
+----------------------+----------------------+----------------------+
| **U ---              | 16 -- 20 de 31       | \~46.3% del espacio  |
| INDETERMINADO**      | parámetros de alto   | combinacional        |
|                      | peso activos         |                      |
+----------------------+----------------------+----------------------+
| **0 --- NORMAL**     | ≤ 15 de 31           | \~50.1% del espacio  |
|                      | parámetros de alto   | combinacional        |
|                      |                      |                      |
|                      | peso activos         |                      |
+----------------------+----------------------+----------------------+

 

**Nota sobre la distribución de intrusión: **El descenso del porcentaje
de intrusión del \~9.5% (n=25) al \~3.6% (n=36) no refleja una
degradación del sistema sino una propiedad matemática de la distribución
binomial B(31, 0.5): a mayor número de parámetros de alto peso, la masa
de probabilidad se concentra más en la región central y los eventos
extremos ---alta activación simultánea de muchos parámetros--- se
vuelven estadísticamente más raros. Esto implica que cuando un vector de
n=36 se clasifica como intrusión, la evidencia acumulada es
proporcionalmente mayor que en n=9, lo que reduce el riesgo de falso
positivo en los casos que sí alcanzan el umbral.

\

 

### 6.  Valoración: ¿es recomendable la extensión a n=36? {#valoracin-es-recomendable-la-extensin-a-n36}

###### 6.1   Argumentos a favor {#argumentos-a-favor}

Los 11 nuevos parámetros cubren dos dominios del ciclo de amenaza
completamente ausentes en n=25: la capa de autenticación e identidad
(P26--P31) y la capa de evasión forense (P32--P36, salvo P35). La
verificación adversarial confirma que ninguno de los 11 parámetros
nuevos es semánticamente redundante con ninguno de los 25 existentes.

La extensión cierra el ciclo de amenaza de manera más completa que
ningún nivel anterior: n=9 detectaba presencia de canales activos; n=16
añadió persistencia de sistema; n=25 añadió acceso al almacenamiento y
contexto operativo; n=36 añade control de identidad y evasión forense.
El sistema puede ahora detectar no solo que un proceso malicioso existe
y opera, sino también que ha intentado eliminar las evidencias de su
actividad.

El umbral proporcional de ≥21/31 parámetros de alto peso garantiza que
las clasificaciones de intrusión en n=36, aunque estadísticamente más
escasas, sean cualitativamente más contundentes que en niveles
anteriores: requieren activación simultánea de al menos el 68% de los
sensores de alto peso.

 

###### 6.2   Limitaciones y riesgos {#limitaciones-y-riesgos}

P35 (comportamiento fuera de ventana temporal) introduce por primera vez
un parámetro de naturaleza estadística en el sistema, que es
fundamentalmente distinto de los demás. Todos los parámetros de P1 a P34
son observaciones directas de eventos discretos; P35 es una inferencia
sobre un perfil histórico. Esto rompe parcialmente la homogeneidad del
vector de características y puede afectar a la convergencia del
clasificador durante el entrenamiento.

P27 (biometría fallida reiterada) y P33 (DNS no categorizados) requieren
calibración de umbrales de activación específicos para cada usuario y
entorno. Sin esa calibración, la tasa de falsos positivos puede ser
inasumible en dispositivos de uso intensivo.

P30 y P31 (SMS y llamadas sin interacción) solo son aplicables a
smartwatches con tarjeta SIM propia. En dispositivos que operan
exclusivamente por Bluetooth o WiFi anclado al teléfono, estos dos
parámetros valen siempre 0, lo que reduce efectivamente el espacio de
parámetros funcionales a n=34 para esos dispositivos. Esta
heterogeneidad de plataforma debe contemplarse en el diseño del
clasificador.

El número de parámetros que requieren privilegios de sistema (P19, P20,
P28, P33, P34) ha aumentado respecto a n=25. En despliegues sobre
dispositivos de consumo estándar, estos parámetros pueden no ser
accesibles sin modificar el sistema operativo del dispositivo objetivo.

 

###### 6.3   Conclusión y recomendación {#conclusin-y-recomendacin}

La extensión a n=36 es conceptualmente sólida y los 11 parámetros nuevos
aportan cobertura genuina. Sin embargo, la recomendación de
implementación es más cautelosa que en el caso de n=25. Se propone una
secuencia de implementación en dos fases: una primera fase implementa
P26, P28, P29, P32, P33, P34 y P36, que son los de mayor certeza técnica
y menor dependencia de baseline histórico; una segunda fase incorpora
P27, P30, P31 y P35 una vez validados los umbrales de activación sobre
datos reales.

En ningún caso se recomienda avanzar a n=36 sin haber validado
empíricamente n=25 previamente. La progresión lógica del sistema exige
que cada nivel demuestre su utilidad sobre el anterior antes de escalar.
La arquitectura formal de la progresión n=b² es correcta e internamente
consistente; lo que está pendiente de demostración es que cada nueva
capa de parámetros mejora efectivamente la sensibilidad del clasificador
sobre datos reales de dispositivos comprometidos.

\

 

## Framework avanzado {#framework-avanzado}

#### Extensión del espacio de parámetros {#extensin-del-espacio-de-parmetros}

####   {#r4455816606}

**De n=36 a n=49**: integridad del sistema y gestión en entornos
gestionados y

médicos

     
  -- --
     

\

**PREMISA DE APLICACIÓN: ENTORNO GESTIONADO**

*Este nivel de extensión requiere dispositivos bajo gestión MDM
corporativa o clínica, con la app del framework firmada con certificado
de fabricante o de gestión empresarial. No es aplicable a dispositivos
de consumo estándar sin modificar.*

 

 

### 1.  Introducción y posición en la serie {#introduccin-y-posicin-en-la-serie}

Este documento **es el cuarto de la serie **de extensión formal del
*framework *y cubre el salto de n=36 (b=6) a n=49 (b=7). **Con n=49 **se
alcanza la cuarta extensión de la progresión n=b² con b≥3, que queda:
n=9 (base implementado), n=16, n=25, n=36 y n=49. El espacio
combinacional asciende de 2³⁶ ≈ 68.700 millones a 2⁴⁹ ≈ 562 billones de
vectores posibles.

Este nivel introduce una diferencia cualitativa respecto a los
anteriores: los 13 nuevos parámetros (P37--P49) se organizan en dos
capas nuevas ---Integridad del Sistema Operativo y Gestión
Corporativa/MDM--- que requieren privilegios de sistema que no están
disponibles en dispositivos de consumo estándar. El ámbito de aplicación
cambia por tanto de manera explícita: n=49 está concebido para entornos
gestionados, **con especial relevancia en el sector médico**, donde la
normativa vigente (HIPAA, RGPD, Esquema Nacional de Seguridad) impone
requisitos de seguridad que los niveles anteriores no cubren en su
totalidad.

Los documentos anteriores de la serie establecen la base técnica y dan
por conocidos los 36 parámetros definidos en ellos. Este documento no
los repite salvo en la tabla de resumen de la sección 2, donde se
recogen de forma compacta como línea base.

 

 

### 2.  Premisas de aplicación para n=49 {#premisas-de-aplicacin-para-n49}

La viabilidad técnica y empírica de los nuevos parámetros está
condicionada a la satisfacción de las siguientes premisas. Son
condiciones necesarias, no suficientes. Su incumplimiento no invalida
los niveles n=9 a n=36, que continúan siendo aplicables en dispositivos
de consumo.

     
  -- --
     

\

El dispositivo debe estar enrolado en un sistema de gestión de
dispositivos móviles (MDM) activo

---Google Android Enterprise, Microsoft Intune, VMware Workspace ONE u
equivalente--- que proporcione un perfil de trabajo con políticas de
seguridad aplicadas de forma forzosa. El perfil MDM es condición
necesaria para P44, P45, P46, P47 y P48.

\

La aplicación que implementa el framework debe estar firmada con el
certificado del fabricante del dispositivo o con un certificado de
perfil empresarial de nivel platform. Esto es necesario para acceder a
AppOpsManager (P43), AccessibilityManager (P37), MediaProjectionManager
(P38), IFF_PROMISC flag (P41) y BackupManager (P48). En el contexto
médico, esta firma puede obtenerse a través del fabricante del
dispositivo clínico o mediante el MDM del centro sanitario.

     
  -- --
     

\

Los dispositivos en producción clínica o corporativa deben tener el modo
desarrollador desactivado como condición de partida. P49 detecta la
violación de esta premisa. Si P49 se activa en un dispositivo que debía
estar en producción, el evento tiene valor diagnóstico máximo: implica
que alguien ha modificado el estado de seguridad del dispositivo de
forma deliberada.

     
  -- --
     

\

El MDM debe imponer una política de bloqueo de pantalla con PIN de al
menos 6 dígitos o autenticación biométrica con factor secundario. P47
detecta la eliminación de esta política. Sin la premisa P-4, P47 no
tiene baseline contra el que comparar.

     
  -- --
     

\

Todo el tráfico del dispositivo debe circular por la VPN corporativa o
clínica. En entornos hospitalarios, esta premisa es a menudo obligatoria
por normativa interna y por requisitos de auditoría de acceso a historia
clínica electrónica (HCE). P45 detecta cualquier tráfico que salga fuera
del túnel VPN.

     
  -- --
     

\

En el sector sanitario, los dispositivos inteligentes ---incluyendo
smartwatches clínicos para telemonitorización de constantes vitales---
manejan datos de categoría especial según el artículo 9 del RGPD: datos
de salud. Una intrusión en un dispositivo que transmite frecuencia
cardíaca, saturación de oxígeno o parámetros de glucosa en tiempo real
puede tener consecuencias directas sobre la seguridad del paciente. Los
parámetros P37--P43 (capa Integridad del SO) son especialmente
relevantes en este contexto porque un atacante con control del servicio
de accesibilidad (P37) o con capacidad de proyección de pantalla (P38)
puede manipular los valores visualizados en la interfaz clínica sin
modificar los datos en la base de datos, lo que crea un escenario de
ataque que ningún monitor de integridad de datos detectaría. La
detección temprana a nivel de comportamiento del SO, que es lo que
proveen P37--P43, es la única capa de defensa contra este vector.

 

 

 

 

### 3.  Viabilidad técnica, temporal y empírica {#viabilidad-tcnica-temporal-y-emprica}

###### 3.1   Rendimiento en tiempo

Con n=49 la imagen polar tiene 49 ejes separados 7,35° entre sí. La
generación de la imagen y la

inferencia de ResNet34 sobre ella no dependen directamente de n: el
tiempo de inferencia está

\

determinado por la resolución de la imagen de entrada y la arquitectura
de la red, no por el número de parámetros del vector que la genera. El
tiempo de inferencia de ResNet34 sobre una imagen 224×224 en hardware de
gama media es inferior a 50ms, independientemente de si el vector tiene
9, 36 o 49 componentes.

El coste computacional adicional de n=49 respecto a n=36 es el de
capturar los 13 nuevos parámetros. La mayoría (P37, P38, P40, P42, P43,
P44, P47, P49) son eventos discretos que se capturan mediante listeners
de Android/iOS y no requieren polling. Los que sí requieren polling son
P39 (verificación de root, recomendado cada 300s), P41 (flag de red,
recomendado cada 60s), P45 (estado VPN, event-driven), P46 (NTP delta,
cada 30s) y P48 (backup activo, event- driven). El incremento en tráfico
de telemetría respecto a n=36 es estimado en menos del 15%.

 

###### 3.2   Suficiencia del muestreo

El espacio combinacional de n=49 es 2⁴⁹ ≈ 5,6×10¹⁴ vectores. Esta
magnitud hace imposible el muestreo exhaustivo, igual que en los niveles
anteriores. La suficiencia del muestreo se garantiza por el mismo
mecanismo que justifica el uso de ResNet34: la red aprende manifolds de
baja dimensionalidad en el espacio de vectores, no reglas sobre
combinaciones individuales. Los estudios de generalización de ResNet34
sobre espacios de alta dimensionalidad muestran que con 10.000--50.000
ejemplos etiquetados es posible alcanzar una tasa de clasificación
superior al 90% en espacios de hasta 10⁶ clases. Para el espacio
continuo de formas polares de n=49, el número de ejemplos necesario es
comparable al de niveles anteriores por la invariancia rotacional de la
red.

La recomendación de muestreo mínimo para la fase de validación
experimental es: 500 instancias de intrusión confirmada, 2.000
instancias de comportamiento normal y 1.000 instancias de zona
indeterminada, obtenidas en entornos gestionados reales con dispositivos
de los fabricantes más representativos del mercado clínico (Samsung
Galaxy Watch, Apple Watch, Fitbit Sense en modo clínico).

 

###### 3.3   Viabilidad empírica {#viabilidad-emprica}

La viabilidad empírica de n=49 es más alta que la de n=36 en entornos
gestionados porque las premisas P-1 a P-5 son precisamente las
condiciones que ya deben cumplirse por normativa en ese contexto. Un
centro sanitario que cumple la normativa ENS de nivel Alto ya tiene MDM
activo (P-1), VPN obligatoria (P-5) y política de pantalla de bloqueo
forzada (P-4). La instrumentación adicional necesaria para capturar
P37--P49 se reduce a instalar la app del framework con los permisos de
sistema apropiados, algo que el MDM puede desplegar de forma
centralizada en todos los dispositivos del parque sin intervención del
usuario.

El riesgo de viabilidad empírica más alto es P41 (modo promiscuo), que
requiere llamadas ioctl al kernel. En Android con permisos de sistema es
alcanzable, pero en Apple Watch con watchOS el acceso a ese nivel de red
no está disponible sin jailbreak. Para los dispositivos watchOS, P41
debe asignarse valor U de forma sistemática, lo que es el comportamiento
correcto del framework para parámetros no medibles en la plataforma.

 

 

 

 

### 4.  Resumen del estado en n=36

Los 36 parámetros del nivel anterior se presentan a continuación como
línea base. Se omiten las

justificaciones individuales, que se encuentran en los documentos
anteriores de la serie.

 

+----------+-------------------+----------+-----------------+----------+--------------+
| **Cód.** | **Parámetro**     | **Capa** | **Herramienta** | **Peso** | **Estado**   |
+==========+===================+==========+=================+==========+==============+
| **P1**   | URL no autorizada | Red      | Snort           | ★        | Original n=9 |
|          |                   |          |                 |          |              |
|          |                   |          |                 | **ALTO** |              |
+----------+-------------------+----------+-----------------+----------+--------------+

\

+----------+----------+----------+----------+----------+----------+---+
| **Cód.** | **Par    | **Capa** | **Herra  | **Peso** | **       |   |
|          | ámetro** |          | mienta** |          | Estado** |   |
+==========+==========+==========+==========+==========+==========+===+
| **P2**   | Comu     | Red      | Snort    | ★        | Original |   |
|          | nicación |          |          |          | n=9      |   |
|          | no       |          |          | **ALTO** |          |   |
|          | cifrada  |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P3**   | Trans    | Red      | Snort    | ★        | Original |   |
|          | ferencia |          |          |          | n=9      |   |
|          | al       |          |          | **ALTO** |          |   |
|          | exterior |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P4**   | Cer      | Red      | TrustMan | ★        | n=16     |   |
|          | tificado |          | ager/ssl |          |          |   |
|          | TLS      |          |          | **ALTO** |          |   |
|          | inválido |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P5**   | BSSID no | Cone     | Kismet   | ★        | Original |   |
|          | au       | ctividad |          |          | n=9      |   |
|          | torizado |          |          | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P6**   | B        | Cone     | Kismet   | **bajo** | Original |   |
|          | luetooth | ctividad | /PyBluez |          | n=9      |   |
|          | no       |          |          |          |          |   |
|          | au       |          |          |          |          |   |
|          | torizado |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P7**   | GPS no   | Cone     | K        | ★        | Original |   |
|          | au       | ctividad | ismet/Gn |          | n=9      |   |
|          | torizado |          | ssStatus | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P8**   | NFC no   | Cone     | NfcAdapt | ★        | n=16     |   |
|          | au       | ctividad | er/nfcpy |          |          |   |
|          | torizado |          |          | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P9**   | Cámara   | Sensores | cv2      | ★        | Original |   |
|          | no       |          | /camera2 |          | n=9      |   |
|          | au       |          |          | **ALTO** |          |   |
|          | torizada |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P10**  | M        | Sensores | PyAu     | ★        | Original |   |
|          | icrófono |          | dio/Audi |          | n=9      |   |
|          | no       |          | oManager | **ALTO** |          |   |
|          | au       |          |          |          |          |   |
|          | torizado |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P11**  | Sensores | Sensores | SensorM  | ★        | n=16     |   |
|          | de salud |          | anager/H |          |          |   |
|          |          |          | ealthKit | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P12**  | Proceso  | Sensores | psutil   | ★        | n=16     |   |
|          | des      |          | /Activit |          |          |   |
|          | conocido |          | yManager | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P13**  | Pa       | Sistema  | psutil   | **bajo** | Original |   |
|          | rámetros |          |          |          | n=9      |   |
|          | físicos  |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P14**  | Escalada | Sistema  | Packag   | ★        | n=16     |   |
|          | de       |          | eManager |          |          |   |
|          | permisos |          |          | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P15**  | Acceso   | Sistema  | Conten   | ★        | n=16     |   |
|          | datos    |          | tResolve |          |          |   |
|          | pe       |          | r/AppOps | **ALTO** |          |   |
|          | rsonales |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P16**  | Conexión | Sistema  | Telephon | ★        | n=16     |   |
|          | celular  |          | yManager |          |          |   |
|          |          |          |          | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P17**  | Almace   | Almace   | St       | ★        | n=25     |   |
|          | namiento | namiento | orageMan |          |          |   |
|          | externo  |          | ager/SAF | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P18**  | Modi     | Almace   | inotifyw | ★        | n=25     |   |
|          | ficación | namiento | ait/File |          |          |   |
|          | de       |          | Observer | **ALTO** |          |   |
|          | sistema  |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P19**  | IPC      | Almace   | Binder   | ★        | n=25     |   |
|          | anómala  | namiento | monito   |          |          |   |
|          |          |          | r/strace | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P20**  | Port     | Almace   | Clipboar | ★        | n=25     |   |
|          | apapeles | namiento | dManager |          |          |   |
|          |          |          |          | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P21**  | Config.  | Contexto | Con      | ★        | n=25     |   |
|          | red      |          | nectivit |          |          |   |
|          | mo       |          | yManager | **ALTO** |          |   |
|          | dificada |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P22**  | Sensores | Contexto | Senso    | **bajo** | n=25     |   |
|          | de       |          | rManager |          |          |   |
|          | mo       |          | TYPE_ACC |          |          |   |
|          | vimiento |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P23**  | Modo     | Contexto | Setting  | ★        | n=25     |   |
|          | avión    |          | s.Global |          |          |   |
|          | i        |          | observer | **ALTO** |          |   |
|          | rregular |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P24**  | Sincro   | Contexto | Cont     | ★        | n=25     |   |
|          | nización |          | entResol |          |          |   |
|          | externa  |          | ver/Sync | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+

\

+----------+----------+----------+----------+----------+----------+---+
| **Cód.** | **Par    | **Capa** | **Herra  | **Peso** | **       |   |
|          | ámetro** |          | mienta** |          | Estado** |   |
+==========+==========+==========+==========+==========+==========+===+
| **P25**  | Push     | Contexto | FCM/Noti | **bajo** | n=25     |   |
|          | noti     |          | fication |          |          |   |
|          | fication |          | Listener |          |          |   |
|          | anómala  |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P26**  | Cambio   | Auten    | Accoun   | ★        | n=36     |   |
|          | de       | ticación | tManager |          |          |   |
|          | cred     |          |          | **ALTO** |          |   |
|          | enciales |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P27**  | B        | Auten    | Biometri | ★        | n=36     |   |
|          | iometría | ticación | cManager |          |          |   |
|          | fallida  |          |          | **ALTO** |          |   |
|          | r        |          |          |          |          |   |
|          | eiterada |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P28**  | Acceso   | Auten    | Android  | ★        | n=36     |   |
|          | al       | ticación | Keystore |          |          |   |
|          | keystore |          |          | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P29**  | Token de | Auten    | O        | ★        | n=36     |   |
|          | sesión   | ticación | Auth/JWT |          |          |   |
|          |          |          | monitor  | **ALTO** |          |   |
|          | ma       |          |          |          |          |   |
|          | nipulado |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P30**  | SMS/MMS  | Com.     | Sm       | ★        | n=36     |   |
|          | sin      | /Evasión | sManager |          |          |   |
|          | int      |          |          | **ALTO** |          |   |
|          | eracción |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P31**  | Llamada  | Com.     | Teleco   | ★        | n=36     |   |
|          | sin      | /Evasión | mManager |          |          |   |
|          | int      |          |          | **ALTO** |          |   |
|          | eracción |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P32**  | API de   | Com.     | Pa       | ★        | n=36     |   |
|          | pago no  | /Evasión | ymentMan |          |          |   |
|          | au       |          | ager/NFC | **ALTO** |          |   |
|          | torizada |          | HCE      |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P33**  | DNS a    | Com.     | DNS      | ★        | n=36     |   |
|          | dominios | /Evasión | sniffe   |          |          |   |
|          | no       |          | r/dnstap | **ALTO** |          |   |
|          | categ.   |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P34**  | Borrad   | Com.     | au       | ★        | n=36     |   |
|          | o/modif. | /Evasión | ditd/ino |          |          |   |
|          | de logs  |          | tifywait | **ALTO** |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P35**  | Compor   | Com.     | Perfil   | **bajo** | n=36     |   |
|          | tamiento | /Evasión | est      |          |          |   |
|          | fuera    |          | adístico |          |          |   |
|          | horario  |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+
| **P36**  | Tráfico  | Com.     | NetS     | ★        | n=36     |   |
|          | cifrado  | /Evasión | tat/eBPF |          |          |   |
|          | a IPs    |          | probe    | **ALTO** |          |   |
|          | anón.    |          |          |          |          |   |
+----------+----------+----------+----------+----------+----------+---+

 

**Estado en n=36: **36 parámetros, 31 de alto peso, 5 de bajo peso (P6,
P13, P22, P25, P35). Umbral de intrusión: ≥21/31 parámetros de alto peso
activos (\~3.5%).

 

 

 

 

### 5.  Justificación del salto de n=36 a n=49 {#justificacin-del-salto-de-n36-a-n49}

Los 36 parámetros existentes asumen implícitamente que el sistema
operativo del dispositivo es fiable y que el entorno de ejecución del
framework no está comprometido. Esta asunción es razonable en entornos
de consumo pero insuficiente en entornos médicos o corporativos de alta
seguridad, donde el atacante puede tener acceso físico al dispositivo,
puede haber comprometido el entorno de gestión MDM, o puede estar
utilizando el propio SO como vector de ataque mediante servicios de
accesibilidad o proyección de pantalla.

El análisis de los vectores de amenaza no cubiertos en n=36 revela dos
dominios sin representación. El primero es la integridad del sistema
operativo como plataforma de ejecución: si el SO está comprometido
(root, overlay, accesibilidad abusada), los parámetros P1--P36 pueden
producir falsos negativos porque el malware puede interceptar o suprimir
sus señales. El segundo dominio es la gestión y cumplimiento de política
corporativa: en un entorno gestionado, la desviación de la política de
seguridad ---MDM modificado, VPN desactivada, pantalla de bloqueo
eliminada, ADB activo--- es por sí misma un indicador de intrusión o de
preparación para ella, independientemente de si algún parámetro de los
36 existentes ha disparado todavía.

El espacio combinacional sube de 2³⁶ a 2⁴⁹, un factor de multiplicación
de 8.192. Esta magnitud no tiene implicación práctica en el tiempo de
inferencia (que es el de la CNN, no el del vector) pero sí

\

refuerza el argumento de necesidad de IA frente a sistemas de reglas,
que ya no son competitivos

a partir de n=16 y son computacionalmente inviables a n=49.

 

 

 

 

### 6.  Los 13 nuevos parámetros: P37 a P49 {#los-13-nuevos-parmetros-p37-a-p49}

Los 13 nuevos parámetros se distribuyen en dos capas nuevas de 7
parámetros cada una, siguiendo la arquitectura de 7 capas de 7
parámetros de n=49: Capa Integridad del Sistema Operativo (P37--P43) y
Capa Gestión Corporativa/MDM (P44--P49, con la séptima posición ocupada
por P43 en la reorganización final de 7×7). Cada parámetro incluye
justificación de no redundancia con los 36 existentes, herramienta de
captura, análisis adversarial y relevancia específica en entorno médico
donde aplica.

 

+----------+----------+----------+----------+----------+----------+
| **Có     | **       | **Capa** | **Herra  | **Pes    | **No     |
| d.**     | Nombre** |          | mienta** | o**      | red      |
|          |          |          |          |          | undancia |
|          |          |          |          |          | con      |
|          |          |          |          |          | n=36**   |
+==========+==========+==========+==========+==========+==========+
| **P3 7** | Access   | Integri  | Acces    | ★ **ALT  | P14      |
|          | ibilityS | dad SO   | sibility | O**      | cubre    |
|          | ervice   |          | Manager. |          | escalada |
|          | no       |          | getEnabl |          | de       |
|          | au       |          | edAccess |          | permisos |
|          | torizado |          | ibilityS |          | por      |
|          |          |          | erviceLi |          | Package  |
|          |          |          | st()     |          | Manager. |
|          |          |          |          |          | P37      |
|          |          |          |          |          | cubre el |
|          |          |          |          |          | abuso    |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | de un    |
|          |          |          |          |          | servicio |
|          |          |          |          |          | de       |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | acces    |
|          |          |          |          |          | ibilidad |
|          |          |          |          |          | con      |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | permiso  |
|          |          |          |          |          | c        |
|          |          |          |          |          | oncedido |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | por el   |
|          |          |          |          |          | usuario  |
|          |          |          |          |          | (no      |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | es       |
|          |          |          |          |          | calada): |
|          |          |          |          |          | da       |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | acceso   |
|          |          |          |          |          | total a  |
|          |          |          |          |          | la       |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | p        |
|          |          |          |          |          | antalla, |
|          |          |          |          |          | texto y  |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | co       |
|          |          |          |          |          | ntroles. |
|          |          |          |          |          | En       |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | entorno  |
|          |          |          |          |          | médico   |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | puede    |
|          |          |          |          |          | leer y   |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | m        |
|          |          |          |          |          | anipular |
|          |          |          |          |          | in       |
|          |          |          |          |          | terfaces |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | c        |
|          |          |          |          |          | línicas. |
+----------+----------+----------+----------+----------+----------+
| **P3 8** | G        | Integri  | MediaP   | ★ **ALT  | P9       |
|          | rabación | dad SO   | rojectio | O**      | detecta  |
|          | de       |          | nManager |          | cámara   |
|          | pantalla |          | ---      |          | física.  |
|          | no       |          | d        |          | P38      |
|          | au       |          | etección |          | detecta  |
|          | torizada |          | de       |          | MediaPr  |
|          |          |          | sesión   |          | ojection |
|          |          |          | activa   |          |          |
|          |          |          | fuera de |          | API:     |
|          |          |          | app      |          | captura  |
|          |          |          | au       |          | digital  |
|          |          |          | torizada |          |          |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | de       |
|          |          |          |          |          | pantalla |
|          |          |          |          |          | sin      |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | activar  |
|          |          |          |          |          | hardware |
|          |          |          |          |          | de       |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | cámara.  |
|          |          |          |          |          | En       |
|          |          |          |          |          | entorno  |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | clínico  |
|          |          |          |          |          | puede    |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | capturar |
|          |          |          |          |          | HCE,     |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | valores  |
|          |          |          |          |          | de       |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | co       |
|          |          |          |          |          | nstantes |
|          |          |          |          |          | vitales  |
|          |          |          |          |          | y        |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | cred     |
|          |          |          |          |          | enciales |
|          |          |          |          |          | de       |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | acceso a |
|          |          |          |          |          | sistemas |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | hospit   |
|          |          |          |          |          | alarios. |
+----------+----------+----------+----------+----------+----------+
| **P3 9** | Dis      | Integri  | RootBeer | ★ **ALT  | El root  |
|          | positivo | dad SO   | /        | O**      | no       |
|          | rooteado |          | S        |          | activa   |
|          | o        |          | afetyNet |          | ningún   |
|          | bo       |          | Att      |          | p        |
|          | otloader |          | estation |          | arámetro |
|          |          |          | /        |          | de       |
|          | desb     |          | comp     |          | P1--P36. |
|          | loqueado |          | robación |          | Es la    |
|          |          |          |          |          |          |
|          |          |          | binarios |          | c        |
|          |          |          | su,      |          | ondición |
|          |          |          | busybox, |          | que      |
|          |          |          | Magisk   |          |          |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | invalida |
|          |          |          |          |          | la       |
|          |          |          |          |          | fi       |
|          |          |          |          |          | abilidad |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | de P12,  |
|          |          |          |          |          | P18,     |
|          |          |          |          |          | P34:     |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | un       |
|          |          |          |          |          | proceso  |
|          |          |          |          |          | con root |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | puede    |
|          |          |          |          |          | fa       |
|          |          |          |          |          | lsificar |
|          |          |          |          |          | sus      |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | res      |
|          |          |          |          |          | puestas. |
|          |          |          |          |          | En       |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | entorno  |
|          |          |          |          |          | médico   |
|          |          |          |          |          | el       |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | root es  |
|          |          |          |          |          | una      |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | v        |
|          |          |          |          |          | iolación |
|          |          |          |          |          | de la    |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | premisa  |
|          |          |          |          |          | de       |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | c        |
|          |          |          |          |          | onfianza |
|          |          |          |          |          | del      |
+----------+----------+----------+----------+----------+----------+
|          |          |          |          |          | disp     |
|          |          |          |          |          | ositivo. |
+----------+----------+----------+----------+----------+----------+

\

+----------+----------+----------+----------+----------+----------+
| **Có     | **       | **Capa** | **Herra  | **Pes    | **No     |
| d.**     | Nombre** |          | mienta** | o**      | red      |
|          |          |          |          |          | undancia |
|          |          |          |          |          | con      |
|          |          |          |          |          | n=36**   |
+==========+==========+==========+==========+==========+==========+
| **P4 0** | Ins      | Integri  | PackageI | ★ **ALT  | P12      |
|          | talación | dad SO   | nstaller | O**      | detecta  |
|          | de       |          | .Session |          | el       |
|          | paquete  |          | Callback |          | proceso  |
|          | desde    |          | ---      |          | en       |
|          | fuente   |          | onCr     |          | ej       |
|          | no       |          | eated/on |          | ecución. |
|          | au       |          | Finished |          | P40      |
|          | torizada |          | fuera de |          | detecta  |
|          |          |          | stores   |          | la       |
|          |          |          | aut      |          | ins      |
|          |          |          | orizados |          | talación |
|          |          |          |          |          | si       |
|          |          |          |          |          | lenciosa |
|          |          |          |          |          | antes de |
|          |          |          |          |          | la       |
|          |          |          |          |          | primera  |
|          |          |          |          |          | ej       |
|          |          |          |          |          | ecución, |
|          |          |          |          |          | que      |
|          |          |          |          |          | puede    |
|          |          |          |          |          | ocurrir  |
|          |          |          |          |          | y        |
|          |          |          |          |          | re       |
|          |          |          |          |          | vertirse |
|          |          |          |          |          | entre    |
|          |          |          |          |          | m        |
|          |          |          |          |          | uestreos |
|          |          |          |          |          | de P12.  |
|          |          |          |          |          | En       |
|          |          |          |          |          | entorno  |
|          |          |          |          |          | médico   |
|          |          |          |          |          | la       |
|          |          |          |          |          | ins      |
|          |          |          |          |          | talación |
|          |          |          |          |          | no       |
|          |          |          |          |          | au       |
|          |          |          |          |          | torizada |
|          |          |          |          |          | de apps  |
|          |          |          |          |          | viola la |
|          |          |          |          |          | política |
|          |          |          |          |          | de       |
|          |          |          |          |          | dis      |
|          |          |          |          |          | positivo |
|          |          |          |          |          | clínico. |
+----------+----------+----------+----------+----------+----------+
| **P4 1** | Interfaz | Integri  | N        | ★ **ALT  | Ningún   |
|          | de red   | dad SO   | etworkIn | O**      | p        |
|          | en modo  |          | terface. |          | arámetro |
|          | p        |          | isUp() + |          | detecta  |
|          | romiscuo |          | IFF      |          | sniffing |
|          |          |          | _PROMISC |          | pasivo   |
|          |          |          | flag vía |          | de red.  |
|          |          |          | ioctl    |          | P3=0     |
|          |          |          |          |          | porque   |
|          |          |          | SIOC     |          | no hay   |
|          |          |          | GIFFLAGS |          | trans    |
|          |          |          |          |          | ferencia |
|          |          |          |          |          | al       |
|          |          |          |          |          | e        |
|          |          |          |          |          | xterior; |
|          |          |          |          |          | P1=0     |
|          |          |          |          |          | porque   |
|          |          |          |          |          | no hay   |
|          |          |          |          |          | URL. En  |
|          |          |          |          |          | entorno  |
|          |          |          |          |          | hosp     |
|          |          |          |          |          | italario |
|          |          |          |          |          | el       |
|          |          |          |          |          | sniffing |
|          |          |          |          |          | pasivo   |
|          |          |          |          |          | de la    |
|          |          |          |          |          | red      |
|          |          |          |          |          | clínica  |
|          |          |          |          |          | puede    |
|          |          |          |          |          | capturar |
|          |          |          |          |          | tráfico  |
|          |          |          |          |          | de otros |
|          |          |          |          |          | disp     |
|          |          |          |          |          | ositivos |
|          |          |          |          |          | del      |
|          |          |          |          |          | p        |
|          |          |          |          |          | aciente. |
|          |          |          |          |          | U en     |
|          |          |          |          |          | watchOS  |
|          |          |          |          |          | sin      |
|          |          |          |          |          | permisos |
|          |          |          |          |          | de       |
|          |          |          |          |          | kernel.  |
+----------+----------+----------+----------+----------+----------+
| **P4 2** | Puerto   | Integri  | UsbMa    | ★ **ALT  | Canal de |
|          | USB/OTG  | dad SO   | nager.AC | O**      | exfi     |
|          |          |          | TION_USB |          | ltración |
|          | con      |          | _DEVICE_ |          | física   |
|          | dis      |          | ATTACHED |          | fuera    |
|          | positivo |          | +        |          | del      |
|          | no       |          |          |          | alcance  |
|          | re       |          | com      |          | de todos |
|          | conocido |          | paración |          | los      |
|          |          |          | VID/PID  |          | pa       |
|          |          |          | contra   |          | rámetros |
|          |          |          | lista    |          | de red   |
|          |          |          | blanca   |          | inal     |
|          |          |          |          |          | ámbrica. |
|          |          |          |          |          | En       |
|          |          |          |          |          | entorno  |
|          |          |          |          |          | médico   |
|          |          |          |          |          | el       |
|          |          |          |          |          | acceso   |
|          |          |          |          |          | físico   |
|          |          |          |          |          | al       |
|          |          |          |          |          | dis      |
|          |          |          |          |          | positivo |
|          |          |          |          |          | puede    |
|          |          |          |          |          | pr       |
|          |          |          |          |          | oducirse |
|          |          |          |          |          | durante  |
|          |          |          |          |          | procedi  |
|          |          |          |          |          | mientos. |
|          |          |          |          |          | La       |
|          |          |          |          |          | exfi     |
|          |          |          |          |          | ltración |
|          |          |          |          |          | por USB  |
|          |          |          |          |          | no deja  |
|          |          |          |          |          | rastro   |
|          |          |          |          |          | en       |
|          |          |          |          |          | P1--P36. |
+----------+----------+----------+----------+----------+----------+
| **P4 3** | Super    | Integri  | Ap       | ★ **ALT  | Windo    |
|          | posición | dad SO   | pOpsMana | O**      | wManager |
|          | de UI no |          | ger.getO |          |          |
|          | au       |          | psForPac |          | TYPE_    |
|          | torizada |          | kage(OPS |          | SYSTEM_O |
|          | (overl   |          | TR_SYSTE |          |          |
|          | ay/tapja |          | M_ALERT  |          | VERLAY   |
|          | cking)   |          | _WINDOW) |          | permite  |
|          |          |          | contra   |          | su       |
|          |          |          | lista    |          | perponer |
|          |          |          | blanca   |          | c        |
|          |          |          |          |          | ontenido |
|          |          |          |          |          | i        |
|          |          |          |          |          | nvisible |
|          |          |          |          |          | para     |
|          |          |          |          |          | capturar |
|          |          |          |          |          | crede    |
|          |          |          |          |          | nciales. |
|          |          |          |          |          | En       |
|          |          |          |          |          | entorno  |
|          |          |          |          |          | médico   |
|          |          |          |          |          | puede    |
|          |          |          |          |          | int      |
|          |          |          |          |          | erceptar |
|          |          |          |          |          | PIN de   |
|          |          |          |          |          | acceso a |
|          |          |          |          |          | HCE o    |
|          |          |          |          |          | fa       |
|          |          |          |          |          | lsificar |
|          |          |          |          |          | valores  |
|          |          |          |          |          | m        |
|          |          |          |          |          | ostrados |
|          |          |          |          |          | al       |
|          |          |          |          |          | pro      |
|          |          |          |          |          | fesional |
|          |          |          |          |          | sa       |
|          |          |          |          |          | nitario. |
|          |          |          |          |          | No       |
|          |          |          |          |          | cubierto |
|          |          |          |          |          | por      |
|          |          |          |          |          | ninguno  |
|          |          |          |          |          | de los   |
|          |          |          |          |          | 36       |
|          |          |          |          |          | par      |
|          |          |          |          |          | ámetros. |
+----------+----------+----------+----------+----------+----------+

\

+----------+----------+----------+----------+----------+----------+
| **Có     | **       | **Capa** | **Herra  | **Pes    | **No     |
| d.**     | Nombre** |          | mienta** | o**      | red      |
|          |          |          |          |          | undancia |
|          |          |          |          |          | con      |
|          |          |          |          |          | n=36**   |
+==========+==========+==========+==========+==========+==========+
| **P4 4** | Perfil   | Gestió n | DeviceP  | ★ **ALT  | P14      |
|          | MDM o    | MDM      | olicyMan | O**      | detecta  |
|          | cer      |          | ager.get |          | escalada |
|          | tificado |          | ActiveAd |          | de       |
|          | de       |          | mins() + |          | permisos |
|          | gestión  |          | veri     |          | de app.  |
|          | mo       |          | ficación |          | P44      |
|          | dificado |          |          |          | detecta  |
|          |          |          | hash del |          | la       |
|          |          |          | perfil   |          | modi     |
|          |          |          | MDM vs.  |          | ficación |
|          |          |          | baseline |          | del      |
|          |          |          |          |          | perfil   |
|          |          |          |          |          | de       |
|          |          |          |          |          | gestión  |
|          |          |          |          |          | corp     |
|          |          |          |          |          | orativo, |
|          |          |          |          |          | que es   |
|          |          |          |          |          | el       |
|          |          |          |          |          | m        |
|          |          |          |          |          | ecanismo |
|          |          |          |          |          | de       |
|          |          |          |          |          | control  |
|          |          |          |          |          | de todos |
|          |          |          |          |          | los      |
|          |          |          |          |          | demás    |
|          |          |          |          |          | pa       |
|          |          |          |          |          | rámetros |
|          |          |          |          |          | de       |
|          |          |          |          |          | p        |
|          |          |          |          |          | olítica. |
|          |          |          |          |          | Su       |
|          |          |          |          |          | co       |
|          |          |          |          |          | mpromiso |
|          |          |          |          |          | anula    |
|          |          |          |          |          | las      |
|          |          |          |          |          | g        |
|          |          |          |          |          | arantías |
|          |          |          |          |          | de P47,  |
|          |          |          |          |          | P45 y    |
|          |          |          |          |          | P46.     |
+----------+----------+----------+----------+----------+----------+
| **P4 5** | VPN      | Gestió n | VpnS     | ★ **ALT  | P36      |
|          |          | MDM      | ervice.B | O**      | detecta  |
|          | cor      |          | uilder + |          | tráfico  |
|          | porativa |          | Con      |          | cifrado  |
|          | des      |          | nectivit |          | a IPs    |
|          | activada |          | yManager |          | a        |
|          | o        |          | ---      |          | nónimas. |
|          | by       |          |          |          | P45      |
|          | passeada |          | d        |          | detecta  |
|          |          |          | etección |          | la       |
|          |          |          | de       |          | ausencia |
|          |          |          | tráfico  |          | del      |
|          |          |          | fuera    |          | túnel    |
|          |          |          | del      |          | VPN      |
|          |          |          | túnel    |          | corp     |
|          |          |          | VPN      |          | orativo, |
|          |          |          |          |          | que      |
|          |          |          |          |          | puede    |
|          |          |          |          |          | c        |
|          |          |          |          |          | oexistir |
|          |          |          |          |          | con      |
|          |          |          |          |          | tráfico  |
|          |          |          |          |          | apare    |
|          |          |          |          |          | ntemente |
|          |          |          |          |          | normal   |
|          |          |          |          |          | hacia    |
|          |          |          |          |          | destinos |
|          |          |          |          |          | aut      |
|          |          |          |          |          | orizados |
|          |          |          |          |          | pero sin |
|          |          |          |          |          | el       |
|          |          |          |          |          | cifrado  |
|          |          |          |          |          | de       |
|          |          |          |          |          | tr       |
|          |          |          |          |          | ansporte |
|          |          |          |          |          | corp     |
|          |          |          |          |          | orativo. |
|          |          |          |          |          |          |
|          |          |          |          |          | Obl      |
|          |          |          |          |          | igatorio |
|          |          |          |          |          | en       |
|          |          |          |          |          | entornos |
|          |          |          |          |          | con      |
|          |          |          |          |          | acceso a |
|          |          |          |          |          | HCE.     |
+----------+----------+----------+----------+----------+----------+
| **P4 6** | Hora del | Gestió n | S        |          | Bajo     |
|          | sistema  | MDM      | ettings. |          | peso:    |
|          | mo       |          | Global.A | **baj    | cambios  |
|          | dificada |          | UTO_TIME | o**      | l        |
|          | sin      |          | ob       |          | egítimos |
|          | auto     |          | server + |          | de zona  |
|          | rización |          | delta    |          | horaria  |
|          |          |          | con      |          | y        |
|          |          |          |          |          | sincro   |
|          |          |          | servidor |          | nización |
|          |          |          | NTP de   |          | NTP son  |
|          |          |          | re       |          | fre      |
|          |          |          | ferencia |          | cuentes. |
|          |          |          | cor      |          |          |
|          |          |          | porativo |          | Re       |
|          |          |          |          |          | levancia |
|          |          |          |          |          | alta en  |
|          |          |          |          |          | entorno  |
|          |          |          |          |          | médico:  |
|          |          |          |          |          | los      |
|          |          |          |          |          | ti       |
|          |          |          |          |          | mestamps |
|          |          |          |          |          | de datos |
|          |          |          |          |          | clínicos |
|          |          |          |          |          | tienen   |
|          |          |          |          |          | valor    |
|          |          |          |          |          | legal y  |
|          |          |          |          |          | p        |
|          |          |          |          |          | ericial. |
|          |          |          |          |          | Una      |
|          |          |          |          |          | modi     |
|          |          |          |          |          | ficación |
|          |          |          |          |          | m        |
|          |          |          |          |          | aliciosa |
|          |          |          |          |          | del      |
|          |          |          |          |          | reloj    |
|          |          |          |          |          | puede    |
|          |          |          |          |          | i        |
|          |          |          |          |          | nvalidar |
|          |          |          |          |          | trazas   |
|          |          |          |          |          | de       |
|          |          |          |          |          | monito   |
|          |          |          |          |          | rización |
|          |          |          |          |          | o crear  |
|          |          |          |          |          | ventanas |
|          |          |          |          |          | falsas   |
|          |          |          |          |          | de       |
|          |          |          |          |          | nor      |
|          |          |          |          |          | malidad. |
+----------+----------+----------+----------+----------+----------+
| **P4 7** | Política | Gestió n | DevicePo | ★ **ALT  | No       |
|          | de       | MDM      | licyMana | O**      | cubierto |
|          | pantalla |          | ger.getP |          | en n=36. |
|          | de       |          | asswordQ |          | En       |
|          | bloqueo  |          | uality() |          | entorno  |
|          | e        |          | +        |          | médico   |
|          | liminada |          |          |          | la       |
|          | o        |          | Keyguard |          | pantalla |
|          | de       |          | Manager. |          | de       |
|          | bilitada |          | isDevice |          | bloqueo  |
|          |          |          | Secure() |          | con      |
|          |          |          |          |          | auten    |
|          |          |          |          |          | ticación |
|          |          |          |          |          | fuerte   |
|          |          |          |          |          | es       |
|          |          |          |          |          | obl      |
|          |          |          |          |          | igatoria |
|          |          |          |          |          | por      |
|          |          |          |          |          | HIPAA,   |
|          |          |          |          |          | RGPD     |
|          |          |          |          |          |          |
|          |          |          |          |          | artículo |
|          |          |          |          |          | 32 y ENS |
|          |          |          |          |          | nivel    |
|          |          |          |          |          | Alto. Su |
|          |          |          |          |          | eli      |
|          |          |          |          |          | minación |
|          |          |          |          |          | es un    |
|          |          |          |          |          | i        |
|          |          |          |          |          | ndicador |
|          |          |          |          |          | de       |
|          |          |          |          |          | pre      |
|          |          |          |          |          | paración |
|          |          |          |          |          | para     |
|          |          |          |          |          | acceso   |
|          |          |          |          |          | no       |
|          |          |          |          |          | au       |
|          |          |          |          |          | torizado |
|          |          |          |          |          | o de     |
+----------+----------+----------+----------+----------+----------+

\

  **Có d.**   **Nombre**                                                 **Capa**       **Herramienta**                                                                        **Pes o**     **No redundancia con n=36**
  ----------- ---------------------------------------------------------- -------------- -------------------------------------------------------------------------------------- ------------- ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
                                                                                                                                                                                             manipulación del MDM.
  **P4 8**    Copia de seguridad no autorizada iniciada                  Gestió n MDM   BackupManager.isBackupEnabled() + android:allowBackup monitor + ADB backup detection   ★ **ALT O**   P17 detecta acceso al almacenamiento externo. P48 detecta el evento de backup completo del dispositivo (ADB backup, Google Backup), que puede exfiltrar toda la memoria incluyendo datos de salud sin activar P3 si el destino del backup está en lista blanca de red.
  **P4 9**    Depuración ADB o modo desarrollador activo en producción   Gestió n MDM   Settings.Global.ADB_ENABLED + Settings.Global.DEVELOPMENT_SETTINGS_ENABL ED            ★ **ALT O**   ADB activo en producción clínica habilita shell remoto, backup, instalación de APKs y captura de pantalla sin necesidad de ninguna app. P39 detecta root (que puede requerir ADB previo), pero son condiciones independientes: ADB puede estar activo sin root y vice versa.

 

 

### 7.  Análisis adversarial por capa {#anlisis-adversarial-por-capa}

**7.1   Capa Integridad del Sistema Operativo (P37--P43)**

** **

+---------------------+----------------------+----------------------+
| **Parámetro**       | **Argumento          | **Réplica técnica**  |
|                     | contrario**          |                      |
+=====================+======================+======================+
| **P37 ·**           | Los servicios de     | En un dispositivo    |
|                     | accesibilidad son    | clínico en           |
| **Accessibility**   | usados legítimamente | producción, el       |
|                     | por apps de          | conjunto de apps     |
|                     | discapacidad,        | autorizadas está     |
|                     | lectores de pantalla | definido por el MDM. |
|                     | y apps de control    | Las apps de          |
|                     | parental. Una lista  | accesibilidad        |
|                     | blanca en entorno    | legítimas están en   |
|                     | médico invalidaría   | esa lista. Un        |
|                     | estas apps.          | servicio de          |
|                     |                      | accesibilidad no     |
|                     |                      | incluido en la lista |
|                     |                      | MDM en un            |
|                     |                      | dispositivo          |
|                     |                      | gestionado no tiene  |
|                     |                      | justificación        |
|                     |                      | posible.             |
+---------------------+----------------------+----------------------+
| **P38 ·**           | La proyección de     | Las apps de          |
|                     | pantalla es usada    | tele-asistencia      |
| **MediaProjection** | por apps de          | autorizadas están en |
|                     | tele-asistencia      | la lista blanca del  |
|                     | (TeamViewer,         | MDM. Una sesión de   |
|                     | Anydesk) que son     | MediaProjection      |
|                     | legítimas en entorno | desde una app no     |
|                     | clínico para soporte | incluida en la lista |
|                     | técnico remoto.      | blanca no tiene      |
|                     |                      | justificación en     |
|                     |                      | producción. Además,  |
|                     |                      | las sesiones de      |
|                     |                      | tele- asistencia en  |
|                     |                      | entorno clínico      |
|                     |                      | deben quedar         |
|                     |                      | registradas en el    |
|                     |                      | sistema de           |
|                     |                      | auditoría, lo que    |
|                     |                      | P38 complementa.     |
+---------------------+----------------------+----------------------+

\

+-------------------+--------------------+--------------------+---+
| **Parámetro**     | **Argumento        | **Réplica          |   |
|                   | contrario**        | técnica**          |   |
+===================+====================+====================+===+
| **P39 · Root**    | Algunos            | La distinción es   |   |
|                   | fabricantes de     | entre bootloader   |   |
|                   | dispositivos       | desbloqueado en    |   |
|                   | clínicos           | dispositivo de     |   |
|                   | personalizados     | consumo (anómalo)  |   |
|                   | entregan sus       | y en dispositivo   |   |
|                   | dispositivos con   | clínico            |   |
|                   | bootloader         | certificado con    |   |
|                   | desbloqueado para  | gestión de         |   |
|                   | permitir           | firmware           |   |
|                   | actualizaciones de | documentada        |   |
|                   | firmware OTA.      | (normal). La lista |   |
|                   | Detectar el        | blanca del         |   |
|                   | bootloader         | parámetro debe     |   |
|                   | desbloqueado como  | incluir los        |   |
|                   | intrusión          | modelos de         |   |
|                   | produciría falsos  | dispositivo        |   |
|                   | positivos en esos  | clínico cuyo       |   |
|                   | modelos.           | baseline incluye   |   |
|                   |                    | bootloader         |   |
|                   |                    | desbloqueado de    |   |
|                   |                    | fábrica. Los demás |   |
|                   |                    | casos son          |   |
|                   |                    | intrusión.         |   |
+-------------------+--------------------+--------------------+---+
| **P41 ·**         | El flag            | Para watchOS, P41  |   |
|                   | IFF_PROMISC no es  | toma valor U de    |   |
| **Promiscuo**     | accesible en       | forma sistemática, |   |
|                   | watchOS sin        | que es el          |   |
|                   | jailbreak, lo que  | comportamiento     |   |
|                   | hace P41 no        | correcto del       |   |
|                   | implementable en   | framework para     |   |
|                   | Apple Watch, el    | parámetros no      |   |
|                   | dispositivo        | medibles en la     |   |
|                   | clínico más        | plataforma. La     |   |
|                   | extendido en el    | detección de modo  |   |
|                   | mercado actual.    | promiscuo continúa |   |
|                   |                    | siendo válida y    |   |
|                   |                    | útil en            |   |
|                   |                    | dispositivos       |   |
|                   |                    | Android            |   |
|                   |                    | gestionados        |   |
|                   |                    | (Samsung Galaxy    |   |
|                   |                    | Watch en Android   |   |
|                   |                    | Enterprise).       |   |
+-------------------+--------------------+--------------------+---+
| **P43 · Overlay** | OPSTR_S            | En un entorno      |   |
|                   | YSTEM_ALERT_WINDOW | clínico            |   |
|                   | fue                | gestionado, todas  |   |
|                   |                    | las apps           |   |
|                   | restringido en     | instaladas deben   |   |
|                   | Android 12+ para   | tener              |   |
|                   | apps con           | targetSdkVersion   |   |
|                   | ta                 | actualizado como   |   |
|                   | rgetSdkVersion≥31, | requisito del MDM. |   |
|                   | lo que hace que    | Apps antiguas sin  |   |
|                   | muchas apps        | soporte activo no  |   |
|                   | legítimas antiguas | deben estar en     |   |
|                   | no puedan usarlo.  | producción en un   |   |
|                   | La política puede  | dispositivo        |   |
|                   | haber cambiado     | clínico. Si una    |   |
|                   | entre versiones.   | app antigua con    |   |
|                   |                    | S                  |   |
|                   |                    | YSTEM_ALERT_WINDOW |   |
|                   |                    | activo está en el  |   |
|                   |                    | dispositivo, eso   |   |
|                   |                    | es por sí mismo    |   |
|                   |                    | una violación de   |   |
|                   |                    | la política de     |   |
|                   |                    | actualización.     |   |
+-------------------+--------------------+--------------------+---+

** **

** **

** **

**7.2   Capa Gestión Corporativa/MDM (P44--P49)**

** **

+------------------+------------------------+------------------------+
| **Parámetro**    | **Argumento            | **Réplica técnica**    |
|                  | contrario**            |                        |
+==================+========================+========================+
| **P44 · MDM**    | La verificación del    | DevicePolicyMan        |
|                  | hash del perfil MDM en | ager.getActiveAdmins() |
|                  | cada ciclo de muestreo | es una llamada local   |
|                  | puede generar una      | sin acceso a red y     |
|                  | cantidad significativa | tiene un coste         |
|                  | de llamadas a la API   | computacional mínimo.  |
|                  | de                     | La verificación de     |
|                  | DevicePolicyManager,   | hash del perfil es una |
|                  | con impacto en         | operación sobre un     |
|                  | batería.               | archivo local. La      |
|                  |                        | frecuencia de muestreo |
|                  |                        | recomendada para P44   |
|                  |                        | es 300s, lo que        |
|                  |                        | implica                |
|                  |                        |                        |
|                  |                        | \~288 llamadas/día con |
|                  |                        | coste estimado         |
|                  |                        | inferior al 0.1% de    |
|                  |                        | batería diaria.        |
+------------------+------------------------+------------------------+
| **P45 · VPN**    | Cuando el dispositivo  | P45 debe configurarse  |
|                  | pierde cobertura y se  | con una ventana de     |
|                  | reconecta, hay una     | tolerancia de          |
|                  | ventana de tiempo en   | reconexión             |
|                  | que el tráfico puede   | (recomendado: 30s). Si |
|                  | salir fuera del túnel  | el tráfico fuera del   |
|                  | VPN antes de que se    | túnel persiste más de  |
|                  | restablezca la         | 30 segundos después de |
|                  | conexión. Esto         | recuperar cobertura,   |
|                  | produciría falsos      | se activa. Las         |
|                  | positivos frecuentes   | ventanas de reconexión |
|                  | en entornos con        | legítimas son          |
|                  | cobertura              | inferiores a 30s en el |
|                  | intermitente.          | 95% de los casos según |
|                  |                        | los datos de los       |
|                  |                        | principales MDMs del   |
|                  |                        | mercado hospitalario.  |
+------------------+------------------------+------------------------+
| **P46 · Hora**   | La sincronización NTP  | P46 es parámetro de    |
|                  | automática puede       | bajo peso precisamente |
|                  | producir ajustes de    | por esta razón. El     |
|                  | reloj superiores a     | umbral de activación   |
|                  | varios segundos cuando | recomendado es una     |
|                  | el dispositivo lleva   | desviación sostenida   |
|                  | tiempo sin             | de más de 300 segundos |
|                  | conectividad. Definir  | (5 minutos) respecto   |
|                  | un umbral de           | al servidor NTP        |
|                  | desviación puede ser   | corporativo durante    |
|                  | difícil.               | más de 60 segundos.    |
|                  |                        | Este umbral excluye    |
|                  |                        | todos los ajustes de   |
|                  |                        | sincronización         |
|                  |                        | legítimos, que son     |
|                  |                        | inferiores a 60        |
|                  |                        | segundos incluso tras  |
|                  |                        | periodos largos sin    |
|                  |                        | conectividad.          |
+------------------+------------------------+------------------------+
| **P48 · Backup** | and                    | La premisa de P48 no   |
|                  | roid:allowBackup=false | es que el backup esté  |
|                  | ya está impuesto por   |                        |
|                  | el MDM en dispositivos | desactivado por MDM    |
|                  | gestionados, lo que    | (eso es el estado      |
+------------------+------------------------+------------------------+

\

  **Parámetro**   **Argumento contrario**                                                                                                                                                                              **Réplica técnica**                                                                                                                                                                                                                                                                                                                                                       
  --------------- ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ ---
                  hace que P48 nunca pueda activarse en un entorno correctamente configurado. El parámetro sería redundante con la política MDM.                                                                       deseado), sino que si un atacante modifica el MDM (P44=1) o explota una vulnerabilidad del sistema de backup, P48 detecta que el backup se ha activado a pesar de la política. P44 y P48 son complementarios: P44 detecta la causa, P48 detecta la consecuencia.                                                                                                         
  **P49 · ADB**   Algunos procedimientos de actualización de firmware de dispositivos clínicos requieren activar ADB temporalmente durante el proceso. P49 produciría una alerta durante una actualización legítima.   Las actualizaciones de firmware en entorno clínico deben realizarse fuera de producción y con el dispositivo fuera del parque activo. Una actualización que requiere ADB en un dispositivo en producción clínica activa es una violación del procedimiento de cambio, no del parámetro. P49 alerta correctamente de que algo se está haciendo fuera del procedimiento.   

** **

** **

### 8.  Regla de clasificación propuesta para n=49 {#regla-de-clasificacin-propuesta-para-n49}

En n=49 existen 43 parámetros de alto peso intrusivo (31 de n=36 más los
12 nuevos de alto peso P37--P45, P47--P49) y 6 de bajo peso (P6, P13,
P22, P25, P35 y el nuevo P46). El umbral de clasificación se calibra con
el mismo criterio proporcional de la serie.

La progresión matemática de la distribución binomial B(n_alto, 0.5)
produce una concentración creciente de masa en torno a la media conforme
crece n_alto. Esto significa que el porcentaje de vectores que alcanzan
el umbral de intrusión es estructuralmente decreciente a medida que se
avanza en la serie. Este fenómeno no es un defecto del diseño sino una
consecuencia matemática documentada: cuando ocurre una intrusión en
n=49, la evidencia acumulada es cualitativamente más sólida que en n=9
porque exige la activación simultánea de una fracción inusualmente alta
de todos los sensores.

 

+----------------------+----------------------+----------------------+
| **Clasificación**    | **Condición sobre    | **Distribución       |
|                      | alto peso activos**  | estimada**           |
+======================+======================+======================+
| **1 --- INTRUSIÓN**  | ≥ 29 de 43           | \~1.6% del espacio   |
|                      | parámetros de alto   | combinacional        |
|                      | peso                 |                      |
|                      |                      |                      |
|                      | activos              |                      |
+----------------------+----------------------+----------------------+
| **U ---              | 21 -- 28 de 43       | \~60.3% del espacio  |
| INDETERMINADO**      | parámetros de alto   | combinacional        |
|                      | peso activos         |                      |
+----------------------+----------------------+----------------------+
| **0 --- NORMAL**     | ≤ 20 de 43           | \~38.1% del espacio  |
|                      | parámetros de alto   | combinacional        |
|                      | peso                 |                      |
|                      |                      |                      |
|                      | activos              |                      |
+----------------------+----------------------+----------------------+

 

Tabla de progresión de la distribución a lo largo de la serie (valores
estimados por simulación

Monte Carlo con N=10⁶):

 

  **Nivel**    **n_alto / n_total**   **Umbral intrusión**   **Umbral indet.**   **% Intrusión**   **% Normal**
  ------------ ---------------------- ---------------------- ------------------- ----------------- --------------
  n=9 (base)   7/9                    ≥7                     ≥5                  \~0.8%            \~77%
  n=16         14/16                  ≥10                    ≥7                  \~9.0%            \~40%
  n=25         21/25                  ≥14                    ≥11                 \~9.5%            \~50%
  n=36         31/36                  ≥21                    ≥16                 \~3.5%            \~50%
  **n=49**     **43/49**              **≥29**                **≥21**             **\~1.6%**        **\~38%**

\

 

**Nota sobre el \~38% de normal en n=49: **el porcentaje de vectores
clasificados como normal desciende respecto a n=25 y n=36 porque la zona
indeterminada absorbe una fracción mayor del espacio central de la
distribución binomial al aumentar n_alto. Este resultado es
matemáticamente esperado y no implica degradación del sistema. En
entorno clínico, la zona indeterminada debe interpretarse como señal de
vigilancia activa, no como alarma: es el espacio donde el sistema
requiere supervisión humana adicional, coherente con la filosofía de IA
explicable y auditable del framework.

 

 

 

 

### 9.  Relevancia específica en entorno médico {#relevancia-especfica-en-entorno-mdico}

###### 9.1   Marco normativo

Los dispositivos de telemonitorización de pacientes están sujetos a la
normativa de producto sanitario de clase IIa o IIb (Reglamento EU
2017/745), que exige entre otros requisitos la gestión de riesgos de
ciberseguridad activa durante el ciclo de vida del dispositivo. Los
parámetros P37-- P49 cubren los controles de seguridad exigidos por las
guías de la FDA (Premarket Cybersecurity Guidance 2023), ENISA (Good
practices for security of IoT in the context of Smart Manufacturing) y
el Esquema Nacional de Seguridad en su perfil de cumplimiento para
dispositivos médicos conectados.

 

###### 9.2   Vectores de amenaza específicos del entorno clínico {#vectores-de-amenaza-especficos-del-entorno-clnico}

•       Manipulación de interfaz clínica (P37 + P43): un atacante con
control del servicio de accesibilidad puede modificar los valores
visualizados en la pantalla del smartwatch clínico sin alterar los datos
en el servidor. El profesional sanitario ve valores normales mientras el
registro real muestra anomalías. Este es un vector sin representación en
ningún sistema de detección de intrusiones de red.

•       Exfiltración de datos de salud por canal físico (P42 + P48): un
técnico con acceso físico al dispositivo durante un procedimiento puede
exfiltrar todos los datos de salud del paciente por USB o mediante un
backup ADB en menos de 30 segundos. P42 y P48 son los únicos parámetros
del framework que detectan este vector.

•       Compromiso del timestamp de datos clínicos (P46): en litigios
médicos, los timestamps de los registros de monitorización tienen valor
legal. Una modificación del reloj del sistema previa a un evento clínico
adverso puede hacer indetectable la relación temporal entre el evento y
las mediciones del dispositivo. P46 es el único parámetro que detecta
esta manipulación.

•       Desactivación del canal de comunicación seguro (P45): en
hospitales con acceso a historia clínica electrónica desde el
dispositivo, la desactivación de la VPN corporativa expone el tráfico de
HCE a la red hospitalaria no segmentada. P45 detecta esta condición
antes de que cualquier dato sea exfiltrado.

•       Escalada de privilegios del SO en dispositivo clínico (P39): el
root en un dispositivo clínico invalida la certificación CE del producto
sanitario según el Reglamento EU 2017/745 y puede invalidar la cobertura
del seguro de responsabilidad civil del centro sanitario. Su detección
temprana no es solo una medida de seguridad informática sino una
obligación regulatoria.

\

### 10.   Valoración: ¿es recomendable la extensión a n=49? {#valoracin-es-recomendable-la-extensin-a-n49}

###### 10.1   Condiciones bajo las que sí es recomendable {#condiciones-bajo-las-que-s-es-recomendable}

La extensión a n=49 es recomendable y técnicamente sólida cuando se
satisfacen simultáneamente las premisas P-1 a P-6 enunciadas en la
sección 2. En ese contexto, los 13 nuevos parámetros cubren vectores de
amenaza genuinamente ausentes en n=36, con justificación técnica
verificada y herramientas de captura disponibles en las plataformas
objetivo.

El rendimiento en tiempo es aceptable porque la inferencia de ResNet34
no depende de n y el coste de captura de los nuevos parámetros es
predominantemente event-driven. La viabilidad empírica es alta porque
las premisas de despliegue son condiciones que los entornos gestionados
ya deben satisfacer por normativa. El muestreo suficiente es alcanzable
con los protocolos de recolección de datos estándar en investigación
clínica.

 

###### 10.2   Condiciones bajo las que no es recomendable

La extensión a n=49 no es recomendable en dispositivos de consumo
estándar sin gestión MDM. Los parámetros P44--P49 requieren
infraestructura corporativa que no existe en ese contexto, y P37--P43
requieren privilegios de sistema que solo son alcanzables mediante
modificación del dispositivo. Intentar implementar n=49 en dispositivos
de consumo produciría un sistema con entre 5 y 8 parámetros siempre en
valor U, lo que degrada la información del vector de forma inaceptable.

 

###### 10.3   Recomendación de implementación {#recomendacin-de-implementacin}

Se propone una implementación en tres fases. La primera fase, válida
para entornos gestionados de cualquier sector, implementa P37, P39, P40,
P42, P43, P44, P49: estos siete parámetros son los de mayor claridad
técnica, menor dependencia de baseline histórico y más alto valor
diagnóstico inmediato.

La segunda fase, específica para entornos médicos o de alta seguridad,
añade P38, P41, P45, P47, P48: requieren configuración de VPN
corporativa, política MDM de pantalla de bloqueo activa y, en el caso de
P41, hardware con permisos de kernel disponibles.

La tercera fase, opcional y con mayor requisito de calibración, añade
P46: requiere un servidor NTP de referencia corporativo y calibración
del umbral de desviación sobre datos reales del entorno de despliegue.

En ningún caso se recomienda avanzar a n=49 sin haber validado
empíricamente n=36 en el mismo entorno. La serie completa de documentos
n=9→n=16→n=25→n=36→n=49 representa una progresión formal que debe
recorrerse secuencialmente tanto en el diseño como en la validación. La
solidez del nivel n=49 depende de la solidez demostrada de los niveles
anteriores.

------------------------------------------------------------------------

# **Del sistema de seguridad al sistema de conocimiento: el salto de n=49 a n=625**

------------------------------------------------------------------------

> **SVperitus**: SV porque es un Sistema Vectorial, peritus porque
> construye un experto verificado, y latín porque esa verificación
> formal antes de operar es exactamente lo que el término ya significaba
> hace dos mil años.

------------------------------------------------------------------------

El lector que llega a este punto ha recorrido cinco niveles de un
sistema diseñado para vigilar dispositivos. En cada nivel, el vector ha
crecido en siete parámetros, el espacio combinacional se ha
multiplicado, y los nuevos parámetros han cubierto vectores de amenaza
genuinamente ausentes en el nivel anterior. La serie SVcustos ha
demostrado algo más importante que sus capacidades técnicas: ha
demostrado que la arquitectura es escalable, que el análisis adversarial
por parámetro es un método válido para justificar cada adición, y que la
imagen polar es auditable a cualquier escala dentro de los límites de
resolución visual por capas.

En n=49, esa demostración está completa para el dominio de la
ciberseguridad en dispositivos gestionados. El siguiente cuadrado
perfecto de la serie sería n=64 (b=8), que añadiría siete parámetros más
al sistema de seguridad. Pero el trabajo en n=36 y n=49 había
evidenciado algo que la extensión hacia n=64 no podría resolver: los
parámetros de ciberseguridad relevantes para un smartwatch o dispositivo
similar están razonablemente cubiertos en n=49. Una extensión a n=64
produciría, muy probablemente, redundancias o parámetros de peso tan
bajo que su contribución al vector no justificaría el coste de captura.

El sistema, en cambio, tiene una propiedad que sus aplicaciones de
seguridad no habían agotado: la arquitectura no está ligada al dominio
de sus parámetros. Un vector de n parámetros ternarios puede describir
cualquier objeto cuyo estado sea representable como colección de
indicadores discretos. La imagen polar resultante puede ser clasificada
por ResNet34 con independencia de qué significan esos indicadores. Y la
restricción n=b² garantiza la misma simetría y legibilidad por capas
independientemente del dominio.

Esa propiedad plantea una pregunta que el salto a n=625 intenta
responder: ¿qué ocurre si los parámetros no describen el estado de
seguridad de un dispositivo sino el estado de conocimiento de un agente?
¿Si el valor 1 no significa \"este sensor ha disparado\" sino \"este
área de conocimiento está cubierta en el corpus del agente\"?

El salto de n=49 a n=625 no es una extensión gradual de la serie sino un
cambio de dominio deliberado. El motivo de que n=625 y no n=64 sea el
siguiente nivel es precisamente que el nuevo dominio ---el conocimiento
especializado de un inmunólogo--- tiene una granularidad mínima
razonable de entre 600 y 800 unidades de conocimiento, derivada de la
convergencia de los descriptores MeSH Immunology, el currículo ABIM y la
ontología IUIS. El cuadrado perfecto más cercano a ese rango es n=625
(b=25). La escala no la decide el sistema; la decide el dominio.

Lo que sigue es SVperitus

**SVperitus**

Sistema de vectores de competencia para agentes especialistas

Documento fundacional --- Instancia: Inmunólogo clínico y de
investigación

 

     
  -- --
     

\

Marco teórico, arquitectura técnica, ontología paramétrica n=625, regla
de activación y hoja de ruta hacia producción

 

\

 

### Resumen ejecutivo

\

\

Este documento describe **SVperitus**, un sistema de vectores de
competencia para la construcción de agentes de inteligencia artificial
especialistas. Su primera instancia concreta es el agente inmunólogo,
concebido como un sistema capaz de responder consultas de inmunología
clínica y de investigación con el rigor y la cobertura de conocimiento
de un especialista de nivel MIR- 5/Fellowship.

SVperitus adapta el framework paramétrico del sistema ---diseñado
originalmente para detección de intrusiones en dispositivos inteligentes
mediante vectores de estado binarios--- **al dominio del conocimiento**.
En lugar de parámetros de ciberseguridad, **el vector describe áreas de
competencia del especialista**. En lugar de clasificar
intrusión/normalidad, **el sistema clasifica la cobertura de
conocimiento del agente como suficiente para activación, insuficiente o
indeterminada**. **La imagen polar del vector**, generada por ResNet34,
**es la herramienta de auditoría visual del estado de competencia del
agente en cada momento**.

     
  -- --
     

\

**Puntos clave del documento:**

•       Ontología paramétrica de n=625=25² (b=25): 25 capas temáticas de
25 parámetros de competencia, derivadas de los descriptores MeSH
Immunology y el currículo ABIM/IUIS de inmunología clínica.

•       Arquitectura técnica: pipeline de ingestión académica (PubMed,
Semantic Scholar, arXiv) + base de datos vectorial + LLM con RAG
(Retrieval-Augmented Generation) + fine-tuning opcional con LoRA.

•       Regla del 80%: el agente se activa para producción sólo cuando
≥80% de los 625

parámetros tienen cobertura suficiente verificada en el corpus de
conocimiento.

•       Alcance y limitaciones: el documento define explícitamente qué
puede y qué no puede hacer el agente, las condiciones de supervisión
humana requeridas y los dominios en que su uso es inapropiado sin
revisión experta.

•       Generalización: la arquitectura es agnóstica al dominio. **La
sustitución del vector de competencia define un agente diferente
(ingeniero, abogado, etc.) sin cambios en la infraestructura**.

\

 

### 1.  Fundamento teórico: del parámetro de estado al parámetro de competencia {#fundamento-terico-del-parmetro-de-estado-al-parmetro-de-competencia}

##### 1.1   El framework como base formal

El framework^[1](#_bookmark0 "null")^ establece que el estado de un
sistema puede representarse como un vector de n parámetros ternarios (0,
1, U) organizados en b capas de b parámetros, con n=b² (b≥3). El vector
se transforma en una imagen polar de simetría radial uniforme que una
red neuronal convolucional (ResNet34) clasifica en tres estados:
intrusión, indeterminado o normal. La restricción n=b² no es arbitraria:
garantiza simetría geométrica y organización temática por capas, las dos
propiedades que hacen el sistema auditable visualmente.

**SVperitus **traslada esta arquitectura al dominio del conocimiento con
un cambio semántico preciso: los parámetros ya no son eventos de
ciberseguridad sino áreas de competencia del especialista. El valor 1 no
significa que un sensor ha disparado sino que el corpus de conocimiento
del agente cubre esa área con suficiencia verificable. El valor 0
significa que el área no está cubierta. El valor U significa cobertura
parcial o no determinable. La clasificación del vector no detecta
intrusión sino que determina si el agente está listo para operar en su
dominio.

 

##### 1.2   La analogía paramétrica: gen de conocimiento {#la-analoga-paramtrica-gen-de-conocimiento}

La analogía con el ADN es pertinente a nivel funcional, aunque no
biológico. En genética, un gen es una unidad de información que codifica
una función específica; su presencia, ausencia o expresión parcial
determina el fenotipo del organismo. **En SVperitus, un parámetro de
competencia es una unidad de conocimiento que codifica la capacidad del
agente para responder consultas de un área determinada**. La colección
de parámetros define el fenotipo intelectual del agente.

La diferencia crítica con el ADN es que el fenotipo genético está
determinado por la secuencia y no se modifica durante la vida del
organismo, mientras que el perfil de competencia del agente es dinámico:
se actualiza continuamente a medida que el corpus ingiere nueva
literatura. Esta propiedad es una fortaleza del sistema: el agente
mejora con el tiempo sin necesidad de redefinir su arquitectura.

 

##### 1.3   Por qué n=625 para inmunología {#por-qu-n625-para-inmunologa}

La categoría MeSH Immunology^[2](#_bookmark1 "null")^ contiene 847
descriptores activos. El currículo de inmunología clínica del
ABIM^[3](#_bookmark2 "null")^ identifica 620 áreas de conocimiento
evaluables. La ontología IUIS para inmunodeficiencias
primarias^[4](#_bookmark3 "null")^ añade 180 entidades clínicas
distintas. La convergencia de estas tres fuentes indica que la
granularidad mínima razonable para un especialista en inmunología
clínica y de investigación está entre 600 y 800 unidades de
conocimiento. El cuadrado perfecto más cercano a este rango con b entero
es n=625 (b=25), que proporciona 25 capas temáticas de 25 parámetros
cada una. Esta organización es manejable para la auditoría visual y
suficientemente granular para capturar las especializaciones dentro de
la disciplina.

\

1Lloret Egea, J.A. (2024). Framework: sistema de detección de
intrusiones en dispositivos inteligentes mediante vectores paramétricos
y redes neuronales convolucionales. DOI:
https://doi.org/10.21428/39829d0b.981b7276 2National Library of Medicine
(2024). Medical Subject Headings (MeSH): Immunology category.
https://[www.ncbi.nlm.nih.gov/mesh.](http://www.ncbi.nlm.nih.gov/mesh "null")
847 descriptores activos en la categoría D12/Immunology.

3ABIM (2024). Allergy and Immunology Certification Examination
Blueprint. American Board of Internal Medicine.
https://[www.abim.org/certification/allergy-immunology/](http://www.abim.org/certification/allergy-immunology/ "null")

4International Union of Immunological Societies (IUIS) (2022).
Phenotypical and functional criteria for the classification of inborn
errors of immunity. J Clin Immunol. DOI: 10.1007/s10875-022-01289-3.

\

 

 

### 2.  Ontología paramétrica n=625: las 25 capas de competencia {#ontologa-paramtrica-n625-las-25-capas-de-competencia}

Las 25 capas se han diseñado siguiendo tres principios: exhaustividad
(cada área relevante de la inmunología está representada),
no-redundancia (un área de conocimiento pertenece a una única capa) y
cohesión interna (los 25 parámetros de cada capa comparten el mismo
dominio de observación). Las capas se nombran con identificador C01--C25
y se mapean a los descriptores MeSH correspondientes.

 

  **ID**    **Capa temática**                                 **Paráms.**   **Descripción concisa**
  --------- ------------------------------------------------- ------------- -------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  **C01**   **Inmunología molecular de base**                 25            Biología molecular de las células del sistema inmune: genética de receptores, mecanismos de recombinación V(D)J, expresión génica diferencial y regulación epigenética.
  **C02**   **Células de la inmunidad innata**                25            Tipos celulares, ontogenia, función efectora y receptores de reconocimiento de patrón de las células de la inmunidad innata.
  **C03**   **Células de la inmunidad adaptativa**            25            Subtipos de linfocitos T y B, funciones efectoras, memoria inmunológica y regulación.
  **C04**   **Receptores de reconocimiento y señalización**   25            Receptores de antígeno, correceptores, coestimuladores y vías de señalización intracelular.
  **C05**   **Anticuerpos y sistema del complemento**         25            Estructura, clases, función efectora de inmunoglobulinas y cascadas del complemento.
  **C06**   **Citocinas, quimiocinas e inflamación**          25            Familias de citocinas, redes de regulación, receptores y papel en inflamación fisiológica y patológica.
  **C07**   **Presentación de antígenos y MHC**               25            Procesamiento y presentación antigénica, moléculas HLA, vías endógena/exógena y presentación cruzada.
  **C08**   **Desarrollo del sistema inmune**                 25            Hematopoyesis, ontogenia de células inmunes, desarrollo tímico y de células B, y maduración postnatal.
  **C09**   **Tolerancia inmunológica**                       25            Mecanismos de tolerancia central y periférica, autotolerancia, ruptura tolerogénica y aplicaciones terapéuticas.
  **C10**   **Inmunidad de mucosas**                          25            Sistema inmune de mucosas del tracto gastrointestinal, respiratorio y urogenital; eje microbiota-inmunidad.
  **C11**   **Inmunología del trasplante**                    25            Reconocimiento alogénico, rechazo de órganos, injerto de médula ósea, tolerancia y inmunosupresión.
  **C12**   **Enfermedades autoinmunes sistémicas**           25            Patogenia, diagnóstico serológico, manifestaciones clínicas y tratamiento de las principales enfermedades autoinmunes.
  **C13**   **Inmunodeficiencias primarias (IDP)**            25            Errores innatos de la inmunidad: clasificación, diagnóstico molecular y tratamiento.
  **C14**   **Inmunodeficiencias secundarias e infección**    25            Inmunodeficiencias adquiridas, infección VIH, asplenia, inmunosupresión farmacológica e infecciones oportunistas.
  **C15**   **Alergia e hipersensibilidad**                   25            Clasificación de Gell-Coombs, fisiopatología, diagnóstico y tratamiento de enfermedades alérgicas.
  **C16**   **Inmunología del tumor**                         25            Reconocimiento tumoral, evasión, inmunoedición, inmunoterapia y biomarcadores predictivos.

\

 

  **ID**    **Capa temática**                                **Paráms.**   **Descripción concisa**                                                                                                                       
  --------- ------------------------------------------------ ------------- -------------------------------------------------------------------------------------------------------------------------------------------- ---
  **C17**   **Vacunas e inmunoprofilaxis**                   25            Principios de vacunología, tipos de vacunas, adyuvantes, calendarios, seguridad y programas de inmunización.                                 
  **C18**   **Inmunomodulación terapéutica**                 25            Fármacos biológicos, pequeñas moléculas inmunomoduladoras, terapia génica y celular.                                                         
  **C19**   **Diagnóstico inmunológico de laboratorio**      25            Técnicas analíticas del laboratorio de inmunología clínica: principios, interpretación y limitaciones.                                       
  **C20**   **Bioinformática e inmunología computacional**   25            Herramientas computacionales para análisis de datos inmunológicos: TCR/BCR-seq, proteómica, predicción de epítopos y modelos de IA.          
  **C21**   **Neuroinmunología**                             25            Interacciones sistema nervioso central-sistema inmune, barreras, neuroinflamación y enfermedades neuroinflamatorias.                         
  **C22**   **Inmunología pediátrica y del desarrollo**      25            Particularidades inmunológicas del neonato, lactante, niño y adolescente; síndromes febriles periódicos y autoinflación.                     
  **C23**   **Inmunosenescencia y envejecimiento**           25            Cambios del sistema inmune con la edad, inmunosenescencia, inflamación crónica de bajo grado (inflammaging) y estrategias de intervención.   
  **C24**   **Regulación, ética y buenas prácticas**         25            Marco regulatorio de productos biológicos, ética en investigación inmunológica, acreditación y práctica basada en evidencia.                 
  **C25**   **Inmunología clínica integradora**              25            Diagnóstico diferencial de síntomas immunológicos complejos, presentaciones multisistémicas, medicina de precisión e inteligencia clínica.   

 

**Total: **25 capas × 25 parámetros = 625 parámetros de competencia.
Espacio combinacional: 2⁶²⁵

≈ 10¹⁸⁸ vectores posibles (astronomico, análogo a la diversidad del
repertorio inmune humano

real).

\

 

### 3.  Detalle representativo de cuatro capas

La lista completa de los 625 parámetros excede el alcance práctico de
este documento. Se presentan los 25 parámetros completos de cuatro capas
elegidas por su centralidad diagnóstica, su diversidad metodológica y su
relevancia para ilustrar el principio de no-redundancia entre capas.

 

 

C01 --- Inmunología molecular de base

*Biología molecular de las células del sistema inmune: genética de
receptores, mecanismos de recombinación V(D)J, expresión génica
diferencial y regulación epigenética.*

* *

+--------+-----------------------------+-----------------------------+
| **\#** | **Parámetro de              | **Herramienta de medición / |
|        | competencia**               | fuente**                    |
+========+=============================+=============================+
| 1      | Estructura génica de        | RAG →                       |
|        | inmunoglobulinas            | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 2      | Recombinación V(D)J         | RAG →                       |
|        |                             | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 3      | Maduración por afinidad     | RAG →                       |
|        |                             | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 4      | Cambio de clase de isotipo  | RAG →                       |
|        |                             | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 5      | Hipermutación somática      | RAG →                       |
|        |                             | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 6      | Edición de receptor         | RAG →                       |
|        |                             | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 7      | Regulación epigenética      | RAG →                       |
|        | linfocitaria                | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 8      | Splicing alternativo en     | RAG →                       |
|        | TCR/BCR                     | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 9      | Receptores tipo Toll (TLR): | RAG →                       |
|        | estructura y señal          | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 10     | NF-κB: vía canónica y no    | RAG →                       |
|        | canónica                    | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 11     | Vía JAK-STAT en inmunidad   | RAG →                       |
|        |                             | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 12     | Activación del inflamasoma  | RAG →                       |
|        | NLRP3                       | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 13     | cGAS-STING: inmunidad       | RAG →                       |
|        | innata antiviral            | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 14     | Autofagia e inmunidad       | RAG →                       |
|        |                             | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 15     | Metabolismo celular en      | RAG →                       |
|        | linfocitos activados        | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 16     | Mitocondria y función       | RAG →                       |
|        | efectora                    | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 17     | Exosomas e inmunología      | RAG →                       |
|        |                             | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 18     | Señalización PI3K/AKT/mTOR  | RAG →                       |
|        |                             | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 19     | p53 y apoptosis en células  | RAG →                       |
|        | inmunes                     | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 20     | Senescencia inmunológica    | RAG →                       |
|        | (molecular)                 | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+

\

* *

+--------+--------------------------+--------------------------+---+
| **\#** | **Parámetro de           | **Herramienta de         |   |
|        | competencia**            | medición / fuente**      |   |
+========+==========================+==========================+===+
| 21     | Transcriptómica de       | RAG →                    |   |
|        | célula única (scRNA-seq) | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 22     | Proteómica del secretoma | RAG →                    |   |
|        | inmune                   | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 23     | CRISPR aplicado a genes  | RAG →                    |   |
|        | inmunes                  | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 24     | Edición de ARN en        | RAG →                    |   |
|        | inmunología              | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 25     | Base de datos de         | RAG →                    |   |
|        | estructuras de           | P                        |   |
|        | anticuerpos (SAbDab)     | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+

* *

* *

C06 --- Citocinas, quimiocinas e inflamación

*Familias de citocinas, redes de regulación, receptores y papel en
inflamación fisiológica y patológica.*

* *

+--------+-----------------------------+-----------------------------+
| **\#** | **Parámetro de              | **Herramienta de medición / |
|        | competencia**               | fuente**                    |
+========+=============================+=============================+
| 1      | Familia IL-1: IL-1α, IL-1β, | RAG →                       |
|        | IL-18, IL-33, IL-36         | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 2      | Familia IL-6: señalización  | RAG →                       |
|        | trans y cis                 | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 3      | Familia IL-10: función      | RAG →                       |
|        | antiinflamatoria            | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 4      | Familia IL-12: polarización | RAG →                       |
|        | Th1/Th17                    | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 5      | Familia IL-17: IL-17A a     | RAG →                       |
|        | IL-17F                      | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 6      | Familia IL-2: proliferación | RAG →                       |
|        | y Treg                      | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 7      | TNF-α: vías de señalización | RAG →                       |
|        | y apoptosis                 | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 8      | IFN-γ: activación           | RAG →                       |
|        | macrofágica                 | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 9      | Interferones tipo I:        | RAG →                       |
|        | regulación y función        | PubMed/IEDB/UniProt/bases   |
|        | antiviral                   | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 10     | TGF-β: fibrosis, Treg e     | RAG →                       |
|        | inmunidad                   | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 11     | IL-4 e IL-13: polarización  | RAG →                       |
|        | Th2                         | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 12     | IL-5 en eosinofilopoyesis   | RAG →                       |
|        |                             | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 13     | Quimiocinas CC: CCL2, CCL5, | RAG →                       |
|        | CCL11, CCL17                | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 14     | Quimiocinas CXC: CXCL8,     | RAG →                       |
|        | CXCL10, CXCL12              | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 15     | Receptores de quimiocinas y | RAG →                       |
|        | homing linfocitario         | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 16     | IL-23 y mantenimiento de    | RAG →                       |
|        | Th17                        | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 17     | BAFF y APRIL: supervivencia | RAG →                       |
|        | de células B                | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+

\

* *

+--------+--------------------------+--------------------------+---+
| **\#** | **Parámetro de           | **Herramienta de         |   |
|        | competencia**            | medición / fuente**      |   |
+========+==========================+==========================+===+
| 18     | TSLP en alergia e        | RAG →                    |   |
|        | inflamación epitelial    | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 19     | IL-25 (IL-17E) en        | RAG →                    |   |
|        | inmunidad tipo 2         | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 20     | Tormenta de citocinas:   | RAG →                    |   |
|        | patogenia y criterios    | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 21     | HMGB1 como DAMP          | RAG →                    |   |
|        |                          | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 22     | Prostaglandinas y        | RAG →                    |   |
|        | leucotrienos en          | P                        |   |
|        | inflamación              | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 23     | Resolinas y protectinas: | RAG →                    |   |
|        | resolución               | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 24     | Cascada de coagulación e | RAG →                    |   |
|        | inmunología              | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 25     | Cininas y anafilotoxinas | RAG →                    |   |
|        | C3a/C5a                  | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+

* *

* *

C12 --- Enfermedades autoinmunes sistémicas

*Patogenia, diagnóstico serológico, manifestaciones clínicas y
tratamiento de las principales enfermedades autoinmunes.*

* *

+--------+-----------------------------+-----------------------------+
| **\#** | **Parámetro de              | **Herramienta de medición / |
|        | competencia**               | fuente**                    |
+========+=============================+=============================+
| 1      | Lupus eritematoso sistémico | RAG →                       |
|        | (LES): criterios EULAR/ACR  | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 2      | Anti-dsDNA, anti-Sm:        | RAG →                       |
|        | especificidad en LES        | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 3      | Nefritis lúpica:            | RAG →                       |
|        | clasificación ISN/RPS       | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 4      | Síndrome antifosfolípido:   | RAG →                       |
|        | primario y secundario       | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 5      | Artritis reumatoide:        | RAG →                       |
|        | patogenia de la sinovitis   | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 6      | Factor reumatoide y         | RAG →                       |
|        | anti-CCP                    | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 7      | Síndrome de Sjögren:        | RAG →                       |
|        | anti-Ro/SSA y anti-La/SSB   | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 8      | Esclerodermia: anti-Scl-70  | RAG →                       |
|        | y anticentrómero            | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 9      | Polimiositis y              | RAG →                       |
|        | dermatomiositis:            | PubMed/IEDB/UniProt/bases   |
|        | anticuerpos anti- ARNt      | de datos                    |
|        | sintetasa                   |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 10     | Síndrome de solapamiento    | RAG →                       |
|        | (overlap)                   | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 11     | Enfermedad mixta del tejido | RAG →                       |
|        | conjuntivo (EMTC)           | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 12     | Vasculitis ANCA+ (GPA,      | RAG →                       |
|        | EGPA, PAM)                  | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 13     | Vasculitis de grandes vasos | RAG →                       |
|        | (Takayasu, ACG)             | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 14     | Poliarteritis nodosa:       | RAG →                       |
|        | patogenia                   | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+

\

* *

+--------+--------------------------+--------------------------+---+
| **\#** | **Parámetro de           | **Herramienta de         |   |
|        | competencia**            | medición / fuente**      |   |
+========+==========================+==========================+===+
| 15     | Espondiloartropatías:    | RAG →                    |   |
|        | HLA-B27 y patogenia      | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 16     | Artritis psoriásica: eje | RAG →                    |   |
|        | IL-17/IL-23              | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 17     | Enfermedad inflamatoria  | RAG →                    |   |
|        | intestinal: Crohn y CU   | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 18     | Hepatitis autoinmune:    | RAG →                    |   |
|        | anti-ANA, anti-LKM       | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 19     | Cirrosis biliar          | RAG →                    |   |
|        | primaria: anti-AMA       | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 20     | Tiroiditis de Hashimoto  | RAG →                    |   |
|        |                          | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 21     | Enfermedad de Graves:    | RAG →                    |   |
|        | anticuerpos anti-TSH-R   | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 22     | Síndrome de Goodpasture: | RAG →                    |   |
|        | anti-GBM                 | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 23     | Encefalitis autoinmune:  | RAG →                    |   |
|        | anti-NMDAR               | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 24     | IgG4-RD: criterios y     | RAG →                    |   |
|        | diagnóstico              | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 25     | VEXAS syndrome: UBA1 y   | RAG →                    |   |
|        | autoinflación            | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+

* *

* *

C16 --- Inmunología del tumor

*Reconocimiento tumoral, evasión, inmunoedición, inmunoterapia y
biomarcadores predictivos.*

* *

+--------+-----------------------------+-----------------------------+
| **\#** | **Parámetro de              | **Herramienta de medición / |
|        | competencia**               | fuente**                    |
+========+=============================+=============================+
| 1      | Hipótesis de la vigilancia  | RAG →                       |
|        | inmunológica                | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 2      | Teoría de la inmunoedición: | RAG →                       |
|        | el                          | PubMed/IEDB/UniProt/bases   |
|        | iminación/equilibrio/escape | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 3      | Neoantígenos: carga         | RAG →                       |
|        | mutacional tumoral (TMB)    | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 4      | TILs: pronóstico en cáncer  | RAG →                       |
|        | de mama y melanoma          | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 5      | PD-1/PD-L1: expresión en    | RAG →                       |
|        | tumor y TIL                 | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 6      | CTLA-4 en el contexto       | RAG →                       |
|        | tumoral                     | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 7      | Microambiente tumoral       | RAG →                       |
|        | (TME): células y citocinas  | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 8      | CAF y remodelado del        | RAG →                       |
|        | estroma                     | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 9      | MDSCs en tumor: mecanismos  | RAG →                       |
|        | de supresión                | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 10     | TAMs: polarización M2 en    | RAG →                       |
|        | tumor                       | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+
| 11     | NK en vigilancia            | RAG →                       |
|        | antitumoral                 | PubMed/IEDB/UniProt/bases   |
|        |                             | de datos                    |
|        |                             |                             |
|        |                             | especializadas              |
+--------+-----------------------------+-----------------------------+

\

* *

+--------+--------------------------+--------------------------+---+
| **\#** | **Parámetro de           | **Herramienta de         |   |
|        | competencia**            | medición / fuente**      |   |
+========+==========================+==========================+===+
| 12     | NKG2D y sus ligandos en  | RAG →                    |   |
|        | tumor                    | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 13     | Inmunoterapia:           | RAG →                    |   |
|        | inhibidores de           | P                        |   |
|        | checkpoint (ICI)         | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 14     | Anti-PD-1: nivolumab,    | RAG →                    |   |
|        | pembrolizumab            | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 15     | Anti-PD-L1:              | RAG →                    |   |
|        | atezolizumab, durvalumab | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 16     | Anti-CTLA-4: ipilimumab  | RAG →                    |   |
|        |                          | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 17     | Toxicidades              | RAG →                    |   |
|        | inmunomediadas (irAE):   | P                        |   |
|        | clasificación            | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 18     | Manejo de irAE:          | RAG →                    |   |
|        | corticoides e infliximab | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 19     | CAR-T cells: diseño y    | RAG →                    |   |
|        | generaciones             | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 20     | Terapia con linfocitos   | RAG →                    |   |
|        | TIL adoptivos            | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 21     | Anticuerpos              | RAG →                    |   |
|        | biespecíficos:           | P                        |   |
|        | blinatumomab             | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 22     | ADC (anticuerpos         | RAG →                    |   |
|        | conjugados a fármaco)    | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 23     | Vacunas terapéuticas     | RAG →                    |   |
|        | contra el cáncer         | P                        |   |
|        |                          | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 24     | Biomarcadores            | RAG →                    |   |
|        | predictivos de respuesta | P                        |   |
|        | a ICI                    | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+
| 25     | Resistencia primaria y   | RAG →                    |   |
|        | adquirida a              | P                        |   |
|        | inmunoterapia            | ubMed/IEDB/UniProt/bases |   |
|        |                          | de datos                 |   |
|        |                          |                          |   |
|        |                          | especializadas           |   |
+--------+--------------------------+--------------------------+---+

\

* *

### 4.  Arquitectura técnica del sistema {#arquitectura-tcnica-del-sistema}

##### 4.1   Visión general {#visin-general}

El sistema se estructura en cuatro capas que interactúan de forma
secuencial y en bucle continuo:

\(1\) Definición paramétrica, (2) Adquisición de conocimiento, (3)
Agente de respuesta, y (4) Auditoría de cobertura. El diseño prioriza la
auditabilidad sobre la eficiencia: en todo momento debe ser posible
responder a la pregunta de qué sabe el agente, sobre qué fuentes lo sabe
y con qué nivel de cobertura.

 

  **Capa**          **Función**                                                                                              **Tecnología recomendada**
  ----------------- -------------------------------------------------------------------------------------------------------- -----------------------------------------------------------------------------------
  1 · Definición    Ontología de 625 parámetros; vector de cobertura; imagen polar de auditoría                              YAML/JSON de ontología + Python + matplotlib/ResNet34
  2 · Adquisición   Ingestión académica continua; filtrado por calidad; chunking semántico; generación de embeddings         PubMed API + Semantic Scholar API + LangChain/LlamaIndex + text-embedding-3-large
  3 · Agente        LLM base + RAG sobre corpus específico + fine-tuning opcional; respuesta al usuario                      Llama 3.1 70B / GPT-4 + Weaviate/pgvector + LoRA fine-tuning
  4 · Auditoría     Actualización del vector de cobertura; generación de imagen polar; activación/desactivación del agente   ResNet34 clasificador + dashboard de cobertura + alertas por parámetro

 

 

 

##### 4.2   Capa de adquisición de conocimiento {#capa-de-adquisicin-de-conocimiento}

La adquisición se realiza mediante tres fuentes primarias de acceso
abierto. PubMed/MEDLINE^[5](#_bookmark4 "null")^ proporciona acceso a
\>36 millones de artículos biomédicos indexados con un sistema de
control de calidad editorial consolidado. Semantic
Scholar^[6](#_bookmark5 "null")^ amplía la cobertura a \>200 millones de
artículos con acceso a full-text en un 40-60% de los casos a través de
Unpaywall. ArXiv es la fuente de preprints para los parámetros de la
Capa C20 (bioinformática e inmunología computacional).

El filtrado de calidad aplica tres criterios en cascada: factor de
impacto de la revista (SJR \> 0.5 para artículos generales, SJR \> 1.0
para guías clínicas), fecha de publicación (preferencia por los últimos
10 años, con excepción de artículos fundacionales con \>500 citas) e
indexación en bases de datos de referencia (MEDLINE, EMBASE, Cochrane
para estudios clínicos; GenBank, UniProt para datos moleculares). Los
artículos que superan el filtro se someten a chunking semántico: se
dividen en fragmentos de 512 tokens con superposición de 64 tokens,
etiquetados con los identificadores de los parámetros de las capas
C01--C25 que cubren.

 

 

 

 

 

\

5NCBI (2024). PubMed/MEDLINE API.
https://[www.ncbi.nlm.nih.gov/home/develop/api/.](http://www.ncbi.nlm.nih.gov/home/develop/api/ "null")
Acceso libre a \>36 millones de referencias biomédicas.

6Amini, E.B. et al. (2023). Semantic Scholar Open Research Corpus
(S2ORC). Semantic Scholar API documentation. 200 million+ papers.
https://api.semanticscholar.org

\

 

##### 4.3   Capa de representación vectorial {#capa-de-representacin-vectorial}

Cada chunk se convierte en un vector de embeddings de 3.072 dimensiones
usando el modelo text-embedding-3-large de
OpenAI^[7](#_bookmark6 "null")^ o, en despliegue local, el modelo
nomic-embed-text-v1.5 (768 dimensiones, rendimiento comparable). Los
vectores se almacenan en una base de datos vectorial con soporte para
búsqueda híbrida semántica y léxica. FAISS^[8](#_bookmark7 "null")^ es
la opción recomendada para despliegue local de alto rendimiento;
Weaviate^[9](#_bookmark8 "null")^ para despliegue en nube con
persistencia y actualización incremental.

La cobertura de cada parámetro se estima como el recuento de chunks que
(a) superan un umbral de similitud coseno ≥0.82 con la consulta de
referencia del parámetro y (b) han superado el filtro de calidad
editorial. Un parámetro con ≥50 chunks de cobertura verificada recibe
valor 1; entre 10 y 49 chunks recibe valor U; con menos de 10 recibe
valor 0. Estos umbrales son provisionales y están sujetos a calibración
experimental.

 

##### 4.4   Capa del agente: LLM + RAG

El agente se construye sobre un LLM base con capacidades médicas
demostradas. Las opciones más relevantes son Llama 3.1
70B^[10](#_bookmark9 "null")^, GPT-4o^[11](#_bookmark10 "null")^ y
Med-PaLM 2^[12](#_bookmark11 "null")^. El mecanismo RAG (Retrieval-
Augmented Generation)^[13](#_bookmark12 "null")^ recupera en tiempo de
inferencia los k=8 chunks más relevantes del corpus para la consulta del
usuario, los incluye en el contexto del LLM y los cita en la respuesta.
Esto garantiza que las respuestas del agente sean verificables en la
literatura de referencia, lo que es condición necesaria para el uso en
entornos académicos y clínicos.

El fine-tuning sobre el corpus de inmunología se realiza mediante LoRA
(Low-Rank Adaptation)^[14](#_bookmark13 "null")^, que adapta el LLM base
con un coste computacional reducido (entre 2 y 4 GPUs A100 durante
24--72 horas para 70B parámetros). El fine-tuning no es obligatorio en
la primera fase del despliegue: un LLM base con RAG bien configurado
puede alcanzar rendimiento suficiente sin él. Se recomienda para la fase
de producción plena.

 

##### 4.5   Capa de auditoría: imagen polar SVperitus {#capa-de-auditora-imagen-polar-svperitus}

La imagen polar de cobertura es el elemento que conecta SVperitus con el
framework original. Se genera a partir del vector de cobertura de los
625 parámetros y sigue la misma transformación: cada parámetro ocupa un
eje a 360°/625 ≈ 0,576° de separación. Los valores 0, 1 y U se mapean a
radios 1, 2 y 3 respectivamente. La imagen resultante es un polígono de
625 vértices que ResNet34 clasifica en tres estados: cobertura
suficiente (activación permitida), cobertura insuficiente (agente en
modo de aprendizaje) e indeterminada (requiere revisión parcial).

Con 625 ejes, la imagen polar no es legible eje a eje, pero sí lo es por
sectores: cada grupo de 25 ejes consecutivos corresponde a una capa
C01--C25, y la forma del sector (alto = cobertura, bajo

= ausencia) permite identificar de un vistazo qué capas están cubiertas
y cuáles no. Esta propiedad de lectura por capas es la que justifica la
restricción n=b² con b capas de b parámetros.

\

7OpenAI (2024). text-embedding-3-large model card.
https://platform.openai.com/docs/guides/embeddings. 3072 dimensiones,
rendimiento SOTA en MTEB benchmark.

8Johnson, J. et al. (2021). Billion-scale similarity search with GPUs.
IEEE Transactions on Big Data, 7(3), 535-547. FAISS:
https://github.com/facebookresearch/faiss.

9Anderman, N. & Weaviate Team (2024). Weaviate: Open-source vector
database. https://weaviate.io. Soporta HNSW, BM25 y búsqueda híbrida
semántica+léxica.

10Touvron, H. et al. (2023). Llama 2: Open Foundation and Fine-Tuned
Chat Models. Meta AI. arXiv:2307.09288.

11Nori, H. et al. (2023). Can Generalist Foundation Models Outcompete
Special-Purpose Tuning? Case Study in Medicine. Microsoft Research.
arXiv:2311.16452.

12Singhal, K. et al. (2023). Large language models encode clinical
knowledge. Nature, 620, 172--180. DOI: 10.1038/s41586-023-06291-2.

13Lewis, P. et al. (2020). Retrieval-Augmented Generation for
Knowledge-Intensive NLP Tasks. Advances in Neural Information Processing
Systems 33. arXiv:2005.11401.

14Hu, E.J. et al. (2022). LoRA: Low-Rank Adaptation of Large Language
Models. ICLR 2022. arXiv:2106.09685.

\

 

### 5.  La regla del 80%: activación, auditoría y supervisión {#la-regla-del-80-activacin-auditora-y-supervisin}

##### 5.1   Definición formal {#definicin-formal}

     
  -- --
     

\

** **

La condición dual (cobertura global ≥80% Y cobertura de ≥20/25 capas)
evita el caso de un agente

que cubre el 80% global concentrado en solo algunas capas y tiene
lagunas completas en otras. Un agente que supera el 80% global pero
falla en toda la capa C13 (inmunodeficiencias primarias) no debería
declararse operativo para consultas sobre IDP aunque su cobertura global
sea alta.

 

##### 5.2   Umbrales de cobertura por estado

** **

+----------------------+----------------------+----------------------+
| **Estado del         | **Condición**        | **Consecuencia       |
| agente**             |                      | operativa**          |
+======================+======================+======================+
| OPERATIVO            | ≥500/625 parámetros  | Consultas académicas |
|                      | en 1 Y ≥20 capas con | supervisadas         |
|                      | ≥80% cobertura       | permitidas           |
+----------------------+----------------------+----------------------+
| EN APRENDIZAJE       | 375--499 parámetros  | Solo consultas       |
|                      | en 1 O entre 15 y    | internas de          |
|                      |                      | validación           |
|                      | 19 capas con ≥80%    |                      |
+----------------------+----------------------+----------------------+
| NO OPERATIVO         | \<375 parámetros en  | Desactivado;         |
|                      | 1 O \<15 capas       | ingestión continua   |
|                      |                      | activa               |
|                      | con ≥80%             |                      |
+----------------------+----------------------+----------------------+

** **

Los umbrales cuantitativos (500/625, 375/625, 20/25 capas) son
provisionales. Su calibración definitiva requiere experimentos de
evaluación que correlacionen el valor del vector de cobertura con la
tasa de error del agente en benchmarks de inmunología clínica. El
benchmark recomendado para calibración es el conjunto de preguntas del
examen de certificación ABIM en Alergia e Inmunología, disponible en
dominio público, complementado con casos clínicos estructurados.

 

##### 5.3   Proceso de auditoría periódica {#proceso-de-auditora-peridica}

La auditoría se realiza con una frecuencia mínima semanal en la fase de
aprendizaje y mensual en la fase operativa. El proceso genera tres
artefactos: (1) la imagen polar actualizada del vector de cobertura, (2)
una tabla de cobertura por capa con los parámetros en estado 0 o U
identificados para ingestión prioritaria, y (3) un informe de drift de
cobertura que detecta parámetros que han perdido cobertura (transición
1→U→0) por obsolescencia de las fuentes.

La auditabilidad del sistema es un requisito no negociable justificado
por el principio de IA explicable^[15](#_bookmark14 "null")^. En ninguna
circunstancia debe el agente operar como caja negra: toda respuesta debe
poder trazarse hasta los chunks fuente y hasta los parámetros que los
sustentan.

 

 

 

 

 

\

15Doshi-Velez, F. & Kim, B. (2017). Towards a rigorous science of
interpretable machine learning. arXiv:1702.08608. Marco para la
auditabilidad de sistemas de IA.

\

 

### 6.  Alcance, limitaciones y condiciones de uso

##### 6.1   Lo que el agente puede hacer

•       Responder consultas de inmunología clínica y de investigación a
nivel de especialista, con cita de fuentes verificadas de literatura
académica indexada.

•       Generar resúmenes actualizados sobre cualquiera de los 625
parámetros, indicando la fecha de la literatura más reciente en el
corpus.

•       Asistir en el diagnóstico diferencial de enfermedades
inmunológicas complejas, proporcionando los criterios de clasificación
internacionales (EULAR/ACR/IUIS) con sus referencias.

•       Apoyar la interpretación de pruebas de laboratorio inmunológico
en el contexto clínico proporcionado por el usuario.

•       Generar preguntas de autoevaluación sobre cualquier parámetro o
capa de la ontología, con respuestas justificadas y referencias.

•       Actualizar su cobertura de conocimiento de forma autónoma
mediante ingestión continua de nuevas publicaciones académicas.

 

 

##### 6.2   Lo que el agente no puede hacer

     
  -- --
     

\

** **

•       El agente no tiene acceso a datos clínicos del paciente salvo
los proporcionados explícitamente en la consulta. No accede a historias
clínicas electrónicas ni a sistemas hospitalarios.

•       El agente no puede razonar sobre situaciones que requieren
exploración física, inspección visual directa o procedimientos
diagnósticos no documentables en texto.

•       El agente puede alucinar (generar afirmaciones plausibles pero
incorrectas) en los parámetros con cobertura U o 0 en el vector. El
usuario debe verificar las respuestas sobre parámetros fuera de
cobertura con fuentes primarias.

•       La calidad de las respuestas sobre desarrollos publicados
después de la última actualización del corpus es inferior. La fecha del
chunk más reciente para cada parámetro está disponible en el dashboard
de auditoría.

•       El agente no realiza metaanálisis ni síntesis sistemática de
evidencia. Recupera y parafrasea literatura existente; no genera
evidencia nueva.

 

 

##### 6.3   Condiciones de supervisión humana requeridas {#condiciones-de-supervisin-humana-requeridas}

Cualquier uso del agente en entornos académicos o clínicos requiere
supervisión humana activa^[16](#_bookmark15 "null")^. Se definen tres
niveles de supervisión:

 

\

16Zhan, C. & Miller, M.R. (2003). Administrative data based patient
safety research: a critical review. Quality and Safety in Health Care,
12 (suppl 2), ii58-ii63. DOI: 10.1136/qhc.12.suppl_2.ii58. Referencia
sobre limitaciones de sistemas automáticos de decisión clínica.

\

 

  **Nivel**                  **Contexto de uso**                                            **Requisito de supervisión**                                              
  -------------------------- -------------------------------------------------------------- ------------------------------------------------------------------------ ---
  S1 · Investigación         Revisión de literatura, resúmenes, preguntas de conocimiento   Verificación ocasional por investigador con formación en inmunología     
  S2 · Docencia              Soporte a la enseñanza de inmunología en grado/postgrado       Revisión por docente especialista antes de uso con alumnos               
  S3 · Clínico supervisado   Apoyo a razonamiento diagnóstico de residente o junior         Revisión obligatoria por especialista senior. Nunca como fuente única.   
  S4 · Clínico directo       Decisión terapéutica o diagnóstica sobre paciente real         PROHIBIDO sin supervisión de especialista titulado. Uso no permitido.    

 

 

### 7.  Hoja de ruta hacia producción {#hoja-de-ruta-hacia-produccin}

##### Fase 0 --- Validación conceptual (mes 1--2) {#fase-0-validacin-conceptual-mes-12}

Definir y revisar la ontología completa de los 625 parámetros con
revisión por al menos dos inmunólogos clínicos senior y un inmunólogo de
investigación. Establecer las consultas de referencia para medir la
cobertura de cada parámetro. Definir el benchmark de evaluación
(preguntas ABIM + casos clínicos estructurados). Este es el paso que no
puede omitirse: la calidad del agente depende directamente de la calidad
de la ontología paramétrica.

 

##### Fase 1 --- Prototipo RAG sin fine-tuning (mes 3--5)

Construir el pipeline de ingestión con PubMed y Semantic Scholar.
Ingerir el corpus de los últimos 10 años para las 25 capas. Desplegar el
agente RAG sobre Llama 3.1 70B o GPT-4o sin fine- tuning. Medir la
cobertura inicial del vector y el rendimiento en el benchmark. Esperar
cobertura inicial del 40--55% del vector, con rendimiento en benchmark
del 55--65%. Publicar resultados de la fase para revisión académica.

 

##### Fase 2 --- Fine-tuning y calibración (mes 6--9) {#fase-2-fine-tuning-y-calibracin-mes-69}

Aplicar LoRA fine-tuning sobre el corpus filtrado. Calibrar los umbrales
de cobertura (50 chunks/parámetro para valor 1) correlacionándolos con
el rendimiento en benchmark. Incorporar la retroalimentación del panel
revisor sobre calidad de las respuestas. Esperar cobertura del 65--75%
del vector y rendimiento en benchmark del 70--80%, comparable a
residente de inmunología en años 2--3.

 

##### Fase 3 --- Cobertura operativa (mes 10--18)

Alcanzar la condición de activación del 80% mediante ingestión ampliada.
Validar el agente con un panel de especialistas sobre un conjunto de 200
casos clínicos complejos. Publicar los resultados de validación en
revista indexada de inmunología o de inteligencia artificial aplicada a
medicina.

Desplegar la versión operativa supervisada (niveles S1 y S2) en un
entorno académico controlado. Producción plena: despliegue en plataforma
con API pública, dashboard de auditoría accesible, actualizaciones
mensuales de corpus, y ciclo trimestral de revisión de la ontología
paramétrica.

\

 

### 8.  Generalización del sistema a otros agentes especialistas {#generalizacin-del-sistema-a-otros-agentes-especialistas}

##### 8.1   Principio de independencia de dominio

La arquitectura de SVperitus es completamente agnóstica al dominio del
especialista. Los únicos componentes que cambian al crear un nuevo
agente son: la ontología paramétrica (vector de n=b² parámetros de
competencia del nuevo dominio), el corpus de ingestión (fuentes
académicas del nuevo dominio) y las consultas de referencia para medir
la cobertura de cada parámetro. El pipeline de embeddings, la base de
datos vectorial, el LLM base, el mecanismo RAG, la regla de activación y
el sistema de imagen polar son invariantes.

 

##### 8.2   Ejemplos de instancias posibles

** **

+----------------+----------------+----------------+----------------+
| **Agente**     | **n=b²         | **Fuente de    | *              |
|                | propuesto**    | ontología**    | *Observación** |
+================+================+================+================+
| Ingeniero      | n=529 (23²)    | Eurocódigos +  | 25 capas:      |
| estructural    |                | normativa ISO  | materiales,    |
|                |                | 9001           | cálculo,       |
|                |                |                | sísmica,       |
|                |                |                | normativa,     |
|                |                |                | etc.           |
+----------------+----------------+----------------+----------------+
| Abogado        | n=576 (24²)    | Tesauro        | 24 capas:      |
| mercantilista  |                | jurídico       | contratos,     |
|                |                | EUR-Lex +      | sociedades,    |
|                |                | CENDOJ         | insolvencia,   |
|                |                |                | PI, UE\...     |
+----------------+----------------+----------------+----------------+
| Cardiólogo     | n=625 (25²)    | MeSH           | Misma escala   |
| clínico        |                |                | que            |
|                |                | Cardiology +   | inmunólogo;    |
|                |                | ESC Guidelines | n=25²          |
|                |                |                | coherente      |
+----------------+----------------+----------------+----------------+
| Físico de      | n=484 (22²)    | INSPIRE-HEP +  | 22 capas: QFT, |
| partículas     |                |                | SM, BSM,       |
|                |                | PDG Review     | detectores,    |
|                |                |                | análisis       |
+----------------+----------------+----------------+----------------+
| Maestro de     | n=196 (14²)    | LOMLOE +       | 14 capas:      |
| primaria       |                |                | didáctica,     |
|                |                | Competencias   | psicología,    |
|                |                | MEFP           | materias, etc. |
+----------------+----------------+----------------+----------------+

** **

La imagen polar de cada agente es directamente comparable con la de
cualquier otro agente de la misma escala: un inmunólogo y un cardiólogo,
ambos en n=625, producen imágenes polares de la misma geometría que
pueden superponerse para comparar perfiles de competencia. Esta
propiedad es útil para la evaluación comparativa de agentes y para la
detección de solapamientos de conocimiento entre especialidades.

 

##### 8.3   El vector como pasaporte de competencia

El vector de cobertura de un agente SVperitus es, en última instancia,
un pasaporte formal de su competencia: una representación verificable,
auditable y actualizable de qué sabe el agente y en qué grado. Esta
propiedad tiene implicaciones más allá de los sistemas de IA: el mismo
framework podría aplicarse para definir y auditar el perfil de
competencia de un profesional humano en formación, representando en el
vector no la cobertura del corpus del agente sino la cobertura del
conocimiento del estudiante según su historial de evaluaciones.

\

 

### 9.  Referencias bibliográficas {#referencias-bibliogrficas}

\[1\]  Lloret Egea, J.A. (2024). Framework: sistema de detección de
intrusiones en dispositivos inteligentes mediante vectores paramétricos
y redes neuronales convolucionales. DOI:
https://doi.org/10.21428/39829d0b.981b7276

\[2\]  Lewis, P. et al. (2020). Retrieval-Augmented Generation for
Knowledge-Intensive NLP Tasks. NeurIPS 2020. arXiv:2005.11401.

\[3\]  National Library of Medicine (2024). Medical Subject Headings
(MeSH): Immunology.
https://[www.ncbi.nlm.nih.gov/mesh](http://www.ncbi.nlm.nih.gov/mesh "null")

\[4\]  IUIS Expert Committee (2022). Human inborn errors of immunity:
2022 update on the classification from the International Union of
Immunological Societies. Journal of Clinical Immunology. DOI:
10.1007/s10875-022-01289-3.

\[5\]  Touvron, H. et al. (2023). Llama 2: Open Foundation and
Fine-Tuned Chat Models. Meta AI. arXiv:2307.09288.

\[6\]  OpenAI (2024). GPT-4 Technical Report.
https://openai.com/research/gpt-4

\[7\]  Singhal, K. et al. (2023). Large language models encode clinical
knowledge. Nature, 620, 172--

180\. DOI: 10.1038/s41586-023-06291-2.

\[8\]  Hu, E.J. et al. (2022). LoRA: Low-Rank Adaptation of Large
Language Models. ICLR 2022. arXiv:2106.09685.

\[9\]  NCBI (2024). PubMed E-utilities API.
https://[www.ncbi.nlm.nih.gov/home/develop/api/](http://www.ncbi.nlm.nih.gov/home/develop/api/ "null")

\[10\]  Lo, K. et al. (2020). S2ORC: The Semantic Scholar Open Research
Corpus. ACL 2020. arXiv:1911.02782.

\[11\]  OpenAI (2024). text-embedding-3-large model card.
https://platform.openai.com/docs/guides/embeddings

\[12\]  Johnson, J., Douze, M. & Jégou, H. (2021). Billion-scale
similarity search with GPUs. IEEE Transactions on Big Data, 7(3).
arXiv:1702.08734.

\[13\]  Weaviate (2024). Weaviate Vector Database Documentation.
https://weaviate.io/developers/weaviate

\[14\]  ABIM (2024). Allergy and Immunology Certification Examination
Blueprint.
https://[www.abim.org/certification/allergy-immunology/](http://www.abim.org/certification/allergy-immunology/ "null")

\[15\]  Zhan, C. & Miller, M.R. (2003). Administrative data based
patient safety research: a critical review. Quality and Safety in Health
Care, 12(suppl 2).

\[16\]  Doshi-Velez, F. & Kim, B. (2017). Towards a rigorous science of
interpretable machine learning. arXiv:1702.08608.

\[17\]  Radford, A. et al. (2021). Learning Transferable Visual Models
From Natural Language Supervision (CLIP). ICML 2021. arXiv:2103.00020.

\[18\]  He, K. et al. (2016). Deep Residual Learning for Image
Recognition. CVPR 2016. DOI: 10.1109/CVPR.2016.90. \[ResNet34\]

\[19\]  Vaswani, A. et al. (2017). Attention Is All You Need. NeurIPS
2017. arXiv:1706.03762. \[Arquitectura Transformer base de los LLMs\]

\[20\]  Aletaha, D. & Smolen, J.S. (2018). Diagnosis and Management of
Rheumatoid Arthritis: A Review. JAMA, 320(13). DOI:
10.1001/jama.2018.13103.

\

 

**Apéndice --- Comparación entre Framework original y SVperitus**

** **

** **

  **Dimensión**               **Framework (ciberseguridad)**                      **SVperitus (conocimiento experto)**
  --------------------------- --------------------------------------------------- ----------------------------------------------------
  Dominio de aplicación       Detección de intrusiones en dispositivos            Perfil de competencia de agente IA especialista
  Naturaleza del parámetro    Evento de seguridad observable (binario/ternario)   Área de competencia cognitiva (binario/ternario)
  Valor 1                     Parámetro de intrusión activo                       Área cubierta en corpus con suficiencia verificada
  Valor 0                     Parámetro normal                                    Área no cubierta en corpus
  Valor U                     No determinable / indisponible                      Cobertura parcial o no medible
  Clasificación CNN           Intrusión / Indeterminado / Normal                  Suficiente / Indeterminado / Insuficiente
  Escala (n=b²)               n=9 a n=49 documentado                              n=625 (b=25) para inmunólogo
  Dinámica del vector         Estado instantáneo del dispositivo                  Estado acumulativo del corpus (crece con tiempo)
  Uso de la imagen polar      Clasificación de estado de seguridad                Auditoría visual de cobertura de conocimiento
  Regla de activación         Umbral de alto peso activos para alerta             Regla del 80%: umbral de cobertura para operar
  Invariante arquitectónico   n=b², simetría polar, ResNet34, ternario            Idéntico: n=b², simetría polar, ResNet34, ternario

 

[^1]: Para no perjudicar al idioma español, se usará en adelante de
    forma indistinta la palabra «Framework» inglesa y la palabra «Marco»
    del idioma español.

[^2]: «La inteligencia artificial, AI (del inglés, *artificial
    intelligence*), es realmente el campo más prometedor, que ofrece una
    serie de ámbitos en los que se han utilizado sistemas de
    multivaluados, MVL (del inglés, *multi-valued logics*) debido a su
    posibilidad para abarcar un rango más amplio de estados.

    Una primera área de aplicación la encontramos muy relacionada con la
    posibilidad de representar preocupaciones, sentido común,
    razonamientos, etc. por medio herramientas matemáticas como
    conjuntos difusos y lógica difusa (del inglés, *fuzzy logic*).

    Una segunda área de aplicación, relacionada con la anterior viene
    dada por la automatización de datos y minería de conocimiento. Aquí,
    merecen especial mención los métodos de *clustering* (procedimiento
    basado en la unión de vectores que reúnen una determinada
    característica). En este contexto también se están desarrollando
    técnicas que automatizen sistemas MVL, así como en los métodos de la
    lógica de programación para dichos sistemas. Parte de esta tendencia
    se debe al desarrollo reciente de las lógicas de descripción
    generalizada, llamadas lógicas de descripción difusa, que permiten
    la inserción de herramientas técnicas (grados de verdad, conectivas,
    predicados graduales) procedentes de MVL.». Lógica trivalente.
    (2021, 22 de mayo). *Wikipedia, La enciclopedia libre*. Fecha de
    consulta: 14:58, julio 14, 2021
    desde <https://es.wikipedia.org/w/index.php?title=L%C3%B3gica_trivalente&oldid=135730214>.

[^3]: **Clasificación del SVintrusión: criterio, método y resultados**

    El framework define nueve parámetros de comportamiento. Cada uno
    vale 0 (inactivo), 1 (activo) o U (indeterminado).

    No todos pesan igual. P5 (Bluetooth) y P9 (carga física) se activan
    en uso legítimo: auriculares, ejercicio. Incluirlos inflaría los
    falsos positivos. Los siete restantes ---P1, P2, P3, P4, P6, P7,
    P8--- describen un perfil de spyware: exfiltración por canal
    inseguro, red no autorizada, rastreo GPS y vigilancia audiovisual.
    Su activación simultánea no tiene explicación legítima.

    Regla adoptada: siete o más activos → intrusión. Entre cinco y seis
    → indeterminado. Menos de cinco → normal.

    Sobre 2⁹ = 512 vectores posibles: 46 intrusiones (9%), 210
    indeterminados (41%), 256 normales (50%). El 9% de intrusiones es
    deseable: más alertas generan fatiga y el sistema pierde utilidad.
    El 41% indeterminado es donde actúa el operador humano que el
    framework incorpora por diseño.

    La tabla recoge las 512 combinaciones con resultado en color
    ---rojo, ámbar, verde--- con descripción verbal. Hojas internas:
    distribución y pesos.
