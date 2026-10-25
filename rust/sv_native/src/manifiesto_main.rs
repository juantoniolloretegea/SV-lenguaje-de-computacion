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


// SPDX-License-Identifier: CC-BY-NC-ND-4.0
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
// https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
