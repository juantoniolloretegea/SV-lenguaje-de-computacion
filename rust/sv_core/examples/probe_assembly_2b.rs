use std::{env, fs, process::ExitCode};
use sv_core::{compile_svp_assembly, equivalence_json, SourceProfile, SourceUnit};

fn profile(tag: &str) -> Option<SourceProfile> { SourceProfile::from_tag(tag) }

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("uso: probe_assembly_2b <en|es> <a.svp> <en|es> <b.svp>");
        return ExitCode::from(2);
    }
    let Some(pa) = profile(&args[1]) else { return ExitCode::from(2) };
    let Some(pb) = profile(&args[3]) else { return ExitCode::from(2) };
    let Ok(a) = fs::read_to_string(&args[2]) else { return ExitCode::from(2) };
    let Ok(b) = fs::read_to_string(&args[4]) else { return ExitCode::from(2) };
    let units = [
        SourceUnit::new(&a, &args[2], pa),
        SourceUnit::new(&b, &args[4], pb),
    ];
    match compile_svp_assembly(&units) {
        Ok(program) => {
            println!("{}", equivalence_json(&program));
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("ensamblaje no admitido: {error:?}");
            ExitCode::from(1)
        }
    }
}


// SPDX-License-Identifier: CC-BY-NC-ND-4.0
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
// https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
