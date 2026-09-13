"""Administración S26 R04. Desde raíz del checkout: gestionar.py previo|resultado.
No ensaya semántica: añade revisiones de Sucesos y RETP, conservando anteriores.
"""
from pathlib import Path
from datetime import datetime,timezone
import csv,io,json,sys
R=Path.cwd();E=R/'docs/calidad/riesgos-materiales-s26';F=E/'r04'
S=R/'docs/calidad/Inventario-sv/sucesos'
stage=sys.argv[1];assert stage in ('previo','resultado')
num=228 if stage=='previo' else 229
def read(p):
    with p.open(newline='') as f:
        r=csv.DictReader(f);return r.fieldnames,list(r)
def encoded(fields,rows,header=False):
    out=io.StringIO(newline='');w=csv.DictWriter(out,fields,lineterminator='\n')
    if header:w.writeheader()
    w.writerows(rows);return out.getvalue().encode()
fields,rows=read(S/'SUCESOS_SV.csv');before=[dict(x) for x in rows]
row=next(x for x in rows if x['id']=='S26');assert row['estado']=='en ejecución'
t=datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z')
if stage=='previo':
    result='Variante experimental R04 de recepción del testigo: lectura inicial y posterior reales con cuota; entrega y preservación separadas. Ocho sondas previas P01-P08 con controles y fallos de lectura.'
    verification='Fragmento del conductor I0205 reutilizado; observador literal; quince fixtures R01 preservados y variante de vector Zero a One fijada. Fuentes, oráculos y código comprometidos antes de compilar.'
    next_action='Ejecutar ocho sondas R04 después del precompromiso; conservar la evidencia del lector y del despacho; cotejar sensibilidad al testigo copiado y límites del montaje.'
else:
    results=json.loads((F/'RESULTADOS.json').read_text());assert results['sondas_aprobadas']==8 and results['sondas_fallidas']==0
    result='R04: ocho sondas conformes. Relectura posterior detecta cambio literal y del vector; ausencia, error de lectura y exceso se distinguen sin borrar entrega previa. Fallo o cambio inicial detienen el recorrido; el observador heredado conserva su límite ante copias falsamente posteriores.'
    verification='Una campaña Rust/Cargo 1.98.0, offline, ocho funciones y cero fallos de aserción; informes por caso y fuentes conservados. Realización local por operación; no BD, aislamiento, captura de pantalla ni recuperación acreditados.'
    next_action='Extender la cualificación por las salidas restantes del conductor y fijar tratamiento de interrupción/pánico y sustitución de fuente durante lectura antes de integrar esta recepción en el recorrido completo. Conservar S26 y el relevo Bis/S24.'
row.update(fecha_actualizacion_utc=t,cortes_de_entrada='Lenguaje 10c84db9d330f5aeac1ba8b74f069024c6260e5d; laboratorio b763f5ab9f0dab59265ab1f2e73573ce954730d5',resultado=result,verificacion=verification,siguiente_accion=next_action,
    evidencias='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/riesgos-materiales-s26/r04/README.md',
    referencia_calidad=f'https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-{num}',
    observaciones='S26 sigue en ejecución; los doce casos globales permanecen abiertos. S22/S24 y banco original Bis intactos. Ensayo intra-proceso y de archivos locales; sin transacción durable, GUI, adquisición física, fallo de hardware o resistencia al host acreditados. Sin selección de BD ni promoción productiva.')
hf,hist=read(S/'HISTORIAL_SUCESOS_SV.csv');rev=max(int(x['revision']) for x in hist if x['id']=='S26')+1
assert rev==(6 if stage=='previo' else 7)
(S/'SUCESOS_SV.csv').write_bytes(encoded(fields,rows,True))
p=S/'HISTORIAL_SUCESOS_SV.csv';old=p.read_bytes();p.write_bytes(old+encoded(hf,[dict(revision=rev,**row)]))
p=S/'SUCESOS_SV.md';s=p.read_text();i=s.index('## S26 · ')
p.write_text(s[:i]+'## S26 · '+row['actividad']+'\n\n'+'\n\n'.join('**'+k+':** '+(v or '—') for k,v in row.items() if k not in ('id','actividad'))+'\n\n')
p=R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv';rf,rr=read(p);assert rr[-1]['ID']==f'RETP-2026-{num-1}'
vals=[f'RETP-2026-{num}',t[:10],'','INVENTARIO_Y_BANCO_PREVIO' if stage=='previo' else 'CONTRASTE_MATERIAL_ACOTADO','S26',result,'Continuación expresamente autorizada de S26.','R2-0; LIG; Frame/R1; rectores; S26/RETP-222; realización histórica RETP-220/221.','Expediente S26 R04; Sucesos e historial; RETP CSV/MD; Léame primero; enlace transversal Bis.',verification,'Residencia y fronteras explícitas; brechas y controles parciales delimitados.',row['observaciones'],next_action,row['estado']]
assert len(vals)==len(rf);p.write_bytes(p.read_bytes()+encoded(rf,[dict(zip(rf,vals))]))
p=R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md'
p.write_text(p.read_text()+f'\n\n<a id="retp-{num}"></a>\n\n### RETP-2026-{num} · S26 · R04 {stage}\n\n'+t+'. '+result+' '+verification+' '+next_action+' '+row['observaciones']+' [Inventario](riesgos-materiales-s26/r04/README.md).\n')
p=R/'docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md'
p.write_text(p.read_text()+f'\n### S26 R04 · RETP-2026-{num}\n\n'+result+' '+next_action+' [Inventario y límites](../../riesgos-materiales-s26/r04/README.md).\n')
p=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/ESTADO_WORKFLOW.json';s=json.loads(p.read_text());original=dict(s)
s['seguimiento_transversal_s26']=dict(s['seguimiento_transversal_s26'],registro=f'RETP-2026-{num}',campana_r04='../../../riesgos-materiales-s26/r04/README.md',sondas_r04_previstas=8,sondas_r04_ejecutadas=0 if stage=='previo' else 8,casos_globales_cerrados=0,siguiente_accion=next_action)
p.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n')
assert [x for x in before if x['id']!='S26']==[x for x in rows if x['id']!='S26']
assert {k:v for k,v in original.items() if k!='seguimiento_transversal_s26'}=={k:v for k,v in s.items() if k!='seguimiento_transversal_s26'}
p=E/'README.md';p.write_text(p.read_text()+f'\n## Continuación R04 / RETP-2026-{num}\n\n'+result+' [Inventario material](r04/README.md).\n')
print('S26 revisión',rev,'RETP',num,t)
