from pathlib import Path
import sys,json,shutil
R=Path(__file__).resolve().parent;sys.path.insert(0,str(R.parent/'s6-trazabilidad-total'));from github_io import fetch
prev=json.loads((R/'PUBLICACION_APERTURA.json').read_text())
for k,s in prev.items():
 assert s.get('verified');b=s['base'];api='https://api.github.com/repos/'+b['repo'];h=fetch(api+'/git/ref/heads/'+b['branch'])['object']['sha'];assert h==s['commit'];t=fetch(api+'/git/trees/'+s['tree']+'?recursive=1');assert not t.get('truncated')
 for tail in ['base','arbol']:shutil.copyfile(R/f'{k}-{tail}.json',R/f'{k}-{tail}-antes-apertura.json')
 (R/f'{k}-base.json').write_text(json.dumps(dict(repo=b['repo'],branch=b['branch'],head=h,tree=s['tree'])))
 (R/f'{k}-arbol.json').write_text(json.dumps({x['path']:x['sha'] for x in t['tree'] if x['type']=='blob'}))
 print('Corte de apertura verificado para cierre:',k,h)
