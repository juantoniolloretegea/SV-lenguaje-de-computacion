"""J-D0 fixtures and preservation witnesses for unresolved Domain bindings.

This module supplies SV source and committed expectations to existing transport.
It does not parse SV or implement the compiler. Acceptance of a pending witness
only preserves its representation; it does not certify a complete domain.
"""
import json
from pathlib import Path
from k1_bridge_cases import prepare as transport
from k1_horizon_cases import BASE, horizon, verify


def capture(profile, name, number):
    if profile == 'en':
        return f'capture_spec {name} {{ parameter_id: {number}; observation_domain: W; observation_space: O; failure_symbol: Bottom; mapping: Phi; }}\n'
    return f'especificación_de_captura {name} {{ identificador_de_parámetro: {number}; dominio_de_observación: W; espacio_de_observación: O; símbolo_de_fallo: FalloCaptura; correspondencia: Phi; }}\n'


def admissibility(profile, name, number):
    if profile == 'en':
        return f'admissibility_spec {name} {{ parameter_id: {number}; states: {{Ok,Degraded,NotAdmitted}}; rule: RAdm; }}\n'
    return f'especificación_de_admisibilidad {name} {{ identificador_de_parámetro: {number}; estados: {{Admitido,Degradado,NoAdmitido}}; regla: RAdm; }}\n'


def dependencies(profile):
    ternarizer = ('ternarizer Ter { observation_space: O; partition_zero: B0; partition_one: B1; partition_u: BU; mapping: Tau; }\n' if profile == 'en' else
        'ternarizador Ter { espacio_de_observación: O; partición_cero: B0; partición_uno: B1; partición_u: BU; correspondencia: Tau; }\n')
    return BASE[profile] + horizon(profile, 'Ev') + capture(profile, 'Cap', 1) + admissibility(profile, 'Adm', 1) + ternarizer


def domain(profile, parameters, name='D', captures='Cap', admissions='Adm'):
    if profile == 'en':
        return f'domain {name} {{ parameters: [{parameters}]; interface: I; horizon: H; capture_specs: [{captures}]; admissibility_specs: [{admissions}]; ternarizers: [Ter]; exogeneity_mask: Exo; silent_u: SU; transduction_policy: TP; u_policy: UP; closure_criterion: Close; }}\n'
    return f'dominio {name} {{ parámetros: [{parameters}]; interfaz: I; horizonte: H; especificaciones_de_captura: [{captures}]; especificaciones_de_admisibilidad: [{admissions}]; ternarizadores: [Ter]; máscara_de_exogeneidad: Exo; u_silenciosa: SU; política_de_transducción: TP; política_de_u: UP; criterio_de_clausura: Close; }}\n'


def sources():
    cases = []
    for profile in ['en', 'es']:
        for variant in ['order_local', 'duplicate', 'empty_pending', 'cardinality_pending',
                        'capture_id_pending', 'admissibility_id_pending', 'both_ids_pending', 'chain_mismatch']:
            parameters = 'B,A,Aa,AA' if variant == 'order_local' else ('B,A,B' if variant == 'duplicate' else ('' if variant == 'empty_pending' else 'P'))
            prefix = dependencies(profile)
            captures, admissions = 'Cap', 'Adm'
            expected = {}
            if variant in ['cardinality_pending', 'capture_id_pending', 'both_ids_pending']:
                number = 2 if variant == 'cardinality_pending' else 1
                prefix += capture(profile, 'CapOther', number)
                captures = 'CapOther,Cap'
                expected['CapOther'] = {'parameter_id': number}
            if variant in ['cardinality_pending', 'admissibility_id_pending', 'both_ids_pending', 'chain_mismatch']:
                number = 2 if variant in ['cardinality_pending', 'chain_mismatch'] else 1
                prefix += admissibility(profile, 'AdmOther', number)
                admissions = 'AdmOther' if variant == 'chain_mismatch' else 'AdmOther,Adm'
                expected['AdmOther'] = {'parameter_id': number}
            diagnostic = ('parámetro nominal repetido: B' if variant == 'duplicate' else
                          'parameter_id incompatibles' if variant == 'chain_mismatch' else None)
            expected['D'] = {'parameters': parameters.split(',') if parameters else [],
                             'capture_specs': captures.split(','), 'admissibility_specs': admissions.split(',')}
            source = prefix + domain(profile, parameters, captures=captures, admissions=admissions)
            if variant == 'order_local':
                source += domain(profile, 'AA,B', name='Other')
                expected['Other'] = {'parameters': ['AA', 'B']}
            cases.append(dict(name=f'{profile}_{variant}', units=[dict(profile=profile, file_name='domain.svp', source=source)],
                              diagnostic=diagnostic, expected=expected,
                              scope='pending_binding_representation' if variant.endswith('_pending') else 'J-D0_or_previous_guard'))
    for base_profile, domain_profile in [('en', 'es'), ('es', 'en')]:
        for reverse in [False, True]:
            for repeated in [False, True]:
                parameters = 'B,A,B' if repeated else 'B,A'
                units = [dict(profile=base_profile, file_name='base.svp', source=dependencies(base_profile)),
                         dict(profile=domain_profile, file_name='domain.svp', source=domain(domain_profile, parameters))]
                if reverse:
                    units.reverse()
                cases.append(dict(name=f'assembly_{base_profile}_{int(reverse)}_{int(repeated)}', units=units,
                                  diagnostic='parámetro nominal repetido: B' if repeated else None,
                                  expected={'D': {'parameters': ['B', 'A']}}, scope='J-D0'))
    assert len(cases) == 24
    return cases


def verify_domain(case, proc):
    if case['diagnostic']:
        from oracle_support import assert_rejection
        actual = assert_rejection(proc, case['diagnostic'])
        expected = f'InvalidProgram("Domain D: {case["diagnostic"]}")'
        if actual != expected:
            raise AssertionError(f'diagnóstico incorrecto: {actual}')
        return actual
    return verify(case, proc)


def prepare(native_probe, outdir, wasm_probe=None, wasi_runner=None):
    return transport(native_probe, outdir, wasm_probe, wasi_runner, cases=sources(), verify_case=verify_domain)


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('--native-probe', required=True)
    parser.add_argument('--output-dir', required=True, type=Path)
    parser.add_argument('--wasm-probe')
    parser.add_argument('--wasi-runner')
    args = parser.parse_args()
    if bool(args.wasm_probe) != bool(args.wasi_runner):
        parser.error('WASI exige módulo y anfitrión')
    cases = prepare(args.native_probe, args.output_dir, args.wasm_probe, args.wasi_runner)
    (args.output_dir / 'report.json').write_text(json.dumps({'schema': 'sv-k1-domain-names-v1', 'cases': cases, 'passed': len(cases)}, ensure_ascii=False, indent=2) + '\n')
    print(f'J-D0 y límites de representación: {len(cases)}/24, nativo' + (' y WASI' if args.wasm_probe else ''))
