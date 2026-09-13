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
