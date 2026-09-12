from pathlib import Path
import sys,json,concurrent.futures
R=Path(__file__).resolve().parent;sys.path.insert(0,str(R.parent/'s6-trazabilidad-total'));from github_io import fetch
prev=json.loads((R.parent/'s14-bases/PUBLICACION.json').read_text())
def get(k):
 v=prev[k];b=v['base'];api='https://api.github.com/repos/'+b['repo'];h=fetch(api+'/git/ref/heads/'+b['branch'])['object']['sha'];assert h==v['commit'],(k,h)
 t=fetch(api+'/git/commits/'+h)['tree']['sha'];z=fetch(api+'/git/trees/'+t+'?recursive=1');assert not z.get('truncated')
 (R/(k+'-base.json')).write_text(json.dumps(dict(repo=b['repo'],branch=b['branch'],head=h,tree=t)))
 (R/(k+'-arbol.json')).write_text(json.dumps({x['path']:x['sha'] for x in z['tree'] if x['type']=='blob'}))
 return k,h
with concurrent.futures.ThreadPoolExecutor(2) as ex:
 for r in ex.map(get,prev):print(r,flush=True)
