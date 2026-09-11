# Resultado de evaluación semántica antes de permisos

**RETP-2026-135 · 11/09/2026 · CONTRASTE_PUBLICO_SEMANTICO_SATISFECHO.**

La realización experimental IE004-SEMANTICA-A/1 ha pasado la matriz pública fijada en RETP-134. Se cierra esta ronda sin corrección posterior, sin modificar cupos o esperados y sin nuevas consultas a modelos. La recepción A/V, el uso de reserva, la contención material P4 y la utilidad del agente P5 conservan sus puertas pendientes.

## Corte y evidencia

Fuentes, sucesora de calendario y 61 esperados semánticos publicados antes de ejecutar: Lenguaje `5d0f0e5d1cacf69f03cf040b69e1b5bfbf228a75`; laboratorio `4e556e5d072b55b31a2a9b2c9da255602817c999`. [Fijación](ie004/semantica-a/FIJACION_PREVIA.json), SHA-256 `e5073224e257bf43e837274b867325a38171363a7521921a25714e3498c7c290`. Se comprobaron de nuevo las huellas después de ejecutar.

| Configuración Rust 1.98.0 | Semánticos | Sintácticos | Primitivas | Proceso de prueba, ms |
| --- | ---: | ---: | ---: | ---: |
| Nativo debug | 61/61 | 72/72 | 26/26 | 165,856 |
| Nativo release | 61/61 | 72/72 | 26/26 | 72,227 |
| WASI debug | 61/61 | 72/72 | 26/26 | 277,217 |
| WASI release | 61/61 | 72/72 | 26/26 | 212,129 |

Una ejecución por configuración. Los tiempos incluyen arranque y exportación extensa de evidencia; no son un benchmark del modelo, p95, latencia clínica ni una comparación controlada de velocidad entre destinos. La suma de compilaciones fue 2.858,102 ms y la de procesos de prueba 727,429 ms en el entorno registrado. No se ejecutó Python.

Las salidas, significados, bosques y cargos de cada caso coinciden entre las cuatro configuraciones. Las capturas completas debug/release son idénticas dentro de cada destino. Entre nativo y WASI cambia la cabecera que registra usize y tamaño de gramática fija. [Resultados y hashes](ie004/semantica-a/resultados-1/RESULTADO.json), [reproducción y recuperación](ie004/semantica-a/REPRODUCIR.md).

## Qué se ha ejercido

En C03 y S01, «IgG o IgM» conserva sus dos significados y devuelve PETICION_AMBIGUA, con cero llamadas a política. Revocar permisos no cambia ni significado, ni cuerpo, ni trabajo de esos dos casos. La petición única de IgM alcanza la política y recibe ACCESO_DENEGADO. La misma distinción se ejerció con dos alternativas admitidas y con una alternativa contradictoria.

Los recortes negativos conservan su alcance parcial. La escritura con operando textual igual al excluido resulta contradictoria; con operando distinto llega a la denegación de operación. El contexto insuficiente y los conflictos explícitos permanecen visibles antes de política. Las paráfrasis S34/S35 producen el mismo significado y cuerpo, aunque sus trazas y costes de lectura sean distintos.

Los controles de recursos ejercen bytes y tokens en el límite y por encima, UTF-8 inválido, cuotas reducidas de trabajo/memoria/estados, sumas y productos con overflow, conversión a usize y rechazo previo al tercer push en un vector con máximo lógico dos. Dos gramáticas sintéticas conservan dos y cuatro derivaciones. Estos controles no equivalen a inyectar todos los fallos posibles del asignador o del host.

## Coste y texto

En S26, la petición pública de 128 tokens termina con dato: 621.146 unidades propias, 1.675.752 bytes de capacidad agregada de buffers, pico calculado de capacidad solicitada de 1.777.770 bytes y 1.554 estados semánticos almacenados. Sintaxis y semántica coexisten dentro del mismo cupo. [Detalle y límites de la medición](ie004/semantica-a/COSTE_Y_TEXTO.md).

El español no queda convertido en inglés por la sintaxis de Rust. El perfil habilitado declara su léxico y conserva bytes UTF-8. Las variantes admitidas con tildes mayúsculas pasan. La grafía visualmente equivalente con tilde combinante y el espacio no separable están fuera del normalizador actual: dan SOLICITUD_NO_REPRESENTADA, nunca U. Constituyen límites conocidos de cobertura que una revisión lingüística futura debe tratar expresamente; no se presentan como comprensión correcta de español abierto.

No se ha probado una cota universal para todas las consultas de 128 tokens o todas las combinaciones de 8.192 bytes y 128 tokens. El millón de unidades pertenece a esta contabilidad candidata y no es comparable directamente con los intentos de /2 o MEMO/1. RSS, bytes de pila y realojos físicos permanecen sin medir. La reserva se conserva cerrada, asociada a /2; no se reetiqueta por haber pasado estos casos públicos.

## Continuación concreta

El próximo objeto es revisar la correspondencia semántica con /2 y fijar el corrector y la captura separados de A y V, usando las entradas públicas. Se debe cerrar el formato, las identidades y los fallos observables antes de preparar la captura P3; la contención material sigue en P4. El papel de la IA como participante y su aportación se medirán en P5. No se abre una nueva tanda de español para sustituir ninguna de estas comprobaciones. Véase [siguiente objeto](ie004/semantica-a/SIGUIENTE_OBJETO.md).
