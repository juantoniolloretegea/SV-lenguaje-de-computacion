from pathlib import Path
import subprocess,json,hashlib,csv,io,re,shutil
R=Path('manifiesto-sv/checkout');B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';D=B/'integracion-c02-c05-v0_1';base='18963fa275d2a29633e164d9a2ea5fe1c4f11216'
def dump(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
def old(p):return subprocess.check_output(['git','show',base+':'+p],cwd=R)
def h(b):return hashlib.sha256(b).hexdigest()
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip()==base
for p,v in json.loads((D/'FUENTES.json').read_text())['rectores_y_contratos'].items():assert h((R/p).read_bytes())==v and old(p)==(R/p).read_bytes()
appends=['docs/calidad/Inventario-sv/sucesos/HISTORIAL_SUCESOS_SV.csv','docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv','docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md']
for p in appends:assert (R/p).read_bytes().startswith(old(p))
p='docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv';prev=list(csv.DictReader(io.StringIO(old(p).decode())));now=list(csv.DictReader((R/p).open()));assert len(prev)==len(now)
assert [a['id'] for a,b in zip(prev,now) if a!=b]==['S22']
s=json.loads((B/'ESTADO_WORKFLOW.json').read_text());assert s['escenarios_ejecutados']==2 and s['escenarios_pendientes']==22 and s['variantes_nativas_ejecutadas']==13
assert all(s['preparacion_bis_c'+k]['variantes_ejecutadas']==0 for k in ['02','03','04','05','06','07','08','09','10','11','12'])
assert next(x for x in now if x['id']=='S24')['estado']=='pendiente'
allowed=set(appends+['docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv','docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md',str((B/'ESTADO_WORKFLOW.json').relative_to(R)),'docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md'])
assert set(subprocess.check_output(['git','diff','--name-only'],cwd=R,text=True).splitlines())==allowed
report={'resultado':'CONFORME','base':base,'suceso_modificado':'S22','otros_sucesos':'idénticos al corte de entrada','registros_append_only':appends,'escenarios_originales':{'ejecutados':2,'pendientes':22},'variantes_c01_historicas':13,'filas_c02_c12_previas':202,'integracion_nueva':{'preparados':26,'ejecutados':0},'archivos_seguimiento_modificados':sorted(allowed),'herramientas_comprobadas':{tool:subprocess.check_output([tool,'--version'],text=True).strip() for tool in ('rustc','cargo')},'cambios_rust':False,'bancos_previos_modificados':False}
dump(D/'CONSERVACION.json',report)
shutil.copyfile(__file__,D/'soporte/cerrar_documentacion.py')
# ES: Se escribe evidencia de comprobación, nunca observaciones del receptor.
# EN: Write documentary evidence, never receptor observations.
raw=subprocess.check_output(['python',str(D/'soporte/verificar.py')]);(D/'VERIFICACION_DOCUMENTAL.json').write_bytes(raw+b'\n' if not raw.endswith(b'\n') else raw)
for f in D.glob('*.md'):
 for target in re.findall(r'\]\(([^)]+)\)',f.read_text()):
  if '://' not in target and target!='MANIFIESTO.json':assert (f.parent/target.split('#')[0]).exists(),(f,target)
dump(D/'MANIFIESTO.json',{str(p.relative_to(D)):h(p.read_bytes()) for p in sorted(D.rglob('*')) if p.is_file() and p.name!='MANIFIESTO.json'})
raw2=subprocess.check_output(['python',str(D/'soporte/verificar.py')]);assert json.loads(raw2)==json.loads(raw)
print('Verificación documental conforme; conservación conforme; manifiesto',len(json.loads((D/'MANIFIESTO.json').read_text())),'archivos.')
