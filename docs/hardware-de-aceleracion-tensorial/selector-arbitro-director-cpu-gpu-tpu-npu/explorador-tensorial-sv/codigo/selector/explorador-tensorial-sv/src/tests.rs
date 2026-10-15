//! Casos artificiales de contrato, separados de la exploración física.
use super::*;
use crate::enlace::*;
use selector::{Capacidad, Carga, Cobertura, Destino, Politica, Solicitud};
use std::cell::Cell;
struct Fija(Consulta);
impl Sonda for Fija {
    fn consultar(&self) -> Consulta {
        self.0.clone()
    }
}
fn observacion(id: &str, clase: Clase) -> Observacion {
    Observacion {
        id: id.into(),
        clase,
        nombre: format!("Simulado-{id}"),
        acceso: Acceso::Consultable,
        memoria_total_bytes: Some(100),
        memoria_libre_bytes: Some(80),
        interfaz: "simulada/1".into(),
    }
}
fn fuente() -> Fija {
    let mut c = Consulta::nueva("simulada");
    c.observaciones = vec![
        observacion("cpu", Clase::Cpu),
        observacion("gpu", Clase::Gpu),
    ];
    Fija(c)
}
fn carga() -> Carga {
    Carga {
        contrato: "caso-artificial/1".into(),
        revision: "revision-exacta".into(),
        perfil: "entero-exacto".into(),
        dimensiones: vec![9],
    }
}
fn evidencia(id: &str, tiempo: u64) -> EvidenciaAdaptador {
    EvidenciaAdaptador {
        fuente: "simulada".into(),
        identidad: id.into(),
        destino: Destino {
            servidor: "simulado".into(),
            dispositivo: id.into(),
            adaptador_revision: "prueba/1".into(),
        },
        capacidad: Capacidad {
            carga: carga(),
            cobertura: Cobertura::Completa,
            evidencia: "simulada-no-fisica".into(),
            estimacion_total_us: Some(tiempo),
        },
        contrastada_ms: 0,
        vigencia_ms: 100_000,
    }
}
fn s() -> Solicitud {
    Solicitud {
        id: "simulada-1".into(),
        carga: carga(),
        memoria_adicional_bytes: 20,
        margen_bytes: 5,
        residencia_obligatoria: None,
    }
}
fn p() -> Politica {
    Politica {
        revision: "politica-simulada/1".into(),
        destinos_permitidos: vec![evidencia("cpu", 30).destino, evidencia("gpu", 10).destino],
        preferencia: vec![selector::Clase::Cpu, selector::Clase::Gpu],
        clase_obligatoria: None,
        vigencia_ms: 100_000,
        comparar_estimaciones: true,
    }
}
fn caps(e: &Explorador, i: &Inventario) -> Vec<CapacidadRecibida> {
    vec![
        e.recibir_capacidad(i, evidencia("cpu", 30)).unwrap(),
        e.recibir_capacidad(i, evidencia("gpu", 10)).unwrap(),
    ]
}

