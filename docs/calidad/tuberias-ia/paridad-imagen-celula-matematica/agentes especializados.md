---
article:
  doi: 10.21428/39829d0b.798b68e2
  elocation-id: 9-parametros-y-el-origen-del-sistema
author:
- Juan Antonio Lloret Egea
bibliography: /tmp/tmp-19t7zejAEKeDR1.json
copyright:
  link: "https://creativecommons.org/licenses/by-nc-nd/4.0/"
  text: Creative Commons Attribution-NonCommercial-NoDerivatives 4.0
    International License
  type: CC-BY-NC-ND
csl: /app/dist/server/server/utils/citations/citeStyles/apa-7th-edition.csl
date:
  day: 02
  month: 03
  year: 2026
journal:
  publisher-name: IA eñ ™ - (La Biblia de la IA - The Bible of AI ™ ISSN
    2695-641)
  title: IA eñ ™
link-citations: true
title: "De SVcustos, el marco (framework) de intrusión, hasta SVperitus:
  agentes especializados"
uri: "https://www.itvia.online/pub/9-parametros-y-el-origen-del-sistema"
---

![](https://assets.pubpub.org/c39829d0b-6cf2-4c57-8562-04aa66068f1d/p798b68e2-c80b-469b-b460-6e39316bd33c/u8d2bc69d-73c6-4669-a7e7-fb07fdba4171/svcustos_svperitus_logo_main-21772472269246.png){#nvrpdy9pfsn}

\[video element\]

------------------------------------------------------------------------

# De SVcustos, el marco (framework) de intrusión, hasta SVperitus: agentes especializados {#de-svcustos-el-marco-framework-de-intrusión}

## El nivel base: 9 parámetros y el origen del sistema

(Documento 1 de 8)

Pertenece a la colección: "**De SVcustos, el marco (framework) de
intrusión, hasta SVperitus: agentes especializados" ** DOI:
10.21428/39829d0b.1129de25\
\
**Lugar: Madrid**

------------------------------------------------------------------------

## 1. Introducción

SVcustos es un sistema de detección de intrusiones basado en vectores
ternarios. El nombre proviene de SV (Sistema Vectorial) y custos
(guardián en latín). La marco original (DOI: 10.21428/39829d0b.981b7276)
establece que el campo de aplicación del framework abarca desde
smartwatches y smartphones hasta tablets, PCs y domótica basada en el
estándar KNX: cualquier dispositivo capaz de generar un vector de
parámetros ternarios puede ser evaluado por el sistema.

El concepto central es representar el estado de seguridad de un
dispositivo como un vector de *n* parámetros, donde cada parámetro toma
uno de tres valores: 0 (normal), 1 (activo/intrusión) o U
(indeterminado). El sistema transforma ese vector en una imagen polar
que una red neuronal ResNet34 clasifica en tres estados: **INTRUSIÓN**,
**INDETERMINADO** o **NORMAL**.

Este documento establece el nivel base con *n = 9* parámetros,
organizados en 3 capas temáticas de 3 parámetros cada una. Es el primero
de una serie de 8 que desarrolla la arquitectura SVcustos hasta *n = 49*
y su transposición al dominio del conocimiento experto bajo el nombre
**SVperitus**.

Dado que el ecosistema de dispositivos objetivo es heterogéneo (sistemas
operativos dispares, capacidades de hardware variables, distintos
niveles de apertura a aplicaciones de terceros), este documento separa
deliberadamente la **definición semántica** de cada parámetro (qué se
mide y por qué) de su **implementación técnica por familia de
dispositivos** (cómo se obtiene en cada plataforma). Esta separación es
la piedra angular que permite que el marco sea universal sin sacrificar
rigor técnico.

------------------------------------------------------------------------

## 2. Posición en la serie

Éste es el primer documento de la serie. No tiene precedentes como tal.
Establece las fundaciones: la restricción algebraica *n = b\^2*, la
lógica ternaria, la transformación polar, los 9 parámetros del nivel
base, las familias de dispositivos, y las topologías de despliegue.
Todos los documentos posteriores construyen sobre estas fundaciones.

------------------------------------------------------------------------

## 3. La restricción algebraica n = b²

La restricción invariante establece que *n* debe ser un cuadrado
perfecto: *n = b\^2*, con *b ≥ 3*. Esto garantiza simetría radial
uniforme del polígono polar (n ejes equiespaciados a 360°/n grados) y
organización en *b* capas temáticas de *b* parámetros cada una,
permitiendo auditoría por capas funcionales.

Para el nivel base, *b = 3, n = 9 = 3\^2*. Las tres capas son: **Red**
(comunicaciones de red), **Conectividad** (interfaces de conexión) y
**Sistema** (sensores y hardware). Cada capa contiene exactamente 3
parámetros.

------------------------------------------------------------------------

## 4. Topologías de despliegue

La heterogeneidad de dispositivos obliga a definir tres topologías de
despliegue del agente SVcustos. La topología determina dónde se ejecuta
el agente que mide los 9 parámetros y, por tanto, qué APIs son
accesibles directamente.

### 4.1. Topología autónoma

El agente SVcustos se ejecuta directamente en el dispositivo evaluado.
El dispositivo posee un sistema operativo con APIs suficientes para
medir los 9 parámetros sin asistencia externa. Todos los valores se
obtienen en tiempo real y con latencia mínima.

**Aplica a:** smartphones (Android/iOS), tablets (Android/iPadOS),
smartwatches con SO completo (Wear OS, watchOS) con las limitaciones
propias de cada plataforma.

### 4.2. Topología acoplada (companion)

El dispositivo evaluado no tiene capacidad para ejecutar un agente de
terceros completo. En su lugar, transmite datos brutos al smartphone
vinculado (companion) mediante Bluetooth Low Energy (BLE), y es el
agente SVcustos ejecutándose en el companion quien mide los parámetros.
El companion actúa como puente: recibe telemetría del dispositivo,
evalúa los parámetros factibles, y asigna valor **U** a los que no puede
medir.

**Aplica a:** smartwatches con RTOS (Garmin OS, Zepp OS, FreeRTOS,
HarmonyOS Lite) y dispositivos IoT sin soporte de aplicaciones de
terceros.

### 4.3. Topología híbrida

Casos donde el dispositivo puede medir algunos parámetros autónomamente
pero delega otros al companion. Típico de smartwatches con SO completo
que tienen restricciones en inspección de tráfico de red: el reloj mide
sensores y Bluetooth directamente, pero delega la inspección de URLs y
cifrado al companion.

**Aplica a:** smartwatches Wear OS y watchOS para los parámetros de la
capa Red.

------------------------------------------------------------------------

## 5. Familias de dispositivos

SVcustos opera sobre cuatro familias de dispositivos, cada una con un
ejemplo concreto, una topología natural, y un perfil de cobertura
paramétrica propio.

+-----------+------------------------+----------+--------------------------+
| Familia   | Ejemplo concreto       | T        | Sistema operativo        |
|           |                        | opología |                          |
+===========+========================+==========+==========================+
| S         | Samsung Galaxy S24     | Autónoma | Android 14+ / iOS 17+    |
| martphone | (Android 14)           |          |                          |
+-----------+------------------------+----------+--------------------------+
| Tablet    | iPad Air M2 (iPadOS    | Autónoma | Android 14+ / iPadOS 17+ |
|           | 17)                    |          |                          |
+-----------+------------------------+----------+--------------------------+
| S         | Samsung Galaxy Watch 6 | Híbrida  | Wear OS 4+ / watchOS 10+ |
| martwatch | (Wear OS 4)            |          |                          |
| SO        |                        |          |                          |
| completo  |                        |          |                          |
+-----------+------------------------+----------+--------------------------+
| S         | Garmin Fenix 8 (Garmin | Acoplada | Garmin OS / Zepp OS /    |
| martwatch | OS)                    |          | FreeRTOS / HarmonyOS     |
| RTOS      |                        |          | Lite                     |
+-----------+------------------------+----------+--------------------------+

Los porcentajes de mercado sitúan aproximadamente el 50% de los
smartwatches en plataformas RTOS o híbridas, lo que hace imprescindible
que SVcustos soporte la topología acoplada para no excluir la mitad del
ecosistema *wearable*.

------------------------------------------------------------------------

## 6. Los 9 parámetros: definición semántica

Esta sección define cada parámetro por lo que mide (**semántica**), no
por cómo se obtiene (**implementación**). La semántica es invariante:
independiente de la plataforma, el sistema operativo y la topología de
despliegue. La implementación técnica por familia se detalla en la
sección 7.

### Capa: Red

#### P1: URL no autorizada \[★ ALTO\]

**Semántica:** detecta si el dispositivo está estableciendo conexiones
de red con dominios no incluidos en la lista blanca corporativa. Se
evalúa el dominio raíz resuelto, no la IP (que cambia con CDNs).

**Criterio ternario:**\
1 = dominio no autorizado detectado;\
0 = todos los dominios en lista blanca;\
U = DNS ambiguo (CDN, redirección múltiple) o medición delegada a
*companion* sin confirmación.

+---------------------+------------------------------------------------+
| Argumento contrario | Réplica técnica                                |
+=====================+================================================+
| Los CDN legítimos   | El filtro opera por dominio raíz, no por IP.   |
| resuelven a         | CDNs legítimos (Cloudflare, Akamai) se         |
| dominios            | registran en la lista blanca por dominio raíz. |
| cambiantes; esto    | Sólo dominios no registrados disparan valor 1. |
| generaría falsos    |                                                |
| positivos.          |                                                |
+---------------------+------------------------------------------------+

#### P2: Comunicación no cifrada \[★ ALTO\]

**Semántica:** detecta si el dispositivo transmite datos por canales no
cifrados. Evalúa la presencia de TLS 1.2+ en el tráfico saliente
dirigido a interfaces WAN, excluyendo tráfico local (localhost, mDNS,
multicast).

**Criterio ternario:**\
1 = transmisión sin cifrar detectada;\
0 = todo el tráfico usa TLS 1.2+;\
U = protocolo no identificable o tráfico inspeccionado parcialmente.

+---------------------+-----------------------------------------------+
| Argumento contrario | Réplica técnica                               |
+=====================+===============================================+
| El tráfico local    | Se excluye explícitamente el tráfico a        |
| (localhost, mDNS)   | 127.0.0.1/8, ::1 y multicast local            |
| no va cifrado y es  | (224.0.0.0/4). Solo se evalúa tráfico         |
| legítimo.           | dirigido a interfaces WAN.                    |
+---------------------+-----------------------------------------------+

#### P3: Transferencia de datos al exterior \[★ ALTO\]

**Semántica:** detecta volúmenes anómalos de datos salientes desde el
dispositivo. El umbral se calibra por proceso o aplicación, no
globalmente, para evitar falsos positivos por actualizaciones del
sistema o *backups* autorizados.

**Criterio ternario:**\
1 = volumen de subida excede umbral configurado;\
0 = dentro de límites;\
U = entre 70--100% del umbral o cambio de red reciente.

+------------------------+---------------------------------------------+
| Argumento contrario    | Réplica técnica                             |
+========================+=============================================+
| Las actualizaciones    | El umbral se calibra por UID de aplicación, |
| del sistema o          | no globalmente. Los procesos del sistema    |
| *backups* en la nube   | tienen umbrales propios más altos o están   |
| generan picos          | en lista blanca.                            |
| legítimos de subida.   |                                             |
+------------------------+---------------------------------------------+

### Capa: Conectividad

#### P4: BSSID no autorizado \[★ ALTO\]

**Semántica:** detecta si el dispositivo está conectado a un punto de
acceso WiFi cuyo BSSID no figura en la lista blanca corporativa. En
dispositivos sin WiFi (muchos RTOS), el parámetro no aplica y se fija en
valor 0.

**Criterio ternario:**\
1 = BSSID conectado fuera de lista blanca;\
0 = BSSID en lista blanca o dispositivo sin WiFi;\
U = solo conectividad celular, BSSID enmascarado, o datos no
disponibles.

+--------------------+-------------------------------------------------+
| Argumento          | Réplica técnica                                 |
| contrario          |                                                 |
+====================+=================================================+
| En movilidad, el   | El sistema opera con lista blanca configurable: |
| usuario se conecta | estricta en entornos corporativos, dinámica     |
| a redes nuevas     | (registro tras primera conexión verificada) en  |
| constantemente.    | modo personal.                                  |
+--------------------+-------------------------------------------------+

#### P5: Bluetooth activo \[bajo\]

**Semántica:** detecta si el dispositivo mantiene conexiones Bluetooth
con dispositivos no autorizados. Evalúa la lista de dispositivos
vinculados activos, no el mero estado encendido/apagado del adaptador.

**Criterio ternario:**\
1 = dispositivos no autorizados vinculados;\
0 = apagado o solo dispositivos autorizados;\
U = activo en modo descubrible sin conexiones establecidas.

+----------------------+-----------------------------------------------+
| Argumento contrario  | Réplica técnica                               |
+======================+===============================================+
| Bluetooth es         | El peso es bajo por esta razón. Solo genera   |
| necesario para       | valor 1 cuando hay dispositivos vinculados no |
| auriculares, relojes | registrados, no por estar activo.             |
| y periféricos        |                                               |
| médicos.             |                                               |
+----------------------+-----------------------------------------------+

#### P6: GPS activo \[★ ALTO\]

**Semántica:** detecta si una aplicación no autorizada ha accedido a la
ubicación del dispositivo. Distingue entre permisos del SO (necesarios
pero no suficientes) y autorización corporativa.

**Criterio ternario:**\
1 = app no autorizada ha accedido a ubicación;\
0 = GPS apagado o solo apps autorizadas;\
U = GPS activo sin accesos recientes registrados.

+------------------------+---------------------------------------------+
| Argumento contrario    | Réplica técnica                             |
+========================+=============================================+
| El sistema operativo   | SVcustos opera en la capa de política       |
| ya gestiona permisos   | corporativa, no en la capa de permisos del  |
| de ubicación por       | SO. Una app puede tener permiso otorgado    |
| aplicación.            | por el usuario pero no estar autorizada     |
|                        | corporativamente.                           |
+------------------------+---------------------------------------------+

### Capa: Sistema

#### P7: Cámara no autorizada \[★ ALTO\]

**Semántica:** detecta si una aplicación no autorizada está usando la
cámara del dispositivo. En dispositivos sin cámara (la mayoría de
smartwatches), el parámetro no aplica y se fija en valor 0.

**Criterio ternario:**\
1 = app no autorizada usa la cámara;\
0 = cámara no en uso, solo apps autorizadas, o dispositivo sin cámara;\
U = servicio de sistema con UID no verificable accede a cámara.

+-------------------------+--------------------------------------------+
| Argumento contrario     | Réplica técnica                            |
+=========================+============================================+
| Las videollamadas       | Las apps de videollamada autorizadas       |
| corporativas legítimas  | (Teams, Zoom, Meet) se registran en lista  |
| activan la cámara.      | blanca corporativa. Solo apps no           |
|                         | registradas producen valor 1.              |
+-------------------------+--------------------------------------------+

#### P8: Micrófono no autorizado \[★ ALTO\]

**Semántica:** detecta si una aplicación no autorizada está grabando
audio. Distingue entre escucha pasiva de *wake-word* (hardware, no
monitorizable) y grabación activa de aplicación.

**Criterio ternario:**\
1 = app no autorizada graba audio;\
0 = micrófono no en uso o solo apps autorizadas;\
U = servicio no verificable accede al micrófono, o API no disponible en
el dispositivo.

+----------------------+----------------------------------------------+
| Argumento contrario  | Réplica técnica                              |
+======================+==============================================+
| Los asistentes de    | Los asistentes del sistema están en lista    |
| voz (Google          | blanca. El valor 1 se activa solo con apps   |
| Assistant, Siri)     | de terceros no registradas iniciando         |
| escuchan             | grabación activa.                            |
| permanentemente.     |                                              |
+----------------------+----------------------------------------------+

#### P9: Parámetros físicos anómalos \[bajo\]

**Semántica:** detecta patrones anómalos en los sensores de movimiento y
ambientales del dispositivo (acelerómetro, giroscopio, magnetómetro,
barómetro). Un dispositivo siendo manipulado para extracción física
genera patrones reconocibles.

**Criterio ternario:**\
1 = patrones de manipulación detectados (vibración sostenida sin
desplazamiento, rotaciones rápidas sin aceleración lineal);\
0 = perfil sensorial normal;\
U = dispositivo estático prolongado o sensores intermitentes.

+--------------------+-------------------------------------------------+
| Argumento          | Réplica técnica                                 |
| contrario          |                                                 |
+====================+=================================================+
| Un dispositivo     | El valor U (no 1) se asigna a dispositivos      |
| sobre una mesa     | estáticos. El valor 1 requiere patrones activos |
| está estático      | anómalos, no la simple ausencia de movimiento.  |
| legítimamente.     |                                                 |
+--------------------+-------------------------------------------------+

------------------------------------------------------------------------

## 7. Implementación técnica por familia de dispositivos

Esta sección detalla, para cada familia, exactamente cómo se obtiene
cada parámetro: qué API, clase o SDK se invoca, en qué modo (autónomo,
*companion* o no aplicable) y qué limitaciones reales existen. Las APIs
referenciadas son verificables en la documentación oficial de cada
plataforma.

### 7.1. Smartphone --- Topología autónoma

**Ejemplo:** Samsung Galaxy S24 (Android 14) / iPhone 15 (iOS 17)

El smartphone es el dispositivo de referencia del sistema. Todos los
parámetros se obtienen autónomamente con APIs estándar. No requiere
*companion*.

+-------+-------+-----------------------------+-------------------------------+
| P     | Modo  | Android 14+                 | iOS 17+                       |
| aram. |       |                             |                               |
+=======+=======+=============================+===============================+
| P1    | A     | `NetworkCallback` +         | `NEDNSProxyProvider` +        |
|       |       | `DnsResolver`               | `NWPathMonitor`               |
+-------+-------+-----------------------------+-------------------------------+
| P2    | A     | `VpnService` (inspección    | `NEFilterDataProvider`        |
|       |       | local)                      | (Network Extension)           |
+-------+-------+-----------------------------+-------------------------------+
| P3    | A     | `NetworkStatsMa             | `NWPathMonitor` + contadores  |
|       |       | nager.queryDetailsForUid()` | interfaz                      |
+-------+-------+-----------------------------+-------------------------------+
| P4    | A     | `WifiManager.getC           | `NEHotspotHelper` (Network    |
|       |       | onnectionInfo().getBSSID()` | Extension)                    |
+-------+-------+-----------------------------+-------------------------------+
| P5    | A     | `Blu                        | `CBCentralManager`            |
|       |       | etoothAdapter.getState()` + | (CoreBluetooth)               |
|       |       | `getBondedDevices()`        |                               |
+-------+-------+-----------------------------+-------------------------------+
| P6    | A     | `LocationManager` +         | `CLLocationM                  |
|       |       | `AppO                       | anager.authorizationStatus()` |
|       |       | psManager.checkOpNoThrow()` |                               |
+-------+-------+-----------------------------+-------------------------------+
| P7    | A     | `CameraMana                 | `AVCaptureDevice              |
|       |       | ger.AvailabilityCallback` + | .authorizationStatus(.video)` |
|       |       | `AppOpsManager`             |                               |
+-------+-------+-----------------------------+-------------------------------+
| P8    | A     | `AppOpsManager` +           | `AVAudioSession` +            |
|       |       | `AudioManager.getActiv      | `AVCaptureDevice`             |
|       |       | eRecordingConfigurations()` |                               |
+-------+-------+-----------------------------+-------------------------------+
| P9    | A     | `SensorManager`             | `CMMotionManager`             |
|       |       | (TYPE_ACCELEROMETER,        | (CoreMotion)                  |
|       |       | GYROSCOPE, etc.)            |                               |
+-------+-------+-----------------------------+-------------------------------+

