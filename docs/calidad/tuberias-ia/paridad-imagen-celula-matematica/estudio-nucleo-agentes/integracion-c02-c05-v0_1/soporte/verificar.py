"""ES: Verificación documental independiente; no ejecución SVP/Rust.
EN: Independent documentary verification; not SVP/Rust execution.
"""
from pathlib import Path
import json, hashlib, copy
D=Path(__file__).resolve().parents[1]
def load(p): return json.loads((D/p).read_text())
def digest(b): return hashlib.sha256(b).hexdigest()
def encoded(x): return (json.dumps(x,ensure_ascii=False,indent=2)+'\n').encode()
def walk(x):
 if isinstance(x,dict):
  if {'archivo','sha256','bytes'}<=x.keys():
   p=(D/x['archivo']).resolve(); assert p.is_relative_to(D)
   b=p.read_bytes(); assert digest(b)==x['sha256']; assert len(b)==x['bytes']
  for v in x.values():walk(v)
 elif isinstance(x,list):
  for v in x:walk(v)
files=sorted(D.rglob('*.json'))
for f in files: walk(json.loads(f.read_text()))
bank=load('BANCO_PREVIO.json'); Q=load('SOLICITUDES.json'); C=load('CONTEXTOS_CONFIABLES.json'); O=load('ORACULOS.json'); P=load('PLANES_DE_INYECCION.json'); reg=load('REGISTRO_CONFIABLE.json')
ids=[f'I0205-{i:02}' for i in range(1,27)]
for group in (bank['casos'],Q,C,O,P):assert [r['id'] for r in group]==ids
assert bank['ejecutados']==0
assert all(x['observado'] is None and x['efecto_observado'] is None and x['ejecucion']=='PENDIENTE' for x in O)
assert all(x['estatuto']=='ESTIMULO_FUTURO_NO_OBSERVACION' for x in P)
positives=[x['id'] for x in O if x['recibo_favorable']]
assert positives==[f'I0205-{i:02}' for i in (1,2,3,4,11,17,20)]
state=load('activos/estado-16.json'); symbols=state['vector']; n=len(symbols)
assert n==state['n']==state['b']**2==16 and 'U' in symbols
conv=load('activos/convenio.json'); transforms=load('activos/transformaciones.json')['transformaciones']
# ES: Fórmulas contra el estado independiente; sin llamar al productor futuro.
# EN: Formula checks against the independent state, without a future producer.
for name in ('geometria-identidad.json','geometria-superior.json','geometria-8192.json'):
 g=load('activos/'+name); t=next(t for t in transforms if t['id']==g['transformacion']); m=t['matriz'];tr=t['traslacion']
 assert g['formato']=='BIS-C04-GEOMETRIA/0.1' and g['n']==n
 assert g['leyenda']==conv['leyenda'] and g['convenio']['sha256']==digest((D/'activos/convenio.json').read_bytes())
 assert g['codificacion']=='inyectiva' and g['pretension_equivalencia'] is True and g['cerrada'] is True
 assert len(g['vertices'])==n
 for i,v in enumerate(g['vertices'],1):
  r=conv['radios'][symbols[i-1]]
  assert v=={'posicion':i,'parametro_sintetico':f'P{i}','simbolo':symbols[i-1],'radio':r,'angulo_vueltas':[i-1,n],'x_coef_cos_sin_constante':[m[0][0]*r,m[0][1]*r,tr[0]],'y_coef_cos_sin_constante':[m[1][0]*r,m[1][1]*r,tr[1]]}
 assert g['aristas']==[[i,i+1] for i in range(1,n)]+[[n,1]]
# ES: El negativo coherente debe discrepar realmente del vector custodiado.
# EN: The coherent negative must actually differ from the trusted vector.
bad=load('activos/geometria-permutada.json'); assert any(v['simbolo']!=symbols[i] for i,v in enumerate(bad['vertices']))
assert sorted(v['simbolo'] for v in bad['vertices'])==sorted(symbols)
assert load('activos/estado-alterado.json')['vector']!=symbols
assert Q[8]['solicitud']['geometria']['sha256']==digest((D/'activos/geometria-permutada.json').read_bytes())
assert O[8]['primera_guarda_esperada']=='M02'
# ES: Comprobar deltas literales sin ejecutar las guardas candidatas.
# EN: Check literal deltas without running the candidate guards.
def normalized(q):
 q=copy.deepcopy(q);q['entrega']['invocacion']='INV';return q
def changed(a,b,p=''):
 if isinstance(a,dict) and isinstance(b,dict):
  out=[]
  for k in sorted(a.keys()|b.keys()):
   if k not in a or k not in b:out.append(p+'/'+k)
   else:out+=changed(a[k],b[k],p+'/'+k)
  return out
 return [] if a==b else [p]
