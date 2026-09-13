// Incluido dentro de la copia acotada del conductor I0205.
// Etiquetas del ensayo, no nuevos estados ni códigos del Lenguaje SV.
const WITNESS_LIMIT: usize = 1024;
#[derive(Clone,Debug)]
struct ReadFailure { stage: &'static str, cause: String, bytes_seen: usize }
#[derive(Clone,Debug)]
struct Observation { phase: &'static str, result: Result<Vec<u8>,ReadFailure> }
impl Observation {
    fn read(path:&Path,phase:&'static str)->Self {
        use std::io::Read;
        let result=(|| {
            let f=File::open(path).map_err(|e|ReadFailure{stage:"open",cause:format!("{:?}",e.kind()),bytes_seen:0})?;
            let mut bytes=Vec::with_capacity(WITNESS_LIMIT+1);
            f.take((WITNESS_LIMIT+1) as u64).read_to_end(&mut bytes)
                .map_err(|e|ReadFailure{stage:"read",cause:format!("{:?}",e.kind()),bytes_seen:bytes.len()})?;
            if bytes.len()>WITNESS_LIMIT {return Err(ReadFailure{stage:"quota",cause:"witness exceeds 1024 bytes".into(),bytes_seen:bytes.len()});}
            Ok(bytes)
        })();
        Self{phase,result}
    }
    fn documentary(&self)->J {
        match &self.result {
            Ok(b)=>obj(vec![("fase",s(self.phase)),("estado",s("LEIDO")),("bytes_leidos",n(b.len())),
                ("contenido",J::Array(b.iter().map(|x|n(*x as usize)).collect()))]),
            Err(e)=>obj(vec![("fase",s(self.phase)),("estado",s("FALLO_LECTURA")),
                ("etapa",s(e.stage)),("causa",s(&e.cause)),("bytes_leidos",n(e.bytes_seen)),("contenido",J::Null)]),
        }
    }
}
pub struct MaterialRun {
    path: PathBuf,
    before: Observation,
    after: Option<Observation>,
    delivery: Option<Evidence>,
    status: &'static str,
}
impl MaterialRun {
    pub fn status(&self)->&'static str {self.status}
    pub fn receipt(&self)->Option<&J> {self.delivery.as_ref().and_then(|e|e.receipt.as_ref())}
    pub fn attempts(&self)->usize {self.delivery.as_ref().map_or(0,|e|e.attempts)}
    // Adaptación al observador histórico sólo cuando existen ambas lecturas.
    // No convierte un fallo de lectura en un vector vacío ni en U.
    pub fn legacy_evidence(&self)->Option<Evidence> {
        let before=self.before.result.as_ref().ok()?;
        let after=self.after.as_ref()?.result.as_ref().ok()?;
        let mut e=self.delivery.as_ref()?.clone();
        e.before=before.clone();e.after=after.clone();
        e.preservation_claim=self.status=="CONCORDANCIA_OBSERVADA";
        Some(e)
    }
    pub fn documentary(&self)->J {
        let delivery=self.delivery.as_ref().map(|e| {
            let mut j=e.documentary();
            // Los campos heredados no se ofrecen como observaciones independientes.
            j.set("estado_antes",J::Null);j.set("estado_despues",J::Null);
            j.set("afirmacion_preservacion",J::Bool(false));j
        }).unwrap_or(J::Null);
        obj(vec![("version",s("S26-R04-OBSERVACION/1")),("ruta_custodiada",s(&self.path.to_string_lossy())),
            ("preservacion",s(self.status)),("antes",self.before.documentary()),
            ("despues",self.after.as_ref().map(|o|o.documentary()).unwrap_or(J::Null)),
            ("entrega_previa",delivery)])
    }
}
fn observed_case(dir:&Path,registry:&TrustedRegistry,expected:&[u8],after_operation:impl FnOnce())->MaterialRun {
    let path=dir.join("state.bin");
    let before=Observation::read(&path,"antes_de_operacion");
    if before.result.as_ref().map_or(true,|b|b.as_slice()!=expected) {
        let status=if before.result.is_err(){"NO_ACREDITADO"}else{"ALTERACION_DETECTADA"};
        return MaterialRun{path,before,after:None,delivery:None,status};
    }
    let delivery=run_case_antecedente(dir,registry,expected);
    after_operation();
    let after=Observation::read(&path,"despues_de_operacion");
    let status=match &after.result {Err(_)=>"NO_ACREDITADO",Ok(b) if b==expected=>"CONCORDANCIA_OBSERVADA",Ok(_)=>"ALTERACION_DETECTADA"};
    MaterialRun{path,before,after:Some(after),delivery:Some(delivery),status}
}
pub fn run_observed_case(dir:&Path,registry:&TrustedRegistry,expected:&[u8])->MaterialRun {
    observed_case(dir,registry,expected,||{})
}
#[cfg(test)]
pub(crate) fn run_with_injection(dir:&Path,registry:&TrustedRegistry,expected:&[u8],hook:impl FnOnce())->MaterialRun {
    observed_case(dir,registry,expected,hook)
}
