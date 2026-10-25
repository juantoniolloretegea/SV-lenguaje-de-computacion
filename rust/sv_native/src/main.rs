use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use sv_core::{compile_svp, equivalence_json};

fn main() -> ExitCode {
    let mut args = env::args_os();
    let _exe = args.next();
    let Some(path) = args.next() else {
        eprintln!("uso: sv-native <archivo.svp>");
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        eprintln!("uso: sv-native <archivo.svp>");
        return ExitCode::from(2);
    }

    let path = Path::new(&path);
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("no se pudo leer {}: {error}", path.display());
            return ExitCode::from(2);
        }
    };
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        eprintln!("nombre de archivo no UTF-8");
        return ExitCode::from(2);
    };

    match compile_svp(&source, file_name) {
        Ok(program) => {
            println!("{}", equivalence_json(&program));
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("SVP no admitido por el frente R0-7: {error:?}");
            ExitCode::from(1)
        }
    }
}

// SPDX-License-Identifier: CC-BY-NC-ND-4.0
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
// https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
