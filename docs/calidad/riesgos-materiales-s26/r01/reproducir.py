"""Prepara copias fijadas y ejecuta cuatro sondas Rust; Python sólo orquesta.
Uso desde el checkout: python reproducir.py /ruta/nueva/de/campana
Se conserva stdout/stderr por orden; no se reutiliza un directorio de campaña.
"""
from pathlib import Path
import hashlib, json, os, shutil, subprocess, sys, tarfile
R=Path.cwd(); E=Path(__file__).resolve().parent; out=Path(sys.argv[1]).resolve()
assert not out.exists(), 'new campaign directory required'
for name,sha in json.loads((E/'PRECOMPROMISO.json').read_text()).items():
    assert hashlib.sha256((E/name).read_bytes()).hexdigest()==sha,name
for name,info in json.loads((E/'FUENTES.json').read_text())['archivos'].items():
    assert hashlib.sha256((R/name).read_bytes()).hexdigest()==info['sha256'],name
out.mkdir(parents=True)
B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-realizacion-i0205-v0_1'
shutil.copytree(B/'proyecto',out/'assembly')
with tarfile.open(B/'sv_core-verificado.tar.gz') as t:
    t.extractall(out,filter='data')
assert (out/'sv_core/Cargo.toml').is_file()
project=out/'probe';project.mkdir()
(project/'Cargo.toml').write_text('[package]\nname="sv_s26_probe"\nversion="0.1.0"\nedition="2021"\npublish=false\n[workspace]\n[lib]\npath="probe.rs"\n[dependencies]\nsv_core={path="../sv_core"}\nsv_bis_i0205={path="../assembly"}\n')
shutil.copyfile(E/'probe.rs',project/'probe.rs')
shutil.copyfile(E/'Cargo.lock',project/'Cargo.lock')
shutil.copytree(E/'fixture',out/'fixture');(out/'mutation').mkdir()
env=os.environ.copy();env.update(S26_FIXTURE=str(out/'fixture'),S26_WORK=str(out/'mutation'),CARGO_TARGET_DIR=str(out/'target'))
commands=[['rustc','--version'],['cargo','--version'],['cargo','test','--manifest-path',str(project/'Cargo.toml'),'--locked','--offline','--lib','--','--nocapture','--test-threads=1']]
results=[]
for i,cmd in enumerate(commands):
    with (out/f'{i:02d}.stdout').open('wb') as stdout,(out/f'{i:02d}.stderr').open('wb') as stderr:
        code=subprocess.run(cmd,stdout=stdout,stderr=stderr,env=env).returncode
    results.append({'argv':cmd,'exit_code':code})
    (out/'ORDENES.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n')
    print(i,code,flush=True)
    if code: raise SystemExit(code)
