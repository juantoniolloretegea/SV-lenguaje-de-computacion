# Pilares y restricciones de diseño del Lenguaje de Computación SV

**Fecha:** 5 de septiembre de 2026  
**Repositorio:** `SV-lenguaje-de-computacion`  
**Sede canónica:** `docs/calidad/`  
**Naturaleza:** especificación transversal subordinada y pieza rectora de diseño; no implementación  
**Estado:** `RECTOR_DE_DISENO · NO_DECIDE_DOMINIO · NO_ABRE_FASE`  
**Registro aplicable:** `RETP-2026-073`  
**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

## 0. Función y rango

Este documento fija una frontera que ninguna realización, optimización, perfil, host, agente o dominio puede borrar por comodidad: el Lenguaje de Computación debe custodiar los invariantes algebraico-semánticos del Sistema Vectorial SV y comprobar los contratos que recibe, pero no puede inventar la constitución de un dominio ni sustituir a las unidades competentes que la producen.

Su finalidad es impedir dos fallos simétricos:

1. que el núcleo invada la competencia del dominio y decida tamaños, células, parámetros o asignaciones que no le pertenecen;
2. que el núcleo permanezca mudo ante una declaración ausente, incompleta, contradictoria o imposible y la acepte, la repare o la transforme en un resultado aparentemente válido.

Esta pieza está subordinada a los fundamentos matemáticos y semánticos soberanos del SV, a la doctrina canónica de `U` y a los documentos soberanos aplicables a los agentes especializados. Complementa el [acta de arquitectura de software](./ACTA_TECNICA_DE_ARQUITECTURA_DE_SOFTWARE_NUCLEO_FRONTERA_Y_HOST_SV_2026_09_04.md). No crea una doctrina matemática nueva, no elige una arquitectura de dominio y no constituye un bus.

Cuando un ejemplo, comentario, prueba auxiliar o realización contradiga esta pieza o sus fuentes de rango superior, la contradicción deberá hacerse visible y resolverse en la sede competente. No se permitirá que el comportamiento accidental del código se convierta por inercia en doctrina.

## 1. Pilares algebraico-semánticos que el Lenguaje debe preservar

### 1.1 Alfabeto y célula exacta

El alfabeto semántico canónico es:

[
Sigma = {0,1,U}
]

Para cada célula exacta, la constitución declara un número natural `b` con `b ≥ 3`. El número de parámetros de la célula es:

[
n=b^2
]

y su estado exacto pertenece a:

[
mathcal{S}_n=Sigma^n
]

La cardinalidad del espacio de estados posibles es:

[
|mathcal{S}_n|=3^n
]

Estas igualdades no son una sugerencia de implementación ni una opción del perfil.

### 1.2 La célula no es una matriz b por b

Una célula exacta es un **vector plano, ordenado y posicional de longitud `n`**. La igualdad `n=b²` determina la longitud; no convierte la célula en una tabla, una cuadrícula ni una colección de `b` filas de `b` parámetros.

Por tanto, quedan prohibidas expresiones y realizaciones que traten la célula como «celda `b × b`», «matriz `b × b`» o cualquier estructura bidimensional equivalente, salvo que se declare expresamente como representación auxiliar sin autoridad ontológica y se pruebe una correspondencia exacta, reversible y sin reordenación con el vector canónico.

### 1.3 Mínimo y derivación

La condición `b ≥ 3` es intrínseca. El núcleo debe rechazar toda `CellSpec` con `b < 3`.

El valor `n` no es una segunda decisión libre: se deriva de `b`. Ningún host, perfil, agente, parser o serializador puede aceptar simultáneamente un `b` y un `n` incompatibles, ni sustituir uno por otro, ni escoger el cuadrado «más próximo».

### 1.4 Orden e identidad posicional

Cada posición del vector conserva identidad, orden y vínculo con el parámetro que la constitución competente le haya asignado. Permutar, ordenar alfabéticamente, compactar, deduplicar o reconstruir posiciones cambia el objeto salvo prueba normativa expresa en contrario.

La proyección, serialización, ensamblaje y transporte deberán preservar este orden o declarar una transformación canónica reversible cuya legitimidad haya sido constituida antes.

### 1.5 `U` no es relleno ni fallo técnico

`U` es indeterminación honesta dentro de una posición válidamente constituida. No es:

- un valor para completar una célula mal dimensionada;
- un sustituto de un parámetro ausente;
- un error de captura, parseo, memoria, red, host o periférico;
- una excusa para aceptar una referencia rota;
- una inferencia probabilística, estadística u opaca;
- un valor que un LLM pueda cerrar por plausibilidad.

`Bottom`, rechazo de entrada, indisponibilidad, agotamiento de recursos, fallo de ejecución y `Tri.U` deben permanecer separados.

