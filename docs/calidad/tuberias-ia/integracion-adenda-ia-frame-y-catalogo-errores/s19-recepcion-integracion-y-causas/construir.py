"""Recepción documental S19. Sólo coteja/ordena bytes ya publicados; no ejecuta SV."""
from pathlib import Path
import base64,copy,hashlib,json
D=Path(__file__).resolve().parent
J=lambda p,x:p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
sha=lambda b:hashlib.sha256(b).hexdigest()
S=json.loads((D/'FUENTES_RECIBIDAS.json').read_text())
def source(suffix):
 matches=[(p,x) for p,x in S.items() if p.endswith(suffix)]
 assert len(matches)==1,suffix
 p,x=matches[0];b=base64.b64decode(x['base64'],validate=True)
 assert len(b)==x['bytes'] and sha(b)==x['sha256']
 assert hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()==x['blob']
 return p,x,b
def obj(s):return json.loads(source(s)[2])
def url(s):return source(s)[1]['url']
for p in S:source(p)
old=obj('MATRIZ_COBERTURA_A_L_RESULTADO_S16.json');m=copy.deepcopy(old)
# Juicios documentales explícitos: las capturas demuestran los casos indicados,
# no una equivalencia universal con los ataques originales A–L.
notes={
'A':(['I02','I04','I06','I08'],'El contexto hostil instalado se conserva como dato; el cambio de contexto y la omisión de dependencias se rechazan en casos separados.','S18 no combina en un mismo caso orden hostil y omisión; AH01–AH05 de S11 conservan su alcance. No prueba conducta de un LLM real.'),
'B':([], 'Sin nuevo respaldo de autoridad directa en S18.','Cadena profesional real, permisos y efecto R1 pendientes; preservar S16 y puertas P3/P4/P5/P6.'),
'C':(['I06','I07','I08','I09','I14'],'El recorrido rechaza solicitud/base ausente o ajena y consume una base negativa en la reevaluación.','Cobertura fija; no selección arbitraria de corpus ni caso combinado de citas verdaderas con revocación omitida en S18. Se conserva S1/S2.'),
'D':(['I10','I16','I19','I24'],'Rechaza cambio de valor y cuerpo de otro episodio; detecta sustitución posterior y admite los separadores autorizados.','S18 no ensaya supresión literal de la negación; evidencia complementaria a D01–D04 de S3, sin ampliar aquel criterio.'),
'E':([], 'Sin nuevo respaldo sobre extracción de secretos o canales.','Confidencialidad, vista autorizada y canales efectivos pendientes; P4 conservada.'),
'F':(['I11','I12','I13','I18','I19','I21','I22'],'Separa comunicación, negativa, esquema, exceso y fallos antes/después de la admisión y escritura.','Montaje sintético; no cubre todos los fallos del host ni modifica Indeterminate tras DispatchCommitted en la frontera R1.'),
'G':(['I03','I14'],'Detecta bytes de base cambiados bajo el mismo nombre; la nueva base admitida se consume realmente.','Base concreta del banco; no acredita integridad de todos los componentes ni autenticidad por hash.'),
'H':(['I04','I05','I20','I23'],'Contexto exacto e identidad se comprueban; frontera de 8192/8193 bytes ensayada.','No memoria general ni ensayo específico de truncamiento documental del contexto en S18; conservar antecedentes A/H.'),
'I':(['I06','I07','I08','I09'],'La referencia procede de G1 y se fija fuera de la propuesta; no se constituye por autocita. Clientes externos no fabrican Referencia ni EntregaComprobada.','Instalador y host confiables; no auditor independiente de todas las acciones de producción. Dos rechazos de compilación adicionales, no casos I nuevos.'),
'J':(['I14','I15','I16','I17'],'La reevaluación conserva el original y distingue cuerpo e identidad entre episodios.','Dos episodios intraproceso; no persistencia tras caída ni historia durable general.'),
'K':([], 'Sin nuevo respaldo de revisión humana efectiva.','Identidad/bytes conservados no prueban revisión profesional real ni competencia del revisor.'),
'L':([], 'Sin nuevo respaldo de finalidad y destinatario autorizado.','Límites de tamaño y destino fijado por el instalador no constituyen minimización ni legitimidad del destinatario.')}
for row in m['casos']:
 cases,evidence,gap=notes[row['id']]
 row['recepcion_s19_resultado_s18']={'revision_documental':True,'ensayo_nuevo_en_s19':False,'estado':'EVIDENCIA_COMPLEMENTARIA_EN_RECORRIDO_SINTETICO' if cases else 'SIN_NUEVO_RESPALDO_S18','casos_s18':cases,'evidencia':evidence,'limite':gap,'cierre_universal':False,'fuente':url('ACTA_RESULTADO_S18.md')}
