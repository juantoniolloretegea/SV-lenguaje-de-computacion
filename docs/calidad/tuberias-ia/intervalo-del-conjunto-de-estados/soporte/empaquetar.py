from pathlib import Path
import json,hashlib,base64,shutil,zipfile
r=Path(__file__).resolve().parent;out=r/'entrega/intervalo-del-conjunto-de-estados';out.mkdir(parents=True,exist_ok=True)
def sha(b):return hashlib.sha256(b).hexdigest()
def copy(src,dst=None):
 p=out/(dst or src);p.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(r/src,p)
for n in ['IDENTIDAD_ENTRADA.json','PLAN_PREVIO.md','COMPROMISO_PREVIO.json','CASOS_ESPERADOS.json','CASOS_NUEVOS.json','SUCESION_ESPERADOS.json','CASOS_HEREDADOS_160.json','BASE_FUENTES.json','CAMBIO_INCREMENTAL.patch','ejecutar.py','complemento.py','reproducir.py','focal.rs','focal_heredado_v2.rs','catalogo.rs','CATALOGO.json']:copy(n)
for n in ['preparar.py','modificar.py','empaquetar.py']:copy(n,'soporte/'+n)
for lang in ['es','en']:copy('idiomas/'+lang+'/diagnosticos.json')
for n in ['reproducir.py','focal.rs','corpus.rs','relacional.rs','CATALOGO.json','CORPUS.json','CORPUS_FUENTES.json','catalogo.rs','ESPERADO_RELACIONAL.json']:copy('antecedente-158/'+n)
for p in (r/'antecedente-158/clientes').iterdir():copy(str(p.relative_to(r)))
for folder in ['evidencia','complemento']:
 for p in (r/folder).iterdir():
  if p.is_file() and p.suffix in ['.json','.patch']:copy(str(p.relative_to(r)))
files=[];changes=[]
for p in sorted((r/'candidata').rglob('*')):
 if p.is_file():
  b=p.read_bytes();rel=str(p.relative_to(r/'candidata'));files.append(dict(ruta=rel,bytes=len(b),sha256=sha(b),base64=base64.b64encode(b).decode()))
  if b!=(r/'base'/rel).read_bytes():changes.append(rel)
assert changes==['rust/sv_core/src/frontend.rs'];assert len(files)==77
(out/'CANDIDATA_FUENTES.json').write_text(json.dumps({'archivos':files},ensure_ascii=False,indent=2)+'\n')
copy('candidata/rust/sv_core/src/frontend.rs','cambios/rust/sv_core/src/frontend.rs')
inventory={}
for version in ['base','candidata']:
 p=r/version/'rust/sv_core/src/frontend.rs';mentions=[];fn=None
 for n,line in enumerate(p.read_text().splitlines(),1):
  if line.strip().startswith('fn '):fn=line.strip().split('(')[0][3:]
  if 'FrontendError::' in line:mentions.append(dict(linea=n,funcion=fn,mencion=line.strip()))
 inventory[version]=dict(sha256=sha(p.read_bytes()),menciones=mentions)
inventory['nota']='Inventario estático de menciones, incluidas conversiones y pruebas; no certifica alcanzabilidad ni cobertura universal.'
inventory['pendientes_sin_rango']=['Conversión defensiva String::from_utf8 en tokenize y Nat::from_decimal en square_nat: sin campaña de alcanzabilidad en este incremento.', 'Conversión genérica From<FrontendError> para fallos sin procedencia.']
(out/'INVENTARIO_EMISORES.json').write_text(json.dumps(inventory,ensure_ascii=False,indent=2)+'\n')
logs=json.loads((r/'evidencia/PROCESOS.json').read_text());comp=json.loads((r/'complemento/PROCESOS.json').read_text())
metrics=dict(unidad='Procesos instrumentales, incluida compilación; no por caso',procesos_principales=len(logs),procesos_complementarios=len(comp),tiempo_principal_s=sum(x['duracion_ns'] for x in logs)/1e9,cpu_usuario_principal_s=sum(x['cpu_usuario_s'] for x in logs),cpu_sistema_principal_s=sum(x['cpu_sistema_s'] for x in logs),rss_max_acumulado_hijos_principal_kib=max(x['rss_max_acumulado_hijos_kib'] for x in logs),coste_monetario=None,coste_inferencia=None,comparacion_rendimiento='No constituida; los comprobadores de base y candidata realizan distinta verificación.')
(out/'MEDIDAS.json').write_text(json.dumps(metrics,ensure_ascii=False,indent=2)+'\n')
for p in (r/'intento-01').rglob('*'):
 if p.is_file():copy(str(p.relative_to(r)))
failed=json.loads((r/'intento-01/evidencia/PROCESOS.json').read_text())
metrics['intento_interrumpido']={'procesos':len(failed),'tiempo_s':sum(x['duracion_ns'] for x in failed)/1e9,'cpu_usuario_s':sum(x['cpu_usuario_s'] for x in failed),'cpu_sistema_s':sum(x['cpu_sistema_s'] for x in failed),'rss_max_acumulado_hijos_kib':max(x['rss_max_acumulado_hijos_kib'] for x in failed),'error':'E0786 al leer biblioteca optimizada de la base'}
(out/'MEDIDAS.json').write_text(json.dumps(metrics,ensure_ascii=False,indent=2)+'\n')
readme=(r/'README.md').read_text()
(out/'README.md').write_text(readme)
entries=[dict(ruta=str(p.relative_to(out)),bytes=p.stat().st_size,sha256=sha(p.read_bytes())) for p in sorted(out.rglob('*')) if p.is_file() and p.name not in ['MANIFIESTO.json','PAQUETE_REPRODUCIBLE.zip']]
(out/'MANIFIESTO.json').write_text(json.dumps(dict(algoritmo='SHA-256',archivos=entries),ensure_ascii=False,indent=2)+'\n')
with zipfile.ZipFile(out/'PAQUETE_REPRODUCIBLE.zip','w',zipfile.ZIP_DEFLATED,compresslevel=9) as z:
 for p in sorted(out.rglob('*')):
  if p.is_file() and p.name!='PAQUETE_REPRODUCIBLE.zip':z.write(p,str(p.relative_to(out)))
print(len(entries),'archivos más manifiesto y ZIP')
