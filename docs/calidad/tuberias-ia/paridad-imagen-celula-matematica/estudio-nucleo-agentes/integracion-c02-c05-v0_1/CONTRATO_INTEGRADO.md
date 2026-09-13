# Contrato candidato del recorrido documental C02–C05 · 0.1

Juan Antonio Lloret Egea y Watson · S22 · RETP-2026-218.

## 1. Corte, finalidad y estatuto

Corte del Lenguaje: `18963fa275d2a29633e164d9a2ea5fe1c4f11216`; laboratorio: `3678ae6c2881d74b2d46da99c33775d6a2979108`, rama `lab/playground-sv-permanente`. Los rectores y contratos consultados están identificados por SHA256 en [FUENTES.json](FUENTES.json). Se conserva el workflow V2 y su secuencia BIS-02 → decisión de sedes BIS-03 → realización y pruebas posteriores.

Este incremento compromete un recorrido experimental común y 26 casos antes de su realización. Es una especificación de integración de los contratos C02–C05. Su cometido es comprobar en una futura ejecución la conservación entre fuente SVP, soporte admitido, identidad de la célula, descriptor geométrico exacto y entrega documental local. No acredita todavía esos comportamientos. Los 202 casos previos C02–C12 y los 24 escenarios originales conservan su identidad y estado.

El suceso es el hecho. Su representación matemática, documental o visual tiene identidad y alcance propios. Aquí se prepara un descriptor matemático; no se produce una imagen ni se acredita su interpretación por una IA. Los registros sintéticos no son constituciones operativas de inmunología ni de otro dominio.

## 2. Objetos y frontera de confianza

| Objeto | Función y custodia |
| --- | --- |
| SOLICITUDES.json | Datos no confiables presentados al futuro receptor. No contiene resultados esperados. |
| CONTEXTOS_CONFIABLES.json | Contexto de cada invocación custodiado por el conductor, fuera de la solicitud. Fija vínculo, fuente, soporte, representación y destino esperados. |
| REGISTRO_CONFIABLE.json | Referencias exactas de soporte, constitución sintética, vínculos, convenio y captores admitidos. |
| PLANES_DE_INYECCION.json | Estímulos futuros que el conductor introducirá en la frontera de entrega. No son capturas ni observaciones actuales. |
| ORACULOS.json | Resultados esperados previos, sin campo observado relleno. El sujeto no recibe este archivo. |
| activos/ | Bytes literales de fuente, estado, perfiles y geometrías. Su identidad incluye longitud y SHA256. |

El `id` del caso lo elige el conductor de pruebas; nunca permite al emisor escoger el contexto confiable con el que será juzgado. Los nombres `archivo` identifican activos del banco, no conceden lectura arbitraria del sistema de archivos. En la realización, el conductor cargará una vez cada activo, dentro de cuota, y entregará buffers inmutables. El receptor no resolverá rutas, URL, servicios o comandos contenidos en la solicitud.

Las longitudes y SHA declaradas por la petición se comprueban contra sus bytes recibidos, pero no confieren confianza por sí mismas. La comparación con el registro/contexto independiente se realiza en la guarda semántica correspondiente. El oráculo matemático procede del estado C03; el geométrico positivo procede de los descriptores previamente comprometidos en C04, no de la salida del receptor futuro.

## 3. Encaje de C02, C03 y C04

Primero se recibe un perfil fuente explícito y se compila la fuente mediante la entrada pública Rust que corresponda. El resultado real de compilación determina las dimensiones de **todas** las CellSpec de la unidad, incluidas las no seleccionadas. Una declaración `n` en JSON no sustituye la IR. El soporte v1 contiene {16,25}; v2 contiene {16,25,49}. Las referencias exactas no admiten promoción automática a la versión más reciente.

A continuación se comprueba el vínculo C03 completo: constitución, fuente, arquitectura, nodo, CellSpec, instancia, revisión, estado IR y componente. A/r1, B/r1 y A/r2 comparten bytes de estado; siguen siendo identidades diferentes. `r1` y `r2` son identificadores opacos, sin orden temporal inferido. Los estados usados son `S1`/`S2`, componente `updated_vector`.

C03 conserva como artefacto el **estado matemático**. La integración añade un enlace explícito desde ese estado al **descriptor geométrico C04**, y desde éste a la **entrega C05**. No sustituye el artefacto matemático de C03 por el geométrico. La IR y el estado testigo deberán concordar posicionalmente; el JSON independiente no hace de intérprete del SVP. LIG/0.1 y su `validate_bindings` existente se conservan donde sean aplicables; este montaje sintético no acredita una nueva ligadura de dominio ni suple el BindingContract canónico.

