"""Comprobación documental independiente de archivos generados; sin ejecutar Rust."""
from pathlib import Path
import base64,collections,hashlib,json
D=Path(__file__).resolve().parent
read=lambda n:json.loads((D/n).read_text())
sha=lambda b:hashlib.sha256(b).hexdigest()
s=read('FUENTES_RECIBIDAS.json');assert len(s)==20
for p,x in s.items():
 b=base64.b64decode(x['base64'],validate=True)
 assert len(b)==x['bytes'] and sha(b)==x['sha256'],p
 assert hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()==x['blob'],p
 assert x['corte']=='fbeaa2d394c2dfc6df35e647b2a2b85abcdf5b3b'
def src(end):
 a=[x for p,x in s.items() if p.endswith(end)];assert len(a)==1
 return base64.b64decode(a[0]['base64'])
def obj(end):return json.loads(src(end))
a=obj('MATRIZ_COBERTURA_A_L_RESULTADO_S16.json');b=read('MATRIZ_COBERTURA_A_L_RECEPCION_S18.json')
assert [x['id'] for x in b['casos']]==list('ABCDEFGHIJKL')
for old,new in zip(a['casos'],b['casos']):
 copied=dict(new);added=copied.pop('recepcion_s19_resultado_s18');assert copied==old
 assert not added['ensayo_nuevo_en_s19'] and not added['cierre_universal']
 assert all(i in {'I'+str(n).zfill(2) for n in range(1,25)} for i in added['casos_s18'])
 if new['id'] in 'BEKL':assert not added['casos_s18']
for k,v in a.items():
 if k not in ['version','casos']:assert b[k]==v
captures=obj('EVIDENCIAS_S18.json')['archivos'];assert len(captures)==3382
by={x['ruta']:x for x in captures};assert len(by)==3382
for x in captures:
 raw=base64.b64decode(x['base64'],validate=True);assert sha(raw)==x['sha256'] and len(raw)==x['bytes']
normal=['debug/capturas-'+str(i) for i in range(1,4)]+['release/capturas-'+str(i) for i in range(1,4)]
first={p.removeprefix(normal[0]+'/'):x['base64'] for p,x in by.items() if p.startswith(normal[0]+'/')}
assert len(first)==449
for pref in normal:assert {p.removeprefix(pref+'/'):x['base64'] for p,x in by.items() if p.startswith(pref+'/')}==first
inv=read('INVENTARIO_CAUSAS_Y_ESTADOS.json');assert len(inv['filas'])==20
r=collections.Counter(x['naturaleza'] for x in inv['filas']);assert r=={'estado_conforme':3,'negativa':1,'fallo_o_rechazo':16}
observed={p for p in by if p.startswith('debug/capturas-1/') and p.endswith(('.diagnostico','.admision','.escritura'))}
covered=[];keys=[]
labels={x['id']:(x['es'],x['en']) for x in obj('DIAGNOSTICOS.json')['identificadores']}
for row in inv['filas']:
 assert row['codigo_canonico_sv'] is None
 assert (row['rotulo_es_emitido'],row['rotulo_en_emitido'])==labels[row['codigo_local_emitido']]
 fields=[row[k] for k in ['etapa_emitida','codigo_local_emitido','detalle_textual_emitido_no_parseado','rotulo_es_emitido','rotulo_en_emitido']]
 raw=('\t'.join(fields)+'\n').encode();keys.append(tuple(fields));cases=set()
 for x in row['evidencias']:
  assert x==by[x['ruta']];assert base64.b64decode(x['base64'])==raw
  covered.append(x['ruta']);cases.add(x['ruta'].split('/')[-1].split('.')[0])
 assert row['casos']==sorted(cases)
assert set(covered)==observed and len(covered)==len(observed) and len(set(keys))==20
assert inv['antecedente_s15']['contenido_original']==obj('CAUSAS_OBSERVADAS.json')
assert read('FRONTERAS_RECIBIDAS.json')['antecedente_s16_intacto']==obj('FRONTERAS_DE_FALLO.json')
expected=src('ESPERADO.tsv').decode().splitlines();assert len(expected)==24
for line in expected:
 f=line.split('\t');raw=base64.b64decode(by['debug/capturas-1/'+f[0]+'.diagnostico']['base64']).decode().strip().split('\t')
 assert raw[:2]==f[1:3],(f,raw)
assert read('RECEPCION.json')['resultado_s18_intacto']==obj('RESULTADO.json')
assert read('RECEPCION.json')['ejecuciones_sv_nuevas']==0
manifest=read('MANIFIESTO.json')
for p,x in manifest['archivos'].items():
 data=(D/p).read_bytes();assert len(data)==x['bytes'] and sha(data)==x['sha256'],p
assert set(manifest['archivos'])=={str(p.relative_to(D)) for p in D.rglob('*') if p.is_file()}-{'MANIFIESTO.json','VERIFICACION.json'}
result={'version':'S19-VERIFICACION-DOCUMENTAL/1','resultado':'CONFORME','fuentes_integras':20,'filas_historicas_intactas':12,'capturas_integras':3382,'capturas_identicas_por_ejecucion_normal':449,'ejecuciones_normales_recibidas':6,'casos_finales_cotejados_con_esperado_previo':24,'tuplas_registro':20,'estados_conformes':3,'negativas':1,'variantes_fallo_o_rechazo_por_etapa':16,'antecedentes_s15_s16_intactos':True,'ensayos_funcionales_nuevos':0}
(D/'VERIFICACION.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps(result,ensure_ascii=False))
