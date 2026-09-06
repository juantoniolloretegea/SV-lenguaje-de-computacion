#!/usr/bin/env python3
"""Sensibilidad frente a tres divergencias abiertas; no conformidad ampliada.

La base y las transformaciones reproducen las recetas N0-02/03 y de identidad
de fuente. Un resultado correcto de este banco acredita detección, no reparación.
"""
import argparse
import hashlib
import json
from pathlib import Path
import sys

from oracle_support import (run, assert_success, assert_json_equal, ordered_json,
                            DuplicateJsonMember, OracleError)

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
            # Las tres entradas aún se admiten en el corte. Un cierre posterior
            # debe actualizar este expediente explícitamente; no se tolera cualquier fallo.
            assert_success(py)
            if name != 'semantics_duplicate':
                assert_success(rust)
            elif rust.returncode != 0 or rust.stderr:
                raise OracleError('no se reprodujo la admisión Rust de la sonda duplicada')
            if name == 'control_valid':
                assert_json_equal(py.stdout, rust.stdout)
                if json.loads(rust.stdout)['source_sha256'] != digest(raw):
                    raise OracleError('identidad del control incorrecta')
                record['result'] = 'CONTROL_CONFORME'
            elif name == 'semantics_duplicate':
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
        'schema': 'sv-oracle-sensitivity-v1',
        'scope': 'detector de divergencias conocidas; no cierre de esas divergencias',
        'base_head': '91dc5a3c3b2298ef3fd1b2eefe607f379643e076',
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
    print('Sensibilidad: 1 control válido y 3 divergencias detectadas; 0 divergencias reparadas.')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
