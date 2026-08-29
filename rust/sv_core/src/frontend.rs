use crate::ir::construction;
use crate::identifier_profile::{is_identifier_continue, is_identifier_start};
use crate::{
    AdmissibilityState, IrObjectKind, IrOperationKind, IrProgram, IrQueryContext,
    IrSupervisableTarget, Nat, Tri,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceProfile { En, Es }

impl SourceProfile {
    pub const fn tag(self) -> &'static str {
        match self { Self::En => "en", Self::Es => "es" }
    }

    pub fn from_tag(tag: &str) -> Option<Self> {
        match tag { "en" => Some(Self::En), "es" => Some(Self::Es), _ => None }
    }

    pub const fn abi_code(self) -> u32 {
        match self { Self::En => 0, Self::Es => 1 }
    }

    pub const fn from_abi_code(code: u32) -> Option<Self> {
        match code { 0 => Some(Self::En), 1 => Some(Self::Es), _ => None }
    }
}

const CANONICAL_FORMS: [&str; 154] = ["codomain","output_semantics","cellspec","coupledspec","connector","admissibility_table","capture_spec","admissibility_spec","ternarizer","res_spec","cellstate","coupledstate","graph","semantic_relation","pattern","horizon","frame","transition_data","trajectory","domain","agent","query_spec","let","evaluate","gate","resolve","query","supervise","compose","using","with","context","mechanism","by","in","target","relations","patterns","Base","Supervisor","Composite","Simple","General","CellTarget","ComposedTarget","SystemTarget","PointEval","TrajectoryView","FrameComparison","ArchitectureView","CoverageReport","PointEvaluation","TrajectoryState","CoverageState","PendingU","GlobalCriticality","Cell","Pair","Architecture","Trajectory","DeclaredRelation","DeclaredPattern","edge","entry","Bottom","True","Zero","One","U","max","min","null","None","NaN","admissibility_specs","architecture","arity","b","base_vector","bridges","capture_specs","cell","cell_states","closure_criterion","constraints","criticalities","edges","entries","eval_results","events","exogeneity_mask","failure_symbol","gate_results","horizon_ref","index","induced_parameters","input_codomains","interface","kind","mapping","metadata","nodes","observation_domain","observation_space","output_codomain","parameter_id","parameters","partition_one","partition_u","partition_zero","position","query_engine","query_type","regime","relation","restrictions","role","rule","scope","semantics","silent_u","source","source_codomain","spec","states","supervision","table","target_position","ternarizers","transduction_policy","transition","u_policy","updated_vector","vector","Ok","Degraded","NotAdmitted","source_state","counts","threshold","classification","criticality","deltas","inputs","output","previous","reviewed_to","resolved_to","context_ref","mechanism_ref","response","justification","meta_eval","verdict"];
const STATUS_FOREIGN_CONTEXTUAL: u8 = 253;
const STATUS_FOREIGN_PROTECTED: u8 = 254;
const STATUS_OTHER: u8 = 255;
const FOREIGN_SURFACE_SENTINEL: &str = "__SVP_FOREIGN_SURFACE__";

#[cfg(test)]
#[inline]
fn lookup_en(word: &str) -> Option<u8> {
    match word {
        "codomain" => Some(0u8),
        "output_semantics" => Some(1u8),
        "cellspec" => Some(2u8),
        "coupledspec" => Some(3u8),
        "connector" => Some(4u8),
        "admissibility_table" => Some(5u8),
        "capture_spec" => Some(6u8),
        "admissibility_spec" => Some(7u8),
        "ternarizer" => Some(8u8),
        "res_spec" => Some(9u8),
        "cellstate" => Some(10u8),
        "coupledstate" => Some(11u8),
        "graph" => Some(12u8),
        "semantic_relation" => Some(13u8),
        "pattern" => Some(14u8),
        "horizon" => Some(15u8),
        "frame" => Some(16u8),
        "transition_data" => Some(17u8),
        "trajectory" => Some(18u8),
        "domain" => Some(19u8),
        "agent" => Some(20u8),
        "query_spec" => Some(21u8),
        "let" => Some(22u8),
        "evaluate" => Some(23u8),
        "gate" => Some(24u8),
        "resolve" => Some(25u8),
        "query" => Some(26u8),
        "supervise" => Some(27u8),
        "compose" => Some(28u8),
        "using" => Some(29u8),
        "with" => Some(30u8),
        "context" => Some(31u8),
        "mechanism" => Some(32u8),
        "by" => Some(33u8),
        "in" => Some(34u8),
        "target" => Some(35u8),
        "relations" => Some(36u8),
        "patterns" => Some(37u8),
        "Base" => Some(38u8),
        "Supervisor" => Some(39u8),
        "Composite" => Some(40u8),
        "Simple" => Some(41u8),
        "General" => Some(42u8),
        "CellTarget" => Some(43u8),
        "ComposedTarget" => Some(44u8),
        "SystemTarget" => Some(45u8),
        "PointEval" => Some(46u8),
        "TrajectoryView" => Some(47u8),
        "FrameComparison" => Some(48u8),
        "ArchitectureView" => Some(49u8),
        "CoverageReport" => Some(50u8),
        "PointEvaluation" => Some(51u8),
        "TrajectoryState" => Some(52u8),
        "CoverageState" => Some(53u8),
        "PendingU" => Some(54u8),
        "GlobalCriticality" => Some(55u8),
        "Cell" => Some(56u8),
        "Pair" => Some(57u8),
        "Architecture" => Some(58u8),
        "Trajectory" => Some(59u8),
        "DeclaredRelation" => Some(60u8),
        "DeclaredPattern" => Some(61u8),
        "edge" => Some(62u8),
        "entry" => Some(63u8),
        "Bottom" => Some(64u8),
        "True" => Some(65u8),
        "Zero" => Some(66u8),
        "One" => Some(67u8),
        "U" => Some(68u8),
        "max" => Some(69u8),
        "min" => Some(70u8),
        "null" => Some(71u8),
        "None" => Some(72u8),
        "NaN" => Some(73u8),
        "admissibility_specs" => Some(74u8),
        "architecture" => Some(75u8),
        "arity" => Some(76u8),
        "b" => Some(77u8),
        "base_vector" => Some(78u8),
        "bridges" => Some(79u8),
        "capture_specs" => Some(80u8),
        "cell" => Some(81u8),
        "cell_states" => Some(82u8),
        "closure_criterion" => Some(83u8),
        "constraints" => Some(84u8),
        "criticalities" => Some(85u8),
        "edges" => Some(86u8),
        "entries" => Some(87u8),
        "eval_results" => Some(88u8),
        "events" => Some(89u8),
        "exogeneity_mask" => Some(90u8),
        "failure_symbol" => Some(91u8),
        "gate_results" => Some(92u8),
        "horizon_ref" => Some(93u8),
        "index" => Some(94u8),
        "induced_parameters" => Some(95u8),
        "input_codomains" => Some(96u8),
        "interface" => Some(97u8),
        "kind" => Some(98u8),
        "mapping" => Some(99u8),
        "metadata" => Some(100u8),
        "nodes" => Some(101u8),
        "observation_domain" => Some(102u8),
        "observation_space" => Some(103u8),
        "output_codomain" => Some(104u8),
        "parameter_id" => Some(105u8),
        "parameters" => Some(106u8),
        "partition_one" => Some(107u8),
        "partition_u" => Some(108u8),
        "partition_zero" => Some(109u8),
        "position" => Some(110u8),
        "query_engine" => Some(111u8),
        "query_type" => Some(112u8),
        "regime" => Some(113u8),
        "relation" => Some(114u8),
        "restrictions" => Some(115u8),
        "role" => Some(116u8),
        "rule" => Some(117u8),
        "scope" => Some(118u8),
        "semantics" => Some(119u8),
        "silent_u" => Some(120u8),
        "source" => Some(121u8),
        "source_codomain" => Some(122u8),
        "spec" => Some(123u8),
        "states" => Some(124u8),
        "supervision" => Some(125u8),
        "table" => Some(126u8),
        "target_position" => Some(127u8),
        "ternarizers" => Some(128u8),
        "transduction_policy" => Some(129u8),
        "transition" => Some(130u8),
        "u_policy" => Some(131u8),
        "updated_vector" => Some(132u8),
        "vector" => Some(133u8),
        "Ok" => Some(134u8),
        "Degraded" => Some(135u8),
        "NotAdmitted" => Some(136u8),
        "source_state" => Some(137u8),
        "counts" => Some(138u8),
        "threshold" => Some(139u8),
        "classification" => Some(140u8),
        "criticality" => Some(141u8),
        "deltas" => Some(142u8),
        "inputs" => Some(143u8),
        "output" => Some(144u8),
        "previous" => Some(145u8),
        "reviewed_to" => Some(146u8),
        "resolved_to" => Some(147u8),
        "context_ref" => Some(148u8),
        "mechanism_ref" => Some(149u8),
        "response" => Some(150u8),
        "justification" => Some(151u8),
        "meta_eval" => Some(152u8),
        "verdict" => Some(153u8),
        _ => None,
    }
}

