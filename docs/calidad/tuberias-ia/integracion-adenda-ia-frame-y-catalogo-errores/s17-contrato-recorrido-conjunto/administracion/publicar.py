from pathlib import Path
import sys,json,base64,hashlib
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'s6-trazabilidad-total'))
from github_io import fetch,call
R=Path(__file__).resolve().parent;CP=R/'PUBLICACION.json'
deletes=json.loads((R/'ELIMINACIONES.json').read_text())
maps=json.loads((R/'MAPA.json').read_text());state=json.loads(CP.read_text()) if CP.exists() else {}
def save():CP.write_text(json.dumps(state,ensure_ascii=False,indent=2)+'\n')
def blob(b):return hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
for k,m in maps.items():
 b=json.loads((R/(k+'-base.json')).read_text());s=state.setdefault(k,dict(base=b,files={}));assert s['base']==b
 api='https://api.github.com/repos/'+b['repo']
 if not s.get('published'):assert fetch(api+'/git/ref/heads/'+b['branch'])['object']['sha']==b['head']
for k,m in maps.items():
 s=state[k];b=s['base'];api='https://api.github.com/repos/'+b['repo']
 if s.get('verified'):continue
 old=json.loads((R/(k+'-arbol.json')).read_text())
 for path in deletes.get(k,[]):
  assert path in old;s['files'][path]=None;save()
 for path,local in m.items():
  data=Path(local).read_bytes();h=blob(data)
  if old.get(path)==h:continue
  if s['files'].get(path)==h:continue
  got=call('create_blob',dict(repository_full_name=b['repo'],content=base64.b64encode(data).decode(),encoding='base64'))
  assert got['sha']==h;s['files'][path]=h;save()
 if not s.get('commit'):
  t=call('create_tree',dict(repository_full_name=b['repo'],base_tree_sha=b['tree'],tree_elements=[dict(path=p,mode='100644',type='blob',sha=h) for p,h in s['files'].items()]))
  s['tree']=t['sha'];save()
  c=call('create_commit',dict(repository_full_name=b['repo'],message='calidad: fijar contrato del recorrido documental conjunto; S17 / RETP-188',parent_sha=b['head'],tree_sha=s['tree']))
  s['commit']=c['sha'];save()
 if not s.get('published'):
  assert fetch(api+'/git/ref/heads/'+b['branch'])['object']['sha']==b['head']
  call('update_ref',dict(repository_full_name=b['repo'],branch_name=b['branch'],sha=s['commit'],force=False));s['published']=True;save()
 assert fetch(api+'/git/ref/heads/'+b['branch'])['object']['sha']==s['commit']
 t=fetch(api+'/git/trees/'+s['tree']+'?recursive=1');assert not t.get('truncated');after={x['path']:x['sha'] for x in t['tree'] if x['type']=='blob'}
 for p,local in m.items():assert after[p]==blob(Path(local).read_bytes())
 for p in deletes.get(k,[]):assert p not in after
 for p,h in old.items():
  if p not in s['files']:assert after.get(p)==h,p
 assert set(after)-set(old)<=set(s['files']);s['verified']=True;save()
 print(k,s['commit'],'VERIFICADO',flush=True)
