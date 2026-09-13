"""Administración S26 R01. Desde raíz del checkout: gestionar.py previo|resultado.
No ensaya semántica: añade revisiones de Sucesos y RETP, conservando anteriores.
"""
from pathlib import Path
from datetime import datetime,timezone
import csv,io,json,sys
R=Path.cwd();E=R/'docs/calidad/riesgos-materiales-s26';F=E/'r01'
S=R/'docs/calidad/Inventario-sv/sucesos'
stage=sys.argv[1];assert stage in ('previo','resultado')
num=223 if stage=='previo' else 224
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
    result='Inventario R01 de nueve tramos materiales y disposición de los doce casos S26. Cuatro sondas parciales comprometidas sobre el montaje Rust existente y fixture I0205-01 literal. Cero sondas nuevas ejecutadas.'
    verification='26 archivos fuente identificados por SHA256; quince archivos fixture cotejados contra el archivo histórico. Matriz enlaza residencia, mutadores, confirmación y brechas; oráculos y código fijados antes de campaña.'
    next_action='Ejecutar las cuatro sondas R01 con Cargo --locked --offline tras publicar el precompromiso; conservar resultados y trasladar las brechas de persistencia, captura y autoridad a sus sedes existentes.'
else:
    results=json.loads((F/'RESULTADOS.json').read_text());assert results['sondas_aprobadas']==4 and results['sondas_fallidas']==0
    result='Cuatro sondas R01 conformes en Rust/Cargo 1.98.0: positivo documental, archivo alterado después de recepción con nueva lectura rechazada I02, captura alterada detectada D06 y ausencia/fallo posterior D01/D07. Núcleo y admisor existentes sin cambios.'
    verification='Una campaña nativa offline; cuatro funciones de prueba y cero fallos; stdout/stderr y órdenes conservados. Buffer anterior conserva testigo; fuente con LF sigue compilando con identidad distinta. D06 acredita detección posterior, no ausencia de entrega. Fuentes y precompromiso cotejados.'
    next_action='Recibir el inventario y resultados en los contratos de consulta/captura y R2: concretar autoridad de custodia, corte coherente y confirmación material antes de ensayar concurrencia, reinicio o GUI. Continuar las fronteras pendientes de Bis sin cerrarlas por estas sondas.'
row.update(fecha_actualizacion_utc=t,cortes_de_entrada='Lenguaje 8028193c085a7f1a9e063728569d62f380e1b1de; laboratorio c017c1c472ed3e1e85b8b1efe8f5df6e6b126879',resultado=result,verificacion=verification,siguiente_accion=next_action,
    evidencias='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/riesgos-materiales-s26/INVENTARIO_MATERIAL_R01.md',
    referencia_calidad=f'https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-{num}',
    observaciones='S26 sigue en ejecución; los doce casos globales permanecen abiertos. S22/S24 y banco original Bis intactos. Ensayo intra-proceso y de archivos locales; sin transacción durable, GUI, adquisición física, fallo de hardware o resistencia al host acreditados. Sin selección de BD ni promoción productiva.')
hf,hist=read(S/'HISTORIAL_SUCESOS_SV.csv');rev=max(int(x['revision']) for x in hist if x['id']=='S26')+1
assert rev==(1 if stage=='previo' else 2)
(S/'SUCESOS_SV.csv').write_bytes(encoded(fields,rows,True))
p=S/'HISTORIAL_SUCESOS_SV.csv';old=p.read_bytes();p.write_bytes(old+encoded(hf,[dict(revision=rev,**row)]))
p=S/'SUCESOS_SV.md';s=p.read_text();i=s.index('## S26 · ')
p.write_text(s[:i]+'## S26 · '+row['actividad']+'\n\n'+'\n\n'.join('**'+k+':** '+(v or '—') for k,v in row.items() if k not in ('id','actividad'))+'\n\n')
p=R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv';rf,rr=read(p);assert rr[-1]['ID']==f'RETP-2026-{num-1}'
vals=[f'RETP-2026-{num}',t[:10],'','INVENTARIO_Y_BANCO_PREVIO' if stage=='previo' else 'CONTRASTE_MATERIAL_ACOTADO','S26',result,'Continuación expresamente autorizada de S26.','R2-0; LIG; Frame/R1; rectores; S26/RETP-222; realización histórica RETP-220/221.','Expediente S26 R01; Sucesos e historial; RETP CSV/MD; Léame primero; enlace transversal Bis.',verification,'Residencia y fronteras explícitas; brechas y controles parciales delimitados.',row['observaciones'],next_action,row['estado']]
assert len(vals)==len(rf);p.write_bytes(p.read_bytes()+encoded(rf,[dict(zip(rf,vals))]))
p=R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md'
p.write_text(p.read_text()+f'\n\n<a id="retp-{num}"></a>\n\n### RETP-2026-{num} · S26 · R01 {stage}\n\n'+t+'. '+result+' '+verification+' '+next_action+' '+row['observaciones']+' [Inventario](riesgos-materiales-s26/INVENTARIO_MATERIAL_R01.md).\n')
p=R/'docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md'
p.write_text(p.read_text()+f'\n### S26 R01 · RETP-2026-{num}\n\n'+result+' '+next_action+' [Inventario y límites](../../riesgos-materiales-s26/INVENTARIO_MATERIAL_R01.md).\n')
p=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/ESTADO_WORKFLOW.json';s=json.loads(p.read_text());original=dict(s)
s['seguimiento_transversal_s26']=dict(s['seguimiento_transversal_s26'],registro=f'RETP-2026-{num}',inventario='../../../riesgos-materiales-s26/INVENTARIO_MATERIAL_R01.md',sondas_parciales_previas=4,sondas_parciales_ejecutadas=0 if stage=='previo' else 4,casos_globales_cerrados=0,siguiente_accion=next_action)
p.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n')
assert [x for x in before if x['id']!='S26']==[x for x in rows if x['id']!='S26']
assert {k:v for k,v in original.items() if k!='seguimiento_transversal_s26'}=={k:v for k,v in s.items() if k!='seguimiento_transversal_s26'}
p=E/'README.md';p.write_text(p.read_text()+f'\n## Continuación R01 / RETP-2026-{num}\n\n'+result+' [Inventario material](INVENTARIO_MATERIAL_R01.md).\n')
print('S26 revisión',rev,'RETP',num,t)
