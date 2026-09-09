//! Contraste de transporte del retorno CYB sobre el portador sintético anterior.
//! No constituye arquitectura CYB, no produce Tri ni ejecuta reglas de dominio.
#[allow(dead_code)]
#[path = "../row7_bindings/cases.rs"]
mod carrier;
#[path = "../row7_gh/documentary_json.rs"]
mod json;
#[path = "../row7_gh/contract_json.rs"]
mod contract_json;
use sv_core::bindings::*;
use json::Value;
fn sha256_hex(bytes: &[u8]) -> String {
    // Sólo banco nativo: GNU Coreutils, ya presente en el paquete de prueba.
    // No se incorpora una implementación criptográfica ni dependencia al núcleo.
    use std::{io::Write, process::{Command,Stdio}};
    let mut p=Command::new("sha256sum").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
    p.stdin.take().unwrap().write_all(bytes).unwrap();
    let result=p.wait_with_output().unwrap();assert!(result.status.success());
    let output=String::from_utf8(result.stdout).unwrap();
    let hash=output.strip_suffix("  -\n").unwrap();
    assert!(hash.len()==64 && hash.bytes().all(|b|b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
    hash.into()
}

struct Input { id: String, consumer: String, variant: &'static str, payload: String, side: Option<String> }
fn obj(fields: &[(&str, &Value)]) -> String {
    Value::compact_members(fields.iter().map(|(k,v)| (*k,*v)))
}
fn document(row: &Value, version: usize) -> String {
    let fields = if version == 3 { ["parametro","entrada"] } else { ["clase","datos"] };
    obj(&fields.map(|key| (key,row.get(key).unwrap())))
}
fn inputs() -> Vec<Input> {
    let pr=json::parse(include_bytes!("originales/v03/testigos.json")).unwrap();
    let ct=json::parse(include_bytes!("originales/v04/testigos.json")).unwrap();
    let si=json::parse(include_bytes!("originales/v03/pares_semanticos.json")).unwrap();
    let pc=json::parse(include_bytes!("originales/v04/pares.json")).unwrap();
    let mut out=Vec::new();
    for (version,cases) in [(3,&pr),(4,&ct)] {
        for row in cases.array().unwrap() {
            out.push(Input { id:row.get("id").unwrap().text().unwrap().into(), consumer:format!("{}:{}",if version==3 {"regulated"} else {"continuity"},row.get(if version==3 {"parametro"} else {"clase"}).unwrap().text().unwrap()), variant:"F0",payload:document(row,version),side:None });
        }
    }
    for (version,pairs) in [(3,&si),(4,&pc)] {
        for pair in pairs.array().unwrap() {
            for state in ["a","b"] {
                let full = if version == 3 { pair.get(state).unwrap().compact() } else {
                    let id=pair.get(state).unwrap().text().unwrap();
                    document(ct.array().unwrap().iter().find(|c|c.get("id").unwrap().text().unwrap()==id).unwrap(),4)
                };
                let reduced = if version == 3 {pair.get(state).unwrap().get("technical").unwrap().compact()}
                    else {pair.get("proyeccion_tecnica_comun").unwrap().compact()};
                let consumer = if version==3 {format!("field:{}",pair.get("diferencia").unwrap().text().unwrap())} else {
                    let doc=json::parse(full.as_bytes()).unwrap();format!("continuity:{}",doc.get("clase").unwrap().text().unwrap())
                };
                for variant in ["F0","H","HS"] {
                    out.push(Input { id:format!("{}-{state}",pair.get("id").unwrap().text().unwrap()),consumer:consumer.clone(),variant,
                        payload:if variant=="F0"{full.clone()}else{reduced.clone()},
                        side:if variant=="HS"{Some(full.clone())}else{None} });
                }
            }
        }
    }
    assert_eq!(out.len(),186);
    out
}
fn contract(program: &sv_core::IrProgram, input:&Input) -> BindingContract {
    let mut c=carrier::base(program);
    c.identifier="CYB-CONSUMO-0.1".into();
    c.instances.truncate(1); c.instances[0].ternarizer=None;
    c.operations[0].uses.truncate(1); c.operations[0].uses[0].destination=None;
    c.operations[0].requires_destination=false;
    c.artifacts.retain(|a| !["Tau","Compartir","Lateral"].contains(&a.reference.identifier.as_str()));
    let payload=c.artifacts.iter_mut().find(|a|a.reference.identifier=="FuenteP").unwrap();
    payload.bytes=input.payload.as_bytes().to_vec(); payload.reference.sha256=sha256_hex(&payload.bytes);
    c.instances[0].provenance=vec![payload.reference.clone()];
    if let Some(bytes)=&input.side {
        let a=BindingArtifact {reference:ExactReference {identifier:"Lateral".into(),version:"1".into(),sha256:sha256_hex(bytes.as_bytes())},
            kind:ArtifactKind::SideInformation,bytes:bytes.as_bytes().to_vec()};
        c.operations[0].input_scope=BindingInputScope::BindingsWithSideInformation;
        c.operations[0].side_information=vec![a.reference.clone()]; c.artifacts.push(a);
    }
    let manifest=json::parse(include_bytes!("consumidores.json")).unwrap();
    let consumer=manifest.get("consumers").unwrap().array().unwrap().iter()
        .find(|c|c.get("key").unwrap().text().unwrap()==input.consumer).unwrap();
    let definition=format!("{{\"schema\":\"CYB-CONSUMO/0.1\",\"profile\":\"CYB-DOCUMENTAL\",\"version\":\"1\",\"consumer\":{},\"variant\":\"{}\",\"rules\":{},\"implementation\":{},\"sources_sha256\":{}}}",
        consumer.compact(),input.variant,manifest.get("rules").unwrap().compact(),
        manifest.get("implementation").unwrap().compact(),manifest.get("sources_sha256").unwrap().compact());
    let operation=c.artifacts.iter_mut().find(|a|a.reference.identifier=="OP").unwrap();
    operation.bytes=definition.into_bytes();operation.reference.sha256=sha256_hex(&operation.bytes);
    c.operations[0].definition=operation.reference.clone();
    for (name,bytes) in [("ReglasDocumentales",include_bytes!("reglas_documentales.mjs").as_slice()),
                        ("ConsumidorDocumental",include_bytes!("consumir.mjs").as_slice())] {
        c.artifacts.push(BindingArtifact {reference:ExactReference {identifier:name.into(),version:"1".into(),sha256:sha256_hex(bytes)},
            kind:ArtifactKind::OperationDefinition,bytes:bytes.to_vec()});
    }
    c
}
fn request(c:&BindingContract)->BindingRequest {
    BindingRequest { contract:ExactReference {identifier:c.identifier.clone(),version:c.version.clone(),sha256:binding_contract_sha256(c)},
        operation:"OP".into(),operation_version:"1".into() }
}
fn get<'a>(v:&'a ValidatedBindings,r:&ExactReference)->&'a [u8] {
    &v.contract().artifacts.iter().find(|a|&a.reference==r).unwrap().bytes
}
fn checked(program:&sv_core::IrProgram,input:&Input)->ValidatedBindings {
    let c=contract(program,input); let q=request(&c);
    let v=validate_bindings(program,c.clone(),&q).unwrap();
    assert_eq!(v.contract(),&c);assert_eq!(v.program(),program);assert_eq!(v.expectation(),&q.contract);
    let i=v.instances_in_use_order().next().unwrap();
    assert_eq!(get(&v,&i.provenance[0]),input.payload.as_bytes());
    assert_eq!(i.ternarizer,None);assert_eq!(v.operation().uses[0].destination,None);
    if let Some(side)=&input.side {assert_eq!(get(&v,&v.operation().side_information[0]),side.as_bytes());}
    v
}
pub fn check_entry(entry:usize) {
    let program=carrier::program(entry).unwrap();
    let rows=inputs();
    for row in &rows {checked(&program,row);}
    for pair in rows[78..].chunks_exact(6) {
        let a=checked(&program,&pair[1]);let b=checked(&program,&pair[4]);
        assert_eq!(a.contract(),b.contract(),"H filtra la identidad del estado");
        assert_ne!(checked(&program,&pair[0]).contract(),checked(&program,&pair[3]).contract());
        assert_ne!(checked(&program,&pair[2]).contract(),checked(&program,&pair[5]).contract());
    }
}
pub fn check_rejections() {
    for entry in 0..4 {
        let p=carrier::program(entry).unwrap();let rows=inputs();
        for attack in 0..6 {
            let mut c=contract(&p,if attack==5 {&rows[80]} else {&rows[0]});
            let good=c.clone();
            let (kind,subject)=match attack {
                0=>{c.artifacts.iter_mut().find(|a|a.reference.identifier=="FuenteP").unwrap().bytes.push(0);(BindingErrorKind::ArtifactIntegrity,"FuenteP")},
                1=>{c.instances[0].provenance[0].version="2".into();(BindingErrorKind::ExactReference,"FuenteP")},
                2=>{c.instances[0].provenance[0].identifier="Ausente".into();(BindingErrorKind::MissingReference,"Ausente")},
                3=>{c.operations[0].input_scope=BindingInputScope::BindingsWithSideInformation;(BindingErrorKind::SideInformation,"OP")},
                4=>{c.operations[0].side_information=vec![c.instances[0].provenance[0].clone()];(BindingErrorKind::SideInformation,"OP")},
                5=>{c.operations[0].side_information.clear();(BindingErrorKind::SideInformation,"OP")},
                _=>unreachable!()
            };
            let actual=validate_bindings(&p,c.clone(),&request(&c)).unwrap_err();
            assert_eq!(actual,BindingError {kind,subject:subject.into()});
            validate_bindings(&p,good.clone(),&request(&good)).unwrap();
        }
    }
}
pub fn check_independent_instances() {
    let p=carrier::program(0).unwrap();let rows=inputs();let mut c=contract(&p,&rows[0]);
    let mut a=c.artifacts.iter().find(|a|a.reference.identifier=="FuenteP").unwrap().clone();
    a.reference.identifier="FuenteP2".into();a.bytes=rows[1].payload.as_bytes().to_vec();a.reference.sha256=sha256_hex(&a.bytes);
    let mut i=c.instances[0].clone();i.identifier="I2".into();i.provenance=vec![a.reference.clone()];
    c.instances.push(i);c.artifacts.push(a);
    let mut u=c.operations[0].uses[0].clone();u.identifier="U2".into();u.instance="I2".into();c.operations[0].uses.push(u);
    let v=validate_bindings(&p,c.clone(),&request(&c)).unwrap();
    let instances=v.instances_in_use_order().collect::<Vec<_>>();
    assert_eq!(instances.len(),2);assert_eq!(instances[0].parameter,instances[1].parameter);
    assert_eq!(get(&v,&instances[0].provenance[0]),rows[0].payload.as_bytes());
    assert_eq!(get(&v,&instances[1].provenance[0]),rows[1].payload.as_bytes());
    assert_ne!(instances[0].identifier,instances[1].identifier);
}
pub fn check_operation_artifacts() {
    let p=carrier::program(0).unwrap();let rows=inputs();
    for row in &rows {
        let v=checked(&p,row);
        let d=json::parse(get(&v,&v.operation().definition)).unwrap();
        assert_eq!(d.get("consumer").unwrap().get("key").unwrap().text().unwrap(),row.consumer);
        assert_eq!(d.get("variant").unwrap().text().unwrap(),row.variant);
        for (name,bytes) in [("ReglasDocumentales",include_bytes!("reglas_documentales.mjs").as_slice()),
                            ("ConsumidorDocumental",include_bytes!("consumir.mjs").as_slice())] {
            let a=v.contract().artifacts.iter().find(|a|a.reference.identifier==name).unwrap();
            assert_eq!(a.bytes,bytes);
        }
    }
    for name in ["OP","ReglasDocumentales","ConsumidorDocumental"] {
        let good=contract(&p,&rows[0]);let mut bad=good.clone();
        bad.artifacts.iter_mut().find(|a|a.reference.identifier==name).unwrap().bytes.push(0);
        assert_eq!(validate_bindings(&p,bad.clone(),&request(&bad)).unwrap_err(),
            BindingError {kind:BindingErrorKind::ArtifactIntegrity,subject:name.into()});
        validate_bindings(&p,good.clone(),&request(&good)).unwrap();
    }
}
fn hex(bytes:&[u8])->String {bytes.iter().map(|b|format!("{b:02x}")).collect()}
pub fn report()->String {
    let p=carrier::program(0).unwrap();let mut out=Vec::new();
    for row in inputs() {
        let v=checked(&p,&row);
        let i=v.instances_in_use_order().next().unwrap();
        let recovered=if v.operation().input_scope==BindingInputScope::BindingsWithSideInformation {
            get(&v,&v.operation().side_information[0])
        } else {get(&v,&i.provenance[0])};
        out.push(format!("{{\"id\":\"{}\",\"variant\":\"{}\",\"contract_sha256\":\"{}\",\"recovered_hex\":\"{}\",\"contract\":{}}}",
            row.id,row.variant,v.expectation().sha256,hex(recovered),contract_json::json(v.contract())));
    }
    format!("{{\"schema\":\"CYB-LIG-CONSUMO/0.1\",\"rows\":[{}]}}",out.join(","))
}
