//! Recepción administrativa S29 y revisión secuencial S24. No accede a RETP.
#![forbid(unsafe_code)]
#[allow(dead_code)]
#[path="../../riesgos-materiales-s26/r08/auxiliar.rs"] mod auxiliar;
use std::{fs,path::Path,collections::{BTreeMap,BTreeSet}};
fn csvrow(r:&[String])->String{r.iter().map(|s|format!("\"{}\"",s.replace('"',"\"\""))).collect::<Vec<_>>().join(",")+"\n"}
fn section(h:&[String],r:&[String])->String{
 let act=h.iter().position(|s|s=="actividad").unwrap();let mut out=format!("## {} · {}\n\n",r[0],r[act]);
 for(k,v)in h.iter().zip(r){if k=="id"||k=="actividad"{continue;}out+=&format!("**{k}:** {}\n\n",if v.is_empty(){"—"}else{v});}out
}
fn check(c:&[Vec<String>],h:&[Vec<String>],md:&str)->Result<(),String>{
 let mut hh=vec!["revision".into()];hh.extend(c[0].clone());if hh!=h[0]{return Err("cabecera".into());}
 let mut last=BTreeMap::new();let mut seen=BTreeSet::new();
 for r in &h[1..]{let rev=r[0].parse::<u64>().map_err(|_|"revision")?;if !seen.insert((r[1].clone(),rev)){return Err("revision repetida".into());}let e=last.entry(r[1].clone()).or_insert((rev,&r[1..]));if rev>e.0{*e=(rev,&r[1..]);}}
 let mut ids=BTreeSet::new();
 for r in &c[1..]{if !ids.insert(&r[0])||last.get(&r[0]).is_none_or(|x|x.1!=r){return Err("fila/historial".into());}
 let head=format!("## {} · ",r[0]);let start=md.find(&head).ok_or("MD")?;let end=md[start+head.len()..].find("\n## ").map(|n|start+head.len()+n+1).unwrap_or(md.len());let block=&md[start..end];
 for(k,v)in c[0].iter().zip(r){if k=="id"||k=="actividad"{continue;}let p=format!("**{k}:** ");let values:Vec<_>=block.lines().filter_map(|l|l.strip_prefix(&p)).collect();if values!=vec![if v.is_empty(){"—"}else{v.as_str()}]{return Err(format!("MD {} {k}",r[0]));}}
 }Ok(())
}
fn run()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<_>=std::env::args().collect();if a.len()!=4{return Err("uso: registrar ROOT FECHA_UTC COPIA_PREVIA_NUEVA".into());}
 let dir=Path::new(&a[1]).join("docs/calidad/Inventario-sv/sucesos");let date=&a[2];
 let names=["SUCESOS_SV.csv","HISTORIAL_SUCESOS_SV.csv","SUCESOS_SV.md"];
 let b:Vec<Vec<u8>>=names.iter().map(|n|fs::read(dir.join(n))).collect::<Result<_,_>>()?;
 let mut c=auxiliar::csv(&b[0])?;let mut h=auxiliar::csv(&b[1])?;let mut md=String::from_utf8(b[2].clone())?;check(&c,&h,&md)?;
 if c.len()!=30||c.iter().any(|r|r[0]=="S29"){return Err("alta no aplicable".into());}
 let idx=c.iter().position(|r|r[0]=="S24").ok_or("S24")?;let old=c[idx].clone();let header=c[0].clone();
 if h.iter().filter(|r|r.get(1).is_some_and(|s|s=="S24")).count()!=1{return Err("revision S24 inesperada".into());}
 let mut set=|row:&mut Vec<String>,key:&str,value:&str|->Result<(),String>{let i=header.iter().position(|s|s==key).ok_or("campo")?;row[i]=value.into();Ok(())};
 set(&mut c[idx],"fecha_actualizacion_utc",date)?;
 set(&mut c[idx],"unidad_responsable","Watson / W-S26")?;
 set(&mut c[idx],"resultado","Se recibe la instrucción humana de provisionar C# y .NET, con bibliotecas y herramientas necesarias, cuando llegue el turno de GUI. Permanece la secuencia Bis, catálogo y cierre de fase, después GUI.")?;
 set(&mut c[idx],"verificacion","S24 revisión 1; estado pendiente y fechas de inicio/fin vacías conservados. No se ha instalado dotnet ni seleccionado una versión o biblioteca de interfaz. S29 documenta la obligación de provisión por etapa.")?;
 set(&mut c[idx],"siguiente_accion","Tras los cierres de Bis, catálogo y fase, activar S24 y provisionar C#/.NET y sus bibliotecas, compilación, ejecución e integración requeridas; registrar versiones y comprobación del conjunto antes de utilizar la GUI.")?;
 set(&mut c[idx],"observaciones","La indicación tecnológica C#/.NET se recibe el 14/09/2026 y sustituye para la continuación la ausencia de selección de la revisión 0; no adelanta la GUI. Los antecedentes mantienen su contenido. S29 exige Rust para los procesos SV y justificación previa de cada uso de Python; C#/.NET conserva su encargo específico de GUI. Unidad actual W-S26, sin reasignar las autorías históricas.")?;
 let p=header.iter().position(|s|s=="evidencias").unwrap();c[idx][p]+=" ; https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/recuperacion-rust-s25/OBLIGACION_RUST_Y_PROVISION_DEL_ENTORNO_S29.md";
 let mut new=vec![String::new();header.len()];
 for(k,v)in[
 ("id","S29"),("estado","finalizado"),("fecha_alta_utc",date.as_str()),("fecha_actualizacion_utc",date.as_str()),("fecha_fin_utc",date.as_str()),("unidad_responsable","Watson / W-S26"),
 ("actividad","Obligación de Rust, localización de herramientas y provisión verificable del entorno"),
 ("alcance","Incorporación operativa de Rust obligatorio en todos los procesos SV; justificación previa de cada uso de Python; inventario fechado, acceso a la instalación y fuentes oficiales para rustup. Provisión de C#/.NET diferida a S24."),
 ("repositorios_y_ramas","SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente"),
 ("cortes_de_entrada","Recepción LIG finalizada antes de iniciar esta incorporación: Lenguaje 2ebd2215a803bcfb31b1425cba940bd2df19c56a; laboratorio 32ab2bc5b7f279a03b078eea165c180fa7b20d60."),
 ("dependencias","Instrucción humana del 14/09/2026; S25 instalación directa; control de auxiliares S26; S24 conserva su secuencia."),
 ("resultado","Obligación publicada con inventario renovable por programa Rust. Ubuntu 24.04.3 LTS x86_64; Rust/Cargo 1.98.0 en /opt/sv-rust-1.98.0, biblioteca estándar nativa, rustfmt y Clippy identificados. Enlaces oficiales Linux de rustup-init y SHA-256 localizados. S24 recibe C#/.NET para su turno."),
 ("verificacion","estado_rust.rs compilado y ejecutado con retorno 0; lee la distribución, versiones, componentes y rutas. Banco LIG previo compilado offline y conforme, sin repetirlo. Se identifican cc/gcc/ld/ar y se declara rustup, pkg-config y dotnet no localizados en PATH. CSV/Markdown/historial cotejados por Rust."),
 ("evidencias","https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/recuperacion-rust-s25/ESTADO_RUST.md ; https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/recuperacion-rust-s25/OBLIGACION_RUST_Y_PROVISION_DEL_ENTORNO_S29.md ; https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/lab/playground-sv-permanente/laboratorio/tareas-watson/tuberias-ia/recuperacion-rust-s25/OBLIGACION_RUST_Y_PROVISION_DEL_ENTORNO_S29.md"),
 ("referencia_calidad","recuperacion-rust-s25: OBLIGACION_RUST_Y_PROVISION_DEL_ENTORNO_S29.md, ESTADO_RUST.md, estado_rust.rs y registrar_s29.rs; CONTROL_DE_AUXILIARES.md e inicio de lectura actualizados. RETP canónica sin cambios."),
 ("siguiente_accion","Aplicar la obligación desde ahora, consultar y renovar el estado antes de cada etapa y provisionar las dependencias requeridas. Si se aporta rustup-init Linux, verificar su huella oficial y plataforma antes de instalar, conservando Rust 1.98.0 y registrando PATH/componentes. C#/.NET sigue en S24."),
 ("observaciones","Cierre de incorporación de obligación, inventario y acceso; no declara instalado rustup ni migrados todos los auxiliares históricos. Registro posterior a la preparación documental, sin inventar hora de inicio. El estado web es una observación fechada, no un monitor vivo. Conectores, coordinación JavaScript y herramientas de sistema se declaran; no se presentan como escritos en Rust. No se ejecutó Python en S29. No nuevas ramas ni directorios versionados; W-S26 estable.")
 ]{set(&mut new,k,v)?;}
 let mut rev24=vec!["1".into()];rev24.extend(c[idx].clone());let mut rev29=vec!["0".into()];rev29.extend(new.clone());h.push(rev24.clone());h.push(rev29.clone());c.push(new.clone());
 let old_text=String::from_utf8(b[0].clone())?;let lines:Vec<_>=old_text.split_inclusive('\n').collect();if lines.len()!=c.len()-1{return Err("perfil CSV".into());}
 let mut csv=String::new();for(i,l)in lines.iter().enumerate(){csv+=&if i==idx{csvrow(&c[idx])}else{l.to_string()};}csv+=&csvrow(&new);
 let mut history=b[1].clone();history.extend(csvrow(&rev24).as_bytes());history.extend(csvrow(&rev29).as_bytes());
 let heading="## S24 · ";let start=md.find(heading).ok_or("seccion S24")?;let end=md[start+heading.len()..].find("\n## ").map(|n|start+heading.len()+n+1).unwrap_or(md.len());md.replace_range(start..end,&section(&header,&c[idx]));md+=&section(&header,&new);
 check(&auxiliar::csv(csv.as_bytes())?,&auxiliar::csv(&history)?,&md)?;
 if !history.starts_with(&b[1])||c.iter().filter(|r|r[0]=="S24").count()!=1||old[1]!="pendiente"{return Err("conservacion".into());}
 fs::create_dir(&a[3])?;for(n,bytes)in names.iter().zip(&b){fs::write(Path::new(&a[3]).join(n),bytes)?;}
 for(n,bytes)in names.iter().zip([csv.as_bytes(),history.as_slice(),md.as_bytes()]){fs::write(dir.join(n),bytes)?;if fs::read(dir.join(n))?!=bytes{return Err("relectura".into());}}
 println!("S29 revision 0 finalizado; S24 revision 1 pendiente; 30 sucesos concordantes; historial conservado.");
 Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}
