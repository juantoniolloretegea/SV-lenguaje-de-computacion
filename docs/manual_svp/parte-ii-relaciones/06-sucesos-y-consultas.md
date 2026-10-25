# **6.** Sucesos, trayectorias y consultas

[Lectura en la biblioteca](https://documentos-sv.itvia.online/desarrolladores/parte-ii-relaciones/06-sucesos-y-consultas.html) · [Enciclopedia temática](https://documentos-sv.itvia.online/enciclopedia/)

[Índice del manual](../README.md) · [Anterior](05-composicion.md) · [Siguiente](../parte-iii-ejercicios/07-ejercicios-y-diagnosticos.md)

## **6.1.** Representar datos de transición

El primer archivo relaciona un horizonte con dos sucesos y con los parámetros inducidos declarados. Lea sus referencias y compruebe cómo se declara `Arch1`, aunque aparezca después del horizonte.

```svp
-- transition_data_events.svp — TransitionData con sucesos e induced_parameters

codomain K3 = { APTO, NO_APTO, INDETERMINADO };
output_semantics Klin { APTO -> "ok"; NO_APTO -> "no"; INDETERMINADO -> "u"; }

cellspec C1 { b: 3; codomain: K3; semantics: Klin; role: Base; }

horizon H1 {
  architecture: Arch1;
  events: [CambioProfilaxis, NuevoCultivo];
}

transition_data TD1 {
  horizon_ref: H1;
  events: [(CambioProfilaxis, One), (NuevoCultivo, Zero)];
  induced_parameters: [(CCArch, 3, One), (CCArch, 7, Zero)];
}

-- N0-04: arquitectura explícita del horizonte; referencia adelantada.
coupledspec CCArch { cell: C1; bridges: [3, 7]; }
semantic_relation RArch { kind: DeclaredRelation; constraints: [Local]; }
graph Arch1 { nodes: [CCArch]; edges: []; relation: RArch; regime: Simple; }
```

Archivo: [transition_data_events.svp](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/tests/conformance/valid/transition_data_events.svp).

El archivo declara datos. Los identificadores `CambioProfilaxis` y `NuevoCultivo` no constituyen información de un paciente ni acreditan que haya ocurrido una intervención. El resultado de admisión conserva los objetos del programa; este ejemplo no contiene operaciones solicitadas mediante `let`.

## **6.2.** Relacionar marcos, trayectoria, dominio y agente

El segundo archivo reúne declaraciones necesarias para expresar cinco contextos de consulta. Antes de leerlo completo, siga este orden: `F1` y `F2`, `TR1`, `D1`, `AG1`, especificaciones `QS...` y operaciones `Q1` a `Q5`.

<details>
<summary>Programa completo de los cinco contextos</summary>

```svp
-- query_context_all_variants.svp — cobertura de las cinco variantes de QueryContext

codomain K3 = { APTO, NO_APTO, INDETERMINADO };
output_semantics Klin { APTO -> "ok"; NO_APTO -> "no"; INDETERMINADO -> "u"; }

cellspec C1 {
  b: 3;
  codomain: K3;
  semantics: Klin;
  role: Base;
}

capture_spec Cap1 {
  parameter_id: 1;
  observation_domain: DominioObs1;
  observation_space: ObsEsp1;
  failure_symbol: Bottom;
  mapping: MapCaptura1;
}

admissibility_spec Adm1 {
  parameter_id: 1;
  states: {Ok, Degraded, NotAdmitted};
  rule: ReglaAdm1;
}

ternarizer Ter1 {
  observation_space: ObsEsp1;
  partition_zero: Zona0;
  partition_one: Zona1;
  partition_u: ZonaU;
  mapping: MapTer1;
}

cellstate S1 {
  spec: C1;
  vector: [Zero, One, U, Zero, Zero, One, U, Zero, One];
}

coupledspec CC1 { cell: C1; bridges: [3]; }

coupledstate CS1 {
  spec: CC1;
  base_vector: [Zero, One, U, Zero, Zero, One, U, Zero, One];
  updated_vector: [Zero, One, U, Zero, Zero, One, U, Zero, One];
}

semantic_relation RArch {
  kind: DeclaredRelation;
  constraints: [CoherenciaLocal];
}

graph Arch1 {
  nodes: [CC1];
  edges: [];
  relation: RArch;
  regime: Simple;
}

admissibility_table T1 {
  input_codomains: [K3, K3];
  output_codomain: K3;
  table: {
    (APTO, APTO) -> APTO;
    (APTO, NO_APTO) -> NO_APTO;
    (APTO, INDETERMINADO) -> INDETERMINADO;
    (NO_APTO, APTO) -> NO_APTO;
    (NO_APTO, NO_APTO) -> NO_APTO;
    (NO_APTO, INDETERMINADO) -> NO_APTO;
    (INDETERMINADO, APTO) -> INDETERMINADO;
    (INDETERMINADO, NO_APTO) -> NO_APTO;
    (INDETERMINADO, INDETERMINADO) -> INDETERMINADO;
  }
}

let E1 = evaluate(CS1);
let G1 = gate([E1, E1], using: T1);

horizon H1 {
  architecture: Arch1;
  events: [CambioProfilaxis];
}

transition_data TD1 {
  horizon_ref: H1;
  events: [(CambioProfilaxis, One)];
  induced_parameters: [(CC1, 3, One)];
}

frame F1 {
  index: 1;
  architecture: Arch1;
  cell_states: [CS1];
  eval_results: [E1];
  gate_results: [G1];
  supervision: [];
  criticalities: [];
}

frame F2 {
  index: 2;
  architecture: Arch1;
  cell_states: [CS1];
  eval_results: [E1];
  gate_results: [G1];
  supervision: [];
  criticalities: [];
}

trajectory TR1 {
  entries: [entry(frame: F1, transition: TD1), entry(frame: F2)];
}

domain D1 {
  parameters: [P1];
  interface: I1;
  horizon: H1;
  capture_specs: [Cap1];
  admissibility_specs: [Adm1];
  ternarizers: [Ter1];
  exogeneity_mask: Exo1;
  silent_u: SU1;
  transduction_policy: TP1;
  u_policy: UP1;
  closure_criterion: CCrit1;
}

agent AG1 {
  architecture: Arch1;
  domain: D1;
  query_engine: QE1;
}

query_spec QSPoint {
  query_type: PointEvaluation;
  scope: Cell;
  restrictions: [R1];
}

query_spec QSTraj {
  query_type: TrajectoryState;
  scope: Trajectory;
  restrictions: [R1];
}

query_spec QSFrameCmp {
  query_type: FrameComparison;
  scope: Pair;
  restrictions: [R1];
}

query_spec QSArch {
  query_type: GlobalCriticality;
  scope: Architecture;
  restrictions: [R1];
}

query_spec QSCov {
  query_type: CoverageState;
  scope: Architecture;
  restrictions: [R1];
}

let Q1 = query(QSPoint, by: AG1, in: PointEval(F1));
let Q2 = query(QSTraj, by: AG1, in: TrajectoryView(TR1));
let Q3 = query(QSFrameCmp, by: AG1, in: FrameComparison(F1, F2));
let Q4 = query(QSArch, by: AG1, in: ArchitectureView(Arch1, [C1], [E1], [G1]));
let Q5 = query(QSCov, by: AG1, in: CoverageReport(D1, I1, SU1));
```

</details>

Archivo: [query_context_all_variants.svp](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/tests/conformance/valid/query_context_all_variants.svp).

| Operación | Contexto escrito | Qué permite estudiar |
| --- | --- | --- |
| `Q1` | `PointEval(F1)` | Referencia a un marco. |
| `Q2` | `TrajectoryView(TR1)` | Referencia a una trayectoria. |
| `Q3` | `FrameComparison(F1, F2)` | Relación explícita entre dos marcos. |
| `Q4` | `ArchitectureView(Arch1, [C1], [E1], [G1])` | Arquitectura y colecciones declaradas. |
| `Q5` | `CoverageReport(D1, I1, SU1)` | Dominio, interfaz y referencia de U silenciosa. |

`query` es una construcción del lenguaje. Estos ejemplos no establecen una conexión con un proveedor ni envían una pregunta a una IA. El resultado esperado contiene cinco operaciones `query`, además de la evaluación y la compuerta declaradas.

## **6.3.** Ejercicio de seguimiento

Localice qué dato enlaza la primera entrada de `TR1` con una transición. Después determine qué operación compara `F1` y `F2`.

<details>
<summary>Ver solución</summary>

La primera entrada declara `transition: TD1`; la segunda sólo declara su marco. La comparación se expresa en `Q3`, con `FrameComparison(F1, F2)`. El archivo permite estudiar esas relaciones; no acredita por sí solo persistencia de registros ni una respuesta calculada de consulta.

</details>

Fundamento: [parámetros, recorridos y evidencia](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/parametros-rutas.html).

---

[Índice del manual](../README.md) · [Anterior](05-composicion.md) · [Siguiente](../parte-iii-ejercicios/07-ejercicios-y-diagnosticos.md)

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
