//! Transporte documental de pruebas; no ejecuta consultas SV ni Q0.
#[allow(dead_code)]
#[path = "../row7_bindings/cases.rs"]
mod carrier;
use sv_core::bindings::*;

pub struct Input {
    witness: &'static str, state: &'static str, variant: &'static str,
    operation: &'static str, definition: &'static str, payload: &'static str,
    side: &'static str, definition_sha: &'static str, payload_sha: &'static str,
    side_sha: &'static str,
}
include!("entradas.rs");

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn artifact_json(artifact: &BindingArtifact) -> String {
    format!("{{\"id\":\"{}\",\"version\":\"{}\",\"sha256\":\"{}\",\"hex\":\"{}\"}}",
            artifact.reference.identifier, artifact.reference.version, artifact.reference.sha256, hex(&artifact.bytes))
}
fn get<'a>(v: &'a ValidatedBindings, reference: &ExactReference) -> &'a BindingArtifact {
    v.contract().artifacts.iter().find(|a| a.reference == *reference).unwrap()
}
fn validate(input: &Input) -> Result<ValidatedBindings, String> {
    let program = carrier::program(0)?;
    let mut contract = carrier::base(&program);
    // El identificador del estado sólo existe en el banco, nunca en el contrato H.
    contract.identifier = format!("{}-{}", input.witness, input.variant);
    contract.instances.truncate(1);
    contract.instances[0].ternarizer = None;
    contract.operations[0].uses.truncate(1);
    contract.operations[0].uses[0].destination = None;
    contract.operations[0].requires_destination = false;
    contract.artifacts.retain(|a| !["Tau", "Compartir", "Lateral"].contains(&a.reference.identifier.as_str()));
    let source = contract.artifacts.iter_mut().find(|a| a.reference.identifier == "FuenteP").unwrap();
    source.bytes = input.payload.as_bytes().to_vec();
    source.reference.sha256 = input.payload_sha.into();
    contract.instances[0].provenance = vec![source.reference.clone()];
    let definition = contract.artifacts.iter_mut().find(|a| a.reference.identifier == "OP").unwrap();
    definition.reference.identifier = input.operation.into();
    definition.reference.sha256 = input.definition_sha.into();
    definition.bytes = input.definition.as_bytes().to_vec();
    contract.operations[0].identifier = input.operation.into();
    contract.operations[0].definition = definition.reference.clone();
    if input.variant == "HS" {
        let side = BindingArtifact { reference: ExactReference { identifier:"S".into(), version:"1".into(), sha256:input.side_sha.into() },
                                     kind:ArtifactKind::SideInformation, bytes:input.side.as_bytes().to_vec() };
        contract.operations[0].side_information = vec![side.reference.clone()];
        contract.operations[0].input_scope = BindingInputScope::BindingsWithSideInformation;
        contract.artifacts.push(side);
    }
    let request = BindingRequest { contract:ExactReference { identifier:contract.identifier.clone(),version:contract.version.clone(),sha256:binding_contract_sha256(&contract) },
                                   operation:input.operation.into(),operation_version:"1".into() };
    validate_bindings(&program, contract, &request).map_err(|e| format!("{} {} {}: {e:?}", input.witness,input.state,input.variant))
}
pub fn check_witness(witness: &str) -> Result<(), String> {
    let rows = INPUTS.iter().filter(|i| i.witness == witness).collect::<Vec<_>>();
    if rows.len() != 6 { return Err("Inventario de estados/representaciones incorrecto".into()); }
    let mut reduced = Vec::new();
    for input in rows {
        let value = validate(input)?;
        if input.variant == "H" { reduced.push(value.expectation().sha256.clone()); }
        let instance = value.instances_in_use_order().next().unwrap();
        if get(&value,&instance.provenance[0]).bytes != input.payload.as_bytes() {
            return Err("Bytes de procedencia alterados tras validar".into());
        }
        if value.operation().uses[0].destination.is_some() || instance.ternarizer.is_some() {
            return Err("Destino o transducción introducidos en el portador".into());
        }
        if input.variant == "HS" && get(&value,&value.operation().side_information[0]).bytes != input.side.as_bytes() {
            return Err("Bytes laterales alterados tras validar".into());
        }
    }
    if reduced.len() != 2 || reduced[0] != reduced[1] { return Err("H filtra la identidad del estado perdido".into()); }
    Ok(())
}
pub fn report() -> Result<String, String> {
    let mut rows = Vec::new();
    for input in INPUTS {
        let value = validate(input)?;
        let instance = value.instances_in_use_order().next().unwrap();
        let source = get(&value,&instance.provenance[0]);
        let definition = get(&value,&value.operation().definition);
        let side = value.operation().side_information.iter().map(|r| artifact_json(get(&value,r))).collect::<Vec<_>>().join(",");
        let inventory = value.contract().artifacts.iter().map(|a|format!("\"{}\"",a.reference.identifier)).collect::<Vec<_>>().join(",");
        rows.push(format!("{{\"witness\":\"{}\",\"state\":\"{}\",\"variant\":\"{}\",\"operation\":\"{}\",\"contract_sha256\":\"{}\",\"scope\":\"{:?}\",\"source\":{},\"definition\":{},\"side\":[{}],\"artifact_ids\":[{}]}}",
            input.witness,input.state,input.variant,value.operation().identifier,value.expectation().sha256,value.operation().input_scope,
            artifact_json(source),artifact_json(definition),side,inventory));
    }
    Ok(format!("{{\"schema\":\"GH-LIG-OBSERVACIONES/0.1\",\"results\":[{}]}}", rows.join(",")))
}