m['version']='REVISION-COBERTURA-S19-RECEPCION-S18/1'
m['recepcion_s19']={'corte':next(iter(S.values()))['corte'],'matriz_antecedente':url('MATRIZ_COBERTURA_A_L_RESULTADO_S16.json'),'criterios_y_resultados_anteriores_intactos':True,'nota':'Añade recepción documental por fila. El corte histórico de la matriz se conserva. No declara cerrados los doce ataques originales por el mero éxito del banco integrado.'}
J(D/'MATRIZ_COBERTURA_A_L_RECEPCION_S18.json',m)
text='# Matriz A–L: recepción documental de S18\n\nS19 conserva literalmente los campos anteriores en el JSON asociado y añade esta recepción. No hay ensayo nuevo ni cierre universal.\n\n| Fila | Obligación original | Casos S18 | Respaldo recibido | Límite que permanece |\n|---|---|---|---|---|\n'
for r in m['casos']:
 n=r['recepcion_s19_resultado_s18'];text+='| '+' | '.join([r['id'],r['original_147']['titulo'],', '.join(n['casos_s18']) or '—',n['evidencia'],n['limite']])+' |\n'
text+='\nFuente: [matriz anterior]('+url('MATRIZ_COBERTURA_A_L_RESULTADO_S16.json')+'), [resultado S18]('+url('ACTA_RESULTADO_S18.md')+'). Los JSON conservan la relación completa con S11/S14/S15/S16.\n'
(D/'MATRIZ_COBERTURA_A_L.md').write_text(text)
# Indexación forense de registros textuales; NO constituye causas técnicas
# desde Debug ni aplica esos textos como decisión operativa del SV.
e=obj('EVIDENCIAS_S18.json')['archivos'];by={x['ruta']:x for x in e};assert len(by)==3382
for x in e:
 b=base64.b64decode(x['base64'],validate=True);assert sha(b)==x['sha256'] and len(b)==x['bytes']
normal=['debug/capturas-'+str(i) for i in range(1,4)]+['release/capturas-'+str(i) for i in range(1,4)]
ref={p.split('/',2)[2]:x for p,x in by.items() if p.startswith(normal[0]+'/')};assert len(ref)==449
for pref in normal:
 z={p.split('/',2)[2]:x for p,x in by.items() if p.startswith(pref+'/')};assert set(z)==set(ref)
 for p,x in z.items():assert x['base64']==ref[p]['base64']
groups={}
for p,x in ref.items():
 if not p.endswith(('.diagnostico','.admision','.escritura')):continue
 b=base64.b64decode(x['base64']);fields=b.decode().rstrip('\n').split('\t');assert len(fields)==5
 groups.setdefault(tuple(fields),[]).append(x)
assert len(groups)==20
rows=[]
for i,(key,entries) in enumerate(groups.items(),1):
 stage,code,detail,es,en=key
 rows.append({'id_inventario':'S19-OBS-'+str(i).zfill(2),'naturaleza':'estado_conforme' if code=='CONFORME' else 'negativa' if code=='NEGATIVA_PROVEEDOR' else 'fallo_o_rechazo','origen':'realizacion_sintetica_S18','etapa_emitida':stage,'codigo_local_emitido':code,'detalle_textual_emitido_no_parseado':detail,'rotulo_es_emitido':es,'rotulo_en_emitido':en,'codigo_canonico_sv':None,'correspondencia_canonica':'NO_CONSTITUIDA','casos':sorted({x['ruta'].split('/')[-1].split('.')[0] for x in entries}),'evidencias':[{'ruta':x['ruta'],'bytes':x['bytes'],'sha256':x['sha256'],'base64':x['base64']} for x in entries],'ambito_repeticion':'Mismos bytes en las seis ejecuciones normales; se indexa debug/capturas-1 sin contar duplicados como nuevos casos.'})
