# S26-F01/F02 · Recepción del banco de referencias y ligaduras

**14 de septiembre de 2026. Unidad: Watson / W-S26. Resultado: 10/10 casos conformes en debug y 10/10 en release; retorno 0 en ambas campañas.**

Cada caso recorre las entradas SVP-EN y SVP-ES: veinte ejecuciones caso/entrada por perfil, cuarenta entre ambos perfiles. Son diez casos distintos. No se añaden ejecuciones de INTEGRADO01 ni de los bancos anteriores.

El [banco previo](BANCO_F01_F02_LIGADURAS.md) y sus oráculos se publicaron en Lenguaje `ccbb152f69fa1a954e153fbaac24e4ac337e9ef3` y laboratorio `985423ecc0f5c0127764286bf26ee503ec050c17`. La [corrección instrumental de invocación](AJUSTE_INVOCACION_F01_F02_LIGADURAS.md), necesaria por instalación directa sin rustup, quedó publicada antes de compilar y ejecutar: Lenguaje `7a08f5d9b57a102af5388a5f3ad84a759028994a`; laboratorio `81582b8464e0ab6f37a74ad9ef6f2dcb75e68d4c`. La fuente, los estímulos y los oráculos permanecieron intactos.

## Resultados observados

| Casos | Observación conforme en EN y ES, en ambos perfiles |
| --- | --- |
| 00 | I1 e I2 conservan su identidad aun compartiendo P y el numeral 1: U1/I1/CC1/3 y U2/I2/CCCompartido/3. |
| 01 | FuenteP versión 2, con los mismos bytes y hash del contenido, se rechaza bajo la expectativa A. La expectativa B admite esa versión; los objetos recibidos A y B conservan respectivamente 1 y 2. Mutar copias externas no altera A. |
| 02–03 | Permutar el registro de instancias no sustituye el orden de usos. Permutar los usos conserva el nuevo orden explícito. El testigo de primera coincidencia de P difiere del consumo constituido. |
| 04–05 | Dos posiciones distintas del mismo nodo se preservan. El intercambio de instancias se rechaza bajo la expectativa fija y se admite sólo bajo la otra expectativa explícita del control. |
| 06–09 | Se obtienen exactamente DestinationCollision/U2, DuplicateIdentity/I1, InstanceMissing/I0 y DestinationMissing/U1; no se produce objeto validado para esos candidatos. |

Todos los casos conservan además el control original y su contrato, programa, expectativa y filas consumidas después de la mutación. Estos observadores usan la API pública de LIG/0.1 y los fixtures existentes; no realizan una consulta histórica por Frame ni ejecutan el efecto de la operación declarada.

## Ejecución e integridad

Rust/Cargo 1.98.0, x86_64-unknown-linux-gnu. Compilación offline con Cargo.lock; opt-level 0 y 3, este último con debug-assertions y overflow-checks desactivados. El arnés Rust sigue comparando los oráculos mediante sus aserciones de prueba. Los seis comandos de compilación/ejecución terminaron con retorno 0. Se conservan las 25 advertencias de sv_core por compilación; el banco y las campañas no emitieron diagnósticos en stderr.

R08 cotejó los 2996 archivos del corte fuente, los 2999 del precompromiso inicial y los 3002 del corte publicado con el ajuste y Word. El último conjunto se cotejó antes y después de las dos campañas: mismos modos y blobs. Los recursos incorporados al compilar permanecen dentro de ese árbol completo. Las incidencias de formato de descriptores y del lanzamiento administrativo se conservan separadas de los resultados SV.

[EJECUCIONES_F01_F02_LIGADURAS.json](EJECUCIONES_F01_F02_LIGADURAS.json) identifica fechas, comandos, entorno, huellas, archivos y alcance. [EVIDENCIA_F01_F02_LIGADURAS.tar.gz](EVIDENCIA_F01_F02_LIGADURAS.tar.gz) conserva los canales íntegros, registros de ejecución, descriptores R08, fuentes del banco y adaptadores administrativos Rust. El árbol completo de fuentes se recupera por sus cortes Git; el archivo de evidencia no duplica el repositorio ni los ejecutables, cuyas huellas quedan registradas.

## Alcance y relevo

Se acredita el incremento LIG de referencias, versiones, instancias, destinos y orden de consumo en estos diez casos. La igualdad de bytes no elimina la distinción de versión constituida; el hash del contrato y la expectativa fijada rechazan las sustituciones ensayadas. Estas comparaciones no constituyen autoridad, firma, identidad de adquisición, aislamiento frente a host hostil ni garantía física de RAM.

**F01 y F02 permanecen abiertos en su alcance completo.** F01 requiere la consulta de ocurrencias exactas de igual valor dentro del objeto material que la tenga constituida. F02 requiere resolver el componente por instancia, posición y constitución de célula/Frame en la operación correspondiente. LIG no ofrece esa consulta histórica y el banco no la incorpora por una función inventada en el arnés.

La continuación deberá identificar la interfaz material y la operación aplicables en la secuencia rectora antes de añadir ese contraste. Si siguen ausentes, se conservará ese pendiente en su sede de constitución. No repetir estos diez casos ni INTEGRADO01 por cambios administrativos. Sucesos S26 revisión 25 y los índices enlazan esta recepción y el laboratorio. S26/Bis siguen abiertos y S24 mantiene su secuencia. RETP canónica, núcleo, contratos, workflows y campañas anteriores permanecen sin cambios.
