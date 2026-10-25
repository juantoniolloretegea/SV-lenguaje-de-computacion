//! ES: Transporte de consultas documentales del núcleo a la salida estándar.
//! EN: Transport of core documentary queries to standard output.

use std::io::{self, Write};
use std::process::ExitCode;

/// ES: Emite los bytes exactos; un fallo de escritura es técnico y devuelve 2.
/// EN: Emits exact bytes; a write failure is technical and returns 2.
pub fn mostrar_manifiesto() -> ExitCode {
    match io::stdout().lock().write_all(sv_core::manifiesto_sv().as_bytes()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let _ = writeln!(io::stderr().lock(), "no se pudo escribir el manifiesto / could not write manifesto: {error}");
            ExitCode::from(2)
        }
    }
}

// SPDX-License-Identifier: CC-BY-NC-ND-4.0
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
// https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
