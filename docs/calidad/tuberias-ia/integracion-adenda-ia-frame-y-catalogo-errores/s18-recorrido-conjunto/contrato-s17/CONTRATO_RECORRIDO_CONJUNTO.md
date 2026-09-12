# S17 — Contrato del recorrido documental conjunto

Estado: contrato candidato de laboratorio fijado, versión `S17-CONTRATO/1`. No se ha implementado ni ejecutado este recorrido conjunto. Los resultados de S11, S14 y S15 conservan sus alcances originales; no se suman como si fueran una prueba integrada.

## 1. Objeto y frontera

Enlazar, para un episodio documental acotado, contexto instalado, solicitud original, base efectivamente consumida por G1, propuesta recibida, comprobación y archivo recuperable. Se conserva la identidad y el parentesco de una reevaluación sin sustituir el episodio original. El banco usa exclusivamente los datos artificiales ya fijados en S14.

La IA sólo podrá proponer bytes. El instalador de laboratorio fija las referencias; el productor G1 y los comprobadores Rust deciden qué se admite. Un texto documental que ordene cambiar reglas sigue siendo dato. El experimento no mide la obediencia de un modelo real ni acredita resistencia general a prompt injection.

Este contrato no constituye facultades profesionales, dominio, célula, parámetro, perfil de agente, bus, puerto ni efecto R1. No produce observación → Tri. Ningún error técnico se traduce a U. No modifica semántica, IR 0.3, gramática ni serializador canónico. B/E/K/L y las puertas P3/P4/P5/P6 siguen abiertas en el alcance establecido por S16.

## 2. Pérdidas de interfaz que impiden el enlace directo

| Pieza comprobada | Interfaz material | Pérdida al intentar componer | Adaptación candidata mínima |
| --- | --- | --- | --- |
| S11/S2 | Contexto sobre Referencia de un EnlacePublico y lote fijo | No constituye referencia para una base nueva de S14 | Referencia de episodio instalada desde producción G1; conservar S2 intacto |
| S14 | Historia devuelve Vista con campos públicos; ejecuta A y no solicita V | Vista no es EntregaLiteral ni prueba de comprobación de entrega | Preparar la entrega G1 una vez por episodio y obtener el tipo que G1 valida |
| S14, carga de base | cargar devuelve Base válida o Fallo | El rechazo no devuelve los bytes leídos para el informe de custodia | Envolver o adaptar la carga conservando bytes acotados y causa, sin volver a leer para reconstruirlos |
| S15 | Recibir recibe Referencia S2; Informe devuelve bytes | No enlaza contexto/base dinámicos ni conserva el tipo que admite el escritor | Recepción sucesora versionada y resultado comprobado con constructor privado |
| S3/S11 | Escritor recibe cobertura::Entrega | No admite Informe.cuerpo sin perder la frontera de tipos | Escritor sucesor recibe exclusivamente EntregaComprobada; no constructor desde bytes arbitrarios |

Los pasajes de estas interfaces se conservan en PASAJES_INTERFACES.json. La adaptación crea una realización de laboratorio explícita; no altera hashes del lote S2, no convierte Vista en una credencial ni añade un constructor público al tipo anterior. La pérdida identificada es de composición tecnológica; este corte no justifica ampliar IR.

## 3. Referencias y producción

**Instalación.** El instalador recibe una solicitud exacta, contexto exacto y bytes requeridos de base. Carga el archivo de base con el límite y formato de S14. Debe comparar los bytes leídos con los requeridos antes de consumirlos. Un mismo nombre de archivo con bytes diferentes no es la misma base. Se conserva copia de los bytes efectivamente leídos, no sólo su ruta o huella.

**Consumo.** La vigencia se interpreta de esa Base validada y se pasa a Custodio::abrir; ejecutar_a y no_solicitar_v completan la ruta G1 heredada. No se deriva el recibo esperado mediante etiquetas preparadas por la IA. El banco exige que cambiar a la base negativa cambie la producción a los bytes negativos ya fijados.

