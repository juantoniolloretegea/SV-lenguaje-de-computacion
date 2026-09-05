# Acta técnica de arquitectura de software: núcleo, frontera y host del Lenguaje SV

**Fecha:** 4 de septiembre de 2026  
**Revisión de precisión:** 5 de septiembre de 2026  
**Repositorio:** `SV-lenguaje-de-computacion`  
**Sede:** `docs/calidad/` de `main`  
**Naturaleza:** fijación arquitectónica; no implementación  
**Estado:** arquitectura base constituida; elecciones de host, transporte e interoperabilidad pendientes de prueba  
**Registros aplicables:** `RETP-2026-072`; precisión rectora `RETP-2026-073`

## 1. Objeto

Esta acta fija el reparto mínimo de responsabilidades necesario para construir el sistema completo sin convertir el Lenguaje SV en un conjunto de conectores ni duplicar su semántica en cada plataforma.

Se constituye una arquitectura de tres estratos:

| Estrato | Responsabilidad | Exclusiones |
|---|---|---|
| Núcleo soberano | Parseo, AST, IR, bienformación y validación intrínseca hoy materializados; custodia futura de las operaciones algebraicas puras sólo cuando hayan sido especificadas y probadas. Una única fuente semántica en `sv_core`, reutilizable por ejecución nativa y WebAssembly. | Constitución de células de dominio, asignación de parámetros, cobertura de agentes, red, bases de datos, interfaz, protocolos clínicos, terminologías, autenticación, periféricos y reglas particulares de dominio. |
| Contrato de frontera | Sobre canónico y versionado entre núcleo y anfitrión: identidad, procedencia, orden, límites de recursos, diagnósticos, versiones y resultados. Convierte entradas externas en peticiones cerradas y conserva la causa de cada fallo. | No redefine `Tri`, no corrige datos en silencio, no inventa semántica de dominio y no sustituye al ensamblaje multifuente. |
| Host operacional | Integración con sistemas externos, red, persistencia, mensajería, identidad, autorización, interfaz, observabilidad y adaptadores. En salud puede alojar HL7/FHIR, DICOM, terminologías, historia clínica, laboratorio, telemedicina y software hospitalario. | No interpreta de nuevo el Lenguaje ni mantiene una segunda realización semántica en otro lenguaje. |

La separación es normativa para el diseño; no afirma que los dos estratos exteriores estén realizados.

## 2. Decisiones constituidas

### 2.1 Un solo núcleo semántico

Rust se conserva como lenguaje del núcleo. La posible adopción de .NET para el host no autoriza una reescritura del compilador ni una segunda semántica en C#. Toda plataforma deberá invocar la misma realización de `sv_core` o una derivación compilada de la misma fuente.

El núcleo seguirá siendo independiente de dominio y de infraestructura. Una necesidad de Inmunología, ciberseguridad inteligente o cualquier dominio posterior sólo podrá promoverse al núcleo cuando se demuestre que es un invariante del Lenguaje y no una conveniencia del primer caso estudiado.

### 2.1.1 Núcleo receptor y comprobador, no autor del dominio

La frontera de competencia se fija en los [Pilares y restricciones de diseño del Lenguaje de Computación SV](./PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md).

El núcleo preserva los invariantes universales `Σ={0,1,U}`, `b≥3`, `n=b²` y el estado como vector plano, ordenado y posicional de longitud `n`. No decide para ningún dominio el valor de `b`, el número de células, su composición, la asignación de parámetros ni la cobertura de un agente.

Las unidades competentes de dominio constituyen esas decisiones; las unidades competentes de agente reciben la constitución y declaran cobertura y capacidades; el Lenguaje debe esperar un contrato explícito, comprobarlo cuando sea representable y rechazar su ausencia o incoherencia sin rellenar, redondear, reordenar ni inferir.

La realización vigente sólo acredita la restricción `b≥3`, la derivación `n=b²` y la longitud del vector. La constitución completa dominio→células→agente no está hoy representada ni validada de extremo a extremo. Esta ausencia es una obligación arquitectónica pendiente, no una autorización para simularla mediante cadenas opacas.

### 2.2 WebAssembly como candidata inicial de aislamiento

WebAssembly es la candidata inicial para empotrar el núcleo en un host porque permite una frontera de memoria explícita y conserva una ruta común entre entornos. Esta preferencia no constituye exclusividad perpetua ni declara cerrada la comparación con FFI.

La elección final deberá someter, sobre el mismo corpus y el mismo contrato, al menos una integración real de host con WebAssembly y, si existe necesidad acreditada, su alternativa FFI. La decisión se tomará por aislamiento, comportamiento ante fallo, portabilidad, operabilidad, coste de serialización y mantenimiento; no por intuición ni por una diferencia de rendimiento no demostrada.

