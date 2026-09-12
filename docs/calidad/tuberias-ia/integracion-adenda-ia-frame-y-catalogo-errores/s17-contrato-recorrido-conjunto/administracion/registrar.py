from pathlib import Path
import json,csv,io,hashlib
from datetime import datetime,timezone
R=Path(__file__).resolve().parent;D=R/'entrega';P=R.parent/'s16-fronteras/actualizacion';O=R/'actualizacion';O.mkdir(exist_ok=True);J=lambda p,x:p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n');blob=lambda b:hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
cut=json.loads((D/'CORTE_Y_ALCANCE.json').read_text());T=cut['fecha_cierre_documental_utc'];pub='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion';ip='tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores';wp=ip+'/s17-contrato-recorrido-conjunto';url=pub+'/blob/main/docs/calidad/'+wp+'/ACTA_CONTRATO_S17.md'
for k in ['lenguaje','laboratorio']:
 t=json.loads((R/(k+'-arbol.json')).read_text());prefix='docs/calidad/Inventario-sv/sucesos' if k=='lenguaje' else 'laboratorio/tareas-watson/sucesos-sv'
 for n in ['SUCESOS_SV.csv','SUCESOS_SV.md','HISTORIAL_SUCESOS_SV.csv']:assert t[prefix+'/'+n]==blob((P/n).read_bytes())
