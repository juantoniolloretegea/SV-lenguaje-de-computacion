# Coste observado, propiedad y extensión del texto

**RETP-2026-135.** Datos de una sola matriz pública de IE004-SEMANTICA-A/1. No se añaden ejecuciones para mejorar un promedio.

| Magnitud | Observado en S26, 128 tokens | Cupo de la candidata |
| --- | ---: | ---: |
| Trabajo | 621.146 unidades | 1.000.000 |
| Capacidades simultáneas de buffers | 1.675.752 B | 33.554.432 B |
| Pico calculado incluyendo capacidad vieja y nueva solicitada | 1.777.770 B | 33.554.432 B |
| Estados semánticos almacenados, parciales y finales | 1.554 | 16.384 |
| Profundidad sintáctica observada | 148 | 512 llamadas |
| Profundidad semántica observada | 144 | 512 llamadas |

Son máximos de esta matriz en trabajo, capacidad y estados; no cotas probadas del lenguaje entero. La profundidad es un contador de llamadas, no una medición de bytes de pila. Los resultados coinciden en los cuatro destinos/configuraciones para estos buffers y contadores.

El desglose de trabajo de S26 es [10.416, 10.894, 451.631, 11.684, 0, 40.646, 0, 95.738, 41, 96], según las diez categorías de la sucesora. La búsqueda de extremos ya consume 451.631 unidades. La nueva representación ahorra posiciones vacías y cobra sus búsquedas; no convierte su coste en constante ni acredita el peor caso combinatorio.

Las 41 solicitudes de reserva se distribuyen por normalización, tokens, memo, nodos, familias, índice semántico, estados y raíces como [1, 1, 1, 13, 13, 1, 10, 1]. Los crecimientos pedidos son [0, 0, 0, 12, 12, 0, 9, 0]. Son solicitudes; no se observó la dirección de memoria y no se conoce cuántas movieron físicamente datos. El pico calculado no incluye sobreasignación, metadatos del asignador, original custodiado, gramática fija, pila, runtime ni serialización.

La gramática fija ocupa 12.912 B en nativo y 6.760 B en WASI. Un significado ocupa 91 B, un estado asociado 100 B, un nodo 24 B, una familia 16 B, una celda 12 B y un token 24 B en los dos destinos ensayados. El tamaño del tipo de gramática depende de la representación local de referencias. No se usa este layout como ABI para FFI.

El original mantiene la propiedad de su texto; A lo toma prestado mientras vive la solicitud. La normalización tiene un solo buffer acotado. Los estados almacenan identificadores de campo y enlaces numéricos, evitando multiplicar copias de sinónimos. Las vistas se obtienen mediante intervalos UTF-8 comprobados y no se conservan referencias a elementos que puedan ser realojados. La prueba está compilada con forbid(unsafe_code).

Los tipos fijos pequeños derivan de este repertorio: no son un diseño universal de memo para cualquier dominio. El crecimiento de un catálogo, un campo textual de salida o un normalizador deberá versionar y acreditar sus dimensiones antes de admitirlo. Se exige comprobar suma, producto, conversión, máximo lógico y presupuesto agregado antes de insertar. Una reserva previa por sí sola no constituye un límite.

S24 ejerció 8.192 bytes con relleno de separadores y una consulta válida; S27 rechazó 8.193. S25/S26 ejercieron 127/128 tokens repetidos; S28 rechazó 129. S23 recorrió 4.000 bytes de texto multibyte sin interpretación admisible. S20 y S45 distinguen tilde combinante no admitida de tilde precompuesta admitida. Estos casos no estiman que el español necesite un factor fijo de memoria frente al inglés.

Los perfiles SVP-ES/SVP-EN del lenguaje y la interacción NLP se vinculan por identidad y contrato, no por traducción automática. Sólo se activa IE004-ES-P2/3-COSTE/1. Incorporar otro perfil de interacción exige declarar normalización, unidades, corpus y equivalencias semánticas; no ampliar silenciosamente cupos ni usar permisos para resolver diferencias lingüísticas.
