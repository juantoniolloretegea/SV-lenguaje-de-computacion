# S26 R06 · T0701 · Mutación del mismo objeto y concordancia de corte

**Banco previo: trece casos definidos, sin ejecución al publicar este precompromiso. Unidad responsable: Watson / W-S26.**

Corte del Lenguaje: 9b8418441b981e50586451485604cf68e3a5477d, rama main. Laboratorio: 882f1813feb739a9f5c10e3001816d32185671b8, rama lab/playground-sv-permanente. La recepción posterior se añadirá en RESULTADOS.md. Los oráculos y esta descripción del banco previo se conservan.

## Objeto y reutilización

Se desarrolla R06-T07 de [R06 §5](../README.md): modificación del objeto de archivo entre lecturas, restitución A–B–A y mezcla de dependencias o referentes. Las fronteras de apertura, informe y terminación ya recibidas en LOCAL01 y PROCESO03 mantienen sus evidencias. Este incremento no suma sus resultados para declarar probado un recorrido integrado.

Se reutilizan ReceivedBytes, TrustedContext, TrustedRegistry, admit y certify de Bis I0205, el consumidor de [LOCAL01](../local01/src/recepcion.rs), su separación de perfiles y el fixture I0205-01 de R05. FUENTES.json coteja 107 piezas, incluidos núcleo y dependencias de compilación, rectores, contratos, fixture, consumidor y auxiliares administrativos. No se modifica el núcleo, el admisor ni el consumidor heredado.

Pilares, perfiles/contratos/ensamblaje y transición secuencial fueron leídos completos en esta continuidad; R06, R2-0 y LIG conservan sus obligaciones. La prueba es sintética y documental; no constituye dominio, nueva identidad SV, autoridad, transacción ni perfil tecnológico productivo.

## Frontera y barreras

Cada caso abre y conserva su encargo antes de recibir las cinco piezas. El perfil queda fijado en esa apertura. Las escrituras se realizan mediante descriptores de lectura/escritura sobre el mismo archivo; no se utiliza rename. Se mantiene un descriptor testigo abierto y se registran dev/inode en cada muestra. Estos valores son testigos POSIX locales, no una identidad universal ni prueba de inmutabilidad.

Las barreras son puntos síncronos del arnés Rust, en un solo hilo. P05/P06 interponen un lector que devuelve primero exactamente 3072 bytes de geometría. Al pedir el siguiente tramo, escribe B sobre el mismo objeto, obtiene una muestra material y permite leer el resto. Después de recibir los 6237 bytes restituye A y toma otra muestra, antes de la admisión. No se utiliza una pausa temporal para suponer un orden ni se afirma cobertura de planificaciones concurrentes arbitrarias.

- En P05 se cambia el último byte, pendiente de lectura, de salto de línea a signo de exclamación. Los extremos del archivo son A, pero el búfer recibido termina en ese signo: debe rechazarse por M01.
- En P06 se cambia el primer byte, ya leído. Los bytes recibidos siguen siendo A, aunque la muestra intermedia del archivo es B. El perfil mínimo puede admitir ese contenido; esa aceptación no demuestra continuidad del archivo.
- En P02–P04 la mutación ocurre después de admit y antes de la llamada al consumidor. El consumidor obtiene el descriptor del objeto admitido. P04 había solicitado continuidad del soporte y debe terminar sin captura por capacidad no ofrecida.

Las muestras binarias antes, durante y después, los bytes recibidos, el descriptor admitido y los consumidos quedan en archivos separados. observacion.json registra su orden, identidades y controles; recepcion.json conserva el resultado del consumidor heredado. El emisor instrumental interno del consumidor conserva su etiqueta LOCAL01; la apertura del banco identifica T0701 y W-S26. Los identificadores P01–P13 sólo viven dentro de una instancia de campaña.

## Dependencias y referencias

La copia de una pieza no sustituye el cotejo del conjunto. P07/P08 alteran state.bin antes o después de ReceivedBytes; P13 declara en la solicitud la huella correcta del estado alterado, manteniendo la custodia autorizada original. Se espera rechazo I03 cuando la dependencia recibida no coincide con la custodia; una mutación posterior no cambia la copia recibida.

El registro R05 ya contiene A-r1 y A-r2. P09 recibe explícitamente r2 en solicitud, custodia y captura, con los mismos bytes de estado y geometría: es el control positivo de esa otra revisión. No se deduce r2 de un hash. P10/P12 mezclan referentes en la entrada; P11 ofrece a una admisión r1 una captura r2 de bytes idénticos. P11 examina la certificación posterior a una captura documental: no acredita rechazo antes de todo consumo ni bloqueo de un efecto externo.

El recibo esperado de r1 es el literal del oracle.json heredado. Para el control r2 sólo se sustituyen previamente los campos revision de vinculo y contexto_entrega por el literal r2; todos los demás campos deben conservarse. Ningún resultado del admisor genera su propio oráculo.

## Oráculos previos