**Cobertura:** 9/9 parámetros autónomos. Sin parámetros en U por
limitación de plataforma.

### 7.2. Tablet --- Topología autónoma

**Ejemplo:** iPad Air M2 (iPadOS 17) / Samsung Galaxy Tab S9 (Android
14)

La tablet comparte el *stack* de APIs con el smartphone. Las únicas
diferencias relevantes afectan a modelos WiFi-only: P4 (BSSID) funciona
normalmente ya que la tablet usa WiFi como conexión principal; P6 (GPS)
puede producir valor U en modelos sin módulo GPS (obtención por
posicionamiento WiFi/IP, menos fiable). El resto de parámetros son
idénticos.

**Cobertura:** 9/9 en modelos celulares. 8/9 + 1U (P6) en modelos
WiFi-only sin GPS.

### 7.3. Smartwatch con SO completo --- Topología híbrida

**Ejemplo:** Samsung Galaxy Watch 6 (Wear OS 4, chip Exynos W930, WiFi +
LTE opcional)

Los smartwatches con Wear OS o watchOS ejecutan un agente SVcustos
nativo, pero con restricciones significativas en la capa Red. Wear OS
hereda gran parte de las APIs de Android pero no permite `VpnService` ni
inspección profunda de paquetes. watchOS es aún más restrictivo: el
tráfico de red está fuertemente *sandboxed*. Por ello, los parámetros de
red (P1, P2, P3) operan en modo híbrido: el reloj delega la inspección
de tráfico al *companion* mediante la Wearable Data Layer API (Wear OS)
o WatchConnectivity (watchOS).

