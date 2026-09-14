# S26 R06 · LOCAL01 · Precompromiso de recepción y consumo

Estado de este corte: banco preparado; ninguna ejecución del banco. La recepción posterior se documentará en RESULTADOS.md sin alterar estos oráculos.

Entrada canónica: d589123304905a9162148e73946914382dba995d. Espejo: a0f4aba91af549e80cfaf3daf6379dc05d158204, rama lab/playground-sv-permanente. Autorización humana: continuar R06. R06 es una pieza de S26, no el sexto apartado ni el cierre de la fase R0.

## Necesidad y reutilización

Se realiza el primer incremento local T01/T02/T05/T06 de ../README.md §5. R05 dejó visibles la pérdida del informe por pánico y la diferencia entre igualdad de bytes e identidad de soporte. Se reutilizan ReceivedBytes, TrustedContext, TrustedRegistry, admit, AdmittedDelivery y certify de Bis I0205, así como el fixture I0205-01 de R05, su geometría literal y oracle.json. No se reinventa el registro soberano, la identidad SV ni la certificación.

El manifiesto local enlaza las fuentes Bis existentes con rust/sv_core del corte de entrada, cotejadas en FUENTES.json. Esta compilación usa el núcleo vigente; no suplanta ni reescribe las campañas históricas realizadas sobre otro corte. Pilares, perfiles/contratos/ensamblaje y transición secuencial se han consultado completos, además de R02, R06, R2-0 y LIG/0.1. No se cambia núcleo, IR, constitución del dominio, cobertura del agente ni contrato productivo.

## Frontera material

El arnés carga primero custodia y registro desde el fixture fijado. Recibe cinco piezas con los límites de ReceivedBytes; admit comprueba su coherencia con la custodia. Conserva un objeto admitido con buffers propios. El consumidor local copia el descriptor prestado por ese mismo objeto. Las sustituciones de ruta ocurren después de admit y antes de esa copia. No hay reapertura de geometría para producir la captura.

La apertura se constituye antes del recorrido: emisor arnés-R06-local01, ámbito una instancia de campaña local, identificadores P01…P12. Máximo 32 identificadores, 64 bytes ASCII por identificador; duplicados y vacío rechazados. La correlación sólo vive en RAM en esa instancia; no es identidad global, persistente, fuente de autoridad ni permiso de reintento. La apertura conserva el encargo con su operación, invocación y referentes; run coteja sus referentes con los del objeto admitido antes del consumo.

Informe final y observación de terminación son piezas distintas. La envoltura local tiene magia/versionado de 8 bytes, longitud de cuerpo u32 big endian y JSON de dos campos: intento y recibo. Longitud exacta, cuota, correlación y concordancia con certificación local se exigen conjuntamente; EOF no basta. El oráculo del banco compara además con el recibo literal heredado, independiente del recibo generado.

catch_unwind rodea el recorrido dentro del mismo proceso, con panic=unwind. Apertura, barreras y captura se conservan fuera de la clausura que puede entrar en pánico. No sobrevive abort, agotamiento fatal, muerte de proceso/host ni corrupción arbitraria de RAM. El captor es una frontera de copia local controlada, no GUI, sensor o tercero independiente frente al host. P06 escribe realmente en state.bin antes del pánico: la relectura posterior del arnés debe conservar ese efecto y no atribuir rollback.

El perfil queda fijado en la apertura. ContentRefs ofrece concordancia local de contenido y referentes; SupportContinuity se declara no acreditado y no despacha. Un inode se observa sólo como testigo POSIX local de sustitución, con el original abierto; no se convierte en identidad SV ni obligación universal.

## Oráculos previos

