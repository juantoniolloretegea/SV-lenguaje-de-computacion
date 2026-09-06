# Acta técnica de perfiles, contratos y ensamblaje: representación, dominio y soporte tecnológico del Lenguaje SV

**Fecha:** 6 de septiembre de 2026  
**Sede canónica:** `SV-lenguaje-de-computacion/docs/calidad/`  
**Naturaleza:** fijación arquitectónica y de condiciones de suficiencia para el cierre; subordinada a los fundamentos y Pilares  
**Estado:** `MARCO_DE_DISENO_CONSTITUIDO · CONTRATOS_EJECUTABLES_PENDIENTES_EN_SU_ALCANCE`  
**Registro:** `RETP-2026-075`  
**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

> **Lectura prioritaria antes de proponer el cierre del núcleo o una ampliación de sus perfiles y contratos.** El objeto es determinar, desde necesidades constituidas y ejemplos comprobables, qué debe poder representar, preservar, validar y ejecutar el Lenguaje. La realización tecnológica se ensaya previamente en el laboratorio y sus resultados se reciben con alcance explícito. Esta acta fija el método y las obligaciones; no afirma que todos los contratos estén ya materializados en la DSL o la IR.

## 1. Objeto, rango y relación con el trabajo existente

Se fija una estructura de trabajo formada por tres componentes contractuales: **representación**, **dominio de conocimiento** y **soporte tecnológico de ejecución e intercambio**. Cada componente conserva identidad, versión, procedencia, alcance y condiciones de compatibilidad. Sus relaciones y ensamblajes deben declararse y comprobarse; compartir un nombre, un formato o una plataforma no acredita compatibilidad.

La finalidad inmediata es proporcionar al frente del Lenguaje las obligaciones que necesita conocer **antes de consolidar su núcleo**, sin introducir conocimiento clínico ni mecanismos particulares de infraestructura en la semántica universal. La apertura futura se apoya en contratos verificables y control de versiones. No se promete representabilidad de necesidades todavía no formuladas.

El marco desarrolla los [Pilares y restricciones de diseño](./PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), el [acta de arquitectura de núcleo, frontera y host](./ACTA_TECNICA_DE_ARQUITECTURA_DE_SOFTWARE_NUCLEO_FRONTERA_Y_HOST_SV_2026_09_04.md) y el [contrato de suficiencia representacional por operación](../arquitectura/CONTRATO_MINIMO_DE_SUFIENCIA_REPRESENTACIONAL_POR_OPERACION_PARA_EL_LENGUAJE_SV_2026_08_21.md). Ninguna de sus disposiciones altera Σ={0,1,U}, b≥3, n=b², el carácter plano, ordenado y posicional de la célula ni la autoridad de las unidades de dominio y agente.

