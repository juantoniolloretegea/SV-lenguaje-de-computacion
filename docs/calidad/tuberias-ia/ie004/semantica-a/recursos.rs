// Todas las dimensiones variables pertenecen a una solicitud. Sin allocator propio ni unsafe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Error { Overflow, Conversion, Trabajo, Memoria, Estados, Reserva, Rango, Utf8, Bytes, Tokens, Perfil, Version, Contexto, Constitucion, Profundidad }
type R<T> = Result<T, Error>;
fn suma(a: u64, b: u64) -> R<u64> { a.checked_add(b).ok_or(Error::Overflow) }
fn producto(a: u64, b: u64) -> R<u64> { a.checked_mul(b).ok_or(Error::Overflow) }
fn indice(n: u64) -> R<usize> { usize::try_from(n).map_err(|_| Error::Conversion) }
fn vista(s: &str, a: u64, b: u64) -> R<&str> {
    if a > b || b > s.len() as u64 { return Err(Error::Rango); }
    s.get(indice(a)?..indice(b)?).ok_or(Error::Utf8)
}
const NULO: u32 = u32::MAX;
const PERFIL: &str = "IE004-ES-P2/3-COSTE/1";
const BASE: &str = "K-IE004/1";
const POLITICA: &str = "P-IE004/1";
#[derive(Clone, Copy)]
struct Cupos { trabajo: u64, memoria: u64, estados: u64 }
const CUPOS: Cupos = Cupos { trabajo: 1_000_000, memoria: 32*1024*1024, estados: 16384 };
#[derive(Default,Debug)]
struct Recursos {
    unidades: u64, categorias: [u64; 10], capacidad: u64, pico_solicitado: u64,
    reservas: [u64; 8], crecimientos: [u64; 8], capacidad_anterior_acumulada: [u64; 8],
    capacidad_nueva_acumulada: [u64; 8], estados: u64,
}
impl Recursos {
    fn cobrar(&mut self, c: Cupos, tipo: usize, n: u64) -> R<()> {
        let u = suma(self.unidades, n)?;
        if u > c.trabajo { return Err(Error::Trabajo); }
        self.categorias[tipo] = suma(self.categorias[tipo], n)?;
        self.unidades = u; Ok(())
    }
    fn reservar<T>(&mut self, c: Cupos, clase: usize, v: &mut Vec<T>, n: u64) -> R<()> {
        if n <= v.capacity() as u64 { return Ok(()); }
        let nueva = producto(n, std::mem::size_of::<T>() as u64)?;
        let vieja = producto(v.capacity() as u64, std::mem::size_of::<T>() as u64)?;
        let pico = suma(self.capacidad, nueva)?;
        if pico > c.memoria { return Err(Error::Memoria); }
        self.cobrar(c, 8, 1)?;
        self.pico_solicitado = self.pico_solicitado.max(pico);
        self.reservas[clase] = suma(self.reservas[clase], 1)?;
        if vieja != 0 { self.crecimientos[clase] = suma(self.crecimientos[clase], 1)?; }
        self.capacidad_anterior_acumulada[clase] = suma(self.capacidad_anterior_acumulada[clase], vieja)?;
        self.capacidad_nueva_acumulada[clase] = suma(self.capacidad_nueva_acumulada[clase], nueva)?;
        v.try_reserve_exact(indice(n)?.checked_sub(v.len()).ok_or(Error::Rango)?).map_err(|_|Error::Reserva)?;
        let real = producto(v.capacity() as u64, std::mem::size_of::<T>() as u64)?;
        self.capacidad = suma(self.capacidad.checked_sub(vieja).ok_or(Error::Rango)?, real)?;
        if self.capacidad > c.memoria { return Err(Error::Memoria); }
        Ok(())
    }
    fn poner<T>(&mut self, c: Cupos, clase: usize, v: &mut Vec<T>, x: T, max: u64) -> R<()> {
        let n = suma(v.len() as u64, 1)?;
        if n > max { return Err(Error::Memoria); }
        if n > v.capacity() as u64 { self.reservar(c, clase, v, producto(n, 2)?.min(max))?; }
        self.cobrar(c, 7, 1)?;
        // La comprobación de capacidad precede al push: éste no decide crecimiento.
        if v.len() >= v.capacity() { return Err(Error::Reserva); }
        v.push(x); Ok(())
    }
}
#[derive(Clone, Copy, Debug)]
struct Token { inicio: u64, fin: u64, convertido: bool }
fn separador(c: char) -> bool { matches!(c, ' '|'\t'|'\n'|'\r') }
fn signo(c: char) -> bool { matches!(c, '¿'|'?'|'.'|','|':'|';') }
fn minuscula(c: char) -> char {
    match c { 'A'..='Z' => c.to_ascii_lowercase(), 'Á'=>'á','É'=>'é','Í'=>'í','Ó'=>'ó','Ú'=>'ú','Ü'=>'ü','Ñ'=>'ñ', _=>c }
}
struct Lexico { normal: Vec<u8>, tokens: Vec<Token> }
impl Lexico {
    fn nueva(q: &str, r: &mut Recursos, c: Cupos) -> R<Self> {
        if q.len() > 8192 { return Err(Error::Bytes); }
        let mut normal = Vec::new(); r.reservar(c,0,&mut normal,q.len() as u64)?;
        for x in q.chars() {
            let y = minuscula(x);
            if x.len_utf8() != y.len_utf8() { return Err(Error::Constitucion); }
            let mut b = [0u8;4];
            for byte in y.encode_utf8(&mut b).bytes() {
                r.cobrar(c,0,1)?;
                if normal.len() >= q.len() || normal.len() >= normal.capacity() { return Err(Error::Bytes); }
                normal.push(byte);
            }
        }
        let mut tokens = Vec::new(); r.reservar(c,1,&mut tokens,128)?;
        let mut i = 0;
        while i < q.len() {
            let ch=q.get(i..).ok_or(Error::Rango)?.chars().next().ok_or(Error::Rango)?;
            r.cobrar(c,0,ch.len_utf8() as u64)?;
            if separador(ch) { i=indice(suma(i as u64,ch.len_utf8() as u64)?)?; continue; }
            let inicio=i;
            if q.get(i..).ok_or(Error::Rango)?.starts_with("...") { i=indice(suma(i as u64,3)?)?; }
            else if signo(ch) { i=indice(suma(i as u64,ch.len_utf8() as u64)?)?; }
            else {
                while i<q.len() {
                    let x=q.get(i..).ok_or(Error::Rango)?.chars().next().ok_or(Error::Rango)?;
                    r.cobrar(c,0,x.len_utf8() as u64)?;
                    if separador(x)||signo(x) { break; }
                    i=indice(suma(i as u64,x.len_utf8() as u64)?)?;
                }
            }
            if tokens.len()==128 { return Err(Error::Tokens); }
            r.cobrar(c,0,(i-inicio) as u64)?;
            let convertido=q.as_bytes()[inicio..i]!=normal[inicio..i];
            r.poner(c,1,&mut tokens,Token{inicio:inicio as u64,fin:i as u64,convertido},128)?;
        }
        Ok(Self{normal,tokens})
    }
    fn token(&self,i:usize)->R<&str> {
        let t=self.tokens.get(i).ok_or(Error::Rango)?;
        std::str::from_utf8(self.normal.get(indice(t.inicio)?..indice(t.fin)?).ok_or(Error::Rango)?).map_err(|_|Error::Utf8)
    }
}
