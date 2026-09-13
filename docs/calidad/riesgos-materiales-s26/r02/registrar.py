"""Recepción documental S26 R02 / RETP-225. Ejecutar desde raíz del checkout.
Ediciones explícitas; no modifica código SV ni ejecuta sondas de comportamiento.
"""
from pathlib import Path
from datetime import datetime, timezone
import csv
import hashlib
import io
import json
import re

R = Path.cwd()
E = R / 'docs/calidad/riesgos-materiales-s26'
S = R / 'docs/calidad/Inventario-sv/sucesos'
BASE = '829cf7611bd5a543dbacf381a1e691d9b5fc4ce4'
LAB = 'f0bc7f1d56a0d1c124fc031e9a1eb199e7b7295b'
DOC = 'RECEPCION_CONTRACTUAL_R02.md'
RESULT = ('Recibidas ocho obligaciones materiales y ocho discriminadores previos '
          'en S26, con relevo a R2-0, consulta histórica y DFL-003/004/005/006. '
          'Se precisan custodia, lectura coherente, residencia, confirmación, '
          'índices, consumidor, observador y recursos.')
NEXT = ('Preparar T01/T02/T07 sobre copia del montaje existente: sustitución de '
        'propuesta bajo expectativa fija, mezcla de dependencias y lectura '
        'posterior efectiva. Publicar fixtures, código y oráculos antes de ejecutarlos.')
LIMIT = ('Incremento documental: cero nuevas pruebas de comportamiento; cuatro '
         'sondas R01 anteriores conservadas; cero casos globales S26 cerrados. '
         'Sin BD, GUI, nueva primitiva o garantía material acreditadas. '
         'S22 activo y S24 pendiente conservan su secuencia.')


def digest(b):
    return hashlib.sha256(b).hexdigest()


def read_csv(p):
    with p.open(newline='') as f:
        q = csv.DictReader(f)
        return q.fieldnames, list(q)


def encode_csv(fields, rows, header=False):
    out = io.StringIO(newline='')
    w = csv.DictWriter(out, fields, lineterminator='\n')
    if header:
        w.writeheader()
    w.writerows(rows)
    return out.getvalue().encode()


assert not (E / 'r02/VERIFICACION.json').exists(), 'No repetir la incorporación'
frozen = json.loads((E / 'r01/FUENTES.json').read_text())['archivos']
for name, record in frozen.items():
    assert digest((R / name).read_bytes()) == record['sha256'], name
preserved = {str(p.relative_to(R)): digest(p.read_bytes())
             for p in (E / 'r01').rglob('*') if p.is_file() and '__pycache__' not in p.parts}
for base in [R / 'rust', R / 'docs/calidad/auditoria-consultas-identidad']:
    for p in base.rglob('*'):
        if p.is_file() and 'target' not in p.parts and '__pycache__' not in p.parts:
            preserved[str(p.relative_to(R))] = digest(p.read_bytes())

t = datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00', 'Z')
edits = {}
before = {}


def change(p, b):
    before[str(p.relative_to(R))] = p.read_bytes()
    edits[p] = b


def append(p, text):
    old = p.read_bytes()
    change(p, old + text.encode())


fields, rows = read_csv(S / 'SUCESOS_SV.csv')
old_rows = [dict(x) for x in rows]
row = next(x for x in rows if x['id'] == 'S26')
assert row['estado'] == 'en ejecución'
assert next(x for x in rows if x['id'] == 'S24')['estado'] == 'pendiente'
assert next(x for x in rows if x['id'] == 'S22')['estado'] == 'en ejecución'
row.update(fecha_actualizacion_utc=t,
           cortes_de_entrada=f'Lenguaje {BASE}; laboratorio {LAB}',
           resultado=RESULT, verificacion='Cotejo de 26 fuentes R01 y de la publicación previa; '
           'verificación administrativa de preservación, correspondencia y enlaces. ' + LIMIT,
           evidencias='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/'
           'docs/calidad/riesgos-materiales-s26/' + DOC,
           referencia_calidad='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/'
           'blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-225',
           siguiente_accion=NEXT, observaciones=LIMIT)
