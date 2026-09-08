#[path = "../../../tests/row7_bindings/cases.rs"]
mod cases;
#[test]
fn l01_version() { cases::check_case(0).unwrap(); }
#[test]
fn l02_orden_expectativa() { cases::check_case(1).unwrap(); }
#[test]
fn l03_programa() { cases::check_case(2).unwrap(); }
#[test]
fn l04_bytes() { cases::check_case(3).unwrap(); }
#[test]
fn l05_version_duplicada() { cases::check_case(4).unwrap(); }
#[test]
fn l06_referencia_exacta() { cases::check_case(5).unwrap(); }
#[test]
fn l06_referencia_ausente() { cases::check_case(6).unwrap(); }
#[test]
fn l07_clase() { cases::check_case(7).unwrap(); }
#[test]
fn l08_instancia_duplicada() { cases::check_case(8).unwrap(); }
#[test]
fn l09_propietario() { cases::check_case(9).unwrap(); }
#[test]
fn l09_parametro() { cases::check_case(10).unwrap(); }
#[test]
fn l10_captura_tipo() { cases::check_case(11).unwrap(); }
#[test]
fn l10_captura_ajena() { cases::check_case(12).unwrap(); }
#[test]
fn l10_admision_tipo() { cases::check_case(13).unwrap(); }
#[test]
fn l10_admision_ajena() { cases::check_case(14).unwrap(); }
#[test]
fn l11_numeral_captura() { cases::check_case(15).unwrap(); }
#[test]
fn l11_numeral_admision() { cases::check_case(16).unwrap(); }
#[test]
fn l12_regla() { cases::check_case(17).unwrap(); }
#[test]
fn l13_ternarizador_ajeno() { cases::check_case(18).unwrap(); }
#[test]
fn l13_espacio() { cases::check_case(19).unwrap(); }
#[test]
fn l14_procedencia() { cases::check_case(20).unwrap(); }
#[test]
fn l15_operacion() { cases::check_case(21).unwrap(); }
#[test]
fn l15_version_operacion() { cases::check_case(22).unwrap(); }
#[test]
fn l16_instancia_ausente() { cases::check_case(23).unwrap(); }
#[test]
fn l17_destino_ausente() { cases::check_case(24).unwrap(); }
#[test]
fn l18_destino_tipo() { cases::check_case(25).unwrap(); }
#[test]
fn l18_destino_ausente() { cases::check_case(26).unwrap(); }
#[test]
fn l18_nodo_ajeno() { cases::check_case(27).unwrap(); }
#[test]
fn l19_cero() { cases::check_case(28).unwrap(); }
#[test]
fn l19_diez() { cases::check_case(29).unwrap(); }
#[test]
fn l19_nat_grande() { cases::check_case(30).unwrap(); }
#[test]
fn l20_colision() { cases::check_case(31).unwrap(); }
#[test]
fn l21_comparticion_ausente() { cases::check_case(32).unwrap(); }
#[test]
fn l22_alias_posterior() { cases::check_case(33).unwrap(); }
#[test]
fn l22_alias_instancia() { cases::check_case(34).unwrap(); }
#[test]
fn l22_alias_destino() { cases::check_case(35).unwrap(); }
#[test]
fn l23_comparticion_incompleta() { cases::check_case(36).unwrap(); }
#[test]
fn l23_comparticion_orden() { cases::check_case(37).unwrap(); }
#[test]
fn l24_lateral_oculta() { cases::check_case(38).unwrap(); }
#[test]
fn l24_lateral_ausente() { cases::check_case(39).unwrap(); }
#[test]
fn l00_esquema() { cases::check_case(40).unwrap(); }
#[test]
fn l00_identidad_vacia() { cases::check_case(41).unwrap(); }
#[test]
fn l00_uso_duplicado() { cases::check_case(42).unwrap(); }
#[test]
fn l00_operacion_duplicada() { cases::check_case(43).unwrap(); }
#[test]
fn controles_positivos_preservan_identidad_orden_y_multiplicidad() {
    for i in cases::NEGATIVE_COUNT..cases::CASE_COUNT { cases::check_case(i).unwrap(); }
}
