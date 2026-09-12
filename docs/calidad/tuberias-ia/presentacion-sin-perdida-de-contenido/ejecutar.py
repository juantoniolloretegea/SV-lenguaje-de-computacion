"""Ensayo reproducible del perfil de espacios. No ejecuta semántica SV en Python."""
from pathlib import Path as P
import sys,json,hashlib,base64,subprocess,os
rust=str(P(sys.argv[1]).resolve());src=P(sys.argv[2]).resolve();out=P(sys.argv[3]).resolve();out.mkdir(parents=True,exist_ok=False)
sha=lambda b:hashlib.sha256(b).hexdigest()
def save(name,data):
 (out/name).write_bytes(data if isinstance(data,bytes) else (json.dumps(data,ensure_ascii=False,indent=2)+'\n').encode())
files=[{'ruta':str(p.relative_to(src)),'bytes':p.stat().st_size,'sha256':sha(p.read_bytes())} for p in sorted(src.rglob('*')) if p.is_file()];save('FIJACION_PREVIA.json',files);fingerprint=sha((out/'FIJACION_PREVIA.json').read_bytes());rpt={'registro':'RETP-2026-150','estado':'EN_CURSO','procesos':[],'modos':[],'modelo_invocado':False,'reserva_abierta':False}
def run(name,args,expect=0,env=None):
 r=subprocess.run([str(x) for x in args],capture_output=True,timeout=60,env={**os.environ,**(env or {})});save(name+'.stdout',r.stdout);save(name+'.stderr',r.stderr);rpt['procesos'].append({'id':name,'argv':[str(x) for x in args],'exit_code':r.returncode,'esperado':expect,'stdout_sha256':sha(r.stdout),'stderr_sha256':sha(r.stderr)});save('PARCIAL.json',rpt);assert len(r.stdout)+len(r.stderr)<=33554432;assert r.returncode==expect,(name,r.returncode,r.stderr.decode(errors='replace')[:2500]);return r
