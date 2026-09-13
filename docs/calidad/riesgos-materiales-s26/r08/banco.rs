mod auxiliar;
use auxiliar::{R,Request,Injection,FILES};
use std::{env,fs,path::{Path,PathBuf}};
const BEFORE:[&[u8];5]=[
 include_bytes!("fixtures/antes/SUCESOS_SV.csv"),include_bytes!("fixtures/antes/HISTORIAL_SUCESOS_SV.csv"),include_bytes!("fixtures/antes/SUCESOS_SV.md"),include_bytes!("fixtures/antes/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv"),include_bytes!("fixtures/antes/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md")];
const AFTER:[&[u8];5]=[
 include_bytes!("fixtures/despues/SUCESOS_SV.csv"),include_bytes!("fixtures/despues/HISTORIAL_SUCESOS_SV.csv"),include_bytes!("fixtures/despues/SUCESOS_SV.md"),include_bytes!("fixtures/despues/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv"),include_bytes!("fixtures/despues/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md")];
fn io<T>(x:std::io::Result<T>)->R<T>{x.map_err(|e|e.to_string())}
fn require(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn reject(r:R<()>,prefix:&str)->R<()>{match r{Err(e)if e.starts_with(prefix)=>Ok(()),x=>Err(format!("Esperado {prefix}; observado {x:?}"))}}
fn fixture(d:&Path)->R<(PathBuf,PathBuf,PathBuf)>{let a=d.join("antes");let b=d.join("despues");let o=d.join("salida");io(fs::create_dir(&a))?;io(fs::create_dir(&b))?;
 for(i,n)in FILES.iter().enumerate(){io(fs::write(a.join(n),BEFORE[i]))?;io(fs::write(b.join(n),AFTER[i]))?;}Ok((a,b,o))}
fn req()->Request{Request{id:"S1".into(),revision:1,retp:2}}
fn original(a:&Path)->R<()>{for(i,n)in FILES.iter().enumerate(){require(io(fs::read(a.join(n)))?==BEFORE[i],"Origen alterado")?;}Ok(())}
fn pubtext()->String{format!("SV-AUX-PUB/1\nrepo\tfixture/repo\nbranch\tmain\nbase\t{}\ncommit\t{}\ncomplete\ttrue\nentry\tdoc.txt\t100644\tblob\t{}\n","a".repeat(40),"b".repeat(40),"c".repeat(40))}
fn test(id:&str,d:&Path)->R<()>{
 if id=="P15"{
  let r=auxiliar::request(b"SV-AUX-REG/1\nid\tS1\nrevision\t1\nretp\t2\n")?;
  require(r.id=="S1"&&r.revision==1&&r.retp==2,"Solicitud positiva")?;
  require(auxiliar::request(b"SV-AUX-REG/1\nid\tS1\nrevision\t1\nrevision\t99\nretp\t2\n").is_err(),"Solicitud duplicada aceptada")?;
  reject(auxiliar::cli(&["auxiliar".into(),"verified".into()]),"USO")?;return Ok(());
 }
 if matches!(id,"P01"|"P02"|"P03"|"P04"|"P13"){
  let e=d.join("esperado.tsv");let o=d.join("observado.tsv");let good=pubtext();io(fs::write(&e,&good))?;io(fs::write(&o,&good))?;
  auxiliar::check_publication(&e,&o)?;
  return match id{
   "P01"=>Ok(()),
   "P02"=>{io(fs::write(&o,good.replace(&"a".repeat(40),&"d".repeat(40))))?;reject(auxiliar::check_publication(&e,&o),"PUB_BASE")},
   "P03"=>{io(fs::write(&e,good.replace(&"b".repeat(40),&"d".repeat(40))))?;reject(auxiliar::check_publication(&e,&o),"PUB_COMMIT")?;io(fs::write(&o,"{\"verified\":true}"))?;reject(auxiliar::check_publication(&e,&o),"PUB_FORMATO")},
   "P04"=>{io(fs::write(&o,format!("{good}{}\n",good.lines().last().ok_or("fixture")?)))?;reject(auxiliar::check_publication(&e,&o),"PUB_RUTA_DUPLICADA")},
   "P13"=>{io(fs::write(&o,good.replace(&"c".repeat(40),&"d".repeat(40))))?;reject(auxiliar::check_publication(&e,&o),"PUB_ARBOL")?;io(fs::write(&o,good.replace("complete\ttrue","complete\tfalse")))?;reject(auxiliar::check_publication(&e,&o),"PUB_TRUNCADO")},
   _=>Err("Caso desconocido".into())
  };
 }
 if id=="P14"{
  let c=auxiliar::csv(b"a,b\r\n\"x,y\",\"z\"\"w\"\r\n")?;require(c==vec![vec!["a","b"],vec!["x,y","z\"w"]],"CSV positivo")?;
  for bad in [&b"a,a\nx,y\n"[..],&b"a,b\n\"x"[..],&b"a,b\nx\n"[..],&b"a,b\n\"x\"z,y\n"[..]]{require(auxiliar::csv(bad).is_err(),"CSV malformado aceptado")?;}return Ok(());
 }
 let(a,b,o)=fixture(d)?;let r=req();
 match id{
  "P05"=>{auxiliar::stage(&a,&b,&o,&r,Injection::None)?;auxiliar::verify_stage(&a,&b,&o,&r)?;require(auxiliar::bundle(&o)?.bytes==AFTER.map(Vec::from),"Salida positiva")?;},
  "P06"=>{let mut bad=r.clone();bad.revision=99;reject(auxiliar::stage(&a,&b,&o,&bad,Injection::None),"REG_REVISION")?;require(!o.exists(),"Salida creada tras rechazo")?;},
  "P07"=>{io(fs::remove_file(a.join(FILES[2])))?;reject(auxiliar::stage(&a,&b,&o,&r,Injection::None),"IO")?;require(!o.exists(),"Salida ante entrada ausente")?;
   for i in [0,1,3,4]{require(io(fs::read(a.join(FILES[i])))?==BEFORE[i],"Origen alterado")?;}return Ok(());},
  "P08"=>{for n in 1..=5{let out=d.join(format!("parcial-{n}"));reject(auxiliar::stage(&a,&b,&out,&r,Injection::AfterFile(n)),"FALLO_INYECTADO")?;
    require(!out.join("PREPARADO").exists(),"Marca prematura")?;require(io(fs::read_dir(&out))?.count()==n,"Número de escrituras")?;
    require(auxiliar::verify_stage(&a,&b,&out,&r).is_err(),"Parcial aceptado")?;original(&a)?;}},
  "P09"=>{auxiliar::stage(&a,&b,&o,&r,Injection::None)?;io(fs::write(o.join(FILES[0]),b"alterado"))?;reject(auxiliar::verify_stage(&a,&b,&o,&r),"SALIDA_DIFIERE")?;},
  "P10"=>{auxiliar::stage(&a,&b,&o,&r,Injection::None)?;reject(auxiliar::stage(&a,&b,&o,&r,Injection::None),"IO")?;auxiliar::verify_stage(&a,&b,&o,&r)?;},
  "P11"=>{let s=io(fs::read_to_string(b.join(FILES[0])))?;io(fs::write(b.join(FILES[0]),s.replace("en ejecución","inventado")))?;reject(auxiliar::stage(&a,&b,&o,&r,Injection::None),"REG_ESTADO")?;require(!o.exists(),"Salida ante estado inválido")?;},
  "P12"=>{let s=io(fs::read_to_string(b.join(FILES[1])))?;io(fs::write(b.join(FILES[1]),s.replace("\n1,S1,","\n99,S1,")))?;reject(auxiliar::stage(&a,&b,&o,&r,Injection::None),"REG_HIST_NUEVO")?;require(!o.exists(),"Salida ante historial inválido")?;},
  _=>return Err("Caso desconocido".into())
 }
 original(&a)
}
fn run()->R<()>{let a:Vec<_>=env::args().collect();require(a.len()==2,"Uso: banco DIRECTORIO_NUEVO")?;let root=Path::new(&a[1]);io(fs::create_dir(root))?;
 let mut log=String::from("caso\toraculo_cumplido\tdiagnostico\n");let mut passed=0;
 for n in 1..=15{let id=format!("P{n:02}");let d=root.join(&id);io(fs::create_dir(&d))?;let result=test(&id,&d);
  let ok=result.is_ok();if ok{passed+=1;}let msg=result.err().unwrap_or_else(||"esperado".into()).replace(['\t','\n','\r']," ");
  log.push_str(&format!("{id}\t{ok}\t{msg}\n"));io(fs::write(root.join("RESULTADOS.tsv"),&log))?;
 }
 io(fs::write(root.join("PERFIL.txt"),format!("debug_assertions={}\ncasos=15\nconformes={passed}\n",cfg!(debug_assertions))))?;
 print!("{log}");require(passed==15,"Discrepancia con banco previo; preservar resultado")
}
fn main(){if let Err(e)=run(){eprintln!("FALLO_BANCO: {e}");std::process::exit(1);}}
