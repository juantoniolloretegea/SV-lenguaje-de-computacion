from pathlib import Path
import sys,hashlib,json
sys.path.insert(0,'tmp/diagnosticos-158')
from ejecutar import run,r,rustc,e
# La conexión E115 repartida entre unidades no estaba cubierta por la prueba
# de colisión. Se fija su esperado antes de esta ejecución; no cambia código.
sources=['-- é\r\ncodomain K = {A, B};','semántica_de_salida S { A -> "SECRETO"; X -> "x"; }','cellspec C { b: 3; codomain: K; semantics: S; role: Base; }','codomain Z = {A};']
expected={str(i):hashlib.sha256(s.encode()).hexdigest() for i,s in enumerate(sources)}
(r/'ESPERADO_RELACIONAL.json').write_text(json.dumps({'hashes':expected,'sites':[2,1,0],'missing':['B'],'extra':['X'],'cause':'CD.OUTPUT_SEMANTICS_KEYS','code':'E115','test_sha256':hashlib.sha256((r/'relacional.rs').read_bytes()).hexdigest()},indent=2)+'\n')
for mode,flags in [('debug',[]),('release',['-O'])]:
 run(f'02-{mode}-relational-build',[rustc,'--edition=2021',*flags,str(r/'relacional.rs'),'--extern',f'sv_core={r}/libsv_core_new_{mode}.rlib','-o',str(r/f'relacional-{mode}')])
 for rep in range(1,4):
  out=run(f'02-{mode}-relational-{rep}',[str(r/f'relacional-{mode}')]).decode()
  actual={t[1]:t[2] for l in out.splitlines() if (t:=l.split('\t'))[0]=='HASH'};assert actual==expected
print('Procedencia E115 entre cuatro unidades comprobada')
