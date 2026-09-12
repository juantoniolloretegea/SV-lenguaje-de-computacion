from pathlib import Path
from datetime import datetime,timezone
import json,csv,io,hashlib,sys
R=Path(__file__).resolve().parent;D=R/'entrega';fase=sys.argv[1];assert fase in ['apertura','cierre'];closing=fase=='cierre'
P=R/'actualizacion-apertura' if closing else R.parent/'s10-ranquin/actualizacion';O=R/('actualizacion-'+fase);O.mkdir(exist_ok=True)
T=datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z');retp='180' if closing else '179'
blob=lambda b:hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
for k in ['lenguaje','laboratorio']:
 tree=json.loads((R/(k+'-arbol.json')).read_text());prefix='docs/calidad/Inventario-sv/sucesos' if k=='lenguaje' else 'laboratorio/tareas-watson/sucesos-sv'
 for n in ['SUCESOS_SV.csv','SUCESOS_SV.md','HISTORIAL_SUCESOS_SV.csv']:assert tree[prefix+'/'+n]==blob((P/n).read_bytes()),n
tr=json.loads((R/'lenguaje-arbol.json').read_text())
for ext in ['md','csv']:assert tr['docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext]==blob((P/('registro.'+ext)).read_bytes())
ip='tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores';wp=ip+'/s11-contexto-y-cobertura';pub='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion';url=pub+'/blob/main/docs/calidad/'+wp+'/README.md'
with (P/'SUCESOS_SV.csv').open(newline='') as f:rd=csv.DictReader(f);fields=rd.fieldnames;rows=list(rd)
if closing:row=rows[-1].copy();assert row['id']=='S11'
else:
 assert [x['id'] for x in rows]==['S'+str(i) for i in range(11)]
 row={k:'' for k in fields};row.update(id='S11',fecha_alta_utc=T,fecha_inicio_utc=T,unidad_responsable='Watson / W-S0',actividad='Integración A/H: recepción de documento externo, cobertura e identidad contextual',alcance='Diez casos sintéticos nativos; referencia S2 independiente, orden externa como dato, cobertura y archivo S3; identidad, integridad y límite de contexto.',repositorios_y_ramas='SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente',cortes_de_entrada='Lenguaje '+json.loads((R/'lenguaje-base.json').read_text())['head']+'; laboratorio '+json.loads((R/'laboratorio-base.json').read_text())['head'],dependencias='S10; S1/S2/S3; matriz RETP-162 reconciliada; autorización expresa de continuación por Dirección.')
row.update(estado='finalizado' if closing else 'en ejecución',fecha_actualizacion_utc=T,fecha_fin_utc=T if closing else '',evidencias=url,referencia_calidad=pub+'/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-'+retp,observaciones='Montaje de laboratorio y host confiable; documento externo no semántico ni autoridad. No modelo LLM ejecutado, promoción nuclear, pantalla, persistencia hostil ni cierre universal A–L. Reserva P3 y P4/P5/P6 intactos.')
if closing:
 res=json.loads((D/'RESULTADO.json').read_text());assert res['conforme']
 row.update(resultado='Conforme dentro del montaje S11: orden conservada como dato, omisiones y contexto ajeno/alterado rechazados antes de escritura; recibo negativo íntegro.',verificacion=f"60 observaciones de diez casos; seis ejecuciones debug/release; {res['capturas_identicas_por_ejecucion']} capturas idénticas por ejecución; 22 invocaciones; dos sensibilidades detectadas en AH03 y AH06.",siguiente_accion='Fijar siguiente contraste G/J de sustitución de base y reevaluación distinguida, reutilizando identidad/custodia existentes; catálogo ES/EN posterior a la integración aplicable.');rows[-1]=row
else:
 row.update(resultado='Contrato, casos, esperados, código y presupuesto fijados antes de compilar. Preparación material iniciada; campaña funcional todavía no ejecutada.',verificacion='Lectura íntegra de rectores; cortes cotejados; fuentes reutilizadas idénticas; esperado anterior S2 comprobado por contenedor publicado; Rust recuperado con binario idéntico.',siguiente_accion='Ejecutar reproducir.py con presupuesto de 22 invocaciones; detener y conservar cualquier fallo inesperado.');rows.append(row)
s=io.StringIO(newline='');w=csv.DictWriter(s,fieldnames=fields,lineterminator='\n');w.writeheader();w.writerows(rows);(O/'SUCESOS_SV.csv').write_text(s.getvalue())
hist=(P/'HISTORIAL_SUCESOS_SV.csv').read_bytes();s=io.StringIO(newline='');csv.DictWriter(s,fieldnames=['revision']+fields,lineterminator='\n').writerow(dict(revision=1 if closing else 0,**row));(O/'HISTORIAL_SUCESOS_SV.csv').write_bytes(hist+s.getvalue().encode())
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
title=('Cierre acotado' if closing else 'Fijación previa')+' de recepción contextual y cobertura A/H'
entry=f'\n\n<a id="retp-{retp}"></a>\n\n### RETP-2026-{retp} · S11 · {title}\n\n{T}. {row["resultado"]} {row["verificacion"]} [Contrato y evidencia]({wp}/README.md). {row["observaciones"]} Siguiente: {row["siguiente_accion"]}\n'
(O/'registro.md').write_bytes((P/'registro.md').read_bytes()+entry.encode());s=io.StringIO(newline='');csv.writer(s,lineterminator='\n').writerow(['RETP-2026-'+retp,T[:10],'','CIERRE_ACOTADO' if closing else 'FIJACION_PREVIA','S11 / integración 1+3',title,'Continuación autorizada y hueco A/H de matriz reconciliada','Rectores íntegros; RETP-141/147/162/178; S2/S3','Contrato; Rust local; banco; esperados; fijación; matriz; Sucesos; espejo',row['verificacion'],'Recepción contextual mínima de laboratorio; ninguna promoción nuclear',row['observaciones'],row['resultado'],row['estado']]);(O/'registro.csv').write_bytes((P/'registro.csv').read_bytes()+s.getvalue().encode())
readme=(P/'integracion-README.md').read_bytes()
for k in ['lenguaje','laboratorio']:
 tree=json.loads((R/(k+'-arbol.json')).read_text());pref='docs/calidad' if k=='lenguaje' else 'laboratorio/tareas-watson';assert tree[pref+'/'+ip+'/README.md']==blob(readme)
text=f'\n\n## S11 · {title} · RETP-{retp}\n\n> **Relevo vigente:** {T}. {row["resultado"]}\n\n{row["verificacion"]} [S11]({url}). {row["observaciones"]}\n\n**Siguiente acción:** {row["siguiente_accion"]} Se mantiene núcleo → cierre de universos constituidos, prioritariamente inmunología → agentes, sin anticipar constituciones de dominio o capacidades de agente. Ciberseguridad se delimitará con su evidencia, sin presumir suficiente el primer universo.\n'
(O/'integracion-README.md').write_bytes(readme+text.encode())
maps={}
for k in ['laboratorio','lenguaje']:
 pref='docs/calidad' if k=='lenguaje' else 'laboratorio/tareas-watson';ep=pref+('/Inventario-sv/sucesos' if k=='lenguaje' else '/sucesos-sv')
 m={ep+'/'+n:str(O/n) for n in ['SUCESOS_SV.csv','SUCESOS_SV.md','HISTORIAL_SUCESOS_SV.csv']};m.update({pref+'/'+wp+'/'+str(p.relative_to(D)):str(p) for p in D.rglob('*') if p.is_file()});m[pref+'/'+ip+'/README.md']=str(O/'integracion-README.md')
 if k=='lenguaje':
  for ext in ['md','csv']:m[pref+'/REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext]=str(O/('registro.'+ext))
 maps[k]=m
(R/'MAPA.json').write_text(json.dumps(maps,ensure_ascii=False,indent=2)+'\n');(R/'ELIMINACIONES.json').write_text('{"lenguaje":[],"laboratorio":[]}\n')
for p in O.glob('*.csv'):
 z=list(csv.reader(p.open(newline='')));assert all(len(x)==len(z[0]) for x in z)
assert rows[:11]==list(csv.DictReader((R.parent/'s10-ranquin/actualizacion/SUCESOS_SV.csv').open()))
assert (O/'HISTORIAL_SUCESOS_SV.csv').read_bytes().startswith(hist)
print('S11 '+fase+' / RETP-'+retp+' preparado; históricos intactos.')