La fuente `dos-nodos-y-c7.svp` agrega una CellSpec C7 de b=7 a la fuente histórica. Declara nueva identidad de fuente y nuevo vínculo sintético en el registro. La célula seleccionada C1 sigue siendo b=4, n=16; C7 obliga a derivar también 49 de la IR. La constitución sintética de C1 no se amplía silenciosamente a una constitución productiva de C7. Esta variante ensaya la obligación de soporte de toda la unidad, no constituye C7 operativamente.

El descriptor conserva un vector plano y posicional: n=b², b≥3. Para posición i, radio Zero=1, One=2, U=3 y ángulo (i−1)/n vueltas. Debe conservar leyenda, orden, coeficientes exactos y aristas consecutivas con cierre n→1. El primer banco integrado usa n=16 y las transformaciones `identidad` y `superior-horario` ya fijadas en C04. No reduce el soporte general a esos dos casos.

I0205-09 presenta una geometría internamente coherente de otro vector y su SHA propio correcto. Debe alcanzar el contraste matemático y fallar allí. Comparar prematuramente el SHA geométrico esperado ocultaría la ausencia de esa guarda y no satisfaría el caso. Tras comprobar la geometría se exige también la identidad exacta del artefacto destinado a entrega. El caso de 8192 bytes tiene geometría equivalente y espacios finales explícitos: se entrega su buffer exacto, no una reserialización silenciosa.

## 4. Orden de validación y etiquetas experimentales

Estas etiquetas pertenecen sólo al banco. No son nuevas entradas del catálogo canónico de errores del Lenguaje.

| Orden / etiqueta | Comprobación exigida |
| --- | --- |
| R01 | Cuotas inclusivas y sumas verificadas antes de reservas grandes; longitud efectiva, no sólo longitud declarada. |
| P01 | Sobre JSON reconocible y perfil fuente explícito, válido y coincidente con el contexto. Sin autodetección. |
| P02 | Compilación de la fuente por entrada pública Rust y obtención real de IR. |
| S01 | Referencia de soporte existente y cuerpo exacto frente al registro; ausencia no se completa. |
| S02 | Todas las dimensiones derivadas de IR pertenecen a ese soporte. |
| I01 | Campos obligatorios del vínculo presentes; tipos y forma admitidos. |
| I02 | Vínculo completo resuelto y concordante con contexto confiable, incluida fuente real e IR. |
| I03 | Estado matemático íntegro y concordante con el registro y componente de IR. |
| A01 | Metadatos de entrega concuerdan con la operación documental, destino y contexto autorizados externamente. |
| M01 | Convenio, formato, dimensión y transformación requeridos; declaración de codificación y leyenda. |
| M02 | Geometría posicional exacta derivada del estado: posiciones, símbolos, radios, ángulos, coeficientes y cierre. |
| M03 | Identidad exacta del descriptor validado frente al artefacto esperado para esta entrega. |
| D01 | Existe captura final independiente; si falta, entrega no acreditada. |
| D02 | Captor admitido. |
| D03 | Ámbito, invocación, operación, consumidor y canal de captura concordantes. |
| D04 | Instancia y revisión capturadas concordantes. |
| D05 | Representación y transformación capturadas concordantes. |
| D06 | Buffer realmente capturado idéntico al descriptor validado. |

La comprobación completa de campos específicos se produce en su fase. El sobre inicial debe permitir conservar como ausentes `soporte`, `vinculo` o `perfil_fuente` hasta la guarda definida, sin rellenarlos. De otro modo I0205-22 podría ser rechazado por el decodificador de soporte antes de ensayar la fuente inválida. BIS-03 debe resolver esa precedencia al diseñar los tipos de recepción Rust; no puede cambiar el esperado para acomodar el parser elegido.

**D07** es la salida alternativa de fallo del mecanismo de entrega después del despacho y sin captura final. Precede a la clasificación ordinaria de ausencia D01 cuando el fallo está instrumentado. Resultado: `NO_ACREDITADO`, efecto desconocido, sin reintento automático. No significa inexistencia del efecto. Una captura errónea D02–D06 supone en estos estímulos una entrega local al receptor experimental; el rechazo posterior no borra ese hecho.

Para cada negativo debe demostrarse que pasan las guardas anteriores. Un error de compilación no cuenta como éxito de una prueba de identidad. Los casos combinados señalan expresamente su primera guarda. El contexto y la expectativa permanecen independientes del sujeto.

## 5. Recepción, evidencia y preservación

