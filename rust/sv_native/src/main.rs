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
