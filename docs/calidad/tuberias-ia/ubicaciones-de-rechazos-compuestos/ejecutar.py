from pathlib import Path
import base64, datetime, hashlib, json, os, selectors, signal, subprocess, sys, time, resource
r=Path(__file__).resolve().parent
rust=Path(sys.argv[1]).resolve()
work=r/'evidencia';work.mkdir(exist_ok=False)
TIMEOUT_S=120
def sha(b):return hashlib.sha256(b).hexdigest()
def save(name,obj):
 p=work/name;t=p.with_suffix(p.suffix+'.tmp');t.write_text(json.dumps(obj,ensure_ascii=False,indent=2)+'\n');t.replace(p)
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
logs=[];events=[]
# Reutiliza el observador publicado; no cambia el juicio del compilador.
observer=(r/'antecedente-158/reproducir.py').read_text()
exec(observer[observer.index('def run('):observer.index("run('rustc'")])
for p,h in json.loads((r/'COMPROMISO_PREVIO.json').read_text())['archivos'].items():assert sha((r/p).read_bytes())==h
save('IDENTIDAD.json',{'utc':utc(),'rustc_sha256':sha(rust.read_bytes()),'instrumento_sha256':sha(Path(__file__).read_bytes()),'compromiso_sha256':sha((r/'COMPROMISO_PREVIO.json').read_bytes()),'observador_sha256':sha((r/'antecedente-158/reproducir.py').read_bytes()),'timeout_s':120,'limite_flujo_bytes':2097152})
run('rustc',[rust,'--version','--verbose'])
cases=json.loads((r/'CASOS_ESPERADOS.json').read_text())['casos']
def lit(x):return json.dumps(x,ensure_ascii=False)
rows=[]
for c in cases:
 span='None' if c['span'] is None else f"Some({c['span'][0]}..{c['span'][1]})"
 rows.append('('+','.join([lit(c['id']),lit(c['source']),'P::'+('En' if c['profile']=='en' else 'Es'),lit(c['cause']),span,lit(sha(c['source'].encode()))])+')')
focal='''use sv_core::{SourceProfile as P,SourceUnit as U};
use sv_core::compiler_diagnostics as d;
use sv_core::audit_diagnostics::Language;
fn check(id:&str, source:&str, p:P, cause:&str, span:Option<std::ops::Range<usize>>, digest:&str) {
 let start=std::time::Instant::now();
 let old=sv_core::compile_svp_profile(source,"igual.svp",p);
 let result=d::compile(source,"igual.svp",p);
 assert_eq!(old.is_ok(),cause=="OK","{id}: aceptación");
 assert_eq!(old,result.clone().map_err(|r|r.into_legacy()),"{id}: API");
 println!("LEGACY\\t{id}\\t{old:?}");
 if let Err(report)=result {
  if cfg!(candidate) {
   assert_eq!(report.cause().id(),cause,"{id}: causa");
   assert_eq!(report.sites().len(),1,"{id}: sitios");
   assert_eq!(report.sites()[0].span,span,"{id}: intervalo");
   assert_eq!(report.sites()[0].unit,0);
   assert_eq!(report.sources()[0].byte_len,source.len());
   assert_eq!(report.sources()[0].sha256,digest,"{id}: huella original");
   assert_eq!(report.sources()[0].profile,p);
   assert_eq!(report.stage(),d::Stage::Frontend);
   if let Some(ref bytes)=span {assert!(source.is_char_boundary(bytes.start) && source.is_char_boundary(bytes.end));}
   assert_eq!(report.messages()[0].0,if p==P::Es {Language::Es}else{Language::En});
   assert!(!format!("{report:?}").contains("__SVP_FOREIGN_SURFACE__"));
   for (_,m) in report.messages(){assert!(!m.contains("__SVP_FOREIGN_SURFACE__"));}
  }
  let healthy=if p==P::Es {U::new("codomain Sano = {X};","igual.svp",P::En)}else{U::new("codominio Sano = {X};","igual.svp",P::Es)};
  for reverse in [false,true] {
   let unit=U::new(source,"igual.svp",p);let mut units=[unit,healthy];if reverse {units.reverse();}
   let a=d::compile_assembly(&units).unwrap_err();
   let old=sv_core::compile_svp_assembly(&units).unwrap_err();assert_eq!(a.legacy(),&old);
   println!("LEGACY_ASSEMBLY\\t{id}\\t{reverse}\\t{old:?}");
   if cfg!(candidate) {
    assert_eq!(a.cause().id(),cause);assert_eq!(a.sites().len(),1);
    assert_eq!(a.sites()[0].unit,if reverse{1}else{0});assert_eq!(a.sites()[0].span,span);
    let index=if reverse{1}else{0};
    assert_eq!(a.sources()[index].sha256,digest);assert_eq!(a.sources()[index].profile,p);
    assert_eq!(a.sources()[index].byte_len,source.len());
    assert_eq!(a.messages().len(),2);assert_ne!(a.sources()[0].sha256,a.sources()[1].sha256);
   }
  }
 }
 println!("CASE\\t{id}\\t{cause}\\t{}",start.elapsed().as_nanos());
}
fn main(){for (id,s,p,cause,span,digest) in [ROWS] {check(id,s,p,cause,span,digest);}}
'''.replace('ROWS',',\n'.join(rows))
(r/'focal.rs').write_text(focal)
old=(r/'antecedente-158/focal.rs').read_text()
old=old.replace('source,p,"CD.UNSUPPORTED",None);assert_eq!(r.sites()[0].span,None);','source,p,"CD.FOREIGN_SURFACE",None);assert_eq!(&source[r.sites()[0].span.clone().unwrap()],if p==P::Es {"codomain"} else {"codominio"});')
(r/'focal_heredado_v2.rs').write_text(old)
for f in json.loads((r/'antecedente-158/CORPUS_FUENTES.json').read_text())['archivos']:
 b=base64.b64decode(f['base64'],validate=True);assert sha(b)==f['sha256'];p=r/'corpus'/f['ruta'];p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b)
