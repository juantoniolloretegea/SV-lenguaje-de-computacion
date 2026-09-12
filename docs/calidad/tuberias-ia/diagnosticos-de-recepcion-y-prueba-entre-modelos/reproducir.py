"""Reproduce la cápsula pública RETP-157 y conserva cada proceso antes de juzgarlo.
Uso: python reproducir.py /ruta/rustc /directorio/nuevo
Ninguna llamada a modelos, conexión de red o constitución profesional.
"""
from pathlib import Path
import base64, datetime, hashlib, json, os, selectors, signal, subprocess, sys, time, resource
TIMEOUT_S=60
here=Path(__file__).resolve().parent
rust=Path(sys.argv[1]).resolve(); work=Path(sys.argv[2]).resolve();work.mkdir(parents=True,exist_ok=False)
def sha(b):return hashlib.sha256(b).hexdigest()
def save(name,obj):
    p=work/name;t=p.with_suffix(p.suffix+'.tmp');t.write_text(json.dumps(obj,ensure_ascii=False,indent=2)+'\n');t.replace(p)
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
for f in json.loads((here/'CANDIDATA_FUENTES.json').read_text())['archivos']:
    rel=Path(f['ruta']);assert not rel.is_absolute() and '..' not in rel.parts
    data=base64.b64decode(f['base64'],validate=True);assert sha(data)==f['sha256']
    p=work/'candidata'/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(data)
logs=[];events=[];observations=[]
save('IDENTIDAD.json',{'utc':utc(),'instrumento_sha256':sha(Path(__file__).read_bytes()),'rustc_sha256':sha(rust.read_bytes()),'capsula_sha256':sha((here/'CANDIDATA_FUENTES.json').read_bytes()),'esperados_sha256':sha((here/'ESPERADOS.json').read_bytes()),'catalogo_sha256':sha((here/'CATALOGO.json').read_bytes()),'timeout_s':60,'limite_por_flujo_bytes':2097152,'inferencia_proveedor':None,'rss_alcance':'máximo acumulado de hijos del instrumento; no RSS por caso','cpu_alcance':'diferencia RUSAGE_CHILDREN con procesos secuenciales'})

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

run('compilador',[rust,'--version','--verbose'])
expected={x['id']:x for x in json.loads((here/'ESPERADOS.json').read_text())['casos']}
cat={x['clave']:x for x in json.loads((here/'CATALOGO.json').read_text())['filas']}
# Comprueba cada archivo localizado contra el catálogo fijado, sin ejecutar un traductor.
for lang in ['es','en']:
    data=json.loads((here/'idiomas'/lang/'diagnosticos.json').read_text())
    assert data['version']=='RECEPTION-DIAGNOSTICS/1' and data['idioma']==lang
    assert data['mensajes']=={k:v[lang] for k,v in cat.items()}
src=work/'candidata/rust/sv_core/src/lib.rs'
for mode,flags in [('debug',['-C','opt-level=0','-C','overflow-checks=yes']),('release',['-C','opt-level=3','-C','overflow-checks=no'])]:
    exe=work/(mode+'-test')
    run('compilar '+mode,[rust,'--edition=2021','--test',src,*flags,'-o',exe])
    run('regresión '+mode,[exe,'--test-threads=1'],contains='239 passed; 0 failed')
    for repetition in range(1,4):
        row=run(f'focal {mode} {repetition}',[exe,'diagnostic_tests','--test-threads=1','--nocapture'],contains='8 passed; 0 failed')
        seen={}
        for line in row['stdout'].splitlines():
            if 'AUDIT\t' not in line:continue
            _,body=line.split('AUDIT\t',1);id,code,stage,es,en,ns=body.split('\t')
            assert id not in seen,(mode,id,'duplicado');seen[id]=code
            item=dict(configuracion=mode,repeticion=repetition,id=id,clave=code,fase=stage,es=es,en=en,duracion_observacion_ns=int(ns))
            observations.append(item);save('OBSERVACIONES.json',observations)
            assert id in expected and (code,stage)==(expected[id]['clave'],expected[id]['fase']),(id,code,stage)
            if code=='ACREDITADO':assert es==en==''
            else:assert (es,en)==(cat[code]['es'],cat[code]['en']),(id,'presentación')
        assert set(seen)==set(expected),(mode,seen.keys())
lib=work/'libsv_core.rlib'
run('biblioteca ordinaria',[rust,'--edition=2021','--crate-name=sv_core','--crate-type=lib',src,'-o',lib])
run('cliente público compila',[rust,'--edition=2021',here/'clientes/publico.rs','--extern',f'sv_core={lib}','-o',work/'publico'])
run('cliente público ejecuta',[work/'publico'],contains='continuidad vacía sin autoridad')
for name,codes in [('no_instalar',['E0624','E0624']),('no_premisa',['E0451']),('no_desligar',['E0616']),('no_instalar_diagnostico',['E0624','E0624'])]:
    run(name,[rust,'--edition=2021','--error-format=json',here/'clientes'/(name+'.rs'),'--extern',f'sv_core={lib}','-o',work/name],1,codes)
save('RESULTADO.json',{'estado':'CONFORME_EN_ALCANCE','pruebas_por_configuracion':239,'pruebas_heredadas':231,'pruebas_nuevas':8,'configuraciones':['debug','release'],'casos_focales_distintos':40,'repeticiones_por_configuracion':3,'observaciones_focales':len(observations),'causas_locales':len(cat),'idiomas':['es','en'],'procesos':len(logs),'modelos_externos_ejecutados':[],'wasi':'NO_ACREDITADO','navegador':'NO_ACREDITADO','premisa':'SINTETICA_FOR_TEST','cierre_global':False})
print('CONFORME EN ALCANCE; cierre global pendiente',flush=True)
