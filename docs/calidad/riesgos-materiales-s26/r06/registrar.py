"""R06: edición documental y registro append-only de la recepción autorizada.
Ejecutar desde checkout una sola vez. No ejecuta SV ni pruebas de comportamiento.
"""
from pathlib import Path
import json,hashlib,csv,datetime,subprocess
R=Path.cwd();E=Path(__file__).resolve().parent;Q=R/'docs/calidad';S=Q/'Inventario-sv/sucesos';V=Q/'tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad'
cut='b6be761da7223d9f26e5d7a41b83092330597276';lab='8a989c78b69b7a944a4fa1d5195c82d254d80b8a'
mods=['docs/arquitectura/CONTRATO_R2_0_PERSISTENCIA_CONTINUIDAD_Y_RECUPERACION_2026_08_25.md','docs/calidad/riesgos-materiales-s26/RECEPCION_CONTRACTUAL_R02.md','docs/calidad/riesgos-materiales-s26/README.md','docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv','docs/calidad/Inventario-sv/sucesos/HISTORIAL_SUCESOS_SV.csv','docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md','docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv','docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md','docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md','docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/MANIFIESTO.json','docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/ESTADO_WORKFLOW.json']
before={p:(R/p).read_bytes() for p in mods}
def rows(p):
 with p.open(newline='') as f:return list(csv.DictReader(f))
oldrows=rows(S/'SUCESOS_SV.csv');assert oldrows[-1]['id']=='S27'
source_names=set(json.loads((E.parent/'r05/FUENTES.json').read_text())['archivos'])
source_names.update(mods);source_names.update(['AGENTS.md','docs/arquitectura/CONTRATO_DE_CONTINUIDAD_Y_LIGADURAS_POR_OPERACION_FILA_7_2026_09_08.md','docs/arquitectura/CONTRATO_MATERIAL_DE_LIGADURAS_DFL_005_2026_09_08.md'])
for d in ['r03','r04','r05']:
 source_names.update(str(p.relative_to(R)) for p in (E.parent/d).rglob('*') if p.is_file() and '__pycache__' not in p.parts)
# El árbol leído del corte permite cotejar fuentes y candidatos locales sin fiarse de HEAD local antiguo.
# Extracto del árbol recuperado por GitHub al corte indicado, conservado como entrada.
by=json.loads((E/'BLOBS_CORTE.json').read_text())['blobs'];sources={}
for name in sorted(source_names):
 b=(R/name).read_bytes();blob=hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest();assert blob==by[name],name;sources[name]=dict(sha256=hashlib.sha256(b).hexdigest(),bytes=len(b),git_blob_sha1=blob)
(E/'FUENTES.json').write_text(json.dumps(dict(corte_lenguaje=cut,corte_laboratorio=lab,lectura='Rectores íntegros leídos en continuidad; mismos bytes cotejados al corte vigente. R2-0, R02, LIG y R05 consultados materialmente en este incremento. Listado incluye conservación de campañas; no afirma lectura narrativa de cada fixture.',archivos=sources),ensure_ascii=False,indent=2)+'\n')
md=(E/'README.md').read_text();cases=[]
for line in md.splitlines():
 if line.startswith('| R06-T'):
  id,stim,oracle=[x.strip() for x in line.split('|')[1:4]];cases.append(dict(id=id,estimulo=stim,control_y_evidencia=oracle,estado='ESPECIFICADO_NO_EJECUTADO'))