#[cfg(test)]
#[inline]
fn lookup_es(word: &str) -> Option<u8> {
    match word {
        "codominio" => Some(0u8),
        "semántica_de_salida" => Some(1u8),
        "especificación_de_celda" => Some(2u8),
        "especificación_acoplada" => Some(3u8),
        "conector" => Some(4u8),
        "tabla_de_admisibilidad" => Some(5u8),
        "especificación_de_captura" => Some(6u8),
        "especificación_de_admisibilidad" => Some(7u8),
        "ternarizador" => Some(8u8),
        "especificación_de_resolución" => Some(9u8),
        "estado_de_celda" => Some(10u8),
        "estado_acoplado" => Some(11u8),
        "grafo" => Some(12u8),
        "relación_semántica" => Some(13u8),
        "patrón" => Some(14u8),
        "horizonte" => Some(15u8),
        "marco" => Some(16u8),
        "datos_de_transición" => Some(17u8),
        "trayectoria" => Some(18u8),
        "dominio" => Some(19u8),
        "agente" => Some(20u8),
        "especificación_de_consulta" => Some(21u8),
        "sea" => Some(22u8),
        "evaluar" => Some(23u8),
        "compuerta" => Some(24u8),
        "resolver" => Some(25u8),
        "consultar" => Some(26u8),
        "supervisar" => Some(27u8),
        "componer" => Some(28u8),
        "usando" => Some(29u8),
        "con" => Some(30u8),
        "contexto" => Some(31u8),
        "mecanismo" => Some(32u8),
        "por" => Some(33u8),
        "en" => Some(34u8),
        "objetivo" => Some(35u8),
        "relaciones" => Some(36u8),
        "patrones" => Some(37u8),
        "Base" => Some(38u8),
        "Supervisor" => Some(39u8),
        "Compuesto" => Some(40u8),
        "Simple" => Some(41u8),
        "General" => Some(42u8),
        "ObjetivoCelda" => Some(43u8),
        "ObjetivoCompuesto" => Some(44u8),
        "ObjetivoSistema" => Some(45u8),
        "VistaEvaluaciónPuntual" => Some(46u8),
        "VistaTrayectoria" => Some(47u8),
        "ComparaciónMarcos" => Some(48u8),
        "VistaArquitectura" => Some(49u8),
        "InformeCobertura" => Some(50u8),
        "EvaluaciónPuntual" => Some(51u8),
        "EstadoTrayectoria" => Some(52u8),
        "EstadoCobertura" => Some(53u8),
        "UPendiente" => Some(54u8),
        "CriticidadGlobal" => Some(55u8),
        "Celda" => Some(56u8),
        "Par" => Some(57u8),
        "Arquitectura" => Some(58u8),
        "Trayectoria" => Some(59u8),
        "RelaciónDeclarada" => Some(60u8),
        "PatrónDeclarado" => Some(61u8),
        "arista" => Some(62u8),
        "entrada" => Some(63u8),
        "FalloCaptura" => Some(64u8),
        "Verdadero" => Some(65u8),
        "Cero" => Some(66u8),
        "Uno" => Some(67u8),
        "U" => Some(68u8),
        "max" => Some(69u8),
        "min" => Some(70u8),
        "nulo" => Some(71u8),
        "Ninguno" => Some(72u8),
        "NaN" => Some(73u8),
        "especificaciones_de_admisibilidad" => Some(74u8),
        "arquitectura" => Some(75u8),
        "aridad" => Some(76u8),
        "b" => Some(77u8),
        "vector_base" => Some(78u8),
        "puentes" => Some(79u8),
        "especificaciones_de_captura" => Some(80u8),
        "celda" => Some(81u8),
        "estados_de_celda" => Some(82u8),
        "criterio_de_clausura" => Some(83u8),
        "restricciones" => Some(84u8),
        "criticidades" => Some(85u8),
        "aristas" => Some(86u8),
        "entradas_de_trayectoria" => Some(87u8),
        "resultados_de_evaluación" => Some(88u8),
        "sucesos" => Some(89u8),
        "máscara_de_exogeneidad" => Some(90u8),
        "símbolo_de_fallo" => Some(91u8),
        "resultados_de_compuerta" => Some(92u8),
        "referencia_de_horizonte" => Some(93u8),
        "índice" => Some(94u8),
        "parámetros_inducidos" => Some(95u8),
        "codominios_de_entrada" => Some(96u8),
        "interfaz" => Some(97u8),
        "clase" => Some(98u8),
        "correspondencia" => Some(99u8),
        "metadatos" => Some(100u8),
        "nodos" => Some(101u8),
        "dominio_de_observación" => Some(102u8),
        "espacio_de_observación" => Some(103u8),
        "codominio_de_salida" => Some(104u8),
        "identificador_de_parámetro" => Some(105u8),
        "parámetros" => Some(106u8),
        "partición_uno" => Some(107u8),
        "partición_u" => Some(108u8),
        "partición_cero" => Some(109u8),
        "posición" => Some(110u8),
        "motor_de_consulta" => Some(111u8),
        "tipo_de_consulta" => Some(112u8),
        "régimen" => Some(113u8),
        "relación" => Some(114u8),
        "limitaciones" => Some(115u8),
        "rol" => Some(116u8),
        "regla" => Some(117u8),
        "alcance" => Some(118u8),
        "semántica" => Some(119u8),
        "u_silenciosa" => Some(120u8),
        "origen" => Some(121u8),
        "codominio_de_origen" => Some(122u8),
        "especificación" => Some(123u8),
        "estados" => Some(124u8),
        "supervisión" => Some(125u8),
        "tabla" => Some(126u8),
        "posición_objetivo" => Some(127u8),
        "ternarizadores" => Some(128u8),
        "política_de_transducción" => Some(129u8),
        "transición" => Some(130u8),
        "política_de_u" => Some(131u8),
        "vector_actualizado" => Some(132u8),
        "vector" => Some(133u8),
        "Admitido" => Some(134u8),
        "Degradado" => Some(135u8),
        "NoAdmitido" => Some(136u8),
        "estado_de_origen" => Some(137u8),
        "recuentos" => Some(138u8),
        "umbral" => Some(139u8),
        "clasificación" => Some(140u8),
        "criticidad" => Some(141u8),
        "deltas" => Some(142u8),
        "entradas" => Some(143u8),
        "salida" => Some(144u8),
        "valor_anterior" => Some(145u8),
        "valor_revisado" => Some(146u8),
        "valor_resuelto" => Some(147u8),
        "referencia_de_contexto" => Some(148u8),
        "referencia_de_mecanismo" => Some(149u8),
        "respuesta" => Some(150u8),
        "justificación" => Some(151u8),
        "meta_evaluación" => Some(152u8),
        "dictamen" => Some(153u8),
        _ => None,
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SurfaceEntry {
    id: u8,
    profiles: u8,
}

const PROFILE_MASK_EN: u8 = 1;
const PROFILE_MASK_ES: u8 = 2;

#[inline]
fn lookup_surface(word: &str) -> Option<SurfaceEntry> {
    match word {
        "Admitido" => Some(SurfaceEntry { id: 134u8, profiles: 2u8 }),
        "Architecture" => Some(SurfaceEntry { id: 58u8, profiles: 1u8 }),
        "ArchitectureView" => Some(SurfaceEntry { id: 49u8, profiles: 1u8 }),
        "Arquitectura" => Some(SurfaceEntry { id: 58u8, profiles: 2u8 }),
        "Base" => Some(SurfaceEntry { id: 38u8, profiles: 3u8 }),
        "Bottom" => Some(SurfaceEntry { id: 64u8, profiles: 1u8 }),
        "Celda" => Some(SurfaceEntry { id: 56u8, profiles: 2u8 }),
        "Cell" => Some(SurfaceEntry { id: 56u8, profiles: 1u8 }),
        "CellTarget" => Some(SurfaceEntry { id: 43u8, profiles: 1u8 }),
        "Cero" => Some(SurfaceEntry { id: 66u8, profiles: 2u8 }),
        "ComparaciónMarcos" => Some(SurfaceEntry { id: 48u8, profiles: 2u8 }),
        "ComposedTarget" => Some(SurfaceEntry { id: 44u8, profiles: 1u8 }),
        "Composite" => Some(SurfaceEntry { id: 40u8, profiles: 1u8 }),
        "Compuesto" => Some(SurfaceEntry { id: 40u8, profiles: 2u8 }),
        "CoverageReport" => Some(SurfaceEntry { id: 50u8, profiles: 1u8 }),
        "CoverageState" => Some(SurfaceEntry { id: 53u8, profiles: 1u8 }),
        "CriticidadGlobal" => Some(SurfaceEntry { id: 55u8, profiles: 2u8 }),
        "DeclaredPattern" => Some(SurfaceEntry { id: 61u8, profiles: 1u8 }),
        "DeclaredRelation" => Some(SurfaceEntry { id: 60u8, profiles: 1u8 }),
        "Degradado" => Some(SurfaceEntry { id: 135u8, profiles: 2u8 }),
        "Degraded" => Some(SurfaceEntry { id: 135u8, profiles: 1u8 }),
        "EstadoCobertura" => Some(SurfaceEntry { id: 53u8, profiles: 2u8 }),
        "EstadoTrayectoria" => Some(SurfaceEntry { id: 52u8, profiles: 2u8 }),
        "EvaluaciónPuntual" => Some(SurfaceEntry { id: 51u8, profiles: 2u8 }),
        "FalloCaptura" => Some(SurfaceEntry { id: 64u8, profiles: 2u8 }),
        "FrameComparison" => Some(SurfaceEntry { id: 48u8, profiles: 1u8 }),
        "General" => Some(SurfaceEntry { id: 42u8, profiles: 3u8 }),
        "GlobalCriticality" => Some(SurfaceEntry { id: 55u8, profiles: 1u8 }),
        "InformeCobertura" => Some(SurfaceEntry { id: 50u8, profiles: 2u8 }),
        "NaN" => Some(SurfaceEntry { id: 73u8, profiles: 3u8 }),
        "Ninguno" => Some(SurfaceEntry { id: 72u8, profiles: 2u8 }),
        "NoAdmitido" => Some(SurfaceEntry { id: 136u8, profiles: 2u8 }),
        "None" => Some(SurfaceEntry { id: 72u8, profiles: 1u8 }),
        "NotAdmitted" => Some(SurfaceEntry { id: 136u8, profiles: 1u8 }),
        "ObjetivoCelda" => Some(SurfaceEntry { id: 43u8, profiles: 2u8 }),
        "ObjetivoCompuesto" => Some(SurfaceEntry { id: 44u8, profiles: 2u8 }),
        "ObjetivoSistema" => Some(SurfaceEntry { id: 45u8, profiles: 2u8 }),
        "Ok" => Some(SurfaceEntry { id: 134u8, profiles: 1u8 }),
        "One" => Some(SurfaceEntry { id: 67u8, profiles: 1u8 }),
        "Pair" => Some(SurfaceEntry { id: 57u8, profiles: 1u8 }),
        "Par" => Some(SurfaceEntry { id: 57u8, profiles: 2u8 }),
        "PatrónDeclarado" => Some(SurfaceEntry { id: 61u8, profiles: 2u8 }),
        "PendingU" => Some(SurfaceEntry { id: 54u8, profiles: 1u8 }),
        "PointEval" => Some(SurfaceEntry { id: 46u8, profiles: 1u8 }),
        "PointEvaluation" => Some(SurfaceEntry { id: 51u8, profiles: 1u8 }),
        "RelaciónDeclarada" => Some(SurfaceEntry { id: 60u8, profiles: 2u8 }),
        "Simple" => Some(SurfaceEntry { id: 41u8, profiles: 3u8 }),
        "Supervisor" => Some(SurfaceEntry { id: 39u8, profiles: 3u8 }),
        "SystemTarget" => Some(SurfaceEntry { id: 45u8, profiles: 1u8 }),
        "Trajectory" => Some(SurfaceEntry { id: 59u8, profiles: 1u8 }),
        "TrajectoryState" => Some(SurfaceEntry { id: 52u8, profiles: 1u8 }),
        "TrajectoryView" => Some(SurfaceEntry { id: 47u8, profiles: 1u8 }),
        "Trayectoria" => Some(SurfaceEntry { id: 59u8, profiles: 2u8 }),
        "True" => Some(SurfaceEntry { id: 65u8, profiles: 1u8 }),
        "U" => Some(SurfaceEntry { id: 68u8, profiles: 3u8 }),
        "UPendiente" => Some(SurfaceEntry { id: 54u8, profiles: 2u8 }),
        "Uno" => Some(SurfaceEntry { id: 67u8, profiles: 2u8 }),
        "Verdadero" => Some(SurfaceEntry { id: 65u8, profiles: 2u8 }),
        "VistaArquitectura" => Some(SurfaceEntry { id: 49u8, profiles: 2u8 }),
        "VistaEvaluaciónPuntual" => Some(SurfaceEntry { id: 46u8, profiles: 2u8 }),
        "VistaTrayectoria" => Some(SurfaceEntry { id: 47u8, profiles: 2u8 }),
        "Zero" => Some(SurfaceEntry { id: 66u8, profiles: 1u8 }),
        "admissibility_spec" => Some(SurfaceEntry { id: 7u8, profiles: 1u8 }),
        "admissibility_specs" => Some(SurfaceEntry { id: 74u8, profiles: 1u8 }),
        "admissibility_table" => Some(SurfaceEntry { id: 5u8, profiles: 1u8 }),
        "agent" => Some(SurfaceEntry { id: 20u8, profiles: 1u8 }),
        "agente" => Some(SurfaceEntry { id: 20u8, profiles: 2u8 }),
        "alcance" => Some(SurfaceEntry { id: 118u8, profiles: 2u8 }),
        "architecture" => Some(SurfaceEntry { id: 75u8, profiles: 1u8 }),
        "aridad" => Some(SurfaceEntry { id: 76u8, profiles: 2u8 }),
        "arista" => Some(SurfaceEntry { id: 62u8, profiles: 2u8 }),
        "aristas" => Some(SurfaceEntry { id: 86u8, profiles: 2u8 }),
        "arity" => Some(SurfaceEntry { id: 76u8, profiles: 1u8 }),
        "arquitectura" => Some(SurfaceEntry { id: 75u8, profiles: 2u8 }),
        "b" => Some(SurfaceEntry { id: 77u8, profiles: 3u8 }),
        "base_vector" => Some(SurfaceEntry { id: 78u8, profiles: 1u8 }),
        "bridges" => Some(SurfaceEntry { id: 79u8, profiles: 1u8 }),
        "by" => Some(SurfaceEntry { id: 33u8, profiles: 1u8 }),
        "capture_spec" => Some(SurfaceEntry { id: 6u8, profiles: 1u8 }),
        "capture_specs" => Some(SurfaceEntry { id: 80u8, profiles: 1u8 }),
        "celda" => Some(SurfaceEntry { id: 81u8, profiles: 2u8 }),
        "cell" => Some(SurfaceEntry { id: 81u8, profiles: 1u8 }),
        "cell_states" => Some(SurfaceEntry { id: 82u8, profiles: 1u8 }),
        "cellspec" => Some(SurfaceEntry { id: 2u8, profiles: 1u8 }),
        "cellstate" => Some(SurfaceEntry { id: 10u8, profiles: 1u8 }),
        "clase" => Some(SurfaceEntry { id: 98u8, profiles: 2u8 }),
        "clasificación" => Some(SurfaceEntry { id: 140u8, profiles: 2u8 }),
        "classification" => Some(SurfaceEntry { id: 140u8, profiles: 1u8 }),
        "closure_criterion" => Some(SurfaceEntry { id: 83u8, profiles: 1u8 }),
        "codomain" => Some(SurfaceEntry { id: 0u8, profiles: 1u8 }),
        "codominio" => Some(SurfaceEntry { id: 0u8, profiles: 2u8 }),
        "codominio_de_origen" => Some(SurfaceEntry { id: 122u8, profiles: 2u8 }),
        "codominio_de_salida" => Some(SurfaceEntry { id: 104u8, profiles: 2u8 }),
        "codominios_de_entrada" => Some(SurfaceEntry { id: 96u8, profiles: 2u8 }),
        "componer" => Some(SurfaceEntry { id: 28u8, profiles: 2u8 }),
        "compose" => Some(SurfaceEntry { id: 28u8, profiles: 1u8 }),
        "compuerta" => Some(SurfaceEntry { id: 24u8, profiles: 2u8 }),
        "con" => Some(SurfaceEntry { id: 30u8, profiles: 2u8 }),
        "conector" => Some(SurfaceEntry { id: 4u8, profiles: 2u8 }),
        "connector" => Some(SurfaceEntry { id: 4u8, profiles: 1u8 }),
        "constraints" => Some(SurfaceEntry { id: 84u8, profiles: 1u8 }),
        "consultar" => Some(SurfaceEntry { id: 26u8, profiles: 2u8 }),
        "context" => Some(SurfaceEntry { id: 31u8, profiles: 1u8 }),
        "context_ref" => Some(SurfaceEntry { id: 148u8, profiles: 1u8 }),
        "contexto" => Some(SurfaceEntry { id: 31u8, profiles: 2u8 }),
        "correspondencia" => Some(SurfaceEntry { id: 99u8, profiles: 2u8 }),
        "counts" => Some(SurfaceEntry { id: 138u8, profiles: 1u8 }),
        "coupledspec" => Some(SurfaceEntry { id: 3u8, profiles: 1u8 }),
        "coupledstate" => Some(SurfaceEntry { id: 11u8, profiles: 1u8 }),
        "criterio_de_clausura" => Some(SurfaceEntry { id: 83u8, profiles: 2u8 }),
        "criticalities" => Some(SurfaceEntry { id: 85u8, profiles: 1u8 }),
        "criticality" => Some(SurfaceEntry { id: 141u8, profiles: 1u8 }),
        "criticidad" => Some(SurfaceEntry { id: 141u8, profiles: 2u8 }),
        "criticidades" => Some(SurfaceEntry { id: 85u8, profiles: 2u8 }),
        "datos_de_transición" => Some(SurfaceEntry { id: 17u8, profiles: 2u8 }),
        "deltas" => Some(SurfaceEntry { id: 142u8, profiles: 3u8 }),
        "dictamen" => Some(SurfaceEntry { id: 153u8, profiles: 2u8 }),
        "domain" => Some(SurfaceEntry { id: 19u8, profiles: 1u8 }),
        "dominio" => Some(SurfaceEntry { id: 19u8, profiles: 2u8 }),
        "dominio_de_observación" => Some(SurfaceEntry { id: 102u8, profiles: 2u8 }),
        "edge" => Some(SurfaceEntry { id: 62u8, profiles: 1u8 }),
        "edges" => Some(SurfaceEntry { id: 86u8, profiles: 1u8 }),
        "en" => Some(SurfaceEntry { id: 34u8, profiles: 2u8 }),
        "entrada" => Some(SurfaceEntry { id: 63u8, profiles: 2u8 }),
        "entradas" => Some(SurfaceEntry { id: 143u8, profiles: 2u8 }),
        "entradas_de_trayectoria" => Some(SurfaceEntry { id: 87u8, profiles: 2u8 }),
        "entries" => Some(SurfaceEntry { id: 87u8, profiles: 1u8 }),
        "entry" => Some(SurfaceEntry { id: 63u8, profiles: 1u8 }),
        "espacio_de_observación" => Some(SurfaceEntry { id: 103u8, profiles: 2u8 }),
        "especificaciones_de_admisibilidad" => Some(SurfaceEntry { id: 74u8, profiles: 2u8 }),
        "especificaciones_de_captura" => Some(SurfaceEntry { id: 80u8, profiles: 2u8 }),
        "especificación" => Some(SurfaceEntry { id: 123u8, profiles: 2u8 }),
        "especificación_acoplada" => Some(SurfaceEntry { id: 3u8, profiles: 2u8 }),
        "especificación_de_admisibilidad" => Some(SurfaceEntry { id: 7u8, profiles: 2u8 }),
        "especificación_de_captura" => Some(SurfaceEntry { id: 6u8, profiles: 2u8 }),
        "especificación_de_celda" => Some(SurfaceEntry { id: 2u8, profiles: 2u8 }),
        "especificación_de_consulta" => Some(SurfaceEntry { id: 21u8, profiles: 2u8 }),
        "especificación_de_resolución" => Some(SurfaceEntry { id: 9u8, profiles: 2u8 }),
        "estado_acoplado" => Some(SurfaceEntry { id: 11u8, profiles: 2u8 }),
        "estado_de_celda" => Some(SurfaceEntry { id: 10u8, profiles: 2u8 }),
        "estado_de_origen" => Some(SurfaceEntry { id: 137u8, profiles: 2u8 }),
        "estados" => Some(SurfaceEntry { id: 124u8, profiles: 2u8 }),
        "estados_de_celda" => Some(SurfaceEntry { id: 82u8, profiles: 2u8 }),
        "eval_results" => Some(SurfaceEntry { id: 88u8, profiles: 1u8 }),
        "evaluar" => Some(SurfaceEntry { id: 23u8, profiles: 2u8 }),
        "evaluate" => Some(SurfaceEntry { id: 23u8, profiles: 1u8 }),
        "events" => Some(SurfaceEntry { id: 89u8, profiles: 1u8 }),
        "exogeneity_mask" => Some(SurfaceEntry { id: 90u8, profiles: 1u8 }),
        "failure_symbol" => Some(SurfaceEntry { id: 91u8, profiles: 1u8 }),
        "frame" => Some(SurfaceEntry { id: 16u8, profiles: 1u8 }),
        "gate" => Some(SurfaceEntry { id: 24u8, profiles: 1u8 }),
        "gate_results" => Some(SurfaceEntry { id: 92u8, profiles: 1u8 }),
        "grafo" => Some(SurfaceEntry { id: 12u8, profiles: 2u8 }),
        "graph" => Some(SurfaceEntry { id: 12u8, profiles: 1u8 }),
        "horizon" => Some(SurfaceEntry { id: 15u8, profiles: 1u8 }),
        "horizon_ref" => Some(SurfaceEntry { id: 93u8, profiles: 1u8 }),
        "horizonte" => Some(SurfaceEntry { id: 15u8, profiles: 2u8 }),
        "identificador_de_parámetro" => Some(SurfaceEntry { id: 105u8, profiles: 2u8 }),
        "in" => Some(SurfaceEntry { id: 34u8, profiles: 1u8 }),
        "index" => Some(SurfaceEntry { id: 94u8, profiles: 1u8 }),
        "induced_parameters" => Some(SurfaceEntry { id: 95u8, profiles: 1u8 }),
        "input_codomains" => Some(SurfaceEntry { id: 96u8, profiles: 1u8 }),
        "inputs" => Some(SurfaceEntry { id: 143u8, profiles: 1u8 }),
        "interface" => Some(SurfaceEntry { id: 97u8, profiles: 1u8 }),
        "interfaz" => Some(SurfaceEntry { id: 97u8, profiles: 2u8 }),
        "justificación" => Some(SurfaceEntry { id: 151u8, profiles: 2u8 }),
        "justification" => Some(SurfaceEntry { id: 151u8, profiles: 1u8 }),
        "kind" => Some(SurfaceEntry { id: 98u8, profiles: 1u8 }),
        "let" => Some(SurfaceEntry { id: 22u8, profiles: 1u8 }),
        "limitaciones" => Some(SurfaceEntry { id: 115u8, profiles: 2u8 }),
        "mapping" => Some(SurfaceEntry { id: 99u8, profiles: 1u8 }),
        "marco" => Some(SurfaceEntry { id: 16u8, profiles: 2u8 }),
        "max" => Some(SurfaceEntry { id: 69u8, profiles: 3u8 }),
        "mecanismo" => Some(SurfaceEntry { id: 32u8, profiles: 2u8 }),
        "mechanism" => Some(SurfaceEntry { id: 32u8, profiles: 1u8 }),
        "mechanism_ref" => Some(SurfaceEntry { id: 149u8, profiles: 1u8 }),
        "meta_eval" => Some(SurfaceEntry { id: 152u8, profiles: 1u8 }),
        "meta_evaluación" => Some(SurfaceEntry { id: 152u8, profiles: 2u8 }),
        "metadata" => Some(SurfaceEntry { id: 100u8, profiles: 1u8 }),
        "metadatos" => Some(SurfaceEntry { id: 100u8, profiles: 2u8 }),
        "min" => Some(SurfaceEntry { id: 70u8, profiles: 3u8 }),
        "motor_de_consulta" => Some(SurfaceEntry { id: 111u8, profiles: 2u8 }),
        "máscara_de_exogeneidad" => Some(SurfaceEntry { id: 90u8, profiles: 2u8 }),
        "nodes" => Some(SurfaceEntry { id: 101u8, profiles: 1u8 }),
        "nodos" => Some(SurfaceEntry { id: 101u8, profiles: 2u8 }),
        "null" => Some(SurfaceEntry { id: 71u8, profiles: 1u8 }),
        "nulo" => Some(SurfaceEntry { id: 71u8, profiles: 2u8 }),
        "objetivo" => Some(SurfaceEntry { id: 35u8, profiles: 2u8 }),
        "observation_domain" => Some(SurfaceEntry { id: 102u8, profiles: 1u8 }),
        "observation_space" => Some(SurfaceEntry { id: 103u8, profiles: 1u8 }),
        "origen" => Some(SurfaceEntry { id: 121u8, profiles: 2u8 }),
        "output" => Some(SurfaceEntry { id: 144u8, profiles: 1u8 }),
        "output_codomain" => Some(SurfaceEntry { id: 104u8, profiles: 1u8 }),
        "output_semantics" => Some(SurfaceEntry { id: 1u8, profiles: 1u8 }),
        "parameter_id" => Some(SurfaceEntry { id: 105u8, profiles: 1u8 }),
        "parameters" => Some(SurfaceEntry { id: 106u8, profiles: 1u8 }),
        "partición_cero" => Some(SurfaceEntry { id: 109u8, profiles: 2u8 }),
        "partición_u" => Some(SurfaceEntry { id: 108u8, profiles: 2u8 }),
        "partición_uno" => Some(SurfaceEntry { id: 107u8, profiles: 2u8 }),
        "partition_one" => Some(SurfaceEntry { id: 107u8, profiles: 1u8 }),
        "partition_u" => Some(SurfaceEntry { id: 108u8, profiles: 1u8 }),
        "partition_zero" => Some(SurfaceEntry { id: 109u8, profiles: 1u8 }),
        "parámetros" => Some(SurfaceEntry { id: 106u8, profiles: 2u8 }),
        "parámetros_inducidos" => Some(SurfaceEntry { id: 95u8, profiles: 2u8 }),
        "patrones" => Some(SurfaceEntry { id: 37u8, profiles: 2u8 }),
        "patrón" => Some(SurfaceEntry { id: 14u8, profiles: 2u8 }),
        "pattern" => Some(SurfaceEntry { id: 14u8, profiles: 1u8 }),
        "patterns" => Some(SurfaceEntry { id: 37u8, profiles: 1u8 }),
        "política_de_transducción" => Some(SurfaceEntry { id: 129u8, profiles: 2u8 }),
        "política_de_u" => Some(SurfaceEntry { id: 131u8, profiles: 2u8 }),
        "por" => Some(SurfaceEntry { id: 33u8, profiles: 2u8 }),
        "posición" => Some(SurfaceEntry { id: 110u8, profiles: 2u8 }),
        "posición_objetivo" => Some(SurfaceEntry { id: 127u8, profiles: 2u8 }),
        "position" => Some(SurfaceEntry { id: 110u8, profiles: 1u8 }),
        "previous" => Some(SurfaceEntry { id: 145u8, profiles: 1u8 }),
        "puentes" => Some(SurfaceEntry { id: 79u8, profiles: 2u8 }),
        "query" => Some(SurfaceEntry { id: 26u8, profiles: 1u8 }),
        "query_engine" => Some(SurfaceEntry { id: 111u8, profiles: 1u8 }),
        "query_spec" => Some(SurfaceEntry { id: 21u8, profiles: 1u8 }),
        "query_type" => Some(SurfaceEntry { id: 112u8, profiles: 1u8 }),
        "recuentos" => Some(SurfaceEntry { id: 138u8, profiles: 2u8 }),
        "referencia_de_contexto" => Some(SurfaceEntry { id: 148u8, profiles: 2u8 }),
        "referencia_de_horizonte" => Some(SurfaceEntry { id: 93u8, profiles: 2u8 }),
        "referencia_de_mecanismo" => Some(SurfaceEntry { id: 149u8, profiles: 2u8 }),
        "regime" => Some(SurfaceEntry { id: 113u8, profiles: 1u8 }),
        "regla" => Some(SurfaceEntry { id: 117u8, profiles: 2u8 }),
        "relaciones" => Some(SurfaceEntry { id: 36u8, profiles: 2u8 }),
        "relación" => Some(SurfaceEntry { id: 114u8, profiles: 2u8 }),
        "relación_semántica" => Some(SurfaceEntry { id: 13u8, profiles: 2u8 }),
        "relation" => Some(SurfaceEntry { id: 114u8, profiles: 1u8 }),
        "relations" => Some(SurfaceEntry { id: 36u8, profiles: 1u8 }),
        "res_spec" => Some(SurfaceEntry { id: 9u8, profiles: 1u8 }),
        "resolve" => Some(SurfaceEntry { id: 25u8, profiles: 1u8 }),
        "resolved_to" => Some(SurfaceEntry { id: 147u8, profiles: 1u8 }),
        "resolver" => Some(SurfaceEntry { id: 25u8, profiles: 2u8 }),
        "response" => Some(SurfaceEntry { id: 150u8, profiles: 1u8 }),
        "respuesta" => Some(SurfaceEntry { id: 150u8, profiles: 2u8 }),
        "restricciones" => Some(SurfaceEntry { id: 84u8, profiles: 2u8 }),
        "restrictions" => Some(SurfaceEntry { id: 115u8, profiles: 1u8 }),
        "resultados_de_compuerta" => Some(SurfaceEntry { id: 92u8, profiles: 2u8 }),
        "resultados_de_evaluación" => Some(SurfaceEntry { id: 88u8, profiles: 2u8 }),
        "reviewed_to" => Some(SurfaceEntry { id: 146u8, profiles: 1u8 }),
        "rol" => Some(SurfaceEntry { id: 116u8, profiles: 2u8 }),
        "role" => Some(SurfaceEntry { id: 116u8, profiles: 1u8 }),
        "rule" => Some(SurfaceEntry { id: 117u8, profiles: 1u8 }),
        "régimen" => Some(SurfaceEntry { id: 113u8, profiles: 2u8 }),
        "salida" => Some(SurfaceEntry { id: 144u8, profiles: 2u8 }),
        "scope" => Some(SurfaceEntry { id: 118u8, profiles: 1u8 }),
        "sea" => Some(SurfaceEntry { id: 22u8, profiles: 2u8 }),
        "semantic_relation" => Some(SurfaceEntry { id: 13u8, profiles: 1u8 }),
        "semantics" => Some(SurfaceEntry { id: 119u8, profiles: 1u8 }),
        "semántica" => Some(SurfaceEntry { id: 119u8, profiles: 2u8 }),
        "semántica_de_salida" => Some(SurfaceEntry { id: 1u8, profiles: 2u8 }),
        "silent_u" => Some(SurfaceEntry { id: 120u8, profiles: 1u8 }),
        "source" => Some(SurfaceEntry { id: 121u8, profiles: 1u8 }),
        "source_codomain" => Some(SurfaceEntry { id: 122u8, profiles: 1u8 }),
        "source_state" => Some(SurfaceEntry { id: 137u8, profiles: 1u8 }),
        "spec" => Some(SurfaceEntry { id: 123u8, profiles: 1u8 }),
        "states" => Some(SurfaceEntry { id: 124u8, profiles: 1u8 }),
        "sucesos" => Some(SurfaceEntry { id: 89u8, profiles: 2u8 }),
        "supervisar" => Some(SurfaceEntry { id: 27u8, profiles: 2u8 }),
        "supervise" => Some(SurfaceEntry { id: 27u8, profiles: 1u8 }),
        "supervision" => Some(SurfaceEntry { id: 125u8, profiles: 1u8 }),
        "supervisión" => Some(SurfaceEntry { id: 125u8, profiles: 2u8 }),
        "símbolo_de_fallo" => Some(SurfaceEntry { id: 91u8, profiles: 2u8 }),
        "tabla" => Some(SurfaceEntry { id: 126u8, profiles: 2u8 }),
        "tabla_de_admisibilidad" => Some(SurfaceEntry { id: 5u8, profiles: 2u8 }),
        "table" => Some(SurfaceEntry { id: 126u8, profiles: 1u8 }),
        "target" => Some(SurfaceEntry { id: 35u8, profiles: 1u8 }),
        "target_position" => Some(SurfaceEntry { id: 127u8, profiles: 1u8 }),
        "ternarizador" => Some(SurfaceEntry { id: 8u8, profiles: 2u8 }),
        "ternarizadores" => Some(SurfaceEntry { id: 128u8, profiles: 2u8 }),
        "ternarizer" => Some(SurfaceEntry { id: 8u8, profiles: 1u8 }),
        "ternarizers" => Some(SurfaceEntry { id: 128u8, profiles: 1u8 }),
        "threshold" => Some(SurfaceEntry { id: 139u8, profiles: 1u8 }),
        "tipo_de_consulta" => Some(SurfaceEntry { id: 112u8, profiles: 2u8 }),
        "trajectory" => Some(SurfaceEntry { id: 18u8, profiles: 1u8 }),
        "transduction_policy" => Some(SurfaceEntry { id: 129u8, profiles: 1u8 }),
        "transición" => Some(SurfaceEntry { id: 130u8, profiles: 2u8 }),
        "transition" => Some(SurfaceEntry { id: 130u8, profiles: 1u8 }),
        "transition_data" => Some(SurfaceEntry { id: 17u8, profiles: 1u8 }),
        "trayectoria" => Some(SurfaceEntry { id: 18u8, profiles: 2u8 }),
        "u_policy" => Some(SurfaceEntry { id: 131u8, profiles: 1u8 }),
        "u_silenciosa" => Some(SurfaceEntry { id: 120u8, profiles: 2u8 }),
        "umbral" => Some(SurfaceEntry { id: 139u8, profiles: 2u8 }),
        "updated_vector" => Some(SurfaceEntry { id: 132u8, profiles: 1u8 }),
        "usando" => Some(SurfaceEntry { id: 29u8, profiles: 2u8 }),
        "using" => Some(SurfaceEntry { id: 29u8, profiles: 1u8 }),
        "valor_anterior" => Some(SurfaceEntry { id: 145u8, profiles: 2u8 }),
        "valor_resuelto" => Some(SurfaceEntry { id: 147u8, profiles: 2u8 }),
        "valor_revisado" => Some(SurfaceEntry { id: 146u8, profiles: 2u8 }),
        "vector" => Some(SurfaceEntry { id: 133u8, profiles: 3u8 }),
        "vector_actualizado" => Some(SurfaceEntry { id: 132u8, profiles: 2u8 }),
        "vector_base" => Some(SurfaceEntry { id: 78u8, profiles: 2u8 }),
        "verdict" => Some(SurfaceEntry { id: 153u8, profiles: 1u8 }),
        "with" => Some(SurfaceEntry { id: 30u8, profiles: 1u8 }),
        "índice" => Some(SurfaceEntry { id: 94u8, profiles: 2u8 }),
        _ => None,
    }
}

#[inline]
fn classify_surface(profile: SourceProfile, word: &str) -> u8 {
    let Some(entry) = lookup_surface(word) else {
        return STATUS_OTHER;
    };
    let active = match profile {
        SourceProfile::En => PROFILE_MASK_EN,
        SourceProfile::Es => PROFILE_MASK_ES,
    };
    if entry.profiles & active != 0 {
        entry.id
    } else if entry.id < 74 {
        STATUS_FOREIGN_PROTECTED
    } else {
        STATUS_FOREIGN_CONTEXTUAL
    }
}

#[inline]
fn canonical_word<'a>(raw: &'a str, status: u8) -> &'a str {
    if (status as usize) < CANONICAL_FORMS.len() {
        CANONICAL_FORMS[status as usize]
    } else if matches!(status, STATUS_FOREIGN_CONTEXTUAL | STATUS_FOREIGN_PROTECTED) {
        FOREIGN_SURFACE_SENTINEL
    } else {
        raw
    }
}

