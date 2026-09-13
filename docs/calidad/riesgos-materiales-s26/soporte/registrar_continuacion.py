"""Aplica un asiento explícito JSON a Sucesos y RETP. Conserva otras filas y prefijos.
Uso: python .../registrar_continuacion.py asiento.json desde checkout.
"""
from pathlib import Path
import json,csv,io,sys
R=Path.cwd();a=json.loads(Path(sys.argv[1]).read_text());S=R/'docs/calidad/Inventario-sv/sucesos'
def read(p):
 with p.open(newline='') as f:r=csv.DictReader(f);return r.fieldnames,list(r)
def enc(fields,rows,header=False):
 o=io.StringIO(newline='');w=csv.DictWriter(o,fields,lineterminator='\n')
 if header:w.writeheader()
 w.writerows(rows);return o.getvalue().encode()
f,rows=read(S/'SUCESOS_SV.csv');before=[dict(x) for x in rows];id=a['suceso']['id'];existing=next((x for x in rows if x['id']==id),None)
if existing:row=dict(existing);row.update(a['suceso']);rows[rows.index(existing)]=row
else:
 assert int(id[1:])==max(int(x['id'][1:]) for x in rows)+1
 row={k:'' for k in f};row.update(a['suceso']);rows.append(row)
assert set(row)==set(f);assert row['estado'] in ('pendiente','en ejecución','finalizado')
hf,hist=read(S/'HISTORIAL_SUCESOS_SV.csv');rev=1+max([int(x['revision']) for x in hist if x['id']==id],default=-1);assert rev==a['revision']
rf,rr=read(R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv');num=a['retp'];assert rr[-1]['ID']==f'RETP-2026-{num-1}'
assert [x for x in before if x['id']!=id]==[x for x in rows if x['id']!=id]
(S/'SUCESOS_SV.csv').write_bytes(enc(f,rows,True));p=S/'HISTORIAL_SUCESOS_SV.csv';p.write_bytes(p.read_bytes()+enc(hf,[dict(revision=rev,**row)]))
p=S/'SUCESOS_SV.md';s=p.read_text();block='## '+id+' · '+row['actividad']+'\n\n'+'\n\n'.join('**'+k+':** '+(v or '—') for k,v in row.items() if k not in ('id','actividad'))+'\n\n'
if existing:
 i=s.index('## '+id+' · ');end=s.find('\n## S',i+1);end=len(s) if end<0 else end+1;s=s[:i]+block+s[end:]
else:s+='\n'+block
p.write_text(s)
vals=[f'RETP-2026-{num}',row['fecha_actualizacion_utc'][:10],'',a['tipo'],id,row['resultado'],a['motivo'],a['base'],a['artefactos'],row['verificacion'],a['impacto'],row['observaciones'],row['siguiente_accion'],row['estado']]
assert len(vals)==len(rf);p=R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv';p.write_bytes(p.read_bytes()+enc(rf,[dict(zip(rf,vals))]))
p=R/'docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md';p.write_text(p.read_text()+f'\n\n<a id="retp-{num}"></a>\n\n### RETP-2026-{num} · '+id+' · '+a['titulo']+'\n\n'+row['fecha_actualizacion_utc']+'. '+row['resultado']+' '+row['verificacion']+' '+row['observaciones']+' '+row['siguiente_accion']+' [Evidencia]('+row['evidencias']+').\n')
print(id,rev,num)
