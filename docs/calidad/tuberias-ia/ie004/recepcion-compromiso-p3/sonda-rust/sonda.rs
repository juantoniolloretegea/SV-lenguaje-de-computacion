#![forbid(unsafe_code)]
// Instrumento previo de capacidad; NO corrector semántico de IE004-ES-P2/2.
// Reconoce la proyección sintáctica de G01-G25, sin usar los casos reservados.
use std::convert::TryFrom;

#[derive(Clone, Copy)]
enum E { L(&'static str), R(usize), S(&'static [E]), A(&'static [E]), O(&'static E), M(&'static E), D }
macro_rules! l { ($v:literal) => { E::L($v) }; }
macro_rules! r { ($v:literal) => { E::R($v - 1) }; }
macro_rules! s { ($($v:expr),* $(,)?) => { E::S(&[$($v),*]) }; }
macro_rules! a { ($($v:expr),* $(,)?) => { E::A(&[$($v),*]) }; }
macro_rules! o { ($v:expr) => { E::O(&$v) }; }
macro_rules! m { ($v:expr) => { E::M(&$v) }; }
const G: [E; 25] = [
 s!(o!(l!("¿")),r!(2),o!(a!(l!("?"),l!(".")))),
 a!(r!(3),s!(r!(18),a!(l!(";"),l!(":")),r!(3)),s!(r!(3),a!(l!(","),l!(";")),r!(18)),s!(r!(3),l!(","),r!(18),l!(":"),r!(3)),s!(r!(19),l!(","),r!(3)),s!(r!(19),l!("no"),l!(";"),r!(19),l!(","),r!(3)),r!(18)),
 s!(r!(4),o!(r!(20)),o!(r!(21))),
 a!(s!(r!(5),r!(6)),s!(l!("escriba"),r!(23),l!("en"),r!(6)),r!(6),s!(a!(l!("cuál es"),l!("adónde estará"),l!("y")),r!(6)),s!(l!("qué"),r!(9),l!("tiene"),r!(8)),s!(l!("en qué unidad"),a!(l!("está expresada"),l!("figura")),r!(8))),
 s!(o!(a!(l!("sólo"),l!("solo"))),r!(24)),
 s!(r!(7),m!(r!(13))),
 a!(r!(9),r!(10),r!(12),s!(r!(22),l!("registro"))),
 s!(a!(r!(10),s!(r!(22),l!("registro"))),m!(r!(13))),
 s!(r!(22),a!(l!("valor"),l!("dato"),s!(l!("cifra"),o!(l!("registrada"))),l!("unidad"),s!(l!("estado"),o!(l!("registrado"))),l!("fuente"),l!("procedencia"),l!("límites"),l!("alcance"),l!("intervalo de referencia"))),
 a!(r!(11),s!(r!(11),l!("o"),r!(11))),
 s!(o!(a!(l!("el"),l!("la"),l!("esa"))),a!(l!("igg"),l!("iga"),l!("igm"),l!("inmunoglobulina"),l!("inmunoglobina"),l!("linmunoglobina"),s!(l!("hemoglobina"),o!(l!("glicosilada"))))),
 s!(r!(22),a!(l!("actual"),l!("ahora"),l!("anterior"),l!("previo"))),
 a!(r!(12),s!(a!(l!("de"),l!("para")),r!(10)),s!(l!("para"),r!(14)),r!(25),s!(a!(l!("de"),l!("del")),o!(l!("corte")),r!(17)),s!(l!("de la"),r!(17)),s!(l!("en el corte"),r!(17)),s!(l!("del"),a!(l!("dato"),l!("registro")))),
 a!(r!(15),r!(16)),
 a!(l!("caso-a"),l!("caso-b")),
 a!(s!(a!(l!("este"),l!("ese")),a!(l!("caso"),l!("paciente"))),l!("el caso que estamos viendo"),s!(l!("este"),l!("..."))),
 a!(l!("actual"),l!("ahora"),l!("anterior"),l!("previo")),
 a!(s!(l!("no"),r!(24),r!(6)),s!(l!("no escriba"),r!(23),l!("en"),r!(6)),s!(l!("no quiero"),r!(6)),s!(l!("no"),r!(6)),l!("no cambie nada")),
 r!(25),s!(l!("porque estoy revisando"),r!(14)),s!(o!(l!(",")),l!("por favor")),
 o!(a!(l!("el"),l!("la"))),E::D,
 a!(l!("consulte"),l!("lea"),l!("traiga"),l!("dígame"),l!("necesito"),l!("cambie"),l!("elimine")),
 a!(s!(a!(l!("de"),l!("del")),r!(15)),s!(l!("de"),a!(s!(a!(l!("este"),l!("ese")),a!(l!("caso"),l!("paciente"))),s!(l!("este"),l!("...")))),l!("del caso que estamos viendo"))
];

#[derive(Debug, PartialEq, Eq)]
enum Error { Overflow, Conversion, Limite, Reserva, Rango, Utf8, Bytes, Tokens }
type R<T> = Result<T, Error>;
fn suma(a:u64,b:u64)->R<u64>{a.checked_add(b).ok_or(Error::Overflow)}
fn producto(a:u64,b:u64)->R<u64>{a.checked_mul(b).ok_or(Error::Overflow)}
fn indice(n:u64)->R<usize>{usize::try_from(n).map_err(|_|Error::Conversion)}
fn vista(s:&str,inicio:u64,fin:u64)->R<&str>{
 if inicio>fin || fin>u64::try_from(s.len()).map_err(|_|Error::Conversion)? {return Err(Error::Rango)}
 s.get(indice(inicio)?..indice(fin)?).ok_or(Error::Utf8)
}
#[derive(Default)]
struct Cuenta { usado:u64, limite:u64 }
impl Cuenta {
 fn cobrar(&mut self,n:u64)->R<()> {
  let siguiente=suma(self.usado,n)?;
  if siguiente>self.limite{return Err(Error::Limite)}
  self.usado=siguiente;Ok(())
 }
}
struct Token { inicio:u64,fin:u64,normal:String }
fn separador(c:char)->bool {matches!(c,' '| '\t' | '\n' | '\r')}
fn signo(c:char)->bool {matches!(c,'¿'|'?'|'.'|','|':'|';')}
fn minuscula(c:char)->char {
 match c {'A'..='Z'=>c.to_ascii_lowercase(),'Á'=>'á','É'=>'é','Í'=>'í','Ó'=>'ó','Ú'=>'ú','Ü'=>'ü','Ñ'=>'ñ',_=>c}
}
fn lexicalizar(q:&str)->R<Vec<Token>>{
 if q.len()>8192{return Err(Error::Bytes)}
 let mut t=Vec::new();t.try_reserve_exact(128).map_err(|_|Error::Reserva)?;
 let mut i=0usize;
 while i<q.len(){
  let c=q.get(i..).ok_or(Error::Rango)?.chars().next().ok_or(Error::Rango)?;
  if separador(c){i=indice(suma(i as u64,c.len_utf8() as u64)?)?;continue}
  let desde=i;
  if q.get(i..).ok_or(Error::Rango)?.starts_with("..."){i=indice(suma(i as u64,3)?)?}
  else if signo(c){i=indice(suma(i as u64,c.len_utf8() as u64)?)?}
  else{
   while i<q.len(){
    let c=q.get(i..).ok_or(Error::Rango)?.chars().next().ok_or(Error::Rango)?;
    if separador(c)||signo(c){break}
    i=indice(suma(i as u64,c.len_utf8() as u64)?)?;
   }
  }
  if t.len()==128{return Err(Error::Tokens)}
  let mut normal=String::new();normal.try_reserve_exact(i-desde).map_err(|_|Error::Reserva)?;
  for c in vista(q,desde as u64,i as u64)?.chars(){normal.push(minuscula(c))}
  t.push(Token{inicio:desde as u64,fin:i as u64,normal});
 }
 Ok(t)
}

struct Analisis<'a>{tokens:&'a[Token],tabla:Vec<u8>,ancho:usize,cuenta:Cuenta,estados:u64,barridos:u64,ultimo:[usize;4]}
impl<'a> Analisis<'a>{
 fn nuevo(tokens:&'a[Token])->R<Self>{
  let ancho=suma(tokens.len() as u64,1)?;
  let total=producto(25,producto(ancho,ancho)?)?;
  if total>32*1024*1024{return Err(Error::Limite)}
  let mut tabla=Vec::new();tabla.try_reserve_exact(indice(total)?).map_err(|_|Error::Reserva)?;tabla.resize(indice(total)?,0);
  Ok(Self{tokens,tabla,ancho:indice(ancho)?,cuenta:Cuenta{usado:0,limite:1_000_000},estados:0,barridos:0,ultimo:[0;4]})
 }
 fn pos(&self,g:usize,i:usize,j:usize)->R<usize>{
  indice(suma(producto(suma(producto(g as u64,self.ancho as u64)?,i as u64)?,self.ancho as u64)?,j as u64)?)
 }
 fn secuencia(&mut self,x:&[E],i:usize,j:usize)->R<bool>{
  let Some((primero,resto))=x.split_first() else {return Ok(i==j)};
  let mut alguno=false;
  // Particiones por extremo de hijo creciente. Todas pagan, incluidas fallidas y duplicadas.
  for k in i..=j {
   self.cuenta.cobrar(1)?;
   if self.probar(*primero,i,k)? && self.secuencia(resto,k,j)? {alguno=true}
  }
  Ok(alguno)
 }
 fn probar(&mut self,e:E,i:usize,j:usize)->R<bool>{
  match e {
   E::L(s)=>{
    let n=s.split(' ').count();if j-i!=n{return Ok(false)}
    let mut igual=true;
    for (k,p) in s.split(' ').enumerate(){
     if self.tokens.get(i+k).ok_or(Error::Rango)?.normal!=p{igual=false}
    }
    Ok(igual)
   },
   E::D=>Ok(j==i+1 && self.tokens.get(i).is_some_and(|t| (1..=10).contains(&t.normal.len())&&t.normal.bytes().all(|b|b.is_ascii_digit()))),
   E::R(g)=>Ok(*self.tabla.get(self.pos(g,i,j)?).ok_or(Error::Rango)?!=0),
   E::S(x)=>self.secuencia(x,i,j),
   E::A(x)=>{
    let mut alguno=false;
    for e in x{self.cuenta.cobrar(1)?;if self.probar(*e,i,j)?{alguno=true}}
    Ok(alguno)
   },
   E::O(e)=>{let presente=self.probar(*e,i,j)?;Ok(i==j||presente)},
   E::M(e)=>{
    if i==j{return Ok(true)}
    let mut alguno=false;
    for k in i+1..=j{
     self.cuenta.cobrar(1)?;
     if self.probar(*e,i,k)? && self.probar(E::M(e),k,j)?{alguno=true}
    }
    Ok(alguno)
   }
  }
 }
 fn cerrar(&mut self)->R<bool>{
  loop{
   self.barridos=suma(self.barridos,1)?;
   let antes=self.estados;
   for longitud in 0..self.ancho{
    for inicio in 0..self.ancho-longitud{
     let fin=inicio+longitud;
     for (regla,e) in G.iter().enumerate(){
      let alternativas=match e {E::A(x)=>*x,_=>std::slice::from_ref(e)};
      for (alt,e) in alternativas.iter().enumerate(){
       self.ultimo=[longitud,inicio,regla+1,alt+1];self.cuenta.cobrar(1)?;
       if self.probar(*e,inicio,fin)?{
        let p=self.pos(regla,inicio,fin)?;
        if *self.tabla.get(p).ok_or(Error::Rango)?==0{
         self.estados=suma(self.estados,1)?;
         *self.tabla.get_mut(p).ok_or(Error::Rango)?=1;
        }
       }
      }
     }
    }
   }
   if self.estados==antes {return Ok(*self.tabla.get(self.pos(0,0,self.ancho-1)?).ok_or(Error::Rango)?!=0)}
  }
 }
}
fn primitivas()->usize{
 let mut n=0;
 macro_rules! comprobar {($p:expr)=>{{assert!($p);n+=1;}}}
 comprobar!(suma(u64::MAX,1)==Err(Error::Overflow));
 comprobar!(suma(u64::MAX-1,1)==Ok(u64::MAX));
 comprobar!(producto(u64::MAX,2)==Err(Error::Overflow));
 comprobar!(producto(u64::MAX,1)==Ok(u64::MAX));
 comprobar!(producto(0,u64::MAX)==Ok(0));
 let original=String::from("aá🦀z");
 comprobar!(vista(&original,1,3)==Ok("á"));
 comprobar!(vista(&original,3,7)==Ok("🦀"));
 comprobar!(vista(&original,2,3)==Err(Error::Utf8));
 comprobar!(vista(&original,3,6)==Err(Error::Utf8));
 comprobar!(vista(&original,7,3)==Err(Error::Rango));
 comprobar!(vista(&original,0,u64::MAX)==Err(Error::Rango));
 comprobar!(vista(&original,8,8)==Ok(""));
 let mut a=Cuenta{usado:999_999,limite:1_000_000};
 comprobar!(a.cobrar(1).is_ok());comprobar!(a.cobrar(1)==Err(Error::Limite));comprobar!(a.usado==1_000_000);
 let mut v=Cuenta{usado:u64::MAX,limite:u64::MAX};
 comprobar!(v.cobrar(1)==Err(Error::Overflow));comprobar!(v.usado==u64::MAX);comprobar!(a.usado==1_000_000);
 comprobar!(if usize::BITS==32{indice(u64::MAX)==Err(Error::Conversion)}else{indice(u64::MAX).is_ok()});
 let mut buffer:Vec<u8>=Vec::new();
 comprobar!(buffer.try_reserve_exact(usize::MAX).is_err());comprobar!(buffer.is_empty());
 n
}
include!("casos.rs");
fn main(){
 println!("{{\"instrumento\":\"IE004-R10-SINTAXIS/1\",\"primitivas_correctas\":{},\"usize_bits\":{}}}",primitivas(),usize::BITS);
 for (id,q,_) in CASOS{
  match lexicalizar(q){
   Err(e)=>println!("{{\"id\":\"{}\",\"bytes\":{},\"estado\":\"{:?}\",\"intentos\":0}}",id,q.len(),e),
   Ok(tokens)=>{
    // Las vistas se obtienen sólo del original; jamás de otro texto con igual longitud.
    for t in &tokens{assert!(vista(q,t.inicio,t.fin).is_ok());}
    match Analisis::nuevo(&tokens){
     Err(e)=>println!("{{\"id\":\"{}\",\"estado\":\"{:?}\",\"intentos\":0}}",id,e),
     Ok(mut a)=>{
      let resultado=a.cerrar();
      let estado=match resultado{Ok(true)=>"DERIVA",Ok(false)=>"NO_DERIVA",Err(Error::Limite)=>"PRESUPUESTO_AGOTADO",Err(_)=>"ERROR_TECNICO"};
      println!("{{\"id\":\"{}\",\"bytes\":{},\"tokens\":{},\"estado\":\"{}\",\"intentos\":{},\"barridos_iniciados\":{},\"estados_sintacticos\":{},\"tabla_bytes\":{},\"ultimo\":{:?}}}",id,q.len(),tokens.len(),estado,a.cuenta.usado,a.barridos,a.estados,a.tabla.capacity(),a.ultimo);
     }
    }
   }
  }
 }
}