Si se adopta WebAssembly, la ausencia de importaciones no previstas, la identidad de resultados respecto de la ejecución nativa y los límites de memoria y tiempo deberán ser invariantes automáticos de regresión.

### 2.3 .NET como candidata de host, no como decisión cerrada

.NET es una candidata razonable para el estrato operacional por su ecosistema de integración, pero esta acta no fija versión, biblioteca, proveedor ni marco clínico. Antes de incorporarlo se exigirán:

1. prototipo mínimo contra el contrato de frontera real;
2. prueba de despliegue en los entornos objetivo;
3. comprobación de licencias, mantenimiento, soporte y cadena de suministro de cada dependencia;
4. tratamiento observable de cancelación, espera, caída, memoria agotada y respuesta parcial;
5. comparación reproducible con la alternativa técnicamente pertinente.

Ninguna biblioteca se incorpora por popularidad o por disponibilidad de un conector.

### 2.4 La red de regresión es parte de la arquitectura

La revisión del corte candidato `PR #61@fafd65b887658d8aecf429aa1fb78b7f78174e92` confirma dos mecanismos de fuga que deberán cerrarse antes de atribuir protección completa al núcleo:

- los 68 casos negativos no contienen un caso que declare un `failure_symbol` distinto de `Bottom`; por ello no ejercen directamente la separación `Bottom ≠ Tri.U`;
- `tests/r0_7_equivalence.py` y `tests/r0_wasm_parity.py` comparan JSON después de aplicar `sort_keys=True`; esa comparación demuestra equivalencia estructural, pero no el orden canónico ni la identidad byte a byte de la salida emitida.

En consecuencia, una red verde no podrá interpretarse como protección suficiente mientras falten, al menos:

1. un contraejemplo explícito de símbolo de fallo no canónico, ejercido en referencia, Rust nativo y WebAssembly;
2. un oráculo de bytes crudos —o de su hash— que no reordene el JSON antes de comparar;
3. ejecución desde Rust de los contratos negativos y de salida que hoy sólo pueden quedar cubiertos por la orquestación externa.

El resultado privado de una campaña de mutación no se incorpora como métrica normativa. Se constituyen únicamente las dos causas comprobables anteriores y los oráculos necesarios para cerrarlas.

## 3. Contrato que deberá cerrar la frontera

El contrato de frontera deberá especificar, como mínimo:

- versión de ABI y compatibilidad;
- identidad del programa y de cada unidad de entrada;
- procedencia por objeto, orden y política de homónimos;
- codificación aceptada y tratamiento explícito de BOM, bytes no válidos y entrada vacía;
- límites de tamaño, memoria, tiempo y cancelación;
- distinción cerrada entre rechazo de entrada, fallo del núcleo, indisponibilidad del host y resultado válido del dominio;
- prohibición de convertir un fallo técnico en `Tri.U`;
- forma canónica de petición, respuesta y diagnóstico;
- correlación, idempotencia, repetición y trazabilidad entre sistemas;
- política de evolución y negociación de versiones.

El ensamblaje de fuentes no es este contrato. Ensamblar unidades y cruzar una frontera de ejecución son operaciones distintas y conservarán especificaciones separadas.

## 4. Dominios, perfiles y agentes

Se mantienen las siguientes distinciones:

1. **Perfil fuente:** lleva una sintaxis concreta a la misma IR sin introducir significado de dominio.
2. **Perfil o constitución de dominio:** deberá definir por completo, en su sede competente, el vocabulario, compromisos, identidades, fuentes, parámetros, células, asignación posicional, reglas, criticidades y límites del dominio fuera del núcleo universal.
3. **Agente:** será una realización consumidora de una constitución recibida. Podrá utilizar todo un dominio, una parte declarada o varios dominios compatibles sólo cuando su contrato lo represente y valide; no se identifica por defecto con ninguno de ellos.

La completitud del dominio y la cobertura total, parcial o multidominio del agente son requisitos arquitectónicos. No son hoy capacidades acreditadas por la sintaxis o la IR vigentes: `Domain` no contiene una versión completa del contrato y `Agent` sólo admite una referencia `domain` sin campo de cobertura. No se inferirán por igualdad de nombres ni por presencia de un objeto `Domain`.

La composición de dominios no enuncia por sí sola un superagente. Tampoco autoriza a tratar un perfil de dominio como si fuera un perfil lingüístico ES/EN.

## 5. Dos dominios como presión arquitectónica

Inmunología permanece como primer caso director. Ciberseguridad inteligente permanece como segundo falsador heterogéneo. Dos casos permiten detectar generalizaciones prematuras; no prueban universalidad absoluta.

La periferia no se aplaza como un simple puerto. En el caso médico incluye cruces de identidad entre sistemas, datos asincrónicos o obsoletos, pruebas, ingresos, tratamientos, telemedicina y disponibilidad parcial. Esas mismas discontinuidades son superficie de amenaza para ciberseguridad inteligente. Por ello, identidad, procedencia, autorización, integridad, frescura, repetición, recuperación y observabilidad deberán diseñarse conjuntamente en frontera y host, sin introducir HL7, FHIR, DICOM ni reglas clínicas en el núcleo por defecto.

