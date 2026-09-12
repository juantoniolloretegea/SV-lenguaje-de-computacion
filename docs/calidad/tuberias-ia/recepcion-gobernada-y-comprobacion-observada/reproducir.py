"""Reproduce la candidata técnica; no constituye una raíz profesional."""
from pathlib import Path
import sys, json, base64, hashlib, subprocess
here = Path(__file__).resolve().parent
rust = Path(sys.argv[1]).resolve()
work = Path(sys.argv[2]).resolve()
work.mkdir(exist_ok=False)
for f in json.loads((here / 'CANDIDATA_FUENTES.json').read_text())['archivos']:
    rel = Path(f['ruta'])
    assert not rel.is_absolute() and '..' not in rel.parts
    data = base64.b64decode(f['base64'])
    assert hashlib.sha256(data).hexdigest() == f['sha256']
    p = work / 'candidata' / rel
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_bytes(data)
logs = []
def run(name, args, code=0, codes=None, contains=None):
    args = list(map(str, args))
    p = subprocess.run(args, capture_output=True, text=True, timeout=60)
    row = dict(paso=name, argv=args, codigo=p.returncode, stdout=p.stdout, stderr=p.stderr)
    logs.append(row)
    (work / 'REPRODUCCION.json').write_text(json.dumps(logs, ensure_ascii=False, indent=2) + '\n')
    assert p.returncode == code, row
    if contains:
        assert contains in p.stdout, row
    if codes is not None:
        actual = []
        for line in p.stderr.splitlines():
            try:
                d = json.loads(line)
            except ValueError:
                continue
            if d.get('level') == 'error' and d.get('code'):
                actual.append(d['code']['code'])
        assert sorted(actual) == sorted(codes), row
    print(name, 'conforme', flush=True)
src = work / 'candidata/rust/sv_core/src/lib.rs'
for mode, flags in [('debug', ['-C', 'opt-level=0', '-C', 'overflow-checks=yes']),
                    ('release', ['-C', 'opt-level=3', '-C', 'overflow-checks=no'])]:
    exe = work / (mode + '-test')
    run('compilar ' + mode, [rust, '--edition=2021', '--test', src, *flags, '-o', exe])
    run('ejecutar ' + mode, [exe, '--test-threads=1'], contains='231 passed; 0 failed')
lib = work / 'libsv_core.rlib'
run('biblioteca ordinaria', [rust, '--edition=2021', '--crate-name=sv_core', '--crate-type=lib', src, '-o', lib])
run('cliente público compila', [rust, '--edition=2021', here/'clientes/publico.rs', '--extern', f'sv_core={lib}', '-o', work/'publico'])
run('cliente público ejecuta', [work/'publico'], contains='continuidad vacía sin autoridad')
for name, codes in [('no_instalar', ['E0624','E0624']), ('no_premisa', ['E0451']), ('no_desligar', ['E0616'])]:
    run(name, [rust, '--edition=2021', '--error-format=json', here/'clientes'/(name+'.rs'),
               '--extern', f'sv_core={lib}', '-o', work/name], 1, codes)
