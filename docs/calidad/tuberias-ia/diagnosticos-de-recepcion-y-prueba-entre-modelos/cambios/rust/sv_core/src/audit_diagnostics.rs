//! RETP-157. Identidad diagnóstica de fronteras candidatas; no autoridad SV.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language { Es, En }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticStage { PrepareReception, Receive, PrepareCheck, Compare }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCause {
    EmptyAct,
    EmptyIssuer,
    EmptyVersion,
    ActTooLarge,
    IssuerTooLarge,
    VersionTooLarge,
    IssuerMismatch,
    VersionMismatch,
    DeclarationsMismatch,
    ActMismatch,
    AlreadyAttempted,
    GenesisRejected,
    ForeignBinding,
    CoreRequirement,
    EmptyContract,
    ContractTooLarge,
    ExpectedTooLarge,
    ObservedTooLarge,
    ExactMismatch,
    MissingObservation,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Diagnostic { cause: DiagnosticCause, stage: DiagnosticStage }
impl Diagnostic {
 pub const VERSION: &'static str = "RECEPTION-DIAGNOSTICS/1";
 pub(crate) const fn new(cause: DiagnosticCause, stage: DiagnosticStage) -> Self { Self { cause, stage } }
 pub const fn cause(&self) -> DiagnosticCause { self.cause }
 pub const fn stage(&self) -> DiagnosticStage { self.stage }
 pub const fn code(&self) -> &'static str { match self.cause {
 DiagnosticCause::EmptyAct => "RG.EMPTY_ACT",
 DiagnosticCause::EmptyIssuer => "RG.EMPTY_ISSUER",
 DiagnosticCause::EmptyVersion => "RG.EMPTY_VERSION",
 DiagnosticCause::ActTooLarge => "RG.ACT_TOO_LARGE",
 DiagnosticCause::IssuerTooLarge => "RG.ISSUER_TOO_LARGE",
 DiagnosticCause::VersionTooLarge => "RG.VERSION_TOO_LARGE",
 DiagnosticCause::IssuerMismatch => "RG.ISSUER_MISMATCH",
 DiagnosticCause::VersionMismatch => "RG.VERSION_MISMATCH",
 DiagnosticCause::DeclarationsMismatch => "RG.DECLARATIONS_MISMATCH",
 DiagnosticCause::ActMismatch => "RG.ACT_MISMATCH",
 DiagnosticCause::AlreadyAttempted => "RG.ALREADY_ATTEMPTED",
 DiagnosticCause::GenesisRejected => "RG.GENESIS_REJECTED",
 DiagnosticCause::ForeignBinding => "OC.FOREIGN_BINDING",
 DiagnosticCause::CoreRequirement => "OC.CORE_REQUIREMENT",
 DiagnosticCause::EmptyContract => "OC.EMPTY_CONTRACT",
 DiagnosticCause::ContractTooLarge => "OC.CONTRACT_TOO_LARGE",
 DiagnosticCause::ExpectedTooLarge => "OC.EXPECTED_TOO_LARGE",
 DiagnosticCause::ObservedTooLarge => "OC.OBSERVED_TOO_LARGE",
 DiagnosticCause::ExactMismatch => "OC.EXACT_MISMATCH",
 DiagnosticCause::MissingObservation => "OC.MISSING_OBSERVATION",
 } }
 // Sólo texto estático: ningún dato recibido se inserta en el diagnóstico.
 // El resultado no se calcula a partir del idioma ni del texto mostrado.
 pub const fn message(&self, language: Language) -> &'static str { match (self.cause, language) {
 (DiagnosticCause::EmptyAct, Language::Es) => "El acto recibido está vacío.",
 (DiagnosticCause::EmptyAct, Language::En) => "The received act is empty.",
 (DiagnosticCause::EmptyIssuer, Language::Es) => "La declaración de emisor está vacía.",
 (DiagnosticCause::EmptyIssuer, Language::En) => "The issuer declaration is empty.",
 (DiagnosticCause::EmptyVersion, Language::Es) => "La versión declarada está vacía.",
 (DiagnosticCause::EmptyVersion, Language::En) => "The declared version is empty.",
 (DiagnosticCause::ActTooLarge, Language::Es) => "El acto supera el límite de bytes.",
 (DiagnosticCause::ActTooLarge, Language::En) => "The act exceeds the byte limit.",
 (DiagnosticCause::IssuerTooLarge, Language::Es) => "La declaración de emisor supera el límite de bytes.",
 (DiagnosticCause::IssuerTooLarge, Language::En) => "The issuer declaration exceeds the byte limit.",
 (DiagnosticCause::VersionTooLarge, Language::Es) => "La versión declarada supera el límite de bytes.",
 (DiagnosticCause::VersionTooLarge, Language::En) => "The declared version exceeds the byte limit.",
 (DiagnosticCause::IssuerMismatch, Language::Es) => "El emisor declarado no coincide con el instalado.",
 (DiagnosticCause::IssuerMismatch, Language::En) => "The declared issuer does not match the installed issuer.",
 (DiagnosticCause::VersionMismatch, Language::Es) => "La versión declarada no coincide con la instalada.",
 (DiagnosticCause::VersionMismatch, Language::En) => "The declared version does not match the installed version.",
 (DiagnosticCause::DeclarationsMismatch, Language::Es) => "El emisor y la versión declarados no coinciden con los instalados.",
 (DiagnosticCause::DeclarationsMismatch, Language::En) => "The declared issuer and version do not match the installed declarations.",
 (DiagnosticCause::ActMismatch, Language::Es) => "El acto recibido difiere del acto instalado.",
 (DiagnosticCause::ActMismatch, Language::En) => "The received act differs from the installed act.",
 (DiagnosticCause::AlreadyAttempted, Language::Es) => "El intento de génesis de esta preparación ya se ha consumido.",
 (DiagnosticCause::AlreadyAttempted, Language::En) => "The genesis attempt for this preparation has already been consumed.",
 (DiagnosticCause::GenesisRejected, Language::Es) => "T-0 ha rechazado el plan; se conserva su error tipado.",
 (DiagnosticCause::GenesisRejected, Language::En) => "T-0 rejected the plan; its typed error is retained.",
 (DiagnosticCause::ForeignBinding, Language::Es) => "La obligación o su aplicabilidad no pertenece al vínculo exigido.",
 (DiagnosticCause::ForeignBinding, Language::En) => "The requirement or its applicability does not belong to the required binding.",
 (DiagnosticCause::CoreRequirement, Language::Es) => "La igualdad de bytes no puede verificar esta obligación nuclear.",
 (DiagnosticCause::CoreRequirement, Language::En) => "Byte equality cannot verify this core requirement.",
 (DiagnosticCause::EmptyContract, Language::Es) => "El contrato de comprobación está vacío.",
 (DiagnosticCause::EmptyContract, Language::En) => "The check contract is empty.",
 (DiagnosticCause::ContractTooLarge, Language::Es) => "El contrato de comprobación supera el límite de bytes.",
 (DiagnosticCause::ContractTooLarge, Language::En) => "The check contract exceeds the byte limit.",
 (DiagnosticCause::ExpectedTooLarge, Language::Es) => "La referencia esperada supera el límite de bytes.",
 (DiagnosticCause::ExpectedTooLarge, Language::En) => "The expected reference exceeds the byte limit.",
 (DiagnosticCause::ObservedTooLarge, Language::Es) => "La observación supera el límite de bytes.",
 (DiagnosticCause::ObservedTooLarge, Language::En) => "The observation exceeds the byte limit.",
 (DiagnosticCause::ExactMismatch, Language::Es) => "La observación difiere de la referencia exacta.",
 (DiagnosticCause::ExactMismatch, Language::En) => "The observation differs from the exact reference.",
 (DiagnosticCause::MissingObservation, Language::Es) => "No se dispone de la observación necesaria para comprobar.",
 (DiagnosticCause::MissingObservation, Language::En) => "The observation required for verification is unavailable.",
 } }
}
