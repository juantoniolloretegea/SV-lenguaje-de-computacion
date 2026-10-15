//! Ensayo de observación: la referencia independiente no alimenta al Selector.
use explorador_tensorial_sv::{sondas, Explorador, Sonda};
use serde::Deserialize;
use std::collections::BTreeSet;

#[derive(Deserialize)]
struct Referencia {
    cpu: Vec<String>,
    gpu: Vec<String>,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 2 {
        return Err("Uso: contraste_fisico referencia.json".into());
    }
    let referencia: Referencia = serde_json::from_slice(&std::fs::read(&a[1])?)?;
    let e = Explorador::nuevo();
    let sondas: [&dyn Sonda; 4] = [
        &sondas::Cpu,
        &sondas::Sistema,
        &sondas::Nvidia,
        &sondas::SinClienteTpu,
    ];
    let mut tiempos = vec![];
    let mut fallos = 0;
    let mut capacidades = 0;
    let mut final_i = None;
    for _ in 0..20 {
        let i = e.explorar(&sondas)?;
        let nombres = |fuente: &str| -> BTreeSet<String> {
            i.consultas()
                .iter()
                .filter(|q| q.fuente == fuente)
                .flat_map(|q| q.observaciones.iter().map(|o| o.nombre.clone()))
                .collect()
        };
        let cpu: BTreeSet<_> = referencia.cpu.iter().cloned().collect();
        let gpu: BTreeSet<_> = referencia.gpu.iter().cloned().collect();
        if nombres("cpu-sistema") != cpu || (cfg!(windows) && nombres("windows-pnp") != gpu) {
            fallos += 1;
        }
        capacidades += e.recursos(&i, &[], 10_000)?.len();
        tiempos.push(i.duracion_us());
        final_i = Some(i);
    }
    let i = final_i.ok_or("Sin muestras")?;
    let mut orden = tiempos.clone();
    orden.sort();
    let consultas:Vec<_>=i.consultas().iter().map(|q|serde_json::json!({
        "fuente":q.fuente,"estado":q.estado,"observaciones":q.observaciones.iter().map(|o|serde_json::json!({
            "clase":o.clase,"nombre":o.nombre,"acceso":o.acceso,
            "memoria_total_bytes":o.memoria_total_bytes,"memoria_libre_bytes":o.memoria_libre_bytes
        })).collect::<Vec<_>>()
    })).collect();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "sistema":std::env::consts::OS,"muestras":20,"duraciones_us":tiempos,"minimo_us":orden[0],
            "mediana_us":(orden[9]+orden[10]) as f64/2.0,"maximo_us":orden[19],
            "discordancias_referencia":fallos,"capacidades_creadas_por_deteccion":capacidades,
            "alcance":"Observación de CPU y dispositivos declarados por el sistema; sin cálculo acelerado ni inferencia",
            "referencia":if cfg!(windows){"CIM Win32_Processor y Win32_VideoController"}else{"/proc/cpuinfo; PCI no cotejado por nombre comercial"},
            "consultas":consultas
        }))?
    );
    if fallos != 0 || capacidades != 0 {
        return Err("Contraste no conforme".into());
    }
    Ok(())
}
