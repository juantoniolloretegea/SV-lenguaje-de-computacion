// Decodificador acotado del transporte. No conoce gramática, permisos ni datos SV.
use std::io::{Read, Write};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FalloT { Bytes, Utf8, Json, Duplicada, Esquema, Entero, Profundidad, Memoria, Trabajo, Reserva, Io, Truncado, Correlacion, Certificado }
type RT<T> = Result<T,FalloT>;
#[derive(Debug)]
struct CuentaT { trabajo:u64, max_trabajo:u64, reservado:u64, max_memoria:u64, recibidos:u64, instantanea_bytes:u64, nodos_json:u64 }
impl CuentaT {
    fn nueva(v:bool)->Self{Self{trabajo:0,max_trabajo:if v{1_000_000}else{2_000_000},reservado:0,max_memoria:if v{4*1024*1024}else{1024*1024},recibidos:0,instantanea_bytes:0,nodos_json:0}}
    fn cobrar(&mut self,n:u64)->RT<()>{let x=self.trabajo.checked_add(n).ok_or(FalloT::Entero)?;if x>self.max_trabajo{return Err(FalloT::Trabajo)}self.trabajo=x;Ok(())}
    // Contabilidad conservadora: las asignaciones liberadas no restituyen saldo.
    fn reservar<T>(&mut self,v:&mut Vec<T>,n:usize)->RT<()>{
        if n<=v.capacity(){return Ok(())}
        let bytes=(n as u64).checked_mul(std::mem::size_of::<T>() as u64).ok_or(FalloT::Entero)?;
        let peak=self.reservado.checked_add(bytes).ok_or(FalloT::Entero)?;
        if peak>self.max_memoria{return Err(FalloT::Memoria)}
        self.cobrar(1)?;v.try_reserve_exact(n.checked_sub(v.len()).ok_or(FalloT::Entero)?).map_err(|_|FalloT::Reserva)?;
        let real=(v.capacity() as u64).checked_mul(std::mem::size_of::<T>() as u64).ok_or(FalloT::Entero)?;
        self.reservado=self.reservado.checked_add(real).ok_or(FalloT::Entero)?;
        if self.reservado>self.max_memoria{return Err(FalloT::Memoria)}Ok(())
    }
    fn poner<T>(&mut self,v:&mut Vec<T>,x:T,max:usize)->RT<()>{
        let n=v.len().checked_add(1).ok_or(FalloT::Entero)?;if n>max{return Err(FalloT::Memoria)}
        if n>v.capacity(){self.reservar(v,n.checked_mul(2).ok_or(FalloT::Entero)?.min(max))?;}
        self.cobrar(1)?;if v.len()>=v.capacity(){return Err(FalloT::Reserva)}v.push(x);Ok(())
    }
    fn igual(&mut self,a:&str,b:&str)->RT<bool>{self.cobrar(a.len().max(b.len()) as u64+1)?;Ok(a==b)}
}
fn recibir<Rd:Read>(input:&mut Rd,max:usize,c:&mut CuentaT)->RT<Vec<u8>>{
    let mut b=Vec::new();c.reservar(&mut b,max)?;let mut buf=[0u8;4096];
    loop{let pendiente=max.checked_sub(b.len()).ok_or(FalloT::Entero)?;let limite=buf.len().min(pendiente.checked_add(1).ok_or(FalloT::Entero)?);
        let n=input.read(&mut buf[..limite]).map_err(|_|FalloT::Io)?;
        c.recibidos=c.recibidos.checked_add(n as u64).ok_or(FalloT::Entero)?;c.cobrar(n as u64)?;
        if n==0{return Ok(b)}if n>pendiente{return Err(FalloT::Bytes)}
        if b.len().checked_add(n).ok_or(FalloT::Entero)?>b.capacity(){return Err(FalloT::Reserva)}b.extend_from_slice(&buf[..n]);
    }
}
#[derive(Debug)]
enum J { N, B(bool), U(u64), S(String), A(Vec<J>), O(Vec<(String,J)>) }
struct Decoder<'a,'b>{b:&'a[u8],i:usize,c:&'b mut CuentaT}
impl<'a,'b> Decoder<'a,'b>{
    fn ver(&mut self)->RT<Option<u8>>{if self.i>=self.b.len(){return Ok(None)}self.c.cobrar(1)?;Ok(Some(self.b[self.i]))}
    fn tomar(&mut self)->RT<u8>{let x=self.ver()?.ok_or(FalloT::Truncado)?;self.i=self.i.checked_add(1).ok_or(FalloT::Entero)?;Ok(x)}
    fn blanco(&mut self)->RT<()>{while matches!(self.ver()?,Some(b' '|b'\t'|b'\r'|b'\n')){self.i+=1;}Ok(())}
    fn exigir(&mut self,b:u8)->RT<()>{if self.tomar()?!=b{return Err(FalloT::Json)}Ok(())}
    fn hex4(&mut self)->RT<u32>{let mut x=0;for _ in 0..4{let b=self.tomar()?;let d=match b{b'0'..=b'9'=>(b-b'0') as u32,b'a'..=b'f'=>(b-b'a'+10) as u32,b'A'..=b'F'=>(b-b'A'+10) as u32,_=>return Err(FalloT::Json)};x=x*16+d;}Ok(x)}
    fn cadena(&mut self)->RT<String>{
        self.exigir(b'"')?;let mut v=Vec::new();
        loop{let b=self.tomar()?;match b{
            b'"'=>break,0..=31=>return Err(FalloT::Json),b'\\'=>{
                let e=self.tomar()?;let decoded=match e{b'"'=>Some(b'"'),b'\\'=>Some(b'\\'),b'/'=>Some(b'/'),b'b'=>Some(8),b'f'=>Some(12),b'n'=>Some(b'\n'),b'r'=>Some(b'\r'),b't'=>Some(b'\t'),b'u'=>None,_=>return Err(FalloT::Json)};
                if let Some(x)=decoded{self.c.poner(&mut v,x,262144)?;}else{
                    let hi=self.hex4()?;let cp=if (0xd800..=0xdbff).contains(&hi){self.exigir(b'\\')?;self.exigir(b'u')?;let lo=self.hex4()?;if !(0xdc00..=0xdfff).contains(&lo){return Err(FalloT::Json)}0x10000+((hi-0xd800)<<10)+(lo-0xdc00)}else{hi};
                    let ch=char::from_u32(cp).ok_or(FalloT::Json)?;let mut bytes=[0u8;4];for x in ch.encode_utf8(&mut bytes).bytes(){self.c.poner(&mut v,x,262144)?;}
                }
            },x=>self.c.poner(&mut v,x,262144)?,}}
        String::from_utf8(v).map_err(|_|FalloT::Utf8)
    }
    fn valor(&mut self,depth:usize,tope:usize)->RT<J>{
        if depth>96{return Err(FalloT::Profundidad)}self.c.nodos_json=self.c.nodos_json.checked_add(1).ok_or(FalloT::Entero)?;if self.c.nodos_json>8192{return Err(FalloT::Memoria)}
        self.blanco()?;match self.ver()?.ok_or(FalloT::Truncado)?{
            b'"'=>Ok(J::S(self.cadena()?)),b'n'=>{for b in b"null"{self.exigir(*b)?;}Ok(J::N)},b't'=>{for b in b"true"{self.exigir(*b)?;}Ok(J::B(true))},b'f'=>{for b in b"false"{self.exigir(*b)?;}Ok(J::B(false))},
            b'0'..=b'9'=>{let first=self.tomar()?;let mut n=(first-b'0')as u64;if first==b'0'&&matches!(self.ver()?,Some(b'0'..=b'9')){return Err(FalloT::Json)}while let Some(b'0'..=b'9')=self.ver()?{let d=self.tomar()?-b'0';n=n.checked_mul(10).and_then(|n|n.checked_add(d as u64)).ok_or(FalloT::Entero)?;}Ok(J::U(n))},
            b'['=>{self.tomar()?;self.blanco()?;let mut a=Vec::new();if self.ver()?==Some(b']'){self.tomar()?;return Ok(J::A(a))}loop{if a.len()>=tope{return Err(FalloT::Memoria)}let x=self.valor(depth+1,8192)?;self.c.poner(&mut a,x,tope)?;self.blanco()?;match self.tomar()?{b']'=>break,b','=>(),_=>return Err(FalloT::Json)}}Ok(J::A(a))},
            b'{'=>{self.tomar()?;self.blanco()?;let mut o:Vec<(String,J)>=Vec::new();if self.ver()?==Some(b'}'){self.tomar()?;return Ok(J::O(o))}loop{self.blanco()?;let k=self.cadena()?;for(old,_)in &o{if self.c.igual(old,&k)?{return Err(FalloT::Duplicada)}}self.blanco()?;self.exigir(b':')?;self.c.cobrar(k.len()as u64*2)?;let limite=if k=="nodos"{512}else if k=="hijos"{2}else{8192};let x=self.valor(depth+1,limite)?;self.c.poner(&mut o,(k,x),32)?;self.blanco()?;match self.tomar()?{b'}'=>break,b','=>(),_=>return Err(FalloT::Json)}}Ok(J::O(o))},
            _=>Err(FalloT::Json)
        }
    }
}
fn decodificar(b:&[u8],c:&mut CuentaT)->RT<J>{c.cobrar(b.len()as u64)?;std::str::from_utf8(b).map_err(|_|FalloT::Utf8)?;let mut d=Decoder{b,i:0,c};let j=d.valor(0,8192)?;d.blanco()?;if d.i!=b.len(){return Err(FalloT::Json)}Ok(j)}
fn objeto<'a>(j:&'a J,keys:&[&str],c:&mut CuentaT)->RT<&'a[(String,J)]>{let J::O(o)=j else{return Err(FalloT::Esquema)};c.cobrar(1)?;if o.len()!=keys.len(){return Err(FalloT::Esquema)}for(k,_)in o{let mut known=false;for key in keys{known|=c.igual(k,key)?;}if !known{return Err(FalloT::Esquema)}}Ok(o)}
fn campo<'a>(o:&'a[(String,J)],k:&str,c:&mut CuentaT)->RT<&'a J>{for(key,v)in o{if c.igual(key,k)?{return Ok(v)}}Err(FalloT::Esquema)}
fn texto(j:&J)->RT<&str>{if let J::S(s)=j{Ok(s)}else{Err(FalloT::Esquema)}}
fn numero(j:&J)->RT<u64>{if let J::U(n)=j{Ok(*n)}else{Err(FalloT::Esquema)}}
fn lista(j:&J)->RT<&[J]>{if let J::A(a)=j{Ok(a)}else{Err(FalloT::Esquema)}}

