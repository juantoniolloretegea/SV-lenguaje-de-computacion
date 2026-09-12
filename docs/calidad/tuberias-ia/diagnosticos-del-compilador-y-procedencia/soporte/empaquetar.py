from pathlib import Path
import json,base64,hashlib,shutil,difflib,ast
r=Path('tmp/diagnosticos-158');o=Path('entregas/diagnosticos-del-compilador-y-procedencia')
def cap(root):
 return [{'ruta':str(p.relative_to(root)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'base64':base64.b64encode(p.read_bytes()).decode()} for p in sorted(root.rglob('*')) if p.is_file()]
(o/'CANDIDATA_FUENTES.json').write_text(json.dumps({'version':'RETP-158','base':'RETP-157','archivos':cap(r/'candidata')},indent=2)+'\n')
(o/'BASE_FUENTES.json').write_text(json.dumps({'version':'RETP-157','archivos':cap(Path('tmp/continuacion-157/candidata'))},indent=2)+'\n')
(o/'CORPUS_FUENTES.json').write_text(json.dumps({'head':'4cacf3ec6bd5d0f31206197c7374515a56b34a89','archivos':cap(r/'corpus')},indent=2)+'\n')
shutil.copy(r/'CORPUS.json',o/'CORPUS.json');shutil.copy(r/'ESPERADO_RELACIONAL.json',o/'ESPERADO_RELACIONAL.json')
shutil.copy(r/'COMPROMISO_PLAN.json',o/'COMPROMISO_PLAN.json')
for f in ['focal.rs','relacional.rs','corpus.rs','catalogo.rs']:shutil.copy(r/f,o/f)
# Expected IDs/causes were fixed in focal.rs before first execution; this projection
# is an index for the portable receiver, not a new independent oracle.
rows=[]
for l in (r/'evidencia/01-debug-focal-1.stdout').read_text().splitlines():
 _,id,cause,ns=l.split('\t');rows.append({'id':id,'cause':cause})
(o/'INDICE_ESPERADOS.json').write_text(json.dumps({'procedencia':'Proyección de assertions fijadas en focal.rs; IDs cotejados con primera captura. No es oráculo independiente nuevo.','casos':rows},indent=2)+'\n')
patch=[]
for p in sorted((r/'candidata').rglob('*.rs')):
 rel=p.relative_to(r/'candidata');old=Path('tmp/continuacion-157/candidata')/rel
 before=old.read_text() if old.exists() else '';after=p.read_text()
 if before!=after:
  patch+=list(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='a/'+str(rel),tofile='b/'+str(rel)))
  target=o/'cambios'/rel;target.parent.mkdir(parents=True,exist_ok=True);target.write_text(after)
