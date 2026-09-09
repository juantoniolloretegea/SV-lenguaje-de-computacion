//! N0-02: relación total, perfiles fuente y ensamblaje con referencias cruzadas.
use sv_core::{
    compile_svp_assembly, compile_svp_profile, CompileError, IrObjectKind, IrProgram,
    SourceProfile, SourceUnit,
};

const CELL_EN: &str = "cellspec C { b: 3; codomain: K; semantics: S; role: Base; }";
const CELL_ES: &str = "especificación_de_celda C { b: 3; codominio: K; semántica: S; rol: Base; }";

fn declarations(profile: SourceProfile, mappings: &str) -> String {
    match profile {
        SourceProfile::En => format!("codomain K = {{ B, A }}; output_semantics S {{ {mappings} }}"),
        SourceProfile::Es => format!("codominio K = {{ B, A }}; semántica_de_salida S {{ {mappings} }}"),
    }
}

fn reject(result: Result<IrProgram, CompileError>, reason: &str, cell: &str) {
    match result {
        Err(error) if error.legacy_program_message().is_some() => {
            let message = error.legacy_program_message().unwrap();
            for token in ["E115 (InvalidOutputSemantics)", reason, cell, "OutputSemantics S"] {
                assert!(message.contains(token), "diagnóstico inesperado: {message}");
            }
        }
        other => panic!("la relación debía rechazarse antes de emitir JSON: {other:?}"),
    }
}

#[test]
fn n0_02_rechaza_cuatro_defectos_en_es_en_y_referencias_adelantadas() {
    for (mappings, reason) in [
        ("", "ausentes=[A, B]"),
        ("A -> \"a\";", "ausentes=[B]"),
        ("A -> \"a\"; B -> \"b\"; X -> \"x\";", "ajenas=[X]"),
        ("A -> \"a\"; B -> \"b\"; A -> \"a\";", "repetidas=[A]"),
        ("A -> \"a\"; B -> \"b\"; A -> \"otro\";", "repetidas=[A]"),
    ] {
        for (profile, cell) in [(SourceProfile::En, CELL_EN), (SourceProfile::Es, CELL_ES)] {
            let defs = declarations(profile, mappings);
            for source in [format!("{defs}{cell}"), format!("{cell}{defs}")] {
                reject(compile_svp_profile(&source, "n0-02.svp", profile), reason, "CellSpec C");
            }
        }
    }
}

#[test]
fn n0_02_preserva_orden_y_textos_compartidos_en_es_en() {
    for (profile, cell) in [(SourceProfile::En, CELL_EN), (SourceProfile::Es, CELL_ES)] {
        let source = format!("{cell}{}", declarations(profile, "A -> \"igual\"; B -> \"igual\";"));
        let program = compile_svp_profile(&source, "n0-02.svp", profile).unwrap();
        match program.objects()[1].kind() {
            IrObjectKind::Codomain { values } => assert_eq!(values, &["B", "A"]),
            other => panic!("objeto inesperado: {other:?}"),
        }
        match program.objects()[2].kind() {
            IrObjectKind::OutputSemantics { mappings } => assert_eq!(
                mappings, &[("A".to_string(), "igual".to_string()), ("B".to_string(), "igual".to_string())]
            ),
            other => panic!("objeto inesperado: {other:?}"),
        }
    }
}

#[test]
fn n0_02_ensambla_relacion_total_entre_unidades_en_es() {
    let defs = declarations(SourceProfile::En, "A -> \"igual\"; B -> \"igual\";");
    for reversed in [false, true] {
        let mut units = vec![
            SourceUnit::new(&defs, "defs-en.svp", SourceProfile::En),
            SourceUnit::new(CELL_ES, "cell-es.svp", SourceProfile::Es),
        ];
        if reversed { units.reverse(); }
        assert!(compile_svp_assembly(&units).is_ok());
    }
}

#[test]
fn n0_02_ensamblaje_no_rescata_relacion_invalida() {
    for (mappings, reason) in [
        ("", "ausentes=[A, B]"),
        ("A -> \"a\";", "ausentes=[B]"),
        ("A -> \"a\"; B -> \"b\"; X -> \"x\";", "ajenas=[X]"),
        ("A -> \"a\"; B -> \"b\"; A -> \"a\";", "repetidas=[A]"),
    ] {
        let defs = declarations(SourceProfile::Es, mappings);
        for reversed in [false, true] {
            let mut units = vec![
                SourceUnit::new(&defs, "defs-es.svp", SourceProfile::Es),
                SourceUnit::new(CELL_EN, "cell-en.svp", SourceProfile::En),
            ];
            if reversed { units.reverse(); }
            reject(compile_svp_assembly(&units), reason, "CellSpec C");
        }
    }
}

#[test]
fn n0_02_cada_celda_comprueba_su_codominio_aunque_comparta_semantica() {
    let defs = declarations(SourceProfile::En, "A -> \"a\"; B -> \"b\";");
    let shared = format!("{defs}{CELL_EN}{}", CELL_EN.replace("C {", "D {"));
    assert!(compile_svp_profile(&shared, "shared.svp", SourceProfile::En).is_ok());
    let other = format!("codominio Other = {{ A, X }}; {}", CELL_ES.replace("C {", "D {").replace("codominio: K", "codominio: Other"));
    reject(compile_svp_assembly(&[
        SourceUnit::new(&format!("{defs}{CELL_EN}"), "first.svp", SourceProfile::En),
        SourceUnit::new(&other, "second.svp", SourceProfile::Es),
    ]), "ausentes=[X]; ajenas=[B]", "CellSpec D");
}

#[test]
fn n0_02_otra_semantica_no_completa_la_referenciada() {
    let source = format!("{}{CELL_EN} output_semantics Other {{ B -> \"b\"; }}", declarations(SourceProfile::En, "A -> \"a\";"));
    reject(compile_svp_profile(&source, "other.svp", SourceProfile::En), "ausentes=[B]", "CellSpec C");
}
