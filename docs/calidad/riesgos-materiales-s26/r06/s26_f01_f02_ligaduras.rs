//! Banco previo S26-F01/F02, limitado a la frontera LIG/0.1.
//! No ofrece una consulta histórica ni ejecuta una operación de dominio.
//! Ejecución prevista con rustc --test y sv_core del corte fijado.
#![forbid(unsafe_code)]

#[allow(dead_code)]
#[path = "../../../../tests/row7_bindings/cases.rs"]
mod fixtures;

use sv_core::bindings::{
    binding_contract_sha256, validate_bindings, BindingContract, BindingError,
    BindingErrorKind, BindingRequest, ExactReference, ValidatedBindings,
};
use sv_core::Nat;

type Row = (String, String, String, String, String, String);

fn row(usage: &str, instance: &str, node: &str, position: &str) -> Row {
    (
        usage.into(), instance.into(), "P".into(), "1".into(),
        node.into(), position.into(),
    )
}

// El consumidor sólo observa los objetos que la API pública ha recibido.
// No busca por nombre de parámetro, no ordena y no construye Tri.
fn consumed(value: &ValidatedBindings) -> Vec<Row> {
    value.operation().uses.iter()
        .zip(value.instances_in_use_order())
        .map(|(usage, instance)| {
            let destination = usage.destination.as_ref().expect("destino exigido");
            (
                usage.identifier.clone(), instance.identifier.clone(),
                instance.parameter.clone(), instance.parameter_id.as_decimal().into(),
                destination.node.clone(), destination.position.as_decimal().into(),
            )
        })
        .collect()
}

fn request(contract: &BindingContract) -> BindingRequest {
    BindingRequest {
        contract: ExactReference {
            identifier: contract.identifier.clone(),
            version: contract.version.clone(),
            sha256: binding_contract_sha256(contract),
        },
        operation: "OP".into(),
        operation_version: "1".into(),
    }
}

fn rejected(
    program: &sv_core::IrProgram,
    contract: BindingContract,
    expected: &BindingRequest,
    kind: BindingErrorKind,
    subject: &str,
) {
    assert_eq!(
        validate_bindings(program, contract, expected).err(),
        Some(BindingError { kind, subject: subject.into() })
    );
}

