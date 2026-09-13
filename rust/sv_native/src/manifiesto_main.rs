use std::process::ExitCode;

fn main() -> ExitCode {
    // ES: La consulta no admite rutas, perfiles ni texto sustitutorio.
    // EN: The query accepts no paths, profiles or replacement text.
    if std::env::args_os().nth(1).is_some() {
        eprintln!("uso / usage: manifiesto-sv");
        return ExitCode::from(2);
    }
    sv_native::mostrar_manifiesto()
}
