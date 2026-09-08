use sv_core::{compile_svp, CompileError};

// Gramática 0.2 §13 y catálogo efectivo §3/E011: el cierre interno es `}`;
// cada salida literal debe pertenecer al codominio declarado. El testigo
// conserva todas las entradas y sólo FUERA infringe esa pertenencia.
const SOURCE: &str = include_str!(
    "../../../tests/conformance/invalid/admissibility_table_output_fuera_codominio.svp"
);

#[test]
fn salida_ajena_al_codominio_alcanza_la_guarda_semantica() {
    match compile_svp(SOURCE, "admissibility_table_output_fuera_codominio.svp") {
        Err(CompileError::InvalidProgram(message)) => {
            assert_eq!(message, "AdmissibilityTable T1: salida fuera de codominio");
        }
        other => panic!("se exigía el rechazo semántico de T1, recibido: {other:?}"),
    }
}

#[test]
fn sustituir_solo_la_salida_por_un_miembro_declarado_admite_la_tabla() {
    assert_eq!(SOURCE.matches("(B) -> FUERA;").count(), 1);
    let control = SOURCE.replace("(B) -> FUERA;", "(B) -> NO_OK;");
    compile_svp(&control, "admissibility_table_control.svp").unwrap();
}

#[test]
fn el_cierre_interno_historico_falla_en_sintaxis() {
    assert_eq!(SOURCE.matches("\n  }\n}").count(), 1);
    let historical = SOURCE.replace("\n  }\n}", "\n  };\n}");
    assert!(matches!(
        compile_svp(&historical, "admissibility_table_cierre_historico.svp"),
        Err(CompileError::Frontend(_))
    ));
}