#[inline]
fn protected_status(status: u8) -> bool { status < 74 || status == STATUS_FOREIGN_PROTECTED }


#[cfg(test)]
mod profile_catalog_tests_2a {
    use super::*;

    pub(super) const EN_FORMS: [&str; 154] = ["codomain", "output_semantics", "cellspec", "coupledspec", "connector", "admissibility_table", "capture_spec", "admissibility_spec", "ternarizer", "res_spec", "cellstate", "coupledstate", "graph", "semantic_relation", "pattern", "horizon", "frame", "transition_data", "trajectory", "domain", "agent", "query_spec", "let", "evaluate", "gate", "resolve", "query", "supervise", "compose", "using", "with", "context", "mechanism", "by", "in", "target", "relations", "patterns", "Base", "Supervisor", "Composite", "Simple", "General", "CellTarget", "ComposedTarget", "SystemTarget", "PointEval", "TrajectoryView", "FrameComparison", "ArchitectureView", "CoverageReport", "PointEvaluation", "TrajectoryState", "CoverageState", "PendingU", "GlobalCriticality", "Cell", "Pair", "Architecture", "Trajectory", "DeclaredRelation", "DeclaredPattern", "edge", "entry", "Bottom", "True", "Zero", "One", "U", "max", "min", "null", "None", "NaN", "admissibility_specs", "architecture", "arity", "b", "base_vector", "bridges", "capture_specs", "cell", "cell_states", "closure_criterion", "constraints", "criticalities", "edges", "entries", "eval_results", "events", "exogeneity_mask", "failure_symbol", "gate_results", "horizon_ref", "index", "induced_parameters", "input_codomains", "interface", "kind", "mapping", "metadata", "nodes", "observation_domain", "observation_space", "output_codomain", "parameter_id", "parameters", "partition_one", "partition_u", "partition_zero", "position", "query_engine", "query_type", "regime", "relation", "restrictions", "role", "rule", "scope", "semantics", "silent_u", "source", "source_codomain", "spec", "states", "supervision", "table", "target_position", "ternarizers", "transduction_policy", "transition", "u_policy", "updated_vector", "vector", "Ok", "Degraded", "NotAdmitted", "source_state", "counts", "threshold", "classification", "criticality", "deltas", "inputs", "output", "previous", "reviewed_to", "resolved_to", "context_ref", "mechanism_ref", "response", "justification", "meta_eval", "verdict"];
    pub(super) const ES_FORMS: [&str; 154] = ["codominio", "semántica_de_salida", "especificación_de_celda", "especificación_acoplada", "conector", "tabla_de_admisibilidad", "especificación_de_captura", "especificación_de_admisibilidad", "ternarizador", "especificación_de_resolución", "estado_de_celda", "estado_acoplado", "grafo", "relación_semántica", "patrón", "horizonte", "marco", "datos_de_transición", "trayectoria", "dominio", "agente", "especificación_de_consulta", "sea", "evaluar", "compuerta", "resolver", "consultar", "supervisar", "componer", "usando", "con", "contexto", "mecanismo", "por", "en", "objetivo", "relaciones", "patrones", "Base", "Supervisor", "Compuesto", "Simple", "General", "ObjetivoCelda", "ObjetivoCompuesto", "ObjetivoSistema", "VistaEvaluaciónPuntual", "VistaTrayectoria", "ComparaciónMarcos", "VistaArquitectura", "InformeCobertura", "EvaluaciónPuntual", "EstadoTrayectoria", "EstadoCobertura", "UPendiente", "CriticidadGlobal", "Celda", "Par", "Arquitectura", "Trayectoria", "RelaciónDeclarada", "PatrónDeclarado", "arista", "entrada", "FalloCaptura", "Verdadero", "Cero", "Uno", "U", "max", "min", "nulo", "Ninguno", "NaN", "especificaciones_de_admisibilidad", "arquitectura", "aridad", "b", "vector_base", "puentes", "especificaciones_de_captura", "celda", "estados_de_celda", "criterio_de_clausura", "restricciones", "criticidades", "aristas", "entradas_de_trayectoria", "resultados_de_evaluación", "sucesos", "máscara_de_exogeneidad", "símbolo_de_fallo", "resultados_de_compuerta", "referencia_de_horizonte", "índice", "parámetros_inducidos", "codominios_de_entrada", "interfaz", "clase", "correspondencia", "metadatos", "nodos", "dominio_de_observación", "espacio_de_observación", "codominio_de_salida", "identificador_de_parámetro", "parámetros", "partición_uno", "partición_u", "partición_cero", "posición", "motor_de_consulta", "tipo_de_consulta", "régimen", "relación", "limitaciones", "rol", "regla", "alcance", "semántica", "u_silenciosa", "origen", "codominio_de_origen", "especificación", "estados", "supervisión", "tabla", "posición_objetivo", "ternarizadores", "política_de_transducción", "transición", "política_de_u", "vector_actualizado", "vector", "Admitido", "Degradado", "NoAdmitido", "estado_de_origen", "recuentos", "umbral", "clasificación", "criticidad", "deltas", "entradas", "salida", "valor_anterior", "valor_revisado", "valor_resuelto", "referencia_de_contexto", "referencia_de_mecanismo", "respuesta", "justificación", "meta_evaluación", "dictamen"];

