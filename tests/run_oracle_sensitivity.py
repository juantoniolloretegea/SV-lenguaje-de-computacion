#!/usr/bin/env python3
"""Sensibilidad a deudas abiertas y regresión del cierre relacional N0-02.

Las sondas CRLF conservan su alcance DFL-008. La semántica duplicada enlazada
se rechaza con E115; la no enlazada conserva un testigo de la deuda N0-03.
"""
import argparse
import hashlib
import json
from pathlib import Path
import sys

from oracle_support import (run, assert_success, assert_json_equal, ordered_json,
                            DuplicateJsonMember, OracleError, assert_python_rejection,
                            assert_rust_rejection)

ROOT = Path(__file__).resolve().parents[1]
CONTROL = (
    b'codomain KK = { Alpha, Beta };\n'
    b'output_semantics SS { Alpha -> "a"; Beta -> "b"; }\n'
    b'cellspec CC { b: 3; codomain: KK; semantics: SS; role: Base; }\n'
)


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--rust-bin', type=Path, required=True)
    parser.add_argument('--output-dir', type=Path, required=True)
    args = parser.parse_args()
    out = args.output_dir.resolve()
    out.mkdir(parents=True, exist_ok=True)
    cases = {
        'control_valid': CONTROL,
        'crlf': CONTROL.replace(b'\n', b'\r\n'),
        'string_crlf': CONTROL.replace(b'"a"', b'"a\r\nb"'),
        'semantics_duplicate': CONTROL.replace(b'Beta -> "b";', b'Beta -> "b"; Alpha -> "other";'),
    }
    cases['semantics_unbound_duplicate'] = cases['semantics_duplicate'].replace(
        b'cellspec CC { b: 3; codomain: KK; semantics: SS; role: Base; }\n', b'')
    records, failures = [], []
    for name, raw in cases.items():
        source = out / f'{name}.svp'
        source.write_bytes(raw)
        observations = {}
        for emitter, command in [
            ('python', [sys.executable, str(ROOT / 'src/svp_main.py'), str(source)]),
            ('rust', [str(args.rust_bin.resolve()), str(source)])]:
            proc = run(command)
            (out / f'{name}.{emitter}.stdout').write_bytes(proc.stdout)
            (out / f'{name}.{emitter}.stderr').write_bytes(proc.stderr)
            observations[emitter] = proc
        py, rust = observations['python'], observations['rust']
        record = {'case': name, 'source_sha256': digest(raw), 'observations': {
            key: {'command': value.args, 'returncode': value.returncode,
                  'stdout_sha256': digest(value.stdout), 'stderr_sha256': digest(value.stderr)}
            for key, value in observations.items()}}
        try:
            if name == 'semantics_duplicate':
                assert_python_rejection(py, 'E115')
                assert_rust_rejection(rust, 'output_semantics_clave_repetida')
                for proc in [py, rust]:
                    if b'repetidas=[Alpha]' not in proc.stderr:
                        raise OracleError('el rechazo no identifica la clave repetida')
            else:
                assert_success(py)
                if name != 'semantics_unbound_duplicate':
                    assert_success(rust)
                elif rust.returncode != 0 or rust.stderr:
                    raise OracleError('no se reprodujo la admisión Rust de la sonda no enlazada')
            if name == 'control_valid':
                assert_json_equal(py.stdout, rust.stdout)
                if json.loads(rust.stdout)['source_sha256'] != digest(raw):
                    raise OracleError('identidad del control incorrecta')
                record['result'] = 'CONTROL_CONFORME'
            elif name == 'semantics_duplicate':
                record['result'] = 'CIERRE_RELACIONAL_N0_02_E115'
            elif name == 'semantics_unbound_duplicate':
                try:
                    ordered_json(rust.stdout)
                except DuplicateJsonMember:
                    pass
                else:
                    raise OracleError('no se detectó el miembro JSON duplicado')
                if ordered_json(py.stdout, reject_duplicates=False) == ordered_json(rust.stdout, reject_duplicates=False):
                    raise OracleError('se perdió la multiplicidad durante el contraste')
                record['result'] = 'DIVERGENCIA_DETECTADA_MIEMBRO_DUPLICADO'
            else:
                py_doc, rust_doc = json.loads(py.stdout), json.loads(rust.stdout)
                if rust_doc['source_sha256'] != digest(raw) or py_doc['source_sha256'] == digest(raw):
                    raise OracleError('no se reprodujo la pérdida de identidad por CRLF')
                if name == 'string_crlf':
                    # Aísla el contenido después de comprobar ambas huellas.
                    if ordered_json(json.dumps(py_doc['objects']).encode()) == ordered_json(json.dumps(rust_doc['objects']).encode()):
                        raise OracleError('no se detectó la modificación del literal CRLF')
                try:
                    assert_json_equal(py.stdout, rust.stdout)
                except OracleError:
                    pass
                else:
                    raise OracleError('el comprobador ocultó la divergencia CRLF')
                record['result'] = 'DIVERGENCIA_DETECTADA_CRLF'
            print(f'{name}: {record["result"]}')
        except (AssertionError, ValueError) as exc:
            record['result'] = 'FALLO_DE_REPRODUCCION_O_DETECCION'
            record['error'] = str(exc)
            failures.append(name)
        records.append(record)
    report = {
        'schema': 'sv-oracle-sensitivity-v2',
        'scope': 'cierre relacional N0-02; sensibilidad a DFL-008 y N0-03 sin cerrar esas deudas',
        'base_head': 'ed61af2fb80641866356a7138cc87763eab005d9',
        'checkout_head': run(['git', 'rev-parse', 'HEAD']).stdout.decode().strip(),
        'rust_binary_sha256': digest(args.rust_bin.read_bytes()),
        'observer_sha256': {name: digest((ROOT / 'tests' / name).read_bytes())
                            for name in ['oracle_support.py', 'run_oracle_sensitivity.py']},
        'python': sys.version, 'cases': records, 'failures': failures,
    }
    (out / 'report.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')
    if failures:
        print('No se acreditó la sensibilidad: ' + ', '.join(failures), file=sys.stderr)
        return 1
    print('Sensibilidad: 1 control válido, 1 rechazo N0-02 y 3 divergencias abiertas detectadas.')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
