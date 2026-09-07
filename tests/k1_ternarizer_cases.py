"""K1-T: SV declarations are not executable observation-to-Tri contracts.

Fixed sources and independent expectations, using the existing process transport.
Equal/distinct partition names are preservation witnesses, never partition proofs.
"""
import json
from pathlib import Path
from k1_bridge_cases import BASE, prepare as transport
from oracle_support import assert_rejection, assert_success, cli_payload

FIELDS = ['observation_space', 'partition_zero', 'partition_one', 'partition_u', 'mapping']
FORMS = {
    'en': ('ternarizer', FIELDS),
    'es': ('ternarizador', ['espacio_de_observación', 'partición_cero', 'partición_uno', 'partición_u', 'correspondencia']),
}


def declaration(profile, partitions):
    kind, fields = FORMS[profile]
    values = ['O', *partitions, 'Tau']
    return f'{kind} Ter {{ ' + ' '.join(f'{key}: {value};' for key, value in zip(fields, values)) + ' }\n'


def state(profile, value):
    if profile == 'en':
        return f'cellstate State {{ spec: C; vector: [{value},{value},{value},{value},{value},{value},{value},{value},{value}]; }}\n'
    return f'estado_de_celda State {{ especificación: C; vector: [{value},{value},{value},{value},{value},{value},{value},{value},{value}]; }}\n'


def probe(profile, variant):
    if variant == 'literal_u': return BASE[profile] + state(profile, 'U')
    if variant in ['bottom', 'not_admitted']:
        value = ({'bottom': 'Bottom', 'not_admitted': 'NotAdmitted'} if profile == 'en' else
                 {'bottom': 'FalloCaptura', 'not_admitted': 'NoAdmitido'})[variant]
        return BASE[profile] + state(profile, value)
    rhs = {'direct_call': 'Ter(Observation)', 'invented_en': 'ternarize(Observation)',
           'invented_es': 'ternarizar(Observation)', 'projection': 'Ter.mapping'}.get(variant)
    return f'{"let" if profile == "en" else "sea"} Result = {rhs};\n' if rhs else ''


def case(name, profile, variant, other=None, reverse=False):
    partitions = ['Same'] * 3 if variant == 'equal_names' else (['BU', 'B0', 'B1'] if variant == 'permuted' else ['B0', 'B1', 'BU'])
    expected = dict(zip(FIELDS, ['O', *partitions, 'Tau']))
    diagnostic = None
    if variant in ['direct_call', 'invented_en', 'invented_es']: diagnostic = 'Frontend(UnexpectedToken("esperado ., recibido Sym(\'(\')"))'
    if variant == 'projection': diagnostic = 'InvalidProgram("Ter no es un resultado de operación")'
    if variant in ['bottom', 'not_admitted']:
        diagnostic = f'Frontend(InvalidTri("{"Bottom" if variant == "bottom" else "NotAdmitted"}"))'
    # Mixed units preserve independent profile boundaries and both global orders.
    units = [dict(profile=profile, file_name='declaration.svp', source=declaration(profile, partitions))]
    if other is None:
        units[0]['source'] += probe(profile, variant)
    else:
        units.append(dict(profile=other, file_name='consumer.svp', source=probe(other, variant)))
        if reverse: units.reverse()
    return dict(name=name, units=units, diagnostic=diagnostic, expected=expected,
                literal_u=variant == 'literal_u', scope='K1-T_non_executable_boundary')


def sources():
    cases = []
    variants = ['distinct_names', 'equal_names', 'permuted', 'literal_u', 'direct_call',
                'invented_en', 'invented_es', 'projection', 'bottom', 'not_admitted']
    for profile in ['en', 'es']:
        for variant in variants: cases.append(case(f'{profile}_{variant}', profile, variant))
    for profile, other in [('en', 'es'), ('es', 'en')]:
        for reverse in [False, True]:
            for variant in ['distinct_names', 'equal_names', 'literal_u', 'direct_call', 'projection']:
                cases.append(case(f'assembly_{profile}_{int(reverse)}_{variant}', profile, variant, other, reverse))
    assert len(cases) == 40
    return cases


def verify(case, proc):
    if case['diagnostic']:
        actual = assert_rejection(proc, case['diagnostic'])
        if actual != case['diagnostic']: raise AssertionError(f'diagnóstico distinto: {actual}')
        return actual
    assert_success(proc)
    document = json.loads(proc.stdout)
    actual = next(o for o in document['objects'] if o['name'] == 'Ter')
    if actual['fields'] != case['expected']:
        raise AssertionError('Ternarizer: nombres perdidos, reinterpretados o campos fabricados')
    if document['operations']:
        raise AssertionError('una declaración no puede fabricar una operación')
    expected_names = {'Ter', 'K', 'S', 'C', 'State'} if case['literal_u'] else {'Ter'}
    if {o['name'] for o in document['objects']} != expected_names:
        raise AssertionError('objetos o resultados fabricados')
    if case['literal_u']:
        vector = next(o for o in document['objects'] if o['name'] == 'State')['fields']['vector']
        if vector != ['U'] * 9: raise AssertionError('Tri.U explícita perdida o sustituida')
    return cli_payload(proc.stdout).decode('utf-8')


def prepare(native_probe, outdir, wasm_probe=None, wasi_runner=None):
    return transport(native_probe, outdir, wasm_probe, wasi_runner, cases=sources(), verify_case=verify)


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('--native-probe', required=True)
    parser.add_argument('--output-dir', required=True, type=Path)
    parser.add_argument('--wasm-probe')
    parser.add_argument('--wasi-runner')
    args = parser.parse_args()
    if bool(args.wasm_probe) != bool(args.wasi_runner): parser.error('WASI exige módulo y anfitrión')
    cases = prepare(args.native_probe, args.output_dir, args.wasm_probe, args.wasi_runner)
    (args.output_dir / 'report.json').write_text(json.dumps({'schema': 'sv-k1-ternarizer-boundary-v1', 'cases': cases, 'passed': len(cases)}, ensure_ascii=False, indent=2) + '\n')
    print(f'K1-T: {len(cases)}/40, nativo' + (' y WASI' if args.wasm_probe else ''))
