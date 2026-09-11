# Recepción separada A/V · IE004-RECEPCION-AV/1

**RETP-2026-136 · 11/09/2026 · candidata de laboratorio.** Desarrolla el objeto de RETP-135. Conserva `IE004-ES-P2/3-COSTE/1`, `K-IE004/1` y `P-IE004/1`. No abre la reserva ni acredita P4/P5.

## 1. Solicitud A y montaje confiable

El ejecutable de recepción, compilado sin `controles_publicos`, recibe por stdin una única solicitud JSON UTF-8, con exactamente `version`, `id`, `pregunta`, `contexto`. La versión es `IE004-A-SOLICITUD/1`. El identificador es `PUBLICO-` seguido de mayúsculas ASCII, dígitos o guiones, máximo 32 bytes, o uno de `P3-01`…`P3-24`. Admitir la forma de un identificador P3 no constituye un encargo ni apertura de reserva.

`pregunta` conserva el texto decodificado, máximo 8.192 bytes y 128 tokens. `contexto` tiene exactamente `operacion`, `objeto`, `parametro`, `momento`, `campo`, cada uno nulo o un nombre cerrado: LEER/ESCRIBIR; CASO-A/CASO-B; IGG/IGA/IGM; ACTUAL/ANTERIOR; VALOR/UNIDAD/ESTADO/FUENTE/ALCANCE. Los bits internos son los de la realización anterior. La decodificación JSON de escapes y pares sustitutos válidos recupera la cadena: no hace normalización lingüística adicional. Se rechazan duplicados incluso cuando una clave usa escapes, claves adicionales, enteros fuera de u64, UTF-8 inválido y contenido posterior al objeto.

Perfil/base/política y su código son constantes del montaje `M-IE004-AV/1`. El conductor confiable puede seleccionar la instantánea de vigencia ordinaria o `revocado` mediante argumento de proceso. V no puede seleccionar ese argumento ni introducir campos de montaje en A. Se analiza íntegramente el significado antes de la única puerta de política.

El propietario de los bytes recibidos y del árbol JSON vive durante la resolución; la solicitud toma préstamos de lectura de sus cadenas. No hay estructura autorreferencial. La gramática vive durante el préstamo del motor. Los cuerpos y trazas devueltos poseen sus bytes y sobreviven al fin de esos préstamos.

## 2. Formato de entrega de A

El cuerpo `IE004-A-CUERPO/1` es JSON de orden fijo con salto LF final. Contiene montaje, perfil, base, política, fijación de fuente, vigencia, estado de admisión, contexto y resolución. Ésta conserva estado, literal, ruta, causas, llamadas a política, fuente y alcance. El identificador y texto original van en la traza; no se incluyen en el cuerpo para impedir que dos paráfrasis equivalentes resulten diferentes sólo por sus bytes originales. Las comparaciones de cuerpos exigen iguales versiones, contexto y vigencia. Con distinta vigencia se comparan además significado y llamadas a política, sin borrar el cambio de montaje.

El marco binario de entrega es: ocho bytes ASCII `SVAC0001`, longitud del cuerpo u64 little-endian, cuerpo completo y SHA-256 de sus bytes (32 bytes). Máximo: 4.096 bytes de cuerpo y 4.144 de marco. No hay bytes posteriores. `validar` comprueba longitud, huella, JSON y versión antes de devolver el cuerpo. El conductor exige también salida de proceso satisfactoria; sólo entonces deposita mediante renombrado el archivo completo y registra `CUERPO_A_ENTREGADO`. Un prefijo de marco nunca cuenta como dato entregado. SHA-256 comprueba integridad, no autenticidad frente a un host comprometido.

La traza base se construye completa y se escribe antes del marco. Conserva original, identidad del transporte, normalización, intervalos de tokens, significados finales completos cuando los hay, resolución y contadores. No reproduce el bosque interno íntegro de RETP-135; las fuentes y aquella evidencia histórica permanecen recuperables. Un fallo de admisión registra su capa y no inicia el motor. El original malformado se conserva en los archivos de la campaña; una huella no sustituye esos bytes.

## 3. Sobre y certificado V

V llega por otro proceso e input. `v` recibe una copia de la solicitud confiable por un argumento construido por el conductor, la huella del lote y la del cuerpo A ya entregado; sólo la propuesta usa stdin. La copia tiene el mismo máximo de 65.536 bytes y se decodifica bajo la cuenta V. No se preabren directorios en WASI, no se hereda entorno WASI y no hay red ni llamada a proveedor. La copia posee su vida independiente; no conserva punteros al proceso A.

El sobre JSON tiene exactamente `version: IE004-P3-V/1`, `id`, `solicitudes_sha256` y `certificado`. El certificado contiene exactamente `version: IE004-DERIVACION-EXP/1`, `perfil`, `fuente_fijada_sha256`, `pregunta_sha256`, `raiz`, `nodos`. Perfil y fijación deben ser los vigentes; no se interpreta un certificado /2 como /3.

