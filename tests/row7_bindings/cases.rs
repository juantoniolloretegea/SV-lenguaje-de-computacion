use sv_core::bindings::*;
use sv_core::{compile_svp, compile_svp_profile, compile_svp_assembly, SourceProfile, SourceUnit, IrProgram, Nat};

fn n(value: u64) -> Nat { Nat::from_u64(value) }
fn artifact(id: &str, kind: ArtifactKind, bytes: &str, sha256: &str) -> BindingArtifact {
    BindingArtifact { reference: ExactReference { identifier: id.into(), version: "1".into(), sha256: sha256.into() }, kind, bytes: bytes.as_bytes().to_vec() }
}
fn reference(c: &BindingContract, id: &str) -> ExactReference {
    c.artifacts.iter().find(|a| a.reference.identifier == id).unwrap().reference.clone()
}
pub fn program(entry: usize) -> Result<IrProgram, String> {
    let en = include_str!("base.svp"); let es = include_str!("base.es.svp");
    let (a,b) = en.split_at(en.find("agent AG").unwrap());
    match entry {
        0 => compile_svp(en, "ligaduras.svp"),
        1 => compile_svp_profile(es, "ligaduras.es.svp", SourceProfile::Es),
        2 => compile_svp_assembly(&[SourceUnit::new(a,"constitucion.svp",SourceProfile::En),SourceUnit::new(b,"agente.svp",SourceProfile::En)]),
        3 => compile_svp_assembly(&[SourceUnit::new(b,"agente.svp",SourceProfile::En),SourceUnit::new(a,"constitucion.svp",SourceProfile::En)]),
        _ => unreachable!(),
    }.map_err(|e| format!("base del testigo inválida: {e:?}"))
}
pub fn base(program: &IrProgram) -> BindingContract {
    let artifacts = vec![
        artifact("ConstitucionD", ArtifactKind::Constitution, "testigo declarativo ConstitucionD; versión 1", "638faf3dbe456f21617e74ed47958760fc859618acaae15b1e05b64aebb05655"),
        artifact("AutorD", ArtifactKind::AuthorityDeclaration, "testigo declarativo AutorD; versión 1", "36375a3111fef5161645ac4b1e4ec10518f5901a1b4fdf265e84fc2b323ba78a"),
        artifact("Phi", ArtifactKind::CaptureDefinition, "testigo declarativo Phi; versión 1", "3dbd76838d7adbfc6ac3956bbf9198cfbd231c8753402d68289b521b488bdea2"),
        artifact("Regla", ArtifactKind::AdmissionDefinition, "testigo declarativo Regla; versión 1", "6cfa1cfae98953b6a764d66cab0454ae311dc28e3ab738318d399062801fc32e"),
        artifact("Tau", ArtifactKind::TernarizerDefinition, "testigo declarativo Tau; versión 1", "a133a1dd0956bed7ef931d0cd0825bd35c4a8dd2bc08e8cbc8db33cf28ddc0ec"),
        artifact("FuenteP", ArtifactKind::Provenance, "testigo declarativo FuenteP; versión 1", "3f51c53b0defd152aec0772af31c8d2a0f9a3479cb1304e8b74ccddba97b4cb9"),
        artifact("Compartir", ArtifactKind::SharingRule, "testigo declarativo Compartir; versión 1", "49815cc59fd502f234f033c49be3676cab78b1ae54b35815ef7d61354aaa8390"),
        artifact("OP", ArtifactKind::OperationDefinition, "testigo declarativo OP; versión 1", "e8e7648c6db782473bce6aa262e1a87f5f3162304f808dba45ec0d234e80a093"),
        artifact("Lateral", ArtifactKind::SideInformation, "testigo declarativo Lateral; versión 1", "2e0bf82239f13d035cf51a804267ac652683cbf54746dfb72113ae33103d459d"),
    ];
    let get = |id: &str| artifacts.iter().find(|a| a.reference.identifier == id).unwrap().reference.clone();
    let instance = |id: &str| ParameterInstanceBinding { identifier:id.into(), owner:"D".into(), parameter:"P".into(), parameter_id:n(1),
        capture:RuleBinding { object:"Cap".into(), definition:get("Phi") }, admission:RuleBinding {object:"Adm".into(),definition:get("Regla")},
        ternarizer:Some(RuleBinding {object:"Ter".into(),definition:get("Tau")}), provenance:vec![get("FuenteP")] };
    let usage = |id:&str, instance:&str, node:&str| BindingUse { identifier:id.into(), instance:instance.into(),
        destination:Some(BindingDestination {node:node.into(),position:n(3)}),alias_of:None };
    BindingContract { schema:BINDING_SCHEMA.into(),identifier:"ContratoD".into(),version:"1".into(),program:ProgramIdentity::of(program),domain:"D".into(),agent:"AG".into(),
        constitution:get("ConstitucionD"),authority:get("AutorD"),instances:vec![instance("I1"),instance("I2")],
        operations:vec![OperationBindings {identifier:"OP".into(),version:"1".into(),definition:get("OP"),
            uses:vec![usage("U1","I1","CC1"),usage("U2","I2","CCCompartido")],requires_destination:true,sharing:vec![],
            input_scope:BindingInputScope::BindingsOnly,side_information:vec![]}],artifacts }
}
fn request(c: &BindingContract) -> BindingRequest {
    BindingRequest {contract:ExactReference {identifier:c.identifier.clone(),version:c.version.clone(),sha256:binding_contract_sha256(c)},operation:"OP".into(),operation_version:"1".into()}
}
fn shared(c: &mut BindingContract) {
    let rule=reference(c,"Compartir"); c.operations[0].uses[1].instance="I1".into();
    c.operations[0].sharing=vec![SharingDeclaration {instance:"I1".into(),use_ids:vec!["U1".into(),"U2".into()],rule}];
}
fn alias(c: &mut BindingContract) {
    shared(c); c.operations[0].uses[1].destination=c.operations[0].uses[0].destination.clone(); c.operations[0].uses[1].alias_of=Some("U1".into());
}
pub const NEGATIVE_COUNT: usize = 44;
pub const CASE_COUNT: usize = NEGATIVE_COUNT + 9;
pub fn check_case(index: usize) -> Result<String,String> {
    let mut rows=Vec::new();
    for entry in 0..4 {
        let program=program(entry)?; let mut c=base(&program);
        if index >= NEGATIVE_COUNT { rows.push(positive(index-NEGATIVE_COUNT, entry, &program, c)?); continue; }
        match index {
            32 => { shared(&mut c); }
            33 => { alias(&mut c); }
            34 => { alias(&mut c); }
            35 => { alias(&mut c); }
            36 => { shared(&mut c); }
            37 => { shared(&mut c); }
            39 => { c.operations[0].input_scope=BindingInputScope::BindingsWithSideInformation; c.operations[0].side_information=vec![reference(&c,"Lateral")]; }
            _ => {}
        }
        let control=c.clone(); let mut q=request(&c);
        let (id, expected, subject, recalculate) = match index {
            0 => { c.version="2".into(); ("L01_version",BindingErrorKind::ContractIdentity,"ContratoD",false) },
            1 => { c.operations[0].uses.swap(0,1); ("L02_orden_expectativa",BindingErrorKind::ContractIdentity,"ContratoD",false) },
            2 => { c.program.source_file="otro.svp".into(); ("L03_programa",BindingErrorKind::ProgramIdentity,"otro.svp",true) },
            3 => { c.artifacts[0].bytes.push(0); ("L04_bytes",BindingErrorKind::ArtifactIntegrity,"ConstitucionD",true) },
            4 => { let mut a=c.artifacts[0].clone(); a.reference.version="2".into(); c.artifacts.push(a); ("L05_version_duplicada",BindingErrorKind::DuplicateIdentity,"ConstitucionD",true) },
            5 => { c.instances[0].capture.definition.version="2".into(); ("L06_referencia_exacta",BindingErrorKind::ExactReference,"Phi",true) },
            6 => { c.instances[0].capture.definition.identifier="Ausente".into(); ("L06_referencia_ausente",BindingErrorKind::MissingReference,"Ausente",true) },
            7 => { c.artifacts[2].kind=ArtifactKind::Provenance; ("L07_clase",BindingErrorKind::ArtifactKind,"Phi",true) },
            8 => { c.instances[1].identifier="I1".into(); ("L08_instancia_duplicada",BindingErrorKind::DuplicateIdentity,"I1",true) },
            9 => { c.instances[0].owner="D2".into(); ("L09_propietario",BindingErrorKind::Owner,"I1",true) },
            10 => { c.instances[0].parameter="Ajeno".into(); ("L09_parametro",BindingErrorKind::DomainMembership,"Ajeno",true) },
            11 => { c.instances[0].capture.object="C1".into(); ("L10_captura_tipo",BindingErrorKind::ObjectType,"C1",true) },
            12 => { c.instances[0].capture.object="CapFuera".into(); ("L10_captura_ajena",BindingErrorKind::DomainMembership,"CapFuera",true) },
            13 => { c.instances[0].admission.object="C1".into(); ("L10_admision_tipo",BindingErrorKind::ObjectType,"C1",true) },
            14 => { c.instances[0].admission.object="AdmFuera".into(); ("L10_admision_ajena",BindingErrorKind::DomainMembership,"AdmFuera",true) },
            15 => { c.instances[0].capture.object="Cap2".into(); ("L11_numeral_captura",BindingErrorKind::ParameterIdentity,"Cap2",true) },
            16 => { c.instances[0].admission.object="Adm2".into(); ("L11_numeral_admision",BindingErrorKind::ParameterIdentity,"Adm2",true) },
            17 => { let mut a=c.artifacts[2].clone(); a.reference.identifier="OtraPhi".into(); c.instances[0].capture.definition=a.reference.clone(); c.artifacts.push(a); ("L12_regla",BindingErrorKind::RuleIdentity,"Cap",true) },
            18 => { c.instances[0].ternarizer.as_mut().unwrap().object="TerFuera".into(); ("L13_ternarizador_ajeno",BindingErrorKind::DomainMembership,"TerFuera",true) },
            19 => { c.instances[0].ternarizer.as_mut().unwrap().object="Ter2".into(); ("L13_espacio",BindingErrorKind::ObservationSpace,"Ter2",true) },
            20 => { c.instances[0].provenance.clear(); ("L14_procedencia",BindingErrorKind::ProvenanceMissing,"I1",true) },
            21 => { q.operation="Otra".into(); ("L15_operacion",BindingErrorKind::OperationIdentity,"Otra",true) },
            22 => { q.operation_version="2".into(); ("L15_version_operacion",BindingErrorKind::OperationIdentity,"OP",true) },
            23 => { c.operations[0].uses[0].instance="I0".into(); ("L16_instancia_ausente",BindingErrorKind::InstanceMissing,"I0",true) },
            24 => { c.operations[0].uses[0].destination=None; ("L17_destino_ausente",BindingErrorKind::DestinationMissing,"U1",true) },
            25 => { c.operations[0].uses[0].destination.as_mut().unwrap().node="C1".into(); ("L18_destino_tipo",BindingErrorKind::ObjectType,"C1",true) },
            26 => { c.operations[0].uses[0].destination.as_mut().unwrap().node="C0".into(); ("L18_destino_ausente",BindingErrorKind::MissingReference,"C0",true) },
            27 => { c.operations[0].uses[0].destination.as_mut().unwrap().node="CCFuera".into(); ("L18_nodo_ajeno",BindingErrorKind::NodeMembership,"CCFuera",true) },
            28 => { c.operations[0].uses[0].destination.as_mut().unwrap().position=n(0); ("L19_cero",BindingErrorKind::PositionRange,"U1",true) },
            29 => { c.operations[0].uses[0].destination.as_mut().unwrap().position=n(10); ("L19_diez",BindingErrorKind::PositionRange,"U1",true) },
            30 => { c.operations[0].uses[0].destination.as_mut().unwrap().position=Nat::from_decimal("184467440737095516160").unwrap(); ("L19_nat_grande",BindingErrorKind::PositionRange,"U1",true) },
            31 => { c.operations[0].uses[1].destination=c.operations[0].uses[0].destination.clone(); ("L20_colision",BindingErrorKind::DestinationCollision,"U2",true) },
            32 => { c.operations[0].sharing.clear(); ("L21_comparticion_ausente",BindingErrorKind::Sharing,"I1",true) },
            33 => { c.operations[0].uses.swap(0,1); ("L22_alias_posterior",BindingErrorKind::Alias,"U2",true) },
            34 => { c.operations[0].uses[1].instance="I2".into(); ("L22_alias_instancia",BindingErrorKind::Alias,"U2",true) },
            35 => { c.operations[0].uses[1].destination.as_mut().unwrap().position=n(4); ("L22_alias_destino",BindingErrorKind::Alias,"U2",true) },
            36 => { c.operations[0].sharing[0].use_ids.pop(); ("L23_comparticion_incompleta",BindingErrorKind::Sharing,"I1",true) },
            37 => { c.operations[0].sharing[0].use_ids.swap(0,1); ("L23_comparticion_orden",BindingErrorKind::Sharing,"I1",true) },
            38 => { c.operations[0].side_information=vec![reference(&c,"Lateral")]; ("L24_lateral_oculta",BindingErrorKind::SideInformation,"OP",true) },
            39 => { c.operations[0].side_information.clear(); ("L24_lateral_ausente",BindingErrorKind::SideInformation,"OP",true) },
            40 => { c.schema="LIG/otro".into(); ("L00_esquema",BindingErrorKind::Schema,"LIG/otro",true) },
            41 => { c.identifier.clear(); ("L00_identidad_vacia",BindingErrorKind::InvalidIdentity,"",true) },
            42 => { c.operations[0].uses[1].identifier="U1".into(); ("L00_uso_duplicado",BindingErrorKind::DuplicateIdentity,"U1",true) },
            43 => { c.operations.push(c.operations[0].clone()); ("L00_operacion_duplicada",BindingErrorKind::DuplicateIdentity,"OP",true) },
            _ => unreachable!(),
        };
        if recalculate { q.contract=request(&c).contract; }
        let actual=validate_bindings(&program,c,&q).err();
        let expected_error=BindingError {kind:expected,subject:subject.into()};
        if actual.as_ref()!=Some(&expected_error) { return Err(format!("{id}/{entry}: esperado {expected_error:?}, obtenido {actual:?}")); }
        validate_bindings(&program,control.clone(),&request(&control)).map_err(|e|format!("{id}/{entry}: control reparado rechazado {e:?}"))?;
        rows.push(format!("{{\"id\":\"{id}\",\"entry\":{entry},\"kind\":\"{expected:?}\",\"subject\":\"{subject}\",\"repaired_control\":true}}"));
    }
    Ok(rows.join(","))
}
fn positive(index:usize, entry:usize, program:&IrProgram, mut c:BindingContract)->Result<String,String> {
    match index {
        0 => {},
        1 => { c.operations[0].uses[0].destination.as_mut().unwrap().position=n(1); c.operations[0].uses[1].destination.as_mut().unwrap().position=n(9); },
        2 => shared(&mut c),
        3 => alias(&mut c),
        4 => { c.operations[0].uses.swap(0,1); },
        5 => { c.operations[0].requires_destination=false; c.operations[0].uses[0].destination=None; },
        6 => { c.operations[0].input_scope=BindingInputScope::BindingsWithSideInformation; c.operations[0].side_information=vec![reference(&c,"Lateral")]; },
        7 => { c.instances[0].ternarizer=None; },
        8 => {
            let mut fixed=c.clone(); alias(&mut fixed);
            fixed.program=ProgramIdentity {source_file:"identidad-fija.svp".into(),source_sha256:"a".repeat(64),projection_sha256:"b".repeat(64)};
            fixed.operations[0].input_scope=BindingInputScope::BindingsWithSideInformation;
            fixed.operations[0].side_information=vec![reference(&fixed,"Lateral")];
            if binding_contract_sha256(&fixed) != include_str!("synthetic_hash_contract.sha256").trim() {
                return Err("codificación distinta del testigo independiente".into());
            }
        },
        _=>unreachable!(),
    }
    let q=request(&c); let expected_order=c.operations[0].uses.iter().map(|u|u.instance.clone()).collect::<Vec<_>>();
    let validated=validate_bindings(program,c.clone(),&q).map_err(|e|format!("positivo {index}/{entry}: {e:?}"))?;
    if validated.operation()!=&c.operations[0] || validated.contract()!=&c || validated.program()!=program || validated.expectation()!=&q.contract {
        return Err(format!("positivo {index}/{entry}: pérdida de representación"));
    }
    if validated.instances_in_use_order().map(|i|i.identifier.clone()).collect::<Vec<_>>()!=expected_order {
        return Err(format!("positivo {index}/{entry}: orden o multiplicidad alterados"));
    }
    if index==4 && binding_contract_sha256(&c)==binding_contract_sha256(&base(program)) { return Err("permutación sin huella distinta".into()); }
    // Una copia mutable del dato de entrada no puede alterar el resultado ya validado.
    c.operations[0].uses.clear();
    if validated.operation().uses.len()!=2 { return Err("el resultado comparte mutación con su entrada".into()); }
    Ok(format!("{{\"id\":\"LP{index:02}\",\"entry\":{entry},\"kind\":\"ACCEPTED\",\"preserved\":true}}"))
}
pub fn report()->Result<String,String> {
    let rows=(0..CASE_COUNT).map(check_case).collect::<Result<Vec<_>,_>>()?;
    Ok(format!("{{\"schema\":\"LIG-TEST/0.1\",\"cases\":{CASE_COUNT},\"entries_per_case\":4,\"results\":[{}]}}",rows.join(",")))
}
