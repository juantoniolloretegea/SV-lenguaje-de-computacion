from pathlib import Path
import csv,io,json,sys
from datetime import datetime,timezone
sys.path.insert(0,'p1p3-bis');from gestion import csvbytes
R=Path('manifiesto-sv/checkout');E=R/'docs/calidad/Inventario-sv/sucesos';B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';L=R/'docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md'
def now():return datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z')
def rows():
 with (E/'SUCESOS_SV.csv').open(newline='') as f:r=csv.DictReader(f);return r.fieldnames,list(r)
def guardar(fields,rs,row):
 (E/'SUCESOS_SV.csv').write_bytes(csvbytes(fields,rs));p=E/'HISTORIAL_SUCESOS_SV.csv';old=p.read_bytes();hist=list(csv.DictReader(io.StringIO(old.decode())));rev=max([int(h['revision']) for h in hist if h['id']==row['id']],default=-1)+1;p.write_bytes(old+csvbytes(['revision']+fields,[dict(revision=rev,**row)],False))
 t='# Sucesos SV · Estado vigente\n\n[Reglas](README.md) · [CSV](SUCESOS_SV.csv) · [Historial](HISTORIAL_SUCESOS_SV.csv)\n\nFechas UTC. Las revisiones previas permanecen en el historial.\n\n'
 for x in rs:t+='## '+x['id']+' · '+x['actividad']+'\n\n'+'\n\n'.join('**'+k+':** '+(v or '—') for k,v in x.items() if k not in ['id','actividad'])+'\n\n'
 (E/'SUCESOS_SV.md').write_text(t)
def retp(num,title,t,row,link,kind,base,impact):
 for ext in ['md','csv']:
  p=R/('docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+ext);old=p.read_bytes();assert ('RETP-2026-'+str(num)).encode() not in old
  if ext=='md':add=('\n\n<a id="retp-'+str(num)+'"></a>\n\n### RETP-2026-'+str(num)+' · '+title+'\n\n'+t+'. '+row['resultado']+' '+row['verificacion']+' [Referencia]('+link+'). '+row['siguiente_accion']+' '+row['observaciones']+'\n').encode()
  else:
   fs=next(csv.reader(io.StringIO(old.decode())));values=['RETP-2026-'+str(num),t[:10],'',kind,row['id'],row['resultado'],'Instrucción expresa del autor; continuidad del workflow',base,'Sucesos CSV/MD e historial; Léame primero; estado y expediente',row['verificacion'],impact,row['observaciones'],row['siguiente_accion'],row['estado']];add=csvbytes(fs,[dict(zip(fs,values))],False)
  p.write_bytes(old+add)
if __name__=='__main__':
 f,rs=rows();assert rs[-1]['id']=='S23';t=now();r=dict.fromkeys(f,'');r.update(id='S24',estado='pendiente',fecha_alta_utc=t,fecha_actualizacion_utc=t,unidad_responsable='Watson / W-S0',actividad='Bis → catálogo y cierre de fase → análisis e instalación de la GUI',alcance='Seguimiento del relevo secuenciado hacia la GUI. Completar S22/(p1+p3)-Bis, después el catálogo de errores y el cierre de la fase aplicable; sólo entonces analizar, seleccionar e instalar la GUI.',repositorios_y_ramas='SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente',cortes_de_entrada='Lenguaje ba2f964e47aac7ec42a1d2eb3f5d97bf81e6af96; laboratorio b6186da2d0a0ee599dce620d88c349a49c6407a8',dependencias='S22 finalizado en su alcance; catálogo de errores cerrado; cierre documentado de la fase aplicable.',resultado='Secuencia registrada como suceso propio pendiente por instrucción expresa del autor.',verificacion='Estado pendiente, fechas de inicio y fin vacías; dependencia de S22 y de los cierres posteriores explícita. Alta con identificador consecutivo S24 y revisión 0.',evidencias='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md#s24--bis--catálogo-y-cierre-de-fase--análisis-e-instalación-de-la-gui',referencia_calidad='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-204',siguiente_accion='Mantener S24 pendiente mientras se ejecuta S22. Tras los cierres del Bis, catálogo y fase, abrir el análisis de GUI y registrar su inicio.',observaciones='S24 conserva el orden y la futura activación de GUI; no duplica ni altera el estado en ejecución de S22. No selecciona C#/.NET, Ratatui ni otra tecnología. No autoriza adelantar la instalación ni declara ejecutado un hito futuro.')
 rs.append(r);guardar(f,rs,r);retp(204,'S24 · Alta pendiente de la secuencia Bis, catálogo y GUI',t,r,'Inventario-sv/sucesos/SUCESOS_SV.md','ALTA_PENDIENTE','Instrucción humana; reglas Sucesos SV; RETP-203','Suceso pendiente identificado; S22 sigue en ejecución')
 p=B/'ESTADO_WORKFLOW.json';s=json.loads(p.read_text());s['suceso_secuencia_posterior']={'id':'S24','estado':'pendiente','registro':'RETP-2026-204','condicion':'Finalizar Bis, catálogo y fase antes del análisis e instalación de GUI'};p.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n')
 text=L.read_text();needle='**Orden humano vigente:**';text=text.replace(needle,'**S24 · pendiente / RETP-2026-204.** La secuencia tiene [suceso propio](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md). Sus fechas de inicio y fin permanecen vacías. S22 continúa en ejecución.\n\n'+needle);L.write_text(text)
 print(t)
