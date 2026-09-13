//! ES: Sondas de residencia y captura del montaje Bis existente, no de persistencia.
//! EN: Residence and capture probes for the existing Bis assembly, not persistence.
#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};
    use sv_bis_i0205::{admit, certify, json::{self, J}, AdmittedDelivery,
        Capture, ReceivedBytes, TrustedContext, TrustedRegistry};

    fn fixture() -> PathBuf { PathBuf::from(std::env::var_os("S26_FIXTURE").expect("fixture path")) }
    fn raw(name: &str) -> Vec<u8> { fs::read(fixture().join(name)).expect("fixture read") }
    fn doc(name: &str) -> J { json::decode(&raw(name)).expect("fixture JSON") }
    fn custody() -> (TrustedContext, TrustedRegistry) {
        (TrustedContext::from_custody(&raw("context.json")).expect("context"),
         TrustedRegistry::from_custody(&raw("registry.json"), &raw("constitution.bin"),
             &raw("convention.bin"), &raw("transforms.bin")).expect("registry"))
    }
    fn receive(source: &mut impl std::io::Read) -> ReceivedBytes {
        ReceivedBytes::read(&mut raw("request.json").as_slice(), source,
            &mut raw("state.bin").as_slice(), Some(&mut raw("support.bin").as_slice()),
            &mut raw("geometry.bin").as_slice()).expect("bounded reception")
    }
    fn check_witness(a: &AdmittedDelivery) {
        assert_eq!(a.descriptor(), raw("expected.bin"));
        let labels = J::Array(a.vector().iter().map(|x| json::s(x.ir_label())).collect());
        assert_eq!(&labels, doc("canonical.bin").field("vector"));
    }
    fn capture() -> Capture {
        Capture { captor: "captor-local-i0205/1".into(),
            context: doc("plan.json").field("contexto_captura").clone(),
            bytes: raw("expected.bin") }
    }

    #[test]
    fn s26_r01_p01_original_delivery() {
        let (c,r)=custody(); let a=admit(receive(&mut raw("source.bin").as_slice()),&c,&r).expect("admitted");
        check_witness(&a);
        let receipt=certify(&a,Some(&capture()),false,&r).expect("certified");
        assert_eq!(&receipt, doc("oracle.json").field("recibo_esperado"));
        println!("S26-R01-P01: original vector, descriptor and full receipt match fixed witnesses");
    }

    #[test]
    fn s26_r01_p02_file_changed_after_reception() {
        let work=PathBuf::from(std::env::var_os("S26_WORK").expect("work path"));
        let path=work.join("source-replaced.bin");
        assert!(!path.exists(),"fresh test directory required");
        let old=raw("source.bin"); fs::write(&path,&old).expect("initial file");
        let request=receive(&mut fs::File::open(&path).expect("first open"));
        let mut changed=old.clone(); changed.push(b'\n');
        fs::write(&path,&changed).expect("replace source after reception");
        assert_eq!(fs::read(&path).expect("actual reread"),changed);
        let ctx=doc("context.json");
        let profile=sv_core::SourceProfile::from_tag(ctx.field("perfil_fuente").text().unwrap()).unwrap();
        let name=ctx.field("vinculo").field("programa_fuente").field("source_file").text().unwrap();
        let old_ir=sv_core::compile_svp_profile(std::str::from_utf8(&old).unwrap(),name,profile).unwrap();
        let new_ir=sv_core::compile_svp_profile(std::str::from_utf8(&changed).unwrap(),name,profile).unwrap();
        assert_eq!(old_ir.objects(),new_ir.objects());
        assert_eq!(old_ir.operations(),new_ir.operations());
        assert_ne!(old_ir.source_sha256(),new_ir.source_sha256());
        let (c,r)=custody(); let a=admit(request,&c,&r).expect("owned earlier buffer admitted");
        check_witness(&a);
        let fresh=receive(&mut fs::File::open(&path).expect("second open"));
        let rejection=match admit(fresh,&c,&r) { Ok(_)=>panic!("altered source accepted"),Err(e)=>e };
        assert_eq!(rejection.guard,"I02");
        check_witness(&a);
        assert!(certify(&a,Some(&capture()),false,&r).is_ok());
        println!("S26-R01-P02: file now differs; prior buffer preserved; fresh read rejected I02; source hashes {} -> {}",old_ir.source_sha256(),new_ir.source_sha256());
    }

    #[test]
    fn s26_r01_p03_mutated_capture() {
        let (c,r)=custody(); let a=admit(receive(&mut raw("source.bin").as_slice()),&c,&r).unwrap();
        let mut altered=capture(); altered.bytes.push(b' ');
        let e=certify(&a,Some(&altered),false,&r).expect_err("altered capture");
        assert_eq!(e.guard,"D06"); check_witness(&a);
        assert!(certify(&a,Some(&capture()),false,&r).is_ok());
        println!("S26-R01-P03: D06 detected altered Capture; admitted descriptor preserved; no claim of preventing prior delivery");
    }

    #[test]
    fn s26_r01_p04_missing_capture_and_post_dispatch_flag() {
        let (c,r)=custody(); let a=admit(receive(&mut raw("source.bin").as_slice()),&c,&r).unwrap();
        assert_eq!(certify(&a,None,false,&r).expect_err("missing capture").guard,"D01");
        assert_eq!(certify(&a,None,true,&r).expect_err("post dispatch flag").guard,"D07");
        check_witness(&a);
        println!("S26-R01-P04: D01 and D07; admitted data preserved; no successful receipt from either call; physical effect not tested");
    }
}
