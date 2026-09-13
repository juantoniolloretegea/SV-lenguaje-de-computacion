from pathlib import Path
import subprocess, os, json, hashlib
from datetime import datetime, timezone

base = Path(__file__).resolve().parent
root = base.parent / 'manifiesto-sv/checkout'
env = dict(os.environ)
env.pop('SV_RUST_PREFIX', None)
env['PATH'] = ':'.join(x for x in env['PATH'].split(':') if x != '/opt/sv-rust-1.98.0/bin')
results = []

def run(name, args, cwd=base, expected=0):
    r = subprocess.run(args, cwd=cwd, env=env, capture_output=True, timeout=60)
    (base / (name + '.stdout')).write_bytes(r.stdout)
    (base / (name + '.stderr')).write_bytes(r.stderr)
    results.append({'id': name, 'argv': args, 'cwd': str(cwd), 'exit_code': r.returncode})
    assert r.returncode == expected, (name, r.returncode, r.stderr.decode(errors='replace'))
    return r

login = run('sesion-login', ['bash', '-lc', 'command -v cargo rustc; rustc --version; cargo --version'])
assert b'/opt/sv-rust-1.98.0/bin/cargo' in login.stdout
assert b'rustc 1.98.0' in login.stdout and b'cargo 1.98.0' in login.stdout
run('sesion-bashrc', ['bash', '--noprofile', '-c', '. /root/.bashrc; command -v cargo; cargo --version'])
run('idempotencia', ['bash', '--noprofile', '-c', '. /opt/sv-rust-1.98.0/env.sh; first=$PATH; . /opt/sv-rust-1.98.0/env.sh; test "$first" = "$PATH"'])
probe = base.parent / 'rust-recovery-check'
run('sonda-test', ['bash', '-lc', 'cargo test --offline'], probe)
positive = run('sonda-run', ['bash', '-lc', 'cargo run --offline --quiet'], probe)
assert positive.stdout == b'SV_TOOLCHAIN_OK:100\n'
negative = run('sonda-rechazo', ['/opt/sv-rust-1.98.0/bin/rustc', '--edition=2024', 'type_error.rs', '-o', str(base/'no-debe-existir')], probe, 1)
assert b'E0308' in negative.stderr and not (base/'no-debe-existir').exists()
run('bis-c01', ['node', str(root/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis-c01/verificar.mjs'), str(base/'target-native/debug/sv-native'), str(base/'bis-c01-ejecucion')], root)
report = json.loads((base/'bis-c01-ejecucion/RESULTADO.json').read_text())
assert report['conformes'] == 13 and all(x['detectada'] for x in report['sensibilidad'])
payload = {'fecha_utc': datetime.now(timezone.utc).isoformat(), 'corte_compilado': subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(), 'montaje': 'Compilación nativa con directorio target nuevo, Cargo --locked --offline; 25 warnings conservados en build-clean.stderr.', 'resultados': results, 'bis_c01': {'variantes':13,'conformes':13,'sensibilidades_detectadas':4,'nota_corte': 'RESULTADO.json conserva corte_lenguaje del banco histórico; el corte realmente compilado consta aquí.'}, 'c02_c12_ejecutados':0}
(base/'VERIFICACION.json').write_text(json.dumps(payload,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'sesiones':'OK','rustc':'1.98.0','cargo':'1.98.0','bis_c01':'13/13','sensibilidad':'4/4','c02_c12':0}))