+--------+--------+-----------------------------+---------------------------+
| Param. | Modo   | Wear OS 4+ (Galaxy Watch 6) | watchOS 10+ (Apple Watch  |
|        |        |                             | S9)                       |
+========+========+=============================+===========================+
| P1     | H      | Tráfico vía *companion*:    | *Companion* inspecciona   |
|        |        | `WearableListenerService`   | via                       |
|        |        | recibe informe DNS del      | `WCSe                     |
|        |        | smartphone                  | ssion.transferUserInfo()` |
+--------+--------+-----------------------------+---------------------------+
| P2     | H      | `NetworkSecurityPolicy`     | TLS obligatorio por       |
|        |        | (parcial). TLS obligatorio  | defecto en watchOS.       |
|        |        | por defecto en Wear OS 4.   | *Companion* confirma      |
|        |        | Residual vía *companion*.   | excepciones.              |
+--------+--------+-----------------------------+---------------------------+
| P3     | H      | `URLSessionTaskMetrics`     | *Companion* calcula       |
|        |        | parcial. *Companion* envía  | volumen total BLE + WiFi  |
|        |        | informe de volumen por      | transferido al reloj.     |
|        |        | `DataClient`.               |                           |
+--------+--------+-----------------------------+---------------------------+
| P4     | A      | `WifiM                      | `                         |
|        |        | anager.getConnectionInfo()` | CNCopyCurrentNetworkInfo` |
|        |        | en Wear OS (WiFi integrado  | (requiere *entitlement*;  |
|        |        | en Galaxy Watch 6).         | limitado en watchOS 14+). |
+--------+--------+-----------------------------+---------------------------+
| P5     | A      | `BluetoothAdapter` via      | `CBCentralManager`        |
|        |        | `android.bluetooth`.        | (CoreBluetooth). Detecta  |
|        |        | Detecta dispositivos        | periféricos BLE           |
|        |        | vinculados activos.         | conectados.               |
+--------+--------+-----------------------------+---------------------------+
| P6     | A      | `F                          | `CLLocationManager` (GPS  |
|        |        | usedLocationProviderClient` | integrado). Autorización  |
|        |        | (GPS integrado). Sin        | por app disponible.       |
|        |        | `AppOpsManager` completo.   |                           |
+--------+--------+-----------------------------+---------------------------+
| P7     | 0      | Sin cámara en modelos       | Sin cámara → valor fijo   |
|        |        | actuales → valor fijo 0.    | 0.                        |
+--------+--------+-----------------------------+---------------------------+
| P8     | A      | `AudioManager.getActive     | `AVAudioS                 |
|        |        | RecordingConfigurations()`. | ession.sharedInstance()`. |
|        |        | Micrófono integrado.        | Micrófono integrado.      |
+--------+--------+-----------------------------+---------------------------+
| P9     | A      | `SensorManager`:            | `CMMotionManager`: suite  |
|        |        | acelerómetro, giroscopio,   | completa de sensores      |
|        |        | barómetro integrados.       | inerciales.               |
+--------+--------+-----------------------------+---------------------------+

