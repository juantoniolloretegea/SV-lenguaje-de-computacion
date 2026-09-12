"""Receptor público RETP-158: reproducción nativa, no examen ciego entre modelos.
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
    p=work/name;t=p.with_suffix(p.suffix+'.tmp');t.write_text(json.dumps(obj,ensure_ascii=False,indent=2)+'\n');t.replace(p)
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
for filename,folder in [('CANDIDATA_FUENTES.json','candidata'),('BASE_FUENTES.json','base'),('CORPUS_FUENTES.json','corpus')]:
    for f in json.loads((here/filename).read_text())['archivos']:
        rel=Path(f['ruta']);assert not rel.is_absolute() and '..' not in rel.parts
        data=base64.b64decode(f['base64'],validate=True);assert sha(data)==f['sha256']
        p=work/folder/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(data)
logs=[];events=[];observations=[]
save('IDENTIDAD.json',{'utc':utc(),'instrumento_sha256':sha(Path(__file__).read_bytes()),'rustc_sha256':sha(rust.read_bytes()),'archivos':{p.name:sha(p.read_bytes()) for p in here.iterdir() if p.is_file() and p.suffix in ['.json','.rs']},'timeout_s':TIMEOUT_S,'limite_flujo_bytes':2097152,'cpu':'RUSAGE_CHILDREN; diferencias por proceso secuencial','rss':'máximo acumulado de hijos, no consumo por caso','costo_proveedor':None})

def run(name,args,expected_exit=0,codes=None,contains=None):
    args=list(map(str,args));start=time.monotonic_ns();before=resource.getrusage(resource.RUSAGE_CHILDREN)
    events.append({'evento':'inicio','paso':name,'utc':utc(),'argv':args});save('SECUENCIA.json',events)
    p=subprocess.Popen(args,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
    sel=selectors.DefaultSelector();buffers={'stdout':bytearray(),'stderr':bytearray()}
    sel.register(p.stdout,selectors.EVENT_READ,'stdout');sel.register(p.stderr,selectors.EVENT_READ,'stderr');limit=None
    while sel.get_map():
        if time.monotonic_ns()-start>TIMEOUT_S*1_000_000_000 and limit is None:
            limit='TIEMPO';os.killpg(p.pid,signal.SIGKILL)
        for key,_ in sel.select(0.05):
            data=os.read(key.fileobj.fileno(),65536)
            if not data:sel.unregister(key.fileobj);continue
            b=buffers[key.data];available=max(0,2097152-len(b));b.extend(data[:available])
            if len(data)>available and limit is None:
                limit='SALIDA';os.killpg(p.pid,signal.SIGKILL)
    sel.close()
    # Closing both output streams does not imply that the process has ended.
    remaining=max(0.001,TIMEOUT_S-(time.monotonic_ns()-start)/1e9)
    try:rc=p.wait(timeout=remaining)
    except subprocess.TimeoutExpired:
        limit='TIEMPO'
        os.killpg(p.pid,signal.SIGKILL)
        rc=p.wait()
    after=resource.getrusage(resource.RUSAGE_CHILDREN)
    row={'paso':name,'argv':args,'codigo':rc,'limite_excedido':limit,'duracion_ns':time.monotonic_ns()-start,'fin_utc':utc(),'cpu_usuario_s':after.ru_utime-before.ru_utime,'cpu_sistema_s':after.ru_stime-before.ru_stime,'rss_max_acumulado_hijos_kib':after.ru_maxrss}
    for stream,data in buffers.items():
        row[stream+'_base64']=base64.b64encode(data).decode();row[stream+'_sha256']=sha(data);row[stream]=bytes(data).decode('utf-8',errors='backslashreplace')
    row['binario_ejecutado_sha256']=sha(Path(args[0]).read_bytes())
    logs.append(row);save('PROCESOS.json',logs);events.append({'evento':'fin','paso':name,'utc':row['fin_utc'],'codigo':rc,'limite_excedido':limit});save('SECUENCIA.json',events)
    assert limit is None, row
    assert rc==expected_exit,(name,rc,row['stderr'])
    if contains:assert contains in row['stdout'],name
    if codes is not None:
        actual=[]
        for line in row['stderr'].splitlines():
            try:d=json.loads(line)
            except ValueError:continue
            if d.get('level')=='error' and d.get('code'):actual.append(d['code']['code'])
        assert sorted(actual)==sorted(codes),(name,actual,codes)
    print(name,'conforme',flush=True)
    return row

run('rustc',[rust,'--version','--verbose'])
items=json.loads((here/'CORPUS.json').read_text())
for x in items:assert sha((work/'corpus'/x['path']).read_bytes())==x['sha256']
(work/'corpus.tsv').write_text(''.join(x['expected']+'\t'+str(work/'corpus'/x['path'])+'\n' for x in items))
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
                t=line.split('\t')
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
    _,key,code,es,en=line.split('\t');actual.append({'id':key,'code':code or None,'es':es,'en':en})
assert actual==json.loads((here/'CATALOGO.json').read_text())['causas']
for lang in ['es','en']:
    assert json.loads((here/'idiomas'/lang/'diagnosticos.json').read_text())['mensajes']=={x['id']:x[lang] for x in actual}
run('publico-build',[rust,'--edition=2021',here/'clientes/publico.rs','--extern',f'sv_core={lib}','-o',work/'publico'])
run('publico',[work/'publico'],contains='continuidad vacía sin autoridad')
for name,codes in [('no_instalar',['E0624','E0624']),('no_premisa',['E0451']),('no_desligar',['E0616']),('no_instalar_diagnostico',['E0624','E0624'])]:
    run(name,[rust,'--edition=2021','--error-format=json',here/'clientes'/(name+'.rs'),'--extern',f'sv_core={lib}','-o',work/name],1,codes)
save('RESULTADO.json',{'estado':'CONFORME_EN_ALCANCE_NATIVO','unitarios_por_modo':239,'corpus_validos':14,'corpus_invalidos':106,'legacy_identico':True,'focales':29,'observaciones':len(observations),'modos':['debug','release'],'repeticiones':3,'causas_locales':12,'procesos':len(logs),'clientes_negativos':4,'errores_acceso_esperados':6,'wasi':False,'navegador':False,'ejecuciones_proveedores_externos':0,'cierre_global':False})
print('RETP-158 CONFORME EN ALCANCE NATIVO',flush=True)
