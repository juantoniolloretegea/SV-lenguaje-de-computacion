from pathlib import Path
import subprocess,os,json,hashlib,base64,time,sys
root=Path(__file__).resolve().parent
tmp=Path(sys.argv[2]).resolve();tmp.mkdir(exist_ok=False)
src=tmp/'candidata';out=tmp/'resultados';out.mkdir()
rust=Path(sys.argv[1]).resolve()
for f in json.loads((root/'FUENTES_REPRODUCIBLES.json').read_text())['archivos']:
 data=base64.b64decode(f['base64']);assert hashlib.sha256(data).hexdigest()==f['sha256']
 rel=Path(f['ruta']);assert not rel.is_absolute() and '..' not in rel.parts
 p=src/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(data)
(out/'CONTRATO_Y_PLAN.md').write_bytes((root/'CONTRATO_Y_PLAN.md').read_bytes())
sha=lambda b:hashlib.sha256(b).hexdigest()
files=[{'ruta':str(p.relative_to(src)),'bytes':p.stat().st_size,'sha256':sha(p.read_bytes()),'base64':base64.b64encode(p.read_bytes()).decode()} for p in sorted(src.rglob('*')) if p.is_file()]
capsule={'version':'IE004-LECTURA-VINCULADA-FUENTES/1','archivos':files};(out/'FUENTES_REPRODUCIBLES.json').write_text(json.dumps(capsule,ensure_ascii=False,indent=2)+'\n');fingerprint=sha((out/'FUENTES_REPRODUCIBLES.json').read_bytes())
log=[]
def run(label,args,env=None,expected=0,error=None):
 start=time.time();p=subprocess.run([str(x)for x in args],capture_output=True,env=env,timeout=60)
 r={'paso':label,'argv':[str(x)for x in args],'exit_code':p.returncode,'stdout':p.stdout.decode(errors='replace'),'stderr':p.stderr.decode(errors='replace'),'segundos':round(time.time()-start,3),'esperado':expected};log.append(r);(out/'COMANDOS.json').write_text(json.dumps(log,ensure_ascii=False,indent=2)+'\n');assert p.returncode==expected,r
 if error:assert error in r['stderr'],r
 print(label,'OK',flush=True)
run('compilador',[rust,'-vV'])
for mode,flags in [('debug',['-C','opt-level=0','-C','overflow-checks=yes']),('release',['-C','opt-level=3','-C','overflow-checks=no'])]:
 d=tmp/mode;d.mkdir(exist_ok=False)
 common=['--edition=2021',*flags];g=d/'libg1.rlib';l=d/'liblote.rlib'
 run(mode+' g1',[rust,*common,'--crate-type=lib','--crate-name=g1',src/'recibo-g1/candidata/lib.rs','-o',g])
 run(mode+' lote',[rust,*common,'--crate-type=lib','--crate-name=lote',src/'lote-g1/lib.rs','--extern',f'g1={g}','-o',l])
 deps=['--extern',f'g1={g}','--extern',f'lote={l}','-L',f'dependency={d}']
 for name,path in [('regresion',src/'lote-g1/lib.rs'),('lectura',src/'lectura/tests.rs')]:
  exe=d/name;run(mode+' compilar '+name,[rust,*common,'--test',path,*deps,'-o',exe])
  cap=d/(name+'-capturas');cap.mkdir();env=os.environ|{'LG1_FUENTES':fingerprint,'LG1_BINARIO':sha(exe.read_bytes()),'LG1_CAPTURAS':str(cap),'LV_CAPTURAS':str(cap)}
  run(mode+' ejecutar '+name,[exe,'--test-threads=1','--nocapture'],env)
  expected=json.loads((src/'lote-p3/PROCEDENCIA_Y_ESPERADOS.json').read_text())['casos']
  for c in expected:
   file=cap/(c['id']+('.original' if name=='lectura' else '.cuerpo.json'));assert sha(file.read_bytes())==c['cuerpo_esperado_sha256'],file
  print(mode,name,'24 cuerpos iguales al oráculo anterior',flush=True)
 for name,error in [('forjar','E0451'),('vida','E0515')]:
  run(mode+' impedir '+name,[rust,*common,src/f'lectura/{name}.rs',*deps,'-o',d/name],expected=1,error=error)
results={'version':'IE004-LECTURA-VINCULADA-RESULTADO/1','fuentes_sha256':fingerprint,'plan_sha256':sha((out/'CONTRATO_Y_PLAN.md').read_bytes()),'compilador_sha256':sha(rust.read_bytes()),'modos':['debug','release'],'regresion_por_modo':12,'testigos_runtime_por_modo':9,'clientes_rechazados_por_modo':2,'cuerpos_anteriores_iguales_por_modo':24,'procesos':len(log),'estado':'CONFORME_EN_ALCANCE_DOCUMENTAL'}
(out/'RESULTADO.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n')
