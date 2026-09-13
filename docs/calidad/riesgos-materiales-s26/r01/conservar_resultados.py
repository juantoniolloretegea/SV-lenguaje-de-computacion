"""Conserva una campaña ya terminada; no genera resultados de pruebas.
Uso desde checkout: conservar_resultados.py /campana /comprobantes-publicacion
"""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,shutil,sys
R=Path.cwd();E=R/'docs/calidad/riesgos-materiales-s26/r01';C=Path(sys.argv[1]);P=Path(sys.argv[2])
orders=json.loads((C/'ORDENES.json').read_text());assert len(orders)==3 and all(x['exit_code']==0 for x in orders)
stdout=(C/'02.stdout').read_text();assert '4 passed; 0 failed;' in stdout
assert all('S26-R01-P0'+str(i)+':' in stdout for i in range(1,5))
assert (C/'00.stdout').read_text().startswith('rustc 1.98.0 ')
assert (C/'01.stdout').read_text().startswith('cargo 1.98.0 ')
for p,sha in json.loads((E/'PRECOMPROMISO.json').read_text()).items():assert hashlib.sha256((E/p).read_bytes()).hexdigest()==sha,p
for p,info in json.loads((E/'FUENTES.json').read_text())['archivos'].items():assert hashlib.sha256((R/p).read_bytes()).hexdigest()==info['sha256'],p
assert (C/'mutation/source-replaced.bin').read_bytes()==(E/'fixture/source.bin').read_bytes()+b'\n'
O=E/'evidencia';assert not O.exists();O.mkdir()
for name in ['00.stdout','00.stderr','01.stdout','01.stderr','02.stdout','02.stderr','ORDENES.json']:
    shutil.copyfile(C/name,O/name)
shutil.copyfile(C/'mutation/source-replaced.bin',O/'source-replaced.bin')
commits={}
for label in ['publico','laboratorio']:
    state=json.loads((P/('previo-'+label+'.json')).read_text());assert state['verified'];commits[label]=state['commit']
result={'fecha_registro_utc':datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z'),'precompromiso':commits,'campanas':1,'sondas_aprobadas':4,'sondas_fallidas':0,'casos_globales_s26_cerrados':0,'compilador':'rustc 1.98.0','cargo':'1.98.0','orden':'cargo test --locked --offline --lib -- --nocapture --test-threads=1','advertencias_sv_core':25,'resultados':[{'id':'S26-R01-P01','observado':'Vector, descriptor y recibo completo iguales a testigos custodiados.'},{'id':'S26-R01-P02','observado':'Archivo cambiado con LF; IR canónica igual, hash de fuente distinto; recepción anterior conservada; nueva recepción rechazada I02.'},{'id':'S26-R01-P03','observado':'Capture alterada detectada D06; descriptor original intacto; captura original admitida. No acredita prevención de entrega.'},{'id':'S26-R01-P04','observado':'D01 sin Capture y D07 con indicador post-despacho; sin recibo favorable en ambas llamadas. No se inyectó un fallo físico de adaptador.'}],'limites':['Proceso confiable y archivos locales','No escritores concurrentes durante lectura','No commit durable ni recuperación','No GUI o efecto externo','No adquisición física ni reloj','No protección contra host comprometido'],'fuentes_y_precompromiso_conservados':True}
(E/'RESULTADOS.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
(O/'HUELLAS.json').write_text(json.dumps({p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(O.iterdir())},indent=2)+'\n')
p=E.parent/'INVENTARIO_MATERIAL_R01.md'
p.write_text(p.read_text()+'''\n## Resultado de la campaña R01 / RETP-224

Una campaña nueva: **cuatro sondas aprobadas, cero fallos**, Rust/Cargo 1.98.0,
compilación `--locked --offline` desde target nuevo. Se conservan 25 advertencias
del núcleo. [Resultados](r01/RESULTADOS.json), [stdout](r01/evidencia/02.stdout),
[stderr](r01/evidencia/02.stderr) y [órdenes](r01/evidencia/ORDENES.json).

La sonda P02 cambió efectivamente el archivo de la copia de ensayo después de
recibirlo: el buffer anterior se admitió, la nueva lectura se rechazó en I02 y
el descriptor anterior permaneció igual. Las dos fuentes compilaron con iguales
objetos/operaciones y distintas huellas: se distinguió identidad exacta de
equivalencia canónica. No se modificaron los fixtures históricos.

P03 detectó la copia Capture alterada mediante D06 y conservó el descriptor;
eso no prueba que la entrega previa fuese impedida. P04 verificó D01 y D07 ante
ausencia de captura y un indicador de fallo posterior suministrado al certificador;
no produjo un fallo físico de adaptador. El positivo P01 cotejó vector, descriptor
y recibo completo con sus testigos independientes de la admisión.

Los doce casos globales siguen abiertos. R01 aporta evidencia parcial para
F01/F05/F10/F12. Persistencia, cobertura de índices, recuperación, adquisición,
selección concurrente y representación efectiva conservan sus dependencias.
El siguiente trabajo es recibir estas fronteras en las sedes existentes de
consulta/captura y R2 antes de materializar sus contrastes; S22 continúa con las
fronteras pendientes del montaje y S24 mantiene el relevo secuenciado.
''')
print('Resultados conservados; fuentes y expectativas previas intactas.')
