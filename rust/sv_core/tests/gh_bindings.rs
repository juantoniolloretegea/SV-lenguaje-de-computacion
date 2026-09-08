#[allow(dead_code)]
#[path = "../../../tests/row7_gh/probe.rs"]
mod probe;
macro_rules! witness {
    ($name:ident,$id:literal) => { #[test] fn $name() { probe::check_witness($id).unwrap(); } };
}
witness!(unidad_metodo_referencia,"GH-DOC-01");
witness!(atribucion_documental,"GH-DOC-02");
witness!(estado_propuesta,"GH-DOC-03");
witness!(adjudicacion_separada,"GH-DOC-04");
witness!(antecedente_enmienda,"GH-DOC-05");
witness!(u_y_configuracion_ausente,"GH-DOC-06");
witness!(ligaduras_ordenadas,"GH-DOC-07");
witness!(extremo_de_ventana,"GH-DOC-08");