| Caso | Estímulo | Resultado exigido |
|---|---|---|
| P01 | Control A-r1 sin escritura | Informe aceptado; captura y recibo literales R05. |
| P02 | Primer byte de geometría modificado después de admit | Mismo objeto, lectura final B; consumidor recibe A y recibo literal. |
| P03 | A–B–A después de admit | Muestra B distinta, extremos A iguales; consumo A. Sin acreditación de continuidad. |
| P04 | A–B–A con perfil de continuidad fijado antes | NO_EJECUTADO; continuidad_soporte_no_ofrecida; sin captura ni recibo. |
| P05 | Modificar byte aún no leído y restituir A | Búfer recibido distinto pese a extremos iguales; primera guarda M01, sin consumidor. |
| P06 | Modificar byte ya leído y restituir A | Muestra intermedia distinta; búfer A y recibo literal aceptados. No continuidad demostrada. |
| P07 | Estado alterado antes de recepción | Primera guarda I03; sin consumidor; estado material B conservado. |
| P08 | Estado alterado después de recepción y antes de admit | Copia recibida A válida; recibo literal y captura A; archivo de estado B conservado. |
| P09 | Conjunto explícito A-r2 con los mismos bytes | Informe y recibo r2 aceptados con referencias concordantes. |
| P10 | Solicitud r2 frente a custodia r1 | Primera guarda I02; sin consumidor. |
| P11 | Captura r2 para admisión r1, bytes idénticos | RETORNO_ERROR; D04: capture context mismatch; sin recibo aceptado. Captura conservada. |
| P12 | Vínculo r1 y entrega r2 en la solicitud | Primera guarda A01; sin consumidor. |
| P13 | Estado B y su hash correcto propuesto frente a custodia A | Primera guarda I03; sin consumidor. Coherencia autopropuesta no concede autoridad. |

Cada caso comprueba además identidad de soporte en todas las muestras, contenido inicial, contenido final previsto y estado material de la dependencia. Las discrepancias se conservan y la campaña continúa con los otros casos; resultado.json exige trece conformidades para salida 0. Una discrepancia produce salida 1. Un error instrumental interrumpe con salida 2 e interrupcion.json cuando esa escritura es posible, conservando los archivos ya producidos. No hay repetición automática.

## Recursos y límites

ReceivedBytes mantiene sus cuotas: metadatos 4096, fuente 4096, estado 1024, soporte 1024, geometría 8192 y agregado 16384 bytes, con lectura límite+1. El lector instrumental conserva como máximo 8193 bytes recibidos. Lecturas auxiliares y escrituras de inyección se acotan a 8192; estados y su variante a 1024. Hay trece casos secuenciales, una instancia de Session de hasta 32 identificadores y hasta 32 barreras por caso. Las evidencias JSON se limitan a 262144 bytes por archivo; la geometría consumida queda también en binario.

Estas cotas no son una medición de RSS ni una garantía global de RAM, asignación, latencia u OOM. No se añade unsafe. Se presupone un anfitrión confiable y custodia de directorios; no se ensayan corrupción física de RAM, atacante con acceso al proceso, concurrencia hostil, muerte del observador, fallo del host, disco lleno o pérdida de energía. sync_all de la evidencia no acredita durabilidad.

El campo documental continuidad_no_ofrecida_por_el_montaje describe el límite declarado; no es un detector de todas las mutaciones. P04 contrasta la negativa ejecutable a ofrecer ese perfil. En P03/P05/P06 las muestras B proporcionan contraejemplos a inferir inmutabilidad a partir de dos lecturas iguales.

No existe acción externa ni despacho productivo en el banco. Las guardas de admisión/certificación conservan su sede; el arnés no produce Tri.U ni estados R1/R2 nuevos. No se selecciona BD, GUI, host ni tecnología persistente. T07 se recibe sólo en este alcance local; integración completa y casos globales S26 permanecen pendientes.

## Reproducción después del precompromiso

Desde esta carpeta en el repositorio del Lenguaje, con Rust/Cargo 1.98.0 en PATH:

~~~text
cargo build --offline --locked --target-dir /tmp/s26-t0701-build
/tmp/s26-t0701-build/debug/sv_s26_r06_t0701 ../../r05/fixture /tmp/s26-t0701-debug
cargo build --release --offline --locked --target-dir /tmp/s26-t0701-build
/tmp/s26-t0701-build/release/sv_s26_r06_t0701 ../../r05/fixture /tmp/s26-t0701-release
~~~

La compilación preparatoria no ejecuta los casos. Los dos perfiles se ejecutarán sólo después de publicar y cotejar el banco. Se conservarán logs, salidas y huellas del código, dependencias y binarios. El espejo custodia las mismas piezas; sus rutas relativas no constituyen un segundo árbol compilable.

Las pruebas y sus controles son Rust. Shell sólo construye, lanza y transporta archivos; el conector publica y el auxiliar Rust R08 coteja los árboles completos. El editor Rust de Sucesos actualiza exclusivamente sus tres archivos y conserva el historial. Cero Python/Java; sin nueva rama ni escritura de RETP canónica. S26, Bis y la secuencia hacia S24 conservan sus estados.
