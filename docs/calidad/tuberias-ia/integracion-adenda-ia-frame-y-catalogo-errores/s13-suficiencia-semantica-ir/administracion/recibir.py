from pathlib import Path
import json,sys,hashlib,concurrent.futures
R=Path(__file__).resolve().parent;W=R.parent;sys.path.insert(0,str(W/'s6-trazabilidad-total'));from github_io import call
b=json.loads((R/'lenguaje-base.json').read_text());tree=json.loads((R/'lenguaje-arbol.json').read_text());d=R/'lectura';d.mkdir(exist_ok=True)
paths=['README.md','IR_CANONICA_BIENFORMACION_SV_v0_3.md','FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md','GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md','docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md','rust/sv_core/src/ir.rs','rust/sv_core/src/decision_trace.rs','rust/sv_core/src/frame.rs','rust/sv_core/src/execution.rs','rust/sv_core/src/permission.rs','rust/sv_core/src/requirements_coverage.rs','rust/sv_core/src/lib.rs']
if len(sys.argv)>1:paths=sys.argv[1:]
blob=lambda v:hashlib.sha1(b'blob '+str(len(v)).encode()+b'\0'+v).hexdigest()
record=json.loads((R/'FUENTES_RECIBIDAS.json').read_text()) if (R/'FUENTES_RECIBIDAS.json').exists() else {}
def get(path):
 assert path in tree,path
 dest=d/path;dest.parent.mkdir(parents=True,exist_ok=True)
 if dest.exists() and blob(dest.read_bytes())==tree[path]:v=dest.read_bytes()
 else:
  got=call('fetch_file',dict(repository_full_name=b['repo'],path=path,ref=b['head']));assert got['sha']==tree[path];v=got['content'].encode();assert blob(v)==tree[path];dest.write_bytes(v)
 return path,dict(blob=tree[path],bytes=len(v),sha256=hashlib.sha256(v).hexdigest(),corte=b['head'])
with concurrent.futures.ThreadPoolExecutor(4) as ex:
 for p,v in ex.map(get,paths):record[p]=v;print(p,v['bytes'],flush=True)
(R/'FUENTES_RECIBIDAS.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
r=json.loads((W/'s12-rectificacion/entrega/RECTORES.json').read_text())
for p,h in r['piezas_sin_cambio_desde_lectura_previa'].items():assert tree[p]==h
print('Rectores leídos previamente: blobs vigentes idénticos.')