change(S / 'SUCESOS_SV.csv', encode_csv(fields, rows, True))
hf, history = read_csv(S / 'HISTORIAL_SUCESOS_SV.csv')
assert max(int(x['revision']) for x in history if x['id'] == 'S26') == 2
hp = S / 'HISTORIAL_SUCESOS_SV.csv'
change(hp, hp.read_bytes() + encode_csv(hf, [dict(revision=3, **row)]))
mp = S / 'SUCESOS_SV.md'
old = mp.read_text()
start = old.index('## S26 · ')
assert '\n## S' not in old[start + 1:]
section = '## S26 · ' + row['actividad'] + '\n\n' + '\n\n'.join(
    '**' + k + ':** ' + (v or '—') for k, v in row.items() if k not in ('id', 'actividad')) + '\n\n'
change(mp, (old[:start] + section).encode())

retp = R / 'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv'
rf, rr = read_csv(retp)
assert rr[-1]['ID'] == 'RETP-2026-224'
values = ['RETP-2026-225', t[:10], '', 'RECEPCION_CONTRACTUAL_MATERIAL', 'S26', RESULT,
          'Continuación autorizada tras inventario y sondas R01; evitar duplicar contratos existentes.',
          'R2-0 §§4–12; LIG; continuidad por operación; nota de consulta histórica; captura A/V; '
          'Pilares, perfiles y transición secuencial.',
          'Recepción R02; adendas R2-0/consulta/DFL; S26 e historial; RETP CSV/MD; Léame y estado Bis.',
          'r02/VERIFICACION.json; fuentes y evidencia R01 conservadas.',
          'Fronteras materiales y exigencia de evidencia por operación explícitas.', LIMIT, NEXT, 'en ejecución']
assert len(values) == len(rf)
change(retp, retp.read_bytes() + encode_csv(rf, [dict(zip(rf, values))]))
append(R / 'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md',
       '\n\n<a id="retp-225"></a>\n\n### RETP-2026-225 · S26 · Recepción contractual R02\n\n'
       + t + '. ' + RESULT + ' ' + LIMIT + '\n\n' + NEXT
       + '\n\n[Recepción y discriminadores](riesgos-materiales-s26/' + DOC + ').\n')

append(E / 'README.md', '\n## Recepción R02 / RETP-2026-225\n\n' + RESULT + ' ' + LIMIT
       + '\n\n' + NEXT + '\n\n[Recepción contractual](' + DOC + ').\n')
append(R / 'docs/arquitectura/CONTRATO_R2_0_PERSISTENCIA_CONTINUIDAD_Y_RECUPERACION_2026_08_25.md',
       '\n## 19. Recepción material S26 / RETP-2026-225 · 13/09/2026\n\n'
       'La [recepción R02](../calidad/riesgos-materiales-s26/' + DOC + ') concreta, de forma '
       'subordinada a §§4–12, las obligaciones de custodia de la expectativa, lectura coherente, '
       'residencia, confirmación material, cobertura de índices y observación posterior. '
       'Recibe el inventario y las cuatro sondas parciales R01; no constituye otra AStore, '
       'clave universal, BD o primitiva temporal.\n\n'
       'Las realizaciones dependientes deben declarar unidad de transacción, política durable, '
       'confirmación, concurrencia y reconciliación; la aceptación en RAM y el acuse de una API '
       'no acreditan por sí solos esas propiedades. T04–T06 permanecen pendientes del perfil '
       'material correspondiente. S26 no modifica los cierres históricos ni acredita cierre '
       'actual de R2/R3/R4 o de las garantías I/II.\n')
append(R / 'docs/calidad/NOTA_TECNICA_SOBRE_FRAME_HISTORICO_REAPERTURA_Y_CONSULTA_PRESENTE_2026_03_19.md',
       '\n## 12. Recepción S26 / RETP-2026-225 · 13/09/2026\n\n'
       'La [recepción R02](riesgos-materiales-s26/' + DOC + ') precisa la frontera material '
       'de esta nota. Una consulta histórica exacta debe conservar el contenido recuperado '
       'bajo sus dependencias fijadas, sin prometer disponibilidad de todas las llamadas. '
       'La consulta presente debe declarar cobertura y corte, y puede evolucionar sin '
       'reescribir el antecedente. Una respuesta atrasada no se atribuirá a otra selección.\n\n'
       'C02/C05/C06 reciben esas condiciones y T02/T03/T05 sus contrastes previstos. '
       'No se ofrece una API histórica completa ni se cierran CQ1–CQ6 o DFL-003/004/005. '
       'El fallo técnico no constituye U; la reapertura legítima conserva su fundamento '
       'semántico y no se fabrica por un error del transporte.\n')
