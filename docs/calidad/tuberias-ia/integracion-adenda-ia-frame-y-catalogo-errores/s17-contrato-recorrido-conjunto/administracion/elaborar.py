from pathlib import Path
import json,base64,hashlib
R=Path(__file__).resolve().parent;D=R/'entrega'
def j(n,x):(D/n).write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
(D/'CONTRATO_RECORRIDO_CONJUNTO.md').write_text('''# S17 — Contrato del recorrido documental conjunto

Estado: contrato candidato de laboratorio fijado, versión `S17-CONTRATO/1`. No se ha implementado ni ejecutado este recorrido conjunto. Los resultados de S11, S14 y S15 conservan sus alcances originales; no se suman como si fueran una prueba integrada.

## 1. Objeto y frontera

Enlazar, para un episodio documental acotado, contexto instalado, solicitud original, base efectivamente consumida por G1, propuesta recibida, comprobación y archivo recuperable. Se conserva la identidad y el parentesco de una reevaluación sin sustituir el episodio original. El banco usa exclusivamente los datos artificiales ya fijados en S14.

La IA sólo podrá proponer bytes. El instalador de laboratorio fija las referencias; el productor G1 y los comprobadores Rust deciden qué se admite. Un texto documental que ordene cambiar reglas sigue siendo dato. El experimento no mide la obediencia de un modelo real ni acredita resistencia general a prompt injection.

Este contrato no constituye facultades profesionales, dominio, célula, parámetro, perfil de agente, bus, puerto ni efecto R1. No produce observación → Tri. Ningún error técnico se traduce a U. No modifica semántica, IR 0.3, gramática ni serializador canónico. B/E/K/L y las puertas P3/P4/P5/P6 siguen abiertas en el alcance establecido por S16.

## 2. Pérdidas de interfaz que impiden el enlace directo

| Pieza comprobada | Interfaz material | Pérdida al intentar componer | Adaptación candidata mínima |
| --- | --- | --- | --- |
| S11/S2 | Contexto sobre Referencia de un EnlacePublico y lote fijo | No constituye referencia para una base nueva de S14 | Referencia de episodio instalada desde producción G1; conservar S2 intacto |
| S14 | Historia devuelve Vista con campos públicos; ejecuta A y no solicita V | Vista no es EntregaLiteral ni prueba de comprobación de entrega | Preparar la entrega G1 una vez por episodio y obtener el tipo que G1 valida |
| S14, carga de base | cargar devuelve Base válida o Fallo | El rechazo no devuelve los bytes leídos para el informe de custodia | Envolver o adaptar la carga conservando bytes acotados y causa, sin volver a leer para reconstruirlos |
| S15 | Recibir recibe Referencia S2; Informe devuelve bytes | No enlaza contexto/base dinámicos ni conserva el tipo que admite el escritor | Recepción sucesora versionada y resultado comprobado con constructor privado |
| S3/S11 | Escritor recibe cobertura::Entrega | No admite Informe.cuerpo sin perder la frontera de tipos | Escritor sucesor recibe exclusivamente EntregaComprobada; no constructor desde bytes arbitrarios |

Los pasajes de estas interfaces se conservan en PASAJES_INTERFACES.json. La adaptación crea una realización de laboratorio explícita; no altera hashes del lote S2, no convierte Vista en una credencial ni añade un constructor público al tipo anterior. La pérdida identificada es de composición tecnológica; este corte no justifica ampliar IR.

## 3. Referencias y producción

**Instalación.** El instalador recibe una solicitud exacta, contexto exacto y bytes requeridos de base. Carga el archivo de base con el límite y formato de S14. Debe comparar los bytes leídos con los requeridos antes de consumirlos. Un mismo nombre de archivo con bytes diferentes no es la misma base. Se conserva copia de los bytes efectivamente leídos, no sólo su ruta o huella.

**Consumo.** La vigencia se interpreta de esa Base validada y se pasa a Custodio::abrir; ejecutar_a y no_solicitar_v completan la ruta G1 heredada. No se deriva el recibo esperado mediante etiquetas preparadas por la IA. El banco exige que cambiar a la base negativa cambie la producción a los bytes negativos ya fijados.

**Entrega G1.** La realización sucesora conserva el marco recuperado del mismo manejador, llama a comprobar_entrega una vez para ese episodio y obtiene EntregaLiteral mediante entrega. Conserva los errores concretos de G1. No crea una segunda oportunidad oculta si falla esa comprobación. Copiar el marco para satisfacer préstamos de Rust es una operación de custodia; la comprobación sigue siendo la de G1.

**Referencia de episodio.** Vincula identidad G1, eventual padre, solicitud, base consumida, contexto instalado y EntregaLiteral. Los campos capaces de conferir admisión y sus constructores son privados. Las vistas públicas permiten inspección, nunca fabricación de una referencia admitida. Las identidades son locales al custodio de ensayo; no se proclaman autenticación de persona, proveedor ni unicidad global.

**Reevaluación.** Un custodio admite aquí un original y una reevaluación, como S14. La reevaluación exige la misma solicitud y una base distinta explícitamente instalada; produce identidad distinta y padre igual al original. Cada episodio conserva sus propios bytes. Recuperar el original después debe devolver el original, sin recomputarlo con la base nueva. Se conserva el contexto instalado para ambos episodios en este banco; variar también contexto sería otra prueba.

## 4. Transporte de ensayo SV17/1

Es un protocolo binario sintético, no un contrato de proveedor ni un formato SV canónico. Sustituye explícitamente a SV15 para este ensayo. Cabecera: cinco bytes ASCII `SV17` y byte 1. Tipo siguiente: byte ASCII N o R.

- Campo: longitud u32 big endian seguida exactamente de ese número de bytes. Nunca se normalizan UTF-8, espacios, saltos de línea ni cadenas.
- Campo opcional: bandera 0 sin campo, o bandera 1 seguida de campo. Otra bandera es inválida.
- N: un campo de motivo opaco. Debe terminar ahí. Una negativa no es un cuerpo admitido.
- R: dos u64 big endian para la identidad G1, campo contexto, campo cuerpo, campo opcional solicitud y campo opcional base, en ese orden. Debe terminar ahí.

Se lee hasta EOF con máximo de 16 384 bytes y un byte adicional para detectar exceso. Se conservan hasta 16 385 bytes recibidos. Un error de lectura conserva su ErrorKind y los bytes anteriores; impide decodificación y admisión incluso si esos bytes parecían un mensaje completo. No se interpreta un final de red defectuoso como EOF correcto. Longitudes se comprueban con aritmética segura antes de reservar/copiar; el emisor no controla una reserva arbitraria.

Una cabecera anterior SV15 no se interpreta como SV17. Se rechazan tipo, bandera, truncamiento y sobrante inválidos. El contexto instalado y propuesto está limitado a 8 192 bytes; el exceso se rechaza, no se recorta.

## 5. Orden de comprobación y custodia del tipo

El instalador prepara la referencia antes de consultar al emisor. Después de recepción completa y decodificación estricta se sigue este orden:

1. Negativa N: conservar motivo y finalizar sin admisión ni archivo.
2. Respuesta R: límite de contexto; identidad igual a la referencia elegida por el instalador; contexto idéntico al instalado.
3. Cuerpo: comprobar_presentacion de EntregaLiteral G1, con el perfil de presentación heredado, sin cambiar contenidos de cadenas ni orden de claves.
4. Solicitud citada: debe estar presente e igual byte a byte a la solicitud original del episodio.
5. Base citada: debe estar presente e igual byte a byte a la base consumida de ese episodio.
6. Sólo entonces producir EntregaComprobada de constructor privado, ligada a la referencia y a los bytes admitidos. Las vistas o registros no permiten reconstruirla desde fuera.

La cobertura nueva usa FaltaSolicitud/SolicitudDistinta y FaltaBase/BaseDistinta. Esos nombres no renombrarán retroactivamente FaltaCaso/FaltaVigencia de S2: allí se comprobaban filas del lote y del montaje; aquí se comprueban solicitud y base de un episodio producido. La correspondencia es de obligación, no de identidad de formatos.

La comparación de presentación del cuerpo conserva la tolerancia ya fijada fuera de cadenas. Contexto, solicitud y base exigen bytes idénticos. No se mezcla esta regla con la prueba externa S6 de citas literales.

## 6. Archivo y recuperación

El escritor sucesor sólo acepta EntregaComprobada. El destino lo fija el instalador; la IA no lo cambia. Usa creación exclusiva y escribe los bytes admitidos, conservando el resultado de apertura, escritura y flush por separado. Un destino existente se rechaza y sus bytes deben permanecer intactos.

Antes de escribir se puede afirmar que este escritor no actuó. Después de crear el archivo, un fallo de escritura o flush puede dejar bytes parciales: conservar etapa, ErrorKind, bytes observados y estado del archivo. No borrar, reintentar ni declarar ausencia de efecto automáticamente. La prueba de escritura parcial usa un escritor inyectable controlado; no acredita todas las fallas del sistema operativo.

La recuperación debe leer el archivo con límite y volver a comparar contenido con la referencia del episodio. Si después se sustituye por el cuerpo de otro episodio, se detectará el desacuerdo. La detección a posteriori no impide la sustitución. create_new y flush no acreditan persistencia tras caída, atomicidad, ausencia de symlinks en todo el entorno, autenticación del destinatario ni entrega profesional autorizada.

## 7. Diagnóstico observable

El informe conserva una causa estructurada y su etapa. DIAGNOSTICOS.json fija identificadores locales de S17 y rótulos ES/EN. Esos identificadores no son altas en el catálogo canónico. La misma causa alimenta el registro y la presentación; no se reinterpreta una cadena Debug para decidir el resultado.

Cada observación conserva: caso, modo, repetición, identidad/padre disponibles, referencias originales, entrada recibida y longitud, motivo N si existe, causa con detalle específico, acceso a cada puerta, presencia de EntregaComprobada, operación de archivo intentada y bytes finales observados. Para ausencia comprobada se usa el estado explícito ausente; null no significa indistintamente ausencia, fallo de lectura y no observación.

G1 y los errores de presentación se conservan como detalle tipado. Los errores de sistema conservan ErrorKind. Si aparece una variante no fijada en los esperados, detener y registrar; no agruparla como U ni ajustarla al esperado a posteriori. Un informe de recepción correcto no prueba actividad interna completa de un proveedor.

## 8. Banco y ejecución posterior

BANCO_OBLIGACIONES.json fija 24 casos y sus predicados. Los dos cuerpos esperados y la solicitud/bases proceden de la cápsula S14 intacta. Los nuevos contextos se entregan como bytes. No se regenerarán los esperados a partir de la realización sucesora. Los identificadores de episodio se comprobarán mediante relaciones de igualdad/desigualdad y parentesco, no con números inventados como oráculo.

La siguiente campaña deberá fijar y publicar antes de ejecutar: fuentes completas y hashes, compilador ya custodiado, codificación material de cada caso, número exacto de invocaciones, límites de tiempo y espacio, salidas previstas, cuatro mutantes y tres clientes de frontera de tipos. El presupuesto de ejecución no se declara cerrado mientras no exista esa realización. S17 fija obligaciones; no registra compilaciones inexistentes.

Plan de observación normal: los 24 casos en debug y release, tres ejecuciones por modo, 144 observaciones previstas. Sensibilidad prevista: omitir cotejo de contexto, forzar consumo positivo, omitir cotejo de base citada y convertir una negativa en admisión. Cada mutante debe compilar y un caso fijado debe detectarlo. Además, un cliente externo válido debe compilar y dos clientes que intenten fabricar referencia/entrega deben fallar por privacidad o incompatibilidad de tipo, no por un error ajeno al contrato.

Cualquier imposibilidad de obtener el enlace tipado por las interfaces existentes obliga a documentar la pérdida antes de cambiar el contrato. La presión del ensayo no autoriza a fabricar la prueba que se pretendía obtener. No hay reintentos libres ni reparación silenciosa de oráculos.

## 9. Cierre y relevo

Queda cerrado el contrato documental S17 y el banco de obligaciones. La conformidad ejecutable del recorrido permanece pendiente. El siguiente objeto único es realizar la adaptación mínima, fijar su campaña y ejecutar este banco con custodia íntegra. Tras ese resultado se podrá valorar la integración de los puntos 1 y 3 y el relevo al catálogo de errores. No se adelanta el cierre del núcleo, de inmunología ni la decisión sobre agentes.
''')
rows=[]
def case(i,n,setup,stage,cause,checks):rows.append(dict(id=f'I{i:02}',nombre=n,preparacion=setup,etapa_esperada=stage,causa_esperada=cause,predicados=checks))
case(1,'Recorrido original positivo','Base original, solicitud y contexto exactos; R completo; destino ausente.','recuperacion','CONFORME',['G1 produce esperados/positivo.cuerpo','Referencia procede de EntregaLiteral validada','EntregaComprobada presente','Archivo recuperado idéntico al cuerpo admitido'])
case(2,'Instrucción hostil como dato','Instalar contexto-instruccion.txt; R devuelve ese mismo contexto y cuerpo positivo.','recuperacion','CONFORME',['Mismos bytes positivos que I01','Ninguna instrucción documental cambia referencias ni destino','No es ensayo de comportamiento de modelo real'])
case(3,'Cambio de archivo bajo nombre anterior','Archivo contiene base-nueva.bin, pero bytes requeridos son base-original.bin.','carga_base','BASE_DISTINTA',['No producir episodio G1 ni admisión ni archivo','Conservar bytes leídos y requeridos'])
case(4,'Contexto reescrito','Referencia usa contexto.txt; R cambia un byte de contexto conservando la longitud.','contexto','DOCUMENTO_DISTINTO',['No comprobar cuerpo ni producir entrega','Conservar ambas secuencias'])
case(5,'Identidad de respuesta ajena','R usa identidad de reevaluación ante referencia original; demás campos originales.','contexto','IDENTIDAD',['No comparar contenido ni producir entrega'])
case(6,'Solicitud ausente','R correcto salvo bandera de solicitud=0.','cobertura','FALTA_SOLICITUD',['No producir entrega ni archivo'])
case(7,'Solicitud ajena','R correcto salvo un byte de la solicitud citada; solicitud instalada intacta.','cobertura','SOLICITUD_DISTINTA',['No producir entrega ni archivo','No sustituir solicitud instalada'])
case(8,'Base citada ausente','R correcto salvo bandera de base=0.','cobertura','FALTA_BASE',['No producir entrega ni archivo'])
case(9,'Base citada ajena','Episodio original positivo; R cita base-nueva.bin.','cobertura','BASE_DISTINTA',['El productor sigue habiendo consumido base original','No producir entrega ni archivo'])
case(10,'Cuerpo alterado','R original reemplaza únicamente cadena 8.40 por 8.41 en cuerpo.','cuerpo','CONTENIDO_DISTINTO',['Conservar detalle del comprobador G1 de presentación','No producir entrega ni archivo'])
case(11,'Mensaje completo y error tardío','Read entrega R completo de I01 y después ConnectionReset en vez de EOF.','recepcion','COMUNICACION',['Detalle ConnectionReset','Bytes recibidos iguales al mensaje completo','Cero decodificaciones, cero admisiones, ningún archivo'])
case(12,'Negativa explícita','N con motivo opaco fijo: no_entrego; EOF correcto.','protocolo','NEGATIVA_PROVEEDOR',['Motivo exacto no_entrego','Cero comprobaciones de cobertura','Sin entrega ni archivo'])
case(13,'Truncamiento de campo','Mensaje R de I01 sin su último byte, EOF correcto.','protocolo','ESQUEMA_INVALIDO',['Detalle Truncado','Sin entrega ni archivo','No completar el byte ausente'])
case(14,'Reevaluación negativa real','Crear original; instalar base-nueva.bin con misma solicitud/contexto; producir nueva G1 y transportar nueva R.','recuperacion','CONFORME',['G1 produce esperados/negativo.cuerpo','Identidad nueva distinta de original','Padre nueva igual identidad original','Base consumida nueva exacta','Un recibo negativo puede tener entrega documental conforme'])
case(15,'Recuperar original tras reevaluar','En misma Historia de I14 recuperar original y recorrer R original.','recuperacion','CONFORME',['Cuerpo positivo y base original idénticos a antes','Identidad original y padre original=None intactos','No recomputar original usando base nueva'])
case(16,'Atribución cruzada de cuerpo','Tras I14, identidad/contexto/solicitud/base originales; cuerpo negativo nuevo.','cuerpo','CONTENIDO_DISTINTO',['No producir entrega ni archivo','Original sigue siendo positivo'])
case(17,'Identidad ajena al recuperar original','API sucesora de atribución original recibe identidad nueva, base/cuerpo originales.','atribucion','IDENTIDAD',['Rechazo antes de devolver una vista atribuida','No sustituir original'])
case(18,'Destino previamente existente','Admitir R original; destino ya contiene bytes fijos preexistente.','apertura_archivo','IO',['Detalle AlreadyExists','EntregaComprobada existe, entrega archivada no confirmada','Destino conserva exactamente preexistente'])
case(19,'Sustitución posterior','Escribir positivo admitido, sustituir archivo por negativo y recuperar contra referencia original.','recuperacion','CONTENIDO_DISTINTO',['Comparación posterior detecta cambio','Preservar archivo observado negativo','No afirmar prevención'])
case(20,'Contexto sobre límite','Intentar instalar contexto de 8193 bytes ASCII x.','instalacion_contexto','LIMITE_CONTEXTO',['Rechazo sin truncar','No consultar emisor ni generar archivo'])
case(21,'Recepción sobre límite','Read contiene 16385 bytes ASCII x.','recepcion','LIMITE_RECEPCION',['Conservar 16385 bytes','No decodificar ni admitir','No llamar al lector otra vez para corregir'])
case(22,'Escritura parcial fallida','Con entrega válida, escritor controlado acepta primeros 7 bytes y devuelve WriteZero al continuar.','escritura_archivo','IO',['Detalle WriteZero','Observar prefijo de 7 bytes conservado','Registrar escritura intentada y resultado parcial','No reintentar ni declarar ausencia de efecto'])
case(23,'Contexto en límite','Instalar contexto de 8192 bytes ASCII x; transportar los mismos bytes y resto de R original.','recuperacion','CONFORME',['Mensaje total dentro de 16384 bytes','Contexto íntegro de 8192 bytes','Recuperar positivo exacto'])
case(24,'Presentación permitida del cuerpo','Añadir un espacio ASCII antes del cuerpo JSON original en R; conservar cadenas y resto exactos.','recuperacion','CONFORME',['comprobar_presentacion acepta la presentación heredada','Archivo conserva espacio más positivo, sin normalizar','Solicitud/base/contexto siguen idénticos byte a byte'])
j('BANCO_OBLIGACIONES.json',dict(version='S17-BANCO/1',estado='FIJADO_DOCUMENTAL_NO_EJECUTADO',casos=rows,modos=['debug','release'],repeticiones_por_modo=3,observaciones_normales_previstas=144,fuentes_esperados={p.name:dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in sorted((D/'esperados').iterdir())},sensibilidad=[dict(mutante='omitir_contexto',testigo='I04'),dict(mutante='forzar_consumo_positivo',testigo='I14'),dict(mutante='omitir_base_citada',testigo='I09'),dict(mutante='admitir_negativa',testigo='I12')],frontera_tipos=['cliente_externo_valido','fabricar_referencia_desde_bytes_rechazado','fabricar_entrega_desde_bytes_rechazado'],presupuesto_ejecucion='Pendiente de fijación antes de compilar la realización sucesora; no ejecutado.'))
labels=[('CONFORME','Conforme','Conforming'),('BASE_DISTINTA','Base distinta','Different base'),('DOCUMENTO_DISTINTO','Documento distinto','Different document'),('IDENTIDAD','Identidad distinta','Identity mismatch'),('FALTA_SOLICITUD','Falta solicitud','Missing request'),('SOLICITUD_DISTINTA','Solicitud distinta','Different request'),('FALTA_BASE','Falta base','Missing base'),('CONTENIDO_DISTINTO','Contenido distinto','Different content'),('COMUNICACION','Fallo de comunicación','Communication failure'),('NEGATIVA_PROVEEDOR','Negativa del proveedor','Provider refusal'),('ESQUEMA_INVALIDO','Esquema inválido','Invalid schema'),('IO','Fallo de entrada/salida','Input/output failure'),('LIMITE_CONTEXTO','Límite de contexto','Context limit'),('LIMITE_RECEPCION','Límite de recepción','Reception limit')]
j('DIAGNOSTICOS.json',dict(version='S17-DIAGNOSTICOS/1',alcance='Identificadores locales de obligaciones; no catálogo canónico SV ni traducción a Tri.',identificadores=[dict(id=a,es=b,en=c) for a,b,c in labels],detalles_obligatorios={'COMUNICACION':'ErrorKind específico','IO':'ErrorKind específico y etapa apertura/escritura/flush/recuperación','ESQUEMA_INVALIDO':'Cabecera/Tipo/Truncado/Sobrante/Bandera, sin agruparlos','CONTENIDO_DISTINTO':'Variante tipada del comprobador de presentación; detener ante una distinta a la fijada en campaña'},fallos_no_ejercitados='Conservar variante original G1 o de instalación y detener ante resultado no previsto; no inventar una equivalencia de catálogo.'))
# Citas de interfaces existentes, extraídas exactamente, con posiciones y custodia.
src=json.loads((D/'FUENTES_RECIBIDAS.json').read_text());emb=json.loads((D/'INTERFACES_ANTERIORES.json').read_text());passages=[]
needles=[('codigo/bases.rs','pub fn cargar('),('codigo/bases.rs','pub struct Vista'),('codigo/bases.rs','pub fn crear('),('codigo/bases.rs','pub fn reevaluar('),('codigo/receptor.rs','pub fn cuerpo('),('codigo/receptor.rs','pub fn recibir('),('codigo/receptor.rs','format!("{cat}'),('codigo/destino.rs','pub fn escribir'),('cobertura/cobertura.rs','pub struct Referencia'),('cobertura/cobertura.rs','pub struct Entrega'),('fuentes-base/recibo-g1/candidata/entrega.rs','pub struct EntregaLiteral')]
for suffix,needle in needles:
 pool=emb if suffix in emb else src;path=next(p for p in pool if p.endswith(suffix));b=base64.b64decode(pool[path]['base64']);text=b.decode();lines=text.splitlines(keepends=True);indices=[i for i,l in enumerate(lines) if needle in l];assert len(indices)==1,(path,needle,indices);i=indices[0];excerpt=''.join(lines[i:i+3]);start=len(''.join(lines[:i]).encode());passages.append(dict(fuente=path,linea=i+1,inicio_byte=start,fin_byte=start+len(excerpt.encode()),texto=excerpt,sha256_fuente=hashlib.sha256(b).hexdigest()))
j('PASAJES_INTERFACES.json',dict(version='S17-PASAJES/1',pasajes=passages))
print('Contrato, 24 obligaciones, diagnósticos y',len(passages),'pasajes escritos; ninguna ejecución funcional.')