## 2. Reparto de autoridad

| Sede | Debe decidir y entregar | Tiene prohibido decidir |
|---|---|---|
| Unidad competente de dominio | Universo y límites del dominio; inventario de parámetros; constitución de cada célula; valor de `b` de cada célula; número de células; orden y asignación posicional; relaciones y composición; fuentes, criticidades y reglas propias. | Semántica universal distinta de la doctrina SV; relajación de `b ≥ 3`, `n=b²`, `Σ` o del estatuto de `U`. |
| Unidad competente de agente | Identidad, alcance y fase del agente; cobertura explícita de la constitución recibida; capacidades, permisos, límites operativos, acoplamientos y trazabilidad específicos. | Rehacer por conveniencia la constitución del dominio; inventar parámetros o células; ampliar autoridad del núcleo o de una IA auxiliar. |
| Lenguaje y núcleo | Representar lo que su versión autorice; comprobar invariantes universales; comprobar que el contrato recibido es explícito, íntegro y coherente; preservar identidad, orden y procedencia; producir diagnóstico preciso y fallar cerrado. | Elegir el tamaño de una célula de dominio; repartir inventarios; crear, fusionar o dividir células; asignar parámetros; inferir cobertura de agente; completar contratos; fabricar semántica de dominio. |
| Frontera y host | Transportar contratos y resultados; aislar recursos; custodiar identidad, procedencia, autorización, repetición, cancelación, fallos y estados materiales. | Reinterpretar el Lenguaje; producir `0`, `1` o `U` por estado de transporte; mantener una segunda álgebra soberana. |
| IA o LLM auxiliar | Extraer candidatos y observables, organizar, contrastar o redactar dentro del permiso explícito de un agente y bajo custodia. | Crear la constitución del dominio; modificarla; rellenar ausencias; cerrar `U`; asignar `K₃`; emitir decisión soberana; contaminar el núcleo o sus registros. |

La secuencia de autoridad es:

1. el dominio queda delimitado y constituido por la unidad competente;
2. el agente recibe esa constitución y declara su cobertura y capacidades;
3. el Lenguaje recibe contratos ya constituidos y los valida;
4. la frontera y el host sólo materializan y transportan lo validado;
5. cualquier IA auxiliar permanece fuera de la cadena soberana de decisión.

El inventario preliminar de un dominio no es una célula. Su cardinalidad no autoriza al Lenguaje a escoger `b`, a calcular cuántas células «convienen» ni a forzar el inventario dentro de una forma disponible.

## 3. Prohibiciones absolutas de reparación silenciosa

Ante un contrato de dominio o agente ausente, incompleto o contradictorio, quedan prohibidas las siguientes operaciones automáticas:

1. deducir `b` del número de parámetros;
2. redondear la cardinalidad a un cuadrado perfecto;
3. seleccionar el cuadrado inferior, superior o más próximo;
4. rellenar posiciones con `U`, `0`, `1`, valores nulos o parámetros ficticios;
5. omitir, repetir, deduplicar o reordenar parámetros;
6. crear células residuales o fusionar células;
7. escoger el número de células;
8. asignar parámetros a posiciones por orden de llegada, nombre, tipo o conveniencia;
9. inferir que un dominio y un agente son equivalentes por compartir nombre;
10. inferir cobertura total, parcial o multidominio de un agente;
11. aceptar como valor por defecto una versión, procedencia, política o relación ausente;
12. convertir un fallo externo o técnico en `Tri.U`;
13. aceptar campos opacos como si estuvieran semánticamente comprobados;
14. dejar caer, durante lowering o serialización, una obligación declarada por el contrato;
15. hacer que un host, adaptador, bus o modelo reproduzca por su cuenta un juicio del núcleo.

La ausencia de representación vigente no autoriza una emulación provisional mediante cadenas libres. Debe declararse como obligación no representada.

## 4. Contrato que el núcleo deberá esperar sin decidirlo

Cuando la correspondiente versión del Lenguaje constituya esta frontera, la entrada deberá contener de manera explícita y versionada, como mínimo:

- identidad y versión de la constitución de dominio;
- procedencia y huella de la fuente autorizada;
- identidad de cada célula;
- `b` declarado y `n` derivable de forma unívoca;
- secuencia ordenada de posiciones;
- vínculo explícito entre cada posición y su parámetro constituido;
- identidad, procedencia y tipo de cada parámetro;
- relaciones, puentes y composición que sean aplicables;
- reglas de captura, ternarización, admisibilidad y resolución;
- política de `U` y separación de fallos;
- identidad, fase, cobertura y permisos del agente consumidor;
- límites y estados materiales que la frontera de ejecución deba conservar.