base=normalized(Q[0]['solicitud']);deltas={x['id']:changed(base,normalized(x['solicitud'])) for x in Q}
assert deltas['I0205-07']==['/vinculo/estado_ir','/vinculo/instancia_celular','/vinculo/nodo']
assert deltas['I0205-08']==['/vinculo/revision']
assert deltas['I0205-17']==['/nota_documental']
assert deltas['I0205-18']==['/entrega/operacion']
assert deltas['I0205-21']==['/perfil_fuente']
assert deltas['I0205-23']==['/vinculo/revision']
for i in (12,13,14,15,16,25,26):assert not deltas[f'I0205-{i:02}']
assert P[11]['producir_captura'] is False and O[11]['despachos_locales_esperados'] is None
assert P[24]['fallo_despues_de_despacho'] is True and O[24]['despachos_locales_esperados'] is None
assert P[15]['buffer_capturado']['sha256']!=C[15]['geometria']['sha256']
assert Q[3]['solicitud']['fuente']==Q[4]['solicitud']['fuente']
assert C[3]['dimensiones_ir_esperadas']==C[4]['dimensiones_ir_esperadas']==[16,49]
assert load('activos/soporte-v1.json')['tamanos']==[16,25] and load('activos/soporte-v2.json')['tamanos']==[16,25,49]
assert Q[4]['solicitud']['soporte']['referencia']['version']=='1'
assert bank['secuencia_mismo_proceso']==['I0205-05','I0205-04','I0205-05']
for i in (0,1,2):assert C[i]['estado_matematico']==C[0]['estado_matematico'] and C[i]['geometria']==C[0]['geometria']
assert len({(C[i]['vinculo']['instancia_celular'],C[i]['vinculo']['revision']) for i in (0,1,2)})==3
budget=load('PRESUPUESTO.json')['bytes_utf8_inclusivos'];sizes=[]
for row in Q:
 q=row['solicitud']; parts={'solicitud_serializada':len(encoded(q)),'fuente_svp':q['fuente']['bytes'],'estado_matematico':q['estado_matematico']['bytes'],'soporte':q['soporte']['contenido']['bytes'] if q.get('soporte') else 0,'geometria':q['geometria']['bytes']}
 total=sum(parts.values());over=[k for k,v in parts.items() if v>budget[k]]
 assert total<=budget['suma_entrada']
 assert over==(['geometria'] if row['id']=='I0205-19' else [])
 sizes.append({'id':row['id'],'canales_bytes':parts,'total_bytes':total,'excesos_previstos':over})
assert (D/'activos/geometria-8192.json').stat().st_size==8192
assert (D/'activos/geometria-8193.json').stat().st_size==8193
assert load('activos/geometria-8192.json')==load('activos/geometria-identidad.json')
for o,c in zip(O,C):
 if o['recibo_favorable']:
  receipt=o['recibo_esperado'];length=len(encoded(receipt))
  assert receipt['vinculo']==c['vinculo'] and receipt['contexto_entrega']==c['entrega']
  assert receipt['geometria_sha256']==c['geometria']['sha256']
  assert length<=budget['recibo'] and length+c['geometria']['bytes']<=budget['suma_salida']
 else:assert o['recibo_esperado'] is None
for p,entry in load('FUENTES.json')['activos_copiados'].items():assert digest((D/p).read_bytes())==entry['sha256']
if (D/'MANIFIESTO.json').exists():
 for p,h in load('MANIFIESTO.json').items():assert digest((D/p).read_bytes())==h,p
report={'version':'BIS-I0205-VERIFICACION-DOCUMENTAL/0.1','resultado':'CONFORME_EN_ALCANCE_DOCUMENTAL','casos':26,'positivos_previstos':7,'negativos_previstos':19,'ejecutados_rust':0,'sensibilidades_ejecutadas':0,'comprobaciones':['referencias, longitudes y SHA256','26 IDs únicos concordantes en cinco piezas separadas','observados vacíos y planes marcados como estímulos futuros','fórmulas exactas independientes de tres artefactos positivos','negativo coherente distinto del estado','deltas de identidad/revisión/autoridad/perfil','misma fuente bajo v1/v2/v1','cuotas literales, límite 8192 y exceso 8193','conservación de activos copiados'],'limites':['No compilación de nuevos SVP ni ejecución Rust de esta integración','No productor, receptor ni captor implementados','No acreditación de alcance real de guardas; pendiente ejecución','RAM, duración, salida y ES/EN integrados pendientes'],'deltas_solicitudes_respecto_A_r1':deltas,'mediciones_documentales':sizes}
print(json.dumps(report,ensure_ascii=False,indent=2))
