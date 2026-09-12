from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,shutil
R=Path(__file__).resolve().parent; D=R/'entrega'; D.mkdir(exist_ok=True)
J=lambda p,x:p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
sha=lambda b:hashlib.sha256(b).hexdigest()
blob=lambda b:hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
T=datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z')
head=json.loads((R/'lenguaje-base.json').read_text())['head']; tree=json.loads((R/'lenguaje-arbol.json').read_text())
base='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/'+head+'/'
S='docs/calidad/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/'
sources=json.loads((R/'FUENTES_RECIBIDAS.json').read_text())
# Antecedentes locales: sólo se reciben después de comprobar el blob del corte.
for src,p in [(R.parent/'s11-integracion/entrega/ACTA_RESULTADO_S11.md',S+'s11-contexto-y-cobertura/ACTA_RESULTADO_S11.md'),(R.parent/'s11-integracion/entrega/MATRIZ_COBERTURA_A_L_RESULTADO_S11.json',S+'s11-contexto-y-cobertura/MATRIZ_COBERTURA_A_L_RESULTADO_S11.json'),(R.parent/'s11-integracion/entrega/codigo/contexto.rs',S+'s11-contexto-y-cobertura/codigo/contexto.rs'),(R.parent/'s12-rectificacion/entrega/ACTA_RECTIFICACION_DE_RUMBO.md',S+'s12-rectificacion-rumbo/ACTA_RECTIFICACION_DE_RUMBO.md')]:
 b=src.read_bytes();assert tree[p]==blob(b),p
 dest=R/'lectura'/p;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(b)
 sources[p]=dict(blob=blob(b),bytes=len(b),sha256=sha(b),corte=head)
for p,v in sources.items():
 b=(R/'lectura'/p).read_bytes();assert v['blob']==tree[p]==blob(b);assert v['sha256']==sha(b)
 v['url']=base+p
J(D/'FUENTES_RECIBIDAS.json',sources)
passages={}
def excerpt(key,path,start,end):
 lines=(R/'lectura'/path).read_bytes().splitlines(keepends=True)
 assert 1<=start<=end<=len(lines),(path,len(lines),start,end)
 b=b''.join(lines[start-1:end]);passages[key]=dict(fuente=path,linea_inicio=start,linea_fin=end,bytes=len(b),sha256=sha(b),texto=b.decode(),url=base+path+f'#L{start}-L{end}')
I='IR_CANONICA_BIENFORMACION_SV_v0_3.md'; V='IR_CANONICA_BIENFORMACION_SV_v0_2.md'; C='rust/sv_core/src/'; F='docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md'
for a in [('P01',I,16,28),('P02',I,30,77),('P03',I,161,241),('P04',I,243,255),('P05',I,286,345),('P06',V,330,337),('P07',V,451,483),('P08',V,603,609),('P09',V,621,640),('P10',C+'ir.rs',25,65),('P11',C+'ir.rs',97,220),('P12',C+'ir.rs',350,389),('P13',C+'frontend.rs',1793,1810),('P14',C+'lib.rs',148,177),('P15',C+'frame.rs',189,237),('P16',C+'admissibility.rs',1,110),('P17',C+'requirements_coverage.rs',170,223),('P18',C+'requirements_coverage.rs',240,285),('P19',C+'decision_trace.rs',708,752),('P20',C+'decision_trace.rs',806,835),('P21',C+'decision_trace.rs',931,988),('P22',C+'execution.rs',447,515),('P23',C+'transition_data_wellformed.rs',1,34),('P24',F,280,294),('P25',F,325,337),('P26',F,340,365),('P27',F,367,406),('P28',S+'s11-contexto-y-cobertura/codigo/contexto.rs',1,21),('P29',S+'s11-contexto-y-cobertura/ACTA_RESULTADO_S11.md',1,38),('P30',S+'s12-rectificacion-rumbo/ACTA_RECTIFICACION_DE_RUMBO.md',1,43)]:excerpt(*a)
J(D/'PASAJES_COTEJADOS.json',passages)
rows=[]
def row(id,title,criteria,sede,norm,ir,rust,evidence,positive,negative,loss,next_step,refs):
 rows.append(dict(id=id,obligacion=title,criterios_A_L=list(criteria),sede=sede,contrato=norm,representacion_ir=ir,realizacion_rust=rust,evidencia_disponible=evidence,control_positivo=positive,contraejemplo_o_control_negativo=negative,perdida_o_limite=loss,tratamiento=next_step,pasajes=refs,ensayo_funcional_nuevo_en_S13=False))