**Cobertura:** 6/9 autónomos (P4--P6, P8--P9 + P7 fijo a 0) + 3/9
híbridos (P1--P3 vía *companion*). Ningún parámetro queda en U por
limitación de plataforma; la delegación al *companion* garantiza
medición completa siempre que el smartphone esté vinculado.

### 7.4. Smartwatch RTOS --- Topología acoplada (companion)

**Ejemplo:** Garmin Fenix 8 (Garmin OS, chip MediaTek, GPS multi-banda,
Connect IQ 7.0)

Los smartwatches con RTOS no permiten ejecutar un agente SVcustos nativo
con capacidades de inspección. El agente se ejecuta en el smartphone
*companion* (Android/iOS) y obtiene datos del reloj mediante el SDK
propio del fabricante. En el caso de Garmin, el Connect IQ SDK permite
crear *DataFields* y *Widgets* que transmiten telemetría al smartphone
vía Garmin Connect Mobile + BLE. En Amazfit, el Zepp Health SDK
(JavaScript) permite funcionalidad similar a través de la app Zepp.
Crucialmente, lo que el *companion* no puede medir directamente queda
como U, no como un valor inventado.

+-------+-------+----------------------------------------------------------------+
| P     | Modo  | Garmin Fenix 8 (Garmin OS / Connect IQ 7.0)                    |
| aram. |       |                                                                |
+=======+=======+================================================================+
| P1    | C     | Sin acceso directo a DNS. El *companion* inspecciona el        |
|       |       | tráfico generado por el reloj vía el *bridge* BLE: Garmin      |
|       |       | Connect Mobile actúa como proxy de conectividad y registra los |
|       |       | *endpoints* contactados.                                       |
+-------+-------+----------------------------------------------------------------+
| P2    | C     | El *companion* verifica que la comunicación BLE                |
|       |       | reloj↔smartphone usa cifrado AES-128 (estándar BLE Secure). El |
|       |       | tráfico WAN del reloj (sync, AGPS) pasa por el *companion* y   |
|       |       | se inspecciona allí.                                           |
+-------+-------+----------------------------------------------------------------+
| P3    | C     | El *companion* mide el volumen de datos transferidos           |
|       |       | desde/hacia el reloj por sesión BLE. Garmin Connect Mobile     |
|       |       | registra bytes sincronizados por actividad (rutas, salud,      |
|       |       | *watchfaces*).                                                 |
+-------+-------+----------------------------------------------------------------+
| P4    | 0     | Sin WiFi en Garmin Fenix 8 (solo BLE + ANT+). Parámetro no     |
|       |       | aplicable → valor fijo 0. En modelos con WiFi (Garmin Venu 3): |
|       |       | sin API accesible → valor U.                                   |
+-------+-------+----------------------------------------------------------------+
| P5    | C     | El reloj **ES** un dispositivo BLE. El *companion* monitoriza  |
|       |       | qué otros dispositivos BLE se vinculan al reloj vía            |
|       |       | `Android BluetoothAdapter.getBondedDevices()` filtrando por    |
|       |       | nombre/MAC del reloj.                                          |
+-------+-------+----------------------------------------------------------------+
| P6    | C     | GPS integrado (multi-banda en Fenix 8). Datos de ubicación     |
|       |       | accesibles vía Connect IQ SDK: `Position.getInfo()`. El        |
|       |       | *companion* lee historial de posiciones y evalúa si apps no    |
|       |       | autorizadas han accedido.                                      |
+-------+-------+----------------------------------------------------------------+
| P7    | 0     | Sin cámara → valor fijo 0.                                     |
+-------+-------+----------------------------------------------------------------+
| P8    | U     | Micrófono presente en algunos modelos (Garmin Venu 3, no en    |
|       |       | Fenix 8). Donde existe, sin API de terceros para monitorizar   |
|       |       | acceso → valor U. Donde no existe → valor 0.                   |
+-------+-------+----------------------------------------------------------------+
| P9    | C     | Sensores ricos (acelerómetro, giroscopio, barómetro,           |
|       |       | altímetro). Datos accesibles vía Connect IQ Sensor API         |
|       |       | (`Sensor.setEnabledSensors`). El *companion* recibe lecturas y |
|       |       | aplica detector de anomalías.                                  |
+-------+-------+----------------------------------------------------------------+

**Cobertura:** 0/9 autónomos. 6/9 vía *companion* (P1--P3, P5--P6, P9).
2/9 no aplicables → 0 (P4, P7). 1/9 indeterminado estructural (P8 en
modelos con micrófono sin API). El vector resultante refleja con
honestidad lo que es medible: los parámetros que no se pueden evaluar no
se fingen medidos.

------------------------------------------------------------------------

## 8. Tabla de cobertura paramétrica por familia

La siguiente tabla sintetiza el modo de obtención de cada parámetro en
cada familia.\
A = autónomo, H = híbrido (reloj + *companion*), C = *companion*, 0 = no
aplica (hardware inexistente, valor fijo 0), U = indeterminado
estructural (hardware existe pero sin API accesible).

+-------+------------+-------+----------------------+-----------------+
| P     | Smartphone | T     | Smartwatch SO compl. | Smartwatch RTOS |
| aram. |            | ablet |                      |                 |
+=======+============+=======+======================+=================+
| P1    | A          | A     | H                    | C               |
+-------+------------+-------+----------------------+-----------------+
| P2    | A          | A     | H                    | C               |
+-------+------------+-------+----------------------+-----------------+
| P3    | A          | A     | H                    | C               |
+-------+------------+-------+----------------------+-----------------+
| P4    | A          | A     | A                    | 0               |
+-------+------------+-------+----------------------+-----------------+
| P5    | A          | A     | A                    | C               |
+-------+------------+-------+----------------------+-----------------+
| P6    | A          | A\*   | A                    | C               |
+-------+------------+-------+----------------------+-----------------+
| P7    | A          | A     | 0                    | 0               |
+-------+------------+-------+----------------------+-----------------+
| P8    | A          | A     | A                    | U               |
+-------+------------+-------+----------------------+-----------------+
| P9    | A          | A     | A                    | C               |
+-------+------------+-------+----------------------+-----------------+

\* P6 en tablet WiFi-only sin GPS → U.

