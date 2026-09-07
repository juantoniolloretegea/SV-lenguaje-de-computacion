"""J-H1 fixtures: declared types, local identity and separate transition data."""
import json
from pathlib import Path
from k1_bridge_cases import prepare as transport
from oracle_support import assert_success, assert_rejection, cli_payload

BASE = {
 'en': 'codomain K = { A }; output_semantics S { A -> "a"; } cellspec C { b: 3; codomain: K; semantics: S; role: Base; } coupledspec CC { cell: C; bridges: [3]; } semantic_relation R { kind: DeclaredRelation; constraints: [Local]; } graph G { nodes: [CC]; edges: []; relation: R; regime: Simple; }\n',
 'es': 'codominio K = { A }; semántica_de_salida S { A -> "a"; } especificación_de_celda C { b: 3; codominio: K; semántica: S; rol: Base; } especificación_acoplada CC { celda: C; puentes: [3]; } relación_semántica R { clase: RelaciónDeclarada; restricciones: [Local]; } grafo G { nodos: [CC]; aristas: []; relación: R; régimen: Simple; }\n',
}
def horizon(profile, events, name='H'):
 return (f'horizon {name} {{ architecture: G; events: [{events}]; }}\n' if profile=='en' else
         f'horizonte {name} {{ arquitectura: G; sucesos: [{events}]; }}\n')

def transitions(profile):
 if profile=='en':
  return 'transition_data TD1 { horizon_ref: H; events: [(B,One)]; induced_parameters: [(C,3,One)]; } transition_data TD2 { horizon_ref: H; events: [(B,U)]; induced_parameters: [(C,3,U)]; }\n'
 return 'datos_de_transición TD1 { referencia_de_horizonte: H; sucesos: [(B,Uno)]; parámetros_inducidos: [(C,3,Uno)]; } datos_de_transición TD2 { referencia_de_horizonte: H; sucesos: [(B,U)]; parámetros_inducidos: [(C,3,U)]; }\n'

def sources():
 result=[]
 for profile in ['en','es']:
  variants=[('order','B,A',None),('duplicate','B,A,B','tipo de suceso repetido: B'),('empty','','definición incompleta'),('distinct_names','B,Aa,AA',None),('local_identity','B,A',None),('separate_transitions','B,A',None)]
  for name, events, diagnostic in variants:
   source=BASE[profile]+horizon(profile,events)
   expected={'H':{'architecture':'G','events':events.split(',') if events else []}}
   if name=='local_identity':
    source+=horizon(profile,'A,B','Other');expected['Other']={'architecture':'G','events':['A','B']}
   if name=='separate_transitions':
    source+=transitions(profile)
    expected.update({'TD1':{'events':[{'event_type':'B','state':'One'}]},'TD2':{'events':[{'event_type':'B','state':'U'}]}})
   result.append(dict(name=f'{profile}_{name}',units=[dict(profile=profile,file_name='horizon.svp',source=source)],diagnostic=diagnostic,expected=expected))
 for base_profile,h_profile in [('en','es'),('es','en')]:
  for reverse in [False,True]:
   for repeated in [False,True]:
    events='B,A,B' if repeated else 'B,A'
    units=[dict(profile=base_profile,file_name='graph.svp',source=BASE[base_profile]),dict(profile=h_profile,file_name='horizon.svp',source=horizon(h_profile,events))]
    if reverse:units.reverse()
    result.append(dict(name=f'assembly_{base_profile}_{int(reverse)}_{int(repeated)}',units=units,diagnostic='tipo de suceso repetido: B' if repeated else None,expected={'H':{'architecture':'G','events':['B','A']}}))
 assert len(result)==20
 return result

def verify(case,proc):
 if case['diagnostic']:
  actual=assert_rejection(proc,case['diagnostic'])
  expected=f'InvalidProgram("Horizon H: {case["diagnostic"]}")'
  if actual!=expected:raise AssertionError(f'diagnóstico incorrecto: {actual}')
  return actual
 assert_success(proc)
 objects={o['name']:o['fields'] for o in json.loads(proc.stdout)['objects']}
 for name,fields in case['expected'].items():
  for field,value in fields.items():
   if objects[name][field]!=value:raise AssertionError(f'{name}.{field}: pérdida o reordenación')
 return cli_payload(proc.stdout).decode('utf-8')

def prepare(native_probe,outdir,wasm_probe=None,wasi_runner=None):
 return transport(native_probe,outdir,wasm_probe,wasi_runner,cases=sources(),verify_case=verify)

if __name__=='__main__':
 import argparse
 p=argparse.ArgumentParser();p.add_argument('--native-probe',required=True);p.add_argument('--output-dir',required=True,type=Path);p.add_argument('--wasm-probe');p.add_argument('--wasi-runner');a=p.parse_args()
 if bool(a.wasm_probe)!=bool(a.wasi_runner):p.error('WASI exige módulo y anfitrión')
 cases=prepare(a.native_probe,a.output_dir,a.wasm_probe,a.wasi_runner)
 (a.output_dir/'report.json').write_text(json.dumps({'schema':'sv-k1-horizon-types-v1','cases':cases,'passed':len(cases)},ensure_ascii=False,indent=2)+'\n')
 print(f'J-H1: {len(cases)}/20, nativo'+(' y WASI' if a.wasm_probe else ''))