**Entrega G1.** La realización sucesora conserva el marco recuperado del mismo manejador, llama a comprobar_entrega una vez para ese episodio y obtiene EntregaLiteral mediante entrega. Conserva los errores concretos de G1. No crea una segunda oportunidad oculta si falla esa comprobación. Copiar el marco para satisfacer préstamos de Rust es una operación de custodia; la comprobación sigue siendo la de G1.

**Referencia de episodio.** Vincula identidad G1, eventual padre, solicitud, base consumida, contexto instalado y EntregaLiteral. Los campos capaces de conferir admisión y sus constructores son privados. Las vistas públicas permiten inspección, nunca fabricación de una referencia admitida. Las identidades son locales al custodio de ensayo; no se proclaman autenticación de persona, proveedor ni unicidad global.

**Reevaluación.** Un custodio admite aquí un original y una reevaluación, como S14. La reevaluación exige la misma solicitud y una base distinta explícitamente instalada; produce identidad distinta y padre igual al original. Cada episodio conserva sus propios bytes. Recuperar el original después debe devolver el original, sin recomputarlo con la base nueva. Se conserva el contexto instalado para ambos episodios en este banco; variar también contexto sería otra prueba.

## 4. Transporte de ensayo SV17/1

Es un protocolo binario sintético, no un contrato de proveedor ni un formato SV canónico. Sustituye explícitamente a SV15 para este ensayo. Cabecera: cinco bytes ASCII `SV17` y byte 1. Tipo siguiente: byte ASCII N o R.

- Campo: longitud u32 big endian seguida exactamente de ese número de bytes. Nunca se normalizan UTF-8, espacios, saltos de línea ni cadenas.
- Campo opcional: bandera 0 sin campo, o bandera 1 seguida de campo. Otra bandera es inválida.
- N: un campo de motivo opaco. Debe terminar ahí. Una negativa no es un cuerpo admitido.
- R: dos u64 big endian para la identidad G1, campo contexto, campo cuerpo, campo opcional solicitud y campo opcional base, en ese orden. Debe terminar ahí.

Se lee hasta EOF con máximo de 16 384 bytes y un byte adicional para detectar exceso. Se conservan hasta 16 385 bytes recibidos. Un error de lectura conserva su ErrorKind y los bytes anteriores; impide decodificación y admisión incluso si esos bytes parecían un mensaje completo. No se interpreta un final de red defectuoso como EOF correcto. Longitudes se comprueban con aritmética segura antes de reservar/copiar; el emisor no controla una reserva arbitraria.

Una cabecera anterior SV15 no se interpreta como SV17. Se rechazan tipo, bandera, truncamiento y sobrante inválidos. El contexto instalado y propuesto está limitado a 8 192 bytes; el exceso se rechaza, no se recorta.

## 5. Orden de comprobación y custodia del tipo

El instalador prepara la referencia antes de consultar al emisor. Después de recepción completa y decodificación estricta se sigue este orden:

1. Negativa N: conservar motivo y finalizar sin admisión ni archivo.
2. Respuesta R: límite de contexto; identidad igual a la referencia elegida por el instalador; contexto idéntico al instalado.
3. Cuerpo: comprobar_presentacion de EntregaLiteral G1, con el perfil de presentación heredado, sin cambiar contenidos de cadenas ni orden de claves.
4. Solicitud citada: debe estar presente e igual byte a byte a la solicitud original del episodio.
5. Base citada: debe estar presente e igual byte a byte a la base consumida de ese episodio.
6. Sólo entonces producir EntregaComprobada de constructor privado, ligada a la referencia y a los bytes admitidos. Las vistas o registros no permiten reconstruirla desde fuera.

La cobertura nueva usa FaltaSolicitud/SolicitudDistinta y FaltaBase/BaseDistinta. Esos nombres no renombrarán retroactivamente FaltaCaso/FaltaVigencia de S2: allí se comprobaban filas del lote y del montaje; aquí se comprueban solicitud y base de un episodio producido. La correspondencia es de obligación, no de identidad de formatos.

La comparación de presentación del cuerpo conserva la tolerancia ya fijada fuera de cadenas. Contexto, solicitud y base exigen bytes idénticos. No se mezcla esta regla con la prueba externa S6 de citas literales.

## 6. Archivo y recuperación