    #[test]
    fn catalogos_y_aislamiento_interperfil_son_exhaustivos() {
        for id in 0usize..154 {
            let en = EN_FORMS[id];
            let es = ES_FORMS[id];
            assert_eq!(lookup_en(en), Some(id as u8));
            assert_eq!(lookup_es(es), Some(id as u8));
            assert_eq!(classify_surface(SourceProfile::En, en), id as u8);
            assert_eq!(classify_surface(SourceProfile::Es, es), id as u8);
            assert_eq!(protected_status(id as u8), id < 74);

            if en == es {
                assert_eq!(classify_surface(SourceProfile::Es, en), id as u8);
                assert_eq!(classify_surface(SourceProfile::En, es), id as u8);
            } else {
                let foreign_status = if id < 74 {
                    STATUS_FOREIGN_PROTECTED
                } else {
                    STATUS_FOREIGN_CONTEXTUAL
                };
                assert_eq!(classify_surface(SourceProfile::Es, en), foreign_status);
                assert_eq!(classify_surface(SourceProfile::En, es), foreign_status);
                assert_eq!(canonical_word(en, foreign_status), FOREIGN_SURFACE_SENTINEL);
                assert_eq!(canonical_word(es, foreign_status), FOREIGN_SURFACE_SENTINEL);
                assert_eq!(protected_status(foreign_status), id < 74);
            }
        }
    }
}


