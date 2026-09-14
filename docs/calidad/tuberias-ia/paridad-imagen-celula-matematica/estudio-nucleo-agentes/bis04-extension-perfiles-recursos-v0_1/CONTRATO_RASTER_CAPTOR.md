# Rasterización y captor: contrato y banco acotado

S22 · W-S26-02 · RETP-2026-240 · 14/09/2026.
Entrada: Lenguaje 040595027d2ffc6a3cc4b638d55b49ef0d5643f8; laboratorio 621a8407d01319d5c14a09f90d9c63dd2731cfca.
Preparación previa a la ejecución de las ocho sondas.

## Recepción y autoridad

Leídos AGENTS, Pilares completos, acta de perfiles completa, transición completa §§1–30, Sucesos, Léame primero, C04/C05 y contrato/resultado SVG RETP-238/239. Se conserva el vector plano y su descriptor; no se modifica núcleo, IR, gramática ni perfil fuente. El SVG de entrada es el artefacto material de 792 bytes ya recibido, no una nueva producción EN/ES. Este incremento empieza en ese archivo y no repite su campaña.

El inspeccionador Rust preparar_raster_captor.rs ha cotejado la huella del SVG y construido dos secuencias distintas (rotación cíclica e inversión) con el mismo multiconjunto de aristas no dirigidas. Es un testigo geométrico, no una comparación de píxeles. P1 y sentido están en metadatos no dibujados. La imagen sola no debe presentarse como codificación posicional reversible. El captor no produce Tri ni interpreta conocimiento clínico.

## Dependencias recibidas

El paquete oficial [resvg 0.48.1 con dependencias vendorizadas](https://github.com/linebender/resvg/releases/download/v0.48.1/resvg-0.48.1.tar.xz) se recibió mediante archivo humano tras dos accesos directos impedidos por timeout de proxy (índice Cargo y descarga GitHub). SHA256 oficial y recibido: 13ed5a2bae7a01156288ecae5bf944cf7d1c572742c19fc68027947a4d87294c. Tamaño oficial 13023916 bytes.

Se compila sin modificar sus fuentes mediante Rust 1.98.0 y cargo build --locked --offline --release -p resvg. El primer tar devolvió error al intentar conservar propietario 1001; la extracción con --no-same-owner terminó correctamente. No cambió contenido del archivo. Compilación terminada con retorno 0 y versión ejecutable 0.48.1. Las dependencias y licencias se conservan en el archivo oficial y Cargo.lock; no se atribuye a Rust la implementación de tar, Git, shell o los conectores.

Tipografía concreta: DejaVuSans.ttf, 759720 bytes, SHA256 ae7b7855e115a5966d8b1b3f80f254ccc117ec86f9965e202ee2940453837280. Se desactiva carga de fuentes del sistema y se carga sólo este archivo, con familia predeterminada DejaVu Sans. Resuelve explícitamente la familia omitida en el SVG histórico; no se cambian sus bytes ni la leyenda. Fuente ausente/distinta impide el banco.

## Perfil experimental y observador

SVG restringido previo; salida PNG 320×360 sobre blanco. resvg recibe --skip-system-fonts, --use-font-file, --font-family y --background explícitos. El captor Rust abre el PNG realmente escrito, con máximo 1000000 bytes. Comprueba firma y dimensiones antes de decodificar con tiny-skia 0.12.0 y conserva una firma aritmética posicional de los píxeles no blancos. Esa suma es observable de operación, no hash criptográfico ni autenticación.

La referencia geométrica son dieciséis pares literales del convenio anterior, fuera del resultado del rasterizador. Para cada arista se exige tinta en una ventana 5×5 centrada en el suelo de su punto medio proyectado: x=(X+4000000)/25000, y=(Y+4000000)/25000. Se exige ausencia de tinta en y<35, x<15, x>=305 o y>=350. La presencia de leyenda sólo exige tinta en 15<=x<310 y 318<=y<344. Estas ventanas son criterios acotados de sondeo, no tolerancia universal ni demostración completa de contorno, tipografía o legibilidad. No son un oráculo de igualdad de todos los píxeles. No se ajustarán después de ver resultados.

Se mantienen separados productor resvg y comparador geométrico. Codificación y decodificación comparten maquinaria tiny-skia/PNG: no se acredita independencia de códec. Los fallos del propio decodificador, cuotas internas, aislamiento y tiempo máximo de hijo no se cualifican aquí. Una ejecución a la vez, sin red durante el banco; sin recursos SVG externos en la entrada fijada.

## Ocho sondas comprometidas

| ID | Operación | Esperado previo |
| --- | --- | --- |
| R01 | SVG original, render real, apertura del PNG producido | Geometría=true; leyenda presente=true. |
| R02 | Borrar banda y>=310 del PNG | Geometría=true; leyenda=false. |
| R03 | Reflejar horizontalmente la zona y<310 | Geometría=false; leyenda=true. |
| R04 | Borrar x>=200 en la zona y<310 | Geometría=false; leyenda=true. |
| R05 | Sustituir toda la imagen por blanco | Geometría=false; leyenda=false. |
| R06 | Rasterizar leyenda con radios 0/1 intercambiados | Geometría=true; presencia=true: límite esperado, significado NO validado. |
| R07 | Comparar archivo R05 realmente abierto con bytes R01 custodiados | Sustitución detectada por desigualdad. Es comprobación local, no barrera productiva integrada. |
| R08 | Captura correcta R01 y operación sobre R05 | Firma de operación distinta: falsa dependencia detectada. Es sonda explícita, no mutante del consumidor integrado anterior. |

Un retorno 0 significa concordancia con esta tabla, incluida la brecha R06. No significa que la leyenda errónea sea admisible ni que la rasterización completa esté certificada. Un PNG base derivado del rasterizador se usa sólo como referencia de custodia y de mutación, nunca como oráculo independiente de corrección visual.

## Relevo

Conservar PNG, comandos, stdout/stderr, identidades y tabla observada incluso si hay discrepancias. Después constituir verificación independiente del contenido de la leyenda y marcadores visibles de P1/sentido si la operación requiere recuperar esas distinciones de los píxeles; cerrar la frontera de captor autorizado y sus recursos antes de promoverla. No suplir esa carencia con OCR opaco. S26/F01/F02 y las 202 filas originales conservan su estado. Bis abierto; catálogo y cierre de fase preceden a GUI C#/.NET.
