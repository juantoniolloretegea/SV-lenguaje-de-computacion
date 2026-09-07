#!/usr/bin/env python3
"""Cinco fuentes conservadas: identidad y N0-02/N0-03 sin compilador auxiliar.

La sensibilidad se mide inyectando pérdidas en copias del observable.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import sys
from oracle_support import run, assert_success, assert_json_equal, OracleError, assert_rust_rejection

ROOT = Path(__file__).resolve().parents[1]
CONTROL = (
    b'codomain KK = { Alpha, Beta };\n'
    b'output_semantics SS { Alpha -> "a"; Beta -> "b"; }\n'
    b'cellspec CC { b: 3; codomain: KK; semantics: SS; role: Base; }\n'
)

def digest(raw):
    return hashlib.sha256(raw).hexdigest()

def sources():
    cases = {
        'control_valid': CONTROL,
        'crlf': CONTROL.replace(b'\n', b'\r\n'),
        'string_crlf': CONTROL.replace(b'"a"', b'"a\r\nb"'),
        'semantics_duplicate': CONTROL.replace(b'Beta -> "b";', b'Beta -> "b"; Alpha -> "other";'),
    }
    cases['semantics_unbound_duplicate'] = cases['semantics_duplicate'].replace(
        b'cellspec CC { b: 3; codomain: KK; semantics: SS; role: Base; }\n', b'')
    return cases

def verify(name, raw, proc):
    if name in {'semantics_duplicate', 'semantics_unbound_duplicate'}:
        diagnostic = assert_rust_rejection(proc, 'output_semantics_clave_repetida')
        if 'repetidas=[Alpha]' not in diagnostic:
            raise OracleError('el rechazo no identifica Alpha')
        return 'RECHAZO_N0_02_N0_03'
    assert_success(proc)
    doc = json.loads(proc.stdout)
    if doc['source_sha256'] != digest(raw):
        raise OracleError('identidad de fuente alterada')
    mappings = next(o['fields']['mappings'] for o in doc['objects'] if o['name'] == 'SS')
    expected = 'a\r\nb' if name == 'string_crlf' else 'a'
    if mappings != {'Alpha': expected, 'Beta': 'b'}:
        raise OracleError('literal o miembros de semántica alterados')
    if name != 'control_valid':
        corrupted = copy.deepcopy(doc)
        if name == 'crlf':
            corrupted['source_sha256'] = digest(raw.replace(b'\r\n', b'\n'))
        else:
            obj = next(o for o in corrupted['objects'] if o['name'] == 'SS')
            obj['fields']['mappings']['Alpha'] = 'a\nb'
        try:
            assert_json_equal(proc.stdout, json.dumps(corrupted, ensure_ascii=False).encode())
        except OracleError:
            return 'PRESERVADO_Y_PERDIDA_INYECTADA_DETECTADA'
        raise OracleError('el observador no detecta la pérdida inyectada')
    return 'CONTROL_CONFORME'

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--rust-bin', type=Path, required=True)
    parser.add_argument('--output-dir', type=Path, required=True)
    parser.add_argument('--wasm-bin', type=Path)
    parser.add_argument('--wasi-runner', type=Path)
    args = parser.parse_args()
    if bool(args.wasm_bin) != bool(args.wasi_runner):
        parser.error('WASI requiere módulo y ejecutor')
    out = args.output_dir.resolve()
    out.mkdir(parents=True, exist_ok=True)
    records, failures = [], []
    for name, raw in sources().items():
        source = out / f'{name}.svp'
        source.write_bytes(raw)
        command = [str(args.rust_bin.resolve()), str(source)]
        if args.wasm_bin:
            command = ['node', '--no-warnings', str(args.wasi_runner.resolve()), str(args.wasm_bin.resolve()), str(source)]
        proc = run(command)
        (out / f'{name}.stdout').write_bytes(proc.stdout)
        (out / f'{name}.stderr').write_bytes(proc.stderr)
        record = {'case': name, 'source_sha256': digest(raw), 'command': command,
                  'returncode': proc.returncode, 'stdout_sha256': digest(proc.stdout),
                  'stderr_sha256': digest(proc.stderr)}
        try:
            record['result'] = verify(name, raw, proc)
        except (AssertionError, ValueError) as exc:
            record.update(result='FALLO', error=str(exc))
            failures.append(name)
        records.append(record)
        print(f'{name}: {record["result"]}')
    report = {
        'schema': 'sv-oracle-sensitivity-v4',
        'scope': 'cinco fuentes v3; preservación nativa/WASI y pérdida inyectada',
        'retired_frontend_last_cut': 'a09b9efef51f88de29048b9b35e7ac085dc0918f',
        'checkout_head': run(['git', 'rev-parse', 'HEAD']).stdout.decode().strip(),
        'executable_sha256': digest((args.wasm_bin or args.rust_bin).read_bytes()),
        'observer_sha256': {n: digest((ROOT / 'tests' / n).read_bytes())
                           for n in ['oracle_support.py', 'run_oracle_sensitivity.py']},
        'python_observer': sys.version, 'cases': records, 'failures': failures,
    }
    (out / 'report.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')
    return 1 if failures else 0

if __name__ == '__main__':
    raise SystemExit(main())
