# S26 R06 · INTEGRADO01 · Recepción, mutación, consumo y terminación

**Precompromiso: diecisiete casos preparados; ninguna ejecución al publicar este banco. Unidad: Watson / W-S26.**

Entrada del Lenguaje: fbc91824bdf57901c0e24236296f09b82fc661bd, rama main. Laboratorio: b5d19ec1f4abbe13ed1b520eb8363038487df7b1, rama lab/playground-sv-permanente. Se continúa desde S26/revisión 20 y la recepción de T0701. El resultado se incorporará después, conservando este precompromiso.

## Obligación y realización

LOCAL01 contrastó apertura y consumo local; PROCESO03, observación de un hijo y canal; T0701, modificaciones del mismo archivo y concordancia de referencias. Sus resultados no acreditaban que esas fronteras conservaran sus obligaciones al conectarse. INTEGRADO01 fija ese recorrido documental y sus cortes dentro de un anfitrión confiable.

El padre conserva previamente el encargo, el perfil y dos identificadores instrumentales autorizados. Crea cinco archivos de entrada por caso y mantiene abierto un testigo de geometría. El hijo recibe las piezas en ReceivedBytes, ejecuta admit y entrega el objeto al consumidor LOCAL01 sin modificar ese consumidor. El padre provoca las mutaciones mientras el hijo espera un acuse explícito; después recibe fragmentos de captura, informe o rechazo y observa la terminación real del proceso.

Se reutilizan literalmente admisor, certificador y consumidor de LOCAL01. src/main.rs adapta el observador PROCESO03: envoltura propia R06I01/1, cuota de 24 mensajes, rechazo separado, barreras parentales, testigo de archivos y entrada al hijo integrado. src/integrado.rs contiene los escenarios y oráculos. El código de los bancos anteriores permanece intacto. FUENTES.json coteja 112 piezas con el corte de entrada, incluidos los tres rectores leídos completos en esta continuidad, R06, R2-0, LIG, fuentes Rust y fixture R05.

## Fronteras observadas

1. El padre conserva apertura y plan antes de crear el hijo. El hijo comprueba que su intento figure en la apertura, que el encargo coincida con la custodia fijada y que el perfil esté declarado.
2. Antes de recibir, el hijo emite la barrera antes_recepcion y espera. El padre puede cambiar una dependencia conforme al caso.
3. ReceivedBytes recibe las cinco piezas. En I07/I08, un lector acota el primer tramo de geometría a 3072 bytes y emite prefijo_leido antes de recibir el resto. La escritura material la realiza el padre, en otro proceso, mientras el lector espera.
4. El hijo conserva los bytes efectivamente recibidos y emite piezas_recibidas. El padre puede restituir geometría o modificar el estado después de la copia.
5. Tras admit, el hijo conserva el descriptor admitido y emite admision. El padre puede sustituir la ruta, modificar el mismo objeto o ejecutar A–B–A.
6. El consumidor LOCAL01 recibe el objeto admitido y efectúa la captura. La certificación local sigue su contrato. La captura y el estado local se conservan también en archivos del hijo, dentro del alcance de custodia declarado.
7. El protocolo transmite la captura fragmentada y sus referentes. Un recibo sólo se acepta con captura concordante, correlación correcta, contenido esperado y ausencia de conflicto o rechazo incompatible.
8. El padre conserva el canal y una instantánea de frontera antes de liberar o terminar administrativamente al hijo. EOF, truncamiento y plazo conservan su diferencia respecto a una terminación observada.

Los bytes retenidos en RAM se prueban mediante su entrega efectiva y cotejo con el oráculo. No se inspecciona ni se acredita integridad física de RAM. dev/inode se utilizan como testigos locales del objeto abierto y de la ruta, sin convertirlos en identidad SV. Las muestras del padre incluyen objeto abierto, ruta actual y dependencia de estado.

## Informe, rechazo y conflicto

El mensaje rechazo transporta tres textos: etapa, guarda y causa. Es una declaración ligada al intento; no constituye un recibo favorable ni acredita por sí sola la verdad de lo declarado. Se conserva separado de la captura y del primer informe. Si coexiste con un informe favorable del mismo intento, queda marcado el conflicto y el intento no se acepta, cualquiera que sea el orden.

