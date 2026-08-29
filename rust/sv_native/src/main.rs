use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use sv_core::{compile_svp, compile_svp_profile, equivalence_json, SourceProfile};

fn usage() {
    eprintln!("uso: sv-native [--profile en|es] <archivo.svp>");
}

fn main() -> ExitCode {
    let mut args = env::args_os();
    let _exe = args.next();
    let Some(first) = args.next() else {
        usage();
        return ExitCode::from(2);
    };

    let (profile, path) = if first == "--profile" {
        let Some(tag_os) = args.next() else {
            usage();
            return ExitCode::from(2);
        };
        let Some(tag) = tag_os.to_str() else {
            eprintln!("perfil SVP no UTF-8");
            return ExitCode::from(2);
        };
        let Some(profile) = SourceProfile::from_tag(tag) else {
            eprintln!("perfil SVP no admitido: {tag}; use en o es");
            return ExitCode::from(2);
        };
        let Some(path) = args.next() else {
            usage();
            return ExitCode::from(2);
        };
        (Some(profile), path)
    } else {
        (None, first)
    };

    if args.next().is_some() {
        usage();
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

    let result = match profile {
        Some(profile) => compile_svp_profile(&source, file_name, profile),
        None => compile_svp(&source, file_name),
    };
    match result {
        Ok(program) => {
            println!("{}", equivalence_json(&program));
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("SVP no admitido: {error:?}");
            ExitCode::from(1)
        }
    }
}
