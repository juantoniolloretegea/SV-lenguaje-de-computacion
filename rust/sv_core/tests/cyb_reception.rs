#[allow(dead_code)]
#[path = "../../../tests/retorno_cyb/probe.rs"]
mod probe;
#[test] fn cyb_transporte_en() {probe::check_entry(0);}
#[test] fn cyb_transporte_es() {probe::check_entry(1);}
#[test] fn cyb_ensamblaje_directo() {probe::check_entry(2);}
#[test] fn cyb_ensamblaje_inverso() {probe::check_entry(3);}
#[test] fn cyb_rechazos_causales_y_reparaciones() {probe::check_rejections();}
#[test] fn cyb_instancias_documentales_independientes() {probe::check_independent_instances();}

// SPDX-License-Identifier: CC-BY-NC-ND-4.0
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
// https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