Cada nodo tiene exactamente `regla`, `expresion`, `alternativa`, `inicio`, `fin`, `hijos`. Los enteros se reciben como u64 y se convierten de forma comprobada. `regla` es G01…G25 codificada 1…25. `expresion` remite a la [tabla fija de 422 expresiones](TABLA_EXPRESIONES.json), que declara propietario Gxx y transición. Las alternativas binarias se numeran 1/2; repetición usa 1 para vacía, 2 para paso; otras transiciones usan 0. Estas son alternativas de la codificación de expresiones, no una renumeración de las alternativas normativas de Gxx.

Los hijos preceden al padre, con cero, uno o dos enlaces según la transición; la raíz es el último nodo y todos deben ser alcanzables. Se rechazan ciclos, referencias hacia delante, nodos ajenos, regla/expresión/alternativa discordantes y composición de intervalos incorrecta. La raíz cubre todos los tokens. Las fronteras en bytes son B(0)=0, B(k)=inicio del token k para 0<k<T y B(T)=longitud original; conservan los separadores y extremos. Se exigen fronteras UTF-8. Hojas y dígitos se cotejan con el original bajo la conversión léxica fijada. No hay cita de otro texto, nodo comodín o recorte silencioso.

Un éxito se denomina `DERIVACION_SINTACTICA_COMPROBADA`: acredita esa derivación completa, no unicidad semántica, bondad del consejo ni autoría independiente. A ya calculó todas sus interpretaciones. V no contiene un campo de respuesta que pueda sustituir el literal, y el comprobador no llama a la política ni a un ejecutor de efectos. Su anexo `IE004-V-ANEXO/1` incluye correlación, estado, causa y recursos propios; nunca se fusiona con el cuerpo A.

## 4. Límites y contabilidad

| Parte | Límites candidatos y observables |
| --- | --- |
| Transporte A | 65.536 bytes antes de acumular; sólo se observa un byte adicional para detectar exceso. Admisión JSON: 1 MiB de reservas acumuladas, 2.000.000 unidades de recepción/decodificación |
| Motor A | Sin cambios: 1.000.000 unidades /3, 32 MiB de buffers agregados, 16.384 estados, 8.192 bytes, 128 tokens, profundidad 512 |
| Exportación A | Cuerpo 4.096; marco 4.144; traza hasta 65.536 + 1.024 por significado final, máximo 16.842.752 bytes. Son buffers adicionales explícitos; no se presentan como incluidos en los 32 MiB del motor |
| V | Propuesta 262.144 bytes; observación de un byte extra sin acumularlo; 512 nodos, 1.024 enlaces, profundidad de grafo 64; 4 MiB de reservas acumuladas, 1.000.000 unidades |
| JSON | Profundidad máxima 96; 8.192 valores; objetos hasta 32 claves. Las matrices `nodos` e `hijos` se detienen antes de decodificar el elemento 513 o el tercero, respectivamente |

Las cuentas no devuelven saldo por liberación; cada crecimiento carga conservadoramente la capacidad nueva completa y la coexistencia anterior. Se usan reservas fallibles y se comprueba capacidad antes de insertar. La arena V incluye copia de argumento, copia de admisión, JSON decodificado, nodos tipados, léxico propio y anexo/metadatos acotados. El léxico reutiliza código puro de la realización, con objetos y recursos nuevos: su cargo previo conservador es 8×bytes originales + 512, además del resto de inspecciones. No usa memo, arena ni resultados del motor A. Las transiciones, enlaces, campos y bytes inspeccionados pagan V; no son instrucciones de CPU.

La sobreasignación del asignador se comprueba después de reservar; esa comprobación no impone RSS. Gramática fija, pila, runtime, buffers del sistema y copias del conductor conservan medición pendiente en P4/P5. Los plazos son del conductor; no deciden significado ni Tri. Un timeout de V se registra aparte. No se promete contención frente a presión concurrente o host comprometido.

## 5. Campaña pública fijada

[CONTROLES_PUBLICOS.json](CONTROLES_PUBLICOS.json) fija 23 controles A, 25 V y tres relaciones, tres reproducciones Rust por control y configuración; más 22 comprobaciones de primitivas. Cuatro configuraciones: nativo/WASI, debug/release, Rust 1.98.0. El ejecutable ordinario excluye las entradas del corpus; éstas sólo se incorporan al binario de controles.

Nueve escenarios ejercen procesos y pipes reales: V ausente, válida, al máximo, JSON inválido, excesiva, con cupo agotado, mal correlacionada, demorada 50 ms y sin EOF. En el último se inyecta un plazo V de 150 ms; A ya está entregada. Los restantes procesos tienen 30 s y la compilación 120 s. Se preservan las invocaciones, tiempos, entradas, stdout/stderr, cuerpos y anexos. Verificar los archivos del banco antes de la campaña es una operación de custodia; el ejecutable A no necesita los archivos de V.

El éxito exige todos los esperados, paridad de cuerpos, conservación de originales y orden observable A-entregada antes de V-abierta. Se conserva completa la matriz prevista aunque aparezca un fallo; éste impide el verde. La corrección posterior, si necesaria, será una sucesión causal explícita y como máximo un ciclo; no se cambia un esperado para acomodarlo al programa.
