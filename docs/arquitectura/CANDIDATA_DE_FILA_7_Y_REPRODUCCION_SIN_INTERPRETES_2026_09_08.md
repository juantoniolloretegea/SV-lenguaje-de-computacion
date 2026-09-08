# Candidata completa de fila 7 y reproducción sin intérpretes

**RETP-2026-102 · 08/09/2026. Contrato previo.** Entrada: `2f4581cc80307053e2b93b3f2e3be3a3caac8d82`, árbol `7cd32bcb704fd855ce47b3e9aa7282a9b159ed93`. Integra por ascendencia las candidatas #79–#82; no están promovidas a `main@1706099aef4a0e3846706c3963e7c76313adaf68`. Se conservan Pilares RETP-073, perfiles RETP-075, la tabla rectora de catorce filas y sus sucesiones RETP-096…101.

## 1. Decisión de continuidad

El Director dispone continuar sin revisiones externas entre incrementos. La adversarial externa fuerte se hará sobre la candidata completa de fila 7, antes del relevo a Ciberseguridad. Esta instrucción sucede a las menciones anteriores de revisión externa inmediata en #79–#82; no suprime comprobaciones internas, no acredita un cierre por anticipado y no autoriza a abrir el dominio CYB. Las candidatas anteriores conservan sus cortes y evidencias.

La revisión final deberá examinar **toda la diferencia desde `main@1706099…` hasta la candidata final**, incluidas las guardas de contexto y LIG del núcleo. El último incremento por separado no es el objeto suficiente de esa auditoría. Las inserciones de #79/#80 cambian la aceptación de entradas.

## 2. Capacidad material sometida al segundo falsador

La candidata ofrece compilación declarativa por los perfiles ya constituidos; bienformación local H04/H05/H06/H07; y la API Rust LIG/0.1 para comprobar y recuperar las ligaduras de una operación identificada. El transporte de los ocho pares GH-DOC prueba conservación documental y recuperación externa F0/HS, junto con ocho pérdidas H; sus 48 huellas se recalculan externamente conforme a RETP-100. La matriz 15/44/81 de GH-LIG gobierna cada pérdida, su tratamiento, responsable y condición de retorno.

| Obligación recibida | Resolución representacional ofrecida | Capacidad que continúa excluida y sede |
| --- | --- | --- |
| SP-01/02: identidad y constitución | Programa/contrato/referentes exactos, instancias, usos, orden, destino cuando se exige, alias y compartición; RETP-098. | Constitución clínica y autenticación efectiva; dominio/institución. `REQ-IMM-SV-011` conserva `U_NO_DECIDIDO`; no se derivan células de 27 parámetros. |
| SP-03/04: transducción y criticidad | Referentes y coherencia local representados; ausencia de productor reconocida. | K1-T y DFL-006, puerta algebraica y retorno de comprobación. No se ofrece producción observación→Tri ni criticidad Q0. |
| SP-05/06: resumen, orden y no compensación | Ocho pérdidas H localizadas; recuperación documental F0/HS con S explícita y orden conservado. | Recuperación clínica, veto y composición ejecutiva: reglas/productores y obligaciones N3/N4. No se infieren de la existencia de `query` o `compose`. |
| SP-07/08/09: salida, reproducción y fallo | Proyección y transporte deterministas en los testigos identificados; rechazo técnico sin salida clínica. | Selector terminal Q0, reproducción de ejecución clínica y fallos durante efectos: realización algebraica y contrato operacional. |
| SP-10/11: autoridad y soporte | Declaraciones de procedencia preservadas; ningún permiso derivado de una huella. | Autoridad institucional, R1 integrado y soporte R2/R3/R4; DFL-009 en fila 9 y frontera en fila 13. |
| SP-12: perímetro y relevo | Matriz completa, exclusiones por operación y candidata identificada para auditoría. | CYB constituye su propio universo después de la recepción; no completa Q0 ni recibe funciones clínicas tácitas. |