La forma exacta de este contrato y su sede —IR, meta-IR, manifiesto externo enlazado o combinación tipada— permanecen pendientes de decisión propia. Este documento fija el **deber de no omitirlo y de no suplantarlo**, no su sintaxis final.

Un campo presente pero no comprobado no equivale a un contrato válido. Un identificador opaco no acredita identidad, existencia, versión, procedencia ni correspondencia.

## 5. Comportamiento obligatorio ante error

El Lenguaje deberá distinguir, como clases no intercambiables:

1. error sintáctico;
2. infracción algebraica universal;
3. constitución de dominio ausente, incompleta o incoherente;
4. contrato de agente ausente, incompatible o fuera de cobertura;
5. referencia, versión, procedencia u orden inválidos;
6. fallo técnico del núcleo;
7. fallo material de frontera, host, red o periférico;
8. resultado semántico válido que contiene `Tri.U`.

Mientras una versión no pueda representar o comprobar una obligación, deberá declararlo como límite o deuda. No podrá afirmar que la valida.

Cuando una obligación representada falle:

- no se emitirá una IR aceptada como si el programa fuera válido;
- no se materializará un agente como si su cobertura estuviera acreditada;
- no se fabricará una salida de dominio;
- se preservará la causa exacta y la capa donde ocurrió;
- el diagnóstico será estable, comprobable y distinto de `U`.

«Fallar cerrado» no significa responder siempre con un error genérico. Significa impedir la promoción del objeto inválido y hacer visible la causa suficiente para corregirlo sin adivinar.

## 6. Estado verificable de la realización vigente

A fecha de esta pieza, el estado que puede afirmarse es:

| Obligación | Estado en la realización vigente | Consecuencia |
|---|---|---|
| `b ≥ 3` | Implementada en referencia Python y `sv_core`. | Una célula inferior al mínimo se rechaza. |
| Derivación `n=b²` | Implementada en lowering Python y frontend Rust. | `n` no se elige por separado. |
| Estado como vector plano `Vec<Tri>` | Implementado en la IR Rust. | No existe una matriz canónica `b × b`. |
| Longitud del vector igual a `n` | Implementada en validadores Python y Rust. | Un estado de longitud incompatible se rechaza. |
| Constitución completa de células desde un dominio | No representada ni comprobada de extremo a extremo. | No puede inferirse ni darse por cerrada. |
| Asignación posicional dominio → célula | No constituida como contrato tipado completo. | No puede simularse por orden o cadenas opacas. |
| Cobertura parcial o multidominio de agente | No representable hoy en la sintaxis/IR vigente. | Es requisito arquitectónico pendiente, no capacidad implementada. |
| Versión completa del perfil o contrato de dominio | No representable hoy como tal. | Requiere decisión de frontera y sede. |
| Ejecución soberana completa de operaciones algebraicas | La IR v0.3 declara que no está introducida; `sv_core` compila y valida, pero no ejecuta todavía toda el álgebra. | No debe atribuirse al núcleo una capacidad que no posee ni dejar la deuda oculta. |
| Bus central | No constituido. | No puede usarse como premisa ni como sede de autoridad. |
| Host .NET, WebAssembly o FFI final | No decidido. | Continúan como alternativas sometidas a prueba y licenciamiento. |

Esta tabla separa hecho, obligación y aspiración. No degrada los pilares porque una realización todavía no los cubra; impide que la ausencia se convierta en silencio.

## 7. Custodia de la ejecución algebraica

Los pilares algebraicos del SV no pueden tener varias realizaciones soberanas divergentes ni depender de que un host, motor de IA o repositorio no doctrinal los reinterprete.

La arquitectura deberá conducir a una única custodia ejecutable de la semántica y del álgebra canónicas, derivada de sus fuentes soberanas y compartida por los destinos materiales. Hasta que se decida y materialice esa custodia:

- no se afirmará que `sv_core` ejecuta operaciones que sólo representa o valida;
- una realización Python externa podrá servir de referencia o laboratorio, pero no adquirir autoridad por antigüedad o disponibilidad;
- ninguna capa LLM, estadística o probabilística entrará en la cadena de cálculo soberano;
- cualquier promoción de `T(n)`, `K₃`, compuertas u otra operación al núcleo exigirá correspondencia doctrinal, especificación, oráculos positivos y negativos y paridad entre destinos;
- la decisión de custodia se adoptará en su puerta propia, sin improvisarla durante una corrección de parser, IR o dominio.

El Lenguaje no decide qué células necesita un dominio. Sí debe garantizar que, una vez recibida una constitución legítima, la ejecución algebraica no cambie su objeto, su orden, su semántica ni su autoridad.

## 8. Condiciones de aceptación y regresión

Ninguna obligación de esta pieza se considerará protegida sólo porque una prueba general permanezca verde.

Para cada restricción que llegue a ser representable deberán existir:

