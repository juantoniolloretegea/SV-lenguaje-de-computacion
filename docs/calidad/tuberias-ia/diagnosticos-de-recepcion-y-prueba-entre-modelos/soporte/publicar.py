from pathlib import Path
import json,hashlib,sys,base64
sys.path.insert(0,'tmp/presentacion-150')
from github_io import call,fetch
root=Path.cwd();tmp=root/'tmp/continuacion-157';out=root/'entregas/diagnosticos-de-recepcion-y-prueba-entre-modelos'
checkpoint=tmp/'PUBLICACION.json';state=json.loads(checkpoint.read_text()) if checkpoint.exists() else {}
def save():checkpoint.write_text(json.dumps(state,ensure_ascii=False,indent=2)+'\n')
def sha(b):return hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
# Check both bases before the first write. Each reference is checked again before publication.
for key in ['laboratorio','lenguaje']:
 b=json.loads((tmp/f'{key}-base.json').read_text())
 if not state.get(key,{}).get('published'):
  assert fetch('https://api.github.com/repos/'+b['repo']+'/git/ref/heads/'+b['branch'])['object']['sha']==b['head'],key+' cambiado'
for key in ['laboratorio','lenguaje']:
 b=json.loads((tmp/f'{key}-base.json').read_text());repo=b['repo'];branch=b['branch']
 dest='laboratorio/tareas-watson/tuberias-ia' if key=='laboratorio' else 'docs/calidad/tuberias-ia'
 st=state.setdefault(key,{'base':b,'files':{},'folder':dest+'/'+out.name});save()
 if st.get('verified'):continue
 mapping={dest+'/'+out.name+'/'+str(p.relative_to(out)):p for p in out.rglob('*') if p.is_file()}
 mapping[dest+'/inicio.md']=tmp/f'{key}-inicio-nuevo.md'
 if key=='lenguaje':
  mapping['docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md']=tmp/'retp-nuevo.md'
  mapping['docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv']=tmp/'retp-nuevo.csv'
  mapping['docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md']=tmp/'deuda-nueva.md'
 baseblobs={x['path']:x['sha'] for x in json.loads((tmp/f'{key}-tree.json').read_text())['tree'] if x['type']=='blob'}
 for index,(path,p) in enumerate(mapping.items()):
  data=p.read_bytes();expected=sha(data)
  if st['files'].get(path)==expected:continue
  if baseblobs.get(path)==expected:
   st['files'][path]=expected;save();continue
  r=call('create_blob',{'repository_full_name':repo,'content':base64.b64encode(data).decode('ascii'),'encoding':'base64'})
  assert r['sha']==expected
  st['files'][path]=expected;save()
  if index%10==0:print(key,'archivos preparados',index+1,flush=True)
 if not st.get('published'):
  tree=call('create_tree',{'repository_full_name':repo,'base_tree_sha':b['tree'],'tree_elements':[{'path':p,'mode':'100644','type':'blob','sha':h} for p,h in st['files'].items()]});st['tree']=tree['sha'];save()
  commit=call('create_commit',{'repository_full_name':repo,'message':'test: conservar causas de recepción y ES/EN; reproducir RETP-157','parent_sha':b['head'],'tree_sha':tree['sha']});st['commit']=commit['sha'];save()
  assert fetch('https://api.github.com/repos/'+repo+'/git/ref/heads/'+branch)['object']['sha']==b['head']
  call('update_ref',{'repository_full_name':repo,'branch_name':branch,'sha':st['commit'],'force':False})
  assert fetch('https://api.github.com/repos/'+repo+'/git/ref/heads/'+branch)['object']['sha']==st['commit'];st['published']=True;save()
 tree=fetch('https://api.github.com/repos/'+repo+'/git/trees/'+st['tree']+'?recursive=1');actual={x['path']:x['sha'] for x in tree['tree'] if x['type']=='blob'}
 for path,h in st['files'].items():assert actual[path]==h
 st['verified']=True;save();print(key,'VERIFICADO',st['commit'],len(st['files']),flush=True)
for p in out.rglob('*'):
 if p.is_file():
  rel=str(p.relative_to(out));assert state['lenguaje']['files'][state['lenguaje']['folder']+'/'+rel]==state['laboratorio']['files'][state['laboratorio']['folder']+'/'+rel]
state['copias_identicas']=True;save();print('COPIAS IDENTICAS',flush=True)
