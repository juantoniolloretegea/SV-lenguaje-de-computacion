---
article:
  doi: 10.21428/39829d0b.981b7276
  elocation-id: framework-basado-en-imagenes-parametrizadas-sobre-resnet-para-identificar-intrusiones-en-smartwatches-u-otros-dispositivos-afines
author:
- Juan Antonio Lloret Egea
- Celia Medina Lloret
- Adrián Hernández González
- Diana Díaz Raboso
- Carlos Campos
- Kimberly Riveros Guzmán
- Luis Miguel Cortés Carballo
- Héctor Miguel Terrés Lloret
- Miguel Hermoso Hernando
- Instituto tecnológico virtual IAeñ™
bibliography: /tmp/tmp-20A5JAKGbuSeFw.json
copyright:
  link: "https://creativecommons.org/licenses/by-nc-nd/4.0/"
  text: Creative Commons Attribution-NonCommercial-NoDerivatives 4.0
    International License
  type: CC-BY-NC-ND
csl: /app/dist/server/server/utils/citations/citeStyles/apa-7th-edition.csl
date:
  day: 24
  month: 07
  year: 2021
journal:
  publisher-name: IA eñ™ --- La Biblia de la IA™ · ISSN 2695-6411 ·
    Instituto Tecnológico Virtual de la Inteligencia Artificial para el
    Español (ITVIA).
  title: IA eñ ™