tr=json.loads((R/'lenguaje-arbol.json').read_text())
for ext in ['md','csv']:assert tr['docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext]==blob((P/('registro.'+ext)).read_bytes())
with (P/'SUCESOS_SV.csv').open(newline='') as f:rd=csv.DictReader(f);fields=rd.fieldnames;rows=list(rd)
assert [x['id'] for x in rows]==['S'+str(i) for i in range(17)]
start=datetime.fromtimestamp((R/'lenguaje-base.json').stat().st_mtime,timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z')
row={k:'' for k in fields};row.update(id='S17',estado='finalizado',fecha_alta_utc=T,fecha_inicio_utc=start,fecha_actualizacion_utc=T,fecha_fin_utc=T,unidad_responsable='Watson / W-S0',actividad='Contrato del recorrido documental conjunto: contexto, base, recepción, cobertura y entrega',alcance='Contrato candidato S17 y banco de 24 obligaciones. Adaptación explícita de interfaces; sin implementación ni ejecución conjunta.',repositorios_y_ramas='SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente',cortes_de_entrada='Lenguaje '+cut['cortes']['lenguaje']['head']+'; laboratorio '+cut['cortes']['laboratorio']['head'],dependencias='S16 / RETP-187; S11/S14/S15; cápsula original G1/S2; rectores y relevo S12/S13.',resultado='Contrato y obligaciones fijados. Enlace directo insuficiente: lote fijo S2, Vista S14 y bytes de Informe S15 no equivalen a una entrega tipada. Adaptación y conformidad ejecutable pendientes.',verificacion='15 fuentes íntegras, cuatro interfaces encapsuladas y 11 pasajes exactos; 24 casos fijados; cero ensayos funcionales nuevos. Esperados originales S14 intactos.',evidencias=url,referencia_calidad=pub+'/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-188',siguiente_accion=cut['siguiente_objeto'],observaciones='S17 cierra un contrato documental, no una campaña. 144 observaciones normales previstas, ninguna realizada. Presupuesto e implementación por fijar antes de ejecutar. B/E/K/L y puertas P3/P4/P5/P6 vigentes; sin promoción nuclear; agentes por decidir tras inmunología.')
rows.append(row);s=io.StringIO(newline='');w=csv.DictWriter(s,fieldnames=fields,lineterminator='\n');w.writeheader();w.writerows(rows);(O/'SUCESOS_SV.csv').write_text(s.getvalue())
s=io.StringIO(newline='');csv.DictWriter(s,fieldnames=['revision']+fields,lineterminator='\n').writerow(dict(revision=0,**row));(O/'HISTORIAL_SUCESOS_SV.csv').write_bytes((P/'HISTORIAL_SUCESOS_SV.csv').read_bytes()+s.getvalue().encode())
md='# Sucesos SV — estado vigente\n\n[Reglas](README.md) · [CSV](SUCESOS_SV.csv) · [Historial](HISTORIAL_SUCESOS_SV.csv)\n\n| Suceso | Estado | Actividad | Responsable | Actualización UTC |\n| --- | --- | --- | --- | --- |\n'
for x in rows:md+='| '+' | '.join(x[k].replace('|','&#124;') for k in ['id','estado','actividad','unidad_responsable','fecha_actualizacion_utc'])+' |\n'
for x in rows:
 md+='\n## '+x['id']+' · '+x['actividad']+'\n\n'
 for k in fields:
  if k in ['id','actividad']:continue
  v=x[k] or '—'
  if k in ['evidencias','referencia_calidad'] and x[k]:v=' · '.join('[Referencia '+str(i+1)+']('+u+')' for i,u in enumerate(x[k].split(' ; ')))
  md+='**'+k.replace('_',' ').capitalize()+':** '+v+'\n\n'
(O/'SUCESOS_SV.md').write_text(md)
entry=f'\n\n<a id="retp-188"></a>\n\n### RETP-2026-188 · S17 · Contrato del recorrido documental conjunto\n\n{T}. [Acta y contrato]({wp}/ACTA_CONTRATO_S17.md). {row["resultado"]} {row["verificacion"]} Se fija la relación entre producción G1, base consumida, contexto, recepción completa y entrega tipada; no se cambia el lote anterior ni se eleva una lista de bytes a credencial. **Siguiente objeto único:** {row["siguiente_accion"]} {row["observaciones"]}\n'
(O/'registro.md').write_bytes((P/'registro.md').read_bytes()+entry.encode());s=io.StringIO(newline='');csv.writer(s,lineterminator='\n').writerow(['RETP-2026-188',T[:10],'','CONTRATO_DOCUMENTAL','S17 / contrato integración 1+3','Contexto, base, recepción, cobertura y entrega tipada','Luz verde tras S16 / RETP-187','S11; S14; S15; S16; interfaces G1/S2; rectores','Contrato, banco, diagnósticos, fuentes, pasajes, comprobador, Sucesos y espejo',row['verificacion'],row['resultado'],'Sin promoción nuclear; ninguna ejecución integrada acreditada',row['siguiente_accion'],'finalizado']);(O/'registro.csv').write_bytes((P/'registro.csv').read_bytes()+s.getvalue().encode())
readme=(P/'integracion-README.md').read_bytes()
for k in ['lenguaje','laboratorio']:
 pref='docs/calidad' if k=='lenguaje' else 'laboratorio/tareas-watson';assert json.loads((R/(k+'-arbol.json')).read_text())[pref+'/'+ip+'/README.md']==blob(readme)
add=f'\n\n## S17 · Contrato del recorrido conjunto · RETP-188\n\n{T}. [Acta y evidencia]({url}). {row["resultado"]} {row["verificacion"]}\n\n**Siguiente objeto único:** {row["siguiente_accion"]} La conformidad de piezas separadas no acredita una ejecución integrada inexistente. No se adelantan P3/P4/P5/P6 ni agentes; S12/S13 conservan su autoridad.\n';(O/'integracion-README.md').write_bytes(readme+add.encode())
assert rows[:-1]==list(csv.DictReader((P/'SUCESOS_SV.csv').open()))
for p in O.glob('*.csv'):
 z=list(csv.reader(p.open(newline='')));assert all(len(x)==len(z[0]) for x in z)
maps={}
for k in ['laboratorio','lenguaje']:
 pref='docs/calidad' if k=='lenguaje' else 'laboratorio/tareas-watson';ep=pref+('/Inventario-sv/sucesos' if k=='lenguaje' else '/sucesos-sv');m={ep+'/'+n:str(O/n) for n in ['SUCESOS_SV.csv','SUCESOS_SV.md','HISTORIAL_SUCESOS_SV.csv']};m.update({pref+'/'+wp+'/'+str(p.relative_to(D)):str(p) for p in D.rglob('*') if p.is_file()});m[pref+'/'+ip+'/README.md']=str(O/'integracion-README.md')
 if k=='lenguaje':
  for ext in ['md','csv']:m[pref+'/REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext]=str(O/('registro.'+ext))
 maps[k]=m
J(R/'MAPA.json',maps);J(R/'ELIMINACIONES.json',{'laboratorio':[],'lenguaje':[]});print('S17 / RETP-188 preparado; S0–S16 e históricos íntegros.')
