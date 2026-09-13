# S26 R08 · Resultados de reparación acotada del auxiliar

Recepción: 2026-09-13. S26 revisión 12; RETP-2026-235. Estado: candidata auxiliar cualificada en el alcance descrito; S26 permanece en ejecución.

## Procedencia y orden

Se parte de los cuatro defectos reproducidos por R07, conservados sin cambios. La reparación tiene código y procedimiento propios en Rust. El banco, sus quince oráculos, fixtures y límites se publicaron antes de ejecutar en Lenguaje `0152c858e05bb51b3555ac5e998f526e9693ee5c` y laboratorio `6e6e5f3e61095519b3041f1484c28dd98b72a76d`. [Contrato previo](README.md) y [precompromiso](PRECOMPROMISO.json) conservan su redacción previa, incluido el estado «aún no ejecutados» de aquella revisión. Este documento recibe los resultados posteriores.

## Ejecución y resultados

Rust `1.98.0 (88d9e12ae 2026-08-18)`, sin dependencias de Cargo. Fuente idéntica en dos perfiles:

| Perfil | Configuración | Casos conformes | Terminación |
| --- | --- | --- | --- |
| Debug | Compilación ordinaria; debug_assertions=true | 15/15 | salida 0 |
| Optimizado | opt-level=3; debug-assertions=no | 15/15 | salida 0 |

Son quince casos distintos y treinta concordancias entre ambos perfiles. P08 contiene cinco posiciones de fallo dentro de un caso. Los TSV conservan el dictamen del banco y el archivo de evidencia conserva sus directorios reales. El banco implementa el observador; compartir funciones con la candidata limita su independencia. Los contrastes negativos fijados previamente dan sensibilidad a las alteraciones ensayadas, sin demostrar ausencia de todos los defectos.

| Defecto recibido R07 | Respuesta R08 observada |
| --- | --- |
| Guardas eliminadas al optimizar | Bases, revisiones y estados incompatibles rechazados en ambos perfiles mediante condiciones explícitas. |
| Checkpoint ajeno o anterior | Comando/formato verified rechazado; observación anterior no satisface un nuevo commit esperado. |
| Escritura parcial sobre origen | Cinco fallos tras escritura dejan restos en salida separada; origen intacto; comprobación rechazada. |
| Aceptación de conjunto sin cotejo suficiente | Árbol alterado, truncado o con ruta duplicada rechazado; salida alterada rechazada incluso conservando marca final. |

P01/P05/P14/P15 incluyen controles positivos. Los restantes detalles y códigos esperados están en el contrato previo.

## Uso sobre los cinco registros reales

Se tomaron las cinco piezas administrativas del corte de precompromiso, cotejadas por sus blobs. Se preparó S26 revisión 12 y RETP-235 en una candidata separada. La CLI real rechazó primero `REG_RETP_PREFIJO`: el editor había normalizado finales de línea históricos del CSV. Tras conservar literalmente ese prefijo, rechazó `REG_MD_OTRAS_SECCIONES`: se había eliminado una línea en blanco final ajena a S26. Se corrigieron ambos efectos del transporte editorial sin relajar guardas ni modificar oráculos.

Después, `preparar` y `comprobar` terminaron con salida 0 y `COMPROBACION_LOCAL_CONFORME`. El origen siguió byte a byte igual al corte. El archivo contiene `antes`, `candidata`, `preparado`, solicitud y observaciones de las cuatro invocaciones. Las candidatas rechazadas no se archivaron íntegramente antes de corregirlas; sus códigos y causas proceden de las llamadas observadas. No se presentan como nuevas campañas precomprometidas.

La recepción sólo acredita preparación y comprobación locales. La publicación debe tomar los bytes preparados, leer nuevamente el corte remoto y cotejar con la CLI Rust el árbol completo antes de mover la referencia sin force. La evidencia de esa publicación posterior se entrega con sus identificadores, sin hacer que este commit se autocertifique.

## Evidencia e identidades

- [Resultados debug](RESULTADOS_DEBUG.tsv).
- [Resultados optimizados](RESULTADOS_OPT.tsv).
- [Campañas y operación real](EVIDENCIA.tar.gz).
- SHA-256 del archivo: `8db73d915171cb743cd21a785be8a288201684ce6c922e562faeea51c5a71429`. Su contenido se cotejó contra los directorios mediante tar; salida 0.
- CLI compilada: SHA-256 `a6711d7bf433f729e97375044fab2ae3219e4f61ae90442ac866d9f7afef1998`.
- Banco debug: SHA-256 `bf14f708e56c9e5a42290052a33b27fbe3173bf325083975dc56904b81c4e7da`.
- Banco optimizado: SHA-256 `29cb661edc6d01e035df3921012ee04988bd7fe6d0243c975bbc94196c2dbc73`.

Las fuentes y órdenes de compilación están publicadas. Las huellas identifican los ejecutables utilizados; el archivo conserva datos de campañas y operación, no una distribución de esos binarios.

## Fronteras que permanecen abiertas

La carpeta preparada no es una transacción atómica de cinco archivos del checkout. P08 devuelve un error inyectado después de una escritura real; no equivale a apagar el hardware. La relectura y sync_all no certifican durabilidad física. El marcador no sustituye la comprobación.

La CLI de publicación compara descriptores; no los obtiene del servidor, no establece autoridad por sí sola, no detecta dos descriptores falsificados coherentemente, no garantiza frescura del adaptador ni ejecuta CAS remoto. Se presupone custodia de directorios sin escritor concurrente hostil. No se acredita resistencia de RAM, host, GUI ni cuotas globales de recursos.

No se ejecutó Python en R08. El lanzador ejecutó Rust y utilidades de transporte/cotejo de bytes; la edición se realizó mediante parches. La herramienta JavaScript del entorno coordinó llamadas y confeccionó entradas administrativas; Rust verificó las correspondencias implementadas. No se atribuye todo el entorno a Rust ni se certifica su inocuidad.

## Continuación

La candidata puede servir en este perfil administrativo acotado; los auxiliares heredados conservan sus fuentes y limitaciones históricas. Se retoma R06 con correlación de encargo, separación de informe/terminación/efecto e identidad del contenido admitido y consumido. T01/T02/T05/T06 siguen pendientes de su variante y precompromiso propios. R08 no los cuenta como ejecutados ni cierra los doce casos globales S26. Se conserva Bis → catálogo/cierre → S24; dominio y rutas necesarias permanecen en su sede.