El [registro experimental 020 y su candidata PT-SV-LOCAL](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/e97fed715ff5e3ae19bbaccaf2852e9cd3288377/laboratorio-de-infraestructura-SV/registros/020-PERFIL_TECNOLOGICO_Y_CONTINUIDAD_2026_09_06.md) conserva el inventario y las catorce obligaciones tecnológicas trazadas. La presente acta constituye su encuadre de diseño en Calidad; no duplica esa matriz ni promueve automáticamente su candidata a contrato ejecutable. La [ficha pública 020](https://juantoniolloretegea.github.io/SVcustos-dataset/laboratorio-de-infraestructura-SV/documentacion/perfil-tecnologico.html) facilita su recorrido documental.

## 2. Cortes y piezas rectoras consultados

| Sede | Corte de entrada | Uso en esta acta |
|---|---|---|
| Lenguaje, main | 230a205b08f4c54c9c8d9c1c7ad35b2f6ddbbfc4 | AGENTS.md, Pilares completos, arquitectura, RETP-072/073, perfiles fuente, suficiencia representacional y transición desde OP-IMM-001 |
| PR 61, cierre-nuclear-20260904 | ad8e8dd30930e35b75bf5f2fad78938d36233b78 | Radiografía N0 y reserva registral RETP-074 para N0-01; no integración por esta acta |
| Laboratorio privado | e97fed715ff5e3ae19bbaccaf2852e9cd3288377 | Registro 020, sus referencias a contratos y ensayos, y registro 021 de publicación |
| Documentación pública de laboratorios | 566aef09824f080c5cc09722223b5ac7327a32b4 | Ficha 020, índice y catálogo que recibirán el vínculo explícito a Calidad |

Estos cortes son antecedentes identificados. El commit que integre esta acta y la proyección pública posterior quedan registrados por sus respectivos historiales. Los resultados de una campaña mantienen siempre su propia fuente ejecutada y su entorno, aunque avance la rama que los custodia.

## 3. Terminología y límites de la terna

| Término | Significado en este marco | Distinción que debe conservarse |
|---|---|---|
| Perfil fuente | Selección explícita de las formas constitutivas de una unidad de código, como SVP-ES o SVP-EN | No es una nueva semántica ni traduce identificadores, cadenas o datos de dominio |
| Perfil léxico de identificadores | Repertorio y reglas léxicas aplicables a identificadores y naturales | No es el perfil fuente de palabras constitutivas |
| Perfil o constitución de dominio | Conocimiento delimitado y constituido por la unidad competente: parámetros, células, asignaciones, relaciones, fuentes, reglas y límites | No es un catálogo de conectores ni una traducción del vocabulario fuente |
| Perfil de soporte tecnológico | Contrato de capacidades y condiciones materiales de una realización: interfaces, dependencias, recursos, aislamiento, persistencia, transporte y fallos en el alcance declarado | No es el conocimiento del dominio ni una marca de plataforma presentada como garantía |
| Contrato del agente | Identidad, cobertura recibida, operaciones, capacidades requeridas y permisos del agente | Dominio y agente no se identifican por defecto; capacidad disponible no equivale a autorización |
| Representación de información de dominio | Codificación o transformación que conserva determinadas distinciones y puede perder otras | Su suficiencia por operación, tratada en FFL-E, no se deduce del idioma del código |

La terna organiza tres cuestiones: cómo se representa formalmente lo declarado, de qué conocimiento se trata y bajo qué medios materiales puede operar. No establece tres capas de autoridad intercambiables ni un algoritmo único de ensamblaje. Su cardinalidad no tiene una relación constitutiva con los tres valores del alfabeto SV.

Un agente especializado en infraestructuras médicas tiene un dominio de conocimiento técnico y, además, utiliza un soporte tecnológico. Un agente inmunológico puede utilizar medios semejantes con otra cobertura y otros permisos. El conocimiento sobre una tecnología no concede por sí mismo facultad para actuar sobre ella.

## 4. Identidad, versión y contrato de cada componente

Todo perfil que se pretenda admitir deberá quedar ligado a una declaración explícita que permita determinar:

- quién constituye su significado y alcance y cuál es la fuente autorizada;
- qué identidad y versión exactas se utilizan;
- qué requisitos impone y qué capacidades o compromisos ofrece;
- qué referencias a otros perfiles, contratos y realizaciones necesita;
- qué condiciones de compatibilidad, transformación y composición admite;
- qué evidencia respalda cada obligación y qué exclusiones conserva;
- cómo se diagnostica una ausencia, contradicción o incompatibilidad.

La identidad documental permite localizar una pieza; su contrato y su evidencia permiten juzgar el encaje. Una huella acredita la identidad de determinados bytes, no la legitimidad del contenido ni la voluntad humana. Las referencias deberán resolverse contra la fuente y versión autorizadas, con comprobaciones en la sede competente.

El contrato de ensamblaje deberá identificar las versiones efectivamente enlazadas y las ligaduras entre ellas. No se habilita la selección implícita de la última versión, la ampliación de permisos, la equivalencia por nombres ni la sustitución silenciosa de una realización por otra.

La sede de estas declaraciones —IR, meta-IR, manifiesto externo enlazado o combinación tipada— continúa sujeta a decisión propia conforme a los Pilares. Esta acta no crea un tipo denominado PerfilTecnologico ni autoriza a simularlo con campos opacos.

## 5. Tres ensamblajes con contratos distintos

| Operación | Qué debe conservar o comprobar | Alcance acreditado y pendiente |
|---|---|---|
| Ensamblaje multifuente | Bytes originales, nombre y perfil de cada unidad; frontera de fin de archivo; convergencia canónica y validación global; identidad y orden según el contrato aplicable | ES/EN tiene especificación y realización. Las decisiones pendientes de K2 conservan su sede |
| Composición de dominios y agentes | Constituciones recibidas, cobertura, relaciones, tipos, operaciones y autoridad; suficiencia de la información transmitida | No se acredita por ensamblar archivos. Cobertura parcial o multidominio y composición de agentes requieren representación y validación propias |
| Ensamblaje tecnológico | Componentes y conexiones explícitos; correspondencia entre necesidades y capacidades; restricciones conjuntas, permisos, diagnósticos, recursos y fallos | Los prototipos existentes aportan piezas ensayadas. Su composición completa con R1 y contratos de dominio/agente sigue pendiente |

El primer ensamblaje se rige por la [especificación normativa de perfiles fuente, especialmente §§6–10](../../ESPECIFICACION_NORMATIVA_PERFILES_FUENTE_SVP_ES_EN_v1_2026_08_29.md). La composición de dominios y agentes conserva la [transición secuencial desde OP-IMM-001](../dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md) y la [secuencia N0 del corte candidato](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/ad8e8dd30930e35b75bf5f2fad78938d36233b78/docs/arquitectura/N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md).

Los contratos no permiten sumar las garantías de realizaciones separadas para declarar seguro el conjunto. Una composición debe conservar todas las obligaciones aplicables y demostrar el comportamiento de sus fronteras. Tampoco se presume que las necesidades o resultados de Inmunología sean válidos para Ciberseguridad: ambos frentes se contrastan singularmente en su secuencia.

## 6. Qué debe recibir el Lenguaje antes de su cierre

El frente del Lenguaje debe recibir requisitos verificables, ligados a operaciones y casos constituidos. La expresión «dar a conocer al núcleo lo que necesita» significa establecer qué información, relaciones y obligaciones deben sobrevivir al análisis, descenso a IR, validación, serialización, ensamblaje y ejecución que se pretenda acreditar.

| Obligación para la suficiencia del Lenguaje | Qué deberá poder representar, preservar o comprobar | Qué no se transfiere al núcleo |
|---|---|---|
| Identidad y procedencia contractual | Referencias resueltas, versiones, fuente autorizada y ligadura con unidades, objetos y operaciones | La mera cadena nominal ni la decisión externa de quién posee autoridad |
| Constitución de dominio | Identidad y orden de parámetros y posiciones; b declarado y n derivado o comprobado; relaciones y reglas constituidas | Elección de células, b, reparto de parámetros o completado del dominio |
| Cobertura y facultades del agente | Correspondencia explícita con la constitución recibida, operaciones admitidas y límites verificables | Cobertura inferida, creación de un agente multidominio por reunir archivos o permisos implícitos |
| Firmas y resultados de operaciones | Tipos de entrada y salida; pertenencia a codominios; distinción entre salida terminal y Tri; procedencia de resultados | Conversión de un codominio terminal a Tri sin transducción constituida |
| Suficiencia representacional por operación | Transformación declarada, pérdida, requisito de información y evidencia de recuperabilidad o testigo de pérdida cuando proceda | Reconstrucción heurística de distinciones eliminadas o insuficiencia convertida en U |
| Ensamblaje y transformaciones | Orden, nombres, procedencia por objeto, versiones y relaciones que cada contrato exija conservar | Reordenación, normalización o reparación no constituidas |
| Requisitos tecnológicos relevantes | Referencias y condiciones que afecten a la admisión o legitimidad de la operación, con sede de comprobación identificada | Drivers, protocolos concretos, vigilancia general del bus o políticas de plataforma dentro de la semántica universal |
| Autoridad y ejercicio de efectos | Ligaduras constituidas de decisión, permiso, mediación, ejercicio, objeto y contexto en su alcance representable | Firma de transporte usada como sustituto de la procedencia gobernada |
| Diagnóstico y estados materiales | Distinción entre rechazo, fallo técnico, indisponibilidad, estado material no confirmado y resultado semántico válido | Un timeout transformado en cancelación, ausencia de efecto o Tri.U |

Una obligación no queda satisfecha porque aparezca como campo en un documento o como cadena en la IR. Debe existir una representación con significado definido, una sede capaz de comprobarla y evidencia suficiente para la afirmación efectuada. El ámbito formal del Lenguaje y la imposición material de una condición se conservan separados y enlazados.

El núcleo puede comprobar requisitos representados; la frontera y el anfitrión deben imponer las condiciones materiales que les corresponden. Un contrato que declare aislamiento necesita una realización que lo imponga. La validación de su declaración no equivale a demostrar el aislamiento del sistema operativo o del hardware.

## 7. Cómo decidir si una necesidad exige modificar la semántica o la IR

Para cada necesidad se conservará el siguiente expediente mínimo, reutilizando los requisitos y casos ya existentes:

1. Identificar dominio, agente u operación, fuente competente y versiones.
2. Describir el dato, relación, restricción o distinción que la operación necesita conservar.
3. Mostrar su recorrido por las representaciones e interfaces aplicables y localizar la pérdida o ausencia concreta.
4. Aportar un caso positivo y un contraejemplo discriminante; para pérdidas de dominio, utilizar estados acreditados como realizables cuando lo exija FFL-E.
5. Determinar la sede del remedio: perfil fuente, constitución de dominio, contrato del agente, representación del Lenguaje, operación constituida, frontera o realización tecnológica.
6. Justificar cualquier pretensión transversal mediante los contrastes previstos, preservando las competencias de cada dominio.
7. Fijar el cambio mínimo, su versión, diagnóstico y regresiones afectadas, o declarar la exclusión del alcance que se consolida.

Si la representación y la semántica actuales ya conservan todo lo requerido, la necesidad podrá resolverse en el perfil o en su realización sin ampliar el núcleo. Si no lo conservan, se documentará la insuficiencia antes de proponer nuevos tipos o reglas. No se obliga a mantener inmutable una IR insuficiente; tampoco se la amplía por anticipación de escenarios no constituidos.

Una obligación necesaria para una operación incluida en el cierre no puede declararse deuda abierta y, simultáneamente, dar esa operación por admitida. Una capacidad futura excluida puede quedar fuera del cierre, identificada y sin promesa de funcionamiento. El cierre se refiere a una versión, un perímetro operativo y unas garantías demostradas.

## 8. Suficiencia informativa: el soporte también puede alterar lo disponible

El soporte no adquiere autoridad sobre el conocimiento por transportarlo. Sin embargo, una interfaz, una reducción o un ensamblaje pueden perder distinciones necesarias para una operación. Esa pérdida debe hacerse visible y evaluarse conforme al [contrato de suficiencia representacional, §§4–12 y 15–16](../arquitectura/CONTRATO_MINIMO_DE_SUFIENCIA_REPRESENTACIONAL_POR_OPERACION_PARA_EL_LENGUAJE_SV_2026_08_21.md).

En ese contrato, una transformación con pérdida puede ser suficiente para una operación y no para otra. La igualdad de formatos o la presencia de todos los valores no prueba recuperabilidad exacta. Si la recuperación utiliza información adicional, esa dependencia debe declararse; no puede atribuirse al mensaje una información que no transmite.

Esta distinción impide confundir la traducción de formas constitutivas ES/EN con una transformación de información de dominio. La primera conserva la identidad canónica según su especificación. La segunda necesita su propio contrato de significado, pérdida y suficiencia. Los objetos conceptuales de FFL-E no se convierten por esta acta en tipos ejecutables nuevos.

## 9. Prueba previa de los perfiles de soporte tecnológico

**Toda realización de un perfil de soporte tecnológico deberá someterse previamente a ensayo en el laboratorio antes de promoverse como soporte admitido para un alcance operacional del SV.** La exigencia se aplica a las capacidades y garantías que se pretendan utilizar, incluidas sus conexiones cuando se afirme una propiedad del conjunto.

La preparación documental debe preceder a la prueba. Cada expediente identificará contrato candidato, versión de la realización, capacidades requeridas, actor adversarial cuando corresponda, condiciones de aceptación, observadores y presupuesto operacional aplicable. Las magnitudes todavía no aprobadas permanecen pendientes; no se inventan umbrales para declarar viable un resultado.

La recepción del ensayo deberá conservar:

- fuentes, dependencias, artefactos y entorno exactos;
- correspondencia entre obligación, mecanismo, caso, comando, resultado y límite;
- pruebas positivas y negativas o controles de sensibilidad pertinentes;
- separación entre dato semántico, fallo técnico y ausencia de observación;
- medidas de latencia, CPU, memoria, almacenamiento y comportamiento ante fallo que sean necesarias para el alcance, con sus límites de interpretación;
- resultados desfavorables y bloqueos, sin convertirlos retrospectivamente en éxito;
- informe y evidencia custodiados, más la proyección pública expresamente preparada.

La evidencia previa sigue siendo utilizable cuando corresponde a la misma obligación, realización y condiciones relevantes. No se repite una campaña por renombrar un perfil o añadir un enlace documental. Un cambio material requiere identificar qué evidencia deja de cubrirlo y repetir o ampliar únicamente lo afectado, además de las puertas exigibles.

Un ensayo correcto no promueve automáticamente un perfil ni constituye seguridad general, aptitud clínica o independencia de todos los proveedores. Calidad recibe el alcance acreditado y las brechas; la decisión de promoción conserva su autoridad y trazabilidad. PT-SV-LOCAL permanece candidata documental mientras su realización integrada y sus obligaciones necesarias no estén acreditadas.

## 10. Laboratorio y evidencia reutilizable: rutas de continuidad

La sede experimental es la carpeta [laboratorio-de-infraestructura-SV del repositorio privado](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/tree/e97fed715ff5e3ae19bbaccaf2852e9cd3288377/laboratorio-de-infraestructura-SV). Los enlaces privados permiten localizar la evidencia y conservan sus restricciones de acceso. La proyección pública se encuentra en [Documentación de laboratorios](https://juantoniolloretegea.github.io/SVcustos-dataset/laboratorio-de-infraestructura-SV/documentacion/).

| Pieza de continuidad | Qué aporta | Límite que se conserva |
|---|---|---|
| [Contrato experimental 0.2](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/e97fed715ff5e3ae19bbaccaf2852e9cd3288377/laboratorio-de-infraestructura-SV/CONTRATO_EXPERIMENTAL_0_2.md) y [mapa de pruebas](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/e97fed715ff5e3ae19bbaccaf2852e9cd3288377/laboratorio-de-infraestructura-SV/MAPA_PROTOTIPO_0_2.md), privados | Estados, ligaduras, cancelación, control local, límites y oráculos | Mensaje experimental, efecto sintético; no ABI SV ni autoridad R1 |
| [Registro 012, privado](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/e97fed715ff5e3ae19bbaccaf2852e9cd3288377/laboratorio-de-infraestructura-SV/registros/012-CORRECCION_VERIFICADA_2026_09_06.md) | Resultado identificado de la repetición y custodia de los testigos | Las propiedades del prototipo no se atribuyen al núcleo |
| [Registro 016, privado](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/e97fed715ff5e3ae19bbaccaf2852e9cd3288377/laboratorio-de-infraestructura-SV/registros/016-VIGILANCIA_Y_COSTE_2026_09_06.md) y [ficha pública](https://juantoniolloretegea.github.io/SVcustos-dataset/laboratorio-de-infraestructura-SV/documentacion/vigilancia-y-coste.html) | Validez frente a identidad, coste de una/dos compuertas y corrección de la API pública R1 | Compilación y fixture sintética; no ejercicio de R1 ni descarte universal de esclusas |
| [Registro 018, privado](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/e97fed715ff5e3ae19bbaccaf2852e9cd3288377/laboratorio-de-infraestructura-SV/registros/018-COMPARACION_DOTNET_FFI_WASM_2026_09_06.md) y [ficha pública](https://juantoniolloretegea.github.io/SVcustos-dataset/laboratorio-de-infraestructura-SV/documentacion/ffi-dotnet-wasm.html) | Enlaces reales desde .NET a la misma fuente Rust, FFI y WASM, paridad y costes observados | Perfil EN y compilación/serialización; no ensamblaje ni contratos completos; anfitrión comprometido fuera de protección acreditada |
| [Correspondencia CS01–CS09, privado](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/e97fed715ff5e3ae19bbaccaf2852e9cd3288377/laboratorio-de-infraestructura-SV/CORRESPONDENCIA_PROTOCOLO_Y_CONTRATOS_SV_2026_09_06.md) | Criterios de enlace ya especificados; adenda 016 corrige la denominación de la API pública | Su ejecución de enlace permanece pendiente |
| [Matriz 020, privada](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/e97fed715ff5e3ae19bbaccaf2852e9cd3288377/laboratorio-de-infraestructura-SV/registros/020-matriz-obligaciones.csv) y [fuentes identificadas](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/e97fed715ff5e3ae19bbaccaf2852e9cd3288377/laboratorio-de-infraestructura-SV/registros/020-fuentes.json) | Correspondencia documental PT01–PT14 y base reutilizable | IDs documentales, no tipos de DSL/IR ni nueva ejecución |

Los paquetes, comandos, resultados brutos y manifiestos se consultan desde sus registros originales. Permanecen en la sede experimental; esta acta no copia su contenido ni convierte el historial del laboratorio en una especificación soberana.

## 11. Ejemplos de aplicación sin ampliaciones implícitas

**Idioma y ensamblaje.** Una unidad SVP-ES y otra SVP-EN pueden converger según la especificación vigente y conservar sus procedencias individuales. Un futuro perfil italiano o portugués deberá demostrar la correspondencia canónica, unicidad, aislamiento y conservación exigidos. Su posibilidad arquitectónica no equivale a disponibilidad actual ni autoriza a traducir datos del dominio.

**Dominio y suficiencia.** OP-IMM-001 es el caso director ya constituido para contrastar necesidades, pérdidas y responsabilidades. Su inventario de 27 parámetros no determina por sí solo tamaños de célula ni tipos universales de IR. El Lenguaje recibe sus requisitos mediante el [acta de transición y sus documentos de valoración enlazados](../dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md). Ciberseguridad inteligente actúa después como falsador heterogéneo; no hereda automáticamente una solución inmunológica.

**Soporte y realización.** La comparación 018 conserva la misma fuente Rust al invocarla desde .NET por dos rutas. Demuestra el enlace y las propiedades observadas de esa compilación, pero no demuestra que un agente clínico o de infraestructura ya pueda ejercer su contrato sobre cualquiera de ellas. Las obligaciones faltantes se leen en 020 y CS01–CS09.

**Autoridad y transporte.** R1 está cerrado en su alcance intraproceso, según su [acta de cierre](../arquitectura/ACTA_TECNICA_CIERRE_R1_2026_08_25.md). El enlace futuro debe utilizar la vía pública trazada indicada por la adenda 016: decide_permit_traced, TracedPermitDecision::Granted, mediate_traced_permit, execute_traced_mediated y EffectExecutor::execute, bajo ProtectedDecisionContinuity. Una firma de transporte, deduplicación o inaccesibilidad de un constructor Rust no demuestra por sí sola procedencia gobernada ante un receptor remoto.

## 12. Efecto sobre la secuencia y el cierre nuclear

Se conserva la secuencia de continuidad recogida en N0, el relevo desde OP-IMM-001 y el registro 020: **PR61/N0-01; reparación de oráculos; K1 comenzando por N0-02; F y F-IF; retorno G/H de Inmunología e incorporación justificada; falsación I/J de Ciberseguridad; puerta algebraica Rust; comprobación IMM/CYB; K2; contrato operacional de frontera; consolidación**. La composición material de agentes conserva su puerta posterior o su exclusión explícita.

El trabajo tecnológico aporta evidencia y requisitos a esa secuencia. No desplaza las correcciones intrínsecas por una campaña general nueva. F/F-IF recibe necesidades informativas y pérdidas; la puerta algebraica recibe operaciones constituidas y preserva K1-T antes de acreditar producción mediante Ternarizer; K2 resuelve sus decisiones de identidad, procedencia y ensamblaje; la frontera operacional recibe las obligaciones materiales y sus pruebas.

Antes del cierre del alcance elegido deberá existir una correspondencia verificable entre las operaciones admitidas, sus requisitos de representación y semántica, los perfiles y contratos aplicables, la realización disponible y los resultados de aceptación. Las brechas incompatibles con ese alcance bloquean su cierre. Las capacidades excluidas quedan identificadas y no se anuncian como disponibles.

## 13. Regla de relevo y visibilidad documental

Toda unidad que continúe el trabajo deberá identificar el corte vigente, leer los Pilares, esta acta y el acta de fase aplicable, y consultar los registros y ejemplos a los que remiten. Distinguirá expresamente fundamento, obligación, especificación, realización, prueba y límite. Reutilizará el artefacto y el oráculo existentes antes de proponer sustituciones.

Esta acta tendrá entrada destacada en Calidad y vínculo explícito desde Documentación de laboratorios. El vínculo público actúa como referencia a la sede canónica; no mantiene una segunda copia normativa. La candidata y los registros experimentales conservan sus identidades y su función de evidencia.

Una nueva unidad no podrá dar por cerrada una capacidad porque encuentre un perfil nombrado, un campo presente, una compilación correcta o una ficha publicada. Tampoco reiniciará la arquitectura ignorando las decisiones, pruebas y restricciones conservadas en los enlaces.

## 14. Estado y alcance de la decisión

Queda constituido este encuadre de perfiles, contratos y ensamblajes y la exigencia de prueba previa de las realizaciones tecnológicas. Se mantienen sin cambio de realización la gramática, la IR, los perfiles ES/EN, el núcleo Rust y los prototipos. No se selecciona host, ABI, bus ni catálogo de conectores, no se constituye un nuevo dominio o agente y no se acredita integración clínica ni garantía general.

La nueva entrada RETP-075 se refiere a este acto documental. RETP-074 conserva su asignación a N0-01 en la PR #61 pendiente de integración; no se importa ni se da por integrado su cambio funcional mediante esta acta. El estado R0/R1/R2 y las garantías conservan sus registros propios.

El resultado que debe recibir la continuación es concreto: **una obligación de suficiencia y trazabilidad antes de cerrar el alcance nuclear, alimentada por dominios constituidos y realizaciones tecnológicas previamente ensayadas, con ampliaciones de perfiles, contratos o representación sometidas a demostración y versionado**.