I16/I17 inyectan expresamente una declaración adversarial I03 incompatible con un recibo favorable. Ese I03 no representa un rechazo realmente emitido por admit en esos casos. El banco exige conservar la contradicción y rechazar la aceptación; no reinterpreta la declaración inyectada como hecho material de admisión.

La certificación local del hijo y el informe que finalmente recibe el padre son piezas distintas. En I03/I04 puede haberse completado una certificación interna antes de que el proceso muera sin transmitir su informe. En I12 se captura un pánico dentro del hijo: la captura y una escritura anteriores sobreviven, el hijo retorna 0 y el padre sigue sin aceptar un recibo.

I09 conserva una captura local de bytes correctos con referencia r2 para admisión r1. La certificación local rechaza por D04 y el receptor del canal rechaza captura_no_concordante. Los fragmentos recibidos y los bytes del mensaje se conservan; no se atribuye una captura aceptada.

## Oráculos previos

| Caso | Composición fijada | Oráculo del padre y evidencia material |
|---|---|---|
| I01 | Recorrido completo sin mutación | EOF, salida 0, captura A y recibo literal aceptados; estado inicial intacto. |
| I02 | Sustituir la ruta por B tras admit | Objeto de ruta distinto; testigo original A; consumo A y recibo aceptados, ruta final B. |
| I03 | Modificar el mismo objeto tras admit, capturar, escribir y abortar | Captura A conservada, archivo B, escritura local conservada; SIGABRT 6; sin informe. |
| I04 | A–B–A sobre el mismo objeto, capturar, escribir y terminar desde el padre | Muestras B y restitución A conservadas; captura A; SIGKILL 9 en barrera; escritura presente, sin informe. |
| I05 | Modificar estado antes de recibir y abortar después del rechazo | Rechazo I03 ligado antes del aborto; SIGABRT 6; sin consumidor ni captura; estado B. |
| I06 | Modificar estado después de ReceivedBytes; captura válida e informe truncado | Copia A consumida, archivo de estado B; mensaje_truncado; hijo aún vivo en frontera y salida 0 tras liberación; sin recibo. |
| I07 | Cambiar último byte pendiente durante lectura; restituir A antes de admit | Geometría recibida con signo de exclamación final; rechazo M01; extremos A iguales con muestra B; sin consumidor; salida 0. |
| I08 | Cambiar primer byte ya leído; restituir A; transmitir informes contradictorios | Geometría recibida A y muestra intermedia B; captura válida; primer informe conservado y conflicto, sin aceptación. |
| I09 | Mutación tras admit; captura r2 para admisión r1 | Bytes A conservados; D04 local y captura_no_concordante en el padre; sin captura aceptada ni recibo; hijo vivo hasta liberación, salida 0. |
| I10 | Sustitución tras admit; informe A tardío y segundo intento programado B | A se conserva separado; B realiza nueva recepción/admisión/consumo desde entrada A restituida y obtiene su propio recibo; selección B y salida 0. |
| I11 | Sustitución por bytes iguales bajo perfil de continuidad previamente solicitado | Objeto de ruta distinto con bytes A; continuidad_soporte_no_ofrecida, sin consumidor ni recibo; salida 0. |
| I12 | Mutación tras admit; escritura y pánico dentro del consumidor | Captura A y escritura conservadas; PANICO_OBSERVADO local separado del proceso que sale 0; sin recibo aceptado. |
| I13 | Mutación tras admit; captura transmitida y espera sin informe | Captura A conservada; plazo_agotado con hijo vivo; SIGKILL 9 administrativo posterior; sin recibo ni ausencia de efecto inferida. |
| I14 | Sustitución tras admit y envío de informe AJENO | Captura A preservada; informe ajeno sin atribución, sin aceptación; salida 0. |
| I15 | Sustitución por bytes iguales bajo perfil mínimo e informe duplicado | Objeto distinto, contenido A válido; un duplicado concordante, primer informe intacto, aceptación y salida 0. |
| I16 | Mutación tras admit; recibo favorable seguido de rechazo adversarial | Ambos mensajes conservados en ese orden; primer informe literal, conflicto y aceptación bloqueada; salida 0. |
| I17 | Mutación tras admit; rechazo adversarial seguido de recibo favorable | Ambos mensajes conservados en orden inverso; el recibo posterior no elimina el rechazo; conflicto y aceptación bloqueada; salida 0. |

