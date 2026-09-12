# S14 — Identidad de base consumida y recuperación/reevaluación

Contrato SV-S14-BASES-GJ/1. Laboratorio documental de la integración 1+3. Responsable: Watson / W-S0. Autorización expresa de Dirección después de S13 / RETP-182. Cortes de entrada y rectores en RECTORES.json y PROCEDENCIA.json. No modifica gramática, IR, núcleo productivo o fuentes G1 reutilizadas.

## Objeto y sede

G: impedir atribuir a una base fijada un resultado producido consumiendo otra bajo igual nombre. J: conservar el original con su base y distinguir una reevaluación con nueva base y relación explícita. Se recibe S13: este recorrido es documental y de custodia intraproceso; no es QueryResult nativo, transición SV ni historia durable.

Se reutiliza G1 directamente desde la cápsula S2 intacta. No se elimina la fijación de lote/montaje de EnlacePublico ni se afirma ensayar de nuevo la cobertura S2/S11 o el transporte S3. El añadido de laboratorio carga una base de vigencia desde la misma ruta `vigencia.s14`, conserva los bytes que ha leído y deriva exclusivamente de ellos el booleano que pasa a `Custodio::abrir`. La producción y validación del recibo sigue siendo G1. Las dos bases difieren en un byte: `vigente=1` / `vigente=0`. Esto no cambia la base semántica artificial K-IE004/1, política ni contexto de la consulta.

La referencia requerida está fijada independientemente del archivo de entrada; los errores distinguen identidad de carga, formato, límite, atribución, identidad de episodio y cuerpo. Los bytes conservados son los mismos que se interpretan, sin volver a abrir la ruta para decidir. No se atribuye autenticación a la mera igualdad: conductor, referencia, sistema de archivos y host son confiables en el alcance declarado.

Historia contiene un original y como máximo una reevaluación. Los manejadores y recibos permanecen en el mismo Custodio; la reevaluación conserva su padre y su propia base. `atribuir_original` compara identidad, base y cuerpo en ese orden. No hay una operación pública para reemplazar el episodio original. Rechazar nueva evaluación con igual base y limitar a dos actos son restricciones del ensayo, no leyes de una Trajectory.

## Oráculo anterior e independencia

Solicitud A01 y cuerpo positivo se recuperan literalmente de la cápsula anterior; el cuerpo negativo procede del espécimen P3-11 ya recibido y comprobado en S11. El contexto admitido y la consulta son los mismos para este contraste, mientras el identificador de transporte no pertenece al cuerpo. Los dos cuerpos completos se fijan antes de compilar. No se calculan esperados con la realización nueva, no se normalizan cadenas y ambos exigen una llamada de política. Los archivos de evidencia son capturas del conductor; no un servicio profesional de entrega.

## Casos fijados

| Caso | Variación / comprobación | Exigencia |
| --- | --- | --- |
| GJ01 | Base original leída desde ruta común; solicitud A01 | Cuerpo positivo anterior exacto, base original, sin padre |
| GJ02 | Archivo sustituido por base nueva bajo la misma ruta; referencia original | BaseDistinta antes de producir otro acto |
| GJ03 | Solicitud de reevaluación con base original | BaseSinCambio; original conservado |
| GJ04 | Nueva base requerida y cargada; misma solicitud | Cuerpo negativo anterior exacto; identidad distinta; padre=original |
| GJ05 | Recuperar original después de reevaluar | Cuerpo positivo y base original intactos |
| GJ06 | Identidad y cuerpo originales; atribuir base nueva | BaseDistinta |
| GJ07 | Identidad de la reevaluación presentada como original | Identidad |
| GJ08 | Identidad y base originales; cuerpo negativo nuevo | ReciboDistinto |
| GJ09 | Recuperar reevaluación con su relación | Cuerpo negativo, base nueva y padre originales intactos |
| GJ10 | Intentar añadir otra reevaluación tras ocupar las dos posiciones | Capacidad |
| GJ11 | Base requerida y cargada coinciden, pero contienen `vigente=U` | FormatoBase; no se usa U como fallo |
| GJ12 | Base de 65 bytes | LimiteBase; ninguna ampliación automática |

GJ01/GJ04 conservan solicitud, productor, política y fuente; cambia vigencia. GJ01/GJ02 conserva referencia pero cambia archivo cargado. GJ05/GJ06 conserva identidad y cuerpo, cambia sólo base atribuida. El cotejo final vuelve a recuperar el original después de todos los rechazos. Se capturan las bases requeridas, cargadas y conservadas; solicitud, marco, traza, cuerpo, identidad, padre y atribuciones presentadas. Las capturas del archivo cargado se hacen en el conductor secuencial antes de la lectura del receptor; no acreditan ausencia de una carrera bajo otro actor hostil.

## Sensibilidad, presupuesto y parada

Rust 1.98.0 Linux x86_64; SHA-256 del compilador `3690cc576ede140504698405d5d8fa3826aaadbe71699c6c4ed0a565d6f493e2`, igual a S11. Debug opt-level=0/overflow-checks=yes y release opt-level=3/overflow-checks=no. Tres ejecuciones por modo; doce casos, 72 observaciones normales. Comparación literal de stdout y de todas las capturas entre las seis ejecuciones.

Tres controles instrumentales con una guarda o vínculo desactivado cada uno, compilados con la misma biblioteca G1 normal:

- `sin_identidad_carga`: omite comparación de archivo con referencia. Debe fallar GJ02, exit 1.
- `consumo_falso`: declara conservar la base pero pasa siempre vigente=true a G1. Debe fallar GJ04, exit 1. Comprueba que el cambio llega al productor y al cuerpo, no sólo al sobre de trazabilidad.
- `sin_base_original`: omite sólo comparación de base al atribuir original. Debe fallar GJ06, exit 1.

Errores de compilación no cuentan como detecciones. Máximo 17 invocaciones: versión (1), biblioteca y conductor en ambos modos (4), ejecuciones normales (6), compilación y ejecución de tres mutantes (6). Sesenta segundos por invocación, sin reintentos funcionales. Un fallo inesperado detiene y conserva la campaña; no se cambian esperados para acomodarlo.

Base: buffer fijo de 65 bytes para límite de 64, copia retenida fija de 64; dos episodios del ensayo dentro de los cuatro slots preexistentes de G1. G1 mantiene sus propios límites de memoria, trabajo y entrada. El conductor escribe capturas para auditoría; esto no acredita límites globales de E/S, red, CPU productiva o memoria del host. Python sólo materializa, verifica integridad, compila, ejecuta y registra mediciones. Ninguna decisión semántica SV se implementa en Python.

## Alcance del cierre

Conformidad sólo si se cumplen los doce casos, las seis capturas deterministas y las tres sensibilidades fijadas. Se actualizarán G/J con evidencia acotada conservando los demás criterios A–L. No se acreditan versiones arbitrarias de componentes, reglas/corpus generales, reinicio/restauración, durabilidad, host adversario, pantalla, acto de revisión, comportamiento de LLM, canales ocultos o facultades profesionales. No se promueve núcleo, no se activa K1-T/CQ1–CQ6 ni se cierra DFL general. P3/P4/P5/P6 conservan sus puertas. Agentes se valorará tras el cierre de inmunología.