link-citations: true
title: "'Framework' basado en imágenes parametrizadas sobre ResNet para
  identificar intrusiones en 'smartwatches' u otros dispositivos afines.
  (Un eje singular de la publicación \"Estado del arte de la ciencia de
  datos en el idioma español y su aplicación en el campo de la
  Inteligencia Artificial\")"
uri: "https://www.itvia.online/pub/framework-basado-en-imagenes-parametrizadas-sobre-resnet-para-identificar-intrusiones-en-smartwatches-u-otros-dispositivos-afines"
---

> **Nota de actualización --- 19/06/2026.** Esta actualización
> incorpora, antes del cuerpo principal, una nota de continuidad
> bibliográfica y técnica basada en la trayectoria pública indicada en
> ORCID: [0000-0002-6634-3351](https://orcid.org/0000-0002-6634-3351).
> La publicación que aquí se restaura no se presenta como un hecho
> aislado: queda situada dentro de una línea de trabajo iniciada en 2019
> sobre inteligencia artificial, ética, regulación, tecnologías
> emergentes, lógica ternaria, interfaces, procesamiento del lenguaje
> natural, biointeligencia, imágenes parametrizadas y control humano de
> sistemas de IA.
>
> La versión nueva conserva el cuerpo histórico de 2020--2021 ([cuya
> versión original está disponible
> aquí](https://www.itvia.online/pub/framework-basado-en-imagenes-parametrizadas-sobre-resnet-para-identificar-intrusiones-en-smartwatches-u-otros-dispositivos-afines/release/1 "null"))
> y añade ahora, al cierre de la Conclusión, el apartado **12.5.-
> Actualización de continuidad SV --- 19/06/2026**. Esta actualización
> explica el framework de imágenes parametrizadas sobre ResNet como
> antecedente técnico directo de la célula SV, de la imagen/frame, de la
> trazabilidad humano--IA y de la física del suceso desarrollada
> posteriormente en el [Sistema Vectorial
> SV](https://doi.org/10.21428/39829d0b.06811a0d).
>
> ([Acceso directo a la actualización de continuidad
> SV](https://www.itvia.online/pub/framework-basado-en-imagenes-parametrizadas-sobre-resnet-para-identificar-intrusiones-en-smartwatches-u-otros-dispositivos-afines#actualización-de-continuidad-sv-19062026 "null")).

# Abstracto

------------------------------------------------------------------------

Se ha definido un *framework*^[1](#nota-1)^ conceptual y
algebraicamente, inexistente hasta ahora en su morfología, y pionero en
aplicación en el campo de la Inteligencia Artificial (IA) de forma
conjunta; e implementado en laboratorio, en sus aspectos más
estructurales, como un modelo completamente operacional. Su mayor
aportación a la IA a nivel cualitativo es aplicar la conversión o
transducción de parámetros obtenidos con lógica ternaria (sistemas
multivaluados)^[2](#nota-2)^ y asociarlos a una imagen, que es analizada
mediante una red residual artificial ResNet34 para que nos advierta de
una intrusión. El campo de aplicación de este *framework* va desde
*smartwatches*, *tablets* y PC\'s hasta la domótica basada en el
estándar KNX.

Este marco propone para la IA una ingeniería inversa de tal modo que
partiendo de principios matemáticos conocidos y revisables, aplicados en
una imagen gráfica en 2D para detectar intrusiones, sea escrutada la
privacidad y la seguridad de un dispositivo mediante la inteligencia
artificial para mitigar estas lesiones a los usuarios finales.

Asimismo se postula el *framework* de manera que cumpla con las mayores
expectativas de ser una herramienta útil para la sociedad y el ser
humano, alineado con el «Dictamen del Comité Europeo de las Regiones \--
Libro Blanco sobre la inteligencia artificial \-- Un enfoque europeo
orientado a la excelencia y la confianza (URL:
`op.europa.eu/es/publication-detail/-/publication/12738974-40d7-11eb-b27b-01aa75ed71a1/language-es/format-PDF/source-search)`»
y que contribuya a evitar la mala *praxis* de generar cajas negras en el
empleo de la inteligencia artificial haciendo de ella algo inexplicable.

La confianza y la explicabilidad en la tecnología es un objetivo a
corto, medio y largo plazo para que la sociedad no quede empeñada en
manos de tecnologías espurias que hipotecan un desarrollo social
adecuado. «Estas tecnologías han ampliado las oportunidades de libertad
de expresión y de movilización social, cívica y política, pero suscitan
a la vez graves preocupaciones».

------------------------------------------------------------------------

# I.-*Framework*

## 1.- Justificación de la necesidad

Existe una amplia disponibilidad de dispositivos «inteligentes» que
todos tenemos a nuestro alcance, desde cámaras fotográficas digitales,
pasando por *tablets*y *smartwatches*. El problema ocurre cuando el
desconocimiento en el manejo de estos aparatos electrónicos se extiende
a un gran porcentaje de la población, pues ignoran los riesgos que
pueden acarrear. Si bien es cierto que un dispositivo electrónico es
**inteligente** por funcionar de forma interactiva y autónoma, no
debemos olvidar que para que **algo** aporte beneficios y soluciones en
situaciones concretas, ese **algo** debe conocer previamente lo que está
ocurriendo. Todo esto se traduce en una única palabra: **datos**.

Uno de los dispositivos electrónicos que más se ha puesto de moda en los
últimos tiempos es el *smartwatch*. Los *smartwatches* tienen sensores
que identifican modelos o patrones de comportamiento humano junto con
técnicas de aprendizaje automático, el teorema de Bayes, el
procesamiento de datos o el también conocido como método k de los
vecinos más cercanos. Estos procedimientos generan un gran volumen de
información con el que pretendemos cumplir el objetivo de precisar en
los resultados emitidos. Estos sensores son muy útiles para monitorizar
la actividad humana: caminar, montar en bicicleta, \*running \*y subir o
bajar escaleras .

El uso de *smartwatches*puede suponer una grave amenaza para la
seguridad de niños y adolescentes. Los fallos de seguridad más
importantes de los relojes de bajo coste se producen en las aplicaciones
y en la conexión con los servidores que almacenan los datos. Las marcas
de *smartwatch*más utilizadas por menores que hemos analizado son: Carl
Kids Watch, hellOO! Children\'s Smart Watch, SMA-WATCH-M2 y GATOR Watch.
Los problemas más recurrentes que encontramos son: fallos en la
implementación de certificados para conexiones seguras HTTPS e
información relativa al registro electrónico sin cifrar. Sin embargo, en
dispositivos de marcas más reconocidas como Nokia, Samsung o Huawei,
estos problemas no ocurren con frecuencia, ya que se realizan a través
de conexiones cifradas.

En este punto nos parece interesante comparar las características que
afectan a la seguridad de las diferentes marcas de relojes. Esta es una
forma de conocer el nivel de confianza que debemos depositar en cada una
de ellas. El criterio seguido para elegir las marcas es tener en cuenta
el mayor número de ventas del mercado: Samsung, Apple, Fitbit y Garmin.

![](https://assets.pubpub.org/fma4qgfc/31625567334457.svg){#nb01qf40uic}

Crédito de imagen: Seguridad en dispositivos Samsung y Apple por Adrián
Hernández, 2021, Mangosta
(`mangosta.org/seguridad-en-los-dispositivos-samsung-y-apple/)`

![](https://assets.pubpub.org/yjy9unxe/01625567427088.svg){#n05nwtcvwqb}

Crédito de imagen: Seguridad en dispositivos Fitbit y Garmin por Adrián
Hernández, 2021, Mangosta
(`mangosta.org/seguridad-en-dispositivos-fitbit-y-garmin/)`

En la recopilación de datos, Samsung es la única marca que no compila
información *online* de menores de 13 años. Sin embargo, esto no ocurre
con Apple, Fitbit y Garmin, que no hacen distinción de edad. Asimismo,
todas las marcas elegidas para nuestro estudio comparten datos con
terceros para analizar métricas y comparar resultados. De acuerdo con
estas referencia, **solo en Samsung los datos de los usuarios son
vendidos o alquilados**. **Al contrario de lo que ocurre con Apple; pues
es la única de las empresas analizadas que no comparte datos con fines
de publicidad y/o márquetin**. Por tanto, llegamos a la conclusión de
que todas las marcas, al final, realizan una recopilación de datos no
anónimos aunque con ciertos puntos de disimilitud.

## 2.- Definición algebraica y cálculo del alcance del *framework*

La sistemática empleada se basa en la dualidad o binomio de parámetros
determinantes de una intrusión & imagen asociada. La determinación de
cuáles han de ser estos parámetros debe ser fruto de la experiencia en
el campo de la ciberseguridad y que más adelante nosotros propondremos y
serán expuestos.

### 2.1.- Definición algebraica

Dado un número determinado de parámetros de intrusión denominado «n», se
toma como condición de construcción que n pertenezca a la sucesión de
cuadrados perfectos n = b², con b ≥ 3. Para cada instancia del
*framework* se define un grupo ordenado de parámetros Pₓ(1\...n), cuyos
valores sólo pueden pertenecer al intervalo ternario declarado \[0, 1,
U\].

El número máximo de configuraciones posibles se define como m = 3ⁿ. Cada
configuración del grupo Pₓ tiene una imagen asociada Img_y, generada por
una función de transducción desde el conjunto de parámetros hacia una
representación gráfica en el plano.

Forma general del sistema:

> SV_intrusión = \[Pₓ(1\...n), Img_yᵢ\] desde 0 hasta 3ⁿ

Relación funcional entre parámetros e imagen:

> Img_yᵢ = f(Pₓ(1\...n))

La imagen asociada es consecuencia de una transformación del sistema de
parámetros Pₓ. En términos operativos, cada tramo de la imagen se
obtiene mediante diferencias sucesivas entre parámetros ordenados:

> Img_yᵢ ⇔ \[Pₓ₂ − Pₓ₁, Pₓ₃ − Pₓ₂, Pₓ₄ − Pₓ₃, \..., Pₓₙ − Pₓₙ₋₁, Pₓ₁ −
> Pₓₙ\]

La imagen poligonal cerrada asociada a un determinado *array* de
parámetros intrusivos queda determinada por la condición de cierre:

> Σj=1\^{n} (Pₓⱼ₊₁ − Pₓⱼ) = 0, con retorno final Pₓₙ → Pₓ₁

**A) Dominios del framework**

1.  Dominio D1: Pₓ(1\...n) pertenece al dominio ternario \[0, 1, U\].

2.  Dominio D2: Img_yᵢ = f(Pₓ(1\...n)) pertenece a una representación
    gráfica en ℝ².

3.  Dominio D3: Pₓ pertenece a una estructura ordenada de n parámetros.

4.  Dominio D4: Img_y pertenece al conjunto de imágenes generadas por
    las configuraciones de Pₓ.

**B) Coordenadas polares del framework**

La transformación anterior constituye una línea vectorial poligonal
cerrada. En coordenadas polares, cada parámetro se representa mediante
un radio vector con valor modular ternario 0, 1 o U, y con un ángulo α
determinado por el número de parámetros usados en el escaneo de la
intrusión.

Para n = 9 parámetros, el desfase angular entre ejes es:

> α = 360° / 9 = 40°

Esta representación permite que el sistema genere una imagen radial,
poligonal y cerrada para cada configuración de parámetros.

#### 2.1.1.- Naturaleza de las dimensiones del Sistema Vector de intrusión

Al utilizar gráficos radiales para generar imágenes y emplear los ejes
como radios vectores de una circunferencia en el plano, la parte gráfica
queda ubicada en la aplicación:

> f: ℕ → ℝ²

La estructura dimensional puede expresarse así:

1.  Grupo de parámetros Pₓ(1\...n): dimensión de lectura inicial 1.

2.  Grupo de imágenes Img_yᵢ: representación en el plano ℝ².

3.  Grupo de vectores Pₓ: dimensión n.

4.  Grupo de imágenes Img_y: dimensión n en el conjunto de
    configuraciones posibles.

#### 2.1.2.- Propiedades algebraicas de Pₓ(1\...n) e Img_yᵢ

Las propiedades a analizar para este sistema vectorial van desde
elemento neutro, elemento simétrico, propiedad asociativa, conmutativa y
distributiva, hasta su posible lectura como grupo abeliano, anillo,
cuerpo o espacio vectorial. Esta publicación conserva esa formulación
como horizonte algebraico de trabajo y centra su desarrollo en la
construcción conceptual general del *framework* y en la implementación
de laboratorio.

### 2.1.3.- Inferencias de la definición algebraica

A este modelo matemático así definido puede aplicársele la teoría del
álgebra lineal. Asimismo, los gráficos radiales, que encierran áreas en
sus polígonos irregulares definidos por vértices, pueden operarse
algebraicamente mediante la fórmula del área de Gauss y sus propiedades
adscritas. También resultan pertinentes la lógica ternaria aplicable y
el álgebra combinacional de Boole, entre otros desarrollos posibles.

![](https://assets.pubpub.org/0hd0530u/01626132349933.svg){#nutrdr1k7p0}

Crédito imagen: Ejemplo de la fórmula del área de Gauss. Wikipedia,
Isalar, derivative work: Nat2, Polygon_area_formula.jpg. Mangosta. URL
de referencia: `mangosta.org/formula-del-area-de-gauss/`

### 2.2.- Cálculo del alcance y sistemática de uso

El número máximo «m» de vectores posibles con el muestreo de n = 9
parámetros y lógica ternaria (0, 1, U) es:

> m = 3⁹ = 19.683 configuraciones posibles

El número de configuraciones aumenta conforme se incrementa el número de
parámetros n a 16, 25, 36, etc., siguiendo la lógica de matrices
cuadradas indicada. Para n ≤ 9, el uso de IA no queda suficientemente
justificado, porque cualquier lenguaje de programación podría realizar
esta clasificación con un algoritmo o librería específica. Sin embargo,
cuando n \> 9, la potencia de cálculo y la rapidez de los sistemas
gráficos asociados a la inteligencia artificial justifican su empleo en
el manejo de grandes volúmenes de datos.

Para n = 25, la amplitud configuracional es:

> m = 3²⁵ = 847.288.609.443 configuraciones posibles

La metodología inicial para obtener estos parámetros se apoya en la
captura de reglas en *logs* de IDS y en técnicas conocidas de
monitorización de sistemas y aplicaciones. El *framework* admite también
otras metodologías de obtención de parámetros, siempre que puedan
declararse y traducirse a la estructura ternaria correspondiente.

Los parámetros de intrusión sirven como base para construir un modelo
gráfico de positivos, negativos y entrenamiento, explícitamente
separados y ordenados en unidades contenedoras, carpetas o directorios.
Para cada nueva identidad de parámetros e imagen, la inteligencia
artificial previamente entrenada determina si existe o no una intrusión.
La decisión final puede incorporar la confirmación humana, de manera que
la IA actúe como sonda de alerta y escrutinio, mientras el técnico
humano conserva la decisión final.

## 3.- Definiciones singulares y un ejemplo

#### 3.1.- Definición en Programación Orientada a Objetos (POO)

Al *framework* creado puede abstraérsele la idea de una clase denominada
«Intrusión». La herencia de esta clase, definida mediante 9 parámetros,
generaría una clase derivada denominada «Intrusiónbase9», implementable
en cualquier lenguaje de programación. Para cada instancia de esa clase
se obtiene un objeto del sistema:

> SV_intrusión = \[Pₓ(1\...n), Img_yᵢ\] desde 0 hasta 3ⁿ

Ese objeto se clasifica posteriormente como intrusión verdadera o falsa.

#### 3.2.- Definición de 9 parámetros

Estos nueve parámetros significan una instancia del marco general
descrito anteriormente. Por lo tanto heredan las características del
*framework*. Los valores 0, 1 y U especifican, respectivamente: apagado
o no usado, encendido o en uso, e indeterminado.

  Núm. parámetro asociado a Pₓ(1\...n)   Definición del parámetro                                                                           Procedimiento de obtención recomendado
  -------------------------------------- -------------------------------------------------------------------------------------------------- ----------------------------------------
  n = 1 → Pₓ₁                            Destino de web (URL) desconocido o no autorizado                                                   Regla Snort IDS o similar
  n = 2 → Pₓ₂                            Comunicación no cifrada (HTTPS)                                                                    Regla Snort IDS o similar
  n = 3 → Pₓ₃                            Traspaso de datos locales a otro almacenamiento                                                    Regla Snort IDS o similar
  n = 4 → Pₓ₄                            Uso o conexión a BSSID no permitida o desconocida                                                  Regla Kismet-WIDS o similar
  n = 5 → Pₓ₅                            Uso o conexión a puerto Bluetooth no permitida o desconocida                                       Regla Kismet-WIDS o similar
  n = 6 → Pₓ₆                            Uso o conexión a GPS no permitida o no autorizada                                                  Regla Kismet-WIDS o similar
  n = 7 → Pₓ₇                            Uso o conexión a cámara no permitido o desconocido                                                 Monitor de sistema o similar
  n = 8 → Pₓ₈                            Uso de dispositivo de audio ---micro/altavoz--- no permitido o desconocido                         Monitor de sistema o similar
  n = 9 → Pₓ₉                            Monitorización de parámetros físicos ---velocidad, temperatura, batería--- con resultado anormal   Monitor de sistema o similar

#### 3.3.- Un ejemplo de aplicación

De forma general se tiene:

> Img_yᵢ = f(Pₓ(1\...n))

Para una instancia x = 1 con nueve parámetros:

> Img_1ᵢ = f(P1₁, P1₂, P1₃, P1₄, P1₅, P1₆, P1₇, P1₈, P1₉)

La imagen gráfica a escrutar por la inteligencia artificial se crea a
partir de la transformación siguiente:

> Img1ᵢ ⇔ \[P1₂ − P1₁, P1₃ − P1₂, P1₄ − P1₃, \..., P1₉ − P1₈, P1₁ −
> P1₉\]

Los vectores parciales de Img1ᵢ se construyen dinámicamente en el
sentido de las agujas del reloj a partir de los parámetros P1(1\...9),
con un desfase angular de 40°.

Desarrollo de tramos:

> Img1₁ = P1₂ − P1₁; Img1₂ = P1₃ − P1₂; Img1₃ = P1₄ − P1₃; Img1₄ = P1₅ −
> P1₄; Img1₅ = P1₆ − P1₅; Img1₆ = P1₇ − P1₆; Img1₇ = P1₈ − P1₇; Img1₈ =
> P1₉ − P1₈; Img1₉ = P1₁ − P1₉

Asignando valores previamente escaneados al *array* de parámetros p1 =
(0,1,U,0,1,1,0,U,0), estos definen de forma intrínseca P1(1\...9), y su
imagen gráfica asociada Img1ᵢ se construye como la figura de abajo. A
nivel gráfico se asigna el valor «3» a las indeterminaciones U, y «1» o
«2» a los valores binarios 0 y 1, respectivamente.

![](https://assets.pubpub.org/9ubofqlj/51626256322218.svg){#nc8cotixbt7}

Crédito imagen: Ejemplo de captura de parámetros por Juan Antonio Lloret
Egea, 2021, Mangosta. URL de referencia: `mangosta.org/i_parametros-3/`

# II.- Laboratorio del *framework*

## 4.- Implementación de los 9 parámetros

#### 4.1.- Destino de url desconocido o no autorizado

> log tcp HOME_NET any -\> !DireccionIPConfianza any (sid: 1000001; rev:
> 001;)

> log tcp any any -\> !DireccionIPConfianza any (sid: 1000001; rev:
> 001;)

La regla puede ser definida de dos maneras, una inicial en la que
utilizamos la variable **\"HOME_NET\"** previamente definida, en la que
vamos a especificar cuál es nuestra red, o una segunda forma en la que
no especificamos la red que estamos utilizando sino que creamos la regla
con \"any\" y a pesar de ser menos eficiente en cuanto a recursos, nos
aseguramos de que funcione en cualquier situación. La regla puede leerse
de la siguiente forma: \"Crea un \*log \*cuando se utilice el protocolo
TCP desde nuestra red para enviar paquetes desde cualquier puerto a una
dirección distinta a la dirección de confianza en cualquier puerto.\"

#### 4.2.- Comunicación no cifrada (https)

> log tcp HOME_NET any -\> any any (content:\"http\"; sid: 1000002; rev:
> 001;)

> log tcp any any -\> any any (content:\"http\"; sid: 1000002; rev: 001;
> )

La regla puede ser definida de dos formas, una inicial en la que
utilizamos la variable \"HOME_NET\" previamente definida, en la que
vamos a especificar cuál es nuestra red o una segunda forma en la que no
especificamos la red que estamos utilizando sino que creamos la regla
con \"any\". Ésta, a pesar de ser menos eficiente en cuanto a recursos,
nos aseguramos de que funcione en cualquier situación. La regla puede
leerse de la siguiente manera: \"Crea un log cuando se utilice el
protocolo TCP para enviar paquetes desde cualquier puerto a cualquier
dirección y puerto con contenido HTTP\".

#### 4.3.- Traspaso de datos locales a otro almacenamiento

> log tcp HOME_NET !puerto_confianza -\> any any(sid: 1000003; rev:
> 001;)

> log tcp any !puerto_confianza -\> any any(sid: 1000003; rev: 001;)

La regla puede leerse de la siguiente manera: \"Crea un log cuando se
utilice el protocolo TCP para enviar paquetes desde cualquier puerto que
no sea el de confianza a cualquier dirección y puerto.

#### 4.4.- Uso o conexión a BSSID no permitida o desconocida

Para dispositivos Windows, se puede utilizar Kismet o bien integrarlo en
Python, utilizando el modulo subprocess (a partir de Windows 7) que nos
permite obtener el estado de los procesos de los dispositivos y la
respuesta que nos ofrece el sistema operativo tras introducir una
sentencia en la consola del mismo. En este caso para conocer el BSSID al
que esta conectado el dispositivo utilizaremos: \"netsh wlan show
interfaces\", que nos devolverá entre otros muchos datos el BSSID al que
esta conectado el equipo. Si este BSSID no es un BSSID reconocido,
obtendremos un valor de \"1\" a la hora de realizar el gráfico posterior
que utilizará la Inteligencia Artificial para determinar si estamos ante
una intrusión o no. Si el BSSID es reconocido, almacenaremos un valor
\"0\" y si por el contrario no es posible determinarlo, tendremos un
valor indeterminado en este parámetro.

Para Android, se puede utilizar **WifiInfo**, clase definida por Google
para manejar las conexiones del dispositivo y que tiene como herencia la
clase Objeto. Además se utilizará el método getBSSID () para obtener el
el BSSID, en forma de una dirección MAC de seis bytes: XX: XX: XX: XX:
XX: XX. Nuevamente si el BSSID es reconocido, se almacena como un \"0\",
si no lo es, se almacena como un \"1\". Además, si no es posible
determinar el estado de la conexión en el dispositivo, se almacenará
como indeterminado.

#### 4.5.- Uso o conexión a Puerto Bluetooth no permitida o desconocida

Para dispositivos Windows, se puede utilizar Kismet o bien integrarlo en
Python, utilizando la librería PyBluez que nos permite obtener el estado
de la conexión bluetooth.

Para Android, se puede utilizar \*\*BluetoothProfile\* \*\*interfaz
(colección de métodos y constantes), definida por Google para manejar el
Bluetooth en los dispositivos Android. Además necesitaremos utilizar
otra interfaz: \*\*ServiceListener\*\*\*\* \*\*que nos permitirá listar
de manera más rápida los estados (constantes) que buscamos:

**STATE_CONNECTING: Dispositivo en estado de conexión.
\*\*STATE_DISCONNECTING**: Dispositivo en estado de desconexión.
**STATE_CONNECTED**: Dispositivo conectado. **STATE_DISCONNECTED**:
Dispositivo desconectado.

Estos estados pueden parecer iguales dos a dos, la diferencia es que en
los dos primeros obtendremos el estado de nuestra conexión Bluetooth, es
decir, si tenemos activado o no el Bluetooth, y en los dos últimos
recibiremos información sobre si nuestra conexión está o no activa. Sin
embargo, un dispositivo puede tener el Bluetooth activo pero no estar
conectado a ningún otro dispositivo.

#### 4.6.- Uso o conexión a GPS no permitida o no autorizada

Para dispositivos Windows se puede utilizar Kismet o bien integrarlo en
Python, utilizando la librería gpsd que nos permite obtener la posición
del GPS al ejecutar el *script*. Si no existe conexión, no obtendremos
ninguna posición y por lo tanto podremos almacenar el valor como \"0\".
Por el contrario, si obtenemos una posición cualquiera, almacenaremos el
valor como \"1\".

Para Android se puede utilizar `GnssStatus.Callback`**\*\******,*****
clase definida por Google para manejar el sistema global de navegación
por satélite en los dispositivos Android. Otra de las posibilidades era
trabajar con \*\*GpsStatus\*\*\*\* \*\*pero no se podría monitorizar
\"GLONASS\", \"GALILEO\",\"BEIDOU\", etc. Por otra parte, utilizaremos
el método definido \*\*onStarted() \*\*que nos avisa cuando GNSS se
activa y \*\*onStopped()**, su análogo para cuando el proceso se
detiene.

#### 4.7.- Uso o conexión a cámara no permitido o desconocido

Para dispositivos Windows se utiliza la biblioteca cv2 en Python, que
nos permite tener control sobre los dispositivos de vídeo, en este caso
para comprobar si la cámara está activa o no, al ejecutar el fichero. En
el caso de que la cámara esté *online* para el entorno de pruebas nos
aparece un mensaje como este:

![](https://assets.pubpub.org/lw0tmmyd/61626780960729.png){#nytfn9bnyz0}

Crédito de imagen: Salida por consola estado del cámara online

por Adrián Hernández, 2021, Mangosta (`mangosta.org/i_parametros-2/)`

Y además, almacenaremos el valor como \"1\" en la variable determinada
para realizar el gráfico con el resto de parámetros. Si por el contrario
la cámara está *offline*, en el entorno de pruebas nos aparece el
siguiente mensaje:

![](https://assets.pubpub.org/8mm1sv33/31626780826856.png){#n17j2gj1s40}

Image credit: Console output offline camera status

por Adrián Hernández, 2021, Mangosta (`mangosta.org/ii_parametros-2/)`

En este caso se almacenaría el valor \"0\" a la hora de realizar el
gráfico.

Para dispositivos Android, utilizamos `android.hardware.camera`**2** y
utilizamos el \*callback \*de Android: `CameraDevice.StateCallback` que
incluye el método \*\*\*o\*\*\*\*\*nOpened(CameraDevice
camera)\*\*\*\*\*, \*\*\*que nos devuelve si la cámara esta abierta o
no.

#### 4.8.- Uso de dispositivo de audio (micro / altavoz) no permitido o desconocido

Para dispositivos Windows, se utiliza Python y la biblioteca PyAudio, la
cual nos permite capturar una señal de audio a través del micrófono y
posteriormente visualizar su representación temporal. El objetivo es
capturar la onda de audio en el instante de tiempo que consideremos y si
esa onda no es plana, el micrófono estaría activo.

![](https://assets.pubpub.org/sl9s0xqm/01626781141637.png){#nxmjkyrxucb}

Crédito de imagen: Estado del micrófono mediante el uso de las ondas de
audio

por Adrián Hernández, 2021, Mangosta (`mangosta.org/iii_parametros-2/)`

Para Android, se puede utilizar **AudioManager\*\*\*\*\*, \*\*\*una**
\*\*clase definida por Google para manejar el micrófono en los
dispositivos Android. Además, necesitaremos utilizar la clase Context y
la String AUDIO_SERVICE. Asimismo, podemos comprobar desde la API 11 de
Android (correspondiente a HoneyComb - Android 3.O.x) si el micrófono
está en uso, utilizando la constante definida: MODE_IN_COMMUNICATION /
MODE_IN_CALL.

Los pasos a seguir para crear la estructura serían:

1.- Utilizar getSystemService(`java.lang.String`)

2.- Incluir la clase Context y la String AUDIO_SERVICE:
getSystemService(`Context.AUDIO`\_SERVICE)

3.- Incluir la sentencia creada en el paso 2 en la clase AudioManager:
(AudioManager)`context.getSystemService`(`Context.AUDIO`\_SERVICE).

4.- Actualmente el método getMode asociado a AudioManager nos ofrece 5
resultados:

MODE_NORMAL: No hay llamadas ni acciones establecidas. MODE_RINGTONE:
Hay una petición al micrófono. MODE_CALL_SCREENING: Hay una llamada
conectada pero el audio no está en uso. MODE_IN_CALL: Hay una llamada
telefónica. MODE_IN_COMMUNICATION: Existe alguna aplicación que está
realizando comunicaciones de audio/video o VoIP.

Por tanto si el método nos devuelve un \"MODE_IN_CALL\" o
\"MODE_IN_COMMUNICATION\", tendremos el micrófono activo.

#### 4.9.- Monitorización de parámetros físicos (velocidad, temperatura, batería) con resultado anormal

Para dispositivos Windows, se utilizará Python y la biblioteca psutil,
que nos permite monitorizar y recuperar información sobre el sistema
tales como CPU, RAM, uso de disco, red o batería. Además es
multiplataforma lo que nos permitirá en un futuro integrarla sobre
cualquier sistema operativo.

Se puede generar el *log \*utilizando solo psutil y estableciendo a
partir de qué porcentaje de utilización y durante cuánto tiempo
sostenido. Este porcentaje sería considerado un uso anormal en los
parámetros físicos: velocidad, temperatura y batería. También si
queremos monitorizar en tiempo real lo que ocurre tanto con la CPU como
con la RAM, debemos utilizar la biblioteca Matplotlib para generar
gráficos a partir de datos contenidos en listas o \*arrays*.

![](https://assets.pubpub.org/6iw9zu02/71626781601028.png){#nk0wl16hdia}

Crédito de imagen: Monitorización de parámetros físicos por Adrián
Hernández, 2021, Mangosta (`mangosta.org/iv_parametros-2/)`

#### 4.10.- Gráficos obtenidos tras analizar los parámetros

Los gráficos polares que obtenemos al controlar cada uno de los
parámetros y que le permiten a la Inteligencia Artificial clasificar las
imágenes y posteriormente predecir si se trata de una intrusión o no,
siguen este estilo:

![](https://assets.pubpub.org/5gyxzxzo/51626781828904.png){#n3kqalq252g}

Crédito de imagen: Gráfico polar por Adrián Hernández, 2021, Mangosta
(`mangosta.org/vi_parametros-2/)`

## 5.- Diagramación y sistemática en la implementación del *framework*

Para el diseño, la compilación y la ejecución del conjunto del
*framework* en laboratorio cabe plantearse diferentes posibilidades y el
empleo de lenguajes de programación. Por un lado está el uso propio de
la IA respecto a su entrenamiento y clasificación de imágenes, que
denominaremos sistemática (**S1**), y que en ella se ha utilizado
Python. Por otro lado se plantea el diseño de una BD de almacenamiento y
uso de datos; tanto para dispositivos conocidos ya determinados (si son
o no son positivos en intrusión) y también por determinar, para esta
sistemática (**S2**) se ha utilizado SQL. Y por último se hace preciso
la gestión propia del entorno en su conjunto o *shell*y su ensamblado
general, para esta sistemática (**S3**) se ha utilizado Java. (El
lenguaje SQL resulta singular en la sistemática S2. Para las
sistemáticas S1 y S3 podría usarse un único lenguaje de programación a
elegir entre Python y Java, cada uno con sus ventajas y desventajas
respecto al uso de la IA y respecto a los sistemas operativos objetivos
en dispositivos *smart*).

Inicialmente plantearemos los diagramas de flujo de la lógica a emplear,
general y singular; después adicionaremos algunos elementos
complementarios como el paso a paso del uso de la IA y el diseño de la
base de datos y finalmente plantearemos la codificación esencial de los
distintos lenguajes de programación que intervienen: SQL, Python y Java.

### 5.1.- Diagramación General del *framework*

En este apartado abordaremos el esqueleto del proyecto. Para ello hemos
planteado la diagramación general, dividida en tres partes: la relativa
al usuario, la base de datos y la IA.. El principal objetivo es obtener
una visión global de los procedimientos acometidos, de tal forma que
podamos entender todos y cada uno de ellos.

![](https://assets.pubpub.org/oekkq0wb/21627033917365.svg){#ncdtksqgjez}

Crédito de imagen: Diagrama de flujo general del framework por Diana
Díaz, 2021, Mangosta (`mangosta.org/diagramacion-general-framework/)`

El diagrama general ha sido diseñado con la ayuda de la aplicación de
diagramación de código abierto Drawio. Esta diagramación, en concreto,
contiene otros dos diagramas; la conexión Java con la base de datos y el
uso de `fast.ai`\* + Google Colaboratory, también recogidos en la
leyenda de la parte inferior del diagrama.

En primer lugar, comenzamos con la instalación de la aplicación. Al
acceder a la app, el usuario es preguntado si desea que su dispositivo
sea analizado, en caso de afirmar su respuesta, el proceso continúa, de
no ser así, la aplicación se cierra. Si el proceso continúa, el usuario
debe introducir los datos requeridos: el tipo de dispositivo, la marca y
el modelo. Asimismo, se realiza una conexión en Java con la base de
datos, de manera que esta información sea cotejada y por consecuencia,
permita comprobar si este dispositivo se encuentra entre nuestros
registros y dispone de la información suficiente para realizar el
análisis. Si está registrado, se verifica con el histórico si el
dispositivo está realizando alguna acción sin consentimiento del
usuario. Por el contrario, si no está en la base de datos, se requiere
la autorización del usuario para seguir con el proceso. En caso de que
el usuario no consienta este análisis, la aplicación se cierra
inmediatamente. Además, la inteligencia artificial se emplea en este
punto para comprobar si existe algún riesgo de intrusión. Para llevar a
cabo este proceso, se realiza una captura de 9 parámetros
predeterminados, que asignándoles un modelo preentrenado ResNet34 con
`fast.ai` indica si realmente hay una intrusión. Con el resultado de la
IA, un profesional analizará los datos obtenidos y decidirá si son
coherentes. Si es así, los datos se almacenan en la base de datos y
posteriormente se le comunicaría al usuario, en caso contrario, estos
datos se descartarían.

### 5.2.- Diagramación de Java

En este punto se expone la diagramación Java, designada como sistemática
(**S3**). El objetivo principal de la diagramación de Java es definir,
por un lado, cómo Java accede a la base de datos mediante el *driver*
JDBC y, por otro lado, cómo efectúa una comprobación mediante una
consulta SQL.

![](https://assets.pubpub.org/lk87xwr7/21625581227274.svg){#ncvohei5tqq}

Crédito de imagen: Diagrama de Java

por Kimberly Riveros, 2021, Mangosta
(`mangosta.org/diagramacion-de-java/)`

El *driver* JDBC, por sus siglas en inglés, (Java Database Connectivity)
facilita la conexión entre un sistema gestor de base de datos (SGBD) y
cualquier *software*que trabaje con Java *. \*Además del \*driver* JDBC,
hemos utilizado el sistema gestor MySQL para gestionar los archivos de
la base de datos y la herramienta phpMyAdmin, disponible en XAMPP, para
manejar la administración de MySQL. También, hemos trabajado en el
entorno de Java conjuntamente con Eclipse. Eclipse es un *software*de
entorno de desarrollo integrado para conectar la base de datos con Java
\*. \*Es importante añadir que en el diseño de la diagramación de Java
hemos utilizado como herramienta base el programa `Draw.io`, al igual
que se ha hecho en el resto de diagramaciones (**S1, S2 \*\*y \*\*S3)**.

En el siguiente apartado explicaremos cómo se realiza la conexión entre
la base de datos y Java, tanto en local como en la nube, utilizando la
máquina virtual Eclipse. Una vez instalados todos los aplicativos, se
incluye el *driver* JDBC en nuestro proyecto de Eclipse. A modo de
recomendación, es conveniente tener creada la base de datos desde el
principio para evitar posibles errores de conexión.

![](https://assets.pubpub.org/f6au3o4x/21627034201059.svg){#njx488apnkd}

Crédito de imagen: Conexión a la base de datos - Java por Kimberly
Riveros, 2021, Mangosta
(`mangosta.org/conexion-a-la-base-de-datos-java/)`

Posteriormente definimos en Eclipse el objeto \"connection\" para
establecer la conexión y le indicamos la ruta del servidor junto con la
base de datos a la que queramos acceder. Nosotros hemos llamado a la
base de datos \"seguridad\", tanto en local como en la nube. Además es
necesario especificar el usuario y la contraseña porque de lo contrario,
no sería posible acceder a ella. Por último, iniciamos la búsqueda del
*host* y el puerto en phpMyAdmin para incluirlos en la ruta y así
realizar la conexión desde la nube.

![](https://assets.pubpub.org/ryrcm470/41627034284629.svg){#nx4gt72xaow}

Acceso al código para docentes y estudiantes:
`mangosta.org/conexion-a-la-base-de-datos-java001/`

Cada conexión, en local o en la nube, es independiente entre sí, es
decir, cada una se hace en una clase distinta y lo único diferente que
tiene cada clase en Eclipse es la ruta.

![](https://assets.pubpub.org/5cl6yr1z/11627035540414.svg){#nqzstrx0wk1}

Crédito de imagen: Conexión a la base de datos Java

por Kimberly Riveros, 2021, Mangosta
(`mangosta.org/conexion-a-la-base-de-datos-java002/)`

Finalmente una vez comprobada la conexión de la base de datos, podemos
realizar las consultas que deseemos, escribiendo en lenguaje SQL. Por
ejemplo, ejecutamos una consulta con la ayuda de los objetos,
\"Statement*\"* y\* \"*ResultSet*\".\* Como resultado se muestra por
pantalla la tabla y el registro, entre otras, pues éstas han sido
llamadas desde Eclipse.

Este punto será ampliado en el apartado relacionado con la codificación
Java.

### 5.3.- Diagramación de Python

La diagramación de Python está dividida en dos, por un lado, la que hace
referencia a la conexión de la base de datos con Python y, por otro, la
que alude al uso de la inteligencia artificial en Python.

#### 5.3.1 - Diagramación conexión base de datos con Python

La siguiente diagramación expone la manera de conectar la base de datos
mediante el lenguaje de programación Python, definida como sistemática
(**S1**) en la plataforma Google Colaboratory.

![](https://assets.pubpub.org/a95dde3x/51627035791754.svg){#n0i35yn0gpj}

Crédito de imagen: Diagrama de Python por Diana Díaz, 2021, Mangosta
(`mangosta.org/diagramacion-conexion-base-de-datos-con-python/)`

Las herramientas utilizadas para llevar a cabo este procedimiento son:
Google Colaboratory, una aplicación de Google para crear entornos de
máquinas virtuales basado en Jupyter Notebook . Jupyter, un archivo de
extensión IPYNB que combina una multitud de celdas con código o texto
Markdown . El conector MySQL Connector/Python está escrito en Python y
nos permite conectarnos a una base de datos externa, sin embargo, es
necesario un medio de interacción. Por esta razón, hacemos uso del
paquete PyMySQL.

El siguiente paso es conectar la base de datos con Python. En primer
lugar, accedemos a Google Colab y creamos un cuaderno nuevo, el cual, de
forma automática, se convierte en un cuaderno Jupyter. Una vez abierto,
instalamos el conector MySQL Connector/Python con \"pip\", un sistema de
gestión de paquetes escrito en Python. Una vez instalado, lo importamos.
De igual forma, el paquete PyMySQL debe ser instalado. Llegados a este
punto, si no nos ha dado ningún error durante el proceso, ya tenemos
todo preparado para la conexión a la base de datos desde nuestro
cuaderno de Google Colab. Para conectarnos creamos una variable en la
que mediante el paquete PyMySQL, alojamos la información de la ruta:
nombre, usuario, contraseña y nombre de la base de datos a la que nos
queremos conectar. Convertimos la variable en un cursor, ya que mediante
la misma, vamos a realizar una operación de búsqueda en la base de
datos. Teniendo el cursor podemos ejecutar sobre él la consulta que
queramos y, posteriormente, imprimirla por pantalla con el resultado
obtenido. Una vez ejecutadas todas las consultas requeridas, podemos
cerrar la conexión a la base de datos.

Este punto será ampliado en el apartado relacionado con la codificación
Python.

#### 5.3.2 - Diagramación uso de Inteligencia Artificial en Python

La diagramación que se muestra a continuación maqueta la forma de
detectar intrusiones, denominada sistemática (**S3**). La finalidad de
la diagramación del uso de la Inteligencia Artificial en Python es
estudiar las intrusiones a partir de imágenes focalizadas en el IDS. En
nuestro caso, hemos elegido imágenes de gatos y tigres. Asimismo, en el
siguiente apartado están detallados los pasos a seguir para detectar
intrusiones utilizando ResNet34, cuadernos Jupyter y Google
Colaboratory.

![](https://assets.pubpub.org/6myif3va/71625903421863.svg){#nflv7hu1slp}

Crédito de imagen: Diagramación uso de la inteligencia artificial en
Python

por Adrián Hernández, 2021, Mangosta
(`mangosta.org/diagramacion-uso-de-la-inteligencia-artificial-en-python/)`

Para llevar a cabo este procedimiento, hemos utilizado Drawio para hacer
el diagrama, Google Drive y Google Colaboratory, que incluye el lenguaje
de programación Python. Además de estas herramientas, trabajamos con
`fast.ai`, esto es, una biblioteca de aprendizaje profundo que permite
la carga de modelos preentrenados como ResNet34 para una posterior
clasificación de las imágenes. Por tanto, no es necesario descargar
ningún tipo de *software*, pues todo estos procesos los llevaremos a
cabo en la nube.

Accedemos a Google Drive para crear tres carpetas: \"*train*\",
*\"validation*\" y *\"test\"*. Dentro de cada una de ellas, creamos dos
subcarpetas llamadas \"gato\" y \"tigre\" en las que se agruparán las
imágenes, dependiendo del grupo al que pertenezcan.

![](https://assets.pubpub.org/zjah810k/11626516224484.svg){#nv1kkdr34mr}

Crédito imagen: Almacenamiento imágenes de gatos por Kimberly Riveros,
2021, Mangosta (`mangosta.org/gatos/)`

![](https://assets.pubpub.org/3ovdns93/41626516567773.svg){#n5tq6ltzbmo}

Crédito imagen: Almacenamiento imágenes de gatos por Kimberly Riveros,
2021, Mangosta (`mangosta.org/tigres/)`

Posteriormente accedemos a Google Colaboratory y en la primera celda,
indicamos la ruta en la que estará ubicado nuestro *dataset \*(conjunto
de imágenes). De esta forma, podremos entrenar el modelo con esas
imágenes. Por consiguiente, creamos el modelo preentrenado con la
arquitectura ResNet34. Cuando hablamos de modelo preentrenado, nos
referimos a un modelo que permite que otros modelos obtengan resultados
avanzados sin necesidad de disponer de grandes cantidades de cómputo,
tiempo y paciencia. Esta es una forma de encontrar la técnica de
entrenamiento adecuada. Por otro lado, si queremos usar el modelo en un
futuro, no tenemos que volver a entrenarlo. ResNet34, por sus siglas en
inglés (*residual neural network)*, es un modelo preentrenado de
ImageNet. ImageNet es un \*software* de reconocimiento de objetivos
visuales. Tanto ResNet34 como ImageNet nos ayudan a mejorar el
rendimiento y optimizar los resultados en lo que a la detección de
intrusiones se refiere.

El siguiente paso es realizar un ciclo de entrenamiento con un número
determinado de épocas para entrenarlo (véase en el apartado referido al
proceso de detectar intrusiones con Inteligencia Artificial), y así la
Inteligencia Artificial pueda estudiar todo el conjunto de imágenes.
Para interpretar los resultados creamos una matriz de confusión que nos
ayudará a evaluar el rendimiento de nuestro modelo de clasificación de
imágenes. Esta matriz compara los valores reales con los predichos y
así, podremos ver cómo está funcionando nuestro modelo en la
identificación de imágenes. De hecho, en la siguiente representación
gráfica podemos comprobar que la inteligencia artificial ha fallado
únicamente en una imagen, pues clasifica esa imagen como un tigre cuando
en realidad es un gato.

![](https://assets.pubpub.org/u66wz9m9/51626516802440.svg){#n4ptsoc3sef}

Crédito de imagen: Respuesta Inteligencia Artificial por Kimberly
Riveros, 2021, Mangosta
(`mangosta.org/i_respuesta-inteligencia-artificial/)`

![](https://assets.pubpub.org/l335zn31/41626516826424.svg){#no52lkrsv6e}

Crédito de imagen: Respuesta Inteligencia Artificial (matriz de
confusión) por Kimberly Riveros, 2021, Mangosta
(`mangosta.org/ii_respuesta-inteligencia-artificial/)`

## 6.- Paso a paso en el proceso de detectar intrusiones con Inteligencia Artificial: ResNet34, cuadernos Jupyter y Google Colaboratory

Google Colab es un entorno de trabajo que nos permite ejecutar cuadernos
Jupyter en el navegador. Entre sus ventajas destacamos la necesidad de
no configurarlo previamente y la obtención de acceso gratuito a GPU\'s\*
*(Tesla T4 o Tesla K80). Por lo tanto, ofrece la posibilidad de entrenar
y utilizar la inteligencia artificial sin emplear recursos \*hardware*
propios y/o pagar una suscripción para su uso.

Un cuaderno Jupyter es un documento que permite ejecutar celdas de
código en vivo, texto plano, ecuaciones, etc. La estructura que siguen
estos cuadernos es una lista ordenada de entrada/salida. El nombre
Jupyter proviene de: **Julia + \*\*\*\*\*Python \*\*\*\*\*+ R**, que son
los tres lenguajes de programación presentes en los cuadernos Jupyter.
Los dos componentes principales de Jupyter Notebook son un conjunto de
núcleos (*Interpreter*) y el *Dashboard*. De igual forma, cambiando el
núcleo del cuaderno podemos ejecutar otros lenguajes desde Google Colab
como podría ser Java.

Una vez que tenemos el conocimiento suficiente acerca de los componentes
que vamos a utilizar para dar forma a nuestra inteligencia artificial,
es el momento de explicar el procedimiento sobre cómo lo vamos a llevar
a cabo.

El primer paso es preparar el entorno de Google Colab, ya que por
defecto no vamos a poder acceder sin activarlo. Para ello, desde nuestro
Google Drive hacemos clic en Google Colaboratory.

![](https://assets.pubpub.org/hqbijbo1/21625590448284.png){#nv0hjwkizl0}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial: ResNet34, cuadernos Jupyter y Google Colaboratory por Adrián
Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial/)`

O bien, si no nos aparece la opción, hacemos clic en la siguiente URL:
\*\*`colab.research.google.com/**` (URL: `colab.research.google.com/)`

Una vez estemos dentro de Google Colab, tenemos los cuadernos
predeterminados por Google para iniciarnos en Python desde la nube. Sin
embargo, también podemos crear un cuaderno en blanco para empezar a
realizar las pruebas pertinentes. Para realizar esta acción, hacemos\*
\*clic en la opción: \"Nuevo Cuaderno\".

![](https://assets.pubpub.org/u5hqlvst/41625590474668.png){#n5do4ruzlmm}

Image credit: Process of detecting intrusions with Artificial
Intelligence: ResNet34, Jupyter Notebooks and Google Colaboratory by
Adrián Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial001/)`

Ahora, el nuevo cuaderno creado tiene la siguiente estructura:

![](https://assets.pubpub.org/thonthtt/11625590503302.png){#nz622tn7mbh}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial: ResNet34, cuadernos Jupyter y Google Colaboratory por Adrián
Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial002/)`

La celda que nos aparece inicialmente nos permite escribir código en
Python y ejecutarlo directamente, utilizando el botón \"*Play*\",
situado en la misma celda.

![](https://assets.pubpub.org/o6iodv75/41625672705476.png){#nqqy1gnsgst}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial: ResNet34, cuadernos Jupyter y Google Colaboratory por Adrián
Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial003/)`

Por defecto, Google Colaboratory no viene configurado para utilizar
GPU\'s, así que debemos prepararlo. Para ello, es necesario ir a la
opción de \"Entorno de ejecución\", \"Cambiar tipo de entorno de
ejecución\".

![](https://assets.pubpub.org/grxpwdtx/41625590526112.png){#nzgemlhtify}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial: ResNet34, cuadernos Jupyter y Google Colaboratory por Adrián
Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial004/)`

Una vez situados en la pestaña, hacer clic en el desplegable con la
palabra \"*none\" \*y reemplazarlo por \"GPU*\"\*.

![](https://assets.pubpub.org/jkvodkca/51625590541250.png){#nyjx5tmdg0l}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial: ResNet34, cuadernos Jupyter y Google Colaboratory por Adrián
Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial005/)`

En este momento, Google asignará una GPU para nuestro cuaderno Jupyter,
la cual podremos comprobar, utilizando la siguiente instrucción en una
celda de código:

![](https://assets.pubpub.org/smlajwys/71625672875145.png){#nr5zimvv0eq}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial: ResNet34, cuadernos Jupyter y Google Colaboratory por Adrián
Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial006/)`

Como se puede apreciar en la imagen, nos asignan una Tesla T4, aunque
podría haber sido una Tesla K80, ya que son las dos GPU\'s que Google\*
\*ofrece de manera gratuita. Ahora, ya tenemos configurado el entorno de
Google Colab para trabajar sobre él con inteligencia artificial, aunque
para entrenar nuestro modelo necesitamos crear la estructura de carpetas
que vamos a utilizar. Esto se realizará desde Google Drive para reducir
su complejidad, pero podría realizarse en un equipo local, configurando
el entorno Colab para ello.

Para realizar las primeras pruebas, utilizamos imágenes de gatos y
tigres, persiguiendo que la inteligencia artificial sea capaz, dada una
imagen que nunca ha visto, de determinar si se trata de una imagen de un
tigre o de un gato. La estructura inicial consta de tres carpetas, a las
que podemos nombrar como queramos, pero que por comodidad, se recomienda
atribuirles la siguiente nomenclatura:

![](https://assets.pubpub.org/jlz0kdu3/41625738992031.png){#nfdwb5jmnde}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial: ResNet34, cuadernos Jupyter y Google Colaboratory por Adrián
Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial007/)`

Una carpeta *Test, \*atribuida a las imágenes que la inteligencia
artificial nunca ha visto y servirá para comprobar que la IA es capaz de
clasificar correctamente las representaciones gráficas. Otra carpeta
\*Train*, con fines de entrenar nuestro modelo. Y, en este caso, sí que
habrá una diferenciación entre tigres y gatos, y la inteligencia
artificial sabrá en cada caso si la imagen que está visualizando es un
gato o un tigre. Una carpeta llamada *Validation \*que nos servirá para
validar nuestro modelo. También habrá una diferenciación dentro de la
misma. Esta carpeta nos permitirá conocer cómo clasificaría la
inteligencia artificial la imagen y lo que es realmente dicha imagen.
Dentro de las carpetas \*Train \*y \*Validation*, tendremos la siguiente
estructura:

![](https://assets.pubpub.org/pi8k5t2y/71625739010200.png){#nh4lp5oa5yf}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial: ResNet34, cuadernos Jupyter y Google Colaboratory por Adrián
Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial008/)`

Y dentro de cada carpeta, imágenes correspondientes a gatos o a tigres.
Las imágenes de cada carpeta deben ser distintas, además, cuanto mayor
número de figuras utilicemos, mayor precisión tendrá la inteligencia
artificial para clasificar otras representaciones gráficas.

Una vez creada la estructura de carpetas y preparado el entorno de
Google Colab, empezamos a trabajar con la inteligencia artificial. Para
ello, ejecutamos el código referente a conectar nuestro entorno de
Google Colab con Google Drive, donde se encuentran las carpetas que
servirán a la IA desde el cuaderno creado previamente.

![](https://assets.pubpub.org/wgns4eua/61625673067959.png){#npbl21vkrxp}

Acceso al código para docentes y estudiantes:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificialc001/`

Una vez asociados los dos entornos, definimos qué rutas se van a
utilizar, en este caso root_dir y base_dir.

![](https://assets.pubpub.org/pl2fhkbm/31625673138473.png){#n81wsowk1h1}

Acceso al código para docentes y estudiantes:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificialc002/`

De igual manera, definimos el *path*, es decir, la ruta de nuestro
directorio principal a utilizar.

![](https://assets.pubpub.org/txbrb2bk/51625673177577.png){#nmc6dcmfu60}

Acceso al código para docentes y estudiantes:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificialc003/`

Para realizar el entrenamiento del modelo de inteligencia artificial es
necesario importar fastai. Restnet34 debe su nombre a \"*residual*
*network*\" y se refiere a una red residual que fue introducida por
Microsoft en 2015. Una vez que conocemos `fast.ai`, importamos la
biblioteca:

![](https://assets.pubpub.org/is4al43k/11625673261162.png){#ny5c2zbuxo9}

Acceso al código para docentes y alumnos:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificialc004/`

Estas tres instrucciones sirven para importar las bibliotecas fastai y
fastai vision completas, además del \"**error_rate**\" de **fastai
metrics**. \"Error_rate\" sirve para determinar el grado de error en
nuestro modelo entrenado, es decir, para saber si según nuestros
criterios, debemos mejorar el modelo o por el contrario, el error
obtenido es asumible y podemos utilizar el modelo.

Una vez importadas las bibliotecas necesarias, comprobamos que nuestra
configuración del cuaderno sea correcta. Para ello, imprimimos en
pantalla una imagen aleatoria utilizando el *path \*que hemos definido
previamente y con las siguientes instrucciones: ****open_image****(y la
ruta de la imagen utilizando la variable \*path*) e `img.show`*().*

![](https://assets.pubpub.org/az5lhdcj/11625673319553.png){#n6h79qd8fv2}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial: ResNet34, cuadernos Jupyter y Google Colaboratory por Adrián
Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial013/)`
(.) Acceso al código para docentes y alumnos:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificialc005/`

Cuando comprobamos que la imagen se muestra correctamente, creamos los
datos (muestra de imágenes) que nuestro modelo va a utilizar. Para
empezar, vamos a definir el batch size o tamaño de batch, que nos
permite indicar el número de imágenes que transitamos al mismo tiempo a
la memoria. Como medida preventiva, es recomendable no utilizar un batch
size muy alto o podríamos causar un error de memoria en la tarjeta
gráfica y por consiguiente, el detenimiento del proceso. Para definir el
tamaño de batch\*, \*escribimos el siguiente código:

![](https://assets.pubpub.org/aorskhqt/01625673401540.png){#ng13noix6vw}

Acceso al código para docentes y alumnos:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificialc006/`

En este caso el número 6, indica la cantidad de imágenes transitadas a
la memoria al mismo tiempo. Una vez hemos asignado el batch size,
asignamos los datos que vamos a utilizar, en este caso imágenes. Para
ello, utilizamos la siguiente celda de código:

![](https://assets.pubpub.org/hetwxijb/21625673441651.png){#nej5g8t2of7}

Acceso al código para docentes y alumnos:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificialc007/`

En esta celda definimos la variable *data*, partiendo de la estructura
de carpetas que hemos creado previamente (*Train, Validation \*y
\*Test*) e indicando que esas son las carpetas que se tendrían que
utilizar para sus variables predeterminadas: *train, valid \*y \*test*.

Ya definido el conjunto de datos que vamos a utilizar, inicializamos
nuestro modelo. Antes de eso, creamos el modelo y cargamos
Resnet34,\*\*\* \*\*\*utilizando el siguiente código:

![](https://assets.pubpub.org/0g00s3vf/11625673495268.png){#ndordjfo445}

Acceso al código para docentes y alumnos:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificialc008/`

Utilizamos cnn_learner y le indicamos el conjunto de datos a utilizar
(*data*), el modelo que vamos a utilizar (`models.resnet`**34**) y
además, queremos conocer el \*\*error_rate importado anteriormente de
\*\*`fastai.metrics`\*\*\*\*\*. \*\*\*Esta línea de código, por tanto,
nos sirve para crear y cargar nuestro modelo, por lo que el siguiente
paso es entrenarlo. Para eso utilizamos:

![](https://assets.pubpub.org/og8h3rlo/01625673657619.png){#nw63yqhc4dm}

Acceso al código para docentes y alumnos:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificialc009/`

El número entre paréntesis indica el número de \"***epoch***\" que vamos
a utilizar para el entrenamiento. Pero, ¿qué es un *epoch*? Llamamos
\*epoch \*o época al proceso de pasar todos los datos por la
Inteligencia Artificial. En nuestro caso, todas las imágenes de la
carpeta llamada entrenamiento pasan por la inteligencia artificial. Al
realizar un entrenamiento con 6 épocas, todas las imágenes de la carpeta
\*Train \*pasarán 6 veces por la IA.

![](https://assets.pubpub.org/2kbzucnu/21625673719682.png){#nwevksgmtzp}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial por Adrián Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial018/)`
(.) Acceso al código para docentes y alumnos:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial-resnet34-cuadernos-jupyter-y-google-colaboratoryc009bis/`

Al finalizar el entrenamiento obtendremos el error_rate final de nuestro
modelo de entrenamiento. En el caso de la imagen superior sería 0.003165
o del 0.31%, aproximadamente. Otra de las cuestiones es conocer para qué
sirve el error_rate\*.\*\*\* \*\*Ofrece información sobre lo preciso que
es nuestro modelo y nos permite decidir si realizar un entrenamiento con
más épocas o utilizar el modelo. Si no estamos seguros de cuánta
precisión significa un error de 0.31%, podemos imprimir una matriz de
confusión donde se ve más claro utilizando:

![](https://assets.pubpub.org/6hmgt6c1/21625673809556.png){#nonpscvolge}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial: ResNet34, cuadernos Jupyter y Google Colaboratory por Adrián
Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial-019/)`

En esta matriz de confusión se comparan el eje de abscisas (eje X), las
predicciones que realiza la Inteligencia Artificial (gatos o tigres) y
en el eje de ordenadas (eje Y), lo que realmente es la imagen. En este
caso, la IA acierta en 331 imágenes de gatos y 299 imágenes de tigres,
fallando en 2 imágenes en las que la IA predice que son gatos cuando
realmente son tigres. Por lo tanto, el error que partiría de esta matriz
de confusión sería de → 2/632 = 0.316% %.

Una vez consideramos que el error es asumible, procedemos a guardar el
modelo para poder utilizarlo más adelante. Este procedimiento se realiza
con la siguiente celda de código:

![](https://assets.pubpub.org/yvr8o8cj/11625673879783.png){#ndqv01ers8o}

Acceso al código para docentes y alumnos:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificialc010/`

Entre paréntesis, indicamos la ruta y el nombre con el que vamos a
guardar nuestro modelo. Una vez que el modelo está guardado, podemos
probar su funcionamiento con imágenes que nunca ha visto la IA. Para
este proceso, utilizamos una nueva celda de código que contiene la
siguiente información:

![](https://assets.pubpub.org/7awo0nbc/31625673925603.png){#n1wxf3r5b7x}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial por Adrián Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial020/)`
(URL:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial020/)`
(.) Acceso al código para docentes y alumnos:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificialc011/`

En este punto volvemos a utilizar el modelo almacenado en *learn \*y le
pasamos la imagen de la carpeta \*Test*, en este caso la 520. Podemos
imprimir la imagen dentro del propio Google Colab para verificar que
realmente se cumple lo que la IA va a predecir en el siguiente paso.
Para imprimir la imagen se utiliza \*\*`img.show`(). \*\*Para pedirle a
la inteligencia artificial que analice la imagen que hemos cargado,
tendremos que usar la siguiente línea de código:

![](https://assets.pubpub.org/z2gf5v1t/31625673976123.png){#n7ujxhj6jxe}

Crédito imagen: Proceso de detectar intrusiones con Inteligencia
Artificial por Adrián Hernández, 2021, Mangosta
(`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificial021/)`
(.) Acceso al código para docentes y alumnos:
`mangosta.org/proceso-de-detectar-intrusiones-con-inteligencia-artificialc012/`

En este caso, volvemos a realizar una llamada a nuestro modelo con
\*learn \*y utilizamos predict(img) para utilizar el modelo entrenado.
El resultado en este caso nos lo ofrece como tensor(0) para los gatos y
emplea tensor(1) para los tigres. Además entre corchetes nos da la
probabilidad de que en este caso sea un tigre (99.9%).

## 7.- Creación y estructura de la base de datos

Se ha creado una base de datos atendiendo a la necesidad de recoger los
datos que ofrecen los *smartwatches* u otros dispositivos afines para
comprobar si todos estos datos son autorizados y conocidos por el
usuario o por el contrario, se trata de un uso no autorizado. Para ello,
tenemos en cuenta dos factores: los datos que se necesitan para poder
acceder a la información del usuario y los datos que entrañan un riesgo
por acceder sin consentimiento. En este sentido, centramos el foco de
atención en los datos que recogen los\* smartwatches\*, ofreciendo una
tabla adicional (TSalud). 

La herramienta utilizada, al igual que en los diagramas anteriores, es
Drawio. También hemos usado el lenguaje SQL así como MySQL y
PhpmyAdmin. 

Dos de los pilares fundamentales de esta base de datos son la
importancia de los booleanos y la lógica trivalente, sobre la que
hablaremos más adelante. ¿Pero para qué valen los campos booleanos? ¿Por
qué hemos decidido emplearlos en nuestra base de datos? fundamentalmente
para encontrar información de calidad en el menor tiempo posible y
realizar búsquedas combinadas de varios términos.

Para clarificar posibles dudas sobre la base de datos, explicamos el
diseño y la estructuración de las 5 tablas. La primera tabla y principal
es \"TDispositivo\". Esta tabla se compone de 3 campos que recogen el ID
(número identificativo y que se encuentra en todas la tablas como hilo
conductor), el nombre de la marca (varchar) y el modelo (varchar). La
segunda tabla \"TTipoDispositivo\" dispone de 6 campos: ID, y otros 5
campos booleanos (*smartwatch*, PC portátil, PC sobremesa, *tablet* y
*smartphone*) que responderán con verdadero o falso en función del tipo
de dispositivo del que se trate. La tercera tabla
\"TAccesoNoAutorizado\" se compone de 4 campos: ID y otros 3 booleanos
(cámara, micrófono y GPS). En esta tabla se pretende conocer si el
dispositivo tiene acceso a alguno de estos 3. La cuarta tabla \"TSalud\"
recoge un total de 8 campos: el ID y siete booleanos, (frecuencia
cardíaca, sueño, registro automático de ejercicio, estrés, nivel de
oxígeno, ciclo menstrual y pasos). Se pretende que esta tabla nos
informe sobre la información que recoge el dispositivo. La quinta y
última tabla es \"TDeteccion\" y tiene tan sólo dos campos: ID y un
booleano (positivo). Esta tabla es de gran importancia porque en ella se
almacenará, tras comprobar el resto de datos de las otras tablas, si el
dispositivo es positivo o no, es decir, si tiene o no una intrusión.

**Diagrama entidad - relación**

![](https://assets.pubpub.org/jczr0eqd/31625578022603.png){#nt84ejambdu}

Crédito imagen: Diagrama entidad-relación por Diana Díaz, 2021, Mangosta
(`mangosta.org/diagrama-entidad-relacion/)`

Como podemos apreciar en la imagen este diagrama se compone de tres
elementos fundamentales:

● Entidad, representado como rectángulos que muestran los nombres de
nuestras tablas. Por ejemplo: TAccesoNoAutorizado.

● Atributos, con forma ovalada, definen las características de la
entidad. Siguiendo el ejemplo, aquí las vemos representados por nID,
lCamara, etc).

● Relaciones, con forma romboide, muestran las conexiones entre unas
entidades y otras. En este caso, la conexión entre una entidad y otra
sería: TDispositivo tiene TAccesoNoAutorizado). 

**Lógica trivalente**

Para el diseño de la base de datos nos hemos centrado en la velocidad de
las consultas, ya que es fundamental para un buen desarrollo del
proyecto. Por este motivo se utilizan campos booleanos. 

La lógica bivalente distingue dos valores: verdadero o falso,
representados por 1 y 0 respectivamente . En este caso, iremos más allá
y aplicaremos la lógica ternaria o trivalente, es decir, un sistema
lógico en el que se presentan 3 valores: verdadero, falso o
indeterminado. El tercer valor se interpreta como la posibilidad de que
algo no sea ni verdadero ni falso, sino indefinido. En este contexto, el
indicador categorizado como indeterminado, pues muestra ausencia de
valor, se representa así: .

**Modelo Físico**

Una vez visualizado el diseño y analizada la estructura de las tablas,
estudiamos la base de datos desde el modelo físico, en el que observamos
que la primary *key* de todas las tablas es nID.

![](https://assets.pubpub.org/onl3fq5t/61625578104797.png){#nq4v5td0o6a}

Crédito imagen: Modelo físico por Diana Díaz, 2021, Mangosta
(`mangosta.org/modelo-fisico-2/)`

**Uso de SQL y su sistema de gestión de base de datos**

SQL2 (\*Structured Query Language \*o lenguaje de consulta estructurada)
pasó a ser el estándar del Instituto Nacional Estadounidense de
Estándares (ANSI) en 1986 y de la Organización Internacional de
Normalización (ISO) en 1987. Ha adquirido tanta popularidad por ser un
lenguaje no procedimental, es decir, especifica qué quiere pero no cómo
ni dónde conseguirlo y es relacionalmente completo, pues permite la
realización de cualquier consulta de datos. 

Ahora que ya sabemos cómo funciona SQL, podemos hablar del sistema de
gestión de base de datos (SGBD) que hemos utilizado. Para el desarrollo
de esta base de datos hemos decido usar en primera instancia MySQL. Se
trata de un SGBD relacional de código abierto. Se entiende por código
abierto (*open source*) a un modelo de desarrollo de *software,* basado
en la colaboración abierta que se enfoca más en los beneficios prácticos
como es el acceso al código fuente, lo que lo hace accesible y práctico.
Por su extendido uso, es una opción fiable y estandarizada. Por otra
parte, SQLite\* es de igual manera un motor de base de datos con SQL
integrado, pero su principal ventaja es que no necesita un servidor para
funcionar. Por este motivo, resulta bastante atractivo para trabajar con
aplicaciones como es nuestro caso. Otra de las ventajas es el espacio
que ocupa en el disco duro (\< 500kb) y lo que hace factible que la base
de datos sea gestionable dentro del propio dispositivo en lugar de en
nuestro servidor. 

Pero, ¿qué lo hace destacar frente a otras opciones como SQLite?  La
base de datos ha sido diseñada en su mayoría con campos *booleanos*. Sin
embargo, SQLite no dispone de esta opción. Entre sus clases de
almacenamiento la única opción posible sería la conversión de
*booleanos* a *integer*, ralentizando la base de datos. Asimismo se
trata de un sistema orientado a funcionar con poco tráfico, por lo que
si introducimos una gran cantidad de datos, quizás su sistema no fuera
todo lo eficiente que necesitaríamos. Por lo que consideramos mejor
continuar con el uso de MySQL*,* así como de nuestro servidor. 

**Representación de los de datos**

Para abordar este tema vamos a hablar sobre Ckan y Dkan. Se trata de
herramientas para la gestión de datos dentro de la *web*. Su éxito se
debe a que ambas son plataformas de código abierto, libre y gratuito. La
principal diferencia es que DKAN es una versión de CKAN desarrollada en
Drupal.

![](https://assets.pubpub.org/zec01diq/11625578194737.png){#nayn8tth0oo}

Crédito de imagen: Instantánea de Dkan, 2021, Autoría de National Health
Countil: `getdkan.org/`

Pero, ¿qué es drupal? Drupal es un *software* de gestión de contenidos
(CMS), configurable que permite múltiples servicios como publicación de
encuestas, foros, artículos e imágenes. De igual forma es un sistema
dinámico que permite almacenar los datos en una base de datos y
editarlos en un entorno web.

![](https://assets.pubpub.org/jnr1rvsi/51625578254170.png){#n28k44xrw6r}

Crédito de imagen: Instantánea de Drupal, 2021, Autoría de Drupal:
`www.drupal.org/`

Ahora que hemos visto que ambos son similares pero DKAN es una versión
*a priori \*superior, consideramos más interesante el uso de esta
plataforma. Es necesario destacar que otro factor a tener en cuenta es
que CKAN tiene un consumo muy exigente de \*hardware*, así como una
gestión ineficiente de seguridad tanto en usuarios como en recursos.
Consideramos importante el uso de una de estas plataformas para poder
ofrecer al usuario una perspectiva más clara de los datos y una
transparencia absoluta. Por esta misma razón, es una plataforma usada
por muchos gobiernos, como el de Australia o el de Canadá, entre otros,
así como instituciones sin ánimo de lucro.

## 8.- Codificación básica del *framework*en SQL, Python y Java

### 8.1.- Codificación SQL

Para crear la base de datos hemos utilizado el código SQL. Como
apreciamos, necesitamos usar UTF-8, un formato de codificación de
caracteres Unicode e *ISO* 10646. El motivo de su uso es para detectar
si hay palabras registradas en la base de datos que contengan caracteres
especiales como el **uso de la Ñ,** por ejemplo en la palabra **sueño**.
Por otra parte la tabla TDispositivo (la última en el código) es la que
presenta las claves foráneas y así ser el punto de unión del resto de
tablas.

![](https://assets.pubpub.org/zj4ucxwz/51627036140694.jpeg){#nlek33s3bcg}

Crédito de imagen: Codificación SQL por Diana Díaz, 2021, Mangosta
(`mangosta.org/iii_codificacion-sql/)` (.) Acceso al código para
docentes y alumnos: `mangosta.org/codificacion-sql001/`

![](https://assets.pubpub.org/pdv53nz3/61627036795988.jpeg){#niaearti2w8}

Crédito de imagen: Codificación SQL por Diana Díaz, 2021, Mangosta
(`mangosta.org/iv_codificacion-sql/)` (.) Acceso al código para docentes
y alumnos: `mangosta.org/codificacion-sql002/`

### 8.2.- Codificación Java

En este apartado se añade el código Java con el que hemos accedido a la
base de datos llamada \"seguridad\", así como el resultado de hacer una
consulta en lenguaje SQL. En primer lugar, importamos la librería
`java.sql` para usar la base de datos desde Eclipse:

![](https://assets.pubpub.org/owyoi87v/01627037225052.svg){#nm6iklxfolz}

Crédito de imagen: Codificación Java por Kimberly Riveros, 2021,
Mangosta (`mangosta.org/viii_codificacion-java/)` (.) Acceso al código
para docentes y estudiantes: `mangosta.org/codificacion-java/`

Más adelante, es necesario asegurarnos de que el \*driver \*JDBC esté
debidamente conectado. A continuación, establecemos la conexión con la
base de datos por medio de estas instrucciones:

![](https://assets.pubpub.org/p7r3aold/31627037492712.svg){#ntsfd0ctpob}

Crédito de imagen: Codificación Java por Kimberly Riveros, 2021,
Mangosta (`mangosta.org/ix_codificacion-java/)` (.) Acceso al código
para docentes y estudiantes: `mangosta.org/codificacion-java001/`

La clase DriverManager tiene un método \*\*String \*\*que contiene la
ruta de la base de datos. Es importante recordar que para ello hemos
utilizado el conector JDBC para MySQL que instalamos en el apartado
referente a la diagramación de Java. Adicionalmente, lo hemos probado en
el servidor de base de datos de nuestro ordenador (*localhost*) y en la
nube, cambiando la ruta de la base de datos.

![](https://assets.pubpub.org/cshm9cw3/31627037871710.svg){#nqwdz2x5eaq}

Crédito de imagen: Codificación Java por Kimberly Riveros, 2021,
Mangosta (`mangosta.org/x_codificacion-java/)` (.) Acceso al código para
docentes y estudiantes: `mangosta.org/codificacion-java-002/`

Por defecto, el servidor MySQL abre el puerto 3306. Es necesario
conectarse a este puerto si queremos consultar la base de datos que
hemos creado. Por consiguiente, creamos el objeto \"Statement\". Para
ello, seguimos el paso mostrado en la siguiente figura:

![](https://assets.pubpub.org/htuxv3y7/21627038019469.svg){#nphfb4oecow}

Crédito de imagen: Codificación Java por Kimberly Riveros, 2021,
Mangosta (`mangosta.org/xi_codificacion-java/)` (.) Acceso al código
para docentes y estudiantes: `mangosta.org/codificacion-java-003/`

Asimismo, el objeto \"Resulset\" nos permite obtener los resultados de
la consulta. En nuestro caso, nos mostrará todos los registros de la
tabla demandada, es decir, TDispositivo:

![](https://assets.pubpub.org/muv0jzkx/21627038200555.svg){#n6buo5w8tnm}

Crédito de imagen: Codificación Java por Kimberly Riveros, 2021,
Mangosta (`mangosta.org/xii_codificacion-java/)` (.) Acceso al código
para docentes y estudiantes: `mangosta.org/codificacion-java-004/`

Ahora, es necesario utilizar el bucle \"while\" y la variable
\"miResultset\" para obtener todos los resultados de nuestra consulta.

![](https://assets.pubpub.org/0bzjn5x0/01627038401615.svg){#n0qm3gvla88}

Crédito de imagen: Codificación Java por Kimberly Riveros, 2021,
Mangosta (`mangosta.org/xiii_codificacion-java/)` (.) Acceso al código
para docentes y estudiantes: `mangosta.org/codificacion-java-005/`

En caso de no establecer la conexión de la base de datos en eclipse, al
ejecutar se mostrará por pantalla: \"No funciona\". Realizados todos los
pasos, finalmente obtenemos la tabla con los registros que hemos
consultado:

![](https://assets.pubpub.org/3k3p56y3/61627038571730.svg){#ndf2npzm6q1}

Crédito de imagen: Codificación Java por Kimberly Riveros, 2021,
Mangosta (`mangosta.org/codificacion-java-2/)`

### 8.3.- Codificación Python

En este apartado se añade el código Python que hemos utilizado para
acceder a la base de datos llamada \"seguridad\", así como el resultado
de hacer una consulta en lenguaje SQL. En primer lugar, debemos instalar
el conector mysql-connector-python, un *driver* para comunicarse con
servidores MySQL.

![](https://assets.pubpub.org/wh9bwq66/71627038784686.jpeg){#nc8nqxe8qoa}

Crédito de imagen: Codificación Python por Kimberly Riveros y Diana
Díaz, 2021, Mangosta (`mangosta.org/i_codificacion-python/)` (.) Acceso
al código para docentes y alumnos:
(`mangosta.org/ii_codificacion-python/)`

Seguidamente, realizamos el mismo proceso con PyMySQL, otro paquete para
la interacción con bases de datos MySQL. Y, una vez instalado, lo
importamos.

![](https://assets.pubpub.org/9rg2zmi8/41627038978912.jpeg){#nicwxxux7gd}

Crédito de imagen: Codificación Python por Kimberly Riveros y Diana
Díaz, 2021, Mangosta (`mangosta.org/iii_codificacion-python/)` (.)
Acceso al código para docentes y alumnos:
(`mangosta.org/iv_codificacion-python/)`

![](https://assets.pubpub.org/uh3sh5vw/41627039441931.jpeg){#n2rnqcemitl}

Crédito de imagen: Codificación Python por Kimberly Riveros y Diana
Díaz, 2021, Mangosta (`mangosta.org/x_codificacion-python-2/)` (.)
Acceso al código para docentes y alumnos:
(`mangosta.org/iv_codificacion-python/)`

Teniendo ya los paquetes necesarios, nos conectamos a la base de datos
\"seguridad\". Para ello, creamos una variable llamada \"miConexión\" y
mediante PyMySQL, almacenamos la ruta de la base de datos, el usuario,
la contraseña y el nombre de la base de datos. En definitiva, una serie
de información, necesaria para establecer la conexión con la base de
datos.

![](https://assets.pubpub.org/o8okcecj/21627111637287.jpeg){#nebel9psl14}

Crédito de imagen: Codificación Python por Kimberly Riveros y Diana
Díaz, 2021, Mangosta (`mangosta.org/xi_codificacion-python/)` (.) Acceso
al código para docentes y alumnos:
(`mangosta.org/v_codificacion-python/)`

El siguiente paso es convertir esta variable en un cursor. Para eso,
ejecutamos el cursor con la consulta que deseemos realizar.

![](https://assets.pubpub.org/zl19ca89/71627112096520.jpeg){#n4gs9wntg61}

Crédito de imagen: Codificación Python por Kimberly Riveros y Diana
Díaz, 2021, Mangosta (`mangosta.org/xii_codificacion-python/)` (.)
Acceso al código para docentes y alumnos:
(`mangosta.org/vi_codificacion-python/)`

![](https://assets.pubpub.org/6j2wxpgi/11627112343971.jpeg){#n471ip5146p}

Crédito de imagen: Codificación Python por Kimberly Riveros y Diana
Díaz, 2021, Mangosta (`mangosta.org/xiii_codificacion-python/)` (.)
Acceso al código para docentes y alumnos:
(`mangosta.org/vii_codigo-python/)`

Ya, para terminar, imprimimos por pantalla el resultado de la consulta.
Y, una vez acabado el trabajo en la base de datos, cerramos la conexión.

![](https://assets.pubpub.org/d5zmlpny/21627112638078.jpeg){#nrbbzyzk4cr}

Crédito de imagen: Codificación Python por Kimberly Riveros y Diana
Díaz, 2021, Mangosta (`mangosta.org/xiv_codificacion-python/)` (.)
Acceso al código para docentes y alumnos:
(`mangosta.org/viii_codificacion_python/)`

![](https://assets.pubpub.org/9l3y2l3v/11627112953790.jpeg){#nci5xfjuyu5}

Crédito de imagen: Codificación Python por Kimberly Riveros, 2021,
Mangosta (`mangosta.org/x_codificacion-python/)`

![](https://assets.pubpub.org/u7k6giww/51627113251765.jpeg){#nj4uim1qmhm}

Crédito de imagen: Codificación Python por Kimberly Riveros y Diana
Díaz, 2021, Mangosta (`mangosta.org/xv_codificacion-python/)` (.) Acceso
al código para docentes y alumnos:
(`mangosta.org/ix_codificacion-python/)`

# 9.- *Interfaces*

Usaremos éstas para el *front-end* de la aplicación (UI / UX) o la parte
interactiva donde el usuario interactúa con su aplicación y donde todas
las entradas se dan a través de esta capa. La usabilidad y el diseño del
entorno de las *interfaces \*en los distintos modos de funcionamiento
M1, M2 y M3 (que se definen más adelante, en el pto 10.2) deben poder
permitir un control parental y de gestión de dispositivos suficiente. El
diseño de la \*interface \*es un elemento clave para un uso práctico y
solvente del \*framework*.

#### 9.1.- Kivy

Kivy técnicamente es sólo otra biblioteca de Python. Pero no es una
cualquiera. Es un marco para crear \*interfaces \*de usuario que
funciona en múltiples plataformas como Android, iOS, Windows, Linux y
macOS. Puede pensarse también como una alternativa a React Native,
Flutter, Solar2D, etc.

Kivy fundamentalmente es para los amantes de Python y el *Machine
Learning*. Con la biblioteca Kivy la opción más acertada es usar Conda.
Instalar Kivy con Conda es fácil: «`conda install kivy -c conda-forge`».
Posteriormente podremos compilar una aplicación para iOS o Android a
partir del código fuente y ejecutarla en un simulador para poder
visualizar su aspecto y resultado final.

#### 9.2.- Kotlin

Kotlin es un lenguaje de programación de tipado estático que corre sobre
la máquina virtual de Java (JVM) y que también puede ser compilado a
código fuente de JavaScript. Aunque no tiene una sintaxis compatible con
Java, Kotlin está diseñado para interoperar con código Java y es
conocido por ser utilizado en la creación de aplicaciones de Android.

#### 9.3.- Elementos sociales y legales en el diseño de las *interfaces*

Lo que sigue son indicaciones específicas a la hora de codificar y
desarrollar los entornos gráficos y de comunicación con los usuarios de
este *framework*. Son premisas que en el inicio deben estar siempre
presentes para los desarrolladores para evitar lesivas modificaciones
técnicas futuras en los entornos finales implementados. El marco no debe
olvidar nunca su mayor objetivo: proteger en el mayor de los ámbitos y
favorecer la pluralidad social.

1.  Se deberá atender en su mayor extensión posible a criterios
    conceptuales y de uso en la igualdad de género y accesibilidad así
    como poner énfasis en evitar todo tipo de sesgo.

2.  El leguaje particularmente dirigido a menores de edad deberá ser
    cuidadosamente estudiado para un uso adecuado.

3.  El cumplimiento en la normativa de protección de datos y otros que
    le resulten de aplicación deberá ser un imperativo presencial.

4.  En la medida de lo posible se debe proporcionar suficiente
    información educacional sobre ciberseguridad e inteligencia
    artificial.

# III.- Estadios y modos aplicables al \*framework y directrices de requerimientos mínimos

La caracterización en estadios y modos de este marco se propone para
definir la infraestructura de sistemas y asegurar el nivel de
procesamiento adecuado y su velocidad para la ejecución en los distintos
dispositivos *smart* del mercado, así como para la descripción
generalizada de todos los instrumentos y herramientas asociadas.

## 10.1.- **Estadios**

### 10.1.1.- **Estadio 1 (E1)**: modo laboratorio y aprendizaje de la IA

Es en este estadio donde caben configuraciones y ajustes básicas y
avanzadas. Las distintas configuraciones de marcos y modelos nos
ayudarán a optimizar y mejorar el resultado para que posteriormente éste
y su experiencia asociada sea aplicado al estadio (E2), o modo de
ejecución de la IA.

Para entrenar a la inteligencia artificial podemos partir desde cero o
realizar una transferencia de conocimientos con un modelo entrenado
previamente. También podemos plantear\*\* modelos\*\* específicos para
las imágenes, para lo que ResNet nos ofrece en general resultados muy
aceptables en la proporción de aciertos en una clasificación dada,
*accuracy*, o tasa de aciertos en la predicción. Asimismo podemos usar
diferentes **marcos**, donde los más conocidos son Pytorh, Tensorflow,
Fastai y Keras\... Fastai es nuestro marco por defecto.

### 10.1.2.- **Estadio 2 (E2)**: modo de ejecución de la IA

Es el fruto del estadio E1. Este estado se define para un modelo
entrenado y guardado para su ejecución. No debe admitir configuraciones
avanzadas algunas.

## 10.2.- **Modos**

Los modos son los distintos sistemas de trabajo o maneras de operar que
pueden ser aplicados a este *framework* de cara a los diferentes
dispositivos inteligentes a los que les resulta de aplicación.

### 10.2.1.- **Modo 1(M1)**: modo autónomo o *standalone*

Le resulta de aplicación el estadio E2. Su objeto de definición es que
éste pueda ser descargado y ejecutable como cualquier otra app del
mercado. En él se ejecutarán mayoritariamente *smartphone*, *tablets* y
PC\'s. Se recomienda compatibilidad total del modo M1 para los sistemas
operativos Android 11 o superior; iOS 14.5.1 o superior; sistema
operativo Windows 10 o superior o MAX OS X 10.15 o superior. Las
características *hardware* mínimas serán las recomendadas por los
fabricantes de éstos.

### 10.2.2.- **Modo 2 (M2)**: modo enlazado o esclavo de otro dispositivo

Este modo se concibe para el emparejamiento o vinculación de como mínimo
dos dispositivos con tecnologías compatibles del tipo Bluetooth, WiFi u
otro mediante una app o un *software* satélite complementario para este
fin. Aquí es donde mayoritariamente se emplazarán la mayoría de los
*smartwatches* actuales, referenciados en el punto. 11.1x. Los
requerimientos mínimos aconsejados de *hardware*para el despliegue del
*framework*en modo M2 son: 4GB de memoria interna de almacenamiento,
512MB de RAM y Bluetooth 4.0.

### 10.2.3- **Modo 3 (M3)**: modo servidor o laboratorio de entrenamiento y aprendizaje

Concebido para operar principalmente en el estadio E1. Se concibe en
general el modo M3 para el diseño y gestión de la IA, así como el modo
que embebe y gestiona el\* *modo M1* \*y el modo M2 de todos los
dispositivos relacionados en torno a él. Éste es propio de una
infraestructura Cliente-Servidor. En base a la naturaleza de su
ejecución establecemos dos clases:

#### A) Modo 3A: computación en la nube

Análogo al sistema que nosotros hemos planteado en laboratorio\...
También puede ser implementado mediante una infraestructura *ad hoc\*\*,
\*propia de un proveedor de servicios del \*framework, \*y asumir los
gastos inherentes a su uso y despliegue. AWS de Amazon, Google Cloud,
Azureu otros son posibles candidatos para este fin. Para la gestión
completa del entorno adicionalmente se ha de implementar un servidor
XAMPP o similar en un \*hosting* dedicado y adecuar los servicios para
*Servlets \*de Java para la completa operatividad*. *Para el modo 3B los
requerimientos \*hardware* han de considerar como mínimo los usados en
nuestra configuración gratuita en laboratorio para garantizar un
resultado satisfactorio de ejecución de la IA, así como los indicados
por los fabricantes del *software* asociado a éste.

#### B) Modo 3B: computación en local

Para poder trabajar en este modo podemos efectuar una instalación local
con fastai y Anaconda o Miniconda bajo sistema operativo Linux o
Windows. Asimismo para la gestión propia de la BD se ha de instalar
Mysql o similar. Y Java JDK para el desarrollo y ejecución de la
codificación en Java.

Para este modo 3B el *hardware* ha de ser más específico y exigente,
aunque no debería exceder los 2.500 ó 3.000 euros a precio actual de
mercado para una configuración óptima de un entorno de producción. Debe
ser capaz de ejecutar un *software* similar a Anaconda Enterprise 4 ó 5.
(Los requerimientos son CPU: 2 x 64-bit 2.8 GHz 8.00 GT/s CPUs; RAM: 32
GB (o 16 GB de 1600 MHz DDR3 RAM); 300 GB de almacenamiento en disco y
acceso a Internet). Se aconseja memoria RAM de 64 GB y un disco duro de
característica similares o superiores a 970 EVO Plus (1TB) de Samsung
para todo el conjunto que supone el Modo 3B.

La GPU es el componente que realizará la mayoría de cálculos lentos y
pesados en el proceso de entrenar a una red convolucional.
Mayoritariamente estos cálculos son multiplicaciones de matrices,
convoluciones y funciones de activación. El *hardware*proporcionado por
NVDIA, por ejemplo, el modelo 2080 Ti con 13.45 TFLOPs y 11 GB de
memoria GDDR6 es una base sólida. Para la CPU la marca Intel es una
opción ineludible. La familia i7, el i7-10700k, puede ser una elección
garante.

# IV.- Campo de aplicación

Los entornos donde el marco puede ser implementado generarán
especificaciones propias para cada entorno. Principalmente
caracterizadas por el *hardware, \*en la capa del modelo OSI que se
opere, su sistema operativo y su conectividad a una red determinada e
Internet, y la conexión entre dispositivos propiamente dicha. Se debe
prestar una atención importante a la velocidad de ejecución del
\*framework*, por lo que los procesos aplicables deben ser los más
imprescindibles y livianos que resulte posible y la velocidad aquélla
más óptima que pueda alcanzarse. Todo ello en el mejor balanceo de
rendimiento de consumos en general y tomando en cuenta las capacidades y
la morfología donde pretenda emplearse. Signifiquemos a continuación
algunos de estos entornos. Donde efectuaremos descripciones y propuestas
de *software* y de *hardware*, denotando que ni éstas son únicas ni
necesariamente imprescindibles en la implementación final.

## 11.1- *Smartwatches*

Un *smartwatch*puede suponer un estilo de vida, incluso un concepto de
libertad; son como una carcasa de reloj que porta y camufla tecnología
digital y se incorpora al sujeto como una prenda más diaria, un
*wearable* (tecnología vestible); \"La información que se mueve
contigo\", fue el lema de Android Wear en 2014. Y, aparejado de la mano,
o la muñeca en este caso, puede ir una grave pérdida de la privacidad.

Un reloj inteligente tiene matices que marcarán la diferencia, por
ejemplo, su diseño, su tamaño, su *interface* de usuario, si está o no
está asociado a un teléfono inteligente mediante *Bluetooth*, si el
reloj dispone de tarjeta SIM o no la tiene, si puede ser gestionado como
un dispositivo más en red (WiFi) o no, si realmente es un reloj
inteligente o una pulsera de activad (*fitness\*\*)*; o todo junto, o
parte de éstos.

Una amalgama de *hardware* y de prestaciones y precios de mercado están
asociado a ellos y que determinarán elementos como la pantalla (de onda
acústica, resistiva o capacitiva según su \*touch\*\*, \*y también su
panel: LCD, IPS, AMOLED o OLED), los sensores que incorpore, la memoria
RAM, memoria interna de almacenamiento, microprocesador (de fabricación
propia, Qualcomm, ARM, MediaTek\...), duración de la batería, si
incorporan audio y cámara fotográfica, tecnología de comunicación y
aproximación como NFC, Wifi o RFID, etc.

Podemos encontrarlos desde los precios más ínfimos (≈,\<) a 20 € en el
mercado chino en AliEspress (Tipmant Smartwatch) o en Amazon, hasta
precios que pueden rondar los 2.000€ (**TAG HEUER CONNECTED GOLF
EDITION)**, incluso más caros.

Y si ponemos la mirada hacia el futuro a la hora de inclinarnos por
ciertos tipos de dispositivos, la clave diferenciadora pudiera estar en
si reemplazan al *smartphone*, en si pueden hacer y recibir llamadas
(mediante bandas LTE, por ejemplo, vinculada a la democratización del
5G) y al Internet de la Cosas (IoT).

Habitualmente llevan una versión reducida como sistema operativo de
Android, iOS o Linux (Wear OS, WatchOS y Tizen OS) son los más
habituales. Esa tecnología digital es la plataforma donde puede
ejecutarse nuestro *framework*, en un entorno de S.O. reducido.

#### 11.1.1.- Hardware de algunos relojes inteligentes

  Dispositivo            Procesador                                                    Memoria                                                                    Comunicaciones                                                                       Pantalla                                                                                Cámara            Batería
  ---------------------- ------------------------------------------------------------- -------------------------------------------------------------------------- ------------------------------------------------------------------------------------ --------------------------------------------------------------------------------------- ----------------- -----------------------------
  Samsung Gear 2         Dual-Core 1 GHz Exynos 3250                                   4 GB de memoria interna; 512 MB RAM                                        Bluetooth 4.0 LE                                                                     1,63 pulgadas Super AMOLED; 320 × 320                                                   2,0 megapíxeles   300 mAh
  Samsung Gear S         Qualcomm Snapdragon 400 a 1 GHz                               4 GB de memoria interna; 512 MB RAM                                        Bluetooth 4.0; Wi-Fi; 3G                                                             AMOLED de 2 pulgadas; 360 × 480                                                         ---               300 mAh
  Samsung Gear S2        Exynos 3250                                                   4 GB de memoria interna; 512 MB RAM                                        Bluetooth; BT 4.1; Wi-Fi; NFC                                                        sAMOLED; 360 × 360; 302 ppi                                                             ---               250 mAh
  Moto 360               Single-Core 1 GHz TI OMAP 3 / Qualcomm Snapdragon Wear 3100   4 GB de memoria interna; 512 MB RAM / 1 GB RAM y 8 GB de memoria interna   Bluetooth 4.0 LE / Bluetooth 4.2; Wi-Fi b/g/n; NFC; GPS, GLONASS, Beidou y Galileo   LCD de 1,56 pulgadas; 320 × 290; 205 ppi / circular AMOLED de 1,2 pulgadas; 390 × 390   ---               320 mAh / 355 mAh
  LG G Watch             Qualcomm Snapdragon 400 a 1,2 GHz                             4 GB de memoria interna; 512 MB RAM                                        Bluetooth 4.0                                                                        1,65 pulgadas IPS LCD                                                                   ---               400 mAh
  LG G Watch R           Qualcomm Snapdragon 400 a 1,2 GHz                             4 GB de memoria interna; 512 MB RAM                                        Bluetooth 4.0                                                                        P-OLED de 1,3 pulgadas; 320 × 320                                                       ---               410 mAh
  Sony SmartWatch 3      Quad-core ARM Cortex A7 a 1,2 GHz                             4 GB de memoria interna; 512 MB RAM                                        Bluetooth 4.0; NFC                                                                   1,6 pulgadas; 320 × 320                                                                 ---               420 mAh
  Asus ZenWatch          Qualcomm Snapdragon 400 a 1,2 GHz                             4 GB de memoria interna; 512 MB RAM                                        Bluetooth 4.0                                                                        AMOLED táctil capacitiva de 1,63 pulgadas; 320 × 320                                    ---               369 mAh
  Apple Watch            Apple S1                                                      8 GB de memoria interna; 512 MB RAM                                        Wi-Fi y GPS del iPhone                                                               Retina con Force Touch                                                                  ---               Hasta 18 horas de autonomía
  Apple Watch Series 6   S6 dual-core de 64 bits                                       32 GB de memoria interna; 1 GB RAM                                         LTE y UMTS; Wi-Fi; Bluetooth 5.0                                                     Retina OLED LTPO                                                                        ---               Hasta 18 horas

Crédito figura 2: versión actualizada y mejorada de «Características
hardware. Reloj inteligente». Wikipedia, La enciclopedia libre, 15 de
febrero de 2021. Consulta original: 18 de julio de 2021. URL de
referencia conservada como texto plano:
`es.wikipedia.org/w/index.php?title=Reloj_inteligente&oldid=133248752.`

> El *framework* para cualquier *smartwatch* debe asumir que el estadio
> de funcionamiento recomendado es el **estadio E2** (ejecución de la
> IA) y el **modo M2** (entrelazado). De forma dinámica, en función de
> la evolución de la tecnología asociada a un *smartwatch*, el modo M1
> (autónomo) pudiera ser también una opción viable a medio plazo.

## 11.2.- Otros dispositivos afines

### 11.2.1- \*Smartphones, tablets \*y PC\'s

En la actualidad estos dispositivos han evolucionado de forma
contundente en la gama media del sector, son garantes y suficientes en
general, con altas prestaciones tecnológicas y una baja economía de
coste asociadas. Aunque las *tablets* parece que no terminan de
conseguir un nicho final suficiente de mercado en función de algunos
medidores asociados (Imagen 66). Son durísimos rivales los *smartphones*
y los PC\'s para ellas; porque, en comparación, en la relación
tecnología y precio éstas salen perdiendo probablemente. Las *tablets*
actualmente, ¿parecen más un \'capricho útil\' que una necesidad real
del usuario?\... No obstante fabricantes como Microsoft tienden a
unificar sistemas operativos como Windows 10 y 11 para que su
integración esté asegurada a estos tres niveles o familias de
dispositivos. Que todo sea escalable y transversal. Un todo en uno.

> El *framework* para cualquier elemento del tipo *smartphone*, *tablet*
> o PC debe asumir que el estadio de funcionamiento recomendado para
> éstos es el **estadio E2** (ejecución de la IA) y el **modo M1**
> (autónomo).

![](https://assets.pubpub.org/4y9axx8d/21626768198889.png){#n7b68vvpk1r}

**Crédito imagen:** «**Tendencias de Internet 2021. Estadísticas y
hechos por países». VPNMentor. URL:**
`es.vpnmentor.com/blog/tendencias-de-internet-estadisticas-y-datos-en-los-estados-unidos-y-el-mundo/,`
Mangosta (`mangosta.org/tendencias-2/)`

## 11.2.2.- Domótica

Hablar de domótica es hablar sobre todo de sus tipos de conexiones para
una aplicación total y real de ésta, conjuntamente con el 5G y/o el IoT
y una estructura más que probable en «*Bus* inteligente».

![](https://assets.pubpub.org/0936zh4g/31626930087898.png){#nbjfzmra2ge}

**Todos los sensores y actuadores están conectados a través de un cable
bus.\* \*El sistema completo es denominado \"sistema bus\"**. Crédito
imagen: «KNX Conocimientos básicos del estándar KNX». V9-14. `KNX.org`.
Accesado el 22/07/2021. Url:
`www.knx.org/wAssets/docs/downloads/Marketing/Flyers/KNX-Basics/KNX-Basics_es.pdf,`
Mangosta (`mangosta.org/conocimientos-basicos-2/)`

Actualmente todo un espacio de tecnología de comunicaciones inalámbricas
coexiste: Bluetooth, WiFi, RFID, NFC y otros. Aunque las morfologías de
conexión para la domótica aún están decidiéndose por parte de la
industria y de los estándares que se asocian, pugnan entre ellos
defiendo diferentes intereses.

Una línea de trabajo, por ejemplo, es ZigBee que persigue:
«Comunicaciones seguras con baja tasa de envío de datos y maximización
de la vida útil de sus baterías»; o Z -Wave: «Una red en malla que
utiliza ondas de radio de baja energía para comunicarse de un aparato a
otro, permitiendo el control inalámbrico de electrodomésticos y otros
dispositivos, como control de iluminación, sistemas de seguridad,
termostatos, ventanas, cerraduras, piscinas y garaje abrepuertas»; o
Sigfox: «Un operador de red global y creador de la red 0G fundado en
2009 que implementa redes inalámbricas para conectar dispositivos de
bajo consumo como pueden ser medidores eléctricos, centrales de alarmas
o relojes inteligentes, que necesitan estar continuamente encendidos y
enviando pequeñas cantidades de datos».

Aun dentro de estos tiempos tecnológicos tan convulsos, a nivel de
estrategias y vaivenes, (como un estándar y un valor seguro del mercado
de las comunicaciones para los próximos 5 años), Bluetooth y WiFi
parecen las apuestas más seguras y estables durante este periodo: a
nivel tecnológico y comercial, claro. Y la mayoría de los dispositivos
actuales uno (Bluetooth), u otro (WiFi), o ambos, los implementan:
disponen de esta tecnología tan popular entre los usuarios y
fabricantes.

> El *framework* para cualquier elemento doméstico (diferente a
> *smartwatch*, *smartphone*, *tablet*, y PC) debe asumir que el estadio
> de funcionamiento recomendado para elementos domóticos es el **estadio
> E2** (ejecución de la IA), y disponibles ambos modos M1 (autónomo) &
> M2 (asociado).

#### 11.2.2.1 Dispositivos domóticos con Android & iOS

Las aplicaciones *smarthome* son una evidencia. Son innegables. Y mucho
han contribuido sistemas operativos como Android e iOS. Y cualquier
dispositivo doméstico o *wearable* de los que hemos descrito aquí es
susceptible de implementar estos sistemas operativos. Y también pueden y
han de estar protegido por la implementación de nuestro *framework*. Que
de acuerdo a lo enunciado previamente puede ser \'montado\' sobre estas
tecnologías.

> **Por citar algunas referencias**: Amazon Alexa, Casa, Smartthings,
> Google Home, LIFX, Houseinhand KNX, TaHoma by Somfy, Home Connect App,
> Smart Life, Philips Hue, etc.

#### 11.2.2.2- Elemento multiplataforma JRE (\*Java Runtime Environment) \*en dispositivos domóticos

La máquina virtual Java, o JVM por sus siglas en inglés, es un entorno
multiplataforma ideado para ser portable con independencia de los
sistemas y el entorno que lo envuelva.

«La máquina virtual de Java puede estar implementada en *software*,
*hardware*, una herramienta de desarrollo o un navegador *web*; lee y
ejecuta código precompilado bytecode que es independiente de la
plataforma. La JVM provee definiciones para un conjunto de
instrucciones, un conjunto de registros, un formato para archivos de
clases, la pila, un heap con recolector de basura y un área de memoria.
Cualquier implementación de la JVM que sea aprobada por SUN debe ser
capaz de ejecutar cualquier clase que cumpla con la especificación.».

Es este soporte independiente, por tanto, un entorno domótico objetivo
en la implementación de nuestro DIID (**D**etector de **I**ntrusiones
**I**nteligente **D**omótico).

#### 11.2.2.3- Descripción del estándar de domótica KNX como marco regulador para dispositivos domóticos

El estándar de protocolo KNX se cimenta básicamente en el concepto de
bus inteligente, y su mejor descripción es entenderlo como una «neurona
central» que comunica todos los elementos de una vivienda. Este tipo de
conexión de dispositivos es muy propicio para una mejor y mayor
aplicación de la IA sin duda. La estandarización (ISO/IEC 14543) rige
esta tipología. Ambos, capacidad de aplicar la IA y la estandarización,
constituyen una plataforma ideal de aplicación para nuestro Detector de
Intrusiones Inteligente Domótico(DIID por su acrónimo), para la
implementación y aplicación de *framework* definido en este trabajo de
investigación.

Estos elementos a proteger por el DIID o (DI)\^2\^ van desde:
Interruptores de luz • Teclados de regulación de luz • Detectores de
movimiento • Detectores de presencia• Contactos de ventanas y puertas •
Timbre de la puerta de entrada • Contadores de consumo de agua, gas,
electricidad y calor • Sensores de sobretensión • Sensores de
temperatura ambiente, interior y exterior • Sensores de temperatura en
circuitos de agua caliente y calefacción • Módulos para prefijar la
temperatura de consigna en las habitaciones • Sensores de luminosidad
interior y exterior • Sensores de viento para control de persianas y
toldos • Indicadores de estado o fallo en aparatos de línea blanca •
Sensores de fuga • Sensores de nivel • Receptor de radiofrecuencia en
cierre de puertas • Receptor de infrarrojos de mandos a distancia •
Lectores de huellas dactilares o tarjetas electrónicas para control de
acceso\... Y un largo etcétera de actuadores y módulos.

> Y todos ellos han de ser protegidos, y el *framework*implementado en
> un IDS inteligente es de aplicación necesaria aquí.

![](https://assets.pubpub.org/gu01824o/71626926965330.png){#nuye4zh6iug}

**KNXnet/IP en el modelo de referencia OSI**. Crédito imagen: «KNX
Conocimientos básicos del estándar KNX». V9-14. `KNX.org`. Accesado el
22/07/2021. Url:
`www.knx.org/wAssets/docs/downloads/Marketing/Flyers/KNX-Basics/KNX-Basics_es.pdf,`
Mangosta (`mangosta.org/estandar-knx/)`

Los ensayos realizados en laboratorio aportan el conocimiento suficiente
para inferir que el marco puede ser embebido en un microcontrolador de
la familia PIC 18F, PIC18F4550 u otro similar con la sistemática de KNX,
apoyado por el diseño específico de circuitería electrónica que
implemente la lógica combinacional necesaria.

![](https://assets.pubpub.org/j4824z67/21626932999454.png){#nndvt887gq3}

 **Montaje En\* \*\*Protoboard** \*Del Sistema Pinguino.\*\* Julio F. De
la Cruz G. Accesado el 22/07/200121. Url:
`3.bp.blogspot.com/-JtZH7qb-j-w/UhLyWQdHLjI/AAAAAAAAAjQ/cjoKPWOIxsw/s1600/image7923.png,`
Mangosta (`mangosta.org/montaje-en-protoboard/)`

El diseño de este dispositivo domótico DIID se concibe como una
*appliance* que reúne como mínimo los requisitos *hardware*de un
*smartwatch* para embeber el estadio y modo de funcionamiento inherente
a éste anteriormente definidos, mediante un *software* encapsulado o
*firmware*.

Su filosofía de funcionamiento debe basarse en la técnica *plug&play* de
modo que pudiera ser un elemento de protección más, alojado entre los
dispositivos propios de un cuadro eléctrico de cualquier vivienda que
alberga normalmente un ICP, un diferencial e interruptores
magnetotérmicos.

Concebido por tanto el dispositivo IDS domótico (DIID) como un elemento
más adicional de seguridad y protección en un hogar o un entorno laboral
que implementara el estándar KNX.

# V.- Conclusión

Aquí se ha demostrado que el uso de la IA, en buena parte de sus
dimensiones, puede ser llevada a término con rigor académico, definible
en el campo de las matemáticas y usable en el del laboratorio; y que
puede ser pensada, razonada y aplicada.

El *framework* está motivado por la continua aparición y mejora de
dispositivos móviles en forma de *smartwatches*, *smartphones* y otros
similares que ha propiciado simultáneamente un creciente y desleal
interés en poner bajo la lupa y el control de los aplicativos a sus
usuarios. Éstos son presa de una abusiva injerencia en sus vidas para
telemedir y telegestionar sus hábitos en Internet, y también de
sustraerles de forma ofuscada datos personales como, por ejemplo, cuando
realizan deportes que precisan medir constantes vitales tales como
pulsaciones del corazón, presión arterial, nivel de oxígeno, etc. La
industria que los produce y los gestiona arteramente está canalizando
cada vez más esfuerzos económicos, así como ciencias como la
inteligencia artificial (IA), gestión de flujos de datos y estrategias
para que el resultado final apunte hacia la manipulación, venta y
secuestro de las vidas de los ciudadanos que las usan. Y todo ello con
un probable valedor y último propietario: la Inteligencia Artificial
(IA), llena de oscuridad y mecanismos internos oscuros o \'*Black
Boxes*\'\... Y de sociedades ingenuas, como la nuestra, que lo están
impulsando desconociendo probablemente la causa y el efecto final para
la humanidad.

## 12.1.- Corolario I

Podemos plantear este marco como un espacio generador de marcos, análogo
a un espacio vectorial que genera subespacios vectoriales; y al
SV_intrusión= {\[Pₓ(1\...n), Img_yᵢ\]\_0\^{3\^{(n)}}}como su sistema
generador homólogo, en pro de la transparencia y la explicabilidad de
tecnologías como la IA. Donde cada variable analógica, ambigua,
difícilmente definible, amplia de espectro y pensamiento, y abrumadora
en dimensiones, pueda ser escaneada en sus parámetros más fundamentales
como postula, demuestra y defiende este *framework*en nuestra opinión.
Aquellos parámetros iniciales que incluso un niño se atrevería a dibujar
de algo desconocido para él, un monigote como si fuese un garabato, como
un símbolo o como una primera idea de la niñez. Y a medida que el niño
aprenda y aumente \'sus parámetros\' de definición y valores personales,
entonces la definición será más profunda, concisa y ajustada a su propia
realidad. De igual manera que se presenta en este marco, en sus
cimientos. Para que la inteligencia artificial sea algo que pueda ser
explicable, transparente y dentro del control humano. Porque una
ingeniería inversa siempre podría devolverla al mundo del *homo
sapiens*, nuestro mundo real con todas sus imperfecciones y sus
realidades.

## 12.2.- Corolario II

Por inferencia del corolario I, otra de las mayores preocupaciones
entorno al la inteligencia artificial, el sesgo algorítmico: de género,
cultural, de raza, de discapacidades individuales, de pensamientos e
ideologías\... puede ser mitigado. Tratando de alcanzar una sociedad más
justa en valores. Al lado de sus logros tecnológicos humanos.

## 12.3.- Corolario III

Por inferencia de los corolarios I y II, la Inteligencia Artificial (IA)
**puede ser educada humanamente** para un bien superior de los humanos.

> ##### **¿Por qué y cómo?**

Puede ser educada porque podemos ser selectivos en aquellos parámetros
que queramos sesgar, (como se haría con la raza de un animal pura
sangre), y con ellos entrenar a la IA para que piense y decida en esa
línea determinada de razonamiento; de forma casi genética, puesto que
los parámetros supondrían *quasi* la síntesis genética en laboratorio
del pensamiento de una IA predefinida en función de qué sí y qué no
queremos tomar durante su periodo de entrenamiento.

**Todo comportamiento humano, o pensamiento abstracto o entelequia de
algo, puede ser escaneado (garabateado) por este *****framework***** de
acuerdo a la expresión algebraica**:

> (Σ j=1} \^{j=n} (Pₓₙ-Pₓₙ₋₁))-(Pₓ₁-Pₓₙ)=0 .

Esto último, llevado a sus extremos más negativos y crueles con la
humanidad, al contrario del fin que persigue este *framework*, también
puede generar IA\'s peligrosas; algo sobre lo que movimientos sociales
nacionalistas en Alemania y la teoría del \"superhombre\" también
reflexionaron: El *Übermensch*. Toda ciencia siempre tiene dos caras: el
buen uso y el mal uso para la humanidad.

## 12.4.- Corolario IV

-   Teorema del existencialismo ene-dimensional

> Dado un \*framework, \*como el aquí descrito, todo un universo de
> posibilidades ene-dimensionales pueden ser plasmadas en planos
> gráficos que lo representen a éste en un instante T1 determinado para
> un entorno definido y observable. (En una imagen que vale más que mil
> palabras\...).

**DEMOSTRACIÓN**

Las gráficas de este *framework* se ubican por completo en la aplicación
lineal f: ℕ→ ℝ\^2\... Pero:

«\[\...\] Atendiendo a las coordenadas de cada vector y su naturaleza
algebraica, la dimensión real sería la que sigue:

1.  El grupo de vectores Pₓ(1\...n) {Dim1}ℕ=1.

2.  El grupo de vectores Img_yᵢ {Dim2}ℝ=2.

3.  El grupo de vectores Pₓ {Dim3}ℕ=n.

4.  El grupo de vectores Img_y {Dim4}ℝ=n. \[\...\]».

Por lo que se puede inferir que al ser Img_y {Dim4}ℝ ene-dimensional,
también puede pensarse un Universo teórico de posibilidades, similar al
nuestro existencial, e ilimitado si seguimos el modelo expuesto aquí.
Allí adonde más allá de ℝ³ nuestra propia existencia nos hace tiritar de
dudas y escepticismo. Y que sin embargo queda resulto en planos gráficos
(como nuestra propia existencia en fotografías de recuerdos) mediante
Img_yᵢ {Dim2}ℝ=2; que lo vuelve visual y existencial: terrenal ante
nuestros ojos. Si cabe afectado y/o compatible con el «Principio de
Indeterminación de Heisenberg».

(Cqd: *Quod erat demonstrandum*)*.*

Y si algo hemos aprendido de la historia de la ciencia, es que mientras
la metafísica de los griegos y otros pensadores sembraban ideas y
abstracciones cualitativas, científicos y matemáticos como Galileo
Galilei, Isaac Newton, Laplace, Leibniz y otros «gigantes» lo llevaron
al campo de lo cuantificable e infinitesimalmente discreto y medible. Al
existencialismo matemático observable de Blaise Pascal.

**También hemos aprendido con sorpresa que una mente aniñada, curiosa y
despierta, puede suponer e intuir argumentos físicos y matemáticos
universales, sin necesidad de disponer de toda la información del
conjunto del que se compone \'un ente\'**, como así ya lo hiciera
*Philosophiæ naturalis principia mathematica* de Newton; alguien que
además alumbró el cálculo diferencial y vislumbró el universo matemático
en infinitésimos. Siguiendo la senda de su maestro, **Isaac Barrow**.

> Maestros y alumnos son la esperanza de toda ciencia humana. **El
> camino para aprender de los errores y el camino a seguir en el
> futuro**. Y con ello entrenar y educar también a nuestros probables
> descendientes imperecederos, o mentes robotizadas, que denominados
> como «Inteligencia Artificial (IA)»\... **Y así garantizar un mañana
> humanamente tecnológico, y no, por el contrario, un mañana
> tecnológicamente humano; porque sería un grave error**.
>
> ------------------------------------------------------------------------

## 12.5.- Actualización de continuidad SV --- 19/06/2026

La formulación de este trabajo conserva una intuición que, vista desde
el desarrollo posterior del Sistema Vectorial SV, **ya contenía el
núcleo operativo de una arquitectura mayor**: convertir una
configuración compleja en una imagen parametrizada (***el frame***** o
el suceso que representa**), hacerla legible para una inteligencia
artificial y mantener al ser humano como instancia de comprensión,
revisión y decisión. En **2020--2021** esa intuición se expresaba como
framework de detección de intrusiones mediante parámetros,
representación gráfica y clasificación por ResNet34; **en 2026** queda
reconocida como antecedente técnico directo de la célula SV, de la
imagen/frame y de la lectura factual del suceso.

![](https://assets.pubpub.org/c39829d0b-6cf2-4c57-8562-04aa66068f1d/p981b7276-6b0f-49a6-ba87-8813706ef0ac/u8d2bc69d-73c6-4669-a7e7-fb07fdba4171/image-61781899944618.png){#na0itk6upqb}

> **Aquí hay dos ejemplos de cómo se están usando y perfilando esos
> parámetros años más tarde..., y aún en desarrollo**: [Inmunología
> 1](https://juantoniolloretegea.github.io/SVperitus-dataset/aplicaciones/demo_wasm/index.html "null"),
> e [Imnunología
> 2](https://juantoniolloretegea.github.io/SVperitus-dataset/aplicaciones/demo_wasm/compositor.html "null").

La cadena formal del SV puede resumirse como una continuidad entre cinco
planos: primero, un **dominio** declarado; segundo, una **célula de
parámetros** **ternarios**; tercero, una proyección visible en forma de
imagen o **frame**; cuarto, una lectura de cambio como **suceso**;
quinto, un **retorno auditable** hacia decisión técnica, magnitud, forma
o dictamen. El punto que entonces aparecía como imagen asociada a
parámetros de intrusión se reconoce ahora como una forma temprana de
transducción entre vector, imagen y decisión.

En lógica formal sería:

v pertenece a {0,1,U}\^n

Imagen = proyección visible de una célula vectorial declarada

Suceso = dominio + frontera + traza + residual + retorno

Cierre poligonal = suma de diferencias locales igual a cero

La terna {0,1,U} no funciona como simple codificación instrumental. En
el SV maduro, 0, 1 y U **forman una gramática de decisión bajo
dominio**. U no equivale a probabilidad, ruido, pérdida de dato ni
incertidumbre estadística ordinaria; conserva una no determinación
actual que no debe clausurarse sin fundamento. Esta diferencia resulta
decisiva para la inteligencia artificial, porque impide que el sistema
convierta una falta de determinación en una certeza aparente. También
resulta decisiva para la ciencia, porque obliga a declarar dominio,
frontera, residual y retorno antes de cerrar un resultado.

La imagen parametrizada adquiere así una función más amplia que la
clasificación. La máquina puede procesar el vector; el ser humano puede
reconocer la forma; la ciencia puede conservar la traza del dominio, la
frontera y el residual. Esta convergencia permite una lectura común
entre inteligencia artificial, operador humano y análisis científico. La
imagen no sustituye al cálculo: lo hace visible. La célula no sustituye
al mundo: lo parametriza bajo condiciones declaradas.

El desarrollo posterior del [Sistema Vectorial
SV](https://doi.org/10.21428/39829d0b.06811a0d) **ha exigido además
crear matemáticas factuales (Véase:
**<https://doi.org/10.21428/39829d0b.9f868dc2> ). La matemática heredada
ofrece herramientas extraordinarias para magnitudes, funciones,
continuidad, **NO** probabilidad, **NO** estadística, **NO** tiempo como
primitiva, cálculo diferencial, cálculo integral, modelización, etc.,
sin embargo, la física del suceso requiere otra capa formal **cuando el
objeto no es una función sobre el tiempo**, sino una trayectoria de
hechos declarados dentro de un dominio (**T**rayectoria **P**oligonal
**A**ctiva o TPA) **y con la restricción de que no se reescriben nunca
un suceso (*****append-only*****)**. (Véase: [*Conjunto matemático
unificado del cambio factual, ciclos, medición factual y trayectorias
poligonales de activación en el Sistema Vectorial
SV*](https://doi.org/10.21428/39829d0b.2b3c9808 "null")*)*

![](https://assets.pubpub.org/c39829d0b-6cf2-4c57-8562-04aa66068f1d/p981b7276-6b0f-49a6-ba87-8813706ef0ac/u8d2bc69d-73c6-4669-a7e7-fb07fdba4171/image-11781900540369.png){#nhgayvyq7up}

[Pulse para más
información](https://www.itvia.online/pub/conjunto-matematico-unificado-del-cambio-factual-ciclos-medicion-factual-y-trayectorias-poligonales-de-activacion-en-el-sistema-vectorial-sv#n1rt67h2vix){target="_blank"}

Las matemáticas factuales del SV se orientan al cambio declarado, la
acumulación, la sensibilidad estructural, la frontera, el residual, el
retorno, la clausura y la reconfiguración. (**Véase**: [Sucesos,
horizontes y cambio estructural --- Una aproximación algebraica desde el
Sistema Vectorial
SV](https://doi.org/10.21428/39829d0b.229d91e4 "null"))

------------------------------------------------------------------------

Esta necesidad queda vinculada a la línea matemática posterior:

-   *Nuevas matemáticas del Sistema Vectorial SV y Física factual como
    conjunto iniciador*. DOI: 10.21428/39829d0b.67195860.

-   *Conjunto matemático unificado del cambio factual, ciclos, medición
    factual y trayectorias poligonales de activación en el Sistema
    Vectorial SV*. DOI: 10.21428/39829d0b.2b3c9808.

-   *Fundamentos operatorios absolutos del electromagnetismo factual en
    el Sistema Vectorial SV*. DOI: 10.21428/39829d0b.0cb6137e.

-   *Álgebra de composición intercelular del Sistema Vectorial SV*. DOI:
    10.21428/39829d0b.4ebab177. Página pública:
    [www.itvia.online/algebra-de-composicion-intercelularl-sv](www.itvia.online/algebra-de-composicion-intercelularl-sv "null")
    `.`

    ------------------------------------------------------------------------

La consecuencia conceptual es precisa: el tiempo puede entrar como
medida, registro o comparación, pero no como primitivo de la lectura. El
SV no empieza preguntando cuándo ocurre algo, sino qué dominio está
declarado, qué cambia, bajo qué frontera, con qué traza, qué residual
deja y qué retorno permite. **Esta inversión evita que el tiempo, la
estadística o la inferencia opaca ocupen el lugar de fundamento**.

Desde esta actualización, el valor histórico del framework de imágenes
parametrizadas no queda limitado a la ciberseguridad. Su núcleo se
integra en una cadena más profunda: parámetro, vector, imagen, frame,
suceso, dominio, residual y retorno. Aquella ingeniería de imágenes para
una IA explicable anticipaba una de las funciones centrales del SV:
construir una interfaz común entre máquina, ser humano y ciencia, con
trazabilidad hacia atrás **y sin caja negra**.

La lectura de conjunto queda formulada así: el framework mostró que una
realidad compleja podía ser escaneada por parámetros y devuelta como
imagen; el Sistema Vectorial SV generaliza esa operación como célula
ternaria, álgebra de composición, matemáticas factuales y física del
suceso. La continuidad no depende de una analogía externa, sino de una
misma estructura: capturar diferencias, proyectarlas como forma visible,
conservar la indeterminación honesta y devolver un resultado auditable.

### Continuidad bibliográfica pública sobre inteligencia artificial, imagen parametrizada y SV

La tabla siguiente sitúa esta publicación dentro de una línea pública de
investigación anterior y posterior, y se muestran algunos ejemplos de
esas publicaciones. También lo que ya comenzaba a ser el embrión del SV,
las imágenes de abajo lo acreditan. No actúa como bibliografía
automática, sino como muestra el origen documentado del interés por la
inteligencia artificial, su regulación, sus tecnologías, la lógica
ternaria, la imagen parametrizada y su desarrollo posterior hacia el
Sistema Vectorial SV. (Muchas de ellas disponibles en
[ResarchGate](https://www.researchgate.net/profile/Juan-Lloret-Egea/research "null")
o en
[F](https://figshare.com/authors/Juan_Antonio_Lloret_Egea/7854512 "null")[**igshare**](https://figshare.com/authors/Juan_Antonio_Lloret_Egea/7854512 "null"),
etc.).

+----------------------------------+----------------------------------+
| ![](https://assets.pubp          | ![](https://assets.pubp          |
| ub.org/c39829d0b-6cf2-4c57-8562- | ub.org/c39829d0b-6cf2-4c57-8562- |
| 04aa66068f1d/p981b7276-6b0f-49a6 | 04aa66068f1d/p981b7276-6b0f-49a6 |
| -ba87-8813706ef0ac/u8d2bc69d-73c | -ba87-8813706ef0ac/u8d2bc69d-73c |
| 6-4669-a7e7-fb07fdba4171/image-6 | 6-4669-a7e7-fb07fdba4171/image-4 |
| 1781896476719.png){#nixotx816ha} | 1781896372983.png){#nvgh1l5vexs} |
|                                  |                                  |
| [Pulse para ampliar la           | [Pulse para ampliar la           |
| ima                              | imag                             |
| gen](https://i1.rgstatic.net/pub | en](https://www.researchgate.net |
| lication/337050677_Los_estados_d | /publication/337290865_Backcover |
| e_la_Inteligencia_Artificial_The | _'Los_estados_de_la_Inteligencia |
| _states_of_Artificial_Intelligen | _Artificial_The_states_of_Artifi |
| ce_Volumen_1_2019_Volume_12019/l | cial_Intelligence'_Volumen_1_201 |
| inks/5dc5e0e192851c81803b1067/la | 9_Volume_12019){target="_blank"} |
| rgepreview.png){target="_blank"} |                                  |
+==================================+==================================+
+----------------------------------+----------------------------------+

+-------------------------------+------------+---+
| #### **Título de              | #### **Fe  | # |
| la publicación** {#r65371590} | cha** {#r3 | # |
|                               | 716285968} | # |
|                               |            | # |
|                               |            |   |
|                               |            | * |
|                               |            | * |
|                               |            | D |
|                               |            | O |
|                               |            | I |
|                               |            |   |
|                               |            | o |
|                               |            |   |
|                               |            | i |
|                               |            | d |
|                               |            | e |
|                               |            | n |
|                               |            | t |
|                               |            | i |
|                               |            | f |
|                               |            | i |
|                               |            | c |
|                               |            | a |
|                               |            | c |
|                               |            | i |
|                               |            | ó |
|                               |            | n |
|                               |            |   |
|                               |            | i |
|                               |            | n |
|                               |            | d |
|                               |            | i |
|                               |            | c |
|                               |            | a |
|                               |            | d |
|                               |            | a |
|                               |            | * |
|                               |            | * |
|                               |            |   |
|                               |            | { |
|                               |            | # |
|                               |            | r |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | 9 |
|                               |            | 2 |
|                               |            | 5 |
|                               |            | 7 |
|                               |            | } |
+===============================+============+===+
| *¿Furo de la electrónica:     | 2019-07-14 | I |
| procesadores ópticos?*        |            | S |
|                               |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 2 |
|                               |            | 5 |
|                               |            | 6 |
|                               |            | 5 |
|                               |            | . |
|                               |            | 4 |
|                               |            | 2 |
|                               |            | 7 |
|                               |            | 2 |
|                               |            | 0 |
|                               |            | / |
|                               |            | 1 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 2 |
|                               |            | 5 |
|                               |            | 6 |
|                               |            | 5 |
|                               |            | . |
|                               |            | 4 |
|                               |            | 2 |
|                               |            | 7 |
|                               |            | 2 |
|                               |            | 0 |
|                               |            | / |
|                               |            | 1 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| *¿Futuro de la electrónica:   | 2019-07-14 | I |
| memristores & memoria         |            | S |
| neuromórfica?*                |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | [ |
|                               |            | R |
|                               |            | e |
|                               |            | s |
|                               |            | e |
|                               |            | a |
|                               |            | r |
|                               |            | c |
|                               |            | h |
|                               |            | G |
|                               |            | a |
|                               |            | t |
|                               |            | e |
|                               |            | D |
|                               |            | O |
|                               |            | I |
|                               |            | : |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 7 |
|                               |            | 8 |
|                               |            | 9 |
|                               |            | 8 |
|                               |            | . |
|                               |            | 4 |
|                               |            | 7 |
|                               |            | 0 |
|                               |            | 4 |
|                               |            | 8 |
|                               |            | / |
|                               |            | 1 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 7 |
|                               |            | 8 |
|                               |            | 9 |
|                               |            | 8 |
|                               |            | . |
|                               |            | 4 |
|                               |            | 7 |
|                               |            | 0 |
|                               |            | 4 |
|                               |            | 8 |
|                               |            | / |
|                               |            | 1 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| ***¿Futuro de la electrónica: | 2019-07-14 | I |
| lógica ternaria?***           |            | S |
|                               |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | D |
|                               |            | O |
|                               |            | I |
|                               |            | : |
|                               |            |   |
|                               |            | - |
|                               |            |   |
|                               |            |   |
|                               |            |   |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 7 |
|                               |            | 4 |
|                               |            | 6 |
|                               |            | 5 |
|                               |            | . |
|                               |            | 9 |
|                               |            | 3 |
|                               |            | 2 |
|                               |            | 8 |
|                               |            | 1 |
|                               |            | / |
|                               |            | 1 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 7 |
|                               |            | 4 |
|                               |            | 6 |
|                               |            | 5 |
|                               |            | . |
|                               |            | 9 |
|                               |            | 3 |
|                               |            | 2 |
|                               |            | 8 |
|                               |            | 1 |
|                               |            | / |
|                               |            | 1 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| Quasi *realidades en el       | 2019-07-14 | I |
| horizonte futuro de la        |            | S |
| electrónica*                  |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | D |
|                               |            | O |
|                               |            | I |
|                               |            | : |
|                               |            |   |
|                               |            | - |
|                               |            |   |
|                               |            |   |
|                               |            |   |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 8 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 5 |
|                               |            | 3 |
|                               |            | 7 |
|                               |            | 6 |
|                               |            | 4 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 8 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 5 |
|                               |            | 3 |
|                               |            | 7 |
|                               |            | 6 |
|                               |            | 4 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| *Biointeligencia & análisis   | 2019-07-14 | I |
| de datos*                     |            | S |
|                               |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | D |
|                               |            | O |
|                               |            | I |
|                               |            | : |
|                               |            |   |
|                               |            | - |
|                               |            |   |
|                               |            |   |
|                               |            |   |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 1 |
|                               |            | 1 |
|                               |            | 8 |
|                               |            | 4 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 0 |
|                               |            | 7 |
|                               |            | 2 |
|                               |            | 9 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 1 |
|                               |            | 1 |
|                               |            | 8 |
|                               |            | 4 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 0 |
|                               |            | 7 |
|                               |            | 2 |
|                               |            | 9 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| *Economía y antirreglas de la | 2019-07-16 | I |
| ética en la IA*               |            | S |
|                               |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | D |
|                               |            | O |
|                               |            | I |
|                               |            | : |
|                               |            |   |
|                               |            | - |
|                               |            |   |
|                               |            |   |
|                               |            |   |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 5 |
|                               |            | 0 |
|                               |            | 9 |
|                               |            | 1 |
|                               |            | . |
|                               |            | 8 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | 0 |
|                               |            | 6 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 5 |
|                               |            | 0 |
|                               |            | 9 |
|                               |            | 1 |
|                               |            | . |
|                               |            | 8 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | 0 |
|                               |            | 6 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| *Nuevos horizontes y          | 2019-08-20 | I |
| materiales en la electrónica* |            | S |
|                               |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 5 |
|                               |            | 9 |
|                               |            | 2 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 8 |
|                               |            | 7 |
|                               |            | 0 |
|                               |            | 4 |
|                               |            | 8 |
|                               |            | / |
|                               |            | 1 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 5 |
|                               |            | 9 |
|                               |            | 2 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 8 |
|                               |            | 7 |
|                               |            | 0 |
|                               |            | 4 |
|                               |            | 8 |
|                               |            | / |
|                               |            | 1 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| *Mirando en la historia: en   | 2019-08-24 | I |
| las venas, tejidos y órganos  |            | S |
| artificiales*                 |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            |   |
|                               |            | - |
|                               |            |   |
|                               |            |   |
|                               |            |   |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 6 |
|                               |            | 1 |
|                               |            | 5 |
|                               |            | 5 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 8 |
|                               |            | 5 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 6 |
|                               |            | 1 |
|                               |            | 5 |
|                               |            | 5 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 8 |
|                               |            | 5 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| *Las Interfaces Cerebro       | 2019-08-24 | I |
| Computadora*                  |            | S |
|                               |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
+-------------------------------+------------+---+
| *Procesamiento del Lenguaje   | 2019-08-25 | I |
| Natural (PLN)*                |            | S |
|                               |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | D |
|                               |            | O |
|                               |            | I |
|                               |            | : |
|                               |            |   |
|                               |            | - |
|                               |            |   |
|                               |            |   |
|                               |            |   |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 0 |
|                               |            | 6 |
|                               |            | 3 |
|                               |            | 5 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 9 |
|                               |            | 2 |
|                               |            | 0 |
|                               |            | 5 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 0 |
|                               |            | 6 |
|                               |            | 3 |
|                               |            | 5 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 9 |
|                               |            | 2 |
|                               |            | 0 |
|                               |            | 5 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| *IA empresarial*              | 2019-09-05 | I |
|                               |            | S |
|                               |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | D |
|                               |            | O |
|                               |            | I |
|                               |            | : |
|                               |            |   |
|                               |            | - |
|                               |            |   |
|                               |            |   |
|                               |            |   |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 4 |
|                               |            | 4 |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 5 |
|                               |            | 6 |
|                               |            | 5 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 4 |
|                               |            | 4 |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 5 |
|                               |            | 6 |
|                               |            | 5 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| *IA confiable de IBM*         | 2019-09-05 | I |
|                               |            | S |
|                               |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | D |
|                               |            | O |
|                               |            | I |
|                               |            | : |
|                               |            |   |
|                               |            | - |
|                               |            |   |
|                               |            |   |
|                               |            |   |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 1 |
|                               |            | 5 |
|                               |            | 2 |
|                               |            | 5 |
|                               |            | . |
|                               |            | 0 |
|                               |            | 9 |
|                               |            | 4 |
|                               |            | 4 |
|                               |            | 5 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 1 |
|                               |            | 5 |
|                               |            | 2 |
|                               |            | 5 |
|                               |            | . |
|                               |            | 0 |
|                               |            | 9 |
|                               |            | 4 |
|                               |            | 4 |
|                               |            | 5 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| *Estándares y seguridad en el | 2019-10-20 | I |
| uso humano de la IA*          |            | S |
|                               |            | S |
|                               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | D |
|                               |            | O |
|                               |            | I |
|                               |            | : |
|                               |            |   |
|                               |            | - |
|                               |            |   |
|                               |            |   |
|                               |            |   |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 4 |
|                               |            | 1 |
|                               |            | 8 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 5 |
|                               |            | 0 |
|                               |            | 4 |
|                               |            | 5 |
|                               |            | / |
|                               |            | 1 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 4 |
|                               |            | 1 |
|                               |            | 8 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 5 |
|                               |            | 0 |
|                               |            | 4 |
|                               |            | 5 |
|                               |            | / |
|                               |            | 1 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| *Guía y herramientas para el  | 2019-10-23 | I |
| diseño responsable y la       |            | S |
| implementación de sistemas de |            | S |
| IA*                           |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | D |
|                               |            | O |
|                               |            | I |
|                               |            | : |
|                               |            |   |
|                               |            | - |
|                               |            |   |
|                               |            |   |
|                               |            |   |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 5 |
|                               |            | 3 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 4 |
|                               |            | 4 |
|                               |            | 8 |
|                               |            | 6 |
|                               |            | / |
|                               |            | 1 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 5 |
|                               |            | 3 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 4 |
|                               |            | 4 |
|                               |            | 8 |
|                               |            | 6 |
|                               |            | / |
|                               |            | 1 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| *Regulación de la             | 2019-10-25 | I |
| Inteligencia Artificial en 7  |            | S |
| puntos estratégicos mundiales |            | S |
| y conclusiones*               |            | N |
|                               |            | 2 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | 5 |
|                               |            | - |
|                               |            | 3 |
|                               |            | 8 |
|                               |            | 0 |
|                               |            | 3 |
|                               |            | / |
|                               |            | [ |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 0 |
|                               |            | 2 |
|                               |            | 1 |
|                               |            | 8 |
|                               |            | . |
|                               |            | 4 |
|                               |            | 4 |
|                               |            | 4 |
|                               |            | 8 |
|                               |            | 3 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 1 |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 0 |
|                               |            | / |
|                               |            | R |
|                               |            | G |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 0 |
|                               |            | 2 |
|                               |            | 1 |
|                               |            | 8 |
|                               |            | . |
|                               |            | 4 |
|                               |            | 4 |
|                               |            | 4 |
|                               |            | 8 |
|                               |            | 3 |
|                               |            |   |
|                               |            | " |
|                               |            | n |
|                               |            | u |
|                               |            | l |
|                               |            | l |
|                               |            | " |
|                               |            | ) |
+-------------------------------+------------+---+
| *Posición española en         | 2019-11-22 | [ |
| tecnologías de la IA \|       |            | 1 |
| Spanish position in AI        |            | 0 |
| technologies*                 |            | . |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | / |
|                               |            | o |
|                               |            | s |
|                               |            | f |
|                               |            | . |
|                               |            | i |
|                               |            | o |
|                               |            | / |
|                               |            | y |
|                               |            | u |
|                               |            | b |
|                               |            | 6 |
|                               |            | e |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | / |
|                               |            | o |
|                               |            | s |
|                               |            | f |
|                               |            | . |
|                               |            | i |
|                               |            | o |
|                               |            | / |
|                               |            | y |
|                               |            | u |
|                               |            | b |
|                               |            | 6 |
|                               |            | e |
|                               |            | ) |
+-------------------------------+------------+---+
| *Los 7 mandamientos de la IA  | 2019-11-23 | [ |
| \| The 7 commandments of AI*  |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | 4 |
|                               |            | / |
|                               |            | o |
|                               |            | s |
|                               |            | f |
|                               |            | . |
|                               |            | i |
|                               |            | o |
|                               |            | / |
|                               |            | 6 |
|                               |            | 4 |
|                               |            | 2 |
|                               |            | t |
|                               |            | 9 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | 4 |
|                               |            | / |
|                               |            | o |
|                               |            | s |
|                               |            | f |
|                               |            | . |
|                               |            | i |
|                               |            | o |
|                               |            | / |
|                               |            | 6 |
|                               |            | 4 |
|                               |            | 2 |
|                               |            | t |
|                               |            | 9 |
|                               |            | ) |
+-------------------------------+------------+---+
| *La ética de la inteligencia  | 2019-11-23 | [ |
| artificial \| Ethics of       |            | 1 |
| Artificial Intelligence*      |            | 0 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | / |
|                               |            | o |
|                               |            | s |
|                               |            | f |
|                               |            | . |
|                               |            | i |
|                               |            | o |
|                               |            | / |
|                               |            | h |
|                               |            | 7 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | u |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | / |
|                               |            | o |
|                               |            | s |
|                               |            | f |
|                               |            | . |
|                               |            | i |
|                               |            | o |
|                               |            | / |
|                               |            | h |
|                               |            | 7 |
|                               |            | 6 |
|                               |            | 9 |
|                               |            | u |
|                               |            | ) |
+-------------------------------+------------+---+
| *IA confiable del Instituto   | 2019-11-23 | [ |
| de Ingeniería Eléctrica y     |            | 1 |
| Electrónica \| Reliable AI of |            | 0 |
| the Institute of Electrical   |            | . |
| and Electronic Engineering*   |            | 3 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | / |
|                               |            | o |
|                               |            | s |
|                               |            | f |
|                               |            | . |
|                               |            | i |
|                               |            | o |
|                               |            | / |
|                               |            | x |
|                               |            | t |
|                               |            | k |
|                               |            | 3 |
|                               |            | r |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | / |
|                               |            | o |
|                               |            | s |
|                               |            | f |
|                               |            | . |
|                               |            | i |
|                               |            | o |
|                               |            | / |
|                               |            | x |
|                               |            | t |
|                               |            | k |
|                               |            | 3 |
|                               |            | r |
|                               |            | ) |
+-------------------------------+------------+---+
| *Artificial intelligence as a | 2019-12-10 | [ |
| legal entity and its civil    |            | 1 |
| responsibility within Europe  |            | 0 |
| \| La inteligencia artificial |            | . |
| como ente jurídico y su       |            | 3 |
| responsabilidad civil en el   |            | 1 |
| seno de Europa*               |            | 2 |
|                               |            | 2 |
|                               |            | 4 |
|                               |            | / |
|                               |            | o |
|                               |            | s |
|                               |            | f |
|                               |            | . |
|                               |            | i |
|                               |            | o |
|                               |            | / |
|                               |            | z |
|                               |            | a |
|                               |            | c |
|                               |            | 3 |
|                               |            | 4 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | 4 |
|                               |            | / |
|                               |            | o |
|                               |            | s |
|                               |            | f |
|                               |            | . |
|                               |            | i |
|                               |            | o |
|                               |            | / |
|                               |            | z |
|                               |            | a |
|                               |            | c |
|                               |            | 3 |
|                               |            | 4 |
|                               |            | ) |
+-------------------------------+------------+---+
| *\'Framework\' basado en      | 2021-07-24 | [ |
| imágenes parametrizadas sobre |            | 1 |
| ResNet para identificar       |            | 0 |
| intrusiones en                |            | . |
| \'smartwatches\' u otros      |            | 2 |
| dispositivos afines*          |            | 1 |
|                               |            | 4 |
|                               |            | 2 |
|                               |            | 8 |
|                               |            | / |
|                               |            | 3 |
|                               |            | 9 |
|                               |            | 8 |
|                               |            | 2 |
|                               |            | 9 |
|                               |            | d |
|                               |            | 0 |
|                               |            | b |
|                               |            | . |
|                               |            | 9 |
|                               |            | 8 |
|                               |            | 1 |
|                               |            | b |
|                               |            | 7 |
|                               |            | 2 |
|                               |            | 7 |
|                               |            | 6 |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 2 |
|                               |            | 8 |
|                               |            | / |
|                               |            | 3 |
|                               |            | 9 |
|                               |            | 8 |
|                               |            | 2 |
|                               |            | 9 |
|                               |            | d |
|                               |            | 0 |
|                               |            | b |
|                               |            | . |
|                               |            | 9 |
|                               |            | 8 |
|                               |            | 1 |
|                               |            | b |
|                               |            | 7 |
|                               |            | 2 |
|                               |            | 7 |
|                               |            | 6 |
|                               |            | ) |
+-------------------------------+------------+---+
| *Framework based on           | 2021-08-10 | [ |
| parameterized images on       |            | 1 |
| ResNet to identify intrusions |            | 0 |
| in smartwatches or other      |            | . |
| related devices*              |            | 3 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | 4 |
|                               |            | / |
|                               |            | o |
|                               |            | s |
|                               |            | f |
|                               |            | . |
|                               |            | i |
|                               |            | o |
|                               |            | / |
|                               |            | 9 |
|                               |            | w |
|                               |            | u |
|                               |            | n |
|                               |            | q |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 3 |
|                               |            | 1 |
|                               |            | 2 |
|                               |            | 2 |
|                               |            | 4 |
|                               |            | / |
|                               |            | o |
|                               |            | s |
|                               |            | f |
|                               |            | . |
|                               |            | i |
|                               |            | o |
|                               |            | / |
|                               |            | 9 |
|                               |            | w |
|                               |            | u |
|                               |            | n |
|                               |            | q |
|                               |            | ) |
+-------------------------------+------------+---+
| *Desde SVcustos, el           | 2026-03-01 | [ |
| \'framework\' de intrusión, a |            | 1 |
| SVperitus*                    |            | 0 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 2 |
|                               |            | 8 |
|                               |            | / |
|                               |            | 3 |
|                               |            | 9 |
|                               |            | 8 |
|                               |            | 2 |
|                               |            | 9 |
|                               |            | d |
|                               |            | 0 |
|                               |            | b |
|                               |            | . |
|                               |            | e |
|                               |            | 1 |
|                               |            | d |
|                               |            | f |
|                               |            | 1 |
|                               |            | a |
|                               |            | 4 |
|                               |            | c |
|                               |            | ] |
|                               |            | ( |
|                               |            | h |
|                               |            | t |
|                               |            | t |
|                               |            | p |
|                               |            | s |
|                               |            | : |
|                               |            | / |
|                               |            | / |
|                               |            | d |
|                               |            | o |
|                               |            | i |
|                               |            | . |
|                               |            | o |
|                               |            | r |
|                               |            | g |
|                               |            | / |
|                               |            | 1 |
|                               |            | 0 |
|                               |            | . |
|                               |            | 2 |
|                               |            | 1 |
|                               |            | 4 |
|                               |            | 2 |
|                               |            | 8 |
|                               |            | / |
|                               |            | 3 |
|                               |            | 9 |
|                               |            | 8 |
|                               |            | 2 |
|                               |            | 9 |
|                               |            | d |
|                               |            | 0 |
|                               |            | b |
|                               |            | . |
|                               |            | e |
|                               |            | 1 |
|                               |            | d |
|                               |            | f |
|                               |            | 1 |
|                               |            | a |
|                               |            | 4 |
|                               |            | c |
|                               |            | ) |
+-------------------------------+------------+---+

La lectura de continuidad queda así determinada: el interés por la IA no
aparece en 2026 como consecuencia tardía del Sistema Vectorial SV;
procede de una línea pública iniciada en 2019, atraviesa la reflexión
sobre ética, regulación, confiabilidad, tecnologías emergentes, lógica
ternaria, *interfaces*, lenguaje natural y biointeligencia..., y
encuentra en 2020--2021 una primera forma técnica mediante imágenes
parametrizadas sobre ResNet. El SV maduro generaliza esa vía como célula
ternaria, imagen/frame, suceso, matemáticas factuales y retorno
científico.

> **El mayor logro del Sistema Vectorial SV** sería que el agente
> especializado operara en plena armonía con el experto humano:
> asistiendo sin suplantar, calculando sin decidir, compareciendo con
> estructura visible. El agente no nace del modelo de lenguaje sino del
> suelo algebraico del SV, y toda su legitimidad descansa en ese origen.
> La soberanía del experto y la disciplina del álgebra no son
> concesiones al sistema ---son su condición de posibilidad.

+----------------------------------+----------------------------------+
| ![](https://assets.pubp          | ![](https://assets.pubp          |
| ub.org/c39829d0b-6cf2-4c57-8562- | ub.org/c39829d0b-6cf2-4c57-8562- |
| 04aa66068f1d/p981b7276-6b0f-49a6 | 04aa66068f1d/p981b7276-6b0f-49a6 |
| -ba87-8813706ef0ac/u8d2bc69d-73c | -ba87-8813706ef0ac/u8d2bc69d-73c |
| 6-4669-a7e7-fb07fdba4171/image-1 | 6-4669-a7e7-fb07fdba4171/image-4 |
| 1781899305089.png){#nmgnemeyyte} | 1781899323017.png){#n5vw9wivr19} |
|                                  |                                  |
| [Pulse para ampliar la           | [Pulse para ampliar la           |
| imagen](https://assets.p         | imagen](https://assets.pubpub.   |
| ubpub.org/c39829d0b-6cf2-4c57-85 | org/c39829d0b-6cf2-4c57-8562-04a |
| 62-04aa66068f1d/u8d2bc69d-73c6-4 | a66068f1d/p981b7276-6b0f-49a6-ba |
| 669-a7e7-fb07fdba4171/image-1177 | 87-8813706ef0ac/u8d2bc69d-73c6-4 |
| 6110550699.png){target="_blank"} | 669-a7e7-fb07fdba4171/image-4178 |
|                                  | 1899323017.png){target="_blank"} |
+==================================+==================================+
| ![](https://assets.pubp          | ![](https://assets.pubp          |
| ub.org/c39829d0b-6cf2-4c57-8562- | ub.org/c39829d0b-6cf2-4c57-8562- |
| 04aa66068f1d/p981b7276-6b0f-49a6 | 04aa66068f1d/p981b7276-6b0f-49a6 |
| -ba87-8813706ef0ac/u8d2bc69d-73c | -ba87-8813706ef0ac/u8d2bc69d-73c |
| 6-4669-a7e7-fb07fdba4171/image-1 | 6-4669-a7e7-fb07fdba4171/image-2 |
| 1781899355745.png){#n2ja49j4j61} | 1781899365766.png){#nmpqswd6e83} |
|                                  |                                  |
| [Pulse para ampliar la           | [Pulse para ampliar la           |
| imagen](h                        | imagen](https://assets           |
| ttps://assets.pubpub.org/c39829d | .pubpub.org/c39829d0b-6cf2-4c57- |
| 0b-6cf2-4c57-8562-04aa66068f1d/u | 8562-04aa66068f1d/u8d2bc69d-73c6 |
| 8d2bc69d-73c6-4669-a7e7-fb07fdba | -4669-a7e7-fb07fdba4171/4._Acuer |
| 4171/3._Portada_carta_magna-0177 | do_entre_humano_y_humanoide-3177 |
| 6111359092.png){target="_blank"} | 6111397234.png){target="_blank"} |
+----------------------------------+----------------------------------+

# VI.- Sistemas y servicios auxiliares implementados

NextCloud y Gitlab para uso interno o externo de esta publicación.

# VII.- Declaración de conflicto de intereses

Los autores declaran que a fecha de esta versión no tienen ningún
conflicto de intereses. Esta publicación no está subvencionada por
ningún proyecto que pudiera aportarle financiación, ni tampoco bajo el
patrocinio o esponsorización de ninguna marca u otra similar. Cada uno
de los autores se representa así mismo, y actúa de forma independiente.
Asimismo se declara la intención futura de lanzar un *software* definido
en la url `mangosta.org`, y que a su vez este dominio y su \*hosting
\*sirven como herramienta de esta publicación para lograr mejoras
técnicas limitadas en esta plataforma.

# VIII- Manifiesto del equipo de investigación que ha realizado este trabajo: docentes y estudiantes

> Una realidad científica ésta, sobre la ciberseguridad y la IA descrita
> aquí, que es palpable y auditable en un documento nacido fruto de la
> investigación y la educación. Porque la ciencia, opinamos, no debe ser
> un negocio para nadie más que para la sociedad y en beneficio de la
> humanidad, un signo de la mayor *loa* y respeto a todos los
> científicos que nos hicieron posible llegar hasta aquí en el
> conocimiento; aquellas mentes y hombros de «genios gigantes» sobre los
> que nos montamos y logramos caminar mucho más rápido todos.
>
> \^**Juan Antonio Lloret Egea,** en nombre de todo el equipo de
> investigación.\^

**Keywords**: #cienciaabierta, #openscience, #investigación, #research,
#IA, #AI, #inteligenciaartificial #artificialintelligence, #IDS,
#ciberseguridad, #cybersecurity #español #educación #education
#enseñanza #skills

Licencia: Attribution-NonCommercial-NoDerivatives 4.0 International (CC
BY-NC-ND 4.0) (URL: `creativecommons.org/licenses/by-nc-nd/4.0/)`

# IX.- Agradecimientos

-   IES La Arboleda, Alcorcón, Madrid. `www.laarboleda.es/` por
    contribuir con docentes del Departamento de Informática, y alumnado
    de 1º de Desarrollo de Aplicaciones Multiplataforma (DAM) y de 2º de
    Sistemas Microinformáticos en Red (SMR). Asimismo por el apoyo de
    todo el equipo directivo del instituto. Y por los recursos
    aportados, parte de éstos están / fueron expuestos en
    `arboledalan.net/ciberseguridad/`

-   Biblioteca Regional de Murcia, por la ayuda a la difusión de la
    actividad:
    `bibliotecaregional.carm.es/agenda/presentacion-biblia-de-la-ia-ai-bible-publicacion-sobre-inteligencia-artificial/`

-   A título póstumo a *sir* Isaac Newton, **por enseñarnos a mirar y
    educarnos en la manera de hacerlo**: «He sido como
    un **niño** jugando a la **orilla del mar**, que se divierte al
    encontrar de vez en cuando un guijarro más suave que los demás o una
    concha más bonita, mientras el gran océano de la verdad se extiende
    sin descubrir ante mis ojos».

## Notas internas de la publicación

**1.** Para no perjudicar al idioma español, se usará en adelante de
forma indistinta la palabra «Framework» inglesa y la palabra «Marco» del
idioma español.

2.  La inteligencia artificial, AI (del inglés, *artificial
    intelligence*), es realmente el campo más prometedor, que ofrece una
    serie de ámbitos en los que se han utilizado sistemas de
    multivaluados, MVL (del inglés, *multi-valued logics*) debido a su
    posibilidad para abarcar un rango más amplio de estados.

Una primera área de aplicación la encontramos muy relacionada con la
posibilidad de representar preocupaciones, sentido común, razonamientos,
etc. por medio herramientas matemáticas como conjuntos difusos y lógica
difusa (del inglés, *fuzzy logic*).

Una segunda área de aplicación, relacionada con la anterior viene dada
por la automatización de datos y minería de conocimiento. Aquí, merecen
especial mención los métodos de *clustering* (procedimiento basado en la
unión de vectores que reúnen una determinada característica). En este
contexto también se están desarrollando técnicas que automatizen
sistemas MVL, así como en los métodos de la lógica de programación para
dichos sistemas. Parte de esta tendencia se debe al desarrollo reciente
de las lógicas de descripción generalizada, llamadas lógicas de
descripción difusa, que permiten la inserción de herramientas técnicas
(grados de verdad, conectivas, predicados graduales) procedentes de
MVL.». Lógica trivalente. (2021, 22 de mayo). *Wikipedia, La
enciclopedia libre*. Fecha de consulta: 14:58, julio 14, 2021 desde
\`[es.wikipedia.org/w/index.php?title=L%C3%B3gica_trivalente&oldid=135730214](es.wikipedia.org/w/index.php?title=L%C3%B3gica_trivalente&oldid=135730214 "null")
.\`

3.  «Los gráficos radiales comparan varias variables cuantitativas y son
    útiles para visualizar qué variables tienen valores similares, o si
    existen valores atípicos entre las variables. Los gráficos radiales
    se componen de una secuencia de radios, con cada radio que
    representa una sola variable. Los gráficos radiales también son
    útiles para determinar qué variables están puntuando alto o bajo
    dentro de un conjunto de datos». Fuente: IBM, \"Gráficos radiales\"
    accesible en
    \`[www.ibm.com/docs/es/spss-statistics/version-missing?topic=types-radar-charts\`](www.ibm.com/docs/es/spss-statistics/version-missing?topic=types-radar-charts` "null")

4.  «Investigadores de la Universidad George Washington, junto con
    investigadores de la Universidad de California, Los Ángeles, y la
    startup de tecnología profunda Optelligence LLC, han desarrollado un
    acelerador de red neuronal convolucional óptico capaz de procesar
    grandes cantidades de información, del orden de petabytes, por
    segundo. Esta innovación, que aprovecha el paralelismo masivo de la
    luz, presagia una nueva era de procesamiento de señales ópticas para
    el aprendizaje automático con numerosas aplicaciones, que incluyen
    automóviles autónomos, redes 5G, centros de datos, diagnósticos
    biomédicos, seguridad de datos y más.». *Developing Smarter, Faster
    Machine Intelligence with Light.* Universidad George Washington.
    17/12/2020.
