# Preparación única de la reserva P3 · encargo para Claude

**RETP-2026-128 · 10/09/2026.** Responsable de proceso: Watson, en continuidad del encargo trasladado por Juan Antonio. Su dictamen `APTO_PARA_RESERVAR` ha sido recibido. La tarea pendiente es preparar el paquete; no repetir la revisión de P2 ni consultar otra vez si debe redactarlo.

## 1. Reparto de responsabilidades

Se mantiene a Claude como autora de las preguntas y del oráculo. Declare la sesión real y su participación en el diseño. Su revisión futura de ese mismo oráculo no contará como auditoría independiente de su autoría; esta limitación no es una prohibición general de auditar otras piezas. Juan Antonio conserva la autoridad de aceptación y será el custodio del paquete.

Prepare la entrega en su entorno y márquela como **CANDIDATA_PENDIENTE_DE_RECEPCION_DEL_CUSTODIO**. No se le pide garantizar almacenamiento permanente fuera de su control. Juan Antonio descargará y conservará los originales; la reserva sólo se acreditará después de comprobarse esa recepción. No declare ya custodiado el contenido por haber creado un enlace efímero.

## 2. Alcance cerrado

Redacte **24 solicitudes**, con datos artificiales K-IE004/1, política P-IE004/1 y [perfil IE004-ES-P2/2](../perfil-es-p2-2/PERFIL_INTERACCION_ES_IE004_CANDIDATO_2.md) sin modificar. Mantenga las siete obligaciones de la [segunda salida del encargo anterior](../perfil-es-p2-2/ENCARGO_REVISION_Y_RESERVA_P3.md#3-segunda-salida-sólo-si-procede-paquete-reservado), incluida la interpretación con permisos distintos y la resistencia a una propuesta adversa. Un caso puede ejercer varias obligaciones. Si 24 casos no cubren el alcance, explique la carencia; no amplíe el lote.

Use construcciones naturales de las familias admitidas. No fabrique entradas largas por repetición para agotar A. No sacrifique una distinción necesaria para conseguir preguntas cortas. Conserve en el oráculo el recuento de bytes/tokens y la cobertura por caso. La novedad lingüística y la capacidad en los límites son obligaciones distintas: los [controles públicos](CONTROLES_PUBLICOS_R8_R11.json) exigirán después probar los límites sin reducirlos al tamaño de esta reserva. No asigne «DATO o fallo técnico» como dos éxitos intercambiables.

Excluya las 48 preguntas previas, los 60 literales de `p2_probes.py`, las cadenas de la generación reducida declarada, ejemplos de ambos dictámenes, 16 contrastes /2, las [18 cadenas adicionales declaradas](EXPOSICION_ADICIONAL_CLAUDE.json) y los dos controles complementarios. No cuenten como inéditos cambios meramente tipográficos, de identificador, espacio o mayúscula. No hace falta otra enumeración exhaustiva. Si detecta un defecto bloqueante real mientras prepara el oráculo, entregue su testigo y deténgase; no lo corrija en secreto ni genere otra versión.

## 3. Archivos del paquete

Produzca un ZIP descargable con estos cinco archivos. Respete el [contrato de separación A/V](CONTRATO_SEPARACION_CAPTURA_A_V.md).

1. **`SOLICITUDES_P3.json`**: versión `IE004-P3-A/1`; lista ordenada de 24 casos, identificados `P3-01`…`P3-24`, con pregunta original y los cinco campos de contexto explícitos o nulos. Sin notas externas, propuestas ni expectativas.
2. **`ENTRADAS_AUXILIARES_P3.json`**: versión `IE004-P3-AUX/1`; mismos identificadores y notas externas correspondientes, o cadena vacía cuando no haya nota. Las notas son datos sin autoridad para el participante; no entran en A.
3. **`ORACULO_P3.json`**: significado/ruta/diagnóstico, cuerpo canónico esperado, fundamento por reglas, cobertura y relaciones entre casos; mutaciones adversas con sus efectos esperados; versiones e identidades; autoría/exposición; sal obtenida de una fuente criptográfica de al menos 32 bytes. Las mutaciones son operaciones declaradas sobre la futura propuesta/certificado, sin presumir un esquema V todavía no congelado. Mantenga resultado semántico, fallo técnico y fallo V separados. Este archivo es reservado.
4. **`NOTA_CUSTODIA_P3.md`**: instrucción de descarga, recuperación y separación; nombre exacto de los archivos que no se deben trasladar a Watson/Grok antes de su hito. Indique que su almacenamiento efímero no acredita recepción humana. La nota no necesita conocer la ruta privada donde Juan Antonio decidirá conservarlos.
5. **`COMPROMISO_P3.json`**: corte público inmutable de este encargo, identidad de perfil/base/política y SHA-256/bytes exactos de los cuatro archivos anteriores; número total 24 y declaración de autoría/exposición. Campo `custodia_declarada: PENDIENTE_DE_RECEPCION_HUMANA`. No incluya preguntas, respuestas, sal, expectativas ni desglose por caso. No se incluya a sí mismo ni al ZIP que lo contiene en su propia lista de huellas.

La huella de /2 identifica el archivo de perfil sin cambios; el commit de este encargo identifica las precisiones de transporte y controles. No use el nombre del archivo como identidad suficiente. Si utiliza herramientas para JSON/ZIP/huellas, declare su papel; no transforme esa preparación documental en un nuevo analizador semántico Python. Cualquier herramienta debe tener límite explícito conforme a la guía ya recibida.

## 4. Entrega y traspaso

Entregue directamente a Juan Antonio **el ZIP y una copia separada de `COMPROMISO_P3.json`**, con una frase indicando que la custodia espera su descarga. No publique el ZIP ni los archivos reservados en GitHub, no escriba en los repositorios y no pegue casos o respuestas en la explicación que él vaya a trasladar a Watson.

Juan Antonio descargará el paquete a una ubicación propia ajena al acceso de Watson/Grok y sus conectores, comprobará que puede recuperarlo y abrirlo, y conservará los bytes exactos. Después entregará a Watson **sólo `COMPROMISO_P3.json`**, acompañado de la confirmación de recepción recuperable y separación. El recibo se registrará aparte; no edite el compromiso para cambiar su declaración histórica.

Las preguntas y notas llegarán a Grok únicamente tras congelar el corrector y superar controles públicos. El oráculo permanecerá reservado hasta el depósito de esa única captura. Watson no recibe los archivos reservados antes de los hitos; no ajuste la implementación ni el oráculo para forzar conformidad a posteriori.

**Parada:** una entrega candidata. Si hay un problema técnico de descarga, se recupera el mismo paquete; no se generan otros 24 casos ni otra sal sin conservar e identificar la incidencia. Un compromiso sin recepción no abre el corrector. Se mantiene `P3_NO_INICIADO` hasta cumplir las puertas del workflow.