Se cotejan literalmente geometría y recibo del oracle.json de R05, independientemente de la certificación producida. Cada caso verifica además apertura conservada, objeto testigo, identidad de ruta prevista, estado material final, muestras de mutación y bytes efectivamente recibidos/admitidos/consumidos, según corresponda.

I10 tiene dos intentos documentales programados desde la apertura. El segundo no se activa para reintentar un efecto incierto: forma parte del estímulo previo de correlación y ejecuta su propia recepción. No se limita a cambiar el identificador de una captura anterior. No hay acción productiva ni efecto externo en ninguno de los intentos.

## Recursos y cobertura

El padre observa un hijo por caso y un hilo lector de stdout. Las campañas son secuenciales. El canal de control usa acuses G, tuberías anónimas heredadas y ninguna conexión de red. Se conserva el patrón de transferencia exclusiva de fd 0/1 de PROCESO03; sus dos from_raw_fd en la rama ejecutada de hijo están justificados por los descriptores recién creados por Stdio::piped.

Se mantienen 8192 bytes por cuerpo, hasta 24 mensajes, 65536 bytes de canal más un centinela y fragmentos de captura de hasta 3072 bytes. La captura reconstruida no supera 8192. El fixture geométrico es ASCII de 6237 bytes; el fraccionamiento no acredita tratamiento general de cortes UTF-8. El hilo lector transfiere bloques de 512 bytes mediante sync_channel de capacidad 1.

ReceivedBytes conserva sus cuotas por pieza y agregado 16384. Las lecturas instrumentales se acotan a 8192, estados/efectos a 1024; el lector interpuesto retiene como máximo 8193 bytes. Por caso se admiten hasta 12 muestras parentales y 24 barreras. Los archivos documentales se limitan a 1 MiB. El stderr del hijo conocido se conserva en archivo; no se afirma una cuota ejecutable de escritura de stderr frente a un hijo arbitrario.

El plazo de recepción es 1500 ms; I13 fija 150 ms desde la barrera espera_sin_informe. La espera final se limita a 2 s. Son presupuestos instrumentales del ensayo, no umbrales productivos ni métricas de viabilidad. Un fallo inesperado conserva los restos y no provoca repetición automática. Guard termina al hijo si una salida excepcional deja al proceso activo; eso es limpieza del arnés, no compensación del efecto.

Las escrituras ocurren en archivos locales del ensayo. sync_all y la supervivencia del padre no acreditan durabilidad, supervivencia del anfitrión, resistencia a corrupción física de RAM, concurrencia hostil arbitraria ni autoridad externa. No se mide consumo global de RAM/CPU ni se declara protección ante OOM. El padre y el hijo comparten un anfitrión confiable y una custodia de directorios; la separación de procesos no protege frente a su compromiso.

## Ejecución y recepción

Con Rust/Cargo 1.98.0 en PATH, desde esta carpeta y sólo después de publicar:

~~~text
cargo build --offline --locked --target-dir /tmp/s26-integrado01-build
/tmp/s26-integrado01-build/debug/sv_s26_r06_integrado01 ../../r05/fixture /tmp/s26-integrado01-debug
cargo build --release --offline --locked --target-dir /tmp/s26-integrado01-build
/tmp/s26-integrado01-build/release/sv_s26_r06_integrado01 ../../r05/fixture /tmp/s26-integrado01-release
~~~

El perfil release desactiva debug-assertions. El banco no utiliza assert para las guardas de aceptación. Una discrepancia conserva sus controles y produce salida 1 al concluir la campaña; un impedimento excepcional puede interrumpir antes, sin recuento favorable de lo no ejecutado. Se conservarán las campañas y sus causas reales antes de cualquier variante.

Precompromiso y resultados se enlazan desde Sucesos, CSV/Markdown e historial, con espejo en laboratorio. Para seguimiento se consultan las ramas vigentes; los enlaces con hash identifican cortes históricos inmutables. RETP canónica permanece fuera de escritura y no se crea ninguna rama. Las pruebas son Rust; shell sólo construye, lanza y transporta evidencias. No se ejecuta Python/Java.

Este banco contrasta diecisiete composiciones declaradas. No cubre todas las combinaciones posibles ni cierra los doce casos globales S26, la fase R0, R2/R3/R4, Bis o S24. No se constituye BD, GUI, nuevo almacén, host productivo, acción dependiente de efecto ni reintento. La recepción deberá distinguir recorrido documental acreditado, fronteras no cubiertas y siguiente obligación material.
