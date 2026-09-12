from pathlib import Path
from datetime import datetime,timezone
import json,csv,io,hashlib,sys
R=Path(__file__).resolve().parent;D=R/'entrega';fase=sys.argv[1];assert fase in ['apertura','cierre'];closing=fase=='cierre';P=R/'actualizacion-apertura' if closing else R.parent/'s14-bases/actualizacion-cierre';O=R/('actualizacion-'+fase);O.mkdir(exist_ok=True)
T=datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z');retp='186' if closing else '185';blob=lambda b:hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
for k in ['lenguaje','laboratorio']:
 tree=json.loads((R/(k+'-arbol.json')).read_text());prefix='docs/calidad/Inventario-sv/sucesos' if k=='lenguaje' else 'laboratorio/tareas-watson/sucesos-sv'
 for n in ['SUCESOS_SV.csv','SUCESOS_SV.md','HISTORIAL_SUCESOS_SV.csv']:assert tree[prefix+'/'+n]==blob((P/n).read_bytes())
tr=json.loads((R/'lenguaje-arbol.json').read_text())
for ext in ['md','csv']:assert tr['docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext]==blob((P/('registro.'+ext)).read_bytes())
ip='tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores';wp=ip+'/s15-causas-recepcion';pub='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion';url=pub+'/blob/main/docs/calidad/'+wp+('/ACTA_RESULTADO_S15.md' if closing else '/README.md')
with (P/'SUCESOS_SV.csv').open(newline='') as f:rd=csv.DictReader(f);fields=rd.fieldnames;rows=list(rd)
if closing:row=rows[-1].copy();assert row['id']=='S15'
else:
 assert [x['id'] for x in rows]==['S'+str(i) for i in range(15)]
 row={k:'' for k in fields};row.update(id='S15',fecha_alta_utc=T,fecha_inicio_utc=T,unidad_responsable='Watson / W-S0',actividad='F: causas de recepción, protocolo, cobertura y presentación',alcance='Catorce casos sintéticos; receptor acotado, fuentes G1/S2 intactas, registro y presentación textual sin pérdida de causa.',repositorios_y_ramas='SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente',cortes_de_entrada='Lenguaje '+json.loads((R/'lenguaje-base.json').read_text())['head']+'; laboratorio '+json.loads((R/'laboratorio-base.json').read_text())['head'],dependencias='S14 / RETP-184; cobertura S2 y cuerpo esperado anterior; autorización expresa de continuación.')
row.update(estado='finalizado' if closing else 'en ejecución',fecha_actualizacion_utc=T,fecha_fin_utc=T if closing else '',evidencias=url,referencia_calidad=pub+'/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-'+retp,observaciones='Host confiable, archivo y actos de laboratorio. No promoción nuclear, QueryResult nativo, transición SV, historia durable ni comportamiento de LLM. S12/S13 vigentes; agentes por valorar tras inmunología.')
if closing:
 res=json.loads((D/'RESULTADO.json').read_text());assert res['conforme']
 row.update(resultado='Conforme dentro de S15: negativa de protocolo, esquema inválido, no admisión documental y comunicación fallida conservan causa y bytes; sólo la respuesta verificada entrega cuerpo.',verificacion=f"84 observaciones en seis ejecuciones debug/release; {res['capturas_identicas_por_ejecucion']} capturas idénticas por ejecución; 21 invocaciones; tres sensibilidades detectadas en F05/F08/F02.",siguiente_accion='Recibir F acotada en matriz y causas; siguiente revisión de B/E/K/L por sede y productor conforme a S13, antes de promoción nuclear o cierre de catálogo.');rows[-1]=row
else:
 row.update(resultado='Contrato, fuentes, casos, esperados y presupuesto fijados antes de compilar; sin ejecución funcional nueva todavía.',verificacion='Cortes y rectores cotejados; cápsula G1/S2 intacta; esperados previos recuperados; fijación de fuentes y controles instrumentales.',siguiente_accion='Ejecutar reproductor con máximo 21 invocaciones, cero reintentos; conservar y detener ante cualquier fallo inesperado.');rows.append(row)
s=io.StringIO(newline='');w=csv.DictWriter(s,fieldnames=fields,lineterminator='\n');w.writeheader();w.writerows(rows);(O/'SUCESOS_SV.csv').write_text(s.getvalue());hist=(P/'HISTORIAL_SUCESOS_SV.csv').read_bytes();s=io.StringIO(newline='');csv.DictWriter(s,fieldnames=['revision']+fields,lineterminator='\n').writerow(dict(revision=1 if closing else 0,**row));(O/'HISTORIAL_SUCESOS_SV.csv').write_bytes(hist+s.getvalue().encode())
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
title=('Resultado acotado' if closing else 'Fijación previa')+' de causas de recepción y no admisión'
entry=f'\n\n<a id="retp-{retp}"></a>\n\n### RETP-2026-{retp} · S15 · {title}\n\n{T}. {row["resultado"]} {row["verificacion"]} [Contrato y evidencia]({wp}/README.md). {row["observaciones"]} Siguiente: {row["siguiente_accion"]}\n'
(O/'registro.md').write_bytes((P/'registro.md').read_bytes()+entry.encode());s=io.StringIO(newline='');csv.writer(s,lineterminator='\n').writerow(['RETP-2026-'+retp,T[:10],'','CIERRE_ACOTADO' if closing else 'FIJACION_PREVIA','S15 / F integración 1+3',title,'Luz verde tras S14 y objeto F delimitado','S14; S13; rectores; G1/S2; cuerpo anterior','Contrato, código, fuentes, presupuesto, capturas, Sucesos y espejo',row['verificacion'],'Recorrido documental de laboratorio; ninguna promoción nuclear',row['observaciones'],row['resultado'],row['estado']]);(O/'registro.csv').write_bytes((P/'registro.csv').read_bytes()+s.getvalue().encode())
readme=(P/'integracion-README.md').read_bytes()
for k in ['lenguaje','laboratorio']:
 pref='docs/calidad' if k=='lenguaje' else 'laboratorio/tareas-watson';assert json.loads((R/(k+'-arbol.json')).read_text())[pref+'/'+ip+'/README.md']==blob(readme)
add=f'\n\n## S15 · {title} · RETP-{retp}\n\n{T}. [Contrato y evidencia]({url}). {row["resultado"]} {row["verificacion"]}\n\n{row["observaciones"]} **Siguiente:** {row["siguiente_accion"]} El catálogo recibe causas por etapa; las puertas y reservas P3/P4/P5/P6 conservan su estado.\n'
(O/'integracion-README.md').write_bytes(readme+add.encode())
maps={}
for k in ['laboratorio','lenguaje']:
 pref='docs/calidad' if k=='lenguaje' else 'laboratorio/tareas-watson';ep=pref+('/Inventario-sv/sucesos' if k=='lenguaje' else '/sucesos-sv');m={ep+'/'+n:str(O/n) for n in ['SUCESOS_SV.csv','SUCESOS_SV.md','HISTORIAL_SUCESOS_SV.csv']};m.update({pref+'/'+wp+'/'+str(p.relative_to(D)):str(p) for p in D.rglob('*') if p.is_file()});m[pref+'/'+ip+'/README.md']=str(O/'integracion-README.md')
 if k=='lenguaje':
  for ext in ['md','csv']:m[pref+'/REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext]=str(O/('registro.'+ext))
 maps[k]=m
(R/'MAPA.json').write_text(json.dumps(maps,ensure_ascii=False,indent=2)+'\n');(R/'ELIMINACIONES.json').write_text('{"lenguaje":[],"laboratorio":[]}\n')
assert rows[:15]==list(csv.DictReader((R.parent/'s14-bases/actualizacion-cierre/SUCESOS_SV.csv').open()))
for p in O.glob('*.csv'):
 z=list(csv.reader(p.open(newline='')));assert all(len(x)==len(z[0]) for x in z)
assert (O/'HISTORIAL_SUCESOS_SV.csv').read_bytes().startswith(hist)
print('S15 '+fase+' / RETP-'+retp+' preparado; históricos íntegros.')
