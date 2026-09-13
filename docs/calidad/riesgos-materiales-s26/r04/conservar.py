"""Conservación de R04, sin modificar banco ni variante congelados.
Uso: conservar.py /campana /comprobantes_publicacion, desde checkout.
"""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,shutil,sys
E=Path(__file__).resolve().parent;O=Path(sys.argv[1]);C=Path(sys.argv[2]);D=E/'evidencia'
assert not D.exists()
pre=json.loads((C/'previo-publico.json').read_text());lab=json.loads((C/'previo-laboratorio.json').read_text())
assert pre['verified'] and lab['verified']
for n,h in json.loads((E/'PRECOMPROMISO.json').read_text()).items():assert hashlib.sha256((E/n).read_bytes()).hexdigest()==h,n
orders=json.loads((O/'ORDENES.json').read_text());assert len(orders)==3 and all(x['exit_code']==0 for x in orders)
assert (O/'00.stdout').read_text().startswith('rustc 1.98.0 ')
assert (O/'01.stdout').read_text().startswith('cargo 1.98.0 ')
assert '8 passed; 0 failed;' in (O/'02.stdout').read_text()
bank=json.loads((E/'BANCO_PREVIO.json').read_text())
observations={}
for case in bank['casos']:
    j=json.loads((O/'mutation'/(case['id']+'.json')).read_text())
    assert j['preservacion']==case['preservacion_esperada']
    delivery=j['entrega_previa'];assert (delivery['intentos_despacho'] if delivery else 0)==case['intentos_esperados']
    if delivery:assert delivery['recibo'] is not None
    observations[case['id']]=j
p3=observations['P03']
assert json.loads(bytes(p3['despues']['contenido']))['vector'][0]=='One'
assert p3['entrega_previa']['vector_admitido'][0]=='Zero'
assert observations['P04']['despues']['etapa']=='open'
assert observations['P08']['despues']['etapa']=='read'
assert observations['P06']['despues']['bytes_leidos']==1025
D.mkdir()
for p in list(O.glob('*.stdout'))+list(O.glob('*.stderr'))+[O/'ORDENES.json']:
    shutil.copyfile(p,D/p.name)
Q=D/'observaciones';Q.mkdir();paths={}
for case in bank['casos']:
    ident=case['id'];shutil.copyfile(O/'mutation'/(ident+'.json'),Q/(ident+'.json'))
    state=O/'mutation'/ident/'state.bin'
    paths[ident]='archivo' if state.is_file() else 'directorio' if state.is_dir() else 'ausente'
    if state.is_file():shutil.copyfile(state,Q/(ident+'-state.bin'))
(Q/'ESTADO_RUTAS.json').write_text(json.dumps(paths,ensure_ascii=False,indent=2)+'\n')
hashes={str(p.relative_to(D)):hashlib.sha256(p.read_bytes()).hexdigest() for p in D.rglob('*') if p.is_file()}
(D/'HUELLAS.json').write_text(json.dumps(hashes,indent=2)+'\n')
result=dict(version='S26-R04-RESULTADOS/1',fecha_registro_utc=datetime.now(timezone.utc).isoformat(),
    precompromiso_lenguaje=pre['commit'],precompromiso_laboratorio=lab['commit'],
    campanas=1,sondas_aprobadas=8,sondas_fallidas=0,casos_globales_cerrados=0,
    resultado='Relectura instrumentada cualificada en ocho escenarios del recorrido local.',
    hallazgo='P03: archivo posterior contiene One; objeto admitido y entrega previos conservan Zero. Diferencia detectada y no atribuida retrospectivamente a la entrega.',
    limites=['Comparador heredado aún acepta copia falsa; recepción local no autentica al host.',
             'Sin transacción durable, recuperación, GUI o bloqueo de mutaciones intermedias acreditados.',
             'Pánico, interrupción y sustitución durante lectura pendientes; sólo retorno normal del recorrido acotado.'],
    casos=[dict(id=x['id'],preservacion=observations[x['id']]['preservacion'],intentos=x['intentos_esperados']) for x in bank['casos']])
(E/'RESULTADOS.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
with (E/'README.md').open('a') as f:
    f.write('\n## Resultado / RETP-229\n\nOcho sondas conformes a sus oráculos, cero fallos de aserción, '
            'una campaña Rust/Cargo 1.98.0 offline. Precompromiso público `'+pre['commit']+'`. '
            'Fuentes, banco y código congelados conservados.\n\n'
            'P01 admite el control intacto. P02 detecta bytes distintos; P03 detecta el cambio '
            'del primer componente del archivo a One mientras el vector admitido y la entrega '
            'anteriores conservan Zero. El informe distingue esa secuencia: no reescribe la '
            'entrega ni presenta el archivo posterior como su contenido original.\n\n'
            'P04/P08 conservan entrega previa y muestran fallos de apertura/lectura posteriores. '
            'P06 observa 1025 bytes y declara exceso sin recorrer los 2048. P05/P07 detienen '
            'la operación ante fallo o alteración inicial, con cero intentos de despacho. '
            'El control que sustituye after por before sigue haciendo pasar al observador '
            'heredado: se conserva ese límite y no se confunde la recepción ordinaria corregida '
            'con resistencia a evidencia falseada por el mismo proceso.\n\n'
            '[Resultados](RESULTADOS.json), [stdout](evidencia/02.stdout), '
            '[stderr](evidencia/02.stderr), [órdenes](evidencia/ORDENES.json) y '
            '[huellas](evidencia/HUELLAS.json). Las observaciones y archivos posteriores '
            'están en evidencia/observaciones, con inventario de rutas ausentes/directorio.\n\n'
            'Se cualifica esta variante local, no el ejecutable completo ni todos los casos Bis. '
            'El siguiente incremento debe recibir las salidas restantes y los fallos que impiden '
            'el retorno normal, y concretar sustitución durante lectura. S26 sigue abierto; '
            'R2 y el relevo de GUI conservan sus dependencias.\n')
print(json.dumps(result,ensure_ascii=False))
