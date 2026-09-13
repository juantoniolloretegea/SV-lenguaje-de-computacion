# ES: Materializa estímulos y esperados previos; no ejecuta ni valida SVP.
# EN: Materializes prior stimuli and expectations; does not execute or validate SVP.
from pathlib import Path
import json,hashlib,copy,re,tarfile,io,gzip,shutil
R=Path('manifiesto-sv/checkout');B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';D=B/'bis04-realizacion-i0205-v0_1';I=B/'integracion-c02-c05-v0_1';E=B/'bis04-extension-perfiles-recursos-v0_1';W=Path('bis-extension/montaje');inp=W/'entradas'
E.mkdir(exist_ok=True);inp.mkdir(parents=True,exist_ok=True)
def h(b):return hashlib.sha256(b).hexdigest()
def raw(x):return (json.dumps(x,ensure_ascii=False,indent=2)+'\n').encode()
def dump(p,x):p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(raw(x))
def archive(root,dest,prefix):
 m=io.BytesIO()
 with tarfile.open(fileobj=m,mode='w') as t:
  for p in sorted(root.rglob('*')):
   if p.is_file():
    b=p.read_bytes();info=tarfile.TarInfo(prefix+'/'+str(p.relative_to(root)));info.size=len(b);info.mode=0o644;t.addfile(info,io.BytesIO(b))
 dest.write_bytes(gzip.compress(m.getvalue(),mtime=0))
for folder in (D,I,B/'bis03-sedes-i0205-v0_1'):
 for p,sha in json.loads((folder/'MANIFIESTO.json').read_text()).items():assert h((folder/p).read_bytes())==sha,(folder,p)
q0=json.loads((I/'SOLICITUDES.json').read_text())[0]['solicitud'];c0=json.loads((I/'CONTEXTOS_CONFIABLES.json').read_text())[0];p0=json.loads((I/'PLANES_DE_INYECCION.json').read_text())[0];o0=json.loads((I/'ORACULOS.json').read_text())[0];r0=json.loads((I/'REGISTRO_CONFIABLE.json').read_text())
src0=(I/q0['fuente']['archivo']).read_bytes();st0=(I/q0['estado_matematico']['archivo']).read_bytes();sp0=(I/q0['soporte']['contenido']['archivo']).read_bytes();geo0=(I/q0['geometria']['archivo']).read_bytes()
f=(R/'rust/sv_core/src/frontend.rs').read_text();en=json.loads(re.search(r'const EN_FORMS:.*?= (\[.*?\]);',f).group(1));es=json.loads(re.search(r'const ES_FORMS:.*?= (\[.*?\]);',f).group(1));mapping=dict(zip(en,es));used={}
def translate(m):
 s=m.group()
 if s.startswith('--') or s.startswith('"'):return s
 if s in mapping:used[s]=mapping[s]
 return mapping.get(s,s)
