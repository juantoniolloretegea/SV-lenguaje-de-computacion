from pathlib import Path
import json,hashlib,tarfile,gzip,io,shutil,subprocess,csv
R=Path('manifiesto-sv/checkout');B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';D=B/'bis04-realizacion-i0205-v0_1';C=Path('bis-realizacion/reproduccion-r01/ejecucion');base='2067310b30e5aee76120444f0166b01390a04970'
def h(b):return hashlib.sha256(b).hexdigest()
def dump(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
assert json.loads((C/'resultados/RESUMEN.json').read_text())['fallos']==0
assert all(x['conforme'] for x in json.loads((C/'ENCAPSULACION.json').read_text()))
for x in json.loads((C/'ORDENES.json').read_text()):
 if x['orden'][0] in ('cargo',) or x['orden'][0].endswith('sv_bis_i0205'):assert x['exit_code']==0
r01=json.loads((D/'ADENDA_R01_PREVIA.json').read_text());assert h((D/'proyecto/src/lib.rs').read_bytes())==r01['fuente_corregida_sha256']
for p,sha in json.loads((D/'ENTRADAS_MATERIALIZADAS.json').read_text()).items():assert h((Path('bis-realizacion/reproduccion-r01/entradas')/p).read_bytes())==sha
pub=json.loads(Path('bis-realizacion/R01_PUBLICO.json').read_text());lab=json.loads(Path('bis-realizacion/R01_LAB.json').read_text());assert pub['verified'] and lab['verified']
dump(C/'CORTE.json',{'precompromiso_correccion_publico':pub['commit'],'precompromiso_correccion_laboratorio':lab['commit'],'ejecutable_sha256':h(Path('bis-realizacion/reproduccion-r01/proyecto/target/debug/sv_bis_i0205').read_bytes()),'fuente_lib_sha256':r01['fuente_corregida_sha256'],'exit_campana':0})
mem=io.BytesIO();hashes={}
with tarfile.open(fileobj=mem,mode='w') as t:
 for p in sorted(C.rglob('*')):
  if not p.is_file() or p.suffix=='.rlib':continue
  data=p.read_bytes();name='campana-02/'+str(p.relative_to(C));info=tarfile.TarInfo(name);info.size=len(data);info.mode=0o644;t.addfile(info,io.BytesIO(data));hashes[name]=h(data)
(D/'EVIDENCIA_NATIVA_R01.tar.gz').write_bytes(gzip.compress(mem.getvalue(),mtime=0));dump(D/'HUELLAS_EVIDENCIA_R01.json',hashes)
results=json.loads((D/'RESULTADOS.json').read_text());results['segunda_campana']={'resumen':json.loads((C/'resultados/RESUMEN.json').read_text()),'recursos':json.loads((C/'RECURSOS.json').read_text()),'sondas':json.loads((C/'ENCAPSULACION.json').read_text()),'corte':json.loads((C/'CORTE.json').read_text()),'prueba_lectura_agregada':'conforme; R01 y 6145 bytes leídos; test.stdout archivado','discrepancias_integradas':{}}
for i in range(1,27):
 x=json.loads((C/'resultados'/f'I0205-{i:02}.json').read_text());assert x['discrepancias']==[];results['segunda_campana']['discrepancias_integradas'][x['id']]=x['discrepancias']
dump(D/'RESULTADOS.json',results)
p=D/'README.md';s=p.read_text();s=s.replace('Los 26 casos integrados coinciden','En las dos campañas conservadas, los 26 casos integrados coinciden')
s=s.replace('| Test interno de almacenamiento |','| Test de lectura agregada R01 | Conforme en segunda campaña | Saldo 6144; lectura detenida en 6145 bytes incluyendo detector de exceso |\n| Test interno de almacenamiento |')
s=s.replace('Se ejecutaron **30 invocaciones de admisión**:','Se ejecutaron **60 invocaciones de admisión en dos campañas**, 30 por campaña:')
s=s.replace('Los 202 casos originales C02–C12,','La segunda campaña repite las ocho sondas y añade el test de saldo agregado: dos tests internos conformes. Los 202 casos originales C02–C12,')
s=s.replace('[HUELLAS_EVIDENCIA.json](HUELLAS_EVIDENCIA.json) identifica los contenidos del archivo.','[HUELLAS_EVIDENCIA.json](HUELLAS_EVIDENCIA.json) identifica los contenidos del archivo. La versión corregida y final está acreditada por [EVIDENCIA_NATIVA_R01.tar.gz](EVIDENCIA_NATIVA_R01.tar.gz) y [HUELLAS_EVIDENCIA_R01.json](HUELLAS_EVIDENCIA_R01.json).')
s=s.replace('La reproducción consolidada se entrega para uso posterior; la ejecución registrada utilizó ejecutar.py/continuar.py y las órdenes literales archivadas.','La primera campaña utilizó ejecutar.py/continuar.py. La segunda utilizó este mismo script de reproducción desde un directorio nuevo y terminó correctamente; se conservan las órdenes literales de ambas.')
s=s.replace('PRECOMPROMISO.json es una instantánea histórica de ese corte; README se amplía ahora con los resultados.','PRECOMPROMISO.json es una instantánea histórica de ese corte: conserva la identidad del código anterior y el README previo, no pretende ser un manifiesto del expediente final. La corrección R01 y su prueba adicional quedaron fijadas antes de repetir en laboratorio `6ece56f9684f6b4a1045d954066301d64ac96129` y público `d9cea830eacc3a0f808ba88725a514867c57e643`; ADENDA_R01_PREVIA.json conserva ese esperado sin rellenarlo retroactivamente.')
s=s.replace('No se alteraron los bancos para corregir estas incidencias.','La revisión posterior detectó que el saldo agregado se comprobaba después de leer cada canal. Se corrigió la reserva/lectura para limitarla al saldo antes de recibir; una prueba nativa nueva exige detenerse en 6145 bytes cuando quedan 6144. La versión previa de lib.rs se conserva. No se alteraron los bancos para corregir estas incidencias.')
s=s.replace('En la única ejecución medida del binario completo','En la primera ejecución medida del binario completo')
s=s.replace('El intervalo interno fue 100870607 ns.','El intervalo interno fue 100870607 ns. En la segunda campaña, correspondiente a la versión final, se observaron **12032 KiB de RSS máximo**, **106749319 ns externos** y **104407621 ns internos**. Los costes incluyen conductor y observador; no son cotas del núcleo.')
p.write_text(s)
for n in ['ajuste_r01.py','publicar_r01.py','registrar.py','cerrar.py']:shutil.copyfile(Path('bis-realizacion')/n,D/'soporte'/n)
# ES: Conservación exacta de fuentes y bancos; historial append-only.
# EN: Exact source/bank preservation; append-only history.
def old(p):return subprocess.check_output(['git','show',base+':'+p],cwd=R)
for p,v in json.loads((B/'bis03-sedes-i0205-v0_1/FUENTES.json').read_text())['archivos'].items():assert h((R/p).read_bytes())==v['sha256'] and (R/p).read_bytes()==old(p)
for folder in ('integracion-c02-c05-v0_1','bis03-sedes-i0205-v0_1'):
 for p,sha in json.loads((B/folder/'MANIFIESTO.json').read_text()).items():assert h((B/folder/p).read_bytes())==sha
for p,sha in json.loads((D/'NUCLEO_FUENTES.json').read_text())['archivos'].items():assert h((R/'rust/sv_core'/p).read_bytes())==sha
append=['docs/calidad/Inventario-sv/sucesos/HISTORIAL_SUCESOS_SV.csv','docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv','docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md']
for p in append:assert (R/p).read_bytes().startswith(old(p))
p='docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv';a=list(csv.DictReader(io.StringIO(old(p).decode())));b=list(csv.DictReader((R/p).open()));assert len(a)==len(b);assert [x['id'] for x,y in zip(a,b) if x!=y]==['S22']
expected=set(append+['docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv','docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md',str((B/'ESTADO_WORKFLOW.json').relative_to(R)),'docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md'])
assert set(subprocess.check_output(['git','diff','--name-only'],cwd=R,text=True).splitlines())==expected
workflow=json.loads((B/'ESTADO_WORKFLOW.json').read_text());assert workflow['integracion_c02_c05']['casos_ejecutados']==26;assert workflow['escenarios_ejecutados']==2
for e in workflow['etapas']:
 if e['id'] in ('BIS-02','BIS-03','BIS-04'):assert e['estado']=='en ejecución'
dump(D/'CONSERVACION.json',{'resultado':'CONFORME','corte':base,'suceso_actualizado':'S22','otros_sucesos':'intactos, incluidos S24 y S25','append_only':append,'bancos_previos':'identidad/bytes íntegros; no modificar observados históricos','fuentes_rust_productivas':'intactas','realizacion':'crate separado; copia exacta de núcleo','campanas':2,'variantes_integradas_unicas':26,'invocaciones_admision':60,'rust_version':subprocess.check_output(['rustc','--version'],text=True).strip(),'cargo_version':subprocess.check_output(['cargo','--version'],text=True).strip(),'incidencias_conservadas':['lockfile ausente','delimitador del conductor','/usr/bin/time ausente','lectura de saldo agregado corregida y probada']})
dump(D/'MANIFIESTO.json',{str(p.relative_to(D)):h(p.read_bytes()) for p in sorted(D.rglob('*')) if p.is_file() and p.name!='MANIFIESTO.json'})
print('Conservación conforme; dos campañas y reproducción desde cero acreditadas.')
