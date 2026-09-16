//! Edición mecánica y cotejo documental S32, sin campañas SV ni Python.
//! Uso: bin BASE OUT ADENDA_O_PARTE_AMPLIADO UTC HORA_MADRID
//! BASE contiene los seis ficheros del corte 7b6d722760d7a8e017e516e881ce1bfb9c9a8aab.
//! Git sólo calcula identidad de blobs; la edición/correspondencia se verifica en Rust.
use std::{collections::HashSet, env, fs, path::Path, process::Command};
const DIR: &str = "docs/calidad/tuberias-ia/continuacion-15-09-2026/";
const REPORT: &str = "docs/calidad/tuberias-ia/continuacion-15-09-2026/PARTE_ALCANCE_PRIVACIDAD_SEGURIDAD_Y_OP_CYB_001_2026_09_15.md";
const CSV: &str = "docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv";
const MD: &str = "docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md";
const HIST: &str = "docs/calidad/Inventario-sv/sucesos/HISTORIAL_SUCESOS_SV.csv";
const RC: &str = "docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv";
const RM: &str = "docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md";
const CUT: &str = "7b6d722760d7a8e017e516e881ce1bfb9c9a8aab";
const INPUTS: &[(&str,&str)] = &[
 (REPORT,"a5b5dc667d821d09dbb1a0ee14d39375364d4e29"),
 (CSV,"46669a79456c237ae1c8710060505e1563f3e0e1"),
 (MD,"6c0f3a135b53d1e1fe27e44979778fce587c806a"),
 (HIST,"c6407864958d42309ae93d8693c55309da768a38"),
 (RC,"c6583e35b3bb8d1372cb74d09a0e4115d78ba149"),
 (RM,"f95081ff6076f090a1598eb0e9cd1b331548986f"),
];
#[derive(Debug)]
struct Row { cells: Vec<String>, start: usize, end: usize }
fn csv(s: &str) -> Result<Vec<Row>, String> {
 let mut rows=Vec::new(); let mut cells=Vec::new(); let mut field=String::new();
 let mut quoted=false; let mut closed=false; let mut start=0;
 let mut it=s.char_indices().peekable();
 while let Some((i,c))=it.next() {
  if quoted {
   if c=='"' { if it.peek().is_some_and(|(_,x)| *x=='"') { it.next(); field.push('"'); }
    else { quoted=false; closed=true; }
   } else { field.push(c); }
   continue;
  }
  match c {
   '"' if field.is_empty() && !closed => quoted=true,
   '"' => return Err(format!("comilla fuera de campo en {i}")),
   ',' => { cells.push(std::mem::take(&mut field)); closed=false; },
   '\r' if it.peek().is_some_and(|(_,x)| *x=='\n') => {},
   '\n' => { cells.push(std::mem::take(&mut field)); rows.push(Row{cells:std::mem::take(&mut cells),start,end:i+1}); start=i+1; closed=false; },
   _ if closed => return Err(format!("texto tras cierre en {i}")),
   _ => field.push(c),
  }
 }
 if quoted { return Err("comilla sin cerrar".into()); }
 if start<s.len() { cells.push(field); rows.push(Row{cells,start,end:s.len()}); }
 Ok(rows)
}
fn encode(c: &[String]) -> String { c.iter().map(|x|format!("\"{}\"",x.replace('"',"\"\""))).collect::<Vec<_>>().join(",")+"\n" }
fn check_register(s:&str) -> Result<(),String> {
 let r=csv(s)?; let h=&r[0].cells; if h.len()!=18 {return Err("cabecera".into())}
 let mut ids=HashSet::new();
 for row in &r[1..] {
  let c=&row.cells;
  if c.len()!=18 || !ids.insert(c[0].clone()) {return Err("anchura/id duplicado".into())}
  if !["pendiente","en ejecución","finalizado"].contains(&c[1].as_str()) {return Err("estado no canónico".into())}
  if c[1]=="en ejecución" && (c[3].is_empty() || c[6].is_empty() || c[8].is_empty()) {return Err("ejecución sin identificación".into())}
 }
 Ok(())
}
fn write(root:&Path,p:&str,s:&str) { let path=root.join(p); fs::create_dir_all(path.parent().unwrap()).unwrap(); fs::write(path,s).unwrap(); }
fn json(s:&str)->String {
 let mut r=String::from("\""); for ch in s.chars(){match ch {'"'=>r.push_str("\\\""),'\\'=>r.push_str("\\\\"),'\n'=>r.push_str("\\n"),'\r'=>r.push_str("\\r"),'\t'=>r.push_str("\\t"),c if c<' '=>r.push_str(&format!("\\u{:04x}",c as u32)),c=>r.push(c)}} r.push('"'); r
}
// Transporte de los bytes ya comprobados al conector. No genera otra edición.
fn export(base:&Path,out:&Path) {
 let mut paths:Vec<String>=INPUTS.iter().map(|(p,_)|p.to_string()).collect();
 paths.push(format!("{DIR}COTEJO_S32_DISENO.rs")); paths.push(format!("{DIR}COTEJO_S32_DISENO_SALIDA.txt"));
 let mut entries=Vec::new();
 for p in paths {
  let old=fs::read_to_string(base.join(&p)).unwrap_or_default(); let new=fs::read_to_string(out.join(&p)).unwrap();
  let mut prefix=old.bytes().zip(new.bytes()).take_while(|(x,y)|x==y).count();
  while !old.is_char_boundary(prefix)||!new.is_char_boundary(prefix){prefix-=1;}
  let mut suffix=old[prefix..].bytes().rev().zip(new[prefix..].bytes().rev()).take_while(|(x,y)|x==y).count();
  while !old.is_char_boundary(old.len()-suffix)||!new.is_char_boundary(new.len()-suffix){suffix-=1;}
  let text=&new[prefix..new.len()-suffix]; assert_eq!(format!("{}{}{}",&old[..prefix],text,&old[old.len()-suffix..]),new);
  let h=Command::new("git").arg("hash-object").arg(out.join(&p)).output().unwrap(); assert!(h.status.success()); let sha=String::from_utf8(h.stdout).unwrap();
  entries.push(format!("{{\"path\":{},\"from\":{},\"to\":{},\"text\":{},\"sha\":{}}}",json(&p),old[..prefix].encode_utf16().count(),old[..old.len()-suffix].encode_utf16().count(),json(text),json(sha.trim())));
 }
 println!("[{}]",entries.join(","));
}
fn main() {
 let a:Vec<String>=env::args().collect();
 if a.len()==4 && a[1]=="--export" {export(Path::new(&a[2]),Path::new(&a[3]));return;}
 assert_eq!(a.len(),6,"BASE OUT ADENDA UTC HORA_MADRID; o --export BASE OUT");
 let base=Path::new(&a[1]); let out=Path::new(&a[2]); assert_ne!(base,out);
 let timestamp=&a[4]; assert!(timestamp.starts_with("2026-09-16T") && timestamp.ends_with('Z'));
 let mut evidence=format!("S32 / RETP-2026-247\nCorte de entrada: {CUT}\nFecha UTC: {timestamp}\n");
 let version=Command::new(env::var("S32_RUSTC").expect("S32_RUSTC")).arg("--version").output().unwrap();
 assert!(version.status.success()); let v=String::from_utf8(version.stdout).unwrap(); assert!(v.starts_with("rustc 1.98.0 ")); evidence.push_str(&v);
 for (p,sha) in INPUTS {
  let result=Command::new("git").arg("hash-object").arg(base.join(p)).output().unwrap();
  assert!(result.status.success()); assert_eq!(String::from_utf8(result.stdout).unwrap().trim(),*sha,"identidad {p}");
  evidence.push_str(&format!("ENTRADA {sha} {p}\n"));
 }
 let read=|p:&str|fs::read_to_string(base.join(p)).unwrap();
 let old_csv=read(CSV); check_register(&old_csv).unwrap();
 let rows=csv(&old_csv).unwrap(); let header=&rows[0].cells;
 let target=rows.iter().find(|r|r.cells[0]=="S32").unwrap(); let mut c=target.cells.clone();
 let old_hist=read(HIST); let hs=csv(&old_hist).unwrap();
 assert!(hs.iter().all(|r|r.cells.len()==19));
 assert!(!old_hist.contains("W-S32"),"unidad ya ocupada");
 let revs:Vec<u32>=hs.iter().skip(1).filter(|r|r.cells[1]=="S32").map(|r|r.cells[0].parse().unwrap()).collect();
 assert_eq!(revs,vec![0]);
 assert_eq!(hs.iter().rev().find(|r|r.cells[1]=="S32").unwrap().cells[1..],c);
 assert_eq!(c[1],"en ejecución"); assert_eq!(c[6],"W-S26-02");
 c[4]=timestamp.clone(); c[6]="Watson / W-S32".into();
 c[8]="Correspondencia de flujos y obligaciones de privacidad/seguridad desde el diseño; matriz inicial, contratos y puertas de habilitación. Sin ampliar OP-CYB-001 ni implementar conectores.".into();
 c[10]=format!("Lenguaje {CUT}; CYB bbac1b44b1d3b845305e9cde492a08221206d631");
 c[11]="S22 / BIS-03; RETP-2026-242/243; RS01–RS12 y adenda §12 de OP-CYB-001; rutas S28; obligación instrumental S29.".into();
 c[12]="Continuación autorizada publicada en el mismo parte: distinción de usos, obligaciones desde el diseño, matriz inicial de flujos y revisión adversarial documental. Sin objeción bloqueante para este incremento; contratos concretos y prueba material pendientes. S32 y Bis no se cierran.".into();
 c[13]="Lectura de fuentes rectoras y revisión documental finita. Rust 1.98.0: identidades de seis entradas, conservación de antecedentes, concordancia CSV/Markdown/historial y controles discriminantes del cotejador. Cero nuevas pruebas de privacidad, Q1/Q2 o E1–E16.".into();
 c[14]=format!("https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/{REPORT}#s32-diseno-2026-09-16 ; {DIR}COTEJO_S32_DISENO.rs ; {DIR}COTEJO_S32_DISENO_SALIDA.txt");
 c[15]="RETP-2026-247; parte de privacidad §§7–10; antecedentes RETP-2026-242/243.".into();
 c[16]="Resolver en S32 las brechas contractuales de §8, empezando por consulta, autorización, salida y persistencia; fijar sede, contrato, aplicabilidad y aceptación antes de congelar interfaces o cerrar BIS-03. Asignar seguimiento concreto a realizaciones diferidas antes de cerrar S32; no habilitar flujos ni iniciar campañas.".into();
 c[17]="Revisión 1: relevo W-S26-02 a Watson / W-S32 sólo para S32; identidad nueva cotejada en historial. Alta e inicio conservados, fin vacío. S22/S26 intactos. Rust recuperado desde paquete oficial para esta tarea, sin certificar persistencia ni Cargo. Transporte/provisión y conector declarados; sin Python. Sede canónica del Lenguaje; espejos históricos conservan su corte.".into();
 assert_eq!(c[0],target.cells[0]); assert_eq!(c[2],target.cells[2]); assert_eq!(c[3],target.cells[3]); assert_eq!(c[5],""); assert_eq!(c[7],target.cells[7]);
 let encoded=encode(&c); let new_csv=format!("{}{}{}",&old_csv[..target.start],encoded,&old_csv[target.end..]);
 check_register(&new_csv).unwrap(); let ns=csv(&new_csv).unwrap(); assert_eq!(rows.len(),ns.len());
 for (old,new) in rows.iter().zip(&ns) { if old.cells[0]!="S32" {assert_eq!(&old_csv[old.start..old.end],&new_csv[new.start..new.end]);} }
 let old_md=read(MD); let heading="## S32 · Privacidad y seguridad: correspondencia de flujos con OP-CYB-001";
 let start=old_md.find(heading).unwrap(); let tail=&old_md[start..];
 let length=tail[heading.len()..].find("\n## ").map(|i|heading.len()+i).unwrap_or(tail.len());
 let mut block=format!("{heading}\n\n");
 for i in 1..header.len() { if i==7 {continue;} block.push_str(&format!("**{}:** {}\n\n",header[i],if c[i].is_empty(){"—"}else{&c[i]})); }
 let new_md=format!("{}{}{}",&old_md[..start],block,&old_md[start+length..]);
 assert!(new_md.starts_with(&old_md[..start]));
 for i in 1..header.len() { if i!=7 {assert!(block.contains(&format!("**{}:** {}\n",header[i],if c[i].is_empty(){"—"}else{&c[i]})));} }
 let mut snap=vec!["1".into()]; snap.extend(c.clone()); let new_hist=old_hist.clone()+&encode(&snap);
 assert!(old_hist.ends_with('\n')); let nh=csv(&new_hist).unwrap(); assert_eq!(nh.len(),hs.len()+1); assert_eq!(&nh.last().unwrap().cells[1..],&c);
 let original=read(REPORT); let supplied=fs::read_to_string(&a[3]).unwrap();
 let append=supplied.strip_prefix(&original).unwrap_or(&supplied);
 assert!(append.trim_start().starts_with("<a id=\"s32-diseno-2026-09-16\"></a>"));
 assert!(append.contains("## 10.")); let new_report=original.clone()+append;
 assert!(new_report.starts_with(&original));
 let old_rc=read(RC); let old_rm=read(RM);
 let rc_rows=csv(&old_rc).unwrap(); let ids:Vec<_>=rc_rows.iter().skip(1).map(|r|r.cells[0].as_str()).collect();
 assert!(!ids.contains(&"RETP-2026-247")); assert!(ids.contains(&"RETP-2026-246"));
 assert!(ids.iter().filter_map(|id|id.strip_prefix("RETP-2026-").and_then(|s|s.parse::<u32>().ok())).max()==Some(246));
 let retp:Vec<String>=vec!["RETP-2026-247","2026-09-16",&a[5],"Continuación documental y revisión adversarial","S32 / BIS-03",&c[12],"Precisar obligaciones desde el diseño sin imponer despliegues opcionales ni aplazar decisiones de interfaz","Pilares; perfiles; transición; rutas S28; OP-CYB-001; estudio BIS-03; RGPD25/35","Parte S32; Sucesos CSV/Markdown/historial; RETP CSV/Markdown; cotejo Rust",&c[14],"Matriz inicial y puertas explícitas; contratos pendientes","Autorización del experto no implica permisos del agente; investigar no implica exportar ni promover al dominio; contrato no equivale a prueba",&c[16],"en ejecución"].into_iter().map(String::from).collect();
 assert_eq!(retp.len(),rc_rows[0].cells.len());
 let new_rc=old_rc.clone()+&encode(&retp);
 let new_rm=format!("{old_rm}\n<a id=\"retp-247\"></a>\n\n## RETP-2026-247 · S32 · Obligaciones desde el diseño y continuación acotada\n\n**Fecha:** {timestamp}; {} Europe/Madrid. **Estado:** en ejecución. **Unidad:** Watson / W-S32. **Entrada:** `{CUT}`.\n\n{}\n\n[Parte ampliado](tuberias-ia/continuacion-15-09-2026/PARTE_ALCANCE_PRIVACIDAD_SEGURIDAD_Y_OP_CYB_001_2026_09_15.md#s32-diseno-2026-09-16). Se conserva el texto anterior; la evaluación que condiciona el diseño no se aplaza a la habilitación de conectores. Matriz inicial sobre flujos y P01–P10 existentes, sedes funcionales por ligar a contratos concretos y ocho objeciones documentales con disposición. No se promueve ninguna capacidad de consumo, salida, aprendizaje o cambio de dominio.\n\n{}\n\n{}\n\nVerificación: {} Rust 1.98.0 recuperado para esta tarea desde paquete oficial con huella conforme; sin afirmar Cargo, rustup ni persistencia. [Fuente del cotejo](tuberias-ia/continuacion-15-09-2026/COTEJO_S32_DISENO.rs) y [salida](tuberias-ia/continuacion-15-09-2026/COTEJO_S32_DISENO_SALIDA.txt). Sin cambios de código SV, estados S22/S26, universos o campañas. Espejos históricos conservan su corte.\n",a[5],c[12],c[16],c[17],c[13]);
 assert!(new_hist.starts_with(&old_hist) && new_rc.starts_with(&old_rc) && new_rm.starts_with(&old_rm));
 // Controles discriminantes de esta lógica documental, no pruebas de privacidad.
 assert_eq!(csv("a,\"b,c\",\"d\"\"e\"\n").unwrap()[0].cells,vec!["a","b,c","d\"e"]);
 assert_eq!(csv("a,\"b\nc\"\r\n").unwrap()[0].cells,vec!["a","b\nc"]);
 assert!(csv("a,\"b").is_err()); assert!(csv("a,\"b\"x\n").is_err());
 let mut bad=c.clone(); bad[1]="cerrado".into(); let test_header=encode(header);
 assert!(check_register(&(test_header.clone()+&encode(&bad))).is_err());
 assert!(check_register(&(test_header.clone()+&encoded+&encoded)).is_err());
 bad=c.clone(); bad[3]=String::new(); assert!(check_register(&(test_header+&encode(&bad))).is_err());
 evidence.push_str("Conformes: seis identidades de entrada; S32 revisión 1; unidad libre; nombre, alta e inicio conservados; fin vacío.\n");
 evidence.push_str("Conformes: filas ajenas intactas; CSV/Markdown/instantánea concordantes; historial, parte previo y RETP previos conservados byte a byte; RETP-247 libre y añadido.\n");
 evidence.push_str("Controles del cotejador: 2 entradas CSV válidas y 5 negativas discriminantes, todos conformes.\nCero nuevas pruebas de privacidad, Q1/Q2, E1–E16 o campañas SV.\n");
 let outputs=[(REPORT,new_report),(CSV,new_csv),(MD,new_md),(HIST,new_hist),(RC,new_rc),(RM,new_rm)];
 for (p,s) in &outputs {write(out,p,s);assert_eq!(fs::read_to_string(out.join(p)).unwrap(),*s);}
 write(out,&format!("{DIR}COTEJO_S32_DISENO.rs"),include_str!("COTEJO_S32_DISENO.rs"));
 evidence.push_str("Dependencias: biblioteca estándar nativa; cero crates externos; sin Cargo.lock propio.\n");
 for (program,arg) in [("git","--version"),("cc","--version"),("ld","--version")] {
  let r=Command::new(program).arg(arg).output().unwrap(); assert!(r.status.success()); let s=String::from_utf8(r.stdout).unwrap(); evidence.push_str(s.lines().next().unwrap());evidence.push('\n');
 }
 let deps=Command::new("ldd").arg(env::current_exe().unwrap()).output().unwrap();assert!(deps.status.success());
 let ds=String::from_utf8(deps.stdout).unwrap();assert!(!ds.contains("not found"));
 evidence.push_str("Dependencias dinámicas resueltas; direcciones de carga omitidas:\n");
 for line in ds.lines(){evidence.push_str(line.split(" (").next().unwrap().trim());evidence.push('\n');}
 evidence.push_str("La provisión de dependencias de futuras etapas debe comprobarse de nuevo; este cotejo no acredita el entorno del reconocedor.\n");
 evidence.push_str("Lectura posterior de los seis archivos generados conforme. Publicación y lectura del commit deben verificarse por separado.\n");
 write(out,&format!("{DIR}COTEJO_S32_DISENO_SALIDA.txt"),&evidence);
 print!("{evidence}");
}