Las doce SP siguen no ejecutadas de extremo a extremo. Esas ejecuciones no se sustituyen por estos subcierres. DFL-005 no queda cerrada para cualquier operación futura; las que necesiten significado, ejecución o autoridad excluidos no se ofrecen. Tampoco se cierran DFL-001/003/004/006/009/011/012/013. La revisión de la prosa de los tratamientos permanece humana.

## 3. Paquete de fuentes y obligación previa de reproducción

Se preparará un paquete autónomo para **construir esta candidata Rust y ejecutar sus pruebas Rust**, no una nueva distribución web ni un servicio. Debe conservar literalmente los manifiestos Cargo, todo el código Rust de `rust/`, las entradas `.svp` y datos JSON/huellas/Rust de pruebas que éste incorpora. Se incluye LICENSE intacta. No se admiten `.py`, `.pyc`, `.js`, `.mjs`, `.cjs`, binarios previos, cachés, credenciales ni enlaces simbólicos. No se modifican los manifiestos Cargo para esconder ejemplos o pruebas.

La lista exacta procederá de un commit Git, no del directorio de trabajo. Cada archivo llevará SHA-256 en el inventario del paquete; el archivo comprimido tendrá su propia huella. Dos preparaciones del mismo corte deben producir bytes idénticos. El inventario sirve para integridad y exhaustividad respecto de su lista; la autenticidad de la fuente requiere contrastar el commit y la huella publicada fuera del paquete. No se certifica una ejecución por el contenido de un informe.

Se comprobará en un contenedor Ubuntu 24.04 sin intérpretes Python ni Node: Rust/Cargo 1.98.0 montado desde la herramienta de referencia, GCC/libc para enlazar y GNU Coreutils para preparar/comparar. El contenedor se identifica por su digest y su inventario de paquetes. Su preparación puede descargar las herramientas; **la construcción y las pruebas del SV se ejecutan con red deshabilitada y Cargo offline**, sin caché previa ni acceso al repositorio completo. Docker sólo aísla este ensayo de CI; no constituye plataforma del SV, anfitrión ni dependencia de ejecución del producto.

El paquete debe permitir `cargo build --workspace --release --offline`, todas las pruebas Rust del espacio de trabajo, la regeneración G/H `--check` y las sondas LIG/GH. La CLI debe admitir una fuente constituida en cada perfil, emitir salida y rechazar un testigo inválido con retorno 1, IR vacía y la causa prevista. No se fijarán nuevos esperados copiando su salida. El recálculo JavaScript de las 48 huellas seguirá ejercido por el observador externo fuera del contenedor; no se afirmará que éste se ejecuta sin Node.

Controles negativos de embalaje: archivo Python añadido, archivo inesperado aunque tenga extensión admisible, archivo declarado ausente y bytes de fuente alterados. Cada control debe producir su causa de rechazo. Un paquete íntegro debe ser admitido; una mutación rechazada por una causa anterior no acredita la guarda atacada.

## 4. Cierre material y revisión final

La candidata quedará preparada para la adversarial fuerte cuando el alcance de §2, los contratos y fuentes originales, los cuatro flujos existentes y la reproducción de §3 estén comprobados sobre cortes exactos. Se conservará `row7_closed=false` hasta resolver la auditoría y la decisión de entrega. No se requieren nuevas auditorías externas intermedias ni se fusionan las PR por el simple hecho de que CI esté verde.

El repositorio completo conserva 17 archivos Python y fragmentos Python en CI. El paquete sin intérpretes demuestra una ruta autónoma delimitada, no la retirada global de esas herramientas ni la auditoría final de todas las dependencias. La revisión integral del español sigue siendo obligatoria antes del cierre del núcleo y su acta no cambia. La prueba actual no habilita Q0, no agota pruebas de recursos/fallos/geometrías y no constituye la distribución final para cualquier plataforma.

**Estado previo:** preparación y comprobación pendientes. Los resultados se inscribirán en RETP-102 después de ejecutar.
