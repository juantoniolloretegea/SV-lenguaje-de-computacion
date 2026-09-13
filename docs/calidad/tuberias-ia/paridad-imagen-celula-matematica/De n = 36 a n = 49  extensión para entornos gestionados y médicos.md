---
article:
  elocation-id: de-svcustos-el-marco-de-intrusion-hasta-svperitus-de-n--36-a-n--49-extension-para-entornos-gestionados-y-medicos
author:
- Juan Antonio Lloret Egea
bibliography: /tmp/tmp-20oQVvm3QMoluV.json
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
title: "De SVcustos, el marco de intrusión, hasta SVperitus. (De n = 36
  a n = 49: extensión para entornos gestionados y médicos)"
uri: "https://www.itvia.online/pub/de-svcustos-el-marco-de-intrusion-hasta-svperitus-de-n--36-a-n--49-extension-para-entornos-gestionados-y-medicos"
---

![](https://assets.pubpub.org/c39829d0b-6cf2-4c57-8562-04aa66068f1d/p345e290d-3eb6-4ac0-a677-fb26dbdd8c37/u8d2bc69d-73c6-4669-a7e7-fb07fdba4171/image-31772631216336.png){#n1d55z2kgv3}

\[video element\]

# **De SVcustos, el marco de intrusión, hasta SVperitus. **De   n = 36   a    n = 49: extensión para entornos gestionados y médicos {#de-svcustos-el-marco-de-intrusin-hasta-svperitus-de-n-36-a-n-49-extensin-para-entornos-gestionados-y-mdicos}

**(Documento 6 de 8)**

------------------------------------------------------------------------

**ISSN 2695-641**. Madrid, 4 de marzo de 2026

------------------------------------------------------------------------

## 1. Resumen

El Documento 6 extiende la arquitectura SVcustos a n = 49 = 7²
parámetros, reorganizados en 7 capas temáticas de 7 parámetros cada una.
Las seis primeras capas preservan los 36 parámetros del sistema base (n
= 36) en sus primeras seis posiciones y añaden cada una un séptimo
parámetro orientado a entornos gestionados. La séptima capa, denominada
**Gestión MDM**, incorpora 7 parámetros exclusivos de dispositivos bajo
gestión empresarial o clínica. En total, la célula SV(49, 7) introduce
**13 nuevos parámetros (P37--P49)** respecto al sistema base n = 36.

El umbral de clasificación sigue la regla T(n) = ⌊7n/9⌋ = ⌊7×49/9⌋ =
**38**. El espacio combinacional es 3⁴⁹ ≈ 2,39 × 10²³ vectores
ternarios. El documento incluye análisis normativo bajo HIPAA §164.312,
Esquema Nacional de Seguridad (ENS), RGPD Art. 32 y el Reglamento EU
2017/745 de productos sanitarios, y establece la frontera de decisión
entre desplegar SV(49, 7) como célula única o el par de células SV(36,
6) + SV(9, 3) definido en el Documento 5.

------------------------------------------------------------------------

*Document 6 extends the SVcustos architecture to n = 49 = 7² parameters
organized into 7 thematic layers of 7 parameters each. The first six
layers preserve the 36 parameters of the base system and add one
managed-environment-specific parameter per layer; the seventh layer (MDM
Management) contributes 7 parameters exclusive to enterprise- or
clinically-managed devices. The classification threshold follows T(n) =
⌊7n/9⌋ = 38. The combinatorial space is 3⁴⁹ ≈ 2.39 × 10²³ ternary
vectors. The document includes a normative analysis under HIPAA, Spain's
ENS, GDPR and EU Regulation 2017/745, and defines the decision boundary
between deploying SV(49, 7) as a single cell or the dual-cell pair
SV(36, 6) + SV(9, 3) from Document 5.*

------------------------------------------------------------------------

## 2. Posición en la serie

La serie «De SVcustos hasta SVperitus» estudia la arquitectura SVcustos
desde su célula mínima (n = 9, Documento 1) hasta su instancia de mayor
complejidad orientada a seguridad (n = 49, este documento) y su
trasposición al dominio de conocimiento experto (n = 625, Documento 7).

Los cinco documentos anteriores construyeron el sistema de forma
incremental: n = 9 → 16 → 25 → 36 (documentos 1 a 4) y, en el Documento
5, establecieron la arquitectura de célula en par SV(36, 6) + SV(9, 3)
como solución al problema de los parámetros de integridad del sistema
operativo. El Documento 6 define una **segunda frontera de despliegue**:
la célula única SV(49, 7), pensada para entornos donde la gestión
centralizada de dispositivos (MDM/EMM) es un requisito.

> **IMPORTANTE:** La célula SV(49, 7) NO sustituye ni reemplaza al par
> SV(36, 6) + SV(9, 3). Ambas arquitecturas son complementarias y
> atienden fronteras de despliegue distintas. La elección entre una y
> otra depende del entorno operativo, el nivel de gestión disponible y
> los requisitos normativos aplicables (véase Sección 12).

## 3. Recapitulación del recorrido n = 9 → 36

Los documentos previos de la serie establecieron un sistema que crece de
forma cuadrada perfecta (n = b²) y reorganiza sus parámetros en b capas
de b en cada extensión.

+-----+---+----------------+----------------------+---------------------+
| D   | n | Arquitectura   | Umbral T(n)          | Espacio             |
| oc. |   |                |                      |                     |
+=====+===+================+======================+=====================+
| 1   | 9 | 3×3            | n₁ ≥ 7 \| n₀ ≥ 7     | 3⁹ = 19.683         |
+-----+---+----------------+----------------------+---------------------+
| 2   | 1 | 4×4            | n₁ ≥ 12 \| n₀ ≥ 12   | 3¹⁶ ≈ 4,30 × 10⁷    |
|     | 6 |                |                      |                     |
+-----+---+----------------+----------------------+---------------------+
| 3   | 2 | 5×5            | n₁ ≥ 19 \| n₀ ≥ 19   | 3²⁵ ≈ 8,47 × 10¹¹   |
|     | 5 |                |                      |                     |
+-----+---+----------------+----------------------+---------------------+
| 4   | 3 | 6×6            | n₁ ≥ 28 \| n₀ ≥ 28   | 3³⁶ ≈ 1,50 × 10¹⁷   |
|     | 6 |                |                      |                     |
+-----+---+----------------+----------------------+---------------------+
| 5   | 4 | 36+9 (par)     | max(cls₃₆, cls₉)     | 3⁴⁵ ≈ 2,95 × 10²¹   |
|     | 5 |                |                      |                     |
+-----+---+----------------+----------------------+---------------------+
| **  | * | **7×7 (este    | **n₁ ≥ 38 \| n₀ ≥    | **3⁴⁹ ≈ 2,39 ×      |
| 6** | * | doc.)**        | 38**                 | 10²³**              |
|     | 4 |                |                      |                     |
|     | 9 |                |                      |                     |
|     | * |                |                      |                     |
|     | * |                |                      |                     |
+-----+---+----------------+----------------------+---------------------+

## 4. Premisas de aplicación para SV(49, 7)

La viabilidad técnica y empírica de los nuevos parámetros P37--P49 está
condicionada a la satisfacción de las siguientes premisas. Son
condiciones necesarias, no suficientes. Su incumplimiento no invalida
los niveles n = 9 a n = 36, que continúan siendo aplicables en
dispositivos de consumo, ni la arquitectura de par de células del
Documento 5.

+---------+-----------------+------------------------------------------+
| ID      | Premisa         | Descripción técnica                      |
+=========+=================+==========================================+
| P-1     | Gestión         | El dispositivo debe estar enrolado en un |
|         | centralizada    | sistema MDM activo (Google Android       |
|         | MDM activa      | Enterprise, Microsoft Intune, VMware     |
|         |                 | Workspace ONE u equivalente) con perfil  |
|         |                 | de trabajo y políticas de seguridad      |
|         |                 | aplicadas de forma forzosa. Condición    |
|         |                 | necesaria para P43, P44, P45, P47 y P49. |
+---------+-----------------+------------------------------------------+
| P-2     | App del         | La aplicación debe estar firmada con el  |
|         | framework       | certificado del fabricante o con un      |
|         | firmada con     | certificado de perfil empresarial de     |
|         | certificado de  | nivel *platform*. Necesario para acceder |
|         | nivel sistema o | a AccessibilityManager (P37),            |
|         | empresa         | MediaProjectionManager (P38),            |
|         |                 | IFF_PROMISC flag (P41) y BackupManager   |
|         |                 | (P46).                                   |
+---------+-----------------+------------------------------------------+
| P-3     | Dispositivo en  | Los dispositivos en producción clínica o |
|         | modo producción | corporativa deben tener el modo          |
|         | sin modo        | desarrollador desactivado como condición |
|         | desarrollador   | de partida. P47 detecta la violación de  |
|         | habilitado      | esta premisa con valor diagnóstico       |
|         |                 | máximo.                                  |
+---------+-----------------+------------------------------------------+
| P-4     | Política de     | El MDM debe imponer PIN de al menos 6    |
|         | pantalla de     | dígitos o autenticación biométrica con   |
|         | bloqueo de      | factor secundario. P45 detecta la        |
|         | nivel alto      | eliminación de esta política. Sin P-4,   |
|         | aplicada por    | P45 no tiene baseline contra el que      |
|         | MDM             | comparar.                                |
+---------+-----------------+------------------------------------------+
| P-5     | Entorno de red  | Todo el tráfico debe circular por la VPN |
|         | corporativa con | corporativa o clínica. P44 detecta       |
|         | VPN obligatoria | tráfico fuera del túnel. Ventana de      |
|         |                 | tolerancia para reconexiones legítimas:  |
|         |                 | 30 s (el 95% de las reconexiones ocurren |
|         |                 | en menos de 30 s según datos de los      |
|         |                 | principales MDMs del mercado             |
|         |                 | hospitalario).                           |
+---------+-----------------+------------------------------------------+
| P-6     | Justificación   | Los dispositivos clínicos manejan datos  |
|         | reforzada para  | de salud (categoría especial, Art. 9     |
|         | entorno médico  | RGPD). Un atacante con control del       |
|         | (datos Art. 9   | servicio de accesibilidad (P37) o con    |
|         | RGPD)           | capacidad de proyección de pantalla      |
|         |                 | (P38) puede manipular los valores        |
|         |                 | visualizados en la interfaz clínica sin  |
|         |                 | modificar los datos en la base de datos: |
|         |                 | un escenario invisible para cualquier    |
|         |                 | monitor de integridad de datos. La       |
|         |                 | detección a nivel de comportamiento del  |
|         |                 | SO es la única capa de defensa contra    |
|         |                 | este vector.                             |
+---------+-----------------+------------------------------------------+

Un centro sanitario que cumple el ENS en nivel Alto ya satisface P-1
(MDM activo), P-4 (política de bloqueo forzada) y P-5 (VPN obligatoria).
La instrumentación adicional necesaria para capturar P37--P49 se reduce
a instalar el agente SVcustos con los permisos de sistema apropiados,
desplegable de forma centralizada por el propio MDM.

## 5. Problema: ¿por qué n = 36 no es suficiente en entornos gestionados?

El sistema SV(36, 6) fue diseñado para dispositivos de consumo operando
de forma autónoma: detecta comportamientos anómalos en las dimensiones
de red, conectividad, almacenamiento, contexto, autenticación y
comunicaciones. Este diseño no contempla la dimensión de gestión
centralizada que existe en entornos empresariales y clínicos.

En un entorno gestionado, existe un servidor MDM/EMM que impone
políticas, sincroniza configuraciones y mantiene un canal de auditoría
permanente. La ausencia de parámetros que supervisen el **cumplimiento**
de esas políticas crea un punto ciego estructural: un atacante puede
eliminar el perfil MDM, desactivar la VPN corporativa y manipular el
reloj del sistema sin que ningún parámetro de n = 36 lo detecte.

Los marcos normativos aplicables a entornos clínicos (HIPAA, EU
2017/745) y corporativos (ENS, RGPD) exigen controles técnicos
específicos ---cifrado en reposo, integridad de logs, gestión de claves
en hardware--- que no tienen representación paramétrica en n = 36.

El par SV(36, 6) + SV(9, 3) del Documento 5 resuelve el problema de los
parámetros de integridad del sistema operativo, pero no aborda la capa
de gestión MDM. La célula SV(49, 7) integra ambas dimensiones en una
arquitectura unificada orientada al despliegue gestionado.

## 6. Solución: SV(49, 7) --- 7 capas de 7 parámetros

La célula SV(49, 7) reorganiza los 36 parámetros existentes en 7 capas,
ocupando las posiciones 1 a 6 de cada una de las seis primeras capas.
Cada capa gana una séptima posición con un parámetro nuevo orientado a
entorno gestionado (P37--P42). La séptima capa, **Gestión MDM**
(P43--P49), es completamente nueva y sin equivalente en n = 36.

+-------------------------+------------+----------------------------------------------+
| Capa                    | Parámetros | Descripción / Nuevos en SV(49,7)             |
+=========================+============+==============================================+
| C1 --- Red              | P1--P6,    | Red, inspección TLS. Nuevo: **P37** Proxy    |
|                         | **P37**    | SSL/TLS corporativo desactivado              |
+-------------------------+------------+----------------------------------------------+
| C2 --- Conectividad     | P7--P12,   | Interfaces inalámbricas. Nuevo: **P38**      |
|                         | **P38**    | Canal MDM no alcanzable (silencio)           |
+-------------------------+------------+----------------------------------------------+
| C3 --- Almacenamiento   | P13--P18,  | Almacenamiento e integridad. Nuevo: **P39**  |
|                         | **P39**    | Cifrado en reposo desactivado                |
+-------------------------+------------+----------------------------------------------+
| C4 --- Contexto         | P19--P24,  | Contexto físico y geográfico. Nuevo: **P40** |
|                         | **P40**    | Geofencing corporativo violado               |
+-------------------------+------------+----------------------------------------------+
| C5 --- Autenticación    | P25--P30,  | Credenciales y acceso. Nuevo: **P41**        |
|                         | **P41**    | Política de contraseñas MDM incumplida       |
+-------------------------+------------+----------------------------------------------+
| C6 ---                  | P31--P36,  | Evasión y exfiltración. Nuevo: **P42** Canal |
| Comunicaciones/Evasión  | **P42**    | MDM bloqueado activamente                    |
+-------------------------+------------+----------------------------------------------+
| **C7 --- Gestión MDM**  | **         | **Capa nueva exclusiva:** perfil MDM, VPN,   |
|                         | P43--P49** | bloqueo, backup, ADB, reloj, restricciones   |
+-------------------------+------------+----------------------------------------------+

## 7. Parámetros: definición y análisis adversarial

A continuación se detallan los 49 parámetros de SV(49, 7) organizados
por capa. Para cada parámetro se incluye el código, nombre, peso
intrusivo, herramienta de medición en Android, criterio de ternarización
y análisis adversarial (argumento contra / réplica).

------------------------------------------------------------------------

### 7.1. Capa 1 --- Red (P1--P6, P37)

Tráfico de red, inspección de tráfico y protocolos de comunicación
seguros, ampliada con supervisión de inspección SSL/TLS corporativa.

*Marco normativo: Art. 164 HIPAA (transmisión); ENS MP.COM.1; RGPD Art.
32.*

+----+--------+------+--------------------+------+----------------------+
| Có | Nombre | Peso | Herramienta        | V    | Criterio de          |
| d. |        |      | (Android)          | alor | ternarización        |
|    |        |      |                    | 1    |                      |
+====+========+======+====================+======+======================+
| P1 | URL no | ★    | `Netw              | 1    | Solicitud HTTP/S a   |
|    | auto   | ALTO | orkSecurityPolicy` |      | dominio no incluido  |
|    | rizada |      |                    |      | en lista blanca      |
|    |        |      |                    |      | corporativa          |
+----+--------+------+--------------------+------+----------------------+
| P2 | Comuni | ★    | `Netw              | 1    | Tráfico HTTP en      |
|    | cación | ALTO | orkSecurityConfig` |      | claro detectado      |
|    | no     |      |                    |      | (*cleartext traffic  |
|    | c      |      |                    |      | enabled*)            |
|    | ifrada |      |                    |      |                      |
+----+--------+------+--------------------+------+----------------------+
| P3 | T      | ★    | `TrafficStats`     | 1    | Bytes salientes \>   |
|    | ransfe | ALTO |                    |      | umbral hacia IPs     |
|    | rencia |      |                    |      | externas no          |
|    | de     |      |                    |      | registradas          |
|    | datos  |      |                    |      |                      |
|    | al     |      |                    |      |                      |
|    | ex     |      |                    |      |                      |
|    | terior |      |                    |      |                      |
+----+--------+------+--------------------+------+----------------------+
| P4 | Certi  | ★    | `X509TrustManager` | 1    | Certificado          |
|    | ficado | ALTO |                    |      | expirado,            |
|    | TLS    |      |                    |      | autofirmado no en    |
|    | in     |      |                    |      | truststore o cadena  |
|    | válido |      |                    |      | CA rota              |
+----+--------+------+--------------------+------+----------------------+
| P5 | C      | ★    | `Co                | 1    | Cambio en proxy APN, |
|    | onfigu | ALTO | nnectivityManager` |      | DNS primario o       |
|    | ración |      |                    |      | configuración Wi-Fi  |
|    | de red |      |                    |      | fuera del proceso    |
|    | modi   |      |                    |      | MDM                  |
|    | ficada |      |                    |      |                      |
+----+--------+------+--------------------+------+----------------------+
| P6 | DNS    | ★    | `LinkProperties`   | 1    | DNS resolver no      |
|    | modi   | ALTO |                    |      | corresponde al       |
|    | ficado |      |                    |      | asignado por MDM;    |
|    | o      |      |                    |      | respuesta DNS apunta |
|    | rebind |      |                    |      | a IP privada         |
+----+--------+------+--------------------+------+----------------------+
| *  | *      | ★    | `ProxyInfo`        | 1    | El proxy de          |
| *P | *Proxy | ALTO |                    |      | inspección TLS       |
| 37 | de     |      |                    |      | exigido por política |
| ** | insp   |      |                    |      | MDM no está activo o |
|    | ección |      |                    |      | su CA no está        |
|    | S      |      |                    |      | instalada            |
|    | SL/TLS |      |                    |      |                      |
|    | corpo  |      |                    |      |                      |
|    | rativa |      |                    |      |                      |
|    | d      |      |                    |      |                      |
|    | esacti |      |                    |      |                      |
|    | vado** |      |                    |      |                      |
+----+--------+------+--------------------+------+----------------------+

**Análisis adversarial --- Capa 1 (Red)**

+-----+----------------------------------------+----------------------+
| C   | Argumento contra (falsos positivos)    | Réplica del sistema  |
| ód. |                                        |                      |
+=====+========================================+======================+
| P1  | Servicios cloud legítimos con dominios | P1 opera con lista   |
|     | dinámicos (CDN, telemetría de apps     | blanca dinámica      |
|     | autorizadas) pueden activar el         | sincronizada con     |
|     | parámetro.                             | MDM, incluyendo      |
|     |                                        | rangos CDN           |
|     |                                        | autorizados.         |
+-----+----------------------------------------+----------------------+
| P2  | Algunas APIs médicas heredadas y       | P2 activa            |
|     | sistemas LIS/RIS no soportan TLS.      | INDETERMINADO cuando |
|     |                                        | el destino consta en |
|     |                                        | lista de excepciones |
|     |                                        | aprobadas por el     |
|     |                                        | CISO. Fuera de       |
|     |                                        | lista: INTRUSIÓN.    |
+-----+----------------------------------------+----------------------+
| P3  | Backups legítimos, telemetría del EMM  | El EMM mantiene      |
|     | y actualizaciones OTA generan tráfico  | ventanas de backup   |
|     | saliente voluminoso.                   | programadas. P3      |
|     |                                        | compara volumen y    |
|     |                                        | horario con el       |
|     |                                        | perfil autorizado.   |
+-----+----------------------------------------+----------------------+
| P4  | Certificados de CA interna empresarial | El perfil MDM        |
|     | no incluidos en el truststore del      | distribuye el anchor |
|     | sistema.                               | de CA corporativa en |
|     |                                        | el truststore        |
|     |                                        | gestionado.          |
+-----+----------------------------------------+----------------------+
| P5  | Redes Wi-Fi visitadas (hospitales con  | P5 distingue cambios |
|     | portales cautivos) pueden requerir     | iniciados por el     |
|     | configuración manual.                  | usuario de los       |
|     |                                        | propagados por MDM.  |
|     |                                        | Los cambios manuales |
|     |                                        | en dispositivos      |
|     |                                        | gestionados se       |
|     |                                        | registran como 1.    |
+-----+----------------------------------------+----------------------+
| P6  | Split-tunneling VPN puede asignar      | P6 verifica el       |
|     | servidores DNS distintos por interfaz. | resolver activo      |
|     |                                        | contra el perfil MDM |
|     |                                        | vigente. El          |
|     |                                        | split-DNS autorizado |
|     |                                        | se configura como    |
|     |                                        | excepción explícita. |
+-----+----------------------------------------+----------------------+
| P37 | En redes domésticas o de visita el     | P37 verifica la      |
|     | proxy corporativo no es alcanzable.    | presencia de la CA   |
|     |                                        | del proxy en el      |
|     |                                        | truststore. Si el    |
|     |                                        | MDM autoriza         |
|     |                                        | temporalmente la     |
|     |                                        | excepción            |
|     |                                        | (itinerancia), el    |
|     |                                        | parámetro se         |
|     |                                        | registra U, no 1.    |
+-----+----------------------------------------+----------------------+

------------------------------------------------------------------------

### 7.2. Capa 2 --- Conectividad (P7--P12, P38)

Interfaces inalámbricas, redes celulares y radio de corto alcance,
ampliada con supervisión de conectividad MDM.

*Marco normativo: ENS MP.COM.2; RGPD Art. 32; HIPAA §164.312(e).*

+----+-----------+-------+--------------------+-------+----------------------+
| Có | Nombre    | Peso  | Herramienta        | Valor | Criterio de          |
| d. |           |       | (Android)          | 1     | ternarización        |
+====+===========+=======+====================+=======+======================+
| P7 | BSSID no  | ★     | `WifiManager.ge    | 1     | Dispositivo asociado |
|    | a         | ALTO  | tConnectionInfo()` |       | a SSID/BSSID no      |
|    | utorizado |       |                    |       | incluido en lista    |
|    |           |       |                    |       | blanca MDM           |
+----+-----------+-------+--------------------+-------+----------------------+
| P8 | Bluetooth | bajo  | `BluetoothAdapter` | 1     | Emparejamiento       |
|    | no        |       |                    |       | activo con           |
|    | a         |       |                    |       | dispositivo BT no en |
|    | utorizado |       |                    |       | lista blanca         |
+----+-----------+-------+--------------------+-------+----------------------+
| P9 | GPS no    | ★     | `LocationManager`  | 1     | Posición GPS activa  |
|    | a         | ALTO  |                    |       | fuera del geofence   |
|    | utorizado |       |                    |       | corporativo          |
|    |           |       |                    |       | autorizado           |
+----+-----------+-------+--------------------+-------+----------------------+
| P  | NFC no    | ★     | `NfcAdapter`       | 1     | Sesión NFC activa    |
| 10 | a         | ALTO  |                    |       | con tag o lector no  |
|    | utorizado |       |                    |       | autorizado por MDM   |
+----+-----------+-------+--------------------+-------+----------------------+
| P  | Modo      | ★     | `Settings.Global   | 1     | Modo avión activado  |
| 11 | avión     | ALTO  | .AIRPLANE_MODE_ON` |       | sin correspondencia  |
|    | irregular |       |                    |       | con evento MDM       |
|    |           |       |                    |       | programado           |
+----+-----------+-------+--------------------+-------+----------------------+
| P  | Sincr     | ★     | `SyncManager`      | 1     | Adaptador de         |
| 12 | onización | ALTO  |                    |       | sincronización       |
|    | externa   |       |                    |       | activo hacia cuenta  |
|    | no        |       |                    |       | externa no aprobada  |
|    | a         |       |                    |       | por MDM              |
|    | utorizada |       |                    |       |                      |
+----+-----------+-------+--------------------+-------+----------------------+
| *  | **Canal   | ★     | `De                | 1     | El dispositivo no ha |
| *P | MDM no    | ALTO  | vicePolicyManager` |       | respondido al        |
| 38 | a         |       |                    |       | heartbeat MDM en \>  |
| ** | lcanzable |       |                    |       | umbral configurado   |
|    | (silencio |       |                    |       | (ej. 24 h)           |
|    | MDM)**    |       |                    |       |                      |
+----+-----------+-------+--------------------+-------+----------------------+

**Análisis adversarial --- Capa 2 (Conectividad)**

+--------+--------------------------+---------------------------------+
| Cód.   | Argumento contra         | Réplica del sistema             |
+========+==========================+=================================+
| P7     | Redes hospitalarias con  | MDM puede propagar lista de     |
|        | múltiples APs no todas   | BSSID de infraestructura        |
|        | preinscritas en MDM.     | propia. P7 solo activa 1 en     |
|        |                          | redes completamente             |
|        |                          | desconocidas.                   |
+--------+--------------------------+---------------------------------+
| P8     | Periféricos médicos BT   | En entorno médico el MDM        |
|        | (glucómetros, oxímetros) | mantiene lista de dispositivos  |
|        | pueden no estar          | BT autorizados por clase de     |
|        | preaprobados.            | dispositivo (CoD).              |
+--------+--------------------------+---------------------------------+
| P9     | Teletrabajo y trabajo de | P9 opera junto a P40            |
|        | campo legítimos sacan el | (geofencing MDM). Si MDM ha     |
|        | dispositivo del          | registrado itinerancia          |
|        | perímetro.               | autorizada, P9 pasa a U.        |
+--------+--------------------------+---------------------------------+
| P10    | Pagos con tarjetas NFC   | MDM puede filtrar por AID       |
|        | hospitalarias y lectores | (Application Identifier). Las   |
|        | de tarjetas de acceso.   | tarjetas de acceso del centro   |
|        |                          | se incluyen en la lista blanca. |
+--------+--------------------------+---------------------------------+
| P11    | El personal puede        | MDM puede restringir el acceso  |
|        | activar modo avión       | a la palanca de modo avión. Si  |
|        | voluntariamente en zonas | el usuario lo activa sin        |
|        | de silencio (UCI).       | restricción MDM: INDETERMINADO, |
|        |                          | no 1.                           |
+--------+--------------------------+---------------------------------+
| P12    | Sincronización de        | MDM autoriza solo               |
|        | contactos y calendario   | Exchange/Microsoft 365          |
|        | con cuentas              | corporativo. P12 detecta        |
|        | profesionales mixtas.    | cuentas fuera de ese whitelist. |
+--------+--------------------------+---------------------------------+
| P38    | Dispositivos apagados,   | P38 combina tiempo de           |
|        | sin cobertura o en modo  | silencio + último estado        |
|        | ahorro generan silencio  | conocido. Solo activa 1 si el   |
|        | sin causa maliciosa.     | dispositivo tuvo conexión       |
|        |                          | reciente y súbitamente deja de  |
|        |                          | responder en condiciones        |
|        |                          | normales.                       |
+--------+--------------------------+---------------------------------+

------------------------------------------------------------------------

### 7.3. Capa 3 --- Almacenamiento (P13--P18, P39)

Almacenamiento local, integridad de sistema de archivos y cifrado en
reposo, ampliada con cifrado obligatorio HIPAA.

*Marco normativo: HIPAA §164.312(a)(2)(iv); ENS MP.SI.3; RGPD Art.
32(1)(a).*

+----+---------+----+--------------------+-------+----------------------+
| Có | Nombre  | Pe | Herramienta        | Valor | Criterio de          |
| d. |         | so | (Android)          | 1     | ternarización        |
+====+=========+====+====================+=======+======================+
| P  | Almacen | ★  | `En                | 1     | Almacenamiento       |
| 13 | amiento | AL | vironment.getExter |       | externo montado y    |
|    | externo | TO | nalStorageState()` |       | accesible sin        |
|    | no      |    |                    |       | cifrado activo       |
|    | cifrado |    |                    |       |                      |
+----+---------+----+--------------------+-------+----------------------+
| P  | Modif   | ★  | `In                | 1     | Modificación de      |
| 14 | icación | AL | otifyFileObserver` |       | fichero en /system,  |
|    | de      | TO |                    |       | /vendor o /odm sin   |
|    | f       |    |                    |       | OTA autorizada       |
|    | icheros |    |                    |       |                      |
|    | de      |    |                    |       |                      |
|    | sistema |    |                    |       |                      |
+----+---------+----+--------------------+-------+----------------------+
| P  | IPC     | ★  | `Binder`           | 1     | Comunicación IPC     |
| 15 | anómala | AL |                    |       | entre proceso de     |
|    | entre   | TO |                    |       | usuario y sistema    |
|    | p       |    |                    |       | sin canal declarado  |
|    | rocesos |    |                    |       | en manifiesto        |
+----+---------+----+--------------------+-------+----------------------+
| P  | Acceso  | ★  | `ClipboardManager` | 1     | Aplicación en        |
| 16 | al      | AL |                    |       | background leyendo   |
|    | porta   | TO |                    |       | portapapeles sin     |
|    | papeles |    |                    |       | intervención del     |
|    | no      |    |                    |       | usuario              |
|    | aut     |    |                    |       |                      |
|    | orizado |    |                    |       |                      |
+----+---------+----+--------------------+-------+----------------------+
| P  | Almacen | ★  | `StorageManager.ge | 1     | Volumen externo      |
| 17 | amiento | AL | tStorageVolumes()` |       | montado (SD card,    |
|    | externo | TO |                    |       | USB OTG) sin         |
|    | montado |    |                    |       | autorización MDM     |
|    |         |    |                    |       | explícita            |
+----+---------+----+--------------------+-------+----------------------+
| P  | Modif   | ★  | `Settings.System`  | 1     | Cambio en parámetro  |
| 18 | icación | AL |                    |       | de configuración del |
|    | de      | TO |                    |       | sistema fuera del    |
|    | par     |    |                    |       | proceso MDM          |
|    | ámetros |    |                    |       |                      |
|    | de      |    |                    |       |                      |
|    | sistema |    |                    |       |                      |
+----+---------+----+--------------------+-------+----------------------+
| *  | **      | ★  | `DevicePolicyM     | 1     | Retorna              |
| *P | Cifrado | AL | anager.getStorageE |       | `ENCRYPT             |
| 39 | en      | TO | ncryptionStatus()` |       | ION_STATUS_INACTIVE` |
| ** | reposo  |    |                    |       | o                    |
|    | desa    |    |                    |       | `ENCRYPTIO           |
|    | ctivado |    |                    |       | N_STATUS_ACTIVATING` |
|    | o       |    |                    |       |                      |
|    | degr    |    |                    |       |                      |
|    | adado** |    |                    |       |                      |
+----+---------+----+--------------------+-------+----------------------+

**Análisis adversarial --- Capa 3 (Almacenamiento)**

+--------+--------------------------+---------------------------------+
| Cód.   | Argumento contra         | Réplica del sistema             |
+========+==========================+=================================+
| P13    | Tarjetas SD médicas con  | En entorno gestionado el MDM    |
|        | imágenes DICOM pueden    | exige cifrado de almacenamiento |
|        | necesitar acceso sin     | externo. Las SD se formatean    |
|        | cifrado para             | con cifrado antes de uso        |
|        | compatibilidad.          | clínico.                        |
+--------+--------------------------+---------------------------------+
| P14    | Las actualizaciones OTA  | P14 verifica si la modificación |
|        | del fabricante modifican | fue precedida por una sesión    |
|        | ficheros de sistema      | OTA MDM autorizada. Sin sesión  |
|        | legítimamente.           | registrada: 1.                  |
+--------+--------------------------+---------------------------------+
| P15    | Frameworks de            | En entorno gestionado las       |
|        | accesibilidad y sistemas | aplicaciones están firmadas y   |
|        | de input method acceden  | auditadas. P15 opera con lista  |
|        | a APIs de sistema        | de pares IPC autorizados        |
|        | mediante IPC.            | definida en el perfil MDM.      |
+--------+--------------------------+---------------------------------+
| P16    | Gestores de contraseñas  | Android 10+ notifica lecturas   |
|        | y aplicaciones de HCE    | de portapapeles en background.  |
|        | médico acceden al        | MDM puede restringir el acceso  |
|        | portapapeles para        | a apps en lista blanca.         |
|        | autorrelleno.            |                                 |
+--------+--------------------------+---------------------------------+
| P17    | Dispositivos médicos     | MDM puede deshabilitar montaje  |
|        | portátiles con lectores  | automático de almacenamiento    |
|        | de tarjeta integrados    | externo. P17 activa 1 si el     |
|        | para imágenes.           | volumen se monta fuera del      |
|        |                          | proceso autorizado por EMM.     |
+--------+--------------------------+---------------------------------+
| P18    | El usuario puede ajustar | MDM distingue parámetros de     |
|        | brillo, tamaño de fuente | usuario (brillo, volumen) de    |
|        | y volumen sin intención  | parámetros de seguridad         |
|        | maliciosa.               | (UNKNOWN_SOURCES,               |
|        |                          | DEVELOPER_OPTIONS). P18 solo    |
|        |                          | activa 1 para el segundo grupo. |
+--------+--------------------------+---------------------------------+
| *      | Dispositivos muy         | HIPAA §164.312(a)(2)(iv) y ENS  |
| *P39** | antiguos (API \< 23) o   | MP.SI.3 exigen cifrado en       |
|        | fabricantes clínicos con | reposo. P39 es un parámetro de  |
|        | configuraciones          | conformidad normativa: su       |
|        | específicas de cifrado   | activación implica              |
|        | para su software         | incumplimiento regulatorio. La  |
|        | propietario.             | lista blanca debe incluir los   |
|        |                          | modelos cuyo baseline           |
|        |                          | documentado incluye             |
|        |                          | configuración de cifrado        |
|        |                          | específica del fabricante; los  |
|        |                          | demás casos son incumplimiento. |
|        |                          | En entornos médicos este        |
|        |                          | parámetro tiene peso            |
|        |                          | bloqueante: un dispositivo sin  |
|        |                          | cifrado activo no debe acceder  |
|        |                          | a HCE.                          |
+--------+--------------------------+---------------------------------+

------------------------------------------------------------------------

### 7.4. Capa 4 --- Contexto (P19--P24, P40)

Sensores de contexto físico, geolocalización y comportamiento ambiental,
ampliada con geofencing corporativo MDM.

*Marco normativo: RGPD Art. 9 (datos de localización especiales); ENS
MP.COM.4.*

+----+----------+--------+-----------------+------+----------------------+
| Có | Nombre   | Peso   | Herramienta     | V    | Criterio de          |
| d. |          |        | (Android)       | alor | ternarización        |
|    |          |        |                 | 1    |                      |
+====+==========+========+=================+======+======================+
| P  | IPC      | ★ ALTO | `ActivityMan    | 1    | Proceso de sistema   |
| 19 | anómala  |        | ager.getRunning |      | realizando IPC hacia |
|    | de       |        | AppProcesses()` |      | aplicación de        |
|    | sistema  |        |                 |      | usuario sin canal    |
|    |          |        |                 |      | declarado            |
+----+----------+--------+-----------------+------+----------------------+
| P  | Port     | ★ ALTO | `Cl             | 1    | Lectura del          |
| 20 | apapeles |        | ipboardManager` |      | portapapeles desde   |
|    | accedido |        |                 |      | proceso sin foco de  |
|    | en       |        |                 |      | usuario              |
|    | ba       |        |                 |      |                      |
|    | ckground |        |                 |      |                      |
+----+----------+--------+-----------------+------+----------------------+
| P  | Confi    | ★ ALTO | `Connectivi     | 1    | Cambio de proxy, APN |
| 21 | guración |        | tyManager.getLi |      | o ruta de red fuera  |
|    | de red   |        | nkProperties()` |      | de proceso MDM       |
|    | mo       |        |                 |      |                      |
|    | dificada |        |                 |      |                      |
+----+----------+--------+-----------------+------+----------------------+
| P  | Sensores | bajo   | `SensorManager` | 1    | Patrón de movimiento |
| 22 | de       |        |                 |      | estadísticamente     |
|    | mo       |        |                 |      | incompatible con uso |
|    | vimiento |        |                 |      | del dispositivo      |
|    | irr      |        |                 |      |                      |
|    | egulares |        |                 |      |                      |
+----+----------+--------+-----------------+------+----------------------+
| P  | Modo     | ★ ALTO | `Set            | 1    | Activación sin       |
| 23 | avión    |        | tings.Global.AI |      | correspondencia con  |
|    | activado |        | RPLANE_MODE_ON` |      | política MDM o       |
|    | irregu   |        |                 |      | evento programado    |
|    | larmente |        |                 |      |                      |
+----+----------+--------+-----------------+------+----------------------+
| P  | Sincro   | ★ ALTO | `SyncManager.   | 1    | Sincronización       |
| 24 | nización |        | isSyncActive()` |      | iniciada fuera de la |
|    | externa  |        |                 |      | ventana programada   |
|    | no       |        |                 |      | por MDM              |
|    | pr       |        |                 |      |                      |
|    | ogramada |        |                 |      |                      |
+----+----------+--------+-----------------+------+----------------------+
| *  | **Ge     | ★ ALTO | `Ge             | 1    | Dispositivo          |
| *P | ofencing |        | ofencingClient` |      | detectado fuera del  |
| 40 | cor      |        | (Google Play    |      | perímetro geográfico |
| ** | porativo |        | Services)       |      | MDM sin itinerancia  |
|    | v        |        |                 |      | autorizada           |
|    | iolado** |        |                 |      |                      |
+----+----------+--------+-----------------+------+----------------------+

**Análisis adversarial --- Capa 4 (Contexto)**

+--------+--------------------------+---------------------------------+
| Cód.   | Argumento contra         | Réplica del sistema             |
+========+==========================+=================================+
| P19    | Los servicios de         | P19 analiza origen (proceso de  |
|        | actualización del        | sistema) vs destino (app        |
|        | sistema contactan        | usuario). Solo activa 1 si el   |
|        | aplicaciones del         | canal IPC no está declarado en  |
|        | launcher para            | el manifiesto del sistema.      |
|        | notificaciones.          |                                 |
+--------+--------------------------+---------------------------------+
| P20    | Gestores de contraseñas  | P20 complementa a P16. Las apps |
|        | y RPA médica necesitan   | con permiso READ_CLIPBOARD      |
|        | acceso al portapapeles   | aprobado en MDM quedan exentas. |
|        | en segundo plano.        |                                 |
+--------+--------------------------+---------------------------------+
| P21    | Cambios de red al        | MDM registra las redes          |
|        | itinerar entre edificios | corporativas autorizadas. Los   |
|        | hospitalarios con        | cambios debidos a handover      |
|        | distintas subredes.      | entre APs del mismo dominio son |
|        |                          | NORMAL.                         |
+--------+--------------------------+---------------------------------+
| P22    | Dispositivos médicos     | P22 tiene peso bajo. En entorno |
|        | montados en soporte      | médico puede refinarse con      |
|        | estático pueden generar  | contexto de turno de trabajo    |
|        | lecturas de vibración    | (dispositivo en soporte =       |
|        | del entorno.             | normal).                        |
+--------+--------------------------+---------------------------------+
| P23    | En UCI o quirófano el    | MDM puede registrar zonas de    |
|        | personal activa modo     | modo avión obligatorio          |
|        | avión voluntariamente.   | (quirófano, resonancia          |
|        |                          | magnética). P23 valida si la    |
|        |                          | activación coincide con la zona |
|        |                          | georreferenciada.               |
+--------+--------------------------+---------------------------------+
| P24    | Fallos de red pueden     | MDM registra ventanas de        |
|        | desplazar la             | sincronización con tolerancia   |
|        | sincronización           | (±30 min). P24 activa 1 solo    |
|        | programada a un horario  | fuera de esa ventana.           |
|        | distinto.                |                                 |
+--------+--------------------------+---------------------------------+
| P40    | Desplazamientos entre    | El MDM registra itinerancias    |
|        | sedes, trabajo de campo  | autorizadas. P40 activa 1 solo  |
|        | autorizado y visitas     | si el dispositivo sale del      |
|        | inter-hospitalarias.     | perímetro sin itinerancia       |
|        |                          | registrada activa.              |
+--------+--------------------------+---------------------------------+

------------------------------------------------------------------------

### 7.5. Capa 5 --- Autenticación (P25--P30, P41)

Mecanismos de autenticación, gestión de credenciales y política de
acceso, ampliada con cumplimiento de política de contraseñas MDM.

*Marco normativo: HIPAA §164.312(d); ENS IA.5; RGPD Art. 32(1)(b).*

+----+-------------+-----+-----------------+-------+----------------------+
| Có | Nombre      | P   | Herramienta     | Valor | Criterio de          |
| d. |             | eso | (Android)       | 1     | ternarización        |
+====+=============+=====+=================+=======+======================+
| P  | C           | ★   | `Cre            | 1     | Credencial           |
| 25 | redenciales | A   | dentialManager` |       | almacenada detectada |
|    | co          | LTO |                 |       | en base de datos     |
|    | mprometidas |     |                 |       | pública de           |
|    | detectadas  |     |                 |       | filtraciones         |
+----+-------------+-----+-----------------+-------+----------------------+
| P  | Biometría   | ★   | `Bi             | 1     | N intentos           |
| 26 | fallida     | A   | ometricManager` |       | biométricos fallidos |
|    | re          | LTO |                 |       | consecutivos \>      |
|    | petidamente |     |                 |       | umbral configurado   |
|    |             |     |                 |       | (ej. 5)              |
+----+-------------+-----+-----------------+-------+----------------------+
| P  | Keystore    | ★   | `KeyStore`      | 1     | Clave en keystore de |
| 27 | c           | A   |                 |       | software (no         |
|    | omprometido | LTO |                 |       | StrongBox/TEE) en    |
|    |             |     |                 |       | dispositivo que      |
|    |             |     |                 |       | debería usar         |
|    |             |     |                 |       | hardware             |
+----+-------------+-----+-----------------+-------+----------------------+
| P  | Token de    | ★   | `               | 1     | Token con campo      |
| 28 | sesión      | A   | AccountManager. |       | `exp` pasado sigue   |
|    | expirado en | LTO | getAuthToken()` |       | siendo aceptado por  |
|    | uso         |     |                 |       | la aplicación        |
+----+-------------+-----+-----------------+-------+----------------------+
| P  | SMS sin     | ★   | `SmsManager`    | 1     | SMS recibido y       |
| 29 | interacción | A   |                 |       | procesado por app en |
|    | del usuario | LTO |                 |       | background sin       |
|    | activo      |     |                 |       | notificación al      |
|    |             |     |                 |       | usuario              |
+----+-------------+-----+-----------------+-------+----------------------+
| P  | Llamada sin | ★   | `               | 1     | Llamada iniciada o   |
| 30 | interacción | A   | TelecomManager` |       | respondida           |
|    | del usuario | LTO |                 |       | programáticamente    |
|    |             |     |                 |       | sin acción del       |
|    |             |     |                 |       | usuario              |
+----+-------------+-----+-----------------+-------+----------------------+
| *  | **Política  | ★   | `Devi           | 1     | `isActiveP           |
| *P | de          | A   | cePolicyManager |       | asswordSufficient()` |
| 41 | contraseñas | LTO | .isActivePasswo |       | retorna false        |
| ** | MDM         |     | rdSufficient()` |       |                      |
|    | i           |     |                 |       |                      |
|    | ncumplida** |     |                 |       |                      |
+----+-------------+-----+-----------------+-------+----------------------+

**Análisis adversarial --- Capa 5 (Autenticación)**

+--------+--------------------------+---------------------------------+
| Cód.   | Argumento contra         | Réplica del sistema             |
+========+==========================+=================================+
| P25    | El servicio de           | MDM puede provisionar el        |
|        | verificación requiere    | servicio de verificación de     |
|        | conectividad; en red sin | credenciales de forma interna.  |
|        | acceso externo no puede  | P25 usa el endpoint MDM, no el  |
|        | comprobarse.             | público.                        |
+--------+--------------------------+---------------------------------+
| P26    | En entornos clínicos con | MDM puede configurar el umbral  |
|        | guantes, la biometría de | y el método de autenticación    |
|        | huella falla             | por contexto (guantes → PIN).   |
|        | sistemáticamente.        | P26 se adapta al método activo. |
+--------+--------------------------+---------------------------------+
| P27    | Dispositivos antiguos    | MDM exige StrongBox en el       |
|        | sin chip de seguridad    | perfil de dispositivo aprobado. |
|        | dedicado (StrongBox) no  | P27 activa 1 si el dispositivo  |
|        | tienen otra opción.      | sin StrongBox pasa a producción |
|        |                          | clínica: es un parámetro de     |
|        |                          | conformidad de hardware.        |
+--------+--------------------------+---------------------------------+
| P28    | Redes con reloj sesgado  | P28 compara el `exp` del token  |
|        | pueden hacer que tokens  | con la hora sincronizada MDM    |
|        | válidos aparezcan        | (no el reloj local). Si el      |
|        | expirados.               | reloj local está manipulado,    |
|        |                          | P48 ya habrá activado 1.        |
+--------+--------------------------+---------------------------------+
| P29    | Apps de autenticación de | MDM mantiene lista de apps      |
|        | segundo factor (OTP)     | autorizadas para procesar SMS   |
|        | procesan SMS en          | en background. P29 solo activa  |
|        | background               | 1 si la app procesadora no está |
|        | legítimamente.           | en la lista.                    |
+--------+--------------------------+---------------------------------+
| P30    | Sistemas de              | MDM autoriza por paquete las    |
|        | intercomunicación        | aplicaciones que pueden iniciar |
|        | hospitalaria pueden      | llamadas programáticas. P30     |
|        | automatizar llamadas.    | activa 1 solo para paquetes no  |
|        |                          | autorizados.                    |
+--------+--------------------------+---------------------------------+
| P41    | Tras un cambio de        | P41 tiene período de gracia     |
|        | política MDM los         | configurable. Si el dispositivo |
|        | dispositivos tienen un   | está dentro del período         |
|        | período de gracia para   | post-cambio: U. Pasado el       |
|        | actualizar la            | período sin actualizar: 1.      |
|        | contraseña.              | Clave para HIPAA §164.312(d).   |
+--------+--------------------------+---------------------------------+

------------------------------------------------------------------------

### 7.6. Capa 6 --- Comunicaciones y Evasión (P31--P36, P42)

Canales de comunicación, técnicas de evasión y exfiltración, ampliada
con supervisión de conectividad MDM.

*Marco normativo: HIPAA §164.312(e)(2)(ii); ENS MP.COM.3; RGPD Art. 32.*

+----+---------+---------+----------------+-------+----------------------+
| Có | Nombre  | Peso    | Herramienta    | Valor | Criterio de          |
| d. |         |         | (Android)      | 1     | ternarización        |
+====+=========+=========+================+=======+======================+
| P  | API de  | ★ ALTO  | `Payme         | 1     | API de pago invocada |
| 31 | pago    |         | ntRequest API` |       | sin contexto de      |
|    | i       |         |                |       | aplicación de pago   |
|    | nvocada |         |                |       | autorizada           |
+----+---------+---------+----------------+-------+----------------------+
| P  | DNS     | ★ ALTO  | `Lin           | 1     | Servidor DNS activo  |
| 32 | mod     |         | kProperties.ge |       | distinto al asignado |
|    | ificado |         | tDnsServers()` |       | por MDM              |
+----+---------+---------+----------------+-------+----------------------+
| P  | Borrado | ★ ALTO  | `logcat`       | 1     | Logs de sistema      |
| 33 | de logs |         |                |       | eliminados o         |
|    | del     |         |                |       | truncados fuera de   |
|    | sistema |         |                |       | ciclo MDM programado |
+----+---------+---------+----------------+-------+----------------------+
| P  | Horario | ★ ALTO  | `Usag          | 1     | Actividad de         |
| 34 | de      |         | eStatsManager` |       | aplicación fuera del |
|    | ac      |         |                |       | horario laboral      |
|    | tividad |         |                |       | configurado en MDM   |
|    | anómalo |         |                |       |                      |
+----+---------+---------+----------------+-------+----------------------+
| P  | Tráfico | ★ ALTO  | `Networ        | 1     | Tráfico TLS/SSL      |
| 35 | cifrado |         | kStatsManager` |       | hacia IPs no en      |
|    | a IPs   |         |                |       | lista blanca y no    |
|    | a       |         |                |       | resolubles en DNS    |
|    | nónimas |         |                |       | corporativo          |
+----+---------+---------+----------------+-------+----------------------+
| P  | Proceso | ★ ALTO  | `ActivityManag | 1     | Proceso en ejecución |
| 36 | desc    |         | er.getRunningA |       | no incluido en el    |
|    | onocido |         | ppProcesses()` |       | inventario de        |
|    | activo  |         |                |       | aplicaciones         |
|    |         |         |                |       | aprobadas MDM        |
+----+---------+---------+----------------+-------+----------------------+
| *  | **Comun | ★ ALTO  | `Device        | 1     | Tráfico MDM          |
| *P | icación |         | PolicyManager` |       | bloqueado de forma   |
| 42 | con     |         |                |       | activa (firewall     |
| ** | s       |         |                |       | local, hosts file    |
|    | ervidor |         |                |       | modificado, proceso  |
|    | MDM     |         |                |       | MDM terminado)       |
|    | inter   |         |                |       |                      |
|    | rumpida |         |                |       |                      |
|    | (       |         |                |       |                      |
|    | evasión |         |                |       |                      |
|    | ac      |         |                |       |                      |
|    | tiva)** |         |                |       |                      |
+----+---------+---------+----------------+-------+----------------------+

**Análisis adversarial --- Capa 6 (Comunicaciones y Evasión)**

+--------+--------------------------+---------------------------------+
| Cód.   | Argumento contra         | Réplica del sistema             |
+========+==========================+=================================+
| P31    | Apps hospitalarias con   | MDM autoriza por paquete las    |
|        | módulos de copago pueden | apps que pueden invocar APIs de |
|        | invocar APIs de pago     | pago. P31 activa 1 fuera de ese |
|        | legítimamente.           | whitelist.                      |
+--------+--------------------------+---------------------------------+
| P32    | VPN split-tunnel puede   | P32 detecta cambios de DNS      |
|        | asignar DNS distintos    | iniciados desde la capa de      |
|        | por interfaz.            | aplicación (no de red), como    |
|        |                          | malware que modifica el DNS vía |
|        |                          | API.                            |
+--------+--------------------------+---------------------------------+
| P33    | Rotación de logs         | MDM programa la rotación de     |
|        | automática del sistema   | logs. P33 activa 1 si el        |
|        | puede generar falsos     | borrado ocurre fuera del ciclo  |
|        | positivos.               | MDM. Integridad de logs es      |
|        |                          | requisito normativo (ENS, HIPAA |
|        |                          | audit controls).                |
+--------+--------------------------+---------------------------------+
| P34    | Guardias nocturnas y     | MDM configura el horario        |
|        | turnos 24 h son comunes  | laboral por rol y turno. P34    |
|        | en entornos médicos.     | activa 1 solo para actividad    |
|        |                          | fuera del turno asignado al     |
|        |                          | usuario.                        |
+--------+--------------------------+---------------------------------+
| P35    | Servicios cloud con IPs  | MDM mantiene una lista de       |
|        | dinámicas y CDNs pueden  | rangos de IP autorizados (ej.   |
|        | no estar registradas en  | rangos Microsoft Azure para     |
|        | DNS corporativo.         | Microsoft 365). P35 activa 1    |
|        |                          | para tráfico hacia IPs fuera de |
|        |                          | todos los rangos autorizados.   |
+--------+--------------------------+---------------------------------+
| P36    | Actualizaciones          | MDM mantiene inventario de      |
|        | automáticas pueden       | hashes de aplicaciones          |
|        | ejecutar procesos        | aprobadas. P36 activa 1 si el   |
|        | temporales no            | proceso activo no tiene hash    |
|        | inventariados.           | registrado.                     |
+--------+--------------------------+---------------------------------+
| P42    | Sin conectividad de red  | P42 se diferencia de P38        |
|        | el canal MDM no es       | (silencio pasivo) en que        |
|        | alcanzable sin que       | detecta **bloqueo activo**:     |
|        | implique bloqueo activo. | proceso MDM terminado, endpoint |
|        |                          | MDM en lista de denegación      |
|        |                          | local, o tráfico MDM            |
|        |                          | interceptado y descartado. Es   |
|        |                          | el parámetro de evasión         |
|        |                          | específico de entorno           |
|        |                          | gestionado.                     |
+--------+--------------------------+---------------------------------+

------------------------------------------------------------------------

### 7.7. Capa 7 --- Gestión MDM (P43--P49)

> **⚑ Capa exclusiva de SV(49,7). Sin equivalente en SV(36,6) ni en
> SV(9,3).**

Parámetros de cumplimiento de políticas MDM, conformidad normativa y
estado de gestión del dispositivo.

*Marco normativo: HIPAA §164.312; ENS OP.EXP.6 y MP.EQ.3; RGPD Art. 32;
EU 2017/745 Anexo I §17.*

+----+--------------+----+--------------------+-------+----------------------+
| Có | Nombre       | Pe | Herramienta        | Valor | Criterio de          |
| d. |              | so | (Android)          | 1     | ternarización        |
+====+==============+====+====================+=======+======================+
| *  | Perfil MDM   | ★  | `                  | 1     | Perfil MDM           |
| *P | eliminado o  | AL | DevicePolicyManage |       | corporativo          |
| 43 | modificado   | TO | r.isAdminActive()` |       | eliminado o sus      |
| ** |              |    |                    |       | restricciones        |
|    |              |    |                    |       | modificadas fuera    |
|    |              |    |                    |       | del proceso EMM      |
+----+--------------+----+--------------------+-------+----------------------+
| *  | VPN          | ★  | `VpnSer            | 1     | VPN always-on        |
| *P | corporativa  | AL | vice.isAlwaysOnVpn |       | configurada por MDM  |
| 44 | desactivada  | TO | LockdownEnabled()` |       | no está activa;      |
| ** |              |    |                    |       | tráfico sale sin     |
|    |              |    |                    |       | túnel                |
+----+--------------+----+--------------------+-------+----------------------+
| *  | Pantalla de  | ★  | `Devic             | 1     | Bloqueo eliminado, o |
| *P | bloqueo      | AL | ePolicyManager.get |       | calidad reducida a   |
| 45 | eliminada o  | TO | PasswordQuality()` |       | NONE/UNSPECIFIED en  |
| ** | degradada    |    |                    |       | dispositivo que      |
|    |              |    |                    |       | debería tener        |
|    |              |    |                    |       | PIN/biométrico       |
+----+--------------+----+--------------------+-------+----------------------+
| *  | Backup no    | ★  | `BackupManager.    | 1     | Backup de datos de   |
| *P | autorizado a | AL | isBackupEnabled()` |       | aplicación activo    |
| 46 | nube ajena   | TO |                    |       | hacia destino no     |
| ** |              |    |                    |       | aprobado por MDM     |
+----+--------------+----+--------------------+-------+----------------------+
| *  | ADB          | ★  | `Settings.G        | 1     | Android Debug Bridge |
| *P | habilitado   | AL | lobal.ADB_ENABLED` |       | activo sin           |
| 47 | sin sesión   | TO |                    |       | correspondencia con  |
| ** | autorizada   |    |                    |       | sesión de            |
|    |              |    |                    |       | mantenimiento        |
|    |              |    |                    |       | registrada en MDM    |
+----+--------------+----+--------------------+-------+----------------------+
| *  | Hora del     | ★  | `SystemClock` /    | 1     | Hora local difiere   |
| *P | sistema      | AL | NTP offset         |       | en \> N segundos del |
| 48 | manipulada   | TO |                    |       | servidor NTP         |
| ** |              |    |                    |       | corporativo, o       |
|    |              |    |                    |       | AUTO_TIME está       |
|    |              |    |                    |       | desactivado          |
+----+--------------+----+--------------------+-------+----------------------+
| *  | R            | ★  | `DevicePolic       | 1     | Aplicación           |
| *P | estricciones | AL | yManager.getPermit |       | restringida por MDM  |
| 49 | de           | TO | tedInputMethods()` |       | activa, o            |
| ** | aplicaciones |    |                    |       | restricción de       |
|    | MDM          |    |                    |       | funcionalidad        |
|    | incumplidas  |    |                    |       | levantada sin        |
|    |              |    |                    |       | autorización         |
+----+--------------+----+--------------------+-------+----------------------+

**Análisis adversarial --- Capa 7 (Gestión MDM)**

+--------+--------------------------+---------------------------------+
| Cód.   | Argumento contra         | Réplica del sistema             |
+========+==========================+=================================+
| P43    | Una actualización del    | El EMM registra el ciclo de     |
|        | propio EMM puede         | actualización del perfil. P43   |
|        | provocar una             | activa 1 si la ausencia no      |
|        | reinstalación temporal   | corresponde a un ciclo          |
|        | del perfil que genera    | registrado. La eliminación no   |
|        | una ventana de ausencia. | autorizada del perfil MDM es el |
|        |                          | indicador de evasión más grave  |
|        |                          | en entornos gestionados.        |
+--------+--------------------------+---------------------------------+
| P44    | Caídas del servidor VPN  | MDM configura VPN always-on con |
|        | hacen que el dispositivo | lockdown mode: si la VPN cae,   |
|        | salga sin túnel          | el tráfico se bloquea (no sale  |
|        | temporalmente. En        | sin cifrar). Ventana de         |
|        | cobertura intermitente   | tolerancia: 30 s (el 95% de las |
|        | hay ventanas de          | reconexiones ocurren en menos). |
|        | reconexión frecuentes.   | P44 detecta la causa; P46 puede |
|        |                          | detectar la consecuencia.       |
+--------+--------------------------+---------------------------------+
| P45    | Dispositivos en modo     | MDM distingue dispositivos en   |
|        | kiosco médico pueden no  | modo kiosco de los de uso       |
|        | tener bloqueo de         | general. P45 solo aplica a los  |
|        | pantalla por diseño.     | de uso general.                 |
+--------+--------------------------+---------------------------------+
| P46    | `an                      | P46 detecta que el backup se ha |
|        | droid:allowBackup=false` | activado **a pesar** de la      |
|        | impuesto por MDM hace    | política. P43 detecta la causa  |
|        | que P46 rara vez se      | (perfil MDM modificado), P46    |
|        | active en entorno        | detecta la consecuencia (backup |
|        | correctamente            | activado). Crítico para HIPAA:  |
|        | configurado, lo que      | datos PHI no pueden hacer       |
|        | podría parecer           | backup a nube no cubierta por   |
|        | redundante.              | Business Associate Agreement.   |
+--------+--------------------------+---------------------------------+
| P47    | El departamento de TI    | MDM registra las sesiones de    |
|        | puede necesitar ADB para | mantenimiento con ADB           |
|        | mantenimiento de campo.  | autorizado (ventana temporal +  |
|        |                          | identificador del técnico). P47 |
|        |                          | activa 1 si ADB está activo     |
|        |                          | fuera de toda sesión            |
|        |                          | registrada. La combinación      |
|        |                          | P47=1 + P44=0 indica acceso no  |
|        |                          | autorizado de alta severidad.   |
+--------+--------------------------+---------------------------------+
| P48    | En ausencia de           | P48 diferencia deriva natural   |
|        | conectividad el reloj    | (INDETERMINADO) de manipulación |
|        | puede derivar sin        | activa (AUTO_TIME desactivado = |
|        | manipulación activa. La  | 1). Umbral: desviación \> 300 s |
|        | sincronización NTP puede | sostenida durante \> 60 s       |
|        | producir ajustes         | continuos respecto al servidor  |
|        | superiores a varios      | NTP corporativo. Este umbral    |
|        | segundos tras períodos   | excluye todos los ajustes       |
|        | sin conectividad.        | legítimos. La manipulación del  |
|        |                          | reloj anula logs de auditoría y |
|        |                          | tokens JWT. Timestamps tienen   |
|        |                          | valor legal y pericial (HIPAA   |
|        |                          | §164.312(b)).                   |
+--------+--------------------------+---------------------------------+
| P49    | Una actualización de la  | MDM reaplica                    |
|        | app puede resetear los   | `ApplicationRestrictions` en    |
|        | valores de               | cada heartbeat y en cada        |
|        | `                        | actualización. P49 detecta si   |
|        | ApplicationRestrictions` | la restricción fue levantada    |
|        | al instalar.             | entre dos heartbeats, indicando |
|        |                          | manipulación activa. Este       |
|        |                          | parámetro cierra el modelo      |
|        |                          | normativo EU 2017/745: los SaMD |
|        |                          | deben operar dentro de los      |
|        |                          | límites definidos por el        |
|        |                          | fabricante.                     |
+--------+--------------------------+---------------------------------+

------------------------------------------------------------------------

## 8. Regla de clasificación de SV(49, 7)

La célula SV(49, 7) aplica la misma regla de umbral simétrico que la
serie:

    T(n) = ⌊7 × 49 / 9⌋ = ⌊343 / 9⌋ = ⌊38,11⌋ = 38

+--------------------+--------------+------------------------------------+
| Clase              | Condición    | Semántica                          |
+====================+==============+====================================+
| **INTRUSIÓN**      | n₁ ≥ 38      | Al menos 38 de los 49 parámetros   |
|                    |              | activos (valor 1). Dispositivo     |
|                    |              | comprometido con alta confianza.   |
+--------------------+--------------+------------------------------------+
| **NORMAL**         | n₀ ≥ 38      | Al menos 38 de los 49 parámetros   |
|                    |              | en valor 0. Dispositivo conforme a |
|                    |              | política.                          |
+--------------------+--------------+------------------------------------+
| **INDETERMINADO**  | Resto        | Ni intrusión ni normalidad         |
|                    |              | confirmadas. Requiere revisión     |
|                    |              | humana o medición adicional.       |
+--------------------+--------------+------------------------------------+

La región INDETERMINADO ocupa el mayor volumen del espacio combinacional
(véase Sección 10). En entornos clínicos, un dispositivo que permanece
en INDETERMINADO durante más de un umbral de tiempo configurable debe
ser aislado de la red hasta resolución manual, de conformidad con ENS
OP.EXP.6 y HIPAA §164.308(a)(6)(ii).

## 9. Espacio combinacional

El espacio total de vectores ternarios de 49 componentes es 3⁴⁹. El
valor exacto y su aproximación son:

    3⁴⁹ = 239.299.329.230.617.529.590.083 ≈ 2,393 × 10²³ vectores

Para situar esta magnitud: 3⁴⁹ supera en un factor **27 (= 3³)** el
espacio de 3⁴⁶ y es exactamente **81 veces (= 3⁴)** mayor que el espacio
del par SV(45) = 3⁴⁵ ≈ 2,95 × 10²¹. El crecimiento exponencial hace que
la exploración exhaustiva sea computacionalmente inviable incluso para
adversarios con hardware de alto rendimiento: a 10¹² vectores/segundo,
recorrer el espacio completo requeriría \~7,6 × 10³ años.

La restricción algebraica n = b² se cumple estrictamente: 49 = 7².
Cualquier extensión futura mantendrá b entero (b = 8 → n = 64, b = 9 → n
= 81, etc.), preservando la arquitectura de capas cuadradas.

## 10. Distribución del espacio en las tres clases

El número de vectores de INTRUSIÓN se obtiene eligiendo k posiciones
para los valores 1 (C(49, k) combinaciones) y asignando libremente 0 o U
a las 49 − k restantes (2⁴⁹⁻ᵏ combinaciones). La fórmula exacta es:

    S(INTRUSIÓN) = Σₖ₌₃₈⁴⁹ C(49, k) · 2⁴⁹⁻ᵏ

Por la simetría del sistema ternario, S(NORMAL) se obtiene
intercambiando el papel de 0 y 1 y tiene exactamente el mismo valor.

+------------+--------------------+-----------------------------------+
| Clase      | Condición          | Vectores (exacto)                 |
+============+====================+===================================+
| **I        | n₁ ≥ 38            | Σₖ₌₃₈⁴⁹ C(49,k)·2⁴⁹⁻ᵏ ≈ **6,93 ×  |
| NTRUSIÓN** |                    | 10¹³** (≈ 2,89 × 10⁻⁸ % del       |
|            |                    | espacio)                          |
+------------+--------------------+-----------------------------------+
| **NORMAL** | n₀ ≥ 38            | Simétrico: ≈ **6,93 × 10¹³** (≈   |
|            |                    | 2,89 × 10⁻⁸ % del espacio)        |
+------------+--------------------+-----------------------------------+
| **INDET    | Resto              | 3⁴⁹ − 2·S(INTRUSIÓN) ≈ **2,39 ×   |
| ERMINADO** |                    | 10²³** (≈ **99,9999999 %** del    |
|            |                    | espacio)                          |
+------------+--------------------+-----------------------------------+

En n = 49 las clases extremas (INTRUSIÓN / NORMAL) representan una
fracción del orden de 10⁻⁸ % del espacio total: **dos órdenes de
magnitud menos** que en n = 36, donde esa fracción era del orden de 10⁻⁶
%. Prácticamente todo el espacio combinacional es INDETERMINADO. Esto no
es una debilidad del sistema: es una consecuencia directa del aumento de
dimensión y refuerza la necesidad de una política de respuesta a
INDETERMINADO robusta, especialmente en entornos clínicos donde la
ambigüedad debe tratarse con supervisión activa, no como ausencia de
señal.

## 11. Ejemplos de clasificación

+----------+------------------------------+--------------+-----------+
| E        | Parámetros activos (valor 1) | C            | F         |
| scenario |                              | lasificación | undamento |
+==========+==============================+==============+===========+
| Dis      | P1,P3,P33,P35,P              | *            | n₁=41 ≥   |
| positivo | 43,P44,P45,P46,P47,P48,P49 + | *INTRUSIÓN** | 38.       |
| médico   | otros 30 → n₁=41             |              | Perfil    |
| comp     |                              |              | MDM       |
| rometido |                              |              | el        |
|          |                              |              | iminado + |
|          |                              |              | VPN       |
|          |                              |              | desa      |
|          |                              |              | ctivada + |
|          |                              |              | exf       |
|          |                              |              | iltración |
|          |                              |              | activa.   |
+----------+------------------------------+--------------+-----------+
| Dis      | P7 (BSSID desconocido), P38  | **NORMAL**   | n₀=47 ≥   |
| positivo | (silencio MDM) → n₁=2        |              | 38. Solo  |
| en zona  |                              |              | 2         |
| de Wi-Fi |                              |              | p         |
| des      |                              |              | arámetros |
| conocida |                              |              | activos,  |
|          |                              |              | umbral no |
|          |                              |              | a         |
|          |                              |              | lcanzado. |
+----------+------------------------------+--------------+-----------+
| Dis      | P47 (ADB activo), P42 (canal | **IND        | n₁ \< 38, |
| positivo | MDM, sesión de               | ETERMINADO** | n₀ \< 38. |
| de       | mantenimiento) → n₁=2        |              | La sesión |
| mante    | contextuales                 |              | MDM       |
| nimiento |                              |              | autoriza  |
| (ADB     |                              |              | P47; P42  |
| aut      |                              |              | en modo   |
| orizado) |                              |              | mante     |
|          |                              |              | nimiento. |
|          |                              |              | Requiere  |
|          |                              |              | revisión. |
+----------+------------------------------+--------------+-----------+
| Dis      | P39 (cifrado desactivado),   | **IND        | n₁ \< 38  |
| positivo | P27 (keystore software), P41 | ETERMINADO** | pero 3    |
| sin      | (política contraseñas) →     |              | p         |
| cifrado  | n₁=3                         |              | arámetros |
| (        |                              |              | de        |
| hardware |                              |              | co        |
| antiguo, |                              |              | nformidad |
| no       |                              |              | normativa |
| com      |                              |              | activos.  |
| patible) |                              |              | Alerta de |
|          |                              |              | auditoría |
|          |                              |              | HIPAA     |
|          |                              |              | §164.312. |
+----------+------------------------------+--------------+-----------+
| Dis      | Todos los parámetros en 0 →  | **NORMAL**   | n₀=49 ≥   |
| positivo | n₀=49                        |              | 38.       |
| to       |                              |              | Di        |
| talmente |                              |              | spositivo |
| conforme |                              |              | p         |
|          |                              |              | lenamente |
|          |                              |              | g         |
|          |                              |              | estionado |
|          |                              |              | y         |
|          |                              |              | conforme  |
|          |                              |              | a         |
|          |                              |              | política. |
+----------+------------------------------+--------------+-----------+

## 12. Frontera de despliegue: SV(49, 7) frente al par SV(36, 6) + SV(9, 3)

El Documento 5 estableció la arquitectura de célula en par como la
solución canónica para entornos de consumo. La célula SV(49, 7) no
sustituye ese diseño; define una segunda frontera de despliegue con
características distintas.

+-----------+-----------------------------+---------------------------+
| Criterio  | Par SV(36,6) + SV(9,3)      | Célula única SV(49,7)     |
+===========+=============================+===========================+
| Tipo de   | Consumo, BYOD, sin MDM      | Gestionado, corporativo,  |
| entorno   |                             | clínico                   |
+-----------+-----------------------------+---------------------------+
| Servidor  | No necesario                | **Obligatorio**           |
| MDM       |                             |                           |
| d         |                             |                           |
| isponible |                             |                           |
+-----------+-----------------------------+---------------------------+
| Marco     | RGPD básico, opcional       | HIPAA, ENS, RGPD, EU      |
| normativo |                             | 2017/745                  |
+-----------+-----------------------------+---------------------------+
| P         | Célula SV(9,3)              | Integrados en capas       |
| arámetros | independiente               | C1--C6 (P37--P42)         |
| de        |                             |                           |
| i         |                             |                           |
| ntegridad |                             |                           |
| SO        |                             |                           |
+-----------+-----------------------------+---------------------------+
| P         | No contemplados             | **Capa C7 exclusiva       |
| arámetros |                             | (P43--P49)**              |
| de        |                             |                           |
| gestión   |                             |                           |
| MDM       |                             |                           |
+-----------+-----------------------------+---------------------------+
| Co        | Baja (dos células           | Media-alta (requiere MDM  |
| mplejidad | independientes)             | activo)                   |
| de        |                             |                           |
| d         |                             |                           |
| espliegue |                             |                           |
+-----------+-----------------------------+---------------------------+
| Espacio   | 3⁴⁵ ≈ 2,95 × 10²¹           | 3⁴⁹ ≈ 2,39 × 10²³         |
| comb      |                             |                           |
| inacional |                             |                           |
+-----------+-----------------------------+---------------------------+
| Umbral    | max(cls₃₆, cls₉)            | n₁ ≥ 38 unificado         |
| INTRUSIÓN |                             |                           |
+-----------+-----------------------------+---------------------------+
| Detección | ❌ No (punto ciego          | ✓ P42, P43, P47           |
| evasión   | estructural)                |                           |
| MDM       |                             |                           |
+-----------+-----------------------------+---------------------------+
| Co        | No representada             | ✓ P39 (cifrado en reposo) |
| nformidad |                             |                           |
| cifrado   |                             |                           |
| HIPAA     |                             |                           |
+-----------+-----------------------------+---------------------------+
| Re        | Smartphones consumo, apps   | Dispositivos médicos      |
| comendado | móviles                     | SaMD, EMM empresarial     |
| para      |                             |                           |
+-----------+-----------------------------+---------------------------+

La regla de decisión práctica es simple: si el entorno de despliegue
dispone de infraestructura MDM activa y debe cumplir HIPAA, ENS o EU
2017/745, se despliega SV(49, 7). En cualquier otro caso, el par SV(36,
6) + SV(9, 3) ofrece mayor flexibilidad con menor coste de integración.

## 13. Análisis normativo: HIPAA, ENS, RGPD, EU 2017/745

La séptima capa y los parámetros de extensión de SV(49, 7) han sido
diseñados con cobertura directa de los controles técnicos exigidos por
los marcos normativos más relevantes en entornos clínicos y
corporativos.

### 13.1. HIPAA --- Health Insurance Portability and Accountability Act

La Security Rule de HIPAA (45 CFR Part 164) exige controles de acceso
(§164.312(a)), controles de auditoría (§164.312(b)), integridad
(§164.312(c)) y transmisión segura (§164.312(e)).

+-----------------+-------------------------+-------------------------+
| Control HIPAA   | Referencia normativa    | Parámetros SV(49,7)     |
+=================+=========================+=========================+
| Control de      | §164.312(a)(1) /        | P25, P26, P41, P39, P45 |
| acceso          | (a)(2)(iv)              |                         |
+-----------------+-------------------------+-------------------------+
| Control de      | §164.312(b)             | P33, P48, P43           |
| auditoría       |                         |                         |
+-----------------+-------------------------+-------------------------+
| Integridad      | §164.312(c)(1)          | P14, P27, P39           |
+-----------------+-------------------------+-------------------------+
| Transmisión     | §164.312(e)(1)(ii)      | P2, P4, P37, P44        |
| segura          |                         |                         |
+-----------------+-------------------------+-------------------------+
| Backup ePHI     | §164.312(a)(2)(i)       | P46                     |
+-----------------+-------------------------+-------------------------+

### 13.2. ENS --- Esquema Nacional de Seguridad (RD 311/2022)

Para dispositivos en categoría Media o Alta, las medidas de protección
de equipos (MP.EQ) y comunicaciones (MP.COM) son obligatorias.

+------------------+----------------+---------------------------------+
| Medida ENS       | Referencia     | Parámetros SV(49,7)             |
+==================+================+=================================+
| MP.EQ.3 ---      | RD 311/2022    | P39, P45, P43, P44              |
| Protección       | Anexo II       |                                 |
| equipos          |                |                                 |
| portátiles       |                |                                 |
+------------------+----------------+---------------------------------+
| MP.COM.1 ---     | RD 311/2022    | P1, P2, P6, P37, P44            |
| Perímetro seguro |                |                                 |
+------------------+----------------+---------------------------------+
| MP.COM.2 ---     | RD 311/2022    | P35, P44, P37                   |
| Protección de la |                |                                 |
| confidencialidad |                |                                 |
+------------------+----------------+---------------------------------+
| MP.SI.3 ---      | RD 311/2022    | P39                             |
| Cifrado de       |                |                                 |
| información      |                |                                 |
+------------------+----------------+---------------------------------+
| OP.EXP.6 ---     | RD 311/2022    | P33, P43, P48                   |
| Gestión de       |                |                                 |
| incidentes       |                |                                 |
+------------------+----------------+---------------------------------+

### 13.3. RGPD --- Reglamento General de Protección de Datos

El Art. 32 del RGPD exige medidas técnicas apropiadas para garantizar la
seguridad de los datos personales. Los parámetros P39 (cifrado en
reposo), P44 (VPN), P37 (proxy TLS), P46 (backup no autorizado) y P48
(integridad temporal de logs) cubren directamente estos requisitos. El
Art. 9 aplica cuando el dispositivo procesa datos de salud (categoría
especial); en ese caso, P40 (geofencing) y P43 (perfil MDM) adquieren
relevancia en la limitación de acceso por contexto.

### 13.4. EU 2017/745 --- Reglamento de Productos Sanitarios (MDR)

El Reglamento EU 2017/745 clasifica los *Software as a Medical Device*
(SaMD). El Anexo I §17 exige que los SaMD incorporen principios de
mínimo privilegio, protección contra acceso no autorizado y trazabilidad
de eventos de seguridad. SVcustos con arquitectura SV(49, 7) puede
actuar como componente de supervisión dentro de la arquitectura del
SaMD, aportando:

-   **P43 y P49:** verificación de que el SaMD opera dentro de las
    restricciones definidas por el fabricante.

-   **P33 y P48:** integridad de los registros de auditoría requeridos
    por el MDR.

-   **P46:** control de backup no autorizado de datos clínicos
    procesados por el SaMD.

La integración de SVcustos en un SaMD requiere evaluación de conformidad
bajo MDR, incluyendo análisis de riesgo (ISO 14971) y documentación
técnica según Anexo II del MDR. SVcustos no es por sí mismo un producto
sanitario; es un componente de seguridad que puede formar parte de uno.

### 13.5. Vectores de amenaza específicos del entorno clínico

Los siguientes cinco vectores son detectados por SV(49, 7) y no tienen
representación en ningún sistema de detección de intrusiones de red
convencional.

+------------+--------------------+-----------------------------------+
| Vector     | Descripción        | Parámetros detectores             |
+============+====================+===================================+
| **Ma       | Un atacante con    | P37 (AccessibilityService) + P43  |
| nipulación | control del        | (perfil MDM)                      |
| de         | servicio de        |                                   |
| interfaz   | accesibilidad      |                                   |
| clínica**  | puede modificar    |                                   |
|            | los valores        |                                   |
|            | visualizados en la |                                   |
|            | pantalla del       |                                   |
|            | dispositivo        |                                   |
|            | clínico sin        |                                   |
|            | alterar los datos  |                                   |
|            | en el servidor. El |                                   |
|            | profesional        |                                   |
|            | sanitario ve       |                                   |
|            | valores normales   |                                   |
|            | mientras el        |                                   |
|            | registro real      |                                   |
|            | muestra anomalías. |                                   |
|            | Vector invisible   |                                   |
|            | para cualquier     |                                   |
|            | monitor de         |                                   |
|            | integridad de      |                                   |
|            | datos.             |                                   |
+------------+--------------------+-----------------------------------+
| **Ex       | Un técnico con     | P42 (USB/OTG no reconocido) + P47 |
| filtración | acceso físico      | (ADB activo sin sesión            |
| de datos   | puede exfiltrar    | autorizada)                       |
| de salud   | todos los datos de |                                   |
| por canal  | salud del paciente |                                   |
| físico**   | por USB o mediante |                                   |
|            | un backup ADB en   |                                   |
|            | menos de 30        |                                   |
|            | segundos. Este     |                                   |
|            | vector no activa   |                                   |
|            | ninguno de los     |                                   |
|            | parámetros         |                                   |
|            | P1--P36.           |                                   |
+------------+--------------------+-----------------------------------+
| **         | En litigios        | P48 (hora sistema manipulada).    |
| Compromiso | médicos, los       | Umbral: desviación \> 300 s       |
| de         | timestamps tienen  | sostenida durante \> 60 s         |
| timestamp  | valor legal y      | respecto al servidor NTP          |
| con valor  | pericial. Una      | corporativo.                      |
| legal**    | modificación del   |                                   |
|            | reloj previa a un  |                                   |
|            | evento adverso     |                                   |
|            | puede hacer        |                                   |
|            | indetectable la    |                                   |
|            | relación temporal  |                                   |
|            | entre el evento y  |                                   |
|            | las mediciones del |                                   |
|            | dispositivo.       |                                   |
+------------+--------------------+-----------------------------------+
| **         | La desactivación   | P44 (VPN corporativa              |
| Exposición | de la VPN          | desactivada). Ventana de          |
| de HCE a   | corporativa expone | tolerancia: 30 s.                 |
| red        | el tráfico de HCE  |                                   |
| ho         | a la red           |                                   |
| spitalaria | hospitalaria no    |                                   |
| no         | segmentada antes   |                                   |
| se         | de que cualquier   |                                   |
| gmentada** | dato sea           |                                   |
|            | exfiltrado.        |                                   |
+------------+--------------------+-----------------------------------+
| **Escalada | El root en un      | P39 en combinación con P43. P39   |
| de         | dispositivo        | también invalida la fiabilidad de |
| pr         | clínico invalida   | P14, P18 y P34 al permitir        |
| ivilegios: | la certificación   | falsificación desde el nivel de   |
| in         | CE según EU        | kernel.                           |
| validación | 2017/745 y puede   |                                   |
| de         | invalidar la       |                                   |
| cer        | cobertura del      |                                   |
| tificación | seguro de          |                                   |
| CE**       | responsabilidad    |                                   |
|            | civil del centro   |                                   |
|            | sanitario. No es   |                                   |
|            | solo una medida de |                                   |
|            | seguridad          |                                   |
|            | informática: es    |                                   |
|            | una obligación     |                                   |
|            | regulatoria.       |                                   |
+------------+--------------------+-----------------------------------+

## 14. Impacto del despliegue de SV(49, 7)

+------------------+----------------+------------------+--------------+
| Dimensión        | SV(36,6)       | Par SV(45)       | SV(49,7)     |
+==================+================+==================+==============+
| Parámetros       | 36             | 45               | 49           |
| totales          |                |                  |              |
+------------------+----------------+------------------+--------------+
| Capas            | 6×6            | (6×6)+(3×3)      | **7×7**      |
+------------------+----------------+------------------+--------------+
| Cobertura MDM    | ❌ Sin         | ❌ Sin cobertura | **✓ C7       |
|                  | cobertura      |                  | completa**   |
+------------------+----------------+------------------+--------------+
| Conformidad      | Parcial        | Parcial          | **Alta**     |
| HIPAA            |                |                  | (P39, P46,   |
|                  |                |                  | P48)         |
+------------------+----------------+------------------+--------------+
| Detección        | ❌             | ❌               | **✓ P42,     |
| evasión MDM      |                |                  | P43, P47**   |
+------------------+----------------+------------------+--------------+
| Espacio          | 3³⁶ ≈ 1,5×10¹⁷ | 3⁴⁵ ≈ 2,95×10²¹  | 3⁴⁹ ≈        |
| combinacional    |                |                  | 2,39×10²³    |
+------------------+----------------+------------------+--------------+
| Umbral INTRUSIÓN | 28             | max(cls₃₆, cls₉) | **38         |
|                  |                |                  | unificado**  |
+------------------+----------------+------------------+--------------+
| Adecuado para    | ❌             | Limitado         | **✓**        |
| SaMD (EU         |                |                  |              |
| 2017/745)        |                |                  |              |
+------------------+----------------+------------------+--------------+

## 15. Coste de implementación

### 15.1. Coste incremental respecto a SV(36, 6)

El coste adicional de SV(49, 7) respecto a SV(36, 6) se concentra en
cuatro áreas:

-   **Infraestructura MDM:** requiere un servidor EMM activo (Microsoft
    Intune, VMware Workspace ONE, SOTI MobiControl u equivalente). El
    coste de licencia varía entre 4--12 €/dispositivo/mes en soluciones
    SaaS.

-   **Integración de parámetros:** los 13 nuevos parámetros requieren
    acceso a `DevicePolicyManager` (API de Android Enterprise). El
    dispositivo debe estar enrolado en Work Profile o Device Owner mode.

-   **Latencia de clasificación:** la evaluación de 49 parámetros es
    \~36% más costosa que 36, pero el tiempo absoluto sigue siendo
    inferior a 50 ms en dispositivos con SoC moderno (ARM Cortex-A73+).

-   **Formación:** el personal de TI clínico necesita formación
    específica en Android Enterprise, MDM y los requisitos normativos de
    los parámetros de la capa C7.

En entornos donde el MDM ya está desplegado (casos típicos en hospitales
y grandes empresas), el coste incremental es **marginal**: los
parámetros P37--P49 se nutren de APIs que el MDM ya consulta para sus
propios controles.

### 15.2. Plan de implementación en tres fases

Se propone una implementación progresiva. En ningún caso se recomienda
avanzar a SV(49, 7) sin haber validado empíricamente SV(36, 6) en el
mismo entorno.

+-------+----------------+---------------------+------------------------+
| Fase  | Parámetros     | Entorno objetivo    | Criterio de avance     |
+=======+================+=====================+========================+
| *     | P37, P39, P40, | Cualquier entorno   | Mayor claridad         |
| *Fase | P42, P43, P44, | gestionado con MDM  | técnica, sin           |
| 1**   | P49 (7         | activo              | dependencia de         |
|       | parámetros)    |                     | baseline histórico,    |
|       |                |                     | todos event-driven.    |
|       |                |                     | Alto valor diagnóstico |
|       |                |                     | inmediato.             |
+-------+----------------+---------------------+------------------------+
| *     | P38, P41, P45, | Médico o alta       | Requieren VPN          |
| *Fase | P47, P48 (5    | seguridad (ENS      | corporativa activa,    |
| 2**   | parámetros)    | nivel Alto)         | política MDM de        |
|       |                |                     | pantalla de bloqueo y, |
|       |                |                     | en el caso de P41,     |
|       |                |                     | hardware con permisos  |
|       |                |                     | de kernel. P41 toma    |
|       |                |                     | valor U en watchOS.    |
+-------+----------------+---------------------+------------------------+
| *     | P46 (1         | Opcional; máxima    | Requiere servidor NTP  |
| *Fase | parámetro)     | calibración         | de referencia          |
| 3**   |                |                     | corporativo y          |
|       |                |                     | calibración del umbral |
|       |                |                     | (\> 300 s sostenida    |
|       |                |                     | durante \> 60 s) sobre |
|       |                |                     | datos reales del       |
|       |                |                     | entorno. Peso bajo:    |
|       |                |                     | falsos positivos por   |
|       |                |                     | sincronización         |
|       |                |                     | legítima.              |
+-------+----------------+---------------------+------------------------+

## 16. Discusión

### 16.1. La restricción algebraica n = b² como ventaja de diseño

La restricción n = b² no es una limitación: es una invariante que
garantiza que la arquitectura de capas cuadradas es siempre posible y
que el umbral T(n) = ⌊7n/9⌋ se comporta de forma predecible. Para n =
49, T = 38 representa el 77,6% del total de parámetros, una proporción
coherente con la serie:

  n        T(n)     T/n
  -------- -------- -----------
  9        7        77,8%
  16       12       75,0%
  25       19       76,0%
  36       28       77,8%
  **49**   **38**   **77,6%**

### 16.2. La capa C7 como interfaz normativa

La Capa 7 (Gestión MDM) tiene una característica única: sus parámetros
no detectan comportamientos de usuario ni patrones de tráfico, sino
**estados de conformidad** con políticas externas. Esto representa un
salto semántico respecto a las capas C1--C6: mientras que P1 (URL no
autorizada) detecta comportamiento, P43 (perfil MDM eliminado) detecta
la **ausencia de control**. Este patrón de detección negativa es
especialmente robusto frente a adversarios sofisticados que intentan
evadir sin dejar rastro activo.

### 16.3. Grados de libertad en la regla de clasificación

Como se señaló en el Documento 1 (exploración de la condición nU ≥ 1
para INDETERMINADO) y se reafirmó en el Documento 5, cada célula SV
puede particularizar su regla de clasificación según el dominio
semántico de sus parámetros. En SV(49, 7), los parámetros normativos de
C7 podrían recibir tratamiento diferencial: P39 (cifrado desactivado) o
P43 (perfil MDM eliminado) podrían activar INTRUSIÓN directamente, sin
necesidad de alcanzar el umbral 38, si la política del entorno clínico
así lo exige. Esta es una extensión válida que no rompe la arquitectura
base.

### 16.4. Relación con el par SV(36,6) + SV(9,3) del Documento 5

Algunos parámetros de SV(49, 7) cubren territorios semánticamente
próximos a los de la célula SV(9, 3) (integridad del SO). Sin embargo,
el enfoque es distinto: la célula SV(9, 3) detecta si el propio sensor
SVcustos es fiable (*¿está el SO comprometido?*), mientras que SV(49, 7)
detecta si el dispositivo cumple la política corporativa (*¿está el
dispositivo conforme al MDM?*). Son preguntas distintas, con semántica
distinta, que pueden coexistir en despliegues que requieran ambas
dimensiones.

### 16.5. Viabilidad técnica y frecuencias de muestreo

El coste computacional de los parámetros nuevos es predominantemente
event-driven. Los parámetros P37, P38, P40, P42, P43, P44, P47 y P49 se
capturan mediante listeners de Android/iOS sin polling activo. Los que
requieren polling:

+--------+----------------+---------------------------------+---------+
| Cód.   | Parámetro      | Frecuencia recomendada          | Notas   |
+========+================+=================================+=========+
| P38    | Canal MDM      | Heartbeat MDM (típico 4--24 h)  | Event   |
|        | (silencio)     |                                 | -driven |
|        |                |                                 | en      |
|        |                |                                 | dispo   |
|        |                |                                 | sitivos |
|        |                |                                 | con FCM |
|        |                |                                 | activo  |
+--------+----------------+---------------------------------+---------+
| P39    | Cifrado en     | Arranque + cada 3.600 s         | `g      |
|        | reposo         |                                 | etStora |
|        |                |                                 | geEncry |
|        |                |                                 | ptionSt |
|        |                |                                 | atus()` |
|        |                |                                 | es      |
|        |                |                                 | llamada |
|        |                |                                 | local   |
+--------+----------------+---------------------------------+---------+
| P41    | Política       | Cada 300 s                      | `i      |
|        | contraseñas    |                                 | sActive |
|        | MDM            |                                 | Passwor |
|        |                |                                 | dSuffic |
|        |                |                                 | ient()` |
|        |                |                                 | ---     |
|        |                |                                 | coste   |
|        |                |                                 | \< 0,1% |
|        |                |                                 | bate    |
|        |                |                                 | ría/día |
+--------+----------------+---------------------------------+---------+
| P43    | Perfil MDM     | Cada 300 s                      | Verif   |
|        |                |                                 | icación |
|        |                |                                 | de hash |
|        |                |                                 | del     |
|        |                |                                 | perfil  |
|        |                |                                 | sobre   |
|        |                |                                 | archivo |
|        |                |                                 | local   |
+--------+----------------+---------------------------------+---------+
| P45    | Pantalla de    | Cada 600 s                      | `       |
|        | bloqueo        |                                 | getPass |
|        |                |                                 | wordQua |
|        |                |                                 | lity()` |
|        |                |                                 | es      |
|        |                |                                 | llamada |
|        |                |                                 | local   |
|        |                |                                 | sin     |
|        |                |                                 | acceso  |
|        |                |                                 | a red   |
+--------+----------------+---------------------------------+---------+
| P48    | Hora del       | NTP delta cada 30 s; umbral     | Excluye |
|        | sistema        | activación: \> 300 s durante \> | ajustes |
|        |                | 60 s                            | le      |
|        |                |                                 | gítimos |
|        |                |                                 | por     |
|        |                |                                 | diseño  |
|        |                |                                 | del     |
|        |                |                                 | umbral  |
+--------+----------------+---------------------------------+---------+

El incremento en tráfico de telemetría de SV(36, 6) a SV(49, 7) se
estima en **menos del 15%**. Nota de portabilidad: en Apple Watch y
dispositivos con watchOS, P41 y el equivalente de red promiscua toman
valor U de forma sistemática, que es el comportamiento correcto del
framework para parámetros no medibles en la plataforma.

## 17. Conclusión

El Documento 6 completa la arquitectura SVcustos de seguridad con la
definición de SV(49, 7). Sus contribuciones principales son:

-   Extensión coherente de n = 36 a n = 49 preservando la restricción n
    = b² y la regla T(n) = ⌊7n/9⌋.

-   Incorporación de 13 nuevos parámetros (P37--P49) que cubren la
    dimensión de gestión MDM, ausente en todas las arquitecturas
    anteriores.

-   Seis premisas de aplicación (P-1 a P-6) que delimitan con precisión
    el contexto de despliegue válido.

-   Cinco vectores de amenaza clínica específicos (manipulación de
    interfaz, exfiltración física, compromiso de timestamp, exposición
    de HCE, invalidación CE) con representación paramétrica directa en
    la capa C7.

-   Cobertura normativa directa de HIPAA §164.312, ENS MP.EQ.3/MP.COM,
    RGPD Art. 32 y EU 2017/745 Anexo I §17.

-   Plan de implementación en tres fases que permite validar
    progresivamente los 13 nuevos parámetros sin arriesgar la
    estabilidad del sistema base.

-   Un espacio combinacional de 3⁴⁹ ≈ 2,39 × 10²³ vectores ternarios que
    hace inviable la exploración exhaustiva por adversarios.

------------------------------------------------------------------------

**Las dos fronteras de despliegue de SVcustos quedan ahora formalmente
definidas:**

+--------------+------------------+-----------------------------------+
| Frontera     | Arquitectura     | Documento de referencia           |
+==============+==================+===================================+
| **Frontera   | Par de células   | Documento 5 de 8 --- «Células SV  |
| de consumo** | SV(36,6) +       | en par»                           |
|              | SV(9,3) --- 45   |                                   |
|              | parámetros,      |                                   |
|              | arquitectura     |                                   |
|              | dual-cell, regla |                                   |
|              | max(cls₃₆,       |                                   |
|              | cls₉). Sin       |                                   |
|              | requisito de     |                                   |
|              | MDM.             |                                   |
+--------------+------------------+-----------------------------------+
| **Frontera   | Célula única     | Documento 6 de 8 --- este         |
| de entorno   | SV(49,7) --- 49  | documento                         |
| gestionado** | parámetros, 7    |                                   |
|              | capas de 7,      |                                   |
|              | umbral unificado |                                   |
|              | n₁ ≥ 38.         |                                   |
|              | Requiere MDM     |                                   |
|              | activo y         |                                   |
|              | premisas P-1 a   |                                   |
|              | P-5.             |                                   |
+--------------+------------------+-----------------------------------+

Estas dos fronteras no son alternativas excluyentes. Un despliegue
hospitalario puede combinarlas: el par de células para smartwatches de
consumo de los pacientes (sin MDM disponible) y la célula única SV(49,
7) para los dispositivos gestionados del personal clínico. El Documento
7 inaugurará la trasposición de esta misma arquitectura al dominio del
conocimiento experto: SVperitus, donde la invariante algebraica n = b² y
la regla T(n) = ⌊7n/9⌋ siguen operando, pero sobre parámetros de
competencia profesional en lugar de parámetros de seguridad de
dispositivo.

## 18. Referencias

1.  Lloret Egea, J.A. (2025). El nivel base: 9 parámetros y el origen
    del sistema. SVcustos Doc. 1/8.

2.  Lloret Egea, J.A. (2025). De n = 9 a n = 16: primera extensión.
    SVcustos Doc. 2/8.

3.  Lloret Egea, J.A. (2025). De n = 16 a n = 25: segunda extensión.
    SVcustos Doc. 3/8.

4.  Lloret Egea, J.A. (2025). De n = 25 a n = 36: tercera extensión.
    SVcustos Doc. 4/8.

5.  Lloret Egea, J.A. (2026). Células SV en par: n = 36 + n = 9.
    SVcustos Doc. 5/8.

6.  U.S. Department of Health and Human Services (2003). HIPAA Security
    Rule, 45 CFR Part 164.

7.  Gobierno de España (2022). Real Decreto 311/2022 --- Esquema
    Nacional de Seguridad. BOE núm. 29.

8.  Parlamento Europeo y Consejo (2016). Reglamento (UE) 2016/679 ---
    RGPD. DOUE L 119.

9.  Parlamento Europeo y Consejo (2017). Reglamento (UE) 2017/745 ---
    Productos Sanitarios (MDR). DOUE L 117.

10. Google (2024). Android Enterprise Security White Paper.
    <https://developers.google.com/android/work>

11. ENISA (2021). Guidelines for Securing the Internet of Things (IoT).
    ENISA Publications Office.

12. ISO (2019). ISO 14971:2019 --- Application of risk management to
    medical devices.

13. Android Open Source Project (2024). DevicePolicyManager API
    reference. <https://developer.android.com>

14. NIST (2015). SP 800-53 Rev. 5 --- Security and Privacy Controls for
    Information Systems.

------------------------------------------------------------------------

## 19. Mapa de la serie completa

+------------------+-------------------------+-------------------------+
| Doc.             | Título                  | Estado                  |
+==================+=========================+=========================+
| 1 de 8           | *El nivel base: 9       | Publicado               |
|                  | parámetros y el origen  |                         |
|                  | del sistema*            |                         |
+------------------+-------------------------+-------------------------+
| 2 de 8           | *De n = 9 a n = 16:     | Publicado               |
|                  | primera extensión*      |                         |
+------------------+-------------------------+-------------------------+
| 3 de 8           | *De n = 16 a n = 25:    | Publicado               |
|                  | segunda extensión*      |                         |
+------------------+-------------------------+-------------------------+
| 4 de 8           | *De n = 25 a n = 36:    | Publicado               |
|                  | tercera extensión*      |                         |
+------------------+-------------------------+-------------------------+
| 5 de 8           | *Células SV en par: n = | Publicado               |
|                  | 36 + 9 = 45*            |                         |
+------------------+-------------------------+-------------------------+
| **6 de 8**       | ***De n = 36 a n = 49:  | **Este documento**      |
|                  | extensión para entornos |                         |
|                  | gestionados y           |                         |
|                  | médicos***              |                         |
+------------------+-------------------------+-------------------------+
| 7 de 8           | *SVperitus: agentes     | Próximo                 |
|                  | especializados ---      |                         |
|                  | instancia inmunología*  |                         |
+------------------+-------------------------+-------------------------+
| 8 de 8           | *Documento compilador   | Pendiente               |
|                  | de la serie completa*   |                         |
+------------------+-------------------------+-------------------------+

**Documento 7 de 8 --- Próximo:** *SVperitus: agentes especializados ---
instancia inmunología*

El séptimo documento abandona el dominio de la seguridad de dispositivos
y trasplanta la arquitectura SVcustos al dominio del conocimiento
experto. SVperitus (SV = Sistema Vectorial, *peritus* = experto
verificado en latín) define ontologías de n = 625 = 25² parámetros para
verificar la competencia profesional de agentes de IA en dominios
especializados.

La instancia detallada del Documento 7 es el inmunólogo clínico: 25
capas temáticas × 25 parámetros de competencia. El espacio combinacional
es 3⁶²⁵ ≈ 10²⁹⁸. El Documento 7 formalizará el conector arquitectónico
entre SVcustos y SVperitus: la misma invariante algebraica n = b², la
misma regla de umbral T(n) = ⌊7n/9⌋, distinto dominio semántico de los
parámetros.

------------------------------------------------------------------------
