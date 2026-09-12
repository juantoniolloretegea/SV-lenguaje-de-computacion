from pathlib import Path
from datetime import datetime,timezone
import json,csv,io,hashlib,shutil
R=Path(__file__).resolve().parent;P=R.parent/'s12-rectificacion/actualizacion';D=R/'entrega';O=R/'actualizacion';O.mkdir(exist_ok=True)
T=json.loads((D/'CORTE_Y_ALCANCE.json').read_text())['fecha_cierre_documental_utc'];blob=lambda b:hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest();sha=lambda b:hashlib.sha256(b).hexdigest()
def js(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
ip='tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores';wp=ip+'/s13-suficiencia-semantica-ir';pub='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion';url=pub+'/blob/main/docs/calidad/'+wp+'/ACTA_SUFICIENCIA.md'
tr=json.loads((R/'lenguaje-arbol.json').read_text());rect=json.loads((R.parent/'s12-rectificacion/entrega/RECTORES.json').read_text())
for p,h in rect['piezas_sin_cambio_desde_lectura_previa'].items():assert tr[p]==h,p
rect['corte']=json.loads((R/'lenguaje-base.json').read_text())['head'];rect['objeto_actual']='Mapa documental S13 de obligación, sede, contrato, IR, función Rust, evidencia y pérdida. Sin modificación nuclear ni prueba funcional nueva.';js(D/'RECTORES.json',rect)
for k in ['lenguaje','laboratorio']:
 tree=json.loads((R/(k+'-arbol.json')).read_text());pref='docs/calidad/Inventario-sv/sucesos' if k=='lenguaje' else 'laboratorio/tareas-watson/sucesos-sv'
 for n in ['SUCESOS_SV.csv','SUCESOS_SV.md','HISTORIAL_SUCESOS_SV.csv']:assert tree[pref+'/'+n]==blob((P/n).read_bytes())
for ext in ['md','csv']:assert tr['docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext]==blob((P/('registro.'+ext)).read_bytes())
with (P/'SUCESOS_SV.csv').open(newline='') as f:rd=csv.DictReader(f);fields=rd.fieldnames;rows=list(rd)
assert [r['id'] for r in rows]==['S'+str(i) for i in range(13)]
# La recepción comenzó antes del asiento; no se simula una fijación previa.
start=datetime.fromtimestamp((R/'lenguaje-base.json').stat().st_mtime,timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z')
row={k:'' for k in fields};row.update(id='S13',estado='finalizado',fecha_alta_utc=T,fecha_inicio_utc=start,fecha_actualizacion_utc=T,fecha_fin_utc=T,unidad_responsable='Watson / W-S0',actividad='Cotejo de suficiencia para integración 1+3: contrato, IR y realización',alcance='Doce obligaciones, criterios A–L preservados; representación nominal, realización pendiente y sede exterior distinguidas. Sin nuevo ensayo funcional ni promoción nuclear.',repositorios_y_ramas='SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente',cortes_de_entrada='Lenguaje '+rect['corte']+'; laboratorio '+json.loads((R/'laboratorio-base.json').read_text())['head'],dependencias='S12 / RETP-181; rectores vigentes; IR 0.3 y herencia aplicable; deuda actualizada; evidencia S11.',resultado='Mapa documental completado. Suficiencia integral no acreditada: K1-T nominal no habilita transducción; Query declarada no realiza CQ1–CQ6; frontera y candidatas no suplen núcleo.',verificacion='23 fuentes por blob/longitud/SHA-256; 31 pasajes exactos; 12 obligaciones y criterios A–L; comprobador de integridad documental. Cero ensayos funcionales nuevos.',evidencias=url,referencia_calidad=pub+'/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-182',siguiente_accion='Fijar G/J documental: base realmente consumida, recuperación original y reevaluación distinguida; no llamarlo QueryResult/transición nativa ni continuidad durable.',observaciones='Registro al cerrar trabajo documental ya iniciado; fecha de inicio derivada del archivo local de recepción del corte, no de una fijación experimental. Dos interrupciones por intervalos de extracción fuera de rango conservadas. Agentes por valorar tras inmunología.')
rows.append(row)
s=io.StringIO(newline='');w=csv.DictWriter(s,fieldnames=fields,lineterminator='\n');w.writeheader();w.writerows(rows);(O/'SUCESOS_SV.csv').write_text(s.getvalue())
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
entry=f'\n\n<a id="retp-182"></a>\n\n### RETP-2026-182 · S13 · Suficiencia de representación y realización\n\n{T}. [Acta y matriz]({wp}/ACTA_SUFICIENCIA.md). Doce obligaciones enlazan criterios A–L con sede, contrato, IR, función Rust, evidencia y pérdida. Se distingue K1-T nominal, productor QueryResult/CQ1–CQ6 pendiente, cobertura de Frame frente a exhaustividad, cobertura documental frente a verificadores R1, main frente a candidatas y frontera material. 23 fuentes verificadas, 31 pasajes exactos; sin nueva prueba funcional ni promoción nuclear. Mapa completado no equivale a suficiencia integral. Se conserva S12: agentes por valorar tras inmunología. El siguiente objeto G/J se delimita a base documental efectivamente consumida y recuperación/reevaluación diferenciadas; las operaciones dependientes de capacidades nucleares pendientes no quedan habilitadas. No se alteran deudas ni criterios; causas se recogen para el catálogo.\n'
(O/'registro.md').write_bytes((P/'registro.md').read_bytes()+entry.encode());s=io.StringIO(newline='');csv.writer(s,lineterminator='\n').writerow(['RETP-2026-182',T[:10],'','COTEJO_DOCUMENTAL','S13 / suficiencia integración 1+3','Vincular obligaciones con contrato, IR, realización y evidencia','Relevo S12 / RETP-181 y luz verde de Dirección','IR 0.3; herencia 0.2; rectores; DFL; S11','Acta, matriz, pasajes, fuentes, comprobador, Sucesos y espejo','23 fuentes; 31 pasajes; 12 obligaciones; integridad comprobada; cero ensayos funcionales nuevos','Mapa completado; suficiencia integral no acreditada','Consulta declarada no es QueryResult; K1-T nominal; fronteras y candidatas no suplen núcleo','Fijar G/J documental con identidad de carga y recuperación/reevaluación; conservar puertas','finalizado']);(O/'registro.csv').write_bytes((P/'registro.csv').read_bytes()+s.getvalue().encode())
readme=(P/'integracion-README.md').read_bytes()
for k in ['lenguaje','laboratorio']:
 pref='docs/calidad' if k=='lenguaje' else 'laboratorio/tareas-watson';assert json.loads((R/(k+'-arbol.json')).read_text())[pref+'/'+ip+'/README.md']==blob(readme)
add=f'\n\n## Suficiencia contrastada · S13 / RETP-182\n\n{T}. [Acta, matriz y custodia]({url}). Mapa documental completado: doce obligaciones enlazadas a contrato, IR, funciones Rust y evidencia. Suficiencia integral no acreditada: K1-T conserva límite de representación; QueryResult/CQ1–CQ6 y evolución ejecutiva requieren productores; host, carga, pantalla y acto profesional tienen obligaciones exteriores. No se propone una modificación nuclear por este mapa ni se promueven candidatas.\n\n**Siguiente objeto delimitado:** fijar G/J en lectura documental —base realmente consumida, recuperación original y reevaluación separadas— antes de ejecutar. No se lo atribuirá a QueryResult nativo, transición SV o continuidad durable. Todo consumidor que necesite estas capacidades deberá satisfacer sus obligaciones nucleares antes de admitirse. Se conserva la rectificación S12 y la decisión sobre agentes después de cerrar inmunología; el catálogo recoge causas durante la integración.\n'
(O/'integracion-README.md').write_bytes(readme+add.encode())
assert rows[:-1]==list(csv.DictReader((P/'SUCESOS_SV.csv').open()))
for p in O.glob('*.csv'):
 z=list(csv.reader(p.open(newline='')));assert all(len(x)==len(z[0]) for x in z),p
# Mapa de publicación se finaliza después de copiar scripts y manifiesto.
maps={}
for k in ['laboratorio','lenguaje']:
 pref='docs/calidad' if k=='lenguaje' else 'laboratorio/tareas-watson';ep=pref+('/Inventario-sv/sucesos' if k=='lenguaje' else '/sucesos-sv');m={ep+'/'+n:str(O/n) for n in ['SUCESOS_SV.csv','SUCESOS_SV.md','HISTORIAL_SUCESOS_SV.csv']};m.update({pref+'/'+wp+'/'+str(p.relative_to(D)):str(p) for p in D.rglob('*') if p.is_file()});m[pref+'/'+ip+'/README.md']=str(O/'integracion-README.md')
 if k=='lenguaje':
  for ext in ['md','csv']:m[pref+'/REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext]=str(O/('registro.'+ext))
 maps[k]=m
js(R/'MAPA.json',maps);js(R/'ELIMINACIONES.json',{'laboratorio':[],'lenguaje':[]})
print('S13 / RETP-182: documentos y registros preparados; S0–S12 e históricos íntegros.')
