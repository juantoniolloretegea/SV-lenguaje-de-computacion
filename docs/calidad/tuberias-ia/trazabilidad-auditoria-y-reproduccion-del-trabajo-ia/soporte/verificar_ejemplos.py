"""Comprueba los tres fragmentos del documento: python verificar_ejemplos.py /ruta/rustc."""
import sys,re,json,subprocess,time,hashlib,tempfile
from pathlib import Path
root=Path(__file__).resolve().parents[1]
md=root/'TRAZABILIDAD_AUDITORIA_Y_REPRODUCCION_DEL_TRABAJO_IA_2026_09_12.md'
compiler=sys.argv[1] if len(sys.argv)>1 else 'rustc'
version=subprocess.run([compiler,'--version','--verbose'],capture_output=True,text=True,check=True).stdout
blocks=re.findall(r'```rust\n(.*?)\n```',md.read_text(),re.S)
assert len(blocks)==3
result={'alcance':'Tres fragmentos editoriales; una entrada; sin benchmark ni ensayo del núcleo.', 'compilador':version,'esperado':'[4, 8, 12]\n','casos':[]}
with tempfile.TemporaryDirectory(prefix='sv-ejemplos-') as td:
 for i,code in enumerate(blocks,1):
  src=Path(td)/f'caso_{i}.rs';exe=Path(td)/f'caso_{i}'
  source='fn main() {\n'+code+'\n}\n';src.write_text(source)
  row={'caso':i,'fuente':source,'sha256_fuente':hashlib.sha256(source.encode()).hexdigest(),'procesos':[]}
  for args in [[compiler,str(src),'-o',str(exe)],[str(exe)]]:
   start=time.perf_counter();p=subprocess.run(args,capture_output=True,text=True);elapsed=time.perf_counter()-start
   row['procesos'].append({'invocacion':[a.replace(td,'<directorio_temporal>') for a in args],'codigo_salida':p.returncode,'stdout':p.stdout,'stderr':p.stderr,'duracion_observada_s':elapsed})
   if p.returncode:break
  row['conforme']=len(row['procesos'])==2 and all(x['codigo_salida']==0 for x in row['procesos']) and row['procesos'][-1]['stdout']==result['esperado']
  result['casos'].append(row)
result['todos_conformes']=all(x['conforme'] for x in result['casos'])
(root/'soporte/RESULTADOS_EJEMPLOS.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
print('Casos conformes:',sum(x['conforme'] for x in result['casos']),'/ 3')
assert result['todos_conformes']
