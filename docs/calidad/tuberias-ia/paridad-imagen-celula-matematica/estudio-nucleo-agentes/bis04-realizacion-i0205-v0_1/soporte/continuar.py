from pathlib import Path
import subprocess,json,hashlib,datetime,os,time
out=Path('bis-realizacion/campana-01');W=Path('bis-realizacion/montaje');target=W/'proyecto/target/debug'
commands=json.loads((out/'ORDENES.json').read_text());args=[str(target/'sv_bis_i0205'),str(W/'entradas'),str(out/'resultados')];begin=datetime.datetime.now(datetime.timezone.utc).isoformat();t=time.monotonic_ns()
with (out/'campana.stdout').open('wb') as stdout,(out/'campana.stderr').open('wb') as stderr:
 p=subprocess.Popen(args,stdout=stdout,stderr=stderr);_,status,usage=os.wait4(p.pid,0);p.returncode=os.waitstatus_to_exitcode(status)
commands.append({'orden':args,'inicio_utc':begin,'exit_code':p.returncode,'stdout':'campana.stdout','stderr':'campana.stderr','medicion':'os.wait4 del PID del ejecutable Rust, sin incluir Cargo'});(out/'ORDENES.json').write_text(json.dumps(commands,indent=2)+'\n')
(out/'RECURSOS.json').write_text(json.dumps({'elapsed_ns':time.monotonic_ns()-t,'max_rss_kib_linux':usage.ru_maxrss,'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime},indent=2)+'\n')
(out/'INCIDENCIA_MEDICION.json').write_text(json.dumps({'accion':'invocar /usr/bin/time para campaña','error':'FileNotFoundError: /usr/bin/time','ejecucion_rust_en_ese_intento':False,'correccion':'subprocess.Popen + os.wait4; mismo binario y entradas'},indent=2)+'\n')
pub=json.loads(Path('bis-realizacion/PREVIO_PUBLICO.json').read_text());lab=json.loads(Path('bis-realizacion/PREVIO_LAB.json').read_text());results=json.loads((out/'ENCAPSULACION.json').read_text())
(out/'CORTE.json').write_text(json.dumps({'precompromiso_publico':pub['commit'],'precompromiso_laboratorio':lab['commit'],'ejecutable_sha256':hashlib.sha256((target/'sv_bis_i0205').read_bytes()).hexdigest(),'sondas_conformes':sum(x['conforme'] for x in results),'exit_campana':p.returncode},indent=2)+'\n')
print((out/'campana.stdout').read_text());print((out/'campana.stderr').read_text());print('Exit:',p.returncode)
