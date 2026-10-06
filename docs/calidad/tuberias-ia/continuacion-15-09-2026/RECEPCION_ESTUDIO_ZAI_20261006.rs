use std::{fs,path::{Path,PathBuf},process::Command,io::Write};
use serde_json::{Value,json};use sha2::{Sha256,Digest};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn save(p:&Path,b:&[u8])->R<()>{fs::create_dir_all(p.parent().unwrap())?;fs::OpenOptions::new().write(true).create_new(true).open(p)?.write_all(b)?;Ok(())}
fn main()->R<()>{
 let st=PathBuf::from(std::env::args().nth(1).ok_or("ruta")?).canonicalize()?;
 assert!(st.starts_with(std::env::current_dir()?.canonicalize()?));
 assert_eq!(hash(b"abc"),"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
 let data:Value=serde_json::from_slice(&fs::read(st.join("RECUPERADO.json"))?)?;
 let files=data["archivos"].as_array().unwrap();assert_eq!(files.len(),16);
 let get=|n:&str|files.iter().find(|x|x["path"]==n).unwrap()["content"].as_str().unwrap();
 let mut entries=vec![];let mut total=0;
 for f in files{let n=f["path"].as_str().unwrap();assert!(!n.contains('/')&&!n.contains('\\')&&!n.contains(".."));let b=f["content"].as_str().unwrap().as_bytes();
 save(&st.join("originales").join(n),b)?;total+=b.len();entries.push(json!({"path":n,"bytes":b.len(),"sha256":hash(b)}));}
 assert_eq!(total,132549);
 let manifest=get("MANIFIESTO.tsv");let mut count=0;let mut bytes=0;
 for l in manifest.lines().skip(1){let v:Vec<_>=l.split('\t').collect();assert_eq!(v.len(),3);let b=get(v[0]).as_bytes();assert_eq!(b.len(),v[1].parse::<usize>()?);assert_eq!(hash(b),v[2]);count+=1;bytes+=b.len();}
 assert_eq!((count,bytes),(13,127793));
 let proof:Value=serde_json::from_str(get("COTEJO-PUBLICACION.json"))?;assert_eq!(proof["estado"],"CONFORME");assert_eq!(hash(manifest.as_bytes()),proof["manifiesto"]["sha256"]);assert_eq!(manifest.len(),1138);
 assert_eq!(get("RECUPERACION-RUST.tsv").lines().skip(1).count(),13);
 for l in get("RECUPERACION-RUST.tsv").lines().skip(1){let v:Vec<_>=l.split('\t').collect();assert_eq!(v.len(),5);assert_eq!(v[3],"true");assert_eq!(v[4],"CONFORME");assert_eq!(hash(get(v[0]).as_bytes()),v[2]);assert_eq!(get(v[0]).len(),v[1].parse::<usize>()?);}
 let mut commands=vec![];
 for stem in ["CALCULAR","CONTRASTAR"]{let path=st.join(format!("{stem}.exe"));
 let output=Command::new("rustc").args(["--edition","2024","-C","debuginfo=0","-C","opt-level=1"]).arg(st.join("originales").join(format!("{stem}.rs"))).arg("-o").arg(&path).output()?;
 assert!(output.status.success(),"Compilación: {}",String::from_utf8_lossy(&output.stderr));commands.push(path);}
 let calc=Command::new(&commands[0]).arg(st.join("originales/INVENTARIO-PESOS.tsv")).arg(st.join("originales/PARAMETROS.tsv")).output()?;
 assert!(calc.status.success());assert_eq!(calc.stdout,get("MEMORIA-RUST.tsv").as_bytes(),"Cálculo repetido discordante");
 save(&st.join("MEMORIA-REPRODUCIDA.tsv"),&calc.stdout)?;
 let contrast=Command::new(&commands[1]).arg(st.join("originales/INVENTARIO-PESOS.tsv")).arg(st.join("MEMORIA-REPRODUCIDA.tsv")).output()?;
 assert!(contrast.status.success(),"Contraste: {}",String::from_utf8_lossy(&contrast.stderr));assert_eq!(contrast.stdout,get("CONTRASTE-RUST.txt").as_bytes());
 let coverage=get("COBERTURA-Y-MEMORIA.tsv");assert_eq!(coverage.lines().skip(1).count(),141);
 assert!(coverage.ends_with(get("MEMORIA-RUST.tsv").split_once('\n').unwrap().1));
 let prim:Value=serde_json::from_slice(&fs::read(st.join("PRIMARIAS.json"))?)?;let mut primary=vec![];
 for f in prim.as_array().unwrap(){let b=f["content"].as_str().unwrap().as_bytes();assert_eq!(hash(b),f["expected"]["sha256"]);assert_eq!(b.len() as u64,f["expected"]["bytes"].as_u64().unwrap());primary.push(json!({"url":f["url"],"bytes":b.len(),"sha256":hash(b)}));}
 assert_eq!(primary.len(),4);
 let result=json!({"expediente":"ZAI-GLM53FLASH-RUST-AMD-20261006/r1","fecha_recepcion_utc":"2026-10-06T01:55:09Z","estado":"CONFORME_DOCUMENTAL_Y_ARITMETICA_ACOTADA","dictamen_recibido":"Desarrollo sustancial necesario","repositorio_origen":"juantoniolloretegea/SV-sala-de-maquinas","revision":data["revision"],"archivos":entries,"bytes_totales":total,"manifiesto_contenido":{"archivos":count,"bytes":bytes,"estado":"CONFORME"},"calculos":{"reproduccion_literal":true,"sha256_resultado":hash(&calc.stdout),"contraste_repetido":true,"filas_cobertura":141,"escenarios":6,"operaciones_gpu":0,"inferencia":false},"fuentes_primarias_releidas_y_cotejadas":primary,"limites":["Recepción independiente de documentación y aritmética; no reproducción del análisis completo de todas las bases","No descarga o cotejo de pesos, medida de memoria efectiva, motor o GPU","No recepción integral TT-0020 ni clasificación del candidato; Núcleo, semántica V0.2 e IR 0.3 intactos"]});
 save(&st.join("RECEPCION.json"),(serde_json::to_string_pretty(&result)?+"\n").as_bytes())?;
 println!("CONFORME: {total} bytes, 16 archivos, 13 contenidos manifestados, cálculo idéntico, contraste repetido, 141 filas y cuatro fuentes primarias.");
 Ok(())
}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
