//! S26 R03: caracterización parcial de T01, T02 y T07. Sin cambios al admisor.
#[cfg(test)]
#[path = "../assembly/src/observer.rs"]
mod observer;
#[cfg(test)]
mod tests {
    use std::{fs,io::{self,Read,Cursor},path::PathBuf};
    use sv_bis_i0205::{admit,certify,json::{self,J},Capture,ReceivedBytes,TrustedContext,TrustedRegistry};
    fn root()->PathBuf {std::env::var_os("S26_FIXTURE").unwrap().into()}
    fn work()->PathBuf {std::env::var_os("S26_WORK").unwrap().into()}
    fn raw(v:&str,n:&str)->Vec<u8>{fs::read(root().join(v).join(n)).unwrap()}
    fn doc(v:&str,n:&str)->J{json::decode(&raw(v,n)).unwrap()}
    fn custody(v:&str)->(TrustedContext,TrustedRegistry){(
        TrustedContext::from_custody(&raw(v,"context.json")).unwrap(),
        TrustedRegistry::from_custody(&raw(v,"registry.json"),&raw(v,"constitution.bin"),
            &raw(v,"convention.bin"),&raw(v,"transforms.bin")).unwrap())}
    fn receive(v:&str)->ReceivedBytes{ReceivedBytes::read(&mut raw(v,"request.json").as_slice(),
        &mut raw(v,"source.bin").as_slice(),&mut raw(v,"state.bin").as_slice(),
        Some(&mut raw(v,"support.bin").as_slice()),&mut raw(v,"geometry.bin").as_slice()).unwrap()}
    fn capture(v:&str)->Capture{Capture{captor:"captor-local-i0205/1".into(),
        context:doc(v,"plan.json").field("contexto_captura").clone(),bytes:raw(v,"expected.bin")}}
    fn save(n:&str,j:&J){fs::write(work().join(n),json::encode(j,262144).unwrap()).unwrap()}

    #[test]
    fn t01_fixed_expectation_and_replaced_custody(){
        let (a,ar)=custody("A");let (b,br)=custody("B");
        for (v,c,r) in [("A",&a,&ar),("B",&b,&br)] {
            let x=admit(receive(v),c,r).unwrap();
            assert_eq!(x.descriptor(),raw(v,"expected.bin"));
            let receipt=certify(&x,Some(&capture(v)),false,r).unwrap();
            assert_eq!(&receipt,doc(v,"oracle.json").field("recibo_esperado"));
            save(&format!("T01-{v}-receipt.json"),&receipt);
        }
        let e=match admit(receive("B"),&a,&ar){Ok(_)=>panic!("B substituted under A"),Err(e)=>e};
        assert_eq!(e.guard,"I02");assert_eq!(e.cause,"full identity/custody mismatch");
        assert_eq!(e.passed,vec!["R01","P01","P02","S01","S02","I01"]);
        save("T01-rejection.json",&json::obj(vec![("guard",json::s(e.guard)),("cause",json::s(&e.cause))]));
        println!("T01: A/A and B/B full receipts match; B/A rejected I02; replacement of custody accepted locally, not authenticated externally");
    }