row('O01','Separar fallo, no admisión y valor semántico U','F','Núcleo para tipos y transducción; frontera para fallos de proveedor',
'IR 0.3 §§2.1–2.4 prevalece sobre el antecedente 0.2/J1.5: ni Bottom ni no admisión se convierten en Tri.',
'AdmissibilitySpec y CaptureSpec están declarados; Ternarizer conserva cinco identificadores. No representa la partición ni su función ejecutable.',
'admissibility.rs: CaptureOutcome<T>, AdmissibilityState; tipos separados de Tri. IrObjectKind::Ternarizer es declarativo.',
'Inspección del corte; K1-T es una vía expresamente inhabilitada. S11 conserva errores propios y no calcula ternarización.',
'Tri ya constituido y admisión declarativa bien formada, dentro de su alcance. No se ejecuta aquí.',
'No admisión, Bottom o comunicación fallida usados como U: incompatibles con el contrato. El control integrado de fallos de proveedor queda pendiente.',
'Para observación→Tri faltan representación comprobable de espacio/partición/función y productor. Un wrapper no llena esta carencia nuclear.',
'Mantener K1-T inhabilitada; vincular cualquier consumidor futuro con DFL-006 y puerta algebraica. No inventar U ni código efectivo E107.', ['P01','P02','P11','P16','P24'])
row('O02','Producir respuesta, justificación reconstruible y metadatos de consulta','CI','Semántica y realización nuclear de la consulta',
'IR heredada §§3 N4, J5.1/CQ1–CQ6 y §6.4: QueryResult=(r,J,M), justificación sin pasos opacos, interfaz y U/criticidad declaradas.',
'Existe contrato conceptual QueryResult. IrOperationKind::Query guarda spec/by/context; el conjunto material de objetos no aporta por ello el resultado r,J,M.',
'frontend.rs construye la operación Query; compile_svp devuelve IrProgram tras validaciones. Esta ruta no produce la respuesta ejecutiva que describe el contrato.',
'Inspección directa de la entrada pública y reconocimiento vigente DFL-003/004/006. Capturas de S11 son evidencia del montaje, no un QueryResult nativo.',
'Operación de consulta estructuralmente admitida y contexto coherente: acredita sólo esa admisión.',
'Pretender una consulta ejecutiva completa presentando sólo IrOperationKind::Query y texto explicativo: no satisface CQ1–CQ6. Control ejecutivo futuro, no ensayo S13.',
'Pérdida de realización acreditada por contrato/código/deuda: falta materializar y comprobar la cadena completa r,J,M para el consumidor que la necesite. No se demuestra aquí que deba cambiarse la firma semántica.',
'Delimitar primer consumidor y dependencias antes de implementar. No renombrar recibos de laboratorio como QueryResult ni declarar E501 emitido sin emisor comprobado.', ['P06','P07','P08','P11','P12','P13','P14','P24','P26'])
row('O03','Exigir cobertura suficiente con acceso independiente','CHI','Núcleo para relaciones y obligaciones; frontera documental para selección de citas',
'IR 0.3 §4.7: cierre de Frame no exige exhaustividad. CQ4/CQ6 exige porción de interfaz y condiciones de cierre.',
'Frame referencia colecciones coherentes. CoverageReport declara referencias; no produce por su nombre una prueba de exhaustividad.',
'Frame::from_candidate no impone exhaustividad. assess_requirement_coverage contrasta verificadores exigidos/participantes; aggregate_covered_requirement_results impide Accredited sin cobertura y conserva Refuted.',
'S1/S2 y S11 disponen de referencia anterior a la selección; AH03/AH04 omiten vigencia y AH05 presenta vigencia ajena. No se reejecutan en S13.',
'S11 AH01/AH02: citas completas, igual recibo negativo. R1 distingue cobertura de verificadores mediante su regla constituida.',
'S11 omisión→FaltaVigencia; vigencia ajena→VigenciaDistinta. El mutante AH03 que elimina la obligación llega a escribir y es detectado.',
'Cobertura documental y cobertura de verificadores R1 son relaciones distintas. Faltan su enlace profesional y cobertura requerida del consumidor; un Frame válido puede no contener toda evidencia necesaria.',
'Fijar quién constituye cada conjunto requerido y cómo lo recibe el comprobador; preservar el acceso independiente. No ampliar Frame para suplir el contrato del consumidor.', ['P03','P07','P15','P17','P18','P29','P24'])
row('O04','Recibir instrucciones externas como datos sin autoridad','AH','Frontera de recepción y comprobación',
'Las referencias y el documento se fijan antes de selección en S11; una orden incluida no altera la obligación de cobertura.',
'No se necesita convertir documento, prompt, memoria de proveedor ni conector en un nuevo objeto nuclear por este ensayo.',
'S11 Contexto::desde y ::comprobar conservan bytes prestados; comparan tamaño, identidad y documento antes de Referencia::comprobar.',
'S11 AH01–AH10 y dos mutantes: evidencia sintética preservada. Código Contexto cotejado con el blob público.',
'AH02 acepta evidencia completa aunque el documento contenga orden de omitir; mantiene recibo negativo original.',
'AH03 rechazo de omisión; AH06 identidad ajena; AH07/AH08 documento distinto; AH09 exceso. Controles ejecutados en S11, no S13.',
'Host y origen son confiables/sintéticos. No existe aquí observación de un LLM resistiendo prompt injection ni autenticidad del documento frente al host adversario.',
'Conservar frontera explícita y evidencia externa íntegra. Toda futura entrada de proveedor deberá recorrerla y aportar sus capturas reales antes de afirmar resistencia.', ['P28','P29','P30'])
row('O05','Ligar decisión, permiso y efecto sin fabricar autoridad','BL','Frontera R1 y continuidad protegida; raíz profesional externa constituida',
'Recepción RETP-108 y realizaciones R1 distinguen lectura documental de actuar; nombres y expedientes no constituyen facultades.',
'IrProgram y QuerySpec no fabrican permisos. No se introducen credenciales o deployment como primitiva IR por conveniencia del ensayo.',
'decide_permit_traced registra decisión; mediate_traced_permit liga compromiso; execute_traced_mediated enlaza el despacho real. execute_mediated registra DispatchCommitted antes del adaptador y conserva Indeterminate tras error.',
'Funciones presentes en main inspeccionadas. RETP-153/154 documentan contraejemplo y correcciones candidatas, expresamente sin promoción productiva.',
'Decisión propia, cobertura constituida, mediación y facultad admitidas: recorrido R1 condicionado a sus productores.',
'Cruces entre continuidades de igual constitución/ordinal reproducidos en RETP-153; corrección candidata los rechaza. No se presenta main como si ya incluyera esa corrección.',
'No están constituidos aquí la premisa profesional ni el verificador operativo general. Identidad intraproceso tampoco equivale a identidad durable. La ruta profesional requerida permanece inhabilitada.',
'Reutilizar el contrato protegido y promover sólo subconjuntos acreditados cuando proceda. No sortear productores con for_test ni con declaraciones de una IA.', ['P19','P20','P21','P22','P26'])
row('O06','Conservar identidad de invocación y contexto','H','Frontera de pertenencia; continuidad nuclear cuando participa R1',
'Identidad de estado por nodo en Frame es una relación concreta, distinta de invocación, fuente y continuidad de autoridad.',
'Frame, nombres de objetos e IrProgram.source_sha256 no sustituyen la identidad de una invocación.',
'S11 Contexto compara el par recibido con Referencia::identidad; decision_trace liga la decisión en la continuidad disponible.',
'S11 AH06 y mutante de identidad; limitación de cruces R1 recibida por RETP-153.',
'Contexto íntegro y de su propia referencia: caso de control S11.',
'Mismo cuerpo de otra invocación→IdentidadContexto; desactivar guarda hace aceptar AH06 y el ensayo lo detecta.',
'El testigo protege la ligadura local programada; no acredita autenticación del origen ni resistencia a restauración/colisión entre procesos.',
'Conservar separadas identidad de objeto, invocación, continuidad y versión; exigir evidencia adicional donde se cruce cada frontera.', ['P03','P10','P28','P29','P26'])
row('O07','Comprobar fidelidad hasta el objeto entregado','DK','Proyección y presentación con referencia independiente',
'IR 0.3 §§6.2–6.3 exige relación unívoca y conservación de miembros; no verifica significado del texto de OutputSemantics ni pantalla.',
'Campos y orden serializado son obligaciones representacionales. Su conservación no acredita por sí sola identidad del archivo final ni acto humano.',
'S11 reutiliza conductor de archivo S3 y observador que vuelve a leer; el núcleo de compilación sólo devuelve IR admitida en el recorrido examinado.',
'Matriz S11 conserva S3 D01–D04 (archivo y pérdida de negación) y D05 (omisión sin escritura). Acta S11 registra cotejo byte a byte del archivo aceptado.',
'Archivo recuperado coincide con referencia fijada; éste es el objeto observado en esos ensayos.',
'Pérdida de «no » altera cadena, aun con JSON válido; no es espacio de presentación autorizado. Alteración posterior se detecta, sin atribuir prevención.',
'Quedan sin acreditar renderizado final, equivalencia de transportes no ensayados y revisión profesional del objeto mostrado.',
'Conservar original, transformación y objeto efectivamente recibido. Mantener pendiente K y no cerrar fidelidad humana con hash de un antecedente.', ['P05','P09','P14','P29'])
row('O08','Identificar la base realmente utilizada bajo igual nombre','G','Procedencia y carga en frontera/soporte; identidad de fuente ya representada',
'IRProgram identifica fuente/versiones; criterio G exige componente/regla/corpus realmente cargado, no sólo etiqueta.',
'IrProgram.source_file/source_sha256 y cabecera 0.2/0.3/0.1.0 sirven para fuente y esquema. No incluyen por sí solos toda base externa o binario en uso.',
'IrProgram expone esa identidad; S11 conserva entorno y binarios, pero la matriz reconoce sustitución de regla/componente/corpus pendiente.',
'Inspección de campos y matriz A–L reconciliada; no campaña G ejecutada en S13.',
'Candidato: recibir componente/regla/corpus cuyos bytes coinciden con referencia previa y ligar esos mismos bytes al cálculo.',
'Candidato: cambiar una base manteniendo el nombre, o declarar una huella antigua usando bytes nuevos; debe impedirse atribución antigua.',
'Falta contraste entre identidad declarada y dependencia realmente consumida. Una huella aislada acredita bytes examinados, no ejecución con esos bytes.',
'Preparar testigo G en la frontera documental existente; referenciar fuente y configuración real sin añadir campos semánticos por anticipado.', ['P04','P10','P29'])
row('O09','Distinguir recuperación original de nueva evaluación','J','Continuidad y contrato de consulta; custodia material exterior',
'IR 0.2 J4.1–J4.3 y CQ3/CQ5; reproducibilidad requiere datos suficientes. Criterio J conserva base original y relación con base nueva.',
'Frame, TransitionData y Trajectory representan estructura e historia declarada; no equivalen a archivo durable ni motor de transición.',
'Frame no expone mutadores posteriores; transition_data_wellformed::validate_program valida relaciones locales, declara no ejecutar transición ni decidir causalidad.',
'Inspección del límite explícito y DFL-003/004; registros S7–S12 preservan antecedentes pero no prueban reevaluación ejecutable con otra base.',
'Candidato: recuperar recibo con su base original y obtener una nueva evaluación identificada con base nueva, preservando ambas.',
'Candidato: presentar resultado de base nueva como explicación del episodio antiguo; debe rechazarse la atribución y conservarse el original.',
'Falta realización/contraste de recuperación y reevaluación separadas; la historia persistente y la transición SV nativa no se acreditan por append-only declarativo.',
'Fijar G/J documental como siguiente experimento acotado. Si el consumidor exige transición o QueryResult SV, detener esa promoción y realizar sus obligaciones nucleares primero.', ['P07','P08','P09','P15','P23','P24','P29'])
row('O10','Emitir causas y procedencia sin reparaciones ni traducción semántica','F','Diagnóstico nuclear y frontera, proyección ES/EN',
'Una IR canónica; gramática 0.2 e IR 0.3 son versiones distintas. E004/E115 efectivos no se sustituyen por códigos históricos de IR 0.2.',
'Enumeraciones de IR cerradas; versiones canónicas, sin mapa genérico. La prosa ES/EN presenta la causa, no cambia la decisión.',
'compile_svp propaga rechazos; coverage y Contexto preservan variantes. RETP-157–161 añaden causas/ubicaciones en candidatas, no en main productivo.',
'Inspección de versión/emisores citados y deuda actualizada; no se repite corpus ni prueba lingüística en S13.',
'Causa original y ubicación conservadas en su etapa, con mensajes ES/EN que no cambian identidad.',
'Confundir fallo de comunicación, no admisión y revocación; reutilizar E501/E107 históricos como si se hubieran emitido; completar datos ausentes en silencio.',
'Catálogo global y procedencia completa siguen abiertos. No hay en estas fuentes una cabecera independiente denominada semantic_version=0.2.',
'Usar denominación precisa superficie/gramática 0.2 y obligaciones semánticas vigentes; recibir causas observadas con su emisor, versión y alcance para catálogo posterior.', ['P04','P05','P10','P11','P14','P27','P28'])
row('O11','Imponer canales, destinatario, mínimo y límites materiales','EL','Frontera de uso y soporte, P4/P5/DFL-009',
'Criterios E/L requieren vista autorizada, canales y entrega adecuada; integridad de una cita no implica no interferencia ni aislamiento.',
'No se justifica convertir red, sandbox, host o límite del montaje en semántica de célula/IR.',
'Contexto limita documento a 8192 bytes en S11; es una cota local. No constituye política global de recursos ni control de red/destinatario.',
'S11 AH09/AH10 y recepción documental DFL-009; matriz declara E/L sin cierre.',
'AH10 admite el límite de documento; para E/L hacen falta controles con vista, finalidad y destinatario expresamente fijados.',
'AH09 rechaza 8193 bytes. Variar secreto con igual vista autorizada o cambiar destinatario siguen siendo controles materiales pendientes.',
'No se acreditan host adversario, canales laterales, servicio remoto, coste productivo o minimización profesional de salida.',
'Conservar P4/P5 y DFL-009, ensayar canales sólo tras contrato acotado; no extrapolar la cota de documento a toda ejecución.', ['P25','P28','P29'])
row('O12','Ligar revisión y cierre al objeto y alcance comprobados','K','Resolución nuclear identificada; acto de revisión en frontera profesional',
'IR 0.3 §3 distingue revisión y clausura; criterio K requiere objeto realmente mostrado, alcance, acto y autoridad.',
'ResolutionTarget identifica U en estado/posición. Su registro no representa automáticamente un acto humano de revisión documental.',
'La operación Resolve lleva target_state/position/spec/context/mechanism; Frame::from_candidate rechaza criticidades sin productor. Ninguno acredita por sí solo visualización humana.',
'Inspección de representación y alcance; matriz S11 declara ausencia de ensayo de revisión humana.',
'Objetivo U identificado y revisión formal bajo contexto/mecanismo declarado, en su alcance. Acto profesional requiere evidencia propia.',
'Nombre de revisor sin objeto, alcance ni acto: no acredita K. Texto que afirma clausura no constituye estado de clausura soberano.',
'Falta evidencia del acto y objeto efectivamente mostrado; no es una razón para confundir revisión con clausura o añadir autoridad textual.',
'Mantener K pendiente y separar sus productores de ResolutionRecord y de los recibos documentales.', ['P12','P15','P29'])
# Agregar fuente exacta de §3, usada por O12.
excerpt('P31',I,81,159);rows[-1]['pasajes'].append('P31');J(D/'PASAJES_COTEJADOS.json',passages)
prior=json.loads((R.parent/'s11-integracion/entrega/MATRIZ_COBERTURA_A_L_RESULTADO_S11.json').read_text())
for r in rows:
 r['origen_criterios']=base+S+'s11-contexto-y-cobertura/MATRIZ_COBERTURA_A_L_RESULTADO_S11.json'
