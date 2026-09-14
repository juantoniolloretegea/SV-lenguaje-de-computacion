//! Inventario instrumental de lectura. No instala, no consulta red ni ejecuta bancos SV.
#![forbid(unsafe_code)]
use std::{env,fs,path::{Path,PathBuf},process::Command};
fn query(p:&Path,args:&[&str])->(bool,String){
 match Command::new(p).args(args).output(){Ok(o)=>(o.status.success(),String::from_utf8_lossy(&o.stdout).trim().to_string()+&String::from_utf8_lossy(&o.stderr)),Err(e)=>(false,e.to_string())}
}
fn locate(n:&str)->Option<PathBuf>{env::split_paths(&env::var_os("PATH").unwrap_or_default()).map(|d|d.join(n)).find(|p|p.is_file())}
fn run()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<_>=env::args().collect();if a.len()!=2{return Err("uso: estado-rust ARCHIVO_MD_DE_SALIDA".into());}
 let prefix=Path::new("/opt/sv-rust-1.98.0");
 let(_,date)=query(Path::new("/usr/bin/date"),&["-u","+%Y-%m-%dT%H:%M:%SZ"]);
 let os=fs::read_to_string("/etc/os-release")?;
 let(_,arch)=query(Path::new("/usr/bin/uname"),&["-m"]);
 let(mut valid,mut rows)=(true,String::new());
 for(n,arg,required)in[("rustc","--version",true),("cargo","--version",true),("rustfmt","--version",true),("clippy-driver","--version",true)]{
 let p=prefix.join("bin").join(n);let(ok,v)=query(&p,&[arg]);if required{valid&=ok;}
 if n=="rustc"{valid&=v.starts_with("rustc 1.98.0 ");}
 if n=="cargo"{valid&=v.starts_with("cargo 1.98.0 ");}
 rows+=&format!("| {n} | {} | {} |\n",p.display(),v.replace('|',"/").replace('\n'," "));
 }
 let components=fs::read_to_string(prefix.join("lib/rustlib/components"))?;
 let manager=Path::new("/opt/sv-cargo/bin/rustup");
 if manager.is_file(){
 let(o,v)=query(manager,&["--version"]);valid&=o;
 rows+=&format!("| Gestor rustup | {} | {} |\n",manager.display(),v.replace('|',"/").replace('\n'," "));
 let target=fs::canonicalize("/opt/sv-rustup/toolchains/sv-1.98.0")?;
 valid&=target==fs::canonicalize(prefix)?;
 rows+=&format!("| Toolchain enlazado sv-1.98.0 | {} | Destino real cotejado con la instalación directa |\n",target.display());
 }
 let(ok,lib)=query(&prefix.join("bin/rustc"),&["--print","target-libdir"]);valid&=ok&&Path::new(&lib).is_dir();
 rows+=&format!("| Biblioteca estándar nativa | {lib} | Directorio observado; componentes abajo |\n");
 for n in ["cc","gcc","ld","ar","pkg-config","rustup","dotnet"]{
 let p=locate(n);if matches!(n,"cc"|"ld"|"ar"){valid&=p.is_some();}
 rows+=&format!("| {n} | {} | {} |\n",p.as_ref().map(|p|p.display().to_string()).unwrap_or_else(||"—".into()),if p.is_some(){"Localizado en PATH"}else{"No localizado en PATH"});
 }
 let body=format!("# Estado observado de Rust y del entorno nativo\n\nFecha UTC: {date}. Arquitectura: {arch}. Unidad: Watson / W-S26.\n\n**Comprobación instrumental: {}.** Este registro es una observación fechada, no un monitor en directo ni garantía de persistencia después de reemplazar el entorno.\n\n| Componente | Ruta | Observación |\n| --- | --- | --- |\n{rows}\nDistribución observada:\n\n```text\n{os}```\n\nComponentes declarados por la instalación:\n\n```text\n{components}```\n\nEl banco S26-F01/F02 LIG compiló y ejecutó diez casos en debug y diez en release con entradas EN/ES: [recepción](../../riesgos-materiales-s26/r06/RESULTADOS_F01_F02_LIGADURAS.md). Este inventario no repite esas pruebas ni acredita dependencias de etapas futuras.\n\n[Obligación operativa, acceso y rustup](OBLIGACION_RUST_Y_PROVISION_DEL_ENTORNO_S29.md). [Fuente Rust de esta consulta](estado_rust.rs). El programa sólo lee archivos del sistema y consulta versiones/localización; declara los programas auxiliares invocados (date y uname). No usa Python ni red.\n",if valid{"CONFORME PARA LAS HERRAMIENTAS NATIVAS DECLARADAS"}else{"NO CONFORME; REVISAR EL INVENTARIO"});
 fs::write(&a[1],body)?;
 if !valid{return Err("entorno instrumental incompleto".into());}Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}