items=json.loads((r/'antecedente-158/CORPUS.json').read_text())
(r/'corpus.tsv').write_text(''.join(x['expected']+'\t'+str(r/'corpus'/x['path'])+'\n' for x in items))
observations=[]
for mode,flags in [('debug',[]),('release',['-O'])]:
 snapshots={};corpora={}
 for version in ['base','candidata']:
  lib=r/f'libsv_core_{version}_{mode}.rlib';src=r/version/'rust/sv_core/src/lib.rs'
  run(f'{version}-{mode}-build',[rust,'--edition=2021','--crate-name','sv_core','--crate-type','rlib',*flags,src,'-o',lib])
  for name,source,cfg in [('corpus',r/'antecedente-158/corpus.rs',[]),('focal',r/'focal.rs',['--cfg','candidate'] if version=='candidata' else [])]:
   exe=r/f'{name}-{version}-{mode}';run(f'{version}-{mode}-{name}-build',[rust,'--edition=2021',*flags,*cfg,source,'--extern',f'sv_core={lib}','-o',exe])
   if name=='corpus':corpora[version]=run(f'{version}-{mode}-corpus',[exe,r/'corpus.tsv'])['stdout']
   else:
    for rep in range(1,4) if version=='candidata' else [1]:
     out=run(f'{version}-{mode}-focal-{rep}',[exe])['stdout']
     lines=out.splitlines();snapshots[version]=[l for l in lines if l.startswith('LEGACY')]
     if version=='candidata':
      seen=[]
      for line in lines:
       if line.startswith('CASE\t'):
        _,id,cause,ns=line.split('\t');seen.append(id);observations.append({'modo':mode,'repeticion':rep,'id':id,'causa':cause,'duracion_ns':int(ns)})
      assert seen==[c['id'] for c in cases]
      save('OBSERVACIONES.json',observations)
 assert corpora['base']==corpora['candidata'],'Cambio heredado en corpus'
 assert snapshots['base']==snapshots['candidata'],'Cambio heredado en focales/ensamblaje'
 src=r/'candidata/rust/sv_core/src/lib.rs';lib=r/f'libsv_core_candidata_{mode}.rlib'
 exe=r/f'units-{mode}';run(f'{mode}-units-build',[rust,'--edition=2021','--test',*flags,src,'-o',exe]);run(f'{mode}-units',[exe,'--test-threads=1'],contains='239 passed; 0 failed')
 for kind,source in [('heredado',r/'focal_heredado_v2.rs'),('relacional',r/'antecedente-158/relacional.rs')]:
  exe=r/f'{kind}-{mode}';run(f'{mode}-{kind}-build',[rust,'--edition=2021',*flags,source,'--extern',f'sv_core={lib}','-o',exe]);run(f'{mode}-{kind}',[exe])
lib=r/'libsv_core_candidata_debug.rlib'
for name,codes in [('no_instalar',['E0624','E0624']),('no_premisa',['E0451']),('no_desligar',['E0616']),('no_instalar_diagnostico',['E0624','E0624'])]:
 run(name,[rust,'--edition=2021','--error-format=json',r/'antecedente-158/clientes'/(name+'.rs'),'--extern',f'sv_core={lib}','-o',r/name],1,codes)
save('RESULTADO.json',{'estado':'CONFORME_EN_ALCANCE_NATIVO','fuentes_nuevas':54,'fuentes_heredadas_159':40,'fuentes_focales':len(cases),'validas_focales':sum(c['cause']=='OK' for c in cases),'ensamblajes_focales':2*sum(c['cause']!='OK' for c in cases),'observaciones_monofuente':len(observations),'repeticiones':3,'modos':['debug','release'],'corpus':{'validos':14,'invalidos':106,'resultado_heredado_identico':True},'unitarios_por_modo':239,'focales_heredados':29,'esperados_heredados_modificados_expresamente':2,'clientes_negativos':4,'errores_de_acceso':6,'procesos':len(logs),'wasi':False,'navegador':False,'proveedores_externos':0,'cierre_global':False})
print('CONFORME EN ALCANCE NATIVO')
