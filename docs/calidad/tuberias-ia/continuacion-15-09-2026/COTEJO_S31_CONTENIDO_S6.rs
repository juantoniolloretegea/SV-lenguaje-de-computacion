//! Recepción de evidencia ZIP S6: identidad previa y descompresión con CRC en memoria.
//! Uso: cotejo ZIP CUALIFICACION_PUBLICA COMPROMISO VERIFICADOR. Rust 1.98.0; flate2 1.1.9.
//! Las funciones de huella proceden de COTEJO_S31.rs; no es validación criptográfica independiente.
//! No ejecuta contenidos ni escribe los miembros del archivo.
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

use std::{io::Read,collections::BTreeSet};
fn u16l(b:&[u8],p:usize)->usize{u16::from_le_bytes(b[p..p+2].try_into().unwrap()) as usize}
fn u32l(b:&[u8],p:usize)->usize{u32::from_le_bytes(b[p..p+4].try_into().unwrap()) as usize}
fn crc(b:&[u8])->u32{let mut c=!0u32;for x in b{c^=*x as u32;for _ in 0..8{c=(c>>1)^if c&1!=0{0xedb88320}else{0};}}!c}
mod json {
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
fn main(){
 auditbase::selfcheck();
 let args:Vec<String>=std::env::args().collect();assert_eq!(args.len(),5,"uso: cotejo ZIP CUALIFICACION_PUBLICA COMPROMISO VERIFICADOR");let p=args[1].clone();
 assert_eq!(std::fs::metadata(&p).unwrap().len(),1811714);
 let b=std::fs::read(p).unwrap();assert_eq!(b.len(),1811714);
 let blob=auditbase::blob(&b);assert_eq!(blob,"ffe5c4a9b86b26c2409abe4239e6355f7a3bef19");
 let sha=auditbase::h256(&b);assert_eq!(sha,"48e2c1e5acc384687dcd346613a945bf18798a105c72b94b6adb72be23115e1a");
 println!("IDENTIDAD bytes={} blob={} sha256={}",b.len(),blob,sha);
 let e=(b.len().saturating_sub(65557)..b.len()-21).rev().find(|&i|b[i..].starts_with(b"PK\x05\x06")&&i+22+u16l(&b,i+20)==b.len()).unwrap();
 assert_eq!(u16l(&b,e+4),0);assert_eq!(u16l(&b,e+6),0);
 let count=u16l(&b,e+10);assert_eq!(count,487);assert_eq!(u16l(&b,e+8),count);
 let start_cd=u32l(&b,e+16);assert_eq!(start_cd+u32l(&b,e+12),e);
 let mut members=std::collections::BTreeMap::new();let mut pos=start_cd;let mut names=BTreeSet::new();let mut ranges=Vec::new();let mut total=0usize;
 for _ in 0..count {
  assert_eq!(&b[pos..pos+4],b"PK\x01\x02");
  let flags=u16l(&b,pos+8);assert_eq!(flags,0);assert_eq!(u16l(&b,pos+10),8);
  let checksum=u32l(&b,pos+16);let size=u32l(&b,pos+20);let expanded=u32l(&b,pos+24);
  assert!(expanded<=16*1024*1024);total=total.checked_add(expanded).unwrap();assert!(total<=128*1024*1024);
  let n=u16l(&b,pos+28);let name=std::str::from_utf8(&b[pos+46..pos+46+n]).unwrap();
  assert!(names.insert(name.to_owned()));assert!(!name.starts_with('/')&&!name.split('/').any(|s|s==".."));
  let off=u32l(&b,pos+42);assert_eq!(&b[off..off+4],b"PK\x03\x04");
  assert_eq!(u16l(&b,off+6),flags);assert_eq!(u16l(&b,off+8),8);
  assert_eq!(u32l(&b,off+14),checksum);assert_eq!(u32l(&b,off+18),size);assert_eq!(u32l(&b,off+22),expanded);
  assert_eq!(&b[off+30..off+30+u16l(&b,off+26)],name.as_bytes());
  let start=off+30+u16l(&b,off+26)+u16l(&b,off+28);let end=start.checked_add(size).unwrap();assert!(end<=start_cd);
  ranges.push((off,end));
  let mut decoder=flate2::read::DeflateDecoder::new(&b[start..end]);let mut data=Vec::new();
  decoder.by_ref().take(expanded as u64+1).read_to_end(&mut data).unwrap();
  assert_eq!(data.len(),expanded);assert_eq!(decoder.total_in(),size as u64);assert_eq!(crc(&data)as usize,checksum);
  members.insert(name.to_owned(),data);pos+=46+n+u16l(&b,pos+30)+u16l(&b,pos+32);
 }
 assert_eq!(pos,e);ranges.sort_unstable();assert_eq!(ranges[0].0,0);
 for pair in ranges.windows(2){assert_eq!(pair[0].1,pair[1].0);}assert_eq!(ranges.last().unwrap().1,start_cd);
 let mut expected:BTreeSet<String>=(0..482).map(|i|format!("controles/{i:04}.json")).collect();
 for n in ["CUALIFICACION_COMPLETA.json","CLI-referencia.json","CLI-fallo.json","INCIDENCIA_PREPARACION.txt","cualificar.py"]{expected.insert(n.into());}
 assert_eq!(names,expected);
 println!("ZIP entradas=487 controles_json=482 auxiliares=5 metodo=8 nombres_unicos=si");
 println!("INTEGRIDAD cabeceras_y_limites=ok descompresion=487/487 crc=487/487 bytes_descomprimidos={total}");
 println!("ALCANCE recuperacion_e_identidad; sin_extraccion_a_archivos; sin_ejecucion_de_contenidos; sin_repeticion_de_controles");


 let g=&members["cualificar.py"];assert_eq!(g.len(),8588);assert_eq!(auditbase::h256(g),"52684359885d5e84a2a94d9ccc46cd99fb1e915bfa9fbcff42c5472a71717a56");println!("GENERADOR bytes={} sha256={}",g.len(),auditbase::h256(g));
 let c=json::decode(&members["CUALIFICACION_COMPLETA.json"]).unwrap();
 let public_b=std::fs::read(&args[2]).unwrap();assert_eq!(auditbase::blob(&public_b),"63c8c0e2c8ccdef37a7b07d1bfe8286f05763de8");
 let public=json::decode(&public_b).unwrap();
 let co_b=std::fs::read(&args[3]).unwrap();assert_eq!(auditbase::blob(&co_b),"06d8ba491dc249fa0cccd9ea5270f155b2683f40");let commitment=json::decode(&co_b).unwrap();
 let ver=std::fs::read(&args[4]).unwrap();assert_eq!(auditbase::blob(&ver),"e428d9f58530fd8de388ac43f436894722bea6c4");
 assert_eq!(auditbase::h256(&ver),c.field("verificador_sha256").text().unwrap());
 assert_eq!(c.field("verificador_sha256"),commitment.field("verificador_sha256"));
 assert_eq!(auditbase::h256(&members["controles/0000.json"]),commitment.field("referencia_sha256").text().unwrap());
 let controls=c.field("controles").array().unwrap();let pc=public.field("controles").array().unwrap();assert_eq!(controls.len(),482);assert_eq!(pc.len(),482);
 let mut fam=std::collections::BTreeMap::new();
 for(a,b)in controls.iter().zip(pc){let path=format!("controles/{}",a.field("archivo").text().unwrap());assert_eq!(auditbase::h256(&members[&path]),a.field("sha256").text().unwrap());for key in ["id","control","familia","esperado","observado"]{assert_eq!(a.field(key),b.field(key));}assert_eq!(a.field("esperado"),a.field("observado"));*fam.entry(a.field("familia").text().unwrap()).or_insert(0usize)+=1;}
 assert_eq!(fam.len(),17);
 println!("COTEJO controles_sha256=482/482 campos_compartidos=2410/2410 esperado_observado=482/482 familias=17");
 println!("REFERENCIA_PREVIA coincide_con_compromiso=si verificador_publico_coincide=si");
 println!("MANIFIESTOS campo_generador_sha256_completo={} publico={} compromiso={}",c.get("generador_sha256").is_some(),public.get("generador_sha256").is_some(),commitment.get("generador_sha256").is_some());
 for a in c.field("cli").array().unwrap(){let path=format!("CLI-{}.json",a.field("control").text().unwrap());assert_eq!(auditbase::h256(&members[&path]),a.field("salida_sha256").text().unwrap());}println!("CLI huellas_salidas=2/2");
}
