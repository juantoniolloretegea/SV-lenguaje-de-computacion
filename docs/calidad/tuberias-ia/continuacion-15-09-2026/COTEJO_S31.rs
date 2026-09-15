//! Cotejo documental acotado S31. Rust 1.98.0. Sin ejecución de sujetos SV.
//! Uso: cotejo_s31 DIRECTORIO_ENTRADAS. Dependencia flate2 1.1.9 del paquete fijado.
//! Entradas: MAPA.html, sv_s31_frame.b64, ENTRADAS_S31.tsv y sv_s31_p3_0..10.
//! Los nombres sv_s31_* designan copias auxiliares; el TSV conserva las rutas canónicas.
//! Se conserva el código de lectores y huellas del paquete histórico; no es una
//! validación criptográfica independiente ni una nueva campaña funcional.
#![forbid(unsafe_code)]
#![allow(dead_code,unused_imports)]
mod auditbase {
use std::{collections::BTreeMap,fs,path::{Path,PathBuf}};
#[derive(Debug)] enum FalloT { Entero, Utf8 }
type RT<T> = Result<T,FalloT>;
// SHA-256 para integridad de bytes, sin función normativa. Las sumas modulares
// pertenecen exclusivamente a SHA-256; no se usan para recursos ni longitudes.
const SHA_K:[u32;64]=[
0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2];
fn sha_bloque(h:&mut[u32;8],b:&[u8;64]){
    let mut w=[0u32;64];for i in 0..16{w[i]=u32::from_be_bytes([b[i*4],b[i*4+1],b[i*4+2],b[i*4+3]]);}
    for i in 16..64{let x=w[i-15];let y=w[i-2];let s0=x.rotate_right(7)^x.rotate_right(18)^(x>>3);let s1=y.rotate_right(17)^y.rotate_right(19)^(y>>10);w[i]=w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);}
    let[mut a,mut bb,mut c,mut d,mut e,mut f,mut g,mut hh]=*h;
    for i in 0..64{let s1=e.rotate_right(6)^e.rotate_right(11)^e.rotate_right(25);let ch=(e&f)^(!e&g);let t1=hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(SHA_K[i]).wrapping_add(w[i]);let s0=a.rotate_right(2)^a.rotate_right(13)^a.rotate_right(22);let maj=(a&bb)^(a&c)^(bb&c);let t2=s0.wrapping_add(maj);hh=g;g=f;f=e;e=d.wrapping_add(t1);d=c;c=bb;bb=a;a=t1.wrapping_add(t2);}
    for(i,v)in[a,bb,c,d,e,f,g,hh].into_iter().enumerate(){h[i]=h[i].wrapping_add(v);}
}
fn sha256(b:&[u8])->RT<[u8;32]>{
    let bits=u64::try_from(b.len()).map_err(|_|FalloT::Entero)?.checked_mul(8).ok_or(FalloT::Entero)?;
    let mut h=[0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19];
    let mut chunks=b.chunks_exact(64);for x in &mut chunks{let block=<&[u8;64]>::try_from(x).map_err(|_|FalloT::Entero)?;sha_bloque(&mut h,block);}
    let rest=chunks.remainder();let mut end=[0u8;128];end[..rest.len()].copy_from_slice(rest);end[rest.len()]=0x80;let size=if rest.len()<56{64}else{128};end[size-8..size].copy_from_slice(&bits.to_be_bytes());for x in end[..size].chunks_exact(64){sha_bloque(&mut h,<&[u8;64]>::try_from(x).map_err(|_|FalloT::Entero)?);}
    let mut out=[0;32];for(i,x)in h.iter().enumerate(){out[i*4..i*4+4].copy_from_slice(&x.to_be_bytes());}Ok(out)
}
fn hex_sha(b:&[u8;32])->[u8;64]{let mut out=[0;64];let hex=b"0123456789abcdef";for(i,x)in b.iter().enumerate(){out[i*2]=hex[(x>>4)as usize];out[i*2+1]=hex[(x&15)as usize];}out}
fn sha_texto(b:&[u8])->RT<[u8;64]>{Ok(hex_sha(&sha256(b)?))}
fn sha_str(b:&[u8;64])->RT<&str>{std::str::from_utf8(b).map_err(|_|FalloT::Utf8)}