assert len(cases)==8
(E/'DISCRIMINADORES.json').write_text(json.dumps(dict(version='S26-R06-DISCRIMINADORES/1',naturaleza='Especificación contractual; no precompromiso de campaña ejecutable.',casos=cases,pruebas_ejecutadas=0),ensure_ascii=False,indent=2)+'\n')
now=datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ');url='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/'
result='R06: recepción contractual de terminación sin informe y alcance de identidad de fuente. Se separan informe, observación de terminación y efecto; mínimo de encargo/referentes autorizados y contenido consumido; continuidad de soporte sólo si su perfil la exige. Ocho discriminadores especificados, cero ejecutados.'
verification='Fuentes cotejadas por blobs contra corte vigente; campañas anteriores y registros conservados. Verificación documental, sin nueva compilación ni protección ejecutable acreditada.'
next_action='Preparar variante local con correlación explícita, informe y terminación separados y ligadura de contenido admitido-consumido; precomprometer código, fixtures, cuotas y oráculos antes de ejecutar T01/T02/T05/T06. Después concretar observador de proceso y T03/T04/T08; T07 con barreras propias. Conservar S26, Bis y S24.'
seat=dict(suceso=dict(id='S26',fecha_actualizacion_utc=now,cortes_de_entrada='Lenguaje '+cut+'; laboratorio '+lab,resultado=result,verificacion=verification,siguiente_accion=next_action,evidencias=url+'docs/calidad/riesgos-materiales-s26/r06/README.md',referencia_calidad=url+'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-233',observaciones='Recepción de obligaciones, no cierre material. Doce casos globales S26 abiertos; R2/R3/R4, DFL y Bis conservan alcance. No GUI, BD, nueva identidad universal ni selección de host. S27 y el dominio permanecen en su sede.'),revision=10,retp=233,tipo='RECEPCION_CONTRACTUAL_MATERIAL',motivo='Luz verde para concretar los límites R05 antes de integrar.',base='R02 C01-C04/C06-C08; R2-0 §§4-7/10-15; LIG/0.1 §§2-4; R05 P04-P07.',artefactos='R06 y discriminadores; adendas R02/R2-0; Sucesos, historial, RETP CSV/MD, navegación y seguimiento S26.',impacto='Obligaciones recibidas para candidato local; sin alteración de semántica, IR o código productivo.',titulo='R06 · Terminación sin informe e identidad de fuente')
sp=E/'ASIENTO_RETP233.json';sp.write_text(json.dumps(seat,ensure_ascii=False,indent=2)+'\n');subprocess.run(['python',str(E.parent/'soporte/registrar_continuacion.py'),str(sp)],check=True)
adendas={mods[0]:'''\n\n## 21. Recepción de terminación y referentes materiales · S26 R06 / RETP-233

La [recepción R06](../calidad/riesgos-materiales-s26/r06/README.md) desarrolla las obligaciones de §§4–7 y 10–15 tras los límites observados en R05. Distingue informe recibido, observación de terminación y efecto acreditado. Pánico, canal cerrado o informe ausente no acreditan cero efectos ni rollback y no autorizan reintento automático. Una evidencia parcial conserva su propio alcance.

La realización debe ligar el encargo y sus referentes autorizados con el contenido efectivamente consumido. La identidad del objeto del soporte es una exigencia adicional cuando la operación la constituya; mismo contenido no la acredita, y un inode no se adopta como identidad universal. Conservar un descriptor tampoco demuestra inmutabilidad ante escrituras sobre ese objeto. R06 no constituye otra AStore ni añade tiempo como primitiva.

Recepción contractual con ocho discriminadores, cero nuevos ensayos ejecutados. No cierra R2, R3 ni la continuidad durable; el candidato y su observador deberán fijarse y probarse antes de ofrecer la capacidad.
''',mods[1]:'''\n\n## 7. Desarrollo de C01–C04 y C06–C08 · R06 / RETP-233

La [recepción R06](r06/README.md) precisa terminación sin informe, conservación de evidencia parcial, correlación por intento y separación entre contenido, ocurrencia SV, objeto de soporte, procedencia y autoridad. Recibe los contraejemplos R05 P04–P07. La obligación mínima para el próximo montaje local liga encargo, referentes autorizados y contenido consumido; no convierte el inode en identidad soberana ni deduce ausencia de efectos del silencio.

Los ocho discriminadores R06 requieren realización y precompromiso propios. R02 conserva sus obligaciones y la matriz T01–T08 con su corte histórico. No se añaden pruebas ejecutadas por esta adenda ni se cierran los perfiles durables pendientes.
'''}
for name,addition in adendas.items():p=R/name;p.write_text(p.read_text()+addition)
for p,link in [(E.parent/'README.md','r06/README.md'),(V/'LEAME_PRIMERO.md','../../riesgos-materiales-s26/r06/README.md')]:p.write_text(p.read_text()+'\n## S26 R06 · RETP-233\n\n'+result+' '+next_action+' [Recepción y discriminadores]('+link+').\n')
p=R/mods[-1];j=json.loads(p.read_text());prior=json.loads(before[mods[-1]]);j['seguimiento_transversal_s26'].update(registro='RETP-2026-233',recepcion_r06='../../../riesgos-materiales-s26/r06/README.md',discriminadores_r06_especificados=8,discriminadores_r06_ejecutados=0,casos_globales_cerrados=0,siguiente_accion=next_action);p.write_text(json.dumps(j,ensure_ascii=False,indent=2)+'\n');assert {k:v for k,v in prior.items() if k!='seguimiento_transversal_s26'}=={k:v for k,v in j.items() if k!='seguimiento_transversal_s26'}
p=V/'MANIFIESTO.json';j=json.loads(p.read_text());b=(V/'LEAME_PRIMERO.md').read_bytes();j['archivos']['LEAME_PRIMERO.md']=dict(bytes=len(b),sha256=hashlib.sha256(b).hexdigest(),git_blob_sha1=hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest());j['actualizaciones'].append(dict(fecha=now[:10],registro='RETP-2026-233',alcance='Enlace a recepción S26 R06; PDF/MD explicativos conservados.'));p.write_text(json.dumps(j,ensure_ascii=False,indent=2)+'\n')
assert [x for x in oldrows if x['id']!='S26']==[x for x in rows(S/'SUCESOS_SV.csv') if x['id']!='S26']
for name in [mods[0],mods[1],mods[2],mods[4],mods[6],mods[7],mods[8]]:assert (R/name).read_bytes().startswith(before[name]),name
for name,x in sources.items():
 if name not in mods:assert hashlib.sha256((R/name).read_bytes()).hexdigest()==x['sha256'],name
report=dict(conforme=True,corte_lenguaje=cut,corte_laboratorio=lab,fuentes_cotejadas=len(sources),otros_sucesos_conservados=True,estado_bis_fuera_de_s26_conservado=True,antecedentes_sin_modificar=True,nuevos_contrastes_ejecutados=0,archivos_editados={n:dict(antes=hashlib.sha256(b).hexdigest(),despues=hashlib.sha256((R/n).read_bytes()).hexdigest()) for n,b in before.items()})
(E/'VERIFICACION.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n');print('R06: verificación documental conforme;',len(sources),'fuentes; cero ensayos nuevos.')
