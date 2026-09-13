from pathlib import Path
import json,hashlib,csv,io,re,subprocess
R=Path('manifiesto-sv/checkout');B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';D=B/'consolidacion-bis-02';HEAD='4fc7a2ceb2ece6d5d69d12e3216eda48e68f6136'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def old(p):return subprocess.check_output(['git','show',HEAD+':'+str(p.relative_to(R))],cwd=R)
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip()==HEAD
inv=json.loads((D/'INVENTARIO_FAMILIAS.json').read_text());fs=inv['familias'];assert len(fs)==12
original=json.loads((B/'BANCO_PREVIO_BIS_02_v0_1.json').read_text())['casos']
for f in fs:
 bank=B/f['banco'];j=json.loads(bank.read_text());assert sha(bank)==f['banco_sha256'];assert bank.read_bytes()==old(bank)
 assert f['ids_de_fila']==[c['id'] for c in j['casos']] and len(set(f['ids_de_fila']))==f['filas']
 assert f['ejecuciones_nuevas']==0
 pairs=[x for x in original if x['id'].startswith('BIS-'+f['familia']+'-')]
 assert f['escenarios_originales']==[x['id'] for x in pairs] and f['obligaciones']==pairs[0]['obligaciones']==pairs[1]['obligaciones']
 if f['familia']!='C01':assert j['ejecutadas']==0 and f['ejecuciones_historicas']==0
assert sum(f['filas'] for f in fs[1:])==202
r=json.loads((B/'bis-c01/evidencia/ejecucion-1/RESULTADO.json').read_text());assert r['variantes']==r['conformes']==13 and r['banco_sha256']==sha(B/fs[0]['banco'])
assert all(x['conforme'] for x in r['resultados'])
m=json.loads((D/'MATRIZ_COBERTURA_Y_SEDES.json').read_text())['filas'];assert len(m)==12 and len({x['obligacion'] for x in m})==12
for row in m:
 matches=[f for f in fs if row['obligacion'] in f['obligaciones']]
 assert row['familias']==[f['familia'] for f in matches]
 assert row['contratos']==[f['contrato'] for f in matches]
 assert row['acreditacion_integral'] is False and row['estatuto_sede']=='PROPUESTA_PARA_BIS_03_NO_DECISION'
g=json.loads((D/'CONDICIONES_DE_PASO.json').read_text());assert not g['cierre_bis02'] and not g['apertura_formal_bis03']
assert len(g['condiciones'])==len({x['id'] for x in g['condiciones']})==7
for f in json.loads((D/'FUENTES.json').read_text())['fuentes']:assert sha(R/f['ruta'])==f['sha256']
s=json.loads((B/'ESTADO_WORKFLOW.json').read_text());prior=json.loads(old(B/'ESTADO_WORKFLOW.json'))
assert s['etapas']==prior['etapas'] and s['escenarios_ejecutados']==2 and s['escenarios_pendientes']==22
for k in prior:
 if k.startswith('preparacion_bis_') and k!='preparacion_bis_c12':assert s[k]==prior[k]
c12=dict(s['preparacion_bis_c12']);c12['sincronizacion_local']=prior['preparacion_bis_c12']['sincronizacion_local'];assert c12==prior['preparacion_bis_c12']
for path in ['REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv','REGISTRO_EVOLUCION_TECNICA_PROYECTO.md','Inventario-sv/sucesos/HISTORIAL_SUCESOS_SV.csv']:
 p=R/'docs/calidad'/path;assert p.read_bytes().startswith(old(p))
p=R/'docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv';a=list(csv.DictReader(io.StringIO(old(p).decode())));z=list(csv.DictReader(p.open()))
assert [x for x in a if x['id']!='S22']==[x for x in z if x['id']!='S22']
row=next(x for x in z if x['id']=='S24');assert row['estado']=='pendiente' and not row['fecha_inicio_utc'] and not row['fecha_fin_utc']
paths=subprocess.check_output(['git','diff','--name-only'],cwd=R,text=True).splitlines();assert len(paths)==8 and all(p.startswith('docs/calidad/') and '/bis-c' not in p for p in paths)
for p in D.rglob('*.json'):json.loads(p.read_text())
for p in D.rglob('*.md'):
 for link in re.findall(r'\]\(([^)]+)\)',p.read_text()):
  if not link.startswith('http'):assert (p.parent/link).exists() or link=='VERIFICACION.json',(p,link)
subprocess.run(['git','diff','--check'],cwd=R,check=True)
for name in ['preparar.py','verificar.py','publicar.py','verificar_publicacion.py']:assert (D/'administracion'/name).read_bytes()==(Path('bis-consolidacion-02')/name).read_bytes()
result={'estatuto':'Cotejo documental Python; no compilación ni ensayo SV nuevo','corte':HEAD,'familias':12,'obligaciones':12,'filas_c02_c12':202,'variantes_c01_historicas':13,'ejecuciones_nuevas':0,'comprobaciones':['IDs y conteos contrastados con todos los bancos','Relaciones obligación/familia derivadas de los pares originales','Huellas de bancos, contratos y fuentes concordantes','Evidencia C01 y huella de banco enlazadas; sin reejecución','Doce propuestas de sede sin atribución de suficiencia integral','Siete condiciones de paso; BIS-02 abierto y BIS-03 pendiente','Bancos anteriores y prefijos RETP/historial intactos; sólo S22 cambia','S24 pendiente y estados anteriores conservados','Scripts archivados idénticos y enlaces internos comprobados'],'limites':['No conductor ni montaje integrado realizados','No efectos, imágenes o modelos ensayados','Toolchain no localizado; recuperación pendiente']}
(D/'VERIFICACION.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps(result,ensure_ascii=False))