| Caso | Discriminador y esperado literal |
| --- | --- |
| P01 | T01 control: RETORNO, INFORME_LOCAL_ACEPTADO, captura geométrica y recibo iguales a fixture/oráculo R05. |
| P02 | T01 informe ausente: RETORNO e informe_ausente; captura previa conservada; sin recibo aceptado. |
| P03 | T01 quitar último byte: informe_incompleto_o_sobrante; retorno no equivale a éxito. |
| P04 | T01 informe de INTENTO-AJENO: informe_otro_intento; apertura P04 conservada. |
| P05 | T02 pánico antes de despacho: barrera observada, PANICO_OBSERVADO, captura e informe ausentes. |
| P06 | T02 pánico después de captura y escritura real: captura literal conservada, informe ausente; relectura encuentra efecto_local_antes_panico. |
| P07 | T05 rename de geometría diferente tras admit: dev/inode difieren; ruta releída contiene contenido_sustituido; consumidor recibe original y recibo literal. |
| P08 | T06 rename con geometría igual tras admit: objeto distinto, bytes iguales; perfil mínimo admite captura/recibo literal. |
| P09 | T06 perfil superior fijado antes: continuidad_soporte_no_ofrecida, NO_EJECUTADO, ninguna captura ni recibo. |
| P10 | T01 defensa adicional: recibo sustituido por null, recibo_no_concordante. |
| P11 | T01 defensa adicional: informe de 4097 bytes, informe_fuera_cuota. No es T08 de proceso. |
| P12 | Defensa de ligadura: apertura con revisión ajena, apertura_no_ligada_al_admitido antes de ejecutar. |
| P13 | Correlación instrumental: rechazar intento duplicado, vacío y número 33. No es T04 de informes tardíos. |

Cada caso conserva observacion.json con bytes recibidos, captura, barreras, identidad instrumental, relecturas y comprobaciones. resultado.json exige todos los oráculos; una discrepancia queda conservada y provoca salida 1. Fallos de infraestructura abortan la campaña con diagnóstico, sin contabilizar conformidad. Los dos pánicos deliberados imprimen el hook estándar; su captura se decide por evidencia, no por ausencia de stderr.

## Recursos, autoridad y límites

Entradas de ensayo leídas hasta límite+1: archivos auxiliares/custodia 8192 bytes por pieza; state 1024; ReceivedBytes conserva cuotas por canal y agregado 16384. Informe admitido <=4096; el canal controlado conserva hasta 4097 como centinela de exceso. Una captura <=8192 y recibo <=2048 conforme al admisor. Salida documental <=262144 por archivo; campaña de 13 casos en carpeta nueva. Son cotas locales de estas piezas, no límite global de RAM/latencia ni resistencia a denegación de servicio. Directorios y fuentes del arnés bajo custodia del operador, sin escritor concurrente hostil salvo inyecciones definidas. Escritura y sync_all de evidencias no certifican durabilidad frente a pérdida de energía.

La fuente canónica del fixture y de la expectativa está fijada antes del ensayo. Comparar hashes o JSON coherentes no acredita autoridad por sí mismo. La certificación aceptada es sólo de entrega documental local; efecto externo permanece NO_ACREDITADO. No existe ejecución de acciones dependientes, reintento automático, recuperación o transacción productiva. La protección frente al pánico de la propia construcción/serialización del observador queda fuera de la frontera.

T03, T04, T07 y T08 siguen pendientes con sus protocolos y barreras propios. Cero casos globales S26 cerrados; R2 material, host, RAM física, persistencia, BD y GUI no acreditados. S22/Bis → catálogo/cierre → S24 conserva el orden.

## Ejecución reproducible después de publicar este banco

Desde esta carpeta en el repositorio canónico:
```text
/opt/sv-rust-1.98.0/bin/cargo build --offline --locked --target-dir /tmp/s26-r06-local01-build
/tmp/s26-r06-local01-build/debug/sv_s26_r06_local01 ../../r05/fixture /tmp/s26-r06-local01-debug
/opt/sv-rust-1.98.0/bin/cargo build --release --offline --locked --target-dir /tmp/s26-r06-local01-build
/tmp/s26-r06-local01-build/release/sv_s26_r06_local01 ../../r05/fixture /tmp/s26-r06-local01-opt
```

Se contrastan ambos perfiles para comprobar que las guardas no dependen de debug_assertions. El espejo conserva piezas, pero no constituye un segundo árbol de compilación con estas rutas relativas. Cero Python. Shell sólo lanza compilador, ejecutables y transporta bytes. La candidata auxiliar Rust R08 prepara/comprueba registros y coteja los árboles remotos completos; el conector GitHub publica sin force. Sus límites de autoridad, frescura y ausencia de CAS permanecen.

