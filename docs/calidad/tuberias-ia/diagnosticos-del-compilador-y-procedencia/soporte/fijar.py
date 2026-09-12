from pathlib import Path
import sys,json,hashlib
sys.path.insert(0,'tmp/presentacion-150')
from github_io import fetch,call
root=Path('tmp/diagnosticos-158')
for key,repo,branch in [('lenguaje','SV-lenguaje-de-computacion','main'),('laboratorio','SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente')]:
 full='juantoniolloretegea/'+repo
 head=fetch(f'https://api.github.com/repos/{full}/git/ref/heads/{branch}')['object']['sha']
 c=fetch(f'https://api.github.com/repos/{full}/git/commits/{head}')
 tree=fetch(f'https://api.github.com/repos/{full}/git/trees/{head}?recursive=1');assert not tree.get('truncated')
 (root/f'{key}-base.json').write_text(json.dumps(dict(repo=full,branch=branch,head=head,tree=c['tree']['sha']),indent=2)+'\n')
 (root/f'{key}-tree.json').write_text(json.dumps(tree,indent=2)+'\n')
 print(key,head,flush=True)
 if key=='lenguaje':
  for p in ['AGENTS.md','docs/calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md','docs/calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md','docs/dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md','docs/calidad/tuberias-ia/WORKFLOW_ACOTADO_SUBORDINACION_IA_ES_V2_2026_09_11.md']:
   old=Path('tmp/algebra-151/lenguaje')/p;b=old.read_bytes();expected=next(x['sha'] for x in tree['tree'] if x['path']==p)
   assert hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()==expected,p
  print('rectores: mismos bytes ya leídos',flush=True)