El escritor sucesor sólo acepta EntregaComprobada. El destino lo fija el instalador; la IA no lo cambia. Usa creación exclusiva y escribe los bytes admitidos, conservando el resultado de apertura, escritura y flush por separado. Un destino existente se rechaza y sus bytes deben permanecer intactos.

Antes de escribir se puede afirmar que este escritor no actuó. Después de crear el archivo, un fallo de escritura o flush puede dejar bytes parciales: conservar etapa, ErrorKind, bytes observados y estado del archivo. No borrar, reintentar ni declarar ausencia de efecto automáticamente. La prueba de escritura parcial usa un escritor inyectable controlado; no acredita todas las fallas del sistema operativo.

La recuperación debe leer el archivo con límite y volver a comparar contenido con la referencia del episodio. Si después se sustituye por el cuerpo de otro episodio, se detectará el desacuerdo. La detección a posteriori no impide la sustitución. create_new y flush no acreditan persistencia tras caída, atomicidad, ausencia de symlinks en todo el entorno, autenticación del destinatario ni entrega profesional autorizada.

## 7. Diagnóstico observable

El informe conserva una causa estructurada y su etapa. DIAGNOSTICOS.json fija identificadores locales de S17 y rótulos ES/EN. Esos identificadores no son altas en el catálogo canónico. La misma causa alimenta el registro y la presentación; no se reinterpreta una cadena Debug para decidir el resultado.

Cada observación conserva: caso, modo, repetición, identidad/padre disponibles, referencias originales, entrada recibida y longitud, motivo N si existe, causa con detalle específico, acceso a cada puerta, presencia de EntregaComprobada, operación de archivo intentada y bytes finales observados. Para ausencia comprobada se usa el estado explícito ausente; null no significa indistintamente ausencia, fallo de lectura y no observación.

G1 y los errores de presentación se conservan como detalle tipado. Los errores de sistema conservan ErrorKind. Si aparece una variante no fijada en los esperados, detener y registrar; no agruparla como U ni ajustarla al esperado a posteriori. Un informe de recepción correcto no prueba actividad interna completa de un proveedor.

## 8. Banco y ejecución posterior

BANCO_OBLIGACIONES.json fija 24 casos y sus predicados. Los dos cuerpos esperados y la solicitud/bases proceden de la cápsula S14 intacta. Los nuevos contextos se entregan como bytes. No se regenerarán los esperados a partir de la realización sucesora. Los identificadores de episodio se comprobarán mediante relaciones de igualdad/desigualdad y parentesco, no con números inventados como oráculo.

La siguiente campaña deberá fijar y publicar antes de ejecutar: fuentes completas y hashes, compilador ya custodiado, codificación material de cada caso, número exacto de invocaciones, límites de tiempo y espacio, salidas previstas, cuatro mutantes y tres clientes de frontera de tipos. El presupuesto de ejecución no se declara cerrado mientras no exista esa realización. S17 fija obligaciones; no registra compilaciones inexistentes.

Plan de observación normal: los 24 casos en debug y release, tres ejecuciones por modo, 144 observaciones previstas. Sensibilidad prevista: omitir cotejo de contexto, forzar consumo positivo, omitir cotejo de base citada y convertir una negativa en admisión. Cada mutante debe compilar y un caso fijado debe detectarlo. Además, un cliente externo válido debe compilar y dos clientes que intenten fabricar referencia/entrega deben fallar por privacidad o incompatibilidad de tipo, no por un error ajeno al contrato.

Cualquier imposibilidad de obtener el enlace tipado por las interfaces existentes obliga a documentar la pérdida antes de cambiar el contrato. La presión del ensayo no autoriza a fabricar la prueba que se pretendía obtener. No hay reintentos libres ni reparación silenciosa de oráculos.

## 9. Cierre y relevo

Queda cerrado el contrato documental S17 y el banco de obligaciones. La conformidad ejecutable del recorrido permanece pendiente. El siguiente objeto único es realizar la adaptación mínima, fijar su campaña y ejecutar este banco con custodia íntegra. Tras ese resultado se podrá valorar la integración de los puntos 1 y 3 y el relevo al catálogo de errores. No se adelanta el cierre del núcleo, de inmunología ni la decisión sobre agentes.
