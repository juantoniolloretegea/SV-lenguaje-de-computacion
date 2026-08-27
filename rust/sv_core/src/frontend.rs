use crate::ir::construction;
use crate::identifier_profile::{is_identifier_continue, is_identifier_start, is_reserved_word};
use crate::{
    AdmissibilityState, IrObjectKind, IrOperationKind, IrProgram, IrQueryContext,
    IrSupervisableTarget, Nat, Tri,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrontendError {
    UnexpectedEnd,
    UnexpectedToken(String),
    Unsupported(String),
    InvalidNatural(String),
    InvalidAdmissibilityState(String),
    InvalidTri(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Word(String),
    Nat(String),
    Text(String),
    Sym(char),
    Arrow,
    Eof,
}

pub fn compile_svp(source: &str, source_file: &str) -> Result<IrProgram, FrontendError> {
    let tokens = tokenize(source)?;
    Parser::new(tokens, source, source_file).parse()
}

fn tokenize(source: &str) -> Result<Vec<Token>, FrontendError> {
    let bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];

        // Espacio léxico cerrado: SP, HT, CR y LF.
        if matches!(b, b' ' | b'\t' | b'\r' | b'\n') {
            i += 1;
            continue;
        }
        if b == b'-' && i + 1 < bytes.len() && bytes[i + 1] == b'-' {
            i += 2;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if b == b'-' && i + 1 < bytes.len() && bytes[i + 1] == b'>' {
            out.push(Token::Arrow);
            i += 2;
            continue;
        }
        if b == b'"' {
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i] != b'"' {
                i += 1;
            }
            if i >= bytes.len() {
                return Err(FrontendError::UnexpectedEnd);
            }
            let text = std::str::from_utf8(&bytes[start..i])
                .map_err(|_| FrontendError::UnexpectedToken("cadena no UTF-8".into()))?;
            out.push(Token::Text(text.to_owned()));
            i += 1;
            continue;
        }
        if b.is_ascii_digit() {
            let start = i;
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            out.push(Token::Nat(
                std::str::from_utf8(&bytes[start..i]).unwrap().to_owned(),
            ));
            continue;
        }

        let ch = source[i..]
            .chars()
            .next()
            .expect("i apunta a una frontera UTF-8 válida");
        if is_identifier_start(ch) {
            let start = i;
            i += ch.len_utf8();
            while i < bytes.len() {
                let next = source[i..]
                    .chars()
                    .next()
                    .expect("i apunta a una frontera UTF-8 válida");
                if !is_identifier_continue(next) {
                    break;
                }
                i += next.len_utf8();
            }
            out.push(Token::Word(source[start..i].to_owned()));
            continue;
        }

        if ch.is_ascii() && "{}[]();:,=.".contains(ch) {
            out.push(Token::Sym(ch));
            i += 1;
            continue;
        }
        return Err(FrontendError::UnexpectedToken(format!(
            "carácter léxico no admitido U+{:04X}",
            ch as u32
        )));
    }
    out.push(Token::Eof);
    Ok(out)
}

