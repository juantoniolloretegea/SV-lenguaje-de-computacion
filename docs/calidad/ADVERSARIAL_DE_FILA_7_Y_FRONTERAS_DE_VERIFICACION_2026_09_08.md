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


## 6. Corrección y comprobación material

El contrato previo se fijó en `f6986cf828fb9e4047f0f68d1276d22b4835d29f`. La realización es `0b7fa120f3f09f98399f07bf87cda9410ca1b54e`, árbol `b66ac3cbc4287d594b3b97b214ef81c171b4ace2`, en la [PR #84](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/84), apilada sobre #83. La fusión virtual `b821494b08cbe9078d283e1862b107b076d75f5c` tiene los padres #83 y esta cabeza y el mismo árbol material. No es integración en main.

| Flujo de la candidata material | Ejecución | Resultado |
| --- | --- | --- |
| R0 Rust y compatibilidad adicional | [34224986578](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34224986578) | Conforme |
| Conformidad SVP | [34224986677](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34224986677) | Conforme |
| R0-8 nativo | [34224986628](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34224986628) | Conforme |
| Paridad nativa, WASI y navegador | [34224986579](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34224986579) | Conforme |
| Fuentes y ejecución sin intérpretes | [34224986696](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34224986696) | Conforme |

AF-01 queda detectado por once ataques ME01–ME11, cada uno ejecutado contra la CLI real en un proceso separado: retorno 1, salida estándar vacía y causa exacta. Los controles íntegro y reserializado se admiten. Se preservan los 28 ataques semánticos previos, las 48 huellas recalculadas, 16 recuperaciones F0, 16 HS, 16 controles H y ocho pérdidas. La proyección interna del informe y la matriz también atraviesan el lector estricto. El parámetro `--matriz` permite ensayar los bytes de una matriz alternativa sin cambiar la matriz comprometida.

AF-02 queda detectado por ocho ataques de paquete: los cuatro anteriores, FIFO con nombre Python, FIFO con nombre Rust permitido, FIFO en lugar de `CORTE_GIT.txt` y enlace. El control precede a la lectura de los metadatos; un límite externo de diez segundos hace que un bloqueo sea fallo del ensayo y nunca rechazo válido. AF-03 queda detectado por la comparación bajo máscaras 0000 y 0077, aplicadas tanto al proceso como a la configuración Git. Los paquetes son idénticos. Con el empaquetador corregido, el corte anterior #83 reproduce también literalmente su archivo publicado.

Se mantienen corpus 14/14 válidos y 106/106 inválidos, 43/43 mutantes dirigidos detectados y paridad entre los tres destinos. El entorno aislado conserva 348 pruebas Rust conformes, sin Python/Node ni red durante construcción y ejecución. Node verifica el informe fuera del aislamiento. El paquete sigue conteniendo 227 archivos Git y dos objetos de inventario: no se ha añadido JavaScript al paquete Rust.

**Custodia descargada y verificada:** artefacto `10055323351`, 627275 bytes; ZIP SHA-256 `45dff56767f8b55c24860b40505da71efebcaf65a3047be3ef3bc9ec34d586ba`. Paquete interior SHA-256 `67fbb2a206f1665216ce26e4b1d2669bd5f606b8cc23d31dbc2e33e972939700`. Ejecutable SHA-256 `cb0db4112cea29ed01972f88a549b0add8bb43a261034f0e8f37cb04974b8e7a`, idéntico al descargado de #83. Estas tres huellas se han recalculado localmente; no se toman únicamente de los metadatos del proveedor. Los archivos `controles-paquete.txt`, `controles-permisos.txt`, `verificacion-externa.json` y `ejecucion.log` conservan la evidencia causal.

Como contraste adicional de no estrechamiento, el ejecutable auditado admite una especificación `b=4294967296` y destino `18446744073709551616` (n=b², superior a u64), sin construir un vector de esa longitud. El destino inmediatamente superior se rechaza por E406 y el límite exacto. El ensayo no atribuye viabilidad de recursos a un vector de ese tamaño.

Reproducción de los controles, desde la raíz y con las herramientas de verificación declaradas:

```sh
node tests/row7_gh/verificar.mjs INFORME_GH.json --autoprueba
bash tests/row7_candidate/empaquetar.sh 0b7fa120f3f09f98399f07bf87cda9410ca1b54e DIRECTORIO_NUEVO
bash tests/row7_candidate/autoprueba_paquete.sh DIRECTORIO_NUEVO/fuentes
bash tests/row7_candidate/autoprueba_permisos.sh 0b7fa120f3f09f98399f07bf87cda9410ca1b54e
```

## 7. Dictamen y continuidad

**Estado: `ADVERSARIAL_CONCLUIDA_CORRECCIONES_VERIFICADAS_NO_PROMOVIDAS`.** AF-01, AF-02 y AF-03 quedan corregidos y protegidos en esta candidata. La adversarial del conjunto #79–#83, con esta corrección, no ha dejado un contraejemplo pendiente dentro de la capacidad representacional examinada. No constituye una prueba de ausencia universal de defectos.

La candidata corregida es técnicamente apta, en ese alcance, para su integración gobernada y posterior entrega al segundo falsador. Lo pendiente de este relevo es conservar las identidades y verificaciones al integrar la pila y registrar la decisión de entrega. La fila 7 no se declara integrada ni administrativamente cerrada aquí; Ciberseguridad no se activa mediante esta PR. La unidad correspondiente constituirá su universo en la fila 8. No se exige repetir F, F-IF o G/H.

El cierre de estos tres hallazgos no cierra DFL-001 en general, DFL-013 ni la retirada residual de Python. El ensayo aislado acredita autonomía del paquete examinado; el repositorio y su verificación completa siguen conteniendo 17 auxiliares Python, JavaScript de observación/transporte/navegador y fragmentos de CI expresamente declarados. La entrega final sin Python, la revisión integral del español, el versionado de `cell_ref`, las capacidades ejecutivas y las sedes posteriores conservan sus obligaciones. No se confunde esta candidata con la distribución productiva viva.

La cola registral posterior al corte material sólo incorpora este resultado y la continuidad de deuda. La cabeza final y la repetición de los cinco flujos se identifican en la PR, evitando autorreferenciar una confirmación desde sus propios bytes.