#[cfg(test)]
mod union_surface_index_tests_2a {
    use super::*;

    #[test]
    fn indice_conjunto_preserva_identidad_y_pertenencia_de_perfil() {
        assert_eq!(297, 297);
        for id in 0usize..154 {
            let en = super::profile_catalog_tests_2a::EN_FORMS[id];
            let es = super::profile_catalog_tests_2a::ES_FORMS[id];
            let ee = lookup_surface(en).expect("forma EN ausente");
            let se = lookup_surface(es).expect("forma ES ausente");
            assert_eq!(ee.id, id as u8);
            assert_eq!(se.id, id as u8);
            assert_ne!(ee.profiles & PROFILE_MASK_EN, 0);
            assert_ne!(se.profiles & PROFILE_MASK_ES, 0);
            if en == es {
                assert_eq!(ee.profiles, PROFILE_MASK_EN | PROFILE_MASK_ES);
                assert_eq!(se.profiles, PROFILE_MASK_EN | PROFILE_MASK_ES);
            }
        }
    }
}


#[cfg(test)]
mod source_profile_public_tests_2a {
    use super::SourceProfile;

    #[test]
    fn selector_textual_y_abi_son_cerrados_y_estables() {
        assert_eq!(SourceProfile::from_tag("en"), Some(SourceProfile::En));
        assert_eq!(SourceProfile::from_tag("es"), Some(SourceProfile::Es));
        assert_eq!(SourceProfile::En.tag(), "en");
        assert_eq!(SourceProfile::Es.tag(), "es");
        assert_eq!(SourceProfile::En.abi_code(), 0);
        assert_eq!(SourceProfile::Es.abi_code(), 1);
        assert_eq!(SourceProfile::from_abi_code(0), Some(SourceProfile::En));
        assert_eq!(SourceProfile::from_abi_code(1), Some(SourceProfile::Es));
        for bad in ["", "EN", "ES", "spanish", "english", "auto", "es-ES", "en-US"] {
            assert_eq!(SourceProfile::from_tag(bad), None, "alias no admitido: {bad}");
        }
        assert_eq!(SourceProfile::from_abi_code(2), None);
        assert_eq!(SourceProfile::from_abi_code(u32::MAX), None);
    }
}

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
enum Token<'a> {
    Word(&'a str, u8),
    Nat(String),
    Text(String),
    Sym(char),
    Arrow,
    Eof,
}

