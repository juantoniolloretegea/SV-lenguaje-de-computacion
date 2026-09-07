#!/usr/bin/env python3
"""Contrato CLI nativo: fuente SV, stdout, rechazo y fallo de uso separados."""
import argparse
from pathlib import Path
from oracle_support import run, assert_success, assert_json_equal, assert_rust_rejection

ROOT = Path(__file__).resolve().parents[1]

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--rust-bin', type=Path, required=True)
    args = parser.parse_args()
    binary = str(args.rust_bin.resolve())
    valid = ROOT / 'tests/conformance/valid/cell_basic.svp'
    proc = run([binary, str(valid)])
    assert_success(proc)
    assert_json_equal(proc.stdout, valid.with_suffix('.expected.json').read_bytes())
    bad = run([binary, str(ROOT / 'tests/conformance/invalid/u_coercion.svp')])
    assert_rust_rejection(bad, 'u_coercion')
    for arguments in [[], ['--profile', 'xx', str(valid)], [str(valid), '-o', 'unused.json']]:
        misuse = run([binary, *arguments])
        assert misuse.returncode == 2 and not misuse.stdout and misuse.stderr
    print('CLI nativa: admisión, rechazo y uso incorrecto separados; -o retirado con la CLI Python.')
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
