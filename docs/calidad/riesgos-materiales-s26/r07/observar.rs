//! Conductor y oráculos Rust. Python es exclusivamente el objeto heredado bajo ensayo.
use std::{env, fs, io, path::{Path, PathBuf}, process::{Command, Stdio}, thread, time::{Duration, Instant}};
const PUBLICADOR: &[u8] = include_bytes!("../soporte/publicar_archivos.py");
const REGISTRADOR: &[u8] = include_bytes!("../soporte/registrar_continuacion.py");
const MOCK: &str = r#"from pathlib import Path
A='a'*40
def log(x):
 p=Path('llamadas.txt');p.write_text((p.read_text() if p.exists() else '')+x+'\n')
def fetch(url):
 log('fetch '+url)
 if '/git/ref/' in url:return {'object':{'sha':A}}
 if '/git/trees/' in url:return {'sha':'b'*40,'truncated':False,'tree':[]}
 raise RuntimeError('URL no prevista')
def call(name,args):
 log('call '+name)
 if name=='create_tree':return {'sha':'b'*40}
 if name=='create_commit':return {'sha':'c'*40}
 if name=='update_ref':return {'success':True}
 raise RuntimeError('Operacion no prevista')
"#;
const DRIVER: &str = r#"import sys,json
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent))
from publicar_archivos import publicar
print('DEBUG='+str(__debug__),flush=True)
bad=sys.argv[1]=='bad'
stale=sys.argv[1]=='stale'
r=publicar('fixture/otro' if stale else 'fixture/repo','main',{'out.txt':'inexistente.txt'} if stale else {},'fixture','checkpoint.json',expected_base=('d' if bad or stale else 'a')*40)
print('RETORNO='+json.dumps(r,sort_keys=True),flush=True)
"#;
const HEADER: &str = "id,estado,fecha_alta_utc,fecha_inicio_utc,fecha_actualizacion_utc,fecha_fin_utc,unidad_responsable,actividad,alcance,repositorios_y_ramas,cortes_de_entrada,dependencias,resultado,verificacion,evidencias,referencia_calidad,siguiente_accion,observaciones\n";
const ROW: &str = "S1,en ejecución,2026-01-01,2026-01-01,2026-01-01,,fixture,actividad,alcance,fixture/repo,base,,antes,pendiente,evidencia,referencia,siguiente,observacion\n";
const RH: &str = "ID,Fecha,Hora_Europe_Madrid,Tipo_Hito,Frente_Fase,Resumen_Cambio,Motivo_Argumento,Base_Doctrinal_Tecnica,Artefactos_Afectados,Evidencia,Impacto,Objecion_Adversarial,Decision,Estado\n";
const RR: &str = "RETP-2026-1,2026-01-01,,fixture,S1,antes,motivo,base,artefactos,evidencia,impacto,objecion,decision,en ejecución\n";
const SUC: &str = "docs/calidad/Inventario-sv/sucesos";
const RETP: &str = "docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO";
fn put(p: impl AsRef<Path>, bytes: impl AsRef<[u8]>) -> io::Result<()> {
    let p=p.as_ref(); if let Some(parent)=p.parent(){fs::create_dir_all(parent)?;} fs::write(p,bytes)
}
fn read(p: impl AsRef<Path>)->io::Result<String>{fs::read_to_string(p)}
fn require(ok:bool,msg:&str)->io::Result<()>{if ok{Ok(())}else{Err(io::Error::other(msg))}}
struct Run{success:bool,code:Option<i32>,out:String,err:String,timed_out:bool}
fn execute(python:&Path,dir:&Path,script:&str,args:&[&str],optimized:bool)->io::Result<Run>{
    let out=fs::File::create(dir.join("stdout.txt"))?;
    let err=fs::File::create(dir.join("stderr.txt"))?;
    let mut cmd=Command::new(python);
    cmd.env_clear().current_dir(dir).arg("-I").arg("-B");
    if optimized {cmd.arg("-O");}
    cmd.arg(script).args(args).stdin(Stdio::null()).stdout(out).stderr(err);
    let mut child=cmd.spawn()?; let start=Instant::now(); let mut timed_out=false;
    let status=loop{
        if let Some(s)=child.try_wait()?{break s;}
        if start.elapsed()>Duration::from_secs(10){timed_out=true;child.kill()?;break child.wait()?;}
        thread::sleep(Duration::from_millis(10));
    };
    // Cuota comprobada después de terminar: no se presenta como límite preventivo.
    require(fs::metadata(dir.join("stdout.txt"))?.len()<=65536,"stdout excede cuota de recepción")?;
    require(fs::metadata(dir.join("stderr.txt"))?.len()<=65536,"stderr excede cuota de recepción")?;
    let r=Run{success:status.success(),code:status.code(),out:read(dir.join("stdout.txt"))?,err:read(dir.join("stderr.txt"))?,timed_out};
    put(dir.join("terminacion.txt"),format!("exit_code={:?}\nsuccess={}\ntimed_out={}\n",r.code,r.success,r.timed_out))?;
    Ok(r)
}
fn publisher(root:&Path,python:&Path,id:&str,arg:&str,opt:bool)->io::Result<bool>{
    let d=root.join(id);fs::create_dir(&d)?;
    put(d.join("publicar_archivos.py"),PUBLICADOR)?;put(d.join("github_io.py"),MOCK)?;put(d.join("driver.py"),DRIVER)?;
    let stale="{\"verified\":true,\"commit\":\"anterior\"}\n";
    if arg=="stale"{put(d.join("checkpoint.json"),stale)?;}
    let r=execute(python,&d,"driver.py",&[arg],opt)?;
    let calls=match read(d.join("llamadas.txt")){Ok(s)=>s,Err(e) if e.kind()==io::ErrorKind::NotFound=>String::new(),Err(e)=>return Err(e)};
    let cp=match read(d.join("checkpoint.json")){Ok(s)=>s,Err(e) if e.kind()==io::ErrorKind::NotFound=>String::new(),Err(e)=>return Err(e)};
    let debug=if opt{"DEBUG=False\n"}else{"DEBUG=True\n"};
    let ok=match id{
        "P01"|"P03"=>r.success && r.err.is_empty() && calls.lines().filter(|x|*x=="call update_ref").count()==1 && cp.contains("\"verified\": true") && r.out.contains("PUBLICACION_VERIFICADA"),
        "P02"=>!r.success && r.err.contains("AssertionError") && calls.lines().count()==1 && !calls.contains("call ") && cp.is_empty(),
        "P04"=>r.success && r.err.is_empty() && calls.is_empty() && cp==stale && r.out.contains("\"commit\": \"anterior\"") && !d.join("inexistente.txt").exists(),
        _=>false
    } && !r.timed_out && r.out.starts_with(debug);
    put(d.join("observacion.txt"),format!("oraculo_cumplido={ok}\noptimizado={opt}\nmodo={arg}\n"))?;
    Ok(ok)
}
fn registrar(root:&Path,python:&Path,id:&str,revision:u32,opt:bool,missing:bool)->io::Result<bool>{
    let d=root.join(id);fs::create_dir(&d)?;put(d.join("registrar_continuacion.py"),REGISTRADOR)?;
    let s=d.join(SUC);fs::create_dir_all(&s)?;
    let before=format!("{HEADER}{ROW}");let hist=format!("revision,{HEADER}0,{ROW}");let retp=format!("{RH}{RR}");
    put(s.join("SUCESOS_SV.csv"),&before)?;put(s.join("HISTORIAL_SUCESOS_SV.csv"),&hist)?;
    if !missing{put(s.join("SUCESOS_SV.md"),"# Sucesos\n\n## S1 · actividad\n\nantes\n")?;}
    put(d.join(format!("{RETP}.csv")),&retp)?;put(d.join(format!("{RETP}.md")),"# RETP\n")?;
    let seat=format!(r#"{{"suceso":{{"id":"S1","resultado":"despues"}},"revision":{revision},"retp":2,"tipo":"fixture","motivo":"motivo","base":"base","artefactos":"artefactos","impacto":"impacto","titulo":"titulo"}}"#);
    put(d.join("asiento.json"),seat)?;
    let r=execute(python,&d,"registrar_continuacion.py",&["asiento.json"],opt)?;
    let after=read(s.join("SUCESOS_SV.csv"))?;let h=read(s.join("HISTORIAL_SUCESOS_SV.csv"))?;
    let rt=read(d.join(format!("{RETP}.csv")))?;let rm=read(d.join(format!("{RETP}.md")))?;
    let csv_changed=after==before.replace(",antes,",",despues,");
    let history_changed=h==format!("{hist}1,{}",ROW.replace(",antes,",",despues,"));
    let ok=match id{
        "P05"|"P07"=>r.success && r.err.is_empty() && r.out=="S1 1 2\n" && csv_changed && history_changed && rt.starts_with(&retp) && rt.lines().count()==3 && rt.contains("RETP-2026-2,") && rm.starts_with("# RETP\n") && rm.contains("RETP-2026-2") && read(s.join("SUCESOS_SV.md"))?.contains("**resultado:** despues"),
        "P06"=>!r.success && r.err.contains("AssertionError") && after==before && h==hist && rt==retp && rm=="# RETP\n" && read(s.join("SUCESOS_SV.md"))?=="# Sucesos\n\n## S1 · actividad\n\nantes\n",
        "P08"=>!r.success && r.err.contains("FileNotFoundError") && csv_changed && history_changed && rt==retp && rm=="# RETP\n" && !s.join("SUCESOS_SV.md").exists(),
        _=>false
    } && !r.timed_out;
    put(d.join("observacion.txt"),format!("oraculo_cumplido={ok}\nrevision_solicitada={revision}\noptimizado={opt}\nmarkdown_ausente={missing}\ncsv_actualizado={csv_changed}\nhistorial_actualizado={history_changed}\nretp_sin_cambios={}\n",rt==retp && rm=="# RETP\n"))?;
    Ok(ok)
}
fn run()->io::Result<()>{
    let a:Vec<_>=env::args_os().collect();require(a.len()==3,"Uso: observar DIRECTORIO_NUEVO PYTHON_ABSOLUTO")?;
    let root=PathBuf::from(&a[1]);let python=PathBuf::from(&a[2]);require(python.is_absolute(),"Python debe ser ruta absoluta")?;
    fs::create_dir(&root)?; // Nunca reutiliza una campaña ni confía en checkpoint.
    let version=Command::new(&python).env_clear().arg("--version").output()?;
    require(version.status.success(),"No se pudo identificar Python")?;put(root.join("python-version.txt"),&version.stdout)?;
    let mut rows=String::from("caso\toraculo_cumplido\tinterpretacion\n");let mut all=true;
    for (id,arg,opt,meaning) in [("P01","good",false,"control_positivo"),("P02","bad",false,"rechazo_normal"),("P03","bad",true,"guarda_eludida"),("P04","stale",false,"checkpoint_ajeno_aceptado")]{
        let ok=publisher(&root,&python,id,arg,opt)?;all &=ok;rows.push_str(&format!("{id}\t{ok}\t{meaning}\n"));put(root.join("RESULTADOS.tsv"),&rows)?;
    }
    for (id,rev,opt,missing,meaning) in [("P05",1,false,false,"control_positivo"),("P06",99,false,false,"rechazo_normal"),("P07",99,true,false,"revision_incompatible_aceptada"),("P08",1,false,true,"escritura_parcial_tras_error")]{
        let ok=registrar(&root,&python,id,rev,opt,missing)?;all &=ok;rows.push_str(&format!("{id}\t{ok}\t{meaning}\n"));put(root.join("RESULTADOS.tsv"),&rows)?;
    }
    print!("{rows}");require(all,"Discrepancia con banco previo; conservar campaña y revisar sin cambiar oráculo")
}
fn main(){if let Err(e)=run(){eprintln!("ERROR_CONDUCTOR: {e}");std::process::exit(1);}}
