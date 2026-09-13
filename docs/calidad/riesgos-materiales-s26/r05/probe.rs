#[path="conductor.rs"]
mod conductor;
#[cfg(all(test,not(target_os="linux")))]
compile_error!("S26 R05 cualifica exclusivamente esta realización Linux");
#[cfg(test)]
mod tests {
    use super::conductor;
    use std::{fs,path::PathBuf,panic::{catch_unwind,AssertUnwindSafe},os::unix::fs::MetadataExt};
    use sv_bis_i0205::{json::{self,J,obj,s},TrustedRegistry};
    fn fixture()->PathBuf{std::env::var_os("S26_FIXTURE").unwrap().into()}
    fn work()->PathBuf{std::env::var_os("S26_WORK").unwrap().into()}
    fn raw(n:&str)->Vec<u8>{fs::read(fixture().join(n)).unwrap()}
    fn doc(n:&str)->J{json::decode(&raw(n)).unwrap()}
    fn setup(id:&str)->(PathBuf,TrustedRegistry,Vec<u8>) {
        let dir=work().join(id);assert!(!dir.exists());fs::create_dir(&dir).unwrap();
        for p in fs::read_dir(fixture()).unwrap(){let p=p.unwrap().path();fs::copy(&p,dir.join(p.file_name().unwrap())).unwrap();}
        let r=TrustedRegistry::from_custody(&raw("registry.json"),&raw("constitution.bin"),&raw("convention.bin"),&raw("transforms.bin")).unwrap();
        (dir,r,raw("canonical.bin"))
    }
    fn save(id:&str,j:&J){fs::write(work().join(format!("{id}.json")),json::encode(j,300000).unwrap()).unwrap();}
    fn bytes(j:&J)->Vec<u8>{j.field("contenido").array().unwrap().iter().map(|v|u8::try_from(v.uint().unwrap()).unwrap()).collect()}
    #[test]
    fn p01_positive(){
        let(d,r,c)=setup("P01");let x=conductor::run_observed_case(&d,&r,&c);
        assert_eq!(x.status(),"CONCORDANCIA_OBSERVADA");assert_eq!(x.attempts(),1);
        assert_eq!(x.receipt(),Some(doc("oracle.json").field("recibo_esperado")));save("P01",&x.documentary());
    }
    #[test]
    fn p02_rejection_returns_and_rereads(){
        let(d,r,c)=setup("P02");fs::write(d.join("request.json"),b"{").unwrap();
        let x=conductor::run_observed_case(&d,&r,&c);let j=x.documentary();
        assert_eq!(x.status(),"CONCORDANCIA_OBSERVADA");assert_eq!(x.attempts(),0);assert!(x.receipt().is_none());
        assert_eq!(bytes(j.field("despues")),c);assert_eq!(j.field("entrega_previa").field("resultado"),&s("RECHAZADO"));save("P02",&j);
    }
    #[test]
    fn p03_missing_capture_is_not_preserved_delivery(){
        let(d,r,c)=setup("P03");let mut plan=doc("plan.json");plan.set("producir_captura",J::Bool(false));
        fs::write(d.join("plan.json"),json::encode(&plan,8192).unwrap()).unwrap();
        let x=conductor::run_observed_case(&d,&r,&c);let j=x.documentary();
        assert_eq!(x.status(),"CONCORDANCIA_OBSERVADA");assert_eq!(x.attempts(),1);assert!(x.receipt().is_none());
        assert_eq!(bytes(j.field("despues")),c);assert_eq!(j.field("entrega_previa").field("resultado"),&s("NO_ACREDITADO"));
        assert_eq!(j.field("entrega_previa").field("primera_guarda"),&s("D01"));save("P03",&j);
    }
    #[test]
    fn p04_loader_panic_has_no_final_observation(){
        let(d,r,c)=setup("P04");fs::remove_file(d.join("context.json")).unwrap();let mut returned=None;
        let outcome=catch_unwind(AssertUnwindSafe(||{returned=Some(conductor::run_observed_case(&d,&r,&c));}));
        assert!(outcome.is_err());assert!(returned.is_none());assert_eq!(fs::read(d.join("state.bin")).unwrap(),c);
        save("P04",&obj(vec![("panico_observado",J::Bool(true)),("informe_recepcion",J::Null),("lectura_posterior_del_arnes",s("bytes iguales al control")),("intentos_del_recorrido",J::Null),("limite",s("Lectura externa posterior al unwind; no es la relectura del conductor ni demuestra ausencia de efectos"))]));
    }
    #[test]
    fn p05_panic_after_operation_does_not_mean_zero_effects(){
        let(d,r,c)=setup("P05");let altered=raw("vector-alterado.bin");let mut reached=false;let mut returned=None;
        let outcome=catch_unwind(AssertUnwindSafe(||{returned=Some(conductor::run_with_injection(&d,&r,&c,||{
            reached=true;fs::write(d.join("state.bin"),&altered).unwrap();panic!("S26 R05 inyección posterior a operación");}));}));
        assert!(outcome.is_err());assert!(reached);assert!(returned.is_none());assert_eq!(fs::read(d.join("state.bin")).unwrap(),altered);
        save("P05",&obj(vec![("panico_observado",J::Bool(true)),("hook_posterior_alcanzado",J::Bool(reached)),("informe_recepcion",J::Null),("intentos_del_recorrido",J::Null),("estado_posterior_del_arnes",json::decode(&altered).unwrap()),("limite",s("La operación retornó al hook, pero no existe MaterialRun final; no se reconstruye un recibo ni se afirma rollback"))]));
    }
    #[test]
    fn p06_replaced_path_open_reader_keeps_previous_bytes(){
        let(d,r,c)=setup("P06");let altered=raw("vector-alterado.bin");let old=fs::metadata(d.join("state.bin")).unwrap().ino();
        fs::write(d.join("replacement.bin"),&altered).unwrap();let replacement=fs::metadata(d.join("replacement.bin")).unwrap().ino();assert_ne!(old,replacement);
        let x=conductor::run_with_source_injection(&d,&r,&c,||fs::rename(d.join("replacement.bin"),d.join("state.bin")).unwrap());let mut j=x.documentary();
        assert_eq!(bytes(j.field("antes")),c);assert_eq!(bytes(j.field("despues")),altered);
        assert_eq!(x.status(),"ALTERACION_DETECTADA");assert_eq!(x.attempts(),0);assert!(x.receipt().is_none());
        assert_eq!(fs::metadata(d.join("state.bin")).unwrap().ino(),replacement);
        j.set("inode_antes",s(&old.to_string()));j.set("inode_reemplazo",s(&replacement.to_string()));save("P06",&j);
    }
    #[test]
    fn p07_same_bytes_do_not_accredit_file_identity(){
        let(d,r,c)=setup("P07");let old=fs::metadata(d.join("state.bin")).unwrap().ino();
        fs::write(d.join("replacement.bin"),&c).unwrap();let replacement=fs::metadata(d.join("replacement.bin")).unwrap().ino();assert_ne!(old,replacement);
        let x=conductor::run_with_source_injection(&d,&r,&c,||fs::rename(d.join("replacement.bin"),d.join("state.bin")).unwrap());let mut j=x.documentary();
        assert_eq!(bytes(j.field("antes")),c);assert_eq!(bytes(j.field("despues")),c);assert_eq!(x.status(),"CONCORDANCIA_OBSERVADA");
        assert_eq!(x.receipt(),Some(doc("oracle.json").field("recibo_esperado")));assert_eq!(x.attempts(),1);
        assert_eq!(fs::metadata(d.join("state.bin")).unwrap().ino(),replacement);
        j.set("inode_antes",s(&old.to_string()));j.set("inode_reemplazo",s(&replacement.to_string()));j.set("identidad_archivo_preservada_en_esta_sonda",J::Bool(false));
        j.set("limite",s("Concordancia de bytes y entrega no acreditan identidad del objeto abierto; inode es observación local, no identidad universal"));save("P07",&j);
    }
}