La futura frontera de entrega será un receptor documental de pruebas, local, con captura de los bytes que recibe y del contexto con que los recibe. El captor y el observador se instrumentarán fuera del validador sujeto. No se admite acreditar entrega copiando el plan de inyección a un campo `observado`. Comprobar un archivo y volverlo a abrir para entregar no basta: se usará el mismo buffer inmutable validado y se contrastará la captura final.

Un recibo favorable deberá identificar el caso, perfil fuente, hash de fuente, referencia de soporte, dimensiones derivadas, vínculo íntegro, hash del estado, convenio, transformación, hash y longitud geométricos y contexto final. El observador contrastará esos campos con el contexto confiable y con la captura. Un campo de éxito autorreferido no sustituye esa confrontación. La salida `ENTREGA_DOCUMENTAL_CONCORDANTE` acredita sólo lo que su nombre delimita bajo este contrato y la evidencia futura; no consumo visual ni comprensión de IA.

El estado inicial debe permanecer idéntico al finalizar cada caso, incluido el rechazo y el fallo posterior a despacho. `U` ya presente es un símbolo válido; no es resultado de error, autorización ni relleno. No se devuelve una célula nueva llena de U para encubrir un fallo. No se transforma `Bottom` en `Tri.U`.

La traza registrará solicitud literal, contexto seleccionado, versiones, hashes y contenido de ejecutables fuente/scripts/parches, órdenes invocadas, stdout/stderr, estados de salida, capturas y veredicto del observador. Servicios y conectores productivos usados: ninguno en la futura ruta local; cualquier ampliación deberá declararlos antes de ensayarse. La infraestructura administrativa de publicación se registra separadamente. No se exige extraer pensamiento interno de una IA para acreditar acciones externas.

I0205-17 trata una nota imperativa como bytes documentales. No se crea intérprete de notas ni filtro de palabras como protección. I0205-18 rechaza el cambio de operación explícita. La separación no acredita toda la autoridad de futuros agentes externos, scripts, conectores o efectos R1.

## 6. Recursos, perfiles y sensibilidades

[PRESUPUESTO.json](PRESUPUESTO.json) declara límites nuevos para este experimento. La cuota de geometría de 8192 bytes responde al descriptor histórico de 6237 bytes; no modifica retroactivamente el límite textual del banco C11. El cómputo agregado incluye los bytes de la solicitud individual serializada, fuente SVP, estado, soporte y geometría, contados por canal aunque haya referencia repetida. Los activos confiables del conductor se mantienen fuera de esa cuota de recepción y deberán medirse dentro de RAM del proceso. El tamaño JSON individual se fija con UTF-8, sangría 2, caracteres Unicode literales y LF final, según el verificador administrativo.

Los límites se contrastarán con aritmética comprobada y representabilidad `usize`, antes de asignación no acotada. Los tamaños de memoria y duración de proceso siguen sin constituirse: no se deducen del archivo ni habilitan operación productiva. Concurrencia 1, cero llamadas externas y cero reintentos. Las salidas también tienen cuota; excederla impedirá el recibo favorable y conservará los efectos ya observados. El banco inicial no cierra pruebas de agotamiento real de memoria ni todas las cuotas de salida.

El perfil SVP integrado es `en`. La paridad ES/EN conserva su banco C12 y queda pendiente en la integración; no se afirma ensayada. La documentación Rust de la futura realización mantendrá la política ES/EN existente, independiente del idioma de la interfaz humana.

La secuencia v1→v2→v1 se ejecutará en un mismo proceso con tres invocaciones separadas, reiniciando el contexto de cada una y conservando únicamente la caché cuya discriminación se ensaya. Los cuatro mutantes del observador fijados en BANCO_PREVIO.json deberán provocar rechazo. Son sensibilidades previstas; aún no ejecutadas.

## 7. Condiciones para el siguiente paso

El compromiso publica bytes, hashes, oráculos y límites. Sigue pendiente decidir la sede y la interfaz real en BIS-03: recepción tipada, derivación desde IR, servicio de soporte, enlace de identidad/revisión, representación y frontera de captura. Esa decisión deberá justificar si basta composición externa o exige cambios de semántica/IR. No se decide por anticipado mediante este JSON.

Después se realizará el receptor Rust y su observador, se compilarán con Cargo y se ejecutarán los casos comprometidos. Si un estímulo o esperado resulta erróneo se conservará esta versión, se documentará la causa y se comprometerá su corrección antes de repetir. Python aquí comprueba el expediente documental; no valida semántica SVP ni sustituye ejecución Rust.

BIS-02 y S22 siguen abiertos. No se declara apertura formal de BIS-03 ni cierre global de sus condiciones. Se conserva S24 pendiente: Bis → catálogo y cierre de fase → análisis e instalación de la GUI.