- caso positivo mínimo;
- contraejemplo negativo que rompa exactamente esa restricción;
- diagnóstico de la capa correcta;
- prueba de que no se sustituye el fallo por `U`;
- conservación de orden, identidad y procedencia en la proyección;
- paridad sobre la misma entrada entre referencia autorizada, Rust nativo y WebAssembly cuando dichos destinos sean aplicables;
- oráculo de bytes o huella cuando se afirme identidad byte a byte;
- prueba de que host, adaptador o agente no pueden eludir el juicio del núcleo.

Los casos de prueba no elegirán tamaños, células ni parámetros para un dominio real. Podrán ejercitar invariantes universales con objetos sintéticos o contratos de dominio ya constituidos.

## 9. Control de cambios

Ninguna dependencia, actualización de compilador, biblioteca comunitaria, host o plataforma mantiene por sí sola la semántica del SV. La comunidad puede mantener la maquinaria; la doctrina, sus contratos y sus pruebas siguen siendo responsabilidad propia del proyecto.

Toda modificación que afecte a estos pilares deberá:

1. identificar su fuente soberana;
2. declarar la sede competente que toma la decisión;
3. demostrar que no convierte una conveniencia de dominio en invariante universal;
4. someterse a falsación con al menos un caso heterogéneo cuando la pretensión sea transversal;
5. actualizar especificación, código, corpus, diagnósticos y trazabilidad de forma coherente;
6. conservar compatibilidad o declarar ruptura de versión;
7. impedir que una representación auxiliar sustituya al objeto exacto.

Una actualización de mantenimiento no puede introducir por omisión una nueva autoridad, un valor adicional de `Tri`, una nueva semántica de `U`, un tamaño de célula o una regla de asignación de parámetros.

## 10. Efecto sobre la arquitectura en estudio

Esta pieza no decide un «perfil central», un bus, un host ni los perfiles específicos de los agentes. Establece el suelo que cualquiera de esas piezas deberá respetar si llega a constituirse.

La arquitectura en estudio deberá distinguir, sin mezclarlos:

- núcleo algebraico-semántico;
- Lenguaje y representación;
- contrato de constitución de dominio;
- contrato y cobertura del agente;
- frontera de ejecución;
- host y capacidades;
- acoplamiento custodial con fuentes externas;
- IA auxiliar sin autoridad soberana.

Ciberseguridad inteligente e Inmunología operan como presiones arquitectónicas heterogéneas, no como propietarias del SV. Sus resultados podrán falsar una generalización o aportar contratos constituidos; no autorizan al núcleo a diseñar sus células.

## 11. Estado resultante

~~~text
ALFABETO_CANONICO = {0,1,U}
CELULA_EXACTA = VECTOR_PLANO_ORDENADO
B_MINIMO = 3
N = B_AL_CUADRADO
CARDINALIDAD_ESTADOS = 3_ELEVADO_A_N
MATRIZ_B_POR_B = NO_CANONICA
U_COMO_RELLENO = PROHIBIDO
FALLO_TECNICO_COMO_U = PROHIBIDO
INFERENCIA_OPACA_EN_CADENA_SOBERANA = PROHIBIDA

CONSTITUCION_DE_DOMINIO = COMPETENCIA_EXTERNA_AL_NUCLEO
CONSTITUCION_DE_CELULAS = UNIDAD_COMPETENTE_DE_DOMINIO
CONSTITUCION_DE_AGENTE = POSTERIOR_A_CONSTITUCION_RECIBIDA
ELECCION_DE_B_POR_EL_NUCLEO = PROHIBIDA
ELECCION_DEL_NUMERO_DE_CELULAS_POR_EL_NUCLEO = PROHIBIDA
ASIGNACION_DE_PARAMETROS_POR_EL_NUCLEO = PROHIBIDA
RELLENO_REDONDEO_REORDENACION_REPARACION = PROHIBIDOS

NUCLEO = VALIDA_Y_PRESERVA_SIN_SUPLANTAR
CONTRATO_DE_DOMINIO_Y_AGENTE = OBLIGATORIO_Y_PENDIENTE_DE_SEDE
COBERTURA_DE_AGENTE = REQUISITO_NO_REPRESENTABLE_HOY
EJECUCION_ALGEBRAICA_COMPLETA_EN_SV_CORE = NO_ACREDITADA
BUS_CENTRAL = NO_CONSTITUIDO
HOST_FINAL = NO_DECIDIDO
FASE_MATERIAL_ABIERTA = NO
~~~

Queda fijada esta frontera como condición de diseño, revisión e implementación. El núcleo no recibirá autoridad para decidir el dominio; tampoco recibirá permiso para ignorar, completar o aceptar en silencio aquello que el dominio o el agente deban declarar.
