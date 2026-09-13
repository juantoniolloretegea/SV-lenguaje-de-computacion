"""Conservar resultados de R03 sin modificar banco, sondas ni fixture.
Uso desde checkout: conservar.py /campana /comprobantes_publicacion
No reinterpreta semántica SV; exige las tres aserciones Rust comprometidas.
"""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,shutil,sys
E=Path(__file__).resolve().parent
O=Path(sys.argv[1]);C=Path(sys.argv[2]);D=E/'evidencia'
assert not D.exists()
pre=json.loads((C/'previo-publico.json').read_text());lab=json.loads((C/'previo-laboratorio.json').read_text())
assert pre['verified'] and lab['verified']
for name,h in json.loads((E/'PRECOMPROMISO.json').read_text()).items():
    assert hashlib.sha256((E/name).read_bytes()).hexdigest()==h,name
orders=json.loads((O/'ORDENES.json').read_text())
assert len(orders)==3 and all(x['exit_code']==0 for x in orders)
assert (O/'00.stdout').read_text().startswith('rustc 1.98.0 ')
assert (O/'01.stdout').read_text().startswith('cargo 1.98.0 ')
stdout=(O/'02.stdout').read_text()
assert '3 passed; 0 failed;' in stdout
for name in ['t01_fixed_expectation_and_replaced_custody','t02_mixed_read_at_explicit_barrier','t07_copied_after_vs_observed_after']:
    assert 'tests::'+name in stdout,name
D.mkdir()
for p in list(O.glob('*.stdout'))+list(O.glob('*.stderr'))+[O/'ORDENES.json']:
    shutil.copyfile(p,D/p.name)
shutil.copytree(O/'mutation',D/'observaciones')
trace=json.loads((D/'observaciones/T02-B-trace.json').read_text())
assert trace['injected'] is True and trace['source_version']=='B'
copied=json.loads((D/'observaciones/T07-copied-after.json').read_text())
reread=json.loads((D/'observaciones/T07-reread-after.json').read_text())
assert copied['estado_antes']==copied['estado_despues']
assert reread['estado_antes']!=reread['estado_despues']
assert bytes(reread['estado_despues'])==(D/'observaciones/T07-state.bin').read_bytes()
hashes={str(p.relative_to(D)):hashlib.sha256(p.read_bytes()).hexdigest() for p in D.rglob('*') if p.is_file()}
(D/'HUELLAS.json').write_text(json.dumps(hashes,ensure_ascii=False,indent=2)+'\n')
result=dict(version='S26-R03-RESULTADOS/1',fecha_registro_utc=datetime.now(timezone.utc).isoformat(),
    precompromiso_lenguaje=pre['commit'],precompromiso_laboratorio=lab['commit'],
    campanas=1,sondas_aprobadas=3,sondas_fallidas=0,casos_globales_cerrados=0,
    resultado='Tres sondas conformes a sus oráculos, incluido límite reproducido en T07.',
    t01='A/A y B/B certificados contra recibos íntegros; B/A rechazado I02 por full identity/custody mismatch.',
    t02='Archivo B leído tras metadatos A; rechazo I02 source identity; control sin inyección admitido.',
    t07='Archivo alterado con after copiado recibe conformidad del observador histórico; after releído provoca exactamente preservacion real del estado. Control intacto conforme.',
    limite='No autenticación externa de custodia, transacción durable, GUI, fallo físico, aislamiento ni concurrencia general acreditados.',
    siguiente='Integrar recepción instrumentada del testigo posterior mediante variante y oráculos propios; no reescribir evidencia anterior.')
(E/'RESULTADOS.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
with (E/'README.md').open('a') as f:
    f.write('\n## Resultado / RETP-227\n\nUna campaña Rust/Cargo 1.98.0, tres sondas conformes '
            'a sus oráculos y cero fallos de aserción. Precompromiso público `'+pre['commit']+'`. '
            'Los archivos congelados permanecen idénticos.\n\n'
            '**T07 reproduce una limitación material:** el observador existente devuelve conformidad '
            'si recibe como `after` una copia inicial, aunque el archivo de ensayo haya cambiado. '
            'Al suministrarle la relectura efectiva detecta exactamente la pérdida de preservación. '
            'La comparación funciona sobre sus entradas; no acredita de dónde se obtuvieron. '
            'No se atribuye este archivo de ensayo a una BD ni a la memoria interna del núcleo.\n\n'
            'T01 distingue B bajo expectativa A de B bajo su custodia declarada; este último positivo '
            'no prueba legitimidad externa del custodio. T02 rechaza la mezcla observada en el punto '
            'de lectura instrumentado. Son alcances parciales, no cierres globales de S26.\n\n'
            '[Resultados](RESULTADOS.json), [salida Rust](evidencia/02.stdout), '
            '[diagnósticos de compilación](evidencia/02.stderr), [órdenes](evidencia/ORDENES.json) '
            'y [huellas](evidencia/HUELLAS.json). Se conservan el testigo copiado y el releído '
            'en evidencia/observaciones.\n\n'
            'El siguiente incremento deberá cualificar la recepción del testigo posterior en '
            'el conductor con una variante identificada y nuevos oráculos. El montaje histórico '
            'permanece sin modificar; estas sondas no acreditan aún esa integración.\n')
print(json.dumps(result,ensure_ascii=False))
