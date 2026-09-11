# Adaptador estricto de lote · IE004-LOTE-ADAPTADOR/1

**RETP-2026-138 · 11/09/2026 · preparación anterior a la campaña.** Continúa el objeto de RETP-137 autorizado por Juan Antonio. Corte Calidad `b5bb421a57d897faf003654fcaf9e5a7a120a1b0`; laboratorio `2885ed16ef2a578bd968ae2ff894c8dfcf7f417f`. AGENTS, Pilares, perfiles/ensamblaje, transición §§1–30 y arquitectura conservan las identidades de las lecturas completas anteriores. Se cotejaron de nuevo; se consultaron workflow, encargo de reserva, contrato de separación, compromiso público, recibo humano, /2 y correspondencia /3. Ningún original reservado se ha leído.

## Entrada y conservación

Se aplica exactamente `IE004-P3-TRANSPORTE/1`: raíz con sólo `version: IE004-P3-A/1` y `casos`; 24 objetos, P3-01…P3-24 una vez y en orden; sólo `id`, `pregunta`, `contexto`. El contexto contiene exactamente los cinco campos de IE-004, con valores cerrados o null. El orden de las claves JSON no tiene significado. Se rechazan duplicados incluso escapados, claves adicionales, tipos impropios, cardinalidad u orden incorrectos, UTF-8 inválido, sustitutos aislados y contenido posterior. El elemento 25 no se decodifica. La estructura es fija, sin recursión ni mapas extensibles.

La entrada es stdin. El único argumento es la SHA-256 esperada, obtenida del custodio/conductor confiable y comprobada sobre **todos los bytes recibidos antes de decodificar**. En la campaña corresponde al lote público fijado. En una futura captura corresponderá al compromiso y hito autorizados: el programa no autentica por sí solo al custodio ni decide esa asociación. Una huella que acompaña al atacante no constituye confianza. No acepta paths, URL, notas, propuestas ni oráculo.

Cada caso posee su pregunta y contexto. Los rangos de su objeto JSON se refieren al buffer de entrada vivo. La salida posee sus bytes; no devuelve préstamos a objetos destruidos. Los escapes JSON se decodifican; se conservan exactamente los valores Unicode resultantes, sin NFC/NFD, eliminación de acentos o espacios, compleción contextual o reinterpretación de nulos. Los bytes originales y sus rangos se custodian aparte: reserializar una solicitud no se presenta como identidad byte a byte del archivo original.

## Límites previos

| Objeto | Cota y aplicación |
| --- | --- |
| Lote recibido | 2 MiB; se observa a lo sumo un byte adicional para detectar exceso, sin acumularlo |
| Casos | Exactamente 24; capacidad reservada antes de insertar; el elemento 25 no se analiza |
| Pregunta | 8192 bytes decodificados, controlados antes de cada inserción; 128 tokens con el léxico heredado sin cambios |
| Claves / id / versión / valores | 64 / 5 / 32 / 32 bytes; claves y valores además cerrados por esquema |
| Preparación de lote | 24.000.000 unidades propias, 16 MiB de reservas acumuladas; no descuenta ni transfiere saldo a A o V |
| Solicitud individual | Buffer hasta 65536 bytes; se libera tras copiarla a la salida completa |
| Salida / traza | Payload hasta 2 MiB, marco hasta 2 MiB + 48 bytes; traza hasta 65536 bytes; reservas incluidas en la cuenta del lote |
| Proceso de ensayo | 30 s; compilación 120 s; captura stdout 2 MiB + 48, stderr 256 KiB. Son cotas del conductor, no latencias productivas |

Las reservas son fallibles; suma y multiplicación de dimensiones comprobadas. El acumulador no recupera saldo por liberación y carga la capacidad real tras reservar. El léxico se usa exclusivamente para contar la cota de tokens, con buffers nuevos y cargo conservador previo de `bytes + 128*sizeof(Token)` y `8*bytes + 1024` unidades. No hace análisis gramatical o semántico. Un texto combinante o un símbolo desconocido puede pasar el transporte y ser rechazado por el motor, conservando su original. `TOKENS_LIMITE` es fallo técnico del adaptador, no `Tri.U`.

