//! LIG/0.1: ligaduras declaradas por operación, comprobadas por el núcleo.
//! Las huellas fijan bytes y referentes. No conceden autoridad ni ejecutan reglas.

use std::collections::{BTreeMap, BTreeSet};
use crate::{equivalence_json, frontend::sha256_hex, IrObjectKind, IrProgram, Nat};

pub const BINDING_SCHEMA: &str = "LIG/0.1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactReference {
    pub identifier: String,
    pub version: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactKind {
    Constitution, AuthorityDeclaration, CaptureDefinition, AdmissionDefinition,
    TernarizerDefinition, Provenance, SharingRule, OperationDefinition, SideInformation,
}

impl ArtifactKind {
    fn label(self) -> &'static str {
        match self {
            Self::Constitution => "Constitution", Self::AuthorityDeclaration => "AuthorityDeclaration",
            Self::CaptureDefinition => "CaptureDefinition", Self::AdmissionDefinition => "AdmissionDefinition",
            Self::TernarizerDefinition => "TernarizerDefinition", Self::Provenance => "Provenance",
            Self::SharingRule => "SharingRule", Self::OperationDefinition => "OperationDefinition",
            Self::SideInformation => "SideInformation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingArtifact {
    pub reference: ExactReference,
    pub kind: ArtifactKind,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramIdentity {
    pub source_file: String,
    pub source_sha256: String,
    pub projection_sha256: String,
}

impl ProgramIdentity {
    pub fn of(program: &IrProgram) -> Self {
        Self { source_file: program.source_file().to_owned(),
            source_sha256: program.source_sha256().to_owned(),
            projection_sha256: sha256_hex(equivalence_json(program).as_bytes()) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleBinding {
    pub object: String,
    pub definition: ExactReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterInstanceBinding {
    pub identifier: String,
    pub owner: String,
    pub parameter: String,
    pub parameter_id: Nat,
    pub capture: RuleBinding,
    pub admission: RuleBinding,
    pub ternarizer: Option<RuleBinding>,
    pub provenance: Vec<ExactReference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingDestination {
    pub node: String,
    pub position: Nat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingUse {
    pub identifier: String,
    pub instance: String,
    pub destination: Option<BindingDestination>,
    pub alias_of: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharingDeclaration {
    pub instance: String,
    pub use_ids: Vec<String>,
    pub rule: ExactReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingInputScope { BindingsOnly, BindingsWithSideInformation }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationBindings {
    pub identifier: String,
    pub version: String,
    pub definition: ExactReference,
    pub uses: Vec<BindingUse>,
    pub requires_destination: bool,
    pub sharing: Vec<SharingDeclaration>,
    pub input_scope: BindingInputScope,
    pub side_information: Vec<ExactReference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingContract {
    pub schema: String,
    pub identifier: String,
    pub version: String,
    pub program: ProgramIdentity,
    pub domain: String,
    pub agent: String,
    pub constitution: ExactReference,
    pub authority: ExactReference,
    pub instances: Vec<ParameterInstanceBinding>,
    pub operations: Vec<OperationBindings>,
    pub artifacts: Vec<BindingArtifact>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingRequest {
    pub contract: ExactReference,
    pub operation: String,
    pub operation_version: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingErrorKind {
    Schema, InvalidIdentity, ContractIdentity, ProgramIdentity, DuplicateIdentity,
    ArtifactIntegrity, MissingReference, ExactReference, ArtifactKind, ObjectType,
    DomainMembership, Owner, ParameterIdentity, RuleIdentity, ObservationSpace,
    ProvenanceMissing, OperationIdentity, InstanceMissing, DestinationMissing,
    NodeMembership, PositionRange, DestinationCollision, Alias, Sharing, SideInformation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingError { pub kind: BindingErrorKind, pub subject: String }

fn error(kind: BindingErrorKind, subject: &str) -> BindingError {
    BindingError { kind, subject: subject.to_owned() }
}

/// Resultado con construcción privada y acceso de sólo lectura. Conserva el
/// contrato completo y el programa exacto; no permite cambiar una ligadura tras comprobarla.
#[derive(Debug)]
pub struct ValidatedBindings {
    contract: BindingContract,
    operation_index: usize,
    expectation: ExactReference,
    program: IrProgram,
}

impl ValidatedBindings {
    pub fn contract(&self) -> &BindingContract { &self.contract }
    pub fn operation(&self) -> &OperationBindings { &self.contract.operations[self.operation_index] }
    pub fn expectation(&self) -> &ExactReference { &self.expectation }
    pub fn program(&self) -> &IrProgram { &self.program }
    /// Recupera las instancias en el orden de uso, sin deduplicación ni resolución implícita de alias.
    pub fn instances_in_use_order(&self) -> impl Iterator<Item = &ParameterInstanceBinding> {
        self.operation().uses.iter().map(|usage| {
            self.contract.instances.iter().find(|instance| instance.identifier == usage.instance)
                .expect("la construcción privada comprobó cada referencia de instancia")
        })
    }
}

fn identity(value: &str) -> bool { !value.trim().is_empty() && !value.chars().any(char::is_control) }
fn digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn check_identity(value: &str) -> Result<(), BindingError> {
    if !identity(value) { return Err(error(BindingErrorKind::InvalidIdentity, value)); }
    Ok(())
}

struct Registry<'a> { artifacts: BTreeMap<&'a str, &'a BindingArtifact> }
impl<'a> Registry<'a> {
    fn new(artifacts: &'a [BindingArtifact]) -> Result<Self, BindingError> {
        let mut map = BTreeMap::new();
        for artifact in artifacts {
            let reference = &artifact.reference;
            check_identity(&reference.identifier)?; check_identity(&reference.version)?;
            if map.insert(reference.identifier.as_str(), artifact).is_some() {
                return Err(error(BindingErrorKind::DuplicateIdentity, &reference.identifier));
            }
            if !digest(&reference.sha256) || sha256_hex(&artifact.bytes) != reference.sha256 {
                return Err(error(BindingErrorKind::ArtifactIntegrity, &reference.identifier));
            }
        }
        Ok(Self { artifacts: map })
    }
    fn require(&self, reference: &ExactReference, kind: ArtifactKind) -> Result<(), BindingError> {
        let artifact = self.artifacts.get(reference.identifier.as_str())
            .ok_or_else(|| error(BindingErrorKind::MissingReference, &reference.identifier))?;
        if &artifact.reference != reference {
            return Err(error(BindingErrorKind::ExactReference, &reference.identifier));
        }
        if artifact.kind != kind { return Err(error(BindingErrorKind::ArtifactKind, &reference.identifier)); }
        Ok(())
    }
}

fn object<'a>(program: &'a IrProgram, name: &str) -> Result<&'a IrObjectKind, BindingError> {
    program.objects().iter().find(|o| o.name() == name).map(|o| o.kind())
        .ok_or_else(|| error(BindingErrorKind::MissingReference, name))
}

fn member(name: &str, list: &[String]) -> Result<(), BindingError> {
    if !list.iter().any(|item| item == name) { return Err(error(BindingErrorKind::DomainMembership, name)); }
    Ok(())
}

fn instance_check(program: &IrProgram, contract: &BindingContract, registry: &Registry<'_>,
    instance: &ParameterInstanceBinding) -> Result<(), BindingError> {
    let IrObjectKind::Domain { parameters, capture_specs, admissibility_specs, ternarizers, .. } = object(program, &contract.domain)?
        else { return Err(error(BindingErrorKind::ObjectType, &contract.domain)); };
    if instance.owner != contract.domain { return Err(error(BindingErrorKind::Owner, &instance.identifier)); }
    member(&instance.parameter, parameters)?;
    let IrObjectKind::CaptureSpec { parameter_id, mapping, observation_space, .. } = object(program, &instance.capture.object)?
        else { return Err(error(BindingErrorKind::ObjectType, &instance.capture.object)); };
    member(&instance.capture.object, capture_specs)?;
    if parameter_id != &instance.parameter_id { return Err(error(BindingErrorKind::ParameterIdentity, &instance.capture.object)); }
    registry.require(&instance.capture.definition, ArtifactKind::CaptureDefinition)?;
    if mapping != &instance.capture.definition.identifier { return Err(error(BindingErrorKind::RuleIdentity, &instance.capture.object)); }
    let IrObjectKind::AdmissibilitySpec { parameter_id, rule, .. } = object(program, &instance.admission.object)?
        else { return Err(error(BindingErrorKind::ObjectType, &instance.admission.object)); };
    member(&instance.admission.object, admissibility_specs)?;
    if parameter_id != &instance.parameter_id { return Err(error(BindingErrorKind::ParameterIdentity, &instance.admission.object)); }
    registry.require(&instance.admission.definition, ArtifactKind::AdmissionDefinition)?;
    if rule != &instance.admission.definition.identifier { return Err(error(BindingErrorKind::RuleIdentity, &instance.admission.object)); }
    if let Some(ter) = &instance.ternarizer {
        let IrObjectKind::Ternarizer { mapping, observation_space: ter_space, .. } = object(program, &ter.object)?
            else { return Err(error(BindingErrorKind::ObjectType, &ter.object)); };
        member(&ter.object, ternarizers)?;
        if ter_space != observation_space { return Err(error(BindingErrorKind::ObservationSpace, &ter.object)); }
        registry.require(&ter.definition, ArtifactKind::TernarizerDefinition)?;
        if mapping != &ter.definition.identifier { return Err(error(BindingErrorKind::RuleIdentity, &ter.object)); }
    }
    if instance.provenance.is_empty() { return Err(error(BindingErrorKind::ProvenanceMissing, &instance.identifier)); }
    let mut seen = BTreeSet::new();
    for reference in &instance.provenance {
        if !seen.insert(&reference.identifier) { return Err(error(BindingErrorKind::DuplicateIdentity, &reference.identifier)); }
        registry.require(reference, ArtifactKind::Provenance)?;
    }
    Ok(())
}

/// Comprueba sólo la capacidad LIG/0.1 y la operación solicitada. No ejecuta ni
/// autentica los artefactos aportados, no concede permiso y no produce Tri.
pub fn validate_bindings(program: &IrProgram, contract: BindingContract, request: &BindingRequest)
    -> Result<ValidatedBindings, BindingError> {
    if contract.schema != BINDING_SCHEMA { return Err(error(BindingErrorKind::Schema, &contract.schema)); }
    check_identity(&contract.identifier)?; check_identity(&contract.version)?;
    if request.contract.identifier != contract.identifier || request.contract.version != contract.version
        || request.contract.sha256 != binding_contract_sha256(&contract) {
        return Err(error(BindingErrorKind::ContractIdentity, &contract.identifier));
    }
    if contract.program != ProgramIdentity::of(program) {
        return Err(error(BindingErrorKind::ProgramIdentity, &contract.program.source_file));
    }
    let registry = Registry::new(&contract.artifacts)?;
    registry.require(&contract.constitution, ArtifactKind::Constitution)?;
    registry.require(&contract.authority, ArtifactKind::AuthorityDeclaration)?;
    let IrObjectKind::Agent { architecture, domain, .. } = object(program, &contract.agent)?
        else { return Err(error(BindingErrorKind::ObjectType, &contract.agent)); };
    if domain != &contract.domain { return Err(error(BindingErrorKind::DomainMembership, &contract.domain)); }
    let IrObjectKind::Domain { .. } = object(program, domain)?
        else { return Err(error(BindingErrorKind::ObjectType, domain)); };
    let IrObjectKind::CompositionGraph { nodes, .. } = object(program, architecture)?
        else { return Err(error(BindingErrorKind::ObjectType, architecture)); };

    let mut instances = BTreeMap::new();
    for instance in &contract.instances {
        check_identity(&instance.identifier)?;
        if instances.insert(instance.identifier.as_str(), instance).is_some() {
            return Err(error(BindingErrorKind::DuplicateIdentity, &instance.identifier));
        }
    }
    let mut operation_ids = BTreeSet::new();
    for operation in &contract.operations {
        check_identity(&operation.identifier)?; check_identity(&operation.version)?;
        if !operation_ids.insert(&operation.identifier) { return Err(error(BindingErrorKind::DuplicateIdentity, &operation.identifier)); }
    }
    let index = contract.operations.iter().position(|op| op.identifier == request.operation && op.version == request.operation_version)
        .ok_or_else(|| error(BindingErrorKind::OperationIdentity, &request.operation))?;
    let operation = &contract.operations[index];
    registry.require(&operation.definition, ArtifactKind::OperationDefinition)?;
    if operation.definition.identifier != operation.identifier || operation.definition.version != operation.version {
        return Err(error(BindingErrorKind::RuleIdentity, &operation.identifier));
    }
    if operation.uses.is_empty() { return Err(error(BindingErrorKind::InstanceMissing, &operation.identifier)); }
    let mut uses: BTreeMap<&str, &BindingUse> = BTreeMap::new();
    let mut destinations = BTreeSet::new();
    let mut use_ids_by_instance: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for usage in &operation.uses {
        check_identity(&usage.identifier)?;
        if uses.contains_key(usage.identifier.as_str()) { return Err(error(BindingErrorKind::DuplicateIdentity, &usage.identifier)); }
        let instance = instances.get(usage.instance.as_str())
            .ok_or_else(|| error(BindingErrorKind::InstanceMissing, &usage.instance))?;
        instance_check(program, &contract, &registry, instance)?;
        if operation.requires_destination && usage.destination.is_none() {
            return Err(error(BindingErrorKind::DestinationMissing, &usage.identifier));
        }
        if let Some(destination) = &usage.destination {
            let IrObjectKind::CoupledSpec { cell, .. } = object(program, &destination.node)?
                else { return Err(error(BindingErrorKind::ObjectType, &destination.node)); };
            if !nodes.iter().any(|node| node == &destination.node) {
                return Err(error(BindingErrorKind::NodeMembership, &destination.node));
            }
            let IrObjectKind::CellSpec { n, .. } = object(program, cell)?
                else { return Err(error(BindingErrorKind::ObjectType, cell)); };
            let position = destination.position.as_decimal(); let bound = n.as_decimal();
            if position == "0" || (position.len(), position.as_bytes()) > (bound.len(), bound.as_bytes()) {
                return Err(error(BindingErrorKind::PositionRange, &usage.identifier));
            }
            if usage.alias_of.is_none() && !destinations.insert((destination.node.as_str(), position)) {
                return Err(error(BindingErrorKind::DestinationCollision, &usage.identifier));
            }
        }
        if let Some(alias) = &usage.alias_of {
            let previous = uses.get(alias.as_str()).ok_or_else(|| error(BindingErrorKind::Alias, &usage.identifier))?;
            if previous.instance != usage.instance || previous.destination != usage.destination {
                return Err(error(BindingErrorKind::Alias, &usage.identifier));
            }
        }
        use_ids_by_instance.entry(usage.instance.as_str()).or_default().push(usage.identifier.clone());
        uses.insert(&usage.identifier, usage);
    }
    let mut sharing = BTreeMap::new();
    for declaration in &operation.sharing {
        registry.require(&declaration.rule, ArtifactKind::SharingRule)?;
        if sharing.insert(declaration.instance.as_str(), declaration).is_some() {
            return Err(error(BindingErrorKind::DuplicateIdentity, &declaration.instance));
        }
        let actual = use_ids_by_instance.get(declaration.instance.as_str())
            .ok_or_else(|| error(BindingErrorKind::Sharing, &declaration.instance))?;
        if actual.len() < 2 || actual != &declaration.use_ids {
            return Err(error(BindingErrorKind::Sharing, &declaration.instance));
        }
    }
    for (instance, actual) in use_ids_by_instance {
        if actual.len() > 1 && !sharing.contains_key(instance) {
            return Err(error(BindingErrorKind::Sharing, instance));
        }
    }
    let needs_side = operation.input_scope == BindingInputScope::BindingsWithSideInformation;
    if needs_side == operation.side_information.is_empty() {
        return Err(error(BindingErrorKind::SideInformation, &operation.identifier));
    }
    let mut side_ids = BTreeSet::new();
    for reference in &operation.side_information {
        if !side_ids.insert(&reference.identifier) { return Err(error(BindingErrorKind::DuplicateIdentity, &reference.identifier)); }
        registry.require(reference, ArtifactKind::SideInformation)?;
    }
    Ok(ValidatedBindings { contract, operation_index: index, expectation: request.contract.clone(), program: program.clone() })
}

// Codificación de identidad independiente de Debug y de la disposición de memoria.
struct Encoder(Vec<u8>);
impl Encoder {
    fn bytes(&mut self, value: &[u8]) { self.len(value.len()); self.0.extend_from_slice(value); }
    fn text(&mut self, value: &str) { self.bytes(value.as_bytes()); }
    fn len(&mut self, value: usize) { self.0.extend_from_slice(&(value as u64).to_be_bytes()); }
    fn boolean(&mut self, value: bool) { self.0.push(u8::from(value)); }
    fn reference(&mut self, r: &ExactReference) { self.text(&r.identifier); self.text(&r.version); self.text(&r.sha256); }
    fn refs(&mut self, refs: &[ExactReference]) { self.len(refs.len()); for r in refs { self.reference(r); } }
    fn rule(&mut self, rule: &RuleBinding) { self.text(&rule.object); self.reference(&rule.definition); }
}

/// Huella de la representación recibida; no es un constructor de autoridad ni
/// debe sustituir la expectativa fijada por el consumidor antes de validar.
pub fn binding_contract_sha256(c: &BindingContract) -> String {
    let mut e = Encoder(b"SV-LIG-0.1\0".to_vec());
    e.text(&c.schema); e.text(&c.identifier); e.text(&c.version);
    e.text(&c.program.source_file); e.text(&c.program.source_sha256); e.text(&c.program.projection_sha256);
    e.text(&c.domain); e.text(&c.agent); e.reference(&c.constitution); e.reference(&c.authority);
    e.len(c.instances.len());
    for i in &c.instances {
        e.text(&i.identifier); e.text(&i.owner); e.text(&i.parameter); e.text(i.parameter_id.as_decimal());
        e.rule(&i.capture); e.rule(&i.admission); e.boolean(i.ternarizer.is_some());
        if let Some(rule) = &i.ternarizer { e.rule(rule); }
        e.refs(&i.provenance);
    }
    e.len(c.operations.len());
    for op in &c.operations {
        e.text(&op.identifier); e.text(&op.version); e.reference(&op.definition); e.len(op.uses.len());
        for usage in &op.uses {
            e.text(&usage.identifier); e.text(&usage.instance); e.boolean(usage.destination.is_some());
            if let Some(d) = &usage.destination { e.text(&d.node); e.text(d.position.as_decimal()); }
            e.boolean(usage.alias_of.is_some()); if let Some(alias) = &usage.alias_of { e.text(alias); }
        }
        e.boolean(op.requires_destination); e.len(op.sharing.len());
        for sharing in &op.sharing {
            e.text(&sharing.instance); e.len(sharing.use_ids.len()); for id in &sharing.use_ids { e.text(id); }
            e.reference(&sharing.rule);
        }
        e.text(match op.input_scope { BindingInputScope::BindingsOnly => "BindingsOnly", BindingInputScope::BindingsWithSideInformation => "BindingsWithSideInformation" });
        e.refs(&op.side_information);
    }
    e.len(c.artifacts.len());
    for artifact in &c.artifacts { e.reference(&artifact.reference); e.text(artifact.kind.label()); e.bytes(&artifact.bytes); }
    sha256_hex(&e.0)
}
