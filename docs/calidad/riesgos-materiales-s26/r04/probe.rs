#[path="conductor.rs"]
mod conductor;
#[cfg(test)]
mod tests {
    use super::conductor;
    use std::{fs,path::PathBuf};
    use sv_bis_i0205::{json::{self,J},TrustedRegistry};
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
    fn save(id:&str,r:&conductor::MaterialRun){
        fs::write(work().join(format!("{id}.json")),json::encode(&r.documentary(),300000).unwrap()).unwrap();
    }
    fn delivered(r:&conductor::MaterialRun){assert_eq!(r.attempts(),1);assert_eq!(r.receipt(),Some(doc("oracle.json").field("recibo_esperado")));}
    #[test]
    fn p01_real_read_positive(){
        let(d,r,c)=setup("P01");let result=conductor::run_observed_case(&d,&r,&c);
        delivered(&result);assert_eq!(result.status(),"CONCORDANCIA_OBSERVADA");
        let e=result.legacy_evidence().unwrap();assert_eq!(e.before,c);assert_eq!(e.after,c);
        assert!(conductor::inspect(&e,&doc("oracle.json"),&c,Some(&raw("expected.bin"))).is_empty());save("P01",&result);
    }
    #[test]
    fn p02_literal_change(){
        let(d,r,c)=setup("P02");let mut b=c.clone();b.push(b' ');
        let result=conductor::run_with_injection(&d,&r,&c,||fs::write(d.join("state.bin"),&b).unwrap());
        delivered(&result);assert_eq!(result.status(),"ALTERACION_DETECTADA");
        let e=result.legacy_evidence().unwrap();assert_eq!(e.after,b);
        assert_eq!(conductor::inspect(&e,&doc("oracle.json"),&c,Some(&raw("expected.bin"))),vec!["preservacion real del estado"]);save("P02",&result);
    }
    #[test]
    fn p03_vector_change_and_copied_witness_sensitivity(){
        let(d,r,c)=setup("P03");let b=raw("vector-alterado.bin");
        let before=doc("canonical.bin");let after=json::decode(&b).unwrap();
        assert_eq!(before.field("vector").array().unwrap()[0],json::s("Zero"));
        assert_eq!(after.field("vector").array().unwrap()[0],json::s("One"));
        assert_eq!(&before.field("vector").array().unwrap()[1..],&after.field("vector").array().unwrap()[1..]);
        let result=conductor::run_with_injection(&d,&r,&c,||fs::write(d.join("state.bin"),&b).unwrap());
        delivered(&result);assert_eq!(result.status(),"ALTERACION_DETECTADA");
        let mut e=result.legacy_evidence().unwrap();assert_eq!(e.after,b);
        assert_eq!(conductor::inspect(&e,&doc("oracle.json"),&c,Some(&raw("expected.bin"))),vec!["preservacion real del estado"]);
        e.after=e.before.clone();e.preservation_claim=true;
        assert!(conductor::inspect(&e,&doc("oracle.json"),&c,Some(&raw("expected.bin"))).is_empty());
        save("P03",&result);
    }
    #[test]
    fn p04_missing_after_does_not_erase_delivery(){
        let(d,r,c)=setup("P04");let result=conductor::run_with_injection(&d,&r,&c,||fs::remove_file(d.join("state.bin")).unwrap());
        delivered(&result);assert_eq!(result.status(),"NO_ACREDITADO");assert!(result.legacy_evidence().is_none());
        assert_eq!(result.documentary().field("despues").field("etapa"),&json::s("open"));save("P04",&result);
    }
    #[test]
    fn p05_missing_before_stops_operation(){
        let(d,r,c)=setup("P05");fs::remove_file(d.join("state.bin")).unwrap();
        let result=conductor::run_with_injection(&d,&r,&c,||panic!("operation reached"));
        assert_eq!(result.status(),"NO_ACREDITADO");assert_eq!(result.attempts(),0);assert!(result.receipt().is_none());
        assert_eq!(result.documentary().field("despues"),&J::Null);save("P05",&result);
    }
    #[test]
    fn p06_excess_after_is_not_u(){
        let(d,r,c)=setup("P06");let result=conductor::run_with_injection(&d,&r,&c,||fs::write(d.join("state.bin"),vec![b' ';2048]).unwrap());
        delivered(&result);assert_eq!(result.status(),"NO_ACREDITADO");assert!(result.legacy_evidence().is_none());
        let j=result.documentary();assert_eq!(j.field("despues").field("etapa"),&json::s("quota"));
        assert_eq!(j.field("despues").field("bytes_leidos").uint(),Some(1025));save("P06",&result);
    }
    #[test]
    fn p07_changed_before_stops_operation(){
        let(d,r,c)=setup("P07");fs::write(d.join("state.bin"),raw("vector-alterado.bin")).unwrap();
        let result=conductor::run_with_injection(&d,&r,&c,||panic!("operation reached"));
        assert_eq!(result.status(),"ALTERACION_DETECTADA");assert_eq!(result.attempts(),0);assert!(result.receipt().is_none());save("P07",&result);
    }
    #[test]
    fn p08_read_error_after_is_explicit(){
        let(d,r,c)=setup("P08");let result=conductor::run_with_injection(&d,&r,&c,||{
            fs::remove_file(d.join("state.bin")).unwrap();fs::create_dir(d.join("state.bin")).unwrap();});
        delivered(&result);assert_eq!(result.status(),"NO_ACREDITADO");assert!(result.legacy_evidence().is_none());
        assert_eq!(result.documentary().field("despues").field("etapa"),&json::s("read"));save("P08",&result);
    }
}
