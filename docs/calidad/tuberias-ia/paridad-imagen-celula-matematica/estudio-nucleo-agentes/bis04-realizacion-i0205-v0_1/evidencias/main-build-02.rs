//! ES: Conductor local, receptor y observador experimental, sin servicios externos.
//! EN: Local harness, receiver and experimental observer, without external services.
mod observer;
use observer::{Evidence,inspect};
use sv_bis_i0205::{*,json::{J,s,n,obj}};
use std::{path::{Path,PathBuf},fs::{self,File},io::Cursor};
fn bytes(p:&Path,max:usize)->Vec<u8>{json::receive(&mut File::open(p).unwrap_or_else(|e|panic!("loader {}: {e}",p.display())),max).unwrap_or_else(|e|panic!("loader {}: {e}",p.display()))}
fn document(p:&Path)->J{json::decode(&bytes(p,8192)).unwrap_or_else(|e|panic!("custody document {}: {e}",p.display()))}
fn write(p:&Path,j:&J){fs::write(p,json::encode(j,300000).expect("bounded report")).expect("report write")}
fn rejection(e:&mut Evidence,r:Rejection){e.result=if matches!(r.guard,"D01"|"D07"){"NO_ACREDITADO"}else{"RECHAZADO"}.into();e.guard=Some(r.guard.into());e.cause=r.cause;e.passed=r.passed.iter().map(|x|(*x).into()).collect();}
// ES: Captura los bytes que entran en el receptor, no los declarados en el plan.
// EN: Captures bytes entering the receiver, not bytes asserted by the plan.
fn receiver(input:&[u8],context:J,captor:String)->Capture{Capture{bytes:input.to_vec(),context,captor}}
fn run_case(dir:&Path,registry:&TrustedRegistry,canonical:&[u8])->Evidence{
 let context=TrustedContext::from_custody(&bytes(&dir.join("context.json"),8192)).expect("trusted context");
 let mut meta=File::open(dir.join("request.json")).expect("metadata");let mut source=File::open(dir.join("source.bin")).expect("source");let mut state=File::open(dir.join("state.bin")).expect("state");let mut support=if dir.join("support.bin").exists(){Some(File::open(dir.join("support.bin")).expect("support"))}else{None};let mut geometry=File::open(dir.join("geometry.bin")).expect("geometry");
 let mut e=Evidence{result:String::new(),guard:None,cause:String::new(),passed:vec![],receipt:None,capture:None,dispatches:Some(0),attempts:0,before:canonical.to_vec(),after:canonical.to_vec(),admitted_vector:None,preservation_claim:true};
 match ReceivedBytes::read(&mut meta,&mut source,&mut state,support.as_mut(),&mut geometry).and_then(|r|admit(r,&context,registry)){
 Err(r)=>rejection(&mut e,r),Ok(a)=>{
 e.admitted_vector=Some(J::Array(a.vector().iter().map(|t|s(t.ir_label())).collect()));
 let plan=document(&dir.join("plan.json"));e.attempts=1;
 // ES: El plan sólo introduce una sustitución externa cuando difiere del buffer.
 // EN: The plan introduces an external substitution only when it differs from the buffer.
 if plan.field("producir_captura").boolean()==Some(true){
 let replacement=if plan.field("buffer_capturado").field("sha256").text()!=Some(sha::hash(a.descriptor()).as_str()){Some(bytes(&dir.join("injection.bin"),8192))}else{None};
 let transported=replacement.as_deref().unwrap_or(a.descriptor());
 e.capture=Some(receiver(transported,plan.field("contexto_captura").clone(),plan.field("captor").text().expect("captor").into()));e.dispatches=Some(1);
 }else{e.dispatches=None;}
 match certify(&a,e.capture.as_ref(),plan.field("fallo_despues_de_despacho").boolean()==Some(true),registry){Err(r)=>rejection(&mut e,r),Ok(receipt)=>{e.result="ENTREGA_DOCUMENTAL_CONCORDANTE".into();e.receipt=Some(receipt);e.passed=a.passed().iter().chain(["D01","D02","D03","D04","D05","D06"].iter()).map(|x|(*x).into()).collect();}}
 }}e
}
fn instrumental(root:&Path,out:&Path)->usize{
 let mut failures=0;for i in 1..=19{let id=if i<=14{format!("J{i:02}")}else{format!("H{:02}",i-14)};let dir=root.join(&id);let input=bytes(&dir.join("input.bin"),8192);let oracle=document(&dir.join("expected.json"));let mut faults=vec![];
 let observed=if i<=14{match json::decode(&input){Err(e)=>{if oracle.field("causa").text()!=Some(e){faults.push(s("unexpected JSON error"))}obj(vec![("resultado",s("RECHAZADO")),("causa",s(e))])},Ok(j)=>{
 if oracle.field("resultado").text()!=Some("ADMISIBLE_DECODIFICADOR"){faults.push(s("unexpected JSON acceptance"))}
 if let Some(lexeme)=oracle.field("lexema_numero").text(){if j.lexeme()!=Some(lexeme){faults.push(s("lexeme"))}}
 if let Some(pairs)=oracle.field("valores_texto").pairs(){for(k,v)in pairs{if j.field(k)!=v{faults.push(s("decoded text"))}}}
 if id=="J03"&&j.uint().is_some(){faults.push(s("overflow silently accepted as usize"))}
 obj(vec![("resultado",s("ADMISIBLE_DECODIFICADOR")),("ast",j)])}}else{let h=sha::hash(&input);if oracle.field("sha256").text()!=Some(h.as_str()){faults.push(s("SHA mismatch"))}obj(vec![("sha256",s(&h))])};
 if !faults.is_empty(){failures+=1;}write(&out.join(format!("{id}.json")),&obj(vec![("id",s(&id)),("observado",observed),("discrepancias",J::Array(faults))]));println!("{id}: {}",if failures==0{"processed"}else{"inspect results"});
 }failures
}
fn main(){let args:Vec<String>=std::env::args().collect();assert!(args.len()==3,"input directory and output directory required");let input=PathBuf::from(&args[1]);let out=PathBuf::from(&args[2]);fs::create_dir_all(&out).expect("output");
 let started=std::time::Instant::now();let mut failures=instrumental(&input.join("instrumental"),&out);
 let registry=TrustedRegistry::from_custody(&bytes(&input.join("registry.json"),8192),&bytes(&input.join("constitution.bin"),1024),&bytes(&input.join("convention.bin"),1024),&bytes(&input.join("transforms.bin"),2048)).expect("registry");
 let canonical=bytes(&input.join("canonical.bin"),1024);let mut evidence=Vec::new();let mut oracle_store=Vec::new();let mut expected_store=Vec::new();
 for i in 1..=26{let id=format!("I0205-{i:02}");let dir=input.join(&id);let oracle=document(&dir.join("oracle.json"));let expected=if dir.join("expected.bin").exists(){Some(bytes(&dir.join("expected.bin"),8192))}else{None};let e=run_case(&dir,&registry,&canonical);let faults=inspect(&e,&oracle,&canonical,expected.as_deref());if !faults.is_empty(){failures+=1;}println!("{id}: {} {:?}; observer {:?}",e.result,e.guard,faults);write(&out.join(format!("{id}.json")),&obj(vec![("id",s(&id)),("observado",e.documentary()),("discrepancias",J::Array(faults.iter().map(|x|s(x)).collect()))]));evidence.push(e);oracle_store.push(oracle);expected_store.push(expected);}
 for (run,index) in [5,4,5].into_iter().enumerate(){let id=format!("I0205-{index:02}");let e=run_case(&input.join(&id),&registry,&canonical);let faults=inspect(&e,&oracle_store[index-1],&canonical,expected_store[index-1].as_deref());if !faults.is_empty(){failures+=1;}write(&out.join(format!("SEQ-{}.json",run+1)),&obj(vec![("caso",s(&id)),("observado",e.documentary()),("discrepancias",J::Array(faults.iter().map(|x|s(x)).collect()))]));println!("SEQ {} {id}: {:?}",run+1,e.guard);}
 // ES: Mutantes de evidencia comprometidos antes de observar; no alteran los oráculos.
 // EN: Evidence mutants committed before observation; do not modify the oracles.
 for i in 0..4{let index=[0,8,11,23][i];let mut e=evidence[index].clone();match i{
 0=>{e.capture.as_mut().expect("OBS01 base capture").context=document(&input.join("I0205-02/context.json")).field("entrega").clone();},
 1=>{e.result="ENTREGA_DOCUMENTAL_CONCORDANTE".into();e.guard=None;e.receipt=evidence[0].receipt.clone();e.receipt.as_mut().unwrap().set("geometria_sha256",s(&sha::hash(&bytes(&input.join("I0205-09/geometry.bin"),8192))));e.capture=Some(receiver(&bytes(&input.join("I0205-09/geometry.bin"),8192),document(&input.join("I0205-09/context.json")).field("entrega").clone(),"captor-local-i0205/1".into()));},
 2=>{e.result="ENTREGA_DOCUMENTAL_CONCORDANTE".into();e.guard=None;e.receipt=evidence[0].receipt.clone();},
 _=>{let mut state=json::decode(&canonical).expect("canonical");state.set("vector",J::Array((0..16).map(|_|s("U")).collect()));e.after=json::encode(&state,1024).unwrap();e.preservation_claim=true;}}
 let faults=inspect(&e,&oracle_store[index],&canonical,expected_store[index].as_deref());if faults.is_empty(){failures+=1;}write(&out.join(format!("OBS-{:02}.json",i+1)),&obj(vec![("evidencia_mutada",e.documentary()),("discrepancias",J::Array(faults.iter().map(|x|s(x)).collect()))]));println!("OBS-{:02}: {:?}",i+1,faults);}
 // ES: La lectura externa se ejerce sobre un objeto realmente admitido.
 // EN: External read access is exercised on an actually admitted object.
 let dir=input.join("I0205-01");let mut m=Cursor::new(bytes(&dir.join("request.json"),4096));let mut src=Cursor::new(bytes(&dir.join("source.bin"),4096));let mut st=Cursor::new(bytes(&dir.join("state.bin"),1024));let mut sp=Cursor::new(bytes(&dir.join("support.bin"),1024));let mut geo=Cursor::new(bytes(&dir.join("geometry.bin"),8192));
 let a=admit(ReceivedBytes::read(&mut m,&mut src,&mut st,Some(&mut sp),&mut geo).unwrap(),&TrustedContext::from_custody(&bytes(&dir.join("context.json"),8192)).unwrap(),&registry).unwrap();
 let readonly=a.vector().len()==16&&a.identity().documentary().field("revision").text()==Some("r1")&&a.context().documentary().field("invocacion").text()==Some("I0205-01")&&a.descriptor()==expected_store[0].as_ref().unwrap().as_slice();if !readonly{failures+=1;}
 write(&out.join("RESUMEN.json"),&obj(vec![("fallos",n(failures)),("instrumentales",n(19)),("integrados",n(26)),("secuencia_invocaciones",n(3)),("sensibilidades",n(4)),("lectura_externa_objeto_admitido",J::Bool(readonly)),("invocacion_lectura_adicional",n(1)),("fixed_vector_bytes",n(a.storage_bytes())),("duracion_ns",J::Number(started.elapsed().as_nanos().to_string()))]));
 if failures>0{std::process::exit(1)}
}