#[test]
fn deteccion_no_es_capacidad() {
    let e = Explorador::nuevo();
    let i = e.explorar(&[&fuente()]).unwrap();
    assert!(e.recursos(&i, &[], 1000).unwrap().is_empty());
    assert!(e.seleccionar(&i, &[], s(), p()).is_err());
}
#[test]
fn seleccion_y_revalidacion_conformes() {
    let e = Explorador::nuevo();
    let f = fuente();
    let i = e.explorar(&[&f]).unwrap();
    let cs = caps(&e, &i);
    let d = e.seleccionar(&i, &cs, s(), p()).unwrap();
    assert_eq!(d.decision().clase, selector::Clase::Gpu);
    assert_eq!(
        e.ejecutar_revalidado(&[&f], &cs, d, |d| Ok(d.clase))
            .unwrap(),
        selector::Clase::Gpu
    );
}
#[test]
fn politica_cpu_prevalece() {
    let e = Explorador::nuevo();
    let i = e.explorar(&[&fuente()]).unwrap();
    let cs = caps(&e, &i);
    let mut pol = p();
    pol.clase_obligatoria = Some(selector::Clase::Cpu);
    assert_eq!(
        e.seleccionar(&i, &cs, s(), pol).unwrap().decision().clase,
        selector::Clase::Cpu
    );
}
#[test]
fn identidad_duplicada_se_rechaza() {
    let e = Explorador::nuevo();
    let mut f = fuente();
    f.0.observaciones.push(f.0.observaciones[0].clone());
    assert!(e.explorar(&[&f]).is_err());
}
#[test]
fn fuente_duplicada_se_rechaza() {
    let e = Explorador::nuevo();
    let f = fuente();
    assert!(e.explorar(&[&f, &f]).is_err());
}
#[test]
fn memoria_imposible_se_rechaza() {
    let e = Explorador::nuevo();
    let mut f = fuente();
    f.0.observaciones[0].memoria_libre_bytes = Some(101);
    assert!(e.explorar(&[&f]).is_err());
}
#[test]
fn fuente_parcial_no_promueve() {
    let e = Explorador::nuevo();
    let mut f = fuente();
    f.0.estado = EstadoConsulta::Parcial;
    let i = e.explorar(&[&f]).unwrap();
    assert!(e.recibir_capacidad(&i, evidencia("gpu", 10)).is_err());
}
#[test]
fn biblioteca_ausente_no_prueba_dispositivo_ausente() {
    let e = Explorador::nuevo();
    let f = Fija(Consulta::fallo(
        "simulada",
        EstadoConsulta::NoDisponible,
        "Biblioteca no disponible".into(),
    ));
    let i = e.explorar(&[&f]).unwrap();
    assert_eq!(i.consultas[0].estado, EstadoConsulta::NoDisponible);
    assert!(i.consultas[0].observaciones.is_empty());
}
#[test]
fn inventario_de_otra_sesion_se_rechaza() {
    let a = Explorador::nuevo();
    let b = Explorador::nuevo();
    let i = a.explorar(&[&fuente()]).unwrap();
    assert!(b.recursos(&i, &[], 1000).is_err());
}
#[test]
fn capacidad_de_otra_sesion_se_rechaza() {
    let a = Explorador::nuevo();
    let b = Explorador::nuevo();
    let f = fuente();
    let ia = a.explorar(&[&f]).unwrap();
    let ib = b.explorar(&[&f]).unwrap();
    let cs = caps(&a, &ia);
    assert!(b.recursos(&ib, &cs, 1000).is_err());
}
#[test]
fn observacion_vencida_se_rechaza() {
    let e = Explorador::nuevo();
    let i = e.explorar(&[&fuente()]).unwrap();
    assert!(e.recursos_en(&i, &[], 5, i.observado_ms + 6).is_err());
}
#[test]
fn observacion_futura_se_rechaza() {
    let e = Explorador::nuevo();
    let mut i = e.explorar(&[&fuente()]).unwrap();
    i.observado_ms = 2;
    assert!(e.recursos_en(&i, &[], 100, 1).is_err());
}
#[test]
fn capacidad_vencida_se_rechaza() {
    let e = Explorador::nuevo();
    let i = e.explorar(&[&fuente()]).unwrap();
    let mut cs = caps(&e, &i);
    cs[0].vigencia_ms = 5;
    assert!(e.recursos_en(&i, &cs, 1000, 6).is_err());
}
#[test]
fn clase_desconocida_no_promueve() {
    let e = Explorador::nuevo();
    let mut f = fuente();
    f.0.observaciones[1].clase = Clase::AceleradorSinClasificar;
    let i = e.explorar(&[&f]).unwrap();
    assert!(e.recibir_capacidad(&i, evidencia("gpu", 10)).is_err());
}
#[test]
fn memoria_desconocida_impide_seleccion() {
    let e = Explorador::nuevo();
    let mut f = fuente();
    for o in &mut f.0.observaciones {
        o.memoria_libre_bytes = None;
    }
    let i = e.explorar(&[&f]).unwrap();
    let cs = caps(&e, &i);
    assert!(e.seleccionar(&i, &cs, s(), p()).is_err());
}
#[test]
fn cobertura_parcial_no_promueve() {
    let e = Explorador::nuevo();
    let i = e.explorar(&[&fuente()]).unwrap();
    let mut c = evidencia("gpu", 10);
    c.capacidad.cobertura = Cobertura::Parcial;
    assert!(e.recibir_capacidad(&i, c).is_err());
}
#[test]
fn carga_distinta_no_se_admite() {
    let e = Explorador::nuevo();
    let i = e.explorar(&[&fuente()]).unwrap();
    let cs = caps(&e, &i);
    let mut s = s();
    s.carga.revision = "otra".into();
    assert!(e.seleccionar(&i, &cs, s, p()).is_err());
}
#[test]
fn desaparicion_impide_efecto_sin_sustituir_cpu() {
    let e = Explorador::nuevo();
    let mut f = fuente();
    let i = e.explorar(&[&f]).unwrap();
    let cs = caps(&e, &i);
    let d = e.seleccionar(&i, &cs, s(), p()).unwrap();
    f.0.observaciones.pop();
    let efecto = Cell::new(false);
    assert!(e
        .ejecutar_revalidado(&[&f], &cs, d, |_| {
            efecto.set(true);
            Ok(())
        })
        .is_err());
    assert!(!efecto.get());
}
#[test]
fn sustitucion_dispositivo_impide_efecto() {
    let e = Explorador::nuevo();
    let mut f = fuente();
    let i = e.explorar(&[&f]).unwrap();
    let cs = caps(&e, &i);
    let d = e.seleccionar(&i, &cs, s(), p()).unwrap();
    f.0.observaciones[1].nombre = "otro dispositivo".into();
    assert!(e
        .ejecutar_revalidado(&[&f], &cs, d, |_| -> Result<(), String> {
            panic!("Efecto prohibido")
        })
        .is_err());
}
#[test]
fn memoria_insuficiente_impide_efecto_sin_sustitucion() {
    let e = Explorador::nuevo();
    let mut f = fuente();
    let i = e.explorar(&[&f]).unwrap();
    let cs = caps(&e, &i);
    let d = e.seleccionar(&i, &cs, s(), p()).unwrap();
    f.0.observaciones[1].memoria_libre_bytes = Some(1);
    assert!(e
        .ejecutar_revalidado(&[&f], &cs, d, |_| -> Result<(), String> {
            panic!("Efecto prohibido")
        })
        .is_err());
}
#[test]
fn no_clasifica_por_nombre_comercial() {
    assert_eq!(clase_pci(0x020000, "nvidia"), None);
    assert_eq!(clase_pci(0x030000, "amdgpu"), Some(Clase::Gpu));
    assert_eq!(
        clase_pci(0x120000, "desconocido"),
        Some(Clase::AceleradorSinClasificar)
    );
    assert_eq!(clase_pci(0x120000, "amdxdna"), Some(Clase::Npu));
    assert_eq!(clase_pci(0x120000, "ivpu"), Some(Clase::Npu));
}
struct PjrtFijo {
    plataforma: String,
    ds: Vec<Observacion>,
}
impl sondas::ConsultaTpu for PjrtFijo {
    fn plataforma(&self) -> Result<String, String> {
        Ok(self.plataforma.clone())
    }
    fn dispositivos(&self) -> Result<Vec<Observacion>, String> {
        Ok(self.ds.clone())
    }
}
#[test]
fn pjrt_rechaza_plataforma_cpu() {
    let f = PjrtFijo {
        plataforma: "cpu".into(),
        ds: vec![],
    };
    let c = sondas::Tpu(&f).consultar();
    assert_eq!(c.estado, EstadoConsulta::NoDisponible);
}
#[test]
fn pjrt_conserva_varios_dispositivos() {
    let f = PjrtFijo {
        plataforma: "tpu".into(),
        ds: vec![
            observacion("tpu0", Clase::Tpu),
            observacion("tpu1", Clase::Tpu),
        ],
    };
    let e = Explorador::nuevo();
    let i = e.explorar(&[&sondas::Tpu(&f)]).unwrap();
    assert_eq!(i.consultas[0].observaciones.len(), 2);
    assert!(e.recursos(&i, &[], 1000).unwrap().is_empty());
}