src_es=re.sub(r'--[^\n]*|"(?:\\.|[^"\\])*"|[A-Za-z_][A-Za-z_0-9]*',translate,src0.decode()).encode();dump(E/'MAPEO_EXPLICITO.json',used)
stages='R01 P01 P02 S01 S02 I01 I02 I03 A01 M01 M02 M03 D01 D02 D03 D04 D05 D06'.split();bank=[]
def case(id,description,profile='en',source=None,source_size=None,state_size=None,support_size=None,metadata_size=None,consumer_size=None,guard=None,late=False,edit=None,capture=None):
 q,c,p,o,r=copy.deepcopy((q0,c0,p0,o0,r0));source=src0 if source is None else source
 if source_size:source+=b' '*(source_size-len(source));assert len(source)==source_size
 state=st0+b' '*max(0,(state_size or len(st0))-len(st0));support=sp0+b' '*max(0,(support_size or len(sp0))-len(sp0));geo=geo0
 for a in (c,p,o):a['id']=id
 q['perfil_fuente']=c['perfil_fuente']=profile
 for k,data in [('fuente',source),('estado_matematico',state),('geometria',geo)]:
  ref=copy.deepcopy(q[k]);ref.update(sha256=h(data),bytes=len(data));q[k]=copy.deepcopy(ref);c[k]=copy.deepcopy(ref)
 q['soporte']['referencia']['sha256']=h(support);q['soporte']['contenido'].update(sha256=h(support),bytes=len(support));c['soporte']=copy.deepcopy(q['soporte']['referencia']);r['soportes'][0]['referencia']=copy.deepcopy(c['soporte']);r['soportes'][0]['contenido']=copy.deepcopy(q['soporte']['contenido'])
 q['vinculo']['programa_fuente']['source_sha256']=h(source);c['vinculo']=copy.deepcopy(q['vinculo']);r['vinculos']['A-r1']=copy.deepcopy(q['vinculo']);r['estado_matematico']=copy.deepcopy(q['estado_matematico'])
 q['entrega']['invocacion']=id
 if consumer_size:q['entrega']['consumidor']='c'*consumer_size
 c['entrega']=copy.deepcopy(q['entrega']);p['contexto_captura']=copy.deepcopy(q['entrega']);p['buffer_capturado']=copy.deepcopy(q['geometria'])
 receipt=o['recibo_esperado'];receipt.update(caso=id,perfil_fuente=profile,fuente_sha256=h(source),soporte=copy.deepcopy(c['soporte']),vinculo=copy.deepcopy(q['vinculo']),estado_sha256=h(state),contexto_entrega=copy.deepcopy(q['entrega']))
 if id in ('RC01','RC02'):
  target=2048 if id=='RC01' else 2049
  size=len(json.dumps(receipt,ensure_ascii=False,separators=(',',':')).encode());consumer=q['entrega']['consumidor']+'c'*(target-size)
  for a in (q['entrega'],c['entrega'],p['contexto_captura'],receipt['contexto_entrega']):a['consumidor']=consumer
  assert len(json.dumps(receipt,ensure_ascii=False,separators=(',',':')).encode())==target
 o.update(estado_matematico_esperado=copy.deepcopy(q['estado_matematico']),contenido_entrega_esperado=copy.deepcopy(q['geometria']),contexto_entrega_esperado=copy.deepcopy(c['entrega']),primera_guarda_esperada=guard,recibo_favorable=guard is None,despachos_locales_esperados=1 if guard is None or guard.startswith('D') else 0,guardas_previas=stages[:12] if late else stages[:] if guard is None else stages[:stages.index(guard)])
 if guard:o.update(resultado_esperado='RECHAZADO',recibo_esperado=None,contenido_entrega_esperado=None,contexto_entrega_esperado=None)
 if edit:edit(q,c)
 injection=None
 if capture:
  key,value=capture
  if key=='bytes':injection=value;p['buffer_capturado'].update(sha256=h(value),bytes=len(value))
  else:p['contexto_captura'][key]=value
 q['nota_documental']='ñ' if id in ('RC03','RC04') else ''
 if metadata_size:
  q['nota_documental']+='x'*(metadata_size-len(raw(q)));assert len(raw(q))==metadata_size
 dest=inp/id;dest.mkdir(exist_ok=True)
 for name,x in [('request.json',q),('context.json',c),('plan.json',p),('oracle.json',o),('registry.json',r)]:dump(dest/name,x)
 for name,data in [('source.bin',source),('state.bin',state),('support.bin',support),('geometry.bin',geo),('canonical.bin',state)]: (dest/name).write_bytes(data)
 for k,name in [('constitucion','constitution.bin'),('convenio','convention.bin'),('transformaciones','transforms.bin')]:shutil.copyfile(I/r[k]['archivo'],dest/name)
 if injection is not None:(dest/'injection.bin').write_bytes(injection)
 if guard is None:(dest/'expected.bin').write_bytes(geo)
 lengths=[len(raw(q)),len(source),len(state),len(support),len(geo)];read=[];total=0;stopped=False
 for actual,cap in zip(lengths,[4096,4096,1024,1024,8192]):
  limit=min(cap,16384-total);got=0 if stopped else min(actual,limit+1);read.append(got)
  if not stopped:
   if actual>limit:stopped=True
   else:total+=actual
 o['bytes_leidos_esperados']=read;dump(dest/'oracle.json',o)
 bank.append({'id':id,'objeto':description,'resultado_esperado':o['resultado_esperado'],'primera_guarda_esperada':guard,'guardas_previas':o['guardas_previas'],'bytes_canales':lengths,'bytes_totales':sum(lengths),'bytes_leidos_esperados':read,'ejecucion':'PENDIENTE','observado':None})