(o/'CAMBIO_INCREMENTAL.patch').write_text(''.join(patch))
# Reuse the already tested bounded subprocess implementation verbatim.
original=Path('entregas/diagnosticos-de-recepcion-y-prueba-entre-modelos/reproducir.py').read_text();tree=ast.parse(original)
run=next(ast.get_source_segment(original,n) for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=='run')
header='''"""Receptor público RETP-158: reproducción nativa, no examen ciego entre modelos.
python reproducir.py /ruta/rustc /directorio/nuevo
Linux/Python 3.10+; límites de 120 s por proceso y 2 MiB por flujo. Sin red.
"""
from pathlib import Path
import base64, datetime, hashlib, json, os, selectors, signal, subprocess, sys, time, resource
TIMEOUT_S=120
here=Path(__file__).resolve().parent
rust=Path(sys.argv[1]).resolve();work=Path(sys.argv[2]).resolve();work.mkdir(parents=True,exist_ok=False)
def sha(b):return hashlib.sha256(b).hexdigest()
def save(name,obj):
    p=work/name;t=p.with_suffix(p.suffix+'.tmp');t.write_text(json.dumps(obj,ensure_ascii=False,indent=2)+'\\n');t.replace(p)
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
for filename,folder in [('CANDIDATA_FUENTES.json','candidata'),('BASE_FUENTES.json','base'),('CORPUS_FUENTES.json','corpus')]:
    for f in json.loads((here/filename).read_text())['archivos']:
        rel=Path(f['ruta']);assert not rel.is_absolute() and '..' not in rel.parts
        data=base64.b64decode(f['base64'],validate=True);assert sha(data)==f['sha256']
        p=work/folder/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(data)
logs=[];events=[];observations=[]
save('IDENTIDAD.json',{'utc':utc(),'instrumento_sha256':sha(Path(__file__).read_bytes()),'rustc_sha256':sha(rust.read_bytes()),'archivos':{p.name:sha(p.read_bytes()) for p in here.iterdir() if p.is_file() and p.suffix in ['.json','.rs']},'timeout_s':TIMEOUT_S,'limite_flujo_bytes':2097152,'cpu':'RUSAGE_CHILDREN; diferencias por proceso secuencial','rss':'máximo acumulado de hijos, no consumo por caso','costo_proveedor':None})
'''
body='''
run('rustc',[rust,'--version','--verbose'])
items=json.loads((here/'CORPUS.json').read_text())
for x in items:assert sha((work/'corpus'/x['path']).read_bytes())==x['sha256']
(work/'corpus.tsv').write_text(''.join(x['expected']+'\\t'+str(work/'corpus'/x['path'])+'\\n' for x in items))
expected={x['id']:x['cause'] for x in json.loads((here/'INDICE_ESPERADOS.json').read_text())['casos']}
rel=json.loads((here/'ESPERADO_RELACIONAL.json').read_text())
for mode,flags in [('debug',[]),('release',['-O'])]:
    outputs={}
    for name in ['base','candidata']:
        src=work/name/'rust/sv_core/src/lib.rs';lib=work/f'{name}-{mode}.rlib'
        run(f'{name}-{mode}-compile',[rust,'--edition=2021','--crate-name','sv_core','--crate-type','rlib',*flags,src,'-o',lib])
        # Rust requires the lib prefix for an extern artifact.
        target=work/f'libsv_core_{name}_{mode}.rlib';lib.rename(target);lib=target
        exe=work/f'corpus-{name}-{mode}'
        run(f'{name}-{mode}-corpus-build',[rust,'--edition=2021',*flags,here/'corpus.rs','--extern',f'sv_core={lib}','-o',exe])
        outputs[name]=run(f'{name}-{mode}-corpus',[exe,work/'corpus.tsv'])['stdout']
    assert outputs['base']==outputs['candidata'],'El resultado heredado ha cambiado'
    lib=work/f'libsv_core_candidata_{mode}.rlib';src=work/'candidata/rust/sv_core/src/lib.rs'
    run(f'{mode}-unit-build',[rust,'--edition=2021','--test',*flags,src,'-o',work/f'units-{mode}'])
    run(f'{mode}-units',[work/f'units-{mode}','--test-threads=1'],contains='239 passed; 0 failed')
    for kind in ['focal','relacional']:
        exe=work/f'{kind}-{mode}';run(f'{mode}-{kind}-build',[rust,'--edition=2021',*flags,here/f'{kind}.rs','--extern',f'sv_core={lib}','-o',exe])
        for rep in range(1,4):
            row=run(f'{mode}-{kind}-{rep}',[exe]);seen={};hashes={}
            for line in row['stdout'].splitlines():
                t=line.split('\\t')
                if t[0]=='CASE':
                    _,id,cause,ns=t;assert id not in seen;seen[id]=cause
                    observations.append({'mode':mode,'rep':rep,'id':id,'cause':cause,'elapsed_ns':int(ns)})
                if t[0]=='HASH':hashes[t[1]]=t[2]
            if kind=='focal':assert seen==expected
            else:assert seen=={'E115_FOUR_UNITS':'CD.OUTPUT_SEMANTICS_KEYS'} and hashes==rel['hashes']
            save('OBSERVACIONES.json',observations)
lib=work/'libsv_core_candidata_debug.rlib'
run('catalogo-build',[rust,'--edition=2021',here/'catalogo.rs','--extern',f'sv_core={lib}','-o',work/'catalogo'])
rows=run('catalogo',[work/'catalogo'])['stdout'].splitlines();actual=[]
for line in rows:
    _,key,code,es,en=line.split('\\t');actual.append({'id':key,'code':code or None,'es':es,'en':en})
assert actual==json.loads((here/'CATALOGO.json').read_text())['causas']
for lang in ['es','en']:
    assert json.loads((here/'idiomas'/lang/'diagnosticos.json').read_text())['mensajes']=={x['id']:x[lang] for x in actual}
run('publico-build',[rust,'--edition=2021',here/'clientes/publico.rs','--extern',f'sv_core={lib}','-o',work/'publico'])
run('publico',[work/'publico'],contains='continuidad vacía sin autoridad')
for name,codes in [('no_instalar',['E0624','E0624']),('no_premisa',['E0451']),('no_desligar',['E0616']),('no_instalar_diagnostico',['E0624','E0624'])]:
    run(name,[rust,'--edition=2021','--error-format=json',here/'clientes'/(name+'.rs'),'--extern',f'sv_core={lib}','-o',work/name],1,codes)
save('RESULTADO.json',{'estado':'CONFORME_EN_ALCANCE_NATIVO','unitarios_por_modo':239,'corpus_validos':14,'corpus_invalidos':106,'legacy_identico':True,'focales':29,'observaciones':len(observations),'modos':['debug','release'],'repeticiones':3,'causas_locales':12,'procesos':len(logs),'clientes_negativos':4,'errores_acceso_esperados':6,'wasi':False,'navegador':False,'ejecuciones_proveedores_externos':0,'cierre_global':False})
print('RETP-158 CONFORME EN ALCANCE NATIVO',flush=True)
'''
(o/'reproducir.py').write_text(header+'\n'+run+'\n'+body)
compile((o/'reproducir.py').read_text(),'reproducir.py','exec')
print('Cápsulas, parche y receptor portátil preparados')
