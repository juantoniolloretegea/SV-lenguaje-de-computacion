"""Conductor de ensayo; no realiza semántica SV. Uso: python ejecutar.py RUSTC FUENTES SALIDA_NUEVA"""
import pathlib,subprocess,sys,os,json,hashlib,struct,base64
R=pathlib.Path
rust=str(R(sys.argv[1]).resolve());src=R(sys.argv[2]).resolve();out=R(sys.argv[3]).resolve();out.mkdir(parents=True,exist_ok=False)
sha=lambda b:hashlib.sha256(b).hexdigest()
def save(n,x):
 b=x if isinstance(x,bytes) else (json.dumps(x,ensure_ascii=False,indent=2)+'\n').encode();(out/n).write_bytes(b)
manifest=[{'ruta':str(p.relative_to(src)),'bytes':p.stat().st_size,'sha256':sha(p.read_bytes())} for p in sorted(src.rglob('*')) if p.is_file()];save('FIJACION_PREVIA.json',manifest);fingerprint=sha((out/'FIJACION_PREVIA.json').read_bytes());processes=[];result={'registro':'RETP-2026-149','estado':'EN_CURSO','procesos':processes,'modos':[],'modelo_invocado':False,'reserva_abierta':False}
def run(id,args,expect=0,env=None):
 r=subprocess.run([str(x) for x in args],capture_output=True,timeout=60,env={**os.environ,**(env or {})});save(id+'.stdout',r.stdout);save(id+'.stderr',r.stderr)
 row={'id':id,'argv':[str(x) for x in args],'exit_code':r.returncode,'esperado':expect,'stdout_sha256':sha(r.stdout),'stderr_sha256':sha(r.stderr)};processes.append(row);save('RESULTADO_PARCIAL.json',result)
 assert len(r.stdout)+len(r.stderr)<=32*1024*1024,'salida mayor que cupo observado'
 assert r.returncode==expect,(id,r.returncode,r.stderr.decode(errors='replace')[:2000]);return r
