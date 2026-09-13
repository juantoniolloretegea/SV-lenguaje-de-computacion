from pathlib import Path
import sys,json,hashlib,base64
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'s6-trazabilidad-total'))
import github_io as g

def publicar(repo,branch,mapping,message,checkpoint,expected_base=None):
 p=Path(checkpoint);s=json.loads(p.read_text()) if p.exists() else {};api='https://api.github.com/repos/'+repo
 def save():p.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n')
 if not s:
  h=g.fetch(api+'/git/ref/heads/'+branch)['object']['sha']
  if expected_base:assert h==expected_base,(h,expected_base)
  t=g.fetch(api+'/git/trees/'+h+'?recursive=1');assert not t.get('truncated');s.update(base=h,tree=t['sha'],original={x['path']:(x['sha'],x['mode']) for x in t['tree'] if x['type']=='blob'},files={});save()
 if s.get('verified'):return s
 if not s.get('published'):assert g.fetch(api+'/git/ref/heads/'+branch)['object']['sha']==s['base']
 for i,(path,local) in enumerate(mapping.items()):
  b=Path(local).read_bytes();sha=hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
  if s['files'].get(path)==sha:continue
  if s['original'].get(path,[None])[0]!=sha:
   res=g.call('create_blob',dict(repository_full_name=repo,content=base64.b64encode(b).decode(),encoding='base64'));assert res['sha']==sha
  s['files'][path]=sha;save()
  if i%10==0:print('Copias preparadas',i+1,flush=True)
 if not s.get('published'):
  t=g.call('create_tree',dict(repository_full_name=repo,base_tree_sha=s['tree'],tree_elements=[dict(path=p,sha=h,mode=s['original'].get(p,[None,'100644'])[1],type='blob') for p,h in s['files'].items()]));s['new_tree']=t['sha'];save()
  c=g.call('create_commit',dict(repository_full_name=repo,message=message,parent_sha=s['base'],tree_sha=t['sha']));s['commit']=c['sha'];save()
  assert g.fetch(api+'/git/ref/heads/'+branch)['object']['sha']==s['base']
  g.call('update_ref',dict(repository_full_name=repo,branch_name=branch,sha=s['commit'],force=False));s['published']=True;save()
 t=g.fetch(api+'/git/trees/'+s['commit']+'?recursive=1');assert not t.get('truncated');expected={p:h[0] for p,h in s['original'].items()};expected.update(s['files']);assert {x['path']:x['sha'] for x in t['tree'] if x['type']=='blob'}==expected
 s['verified']=True;save();print('PUBLICACION_VERIFICADA',repo,s['commit'],flush=True);return s
