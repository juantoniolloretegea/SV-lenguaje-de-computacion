//! Control acotado de conservación, registros y rutas documentales. Rust 1.98.0.
//! Uso: control ENTRADAS_TSV RUTAS_GIT CSV HISTORIAL MD
//! ENTRADAS_TSV: ruta local, ruta canónica, bytes y blob esperado; sin cabecera.
//! RUTAS_GIT: git ls-tree -r --name-only del corte fijado.
//! No ejecuta programas de los expedientes. No accede a la red.
//! Rutas locales adaptables; los tamaños y blobs esperados no se modifican.
//! Alcance de enlaces: destinos de enlaces Markdown relativos; no valida anclas ni descarga destinos externos.
#![forbid(unsafe_code)]
#![allow(dead_code,unused_imports)]
use std::{fs,collections::{BTreeMap,BTreeSet}};
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




fn records(s:&str)->Vec<(usize,usize)> {
 let b=s.as_bytes(); let(mut i,mut start,mut quoted)=(0,0,false); let mut r=Vec::new();
 while i<b.len(){if b[i]==b'"'{if quoted && i+1<b.len()&&b[i+1]==b'"'{i+=1;}else{quoted=!quoted;}}
 else if b[i]==b'\n'&&!quoted{r.push((start,i+1));start=i+1;}i+=1;}
 assert!(!quoted);if start<b.len(){r.push((start,b.len()));}r
}
fn fields(s:&str)->Vec<String>{
 let s=s.trim_end_matches(['\n','\r']);let b=s.as_bytes();let(mut i,mut quoted)=(0,false);let mut out=Vec::new();let mut f=Vec::new();
 while i<b.len(){match b[i]{b'"'=>{if quoted&&i+1<b.len()&&b[i+1]==b'"'{f.push(b'"');i+=1;}else{quoted=!quoted;}},
 b',' if !quoted=>{out.push(String::from_utf8(std::mem::take(&mut f)).unwrap());},c=>f.push(c)}i+=1;}
 assert!(!quoted);out.push(String::from_utf8(f).unwrap());out
}
fn serial(r:&[String])->String{r.iter().map(|s|format!("\"{}\"",s.replace('"',"\"\""))).collect::<Vec<_>>().join(",")+"\n"}

fn decode_url(s:&str)->String{let b=s.as_bytes();let mut v=Vec::new();let mut i=0;while i<b.len(){if b[i]==b'%'&&i+2<b.len(){let h=std::str::from_utf8(&b[i+1..i+3]).ok().and_then(|x|u8::from_str_radix(x,16).ok());if let Some(x)=h{v.push(x);i+=3;continue;}}v.push(b[i]);i+=1;}String::from_utf8(v).unwrap()}
fn main(){
 auditbase::selfcheck();
 let a:Vec<String>=std::env::args().collect();assert_eq!(a.len(),6);
 let routes:BTreeSet<String>=fs::read_to_string(&a[2]).unwrap().lines().map(str::to_owned).collect();
 let mut cnt=0;let mut rust=0;let mut mdlinks=0;let mut missing=0;let mut ext=0;
 for line in fs::read_to_string(&a[1]).unwrap().lines(){
  let f:Vec<_>=line.split('\t').collect();assert_eq!(f.len(),4);let b=fs::read(f[0]).unwrap();
  assert_eq!(b.len(),f[2].parse::<usize>().unwrap(),"{}",f[1]);assert_eq!(auditbase::blob(&b),f[3],"{}",f[1]);cnt+=1;
  println!("IDENTIDAD\t{}\t{}\t{}\t{}",f[1],b.len(),f[3],auditbase::h256(&b));
  if f[1].ends_with(".rs"){rust+=1;let s=std::str::from_utf8(&b).unwrap();println!("FUENTE\t{}\tforbid_unsafe={}\tinvocacion_subproceso={}\tinclude_externo={}",f[1],s.contains("#![forbid(unsafe_code)]"),s.contains("Command::"),s.contains("#[path")||s.contains("include!("));}
  if !f[1].ends_with(".md"){continue;}
  for link in auditbase::links(std::str::from_utf8(&b).unwrap()){
   if link.contains("://"){ext+=1;continue;}if link.starts_with('#'){continue;}
   let decoded=decode_url(link.split('#').next().unwrap());let path=decoded.as_str();if path.is_empty(){continue;}mdlinks+=1;
   let full=if path.starts_with('/') {path.trim_start_matches('/').to_string()}else{format!("{}/{}",auditbase::parent(f[1]).display(),path)};
   let found=auditbase::norm(&full).map(|p|routes.contains(&p)||routes.iter().any(|r|r.starts_with(&(p.clone()+"/")))).unwrap_or(false);
   if !found{missing+=1;println!("RUTA_AUSENTE\t{}\t{}",f[1],link);}
  }
 }
 let csv=fs::read_to_string(&a[3]).unwrap();let rr=records(&csv);let h=fields(&csv[rr[0].0..rr[0].1]);
 let hist=fs::read_to_string(&a[4]).unwrap();let hr=records(&hist);let mut histories:BTreeMap<String,Vec<Vec<String>>>=BTreeMap::new();
 for (s,e) in hr.iter().skip(1){let r=fields(&hist[*s..*e]);assert_eq!(r.len(),h.len()+1);histories.entry(r[1].clone()).or_default().push(r);}
 let md=fs::read_to_string(&a[5]).unwrap();let mut seen=BTreeSet::new();let mut md_diff=0;
 for (s,e) in rr.iter().skip(1){
  let r=fields(&csv[*s..*e]);assert_eq!(r.len(),h.len());assert!(seen.insert(r[0].clone()));
  assert!(["pendiente","en ejecución","finalizado"].contains(&r[1].as_str()));
  let hh=&histories[&r[0]];for (i,x)in hh.iter().enumerate(){assert_eq!(x[0].parse::<usize>().unwrap(),i);}
  assert_eq!(&hh.last().unwrap()[1..],&r[..],"historial {}",r[0]);
  let heading=format!("## {} · {}",r[0],r[7]);let pos=md.find(&heading).unwrap();let end=md[pos+heading.len()..].find("\n## ").map(|i|pos+heading.len()+i).unwrap_or(md.len());let block=&md[pos..end];
  for (i,key)in h.iter().enumerate(){if i==0||i==7{continue;}let expected=format!("**{key}:** {}",if r[i].is_empty(){"—"}else{&r[i]});if !block.contains(&expected){md_diff+=1;println!("DIFERENCIA_MD\t{}\t{}",r[0],key);}}
 }
 for i in 0..seen.len(){assert!(seen.contains(&format!("S{i}")));}
 assert_eq!(seen.len(),histories.len());
 println!("RESUMEN\tidentidades={}\tfuentes_rust={}\tsucesos={}\tinstantaneas={}\tmd_diferencias={}\tenlaces_relativos={}\trutas_ausentes={}\tenlaces_externos_no_descargados={}",cnt,rust,seen.len(),hr.len()-1,md_diff,mdlinks,missing,ext);
 if md_diff!=0||missing!=0{std::process::exit(1);}
}
