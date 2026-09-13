# Realización nativa C02–C05 · BIS-04 · 0.1

Juan Antonio Lloret Egea y Watson · S22 · RETP-2026-220 · 13 de septiembre de 2026.

**Implementada, compilada y ensayada en Rust.** En las dos campañas conservadas, los 26 casos integrados coinciden con los oráculos comprometidos: 7 entregas documentales concordantes, 17 rechazos y 2 entregas no acreditadas. Ningún esperado se ha modificado. BIS-04 queda en ejecución acotada; el workflow completo sigue abierto.

## Evidencia y alcance

| Grupo | Resultado observado | Alcance |
| --- | --- | --- |
| JSON J01–J14 | 14 conformes: 6 admisibles, 8 rechazos | Unicode, claves duplicadas, enteros exactos, límites de profundidad/valores/miembros |
| SHA H01–H05 | 5 huellas exactas | Vacío, abc y fronteras de bloque 55/56/64 bytes |
| I0205-01–26 | 26 conformes | Fuente→IR→soporte→identidad→estado→geometría→captura local |
| v1→v2→v1 | rechazo S02 → admisión → rechazo S02 | 3 invocaciones, 2 variantes, mismo proceso; sin caché en esta realización |
| OBS-01–04 | 4 evidencias defectuosas rechazadas | Contexto ajeno, éxito fabricado, captura ausente, fallo convertido en U |
| ENC01–08 | 7 errores de compilación esperados y 1 lectura compilada | Privacidad, ausencia de crecimiento, sustitución y Default |
| Test de lectura agregada R01 | Conforme en segunda campaña | Saldo 6144; lectura detenida en 6145 bytes incluyendo detector de exceso |
| Test interno de almacenamiento | 1 test, 5 comprobaciones conformes | Longitud 15 rechazada; 49 rechazado en v1; arrays 16/25/49 conservados con soporte compatible |
| Lectura de objeto admitido | Conforme | Una invocación adicional de I0205-01; no variante nueva |

Se ejecutaron **60 invocaciones de admisión en dos campañas**, 30 por campaña: 26 casos, 3 repeticiones secuenciales y 1 lectura adicional. Las cuatro sensibilidades modifican evidencia ya producida; no son cuatro ejecuciones nuevas del sujeto. La segunda campaña repite las ocho sondas y añade el test de saldo agregado: dos tests internos conformes. Los 202 casos originales C02–C12, los 24 escenarios originales y las 13 variantes históricas C01 mantienen su contabilidad independiente.

[RESULTADOS.json](RESULTADOS.json) reúne guardas, recibos y conteos. [EVIDENCIA_NATIVA.tar.gz](EVIDENCIA_NATIVA.tar.gz) conserva los registros completos, diagnósticos, órdenes, códigos de salida, estados antes/después y bytes de cada captura. [HUELLAS_EVIDENCIA.json](HUELLAS_EVIDENCIA.json) identifica los contenidos del archivo. La versión corregida y final está acreditada por [EVIDENCIA_NATIVA_R01.tar.gz](EVIDENCIA_NATIVA_R01.tar.gz) y [HUELLAS_EVIDENCIA_R01.json](HUELLAS_EVIDENCIA_R01.json). Los resultados esperados siguen en sus expedientes previos; sus campos observado permanecen vacíos por conservación histórica.

## Comprobaciones que importan

I0205-05 deriva 49 de una CellSpec no seleccionada y la rechaza bajo soporte v1. I0205-06 rechaza un cuerpo de soporte alterado aunque su huella propia sea correcta. I0205-07/08 distinguen nodo/instancia y revisión aun compartiendo bytes. I0205-09 llega a M02: falla la geometría matemática, antes de comparar el hash geométrico esperado. I0205-22 conserva la precedencia P02 frente a soporte ausente. La cuota admite 8192 bytes y rechaza 8193; la entrega conserva los espacios finales del descriptor admitido.

I0205-12 y 25 mantienen **NO_ACREDITADO**, despacho observado desconocido y un intento registrado; no afirman cero efectos ni convierten el fallo en U. Los rechazos D02–D06 conservan el despacho local ocurrido. La nota imperativa de I0205-17 permanece documental; no se interpreta ni crea permisos.

El sujeto usa `sv_core::compile_svp_profile` con perfil explícito y recibe buffers propios acotados. El conductor selecciona el contexto externamente. `AdmittedDelivery` tiene campos privados y un enum de arrays; la API pública devuelve slices de lectura. El captor observa el buffer recibido. El observador está en módulo separado y no llama a admit/certify para decidir conformidad.

## Reproducción

En Linux x86_64 con Rust/Cargo 1.98.0 disponibles en PATH y Python 3.12 o posterior para administrar los archivos, ejecutar desde este expediente:

```sh
python soporte/reproducir.py /tmp/sv-i0205-reproduccion
```

El directorio de destino debe ser nuevo. [El script](soporte/reproducir.py) extrae las entradas y la copia verificada de sv_core, comprueba sus huellas y ejecuta Cargo con `--locked --offline`, las sondas rustc y el binario Rust. Python no interpreta SVP ni decide las admisiones. No descarga paquetes ni necesita el equipo del autor. La primera campaña utilizó ejecutar.py/continuar.py. La segunda utilizó este mismo script de reproducción desde un directorio nuevo y terminó correctamente; se conservan las órdenes literales de ambas.

