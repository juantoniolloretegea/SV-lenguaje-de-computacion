use explorador_tensorial_sv::{sondas, Explorador, Sonda};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 1 {
        return Err("Uso: explorador-tensorial-sv; inventario JSON por salida estándar".into());
    }
    let e = Explorador::nuevo();
    let ss: [&dyn Sonda; 4] = [
        &sondas::Cpu,
        &sondas::Sistema,
        &sondas::Nvidia,
        &sondas::SinClienteTpu,
    ];
    let i = e.explorar(&ss)?;
    println!("{}", serde_json::to_string_pretty(&i)?);
    Ok(())
}