assert set('ABCDEFGHIJKL')=={c for r in rows for c in r['criterios_A_L']}
mat=dict(version='SV-S13-SUFICIENCIA/1',fecha_utc=T,corte=head,alcance='Cotejo documental y estructural; cero nuevas ejecuciones funcionales Rust, cero modelos consultados. Los controles candidatos no están ejecutados.',obligaciones=rows,cobertura_criterios={c:[r['id'] for r in rows if c in r['criterios_A_L']] for c in 'ABCDEFGHIJKL'},criterios_anteriores_preservados=[c['original_147'] for c in prior['casos']],promocion_nuclear=False,cierre_integral=False)
J(D/'MATRIZ_SUFiCIENCIA.json',mat)
# Nombre estable sin mezcla accidental de mayúsculas.
(D/'MATRIZ_SUFiCIENCIA.json').rename(D/'MATRIZ_SUFICIENCIA.json')
J(D/'CORTE_Y_ALCANCE.json',dict(fecha_cierre_documental_utc=T,cortes={k:json.loads((R/(k+'-base.json')).read_text()) for k in ['lenguaje','laboratorio']},lectura='Fuentes recibidas verificadas por blob y SHA-256; pasajes explícitos conservados. No se declara lectura íntegra de todo archivo recibido ni auditoría de toda la base de código.',ejecuciones_funcionales_nuevas=0,modelos_consultados=0,errores_o_correcciones_de_resultados_anteriores=[],comparacion_A_L='Referencias a criterios previos, sin modificarlos ni convertir cobertura de la matriz en aprobación.'))
text='''# S13 — Suficiencia de representación y realización para la integración 1+3

**Dictamen: mapa documental completado; suficiencia integral todavía no acreditada.** No se propone una extensión de IR en este acto. Se localizan por separado una representación nominal insuficiente para habilitar transducción, realizaciones ejecutivas pendientes y obligaciones materiales de frontera.

Este dictamen permite avanzar con objeto delimitado. No permite anunciar que toda consulta, transición o actuación profesional ya puede ejecutarse de forma nativa y auditada.

'''+f'Fecha de cierre documental: {T}. Responsable: Watson / W-S0. Suceso S13 / RETP-182. Corte de entrada: `{head}`.\n\n'+'''## Objeto y método

Se cumple el relevo S12: vincular cada obligación con contrato, representación, función Rust, evidencia anterior, control y pérdida pendiente. La matriz cubre los doce criterios A–L como inventario de obligaciones; **cobertura del inventario no significa que los doce criterios estén superados**. Se mantienen sus textos originales en el JSON.

Se han cotejado los pasajes indicados de las fuentes normativas, la ruta de compilación, tipos y funciones y el registro de deuda con sus recepciones posteriores. Las fuentes recibidas se identifican por corte, blob, longitud y SHA-256; los pasajes conservan texto exacto y líneas. La comprobación auxiliar verifica integridad y enlaces de la matriz, no decide por sí sola suficiencia semántica. No se han ejecutado nuevas pruebas funcionales Rust ni interrogado modelos en S13. Los resultados S11 citados son anteriores.

El nombre documental preciso es **gramática/superficie 0.2, IR 0.3 y serializador 0.1.0**. Las obligaciones semánticas se leen en la frontera normativa, IR 0.3 y partes heredadas no sustituidas de IR 0.2. No se inventa una cabecera de versión semántica independiente ni se aplica un párrafo histórico contra una rectificación vigente.

## Tres pérdidas distintas

1. **Transducción observación→Tri.** K1-T declara que cinco nombres no representan espacio, conjuntos ni función verificables. Aquí hay un límite de representación y de producción expresamente reconocido. La ruta sigue inhabilitada para 0, 1 y U; se conserva la puerta algebraica. Un texto de IA o un `mapping` nominal no la habilita.
2. **Consulta y evolución ejecutivas.** El contrato de QueryResult=(r,J,M) y CQ1–CQ6 existe. La ruta examinada construye una operación Query y devuelve IrProgram validada. Eso no produce la respuesta con la justificación requerida. TransitionData valida relaciones locales y declara no ejecutar la transición. DFL-003/004/006 conserva estos límites. S13 no demuestra que deba cambiarse la firma del contrato; identifica lo que falta realizar para un consumidor dependiente.
3. **Frontera material.** Carga efectiva, custodia durable, host, canales, archivo/pantalla y acto profesional tienen obligaciones propias. Añadir sus nombres a Frame no las realiza. S11 conserva evidencia contextual sintética, pero no prueba comportamiento de un LLM frente a inyección ni autenticidad frente al host adversario.

## Correspondencia de obligaciones

| ID | Obligación | Sede y límite decisivo |
| --- | --- | --- |
'''
for r in rows:text+='| '+r['id']+' | '+r['obligacion']+' | '+r['sede']+' |\n'
text+='''
La [matriz completa](MATRIZ_SUFICIENCIA.json) contiene contrato, IR, funciones, evidencia, control positivo, negativo, pérdida y tratamiento por fila. Los [pasajes cotejados](PASAJES_COTEJADOS.json) permiten recuperar el respaldo exacto en el corte original.

## Decisiones técnicas de este corte

- **Frame coherente no significa evidencia exhaustiva.** Se respetan ambos contratos. La cobertura de citas de S2/S11 y la cobertura de verificadores de R1 no se intercambian; falta demostrar el enlace donde una operación profesional lo necesite.
- **Main y candidatas permanecen separados.** R1 trazado existe en el código inspeccionado; RETP-153 registra cruces entre continuidades y su corrección candidata, y RETP-154 los productores acotados. Esas evidencias no se presentan como promoción a main ni como autoridad profesional constituida.
- **Los rechazos conservan sede y causa.** E004/E115 efectivos, variantes locales de contexto/cobertura y códigos históricos no se mezclan. Las candidatas de diagnóstico RETP-157–161 siguen con su alcance; DFL-001/011 permanece abierta.
- **No se cierran deudas por el resultado de un conductor.** Los límites conocidos siguen asociados a sus DFL y reservas existentes. Esta matriz no abre otra numeración de deuda ni modifica doctrina, semántica, IR, núcleo, universos o catálogo canónico.

## Relevo acotado G/J

La siguiente realización experimental puede estudiar **carga de una base documental y recuperación original frente a reevaluación**, con los contratos S2/S3/S11 como antecedentes. Su sede inicial es la frontera documental. Debe ligar la dependencia realmente consumida, el resultado y su base; conservar el original; identificar una reevaluación como nuevo acto con otra base y relación explícita.

Antes de ejecutar: fijar en el contrato el componente exacto que cambia, la dependencia consumida, las bases original/nueva, controles positivos, sustitución bajo igual nombre, falsa atribución al pasado, errores previstos y presupuesto. La evidencia debe distinguir declaraciones de huella y bytes realmente usados; el observador debe tener acceso independiente. Los rechazos candidatos aquí descritos no reciben todavía códigos canónicos ni se contabilizan como ensayos.

Ese montaje no podrá llamarse QueryResult nativo, transición SV ni continuidad durable. Si la operación escogida exige cualquiera de esas capacidades, su consumidor, representación y productor nuclear deberán quedar realizados y comprobados antes de admitirla. Así se puede continuar la integración sin que una estructura de laboratorio oculte una carencia del núcleo.

Se mantiene la recogida de causas durante la integración y la consolidación posterior del catálogo para el alcance correspondiente. La posición de agentes se valorará después del cierre de inmunología, tal como fijó Dirección; no queda decidida ahora. Tampoco se decide por este mapa la suficiencia general del primer universo de ciberseguridad. P3 y P4/P5/P6 conservan sus puertas y reservas.

## Custodia y reproducción del cotejo

[Fuentes](FUENTES_RECIBIDAS.json), [corte y alcance](CORTE_Y_ALCANCE.json), [rectores](RECTORES.json), [manifiesto](MANIFIESTO.json) y [comprobador de integridad](verificar.py). Para reproducir el cotejo, sitúe las fuentes de entrada con sus rutas de repositorio en un directorio y ejecute `python verificar.py --raiz-fuentes DIRECTORIO`. El comprobador exige los bytes exactos del corte, verifica los pasajes y las relaciones de la matriz. No modifica fuentes y no resuelve automáticamente las conclusiones de esta acta.

Los antecedentes S11 y la rectificación S12 se conservan; el relevo presente los sucede sin reescribirlos. Las copias pública y de laboratorio reciben los mismos bytes de esta carpeta. La verificación de publicación debe comprobar ambos árboles y la conservación de todos los blobs ajenos al mapa de publicación.
'''
(D/'ACTA_SUFICIENCIA.md').write_text(text)
print('Matriz:',len(rows),'obligaciones;',len(passages),'pasajes;',len(sources),'fuentes cotejadas.')
