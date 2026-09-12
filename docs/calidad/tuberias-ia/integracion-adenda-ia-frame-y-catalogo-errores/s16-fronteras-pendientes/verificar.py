from pathlib import Path
import hashlib,base64,json
D=Path(__file__).resolve().parent;sha=lambda b:hashlib.sha256(b).hexdigest();blob=lambda b:hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
f=json.loads((D/'FUENTES_RECIBIDAS.json').read_text());banco={}
for p,x in f.items():
 b=base64.b64decode(x['base64'],validate=True);assert len(b)==x['bytes'] and sha(b)==x['sha256'] and blob(b)==x['blob'];banco[p]=b
pasajes=json.loads((D/'PASAJES_COTEJADOS.json').read_text());ids=set()
for p in pasajes:
 assert p['id'] not in ids;ids.add(p['id']);b=''.join(banco[p['ruta']].decode().splitlines(keepends=True)[p['linea_inicio']-1:p['linea_fin']]).encode();assert b==p['texto'].encode() and sha(b)==p['sha256']
m=json.loads((D/'MATRIZ_FRONTERAS.json').read_text());assert [x['id'] for x in m['filas']]==['B','E','K','L']
a=json.loads((D/'MATRIZ_COBERTURA_A_L_RESULTADO_S16.json').read_text());p=next(p for p in banco if p.endswith('MATRIZ_COBERTURA_A_L_RESULTADO_S15.json'));old=json.loads(banco[p]);assert [{k:v for k,v in c.items() if k!='resultado_s16'} for c in a['casos']]==old['casos']
for r in m['filas']:
 assert all(x in ids for x in r['respaldo']);assert r['criterio_original']==next(x for x in old['casos'] if x['id']==r['id'])['original_147'];assert not r['nueva_prueba_funcional']
for r in json.loads((D/'FRONTERAS_DE_FALLO.json').read_text())['filas']:assert all(x in ids for x in r['respaldo'])
manifest=D/'MANIFIESTO.json'
if manifest.exists():
 for n,x in json.loads(manifest.read_text())['archivos'].items():assert len((D/n).read_bytes())==x['bytes'] and sha((D/n).read_bytes())==x['sha256']
print(json.dumps(dict(version='S16-COTEJO/1',fuentes=len(f),pasajes=len(pasajes),fronteras=len(m['filas']),criterios_anteriores_intactos=True,integridad_documental=True,ensayos_funcionales_nuevos=0,limite='Comprueba custodia y correspondencia documental, no suficiencia semántica ni garantía material.'),ensure_ascii=False,indent=2))