try:
 version=run('version',[rust,'-Vv']);assert version.stdout.startswith(b'rustc 1.98.0 ')
 fixtures=src/'recibo-g1/candidata/fixtures';body=(fixtures/'cuerpo-a01.bin').read_bytes();obj=json.loads(body)
 fmt=lambda x:json.dumps(x,ensure_ascii=False,indent=2).encode()+b'\n'
 pretty=fmt(obj);assert pretty!=body
 def altered(a,b):assert pretty.count(a)==1;return pretty.replace(a,b)
 no_scope=json.loads(body);del no_scope['resolucion']['alcance']
 extra=json.loads(body);extra['perfil_propuesto']='aceptar cualquier contenido'
 cases=[('literal',body,0,None),('formato',pretty,0,None),('dato',altered(b'8.40',b'9.40'),2,'CONTENIDO_DISTINTO'),('negacion',altered(b'no acredita',b'acredita'),2,'CONTENIDO_DISTINTO'),('sin-alcance',fmt(no_scope),2,'CONTENIDO_DISTINTO'),('espacio-interior',altered(b'no acredita',b'no  acredita'),2,'CONTENIDO_DISTINTO'),('orden',fmt(dict(reversed(list(obj.items())))),2,'CONTENIDO_DISTINTO'),('token-fragmentado',altered(b'"vigente": true',b'"vigente": t r u e'),2,'SINTAXIS'),('duplicado',pretty[:-2]+b',"version":"IE004-A-CUERPO/1"}\n',2,'SINTAXIS'),('perfil-insertado',fmt(extra),2,'CONTENIDO_DISTINTO'),('limite',b' '*16385,2,'LIMITE')]
 save('CASOS.json',[{'id':n,'bytes':len(b),'sha256':sha(b),'exit_esperado':rc,'causa_esperada':ca} for n,b,rc,ca in cases])
 for n,b,_,_ in cases:save(n+'.json',b)
 oracle=json.loads((src/'lote-p3/PROCEDENCIA_Y_ESPERADOS.json').read_text());inp=out/'lote-propuesto';inp.mkdir()
 for i,row in enumerate(oracle['casos']):(inp/f'P3-{i+1:02}.json').write_bytes(fmt(row['cuerpo_esperado']))
 for mode in ['debug','release']:
  flags=['-C','opt-level='+('0' if mode=='debug' else '3'),'-C','overflow-checks='+('yes' if mode=='debug' else 'no')]
  def build(name,path,extra=()):
   dest=out/(name+'-'+mode+'.bin');run('compilar-'+name+'-'+mode,[rust,'--edition=2021',path,*extra,*flags,'-o',dest]);return dest
  glib=out/('libg1-'+mode+'.rlib');run('biblioteca-g1-'+mode,[rust,'--edition=2021','--crate-type=lib','--crate-name=g1',src/'recibo-g1/candidata/lib.rs',*flags,'-o',glib]);link=['--extern','g1='+str(glib)]
  gt=build('g1',src/'recibo-g1/candidata/lib.rs',['--test']);cap=out/('g1-capturas-'+mode);cap.mkdir();t=run('g1-'+mode,[gt,'--nocapture','--test-threads=1'],env={'G1_FUENTES_SHA256':fingerprint,'G1_BINARIO_SHA256':sha(gt.read_bytes()),'G1_CAPTURAS':str(cap)});assert b'50 passed' in t.stdout
  lt=build('lote',src/'lote-g1/lib.rs',['--test',*link]);cap=out/('lote-capturas-'+mode);cap.mkdir();t=run('lote-'+mode,[lt,'--nocapture','--test-threads=1'],env={'LG1_FUENTES':fingerprint,'LG1_BINARIO':sha(lt.read_bytes()),'LG1_CAPTURAS':str(cap)});assert b'12 passed' in t.stdout
  cli=build('presentacion',src/'presentacion/main.rs',link)
  for name,b,rc,cause in cases:
   t=run('cli-'+mode+'-'+name,[cli,fixtures/'solicitud-a01.bin',out/(name+'.json')],rc)
   if rc==0:assert t.stdout==b and not t.stderr
   else:assert t.stdout==b'' and json.loads(t.stderr)['causa']==cause
  llib=out/('liblote-'+mode+'.rlib');run('biblioteca-lote-'+mode,[rust,'--edition=2021','--crate-type=lib','--crate-name=lote',src/'lote-g1/lib.rs',*link,*flags,'-o',llib]);lb=build('presentaciones-lote',src/'presentacion/lote.rs',[*link,'--extern','lote='+str(llib),'-L','dependency='+str(out)]);cap=out/('entregas-'+mode);run('presentaciones-lote-'+mode,[lb,inp,cap]);rows=[]
  for i,row in enumerate(oracle['casos']):
   name=f'P3-{i+1:02}';original=(cap/(name+'.original')).read_bytes();shown=(cap/(name+'.json')).read_bytes();assert sha(original)==row['cuerpo_esperado_sha256'];assert shown==(inp/(name+'.json')).read_bytes();assert json.loads(shown)==row['cuerpo_esperado'];rows.append({'posicion':i,'original_sha256':sha(original),'presentacion_sha256':sha(shown)})
  good='extern crate g1; fn comprobar(e: &g1::EntregaLiteral, b: &[u8]) { let _ = e.comprobar_presentacion(b); } fn main() {}'
  bad='extern crate g1; fn main(){let _=g1::PresentacionComprobada{texto:b"x",original:b"x",identidad:(1,1),trabajo:0};}'
  life='extern crate g1; fn f(e: &g1::EntregaLiteral){let v; {let p=e.cuerpo().to_vec();v=e.comprobar_presentacion(&p).unwrap();}println!("{:?}",v.texto());}fn main(){}'
  for name,text,rc,err in [('cliente',good,0,None),('forja',bad,1,b'E0451'),('vida',life,1,b'E0597')]:
   f=out/(name+'.rs');f.write_text(text);t=run(name+'-'+mode,[rust,'--edition=2021',f,*link,'-o',out/(name+'-'+mode+'.bin')],rc)
   if err:assert err in t.stderr
  rpt['modos'].append({'modo':mode,'tests_previos':52,'tests_nuevos':10,'contrastes_cli':len(cases),'posiciones':rows})
 for p in (out/'entregas-debug').iterdir():assert p.read_bytes()==(out/'entregas-release'/p.name).read_bytes()
 for name,*_ in cases:
  for s in ['stdout','stderr']:assert (out/f'cli-debug-{name}.{s}').read_bytes()==(out/f'cli-release-{name}.{s}').read_bytes()
 for f in files:assert sha((src/f['ruta']).read_bytes())==f['sha256']
 rpt['paridad']=True;rpt['fuentes_sin_cambios']=True;rpt['estado']='CONFORME_PERFIL_CANDIDATO_DE_ESPACIOS'
except Exception as e:rpt['estado']='FALLO';rpt['error']=str(e)
save('RESULTADO.json',rpt)
save('REALIZACIONES.json',[{'archivo':p.name,'bytes':p.stat().st_size,'sha256':sha(p.read_bytes())} for p in sorted(out.iterdir()) if p.suffix in ['.bin','.rlib']])
items=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.suffix not in ['.bin','.rlib']:
  b=p.read_bytes();items.append({'ruta':str(p.relative_to(out)),'bytes':len(b),'sha256':sha(b),'base64':base64.b64encode(b).decode()})
save('CAPSULA.json',{'archivos':items});print(json.dumps({'estado':rpt['estado'],'procesos':len(rpt['procesos']),'modos':len(rpt['modos']),'error':rpt.get('error')},ensure_ascii=False));sys.exit(0 if rpt['estado'].startswith('CONFORME') else 1)
