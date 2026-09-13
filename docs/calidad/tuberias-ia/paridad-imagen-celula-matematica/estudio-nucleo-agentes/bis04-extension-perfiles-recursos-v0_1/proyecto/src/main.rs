//! ES: Conductor local, receptor y observador experimental, sin servicios externos.
//! EN: Local harness, receiver and experimental observer, without external services.
mod observer;
use observer::{inspect, Evidence};
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};
use sv_bis_i0205::{
    json::{n, obj, s, J},
    *,
};
fn bytes(p: &Path, max: usize) -> Vec<u8> {
    json::receive(
        &mut File::open(p).unwrap_or_else(|e| panic!("loader {}: {e}", p.display())),
        max,
    )
    .unwrap_or_else(|e| panic!("loader {}: {e}", p.display()))
}
fn document(p: &Path) -> J {
    json::decode(&bytes(p, 8192))
        .unwrap_or_else(|e| panic!("custody document {}: {e}", p.display()))
}
fn write(p: &Path, j: &J) {
    fs::write(p, json::encode(j, 300000).expect("bounded report")).expect("report write")
}
fn rejection(e: &mut Evidence, r: Rejection) {
    e.result = if matches!(r.guard, "D01" | "D07") {
        "NO_ACREDITADO"
    } else {
        "RECHAZADO"
    }
    .into();
    e.guard = Some(r.guard.into());
    e.cause = r.cause;
    e.passed = r.passed.iter().map(|x| (*x).into()).collect();
}
// ES: Captura los bytes que entran en el receptor, no los declarados en el plan.
// EN: Captures bytes entering the receiver, not bytes asserted by the plan.
fn receiver(input: &[u8], context: J, captor: String) -> Capture {
    Capture {
        bytes: input.to_vec(),
        context,
        captor,
    }
}
// ES: Contabiliza bytes devueltos por cada canal real al receptor acotado.
// EN: Counts bytes returned by each real channel to the bounded receiver.
struct Counted { file: File, count: usize }
impl Counted { fn open(p: impl AsRef<Path>) -> Self {Self {file: File::open(p).expect("channel"),count:0}} }
impl Read for Counted { fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {let n=self.file.read(b)?;self.count+=n;Ok(n)} }
fn run_case(dir: &Path, registry: &TrustedRegistry, canonical: &[u8]) -> (Evidence, [usize;5]) {
    let context = TrustedContext::from_custody(&bytes(&dir.join("context.json"), 8192))
        .expect("trusted context");
    let mut meta = Counted::open(dir.join("request.json"));
    let mut source = Counted::open(dir.join("source.bin"));
    let mut state = Counted::open(dir.join("state.bin"));
    let mut support = if dir.join("support.bin").exists() {
        Some(Counted::open(dir.join("support.bin")))
    } else {
        None
    };
    let mut geometry = Counted::open(dir.join("geometry.bin"));
    let mut e = Evidence {
        result: String::new(),
        guard: None,
        cause: String::new(),
        passed: vec![],
        receipt: None,
        capture: None,
        dispatches: Some(0),
        attempts: 0,
        before: canonical.to_vec(),
        after: canonical.to_vec(),
        admitted_vector: None,
        preservation_claim: true,
    };
    match ReceivedBytes::read(
        &mut meta,
        &mut source,
        &mut state,
        support.as_mut(),
        &mut geometry,
    )
    .and_then(|r| admit(r, &context, registry))
    {
        Err(r) => rejection(&mut e, r),
        Ok(a) => {
            e.admitted_vector = Some(J::Array(
                a.vector().iter().map(|t| s(t.ir_label())).collect(),
            ));
            let plan = document(&dir.join("plan.json"));
            e.attempts = 1;
            // ES: El plan sólo introduce una sustitución externa cuando difiere del buffer.
            // EN: The plan introduces an external substitution only when it differs from the buffer.
            if plan.field("producir_captura").boolean() == Some(true) {
                let replacement = if plan.field("buffer_capturado").field("sha256").text()
                    != Some(sha::hash(a.descriptor()).as_str())
                {
                    Some(bytes(&dir.join("injection.bin"), 8192))
                } else {
                    None
                };
                let transported = replacement.as_deref().unwrap_or(a.descriptor());
                e.capture = Some(receiver(
                    transported,
                    plan.field("contexto_captura").clone(),
                    plan.field("captor").text().expect("captor").into(),
                ));
                e.dispatches = Some(1);
            } else {
                e.dispatches = None;
            }
            match certify(
                &a,
                e.capture.as_ref(),
                plan.field("fallo_despues_de_despacho").boolean() == Some(true),
                registry,
            ) {
                Err(r) => rejection(&mut e, r),
                Ok(receipt) => {
                    e.result = "ENTREGA_DOCUMENTAL_CONCORDANTE".into();
                    e.receipt = Some(receipt);
                    e.passed = a
                        .passed()
                        .iter()
                        .chain(["D01", "D02", "D03", "D04", "D05", "D06"].iter())
                        .map(|x| (*x).into())
                        .collect();
                }
            }
        }
    }
    (e, [meta.count,source.count,state.count,support.map(|x|x.count).unwrap_or(0),geometry.count])
}