    struct ChangeAtEof {inner:Cursor<Vec<u8>>,path:PathBuf,inject:bool,fired:bool}
    impl Read for ChangeAtEof {
        fn read(&mut self,b:&mut [u8])->io::Result<usize>{
            let n=self.inner.read(b)?;
            if n==0 && !self.fired {
                self.fired=true;
                if self.inject {fs::write(&self.path,raw("B","source.bin"))?;}
            }
            Ok(n)
        }
    }
    struct CountRead {inner:fs::File,seen:Vec<u8>}
    impl Read for CountRead {
        fn read(&mut self,b:&mut [u8])->io::Result<usize>{let n=self.inner.read(b)?;self.seen.extend_from_slice(&b[..n]);Ok(n)}
    }
    #[test]
    fn t02_mixed_read_at_explicit_barrier(){
        let(c,r)=custody("A");
        for inject in [false,true] {
            let path=work().join(if inject{"T02-mixed-source.bin"}else{"T02-control-source.bin"});
            assert!(!path.exists());fs::write(&path,raw("A","source.bin")).unwrap();
            let mut meta=ChangeAtEof{inner:Cursor::new(raw("A","request.json")),path:path.clone(),inject,fired:false};
            let mut source=CountRead{inner:fs::File::open(&path).unwrap(),seen:Vec::new()};
            let request=ReceivedBytes::read(&mut meta,&mut source,&mut raw("A","state.bin").as_slice(),
                Some(&mut raw("A","support.bin").as_slice()),&mut raw("A","geometry.bin").as_slice()).unwrap();
            assert!(meta.fired);let v=if inject{"B"}else{"A"};assert_eq!(source.seen,raw(v,"source.bin"));
            assert_eq!(fs::read(&path).unwrap(),source.seen);
            let outcome=admit(request,&c,&r);
            if inject {
                let e=match outcome{Ok(_)=>panic!("mixed source admitted"),Err(e)=>e};
                assert_eq!(e.guard,"I02");assert_eq!(e.cause,"source identity");
                assert_eq!(e.passed,vec!["R01","P01","P02","S01","S02","I01"]);
            }else{assert_eq!(outcome.unwrap().descriptor(),raw("A","expected.bin"));}
            save(&format!("T02-{v}-trace.json"),&json::obj(vec![("metadata",json::s("A")),
                ("barrier",json::s("metadata EOF before source read")),("injected",J::Bool(inject)),
                ("source_read_bytes",json::n(source.seen.len())),("source_version",json::s(v))]));
        }
        println!("T02: actual source B read after metadata A EOF injection; I02 source identity; unchanged control admitted");
    }

    #[test]
    fn t07_copied_after_vs_observed_after(){
        let(c,r)=custody("A");let a=admit(receive("A"),&c,&r).unwrap();
        let cap=capture("A");let receipt=certify(&a,Some(&cap),false,&r).unwrap();
        let canonical=raw("A","canonical.bin");let geometry=raw("A","expected.bin");let oracle=doc("A","oracle.json");
        let path=work().join("T07-state.bin");assert!(!path.exists());fs::write(&path,&canonical).unwrap();
        let before=fs::read(&path).unwrap();
        let mut e=super::observer::Evidence{result:"ENTREGA_DOCUMENTAL_CONCORDANTE".into(),guard:None,cause:String::new(),
            passed:["R01","P01","P02","S01","S02","I01","I02","I03","A01","M01","M02","M03","D01","D02","D03","D04","D05","D06"].iter().map(|s|s.to_string()).collect(),
            receipt:Some(receipt),capture:Some(cap),dispatches:Some(1),attempts:1,before:before.clone(),
            after:fs::read(&path).unwrap(),admitted_vector:Some(J::Array(a.vector().iter().map(|t|json::s(t.ir_label())).collect())),preservation_claim:true};
        assert!(super::observer::inspect(&e,&oracle,&canonical,Some(&geometry)).is_empty());
        save("T07-control.json",&e.documentary());
        let copied_after=before.clone();let mut changed=canonical.clone();changed.push(b' ');
        fs::write(&path,&changed).unwrap();let observed_after=fs::read(&path).unwrap();
        assert_eq!(observed_after,changed);assert_ne!(observed_after,before);
        e.after=copied_after;
        assert!(super::observer::inspect(&e,&oracle,&canonical,Some(&geometry)).is_empty(),"documentary observer cannot infer unread file");
        save("T07-copied-after.json",&e.documentary());
        e.after=observed_after;
        assert_eq!(super::observer::inspect(&e,&oracle,&canonical,Some(&geometry)),vec!["preservacion real del estado"]);
        save("T07-reread-after.json",&e.documentary());
        println!("T07: observer accepts initial copies despite changed file; actual reread yields exactly preservacion real del estado; control accepted");
    }
}
