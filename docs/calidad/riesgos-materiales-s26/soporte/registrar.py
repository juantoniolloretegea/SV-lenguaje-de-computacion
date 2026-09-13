"""Alta documental S26: ejecutar desde la raíz del checkout sobre el corte fijado.
Python administra archivos y CSV; no implementa ni ensaya semántica SV.
"""
from pathlib import Path
from datetime import datetime, timezone
import csv
import io
import json
import subprocess

R = Path.cwd()
BASE = '2fc90a08575484ce7c2af9f3fc544c2249c90c03'
LAB = '500b283729a8930f8c23033df3fe369d5370759b'
assert subprocess.check_output(['git','rev-parse','HEAD'], text=True).strip() == BASE
E = R/'docs/calidad/Inventario-sv/sucesos'
D = R/'docs/calidad/riesgos-materiales-s26'
url = 'https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/'
def read(p):
    with p.open(newline='') as f:
        reader = csv.DictReader(f)
        return reader.fieldnames, list(reader)
def encoded(fields, rows, header=False):
    out = io.StringIO(newline='')
    w = csv.DictWriter(out, fields, lineterminator='\n')
    if header: w.writeheader()
    w.writerows(rows)
    return out.getvalue().encode()
fields, rows = read(E/'SUCESOS_SV.csv')
assert max(int(x['id'][1:]) for x in rows) == 25
t = datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z')
row = dict.fromkeys(fields, '')
row.update(id='S26',estado='en ejecución',fecha_alta_utc=t,fecha_inicio_utc=t,
    fecha_actualizacion_utc=t,unidad_responsable='Watson / W-S0',
    actividad='Riesgos materiales del frame: fallos a provocar y evidencia exigible',
    alcance='Estudio transversal de identidad, consultas, RAM, persistencia, transacciones, índices, recuperación y consumidores, incluida futura GUI. Matriz previa; sin selección tecnológica ni modificación productiva.',
    repositorios_y_ramas='SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente',
    cortes_de_entrada=f'Lenguaje {BASE}; laboratorio {LAB}',
    dependencias='S22 / RETP-218 a 221 como antecedente; S24 como relevo futuro; R2-0, LIG, Frame/R1 y rectores. S26 puede estudiarse mientras S22 permanece abierto.',
    resultado='Alta de seguimiento y matriz documental de doce fallos a provocar, con controles positivos y evidencia exigida. Se distinguen detección, impedimento, fallo observado y resultado no acreditado. Cero casos S26 ejecutados.',
    verificacion='Conservación de registros previos, concordancia CSV/Markdown e historial y enlaces comprobados por soporte/registrar.py. Publicación y espejo se cotejan mediante soporte/publicar.py; sus commits identifican la incorporación.',
    evidencias=url+'docs/calidad/riesgos-materiales-s26/README.md',
    referencia_calidad=url+'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-222',
    siguiente_accion='Inventariar residencia, copias, mutadores, autoridad y fronteras de validación/confirmación/recuperación por recorrido; fijar montaje, estímulos y oráculos antes de ensayar. Entregar pendientes y evidencias al relevo S24.',
    observaciones='Reflexión iniciada antes del alta; fecha actual de seguimiento, sin retrofechar. S22 y S24 conservan estados y secuencia. Doce casos pendientes; no cierre de R2/R3/R4, DFL ni Bis; no se recuentan pruebas anteriores como S26. GUI no seleccionada ni instalada.')
old = {p:p.read_bytes() for p in [E/'SUCESOS_SV.csv', E/'SUCESOS_SV.md', E/'HISTORIAL_SUCESOS_SV.csv', R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv', R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md']}
for p in old:
    assert old[p] == subprocess.check_output(['git','show',BASE+':'+str(p.relative_to(R))])