pub fn h256(b:&[u8])->String { String::from_utf8(sha_texto(b).unwrap().to_vec()).unwrap() }
pub fn sha1(b:&[u8])->String {
 let mut h=[0x67452301u32,0xefcdab89,0x98badcfe,0x10325476,0xc3d2e1f0];
 let mut data=b.to_vec();data.push(0x80);while data.len()%64!=56{data.push(0)}data.extend_from_slice(&(u64::try_from(b.len()).unwrap().checked_mul(8).unwrap()).to_be_bytes());
 for chunk in data.chunks_exact(64){let mut w=[0u32;80];for i in 0..16{w[i]=u32::from_be_bytes(chunk[i*4..i*4+4].try_into().unwrap());}for i in 16..80{w[i]=(w[i-3]^w[i-8]^w[i-14]^w[i-16]).rotate_left(1);}
 let [mut a,mut bb,mut c,mut d,mut e]=h;for (i,x) in w.iter().enumerate(){let(f,k)=match i{0..=19=>((bb&c)|(!bb&d),0x5a827999),20..=39=>(bb^c^d,0x6ed9eba1),40..=59=>((bb&c)|(bb&d)|(c&d),0x8f1bbcdc),_=>(bb^c^d,0xca62c1d6)};let t=a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(*x);e=d;d=c;c=bb.rotate_left(30);bb=a;a=t;}for(i,x)in[a,bb,c,d,e].iter().enumerate(){h[i]=h[i].wrapping_add(*x);}}
 h.iter().map(|x|format!("{x:08x}")).collect()
}
pub fn blob(b:&[u8])->String { let mut v=format!("blob {}\0",b.len()).into_bytes();v.extend_from_slice(b);sha1(&v) }
#[derive(Clone,Debug)]pub struct Entry{pub path:String,pub mode:String,pub kind:String,pub sha:String,pub size:usize}
pub fn tree(p:&Path)->Vec<Entry>{let s=fs::read_to_string(p).unwrap();let mut seen=std::collections::BTreeSet::new();s.lines().map(|l|{let f:Vec<_>=l.split('\t').collect();assert_eq!(f.len(),5);assert!(seen.insert(f[0]));assert!(!f[0].starts_with('/')&&!f[0].split('/').any(|x|x==".."));assert_eq!(f[3].len(),40);Entry{path:f[0].into(),mode:f[1].into(),kind:f[2].into(),sha:f[3].into(),size:f[4].parse().unwrap_or(0)}}).collect()}
pub fn read(p:&Path)->std::io::Result<Vec<u8>>{use std::io::Read;let m=fs::symlink_metadata(p)?;if !m.is_file()||m.len()>128*1024*1024{return Err(std::io::Error::other("tipo/cuota"))}let mut b=Vec::new();fs::File::open(p)?.take(128*1024*1024+1).read_to_end(&mut b)?;if b.len()>128*1024*1024{return Err(std::io::Error::other("cuota"))}Ok(b)}
pub fn q(s:&str)->String{let mut out=String::from("\"");for c in s.chars(){match c{'"'=>out.push_str("\\\""),'\\'=>out.push_str("\\\\"),'\n'=>out.push_str("\\n"),'\r'=>out.push_str("\\r"),'\t'=>out.push_str("\\t"),c if c<' '=>out.push_str(&format!("\\u{:04x}",c as u32)),c=>out.push(c)}}out.push('"');out}
pub fn csvout(rows:&[Vec<String>])->String{rows.iter().map(|r|r.iter().map(|s|format!("\"{}\"",s.replace('"',"\"\""))).collect::<Vec<_>>().join(",")).collect::<Vec<_>>().join("\n")+"\n"}
pub fn norm(p:&str)->Option<String>{let mut v=vec![];for x in p.split('/'){match x{""|"."=>(),".."=>{v.pop()?;},_=>v.push(x)}}Some(v.join("/"))}
pub fn links(s:&str)->Vec<String>{let mut out=vec![];let mut rest=s;while let Some(i)=rest.find("]("){rest=&rest[i+2..];let mut depth=1;let mut end=None;for (j,c)in rest.char_indices(){if c=='(' {depth+=1}if c==')'{depth-=1;if depth==0{end=Some(j);break}}}if let Some(n)=end{out.push(rest[..n].trim_matches(|c|c=='<'||c=='>').to_string());rest=&rest[n+1..];}else{break}}out}
pub fn selfcheck(){assert_eq!(sha1(b""),"da39a3ee5e6b4b0d3255bfef95601890afd80709");assert_eq!(sha1(b"abc"),"a9993e364706816aba3e25717850c26c9cd0d89d");assert_eq!(blob(b""),"e69de29bb2d1d6434b8b29ae775ad8c2e48c5391");assert_eq!(h256(b""),"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");assert_eq!(h256(b"abc"),"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");assert_ne!(blob(b"A"),blob(b"B"));assert_eq!(norm("a/b/../c").as_deref(),Some("a/c"));assert_eq!(norm("../a"),None);}
pub fn put(p:&Path,b:&[u8]){fs::create_dir_all(p.parent().unwrap()).unwrap();fs::write(p,b).unwrap()}
pub fn index(v:&[Entry])->BTreeMap<String,Entry>{v.iter().map(|e|(e.path.clone(),e.clone())).collect()}
pub fn parent(p:&str)->PathBuf{Path::new(p).parent().unwrap_or(Path::new("")).to_path_buf()}

}
mod auxiliar_historico {
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

}
mod json_auditoria {
//! ES: AST documental acotada; adaptación de recepción/escapes IE004.
//! EN: Bounded documentary AST; IE004 reception/escape adaptation.
use std::io::Read;
#[derive(Clone, Debug)]
pub enum J {
    Null,
    Bool(bool),
    Number(String),
    Text(String),
    Array(Vec<J>),
    Object(Vec<(String, J)>),
}
impl PartialEq for J {
    fn eq(&self, b: &Self) -> bool {
        match (self, b) {
            (Self::Null, Self::Null) => true,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Number(a), Self::Number(b)) | (Self::Text(a), Self::Text(b)) => a == b,
            (Self::Array(a), Self::Array(b)) => a == b,
            (Self::Object(a), Self::Object(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .all(|(k, v)| b.iter().any(|(q, w)| k == q && v == w))
            }
            _ => false,
        }
    }
}
impl J {
    pub fn get(&self, k: &str) -> Option<&J> {
        if let Self::Object(v) = self {
            v.iter().find(|(s, _)| s == k).map(|(_, j)| j)
        } else {
            None
        }
    }
    pub fn field(&self, k: &str) -> &J {
        self.get(k).unwrap_or(&J::Null)
    }
    pub fn text(&self) -> Option<&str> {
        if let Self::Text(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn lexeme(&self) -> Option<&str> {
        if let Self::Number(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn uint(&self) -> Option<usize> {
        self.lexeme()?.parse().ok()
    }
    pub fn array(&self) -> Option<&[J]> {
        if let Self::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }
    pub fn boolean(&self) -> Option<bool> {
        if let Self::Bool(x) = self {
            Some(*x)
        } else {
            None
        }
    }
    pub fn pairs(&self) -> Option<&[(String, J)]> {
        if let Self::Object(a) = self {
            Some(a)
        } else {
            None
        }
    }
    pub fn set(&mut self, k: &str, v: J) {
        if let Self::Object(a) = self {
            if let Some((_, x)) = a.iter_mut().find(|(s, _)| s == k) {
                *x = v
            } else {
                a.push((k.into(), v))
            }
        }
    }
}
pub fn s(x: &str) -> J {
    J::Text(x.into())
}
pub fn n(x: usize) -> J {
    J::Number(x.to_string())
}
pub fn obj(v: Vec<(&str, J)>) -> J {
    J::Object(v.into_iter().map(|(k, v)| (k.into(), v)).collect())
}
pub type E = &'static str;
// ES: Se lee como máximo cuota+1, sin abrir rutas procedentes del sujeto.
// EN: Read at most quota+1, without opening paths supplied by the subject.
pub fn receive(r: &mut impl Read, max: usize) -> Result<Vec<u8>, E> {
    let mut out = Vec::new();
    out.try_reserve_exact(max).map_err(|_| "RESERVA")?;
    let mut buf = [0u8; 4096];
    loop {
        let remaining = max.checked_sub(out.len()).ok_or("BYTES")?;
        let cap = 4096.min(remaining.checked_add(1).ok_or("BYTES")?);
        let len = r.read(&mut buf[..cap]).map_err(|_| "IO")?;
        if len == 0 {
            return Ok(out);
        }
        if len > remaining {
            return Err("BYTES");
        }
        out.extend_from_slice(&buf[..len]);
    }
}
fn push<T>(v: &mut Vec<T>, x: T, max: usize) -> Result<(), E> {
    if v.len() >= max {
        return Err("RESERVA");
    }
    v.try_reserve_exact(1).map_err(|_| "RESERVA")?;
    v.push(x);
    Ok(())
}
struct Decoder<'a> {
    b: &'a [u8],
    i: usize,
    nodes: usize,
}
impl Decoder<'_> {
    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }
    fn take(&mut self) -> Result<u8, E> {
        let b = self.peek().ok_or("JSON_SINTAXIS")?;
        self.i += 1;
        Ok(b)
    }
    fn ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.i += 1
        }
    }
    fn expect(&mut self, b: u8) -> Result<(), E> {
        if self.take()? == b {
            Ok(())
        } else {
            Err("JSON_SINTAXIS")
        }
    }
    fn hex4(&mut self) -> Result<u32, E> {
        let mut x = 0;
        for _ in 0..4 {
            let d = match self.take()? {
                b @ b'0'..=b'9' => (b - b'0') as u32,
                b @ b'a'..=b'f' => (b - b'a' + 10) as u32,
                b @ b'A'..=b'F' => (b - b'A' + 10) as u32,
                _ => return Err("JSON_SINTAXIS"),
            };
            x = x * 16 + d;
        }
        Ok(x)
    }
    fn string(&mut self) -> Result<String, E> {
        self.expect(b'"')?;
        let mut v = Vec::new();
        loop {
            match self.take()? {
                b'"' => break,
                0..=31 => return Err("JSON_SINTAXIS"),
                b'\\' => {
                    let e = self.take()?;
                    let decoded = match e {
                        b'"' => Some(b'"'),
                        b'\\' => Some(b'\\'),
                        b'/' => Some(b'/'),
                        b'b' => Some(8),
                        b'f' => Some(12),
                        b'n' => Some(b'\n'),
                        b'r' => Some(b'\r'),
                        b't' => Some(b'\t'),
                        b'u' => None,
                        _ => return Err("JSON_SINTAXIS"),
                    };
                    if let Some(x) = decoded {
                        push(&mut v, x, 67108864)?
                    } else {
                        let hi = self.hex4()?;
                        let cp = if (0xd800..=0xdbff).contains(&hi) {
                            self.expect(b'\\')?;
                            self.expect(b'u')?;
                            let lo = self.hex4()?;
                            if !(0xdc00..=0xdfff).contains(&lo) {
                                return Err("JSON_SINTAXIS");
                            }
                            0x10000 + ((hi - 0xd800) << 10) + (lo - 0xdc00)
                        } else {
                            hi
                        };
                        let ch = char::from_u32(cp).ok_or("JSON_SINTAXIS")?;
                        let mut bytes = [0u8; 4];
                        for x in ch.encode_utf8(&mut bytes).bytes() {
                            push(&mut v, x, 67108864)?
                        }
                    }
                }
                x => push(&mut v, x, 67108864)?,
            }
        }
        String::from_utf8(v).map_err(|_| "UTF8")
    }
    fn value(&mut self, depth: usize) -> Result<J, E> {
        self.ws();
        if matches!(self.peek(), Some(b'[' | b'{')) && depth >= 32 {
            return Err("PROFUNDIDAD");
        }
        self.nodes += 1;
        if self.nodes > 1000000 {
            return Err("NODOS");
        }
        match self.peek().ok_or("JSON_SINTAXIS")? {
            b'"' => Ok(J::Text(self.string()?)),
            b'n' => {
                for b in b"null" {
                    self.expect(*b)?
                }
                Ok(J::Null)
            }
            b't' => {
                for b in b"true" {
                    self.expect(*b)?
                }
                Ok(J::Bool(true))
            }
            b'f' => {
                for b in b"false" {
                    self.expect(*b)?
                }
                Ok(J::Bool(false))
            }
            b'-' | b'0'..=b'9' => {
                let start = self.i;
                if self.peek() == Some(b'-') {
                    self.i += 1;
                }
                let first = self.take()?;
                if !first.is_ascii_digit() {
                    return Err("NUMERO_FORMA");
                }
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.i += 1;
                }
                let digits = &self.b[start..self.i];
                if first == b'0' && self.i - start > if digits[0] == b'-' { 2 } else { 1 } {return Err("NUMERO_FORMA")}
                if self.peek()==Some(b'.'){self.i+=1;let p=self.i;while matches!(self.peek(),Some(b'0'..=b'9')){self.i+=1}if self.i==p{return Err("NUMERO_FORMA")}}
                if matches!(self.peek(),Some(b'e'|b'E')){self.i+=1;if matches!(self.peek(),Some(b'+'|b'-')){self.i+=1}let p=self.i;while matches!(self.peek(),Some(b'0'..=b'9')){self.i+=1}if self.i==p{return Err("NUMERO_FORMA")}}
                let digits = &self.b[start..self.i];
                Ok(J::Number(
                    std::str::from_utf8(digits).map_err(|_| "UTF8")?.into(),
                ))
            }
            b'[' => {
                self.i += 1;
                self.ws();
                let mut a = Vec::new();
                if self.peek() == Some(b']') {
                    self.i += 1;
                    return Ok(J::Array(a));
                }
                loop {
                    let x = self.value(depth + 1)?;
                    push(&mut a, x, 1000000)?;
                    self.ws();
                    match self.take()? {
                        b']' => break,
                        b',' => (),
                        _ => return Err("JSON_SINTAXIS"),
                    }
                }
                Ok(J::Array(a))
            }
            b'{' => {
                self.i += 1;
                self.ws();
                let mut a: Vec<(String, J)> = Vec::new();
                if self.peek() == Some(b'}') {
                    self.i += 1;
                    return Ok(J::Object(a));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    if a.iter().any(|(old, _)| old == &k) {
                        return Err("CLAVE_DUPLICADA");
                    }
                    if a.len() >= 65536 {
                        return Err("MIEMBROS");
                    }
                    self.ws();
                    self.expect(b':')?;
                    let x = self.value(depth + 1)?;
                    push(&mut a, (k, x), 65536)?;
                    self.ws();
                    match self.take()? {
                        b'}' => break,
                        b',' => (),
                        _ => return Err("JSON_SINTAXIS"),
                    }
                }
                Ok(J::Object(a))
            }
            _ => Err("JSON_SINTAXIS"),
        }
    }
}
pub fn decode(b: &[u8]) -> Result<J, E> {
    if b.len() > 67108864 {
        return Err("BYTES");
    }
    std::str::from_utf8(b).map_err(|_| "UTF8")?;
    let mut d = Decoder { b, i: 0, nodes: 0 };
    let j = d.value(0)?;
    d.ws();
    if d.i != b.len() {
        return Err("JSON_SINTAXIS");
    }
    Ok(j)
}
// ES: Serialización de recibos limitada antes de cada escritura.
// EN: Receipt serialization bounded before every write.
pub fn encode(j: &J, max: usize) -> Result<Vec<u8>, E> {
    struct Out {
        b: Vec<u8>,
        max: usize,
    }
    impl Out {
        fn raw(&mut self, b: &[u8]) -> Result<(), E> {
            if self.b.len().checked_add(b.len()).ok_or("BYTES")? > self.max {
                return Err("BYTES");
            }
            self.b.extend_from_slice(b);
            Ok(())
        }
        fn string(&mut self, s: &str) -> Result<(), E> {
            self.raw(b"\"")?;
            for c in s.chars() {
                match c {
                    '"' => self.raw(b"\\\"")?,
                    '\\' => self.raw(b"\\\\")?,
                    c if (c as u32) < 32 => self.raw(format!("\\u{:04x}", c as u32).as_bytes())?,
                    c => {
                        let mut b = [0; 4];
                        self.raw(c.encode_utf8(&mut b).as_bytes())?
                    }
                }
            }
            self.raw(b"\"")
        }
        fn value(&mut self, j: &J) -> Result<(), E> {
            match j {
                J::Null => self.raw(b"null"),
                J::Bool(b) => self.raw(if *b { b"true" } else { b"false" }),
                J::Number(s) => self.raw(s.as_bytes()),
                J::Text(s) => self.string(s),
                J::Array(a) => {
                    self.raw(b"[")?;
                    for (i, j) in a.iter().enumerate() {
                        if i > 0 {
                            self.raw(b",")?
                        }
                        self.value(j)?
                    }
                    self.raw(b"]")
                }
                J::Object(a) => {
                    self.raw(b"{")?;
                    for (i, (k, j)) in a.iter().enumerate() {
                        if i > 0 {
                            self.raw(b",")?
                        }
                        self.string(k)?;
                        self.raw(b":")?;
                        self.value(j)?
                    }
                    self.raw(b"}")
                }
            }
        }
    }
    let mut b = Vec::new();
    b.try_reserve_exact(max).map_err(|_| "RESERVA")?;
    let mut o = Out { b, max };
    o.value(j)?;
    Ok(o.b)
}

}
mod identidad {
use crate::auditbase::*;use crate::auxiliar_historico::csv;
use std::{fs,path::{Path,PathBuf},collections::BTreeSet,io::Read};
const MAX:usize=512*1024*1024;
fn b64(s:&[u8])->Vec<u8>{let b:Vec<_>=s.iter().copied().filter(|b|!b.is_ascii_whitespace()).collect();assert_eq!(b.len()%4,0);let mut out=vec![];for c in b.chunks_exact(4){let mut v=[0;4];for j in 0..4{v[j]=match c[j]{b'A'..=b'Z'=>c[j]-b'A',b'a'..=b'z'=>c[j]-b'a'+26,b'0'..=b'9'=>c[j]-b'0'+52,b'+'=>62,b'/'=>63,b'='=>0,_=>panic!("base64")}}out.push(v[0]<<2|v[1]>>4);if c[2]!=b'='{out.push(v[1]<<4|v[2]>>2)}if c[3]!=b'='{out.push(v[2]<<6|v[3])}}out}
fn inflate(mut r:impl Read)->Vec<u8>{let mut out=vec![];r.by_ref().take(MAX as u64+1).read_to_end(&mut out).unwrap();assert!(out.len()<=MAX);out}
fn txt(b:&[u8])->String{String::from_utf8(b.iter().copied().take_while(|b|*b!=0).collect()).unwrap()}
fn oct(b:&[u8])->usize{usize::from_str_radix(txt(b).trim(),8).unwrap_or(0)}
fn crc(b:&[u8])->u32{let mut c=!0u32;for x in b{c^=*x as u32;for _ in 0..8{c=(c>>1)^if c&1!=0{0xedb88320}else{0};}}!c}
fn putfile(_root:&Path,_source:&str,name:&str,b:&[u8],rows:&mut Vec<Vec<String>>){
 let target=match name {"salida/REFERENCIAS_SHA256.csv"=>Some("sv_s31_refs"),"salida/NODOS_VERIFICADOS.json"=>Some("sv_s31_nodes"),"entrada/NODOS.json"=>Some("sv_s31_defs"),"README_REPRODUCCION.md"=>Some("sv_s31_readme"),_=>None};
 if let Some(p)=target{fs::write(p,b).unwrap();rows.push(vec![name.into(),b.len().to_string(),h256(b)]);}
}
fn tar(root:&Path,source:&str,b:&[u8],rows:&mut Vec<Vec<String>>){let mut i=0;let mut long=None;while i+512<=b.len(){let hdr=&b[i..i+512];if hdr.iter().all(|b|*b==0){break}let checksum:usize=hdr.iter().enumerate().map(|(n,b)|if (148..156).contains(&n){32}else{*b as usize}).sum();assert_eq!(checksum,oct(&hdr[148..156]),"checksum tar {source}");let n=oct(&hdr[124..136]);let start=i+512;let end=start.checked_add(n).unwrap();assert!(end<=b.len());let data=&b[start..end];let mut name=txt(&hdr[..100]);let prefix=txt(&hdr[345..500]);if !prefix.is_empty(){name=format!("{prefix}/{name}")}if let Some(l)=long.take(){name=l}match hdr[156]{b'0'|0=>putfile(root,source,&name,data,rows),b'L'=>long=Some(txt(data)),b'x'=>{for line in std::str::from_utf8(data).unwrap().lines(){if let Some((_,p))=line.split_once(" path="){long=Some(p.to_string());}}},b'5'|b'2'|b'g'=>(),t=>panic!("tar tipo {t}")};i=end.div_ceil(512)*512;}}
fn u16le(b:&[u8],i:usize)->usize{u16::from_le_bytes(b[i..i+2].try_into().unwrap())as usize}fn u32le(b:&[u8],i:usize)->usize{u32::from_le_bytes(b[i..i+4].try_into().unwrap())as usize}
fn zip(root:&Path,source:&str,b:&[u8],rows:&mut Vec<Vec<String>>){let e=(b.len().saturating_sub(65557)..b.len().saturating_sub(21)).rev().find(|&i|b[i..].starts_with(b"PK\x05\x06")&&i+22+u16le(b,i+20)==b.len()).expect("zip end");let mut i=u32le(b,e+16);let count=u16le(b,e+10);assert!(count<65535);for _ in 0..count{assert_eq!(&b[i..i+4],b"PK\x01\x02");let method=u16le(b,i+10);let size=u32le(b,i+20);let expanded=u32le(b,i+24);let n=u16le(b,i+28);let name=std::str::from_utf8(&b[i+46..i+46+n]).unwrap();let off=u32le(b,i+42);assert_eq!(&b[off..off+4],b"PK\x03\x04");assert_eq!(u16le(b,i+8)&1,0,"zip encrypted");let start=off+30+u16le(b,off+26)+u16le(b,off+28);let compressed=&b[start..start+size];let data=match method{0=>compressed.to_vec(),8=>inflate(flate2::read::DeflateDecoder::new(compressed)),_=>panic!("zip metodo")};assert_eq!(data.len(),expanded);assert_eq!(crc(&data)as usize,u32le(b,i+16));if !name.ends_with('/'){putfile(root,source,name,&data,rows)}i+=46+n+u16le(b,i+30)+u16le(b,i+32);}}

pub fn run(){
 selfcheck();let b=fs::read("MAPA.html").unwrap();
 assert_eq!(blob(&b),"24e25dbf1255ddf25d5f1e7eefa8d89d4b12ed3e");println!("HTML {} {}",b.len(),h256(&b));
 let html=std::str::from_utf8(&b).unwrap();
 let marker="UEsDB";let pos=html.find(marker).unwrap();let end=pos+html[pos..].find("</script>").unwrap();
 let zipbytes=b64(html[pos..end].as_bytes());println!("ZIP {} {}",zipbytes.len(),h256(&zipbytes));
 let mut rows=vec![];zip(Path::new("/tmp"),"html",&zipbytes,&mut rows);assert_eq!(rows.len(),4);for r in rows{println!("EMBEBIDO {}",r.join("\t"));}
 let png=b64(&fs::read("sv_s31_frame.b64").unwrap());assert_eq!(png.len(),255245);assert_eq!(blob(&png),"f123822692430d79b0b5b44d2aa4ffca4c238c94");fs::write("sv_s31_frame-original.png",&png).unwrap();println!("PNG {} {} {}",png.len(),blob(&png),h256(&png));
 let manifest=fs::read_to_string("ENTRADAS_S31.tsv").unwrap();for line in manifest.lines(){let c:Vec<_>=line.split('\t').collect();let bytes=fs::read(c[0]).unwrap();assert_eq!(blob(&bytes),c[1],"ENTRADA {}",c[0]);println!("ENTRADA {} {} {} {}",c[0],bytes.len(),c[1],h256(&bytes));}
}


}
mod contraste {
use crate::{auditbase,json_auditoria,auxiliar_historico};
use std::{fs,collections::BTreeMap};
fn canon(s:&str)->String{for (a,b) in [("lab:laboratorio/tareas-watson/tuberias-ia/","main:docs/calidad/tuberias-ia/"),("lab:laboratorio/tareas-watson/riesgos-materiales-s26/","main:docs/calidad/riesgos-materiales-s26/"),("lab:laboratorio/tareas-watson/sucesos-sv/","main:docs/calidad/Inventario-sv/sucesos/")] {if let Some(t)=s.strip_prefix(a){return format!("{b}{t}")}}s.into()}
fn matches(src:&str,scope:&str)->bool{let raw=src.split('#').next().unwrap();let test=|a:&str,b:&str|a==b||(b.ends_with('/')&&a.starts_with(b));test(raw,scope)||test(&canon(raw),&canon(scope))}
pub fn run(){
 
 let h=fs::read("MAPA.html").unwrap();
 println!("HTML bytes={} blob={} sha256={}",h.len(),auditbase::blob(&h),auditbase::h256(&h));
 let s=String::from_utf8(h).unwrap();let k=s.find("id=\"node-data\"").unwrap();let a=k+s[k..].find('>').unwrap()+1;let b=a+s[a..].find("</script>").unwrap();
 let n=json_auditoria::decode(s[a..b].as_bytes()).unwrap();let nodes=n.array().unwrap();
 assert_eq!(auditbase::blob(s.as_bytes()),"24e25dbf1255ddf25d5f1e7eefa8d89d4b12ed3e");
 let embedded=json_auditoria::decode(&fs::read("sv_s31_nodes").unwrap()).unwrap();
 let mut diffs=BTreeMap::new();for(p,z)in nodes.iter().zip(embedded.array().unwrap()){assert_eq!(p.get("id").unwrap().text(),z.get("id").unwrap().text());for (k,v) in z.pairs().unwrap(){let other=p.get(k).unwrap();if json_auditoria::encode(v,1000000).unwrap()!=json_auditoria::encode(other,1000000).unwrap(){*diffs.entry(k.clone()).or_insert(0)+=1;}}}
 println!("DIFERENCIAS_CAMPOS_COMUNES {:?}",diffs);
 let mut oldlinks=0;let mut added=0;let mut removed=0;let mut allpaths=0;
 let defs0=json_auditoria::decode(&fs::read("sv_s31_defs").unwrap()).unwrap();
 for((p,z),d)in nodes.iter().zip(embedded.array().unwrap()).zip(defs0.array().unwrap()){
 let toset=|j:&json_auditoria::J|j.get("links").unwrap().array().unwrap().iter().map(|v|v.text().unwrap().to_string()).collect::<std::collections::BTreeSet<_>>();let pp=toset(p);let zz=toset(z);oldlinks+=zz.len();added+=pp.difference(&zz).count();removed+=zz.difference(&pp).count();
 assert_eq!(json_auditoria::encode(p.get("paths").unwrap(),1000000).unwrap(),json_auditoria::encode(d.get("paths").unwrap(),1000000).unwrap());allpaths+=p.get("paths").unwrap().array().unwrap().len();}
 println!("ENLACES salida={oldlinks} anadidos={added} retirados={removed} paths_fieles={allpaths}");
 let mut links=0;let mut private=0;let mut unresolved=0;let mut affected=0;
 for j in nodes {let u=j.get("unresolved").unwrap().uint().unwrap();unresolved+=u;if u>0{affected+=1;}for l in j.get("links").unwrap().array().unwrap(){links+=1;if l.text().unwrap().contains("/SV-matematica-semantica-cuaternaria/"){private+=1}}}
 println!("nodos={} afectados={} unresolved={} enlaces={} privados={}",nodes.len(),affected,unresolved,links,private);
 let defs=json_auditoria::decode(&fs::read("sv_s31_defs").unwrap()).unwrap();let defs=defs.array().unwrap();
 let rows=auxiliar_historico::csv(&fs::read("sv_s31_refs").unwrap()).unwrap();let mut amber=0;let mut unassigned=0;let mut duplicated=0;let mut sources=BTreeMap::new();
 for r in &rows[1..] {if !r[3].starts_with("AMBAR"){continue}amber+=1;let count=defs.iter().filter(|j|j.get("paths").unwrap().array().unwrap().iter().any(|p|matches(&r[0],p.text().unwrap()))).count();if count==0 {unassigned+=1;*sources.entry(r[0].clone()).or_insert(0)+=1;}if count>1{duplicated+=1}}
 println!("referencias={} ambar={} sin_nodo={} multiples_nodos={}",rows.len()-1,amber,unassigned,duplicated);for(s,n)in sources{println!("SIN_NODO {n} {s}");}
 let j=json_auditoria::decode(&fs::read("sv_s31_p3_0").unwrap()).unwrap();println!("PUBLICO version={} casos={}",j.get("version").unwrap().text().unwrap(),j.get("casos").unwrap().array().unwrap().len());
 let expected=json_auditoria::decode(&fs::read("sv_s31_p3_5").unwrap()).unwrap();
 for name in ["1","7"] {let v=json_auditoria::decode(&fs::read(format!("sv_s31_p3_{name}")).unwrap()).unwrap();for c in v.get("casos").unwrap().array().unwrap(){if c.get("vigente").unwrap().boolean()==Some(false){let id=c.get("id").unwrap().text().unwrap();let source=c.get("fuente").unwrap().text().unwrap();let e=expected.get("casos").unwrap().array().unwrap().iter().find(|x|x.get("id").unwrap().text()==Some(id)).unwrap();println!("VIGENCIA archivo={name} id={id} fuente={source} esperado_historico={}",e.get("cuerpo_esperado").unwrap().get("vigente").unwrap().boolean().unwrap());}}}
}


}
fn main() {
 let dir=std::env::args_os().nth(1).expect("DIRECTORIO_ENTRADAS");
 std::env::set_current_dir(dir).expect("DIRECTORIO_NO_ACCESIBLE");
 identidad::run();
 contraste::run();
}
