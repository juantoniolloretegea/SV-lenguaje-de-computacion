from pathlib import Path
import json,base64,hashlib,ast
D=Path(__file__).resolve().parent
sha=lambda b:hashlib.sha256(b).hexdigest()
blob=lambda b:hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
sources=json.loads((D/'FUENTES_RECIBIDAS.json').read_text());interfaces=json.loads((D/'INTERFACES_ANTERIORES.json').read_text())
assert len(sources)==15 and len(interfaces)==4
raw={}
for p,x in sources.items():
 b=base64.b64decode(x['base64'],validate=True);assert len(b)==x['bytes'] and sha(b)==x['sha256'] and blob(b)==x['blob'];raw[p]=b
caps=json.loads(next(b for p,b in raw.items() if p.endswith('/FUENTES_S2.json')))
for p,x in interfaces.items():
 original=next(a for a in caps['archivos'] if a['ruta']==p);assert x==original
 b=base64.b64decode(x['base64'],validate=True);assert sha(b)==x['sha256'];raw[p]=b
passages=json.loads((D/'PASAJES_INTERFACES.json').read_text())['pasajes'];assert len(passages)==11
for p in passages:
 b=raw[p['fuente']];assert sha(b)==p['sha256_fuente'];assert b[p['inicio_byte']:p['fin_byte']]==p['texto'].encode();assert b[:p['inicio_byte']].count(b'\n')+1==p['linea']
bank=json.loads((D/'BANCO_OBLIGACIONES.json').read_text());cases=bank['casos'];assert [c['id'] for c in cases]==[f'I{i:02}' for i in range(1,25)]
assert bank['observaciones_normales_previstas']==len(cases)*len(bank['modos'])*bank['repeticiones_por_modo']==144
labels=json.loads((D/'DIAGNOSTICOS.json').read_text())['identificadores'];assert len({x['id'] for x in labels})==len(labels)
assert {c['causa_esperada'] for c in cases}<={x['id'] for x in labels}
for c in cases:assert c['predicados'] and c['preparacion'] and c['etapa_esperada']
assert {m['testigo'] for m in bank['sensibilidad']}=={'I04','I14','I09','I12'}
assert len(bank['frontera_tipos'])==3
for name,x in bank['fuentes_esperados'].items():
 b=(D/'esperados'/name).read_bytes();assert len(b)==x['bytes'] and sha(b)==x['sha256']
 if name not in ['contexto.txt','contexto-instruccion.txt']:
  assert b==next(b0 for p,b0 in raw.items() if p.endswith('/especimenes/'+name))
pos=(D/'esperados/positivo.cuerpo').read_bytes();neg=(D/'esperados/negativo.cuerpo').read_bytes();req=(D/'esperados/solicitud.bin').read_bytes();base=(D/'esperados/base-original.bin').read_bytes()
for b in [pos,neg,req]:json.loads(b)
assert pos!=neg and b'8.40' in pos and pos.replace(b'8.40',b'8.41',1)!=pos
# Comprobación aritmética documental del transporte R, no ejecución del receptor.
wire_i23=5+1+16+(4+8192)+(4+len(pos))+(1+4+len(req))+(1+4+len(base));assert wire_i23<=16384
for p in D.rglob('*.py'):ast.parse(p.read_text())
manifest=D/'MANIFIESTO.json'
if manifest.exists():
 files=json.loads(manifest.read_text())['archivos'];actual={str(p.relative_to(D)) for p in D.rglob('*') if p.is_file() and p.name!='MANIFIESTO.json'};assert actual==set(files)
 for p,x in files.items():b=(D/p).read_bytes();assert len(b)==x['bytes'] and sha(b)==x['sha256'],p
report=dict(version='S17-COTEJO/1',fuentes_integras=len(sources),interfaces_identicas_capsula=len(interfaces),pasajes_exactos=len(passages),casos_fijados=len(cases),observaciones_funcionales_realizadas=0,observaciones_normales_previstas=144,bytes_transporte_previstos_I23=wire_i23,resultado='COTEJO_DOCUMENTAL_CONFORME; no es conformidad ejecutable del recorrido.')
print(json.dumps(report,ensure_ascii=False,indent=2))