try:
 run('version',[rust,'-Vv']);assert (out/'version.stdout').read_text().startswith('rustc 1.98.0 ')
 fixtures=src/'recibo-g1/candidata/fixtures';frame=(fixtures/'marco-a01.bin').read_bytes();body=frame[16:-32];assert sha(body)==frame[-32:].hex()
 def changed(a,b,rehash=True):
  assert body.count(a)==1;bb=body.replace(a,b);return frame[:8]+struct.pack('<Q',len(bb))+bb+(hashlib.sha256(bb).digest() if rehash else frame[-32:])
 cases=[('original',frame,0,None),('dato-sin-rehash',changed(b'8.40',b'9.40',False),2,'INTEGRIDAD'),('dato-rehash',changed(b'8.40',b'9.40'),2,'CONTENIDO_DISTINTO'),('negacion-rehash',changed(b'no acredita',b'acredita'),2,'CONTENIDO_DISTINTO')]
 for name,b,_,_ in cases:save(name+'.marco',b)
 for mode in ['debug','release']:
  flags=['-C','opt-level='+('0' if mode=='debug' else '3'),'-C','overflow-checks='+('yes' if mode=='debug' else 'no')]
  def build(id,path,extra=()):
   dest=out/(id+'-'+mode+'.bin');run('compilar-'+id+'-'+mode,[rust,'--edition=2021',path,*extra,*flags,'-o',dest]);return dest
  glib=out/('libg1-'+mode+'.rlib');run('biblioteca-g1-'+mode,[rust,'--edition=2021','--crate-type=lib','--crate-name=g1',src/'recibo-g1/candidata/lib.rs',*flags,'-o',glib]);link=['--extern','g1='+str(glib)]
  gt=build('g1-tests',src/'recibo-g1/candidata/lib.rs',['--test']);capture=out/('g1-capturas-'+mode);capture.mkdir();r=run('g1-tests-'+mode,[gt,'--nocapture','--test-threads=1'],env={'G1_FUENTES_SHA256':fingerprint,'G1_BINARIO_SHA256':sha(gt.read_bytes()),'G1_CAPTURAS':str(capture)});assert b'40 passed' in r.stdout
  lt=build('lote-tests',src/'lote-g1/lib.rs',['--test',*link]);capture=out/('lote-capturas-'+mode);capture.mkdir();r=run('lote-tests-'+mode,[lt,'--nocapture','--test-threads=1'],env={'LG1_FUENTES':fingerprint,'LG1_BINARIO':sha(lt.read_bytes()),'LG1_CAPTURAS':str(capture)});assert b'12 passed' in r.stdout
  cli=build('conductor',src/'entrega-literal/main.rs',link)
  for name,b,code,cause in cases:
   r=run('conductor-'+mode+'-'+name,[cli,fixtures/'solicitud-a01.bin',out/(name+'.marco')],code)
   if code==0:assert r.stdout==body and not r.stderr
   else:assert r.stdout==b'' and json.loads(r.stderr)['causa']==cause
  llib=out/('liblote-'+mode+'.rlib');run('biblioteca-lote-'+mode,[rust,'--edition=2021','--crate-type=lib','--crate-name=lote',src/'lote-g1/lib.rs',*link,*flags,'-o',llib]);llink=[*link,'--extern','lote='+str(llib),'-L','dependency='+str(out)]
  lb=build('entregas-lote',src/'entrega-literal/lote.rs',llink);cap=out/('entregas-'+mode);run('entregas-lote-'+mode,[lb,cap]);oracle=json.loads((src/'lote-p3/PROCEDENCIA_Y_ESPERADOS.json').read_text());rows=[]
  for i,row in enumerate(oracle['casos']):
   p=cap/f'P3-{i+1:02}.json';b=p.read_bytes();assert sha(b)==row['cuerpo_esperado_sha256'];assert json.loads(b)==row['cuerpo_esperado'];rows.append({'posicion':i,'sha256':sha(b)})
  result['modos'].append({'modo':mode,'tests_g1_previos':29,'tests_entrega_nuevos':11,'tests_lote_previos':12,'contrastes_cli':4,'entregas_lote':rows})
  # Cada negativo se compila después de un cliente real válido; se exige causa de Rust.
  negatives=[('forja','extern crate g1; fn main(){let _=g1::EntregaLiteral{cuerpo:b"falso",identidad:(1,1)};}',b'E0451'),('vida','extern crate g1; fn main(){let e; { let mut c=g1::Custodio::nuevo(g1::Realizacion{fuentes_sha256:[0;32],binario_sha256:[0;32]}).unwrap();let h=c.abrir(&mut &b"{"[..],true).unwrap();c.ejecutar_a(&h).unwrap();let m=c.recuperar(&h,g1::Papel::Marco).unwrap().to_vec();c.comprobar_entrega(&h,&m).unwrap();e=c.entrega(&h).unwrap(); } println!("{:?}",e.cuerpo());}',b'E0597')]
  for name,text,error in negatives:
   p=out/(name+'.rs');p.write_text(text);r=run(name+'-'+mode,[rust,'--edition=2021',p,*link,'-o',out/(name+'.bin')],1);assert error in r.stderr
 # Los 24 originales y los cuatro resultados del conductor deben ser idénticos entre modos.
 for p in (out/'entregas-debug').iterdir():assert p.read_bytes()==(out/'entregas-release'/p.name).read_bytes()
 for name,*_ in cases:
  for suffix in ['stdout','stderr']:assert (out/f'conductor-debug-{name}.{suffix}').read_bytes()==(out/f'conductor-release-{name}.{suffix}').read_bytes()
 for f in manifest:assert sha((src/f['ruta']).read_bytes())==f['sha256']
 result['paridad_entregas']=True;result['fuentes_sin_cambios_durante_ensayo']=True;result['estado']='CONFORME_EN_FRONTERA_LITERAL_CANDIDATA'
except Exception as e:result['estado']='FALLO';result['error']=str(e)
save('RESULTADO.json',result)
items=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.suffix not in ['.bin','.rlib']:
  b=p.read_bytes();items.append({'ruta':str(p.relative_to(out)),'bytes':len(b),'sha256':sha(b),'base64':base64.b64encode(b).decode()})
save('CAPSULA.json',{'archivos':items})
print(json.dumps({'estado':result['estado'],'procesos':len(processes),'modos':len(result['modos']),'error':result.get('error')},ensure_ascii=False));sys.exit(0 if result['estado'].startswith('CONFORME') else 1)