// Salida acotada; se construye entera antes de escribir el primer byte de un marco.
struct SalidaBytes{b:Vec<u8>,max:usize}
impl SalidaBytes{
    fn nueva(max:usize)->RT<Self>{let mut b=Vec::new();b.try_reserve_exact(max).map_err(|_|FalloT::Reserva)?;Ok(Self{b,max})}
    fn raw(&mut self,s:&str)->RT<()>{if self.b.len().checked_add(s.len()).ok_or(FalloT::Entero)?>self.max{return Err(FalloT::Bytes)}self.b.extend_from_slice(s.as_bytes());Ok(())}
    fn str(&mut self,s:&str)->RT<()>{self.raw("\"")?;for ch in s.chars(){match ch{'"'=>self.raw("\\\"")?,'\\'=>self.raw("\\\\")?,'\n'=>self.raw("\\n")?,'\r'=>self.raw("\\r")?,'\t'=>self.raw("\\t")?,c if (c as u32)<32=>{let hex=b"0123456789abcdef";let v=c as usize;let a=[b'\\',b'u',b'0',b'0',hex[v/16],hex[v%16]];self.raw(std::str::from_utf8(&a).map_err(|_|FalloT::Utf8)?)?},c=>{let mut b=[0;4];self.raw(c.encode_utf8(&mut b))?}}}self.raw("\"")}
    fn uint(&mut self,mut n:u64)->RT<()>{let mut b=[0u8;20];let mut i=20;loop{i-=1;b[i]=b'0'+(n%10)as u8;n/=10;if n==0{break}}self.raw(std::str::from_utf8(&b[i..]).map_err(|_|FalloT::Utf8)?)}
    fn boolean(&mut self,v:bool)->RT<()>{self.raw(if v{"true"}else{"false"})}
}