**Cobertura:** Smartphone 9A, Tablet 9A (8A+1U WiFi-only), Smartwatch SO
6A+3H, Smartwatch RTOS 6C+2×0+1U.

------------------------------------------------------------------------

## 9. Del vector al polígono polar

Cada vector de 9 valores ternarios se transforma en una imagen polar: 9
ejes radiales equiespaciados a 40° (360°/9), partiendo desde las 12 en
punto en sentido horario. El mapeo de radios es:

-   valor 0 → radio 1 (zona segura),\

-   valor 1 → radio 2 (zona de intrusión),\

-   valor U → radio 3 (zona de incertidumbre).

Los 9 puntos se conectan formando un polígono cerrado cuya forma
codifica visualmente el estado de seguridad.

Esta transformación es independiente de la familia de dispositivos: un
vector `(1,0,0,U,0,0,0,0,1)` produce exactamente el mismo polígono polar
ya sea originado en un smartphone o en un smartwatch RTOS vía
*companion*.

------------------------------------------------------------------------

## 10. Regla de clasificación

La clasificación opera sobre el conteo estricto de parámetros
confirmados en valor 1 (n₁) y parámetros indeterminados (nᵤ). La
puntuación ponderada `score = n₁ × 1,0 + nᵤ × 0,5` se mantiene como
gradiente de riesgo dentro de la clase INDETERMINADO, pero no determina
la clasificación INTRUSIÓN.

**Regla estricta**

-   **INTRUSIÓN:** n₁ ≥ 7. Al menos 7 de los 9 parámetros confirmados en
    valor 1. La incertidumbre no sustituye a la evidencia.\

-   **INDETERMINADO:** n₁ ∈ {5, 6} y nᵤ ≥ 1. Requiere revisión humana o
    análisis autónomo adicional. Estos vectores podrían escalar a
    INTRUSIÓN si sus U se confirman.\

-   **NORMAL:** Todo lo demás.

**Distribución del espacio ternario 3\^9 = 19.683**

  Clasificación   Vectores   Porcentaje
  --------------- ---------- ------------
  INTRUSIÓN       163        0,828%
  INDETERMINADO   2.478      12,590%
  NORMAL          17.042     86,582%
  **TOTAL**       19.683     100%

Dato relevante: de los 2.478 INDETERMINADOS, 1.974 (79,7%) podrían
escalar a INTRUSIÓN si sus U se confirmaran como 1. Los otros 504 (n₁ =
5, nᵤ = 1) tienen máximo potencial n₁ = 6 \< 7, pero merecen revisión
humana por tener 5 positivos confirmados con incertidumbre.

------------------------------------------------------------------------

## 11. Cuatro vectores ilustrativos: uno por familia

Cada ejemplo muestra un vector real que podría producirse en esa
familia, explicando cómo se obtiene cada valor y por qué la
clasificación es la que es.

### 11.1. Smartphone (Samsung Galaxy S24) → INTRUSIÓN

**Vector:** `(1, 1, 1, 1, 0, 1, 1, 1, 0)`

Topología: autónoma. Todos los valores obtenidos directamente en el
dispositivo.

Conteo: n₁ = 7, nᵤ = 0, n₀ = 2.\
Clasificación: **INTRUSIÓN** (n₁ ≥ 7).

+-------+----------------------------+---------------------------------+
| P     | Cómo se obtiene            | Valor y razón                   |
| aram. |                            |                                 |
+=======+============================+=================================+
| P1    | `NetworkCallback`          | 1 --- dominio no registrado en  |
|       | intercepta resolución DNS  | lista blanca.                   |
|       | a dominio no autorizado.   |                                 |
+-------+----------------------------+---------------------------------+
| P2    | `VpnService` local detecta | 1 --- transmisión no cifrada.   |
|       | paquete HTTP (sin TLS)     |                                 |
|       | dirigido a IP externa.     |                                 |
+-------+----------------------------+---------------------------------+
| P3    | `NetworkStatsManager`      | 1 --- excede umbral.            |
|       | registra 48 MB de subida   |                                 |
|       | por UID de app sospechosa  |                                 |
|       | en 10 min.                 |                                 |
+-------+----------------------------+---------------------------------+
| P4    | `WifiManager.getCo         | 1 --- BSSID fuera de lista      |
|       | nnectionInfo().getBSSID()` | blanca.                         |
|       | devuelve BSSID             |                                 |
|       | desconocido.               |                                 |
+-------+----------------------------+---------------------------------+
| P5    | `BluetoothA                | 0 --- dispositivos autorizados. |
|       | dapter.getBondedDevices()` |                                 |
|       | muestra solo auriculares   |                                 |
|       | autorizados.               |                                 |
+-------+----------------------------+---------------------------------+
| P6    | `AppOpsManager` detecta    | 1 --- app no autorizada         |
|       | acceso a ubicación por app | accedió.                        |
|       | no autorizada              |                                 |
|       | corporativamente.          |                                 |
+-------+----------------------------+---------------------------------+
| P7    | `CameraMan                 | 1 --- uso no autorizado.        |
|       | ager.AvailabilityCallback` |                                 |
|       | detecta app no registrada  |                                 |
|       | usando cámara.             |                                 |
+-------+----------------------------+---------------------------------+
| P8    | `AudioManager.getActive    | 1 --- grabación no autorizada.  |
|       | RecordingConfigurations()` |                                 |
|       | detecta grabación por app  |                                 |
|       | no registrada.             |                                 |
+-------+----------------------------+---------------------------------+
| P9    | `SensorManager` reporta    | 0 --- sin anomalías.            |
|       | perfil inercial dentro de  |                                 |
|       | lo normal.                 |                                 |
+-------+----------------------------+---------------------------------+

**Interpretación:** perfil inequívocamente intrusivo. La capa Red
completa está comprometida (exfiltración activa), el dispositivo está en
un WiFi no autorizado, y la cámara y micrófono están siendo utilizados
por apps no registradas. 7 de 9 parámetros confirmados sin ninguna
ambigüedad.

### 11.2. Tablet (iPad Air M2, iPadOS 17) → NORMAL

**Vector:** `(0, 0, 0, 0, 0, 0, 0, 0, 0)`

Topología: autónoma. Todos los valores obtenidos directamente.

Conteo: n₁ = 0, nᵤ = 0, n₀ = 9.\
Clasificación: **NORMAL**.

Todos los parámetros en valor 0: el tráfico de red es cifrado y dirigido
a dominios autorizados, el dispositivo está conectado a la red WiFi
corporativa, no hay apps no autorizadas accediendo a sensores. Nótese
que la cobertura es completa (9/9) porque este modelo (WiFi + Cellular)
tiene GPS nativo. En un iPad WiFi-only sin GPS, P6 podría ser U, pero la
clasificación seguiría siendo NORMAL porque n₁ = 0.

### 11.3. Smartwatch SO completo (Galaxy Watch 6, Wear OS 4) → INDETERMINADO

**Vector:** `(1, U, 1, U, 1, 1, 0, 1, 0)`

Topología: híbrida. P1 y P3 delegados al *companion*; P4 y P8 medidos
autónomamente.

Conteo: n₁ = 5, nᵤ = 2, n₀ = 2.\
Clasificación: **INDETERMINADO** (n₁ ∈ {5, 6} ∧ nᵤ ≥ 1).

