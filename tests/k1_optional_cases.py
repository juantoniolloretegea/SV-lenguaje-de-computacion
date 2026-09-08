"""Fixed SV sources for optional-field grammar; process observation only.

Expectations follow Grammar 0.1 sections 5.4-5.5, retained by 0.2.
No SV parser, compiler, or inferred domain interpretation lives here.
"""
import json
from pathlib import Path
from k1_bridge_cases import prepare as transport
from k1_horizon_cases import BASE, horizon
from oracle_support import assert_rejection, assert_success, cli_payload

FORMS = {
    'en': dict(relation='semantic_relation', pattern='pattern', kind='kind',
               relation_kind='DeclaredRelation', pattern_kind='DeclaredPattern',
               table='table', arity='arity', constraints='constraints'),
    'es': dict(relation='relación_semántica', pattern='patrón', kind='clase',
               relation_kind='RelaciónDeclarada', pattern_kind='PatrónDeclarado',
               table='tabla', arity='aridad', constraints='restricciones'),
}


def declaration(profile, kind, fields, name='Probe'):
    f = FORMS[profile]
    body = ' '.join(f'{f[key]}: {value};' for key, value in fields)
    return f'{f[kind]} {name} {{ {f["kind"]}: {f[kind + "_kind"]}; {body} }}\n'


def variants(kind):
    first, value, other, native_value = ('table', 'TabA', 'TabB', 'TabA') if kind == 'relation' else ('arity', '2', '3', 2)
    return [
        ('none', [], None, {}),
        ('first', [(first, value)], None, {first: native_value}),
        ('last_empty', [('constraints', '[]')], None, {'constraints': []}),
        ('both', [(first, value), ('constraints', '[B,A,B]')], None, {first: native_value, 'constraints': ['B', 'A', 'B']}),
        ('first_same', [(first, value), (first, value)], 'repetido: ' + first, None),
        ('first_different', [(first, value), (first, other)], 'repetido: ' + first, None),
        ('last_empty_repeat', [('constraints', '[]'), ('constraints', '[B]')], 'repetido: constraints', None),
        ('last_repeat', [(first, value), ('constraints', '[A]'), ('constraints', '[B]')], 'repetido: constraints', None),
        ('reverse', [('constraints', '[B]'), (first, value)], 'fuera de orden: ' + first, None),
    ]


def evolution(profile, family, mode):
    if profile == 'en':
        metadata = '' if family != 'metadata' or mode == 'omitted' else 'metadata: [B,A,B];'
        if family == 'metadata' and mode == 'repeated': metadata += ' metadata: [];'
        source = BASE[profile] + horizon(profile, 'Ev') + f'transition_data TD {{ horizon_ref: H; events: [(Ev,One)]; induced_parameters: [(CC,3,One)]; {metadata} }}\n'
        for i in [0, 1]:
            source += f'frame F{i} {{ index: {i}; architecture: G; cell_states: []; eval_results: []; gate_results: []; supervision: []; criticalities: []; }}\n'
        transition = '' if mode == 'omitted' else ', transition: TD'
        if mode == 'repeated': transition += ', transition: TD'
        entries = f'entry(frame: F0{transition})' + (', entry(frame: F1)' if mode != 'omitted' else '')
        if family == 'transition': source += f'trajectory Tr {{ entries: [{entries}]; }}\n'
    else:
        metadata = '' if family != 'metadata' or mode == 'omitted' else 'metadatos: [B,A,B];'
        if family == 'metadata' and mode == 'repeated': metadata += ' metadatos: [];'
        source = BASE[profile] + horizon(profile, 'Ev') + f'datos_de_transición TD {{ referencia_de_horizonte: H; sucesos: [(Ev,Uno)]; parámetros_inducidos: [(CC,3,Uno)]; {metadata} }}\n'
        for i in [0, 1]:
            source += f'marco F{i} {{ índice: {i}; arquitectura: G; estados_de_celda: []; resultados_de_evaluación: []; resultados_de_compuerta: []; supervisión: []; criticidades: []; }}\n'
        transition = '' if mode == 'omitted' else ', transición: TD'
        if mode == 'repeated': transition += ', transición: TD'
        entries = f'entrada(marco: F0{transition})' + (', entrada(marco: F1)' if mode != 'omitted' else '')
        if family == 'transition': source += f'trayectoria Tr {{ entradas_de_trayectoria: [{entries}]; }}\n'
    return source


