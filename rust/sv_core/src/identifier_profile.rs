//! Perfil léxico cerrado de identificadores SVP para la Gramática 0.2.
//!
//! El conjunto es deliberadamente finito y no depende de las propiedades
//! Unicode suministradas por la biblioteca estándar o por el sistema anfitrión.

pub(crate) const PROFILE_ID: &str = "svp-grammar-0.2-lex-es-1";

pub(crate) fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic()
        || matches!(
            ch,
            'Á' | 'É' | 'Í' | 'Ó' | 'Ú' | 'Ü' | 'Ñ'
                | 'á' | 'é' | 'í' | 'ó' | 'ú' | 'ü' | 'ñ'
        )
}

pub(crate) fn is_identifier_continue(ch: char) -> bool {
    is_identifier_start(ch) || ch == '_' || ch.is_ascii_digit()
}


pub(crate) fn is_reserved_word(word: &str) -> bool {
    matches!(
        word,
        "codomain"
            | "output_semantics"
            | "cellspec"
            | "coupledspec"
            | "connector"
            | "admissibility_table"
            | "capture_spec"
            | "admissibility_spec"
            | "ternarizer"
            | "res_spec"
            | "cellstate"
            | "coupledstate"
            | "graph"
            | "semantic_relation"
            | "pattern"
            | "horizon"
            | "frame"
            | "transition_data"
            | "trajectory"
            | "domain"
            | "agent"
            | "query_spec"
            | "let"
            | "evaluate"
            | "gate"
            | "resolve"
            | "query"
            | "supervise"
            | "compose"
            | "using"
            | "with"
            | "context"
            | "mechanism"
            | "by"
            | "in"
            | "target"
            | "relations"
            | "patterns"
            | "Base"
            | "Supervisor"
            | "Composite"
            | "Simple"
            | "General"
            | "CellTarget"
            | "ComposedTarget"
            | "SystemTarget"
            | "PointEval"
            | "TrajectoryView"
            | "FrameComparison"
            | "ArchitectureView"
            | "CoverageReport"
            | "PointEvaluation"
            | "TrajectoryState"
            | "CoverageState"
            | "PendingU"
            | "GlobalCriticality"
            | "Cell"
            | "Pair"
            | "Architecture"
            | "Trajectory"
            | "DeclaredRelation"
            | "DeclaredPattern"
            | "edge"
            | "entry"
            | "Bottom"
            | "True"
            | "Zero"
            | "One"
            | "U"
            | "max"
            | "min"
            | "null"
            | "None"
            | "NaN"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cubre_la_ortografia_espanola_del_perfil() {
        for ch in ['ñ', 'Ñ', 'á', 'é', 'í', 'ó', 'ú', 'ü', 'Á', 'É', 'Í', 'Ó', 'Ú', 'Ü'] {
            assert!(is_identifier_start(ch));
        }
    }

    #[test]
    fn las_palabras_reservadas_no_son_identificadores() {
        for word in ["frame", "let", "Zero", "CellTarget", "max"] {
            assert!(is_reserved_word(word));
        }
        for word in ["Marco", "Señal", "Regla_1"] {
            assert!(!is_reserved_word(word));
        }
    }

    #[test]
    fn excluye_formas_no_canónicas_y_otros_alfabetos() {
        for ch in ['_', 'α', 'С', 'ö', '́', '٣'] {
            assert!(!is_identifier_start(ch));
        }
        assert!(is_identifier_continue('_'));
        assert!(is_identifier_continue('7'));
        assert!(!is_identifier_continue('٣'));
    }
}