struct Parser<'a> {
    tokens: Vec<Token>,
    pos: usize,
    source: &'a str,
    source_file: &'a str,
    objects: Vec<crate::IrObject>,
    operations: Vec<crate::IrOperation>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token>, source: &'a str, source_file: &'a str) -> Self {
        Self {
            tokens,
            pos: 0,
            source,
            source_file,
            objects: Vec::new(),
            operations: Vec::new(),
        }
    }

    fn parse(mut self) -> Result<IrProgram, FrontendError> {
        while !matches!(self.peek(), Token::Eof) {
            match self.peek_word()?.to_owned().as_str() {
                "codomain" => self.parse_codomain()?,
                "output_semantics" => self.parse_output_semantics()?,
                "cellspec" => self.parse_cellspec()?,
                "coupledspec" => self.parse_coupledspec()?,
                "connector" => self.parse_connector()?,
                "admissibility_table" => self.parse_admissibility_table()?,
                "capture_spec" => self.parse_capture_spec()?,
                "admissibility_spec" => self.parse_admissibility_spec()?,
                "ternarizer" => self.parse_ternarizer()?,
                "res_spec" => self.parse_res_spec()?,
                "cellstate" => self.parse_cellstate()?,
                "coupledstate" => self.parse_coupledstate()?,
                "semantic_relation" => self.parse_semantic_relation()?,
                "pattern" => self.parse_pattern()?,
                "graph" => self.parse_graph()?,
                "horizon" => self.parse_horizon()?,
                "frame" => self.parse_frame()?,
                "transition_data" => self.parse_transition_data()?,
                "trajectory" => self.parse_trajectory()?,
                "domain" => self.parse_domain()?,
                "agent" => self.parse_agent()?,
                "query_spec" => self.parse_query_spec()?,
                "let" => self.parse_let()?,
                other => return Err(FrontendError::Unsupported(other.to_owned())),
            }
        }
        Ok(construction::program(
            self.source_file,
            sha256_hex(self.source.as_bytes()),
            self.objects,
            self.operations,
        ))
    }

    fn parse_codomain(&mut self) -> Result<(), FrontendError> {
        self.word("codomain")?;
        let name = self.take_word()?;
        self.sym('=')?;
        let values = self.word_set()?;
        self.sym(';')?;
        self.objects
            .push(construction::object(name, IrObjectKind::Codomain { values }));
        Ok(())
    }

    fn parse_output_semantics(&mut self) -> Result<(), FrontendError> {
        self.word("output_semantics")?;
        let name = self.take_word()?;
        self.sym('{')?;
        let mut mappings = Vec::new();
        while !self.at_sym('}') {
            let key = self.take_word()?;
            self.arrow()?;
            let value = self.take_text()?;
            self.sym(';')?;
            mappings.push((key, value));
        }
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::OutputSemantics { mappings },
        ));
        Ok(())
    }

    fn parse_cellspec(&mut self) -> Result<(), FrontendError> {
        self.word("cellspec")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("b")?;
        self.sym(':')?;
        let b = self.take_nat()?;
        self.sym(';')?;
        self.word("codomain")?;
        self.sym(':')?;
        let codomain = self.take_word()?;
        self.sym(';')?;
        self.word("semantics")?;
        self.sym(':')?;
        let semantics = self.take_word()?;
        self.sym(';')?;
        self.word("role")?;
        self.sym(':')?;
        let role = self.take_raw_word()?;
        self.sym(';')?;
        self.sym('}')?;
        let n = square_nat(&b)?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::CellSpec {
                b,
                n,
                codomain,
                semantics,
                role,
            },
        ));
        Ok(())
    }

    fn parse_coupledspec(&mut self) -> Result<(), FrontendError> {
        self.word("coupledspec")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("cell")?;
        self.sym(':')?;
        let cell = self.take_word()?;
        self.sym(';')?;
        self.word("bridges")?;
        self.sym(':')?;
        let bridges = self.nat_list()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::CoupledSpec { cell, bridges },
        ));
        Ok(())
    }

    fn parse_connector(&mut self) -> Result<(), FrontendError> {
        self.word("connector")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("source_codomain")?;
        self.sym(':')?;
        let source_codomain = self.take_word()?;
        self.sym(';')?;
        self.word("target_position")?;
        self.sym(':')?;
        let target_position = self.take_nat()?;
        self.sym(';')?;
        self.word("mapping")?;
        self.sym(':')?;
        self.sym('{')?;
        let mut mapping = Vec::new();
        while !self.at_sym('}') {
            let key = self.take_word()?;
            self.arrow()?;
            let value = self.take_tri()?;
            self.sym(';')?;
            mapping.push((key, value));
        }
        self.sym('}')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::Connector {
                source_codomain,
                target_position,
                mapping,
            },
        ));
        Ok(())
    }

    fn parse_admissibility_table(&mut self) -> Result<(), FrontendError> {
        self.word("admissibility_table")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("input_codomains")?;
        self.sym(':')?;
        let input_codomains = self.word_list()?;
        self.sym(';')?;
        self.word("output_codomain")?;
        self.sym(':')?;
        let output_codomain = self.take_word()?;
        self.sym(';')?;
        self.word("table")?;
        self.sym(':')?;
        self.sym('{')?;
        let mut table = Vec::new();
        while !self.at_sym('}') {
            let inputs = self.word_tuple()?;
            self.arrow()?;
            let output = self.take_word()?;
            self.sym(';')?;
            table.push((inputs, output));
        }
        self.sym('}')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::AdmissibilityTable {
                input_codomains,
                output_codomain,
                table,
            },
        ));
        Ok(())
    }

    fn parse_capture_spec(&mut self) -> Result<(), FrontendError> {
        self.word("capture_spec")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("parameter_id")?;
        self.sym(':')?;
        let parameter_id = self.take_nat()?;
        self.sym(';')?;
        self.word("observation_domain")?;
        self.sym(':')?;
        let observation_domain = self.take_word()?;
        self.sym(';')?;
        self.word("observation_space")?;
        self.sym(':')?;
        let observation_space = self.take_word()?;
        self.sym(';')?;
        self.word("failure_symbol")?;
        self.sym(':')?;
        let failure_symbol = self.take_raw_word()?;
        self.sym(';')?;
        self.word("mapping")?;
        self.sym(':')?;
        let mapping = self.take_word()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::CaptureSpec {
                parameter_id,
                observation_domain,
                observation_space,
                failure_symbol,
                mapping,
            },
        ));
        Ok(())
    }

    fn parse_admissibility_spec(&mut self) -> Result<(), FrontendError> {
        self.word("admissibility_spec")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("parameter_id")?;
        self.sym(':')?;
        let parameter_id = self.take_nat()?;
        self.sym(';')?;
        self.word("states")?;
        self.sym(':')?;
        self.sym('{')?;
        let mut states = Vec::new();
        loop {
            let label = self.take_word()?;
            let state = AdmissibilityState::try_from(label.as_str())
                .map_err(|_| FrontendError::InvalidAdmissibilityState(label))?;
            states.push(state);
            if self.at_sym(',') {
                self.sym(',')?;
            } else {
                break;
            }
        }
        self.sym('}')?;
        self.sym(';')?;
        if states.len() != 3 {
            return Err(FrontendError::InvalidAdmissibilityState(format!(
                "{} estados",
                states.len()
            )));
        }
        self.word("rule")?;
        self.sym(':')?;
        let rule = self.take_word()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::AdmissibilitySpec {
                parameter_id,
                states: [states[0], states[1], states[2]],
                rule,
            },
        ));
        Ok(())
    }

    fn parse_ternarizer(&mut self) -> Result<(), FrontendError> {
        self.word("ternarizer")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("observation_space")?;
        self.sym(':')?;
        let observation_space = self.take_word()?;
        self.sym(';')?;
        self.word("partition_zero")?;
        self.sym(':')?;
        let partition_zero = self.take_word()?;
        self.sym(';')?;
        self.word("partition_one")?;
        self.sym(':')?;
        let partition_one = self.take_word()?;
        self.sym(';')?;
        self.word("partition_u")?;
        self.sym(':')?;
        let partition_u = self.take_word()?;
        self.sym(';')?;
        self.word("mapping")?;
        self.sym(':')?;
        let mapping = self.take_word()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::Ternarizer {
                observation_space,
                partition_zero,
                partition_one,
                partition_u,
                mapping,
            },
        ));
        Ok(())
    }

    fn parse_res_spec(&mut self) -> Result<(), FrontendError> {
        self.word("res_spec")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("context")?;
        self.sym(':')?;
        let context = self.take_word()?;
        self.sym(';')?;
        self.word("mechanism")?;
        self.sym(':')?;
        let mechanism = self.take_word()?;
        self.sym(';')?;
        self.word("mapping")?;
        self.sym(':')?;
        let mapping = self.take_word()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::ResSpec {
                context,
                mechanism,
                mapping,
            },
        ));
        Ok(())
    }

    fn parse_cellstate(&mut self) -> Result<(), FrontendError> {
        self.word("cellstate")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("spec")?;
        self.sym(':')?;
        let spec = self.take_word()?;
        self.sym(';')?;
        self.word("vector")?;
        self.sym(':')?;
        let vector = self.tri_list()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::CellState { spec, vector },
        ));
        Ok(())
    }

    fn parse_coupledstate(&mut self) -> Result<(), FrontendError> {
        self.word("coupledstate")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("spec")?;
        self.sym(':')?;
        let spec = self.take_word()?;
        self.sym(';')?;
        self.word("base_vector")?;
        self.sym(':')?;
        let base_vector = self.tri_list()?;
        self.sym(';')?;
        self.word("updated_vector")?;
        self.sym(':')?;
        let updated_vector = self.tri_list()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::CoupledState {
                spec,
                base_vector,
                updated_vector,
            },
        ));
        Ok(())
    }

    fn parse_semantic_relation(&mut self) -> Result<(), FrontendError> {
        self.word("semantic_relation")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("kind")?;
        self.sym(':')?;
        let kind = self.take_raw_word()?;
        self.sym(';')?;
        let mut table = None;
        let mut constraints = None;
        while !self.at_sym('}') {
            let field = self.take_word()?;
            self.sym(':')?;
            match field.as_str() {
                "table" => table = Some(self.take_word()?),
                "constraints" => constraints = Some(self.word_list()?),
                other => return Err(FrontendError::Unsupported(other.to_owned())),
            }
            self.sym(';')?;
        }
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::SemanticRelation {
                kind,
                table,
                constraints,
            },
        ));
        Ok(())
    }

    fn parse_pattern(&mut self) -> Result<(), FrontendError> {
        self.word("pattern")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("kind")?;
        self.sym(':')?;
        let kind = self.take_raw_word()?;
        self.sym(';')?;
        let mut arity = None;
        let mut constraints = None;
        while !self.at_sym('}') {
            let field = self.take_word()?;
            self.sym(':')?;
            match field.as_str() {
                "arity" => arity = Some(self.take_nat()?),
                "constraints" => constraints = Some(self.word_list()?),
                other => return Err(FrontendError::Unsupported(other.to_owned())),
            }
            self.sym(';')?;
        }
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::Pattern {
                kind,
                arity,
                constraints,
            },
        ));
        Ok(())
    }

    fn parse_graph(&mut self) -> Result<(), FrontendError> {
        self.word("graph")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("nodes")?;
        self.sym(':')?;
        let nodes = self.word_list()?;
        self.sym(';')?;
        self.word("edges")?;
        self.sym(':')?;
        let edges = self.edge_list()?;
        self.sym(';')?;
        self.word("relation")?;
        self.sym(':')?;
        let relation = self.take_word()?;
        self.sym(';')?;
        self.word("regime")?;
        self.sym(':')?;
        let regime = self.take_raw_word()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::CompositionGraph {
                nodes,
                edges,
                relation,
                regime,
            },
        ));
        Ok(())
    }

    fn parse_horizon(&mut self) -> Result<(), FrontendError> {
        self.word("horizon")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("architecture")?;
        self.sym(':')?;
        let architecture = self.take_word()?;
        self.sym(';')?;
        self.word("events")?;
        self.sym(':')?;
        let events = self.word_list()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::Horizon {
                architecture,
                events,
            },
        ));
        Ok(())
    }

    fn parse_frame(&mut self) -> Result<(), FrontendError> {
        self.word("frame")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("index")?;
        self.sym(':')?;
        let index = self.take_nat()?;
        self.sym(';')?;
        self.word("architecture")?;
        self.sym(':')?;
        let architecture = self.take_word()?;
        self.sym(';')?;
        self.word("cell_states")?;
        self.sym(':')?;
        let cell_states = self.word_list()?;
        self.sym(';')?;
        self.word("eval_results")?;
        self.sym(':')?;
        let eval_results = self.word_list()?;
        self.sym(';')?;
        self.word("gate_results")?;
        self.sym(':')?;
        let gate_results = self.word_list()?;
        self.sym(';')?;
        self.word("supervision")?;
        self.sym(':')?;
        let supervision = self.word_list()?;
        self.sym(';')?;
        self.word("criticalities")?;
        self.sym(':')?;
        let criticalities = self.word_list()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::Frame {
                index,
                architecture,
                cell_states,
                eval_results,
                gate_results,
                supervision,
                criticalities,
            },
        ));
        Ok(())
    }

    fn parse_transition_data(&mut self) -> Result<(), FrontendError> {
        self.word("transition_data")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("horizon_ref")?;
        self.sym(':')?;
        let horizon_ref = self.take_word()?;
        self.sym(';')?;
        self.word("events")?;
        self.sym(':')?;
        let events = self.event_list()?;
        self.sym(';')?;
        self.word("induced_parameters")?;
        self.sym(':')?;
        let induced_parameters = self.induced_parameter_list()?;
        self.sym(';')?;
        let mut metadata = None;
        if !self.at_sym('}') {
            self.word("metadata")?;
            self.sym(':')?;
            metadata = Some(self.word_list()?);
            self.sym(';')?;
        }
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::TransitionData {
                horizon_ref,
                events,
                induced_parameters,
                metadata,
            },
        ));
        Ok(())
    }

    fn parse_trajectory(&mut self) -> Result<(), FrontendError> {
        self.word("trajectory")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("entries")?;
        self.sym(':')?;
        let entries = self.trajectory_entries()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::Trajectory { entries },
        ));
        Ok(())
    }

    fn parse_domain(&mut self) -> Result<(), FrontendError> {
        self.word("domain")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("parameters")?;
        self.sym(':')?;
        let parameters = self.word_list()?;
        self.sym(';')?;
        self.word("interface")?;
        self.sym(':')?;
        let interface = self.take_word()?;
        self.sym(';')?;
        self.word("horizon")?;
        self.sym(':')?;
        let horizon = self.take_word()?;
        self.sym(';')?;
        self.word("capture_specs")?;
        self.sym(':')?;
        let capture_specs = self.word_list()?;
        self.sym(';')?;
        self.word("admissibility_specs")?;
        self.sym(':')?;
        let admissibility_specs = self.word_list()?;
        self.sym(';')?;
        self.word("ternarizers")?;
        self.sym(':')?;
        let ternarizers = self.word_list()?;
        self.sym(';')?;
        self.word("exogeneity_mask")?;
        self.sym(':')?;
        let exogeneity_mask = self.take_word()?;
        self.sym(';')?;
        self.word("silent_u")?;
        self.sym(':')?;
        let silent_u = self.take_word()?;
        self.sym(';')?;
        self.word("transduction_policy")?;
        self.sym(':')?;
        let transduction_policy = self.take_word()?;
        self.sym(';')?;
        self.word("u_policy")?;
        self.sym(':')?;
        let u_policy = self.take_word()?;
        self.sym(';')?;
        self.word("closure_criterion")?;
        self.sym(':')?;
        let closure_criterion = self.take_word()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::Domain {
                parameters,
                interface,
                horizon,
                capture_specs,
                admissibility_specs,
                ternarizers,
                exogeneity_mask,
                silent_u,
                transduction_policy,
                u_policy,
                closure_criterion,
            },
        ));
        Ok(())
    }

    fn parse_agent(&mut self) -> Result<(), FrontendError> {
        self.word("agent")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("architecture")?;
        self.sym(':')?;
        let architecture = self.take_word()?;
        self.sym(';')?;
        self.word("domain")?;
        self.sym(':')?;
        let domain = self.take_word()?;
        self.sym(';')?;
        self.word("query_engine")?;
        self.sym(':')?;
        let query_engine = self.take_word()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::Agent {
                architecture,
                domain,
                query_engine,
            },
        ));
        Ok(())
    }

    fn parse_query_spec(&mut self) -> Result<(), FrontendError> {
        self.word("query_spec")?;
        let name = self.take_word()?;
        self.sym('{')?;
        self.word("query_type")?;
        self.sym(':')?;
        let query_type = self.take_raw_word()?;
        self.sym(';')?;
        self.word("scope")?;
        self.sym(':')?;
        let scope = self.take_raw_word()?;
        self.sym(';')?;
        self.word("restrictions")?;
        self.sym(':')?;
        let restrictions = self.word_list()?;
        self.sym(';')?;
        self.sym('}')?;
        self.objects.push(construction::object(
            name,
            IrObjectKind::QuerySpec {
                query_type,
                scope,
                restrictions,
            },
        ));
        Ok(())
    }

    fn parse_let(&mut self) -> Result<(), FrontendError> {
        self.word("let")?;
        let name = self.take_word()?;
        self.sym('=')?;
        let first = self.take_raw_word()?;
        match first.as_str() {
            "evaluate" => {
                self.sym('(')?;
                let state = self.take_word()?;
                self.sym(')')?;
                self.sym(';')?;
                self.operations.push(construction::operation(
                    name,
                    IrOperationKind::Evaluate { state },
                ));
            }
            "gate" => {
                self.sym('(')?;
                let eval_results = self.word_list()?;
                self.sym(',')?;
                self.word("using")?;
                self.sym(':')?;
                let table = self.take_word()?;
                self.sym(')')?;
                self.sym(';')?;
                self.operations.push(construction::operation(
                    name,
                    IrOperationKind::Gate {
                        eval_results,
                        table,
                    },
                ));
            }
            "resolve" => {
                self.sym('(')?;
                self.sym('(')?;
                let target_state = self.take_word()?;
                self.sym(',')?;
                let target_position = self.take_nat()?;
                self.sym(')')?;
                self.sym(',')?;
                self.word("with")?;
                self.sym(':')?;
                let with_spec = self.take_word()?;
                self.sym(',')?;
                self.word("context")?;
                self.sym(':')?;
                let context_instance = self.take_word()?;
                self.sym(',')?;
                self.word("mechanism")?;
                self.sym(':')?;
                let mechanism_instance = self.take_word()?;
                self.sym(')')?;
                self.sym(';')?;
                self.operations.push(construction::operation(
                    name,
                    IrOperationKind::Resolve {
                        target_state,
                        target_position,
                        with_spec,
                        context_instance,
                        mechanism_instance,
                    },
                ));
            }
            "supervise" => {
                self.sym('(')?;
                let meta_eval = self.take_word()?;
                self.sym(',')?;
                self.word("target")?;
                self.sym(':')?;
                let variant = self.take_raw_word()?;
                self.sym('(')?;
                let reference = self.take_word()?;
                self.sym(')')?;
                self.sym(')')?;
                self.sym(';')?;
                let target = match variant.as_str() {
                    "CellTarget" => IrSupervisableTarget::Cell { reference },
                    "ComposedTarget" => IrSupervisableTarget::Composed { reference },
                    "SystemTarget" => IrSupervisableTarget::System { reference },
                    other => return Err(FrontendError::Unsupported(other.to_owned())),
                };
                self.operations.push(construction::operation(
                    name,
                    IrOperationKind::Supervise { meta_eval, target },
                ));
            }
            "compose" => {
                self.sym('(')?;
                let graph = self.take_word()?;
                self.sym(',')?;
                self.word("relations")?;
                self.sym(':')?;
                let relations = self.word_list()?;
                self.sym(',')?;
                self.word("patterns")?;
                self.sym(':')?;
                let patterns = self.word_list()?;
                self.sym(')')?;
                self.sym(';')?;
                self.operations.push(construction::operation(
                    name,
                    IrOperationKind::Compose {
                        graph,
                        relations,
                        patterns,
                    },
                ));
            }
            "query" => {
                self.sym('(')?;
                let spec = self.take_word()?;
                self.sym(',')?;
                self.word("by")?;
                self.sym(':')?;
                let by = self.take_word()?;
                self.sym(',')?;
                self.word("in")?;
                self.sym(':')?;
                let context = self.query_context()?;
                self.sym(')')?;
                self.sym(';')?;
                self.operations.push(construction::operation(
                    name,
                    IrOperationKind::Query { spec, by, context },
                ));
            }
            source => {
                if is_reserved_word(source) {
                    return Err(FrontendError::UnexpectedToken(format!(
                        "palabra reservada donde se esperaba identificador: {source}"
                    )));
                }
                self.sym('.')?;
                let field = self.take_word()?;
                self.sym(';')?;
                self.operations.push(construction::operation(
                    name,
                    IrOperationKind::Projection {
                        source: source.to_owned(),
                        field,
                    },
                ));
            }
        }
        Ok(())
    }

    fn query_context(&mut self) -> Result<IrQueryContext, FrontendError> {
        let variant = self.take_raw_word()?;
        self.sym('(')?;
        let context = match variant.as_str() {
            "PointEval" => IrQueryContext::PointEval {
                reference: self.take_word()?,
            },
            "TrajectoryView" => IrQueryContext::TrajectoryView {
                reference: self.take_word()?,
            },
            "FrameComparison" => {
                let a = self.take_word()?;
                self.sym(',')?;
                let b = self.take_word()?;
                IrQueryContext::FrameComparison { references: [a, b] }
            }
            "ArchitectureView" => {
                let architecture = self.take_word()?;
                self.sym(',')?;
                let cells = self.word_list()?;
                self.sym(',')?;
                let evals = self.word_list()?;
                self.sym(',')?;
                let gates = self.word_list()?;
                IrQueryContext::ArchitectureView {
                    architecture,
                    cells,
                    evals,
                    gates,
                }
            }
            "CoverageReport" => {
                let a = self.take_word()?;
                self.sym(',')?;
                let b = self.take_word()?;
                self.sym(',')?;
                let c = self.take_word()?;
                IrQueryContext::CoverageReport {
                    references: [a, b, c],
                }
            }
            other => return Err(FrontendError::Unsupported(other.to_owned())),
        };
        self.sym(')')?;
        Ok(context)
    }

    fn edge_list(&mut self) -> Result<Vec<(String, String, Nat, String)>, FrontendError> {
        self.sym('[')?;
        let mut out = Vec::new();
        if !self.at_sym(']') {
            loop {
                self.word("edge")?;
                self.sym('(')?;
                self.word("source")?;
                self.sym(':')?;
                let source = self.take_word()?;
                self.sym(',')?;
                self.word("target")?;
                self.sym(':')?;
                let target = self.take_word()?;
                self.sym(',')?;
                self.word("position")?;
                self.sym(':')?;
                let position = self.take_nat()?;
                self.sym(',')?;
                self.word("connector")?;
                self.sym(':')?;
                let connector = self.take_word()?;
                self.sym(')')?;
                out.push((source, target, position, connector));
                if self.at_sym(',') {
                    self.sym(',')?;
                } else {
                    break;
                }
            }
        }
        self.sym(']')?;
        Ok(out)
    }

    fn event_list(&mut self) -> Result<Vec<(String, Tri)>, FrontendError> {
        self.sym('[')?;
        let mut out = Vec::new();
        if !self.at_sym(']') {
            loop {
                self.sym('(')?;
                let event = self.take_word()?;
                self.sym(',')?;
                let state = self.take_tri()?;
                self.sym(')')?;
                out.push((event, state));
                if self.at_sym(',') {
                    self.sym(',')?;
                } else {
                    break;
                }
            }
        }
        self.sym(']')?;
        Ok(out)
    }

    fn induced_parameter_list(&mut self) -> Result<Vec<(String, Nat, Tri)>, FrontendError> {
        self.sym('[')?;
        let mut out = Vec::new();
        if !self.at_sym(']') {
            loop {
                self.sym('(')?;
                let cell_ref = self.take_word()?;
                self.sym(',')?;
                let position = self.take_nat()?;
                self.sym(',')?;
                let value = self.take_tri()?;
                self.sym(')')?;
                out.push((cell_ref, position, value));
                if self.at_sym(',') {
                    self.sym(',')?;
                } else {
                    break;
                }
            }
        }
        self.sym(']')?;
        Ok(out)
    }

    fn trajectory_entries(&mut self) -> Result<Vec<(String, Option<String>)>, FrontendError> {
        self.sym('[')?;
        let mut out = Vec::new();
        if !self.at_sym(']') {
            loop {
                self.word("entry")?;
                self.sym('(')?;
                self.word("frame")?;
                self.sym(':')?;
                let frame = self.take_word()?;
                let transition = if self.at_sym(',') {
                    self.sym(',')?;
                    self.word("transition")?;
                    self.sym(':')?;
                    Some(self.take_word()?)
                } else {
                    None
                };
                self.sym(')')?;
                out.push((frame, transition));
                if self.at_sym(',') {
                    self.sym(',')?;
                } else {
                    break;
                }
            }
        }
        self.sym(']')?;
        Ok(out)
    }

    fn word_tuple(&mut self) -> Result<Vec<String>, FrontendError> {
        self.sym('(')?;
        let mut out = Vec::new();
        loop {
            out.push(self.take_word()?);
            if self.at_sym(',') {
                self.sym(',')?;
            } else {
                break;
            }
        }
        self.sym(')')?;
        Ok(out)
    }

    fn word_set(&mut self) -> Result<Vec<String>, FrontendError> {
        self.sym('{')?;
        let mut values = Vec::new();
        if !self.at_sym('}') {
            loop {
                values.push(self.take_word()?);
                if self.at_sym(',') {
                    self.sym(',')?;
                } else {
                    break;
                }
            }
        }
        self.sym('}')?;
        Ok(values)
    }

    fn word_list(&mut self) -> Result<Vec<String>, FrontendError> {
        self.sym('[')?;
        let mut values = Vec::new();
        if !self.at_sym(']') {
            loop {
                values.push(self.take_word()?);
                if self.at_sym(',') {
                    self.sym(',')?;
                } else {
                    break;
                }
            }
        }
        self.sym(']')?;
        Ok(values)
    }

    fn nat_list(&mut self) -> Result<Vec<Nat>, FrontendError> {
        self.sym('[')?;
        let mut values = Vec::new();
        if !self.at_sym(']') {
            loop {
                values.push(self.take_nat()?);
                if self.at_sym(',') {
                    self.sym(',')?;
                } else {
                    break;
                }
            }
        }
        self.sym(']')?;
        Ok(values)
    }

    fn tri_list(&mut self) -> Result<Vec<Tri>, FrontendError> {
        self.sym('[')?;
        let mut values = Vec::new();
        if !self.at_sym(']') {
            loop {
                values.push(self.take_tri()?);
                if self.at_sym(',') {
                    self.sym(',')?;
                } else {
                    break;
                }
            }
        }
        self.sym(']')?;
        Ok(values)
    }

    fn take_tri(&mut self) -> Result<Tri, FrontendError> {
        let label = self.take_raw_word()?;
        match label.as_str() {
            "Zero" => Ok(Tri::Zero),
            "One" => Ok(Tri::One),
            "U" => Ok(Tri::U),
            _ => Err(FrontendError::InvalidTri(label)),
        }
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.pos)
            .expect("el token EOF garantiza una posición válida")
    }

    fn peek_word(&self) -> Result<&str, FrontendError> {
        match self.peek() {
            Token::Word(value) => Ok(value),
            Token::Eof => Err(FrontendError::UnexpectedEnd),
            other => Err(FrontendError::UnexpectedToken(format!("{other:?}"))),
        }
    }

    fn take_raw_word(&mut self) -> Result<String, FrontendError> {
        match self.tokens.get(self.pos).cloned() {
            Some(Token::Word(value)) => {
                self.pos += 1;
                Ok(value)
            }
            Some(Token::Eof) | None => Err(FrontendError::UnexpectedEnd),
            Some(other) => Err(FrontendError::UnexpectedToken(format!("{other:?}"))),
        }
    }

    fn take_word(&mut self) -> Result<String, FrontendError> {
        let value = self.take_raw_word()?;
        if is_reserved_word(&value) {
            Err(FrontendError::UnexpectedToken(format!(
                "palabra reservada donde se esperaba identificador: {value}"
            )))
        } else {
            Ok(value)
        }
    }

    fn take_text(&mut self) -> Result<String, FrontendError> {
        match self.tokens.get(self.pos).cloned() {
            Some(Token::Text(value)) => {
                self.pos += 1;
                Ok(value)
            }
            Some(Token::Eof) | None => Err(FrontendError::UnexpectedEnd),
            Some(other) => Err(FrontendError::UnexpectedToken(format!("{other:?}"))),
        }
    }

    fn take_nat(&mut self) -> Result<Nat, FrontendError> {
        match self.tokens.get(self.pos).cloned() {
            Some(Token::Nat(value)) => {
                self.pos += 1;
                Nat::from_decimal(&value).map_err(|_| FrontendError::InvalidNatural(value))
            }
            Some(Token::Eof) | None => Err(FrontendError::UnexpectedEnd),
            Some(other) => Err(FrontendError::UnexpectedToken(format!("{other:?}"))),
        }
    }

    fn word(&mut self, expected: &str) -> Result<(), FrontendError> {
        let got = self.take_raw_word()?;
        if got == expected {
            Ok(())
        } else {
            Err(FrontendError::UnexpectedToken(format!(
                "esperado {expected}, recibido {got}"
            )))
        }
    }

    fn sym(&mut self, expected: char) -> Result<(), FrontendError> {
        match self.tokens.get(self.pos).cloned() {
            Some(Token::Sym(got)) if got == expected => {
                self.pos += 1;
                Ok(())
            }
            Some(Token::Eof) | None => Err(FrontendError::UnexpectedEnd),
            Some(other) => Err(FrontendError::UnexpectedToken(format!(
                "esperado {expected}, recibido {other:?}"
            ))),
        }
    }

    fn arrow(&mut self) -> Result<(), FrontendError> {
        match self.tokens.get(self.pos) {
            Some(Token::Arrow) => {
                self.pos += 1;
                Ok(())
            }
            Some(Token::Eof) | None => Err(FrontendError::UnexpectedEnd),
            Some(other) => Err(FrontendError::UnexpectedToken(format!(
                "esperado ->, recibido {other:?}"
            ))),
        }
    }

    fn at_sym(&self, ch: char) -> bool {
        matches!(self.peek(), Token::Sym(got) if *got == ch)
    }
}

