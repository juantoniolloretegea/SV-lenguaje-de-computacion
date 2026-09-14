//! Actualización administrativa S29 r0 -> r1, con historial conservado.
#![forbid(unsafe_code)]
#[allow(dead_code)]
#[path="../../riesgos-materiales-s26/r08/auxiliar.rs"] mod auxiliar;
use std::{fs,path::Path,collections::BTreeMap};
fn row(r:&[String])->String{r.iter().map(|v|format!("\"{}\"",v.replace('"',"\"\""))).collect::<Vec<_>>().join(",")+"\n"}
fn check(c:&[Vec<String>],h:&[Vec<String>],md:&str)->Result<(),Box<dyn std::error::Error>>{
 let mut last=BTreeMap::new();
 for r in &h[1..]{let n=r[0].parse::<usize>()?;let e=last.entry(&r[1]).or_insert((n,&r[1..]));if n>e.0{*e=(n,&r[1..]);}}
 for r in &c[1..]{if last.get(&r[0]).is_none_or(|x|x.1!=r){return Err("historial discordante".into());}
 let start=md.find(&format!("## {} · ",r[0])).ok_or("seccion ausente")?;
 let end=md[start+1..].find("\n## ").map(|x|start+1+x+1).unwrap_or(md.len());let block=&md[start..end];
 for(k,v)in c[0].iter().zip(r){if k=="id"||k=="actividad"{continue;}
 let p=format!("**{k}:** ");let values:Vec<_>=block.lines().filter_map(|l|l.strip_prefix(&p)).collect();
 if values!=vec![if v.is_empty(){"—"}else{v.as_str()}]{return Err("Markdown discordante".into());}}
 }Ok(())
}
fn run()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<_>=std::env::args().collect();if a.len()!=3{return Err("uso: registrar ROOT FECHA_UTC".into());}
 let dir=Path::new(&a[1]).join("docs/calidad/Inventario-sv/sucesos");
 let old=fs::read_to_string(dir.join("SUCESOS_SV.csv"))?;let history=fs::read_to_string(dir.join("HISTORIAL_SUCESOS_SV.csv"))?;let md=fs::read_to_string(dir.join("SUCESOS_SV.md"))?;
 let mut c=auxiliar::csv(old.as_bytes())?;let h=auxiliar::csv(history.as_bytes())?;check(&c,&h,&md)?;
 let ix=c.iter().position(|r|r[0]=="S29").ok_or("S29")?;
 let rh:Vec<_>=h.iter().filter(|r|r.get(1).is_some_and(|s|s=="S29")).collect();if rh.len()!=1||rh[0][0]!="0"||c.len()!=31{return Err("revision no aplicable".into());}
 let header=c[0].clone();let r=&mut c[ix];
 for(k,v)in[
 ("fecha_actualizacion_utc",a[2].as_str()),("fecha_fin_utc",a[2].as_str()),
 ("resultado","Rustup 1.29.1 instalado en /opt/sv-cargo/bin/rustup; RUSTUP_HOME=/opt/sv-rustup y CARGO_HOME=/opt/sv-cargo. Toolchain personalizado sv-1.98.0 enlazado a /opt/sv-rust-1.98.0 y predeterminado. Rust/Cargo 1.98.0 conservados. Obligación Rust vigente; GUI C#/.NET diferida a S24."),
 ("verificacion","ELF Linux x86-64 y SHA-256 dda7234360b7f578ca8b0ddcb80145646fa61a67c1720a5abc7051b35c9fcb71 coincidente con archivo oficial. Instalación sin descargar toolchain, enlace y selección con salida 0. Inventario Rust renovado con salida 0, versiones y destino real del enlace cotejados. CSV/Markdown/historial concordantes."),
 ("evidencias","https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/recuperacion-rust-s25/RECEPCION_RUSTUP_S29.md ; https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/recuperacion-rust-s25/ESTADO_RUST.md"),
 ("referencia_calidad","RECEPCION_RUSTUP_S29.md, env-rustup.sh, estado_rust.rs y registrar_rustup_s29.rs en recuperacion-rust-s25; espejo de laboratorio en la misma recepción."),
 ("siguiente_accion","Activar mediante . /opt/sv-rustup/env.sh en cada consola que lo necesite. Continuar S26 desde revisión 25 sin repetir LIG; provisionar y verificar componentes requeridos por cada etapa. C#/.NET conserva S24."),
 ("observaciones","Revisión 1 posterior a la instalación autorizada. Se conserva revisión 0. Primer --help rechazado por nombre UUID del adjunto; corregido a rustup-init sin cambiar bytes. Advertencia del instalador por Rust preexistente conservada. Sin Python. Herramientas de sistema y coordinación del conector declaradas, sin atribuirles implementación Rust. No acredita instalación de componentes adicionales, persistencia tras reemplazo del contenedor ni migración de auxiliares históricos. RETP sin cambios; no nuevas ramas ni directorios versionados.")
 ]{let p=header.iter().position(|s|s==k).ok_or("campo")?;r[p]=v.into();}
 let lines:Vec<_>=old.split_inclusive('\n').collect();if lines.len()!=c.len(){return Err("CSV multilinea no previsto".into());}
 let mut out=String::new();for(i,l)in lines.iter().enumerate(){if i==ix{out+=&row(&c[ix]);}else{out+=l;}}
 let mut rev=vec!["1".into()];rev.extend(c[ix].clone());let updated_history=history.clone()+&row(&rev);
 let start=md.find("## S29 · ").ok_or("S29 MD")?;if md[start+1..].contains("\n## "){return Err("S29 no terminal".into());}
 let act=header.iter().position(|s|s=="actividad").unwrap();let mut block=format!("## S29 · {}\n\n",c[ix][act]);
 for(k,v)in header.iter().zip(&c[ix]){if k=="id"||k=="actividad"{continue;}block+=&format!("**{k}:** {}\n\n",if v.is_empty(){"—"}else{v});}
 let updated_md=md[..start].to_string()+block.trim_end()+"\n";
 let parsed=auxiliar::csv(out.as_bytes())?;check(&parsed,&auxiliar::csv(updated_history.as_bytes())?,&updated_md)?;
 let old_rows=auxiliar::csv(old.as_bytes())?;for i in 0..parsed.len(){if i!=ix&&parsed[i]!=old_rows[i]{return Err("fila ajena modificada".into());}}
 if !updated_history.starts_with(&history){return Err("historia perdida".into());}
 for(n,v)in[("SUCESOS_SV.csv",out),("HISTORIAL_SUCESOS_SV.csv",updated_history),("SUCESOS_SV.md",updated_md)]{fs::write(dir.join(n),&v)?;if fs::read_to_string(dir.join(n))?!=v{return Err("relectura".into());}}
 println!("S29 revision 1; 30 sucesos concordantes; antecedentes y filas ajenas conservados");Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}
