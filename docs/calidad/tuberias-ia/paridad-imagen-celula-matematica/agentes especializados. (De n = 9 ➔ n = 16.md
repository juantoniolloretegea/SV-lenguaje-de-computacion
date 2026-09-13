---
article:
  elocation-id: 5oae74uf
author:
- Juan Antonio Lloret Egea
bibliography: /tmp/tmp-20sFr2cCDO9gCw.json
copyright:
  link: "https://creativecommons.org/licenses/by-nc-nd/4.0/"
  text: Creative Commons Attribution-NonCommercial-NoDerivatives 4.0
    International License
  type: CC-BY-NC-ND
csl: /app/dist/server/server/utils/citations/citeStyles/apa-6th-edition.csl
date:
  day: 03
  month: 03
  year: 2026
journal:
  publisher-name: IA eñ ™ - (La Biblia de la IA - The Bible of AI ™ ISSN
    2695-641)
  title: IA eñ ™
link-citations: true
title: "De SVcustos, el marco (framework) de intrusión, hasta SVperitus:
  agentes especializados. (De n = 9 ➔ n = 16: primera extensión del
  espacio paramétrico)"
uri: "https://www.itvia.online/pub/5oae74uf"
---

![](https://assets.pubpub.org/c39829d0b-6cf2-4c57-8562-04aa66068f1d/p5beeba7c-be08-4b00-83ac-8bec62e604af/u8d2bc69d-73c6-4669-a7e7-fb07fdba4171/svcustos_svperitus_logo_main-41772486283287.png){#nbjk2jq680v}

\[video element\]

# De SVcustos, el marco (framework) de intrusión, hasta SVperitus: agentes especializados. (De  n = 9  ➔  n = 16: primera extensión del espacio paramétrico) {#de-svcustos-el-marco-framework-de-intrusin-hasta-svperitus-agentes-especializados-de-n-9-n-16-primera-extensin-del-espacio-paramtrico}

(Documento 2 de 8)

Pertenece a la colección: «[De SVcustos, el marco (framework) de
intrusión, hasta SVperitus: agentes
especializados](https://www.itvia.online/de-svcustos-el-marco-framework-de-intrusion-hasta-svperitus-agentes-especializados "null")».**
**DOI: 10.21428/39829d0b.1129de25\
\
**Lugar: Madrid**

## **Resumen**

Este segundo documento de la serie *«De SVcustos, el marco (framework)
de intrusión, hasta SVperitus: agentes especializados»* extiende el
nivel base de SVcustos desde 9 a 16 parámetros ternarios (n = 16 = 4²),
organizados en 4 capas de 4 parámetros (Red, Conectividad, Sensores y
Sistema). Se preservan los 9 parámetros originales y se incorporan 7
nuevos vectores de amenaza (TLS, NFC, sensores de salud, procesos,
permisos, datos personales y conexión celular), todos ellos trazables,
al menos a nivel de sistema o MDM, a APIs públicas de Android e iOS y a
escenarios contemporáneos de ataque sobre dispositivos conectados. Sobre
el espacio completo 3¹⁶ (43.046.721 vectores) se formaliza por primera
vez en la serie la regla general de clasificación estricta de SVcustos
(INTRUSIÓN si n₁ ≥ ⌊7n/9⌋, NORMAL si n₀ ≥ ⌊7n/9⌋, INDETERMINADO en el
resto), estableciendo en n = 16 un núcleo algebraico auditable y
simétrico.

El documento describe la transformación de cada vector ternario en un
polígono polar de 16 ejes, interpretable visualmente por capas, y la
construcción de un *pipeline* de clasificación automática basado en
ResNet34. En la fase actual, la red se entrena con vectores sintéticos
etiquetados exclusivamente mediante la regla estricta, de modo que la
arquitectura de IA implementa y pone a prueba la representación
geométrica propuesta y deja preparada la infraestructura técnica para,
en fases posteriores, integrar trazas empíricas e información experta,
especialmente en la región INDETERMINADO. El resultado es un marco
vectorial ternario que combina rigor matemático, diseño orientado al
despliegue sobre familias reales de dispositivos y una arquitectura de
IA ya implementada y preparada para evolucionar, desde el laboratorio,
hacia entornos productivos sin perder la trazabilidad algebraica
original.

## **Abstract**

This second document in the series *"From SVcustos, the intrusion
framework, to SVperitus: specialised agents"* extends the SVcustos base
level from 9 to 16 ternary parameters (n = 16 = 4²), arranged in 4
layers of 4 parameters (Network, Connectivity, Sensors and System). The
9 original parameters are preserved and 7 new threat vectors are
introduced (TLS, NFC, health sensors, processes, permissions, personal
data and cellular connection), all of them traceable, at least at system
or MDM level, to public Android and iOS APIs and to contemporary attack
scenarios on connected devices. Over the full 3¹⁶ space (43,046,721
vectors) the general strict classification rule of SVcustos is
formalised for the first time in the series (INTRUSION if n₁ ≥ ⌊7n/9⌋,
NORMAL if n₀ ≥ ⌊7n/9⌋, INDETERMINATE otherwise), thus establishing, for
n = 16, an auditable and symmetric algebraic core.

The document describes the transformation of each ternary vector into a
16-axis polar polygon, visually interpretable by layers, and the
construction of an automatic classification pipeline based on ResNet34.
In the current phase, the network is trained with synthetic vectors
labelled exclusively by the strict rule, so that the AI architecture
implements and stress-tests the proposed geometric representation and
leaves the technical infrastructure ready to integrate, in later phases,
empirical traces and expert knowledge, particularly within the
INDETERMINATE region. The result is a ternary vector framework that
combines mathematical rigour, a design oriented towards deployment on
real device families, and an AI architecture already implemented and
prepared to evolve from the laboratory to production environments
without losing the original algebraic traceability.

## 1. Posición en la serie {#posicin-en-la-serie}

Este es el segundo documento de una serie de 8 que describe la evolución
completa del sistema SVcustos, desde su nivel base (n = 9) hasta su
transposición al dominio de conocimiento experto (SVperitus, n = 625).

Documento 1: «El nivel base: 9 parámetros y el origen del sistema».
Estableció los fundamentos del sistema: la restricción algebraica n =
b², la lógica ternaria, la transformación polar y el umbral estricto de
intrusión en n₁ ≥ 7 (es decir, n₁ ≥ ⌊7n/9⌋ para n = 9), junto con los 9
parámetros organizados en 3 capas de 3 (Red, Conectividad, Sistema).
Espacio: 3⁹ = 19.683 vectores. La regla general simétrica de
clasificación (INTRUSIÓN si n₁ ≥ ⌊7n/9⌋, NORMAL si n₀ ≥ ⌊7n/9⌋,
INDETERMINADO en el resto) se formaliza por primera vez en este
Documento 2 y se aplicará ya en n = 16 y niveles superiores.

Este documento (Documento 2): presenta la primera extensión del sistema
a n = 16 = 4² parámetros, reorganizados en 4 capas de 4. Describe la
reestructuración de los 9 parámetros originales, los 7 nuevos
parámetros, y la regla de clasificación para un espacio de 3¹⁶ =
43.046.721 vectores ternarios con umbral n₁ ≥ 12.

Los documentos posteriores de la serie son: Documento 3 (n = 16 → n =
25), Documento 4 (n = 25 → n = 36), Documento 5 (Arquitectura de células
SV en par: n = 36 + n = 9), Documento 6 (n = 36 → n = 49), Documento 7
(SVperitus: agentes especializados, n = 625) y Documento 8 (Compilador
de la serie completa).

Más allá de esta serie, los planos de implementación del sistema
---modelo de datos relacional, flujo de entrenamiento con ResNet y
elección tecnológica (Kotlin, SQL y entorno de laboratorio)--- se
describieron previamente en el trabajo «"Framework" basado en imágenes
parametrizadas sobre ResNet para identificar intrusiones en
"smartwatches" u otros dispositivos afines» (Lloret Egea et al., 2021;
DOI: <https://doi.org/10.21428/39829d0b.981b7276>). El presente
Documento 2 asume ese marco de referencia y se centra en la extensión
algebraica a n=16n = 16n=16 y en la formalización de la regla general de
clasificación.

## 2. Estado del sistema en n = 9

El nivel base del sistema SVcustos opera con 9 parámetros ternarios
organizados en 3 capas de 3 (b = 3, n = b² = 9). Cada parámetro toma uno
de tres valores: 0 (normal), 1 (activo/intrusión) o U (indeterminado).
El espacio combinacional es 3⁹ = 19.683 vectores únicos. En el Documento
1, la regla de clasificación se implementó así: INTRUSIÓN si n₁ ≥ 7;
INDETERMINADO si n₁ ∈ {5, 6} y nᵤ ≥ 1; NORMAL en el resto de casos. Esta
elección produjo la distribución 0,828 % / 12,590 % / 86,582 % sobre el
espacio 3⁹ (INTRUSIÓN / INDETERMINADO / NORMAL). En el presente
Documento 2, esta filosofía se formaliza mediante una regla algebraica
general, que se utilizará para n = 16 y niveles superiores: INTRUSIÓN si
n₁ ≥ ⌊7n/9⌋, NORMAL si n₀ ≥ ⌊7n/9⌋, INDETERMINADO en el resto (véase la
sección 8).

+-------------+-------------+-------------+-------------+-------------+
| Cód.        | Nombre      | Capa        | Peso        | Herramienta |
|             |             |             |             | principal   |
+=============+=============+=============+=============+=============+
| P1          | URL no      | Red         | ★ ALTO      | DnsResolver |
|             | autorizada  |             |             | /           |
|             |             |             |             | VpnService  |
+-------------+-------------+-------------+-------------+-------------+
| P2          | C           | Red         | ★ ALTO      | T           |
|             | omunicación |             |             | rafficStats |
|             | no cifrada  |             |             | /           |
|             |             |             |             | VpnService  |
+-------------+-------------+-------------+-------------+-------------+
| P3          | Tr          | Red         | ★ ALTO      | NetworkS    |
|             | ansferencia |             |             | tatsManager |
|             | de datos al |             |             |             |
|             | exterior    |             |             |             |
+-------------+-------------+-------------+-------------+-------------+
| P4          | BSSID no    | C           | ★ ALTO      | WifiManager |
|             | autorizado  | onectividad |             |             |
+-------------+-------------+-------------+-------------+-------------+
| P5          | Bluetooth   | C           | bajo        | Bluet       |
|             | activo      | onectividad |             | oothAdapter |
+-------------+-------------+-------------+-------------+-------------+
| P6          | GPS activo  | C           | ★ ALTO      | Loca        |
|             |             | onectividad |             | tionManager |
+-------------+-------------+-------------+-------------+-------------+
| P7          | Cámara no   | Sistema     | ★ ALTO      | Ca          |
|             | autorizada  |             |             | meraManager |
+-------------+-------------+-------------+-------------+-------------+
| P8          | Micrófono   | Sistema     | ★ ALTO      | Ap          |
|             | no          |             |             | pOpsManager |
|             | autorizado  |             |             | /           |
|             |             |             |             | A           |
|             |             |             |             | udioManager |
+-------------+-------------+-------------+-------------+-------------+
| P9          | Parámetros  | Sistema     | bajo        | Se          |
|             | físicos     |             |             | nsorManager |
|             | anómalos    |             |             |             |
+-------------+-------------+-------------+-------------+-------------+

## 3. Justificación del salto a n = 16 {#justificacin-del-salto-a-n-16}

La restricción algebraica del sistema SVcustos establece que n = b² con
b ≥ 3. El siguiente valor válido tras n = 9 (b = 3) es n = 16 (b = 4).
Este salto no es arbitrario: responde a vectores de amenaza concretos
que el nivel base no cubre.

### 3.1. Vectores de amenaza no cubiertos por n = 9

El nivel base de 9 parámetros deja descubiertos cuatro flancos críticos
que un atacante sofisticado puede explotar:

Ataque MITM sobre canal cifrado: P2 detecta comunicación no cifrada,
pero un atacante que interpone un certificado TLS falso establece un
canal cifrado que P2 no puede distinguir del legítimo. Se necesita P4
(Certificado TLS inválido).

Exfiltración por NFC: El nivel base monitoriza WiFi (P4/P5) y Bluetooth
(P5/P6), pero no la comunicación de campo cercano. Un dispositivo NFC
malicioso en proximidad puede extraer datos sin activar ningún parámetro
de red. Se necesita P8 (NFC no autorizado).

Espionaje de datos de salud: Los wearables con sensores biomédicos
exponen datos de frecuencia cardíaca, oximetría y actividad que pueden
revelar el estado emocional y físico del usuario. Ningún parámetro del
nivel base cubre este vector. Se necesita P11 (Sensores de salud).

Persistencia de malware y escalada: El nivel base detecta indicadores de
red y sensores, pero no la presencia directa de procesos maliciosos
(P12), la escalada silenciosa de permisos (P14), la exfiltración de
datos locales (P15) ni la exfiltración por canal celular independiente
(P16).

### 3.2. Estructura: 4 capas × 4 parámetros {#estructura-4-capas-4-parmetros}

El salto a n = 16 = 4² organiza los parámetros en 4 capas temáticas de 4
parámetros cada una: Red (P1--P4), Conectividad (P5--P8), Sensores
(P9--P12) y Sistema (P13--P16). Esta reorganización implica renumerar
los 9 parámetros originales para que cada capa mantenga coherencia
temática, y situar los 7 nuevos parámetros en las posiciones que
completan cada capa.

## 4. Reestructuración de los 9 parámetros originales en 4 capas {#reestructuracin-de-los-9-parmetros-originales-en-4-capas}

Los 9 parámetros originales no pierden ninguno de sus valores ni se
redefinen. Únicamente se renumeran para encajar en la arquitectura de 4
capas, dejando posiciones libres que serán ocupadas por los 7 nuevos
parámetros. La tabla siguiente muestra la reorganización:

+----------------+----------------+----------------+----------------+
| Capa Red       | Capa           | Capa Sensores  | Capa Sistema   |
| (P1--P4)       | Conectividad   | (P9--P12)      | (P13--P16)     |
|                | (P5--P8)       |                |                |
+================+================+================+================+
| P1 URL no      | P5 BSSID (ex   | P9 Cámara (ex  | P13 Físicos    |
| autorizada P2  | P4) P6         | P7) P10        | (ex P9) P14    |
| No cifrada P3  | Bluetooth (ex  | Micrófono (ex  | Permisos       |
| Trans.         | P5) P7 GPS (ex | P8) P11 Salud  | ★nuevo P15     |
| exterior P4    | P6) P8 NFC     | ★nuevo P12     | Datos pers.    |
| TLS inválido   | ★nuevo         | Proceso ★nuevo | ★nuevo P16     |
| ★nuevo         |                |                | Celular ★nuevo |
+----------------+----------------+----------------+----------------+

Distribución resultante: 14 parámetros de alto peso intrusivo (★ ALTO) y
2 de peso bajo (P6 Bluetooth, P13 Parámetros físicos). Total conservados
del nivel base: 9. Nuevos incorporados: 7 (P4, P8, P11, P12, P14, P15,
P16).

## 5. Los 7 nuevos parámetros: descripción completa {#los-7-nuevos-parmetros-descripcin-completa}

Para cada nuevo parámetro se describe: el vector de amenaza que cubre,
la herramienta real de captura con nombre de API o clase para Android e
iOS, el criterio exacto de ternarización (qué produce valor 0, qué
produce valor 1 y qué produce valor U), su peso intrusivo, y un análisis
adversarial con el argumento técnico contrario más sólido y la réplica
correspondiente.

### 5.1. P4 --- Certificado TLS inválido o autofirmado \[Capa Red\] {#p4-certificado-tls-invlido-o-autofirmado-capa-red}

Peso intrusivo: ★ ALTO

Herramienta de captura: Android: implementación de X509TrustManager
personalizado mediante TrustManagerFactory que intercepta la cadena de
certificados antes de aceptar la conexión. Se valida que el certificado
pertenezca a una CA de confianza del sistema o a una lista blanca de
certificados pinned. OkHttpClient.Builder().sslSocketFactory() para
integración transparente. iOS: SecTrustEvaluateWithError() en
Security.framework para evaluación de la cadena de confianza,
URLSessionDelegate con urlSession(\_:didReceive:completionHandler:) para
interceptar challenges TLS.

Criterio de ternarización: Valor 1: se detecta un certificado TLS que no
pertenece a una CA de confianza del sistema, es autofirmado, está
expirado, o no coincide con el dominio solicitado. Valor 0: todos los
certificados TLS presentados por los servidores contactados son válidos,
firmados por CA reconocida y coinciden con el dominio. Valor U: la
conexión utiliza certificate pinning propietario que impide la
validación estándar, o el handshake TLS fue interrumpido antes de
completar la verificación.

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| El certificate pinning es        | El parámetro no valida el        |
| invasivo: rompe actualizaciones  | certificado del servidor en      |
| legítimas de app cuando el       | abstracto, sino contra la lista  |
| servidor rota su certificado.    | blanca de dominios autorizados   |
| Las CDNs usan wildcard certs que | del dispositivo. Un certificado  |
| no son sospechosos.              | válido para un dominio no        |
|                                  | autorizado sigue disparando P1.  |
|                                  | P4 solo actúa cuando la URL es   |
|                                  | aparentemente autorizada pero el |
|                                  | certificado no cuadra:           |
|                                  | exactamente el escenario MITM.   |
+----------------------------------+----------------------------------+

### 5.2. P8 --- NFC no autorizado \[Capa Conectividad\]

Peso intrusivo: ★ ALTO

Herramienta de captura: Android: NfcAdapter.getDefaultAdapter() +
NfcAdapter.enableReaderMode() con callback para monitorizar sesiones NFC
activas. Se registra el identificador del dispositivo remoto y se
compara con la lista blanca. IsoDep.connect() para distinguir sesiones
de pago (protocolo ISO-DEP/EMV) de sesiones de lectura de datos (NDEF).
iOS: CoreNFC NFCTagReaderSession + NFCNDEFReaderSession para detectar
interacciones NFC. NFCVASReaderSession para sesiones de valor agregado.

Criterio de ternarización: Valor 1: se detecta una sesión NFC de
intercambio de datos con un dispositivo o lector no autorizado, fuera
del protocolo de pago estándar. Valor 0: no hay actividad NFC, o toda la
actividad NFC corresponde a pagos autorizados (ISO-DEP/EMV) o
dispositivos en lista blanca. Valor U: se detecta actividad NFC pero no
se puede determinar el tipo de sesión (chip NFC pasivo sin
identificador, sesión interrumpida antes de completar el handshake).

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| La lista blanca de lectores NFC  | El parámetro no mide si se       |
| es prácticamente imposible de    | realiza un pago sino si se       |
| mantener para un usuario         | establece una sesión NFC de      |
| doméstico. Cualquier terminal de | intercambio de datos fuera de la |
| pago en un comercio nunca estará | app de pago autorizada.          |
| en esa lista: cada pago legítimo | NfcAdapter.enableReaderMode()    |
| dispara P8.                      | distingue sesiones de pago       |
|                                  | (protocolo ISO-DEP/EMV) de       |
|                                  | sesiones de lectura de datos     |
|                                  | (NDEF). Las primeras se          |
|                                  | excluyen; las segundas, si       |
|                                  | provienen de un lector no        |
|                                  | identificado, son las que        |
|                                  | generan valor 1.                 |
+----------------------------------+----------------------------------+

### 5.3. P11 --- Sensores de salud no autorizados \[Capa Sensores\]

Peso intrusivo: ★ ALTO

Herramienta de captura: Android: SensorManager.registerListener()
auditando qué aplicaciones registran listeners para
Sensor.TYPE_HEART_RATE, TYPE_STEP_COUNTER y
TYPE_LOW_LATENCY_OFFBODY_DETECT. Se comparan contra apps con permiso
BODY_SENSORS declarado legítimamente en la política. AppOpsManager para
historial de acceso. iOS: HealthKit
HKHealthStore.authorizationStatus(for:) para verificar qué apps tienen
acceso a datos de salud. CMPedometer y HKObserverQuery para detectar
accesos en tiempo real.

Criterio de ternarización: Valor 1: una aplicación sin permiso
BODY_SENSORS declarado en la política está accediendo a datos de
frecuencia cardíaca, oximetría, pasos o sueño del dispositivo. Valor 0:
ninguna aplicación no autorizada accede a sensores de salud, o el
dispositivo no dispone de dichos sensores. Valor U: el dispositivo
dispone de sensores de salud pero el sistema operativo no ofrece
granularidad suficiente para auditar qué aplicación los consulta
(dispositivos pre-2019 con API de sensores limitada).

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| En versiones anteriores a        | Argumento válido para            |
| Android 10, el permiso           | dispositivos pre-2019. El        |
| BODY_SENSORS es único para todos | sistema se define como           |
| los sensores y no distingue por  | prospectivo, y los dispositivos  |
| tipo. Muchos wearables del       | actuales (Samsung Galaxy Watch   |
| documento original corren        | con One UI Watch, Apple Watch    |
| versiones antiguas de Wear OS    | Series 6+ con watchOS 7+) sí     |
| donde esta auditoría no es       | ofrecen esta granularidad. El    |
| posible.                         | parámetro aplica a instancias    |
|                                  | modernas, no a modelos de 2014.  |
|                                  | Para dispositivos antiguos, el   |
|                                  | valor U es el comportamiento     |
|                                  | correcto.                        |
+----------------------------------+----------------------------------+

### 5.4. P12 --- Proceso desconocido en ejecución \[Capa Sensores\] {#p12-proceso-desconocido-en-ejecucin-capa-sensores}

Peso intrusivo: ★ ALTO

Herramienta de captura: Android:
ActivityManager.getRunningAppProcesses() para lista de procesos activos.
UsageStatsManager.queryUsageStats() para historial de ejecución.
PackageManager.getInstalledPackages() para verificar que cada proceso
corresponde a un paquete conocido. En entornos con privilegios de
sistema: /proc filesystem para inspección directa. iOS: sysctl() con
KERN_PROC para enumeración de procesos. ProcessInfo.processInfo para
información del proceso actual.

Criterio de ternarización: Valor 1: se detecta al menos un proceso en
ejecución cuyo nombre o paquete no aparece en la lista blanca del
dispositivo. Valor 0: todos los procesos en ejecución corresponden a
paquetes conocidos y autorizados. Valor U: la lista de procesos no es
accesible con los privilegios actuales de la aplicación (restricciones
de Android 10+ sobre visibilidad de procesos).

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| Los procesos de sistema del      | La lista blanca se construye en  |
| fabricante (Samsung Health       | la primera ejecución del sistema |
| Service, Bixby Background        | en modo de laboratorio (estado   |
| Service) son opacos y numerosos. | limpio certificado del           |
| La lista blanca inicial sería    | dispositivo). Las                |
| imposible de definir sin conocer | actualizaciones de sistema se    |
| el firmware exacto del           | gestionan con una versión de     |
| dispositivo. Cualquier           | lista blanca vinculada a la      |
| actualización de sistema la      | versión de firmware. Es un       |
| invalida.                        | problema de mantenimiento        |
|                                  | conocido en EDR (Endpoint        |
|                                  | Detection and Response) con      |
|                                  | soluciones establecidas en la    |
|                                  | industria.                       |
+----------------------------------+----------------------------------+

### 5.5. P14 --- Escalada de permisos de aplicación \[Capa Sistema\] {#p14-escalada-de-permisos-de-aplicacin-capa-sistema}

Peso intrusivo: ★ ALTO

Herramienta de captura: Android:
PackageManager.getPackageInfo(packageName, GET_PERMISSIONS) devuelve los
permisos concedidos. Se almacena el estado en la base de datos en el
momento de instalación (baseline) y se compara periódicamente.
AppOpsManager.getOpsForPackage() para verificar uso efectivo de
permisos. iOS: no existe equivalente directo; se monitoriza mediante MDM
profile con app inventory y permission snapshots.

Criterio de ternarización: Valor 1: una aplicación ha adquirido permisos
que no tenía en el baseline de instalación sin acción explícita del
usuario (auto-grant, explotación de vulnerabilidad, downgrade de
protección). Valor 0: los permisos de todas las aplicaciones coinciden
con su baseline o fueron modificados mediante interacción explícita del
usuario (diálogo de concesión estándar del SO). Valor U: la comparación
contra baseline no es posible (primera ejecución sin baseline
disponible, migración de dispositivo, reinstalación del sistema de
monitorización).

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| Una escalada de permisos que     | La base de datos del sistema     |
| ocurrió entre dos capturas y     | almacena el historial completo.  |
| luego fue revertida no se        | La comparación no es             |
| detecta. El parámetro mide       | captura-a-captura sino           |
| estado actual, no historia.      | captura-contra-baseline de       |
|                                  | instalación, que persiste entre  |
|                                  | ciclos y hace auditable          |
|                                  | cualquier cambio anterior,       |
|                                  | aunque el estado actual sea      |
|                                  | idéntico al original.            |
+----------------------------------+----------------------------------+

## 5.6. P15 --- Acceso no autorizado a datos personales \[Capa Sistema\]

Peso intrusivo: ★ ALTO

Herramienta de captura: Android: AppOpsManager.getOpsForPackage()
devuelve el historial de operaciones por app desde Android 11.
Comparación contra permisos declarados: si la app accede a
Contacts.CONTENT_URI sin READ_CONTACTS, o a CallLog.CONTENT_URI sin
READ_CALL_LOG, se genera valor 1. ContentResolver.query() con
observadores registrados. iOS:
CNContactStore.authorizationStatus(for:) + EventKit
EKEventStore.authorizationStatus(for:) para monitorizar autorizaciones
de acceso a datos personales.

Criterio de ternarización: Valor 1: una aplicación ha accedido a
contactos, calendario, mensajes o historial de llamadas sin permiso
explícito declarado, o el acceso es inconsistente con su función
declarada. Valor 0: todos los accesos a datos personales son realizados
por aplicaciones con permisos explícitos y coherentes con su función.
Valor U: el historial de operaciones no es accesible con los privilegios
actuales (requiere permiso GET_APP_OPS_STATS de nivel firma o sistema).

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| AppOpsManager requiere el        | El sistema se concibe con una    |
| permiso GET_APP_OPS_STATS, que   | app con permisos de gestión del  |
| es de nivel firma o sistema. Una | sistema o en el contexto del     |
| app de terceros no puede acceder | sistema operativo del            |
| a estos logs sin privilegios de  | dispositivo. En ese contexto los |
| sistema.                         | privilegios son alcanzables. La  |
|                                  | implementación requiere que la   |
|                                  | app del sistema esté firmada con |
|                                  | el certificado del fabricante,   |
|                                  | lo que es una barrera de entrada |
|                                  | real pero no un obstáculo        |
|                                  | conceptual para el diseño del    |
|                                  | sistema.                         |
+----------------------------------+----------------------------------+

### 5.7. P16 --- Conexión celular no autorizada \[Capa Sistema\] {#p16-conexin-celular-no-autorizada-capa-sistema}

Peso intrusivo: ★ ALTO

Herramienta de captura: Android: TelephonyManager.getDataState() +
getNetworkOperatorName() + getSimOperatorName(). Comparación del
operador activo contra lista blanca de operadores autorizados.
TelephonyManager.listen(PhoneStateListener.LISTEN_DATA_CONNECTION_STATE)
para monitorización en tiempo real. iOS:
CTTelephonyNetworkInfo().serviceCurrentRadioAccessTechnology para
verificar el estado de la conexión celular. CTCarrier para información
del operador activo.

Criterio de ternarización: Valor 1: el dispositivo está transmitiendo
datos a través de una conexión celular (LTE/5G) hacia un operador no
autorizado, o la conexión celular está activa cuando debería estar
deshabilitada por política. Valor 0: no hay conexión celular activa, o
la conexión es con un operador autorizado y dentro de la política. Valor
U: el dispositivo dispone de hardware celular pero no se puede
determinar el operador (SIM no insertada, modo avión parcial, roaming
con identificador de red temporal).

Análisis adversarial:

+----------------------------------+----------------------------------+
| Argumento contrario              | Réplica técnica                  |
+==================================+==================================+
| Solo aplica al subconjunto de    | El documento es explícitamente   |
| dispositivos con conectividad    | prospectivo y contempla la       |
| celular independiente. Muchos    | democratización del 5G como      |
| smartwatches no tienen SIM. P16  | tendencia. Los dispositivos      |
| sería U permanente para la       | actuales con LTE ya tienen       |
| mayoría de esos dispositivos.    | cobertura amplia. El valor U     |
|                                  | para dispositivos sin SIM es el  |
|                                  | comportamiento correcto:         |
|                                  | indeterminado, no falso          |
|                                  | negativo. La cobertura será      |
|                                  | universal en el siguiente ciclo  |
|                                  | tecnológico.                     |
+----------------------------------+----------------------------------+

## 6. Tabla completa de los 16 parámetros {#tabla-completa-de-los-16-parmetros}

+-----+----------+-----+------------+--------+------------+
| C   | P        | C   | H          | Peso   | P          |
| ód. | arámetro | apa | erramienta |        | rocedencia |
+=====+==========+=====+============+========+============+
| P1  | URL no   | Red | D          | ★ ALTO | Original   |
|     | au       |     | nsResolver |        | n=9        |
|     | torizada |     |            |        |            |
+-----+----------+-----+------------+--------+------------+
| P2  | Comu     | Red | Tr         | ★ ALTO | Original   |
|     | nicación |     | afficStats |        | n=9        |
|     | no       |     |            |        |            |
|     | cifrada  |     |            |        |            |
+-----+----------+-----+------------+--------+------------+
| P3  | Trans    | Red | NetworkSt  | ★ ALTO | Original   |
|     | ferencia |     | atsManager |        | n=9        |
|     | de datos |     |            |        |            |
|     | al       |     |            |        |            |
|     | exterior |     |            |        |            |
+-----+----------+-----+------------+--------+------------+
| P4  | Cer      | Red | impl       | ★ ALTO | Nuevo n=16 |
|     | tificado |     | ementación |        |            |
|     | TLS      |     |            |        |            |
|     | inválido |     |            |        |            |
|     | o        |     |            |        |            |
|     | aut      |     |            |        |            |
|     | ofirmado |     |            |        |            |
+-----+----------+-----+------------+--------+------------+
| P5  | BSSID no | Con | W          | ★ ALTO | Original   |
|     | au       | ect | ifiManager |        | n=9        |
|     | torizado | ivi |            |        | (          |
|     |          | dad |            |        | renumerado |
|     |          |     |            |        | de P4)     |
+-----+----------+-----+------------+--------+------------+
| P6  | B        | Con | Blueto     | bajo   | Original   |
|     | luetooth | ect | othAdapter |        | n=9        |
|     | no       | ivi |            |        | (          |
|     | au       | dad |            |        | renumerado |
|     | torizado |     |            |        | de P5)     |
+-----+----------+-----+------------+--------+------------+
| P7  | GPS no   | Con | Locat      | ★ ALTO | Original   |
|     | au       | ect | ionManager |        | n=9        |
|     | torizado | ivi |            |        | (          |
|     |          | dad |            |        | renumerado |
|     |          |     |            |        | de P6)     |
+-----+----------+-----+------------+--------+------------+
| P8  | NFC no   | Con | NfcAdapter | ★ ALTO | Nuevo n=16 |
|     | au       | ect |            |        |            |
|     | torizado | ivi |            |        |            |
|     |          | dad |            |        |            |
+-----+----------+-----+------------+--------+------------+
| P9  | Cámara   | Se  | Cam        | ★ ALTO | Original   |
|     | no       | nso | eraManager |        | n=9        |
|     | au       | res |            |        | (          |
|     | torizada |     |            |        | renumerado |
|     |          |     |            |        | de P7)     |
+-----+----------+-----+------------+--------+------------+
| P10 | M        | Se  | App        | ★ ALTO | Original   |
|     | icrófono | nso | OpsManager |        | n=9        |
|     | no       | res |            |        | (          |
|     | au       |     |            |        | renumerado |
|     | torizado |     |            |        | de P8)     |
+-----+----------+-----+------------+--------+------------+
| P11 | Sensores | Se  | Sen        | ★ ALTO | Nuevo n=16 |
|     | de salud | nso | sorManager |        |            |
|     | no       | res |            |        |            |
|     | aut      |     |            |        |            |
|     | orizados |     |            |        |            |
+-----+----------+-----+------------+--------+------------+
| P12 | Proceso  | Se  | Activ      | ★ ALTO | Nuevo n=16 |
|     | des      | nso | ityManager |        |            |
|     | conocido | res |            |        |            |
|     | en       |     |            |        |            |
|     | e        |     |            |        |            |
|     | jecución |     |            |        |            |
+-----+----------+-----+------------+--------+------------+
| P13 | Pa       | S   | Sen        | bajo   | Original   |
|     | rámetros | ist | sorManager |        | n=9        |
|     | físicos  | ema |            |        | (          |
|     | anómalos |     |            |        | renumerado |
|     |          |     |            |        | de P9)     |
+-----+----------+-----+------------+--------+------------+
| P14 | Escalada | S   | Pack       | ★ ALTO | Nuevo n=16 |
|     | de       | ist | ageManager |        |            |
|     | permisos | ema |            |        |            |
|     | de       |     |            |        |            |
|     | ap       |     |            |        |            |
|     | licación |     |            |        |            |
+-----+----------+-----+------------+--------+------------+
| P15 | Acceso   | S   | App        | ★ ALTO | Nuevo n=16 |
|     | no       | ist | OpsManager |        |            |
|     | au       | ema |            |        |            |
|     | torizado |     |            |        |            |
|     | a datos  |     |            |        |            |
|     | pe       |     |            |        |            |
|     | rsonales |     |            |        |            |
+-----+----------+-----+------------+--------+------------+
| P16 | Conexión | S   | Teleph     | ★ ALTO | Nuevo n=16 |
|     | celular  | ist | onyManager |        |            |
|     | no       | ema |            |        |            |
|     | au       |     |            |        |            |
|     | torizada |     |            |        |            |
+-----+----------+-----+------------+--------+------------+

## 7. Del vector de 16 parámetros al polígono polar {#del-vector-de-16-parmetros-al-polgono-polar}

El vector ternario de 16 componentes se transforma en un polígono en
coordenadas polares siguiendo el mismo principio que el nivel base. Cada
parámetro ocupa un eje de los 16 equiespaciados a 22,5° entre sí (360° /
16 = 22,5°). Los valores lógicos se mapean a radios: valor 0 → radio 1
(mínimo, cerca del centro), valor 1 → radio 2 (máximo, periferia), valor
U → radio 3 (intermedio, zona de incertidumbre).

Esta representación tiene dos propiedades relevantes. Primera: los
vectores con muchos parámetros en valor 1 producen polígonos expandidos
hacia la periferia, visualmente distinguibles de los vectores normales
(compactos cerca del centro). Segunda: la organización en 4 capas
(cuatro ejes consecutivos por capa) hace que los patrones de activación
por capa sean inmediatamente visibles como deformaciones sectoriales del
polígono. Un analista puede identificar a simple vista si la activación
se concentra en la capa de Red (cuadrante 0°--90°), Conectividad
(90°--180°), Sensores (180°--270°) o Sistema (270°--360°).

La imagen polar resultante puede utilizarse directamente como entrada de
la red neuronal convolucional ResNet34, que clasifica la imagen en tres
estados (INTRUSIÓN, INDETERMINADO, NORMAL). La resolución angular de
22,5° es suficiente para que la CNN discrimine las 4 capas sin
ambigüedad.

## 8. Regla de clasificación para n = 16 {#regla-de-clasificacin-para-n-16}

### 8.1. Principio de clasificación estricta {#principio-de-clasificacin-estricta}

El sistema SVcustos mantiene en n = 16 el mismo principio de
clasificación estricta establecido en el nivel base: solo los parámetros
confirmados en valor 1 determinan la entrada en estado de INTRUSIÓN. Los
parámetros en valor U (indeterminados) no cuentan para el umbral de
intrusión, porque la incertidumbre no confirma amenaza.

### 8.2. Umbral estricto para n = 16

La regla de clasificación opera sobre tres conteos:

n₁ = número de parámetros con valor 1 (confirmados activos).

n₀ = número de parámetros con valor 0 (confirmados normales).

nᵤ = número de parámetros con valor U (indeterminados).

La clasificación se determina como sigue:

INTRUSIÓN: n₁ ≥ 12. Al menos 12 de los 16 parámetros están confirmados
en valor 1. El umbral se obtiene de la razón del nivel base: ⌊7 × 16 /
9⌋ = ⌊12,44⌋ = 12.

NORMAL: n₀ ≥ 12. Al menos 12 de los 16 parámetros están confirmados en
valor 0. Simétrica a la regla de intrusión.

INDETERMINADO: todo vector que no cumpla ninguna de las dos condiciones
anteriores. Esta es la zona mayoritaria del espacio, donde opera el
gradiente de riesgo.

### 8.3. Gradiente de riesgo dentro de INDETERMINADO

La zona de INDETERMINADO contiene el 99,841 % del espacio ternario.
Dentro de ella, la puntuación ponderada n₁ + 0,5 × nᵤ no determina la
clasificación, sino que ordena los vectores por su nivel de riesgo para
que los analistas puedan priorizar su inspección. Esta distinción es
fundamental: la duda acumulada eleva la prioridad de inspección, pero no
sustituye la confirmación.

### 8.4. Espacio combinacional

Espacio total: 3¹⁶ = 43.046.721 vectores ternarios únicos.

Esto representa una expansión de factor ×2.187 respecto al nivel base
(43.046.721 / 19.683 = 2.187 = 3⁷), correspondiente a los 7 nuevos
parámetros ternarios añadidos.

## 9. Distribución del espacio ternario {#distribucin-del-espacio-ternario}

La distribución algebraica del espacio de 3¹⁶ = 43.046.721 vectores bajo
la regla estricta (n₁ ≥ 12 para INTRUSIÓN, n₀ ≥ 12 para NORMAL) es:

  Clasificación   Vectores     Porcentaje
  --------------- ------------ ------------
  INTRUSIÓN       34.113       0,0792 %
  INDETERMINADO   42.978.495   99,8415 %
  NORMAL          34.113       0,0792 %

La concentración del 99,84 % del espacio en INDETERMINADO es
consecuencia directa de la exigencia del umbral estricto: con 16
parámetros, se necesitan al menos 12 confirmados en valor 1 para
clasificar como INTRUSIÓN. Esto confirma que la clasificación algebraica
actúa como primer filtro del sistema y que, en esta fase del proyecto,
todas las etiquetas de los vectores ternarios se generan exclusivamente
mediante esta regla estricta. La red neuronal ResNet34 se entrena para
aproximar dicha función de decisión a partir de la imagen polar, con un
doble objetivo: validar que la representación geométrica es
suficientemente discriminativa y dejar preparada la infraestructura
técnica para futuras fases en las que puedan incorporarse distribuciones
empíricas procedentes de datos reales o de juicios de experto,
especialmente dentro de la región INDETERMINADO.

### 9.1. Desglose de INTRUSIÓN por nivel de activación {#desglose-de-intrusin-por-nivel-de-activacin}

  n₁   Combinaciones   Vectores   Subtotal
  ---- --------------- ---------- -----------
  12   C(16,12) × 2⁴   29.120     85,37 %
  13   C(16,13) × 2³   4.480      13,13 %
  14   C(16,14) × 2²   480        1,41 %
  15   C(16,15) × 2¹   32         0,09 %
  16   C(16,16) × 2⁰   1          \< 0,01 %

## 10. Tres ejemplos de vectores ilustrativos

### 10.1. Ejemplo INTRUSIÓN {#ejemplo-intrusin}

Vector: (1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1)

Conteo: n₁ = 14, n₀ = 2, nᵤ = 0. Clasificación: INTRUSIÓN (n₁ = 14 ≥
12).

Interpretación: todos los parámetros de las capas Red, Sensores y la
mayoría de Conectividad y Sistema están confirmados activos. Solo P6
(Bluetooth) y P13 (Parámetros físicos) están en estado normal, ambos de
peso bajo. El polígono polar resultante ocupa casi todo el anillo
externo de radio 2 en 14 de los 16 ejes, con solo dos picos hacia el
centro. Gradiente de riesgo: 14,0.

### 10.2. Ejemplo NORMAL

Vector: (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, U, 0, 0, U, 0, U)

Conteo: n₁ = 0, n₀ = 13, nᵤ = 3. Clasificación: NORMAL (n₀ = 13 ≥ 12).

Interpretación: 13 de los 16 parámetros confirman estado normal. Los
tres indeterminados (P11 Salud, P14 Permisos, P16 Celular) no afectan a
la clasificación porque el conteo de normales ya supera el umbral. El
polígono polar es compacto, pegado al centro en la mayoría de ejes.
Gradiente de riesgo: 1,5.

### 10.3. Ejemplo INDETERMINADO

Vector: (1, 1, 0, 1, U, 0, 1, 1, 1, U, 1, 0, U, 1, 0, 1)

Conteo: n₁ = 9, n₀ = 4, nᵤ = 3. Clasificación: INDETERMINADO (n₁ = 9 \<
12 y n₀ = 4 \< 12).

Interpretación: 9 parámetros confirmados activos y 3 indeterminados. Si
los 3 valores U se resolvieran como 1, n₁ llegaría a 12 (umbral de
intrusión). El gradiente de riesgo es 9 + 0,5 × 3 = 10,5, que lo sitúa
en la franja de alta prioridad dentro de INDETERMINADO. Un analista
debería resolver los valores U de P5, P10 y P13 antes de tomar una
decisión.

## 11. Valoración de implementación {#valoracin-de-implementacin}

El salto de n = 9 a n = 16 es técnicamente viable con las APIs actuales
de Android e iOS. Los 7 nuevos parámetros utilizan APIs públicas
documentadas, aunque algunos de ellos (en particular P12 --- Proceso
desconocido en ejecución y P15 --- Acceso no autorizado a datos
personales) requieren privilegios de sistema o integración en entornos
MDM, lo que limita su instrumentación práctica a contextos donde la
aplicación de monitorización esté firmada por el fabricante o desplegada
como agente de seguridad gestionado.

El espacio combinacional de 43.046.721 vectores ternarios es manejable
computacionalmente: la generación completa mediante `itertools.product`
requiere del orden de minutos en hardware estándar, y la clasificación
algebraica de cada vector es una operación de conteo trivial sobre n₁,
n₀ y nᵤ. En la implementación actual, el conjunto de datos para
entrenamiento se construye generando vectores ternarios sintéticos,
etiquetados exclusivamente mediante esta regla algebraica estricta, y
transformándolos en sus correspondientes imágenes polares de 16 ejes. El
entrenamiento de la ResNet34 sobre dichas imágenes es el paso que exige
mayor capacidad de cómputo, pero se mantiene dentro de los rangos
habituales de entrenamiento de redes convolucionales en tareas estándar
de clasificación de imágenes.

La reorganización de 3 capas a 4 capas es transparente para el sistema
de clasificación: la regla estricta opera sobre conteos globales, no
sobre la estructura de capas. Las capas constituyen una organización
semántica para el analista humano y un soporte para la interpretabilidad
visual del polígono polar (identificación inmediata de sectores de
riesgo por familia de parámetros), pero no modifican la lógica de
decisión algebraica del modelo.

En esta fase del proyecto, la red ResNet34 no altera la regla de
decisión del sistema: se limita a aproximar, a partir de las imágenes
polares, la función de clasificación definida por SVcustos en n = 16 y a
validar la idoneidad de la representación geométrica para su uso en
clasificación visual. La incorporación de distribuciones empíricas
procedentes de datos reales o de juicios de experto, especialmente
dentro de la región INDETERMINADO, se reserva explícitamente para fases
posteriores del proyecto, en las que la misma arquitectura podrá
integrar información adicional más allá del conteo estrictamente
algebraico.

### 11.1. Condiciones de aplicabilidad en n = 16

Por último, la completitud del vector de 16 parámetros depende de la
topología de despliegue elegida (autónoma, acoplada o externa) y de la
familia de dispositivos (smartwatch, smartphone, tableta, PC, domótica
KNX). La tabla de implementación por familia anunciada en el Documento 1
se mantiene conceptualmente alineada con esta extensión y se detallará
en un anexo técnico específico, donde se indicarán, para cada familia y
topología, los parámetros efectivamente observables y las APIs
utilizadas.

Además, algunos parámetros (P12 --- Proceso desconocido en ejecución,
P15 --- Acceso a datos personales) requieren privilegios de sistema o
integración MDM para poder observar la información relevante. En
dispositivos gestionados por el usuario final sin agente de seguridad
privilegiado, estos parámetros no deben asumirse observables y el modelo
debe tratar explícitamente los valores U resultantes.

La extensión de n = 9 a n = 16 añade, no obstante, varios condicionantes
específicos. En particular, los parámetros P10--P16 dependen de APIs de
plataforma equivalentes a las citadas en la Tabla 7 (TLS, NFC, sensores
de salud, procesos, datos personales y alertas). En plataformas que
carezcan de estas APIs, o donde su uso esté restringido, dichos
parámetros sólo podrán fijarse en 0 o U, o quedar fuera de la
instrumentación efectiva.

Las condiciones generales de aplicabilidad del modelo descritas en el
Documento 1 se mantienen sin cambios: SVcustos sigue siendo un marco
arquitectónico de alto nivel cuya implementación depende de las
capacidades concretas de cada familia de dispositivos y de las
restricciones normativas del entorno donde se despliega.

## 12. DataSet

**Dataset y código fuente.** El dataset completo para entrenamiento de
ResNet, incluyendo el generador de imágenes polares y los scripts de
entrenamiento y evaluación, está disponible en GitHub:
<https://github.com/juantoniolloretegea/SVcustos-dataset>

## 13. Referencias

Lloret Egea, J. A., Hernández González, A., Díaz Raboso, D., Campos, C.,
Riveros Guzmán, K., Cortés Carballo, L. M., Terrés Lloret, H. M., &
IAeñTM, I. tecnológico virtual. (2021). "Framework" basado en imágenes
parametrizadas sobre ResNet para identificar intrusiones en
"smartwatches" u otros dispositivos afines. (Un eje singular de la
publicación "Estado del arte de la ciencia de datos en el idioma español
y su aplicación en el campo de la Inteligencia Artificial"). IA Eñ TM.
<https://doi.org/10.21428/39829d0b.981b7276>

Lloret Egea, J. A. (2026). De SVcustos, el marco (framework) de
intrusión, hasta SVperitus: agentes especializados. Documento 1 de 8: El
nivel base: 9 parámetros y el origen del sistema. IA eñ ™.

He, K., Zhang, X., Ren, S., & Sun, J. (2016). Deep Residual Learning for
Image Recognition. Proceedings of the IEEE Conference on Computer Vision
and Pattern Recognition (CVPR), 770-778.

Android Developers. (2025). Android API Reference: TrustManagerFactory,
NfcAdapter, AppOpsManager, TelephonyManager.
https://developer.android.com/reference

Apple Developer. (2025). iOS SDK Documentation: Security Framework,
CoreNFC, HealthKit, CoreTelephony.
https://developer.apple.com/documentation

## 14. Próximo documento {#prximo-documento}

Documento 3 de 8: «De n = 16 a n = 25: segunda extensión del espacio
paramétrico». El tercer documento de la serie presenta la segunda
extensión del sistema, que amplía el vector a 5 capas de 5 parámetros (n
= 25 = 5²) e incorpora 9 nuevos parámetros (P17--P25) que cubren
almacenamiento externo, modificación de sistema, comunicación
inter-procesos anómala, portapapeles, configuración de red, movimiento,
modo avión, sincronización externa y notificaciones push. El espacio
combinacional se expande a 3²⁵ ≈ 847.000 millones de vectores ternarios.
El umbral estricto de intrusión se recalibrará a n₁ ≥ 19 (razón ⌊7 × 25
/ 9⌋).