fn square_nat(value: &Nat) -> Result<Nat, FrontendError> {
    let digits = value.as_decimal().as_bytes();
    if digits == b"0" {
        return Ok(Nat::from_u64(0));
    }
    let mut acc = vec![0u32; digits.len() * 2];
    for (i, a) in digits.iter().rev().enumerate() {
        let da = (*a - b'0') as u32;
        for (j, b) in digits.iter().rev().enumerate() {
            let db = (*b - b'0') as u32;
            acc[i + j] += da * db;
        }
    }
    for i in 0..acc.len() - 1 {
        let carry = acc[i] / 10;
        acc[i] %= 10;
        acc[i + 1] += carry;
    }
    while acc.last() == Some(&0) {
        acc.pop();
    }
    let text: String = acc
        .iter()
        .rev()
        .map(|d| char::from(b'0' + (*d as u8)))
        .collect();
    Nat::from_decimal(&text).map_err(|_| FrontendError::InvalidNatural(text))
}

fn sha256_hex(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
        0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
        0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
        0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
        0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
        0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
        0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
        0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2,
    ];
    let mut msg = data.to_vec();
    let bit_len = (msg.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    let mut h = [
        0x6a09e667u32,0xbb67ae85,0x3c6ef372,0xa54ff53a,
        0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19,
    ];
    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i*4], chunk[i*4+1], chunk[i*4+2], chunk[i*4+3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ (w[i-15] >> 3);
            let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ (w[i-2] >> 10);
            w[i] = w[i-16]
                .wrapping_add(s0)
                .wrapping_add(w[i-7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    let mut out = String::with_capacity(64);
    for word in h {
        use std::fmt::Write;
        write!(&mut out, "{word:08x}").unwrap();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_matches_known_vector() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn compiles_all_current_valid_cases_from_actual_svp_text() {
        let cases = [
            ("admissibility_spec_states_permutados.svp", include_str!("../../../tests/conformance/valid/admissibility_spec_states_permutados.svp")),
            ("cell_basic.svp", include_str!("../../../tests/conformance/valid/cell_basic.svp")),
            ("compose_basic.svp", include_str!("../../../tests/conformance/valid/compose_basic.svp")),
            ("frame_cell_spec_compartida_valida.svp", include_str!("../../../tests/conformance/valid/frame_cell_spec_compartida_valida.svp")),
            ("gate_table.svp", include_str!("../../../tests/conformance/valid/gate_table.svp")),
            ("identificadores_espanol.svp", include_str!("../../../tests/conformance/valid/identificadores_espanol.svp")),
            ("query_context_all_variants.svp", include_str!("../../../tests/conformance/valid/query_context_all_variants.svp")),
            ("resolve_projection.svp", include_str!("../../../tests/conformance/valid/resolve_projection.svp")),
            ("supervise_systemtarget_valido.svp", include_str!("../../../tests/conformance/valid/supervise_systemtarget_valido.svp")),
            ("supervise_targets.svp", include_str!("../../../tests/conformance/valid/supervise_targets.svp")),
            ("trajectory_alternance_valid.svp", include_str!("../../../tests/conformance/valid/trajectory_alternance_valid.svp")),
            ("transition_data_events.svp", include_str!("../../../tests/conformance/valid/transition_data_events.svp")),
        ];
        for (name, source) in cases {
            compile_svp(source, name).unwrap_or_else(|error| panic!("{name}: {error:?}"));
        }
    }

    #[test]
    fn query_context_variants_are_materialized_as_closed_ir_variants() {
        let source = include_str!("../../../tests/conformance/valid/query_context_all_variants.svp");
        let ir = compile_svp(source, "query_context_all_variants.svp").expect("SVP canónico válido");
        let queries = ir.operations().iter().filter_map(|op| match op.kind() {
            IrOperationKind::Query { context, .. } => Some(context.variant_label()),
            _ => None,
        }).collect::<Vec<_>>();
        assert_eq!(queries, ["PointEval", "TrajectoryView", "FrameComparison", "ArchitectureView", "CoverageReport"]);
    }
}
