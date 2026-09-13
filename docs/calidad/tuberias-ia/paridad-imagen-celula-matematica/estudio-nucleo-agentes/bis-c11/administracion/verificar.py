from pathlib import Path
import json,csv,hashlib,subprocess,re,io
R=Path('manifiesto-sv/checkout');B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';D=B/'bis-c11';HEAD='b22d03cf369bbe9cab902846d061d5de46f76397'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n')
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip()==HEAD
b=json.loads((D/'BANCO_PREVIO_v0_1.json').read_text());cs=b['casos'];refs=json.loads((D/'ENTRADAS_COMPROMETIDAS.json').read_text())['archivos']
assert b['variantes']==len(cs)==20 and b['ejecutadas']==0 and len({c['id'] for c in cs})==20
assert sum(c['clase']=='positivo' for c in cs)==6 and sum(c['clase']=='negativo' for c in cs)==14
assert len(refs)==len(list((D/'entradas').iterdir()))==4
assert sha(D/b['presupuesto']['ruta'])==b['presupuesto']['sha256']
for p,r in refs.items():assert sha(D/p)==r['sha256'] and (D/p).stat().st_size==r['bytes']
assert len((D/'entradas/utf8-4096.txt').read_text())==2048
assert len((D/'entradas/utf8-4097.txt').read_text())==2049
assert len((D/'entradas/utf8-4096.txt').read_bytes())==4096
assert len((D/'entradas/utf8-4097.txt').read_bytes())==4097
p=json.loads((D/'PRESUPUESTO_SINTETICO.json').read_text());assert p['memoria_proceso_max_bytes'] is None and p['tiempo_maximo_ms'] is None
assert all(type(v) is int and v>0 for v in p['limites'].values())
assert 256*256==p['limites']['imagen_pixeles_decodificados']
assert 256*256*4==p['limites']['imagen_salida_rgba_bytes']
assert (2**32-1)**2*4>2**64-1
node=cs[10]['entrada_o_estimulo'];assert sum(node['celdas_n'])==164 and len(node['celdas_n'])==node['nodos']==8
edges=node['enlaces'];depth=[1]*8
for a,z in edges:assert a<z;depth[z]=max(depth[z],depth[a]+1)
assert max(depth)==4
for c in cs:
 assert c['estado']=='ESPECIFICADO_NO_EJECUTADO' and c['resultado_observado'] is None
 assert c['resultado_esperado_contractual'] and c['sede_a_contrastar'] and c['observador_requerido']
 for f in c['entrada_o_estimulo'].get('archivos',[]):assert f in refs
assert len(cs[12]['entrada_o_estimulo']['variantes'])==2
for f in json.loads((D/'FUENTES.json').read_text())['fuentes']:assert sha(R/f['ruta'])==f['sha256'],f['ruta']
for file in D.glob('*.md'):
 for link in re.findall(r'\]\(([^)]+)\)',file.read_text()):
  if not link.startswith('http'):assert (file.parent/link).exists() or link=='VERIFICACION_DOCUMENTAL.json',(file,link)
for name in ['REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv','REGISTRO_EVOLUCION_TECNICA_PROYECTO.md','Inventario-sv/sucesos/HISTORIAL_SUCESOS_SV.csv']:
 path='docs/calidad/'+name;assert (R/path).read_bytes().startswith(subprocess.check_output(['git','show','HEAD:'+path],cwd=R)),path
path='docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv';old=list(csv.DictReader(io.StringIO(subprocess.check_output(['git','show','HEAD:'+path],cwd=R,text=True))));new=list(csv.DictReader((R/path).open()))
assert [r for r in old if r['id']!='S22']==[r for r in new if r['id']!='S22']
s24=next(r for r in new if r['id']=='S24');assert s24['estado']=='pendiente' and not s24['fecha_inicio_utc'] and not s24['fecha_fin_utc']
s=json.loads((B/'ESTADO_WORKFLOW.json').read_text());assert s['escenarios_ejecutados']==2 and s['escenarios_pendientes']==22 and s['ultimo_retp']=='RETP-2026-214'
assert s['preparacion_bis_c11']['variantes_ejecutadas']==0
changed=subprocess.check_output(['git','diff','--name-only'],cwd=R,text=True).splitlines();assert len(changed)==8 and all(x.startswith('docs/calidad/') and '/bis-c' not in x for x in changed)
subprocess.run(['git','diff','--check'],cwd=R,check=True)
for name in ['preparar.py','verificar.py','publicar.py','verificar_publicacion.py']:assert (D/'administracion'/name).read_bytes()==(Path('bis-c11')/name).read_bytes()
save(D/'VERIFICACION_DOCUMENTAL.json',{'estatuto':'Cotejo auxiliar Python de inventario y cálculos de referencia; no recepción ni control material de recursos en Rust','corte':HEAD,'comprobaciones':['Veinte escenarios únicos: seis positivos, catorce negativos; C11-13 contiene dos subcasos; todos sin resultado observado','Cuatro textos literales4096/4097 bytes ASCII/UTF-8, huellas y tamaños concordantes','Cuotas sintéticas positivas e inclusivas; proceso y duración sin valores inventados','Imagen256×256×4=262144 bytes de salida; producto extremo no representable en u64 calculado sin asignación','Descriptor de ocho nodos, profundidad4 y164 posiciones; sin admisión semántica alegada','Fuentes y enlaces del expediente cotejados; contenido de scripts archivado exacto','Prefijos de historial/RETP conservados; sólo S22 actualizado; C01–C10 intactos; S24 pendiente'],'ensayos_c11_ejecutados':0,'limites':['No compilación nueva ni medición de memoria, tiempo o contención','No imagen comprimida materializada ni asignación gigante','No actividad de un modelo ejecutada; capturas materiales pendientes','La comprobación auxiliar de suma/producto no prueba las guardas de Rust']})
print('Cotejo documental conforme:20 escenarios;4 textos;0 ensayos funcionales C11.')
