//! Recepción documental S32: identidad, conservación y concordancia registral.
//! No aplica el parche recibido ni ejecuta campañas SV.
//! Uso: bin BASE OUT PARCHE RECEPCION UTC HORA_MADRID RUSTC
//! Dependencias: biblioteca estándar; Git para identidad y aplicabilidad textual.
use std::{env, fs, path::Path, process::Command};
const DIR:&str="docs/calidad/tuberias-ia/continuacion-15-09-2026/";
const REPORT:&str="docs/calidad/tuberias-ia/continuacion-15-09-2026/PARTE_ALCANCE_PRIVACIDAD_SEGURIDAD_Y_OP_CYB_001_2026_09_15.md";
const CSV:&str="docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv";
const MD:&str="docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md";
const HIST:&str="docs/calidad/Inventario-sv/sucesos/HISTORIAL_SUCESOS_SV.csv";
const RC:&str="docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv";
const RM:&str="docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md";
const CUT:&str="29fbcb022a731aace46a64ca881c02b4b1bcb13a";
const DEPOSIT:&str="https://github.com/juantoniolloretegea/SVperitus-dataset/blob/19bb22c0cb614c7c05184c017e3dc3859a11b1e7/dominios/inmunologia/cambio-rumbo/05-grok-aportes/PARCHE_S32_CORRESPONDENCIA_C17.diff";
const INPUTS:&[(&str,&str)]=&[
 (REPORT,"9ffb7b2dd29b5dbdaba2de64ca6671bddb386f18"),
 (CSV,"113ca257663ac7c07f951966bed8c7cf11669dba"),
 (MD,"d95eda26c9fb9f62c9cb42db9b5d2a1b801915b4"),
 (HIST,"392286fc529c60b8fa9f2416f0aecb982246a805"),
 (RC,"ee7ecdbfea33c9b0437ec992613feee5364e215d"),
 (RM,"53a5c89b3c5bec798dfea8b7358d4eaff68e91fc")
];
struct Row { cells:Vec<String>, start:usize, end:usize }
fn csv(s:&str)->Result<Vec<Row>,String>{
 let mut rows=Vec::new();let mut cells=Vec::new();let mut field=String::new();
 let mut quoted=false;let mut closed=false;let mut start=0;let mut it=s.char_indices().peekable();
 while let Some((i,c))=it.next(){
  if quoted {if c=='"'{if it.peek().is_some_and(|(_,x)|*x=='"'){it.next();field.push('"');}else{quoted=false;closed=true;}}else{field.push(c);}continue;}
  match c {
   '"' if field.is_empty()&&!closed=>quoted=true,
   '"'=>return Err(format!("comilla fuera de campo: {i}")),
   ','=>{cells.push(std::mem::take(&mut field));closed=false;},
   '\r' if it.peek().is_some_and(|(_,x)|*x=='\n')=>{},
   '\n'=>{cells.push(std::mem::take(&mut field));rows.push(Row{cells:std::mem::take(&mut cells),start,end:i+1});start=i+1;closed=false;},
   _ if closed=>return Err(format!("texto tras cierre: {i}")),
   _=>field.push(c)
  }
 }
 if quoted{return Err("comilla sin cerrar".into())}
 if start<s.len(){cells.push(field);rows.push(Row{cells,start,end:s.len()});}Ok(rows)
}
fn encode(c:&[String])->String{c.iter().map(|x|format!("\"{}\"",x.replace('"',"\"\""))).collect::<Vec<_>>().join(",")+"\n"}
fn hash(p:&Path)->String{let r=Command::new("git").arg("hash-object").arg(p).output().unwrap();assert!(r.status.success());String::from_utf8(r.stdout).unwrap().trim().into()}
fn write(root:&Path,p:&str,s:&str){let dest=root.join(p);fs::create_dir_all(dest.parent().unwrap()).unwrap();fs::write(&dest,s).unwrap();assert_eq!(fs::read_to_string(dest).unwrap(),s);}
fn json(s:&str)->String{
 let mut r=String::from("\"");for c in s.chars(){match c{'"'=>r.push_str("\\\""),'\\'=>r.push_str("\\\\"),'\n'=>r.push_str("\\n"),'\r'=>r.push_str("\\r"),'\t'=>r.push_str("\\t"),c if c<' '=>r.push_str(&format!("\\u{:04x}",c as u32)),c=>r.push(c)}}r.push('"');r
}
// Exportación para transporte: reconstruye exactamente los bytes ya comprobados.
fn export(base:&Path,out:&Path){
 let mut paths:Vec<String>=INPUTS.iter().map(|(p,_)|p.to_string()).collect();
 paths.push(format!("{DIR}COTEJO_RECEPCION_PARCHE_S32.rs"));paths.push(format!("{DIR}COTEJO_RECEPCION_PARCHE_S32_SALIDA.txt"));
 let mut entries=Vec::new();for p in paths{
  let old=fs::read_to_string(base.join(&p)).unwrap_or_default();let new=fs::read_to_string(out.join(&p)).unwrap();
  let mut prefix=old.bytes().zip(new.bytes()).take_while(|(a,b)|a==b).count();while !old.is_char_boundary(prefix)||!new.is_char_boundary(prefix){prefix-=1;}
  let mut suffix=old[prefix..].bytes().rev().zip(new[prefix..].bytes().rev()).take_while(|(a,b)|a==b).count();while !old.is_char_boundary(old.len()-suffix)||!new.is_char_boundary(new.len()-suffix){suffix-=1;}
  let text=&new[prefix..new.len()-suffix];assert_eq!(format!("{}{}{}",&old[..prefix],text,&old[old.len()-suffix..]),new);
  entries.push(format!("{{\"path\":{},\"from\":{},\"to\":{},\"text\":{},\"sha\":{}}}",json(&p),old[..prefix].encode_utf16().count(),old[..old.len()-suffix].encode_utf16().count(),json(text),json(&hash(&out.join(&p)))));
 }println!("[{}]",entries.join(","));
}
fn main(){
 let a:Vec<String>=env::args().collect();if a.len()==4&&a[1]=="--export"{export(Path::new(&a[2]),Path::new(&a[3]));return;}assert_eq!(a.len(),8,"BASE OUT PARCHE RECEPCION UTC HORA_MADRID RUSTC");
 let base=Path::new(&a[1]);let out=Path::new(&a[2]);assert_ne!(base,out);
 let patch=fs::canonicalize(&a[3]).unwrap();let utc=&a[5];assert!(utc.starts_with("2026-09-16T")&&utc.ends_with('Z'));
 let v=Command::new(&a[7]).arg("--version").output().unwrap();assert!(v.status.success());let v=String::from_utf8(v.stdout).unwrap();assert!(v.starts_with("rustc 1.98.0 "));
 let mut evidence=format!("Recepción documental S32 / RETP-2026-248\nCorte del Lenguaje: {CUT}\nFecha UTC de recepción: {utc}\n{v}");
 for(p,sha)in INPUTS{assert_eq!(hash(&base.join(p)),*sha,"identidad {p}");evidence.push_str(&format!("ENTRADA {sha} {p}\n"));}
 assert_eq!(hash(&patch),"e8b5e1d7cbb1fef1f84cd87c8f7f946e71cc1de6");
 let ptext=fs::read_to_string(&patch).unwrap();let targets:Vec<_>=ptext.lines().filter_map(|x|x.strip_prefix("+++ b/")).collect();
 assert_eq!(targets,vec![REPORT,MD,CSV,HIST]);
 let check=Command::new("git").args(["apply","--check","--verbose"]).arg(&patch).current_dir(base).output().unwrap();assert!(check.status.success());
 let messages=String::from_utf8(check.stderr).unwrap();assert!(!messages.contains("Skipped patch"));
 for p in &targets{assert!(messages.contains(&format!("Checking patch {p}...")));}
 evidence.push_str(&format!("Objeto recibido: {DEPOSIT}\nBlob Git del parche: e8b5e1d7cbb1fef1f84cd87c8f7f946e71cc1de6\nOrden: git apply --check --verbose PARCHE_S32_CORRESPONDENCIA_C17.diff, sobre la base indicada\nSalida 0, sin aplicación:\n{messages}"));
 let read=|p:&str|fs::read_to_string(base.join(p)).unwrap();let old_csv=read(CSV);let rows=csv(&old_csv).unwrap();let header=&rows[0].cells;assert_eq!(header.len(),18);
 let target=rows.iter().find(|r|r.cells[0]=="S32").unwrap();assert_eq!(rows.iter().filter(|r|r.cells[0]=="S32").count(),1);
 let mut c=target.cells.clone();assert_eq!(c[1],"en ejecución");assert!(c[5].is_empty());
 let old_hist=read(HIST);let hs=csv(&old_hist).unwrap();assert!(hs.iter().all(|r|r.cells.len()==19));
 let revs:Vec<u32>=hs.iter().skip(1).filter(|r|r.cells[1]=="S32").map(|r|r.cells[0].parse().unwrap()).collect();assert_eq!(revs,vec![0,1]);
 assert_eq!(&hs.iter().rev().find(|r|r.cells[1]=="S32").unwrap().cells[1..],c);
 c[4]=utc.clone();
 c[9]="SV-lenguaje-de-computacion: main; depósito recibido en SVperitus-dataset: dominio-inmunologia; fuente OP-CYB-001 en corte fijado.".into();
 c[10]=format!("Lenguaje {CUT}; depósito 19bb22c0cb614c7c05184c017e3dc3859a11b1e7; fuente citada CYB bbac1b44b1d3b845305e9cde492a08221206d631.");
 c[12]="Recibido y enlazado el parche de correspondencia con el control C17 de OP-CYB-001. Identidad y aplicabilidad textual conformes sobre su base declarada. Propuesta pendiente de dictamen sustantivo e integración; no aplicada. S32 y Bis permanecen abiertos.".into();
 c[13]="Lectura completa del parche. Rust 1.98.0: identidad de entradas, conservación de antecedentes y concordancia registral; Git: identidad del blob y comprobación textual sin aplicación. Cero pruebas de privacidad, Q1/Q2 o E1–E16 en esta recepción.".into();
 c[14]=format!("{DEPOSIT} ; {REPORT}#s32-recepcion-parche-c17-2026-09-16 ; {DIR}COTEJO_RECEPCION_PARCHE_S32.rs ; {DIR}COTEJO_RECEPCION_PARCHE_S32_SALIDA.txt ; antecedente: https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/{CUT}/{REPORT}#s32-diseno-2026-09-16 ; https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/{CUT}/{DIR}COTEJO_S32_DISENO.rs ; https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/{CUT}/{DIR}COTEJO_S32_DISENO_SALIDA.txt");
 c[15]="RETP-2026-248: recepción documental; antecedentes RETP-2026-242/243/247; parte de privacidad, recepción del parche de correspondencia con C17.".into();
 c[16]="Contrastar la propuesta con sus fuentes fijadas, resolver sus referencias y preparar la integración sobre el registro vigente. Vincular cada prueba futura con requisito, contrato, entradas, resultado esperado y aceptación. Se conservan las brechas y condiciones de cierre de los apartados 5 y 8 del parte; no habilitar flujos ni iniciar campañas.".into();
 c[17]="Revisión 2: recepción documental de una propuesta depositada. Alta, inicio, unidad responsable y fin conservados. No se aplica la instantánea propuesta. S22 y S26 intactos; las copias históricas conservan sus cortes.".into();
 for i in [0,1,2,3,5,6,7,8,11]{assert_eq!(c[i],target.cells[i]);}
 let encoded=encode(&c);let new_csv=format!("{}{}{}",&old_csv[..target.start],encoded,&old_csv[target.end..]);let ns=csv(&new_csv).unwrap();assert_eq!(ns.len(),rows.len());
 for(old,new)in rows.iter().zip(&ns){assert_eq!(new.cells.len(),18);if old.cells[0]!="S32"{assert_eq!(&old_csv[old.start..old.end],&new_csv[new.start..new.end]);}}
 let old_md=read(MD);let heading="## S32 · Privacidad y seguridad: correspondencia de flujos con OP-CYB-001";let start=old_md.find(heading).unwrap();let tail=&old_md[start..];let len=tail[heading.len()..].find("\n## ").map(|i|heading.len()+i).unwrap_or(tail.len());
 let mut block=format!("{heading}\n\n");for i in 1..header.len(){if i!=7{block.push_str(&format!("**{}:** {}\n\n",header[i],if c[i].is_empty(){"—"}else{&c[i]}));}}
 let new_md=format!("{}{}{}",&old_md[..start],block,&old_md[start+len..]);assert!(new_md.starts_with(&old_md[..start]));
 for i in 1..header.len(){if i!=7{assert!(block.contains(&format!("**{}:** {}\n",header[i],if c[i].is_empty(){"—"}else{&c[i]})));}}
 let mut snap=vec!["2".into()];snap.extend(c.clone());let new_hist=old_hist.clone()+&encode(&snap);assert!(old_hist.ends_with('\n'));let nh=csv(&new_hist).unwrap();assert_eq!(nh.len(),hs.len()+1);assert_eq!(&nh.last().unwrap().cells[1..],c);
 let old_report=read(REPORT);let receipt=fs::read_to_string(&a[4]).unwrap();assert!(receipt.contains("s32-recepcion-parche-c17-2026-09-16"));assert!(receipt.contains(DEPOSIT));assert!(!receipt.contains("## 11."));let new_report=old_report.clone()+"\n"+&receipt;
 let old_rc=read(RC);let rc_rows=csv(&old_rc).unwrap();assert_eq!(rc_rows[0].cells.len(),14);assert_eq!(rc_rows.iter().skip(1).filter_map(|r|r.cells[0].strip_prefix("RETP-2026-").and_then(|s|s.parse::<u32>().ok())).max(),Some(247));
 let retp:Vec<String>=vec!["RETP-2026-248","2026-09-16",&a[6],"Recepción documental","S32 / BIS-03",&c[12],"Conservar la relación entre la propuesta recibida, sus fuentes, el seguimiento y las comprobaciones posteriores.","Sucesos SV README; Acta 001 §§8–9; parte S32 §§5 y 8; depósito 19bb22c0 y base 29fbcb02.","Parte S32; Sucesos CSV y Markdown; historial; RETP CSV y Markdown; cotejo de recepción y salida.",&c[14],"Propuesta identificada y vinculada; condiciones de cierre conservadas.","Aplicabilidad textual no equivale a aceptación sustantiva ni a ensayo de privacidad.",&c[16],"en ejecución"].into_iter().map(String::from).collect();let new_rc=old_rc.clone()+&encode(&retp);assert_eq!(csv(&new_rc).unwrap().len(),rc_rows.len()+1);
 let old_rm=read(RM);let new_rm=format!("{old_rm}\n<a id=\"retp-248\"></a>\n\n## RETP-2026-248 · S32 · Recepción de propuesta de correspondencia con C17\n\n**Fecha de recepción:** {utc}; {} Europe/Madrid. **Estado de S32:** en ejecución. **Corte de entrada:** `{CUT}`.\n\n{}\n\n[Objeto recibido]({DEPOSIT}) · [Recepción y relación con las pruebas](tuberias-ia/continuacion-15-09-2026/PARTE_ALCANCE_PRIVACIDAD_SEGURIDAD_Y_OP_CYB_001_2026_09_15.md#s32-recepcion-parche-c17-2026-09-16).\n\n{}\n\n{}\n\n[Fuente del cotejo](tuberias-ia/continuacion-15-09-2026/COTEJO_RECEPCION_PARCHE_S32.rs) y [salida](tuberias-ia/continuacion-15-09-2026/COTEJO_RECEPCION_PARCHE_S32_SALIDA.txt). Antecedentes RETP-2026-242/243/247 conservados. La recepción no incorpora la sección 11 propuesta, no ejecuta pruebas de privacidad y no cierra S32 ni BIS-03.\n",a[6],c[12],c[13],c[16]);
 assert!(new_report.starts_with(&old_report)&&new_hist.starts_with(&old_hist)&&new_rc.starts_with(&old_rc)&&new_rm.starts_with(&old_rm));
 assert_eq!(csv("a,\"b,c\",\"d\"\"e\"\n").unwrap()[0].cells,vec!["a","b,c","d\"e"]);assert!(csv("a,\"b").is_err());assert!(csv("a,\"b\"x\n").is_err());
 let outputs=[(REPORT,new_report),(CSV,new_csv),(MD,new_md),(HIST,new_hist),(RC,new_rc),(RM,new_rm)];for(p,s)in &outputs{write(out,p,s);}
 for(p,sha)in INPUTS{assert_eq!(hash(&base.join(p)),*sha,"base alterada");}assert_eq!(hash(&patch),"e8b5e1d7cbb1fef1f84cd87c8f7f946e71cc1de6");
 evidence.push_str("Conformes: seis entradas fijadas y parche; cuatro rutas comprobadas sin aplicación.\nConformes: S32 revisión 2; estado, alta, inicio, unidad y fin conservados; filas ajenas intactas.\nConformes: CSV, Markdown e instantánea concordantes; historial, parte previo y RETP previos conservados byte a byte.\nRETP-2026-248 libre y añadido; lectura posterior de los seis archivos generados conforme.\nAnalizador CSV: entrada válida con comas y comillas; dos entradas inválidas rechazadas.\nDependencias del cotejo: biblioteca estándar nativa; sin crates externos ni Cargo.lock propio.\n");
 for(program,arg)in [("git","--version"),("cc","--version"),("ld","--version")]{let r=Command::new(program).arg(arg).output().unwrap();assert!(r.status.success());evidence.push_str(String::from_utf8(r.stdout).unwrap().lines().next().unwrap());evidence.push('\n');}
 let deps=Command::new("ldd").arg(env::current_exe().unwrap()).output().unwrap();assert!(deps.status.success());let deps=String::from_utf8(deps.stdout).unwrap();assert!(!deps.contains("not found"));evidence.push_str("Dependencias dinámicas resueltas:\n");for line in deps.lines(){evidence.push_str(line.split(" (").next().unwrap().trim());evidence.push('\n');}
 evidence.push_str("Edición y comprobaciones registrales: Rust. Transporte de contenido: conector GitHub y utilidades del sistema.\nLa compilación de este auxiliar y su ejecución no son ensayos de privacidad ni campañas SV. Cero Q1/Q2 y E1–E16.\nLa publicación y la lectura posterior del commit se verifican por separado.\n");
 write(out,&format!("{DIR}COTEJO_RECEPCION_PARCHE_S32.rs"),include_str!("COTEJO_RECEPCION_PARCHE_S32.rs"));write(out,&format!("{DIR}COTEJO_RECEPCION_PARCHE_S32_SALIDA.txt"),&evidence);print!("{evidence}");
}
