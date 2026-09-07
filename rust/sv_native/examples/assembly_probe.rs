//! Test transport for the existing public SV API; no production CLI extension.
use std::{env, fs, path::Path, process::ExitCode};
use sv_core::{compile_svp_profile, compile_svp_assembly, equivalence_json, SourceProfile, SourceUnit};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 3 && args.len() != 6 {
        eprintln!("uso de prueba: assembly_probe --profile en|es archivo [--profile en|es archivo]");
        return ExitCode::from(2);
    }
    let mut inputs = Vec::new();
    for chunk in args.chunks_exact(3) {
        let profile = SourceProfile::from_tag(&chunk[1]);
        if chunk[0] != "--profile" || profile.is_none() {
            eprintln!("perfil de prueba inválido");
            return ExitCode::from(2);
        }
        let path = Path::new(&chunk[2]);
        let Some(file) = path.file_name().and_then(|s| s.to_str()) else { return ExitCode::from(2); };
        let source = match fs::read_to_string(path) {
            Ok(source) => source,
            Err(error) => { eprintln!("lectura de prueba: {error}"); return ExitCode::from(2); }
        };
        inputs.push((source, file.to_owned(), profile.unwrap()));
    }
    let units: Vec<_> = inputs.iter().map(|(source, file, profile)| SourceUnit::new(source, file, *profile)).collect();
    let result = if inputs.len() == 1 {
        compile_svp_profile(&inputs[0].0, &inputs[0].1, inputs[0].2)
    } else { compile_svp_assembly(&units) };
    match result {
        Ok(program) => { println!("{}", equivalence_json(&program)); ExitCode::SUCCESS }
        Err(error) => { eprintln!("SVP no admitido: {error:?}"); ExitCode::from(1) }
    }
}
