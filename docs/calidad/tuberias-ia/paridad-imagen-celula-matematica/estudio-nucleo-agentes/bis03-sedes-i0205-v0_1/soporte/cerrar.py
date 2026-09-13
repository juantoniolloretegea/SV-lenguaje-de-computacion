from pathlib import Path
import json,hashlib,subprocess,csv,io,shutil,re
R=Path('manifiesto-sv/checkout');B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';D=B/'bis03-sedes-i0205-v0_1';base='3c4f5f87e2e052daacb25be4a4dfe7c1db0d946d'
def h(b):return hashlib.sha256(b).hexdigest()
def dump(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
def old(p):return subprocess.check_output(['git','show',base+':'+p],cwd=R)
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip()==base
for p,v in json.loads((D/'FUENTES.json').read_text())['archivos'].items():assert h((R/p).read_bytes())==v['sha256'] and old(p)==(R/p).read_bytes()
append=['docs/calidad/Inventario-sv/sucesos/HISTORIAL_SUCESOS_SV.csv','docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv','docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md']
for p in append:assert (R/p).read_bytes().startswith(old(p))
p='docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv';a=list(csv.DictReader(io.StringIO(old(p).decode())));b=list(csv.DictReader((R/p).open()));assert len(a)==len(b);assert [x['id'] for x,y in zip(a,b) if x!=y]==['S22']
s=json.loads((B/'ESTADO_WORKFLOW.json').read_text());assert next(e for e in s['etapas'] if e['id']=='BIS-02')['estado']=='en ejecución';assert next(e for e in s['etapas'] if e['id']=='BIS-03')['estado']=='en ejecución';assert next(e for e in s['etapas'] if e['id']=='BIS-04')['estado']=='pendiente';assert s['integracion_c02_c05']['casos_ejecutados']==0
allowed=set(append+['docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv','docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md',str((B/'ESTADO_WORKFLOW.json').relative_to(R)),'docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md'])
assert set(subprocess.check_output(['git','diff','--name-only'],cwd=R,text=True).splitlines())==allowed
helpers={}
for name,src in [('registro.py','bis-c02/registro.py'),('gestion.py','p1p3-bis/gestion.py'),('publicar_archivos.py','manifiesto-sv/publicar_archivos.py'),('github_io.py','s6-trazabilidad-total/github_io.py')]:
 p=B/'integracion-c02-c05-v0_1/soporte'/name;assert p.read_bytes()==Path(src).read_bytes();helpers[str(p.relative_to(R))]=h(p.read_bytes())
dump(D/'CONSERVACION.json',{'resultado':'CONFORME_DOCUMENTAL','corte':base,'suceso_modificado':'S22','otros_sucesos':'intactos','append_only':append,'seguimiento_modificado':sorted(allowed),'banco_integrado_previo':'26 casos y manifiesto íntegros','filas_previas_c02_c12':202,'ejecuciones_nuevas_rust':0,'workspace_y_fuentes_rust_productivos':'sin cambios','auxiliares_reutilizados_contenido_identico':helpers,'versiones_observadas':{x:subprocess.check_output([x,'--version'],text=True).strip() for x in ('rustc','cargo')}})
for name in ('preparar.py','verificar.py','registrar.py','publicar.py','verificar_publicacion.py','cerrar.py'):shutil.copyfile(Path('bis-sedes')/name,D/'soporte'/name)
raw=subprocess.check_output(['python',str(D/'soporte/verificar.py')]);(D/'VERIFICACION_DOCUMENTAL.json').write_bytes(raw)
for p in D.glob('*.md'):
 for target in re.findall(r'\]\(([^)]+)\)',p.read_text()):
  if '://' not in target and target!='MANIFIESTO.json':assert (p.parent/target).exists(),target
dump(D/'MANIFIESTO.json',{str(p.relative_to(D)):h(p.read_bytes()) for p in sorted(D.rglob('*')) if p.is_file() and p.name!='MANIFIESTO.json'})
assert json.loads(subprocess.check_output(['python',str(D/'soporte/verificar.py')]))==json.loads(raw)
print('Sedes, compatibilidad y conservación conformes. Archivos comprometidos:',len(json.loads((D/'MANIFIESTO.json').read_text())))
