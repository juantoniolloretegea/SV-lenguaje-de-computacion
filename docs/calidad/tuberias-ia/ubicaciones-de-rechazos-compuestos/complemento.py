from pathlib import Path
import base64, datetime, hashlib, json, os, selectors, signal, subprocess, sys, time, resource, shutil, difflib
r=Path(__file__).resolve().parent;rust=Path(sys.argv[1]).resolve()
work=r/'complemento';work.mkdir(exist_ok=False);TIMEOUT_S=120
def sha(b):return hashlib.sha256(b).hexdigest()
def save(name,obj):
 p=work/name;t=p.with_suffix(p.suffix+'.tmp');t.write_text(json.dumps(obj,ensure_ascii=False,indent=2)+'\n');t.replace(p)
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
logs=[];events=[]
observer=(r/'antecedente-158/reproducir.py').read_text();exec(observer[observer.index('def run('):observer.index("run('rustc'")])
cat=json.loads((r/'antecedente-158/CATALOGO.json').read_text())
row={'id':'CD.FOREIGN_SURFACE','code':None,'es':'La grafía no está admitida en esta posición bajo el perfil fuente seleccionado.','en':'The spelling is not allowed at this position under the selected source profile.'}
cat['causas'].insert(4,row);cat['version']='COMPILER-DIAGNOSTICS/2'
(r/'CATALOGO.json').write_text(json.dumps(cat,ensure_ascii=False,indent=2)+'\n')
for lang in ['es','en']:
 p=r/'idiomas'/lang/'diagnosticos.json';p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps({'version':'COMPILER-DIAGNOSTICS/2','mensajes':{x['id']:x[lang] for x in cat['causas']}},ensure_ascii=False,indent=2)+'\n')
source=(r/'antecedente-158/catalogo.rs').read_text().replace('C::Frontend(F::UnexpectedEnd)', 'C::Frontend(F::ForeignSurface),C::Frontend(F::UnexpectedEnd)')
(r/'catalogo.rs').write_text(source)
save('ESPERADOS_PREVIOS.json',{'utc':utc(),'catalogo_sha256':sha((r/'CATALOGO.json').read_bytes()),'catalogo_rs_sha256':sha((r/'catalogo.rs').read_bytes()),'control_base':{'codigo':101,'contiene':'rel_REPEAT_en: intervalo'},'mutacion_intervalo':{'codigo':101,'contiene':'rel_REPEAT_en: intervalo'},'cardinalidad_no_migrada':{'casos':2,'causa':'CD.INVALID_ADMISSIBILITY_STATE','intervalo':None,'ubicacion_comprobacion':'batería focal principal'},'fuentes_candidatas':{str(p.relative_to(r/'candidata')):sha(p.read_bytes()) for p in sorted((r/'candidata').rglob('*')) if p.is_file()}})
lib=r/'libsv_core_candidata_debug.rlib';exe=work/'catalogo'
run('catalogo-build',[rust,'--edition=2021',r/'catalogo.rs','--extern',f'sv_core={lib}','-o',exe])
lines=run('catalogo',[exe])['stdout'].splitlines();actual=[]
for line in lines:
 _,id,code,es,en=line.split('\t');actual.append({'id':id,'code':code or None,'es':es,'en':en})
assert actual==cat['causas']
for lang in ['es','en']:assert json.loads((r/'idiomas'/lang/'diagnosticos.json').read_text())['mensajes']=={x['id']:x[lang] for x in actual}
exe=work/'control-base'
run('control-base-build',[rust,'--edition=2021','--cfg','candidate',r/'focal.rs','--extern',f'sv_core={r}/libsv_core_base_debug.rlib','-o',exe])
row=run('control-base',[exe],101);assert 'rel_REPEAT_en: intervalo' in row['stderr']
shutil.copytree(r/'candidata',work/'mutante')
p=work/'mutante/rust/sv_core/src/frontend.rs';old=p.read_text();needle='            let field_position = self.pos;\n            let field = self.take_raw_word()?;';assert old.count(needle)==2
new=old.replace(needle,'            let field = self.take_raw_word()?;\n            let field_position = self.pos;');p.write_text(new)
(work/'MUTACION.patch').write_text(''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/rust/sv_core/src/frontend.rs',tofile='b/rust/sv_core/src/frontend.rs')))
lib=work/'libsv_core_mutante.rlib';exe=work/'control-intervalo'
run('mutante-build',[rust,'--edition=2021','--crate-name','sv_core','--crate-type','rlib',work/'mutante/rust/sv_core/src/lib.rs','-o',lib])
run('control-intervalo-build',[rust,'--edition=2021','--cfg','candidate',r/'focal.rs','--extern',f'sv_core={lib}','-o',exe])
row=run('control-intervalo',[exe],101);assert 'rel_REPEAT_en: intervalo' in row['stderr']
save('RESULTADO.json',{'estado':'CONFORME','causas_locales':len(actual),'plantillas_es_en':len(actual)*2,'controles_no_migrados':2,'controles_sensibilidad':2,'base_sin_intervalo_detectada':True,'intervalo_desplazado_detectado':True,'procesos':len(logs),'candidata_modificada':False})
print('COMPLEMENTO CONFORME')
