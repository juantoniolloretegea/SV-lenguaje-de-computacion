# Adversarial de fila 7 y fronteras de verificación

Fecha: 8 de septiembre de 2026. Asiento: RETP-2026-103.

## 1. Corte y alcance

Entrada: `main@1706099aef4a0e3846706c3963e7c76313adaf68` y candidata completa `316facc2284858600d2c72fa44e8c715ab047373`, árbol `e477a8ddbaaee3bdcb9a1c9584a8ff59311f726d`, PR #83. La revisión comprende la diferencia acumulada de #79–#83, incluidos contratos, las 578 inserciones en el núcleo, consultas y trayectorias declarativas, LIG/0.1, transporte G/H, observadores, empaquetado y registros.

Rigen Pilares, perfiles y la tabla de catorce filas de la transición. La fila 7 ofrece una candidata representacional por operación. No ofrece Q0, transducción productiva, causalidad ejecutiva, cobertura o permisos materiales, ni una segunda realización semántica. Las doce SP permanecen especificadas sin ejecución integrada. No se modifica el reparto de competencias de los dominios.

La recepción y corrección de #77/#78 ya están integradas en el corte de entrada. El acta del español y RETP-092 se conservan; DFL-011 mantiene la revisión integral antes del cierre nuclear. Ninguna revisión de esta acta autoriza su alteración ni la de README o antecedentes históricos.

## 2. Evidencia examinada y límites de reproducción

Se ha descargado el artefacto `fila7-candidata-sin-interpretes`, identificador `10050650748`, de la ejecución [34213134825](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34213134825). El ZIP tiene 626601 bytes y SHA-256 `4083f5f6e60907d4a1a2e16928bb6059e2326189d6a9e1424ef0564100cc28fe`, recalculado tras la descarga. Su paquete interior tiene SHA-256 `9d62a3a5afef977210a6c40b1f52db0a4a6edfb7ae83135b58808164bb416694`; el ejecutable nativo, `cb0db4112cea29ed01972f88a549b0add8bb43a261034f0e8f37cb04974b8e7a`.

Sobre ese ejecutable se han repetido localmente 14 positivos y 106 negativos del corpus; once negativos de contexto y sus once reparaciones alcanzan respectivamente la causa prevista y la admisión. El observador original conserva 48 transportes, 48 huellas recalculadas, 16/16/16, ocho pérdidas y 28 ataques detectados. Las seis pruebas del lector documental pasan. Esto no equivale a recompilar localmente Rust ni a repetir localmente WASI, navegador o las 43 mutaciones; esas evidencias anteriores se contrastan con sus ejecuciones identificadas.

Se han obtenido directamente del retorno `54fe0d89c9e59065eae2bc8a38f5ec0832ece4b9` los objetos `testigos.json` (SHA-256 `037944fe4ca28524d7e89403d08e881462f7ec349c21e7d9127ccf4c7c7b1f62`) y `contraste.json` (SHA-256 `7b7865985f5b96cc98aeb4d02430a8a480513c4ce229aaffda48589ee3481b39`). El primero coincide literalmente con el testigo comprometido. El inventario receptor conserva exactamente sus 15 requisitos, 44 formulaciones, 81 enlaces, seis familias, 27 identificadores paramétricos y campos seleccionados del perfil. Se ha examinado la asignación de las 44 formulaciones a los trece tratamientos: sus exclusiones no se interpretan como ejecución ni suficiencia clínica. El comprobador automático sólo acredita integridad del inventario, no suficiencia de esas decisiones.

## 3. Contraejemplos reproducidos