+--------+--------------------------------+-----------------------------+
| Param. | Cómo se obtiene                | Valor y razón               |
+========+================================+=============================+
| P1     | *Companion* reporta vía        | 1 --- confirmado por        |
|        | `WearableListenerService` que  | *companion*.                |
|        | el reloj contactó dominio no   |                             |
|        | autorizado.                    |                             |
+--------+--------------------------------+-----------------------------+
| P2     | *Companion* debería verificar  | U --- medición incompleta.  |
|        | cifrado, pero la sesión BLE se |                             |
|        | interrumpió antes de completar |                             |
|        | la inspección.                 |                             |
+--------+--------------------------------+-----------------------------+
| P3     | *Companion* detectó volumen de | 1 --- excede umbral.        |
|        | datos anómalo en la            |                             |
|        | sincronización del reloj vía   |                             |
|        | `DataClient`.                  |                             |
+--------+--------------------------------+-----------------------------+
| P4     | `Wi                            | U --- dato ambiguo.         |
|        | fiManager.getConnectionInfo()` |                             |
|        | en el reloj reporta WiFi, pero |                             |
|        | BSSID cambió durante la        |                             |
|        | medición.                      |                             |
+--------+--------------------------------+-----------------------------+
| P5     | `BluetoothAdapter` en el reloj | 1 --- vinculación no        |
|        | detecta dispositivo BLE no     | autorizada.                 |
|        | registrado vinculado.          |                             |
+--------+--------------------------------+-----------------------------+
| P6     | `FusedLocationProviderClient`  | 1 --- acceso no autorizado. |
|        | en el reloj detecta acceso a   |                             |
|        | ubicación por app no           |                             |
|        | autorizada.                    |                             |
+--------+--------------------------------+-----------------------------+
| P7     | Sin cámara en Galaxy Watch 6.  | 0 --- no aplica.            |
|        | Hardware inexistente.          |                             |
+--------+--------------------------------+-----------------------------+
| P8     | `AudioManager` detecta         | 1 --- grabación no          |
|        | grabación activa por app no    | autorizada.                 |
|        | registrada en micrófono del    |                             |
|        | reloj.                         |                             |
+--------+--------------------------------+-----------------------------+
| P9     | `SensorManager` reporta perfil | 0 --- sin anomalías.        |
|        | inercial normal.               |                             |
+--------+--------------------------------+-----------------------------+

**Interpretación:** 5 parámetros confirmados como intrusivos con 2
indeterminados. Si ambos U se confirmaran como 1, n₁ subiría a 7 →
INTRUSIÓN. Este es exactamente el tipo de vector que requiere
intervención: o bien análisis autónomo adicional (re-medición al
restaurarse la conexión *companion*), o bien revisión humana.

### 11.4. Smartwatch RTOS (Garmin Fenix 8) → NORMAL

**Vector:** `(0, 0, 0, 0, 0, 0, 0, U, 0)`

Topología: acoplada. Todos los valores obtenidos por el *companion*
salvo P8.

Conteo: n₁ = 0, nᵤ = 1, n₀ = 8.\
Clasificación: **NORMAL**.

+------+------------------------------------+-----------------------------+
| Pa   | Cómo se obtiene                    | Valor y razón               |
| ram. |                                    |                             |
+======+====================================+=============================+
| P1   | *Companion* inspecciona            | 0 --- dominios autorizados. |
|      | *endpoints* contactados vía Garmin |                             |
|      | Connect Mobile (*bridge* BLE).     |                             |
+------+------------------------------------+-----------------------------+
| P2   | *Companion* verifica cifrado       | 0 --- cifrado correcto.     |
|      | AES-128 BLE + TLS en tráfico de    |                             |
|      | sincronización.                    |                             |
+------+------------------------------------+-----------------------------+
| P3   | *Companion* mide bytes             | 0 --- volumen normal.       |
|      | sincronizados por sesión BLE.      |                             |
|      | Volumen dentro de lo normal.       |                             |
+------+------------------------------------+-----------------------------+
| P4   | Garmin Fenix 8 no tiene WiFi (solo | 0 --- no aplica.            |
|      | BLE + ANT+). Hardware inexistente. |                             |
+------+------------------------------------+-----------------------------+
| P5   | *Companion* monitoriza             | 0 --- dispositivos          |
|      | vinculaciones BLE del reloj. Solo  | autorizados.                |
|      | sensores ANT+ autorizados.         |                             |
+------+------------------------------------+-----------------------------+
| P6   | Connect IQ `Position.getInfo()` →  | 0 --- sin acceso no         |
|      | *companion* evalúa historial de    | autorizado.                 |
|      | accesos GPS.                       |                             |
+------+------------------------------------+-----------------------------+
| P7   | Sin cámara en Garmin Fenix 8.      | 0 --- no aplica.            |
|      | Hardware inexistente.              |                             |
+------+------------------------------------+-----------------------------+
| P8   | Garmin Fenix 8 no tiene micrófono. | U --- indeterminado         |
|      | Pero otros modelos RTOS sí (Venu   | estructural.                |
|      | 3). Sin API accesible.             |                             |
+------+------------------------------------+-----------------------------+
| P9   | Connect IQ Sensor API →            | 0 --- sin anomalías.        |
|      | *companion* recibe datos de        |                             |
|      | acelerómetro y barómetro. Normal.  |                             |
+------+------------------------------------+-----------------------------+

**Interpretación:** dispositivo sin incidencias. El único valor U (P8)
es un indeterminado estructural: el modelo Fenix 8 no tiene micrófono,
pero el documento registra U porque otros modelos de la misma familia
RTOS (como Garmin Venu 3) sí lo tienen pero sin API accesible. Esto
refleja una limitación de la plataforma, no del framework. SVcustos no
finge conocimiento donde no lo tiene.

**Nota sobre el Fenix 8 específicamente:** al no tener micrófono, P8
podría fijarse a 0 como P4 y P7. Sin embargo, se conserva como U para
que la familia "Smartwatch RTOS" tenga una definición uniforme que cubra
modelos con y sin micrófono. El agente *companion* puede ajustar esto
por modelo concreto.

------------------------------------------------------------------------

## 12. Valoración de implementación

El nivel base n = 9 es implementable en las cuatro familias de
dispositivos, con grados de autonomía distintos. En smartphones y
tablets, la implementación es directa y completa. En smartwatches con SO
completo, la capa Red requiere un *companion* conectado, pero los
parámetros de Conectividad y Sistema son autónomos. En smartwatches
RTOS, toda la evaluación depende del *companion*, pero la cobertura real
alcanza 6--8 de 9 parámetros según el modelo.

El ciclo de muestreo estimado es inferior a 500 ms en smartphones, 1--2
segundos en smartwatches autónomos, y 5--15 segundos en la topología
acoplada (latencia BLE + procesamiento en *companion*).

Limitaciones del nivel base: con sólo 3 capas y 9 parámetros, el sistema
no cubre integridad del SO (detección de *root*), verificación de
certificados TLS, monitorización de procesos desconocidos, NFC,
almacenamiento externo ni permisos dinámicos. Estas limitaciones se
abordan progresivamente en los documentos 2 a 6.

------------------------------------------------------------------------

## 13. Condiciones de aplicabilidad

La arquitectura descrita en este documento define un modelo general de
detección de intrusiones basado en un vector ternario de nueve
parámetros, su proyección a un espacio polar y la posterior
clasificación mediante técnicas de aprendizaje profundo. No obstante, su
implementación práctica y su eficacia operativa están condicionadas por
una serie de factores técnicos, organizativos, normativos y de evolución
tecnológica que conviene explicitar.

Las siguientes condiciones de aplicabilidad delimitan el alcance real
del modelo en su estado actual.

### 1. Ámbito de despliegue previsto

El modelo está diseñado para escenarios de dispositivo gestionado en
contextos corporativos, institucionales o equivalentes, en los que:

-   la entidad responsable del despliegue ejerce control legítimo sobre
    la configuración de los dispositivos, y\

-   existe la posibilidad de instalar agentes con permisos avanzados
    (p. ej., *device owner* / *profile owner* en Android, soluciones con
    Network Extension y gestión MDM en iOS).

Este documento no presupone ni garantiza que todas las funciones
descritas sean desplegables como aplicación de consumo general
distribuida a través de tiendas públicas, sin acuerdos previos con los
fabricantes de los sistemas operativos ni sin un marco de administración
de dispositivos móviles (MDM/EMM).