pub fn compile_svp(source: &str, source_file: &str) -> Result<IrProgram, FrontendError> {
    compile_svp_with_profile(source, source_file, SourceProfile::En)
}

pub fn compile_svp_with_profile(
    source: &str,
    source_file: &str,
    profile: SourceProfile,
) -> Result<IrProgram, FrontendError> {
    let tokens = tokenize(source, profile)?;
    Parser::new(tokens, source, source_file).parse()
}

fn tokenize<'a>(source: &'a str, profile: SourceProfile) -> Result<Vec<Token<'a>>, FrontendError> {
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
            let status = classify_surface(profile, &source[start..i]);
            out.push(Token::Word(&source[start..i], status));
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
    tokens: Vec<Token<'a>>,
    pos: usize,
    source: &'a str,
    source_file: &'a str,
    objects: Vec<crate::IrObject>,
    operations: Vec<crate::IrOperation>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token<'a>>, source: &'a str, source_file: &'a str) -> Self {
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
            match self.peek_word()? {
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
            let label = self.take_raw_word()?;
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
            let field = self.take_raw_word()?;
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
            let field = self.take_raw_word()?;
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
        let (first_raw, first_status) = self.take_dispatch_word()?;
        let first = canonical_word(&first_raw, first_status);
        match first {
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
            _ => {
                if protected_status(first_status) {
                    return Err(FrontendError::UnexpectedToken(format!(
                        "palabra protegida donde se esperaba identificador: {first_raw}"
                    )));
                }
                self.sym('.')?;
                let field = self.take_raw_word()?;
                self.sym(';')?;
                self.operations.push(construction::operation(
                    name,
                    IrOperationKind::Projection {
                        source: first_raw,
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
            Token::Word(raw, status) => Ok(canonical_word(raw, *status)),
            Token::Eof => Err(FrontendError::UnexpectedEnd),
            other => Err(FrontendError::UnexpectedToken(format!("{other:?}"))),
        }
    }

    fn take_raw_word(&mut self) -> Result<String, FrontendError> {
        match self.tokens.get(self.pos).cloned() {
            Some(Token::Word(raw, status)) => {
                self.pos += 1;
                Ok(canonical_word(raw, status).to_owned())
            }
            Some(Token::Eof) | None => Err(FrontendError::UnexpectedEnd),
            Some(other) => Err(FrontendError::UnexpectedToken(format!("{other:?}"))),
        }
    }

    fn take_dispatch_word(&mut self) -> Result<(String, u8), FrontendError> {
        match self.tokens.get(self.pos).cloned() {
            Some(Token::Word(raw, status)) => {
                self.pos += 1;
                Ok((raw.to_owned(), status))
            }
            Some(Token::Eof) | None => Err(FrontendError::UnexpectedEnd),
            Some(other) => Err(FrontendError::UnexpectedToken(format!("{other:?}"))),
        }
    }

    fn take_word(&mut self) -> Result<String, FrontendError> {
        match self.tokens.get(self.pos).cloned() {
            Some(Token::Word(raw, status)) => {
                if protected_status(status) {
                    Err(FrontendError::UnexpectedToken(format!(
                        "palabra protegida donde se esperaba identificador: {raw}"
                    )))
                } else {
                    self.pos += 1;
                    Ok(raw.to_owned())
                }
            }
            Some(Token::Eof) | None => Err(FrontendError::UnexpectedEnd),
            Some(other) => Err(FrontendError::UnexpectedToken(format!("{other:?}"))),
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
        let got = self.peek_word()?;
        if got == expected {
            self.pos += 1;
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

pub(crate) fn sha256_hex(data: &[u8]) -> String {
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
