# Memoria, vectores y arrays en el ensayo de coste

**RETP-2026-133 · 11/09/2026.** Inciso humano incorporado al trabajo de coste: la representación, el tamaño y las posibles recolocaciones afectan al rendimiento. Este documento analiza la fuente y los resultados ya obtenidos; no añade ejecuciones.

## Distinciones necesarias

`Vec<T>` posee un almacenamiento contiguo ampliable. Su cabecera contiene puntero, longitud y capacidad; la cabecera reside donde esté el objeto, no necesariamente en la pila. Para tipos de tamaño no nulo, crecer sobre la capacidad puede exigir otra asignación y trasladar los elementos. `push` no reasigna si cabe en la capacidad disponible. Rust no promete un factor de crecimiento concreto. Reservar anticipadamente reduce ese riesgo dentro de la capacidad reservada, pero ocupa memoria. [Garantías oficiales de Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees).

`[T; N]` tiene longitud fija y almacena sus elementos dentro del propio valor. Puede formar parte de un objeto en la pila, de uno alojado en el heap o de almacenamiento estático; elegir un array no decide por sí solo la ubicación. Un array grande tampoco elimina el coste de inicializarlo o mover el valor que lo contiene. [Referencia oficial de arrays](https://doc.rust-lang.org/reference/types/array.html).

Alcance de consulta de ambas fuentes: definiciones y garantías de representación/capacidad; consulta 11/09/2026. La documentación vigente de std mostraba 1.98.1; el instrumento se compiló con 1.98.0, cuya identidad consta en ENTORNO.json. Estos enlaces no se presentan como una captura inmutable de aquella toolchain.

## Medida del caso CP-T-128

El motor compiló 421 expresiones y reserva una tabla triangular de 8.385 intervalos por expresión. Son 3.530.085 identificadores `u32`, es decir, 14.120.340 bytes. Los 13.206 nodos presentes ocupan el 0,3741 % de las posiciones posibles. Esta ocupación se calcula desde las dimensiones y la salida Rust; no es una medida del hardware.

| Magnitud | Nativo, 64 bits | WASI, 32 bits |
| --- | ---: | ---: |
| Tabla de identificadores | 14.120.340 B | 14.120.340 B |
| Total de capacidades contabilizadas | 16.144.529 B | 15.492.821 B |
| Resto de capacidades | 2.024.189 B | 1.372.481 B |
| Proporción de la tabla | 87,46 % | 91,14 % |

Fuente derivada: [DESGLOSE_MEMORIA.json](DESGLOSE_MEMORIA.json), cotejada con [resultados Rust](resultados/RESULTADO.json). MB decimales en el comentario humano; MiB sólo para los cupos del contrato.

En `Motor::nuevo`, la tabla obtiene capacidad y se inicializa una vez. Después se actualizan posiciones existentes: esa tabla no crece durante el análisis. Otros vectores de nodos, familias y resultados sí pueden crecer. El instrumento registra capacidades y pico solicitado, pero **no cuenta realojos efectivos, cambios físicos de dirección ni bytes físicamente copiados**. Tampoco mide fallos de caché, RSS o consumo total de memoria lineal WASM. No se atribuye a los realojos la memoria de una tabla fija ni se deduce latencia de su ocupación.

## Consecuencia para el siguiente paso

La mayor partida observada procede de reservar un espacio de búsqueda denso poco ocupado. Sustituir su `Vec<u32>` por un array del mismo tamaño conservaría esa partida. Una representación dispersa puede ahorrar posiciones vacías, pero añade sus propios índices, búsquedas y metadatos; su balance necesita una comparación posterior, no una mejora supuesta.

Para construir el evaluador semántico se mantienen estas reglas de implementación:

1. Arrays para dimensiones pequeñas, fijas y declaradas; presupuesto explícito de pila. Buffers grandes con reserva fallible acotada. No trasladar una arena entera a un array local.
2. Declarar propietario y etapa de vida de cada buffer. Durante construcción, identificar nodos por índices comprobados. Prestar slices de sólo lectura durante consumo sin mantener referencias que deban sobrevivir a un crecimiento del propietario. Un índice no da permiso para reutilizar datos de otra solicitud.
3. Contabilizar capacidad, metadatos, inicialización y crecimientos de todos los contenedores de A. La memoria del bosque sintáctico y la de los estados semánticos que coexistan suman dentro de la misma arena de análisis; no asignar 32 MiB a cada una. El pico solicitado de esta sonda no constituye un límite material garantizado del asignador.
4. Añadir contadores de solicitudes de reserva y de crecimiento por clase de buffer, incluyendo la capacidad anterior y nueva. Separar esos contadores de una medición posterior del asignador y de copias físicas: una solicitud puede satisfacerse en el mismo lugar. Medir también el trabajo de inicialización que no refleja la cuenta sintáctica actual.
5. No compartir arenas ni cachés mutables entre A y V. Si se reutiliza almacenamiento entre solicitudes de A, invalidar su estado lógico antes del nuevo uso y verificar que ninguna referencia o resultado anterior permanece accesible.

El ensayo conserva su fuente congelada. Esta observación fija requisitos del siguiente objeto; no abre ahora una campaña de optimización de contenedores, una FFI ni un cambio de dominio. Las mejoras de Rust en propiedad y préstamos no equivalen a aislamiento del host ni cambian el significado de 0/1/U.
