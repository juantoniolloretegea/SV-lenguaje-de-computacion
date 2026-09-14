# S26 R06 LOCAL01 · Resultados y relevo · RETP-2026-236

**Trece casos conformes en debug y trece en release: 26 concordancias con los oráculos precomprometidos.** Incremento local T01/T02/T05/T06 acreditado en la frontera descrita; R06 permanece abierto.

## Procedencia y orden

Entrada: Lenguaje d589123304905a9162148e73946914382dba995d y laboratorio a0f4aba91af549e80cfaf3daf6379dc05d158204. Se consultaron completos Pilares, acta de perfiles/contratos/ensamblaje y transición secuencial, con R02, R06, R2-0 y LIG/0.1. [Fuentes cotejadas](FUENTES.json). Se reutilizaron el admisor, objeto admitido, certificado y fixture I0205-01 existentes; el núcleo vigente y las campañas anteriores permanecen sin modificación.

El [banco y los oráculos](README.md) quedaron publicados **antes de ejecutar**: Lenguaje [8cb31db106b5621b105a5b8e0d469bf0fc8819a1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/8cb31db106b5621b105a5b8e0d469bf0fc8819a1); espejo [1b01d66ff1c531d84f596ccf075bc3bef275b81d](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/commit/1b01d66ff1c531d84f596ccf075bc3bef275b81d). Ambos árboles completos fueron cotejados por el auxiliar Rust R08 y sus ramas verificadas tras publicar sin force.

Compilador: rustc 1.98.0 (88d9e12ae178fab0fb5cc050a94da85685d449ea), host x86_64-unknown-linux-gnu, LLVM 22.1.8. Cargo offline y locked, sin dependencias externas. Los dos perfiles usan panic=unwind; release desactiva debug-assertions. Las compilaciones terminaron con código 0; el núcleo emite 25 advertencias existentes, conservadas en los logs. No se corrigieron como parte de este incremento ni se presentó compilación limpia de advertencias.

## Evidencia observada

| Casos | Debug / release | Hecho acreditado y límite |
| --- | --- | --- |
| P01 | Conforme / conforme | Informe completo del intento correcto; captura y recibo coinciden con los originales fijados. Sólo entrega documental local. |
| P02–P04 | Conforme / conforme | Ausencia, truncamiento y otro intento distinguidos; retorno ordinario no produce aceptación; captura previa conservada. |
| P05 | Conforme / conforme | Pánico observado en barrera anterior al despacho; apertura conservada, captura e informe ausentes. |
| P06 | Conforme / conforme | Pánico posterior a captura y escritura real; captura conservada, informe final ausente; nueva lectura encuentra la escritura. No hubo rollback de esa escritura. |
| P07 | Conforme / conforme | La ruta termina con otro objeto y contenido diferente; el consumidor recibe literalmente el descriptor original admitido. |
| P08 | Conforme / conforme | Sustitución real por objeto distinto con contenido igual; perfil mínimo acepta sin afirmar continuidad del soporte. |
| P09 | Conforme / conforme | Pretensión superior fijada antes de ejecutar: no acreditada; no se ejecuta la frontera de consumo. |
| P10–P12 | Conforme / conforme | Recibo null, exceso de informe y revisión ajena en apertura rechazados con causa concreta. |
| P13 | Conforme / conforme | Rechazados intento duplicado, vacío y número 33. Unicidad instrumental sólo en esa instancia de RAM. |

Ambas campañas completas terminaron con salida 0. Hubo dos pánicos deliberados por campaña, cuatro en total; el hook estándar aparece en stderr y se conserva. No se alteraron código del banco ni oráculos después del precompromiso para obtener conformidad.

## Dónde viven y qué puede mutar

Custodia, apertura, objeto admitido, captura e informe viven en la RAM del proceso de ensayo. La geometría admitida se conserva en un buffer propio y se presta al consumidor local; sustituir el nombre de archivo después no sustituye esos bytes. La captura representa la copia efectuada en esa frontera. El archivo de salida documental se escribe después desde el arnés.

Los archivos de las sondas viven en el sistema de archivos local. P06 demuestra que una escritura puede permanecer aunque el recorrido no produzca informe; P07/P08 que el archivo accesible por ruta puede cambiar sin cambiar la copia admitida. Esta distinción no establece una transacción entre RAM y disco. No hay BD, tablas dinámicas productivas ni reconstrucción de Frame aprobadas por este banco.

Los errores ordinarios se expresan con Result y las guardas se mantienen en release. catch_unwind sólo observa pánicos desenrollables dentro de su clausura; no protege la RAM física, la propia construcción del observador, abort, proceso muerto o host. Las etiquetas del arnés no se introducen en Tri, R1, Frame ni códigos soberanos SV.

La aceptación del recibo no acredita pantalla, efecto clínico, adquisición física, durabilidad o autorización de una acción nueva. La observación conserva efecto externo NO_ACREDITADO; no ejecuta acciones dependientes ni reintenta automáticamente. No hay garantía de ejecución exactamente una vez.

## Conservación y administración

[EVIDENCIA.tar.gz](EVIDENCIA.tar.gz), 111638 bytes, SHA-256:
`08e71f7dbb724a1b5fa8f36dfdfb00695d83c546c09b08657f84db8fed1b21bc`.
Incluye ambas campañas completas: archivos de ensayo, observaciones con bytes literales, verificaciones por caso, resultado.json, logs y solicitud administrativa. Las fuentes se conservan en el precompromiso.

SHA-256 de los ejecutables usados:
- Banco debug: `2b499eb9adf709919c65f535930ebf89f2059d661edadc348f7be2d83546bd6c`.
- Banco release: `58b304aacbeb1f7b9fe14e25c1a38274ac233bef0035457c3027793cdabd0237`.
- Editor documental: `5ab20ee43b053da460fd6408399b1757c585368077d1910f5fc92c813db1382a`.
- Auxiliar R08: `a6711d7bf433f729e97375044fab2ae3219e4f61ae90442ac866d9f7afef1998`.

[REGISTRO.json](REGISTRO.json) conserva la prosa de S26 revisión 13 / RETP-236. El editor Rust [registrar.rs](src/bin/registrar.rs), añadido después de la campaña, sólo produce candidatos fuera del origen. No forma parte de los trece casos ni se presenta como herramienta adversarialmente cualificada. R08 preparar y comprobar han admitido el conjunto real de cinco registros, conservando prefijos históricos y otras filas/secciones. El código del editor y sus entradas quedan revisables; R08 tampoco demuestra la verdad de la prosa. No se ejecutó Python. Shell sólo lanzó herramientas y transportó/archivó bytes; el conector GitHub publica y el auxiliar R08 coteja los descriptores completos.

## Próximo paso

Constituir el protocolo del observador superviviente para T03/T04/T08: correlación previa, framing/cotas, terminación de proceso separada del canal, informes tardíos/duplicados y ausencia de efecto no inferida del silencio. T07 requiere sus propias barreras de modificación/ABA y dependencia de otro corte. No se recontarán los pánicos locales como abort de proceso ni P13 como tratamiento de informes tardíos.

**R06 es una recepción de S26, no el último apartado de R0.** T03/T04/T07/T08 y la integración del recorrido completo siguen pendientes. Cero casos globales S26 cerrados; S22/Bis → catálogo/cierre → S24 conserva su secuencia. R2 material, persistencia, recuperación y GUI no se dan por cerrados. La constitución del dominio y las rutas del agente conservan sus sedes.

