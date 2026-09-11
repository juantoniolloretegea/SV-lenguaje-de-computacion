//! Vista sin árbol ni reserva dinámica. Perfil canónico producido por A/V.
use super::{Fallo,Trabajo};
pub(super) struct Vista<'a>{pub b:&'a[u8]}
struct Lector<'a,'c>{b:&'a[u8],i:usize,c:&'c mut Trabajo}
impl<'a,'c>Lector<'a,'c>{
 fn peek(&mut self)->Result<Option<u8>,Fallo>{self.c.cobrar(1)?;Ok(self.b.get(self.i).copied())}
 fn take(&mut self)->Result<u8,Fallo>{let x=self.peek()?.ok_or(Fallo::Json)?;self.i+=1;Ok(x)}
 fn want(&mut self,x:u8)->Result<(),Fallo>{if self.take()?!=x{return Err(Fallo::Json)}Ok(())}
 fn ws(&mut self)->Result<(),Fallo>{while matches!(self.peek()?,Some(b' '|b'\r'|b'\n'|b'\t')){self.i+=1;}Ok(())}
 fn hex4(&mut self)->Result<u32,Fallo>{let mut n=0;for _ in 0..4{n=n*16+match self.take()?{x@b'0'..=b'9'=>(x-b'0')as u32,x@b'a'..=b'f'=>(x-b'a'+10)as u32,x@b'A'..=b'F'=>(x-b'A'+10)as u32,_=>return Err(Fallo::Json)}}Ok(n)}
 fn string(&mut self)->Result<&'a[u8],Fallo>{let start=self.i;self.want(b'"')?;loop{match self.take()?{
  b'"'=>return Ok(&self.b[start..self.i]),0..=31=>return Err(Fallo::Json),b'\\'=>match self.take()?{
   b'"'|b'\\'|b'/'|b'b'|b'f'|b'n'|b'r'|b't'=>(),b'u'=>{let hi=self.hex4()?;if (0xd800..=0xdbff).contains(&hi){self.want(b'\\')?;self.want(b'u')?;let lo=self.hex4()?;if !(0xdc00..=0xdfff).contains(&lo){return Err(Fallo::Json)}}else if (0xdc00..=0xdfff).contains(&hi){return Err(Fallo::Json)}},_=>return Err(Fallo::Json)},_=>()}}}
 fn key(&mut self)->Result<&'a[u8],Fallo>{let k=self.string()?;if k.len()>66||k[1..k.len()-1].iter().any(|x|!x.is_ascii()||*x==b'\\'){return Err(Fallo::Esquema)}Ok(k)}
 fn value(&mut self,d:usize)->Result<&'a[u8],Fallo>{if d>96{return Err(Fallo::Profundidad)}self.ws()?;let start=self.i;match self.peek()?.ok_or(Fallo::Json)?{
  b'"'=>{self.string()?;},b'n'=>{for x in b"null"{self.want(*x)?}},b't'=>{for x in b"true"{self.want(*x)?}},b'f'=>{for x in b"false"{self.want(*x)?}},
  b'0'..=b'9'=>{let first=self.take()?;let mut n=(first-b'0')as u64;if first==b'0'&&matches!(self.peek()?,Some(b'0'..=b'9')){return Err(Fallo::Json)}while let Some(b'0'..=b'9')=self.peek()?{let digit=(self.take()?-b'0')as u64;n=n.checked_mul(10).and_then(|n|n.checked_add(digit)).ok_or(Fallo::Json)?;}},
  b'['=>{self.take()?;self.ws()?;if self.peek()?==Some(b']'){self.take()?;}else{loop{self.value(d+1)?;self.ws()?;match self.take()?{b']'=>break,b','=>(),_=>return Err(Fallo::Json)}}}},
  b'{'=>{self.take()?;self.ws()?;let mut keys:[Option<&[u8]>;32]=[None;32];let mut count=0;if self.peek()?==Some(b'}'){self.take()?;}else{loop{if count==32{return Err(Fallo::Esquema)}self.ws()?;let k=self.key()?;for old in keys[..count].iter().flatten(){self.c.cobrar(old.len().max(k.len())as u64)?;if *old==k{return Err(Fallo::Duplicado)}}keys[count]=Some(k);count+=1;self.ws()?;self.want(b':')?;self.value(d+1)?;self.ws()?;match self.take()?{b'}'=>break,b','=>(),_=>return Err(Fallo::Json)}}}},
  _=>return Err(Fallo::Json)}Ok(&self.b[start..self.i])}
}
impl<'a>Vista<'a>{
 pub fn nueva(b:&'a[u8],c:&mut Trabajo)->Result<Self,Fallo>{c.cobrar(b.len()as u64)?;std::str::from_utf8(b).map_err(|_|Fallo::Json)?;let mut p=Lector{b,i:0,c};p.value(0)?;p.ws()?;if p.i!=b.len(){return Err(Fallo::Json)}Ok(Self{b})}
 pub fn campo(&self,key:&str,c:&mut Trabajo)->Result<&'a[u8],Fallo>{let mut p=Lector{b:self.b,i:0,c};p.ws()?;p.want(b'{')?;p.ws()?;if p.peek()?==Some(b'}'){return Err(Fallo::Esquema)}loop{p.ws()?;let k=p.key()?;p.ws()?;p.want(b':')?;let v=p.value(1)?;p.c.cobrar(k.len().max(key.len())as u64)?;if &k[1..k.len()-1]==key.as_bytes(){return Ok(v)}p.ws()?;match p.take()?{b'}'=>return Err(Fallo::Esquema),b','=>(),_=>return Err(Fallo::Json)}}}
 pub fn claves(&self,allowed:&[&str],c:&mut Trabajo)->Result<(),Fallo>{let mut p=Lector{b:self.b,i:0,c};p.ws()?;p.want(b'{')?;let mut count=0;loop{p.ws()?;if p.peek()?==Some(b'}'){p.take()?;break}let k=p.key()?;let mut found=false;for a in allowed{p.c.cobrar(k.len().max(a.len())as u64)?;if &k[1..k.len()-1]==a.as_bytes(){found=true}}if !found{return Err(Fallo::Esquema)}count+=1;p.ws()?;p.want(b':')?;p.value(1)?;p.ws()?;match p.take()?{b'}'=>break,b','=>(),_=>return Err(Fallo::Json)}}if count!=allowed.len(){return Err(Fallo::Esquema)}Ok(())}
}