fn run_case(case: usize) {
    // Las dos entradas son los fixtures EN y ES ya constituidos.
    for entry in 0..2 {
        let program = fixtures::program(entry).expect("fixture fuente válido");
        let a = fixtures::base(&program);
        let qa = request(&a); // expectativa fijada antes del ataque
        let held_a = validate_bindings(&program, a.clone(), &qa).expect("control A");
        let rows_a = vec![row("U1", "I1", "CC1", "3"), row("U2", "I2", "CCCompartido", "3")];
        assert_eq!(consumed(&held_a), rows_a);
        let mut b = a.clone();

        match case {
            0 => {
                // Dos instancias con igual parámetro y numeral permanecen distintas.
                assert_eq!(held_a.contract().instances.len(), 2);
                assert_ne!(consumed(&held_a)[0].1, consumed(&held_a)[1].1);
            }
            1 => {
                // Mismos bytes de procedencia, otra versión explícita.
                let provenance = b.artifacts.iter_mut()
                    .find(|artifact| artifact.reference.identifier == "FuenteP")
                    .expect("procedencia");
                provenance.reference.version = "2".into();
                for instance in &mut b.instances {
                    instance.provenance[0].version = "2".into();
                }
                let qb = request(&b); // autorización instrumental separada del control B
                assert_ne!(qa.contract, qb.contract);
                assert_eq!(a.artifacts[5].bytes, b.artifacts[5].bytes);
                assert_eq!(a.artifacts[5].reference.sha256, b.artifacts[5].reference.sha256);
                rejected(&program, b.clone(), &qa, BindingErrorKind::ContractIdentity, "ContratoD");
                let held_b = validate_bindings(&program, b.clone(), &qb).expect("control B");
                assert_eq!(consumed(&held_b), rows_a);
                assert_eq!(held_b.contract(), &b);
                assert_eq!(held_b.contract().instances[0].provenance[0].version, "2");
                assert_eq!(held_a.contract().instances[0].provenance[0].version, "1");
                b.instances.clear();
                let mut changed_request = qb.clone();
                changed_request.contract.version = "alterada".into();
                assert_eq!(held_b.expectation(), &qb.contract);
                assert_ne!(held_b.expectation(), &changed_request.contract);
                assert_eq!(held_b.contract().instances.len(), 2);
            }
            2 => {
                // Cambia el orden del registro de instancias, no el de usos.
                b.instances.reverse();
                let held_b = validate_bindings(&program, b.clone(), &request(&b)).expect("registro permutado");
                assert_eq!(consumed(&held_b), rows_a);
                assert_eq!(held_b.contract().instances[0].identifier, "I2");
                // Testigo de sensibilidad: elegir el primer P confundiría U1 con I2.
                let first_p = &held_b.contract().instances[0].identifier;
                assert_ne!(&consumed(&held_b)[0].1, first_p);
            }
            3 => {
                // El orden de los usos es contractual, no el del registro.
                b.operations[0].uses.reverse();
                let held_b = validate_bindings(&program, b.clone(), &request(&b)).expect("usos permutados");
                assert_eq!(consumed(&held_b), vec![
                    row("U2", "I2", "CCCompartido", "3"), row("U1", "I1", "CC1", "3"),
                ]);
            }
            4 => {
                // Dos posiciones del mismo nodo conservan instancias diferentes.
                let destination = b.operations[0].uses[1].destination.as_mut().unwrap();
                destination.node = "CC1".into();
                destination.position = Nat::from_u64(4);
                let held_b = validate_bindings(&program, b.clone(), &request(&b)).expect("posiciones distintas");
                assert_eq!(consumed(&held_b), vec![
                    row("U1", "I1", "CC1", "3"), row("U2", "I2", "CC1", "4"),
                ]);
            }
            5 => {
                // Sustitución de las instancias bajo una expectativa fija.
                b.operations[0].uses[0].instance = "I2".into();
                b.operations[0].uses[1].instance = "I1".into();
                rejected(&program, b.clone(), &qa, BindingErrorKind::ContractIdentity, "ContratoD");
                let held_b = validate_bindings(&program, b.clone(), &request(&b)).expect("control constituido B");
                assert_eq!(consumed(&held_b), vec![
                    row("U1", "I2", "CC1", "3"), row("U2", "I1", "CCCompartido", "3"),
                ]);
            }
            6 => {
                b.operations[0].uses[1].destination = b.operations[0].uses[0].destination.clone();
                let qb = request(&b); // permite alcanzar la guarda interior
                rejected(&program, b, &qb, BindingErrorKind::DestinationCollision, "U2");
            }
            7 => {
                b.instances[1].identifier = "I1".into();
                let qb = request(&b);
                rejected(&program, b, &qb, BindingErrorKind::DuplicateIdentity, "I1");
            }
            8 => {
                b.operations[0].uses[0].instance = "I0".into();
                let qb = request(&b);
                rejected(&program, b, &qb, BindingErrorKind::InstanceMissing, "I0");
            }
            9 => {
                b.operations[0].uses[0].destination = None;
                let qb = request(&b);
                rejected(&program, b, &qb, BindingErrorKind::DestinationMissing, "U1");
            }
            _ => panic!("caso no precomprometido"),
        }

        // Conservar el objeto ya recibido y verificar el control reparado.
        assert_eq!(held_a.contract(), &a);
        assert_eq!(held_a.expectation(), &qa.contract);
        assert_eq!(held_a.program(), &program);
        assert_eq!(consumed(&held_a), rows_a);
        let repaired = validate_bindings(&program, a.clone(), &qa).expect("control reparado");
        assert_eq!(consumed(&repaired), rows_a);
        println!("S26-LIG case={case:02} entry={entry} oracle=CONFORME");
    }
}

#[test] fn f01_control_identidades_distintas() { run_case(0); }
#[test] fn f01_version_con_bytes_iguales() { run_case(1); }
#[test] fn f02_registro_permutado() { run_case(2); }
#[test] fn f02_orden_de_usos() { run_case(3); }
#[test] fn f02_posiciones_del_mismo_nodo() { run_case(4); }
#[test] fn f01_sustitucion_bajo_expectativa_fija() { run_case(5); }
#[test] fn f02_colision_sin_alias() { run_case(6); }
#[test] fn f02_instancia_ambigua() { run_case(7); }
#[test] fn f02_instancia_ausente() { run_case(8); }
#[test] fn f02_destino_ausente() { run_case(9); }