[ENTRADAS.tar.gz](ENTRADAS.tar.gz) conserva los bytes suministrados al conductor; [ENTRADAS_MATERIALIZADAS.json](ENTRADAS_MATERIALIZADAS.json) permite cotejarlos. [NUCLEO_FUENTES.json](NUCLEO_FUENTES.json) identifica todos los archivos del núcleo copiado y su corte. [Cargo.lock](proyecto/Cargo.lock) fija sólo el montaje y sv_core, sin dependencias de registro.

## Cortes, errores y recursos

Corte de entrada del Lenguaje: `2067310b30e5aee76120444f0166b01390a04970`; laboratorio: `4705ba49c917cb525eeeba4b39f16686afe1e255`. Fuentes y sondas fijadas antes de ejecutar en laboratorio `ce31385d525c2ed9548c33a10aea208cafc45046` y espejo público `2bce84a27f81b92a78ba71237a83de1f67895444`. PRECOMPROMISO.json es una instantánea histórica de ese corte: conserva la identidad del código anterior y el README previo, no pretende ser un manifiesto del expediente final. La corrección R01 y su prueba adicional quedaron fijadas antes de repetir en laboratorio `6ece56f9684f6b4a1045d954066301d64ac96129` y público `d9cea830eacc3a0f808ba88725a514867c57e643`; ADENDA_R01_PREVIA.json conserva ese esperado sin rellenarlo retroactivamente. MANIFIESTO.json identifica el expediente final.

Se conservan tres incidencias de construcción/entorno: faltaba Cargo.lock en el primer intento --locked; se generó offline. El segundo intento detectó una llave ausente en main.rs; se conserva la fuente fallida y el diagnóstico. La construcción siguiente fue correcta. Finalmente faltaba /usr/bin/time; la campaña aún no había arrancado y se sustituyó la medición por os.wait4 del PID del binario. La revisión posterior detectó que el saldo agregado se comprobaba después de leer cada canal. Se corrigió la reserva/lectura para limitarla al saldo antes de recibir; una prueba nativa nueva exige detenerse en 6145 bytes cuando quedan 6144. La versión previa de lib.rs se conserva. No se alteraron los bancos para corregir estas incidencias. El núcleo emitió 25 advertencias heredadas, conservadas en los logs; el montaje no añadió advertencias de compilación.

En la primera ejecución medida del binario completo (conductor, datos confiables, sujeto, observador y escritura de evidencia), el sistema informó **10368 KiB de RSS máximo** y **103721543 ns de duración externa**. El intervalo interno fue 100870607 ns. En la segunda campaña, correspondiente a la versión final, se observaron **12032 KiB de RSS máximo**, **106749319 ns externos** y **104407621 ns internos**. Los costes incluyen conductor y observador; no son cotas del núcleo. `size_of::<FixedVector>()` dio **49 bytes** en esta compilación; no incluye el objeto admitido completo ni el proceso. Se efectúa la copia desde el slice IR al array al construir; no se ha medido el número de copias optimizadas ni se acredita ventaja frente a otro almacenamiento. Estos valores son observaciones de laboratorio, no límites constituidos ni una prueba de viabilidad temporal o de memoria.

## Fuentes rectoras y límites pendientes

Se revalidaron las huellas de todos los archivos de FUENTES.json de RETP-219. Se reutilizan las lecturas completas previas de Pilares y restricciones (05/09), Perfiles/contratos/ensamblaje (06/09), Transición secuencial IMM con sus adendas, workflow V2 y contratos integrados. En este incremento se revisaron AGENTS.md, INTERFAZ.md, CONTRATO_INTEGRADO.md, los auxiliares JSON/SHA completos, las piezas públicas de API/IR necesarias y los datos literales comprometidos. No se atribuye lectura completa nueva a todos los fuentes del núcleo.

JSON/SHA adaptan recepción-av de IE004: sin reglas especiales nodos/hijos, sin marco SVAC y con límites del banco instrumental. La fuente Rust nueva incluye comentarios ES/EN. Servicios y conectores durante la campaña: ninguno; publicación administrativa mediante el conector GitHub y sincronización Git, fuera del sujeto.

Esta realización es un **experimento de entrega documental local**. La custodia es la del conductor, no autenticación de un agente externo ni aislamiento ante un host comprometido. El observador comparte el auxiliar JSON/SHA y sus tipos de evidencia; sus cuatro sensibilidades no garantizan independencia absoluta. No se acreditan imagen renderizada, consumo visual o comprensión por IA, dominio operativo, cierre de DFL-005, agotamiento de memoria, todas las cuotas de salida ni paridad ES/EN integrada. El núcleo productivo, gramática e IR permanecen intactos.

Siguiente objeto: preparar la extensión a paridad ES/EN integrada y a los controles de recursos/captura aún no cubiertos, con banco previo propio. No se cierra globalmente BIS-02/BIS-03 ni se adelantan catálogo y GUI; S24 y S25 conservan sus estados.
