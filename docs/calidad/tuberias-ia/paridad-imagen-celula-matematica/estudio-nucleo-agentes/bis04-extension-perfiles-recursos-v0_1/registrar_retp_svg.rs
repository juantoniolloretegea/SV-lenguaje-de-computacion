//! ES: Incorpora RETP-238/239 con control de sucesión y prefijos intactos.
//! EN: Adds RETP-238/239 with sequence control and intact historical prefixes.
#![forbid(unsafe_code)]
#[allow(dead_code)]
#[path="../../../../riesgos-materiales-s26/r08/auxiliar.rs"]
mod auxiliar;
use std::{fs,path::Path};
fn main(){
 let a:Vec<_>=std::env::args().collect();assert_eq!(a.len(),5);
 let d=Path::new(&a[1]).join("docs/calidad");
 let csv=d.join("REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv");let md=d.join("REGISTRO_EVOLUCION_TECNICA_PROYECTO.md");
 let old=fs::read(&csv).unwrap();let rows=auxiliar::csv(&old).unwrap();
 let previous=rows.last().unwrap()[0].strip_prefix("RETP-2026-").unwrap().parse::<u32>().unwrap();
 let text=fs::read_to_string(&a[2]).unwrap();let fields:Vec<_>=text.lines().map(str::to_string).collect();
 assert_eq!(fields.len(),14);assert_eq!(fields[4],"S22");
 assert!(matches!(fields[0].as_str(),"RETP-2026-238"|"RETP-2026-239"));
 assert_eq!(fields[0],format!("RETP-2026-{}",previous.checked_add(1).unwrap()));
 let mut new=old.clone();if !new.ends_with(b"\n"){new.push(b'\n');}
 let row=fields.iter().map(|s|format!("\"{}\"",s.replace('"',"\"\""))).collect::<Vec<_>>().join(",")+"\n";
 new.extend(row.as_bytes());let parsed=auxiliar::csv(&new).unwrap();assert_eq!(&parsed[..rows.len()],&rows);assert_eq!(parsed.last().unwrap(),&fields);
 let om=fs::read(&md).unwrap();let mut nm=om.clone();nm.extend(fs::read(&a[3]).unwrap());
 let backup=Path::new(&a[4]);fs::create_dir(backup).unwrap();
 for(p,b)in[(&csv,&old),(&md,&om)]{fs::write(backup.join(p.file_name().unwrap()),b).unwrap();}
 for(p,b)in[(&csv,&new),(&md,&nm)]{fs::write(p,b).unwrap();assert_eq!(&fs::read(p).unwrap(),b);}
 println!("{} añadido: 14 columnas y prefijos CSV/MD conservados.",fields[0]);
}
