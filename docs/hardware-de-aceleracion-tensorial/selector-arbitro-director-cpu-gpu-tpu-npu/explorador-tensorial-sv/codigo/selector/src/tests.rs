use super::*;

// Todos los dispositivos y tiempos de estas pruebas son simulados.
fn carga() -> Carga {
    Carga {
        contrato: "operacion-simulada".into(),
        revision: "r1".into(),
        perfil: "entero-exacto".into(),
        dimensiones: vec![36, 3],
    }
}
fn recurso(clase: Clase, id: &str) -> Recurso {
    Recurso {
        destino: Destino {
            servidor: "servidor-simulado".into(),
            dispositivo: id.into(),
            adaptador_revision: "simulado-r1".into(),
        },
        clase,
        estado: Estado::Comprobado,
        observado_ms: 100,
        disponible: true,
        memoria_disponible_bytes: Some(1000),
        capacidades: vec![Capacidad {
            carga: carga(),
            cobertura: Cobertura::Completa,
            evidencia: "evidencia-simulada".into(),
            estimacion_total_us: None,
        }],
    }
}
fn solicitud() -> Solicitud {
    Solicitud {
        id: "solicitud-simulada".into(),
        carga: carga(),
        memoria_adicional_bytes: 400,
        margen_bytes: 100,
        residencia_obligatoria: None,
    }
}
fn politica(recursos: &[Recurso]) -> Politica {
    Politica {
        revision: "politica-simulada-r1".into(),
        destinos_permitidos: recursos.iter().map(|r| r.destino.clone()).collect(),
        preferencia: vec![Clase::Gpu, Clase::Tpu, Clase::Npu, Clase::Cpu],
        clase_obligatoria: None,
        vigencia_ms: 50,
        comparar_estimaciones: false,
    }
}
fn motivo(mut r: Recurso, cambio: impl FnOnce(&mut Recurso), esperado: Motivo) {
    let p = politica(&[r.clone()]);
    cambio(&mut r);
    let Error::NingunDestino(d) = seleccionar(&solicitud(), &p, &[r], 110).unwrap_err() else {
        panic!("tipo de rechazo incorrecto")
    };
    assert_eq!(d[0].motivo, esperado);
}

