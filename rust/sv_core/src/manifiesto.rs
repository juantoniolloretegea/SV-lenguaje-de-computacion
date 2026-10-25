//! ES: Consulta documental incorporada; no ejecuta la capa Rosetta del acta.
//! EN: Embedded documentary query; it does not execute the act's Rosetta layer.

/// ES: Devuelve el acta española íntegra incorporada al compilar, sin E/S ni red.
/// EN: Returns the complete Spanish act embedded at build time, without I/O or network.
///
/// ES: Su disponibilidad no acredita mecanismos de bloqueo de usos prohibidos.
/// EN: Its availability does not attest enforcement of prohibited-use restrictions.
pub fn manifiesto_sv() -> &'static str {
    include_str!("../assets/manifiesto-sv.md")
}


// SPDX-License-Identifier: CC-BY-NC-ND-4.0
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
// https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