def sources():
    cases = []
    for profile in ['en', 'es']:
        for kind in ['relation', 'pattern']:
            for name, fields, error, expected in variants(kind):
                label = 'SemanticRelation' if kind == 'relation' else 'Pattern'
                diagnostic = f'Frontend(UnexpectedToken("{label} Probe: campo opcional {error}"))' if error else None
                cases.append(dict(name=f'{profile}_{kind}_{name}', diagnostic=diagnostic, expected={'Probe': expected} if expected is not None else {},
                                  units=[dict(profile=profile, file_name='optional.svp', source=declaration(profile, kind, fields))], exact_fields=True))
        for family in ['metadata', 'transition']:
            for mode in ['omitted', 'present', 'repeated']:
                expected = {'TD': {'metadata': ['B', 'A', 'B']}} if family == 'metadata' and mode == 'present' else {}
                absent = {'TD': ['metadata']} if family == 'metadata' and mode == 'omitted' else {}
                if family == 'transition' and mode != 'repeated':
                    expected = {'Tr': {'entries': [{'frame': 'F0'}] if mode == 'omitted' else [{'frame': 'F0', 'transition': 'TD'}, {'frame': 'F1'}]}}
                diagnostic = ('esperado }, recibido Word' if family == 'metadata' else "esperado ), recibido Sym(',')") if mode == 'repeated' else None
                cases.append(dict(name=f'{profile}_{family}_{mode}', diagnostic=diagnostic, expected=expected, absent=absent,
                                  units=[dict(profile=profile, file_name='evolution.svp', source=evolution(profile, family, mode))]))
    for profile, other in [('en', 'es'), ('es', 'en')]:
        for kind in ['relation', 'pattern']:
            for reverse in [False, True]:
                for index in [3, 5]:
                    name, fields, error, expected = variants(kind)[index]
                    label = 'SemanticRelation' if kind == 'relation' else 'Pattern'
                    units = [dict(profile=other, file_name='base.svp', source=declaration(other, kind, fields[:1], 'BaseProbe')),
                             dict(profile=profile, file_name='optional.svp', source=declaration(profile, kind, fields))]
                    if reverse: units.reverse()
                    cases.append(dict(name=f'assembly_{profile}_{kind}_{int(reverse)}_{name}', units=units, exact_fields=True,
                                      expected={'Probe': expected} if expected is not None else {},
                                      diagnostic=f'Frontend(UnexpectedToken("{label} Probe: campo opcional {error}"))' if error else None))
    assert len(cases) == 64
    return cases


def verify(case, proc):
    if case['diagnostic']:
        actual = assert_rejection(proc, case['diagnostic'])
        if case['diagnostic'].startswith('Frontend(') and actual != case['diagnostic']:
            raise AssertionError(f'diagnóstico distinto: {actual}')
        return actual
    assert_success(proc)
    objects = {o['name']: o['fields'] for o in json.loads(proc.stdout)['objects']}
    for name, fields in case['expected'].items():
        actual = objects[name]
        if case.get('exact_fields'):
            actual = {key: value for key, value in actual.items() if key != 'kind'}
            if actual != fields: raise AssertionError(f'{name}: campo perdido, añadido o sobrescrito')
        else:
            for key, value in fields.items():
                if actual[key] != value: raise AssertionError(f'{name}.{key}: pérdida u orden incorrecto')
    for name, fields in case.get('absent', {}).items():
        if any(key in objects[name] for key in fields): raise AssertionError('campo omitido fabricado')
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
    (args.output_dir / 'report.json').write_text(json.dumps({'schema': 'sv-k1-optional-fields-v1', 'cases': cases, 'passed': len(cases)}, ensure_ascii=False, indent=2) + '\n')
    print(f'DFL-010: {len(cases)}/64, nativo' + (' y WASI' if args.wasm_probe else ''))
