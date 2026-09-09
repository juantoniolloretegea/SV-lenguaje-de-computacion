#[allow(dead_code)]
#[path = "../../../tests/retorno_cyb/probe.rs"]
mod probe;
#[test] fn cyb_transporte_en() {probe::check_entry(0);}
#[test] fn cyb_transporte_es() {probe::check_entry(1);}
#[test] fn cyb_ensamblaje_directo() {probe::check_entry(2);}
#[test] fn cyb_ensamblaje_inverso() {probe::check_entry(3);}
#[test] fn cyb_rechazos_causales_y_reparaciones() {probe::check_rejections();}
#[test] fn cyb_instancias_documentales_independientes() {probe::check_independent_instances();}
#[test] fn cyb_definicion_y_modulos_exactos() {probe::check_operation_artifacts();}
