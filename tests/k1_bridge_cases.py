"""Synthetic SV fixtures for J-B0; no parser, lowering or SV implementation."""
import json
from pathlib import Path
from oracle_support import run, assert_success, assert_rejection, cli_payload

BASE = {
    'en': 'codomain K = { A };\noutput_semantics S { A -> "a"; }\ncellspec C { b: 3; codomain: K; semantics: S; role: Base; }\n',
    'es': 'codominio K = { A };\nsemántica_de_salida S { A -> "a"; }\nespecificación_de_celda C { b: 3; codominio: K; semántica: S; rol: Base; }\n',
}
def coupled(profile, positions):
    return (f'coupledspec CC {{ cell: C; bridges: [{positions}]; }}\n' if profile == 'en'
            else f'especificación_acoplada CC {{ celda: C; puentes: [{positions}]; }}\n')

def sources():
    cases = []
    variants = [('empty', '', [], None), ('order', '9,1,3', [9,1,3], None),
                ('duplicate', '3,3', None, 'posición puente repetida: 3'),
                ('canonical_nat', '03,3', None, 'posición puente repetida: 3'),
                ('range_first', '3,3,10', None, 'posición puente fuera de rango')]
    for profile in ['en','es']:
        for name, positions, expected, diagnostic in variants:
            cases.append(dict(name=f'{profile}_{name}', units=[dict(profile=profile, file_name='one.svp', source=BASE[profile]+coupled(profile,positions))], bridges=expected, diagnostic=diagnostic))
    for base_profile, cc_profile in [('en','es'),('es','en')]:
        for reverse in [False, True]:
            for name, positions, expected, diagnostic in [variants[1], variants[3]]:
                units=[dict(profile=base_profile, file_name='base.svp', source=BASE[base_profile]),
                       dict(profile=cc_profile, file_name='coupled.svp', source=coupled(cc_profile,positions))]
                if reverse: units.reverse()
                cases.append(dict(name=f'assembly_{base_profile}_{int(reverse)}_{name}', units=units, bridges=expected, diagnostic=diagnostic))
    assert len(cases) == 18
    return cases

def verify(case, proc):
    if case['diagnostic']:
        diagnostic = assert_rejection(proc, case['diagnostic'])
        expected = f'InvalidProgram("CoupledSpec CC: {case["diagnostic"]}")'
        if diagnostic != expected:
            raise AssertionError(f'diagnóstico distinto: {diagnostic!r}')
        return diagnostic
    assert_success(proc)
    document = json.loads(proc.stdout)
    # The prescribed order/empty set is independent of the implementation.
    actual = next(o for o in document['objects'] if o['name'] == 'CC')['fields']['bridges']
    if actual != case['bridges']:
        raise AssertionError(f'BridgeSet perdido o reordenado: {actual!r}')
    return cli_payload(proc.stdout).decode('utf-8')

def prepare(native_probe, outdir, wasm_probe=None, wasi_runner=None, *, cases=None, verify_case=None):
    # Shared process/byte transport; each bank supplies its own SV obligations.
    if cases is None: cases = sources()
    if verify_case is None: verify_case = verify
    outdir = Path(outdir).resolve(); outdir.mkdir(parents=True, exist_ok=True)
    result = []
    for case in cases:
        directory = outdir / case['name']; directory.mkdir(exist_ok=True)
        arguments = []
        for unit in case['units']:
            path = directory / unit['file_name']; path.write_bytes(unit['source'].encode('utf-8'))
            arguments += ['--profile', unit['profile'], str(path)]
        native = run([str(Path(native_probe).resolve()), *arguments])
        payload = verify_case(case, native)
        (directory / 'native.stdout').write_bytes(native.stdout)
        (directory / 'native.stderr').write_bytes(native.stderr)
        record = dict(case, expected_payload=payload, error=bool(case['diagnostic']), native_command=native.args)
        if wasm_probe:
            wasi = run(['node','--no-warnings',str(Path(wasi_runner).resolve()),str(Path(wasm_probe).resolve()),*arguments])
            verify_case(case, wasi)
            (directory / 'wasi.stdout').write_bytes(wasi.stdout)
            (directory / 'wasi.stderr').write_bytes(wasi.stderr)
            if (native.returncode,native.stdout,native.stderr) != (wasi.returncode,wasi.stdout,wasi.stderr):
                raise AssertionError(f'{case["name"]}: divergencia nativo/WASI')
            record['wasi_command'] = wasi.args
        result.append(record)
    return result

if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('--native-probe',required=True)
    parser.add_argument('--output-dir',required=True,type=Path)
    parser.add_argument('--wasm-probe')
    parser.add_argument('--wasi-runner')
    args=parser.parse_args()
    if bool(args.wasm_probe) != bool(args.wasi_runner): parser.error('WASI exige módulo y anfitrión')
    cases=prepare(args.native_probe,args.output_dir,args.wasm_probe,args.wasi_runner)
    (args.output_dir/'report.json').write_text(json.dumps({'schema':'sv-k1-bridge-set-v1','cases':cases,'passed':len(cases)},ensure_ascii=False,indent=2)+'\n')
    print(f'J-B0: {len(cases)}/18, nativo'+(' y WASI' if args.wasm_probe else ''))