append(R / 'docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md',
       '\n## Recepción material S26 / RETP-2026-225 · 13/09/2026\n\n'
       '[Recepción contractual R02](riesgos-materiales-s26/' + DOC + '): '
       'DFL-003/004 reciben selección y corte coherentes, historia fijada frente a proyección '
       'presente, cobertura de índices y correlación del consumidor; DFL-005 recibe custodia '
       'de la expectativa y referentes suficientes por operación; DFL-006 conserva identidad '
       'del productor/adquisición y evidencia material del resultado como dependencias. '
       'R2-0 recibe transacción, confirmación y recuperación. Se reutilizan las sedes; '
       'ninguna deuda se renumera ni se cierra.\n\n' + LIMIT + ' ' + NEXT + '\n')
append(R / 'docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md',
       '\n### S26 R02 · RETP-2026-225\n\n' + RESULT + ' ' + LIMIT + '\n\n' + NEXT
       + '\n\n[Recepción y relevo](../../riesgos-materiales-s26/' + DOC + ').\n')
wp = R / 'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/ESTADO_WORKFLOW.json'
workflow = json.loads(wp.read_text())
old_workflow = json.loads(wp.read_text())
workflow['seguimiento_transversal_s26'].update(
    registro='RETP-2026-225', recepcion_contractual='../../../riesgos-materiales-s26/' + DOC,
    contrastes_r02_definidos=8, contrastes_r02_ejecutados=0, casos_globales_cerrados=0,
    siguiente_accion=NEXT)
change(wp, (json.dumps(workflow, ensure_ascii=False, indent=2) + '\n').encode())

assert [x for x in rows if x['id'] != 'S26'] == [x for x in old_rows if x['id'] != 'S26']
assert {k: v for k, v in workflow.items() if k != 'seguimiento_transversal_s26'} == {
    k: v for k, v in old_workflow.items() if k != 'seguimiento_transversal_s26'}
for p, content in edits.items():
    p.write_bytes(content)
for p, h in preserved.items():
    assert digest((R / p).read_bytes()) == h, p
for p, old_bytes in before.items():
    if p.endswith(('SUCESOS_SV.csv', 'SUCESOS_SV.md', 'ESTADO_WORKFLOW.json')) and not p.endswith('HISTORIAL_SUCESOS_SV.csv'):
        continue
    assert (R / p).read_bytes().startswith(old_bytes), p
_, actual_history = read_csv(hp)
assert actual_history[:-1] == history
assert {k: v for k, v in actual_history[-1].items() if k != 'revision'} == row
_, actual_retps = read_csv(retp)
assert actual_retps[:-1] == rr
assert all('**' + k + ':** ' + (v or '—') in section for k, v in row.items() if k not in ('id', 'actividad'))

text = (E / DOC).read_text()
assert len(re.findall(r'^### C0[1-8] ·', text, re.M)) == 8
assert len(re.findall(r'^\| T0[1-8] /', text, re.M)) == 8
links = re.findall(r'\]\(([^)]+)\)', text)
for link in links:
    assert (E / link.split('#', 1)[0]).resolve().is_file(), link

report = dict(corte_lenguaje=BASE, corte_laboratorio=LAB, fecha_verificacion_utc=t,
              clase='verificacion_documental_no_ensayo_sv', conforme=True,
              fuentes_r01_previas_cotejadas=len(frozen),
              archivos_preservados=preserved, obligaciones=8, discriminadores=8,
              nuevos_contrastes_ejecutados=0, casos_globales_cerrados=0,
              historial_revision=3, retp='RETP-2026-225',
              comprobaciones=['S26 CSV/MD/historial concordantes', 'resto de sucesos conservado',
                              'historial y RETP anteriores conservados', 'estado Bis ajeno a S26 conservado',
                              'R01 y código Rust local conservados', 'adendas sin reescribir antecedente',
                              'enlaces locales de recepción existentes'],
              archivos_editados={p: dict(antes_sha256=digest(b), despues_sha256=digest((R / p).read_bytes()))
                                 for p, b in before.items()},
              fuentes_adicionales={p: digest(b) for p, b in before.items() if any(
                  token in p for token in ('CONTRATO_R2_0', 'NOTA_TECNICA_SOBRE_FRAME', 'REGISTRO_DEUDA_VIVA'))})
(E / 'r02/VERIFICACION.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')
print(json.dumps({k: v for k, v in report.items() if k not in (
    'archivos_preservados', 'archivos_editados', 'fuentes_adicionales')}, ensure_ascii=False))