// ES: Comparación canónica completa de objetos y operaciones; procedencia separada.
// EN: Full canonical object and operation comparison; provenance checked separately.
fn parity(input: &Path, out: &Path) -> usize {
 let specs=document(&input.join("PARIDAD_PREVIA.json"));let mut failures=0;
 for spec in specs.array().expect("parity bank") {
  let id=spec.field("id").text().unwrap();let an=spec.field("a").text().unwrap();let bn=spec.field("b").text().unwrap();
  let a=bytes(&input.join(an),4096);let b=bytes(&input.join(bn),4096);
  let ca=sv_core::compile_svp_profile(std::str::from_utf8(&a).unwrap(),an,sv_core::SourceProfile::from_tag(spec.field("perfil_a").text().unwrap()).unwrap());
  let cb=sv_core::compile_svp_profile(std::str::from_utf8(&b).unwrap(),bn,sv_core::SourceProfile::from_tag(spec.field("perfil_b").text().unwrap()).unwrap());
  let mut faults=vec![];let mut equal=J::Null;let mut objects=J::Null;let mut operations=J::Null;
  match (ca,cb) {
   (Ok(a_ir),Ok(b_ir))=>{
    let same=a_ir.objects()==b_ir.objects() && a_ir.operations()==b_ir.operations();equal=J::Bool(same);
    objects=n(a_ir.objects().len());operations=n(a_ir.operations().len());
    if Some(same)!=spec.field("equivalentes").boolean(){faults.push(s("canonical comparison"));}
    let minimum=spec.field("operaciones_minimas").uint().unwrap();
    if a_ir.objects().is_empty() || b_ir.objects().is_empty() || a_ir.operations().len()<minimum || b_ir.operations().len()<minimum {faults.push(s("vacuous comparison"));}
    if a_ir.source_file()!=an || b_ir.source_file()!=bn || a_ir.source_sha256()!=sha::hash(&a) || b_ir.source_sha256()!=sha::hash(&b) || a==b {faults.push(s("source provenance"));}
   },
   (a,b)=>{faults.push(s(&format!("compilation: {a:?} {b:?}")));}
  }
  if !faults.is_empty(){failures+=1;}
  println!("{id}: equivalent {equal:?}; observer {faults:?}");
  write(&out.join(format!("{id}.json")),&obj(vec![("id",s(id)),("equivalentes",equal),("objetos_a",objects),("operaciones_a",operations),("fuente_a_sha256",s(&sha::hash(&a))),("fuente_b_sha256",s(&sha::hash(&b))),("discrepancias",J::Array(faults))]));
 }
 failures
}
fn main() {
 let args:Vec<String>=std::env::args().collect();assert_eq!(args.len(),3);
 let input=PathBuf::from(&args[1]);let out=PathBuf::from(&args[2]);fs::create_dir_all(&out).expect("output");
 let ids=document(&input.join("CASOS.json"));let mut failures=0;
 for id in ids.array().expect("case bank") {
  let id=id.text().unwrap();let dir=input.join(id);
  let registry=TrustedRegistry::from_custody(&bytes(&dir.join("registry.json"),8192),&bytes(&dir.join("constitution.bin"),1024),&bytes(&dir.join("convention.bin"),1024),&bytes(&dir.join("transforms.bin"),2048)).expect("registry");
  // ES: El estado testigo se carga por custodia; el exceso es un estímulo de recepción.
  // EN: Custody loads the witness state; excess length is a reception stimulus.
  let canonical=bytes(&dir.join("canonical.bin"),2048);let oracle=document(&dir.join("oracle.json"));
  let expected=if dir.join("expected.bin").exists(){Some(bytes(&dir.join("expected.bin"),8192))}else{None};
  let (e,counts)=run_case(&dir,&registry,&canonical);let mut faults=inspect(&e,&oracle,&canonical,expected.as_deref());
  let measured=J::Array(counts.into_iter().map(n).collect());
  if &measured!=oracle.field("bytes_leidos_esperados"){faults.push("read byte counts".into());}
  if let Some(cap)=&e.capture {
   let plan=document(&dir.join("plan.json"));
   if &cap.context!=plan.field("contexto_captura") || Some(cap.captor.as_str())!=plan.field("captor").text() || Some(sha::hash(&cap.bytes).as_str())!=plan.field("buffer_capturado").field("sha256").text(){faults.push("actual capture versus committed stimulus".into());}
  }
  if !faults.is_empty(){failures+=1;}
  println!("{id}: {} {:?}; observer {:?}",e.result,e.guard,faults);
  write(&out.join(format!("{id}.json")),&obj(vec![("id",s(id)),("observado",e.documentary()),("bytes_leidos",measured),("discrepancias",J::Array(faults.iter().map(|x|s(x)).collect()))]));
 }
 failures+=parity(&input,&out);
 write(&out.join("RESUMEN.json"),&obj(vec![("integrados",n(ids.array().unwrap().len())),("comparaciones_paridad",n(5)),("fallos",n(failures))]));
 if failures!=0{std::process::exit(1)}
}
