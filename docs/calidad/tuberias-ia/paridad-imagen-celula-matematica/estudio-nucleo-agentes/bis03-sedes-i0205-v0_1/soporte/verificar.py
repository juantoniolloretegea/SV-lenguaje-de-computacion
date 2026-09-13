"""ES: Comprobación documental de sedes y banco previo; no ejecución Rust.
EN: Documentary check of ownership decisions and precommitted cases, not Rust execution.
"""
from pathlib import Path
import json,hashlib
D=Path(__file__).resolve().parents[1] if Path(__file__).name=='verificar.py' and Path(__file__).parent.name=='soporte' else Path('manifiesto-sv/checkout/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis03-sedes-i0205-v0_1')
B=D.parent
load=lambda p:json.loads((D/p).read_text())
h=lambda b:hashlib.sha256(b).hexdigest()
def metrics(x,depth=0):
 if isinstance(x,(dict,list)):
  vals=list(x.values()) if isinstance(x,dict) else x
  triples=[metrics(v,depth+1) for v in vals]
  return (1+sum(v[0] for v in triples),max([depth+1]+[v[1] for v in triples]),max([len(x) if isinstance(x,dict) else 0]+[v[2] for v in triples]))
 return (1,depth,0)
limits=load('LIMITES_ADAPTADOR.json');bank=load('BANCO_ADAPTADOR_PREVIO.json');cases=bank['casos'];assert len(cases)==19
assert len({c['id'] for c in cases})==19 and all(c['observado'] is None and c['estado']=='PENDIENTE' for c in cases)
for c in cases:
 e=c['entrada'];b=(D/e['archivo']).read_bytes();assert len(b)==e['bytes'] and h(b)==e['sha256'];assert len(b)<=8192
 if c['tipo']=='sha256':assert h(b)==c['esperado']['sha256']
positive=[c for c in cases if c['esperado']['resultado']=='ADMISIBLE_DECODIFICADOR'];assert len(positive)==6
for c in positive:
 x=json.loads((D/c['entrada']['archivo']).read_bytes());n,d,m=metrics(x);assert n<=512 and d<=32 and m<=32
assert metrics(json.loads((D/'activos/J07.bin').read_bytes()))[1]==32
assert metrics(json.loads((D/'activos/J08.bin').read_bytes()))[1]==33
assert metrics(json.loads((D/'activos/J09.bin').read_bytes()))[0]==512
assert metrics(json.loads((D/'activos/J10.bin').read_bytes()))[0]==513
assert metrics(json.loads((D/'activos/J11.bin').read_bytes()))[2]==32
assert metrics(json.loads((D/'activos/J12.bin').read_bytes()))[2]==33
assert (D/'activos/J04.bin').read_bytes()==b'{"a":0,"\\u0061":1}'
assert (D/'activos/J05.bin').read_bytes()==b'"\xff"'
# ES: Medir la estructura de cada canal literal, sin aplicar las guardas del receptor.
# EN: Measure each literal channel's shape without applying receptor guards.
I=B/'integracion-c02-c05-v0_1';q=json.loads((I/'SOLICITUDES.json').read_text());shapes=[]
for item in q:
 req=item['solicitud'];inputs={'solicitud':req}
 for k in ('estado_matematico','geometria'):
  inputs[k]=json.loads((I/req[k]['archivo']).read_bytes())
 if req.get('soporte'):inputs['soporte']=json.loads((I/req['soporte']['contenido']['archivo']).read_bytes())
 for k,x in inputs.items():
  n,d,m=metrics(x);assert n<=512 and d<=32 and m<=32,(item['id'],k,n,d,m)
  shapes.append({'caso':item['id'],'canal':k,'nodos':n,'profundidad':d,'miembros_max':m})
for p,v in json.loads((I/'MANIFIESTO.json').read_text()).items():assert h((I/p).read_bytes())==v
seats=load('DECISION_SEDES.json')['decisiones'];assert len(seats)==12
assert set(o for r in seats for o in r['obligaciones'])=={f'BIS-O{i:02}' for i in range(1,13)}
assert all(r['sede'] and r['decision'] and r['casos'] for r in seats)
if (D/'MANIFIESTO.json').exists():
 for p,v in load('MANIFIESTO.json').items():assert h((D/p).read_bytes())==v,p
report={'resultado':'CONFORME_DOCUMENTAL','decisiones':12,'obligaciones_cartografiadas':12,'casos_integrados_conservados':26,'complemento_json':{'preparados':14,'positivos_previstos':6,'negativos_previstos':8,'ejecutados_rust':0},'complemento_hash':{'preparados':5,'ejecutados_rust':0},'compatibilidad_estructural_26_casos':'conforme; no altera el exceso deliberado de bytes de I0205-19','limites':'No compilación ni ejecución del adaptador, sujeto, captor u observador. Contar JSON no mide RAM de proceso.','estructuras':shapes}
print(json.dumps(report,ensure_ascii=False,indent=2))
