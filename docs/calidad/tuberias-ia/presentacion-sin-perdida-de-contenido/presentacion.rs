//! Perfil candidato de formato, no interfaz profesional ni certificado de dominio.
use super::{EntregaLiteral,Fallo,Trabajo,json_vista::Vista};
pub const VERSION:&str="IE004-PRESENTACION-ESPACIOS/1";
const MAX_PRESENTACION:usize=16384;
const MAX_ORIGINAL:usize=4096;
const MAX_TRABAJO:u64=1_000_000;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum FalloPresentacion { Limite, Sintaxis(Fallo), ContenidoDistinto }
/// Vista prestada e inmutable: sin constructor público, sin autoridad de actuación.
pub struct PresentacionComprobada<'a>{texto:&'a[u8],original:&'a[u8],identidad:(u64,u64),trabajo:u64}
impl PresentacionComprobada<'_>{
 pub fn texto(&self)->&[u8]{self.texto}
 pub fn original(&self)->&[u8]{self.original}
 pub fn identidad(&self)->(u64,u64){self.identidad}
 pub fn perfil(&self)->&'static str{VERSION}
 pub fn trabajo(&self)->u64{self.trabajo}
}
// Conserva cada byte dentro de las cadenas, incluidos espacios y escapes.
// Sólo se usa después del reconocimiento sintáctico completo de ambos textos.
struct Contenido<'a>{b:&'a[u8],i:usize,cadena:bool,escape:bool}
impl<'a> Contenido<'a>{fn nuevo(b:&'a[u8])->Self{Self{b,i:0,cadena:false,escape:false}}}
impl Iterator for Contenido<'_>{type Item=u8;fn next(&mut self)->Option<u8>{
 while let Some(&b)=self.b.get(self.i){self.i+=1;
  if self.cadena {if self.escape{self.escape=false}else if b==b'\\'{self.escape=true}else if b==b'"'{self.cadena=false}return Some(b)}
  if b==b'"'{self.cadena=true;return Some(b)}
  if !matches!(b,b' '|b'\t'|b'\r'|b'\n'){return Some(b)}
 }None
}}
fn comprobar(original:&[u8],texto:&[u8])->Result<u64,FalloPresentacion>{
 if original.len()>MAX_ORIGINAL||texto.len()>MAX_PRESENTACION{return Err(FalloPresentacion::Limite)}
 let mut c=Trabajo::nuevo();c.max=MAX_TRABAJO;
 Vista::nueva(original,&mut c).map_err(FalloPresentacion::Sintaxis)?;
 Vista::nueva(texto,&mut c).map_err(FalloPresentacion::Sintaxis)?;
 c.cobrar((original.len()+texto.len())as u64).map_err(FalloPresentacion::Sintaxis)?;
 if !Contenido::nuevo(original).eq(Contenido::nuevo(texto)){return Err(FalloPresentacion::ContenidoDistinto)}
 Ok(c.usado)
}
impl EntregaLiteral<'_>{
 /// El receptor escoge este perfil explícito. El contenido de la propuesta no lo cambia.
 /// La vida de la vista queda ligada a la entrega original y al texto comprobado.
 pub fn comprobar_presentacion<'a>(&'a self,texto:&'a[u8])->Result<PresentacionComprobada<'a>,FalloPresentacion>{
  let trabajo=comprobar(self.cuerpo(),texto)?;
  Ok(PresentacionComprobada{texto,original:self.cuerpo(),identidad:self.identidad(),trabajo})
 }
}
#[cfg(test)]mod tests;