| ID | Ataque al corte de entrada | Resultado observado | Obligación afectada |
| --- | --- | --- | --- |
| AF-01 | Anteponer una clave `schema` falsa, una `results` vacía o un `owner` falso a su miembro auténtico en los bytes del informe | Los tres informes se aceptan: `JSON.parse` descarta el miembro anterior antes de verificar | Integridad de la entrada del observador, PT04 |
| AF-02 | Añadir un FIFO llamado `invitado.py` al paquete íntegro | `comprobar_paquete.sh` devuelve conformidad porque sólo inventaría archivos regulares y prohíbe enlaces | Exhaustividad del inventario, PT01/PT14 |
| AF-03 | Repetir el empaquetado del mismo corte bajo permisos de archivo derivados de otra configuración de Git | Los permisos 0644/0755 cambian a 0664/0775; cambia el archivo comprimido | Reproducción del paquete, PT14 |

AF-01 no demuestra que Rust emita claves repetidas. Refuta la afirmación de que el lector estricto protege ya todas las entradas reales del observador. AF-02 no prueba presencia de un FIFO en el artefacto publicado. AF-03 no invalida la igualdad de las dos preparaciones hechas en un mismo entorno anterior; identifica una entrada ambiental no fijada.

La revisión del núcleo no ha localizado un contraejemplo nuevo dentro de la capacidad representacional declarada. En particular, un destino designa un `CoupledSpec` del grafo; compartir `CellSpec` no fusiona identidades. LIG valida la operación solicitada, no acredita automáticamente todas las operaciones del contrato ni el significado o autoridad de los bytes externos. La pertenencia nominal comprobada en consultas no se presenta como cobertura o permiso material.

## 4. Contrato previo de la corrección

1. El informe y la matriz se leerán desde bytes mediante el lector documental estricto antes de cualquier conversión que pueda perder miembros, forma numérica u orden. La proyección incrustada tendrá la misma comprobación. Este requisito se limita a estos testigos documentales; no reduce `Nat` ni el dominio numérico general del Lenguaje.
2. Las regresiones atravesarán la entrada efectiva del observador e incluirán claves repetidas externas, anidadas y escapadas; repetición en matriz y proyección; UTF-8 y Unicode inválidos; número fuera del subconjunto, forma numérica e índice de objeto. Cada ataque exigirá su causa. El control íntegro y su reserialización deben admitirse. Se mantendrán los 28 ataques semánticos anteriores y el recálculo de las 48 huellas.
3. El paquete rechazará enlaces y cualquier entrada que no sea directorio o archivo regular antes de leer sus metadatos. Un FIFO no puede ocultarse bajo un nombre permitido ni bloquear la lectura de `CORTE_GIT.txt`. Los cuatro ataques anteriores se mantienen y se añaden FIFO y enlace.
4. El empaquetado fijará su máscara de permisos y la de `git archive`, conservando el bit ejecutable constituido por Git. Dos preparaciones del mismo corte con máscaras de proceso y configuraciones de Git deliberadamente distintas producirán el mismo paquete. Se mantienen orden, tiempo cero, propietario/grupo cero y `gzip -n`. La comparación no demuestra identidad entre todas las versiones posibles de tar o gzip.
5. Se conservarán las salidas causales y la evidencia de repetición en los artefactos de los flujos. No se cambia semántica de `sv_core`, Gramática, IR, LIG, los testigos inmunológicos ni el corpus de conformidad.

## 5. Estado inicial y condición de salida

`CONTRATO_PREVIO_CORRECCION_PENDIENTE`. Los tres contraejemplos impiden aceptar la PR #83 tal como está como cierre de la auditoría fuerte. La corrección requiere regresiones conformes y repetición de los cinco flujos sobre una cabeza exacta.

El repositorio completo conserva 17 auxiliares Python y usos de Python en CI. Node sigue siendo dependencia de observación externa y del ensayo en navegador; no se oculta dentro del paquete Rust. La prueba autónoma del paquete no demuestra retirada global de esas dependencias ni identidad con la versión viva de producción. La entrega final permanece sujeta al inventario completo y al mandato de ausencia de Python en construcción, generación imprescindible y ejecución de la DSL. Esta corrección no constituye distribución final, elección de plataforma ni apertura de Ciberseguridad.