Cualquier extrapolación del modelo a usos fuera de estos contextos
(p. ej., dispositivos personales no gestionados, entornos BYOD sin
políticas claras) requiere un análisis adicional de viabilidad técnica,
jurídica y organizativa.

### 2. Dependencias técnicas externas

La implementabilidad de algunos parámetros y topologías descritos
depende de la disponibilidad y política de uso de determinadas APIs,
*entitlements* y SDKs de terceros:

#### Sistemas operativos móviles (Android / iOS)

En Android, la observación de tráfico, dominios y uso de red por UID se
apoya en componentes como `VpnService`, `NetworkStatsManager` y APIs de
conectividad que, en la práctica, exigen perfiles con permisos
reforzados (p. ej., agentes MDM, *device owner* / *profile owner*).

En iOS, la inspección de tráfico y DNS se basa en Network Extension
(p. ej., `NEFilterDataProvider`, `NEDNSProxyProvider`) y, en su caso, en
`NEHotspotHelper`, cuyo uso está restringido a aplicaciones con
*entitlements* aprobados individualmente por el fabricante.

#### Dispositivos *wearable* con sistema operativo completo

La comunicación reloj--teléfono se fundamenta en APIs como Wearable Data
Layer (Wear OS) o WatchConnectivity (watchOS). El modelo asume la
disponibilidad y estabilidad de estos canales para transmitir telemetría
y parámetros de seguridad de forma fiable.

#### Dispositivos *wearable* sobre RTOS y SDKs de terceros

En relojes basados en RTOS (p. ej., determinadas familias de
dispositivos deportivos), la observación de sensores y eventos depende
de las capacidades expuestas por los SDKs del fabricante (p. ej.,
Connect IQ, Zepp OS u otros).

Cambios en dichos SDKs, en sus políticas de distribución o en la
granularidad de los datos expuestos pueden afectar a la precisión de
algunos parámetros o incluso convertirlos en indeterminados (valor U).

#### Evolución de APIs y políticas de plataforma

Dado que los sistemas operativos móviles y los SDKs de terceros
evolucionan, cualquier implementación deberá incorporar un mecanismo de
revisión periódica de compatibilidad, sustitución de APIs obsoletas y
adaptación a nuevos modelos de permisos.

El presente documento debe entenderse como una fotografía del diseño
arquitectónico en un contexto tecnológico concreto, no como una garantía
de invariabilidad de las plataformas de soporte.

### 3. Limitaciones por plataforma y familia de dispositivo

#### Smartphone y tablet

En dispositivos Android con privilegios suficientes, la cobertura de los
nueve parámetros puede aproximarse a 9/9 en modo autónomo, tal y como se
describe en el cuerpo del documento. No obstante, en iOS la granularidad
de ciertas métricas (especialmente las relativas a uso de red por
aplicación) puede ser inferior, lo que obliga, en algunos casos, a
trabajar con umbrales más conservadores o a considerar valores U en
determinadas configuraciones.

En tablets sin conectividad celular o sin determinados sensores físicos,
algunos parámetros pueden tomar de forma permanente el valor 0 o U
(p. ej., P6 en ausencia de GPS integrado).

#### Relojes con sistema operativo (Wear OS, watchOS, etc.)

El modelo se apoya en una topología híbrida, donde la observación de la
capa de red (P1--P3) se delega en el smartphone, y la capa de
conectividad local y sensores (P4--P9) se mide en el propio reloj.

La afirmación de que determinadas plataformas no ofrecen soporte
generalizado para ciertas funciones (p. ej., inspección de tráfico vía
`VpnService` en el propio reloj) debe entenderse en sentido práctico: el
diseño no depende de disponer de un plano de inspección profundo en el
*wearable*, sino de su acoplamiento con el teléfono gestor.

#### Relojes sobre RTOS

En estos dispositivos, algunos parámetros carecen de sentido físico
(p. ej., P4 y P7 en relojes sin WiFi ni cámara) y otros pueden ser no
observables con las APIs públicas actuales (p. ej., P8 en modelos sin
acceso de terceros al subsistema de micrófono).

Por ello, en esta familia se acepta explícitamente que la cobertura sea
parcial y que ciertos parámetros permanezcan fijados a 0 o U, sin que
ello invalide el modelo, siempre que dicha limitación se refleje
correctamente en la interpretación del vector ternario y en las
políticas de decisión.

### 4. Rendimiento, frecuencia de muestreo y consumo energético

Las estimaciones de tiempos de muestreo y latencia de cálculo indicadas
en el documento (del orden de centenares de milisegundos en smartphone,
segundos en *wearables* y varios segundos en configuraciones acopladas)
deben considerarse valores de referencia, no obligaciones rígidas para
todo despliegue.

En un entorno real, la frecuencia de muestreo de cada parámetro y la
cadencia de actualización del vector ternario deberán ajustarse en
función de:

-   las capacidades de hardware de cada dispositivo,\

-   las restricciones de autonomía y perfiles de uso del usuario,\

-   el nivel de riesgo aceptable en el contexto organizativo.

Será responsabilidad de la implementación concreta definir políticas de
muestreo adaptativo, priorización de parámetros y posibles modos de
funcionamiento (continuo, bajo demanda, degradado) que compatibilicen
seguridad, consumo energético y experiencia de usuario.

### 5. Marco jurídico, privacidad y protección de datos

El modelo propuesto implica la observación de elementos como:

-   dominios y direcciones de red contactados por el dispositivo,\

-   volúmenes de datos transmitidos o recibidos por aplicación o
    interfaz,\

-   acceso a sensores sensibles (ubicación, cámara, micrófono, sensores
    de movimiento),\

-   determinados patrones físicos de uso o manipulación del dispositivo.

La utilización de estas capacidades en un entorno productivo debe
inscribirse en un marco de cumplimiento normativo adecuado, que incluya,
al menos:

-   la existencia de una base jurídica suficiente para el tratamiento de
    dichos datos (p. ej., cumplimiento de obligaciones contractuales,
    interés legítimo correctamente ponderado en el ámbito laboral, o
    cualquier otra prevista por la normativa aplicable),\

-   la información previa y transparente a los usuarios o empleados
    acerca de la existencia de mecanismos de monitorización, su alcance
    y sus finalidades,\

-   la aplicación efectiva del principio de minimización de datos,
    privilegiando el trabajo con resultados agregados o con los propios
    vectores ternarios frente al almacenamiento sistemático de trazas
    crudas de tráfico o de actividad sensorial, salvo que sea
    estrictamente necesario.

El modelo no sustituye a la necesidad de realizar, cuando proceda,
análisis de impacto específicos (p. ej., evaluaciones de impacto en
protección de datos) ni a la adopción de medidas complementarias de
gobernanza y control interno.

### 6. Alcance del documento y evolución futura del modelo

El presente documento tiene carácter fundamentalmente arquitectónico y
metodológico: define el espacio de parámetros, la lógica ternaria, los
criterios de decisión y las topologías de despliegue por familia de
dispositivo.

La eficacia cuantitativa del sistema (tasa de verdaderos positivos, tasa
de falsos positivos, sensibilidad frente a distintos vectores de ataque,
etc.) dependerá de:

-   la calidad y representatividad de los conjuntos de datos utilizados
    para el entrenamiento de los modelos de clasificación (p. ej., redes
    neuronales convolucionales),\

-   el ajuste fino de umbrales y reglas de decisión complementarias,\

-   la integración con otros controles de seguridad presentes en la
    organización.

Cualquier resultado empírico derivado de despliegues piloto o de
experimentos controlados deberá documentarse en informes o publicaciones
posteriores, que complementen este documento con métricas de desempeño y
análisis comparativos.