#[test]
fn cpu_y_gpu_disponibles_se_respeta_la_preferencia() {
    let rs = vec![recurso(Clase::Cpu, "cpu"), recurso(Clase::Gpu, "gpu")];
    let d = seleccionar(&solicitud(), &politica(&rs), &rs, 110).unwrap();
    assert_eq!(d.clase, Clase::Gpu);
    assert_eq!(d.solicitud, "solicitud-simulada");
}
#[test]
fn cpu_solo_si_la_politica_la_admite() {
    let rs = vec![recurso(Clase::Cpu, "cpu")];
    let mut p = politica(&rs);
    assert_eq!(
        seleccionar(&solicitud(), &p, &rs, 110).unwrap().clase,
        Clase::Cpu
    );
    p.clase_obligatoria = Some(Clase::Gpu);
    assert!(matches!(
        seleccionar(&solicitud(), &p, &rs, 110),
        Err(Error::NingunDestino(_))
    ));
}
#[test]
fn tpu_y_npu_no_tienen_preferencia_intrinseca() {
    for clase in [Clase::Tpu, Clase::Npu] {
        let rs = vec![recurso(Clase::Gpu, "gpu"), recurso(clase, "acelerador")];
        let mut p = politica(&rs);
        p.preferencia = vec![clase, Clase::Gpu];
        assert_eq!(
            seleccionar(&solicitud(), &p, &rs, 110).unwrap().clase,
            clase
        );
    }
}
#[test]
fn chip_detectado_no_acredita_capacidad() {
    for estado in [Estado::Detectado, Estado::DisponibleSinContraste] {
        motivo(
            recurso(Clase::Npu, "npu"),
            |r| r.estado = estado,
            Motivo::NoComprobado,
        );
    }
}
#[test]
fn misma_clase_con_distinta_revision_o_precision_se_rechaza() {
    motivo(
        recurso(Clase::Gpu, "gpu"),
        |r| r.capacidades[0].carga.revision = "r2".into(),
        Motivo::CargaNoComprobada,
    );
    motivo(
        recurso(Clase::Gpu, "gpu"),
        |r| r.capacidades[0].carga.perfil = "aproximado".into(),
        Motivo::CargaNoComprobada,
    );
    motivo(
        recurso(Clase::Gpu, "gpu"),
        |r| r.capacidades[0].carga.dimensiones = vec![49, 3],
        Motivo::CargaNoComprobada,
    );
}
#[test]
fn calculo_parcial_npu_cpu_no_pasa_por_npu_completa() {
    motivo(
        recurso(Clase::Npu, "npu"),
        |r| r.capacidades[0].cobertura = Cobertura::Parcial,
        Motivo::CoberturaParcial,
    );
}
#[test]
fn vigencia_y_reloj() {
    motivo(
        recurso(Clase::Gpu, "gpu"),
        |r| r.observado_ms = 10,
        Motivo::ObservacionVencida,
    );
    motivo(
        recurso(Clase::Gpu, "gpu"),
        |r| r.observado_ms = 111,
        Motivo::ObservacionFutura,
    );
}
#[test]
fn no_se_suman_memorias_de_tarjetas() {
    let mut rs = vec![recurso(Clase::Gpu, "g0"), recurso(Clase::Gpu, "g1")];
    for r in &mut rs {
        r.memoria_disponible_bytes = Some(300);
    }
    let Error::NingunDestino(d) = seleccionar(&solicitud(), &politica(&rs), &rs, 110).unwrap_err()
    else {
        panic!()
    };
    assert_eq!(d.len(), 2);
    assert!(d.iter().all(|d| d.motivo == Motivo::MemoriaInsuficiente));
}
#[test]
fn memoria_desconocida_no_equivale_a_disponible() {
    motivo(
        recurso(Clase::Cpu, "cpu"),
        |r| r.memoria_disponible_bytes = None,
        Motivo::MemoriaDesconocida,
    );
}
#[test]
fn cotas_de_memoria_y_desbordamiento() {
    let rs = vec![recurso(Clase::Gpu, "gpu")];
    let p = politica(&rs);
    let mut s = solicitud();
    s.memoria_adicional_bytes = 900;
    assert!(seleccionar(&s, &p, &rs, 110).is_ok());
    s.memoria_adicional_bytes = 901;
    assert!(seleccionar(&s, &p, &rs, 110).is_err());
    s.memoria_adicional_bytes = u64::MAX;
    assert_eq!(
        seleccionar(&s, &p, &rs, 110),
        Err(Error::DesbordamientoMemoria)
    );
}
#[test]
fn disponibilidad_y_evidencia() {
    motivo(
        recurso(Clase::Gpu, "gpu"),
        |r| r.disponible = false,
        Motivo::NoDisponible,
    );
    motivo(
        recurso(Clase::Gpu, "gpu"),
        |r| r.capacidades[0].evidencia.clear(),
        Motivo::EvidenciaAusente,
    );
}
#[test]
fn ningun_servidor_ni_adaptador_se_agrega_por_inferencia() {
    motivo(
        recurso(Clase::Gpu, "gpu"),
        |r| r.destino.servidor = "otro".into(),
        Motivo::NoPermitido,
    );
    motivo(
        recurso(Clase::Gpu, "gpu"),
        |r| r.destino.adaptador_revision = "r2".into(),
        Motivo::NoPermitido,
    );
}
#[test]
fn sesion_residente_no_migra_silenciosamente() {
    let rs = vec![recurso(Clase::Cpu, "cpu"), recurso(Clase::Gpu, "gpu")];
    let mut s = solicitud();
    s.residencia_obligatoria = Some(rs[0].destino.clone());
    assert_eq!(
        seleccionar(&s, &politica(&rs), &rs, 110).unwrap().clase,
        Clase::Cpu
    );
}
#[test]
fn tiempo_total_puede_preferir_cpu_para_carga_pequena() {
    let mut rs = vec![recurso(Clase::Cpu, "cpu"), recurso(Clase::Gpu, "gpu")];
    rs[0].capacidades[0].estimacion_total_us = Some(10);
    rs[1].capacidades[0].estimacion_total_us = Some(100);
    let mut p = politica(&rs);
    p.comparar_estimaciones = true;
    let d = seleccionar(&solicitud(), &p, &rs, 110).unwrap();
    assert_eq!(d.clase, Clase::Cpu);
    assert_eq!(d.criterio, Criterio::MenorTiempoTotalEstimado);
}
#[test]
fn sin_tiempos_completos_no_se_inventa_optimo() {
    let mut rs = vec![recurso(Clase::Cpu, "cpu"), recurso(Clase::Gpu, "gpu")];
    rs[0].capacidades[0].estimacion_total_us = Some(1);
    let mut p = politica(&rs);
    p.comparar_estimaciones = true;
    let d = seleccionar(&solicitud(), &p, &rs, 110).unwrap();
    assert_eq!(d.clase, Clase::Gpu);
    assert_eq!(d.criterio, Criterio::PreferenciaPorEstimacionesIncompletas);
}
#[test]
fn empate_independiente_del_orden_de_inventario() {
    let mut rs = vec![recurso(Clase::Gpu, "g1"), recurso(Clase::Gpu, "g0")];
    let p = politica(&rs);
    let d = seleccionar(&solicitud(), &p, &rs, 110).unwrap();
    rs.reverse();
    assert_eq!(d, seleccionar(&solicitud(), &p, &rs, 110).unwrap());
}
#[test]
fn inventario_ambiguo_se_rechaza() {
    let r = recurso(Clase::Gpu, "gpu");
    let mut rs = vec![r.clone(), r];
    let p = politica(&rs[..1]);
    assert_eq!(
        seleccionar(&solicitud(), &p, &rs, 110),
        Err(Error::InventarioAmbiguo)
    );
    rs.pop();
    let repetida = rs[0].capacidades[0].clone();
    rs[0].capacidades.push(repetida);
    assert_eq!(
        seleccionar(&solicitud(), &p, &rs, 110),
        Err(Error::InventarioAmbiguo)
    );
}
#[test]
fn entradas_incompletas_no_entran_en_seleccion() {
    let rs = vec![recurso(Clase::Gpu, "gpu")];
    let mut p = politica(&rs);
    let mut s = solicitud();
    s.carga.revision.clear();
    assert_eq!(
        seleccionar(&s, &p, &rs, 110),
        Err(Error::SolicitudIncompleta)
    );
    p.preferencia.push(Clase::Gpu);
    assert_eq!(
        seleccionar(&solicitud(), &p, &rs, 110),
        Err(Error::PoliticaInvalida)
    );
}