inv={'version':'S19-INVENTARIO-OBSERVABLES-S18/1','naturaleza':'Recepción de registros ya emitidos. No es un esquema operativo de causas ni una ampliación del catálogo efectivo.','fuente':url('EVIDENCIAS_S18.json'),'total_tuplas':20,'estados_conformes':3,'negativas':1,'variantes_fallo_o_rechazo_por_etapa':16,'filas':rows,'origen_compilador_separado':{'codigo':'E0451','origen':'rustc','objeto':'Dos clientes externos intentan fabricar tipos con campos privados; compilaciones rechazadas según S18.','fuente':url('COMANDOS.json'),'codigo_canonico_sv':None},'antecedente_s15':{'fuente':url('CAUSAS_OBSERVADAS.json'),'contenido_original':obj('CAUSAS_OBSERVADAS.json'),'regla':'Conservado sin renombrar FaltaCaso/FaltaVigencia a FaltaSolicitud/FaltaBase; relación de obligaciones, no identidad de causas. S18 no reejecuta todas las variantes S15.'}}
J(D/'INVENTARIO_CAUSAS_Y_ESTADOS.json',inv)
t='# Inventario recibido de causas y estados S18\n\n20 tuplas distintas de etapa, código, detalle textual y rótulos: 3 estados conformes, 1 negativa y 16 variantes de fallo/rechazo por etapa. Son registros locales del banco; no se asignan códigos canónicos E… por semejanza. Se conserva el detalle literal, sin interpretarlo como una estructura técnica.\n\n| Referencia documental | Etapa | Código local | Detalle emitido | ES / EN | Casos |\n|---|---|---|---|---|---|\n'
for r in rows:t+='| '+' | '.join([r['id_inventario'],r['etapa_emitida'],r['codigo_local_emitido'],r['detalle_textual_emitido_no_parseado'],r['rotulo_es_emitido']+' / '+r['rotulo_en_emitido'],', '.join(r['casos'])])+' |\n'
t+='\nLos identificadores S19-OBS son filas documentales, no nuevos códigos del Lenguaje. Cada fila JSON enlaza capturas originales con bytes, SHA-256 y base64. Admission/escritura/resultado pueden repetir un diagnóstico: no representan pruebas adicionales. `CONFORME` en cobertura no convierte un fallo posterior en éxito global. `E0451` pertenece a rustc. La negativa del proveedor I12 no equivale al recibo documental `PERMISO_REVOCADO` correctamente entregado en I14.\n\nEl inventario S15 se conserva íntegro dentro del JSON. No se homologa su nomenclatura retrospectivamente.\n'
(D/'INVENTARIO_CAUSAS_Y_ESTADOS.md').write_text(t)
ph=obj('FRONTERAS_DE_FALLO.json')
J(D/'FRONTERAS_RECIBIDAS.json',{'version':'S19-FRONTERAS/1','antecedente_s16_intacto':ph,'nuevas_filas_documentales':[
 {'fase':'Antes de producción','casos':['I03','I20'],'resultado':'Rechazo de base/contexto impide constituir el episodio en esos casos.'},
 {'fase':'Antes de entrega comprobada','casos':['I04','I05','I06','I07','I08','I09','I10','I11','I12','I13','I21'],'resultado':'No se emite entrega comprobada ni escritura por este recorrido; no afirma nada de efectos externos al banco.'},
 {'fase':'Tras admisión, apertura fallida','casos':['I18'],'resultado':'AlreadyExists; se conserva el archivo previo. Admisión conforme no significa escritura conforme.'},
 {'fase':'Tras admisión, escritura parcial','casos':['I22'],'resultado':'WriteZero inyectado tras siete bytes. Existe prefijo escrito; no se declara ausencia de efectos ni escritura atómica.'},
 {'fase':'Tras escritura, recuperación','casos':['I19'],'resultado':'Sustitución detectada posteriormente; la escritura inicial conforme no impide alteración ulterior.'},
 {'fase':'Recibo de resolución negativa','casos':['I14'],'resultado':'Entrega documental conforme del recibo PERMISO_REVOCADO; no permiso de actuación ni negativa del proveedor.'}]})
result=obj('RESULTADO.json');assert result['conforme'] and result['observaciones']==144
J(D/'RECEPCION.json',{'version':'S19-RECEPCION/1','estado':'RECEPCION_DOCUMENTAL_CONFORME','resultado_s18_intacto':result,'fuentes_integras':len(S),'matriz_filas':len(m['casos']),'tuplas_observadas':len(rows),'ejecuciones_sv_nuevas':0,'modificacion_nucleo':False,'modificacion_oraculos':False,'entrada_al_catalogo':'HABILITADA_PARA_INVENTARIO_Y_CONTRATO_ACOTADO','cierre_nuclear':False,'cierre_profesional':False})
print('S19: 20 fuentes, 12 filas preservadas, 20 tuplas, 3382 capturas cotejadas; cero ensayos nuevos.')