En consecuencia, el lector debe interpretar este modelo como un marco
estructurado de alto nivel que establece las bases para implementaciones
conformes, sin que ello implique que todas las posibles implantaciones
futuras compartan idénticos niveles de cobertura, granularidad o
rendimiento.

### 7. Tecnologías de instrumentación y continuidad con la publicación original

El marco original del framework (DOI correspondiente) definió ya los
nueve parámetros de comportamiento y presentó un laboratorio de
referencia basado en herramientas concretas de monitorización: sistemas
de detección de intrusiones de red (IDS) como Snort, sistemas de
detección inalámbrica (WIDS) como Kismet y comandos o monitores de
sistema para obtener información de BSSID, tráfico de red y uso de
recursos locales.

En aquel contexto, varios parámetros (especialmente P1, P2 y P3, y parte
de P4--P6) se obtenían a partir de reglas IDS/WIDS y de la inspección de
tráfico en un entorno de PC y red local. Esa implementación sigue siendo
válida como instancia particular del modelo, siempre que la herramienta
elegida proporcione, para cada dispositivo o flujo, la información
necesaria para asignar de forma reproducible los valores 0, 1 o U a los
parámetros definidos en este documento.

El presente "Documento 1" modifica deliberadamente el nivel de
abstracción de la instrumentación: en lugar de fijar el framework a un
conjunto concreto de productos (Snort, Kismet, etc.), describe la
obtención de P1--P9 a través de las APIs oficiales de las plataformas
móviles y *wearables*, organizadas por familias de dispositivo y
topologías de despliegue. Este cambio responde a dos motivos
principales:

1.  La necesidad de cubrir de forma nativa el ecosistema actual
    (smartphones, tablets, relojes inteligentes), en el que la ejecución
    directa de un IDS tipo Snort en el propio dispositivo no es realista
    ni portable.\

2.  El objetivo de evitar un acoplamiento innecesario entre el modelo
    matemático y una herramienta concreta, permitiendo que distintas
    organizaciones utilicen las soluciones de monitorización (IDS, WIDS,
    EDR, agentes MDM, etc.) que mejor se adapten a su infraestructura,
    siempre que respeten la semántica ternaria de los parámetros.

En consecuencia, y a efectos de aplicabilidad:

-   En entornos de red cableada o WiFi corporativa, y en escenarios
    PC/servidor, la capa de observabilidad de red para P1--P4 (y, en su
    caso, parte de P3) puede implementarse mediante IDS/WIDS
    perimetrales (Snort, Suricata, Zeek, Kismet u otros), así como
    mediante sondas de red equivalentes, sin que ello modifique el
    modelo SVcustos.\

-   En entornos móviles y *wearables*, el presente documento adopta como
    referencia la instrumentación basada en agentes sobre el propio
    dispositivo y en APIs de sistema operativo, tal y como se describe
    en las secciones de familias, quedando las soluciones IDS/WIDS como
    complementarias o perimetrales.

De este modo, la publicación original y este Documento 1 se entienden
como capas coherentes del mismo marco: el primero aporta una
implementación histórica de referencia basada en IDS en entornos PC/red;
el segundo abstrae esa idea y la generaliza a múltiples familias de
dispositivos, manteniendo inalterado el significado de los nueve
parámetros y la lógica ternaria de clasificación.

# 14. Material adicional

Tabla de Excel con la **Distribución del espacio ternario 3⁹ = 19.683
vectores.**

\[file element\]

# 15. Referencias

\[1\] Lloret Egea, J.A. et al. «Framework basado en imágenes
parametrizadas sobre ResNet para identificar intrusiones en smartwatches
u otros dispositivos afines». DOI:
https://doi.org/10.21428/39829d0b.981b7276. 2021.

\[2\] He, K., Zhang, X., Ren, S., Sun, J. «Deep Residual Learning for
Image Recognition». CVPR 2016.

\[3\] Android Developers. ConnectivityManager, NetworkStatsManager,
WifiManager, SensorManager, Wearable Data Layer API.
https://developer.android.com/reference

\[4\] Apple Developer. Network Extension, CoreLocation, CoreBluetooth,
CoreMotion, WatchConnectivity. https://developer.apple.com/documentation

\[5\] Garmin Developer. Connect IQ SDK: Sensor API, Position API,
DataField. https://developer.garmin.com/connect-iq/

\[6\] Amazfit/Zepp. Zepp OS SDK (JavaScript).
https://docs.zepp.com/docs/

\[7\] Wear OS Developers. Network access and sync, Health Services API.
https://developer.android.com/training/wearables

\[8\] OWASP. «Mobile Application Security Verification Standard
(MASVS)». https://mas.owasp.org/MASVS/

\[9\] NIST SP 800-124 Rev. 2. «Guidelines for Managing the Security of
Mobile Devices in the Enterprise». 2023.

\[10\] MicroEJ. «Smartwatch Operating Systems».
https://www.microej.com/news/smartwatch-operating-systems/ (datos de
cuota de mercado RTOS \~50%).

------------------------------------------------------------------------

## 14. Próximo documento

**Documento 2: De n=9 a n=16: primera extensión.**

El siguiente documento reorganiza los 9 parámetros base en 4 capas de 4
y añade 7 nuevos: validación TLS, detección NFC, monitorización de
procesos desconocidos, escalada de permisos, exfiltración de datos
personales, conectividad celular y estado de salud del dispositivo. El
espacio ternario crece a `3^16 ≈ 43` millones de vectores. La tabla de
implementación por familia se actualiza con las nuevas APIs requeridas.

------------------------------------------------------------------------

## 15. Mapa completo de la serie

+---+---------------------+---------------------------------------------+
| N | Subtítulo           | Contenido                                   |
| . |                     |                                             |
| º |                     |                                             |
+===+=====================+=============================================+
| 1 | El nivel base: 9    | Restricción n = b², lógica ternaria,        |
|   | parámetros y el     | transformación polar, 9 parámetros,         |
|   | origen del sistema  | familias de dispositivos, topologías.       |
+---+---------------------+---------------------------------------------+
| 2 | De n=9 a n=16:      | 4 capas de 4. Añade TLS, NFC, procesos,     |
|   | primera extensión   | permisos, datos personales, celular.        |
+---+---------------------+---------------------------------------------+
| 3 | De n=16 a n=25:     | 5 capas de 5. Almacenamiento externo, IPC,  |
|   | segunda extensión   | portapapeles, modo avión, sincronización.   |
+---+---------------------+---------------------------------------------+
| 4 | De n=25 a n=36:     | 6 capas de 6. Credenciales, biometría,      |
|   | tercera extensión   | *keystore*, SMS/llamadas, DNS, *logs*.      |
|   |                     | Punto ciego estructural.                    |
+---+---------------------+---------------------------------------------+
| 5 | Arquitectura de     | Dos células independientes para resolver    |
|   | células SV en par:  | los 7 parámetros de integridad del SO.      |
|   | n=36 + n=9          |                                             |
+---+---------------------+---------------------------------------------+
| 6 | De n=36 a n=49:     | 7 capas de 7. Capa MDM (P44--P49).          |
|   | entornos            | Normativa HIPAA/ENS/RGPD.                   |
|   | gestionados y       |                                             |
|   | médicos             |                                             |
+---+---------------------+---------------------------------------------+
| 7 | SVperitus: agentes  | Transposición al conocimiento experto. n =  |
|   | especializados ---  | 625 = 25².                                  |
|   | inmunología         |                                             |
+---+---------------------+---------------------------------------------+
| 8 | Documento           | Unifica los 7 documentos con transiciones y |
|   | compilador de la    | tabla comparativa SVcustos vs SVperitus.    |
|   | serie completa      |                                             |
+---+---------------------+---------------------------------------------+

------------------------------------------------------------------------