case('ES01','Fuente EN y perfil EN')
case('ES02','Fuente ES fiel; mismos identificadores y cadenas',profile='es',source=src_es)
case('ES03','Fuente ES presentada como EN; sin fallback',source=src_es,guard='P02')
case('ES04','Fuente EN presentada como ES; sin fallback',profile='es',guard='P02')
case('ES05','Perfil ausente',guard='P01',edit=lambda q,c:q.pop('perfil_fuente'))
case('ES06','Etiqueta ES no canónica',profile='ES',guard='P01')
case('ES07','Perfil solicitud EN contradice custodia ES',guard='P01',edit=lambda q,c:c.update(perfil_fuente='es'))
case('ES08','Comentario ES distinto; misma IR canónica',profile='es',source=src_es+b'\n-- ES: comentario adicional. EN: additional comment.\n')
case('RC01','Recibo compacto de 2048 bytes')
case('RC02','Recibo compacto de 2049 bytes; rechazo antes de entregar',guard='R01',late=True)
case('RC03','Metadatos UTF-8 de 4096 bytes',metadata_size=4096)
case('RC04','Metadatos UTF-8 de 4098 bytes',metadata_size=4098,guard='R01')
case('RC05','Fuente de 4096 bytes',source_size=4096)
case('RC06','Fuente de 4097 bytes',source_size=4097,guard='R01')
case('RC07','Estado de 1024 bytes',state_size=1024)
case('RC08','Estado de 1025 bytes',state_size=1025,guard='R01')
case('RC09','Soporte de 1024 bytes',support_size=1024)
case('RC10','Soporte de 1025 bytes',support_size=1025,guard='R01')
base=16384-4096-1024-1024-len(geo0)
case('RC11','Suma canales 16384; todos dentro de cuota individual',source_size=4096,state_size=1024,support_size=1024,metadata_size=base)
case('RC12','Suma canales 16385; todos dentro de cuota individual',source_size=4096,state_size=1024,support_size=1024,metadata_size=base+1,guard='R01')
for idx,key,value,g in [(1,'consumidor','otro','D03'),(2,'canal','otro','D03'),(3,'transformacion','superior','D05'),(4,'bytes',geo0[:-1],'D06'),(5,'bytes',geo0+b' ','D06'),(6,'operacion','ejecutar_script','D03')]:case(f'CAP{idx:02}',f'Captura alterada: {key}; variante {idx}',guard=g,capture=(key,value))
dump(E/'BANCO_PREVIO.json',{'estatuto':'Esperados fijados antes de ejecutar; ejemplos sintéticos, sin garantía general','casos':bank})
par=inp/'paridad';par.mkdir(exist_ok=True)
for name in ('base.en.svp','base.es.svp','comentario.es.svp','dato_distinto.es.svp'):shutil.copyfile(B/'bis-c12/entradas'/name,par/name)
pairs=[{'id':'PAR01','a':'ES01/source.bin','b':'ES02/source.bin','perfil_a':'en','perfil_b':'es','equivalentes':True,'operaciones_minimas':0},{'id':'PAR02','a':'ES02/source.bin','b':'ES08/source.bin','perfil_a':'es','perfil_b':'es','equivalentes':True,'operaciones_minimas':0}]
for idx,name,eq in [(3,'base.es.svp',True),(4,'comentario.es.svp',True),(5,'dato_distinto.es.svp',False)]:pairs.append({'id':f'PAR{idx:02}','a':'paridad/base.en.svp','b':'paridad/'+name,'perfil_a':'en','perfil_b':'es','equivalentes':eq,'operaciones_minimas':1})
dump(inp/'PARIDAD_PREVIA.json',pairs);dump(E/'PARIDAD_PREVIA.json',pairs);dump(inp/'CASOS.json',[x['id'] for x in bank]);dump(E/'ENTRADAS_MATERIALIZADAS.json',{str(p.relative_to(inp)):h(p.read_bytes()) for p in sorted(inp.rglob('*')) if p.is_file()});archive(inp,E/'ENTRADAS.tar.gz','entradas')
shutil.copyfile(D/'sv_core-verificado.tar.gz',E/'sv_core-verificado.tar.gz');shutil.copyfile(D/'NUCLEO_FUENTES.json',E/'NUCLEO_FUENTES.json');archive(D/'proyecto',E/'ADMISION_VERIFICADA.tar.gz','sv_bis_i0205');dump(E/'ADMISION_FUENTES.json',{str(p.relative_to(D/'proyecto')):h(p.read_bytes()) for p in sorted((D/'proyecto').rglob('*')) if p.is_file()})
print('Banco previo:',len(bank),'casos y',len(pairs),'comparaciones; sin ejecución Rust.')
