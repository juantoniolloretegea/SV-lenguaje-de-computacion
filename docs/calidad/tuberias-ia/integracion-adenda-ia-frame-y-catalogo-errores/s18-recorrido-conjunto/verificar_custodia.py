#!/usr/bin/env python3
"""Cotejo documental posterior: no sustituye la ejecución funcional Rust."""
from pathlib import Path
import json,hashlib,base64
D=Path(__file__).resolve().parent;sha=lambda b:hashlib.sha256(b).hexdigest()
f=json.loads((D/'FIJACION_PREVIA.json').read_text());assert all(sha((D/n).read_bytes())==h for n,h in f['archivos'].items())
caps=json.loads((D/'EVIDENCIAS_S18.json').read_text())['archivos'];labels={x['id']:(x['es'],x['en']) for x in json.loads((D/'contrato-s17/DIAGNOSTICOS.json').read_text())['identificadores']};counts={};normal={}
for x in caps:
 b=base64.b64decode(x['base64'],validate=True);assert len(b)==x['bytes'] and sha(b)==x['sha256'];path=x['ruta'];k='/'.join(path.split('/')[:-1]);counts[k]=counts.get(k,0)+1
 if path.startswith(('debug/capturas-','release/capturas-')):normal.setdefault(k,{})[path.split('/')[-1]]=x['sha256']
 if path.endswith('.diagnostico') or path.endswith('.admision'):
  a=b.decode().rstrip('\n').split('\t');assert len(a)==5;assert tuple(a[3:])==labels[a[1]],path
logs=json.loads((D/'COMANDOS.json').read_text())
for l in logs:
 for name in ['stdout','stderr']:assert sha(base64.b64decode(l[name+'_base64']))==l[name+'_sha256']
res=json.loads((D/'RESULTADO.json').read_text());assert res['conforme'] and len(logs)==29 and len(normal)==6 and all(len(x)==449 for x in normal.values());first=next(iter(normal.values()));assert all(x==first for x in normal.values())
for mode in ['debug','release']:
 for n in range(1,4):
  l=next(x for x in logs if x['paso']==f'{mode} ejecución {n}');assert l['exit_code']==0 and base64.b64decode(l['stdout_base64'])==(D/'codigo/ESPERADO.tsv').read_bytes()
for m,c in res['sensibilidades'].items():
 l=next(x for x in logs if x['paso']==m+' detección');assert l['exit_code']==1 and 'FALLO '+c in l['stderr']
for k in ['fabricar_referencia rechazado','fabricar_entrega rechazado']:
 l=next(x for x in logs if x['paso']==k);assert l['exit_code']==1 and 'error[E0451]' in l['stderr']
man=json.loads((D/'MANIFIESTO_POSTERIOR.json').read_text())['archivos'];assert set(man)=={str(p.relative_to(D)) for p in D.rglob('*') if p.is_file() and p.name!='MANIFIESTO_POSTERIOR.json'}
for p,h in man.items():b=(D/p).read_bytes();assert len(b)==h['bytes'] and sha(b)==h['sha256'],p
print(json.dumps(dict(version='S18-COTEJO-POSTERIOR/1',archivos_fijados_intactos=len(f['archivos']),capturas_integras=len(caps),capturas_por_ejecucion=counts,diagnosticos_es_en='Coinciden con identificadores y rótulos S17',comandos_integridad_verificada=len(logs),resultado='COTEJO_CONFORME'),ensure_ascii=False,indent=2))
