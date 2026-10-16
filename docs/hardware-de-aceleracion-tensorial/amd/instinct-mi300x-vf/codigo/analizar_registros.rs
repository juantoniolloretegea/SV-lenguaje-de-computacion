//! Análisis retrospectivo: sólo lee resultados; no inicializa HIP ni ejecuta cálculo GPU.
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

fn n(v: &Value, k: &str) -> u64 { v[k].as_u64().expect(k) }
fn resumen(v: &[u64]) -> Value {
    let mut v = v.to_vec(); v.sort_unstable();
    json!({"n":v.len(),"minimo":v[0],"mediana":v[v.len()/2],"maximo":v[v.len()-1]})
}
fn ejecutar(dir: &Path) -> Value {
    let mut filas = Vec::new();
    let mut archivos: Vec<_> = fs::read_dir(dir).unwrap().map(|e|e.unwrap().path()).collect();
    archivos.sort();
    for p in archivos {
        if p.extension().is_none_or(|e|e != "json") { continue; }
        let bytes = fs::read(&p).unwrap();
        let v: Value = serde_json::from_slice(&bytes).unwrap();
        if !v["muestreo"].is_object() { continue; }
        let ms = v["muestreo"]["muestras"].as_array().unwrap();
        let a = ms.first().unwrap(); let z = ms.last().unwrap();
        let delta = |a: u64, b: u64| b as i64-a as i64;
        filas.push(json!({"archivo":p.file_name().unwrap().to_str().unwrap(),
            "sha256":format!("{:x}",Sha256::digest(&bytes)),"id":v["id"],"caso":v["caso"],
            "gpu":n(&v["tiempos"],"lanzamientos_gpu")==1,"tiempos":v["tiempos"],
            "resultado_bytes":v["salida"].as_array().unwrap().len()*4,
            "rss_delta_bytes":delta(n(&a["cpu"],"rss_bytes"),n(&z["cpu"],"rss_bytes")),
            "vram_global_delta_bytes":delta(n(&a["gpu_memoria_usada_bytes"],"valor"),n(&z["gpu_memoria_usada_bytes"],"valor")),
            "cpu_proceso_delta_ms":delta(n(&v["cpu_antes"],"cpu_acumulada_ms"),n(&v["cpu_despues"],"cpu_acumulada_ms")),
            "muestras":ms.len(),"instantes_muestreo_us":ms.iter().map(|m|n(m,"instante_us")).collect::<Vec<_>>()
        }));
    }
    assert_eq!(filas.len(),84);
    let gpu: Vec<_> = filas.iter().filter(|v|v["gpu"]==true).collect(); assert_eq!(gpu.len(),43);
    let carga = |id: &str| *gpu.iter().find(|v|v["id"]==id).unwrap();
    let forzada=carga("carga-gpu"); let auto=carga("carga-automatico");
    let recuperacion:Vec<_>=gpu.iter().map(|v|n(&v["tiempos"],"recuperacion_us")).collect();
    let regulares:Vec<_>=gpu.iter().filter(|v|v["id"]!="carga-gpu").map(|v|n(&v["tiempos"],"recuperacion_us")).collect();
    let cpu=filas.iter().find(|v|v["id"]=="carga-cpu").unwrap();
    json!({"operaciones":filas.len(),"gpu":gpu.len(),"cpu":filas.len()-gpu.len(),
        "recuperacion_gpu_us":resumen(&recuperacion),"recuperacion_gpu_sin_carga_forzada_us":resumen(&regulares),
        "carga_forzada":forzada,"carga_automatica":auto,
        "recuperacion_fraccion_tiempo_total_carga_forzada":n(&forzada["tiempos"],"recuperacion_us") as f64/n(&forzada["tiempos"],"total_us") as f64,
        "carga_cpu_total_us":n(&cpu["tiempos"],"total_us"),
        "carga_gpu_forzada_sobre_cpu":n(&forzada["tiempos"],"total_us") as f64/n(&cpu["tiempos"],"total_us") as f64,
        "operaciones_detalladas":filas})
}
fn main() {
    let args:Vec<_>=std::env::args().collect(); assert_eq!(args.len(),3);
    let r:Vec<_>=args[1..].iter().map(|d|ejecutar(Path::new(d))).collect();
    println!("{}",serde_json::to_string_pretty(&json!({"metodo":"Análisis retrospectivo de los dos registros originales; sin acceso a HIP ni hardware remoto", "unidad_tiempo":"microsegundos, salvo campos explícitos ns o ms", "limite":"Los incrementos de memoria abarcan la operación completa, no exclusivamente la recuperación; VRAM global", "ejecuciones":r})).unwrap());
}
