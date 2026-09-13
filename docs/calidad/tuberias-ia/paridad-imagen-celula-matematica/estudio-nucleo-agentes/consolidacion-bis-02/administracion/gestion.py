from pathlib import Path
import sys,json,csv,io,hashlib,base64
from datetime import datetime,timezone
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'s6-trazabilidad-total'))
from github_io import call,fetch
R=Path(__file__).resolve().parent
PUB='juantoniolloretegea/SV-lenguaje-de-computacion'
PRIV='juantoniolloretegea/SV-matematica-semantica-cuaternaria'
ROOT='docs/calidad/tuberias-ia/paridad-imagen-celula-matematica'
LABROOT='laboratorio/tareas-watson/tuberias-ia/paridad-imagen-celula-matematica'
EVENT='docs/calidad/Inventario-sv/sucesos'
BASE=R/'fuentes/SV-lenguaje-de-computacion'
def now():return datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z')
def sha(b):return hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
def put(p,s):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(s,encoding='utf-8')
def csvbytes(fields,rows,header=True):
 s=io.StringIO(newline='');w=csv.DictWriter(s,fieldnames=fields,lineterminator='\n')
 if header:w.writeheader()
 w.writerows(rows);return s.getvalue().encode()
def registros(stage,closed=False):
 out=R/stage;out.mkdir(exist_ok=True);previous=R/'apertura' if closed else BASE/EVENT
 with (previous/'SUCESOS_SV.csv').open(newline='') as f:r=csv.DictReader(f);fields=r.fieldnames;rows=list(r)
 with (previous/'HISTORIAL_SUCESOS_SV.csv').open(newline='') as f:hist=list(csv.DictReader(f))
 T=now();num=193 if closed else 192
 if closed:
  row=rows[-1];assert row['id']=='S20';row.update(estado='finalizado',fecha_fin_utc=T,fecha_actualizacion_utc=T,resultado='Explicación pública MD/PDF y recepción documental (p1+p3)-Bis. Suficiencia integrada, elección de IA y promoción nuclear pendientes.',verificacion='Fuentes cotejadas contra blobs Git; revisión estática y testigo de índices; PDF renderizado y revisado; copias verificadas al publicar.',siguiente_accion='Fijar contrato y banco de paridad posicional, semántica y operacional; resolver hallazgos del antecedente antes de reutilizarlo; después derivar causas comprobadas al catálogo.')
  revision=1
 else:
  assert rows[-1]['id']=='S19'
  row=dict.fromkeys(fields,'');row.update(id='S20',estado='en ejecución',fecha_alta_utc=T,fecha_inicio_utc=T,fecha_actualizacion_utc=T,unidad_responsable='Watson / W-S0',actividad='(p1+p3)-Bis: paridad entre célula matemática, imagen y uso por agentes',alcance='Recepción de antecedentes 2021 y SVperitus; explicación pública y estudio documental antes del catálogo. Sin cambio del núcleo ni entrenamiento.',repositorios_y_ramas=PUB+': main; '+PRIV+': lab/playground-sv-permanente; SVperitus-dataset: main (lectura)',cortes_de_entrada='Lenguaje 19540321089dc48e4239e1f88324d1056c8caff4; SVperitus 47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c; laboratorio 59ff0937c3c7d50f390116f05baacb2f91a37b38',dependencias='S19 / RETP-191; instrucción expresa de Juan Antonio; Pilares; perfiles; transición; antecedente 2021; código SVperitus.',resultado='Recepción documental iniciada. PDF aportado contiene índice de siete páginas y enlaces; no los capítulos completos.',verificacion='Lectura iniciada antes del alta; se declara sin retrofechar. Sin ejecución de modelos.',siguiente_accion='Preparar explicación MD/PDF, hallazgos y obligaciones pendientes, conservar copias y registrar el cierre documental.',observaciones='La habilitación del catálogo de S19 se conserva como antecedente. Este estudio previo la complementa por instrucción humana. Agentes sin calendario impuesto; inmunología conserva su secuencia. Ningún fallo técnico se transforma en U.')
  rows.append(row);revision=0
 if closed:
  row['cortes_de_entrada']+='; ampliación de fundamentos Lenguaje fa3eb727799c322090e4e9126f81238cd198b9cc'
  row['observaciones']+=' Primacía algebraico-semántica reafirmada por el autor durante la revisión. Discrepancias de antecedentes documentadas sin corregir originales.'
 row['evidencias']='https://github.com/'+PUB+'/blob/main/'+ROOT+'/estudio-nucleo-agentes/README.md'
 row['referencia_calidad']='https://github.com/'+PUB+'/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-'+str(num)
 (out/'SUCESOS_SV.csv').write_bytes(csvbytes(fields,rows))
 hbytes=(previous/'HISTORIAL_SUCESOS_SV.csv').read_bytes();(out/'HISTORIAL_SUCESOS_SV.csv').write_bytes(hbytes+csvbytes(['revision']+fields,[dict(revision=revision,**row)],False))
 text='# Sucesos SV · Estado vigente\n\n[Reglas](README.md) · [CSV](SUCESOS_SV.csv) · [Historial](HISTORIAL_SUCESOS_SV.csv)\n\nFechas UTC. Las revisiones previas permanecen en el historial.\n\n'
 for x in rows:
  text+='## '+x['id']+' · '+x['actividad']+'\n\n'+'\n\n'.join('**'+k+':** '+(v or '—') for k,v in x.items() if k not in ['id','actividad'])+'\n\n'
 put(out/'SUCESOS_SV.md',text)
 regprev=R/'apertura' if closed else BASE/'docs/calidad'
 for ext in ['md','csv']:
  old=(regprev/('REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext)).read_bytes()
  if ext=='md':addition=('\n\n<a id="retp-'+str(num)+'"></a>\n\n### RETP-2026-'+str(num)+' · S20 · '+('Recepción documental' if closed else 'Apertura')+' de (p1+p3)-Bis\n\n'+T+'. '+row['resultado']+' '+row['verificacion']+' [Expediente](tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/README.md). '+row['siguiente_accion']+' Sin modificación de semántica, IR, Rust, modelos o parámetros de dominio.\n').encode()
  else:
   f=next(csv.reader(io.StringIO(old.decode())));vals=['RETP-2026-'+str(num),T[:10],'','RECEPCION_DOCUMENTAL','S20 / (p1+p3)-Bis',row['resultado'],'Recepción expresa del autor antes del catálogo','Pilares; perfiles; transición; S19; SVperitus','MD/PDF, estudio, sucesos, referencias y espejo',row['verificacion'],'Estudio previo al catálogo; sin promoción nuclear','Capacidad NLP/visión y paridad integrada no acreditadas',row['siguiente_accion'],row['estado']];addition=csvbytes(f,[dict(zip(f,vals))],False)
  (out/('REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext)).write_bytes(old+addition)
 return out
def publicar(stage,maps):
 checkpoint=R/('PUBLICACION_'+stage+'.json');state=json.loads(checkpoint.read_text()) if checkpoint.exists() else {}
 def save():put(checkpoint,json.dumps(state,ensure_ascii=False,indent=2)+'\n')
 for repo,branch,mapping in maps:
  api='https://api.github.com/repos/'+repo
  s=state.get(repo)
  if not s:
   h=fetch(api+'/git/ref/heads/'+branch)['object']['sha'];tree=fetch(api+'/git/trees/'+h+'?recursive=1');assert not tree.get('truncated');s=dict(base=h,base_tree=tree['sha'],original={x['path']:x['sha'] for x in tree['tree'] if x['type']=='blob'},files={});state[repo]=s;save()
  if s.get('verified'):continue
  if not s.get('published'):assert fetch(api+'/git/ref/heads/'+branch)['object']['sha']==s['base'],'Base cambió'
  for path,local in mapping.items():
   if local is None:s['files'][path]=None;continue
   data=Path(local).read_bytes();expected=sha(data)
   if s['files'].get(path)==expected:continue
   b=call('create_blob',dict(repository_full_name=repo,content=base64.b64encode(data).decode(),encoding='base64'));assert b['sha']==expected;s['files'][path]=expected;save()
  if not s.get('published'):
   tr=call('create_tree',dict(repository_full_name=repo,base_tree_sha=s['base_tree'],tree_elements=[dict(path=p,mode='100644',type='blob',sha=h) for p,h in s['files'].items()]));s['tree']=tr['sha'];save()
   co=call('create_commit',dict(repository_full_name=repo,message='docs: '+stage+' (p1+p3)-Bis; '+('S22' if stage.startswith('workflow-') else ('S21' if stage.startswith('v2-') else 'S20')),parent_sha=s['base'],tree_sha=s['tree']));s['commit']=co['sha'];save()
   assert fetch(api+'/git/ref/heads/'+branch)['object']['sha']==s['base']
   call('update_ref',dict(repository_full_name=repo,branch_name=branch,sha=s['commit'],force=False));s['published']=True;save()
  t=fetch(api+'/git/trees/'+s['commit']+'?recursive=1');assert not t.get('truncated');actual={x['path']:x['sha'] for x in t['tree'] if x['type']=='blob'}
  expected=dict(s['original'])
  for p,h in s['files'].items():
   if h is None:expected.pop(p,None)
   else:expected[p]=h
  assert actual==expected,'Árbol remoto distinto del cambio previsto'
  s['verified']=True;save();print(repo,s['commit'],'VERIFICADO',flush=True)
 return state
def mapas(stage,files):
 out=R/stage;pub={ROOT+'/'+p:str(f) for p,f in files.items()};priv={LABROOT+'/'+p:str(f) for p,f in files.items()}
 for n in ['SUCESOS_SV.csv','SUCESOS_SV.md','HISTORIAL_SUCESOS_SV.csv']:
  pub[EVENT+'/'+n]=str(out/n);priv['laboratorio/tareas-watson/sucesos-sv/'+n]=str(out/n)
 for ext in ['csv','md']:pub['docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext]=str(out/('REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext))
 return [(PRIV,'lab/playground-sv-permanente',priv),(PUB,'main',pub)]
if __name__=='__main__':
 out=registros('apertura');p=out/'README.md';put(p,'# Estudio (p1+p3)-Bis\n\nS20 · RETP-192 · Recepción documental en ejecución.\n\nObjeto: correspondencia entre célula matemática, imagen y uso por agentes. SVperitus es la sede natural de agentes y antecedentes. El Lenguaje recibe requisitos de paridad y suficiencia antes del catálogo.\n\nSe revisan el antecedente 2021, la impresión del índice de la colección y el código sintético de SVperitus en cortes identificados en Sucesos SV. No se cambian el núcleo, la IR ni los agentes.\n\nEl molde gráfico común no acredita igualdad de significados, decisiones o autoridad. La posible reunión de NLP y visión en una misma IA permanece por contrastar.\n');m=mapas('apertura',{'estudio-nucleo-agentes/README.md':p});m[1][2][ROOT+'/estudio-nucleo-agentes/inicio.md']=None;publicar('apertura',m)
