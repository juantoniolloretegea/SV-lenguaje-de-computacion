"""Administración S26 R03. Desde raíz del checkout: gestionar.py previo|resultado.
No ensaya semántica: añade revisiones de Sucesos y RETP, conservando anteriores.
"""
from pathlib import Path
from datetime import datetime,timezone
import csv,io,json,sys
R=Path.cwd();E=R/'docs/calidad/riesgos-materiales-s26';F=E/'r03'
S=R/'docs/calidad/Inventario-sv/sucesos'
stage=sys.argv[1];assert stage in ('previo','resultado')
num=226 if stage=='previo' else 227
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
    result='Banco R03 previo: tres sondas parciales T01/T02/T07, con controles de sustitución bajo custodia fija, lectura mezclada en barrera explícita y testigo posterior copiado frente a releído.'
    verification='Fixtures A literales de R01; B añade LF y actualiza referencias de fuente. Código y oráculos fijados antes de compilar; cero sondas R03 ejecutadas.'
    next_action='Ejecutar R03 tras publicar precompromiso, conservar evidencia y clasificar el límite del observador sin confundir test verde con prevención material.'
else:
    results=json.loads((F/'RESULTADOS.json').read_text());assert results['sondas_aprobadas']==3 and results['sondas_fallidas']==0
    result='R03: tres sondas conformes a sus oráculos. T01 rechaza B bajo A y admite B bajo su custodia declarada; T02 rechaza fuente mezclada I02; T07 reproduce conformidad con copia inicial pese a archivo alterado y detecta alteración con relectura real.'
    verification='Una campaña nativa offline: tres funciones, cero fallos de aserción. Límite observado en T07; no prueba de protección contra host ni de transacción durable. Evidencia literal, órdenes y fuentes conservadas.'
    next_action='Preparar recepción instrumentada del testigo posterior y su cualificación en el conductor existente; conservar el contraejemplo T07 y exigir nueva campaña antes de acreditar esa recepción. R2 y GUI conservan sus dependencias.'
row.update(fecha_actualizacion_utc=t,cortes_de_entrada='Lenguaje b0f4f5f0093ad59324fcb6e00ed65cc15e3ca8f9; laboratorio e167aeee5d58c745704000e40395a635e1f25d60',resultado=result,verificacion=verification,siguiente_accion=next_action,
    evidencias='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/riesgos-materiales-s26/r03/README.md',
    referencia_calidad=f'https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-{num}',
    observaciones='S26 sigue en ejecución; los doce casos globales permanecen abiertos. S22/S24 y banco original Bis intactos. Ensayo intra-proceso y de archivos locales; sin transacción durable, GUI, adquisición física, fallo de hardware o resistencia al host acreditados. Sin selección de BD ni promoción productiva.')
hf,hist=read(S/'HISTORIAL_SUCESOS_SV.csv');rev=max(int(x['revision']) for x in hist if x['id']=='S26')+1
assert rev==(4 if stage=='previo' else 5)
(S/'SUCESOS_SV.csv').write_bytes(encoded(fields,rows,True))
p=S/'HISTORIAL_SUCESOS_SV.csv';old=p.read_bytes();p.write_bytes(old+encoded(hf,[dict(revision=rev,**row)]))
p=S/'SUCESOS_SV.md';s=p.read_text();i=s.index('## S26 · ')
p.write_text(s[:i]+'## S26 · '+row['actividad']+'\n\n'+'\n\n'.join('**'+k+':** '+(v or '—') for k,v in row.items() if k not in ('id','actividad'))+'\n\n')
p=R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv';rf,rr=read(p);assert rr[-1]['ID']==f'RETP-2026-{num-1}'
vals=[f'RETP-2026-{num}',t[:10],'','INVENTARIO_Y_BANCO_PREVIO' if stage=='previo' else 'CONTRASTE_MATERIAL_ACOTADO','S26',result,'Continuación expresamente autorizada de S26.','R2-0; LIG; Frame/R1; rectores; S26/RETP-222; realización histórica RETP-220/221.','Expediente S26 R03; Sucesos e historial; RETP CSV/MD; Léame primero; enlace transversal Bis.',verification,'Residencia y fronteras explícitas; brechas y controles parciales delimitados.',row['observaciones'],next_action,row['estado']]
assert len(vals)==len(rf);p.write_bytes(p.read_bytes()+encoded(rf,[dict(zip(rf,vals))]))
p=R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md'
p.write_text(p.read_text()+f'\n\n<a id="retp-{num}"></a>\n\n### RETP-2026-{num} · S26 · R03 {stage}\n\n'+t+'. '+result+' '+verification+' '+next_action+' '+row['observaciones']+' [Inventario](riesgos-materiales-s26/r03/README.md).\n')
p=R/'docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md'
p.write_text(p.read_text()+f'\n### S26 R03 · RETP-2026-{num}\n\n'+result+' '+next_action+' [Inventario y límites](../../riesgos-materiales-s26/r03/README.md).\n')
p=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/ESTADO_WORKFLOW.json';s=json.loads(p.read_text());original=dict(s)
s['seguimiento_transversal_s26']=dict(s['seguimiento_transversal_s26'],registro=f'RETP-2026-{num}',campana_r03='../../../riesgos-materiales-s26/r03/README.md',sondas_r03_previstas=3,sondas_r03_ejecutadas=0 if stage=='previo' else 3,casos_globales_cerrados=0,siguiente_accion=next_action)
p.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n')
assert [x for x in before if x['id']!='S26']==[x for x in rows if x['id']!='S26']
assert {k:v for k,v in original.items() if k!='seguimiento_transversal_s26'}=={k:v for k,v in s.items() if k!='seguimiento_transversal_s26'}
p=E/'README.md';p.write_text(p.read_text()+f'\n## Continuación R03 / RETP-2026-{num}\n\n'+result+' [Inventario material](r03/README.md).\n')
print('S26 revisión',rev,'RETP',num,t)