(E/'SUCESOS_SV.csv').write_bytes(old[E/'SUCESOS_SV.csv']+encoded(fields,[row]))
p=E/'HISTORIAL_SUCESOS_SV.csv'
p.write_bytes(old[p]+encoded(['revision']+fields,[dict(revision=0,**row)]))
p=E/'SUCESOS_SV.md'
block='## S26 · '+row['actividad']+'\n\n'+'\n\n'.join('**'+k+':** '+(v or '—') for k,v in row.items() if k not in ('id','actividad'))+'\n\n'
p.write_bytes(old[p]+block.encode())
p=R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv'
rf, rr=read(p)
assert rr[-1]['ID']=='RETP-2026-221'
values=['RETP-2026-222',t[:10],'','ALTA_Y_MATRIZ_PREVIA','S26',row['resultado'],'Instrucción expresa del autor: registrar origen y destino del estudio adversarial material.','Pilares; perfiles/ensamblaje; transición; R2-0; LIG; Frame/R1; RETP-218 a 221; S24.','Expediente S26; Sucesos CSV/MD e historial; RETP CSV/MD; Léame primero; enlace transversal en estado Bis.',row['verificacion'],'Estudio transversal abierto, con doce criterios previos y relevo trazable.',row['observaciones'],row['siguiente_accion'],row['estado']]
assert len(values)==len(rf)
p.write_bytes(old[p]+encoded(rf,[dict(zip(rf,values))]))
p=R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md'
p.write_bytes(old[p]+('\n\n<a id="retp-222"></a>\n\n### RETP-2026-222 · S26 · Riesgos materiales y evidencia previa\n\n'+t+'. '+row['resultado']+' '+row['siguiente_accion']+' '+row['observaciones']+' [Expediente y matriz](riesgos-materiales-s26/README.md).\n').encode())
p=R/'docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md'
p.write_text(p.read_text()+'\n## Seguimiento transversal S26 / RETP-2026-222\n\n'+
    '[Riesgos materiales del frame y matriz previa](../../riesgos-materiales-s26/README.md): doce fallos a provocar y evidencia exigible para distinguir detección e impedimento. Estudio en ejecución; cero casos S26 ejecutados. Abarca RAM, persistencia, consultas, transacciones y consumidores. S22 continúa y S24 conserva Bis → catálogo y cierre de fase → análisis e instalación de GUI.\n')
p=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/ESTADO_WORKFLOW.json'
s=json.loads(p.read_text()); original=json.loads(p.read_text())
assert 'seguimiento_transversal_s26' not in s
s['seguimiento_transversal_s26']={'id':'S26','estado':'en ejecución','registro':'RETP-2026-222','informe':'../../../riesgos-materiales-s26/README.md','casos_previos':12,'casos_ejecutados':0,'siguiente_accion':row['siguiente_accion'],'relacion':'Estudio transversal; conserva estado y siguiente acción material de S22 y secuencia S24.'}
# La ruta se resuelve desde la carpeta del estado: tres niveles hasta calidad.
assert (p.parent/s['seguimiento_transversal_s26']['informe']).resolve().is_file()
p.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n')
assert {k:v for k,v in json.loads(p.read_text()).items() if k!='seguimiento_transversal_s26'}==original
assert read(E/'SUCESOS_SV.csv')[1]==rows+[row]
assert read(E/'HISTORIAL_SUCESOS_SV.csv')[1][-1]==dict(revision='0',**row)
for p,b in old.items(): assert p.read_bytes().startswith(b)
assert (D/'README.md').read_text().count('| S26-F')==12
(D/'VERIFICACION_ADMINISTRATIVA.json').write_text(json.dumps({'fecha_utc':t,'corte_entrada':BASE,'suceso':'S26','revision':0,'registro':'RETP-2026-222','casos_previos':12,'casos_ejecutados':0,'conservacion_registros_previos':True,'otros_sucesos_intactos':True,'estado_bis_previo_intacto':True,'ensayos_funcionales_nuevos':0},ensure_ascii=False,indent=2)+'\n')
print('S26 / RETP-222: alta documental comprobada',t)