## 6. Orden de trabajo que preserva el cierre existente

Esta acta no abre una fase ni modifica la secuencia operativa vigente. Fija el orden de dependencias arquitectónicas:

1. proteger los invariantes intrínsecos del núcleo con pruebas Rust que cubran el corpus negativo, los oráculos canónicos y la paridad nativa/WebAssembly;
2. cerrar los defectos nucleares que no dependen de una decisión de dominio;
3. contrastar la suficiencia representacional con Inmunología y ciberseguridad inteligente, sin herencia automática entre ellas;
4. especificar el contrato de frontera a partir de las pérdidas y discontinuidades demostradas;
5. construir un prototipo mínimo del host y comparar WebAssembly con cualquier alternativa FFI justificada;
6. sólo entonces seleccionar plataforma operacional, adaptadores, persistencia y mensajería para un entorno objetivo concreto.

Cada paso requiere su propia autorización material. Esta acta no integra la PR #61, no abre R2, R3 o R4, no incorpora un laboratorio privado y no autoriza datos reales ni uso clínico.

## 7. Criterios de revisión de la decisión

La elección de Rust para el núcleo se revisará si una prueba reproducible demuestra alguna de estas condiciones:

- imposibilidad material de empotrar el mismo núcleo en un entorno objetivo necesario;
- coste de frontera clínicamente u operacionalmente inaceptable;
- necesidad inevitable de importar capacidades del host que destruya el aislamiento requerido;
- divergencia semántica o de resultados entre realizaciones que no pueda corregirse sin duplicar el núcleo.

La elección del host se revisará por compatibilidad efectiva, licencias, soporte, seguridad, despliegue y coste total de mantenimiento. El mero número de bibliotecas disponibles no decide la arquitectura.

## 8. Límites explícitos

No quedan decididos por esta acta:

- .NET, su versión o una biblioteca concreta como plataforma definitiva;
- WebAssembly como única ABI admisible;
- FFI como solución prohibida;
- un bus, una base de datos, un sistema de identidad o una interfaz de usuario;
- el catálogo de conectores clínicos o de periféricos;
- la semántica ejecutiva de perfiles de dominio o agentes;
- aptitud clínica, conformidad regulatoria, seguridad sanitaria o producción.

Los resultados de auditorías y laboratorios externos han servido para formular estas decisiones, pero no se incorporan como evidencia normativa ni se trasladan al repositorio. Toda magnitud que deba gobernar una promoción futura tendrá que reproducirse y custodiarse en el circuito autorizado.

## 9. Estado resultante

```text
NUCLEO_SEMANTICO = UNO
LENGUAJE_NUCLEO = RUST
EJECUCION_NATIVA_Y_WASM = MISMA_FUENTE_SEMANTICA
CONTRATO_DE_FRONTERA = OBLIGATORIO_Y_PENDIENTE_DE_ESPECIFICACION
WASM = CANDIDATA_INICIAL_NO_EXCLUSIVA
FFI = ALTERNATIVA_CONDICIONADA_A_NECESIDAD_Y_PRUEBA
HOST_DOTNET = CANDIDATO_NO_CONSTITUIDO
PERFIL_FUENTE = DISTINTO_DE_PERFIL_DE_DOMINIO
DOMINIO = DISTINTO_DE_AGENTE
CONTRATO_DE_DOMINIO_COMPLETO = REQUISITO_NO_REPRESENTABLE_HOY
CONSTITUCION_DE_CELULAS = COMPETENCIA_DE_LA_UNIDAD_DE_DOMINIO
ELECCION_DE_B_Y_ASIGNACION_POR_EL_NUCLEO = PROHIBIDA
COBERTURA_DE_AGENTE = REQUISITO_ARQUITECTONICO_NO_REPRESENTABLE_HOY
REPARACION_O_INFERENCIA_SILENCIOSA = PROHIBIDA
EJECUCION_ALGEBRAICA_COMPLETA_EN_SV_CORE = NO_ACREDITADA
INMUNOLOGIA = PRIMER_CASO_DIRECTOR
CIBERSEGURIDAD_INTELIGENTE = SEGUNDO_FALSADOR
PR61_INTEGRADA_POR_ESTA_ACTA = NO
FASE_MATERIAL_ABIERTA_POR_ESTA_ACTA = NO
LABORATORIO_PRIVADO_INCORPORADO = NO
USO_CLINICO_O_DATOS_REALES = NO_AUTORIZADO
```

Queda constituido este reparto como base para las decisiones posteriores de frontera e integración, sin alterar el estado funcional del Lenguaje SV.
