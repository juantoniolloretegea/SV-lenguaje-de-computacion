//! RETP-108: clientes externos; no constituyen autoridad ni arquitectura CYB.
//! Rustc compila contra la biblioteca ordinaria que construyó Cargo, no contra
//! el módulo de pruebas del núcleo. Las macros sólo pertenecen a estas sondas.
use std::{env, fs, path::PathBuf, process::Command, sync::atomic::{AtomicUsize, Ordering}};

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        // Son productos de compilación: se conservan en la salida de Cargo.
        // El entorno aislado mantiene /tmp sin permiso de ejecución.
        let output = env::current_exe().unwrap().parent().unwrap().to_path_buf();
        let path = output.join(format!("sv-cyb-authority-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        fs::create_dir(&path).expect("crear directorio exclusivo de la sonda");
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
}

fn client(source: &str, rejection: Option<(&str, &[&str])>) {
    let scratch = Scratch::new();
    let input = scratch.0.join("cliente.rs");
    fs::write(&input, format!("#![forbid(unsafe_code)]\n{source}\n")).unwrap();
    let executable = scratch.0.join(format!("cliente{}", env::consts::EXE_SUFFIX));
    let deps = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let libraries: Vec<_> = fs::read_dir(&deps).unwrap().map(|entry| entry.unwrap().path())
        .filter(|path| path.file_name().and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("libsv_core") && name.ends_with(".rlib")))
        .collect();
    assert_eq!(libraries.len(), 1, "la biblioteca ordinaria debe identificarse sin ambigüedad: {libraries:?}");
    let output = Command::new(env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
        .args(["--edition=2021", "--error-format=short", "--crate-name", "cliente_cyb", "--extern"])
        .arg(format!("sv_core={}", libraries[0].display())).arg("-L")
        .arg(format!("dependency={}", deps.display()))
        .arg(&input).arg("-o").arg(&executable)
        .output().expect("ejecutar rustc de la herramienta ya declarada");
    let diagnostics = String::from_utf8(output.stderr).unwrap();
    match rejection {
        None => {
            assert!(output.status.success(), "control positivo rechazado: {diagnostics}");
            let result = Command::new(&executable).output().expect("ejecutar control positivo");
            assert!(result.status.success(), "control positivo fallido: {}", String::from_utf8_lossy(&result.stderr));
        }
        Some((code, subjects)) => {
            assert!(!output.status.success(), "autoatribución admitida: {source}");
            // Formato corto de rustc: sólo se acepta un error con código y
            // sujeto pertinentes. Errores de importación/enlace no acreditan la sonda.
            let errors: Vec<_> = diagnostics.lines().filter(|line| line.contains(": error[")).collect();
            assert_eq!(errors.len(), 1, "diagnóstico causal ambiguo: {diagnostics}");
            assert!(errors[0].contains(&format!(": error[{code}]")), "causa distinta: {diagnostics}");
            for subject in subjects {
                assert!(errors[0].contains(subject), "sujeto {subject} ausente: {diagnostics}");
            }
        }
    }
}

#[test]
fn datos_documentales_no_constituyen_autoridad() {
    client(r#"
use sv_core::{ControlId, CheckResult};
use sv_core::bindings::{ArtifactKind, BindingArtifact, ExactReference, ValidatedBindings};
use sv_core::authority::transitions::{AuthorityContinuity, GenesisPlan};
fn referencia(b: &ValidatedBindings) -> &ExactReference { &b.contract().authority }
fn main() {
    let id = ControlId::new("emisor-declarado").unwrap();
    let dato = BindingArtifact {
        reference: ExactReference { identifier: id.as_str().into(), version: "1".into(), sha256: "0".repeat(64) },
        kind: ArtifactKind::AuthorityDeclaration, bytes: b"declaracion no admitida".to_vec(),
    };
    assert_eq!(dato.reference.identifier, "emisor-declarado");
    let _resultado_declarado = CheckResult::Accredited;
    let _lector: fn(&ValidatedBindings) -> &ExactReference = referencia;
    let continuidad = AuthorityContinuity::uninhabited();
    let propuesta = GenesisPlan::new([], []);
    assert_eq!(continuidad.authority_count(), 0);
    assert_eq!(propuesta.authority_count(), 0);
}
"#, None);
}

#[test]
fn macro_de_datos_nominales_admitida() {
    client(r#"
use sv_core::ControlId;
macro_rules! dato { ($s:expr) => { ControlId::new($s).unwrap() }; }
fn main() { assert_eq!(dato!("emisor-declarado").as_str(), "emisor-declarado"); }
"#, None);
}

#[test]
fn fa01_referencia_de_autoridad_privada() {
    client(r#"
use sv_core::{AuthorityRef, ControlId};
fn main() { let _ = AuthorityRef::from_core_id(ControlId::new("emisor").unwrap()); }
"#, Some(("E0624", &["from_core_id", "private"])));
}

#[test]
fn fa02_macro_no_elude_referencia_privada() {
    client(r#"
use sv_core::{AuthorityRef, ControlId};
macro_rules! autoridad { () => { AuthorityRef::from_core_id(ControlId::new("emisor").unwrap()) }; }
fn main() { let _ = autoridad!(); }
"#, Some(("E0624", &["from_core_id", "private"])));
}

#[test]
fn fa03_premisa_externa_no_fabricable() {
    client(r#"
use sv_core::authority::transitions::ExternalGenesisPremise;
fn main() { let _ = ExternalGenesisPremise { consumed: false }; }
"#, Some(("E0451", &["consumed", "ExternalGenesisPremise", "private"])));
}

#[test]
fn fa04_macro_no_elude_premisa_privada() {
    client(r#"
use sv_core::authority::transitions::ExternalGenesisPremise;
macro_rules! premisa { () => { ExternalGenesisPremise { consumed: false } }; }
fn main() { let _ = premisa!(); }
"#, Some(("E0451", &["consumed", "ExternalGenesisPremise", "private"])));
}

#[test]
fn fa05_ligadura_validada_no_es_autoridad() {
    client(r#"
use sv_core::{bindings::ValidatedBindings, ConstitutedAuthority};
fn suplantar(b: &ValidatedBindings) -> ConstitutedAuthority { b.contract().authority.clone() }
fn main() {}
"#, Some(("E0308", &["ConstitutedAuthority", "ExactReference"])));
}

#[test]
fn fa06_resultado_elegido_no_es_permiso() {
    client(r#"
use sv_core::{CheckResult, TracedPermit};
fn main() { let _: TracedPermit = CheckResult::Accredited.into(); }
"#, Some(("E0277", &["TracedPermit", "CheckResult"])));
}

#[test]
fn fa07_booleano_no_sustituye_premisa() {
    client(r#"
use sv_core::authority::transitions::{AuthorityContinuity, GenesisPlan};
fn main() {
    let mut continuidad = AuthorityContinuity::uninhabited();
    let _ = continuidad.apply_genesis(&mut true, GenesisPlan::new([], []));
}
"#, Some(("E0308", &["ExternalGenesisPremise", "bool"])));
}
