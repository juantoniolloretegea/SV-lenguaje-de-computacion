# ES: Conserva evidencias, incidencias y límites; no transforma fallos en éxitos.
# EN: Preserves evidence, incidents and limits; never relabels failures as success.
from pathlib import Path
import json,hashlib,tarfile,io,gzip,subprocess,csv,shutil
R=Path('manifiesto-sv/checkout');B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';E=B/'bis04-extension-perfiles-recursos-v0_1';base='aa1229788f0a6625be43c74e29c38023b8193a4c'
def h(b):return hashlib.sha256(b).hexdigest()
def dump(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
def archive(root,dest,prefix):
 m=io.BytesIO()
 with tarfile.open(fileobj=m,mode='w') as t:
  for p in sorted(root.rglob('*')):
   if p.is_file():
    b=p.read_bytes();info=tarfile.TarInfo(prefix+'/'+str(p.relative_to(root)));info.size=len(b);info.mode=0o644;t.addfile(info,io.BytesIO(b))
 dest.write_bytes(gzip.compress(m.getvalue(),mtime=0))
results={}
for campaign,folder,lab,pub in [('01','reproduccion-01','PREVIO_LAB','PREVIO_PUBLICO'),('R01','reproduccion-r01','R01_LAB','R01_PUBLICO')]:
 C=Path('bis-extension')/folder/'ejecucion';l=json.loads(Path('bis-extension/'+lab+'.json').read_text());p=json.loads(Path('bis-extension/'+pub+'.json').read_text());assert l['verified'] and p['verified']
 orders=json.loads((C/'ORDENES.json').read_text());assert all(x['exit_code']==0 for x in orders[:-1]);assert orders[-1]['exit_code']==(1 if campaign=='01' else 0)
 report={'precompromiso_laboratorio':l['commit'],'precompromiso_publico':p['commit'],'resumen':json.loads((C/'resultados/RESUMEN.json').read_text()),'recursos':json.loads((C/'RECURSOS.json').read_text()),'versiones':{tool:(C/(tool+'.stdout')).read_text().strip() for tool in ('rustc','cargo')},'casos':[],'paridad':[]}
 for f in sorted((C/'resultados').glob('*.json')):
  if f.name=='RESUMEN.json':continue
  x=json.loads(f.read_text())
  if f.name.startswith('PAR'):report['paridad'].append(x)
  else:
   assert x['discrepancias']==[];o=x['observado'];report['casos'].append({'id':x['id'],'resultado':o['resultado'],'primera_guarda':o['primera_guarda'],'guardas_superadas':o['guardas_superadas'],'bytes_leidos':x['bytes_leidos'],'despachos':o['despachos_observados'],'recibo_sha256':h(json.dumps(o['recibo'],ensure_ascii=False,separators=(',',':')).encode()) if o['recibo'] else None,'captura_sha256':h(bytes(o['captura']['bytes'])) if o['captura'] else None,'discrepancias':[]})
 assert len(report['casos'])==26;assert sum(x['resultado']=='ENTREGA_DOCUMENTAL_CONCORDANTE' for x in report['casos'])==9
 assert sum(not x['discrepancias'] for x in report['paridad'])==(2 if campaign=='01' else 5)
 if campaign=='R01':dump(C/'CORTES.json',{'laboratorio':l['commit'],'publico':p['commit'],'estado':'Ambos precompromisos verificados antes de ejecutar esta campaña.'})
 if campaign=='R01':archive(C,E/f'EVIDENCIA_CAMPANA_{campaign}.tar.gz','campana-'+campaign);dump(E/f'HUELLAS_CAMPANA_{campaign}.json',{str(f.relative_to(C)):h(f.read_bytes()) for f in sorted(C.rglob('*')) if f.is_file()})
 results[campaign]=report
# ES: El archivo inicial había sido publicado con la evidencia antes de la adenda.
# EN: The initial archive had already been published with evidence before the addendum.
# ES: CORTES añade sólo procedencia; resultados originales permanecen idénticos.
# EN: CORTES only adds provenance; original outcomes remain identical.
dump(E/'RESULTADOS.json',results)
# ES: Verifica originales contra el corte de entrada, sin modificar los expedientes anteriores.
# EN: Verifies originals against the entry commit, without modifying earlier dossiers.
def old(p):return subprocess.check_output(['git','show',base+':'+p],cwd=R)
for folder in ('integracion-c02-c05-v0_1','bis03-sedes-i0205-v0_1','bis04-realizacion-i0205-v0_1'):
 for p,sha in json.loads((B/folder/'MANIFIESTO.json').read_text()).items():assert h((B/folder/p).read_bytes())==sha
for p,v in json.loads((E/'FUENTES.json').read_text())['archivos'].items():assert h((R/p).read_bytes())==v['sha256'] and (R/p).read_bytes()==old(p)
for p,sha in json.loads((E/'NUCLEO_FUENTES.json').read_text())['archivos'].items():assert h((R/'rust/sv_core'/p).read_bytes())==sha
for folder in ('bis-c12',):
 for p in (B/folder).rglob('*'):
  if p.is_file():assert p.read_bytes()==old(str(p.relative_to(R)))
# ES: Los 26 casos son idénticos en ambas campañas; sólo cambian tres pares de paridad.
# EN: All 26 cases are identical in both campaigns; only three parity pairs change.
a=json.loads((E/'ENTRADAS_MATERIALIZADAS.json').read_text());b=json.loads((E/'ENTRADAS_R01_MATERIALIZADAS.json').read_text())
assert [p for p,v in a.items() if b.get(p)!=v]==['PARIDAD_PREVIA.json']
for p,v in json.loads((E/'PRECOMPROMISO.json').read_text()).items():
 if p not in ('README.md','soporte/reproducir.py'):assert h((E/p).read_bytes())==v,p
for p,v in json.loads((E/'PRECOMPROMISO_R01.json').read_text()).items():assert h((E/p).read_bytes())==v,p
append=['docs/calidad/Inventario-sv/sucesos/HISTORIAL_SUCESOS_SV.csv','docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv','docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md']
for p in append:assert (R/p).read_bytes().startswith(old(p))
p='docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv';a=list(csv.DictReader(io.StringIO(old(p).decode())));b=list(csv.DictReader((R/p).open()));assert len(a)==len(b);assert [x['id'] for x,y in zip(a,b) if x!=y]==['S22']
expected=set(append+['docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv','docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md',str((B/'ESTADO_WORKFLOW.json').relative_to(R)),'docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md'])
assert set(subprocess.check_output(['git','diff','--name-only'],cwd=R,text=True).splitlines())==expected
s=json.loads((B/'ESTADO_WORKFLOW.json').read_text());assert s['escenarios_ejecutados']==2 and s['integracion_c02_c05']['casos_ejecutados']==26
assert all(e['estado']=='en ejecución' for e in s['etapas'] if e['id'] in ('BIS-02','BIS-03','BIS-04'))
dump(E/'CONSERVACION.json',{'resultado':'CONFORME','corte':base,'suceso_actualizado':'S22','otros_sucesos':'intactos, incluidos S24 y S25','append_only':append,'originales_c12':'intactos, incluido el identificador fallido','nuestro_banco_previo':'inmutable; tres pares nuevos identificados -R01','admision_y_nucleo':'idénticos por hash','campanas':2,'integrados_unicos':26,'invocaciones_admision':52,'estado_workflow':'BIS-02/03/04 abiertos; GUI diferida'})
p=E/'README.md';text=p.read_text().replace('Estado inicial: banco previo, pendiente de ejecución nativa. Continuación de RETP-220; no cierre global del Bis.','Estado: **extensión ensayada en Rust/Cargo 1.98.0; RETP-2026-221**. Continuación de RETP-220; no cierre global del Bis.')
insert='''## Resultado y hallazgo

| Comprobación | Campaña inicial | Campaña R01 |
|---|---|---|
| 26 casos integrados | 26 conformes: 9 entregas y 17 rechazos | 26 conformes: 9 entregas y 17 rechazos |
| Paridad de fuentes integradas | 2 comparaciones conformes | 2 comparaciones conformes |
| Pares con operación de evaluación | 3 impedidos por defecto del fixture | 3 variantes nuevas conformes: 2 igualdades y 1 desigualdad |
| Salida del binario | 1; campaña no conforme | 0; campaña conforme |

El compilador rechaza el nombre `Celda` de los fixtures C12 porque pertenece al vocabulario protegido. Ese resultado no indica desigualdad de dos IR: ninguna llegó a producirse. La [adenda R01](ADENDA_R01_PREVIA.json) fija tres pares nuevos con `CeldaPrueba`, manteniendo datos, dimensiones, orden, operación y expectativas. Los archivos originales y el fracaso observado permanecen conservados. El hallazgo se traslada al retorno de documentación/errores. El núcleo y el código de admisión no se modificaron.

Se ejecutaron 52 invocaciones integradas en dos campañas, correspondientes a 26 variantes únicas. No se suman a las 202 filas originales como si fueran su ejecución. Cada campaña incluye además cinco pares de compilaciones; la primera tiene tres comparaciones impedidas, la segunda contiene sus tres variantes corregidas. Los dos pares integrados conservan su identidad.

[RESULTADOS.json](RESULTADOS.json) resume resultados por caso; [EVIDENCIA_CAMPANA_01.tar.gz](EVIDENCIA_CAMPANA_01.tar.gz) y [EVIDENCIA_CAMPANA_R01.tar.gz](EVIDENCIA_CAMPANA_R01.tar.gz) conservan órdenes, diagnósticos, capturas literales, estados, trazas y mediciones. Sus archivos de huellas permiten comprobar los contenidos. La reproducción indicada ejecuta R01; `soporte/reproducir-campana-01.py` conserva la reproducción inicial fallida. Precompromisos históricos no son manifiestos del expediente final: README y reproductor se amplían; los bancos originales permanecen intactos. `MANIFIESTO.json` identifica el expediente final.

'''
text=text.replace('## Alcance fijado antes de ejecutar',insert+'## Alcance fijado antes de ejecutar')
text=text.replace('la evidencia irá en archivos separados.','la evidencia se conserva en archivos separados.')
c1=results['01']['recursos'];c2=results['R01']['recursos'];text+='\n## Costes observados y próximo paso\n\nCampaña inicial: '+str(c1['max_rss_kib_linux'])+' KiB de RSS máximo y '+str(c1['elapsed_ns'])+' ns externos. Campaña R01: '+str(c2['max_rss_kib_linux'])+' KiB y '+str(c2['elapsed_ns'])+' ns externos. Son mediciones del binario completo con conductor, receptor y observador; no cotas del núcleo ni garantías temporales.\n\nSiguiente paso: completar fronteras del descriptor y de salida agregada; preparar después el relevo hacia representación/consumo dentro del workflow. La GUI continúa diferida.\n'
text+='\nPrecompromiso inicial: laboratorio `'+results['01']['precompromiso_laboratorio']+'`, público `'+results['01']['precompromiso_publico']+'`. Adenda R01 previa a repetición: laboratorio `'+results['R01']['precompromiso_laboratorio']+'`, público `'+results['R01']['precompromiso_publico']+'`.\n';p.write_text(text)
for name in ('registrar.py','cerrar.py','publicar_previo.py','publicar_r01.py','publicar_final.py','verificar_publicacion.py'):
 f=Path('bis-extension')/name
 if f.exists():shutil.copyfile(f,E/'soporte'/name)
dump(E/'OPERACIONES_ADMINISTRATIVAS.json',{'servicio':'GitHub API mediante puente github_io existente','repositorios':['juantoniolloretegea/SV-matematica-semantica-cuaternaria:lab/playground-sv-permanente','juantoniolloretegea/SV-lenguaje-de-computacion:main'],'orden':'laboratorio primero, espejo público después; actualización de ref sin force y verificación de árbol','scripts':'Contenido completo de los scripts de este incremento en soporte/; publicar_archivos y github_io son utilidades preexistentes del laboratorio.','herramientas_locales':['Python: archivos, archivos tar, SHA, procesos, medición wait4','Cargo/rustc: compilación nativa','sv_bis_extension: admisión y comparación nativa','git: conservación y sincronización'],'nucleo':'Sin red durante compilación/ejecución offline; todos los datos y planes son fixtures sintéticos','resultado_persistencia':'Se verificará por hashes y commits al concluir la publicación.'})
dump(E/'MANIFIESTO.json',{str(p.relative_to(E)):h(p.read_bytes()) for p in sorted(E.rglob('*')) if p.is_file() and p.name!='MANIFIESTO.json'})
print('Evidencia consolidada; conservación conforme.')
