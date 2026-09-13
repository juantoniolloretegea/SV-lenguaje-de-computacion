# ES: Prepara el conductor Rust; sin ejecución del sujeto.
# EN: Prepares the Rust harness; no subject execution.
exec(open('bis-extension/preparar.py').read().split('for folder in (D,I,')[0])
p=E/'proyecto';(p/'src').mkdir(parents=True,exist_ok=True)
(p/'Cargo.toml').write_text('[package]\nname="sv_bis_extension"\nversion="0.1.0"\nedition="2021"\npublish=false\n[workspace]\n[dependencies]\nsv_bis_i0205={path="../sv_bis_i0205"}\nsv_core={path="../sv_core"}\n')
old=(D/'proyecto/src/main.rs').read_text();code=old[:old.index('fn instrumental(')].replace('io::Cursor,','io::Read,')
code=code.replace('fn run_case(dir: &Path, registry: &TrustedRegistry, canonical: &[u8]) -> Evidence {','''// ES: Contabiliza bytes devueltos por cada canal real al receptor acotado.
// EN: Counts bytes returned by each real channel to the bounded receiver.
struct Counted { file: File, count: usize }
impl Counted { fn open(p: impl AsRef<Path>) -> Self {Self {file: File::open(p).expect("channel"),count:0}} }
impl Read for Counted { fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {let n=self.file.read(b)?;self.count+=n;Ok(n)} }
fn run_case(dir: &Path, registry: &TrustedRegistry, canonical: &[u8]) -> (Evidence, [usize;5]) {''')
for name,label in [('meta','metadata'),('source','source'),('state','state'),('geometry','geometry')]:
 code=code.replace(f'File::open(dir.join("'+{'meta':'request.json','source':'source.bin','state':'state.bin','geometry':'geometry.bin'}[name]+f'")).expect("{label}")',f'Counted::open(dir.join("'+{'meta':'request.json','source':'source.bin','state':'state.bin','geometry':'geometry.bin'}[name]+'"))')
code=code.replace('Some(File::open(dir.join("support.bin")).expect("support"))','Some(Counted::open(dir.join("support.bin")))')
code=code.replace('    e\n}', '    (e, [meta.count,source.count,state.count,support.map(|x|x.count).unwrap_or(0),geometry.count])\n}')
code+='''
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
'''
(p/'src/main.rs').write_text(code)
obs=(D/'proyecto/src/observer.rs').read_text();a=obs.index('    let stages =');b=obs.index('    if let Some(receipt)',a)
obs=obs[:a]+'''    demand!(
        oracle.field("guardas_previas").array().is_some_and(|expected|
            e.passed.iter().map(String::as_str).eq(expected.iter().filter_map(J::text))),
        "precedencia/traza"
    );
'''+obs[b:];(p/'src/observer.rs').write_text(obs)
# ES: Lockfile de paquetes locales; no resuelve dependencias de red.
# EN: Local-package lockfile; resolves no network dependencies.
lock=(D/'proyecto/Cargo.lock').read_text();lock+='\n[[package]]\nname = "sv_bis_extension"\nversion = "0.1.0"\ndependencies = [\n "sv_bis_i0205",\n "sv_core",\n]\n';(p/'Cargo.lock').write_text(lock)
print('Conductor y observador escritos; núcleo y admisión intactos.')
