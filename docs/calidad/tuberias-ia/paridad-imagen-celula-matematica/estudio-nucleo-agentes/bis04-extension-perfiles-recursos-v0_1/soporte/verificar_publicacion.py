from pathlib import Path
import json,subprocess,hashlib,sys
sys.path.insert(0,'s6-trazabilidad-total');import github_io as g
R=Path('manifiesto-sv/checkout')
a=json.loads(Path('bis-extension/PUBLICACION_PUBLICO.json').read_text());b=json.loads(Path('bis-extension/PUBLICACION_LAB.json').read_text())
assert a.get('verified') and b.get('verified')
for repo,branch,state in [('SV-lenguaje-de-computacion','main',a),('SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',b)]:
 head=g.fetch('https://api.github.com/repos/juantoniolloretegea/'+repo+'/git/ref/heads/'+branch)['object']['sha'];assert head==state['commit'],(repo,head)
count=0
for p,h in a['files'].items():
 raw=(R/p).read_bytes();assert hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()==h
 if 'REGISTRO_EVOLUCION_TECNICA_PROYECTO.' in p:continue
 q=('laboratorio/tareas-watson/sucesos-sv/'+Path(p).name) if '/Inventario-sv/sucesos/' in p else p.replace('docs/calidad/','laboratorio/tareas-watson/',1)
 assert b['files'][q]==h,(p,q);count+=1
assert count==len(b['files'])
print('Espejo íntegro:',count,'archivos. Público:',a['commit'],'Laboratorio:',b['commit'])
subprocess.run(['git','fetch','origin','main'],cwd=R,check=True)
assert subprocess.check_output(['git','rev-parse','origin/main'],cwd=R,text=True).strip()==a['commit']
subprocess.run(['git','reset','--mixed',a['commit']],cwd=R,check=True)
assert not subprocess.check_output(['git','status','--porcelain'],cwd=R,text=True)
print('Checkout sincronizado; árbol de trabajo limpio.')