La cuenta incluye recepción, verificación de identidad, decodificación, léxico de cota y serialización, con un cargo previo conservador de 65536 unidades para la traza. No son instrucciones CPU, RSS ni equivalentes a intentos /2. Pila, runtime, copias del conductor y presión concurrente mantienen medición y acreditación material pendientes. La preparación íntegra del lote se mide aparte; no se oculta como coste gratuito.

## Entrega y recorrido A

Tras admitir **todo** el lote se construye una salida JSON `IE004-LOTE-EXTRAIDO/1` con `solicitudes_sha256` del original y 24 solicitudes individuales `IE004-A-SOLICITUD/1`. El marco contiene `SVLT0001`, longitud u64 little-endian, payload y SHA-256 del payload. La traza guarda rangos de cada caso, huellas del objeto original/pregunta/solicitud individual, tokens y contadores. Se entrega completa antes del marco. Un error no produce prefijos de lote utilizables: stdout queda vacío antes de comenzar su entrega.

El conductor exige salida de proceso correcta, longitud exacta, ausencia de bytes posteriores y huella íntegra antes de extraer solicitudes. Una escritura parcial o proceso interrumpido no se acepta. Un defecto en cualquier caso invalida la admisión del lote de custodia completo; no se recortan objetos ni se sirven fragmentos como si el lote fuese válido. Después, cada solicitud se entrega a la interfaz A ya fijada de RETP-136/137; la resolución y el cuerpo no se modifican por el adaptador.

El recorrido A no recibe `ENTRADAS_AUXILIARES_P3.json`, ni descarga, extrae, enumera o abre paquetes V. No se implementa aquí su admisión: permanecen fuera de esta ruta. Las huellas de originales y el nombre de cada propuesta se ligarán al lote confiable en la captura autorizada. Las garantías funcionales de V permanecen en RETP-137, sin repetir su campaña ni declararlas aislamiento P4.

## Campaña cerrada

`CONTROLES_PUBLICOS.json` fija 36 controles, tres reproducciones por configuración nativo/WASI debug/release Rust 1.98.0. `LOTE_SINTETICO_PUBLICO.json` usa 24 posiciones y diez entradas públicas heredadas, con repeticiones declaradas. **No son 24 preguntas inéditas.** `PROCEDENCIA_Y_ESPERADOS.json` conserva las respuestas y sus huellas de RETP-137 fijadas antes de construir el adaptador.

Se contrastarán conservación de pregunta/contexto, rangos sobre bytes originales, integridad y paridad del marco; en el lote base, recorrido efectivo de las 24 solicitudes hasta los cuerpos A, tres veces por configuración. Son pruebas Rust con conductor Node de transporte/captura/comparación; no hay intérprete semántico Python, nuevo modelo ni referencia producida por el candidato para aceptarse a sí mismo. El testigo de 128 tokens se hereda sin ampliarlo. Los controles de escapes/sobrelongitud ejercen exclusivamente la frontera de transporte.

Todos los controles deben pasar y los cuerpos deben coincidir con las huellas anteriores. Se conserva completa una única matriz aunque haya un fallo. Un fallo impide conformidad; sólo cabe una corrección causal explícita con nueva fijación y preservación de la campaña fallida. No se modifica un esperado para acomodarlo a la implementación.

## Continuidad y puertas

La comprobación de tipos de la preparación terminó correctamente; no se había ejecutado ninguna consulta del adaptador. La campaña se inicia sólo después del depósito de esta fijación en Calidad y laboratorio. La matriz de compatibilidad y la decisión preparada forman parte de esta misma publicación. La reserva permanece cerrada; este adaptador no concede autorización de captura, no decide por el custodio, no cambia el compromiso ni acredita P4/P5.
