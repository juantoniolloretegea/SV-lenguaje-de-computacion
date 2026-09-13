//! Auxiliar operativo local; no forma parte del núcleo ni de la semántica SV.
use std::{collections::{BTreeMap, BTreeSet}, fs::{self, File, OpenOptions}, io::{Read, Write}, path::Path};
pub type R<T> = Result<T, String>;
pub const FILES: [&str; 5] = ["SUCESOS_SV.csv", "HISTORIAL_SUCESOS_SV.csv", "SUCESOS_SV.md", "REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv", "REGISTRO_EVOLUCION_TECNICA_PROYECTO.md"];
const LIMIT: u64 = 4 * 1024 * 1024;
fn need(ok: bool, code: &str) -> R<()> { if ok { Ok(()) } else { Err(code.into()) } }
fn io<T>(r: std::io::Result<T>) -> R<T> { r.map_err(|e| format!("IO: {e}")) }
pub fn read(path: &Path) -> R<Vec<u8>> {
    need(io(fs::symlink_metadata(path))?.file_type().is_file(), "NO_REGULAR")?;
    let f=io(File::open(path))?; let mut b=Vec::new();io(f.take(LIMIT+1).read_to_end(&mut b))?;
    need(b.len() as u64<=LIMIT,"CUOTA_ARCHIVO")?;Ok(b)
}
fn text(b: &[u8]) -> R<&str> { std::str::from_utf8(b).map_err(|_| "UTF8".into()) }
fn number(s: &str) -> R<u64> { need(!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()),"NATURAL")?;s.parse().map_err(|_|"NATURAL_RANGO".into()) }
// CSV estricto: comillas dobles escapadas, CRLF o LF; sin normalizar contenido.
pub fn csv(b: &[u8]) -> R<Vec<Vec<String>>> {
    let s=text(b)?;let mut out=Vec::new();let mut row=Vec::new();let mut field=String::new();
    let mut chars=s.chars().peekable();let(mut quoted,mut closed)=(false,false);
    while let Some(c)=chars.next(){
        if quoted { if c=='"' {if chars.peek()==Some(&'"'){chars.next();field.push('"');}else{quoted=false;closed=true;}}else{field.push(c);}continue; }
        if c=='"' {need(field.is_empty()&&!closed,"CSV_COMILLA")?;quoted=true;continue;}
        if c==',' {row.push(std::mem::take(&mut field));closed=false;continue;}
        if c=='\r'||c=='\n' {if c=='\r'{need(chars.next()==Some('\n'),"CSV_CR")?;}
            row.push(std::mem::take(&mut field));out.push(std::mem::take(&mut row));closed=false;continue;}
        need(!closed,"CSV_TRAS_COMILLA")?;field.push(c);
    }
    need(!quoted,"CSV_INCOMPLETO")?;
    if !field.is_empty()||!row.is_empty()||closed {row.push(field);out.push(row);}
    need(!out.is_empty()&&!out[0].is_empty(),"CSV_VACIO")?;
    let n=out[0].len();need(out.iter().all(|r|r.len()==n),"CSV_COLUMNAS")?;
    need(out[0].iter().collect::<BTreeSet<_>>().len()==n,"CSV_CABECERA_DUPLICADA")?;Ok(out)
}
fn col(rows:&[Vec<String>],key:&str)->R<usize>{rows[0].iter().position(|x|x==key).ok_or_else(||format!("COLUMNA: {key}"))}
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Publication { pub context:BTreeMap<String,String>,pub entries:BTreeMap<String,(String,String,String)> }
fn sha(s:&str)->bool{s.len()==40&&s.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))}
pub fn publication(bytes:&[u8])->R<Publication>{
    let s=text(bytes)?;let mut lines=s.lines();need(lines.next()==Some("SV-AUX-PUB/1"),"PUB_FORMATO")?;
    let(mut context,mut entries)=(BTreeMap::new(),BTreeMap::new());
    for line in lines {let f:Vec<_>=line.split('\t').collect();
        if f.first()==Some(&"entry") {need(f.len()==5,"PUB_ENTRY")?;
            let p=f[1];need(!p.is_empty()&&!p.starts_with('/')&&!p.contains('\\')&&p.split('/').all(|x|!x.is_empty()&&x!="."&&x!=".."),"PUB_RUTA")?;
            need(sha(f[4]),"PUB_SHA")?;
            need(matches!((f[2],f[3]),("100644"|"100755"|"120000","blob")|("160000","commit")),"PUB_TIPO_MODO")?;
            need(!entries.contains_key(p),"PUB_RUTA_DUPLICADA")?;entries.insert(p.into(),(f[2].into(),f[3].into(),f[4].into()));
        }else{need(f.len()==2&&matches!(f[0],"repo"|"branch"|"base"|"commit"|"complete"),"PUB_CAMPO")?;
            need(!f[1].is_empty()&&!context.contains_key(f[0]),"PUB_CONTEXTO")?;context.insert(f[0].into(),f[1].into());}
        need(entries.len()<=10000,"PUB_CUOTA")?;
    }
    need(context.len()==5,"PUB_CONTEXTO_INCOMPLETO")?;
    need(context.get("complete").map(String::as_str)==Some("true"),"PUB_TRUNCADO")?;
    need(sha(&context["base"])&&sha(&context["commit"]),"PUB_SHA")?;Ok(Publication{context,entries})
}
pub fn compare(expected:&Publication,observed:&Publication)->R<()> {
    need(expected.context.get("repo")==observed.context.get("repo"),"PUB_REPOSITORIO")?;
    need(expected.context.get("branch")==observed.context.get("branch"),"PUB_RAMA")?;
    need(expected.context.get("base")==observed.context.get("base"),"PUB_BASE")?;
    need(expected.context.get("commit")==observed.context.get("commit"),"PUB_COMMIT")?;
    need(expected.context==observed.context,"PUB_CONTEXTO")?;need(expected.entries==observed.entries,"PUB_ARBOL")
}
pub fn check_publication(expected:&Path,observed:&Path)->R<()> {
    // Siempre vuelve a leer y comparar. No recibe ni interpreta un checkpoint verified.
    compare(&publication(&read(expected)?)?,&publication(&read(observed)?)?)
}
#[derive(Clone,Debug)]pub struct Request{pub id:String,pub revision:u64,pub retp:u64}
pub fn request(b:&[u8])->R<Request>{
    let s=text(b)?;let mut l=s.lines();need(l.next()==Some("SV-AUX-REG/1"),"REG_FORMATO")?;
    let mut m=BTreeMap::new();for line in l{let f:Vec<_>=line.split('\t').collect();need(f.len()==2&&matches!(f[0],"id"|"revision"|"retp"),"REG_CAMPO")?;need(!m.contains_key(f[0]),"REG_DUPLICADO")?;m.insert(f[0],f[1]);}
    need(m.len()==3,"REG_INCOMPLETO")?;let id=m["id"];need(id.starts_with('S'),"REG_ID")?;number(&id[1..])?;
    Ok(Request{id:id.into(),revision:number(m["revision"])?,retp:number(m["retp"]) ?})
}
#[derive(Clone)]pub struct Bundle {pub bytes:[Vec<u8>;5]}
pub fn bundle(dir:&Path)->R<Bundle>{let mut bytes:[Vec<u8>;5]=std::array::from_fn(|_|Vec::new());for(i,name)in FILES.iter().enumerate(){bytes[i]=read(&dir.join(name))?;}Ok(Bundle{bytes})}
fn target<'a>(rows:&'a[Vec<String>],id:&str)->R<&'a Vec<String>>{
    let pos=col(rows,"id")?;let mut ids=BTreeSet::new();for r in &rows[1..]{need(ids.insert(&r[pos]),"REG_ID_DUPLICADO")?;}
    rows[1..].iter().find(|r|r[pos]==id).ok_or_else(||"REG_ID_AUSENTE".into())
}
pub fn validate(before:&Bundle,after:&Bundle,r:&Request)->R<()> {
    // Perfil acotado: actualiza un suceso existente y añade exactamente una revisión y un RETP.
    let b=csv(&before.bytes[0])?;let a=csv(&after.bytes[0])?;
    need(b[0]==a[0]&&b.len()==a.len(),"REG_FORMA")?;
    let old=target(&b,&r.id)?;let new=target(&a,&r.id)?;let id=col(&b,"id")?;
    need(b[1..].iter().zip(&a[1..]).all(|(x,y)|if x[id]==r.id{y[id]==r.id}else{x==y}),"REG_OTRAS_FILAS")?;
    need(matches!(new[col(&a,"estado")?].as_str(),"pendiente"|"en ejecución"|"finalizado"),"REG_ESTADO")?;
    let hb=csv(&before.bytes[1])?;let ha=csv(&after.bytes[1])?;
    let mut hh=vec!["revision".to_string()];hh.extend(b[0].clone());need(hb[0]==hh&&ha[0]==hh,"REG_HIST_CABECERA")?;
    let mut max=None;let mut last=None;let mut seen=BTreeSet::new();
    for row in &hb[1..]{let rev=number(&row[0])?;need(seen.insert((row[1].clone(),rev)),"REG_HIST_DUPLICADO")?;
        if row[1]==r.id&&max.is_none_or(|m|rev>m){max=Some(rev);last=Some(&row[1..]);}}
    let next=max.and_then(|v|v.checked_add(1)).ok_or("REG_REVISION_RANGO")?;
    need(next==r.revision,"REG_REVISION")?;need(last==Some(old.as_slice()),"REG_ACTUAL_HISTORIAL")?;
    need(after.bytes[1].starts_with(&before.bytes[1])&&ha.len()==hb.len()+1&&ha[..hb.len()]==hb,"REG_HIST_PREFIJO")?;
    let last=ha.last().ok_or("REG_HIST_VACIO")?;need(number(&last[0])?==r.revision&&last[1..]==new[..],"REG_HIST_NUEVO")?;
    let rb=csv(&before.bytes[3])?;let ra=csv(&after.bytes[3])?;
    need(rb[0]==ra[0]&&rb.len()>1,"REG_RETP_CABECERA")?;let rid=col(&rb,"ID")?;
    let previous=rb.last().ok_or("REG_RETP_VACIO")?[rid].strip_prefix("RETP-2026-").ok_or("REG_RETP_ID")?;
    need(number(previous)?.checked_add(1)==Some(r.retp),"REG_RETP_SECUENCIA")?;
    need(after.bytes[3].starts_with(&before.bytes[3])&&ra.len()==rb.len()+1&&ra[..rb.len()]==rb,"REG_RETP_PREFIJO")?;
    let rr=ra.last().ok_or("REG_RETP_VACIO")?;need(rr[rid]==format!("RETP-2026-{}",r.retp),"REG_RETP_NUEVO")?;
    for(k,s)in[("Frente_Fase","id"),("Resumen_Cambio","resultado"),("Evidencia","verificacion"),("Objecion_Adversarial","observaciones"),("Decision","siguiente_accion"),("Estado","estado")]{need(rr[col(&ra,k)?]==new[col(&a,s)?],"REG_RETP_CONCORDANCIA")?;}
    let bm=text(&before.bytes[2])?;let am=text(&after.bytes[2])?;let heading=format!("## {} · ",r.id);
    fn section<'a>(s:&'a str,heading:&str)->R<(&'a str,&'a str,&'a str)>{
        let starts:Vec<_>=s.match_indices(heading).filter(|(i,_)|*i==0||s.as_bytes()[i-1]==b'\n').map(|(i,_)|i).collect();need(starts.len()==1,"REG_MD_IDENTIDAD")?;
        let i=starts[0];let end=s[i+heading.len()..].find("\n## ").map(|n|i+heading.len()+n+1).unwrap_or(s.len());Ok((&s[..i],&s[i..end],&s[end..]))
    }
    let (bp,_,bs)=section(bm,&heading)?;let(ap,block,asuf)=section(am,&heading)?;need(bp==ap&&bs==asuf,"REG_MD_OTRAS_SECCIONES")?;
    need(block.starts_with(&format!("{heading}{}\n",new[col(&a,"actividad")?])),"REG_MD_ACTIVIDAD")?;
    for(i,k)in a[0].iter().enumerate(){if k=="id"||k=="actividad"{continue;}
        let label=format!("**{k}:** ");let lines:Vec<_>=block.lines().filter_map(|l|l.strip_prefix(&label)).collect();
        let value=if new[i].is_empty(){"—"}else{new[i].as_str()};need(lines==vec![value],"REG_MD_CAMPO")?;
    }
    let br=text(&before.bytes[4])?;let ar=text(&after.bytes[4])?;need(ar.starts_with(br),"REG_RETP_MD_PREFIJO")?;
    let appended=&ar[br.len()..];need(appended.contains(&format!("<a id=\"retp-{}\"></a>",r.retp)),"REG_RETP_MD_ID")?;
    for key in ["resultado","verificacion","observaciones","siguiente_accion"]{need(appended.contains(&new[col(&a,key)?]),"REG_RETP_MD_CONTENIDO")?;}
    Ok(())
}
#[derive(Clone,Copy)]pub enum Injection{None,AfterFile(usize)}
pub fn stage(before:&Path,candidate:&Path,output:&Path,r:&Request,inject:Injection)->R<()> {
    let source=io(before.canonicalize())?;let candidate=io(candidate.canonicalize())?;
    let parent=io(output.parent().ok_or("SALIDA_PADRE")?.canonicalize())?;
    need(!parent.starts_with(&source)&&!parent.starts_with(&candidate),"SALIDA_EN_ENTRADA")?;
    let name=output.file_name().ok_or("SALIDA_NOMBRE")?;let output=parent.join(name);
    let old=bundle(&source)?;let new=bundle(&candidate)?;validate(&old,&new,r)?;
    io(fs::create_dir(&output))?; // Rechaza destino existente; nunca lo limpia ni reutiliza.
    for(i,name)in FILES.iter().enumerate(){let mut f=io(OpenOptions::new().write(true).create_new(true).open(output.join(name)))?;
        io(f.write_all(&new.bytes[i]))?;io(f.sync_all())?;
        if matches!(inject,Injection::AfterFile(n) if n==i+1){return Err("FALLO_INYECTADO".into());}
    }
    let actual=bundle(&output)?;need(actual.bytes==new.bytes,"SALIDA_DIFIERE")?;
    // Una marca por sí sola no se acepta: verify_stage relee el conjunto completo.
    let mut f=io(OpenOptions::new().write(true).create_new(true).open(output.join("PREPARADO")))?;
    io(f.write_all(b"SV-AUX-PREPARADO/1\n"))?;io(f.sync_all())?;Ok(())
}
pub fn verify_stage(before:&Path,candidate:&Path,output:&Path,r:&Request)->R<()> {
    need(read(&output.join("PREPARADO"))?==b"SV-AUX-PREPARADO/1\n","MARCA")?;
    let actual=bundle(output)?;let wanted=bundle(candidate)?;need(actual.bytes==wanted.bytes,"SALIDA_DIFIERE")?;
    validate(&bundle(before)?,&actual,r)
}
pub fn cli(args:&[String])->R<()> {
    match args.get(1).map(String::as_str){
        Some("publicacion") if args.len()==4=>check_publication(Path::new(&args[2]),Path::new(&args[3])),
        Some("preparar") if args.len()==6=>stage(Path::new(&args[3]),Path::new(&args[4]),Path::new(&args[5]),&request(&read(Path::new(&args[2]))?)?,Injection::None),
        Some("comprobar") if args.len()==6=>verify_stage(Path::new(&args[3]),Path::new(&args[4]),Path::new(&args[5]),&request(&read(Path::new(&args[2]))?)?),
        _=>Err("USO: publicacion ESPERADO OBSERVADO | preparar/comprobar SOLICITUD ANTES CANDIDATA SALIDA".into())
    }
}
