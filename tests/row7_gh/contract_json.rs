//! Proyección íntegra de la sonda, independiente del codificador binario del núcleo.
//! No añade un serializador productivo ni calcula la huella que el observador espera.
use sv_core::bindings::*;

fn text(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"),
            c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"'); out
}
fn object(fields: Vec<(&str,String)>) -> String {
    format!("{{{}}}", fields.into_iter().map(|(k,v)|format!("{}:{v}",text(k))).collect::<Vec<_>>().join(","))
}
fn array(items: Vec<String>) -> String { format!("[{}]",items.join(",")) }
fn reference(r: &ExactReference) -> String { object(vec![("identifier",text(&r.identifier)),("version",text(&r.version)),("sha256",text(&r.sha256))]) }
fn references(rs: &[ExactReference]) -> String { array(rs.iter().map(reference).collect()) }
fn rule(r: &RuleBinding) -> String { object(vec![("object",text(&r.object)),("definition",reference(&r.definition))]) }
fn program(p: &ProgramIdentity) -> String { object(vec![("source_file",text(&p.source_file)),("source_sha256",text(&p.source_sha256)),("projection_sha256",text(&p.projection_sha256))]) }
fn instance(i: &ParameterInstanceBinding) -> String {
    object(vec![("identifier",text(&i.identifier)),("owner",text(&i.owner)),("parameter",text(&i.parameter)),
        ("parameter_id",text(i.parameter_id.as_decimal())),("capture",rule(&i.capture)),("admission",rule(&i.admission)),
        ("ternarizer",i.ternarizer.as_ref().map(rule).unwrap_or_else(||"null".into())),("provenance",references(&i.provenance))])
}
fn usage(u: &BindingUse) -> String {
    object(vec![("identifier",text(&u.identifier)),("instance",text(&u.instance)),
        ("destination",u.destination.as_ref().map(|d|object(vec![("node",text(&d.node)),("position",text(d.position.as_decimal()))])).unwrap_or_else(||"null".into())),
        ("alias_of",u.alias_of.as_ref().map(|a|text(a)).unwrap_or_else(||"null".into()))])
}
fn sharing(s: &SharingDeclaration) -> String {
    object(vec![("instance",text(&s.instance)),("use_ids",array(s.use_ids.iter().map(|u|text(u)).collect())),("rule",reference(&s.rule))])
}
fn operation(o: &OperationBindings) -> String {
    object(vec![("identifier",text(&o.identifier)),("version",text(&o.version)),("definition",reference(&o.definition)),
        ("uses",array(o.uses.iter().map(usage).collect())),("requires_destination",o.requires_destination.to_string()),
        ("sharing",array(o.sharing.iter().map(sharing).collect())),("input_scope",text(match o.input_scope {
            BindingInputScope::BindingsOnly=>"BindingsOnly",BindingInputScope::BindingsWithSideInformation=>"BindingsWithSideInformation"})),
        ("side_information",references(&o.side_information))])
}
fn artifact(a: &BindingArtifact) -> String {
    let kind = match a.kind {
        ArtifactKind::Constitution=>"Constitution",ArtifactKind::AuthorityDeclaration=>"AuthorityDeclaration",
        ArtifactKind::CaptureDefinition=>"CaptureDefinition",ArtifactKind::AdmissionDefinition=>"AdmissionDefinition",
        ArtifactKind::TernarizerDefinition=>"TernarizerDefinition",ArtifactKind::Provenance=>"Provenance",
        ArtifactKind::SharingRule=>"SharingRule",ArtifactKind::OperationDefinition=>"OperationDefinition",ArtifactKind::SideInformation=>"SideInformation",
    };
    object(vec![("reference",reference(&a.reference)),("kind",text(kind)),("hex",text(&a.bytes.iter().map(|b|format!("{b:02x}")).collect::<String>()))])
}
pub fn json(c: &BindingContract) -> String {
    object(vec![("schema",text(&c.schema)),("identifier",text(&c.identifier)),("version",text(&c.version)),("program",program(&c.program)),
        ("domain",text(&c.domain)),("agent",text(&c.agent)),("constitution",reference(&c.constitution)),("authority",reference(&c.authority)),
        ("instances",array(c.instances.iter().map(instance).collect())),("operations",array(c.operations.iter().map(operation).collect())),
        ("artifacts",array(c.artifacts.iter().map(artifact).collect()))])
}
