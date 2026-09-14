# Fronteras conjuntas de descriptor y recibo · banco previo

**Fecha: 14/09/2026. S22; alcance experimental C02–C05. Estado: preparado, sin ejecutar.**

## Fuente, obligación y alcance

Corte de entrada: Lenguaje `1d577e7dffd9bad8d73cd779105c8730301bea0a`; laboratorio `2e2fc695e4e718999f471c7b4fa8520af8138c2f`. Se han consultado completos AGENTS.md, Pilares, acta de perfiles/contratos/ensamblaje, acta de transición y secuencia, workflow Bis V2 y contrato integrado. Rigen [PRESUPUESTO.json](../integracion-c02-c05-v0_1/PRESUPUESTO.json), los límites del adaptador y el relevo de RETP-221.

La recepción de [F01/F02](../../../../riesgos-materiales-s26/r06/CONTINUIDAD_F01_F02_Y_RELEVO_BIS.md) conserva pendiente la consulta histórica completa. Este banco desarrolla las fronteras conjuntas ya previstas en S22; no constituye una consulta de Frame.

Se reutilizan literalmente las semillas ES01 y ES02 de [ENTRADAS_R01.tar.gz](ENTRADAS_R01.tar.gz), SHA-256 `58a4cebb91f024ac1b379e037eead91d1b8d25221607911fbc75089bec440f25`. El estado y la geometría originales proceden del contrato integrado. Se amplían exclusivamente espacios finales de geometría y el campo de consumidor, con referencias y expectativas fijadas antes de ejecutar. Se preservan fuente, constitución, posiciones, vector, leyenda y operaciones.

## Hipótesis y oráculos

El presupuesto exige descriptor ≤ 8192 bytes, recibo ≤ 2048 bytes y suma ≤ 10240 bytes. Con ambos límites individuales activos, el exceso de la suma es inalcanzable por esta interfaz: `8192 + 2048 = 10240`. La guarda agregada permanece defensiva; no se declarará ejecutada su rama de rechazo por alcanzar un rechazo individual anterior.

Cada fila se ejecutará una vez con semilla EN y otra con ES; doce variantes por perfil de compilación. Se prevén dos perfiles de compilación: sin optimización y con optimización 3, `debug-assertions=no`, `overflow-checks=no`. Son seis escenarios de este banco; no modifican los recuentos históricos de Bis.

| Variante | Descriptor | Recibo propuesto | Resultado previo |
| --- | ---: | ---: | --- |
| FS01 | 8191 | 2048 | Entrega literal; suma 10239. |
| FS02 | 8192 | 2048 | Entrega literal; suma 10240. |
| FS03 | 8192 | 2049 | R01, causa `output receipt BYTES`, después de M03; cero despachos. |
| FS04 | 8193 | 2048 | R01, causa `BYTES`, durante recepción; traza de admisión vacía, cero despachos. |
| FS05 | 8192 | 2048 | Consumidor UTF-8 con «ñ»; entrega literal, suma 10240. |
| FS06 | 8192 | 2049 | Consumidor UTF-8 con «ñ»; mismo rechazo de recibo que FS03. |

FS03/FS04/FS06 superan hipotéticamente la suma, pero esa salida no debe llegar al consumidor. Los bytes reales de cada canal deben igualar los comprometidos; en FS04 se permite el byte detector 8193. Cada positivo debe conservar vector, recibo completo y captura literal. Cada negativo debe conservar ausencia de objeto entregado y de despacho. El archivo de estado se relee al terminar; esa comprobación no se presenta como vigilancia física de RAM.

## Preparación y ejecución

[Preparador Rust](preparar_fronteras_salida.rs): no enlaza el admisor; adapta el recibo histórico y fija longitudes sin consultar una salida nueva. [Banco Rust](fronteras_salida.rs): invoca la API existente, registra lecturas, captura desde el buffer admitido y compara con entradas separadas. [Entradas comprometidas](ENTRADAS_FRONTERAS_SALIDA.tar.gz) contiene las doce variantes, sus oráculos y huellas.

El núcleo y el admisor permanecen íntegros. Fuente del admisor: `../bis04-realizacion-i0205-v0_1/proyecto/src/lib.rs`, SHA-256 `a0fa5a26292a50b46823952dae416d5a790d2cfee22d160b5ecf4903a0c88369`. Se utiliza el núcleo del corte de entrada, incluidos sus recursos incorporados. Los comandos exactos, versiones, salidas y retornos se conservarán mediante [registrador Rust](registrar_ejecucion_fronteras.rs).

El observador utiliza los auxiliares JSON/SHA documentales existentes; no constituye una realización semántica independiente. Cuatro sensibilidades alteran evidencia ya producida: recibo aumentado, captura truncada, suma falsa y despacho tras rechazo. Deben ser detectadas sin repetir al sujeto. El presupuesto de recepción sigue siendo 16384 bytes; concurrencia 1, sin red ni reintentos durante las campañas. Los oráculos y fuentes permanecerán intactos desde su publicación.

## Revisión crítica previa y límites

Las pruebas individuales de geometría y recibo anteriores se conservan. El incremento prueba su composición en las fronteras y el conteo UTF-8. No fuerza una rama inalcanzable alterando cuotas para anunciar cobertura. No ofrece consulta histórica, persistencia, resistencia al anfitrión, imagen renderizada o consumo visual de IA. No crea límites de memoria o tiempo del proceso.

Tras publicar y cotejar los dos destinos, ejecutar el banco y conservar cualquier discrepancia sin ajustar los esperados. El resultado debe preparar el relevo de representación/consumo del montaje. S26 y Bis permanecen abiertos. S24 conserva Bis → catálogo y cierre de fase → GUI, con C#/.NET recibido para ese momento.
