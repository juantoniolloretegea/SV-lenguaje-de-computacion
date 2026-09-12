from pathlib import Path
import json,sys,hashlib,concurrent.futures
sys.path.insert(0,'tmp/presentacion-150')
from github_io import call
r=Path('tmp/diagnosticos-158');b=json.loads((r/'lenguaje-base.json').read_text());tree=json.loads((r/'lenguaje-tree.json').read_text())['tree']
items=[x for x in tree if x['type']=='blob' and x['path'].startswith(('tests/conformance/valid/','tests/conformance/invalid/')) and x['path'].endswith('.svp')]
assert len(items)==120

def one(x):
 p=r/'corpus'/x['path'];p.parent.mkdir(parents=True,exist_ok=True)
 if p.exists(): data=p.read_bytes()
 else:
  data=call('fetch',{'url':f"https://github.com/{b['repo']}/blob/{b['head']}/{x['path']}"})['content'].encode();p.write_bytes(data)
 assert hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest()==x['sha'],x['path']
 return {'path':x['path'],'git_blob':x['sha'],'sha256':hashlib.sha256(data).hexdigest(),'expected':'OK' if '/valid/' in x['path'] else 'ERR'}
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as ex:
 result=list(ex.map(one,items))
(r/'CORPUS.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
print('Corpus comprobado',len(result),flush=True)
