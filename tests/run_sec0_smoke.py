#!/usr/bin/env python3
"""Conserva los tres testigos SEC-0B sobre fuentes DSL y el ejecutable nativo."""
import argparse
from pathlib import Path
from oracle_support import run, assert_success, assert_bytes_equal, assert_rejection

ROOT = Path(__file__).resolve().parents[1]

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--rust-bin', type=Path, required=True)
    args = parser.parse_args()
    binary = str(args.rust_bin.resolve())
    for relative in ['tests/adversarial/long_identifier_valid.svp',
                     'tests/conformance/valid/query_context_all_variants.svp']:
        command = [binary, str(ROOT / relative)]
        first, second = run(command), run(command)
        assert_success(first)
        assert_success(second)
        assert_bytes_equal(first.stdout, second.stdout)
    bad = run([binary, str(ROOT / 'tests/adversarial/duplicate_name_invalid.svp')])
    assert_rejection(bad, 'identificador duplicado: DUPLICATE_CELL')
    print('SEC-0B: 3/3 testigos conservados sobre SV nativo.')
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
