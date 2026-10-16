//! Cotejo independiente de los archivos recuperados; no usa el controlador GPU.
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};
use sv_ensayo_gpu_kaggle::{R, casos, referencia};

fn leer(p: &Path) -> R<Value> {
    serde_json::from_slice(&fs::read(p).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}
fn exigir(ok: bool, mensaje: &str) -> R<()> {
    if ok { Ok(()) } else { Err(mensaje.into()) }
}
fn cotejar(dir: &Path) -> R<Value> {
    let manifiesto = leer(&dir.join("INTEGRIDAD.json"))?;
    let mut nombres = BTreeSet::new();
    let mut bytes = 0u64;
    for f in manifiesto["ficheros"].as_array().ok_or("Inventario ausente")? {
        let n = f["archivo"].as_str().ok_or("Nombre ausente")?;
        exigir(!n.contains(['/', '\\']) && n != "." && n != "..", "Ruta no admitida")?;
        exigir(nombres.insert(n.to_owned()), "Archivo duplicado")?;
        let b = fs::read(dir.join(n)).map_err(|e| e.to_string())?;
        exigir(f["bytes"] == b.len() as u64, "Tamaño discordante")?;
        exigir(f["sha256"] == format!("{:x}", Sha256::digest(&b)), "Huella discordante")?;
        bytes += b.len() as u64;
    }
    for f in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let f = f.map_err(|e| e.to_string())?;
        if f.file_type().map_err(|e| e.to_string())?.is_file() {
            let n = f.file_name().to_string_lossy().into_owned();
            exigir(n == "INTEGRIDAD.json" || nombres.contains(&n), "Archivo sin inventariar")?;
        }
    }
    let resultado = leer(&dir.join("RESULTADO.json"))?;
    exigir(resultado["calculo_conforme"] == true && resultado["seleccion_conforme"] == true, "Ensayo no completo")?;
    let dispositivo = leer(&dir.join("DISPOSITIVO.json"))?;
    let gpu = format!("amd-{}", dispositivo["uuid"].as_str().ok_or("UUID ausente")?);
    let decisiones = leer(&dir.join("DECISIONES.json"))?;
    let decisiones = decisiones.as_array().ok_or("Decisiones ausentes")?;
    exigir(decisiones.len() == 18, "Número de decisiones discordante")?;
    let rechazos = leer(&dir.join("RECHAZOS.json"))?;
    exigir(rechazos.as_array().is_some_and(|v| v.len() == 12), "Número de rechazos discordante")?;
    let mut operaciones = 0u64;
    let mut lanzamientos = 0u64;
    let mut incompletas = 0u64;
    let mut filas = Vec::new();
    for s in casos() {
        let esperado = referencia(&s)?;
        let mut cpu_cal = Vec::new();
        let mut gpu_cal = Vec::new();
        let mut modos = vec!["primera-gpu".to_owned()];
        for r in 0..5 { for d in ["cpu", "gpu"] { modos.push(format!("calibracion-{d}-{r}")); } }
        modos.extend(["cpu", "gpu", "automatico"].into_iter().map(str::to_owned));
        for modo in modos {
            let nombre = format!("{}-{modo}.json", s.id);
            exigir(nombres.contains(&nombre), "Operación sin huella")?;
            let v = leer(&dir.join(&nombre))?;
            exigir(v["entrada"] == serde_json::to_value(&s).unwrap(), "Entrada alterada")?;
            exigir(v["salida"] == json!(esperado), "Resultado numérico discordante")?;
            let es_gpu = v["destino"] == gpu;
            if modo.starts_with("calibracion-") {
                let t = v["tiempos"]["total_us"].as_u64().ok_or("Tiempo completo ausente")?;
                if es_gpu { gpu_cal.push(t); } else { cpu_cal.push(t); }
            }
            exigir(es_gpu || v["destino"] == "cpu-anfitrion", "Destino desconocido")?;
            if modo != "automatico" {
                exigir(es_gpu == modo.contains("gpu"), "Destino obligado discordante")?;
            }
            exigir(v["lanzamientos_gpu_observados"] == u64::from(es_gpu), "Lanzamientos discordantes")?;
            if es_gpu {
                exigir(v["tiempos"]["metodo_gpu"] == "device", "Temporización GPU ausente")?;
                exigir(v["tiempos"]["gpu_eventos_ns"].as_u64().is_some_and(|n| n > 0), "Duración GPU no positiva")?;
                exigir(v["tiempos"]["pci"] == dispositivo["pci"], "PCI discordante")?;
                exigir(v["tiempos"]["uuid"] == dispositivo["uuid"], "UUID discordante")?;
            } else { exigir(v["tiempos"]["gpu_eventos_ns"].is_null(), "Duración GPU atribuida a CPU")?; }
            exigir(v["cpu_antes"]["pid"] == v["cpu_despues"]["pid"], "Proceso discordante")?;
            let t0 = v["cpu_antes"]["cpu_acumulada_ms"].as_u64().ok_or("Tiempo CPU inicial ausente")?;
            let t1 = v["cpu_despues"]["cpu_acumulada_ms"].as_u64().ok_or("Tiempo CPU final ausente")?;
            exigir(t1 >= t0, "Tiempo CPU decreciente")?;
            let muestras = v["muestreo"]["muestras"].as_array().ok_or("Muestras ausentes")?;
            exigir(muestras.len() >= 2, "Muestreo insuficiente")?;
            let completa = muestras.iter().all(|m| m["cpu"]["cpu_acumulada_ms"].is_u64() && m["gpu_ocupacion_porcentaje"]["valor"].as_u64().is_some_and(|n|n<=100) && m["gpu_memoria_usada_bytes"]["valor"].as_u64().is_some_and(|n|n<=dispositivo["memoria_total_bytes"].as_u64().unwrap()));
            exigir(v["muestreo"]["completa"] == completa, "Cobertura de muestreo discordante")?;
            incompletas += u64::from(v["muestreo"]["completa"] != true);
            operaciones += 1;
            lanzamientos += u64::from(es_gpu);
        }
        let ds: Vec<_> = decisiones.iter().filter(|d| d["caso"] == s.id).collect();
        exigir(cpu_cal.len()==5 && gpu_cal.len()==5, "Calibración incompleta")?;
        cpu_cal.sort_unstable();
        gpu_cal.sort_unstable();
        exigir(ds.len() == 3, "Decisiones por caso discordantes")?;
        for modo in ["cpu", "gpu", "automatico"] {
            let ds_m: Vec<_> = ds.iter().filter(|d| d["modo"] == modo).collect();
            exigir(ds_m.len() == 1, "Modo de selección duplicado o ausente")?;
            let d = ds_m[0];
            let mc = d["estimacion_cpu_us"].as_u64().ok_or("Mediana CPU ausente")?;
            let mg = d["estimacion_gpu_us"].as_u64().ok_or("Mediana GPU ausente")?;
            exigir(mc==cpu_cal[2] && mg==gpu_cal[2], "Medianas no reproducidas por los registros")?;
            let elegido = if modo == "gpu" || (modo == "automatico" && mg < mc) { gpu.as_str() } else { "cpu-anfitrion" };
            exigir(d["destino"] == elegido, "Selección no corresponde a política y medianas")?;
            let v = leer(&dir.join(format!("{}-{modo}.json", s.id)))?;
            exigir(v["destino"] == elegido && v["tiempos"]["total_us"] == d["medicion_us"], "Decisión y ejecución discordantes")?;
        }
        for tipo in ["destino_inexistente", "memoria_imposible"] {
            exigir(rechazos.as_array().unwrap().iter().filter(|r| r["caso"] == s.id && r["control"] == tipo && !r["motivo"].is_null()).count() == 1, "Rechazo por caso discordante")?;
        }
        let c = leer(&dir.join(format!("{}-cpu.json", s.id)))?;
        let g = leer(&dir.join(format!("{}-gpu.json", s.id)))?;
        let a = ds.iter().find(|d| d["modo"] == "automatico").unwrap();
        filas.push(json!({"caso":s.id,"dimensiones":[s.filas,s.interior,s.columnas],"cpu_us":c["tiempos"]["total_us"],"gpu_total_us":g["tiempos"]["total_us"],"gpu_eventos_ns":g["tiempos"]["gpu_eventos_ns"],"seleccion_automatica":a["destino"],"mediana_cpu_us":a["estimacion_cpu_us"],"mediana_gpu_us":a["estimacion_gpu_us"]}));
    }
    exigir(operaciones == 84 && resultado["lanzamientos_gpu"] == lanzamientos, "Contabilidad discordante")?;
    exigir(resultado["operaciones_con_telemetria_incompleta"] == incompletas, "Telemetría discordante")?;
    Ok(json!({"integridad_conforme":true,"calculo_cotejado":true,"seleccion_cotejada":true,"operaciones":operaciones,"lanzamientos_gpu":lanzamientos,"decisiones":18,"rechazos":12,"archivos":nombres.len(),"bytes":bytes,"telemetrias_incompletas":incompletas,"dispositivo":dispositivo,"casos":filas}))
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let r = if args.len() == 2 { cotejar(Path::new(&args[1])) } else { Err("Uso: cotejar DIRECTORIO".into()) };
    match r { Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()), Err(e) => { eprintln!("{e}"); std::process::exit(1); } }
}
