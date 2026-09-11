const MAX_LOTE:usize=2*1024*1024;
const MAX_SALIDA:usize=2*1024*1024;
const CLAVES:[&str;5]=["operacion","objeto","parametro","momento","campo"];
const VALORES:[&[&str];5]=[&["LEER","ESCRIBIR"],&["CASO-A","CASO-B"],&["IGG","IGA","IGM"],&["ACTUAL","ANTERIOR"],&["VALOR","UNIDAD","ESTADO","FUENTE","ALCANCE"]];
struct Caso{ id:String,pregunta:String,contexto:[Option<String>;5],inicio:usize,fin:usize,tokens:usize }
struct Lote{casos:Vec<Caso>}
fn cuenta_lote()->CuentaT{let mut c=CuentaT::nueva(false);c.max_trabajo=24_000_000;c.max_memoria=16*1024*1024;c}
fn salida(max:usize,c:&mut CuentaT)->RT<SalidaBytes>{let mut b=Vec::new();c.reservar(&mut b,max)?;Ok(SalidaBytes{b,max})}
fn agregar(v:&mut Vec<u8>,bytes:&[u8],max:usize,c:&mut CuentaT)->RT<()>{
    if v.len().checked_add(bytes.len()).ok_or(FalloT::Entero)?>max{return Err(FalloT::Bytes)}
    for b in bytes{c.poner(v,*b,max)?;}Ok(())
}
impl<'a,'b> Decoder<'a,'b>{
    // Límite específico antes de cada inserción; UTF-8 y escapes sin normalización.
    fn cadena_lote(&mut self,max:usize)->RT<String>{
        self.exigir(b'"')?;let mut v=Vec::new();
        loop{match self.tomar()?{
            b'"'=>break,0..=31=>return Err(FalloT::Json),b'\\'=>{
                let e=self.tomar()?;
                let simple=match e{b'"'=>Some(b'"'),b'\\'=>Some(b'\\'),b'/'=>Some(b'/'),b'b'=>Some(8),b'f'=>Some(12),b'n'=>Some(b'\n'),b'r'=>Some(b'\r'),b't'=>Some(b'\t'),b'u'=>None,_=>return Err(FalloT::Json)};
                if let Some(b)=simple{agregar(&mut v,&[b],max,self.c)?;}else{
                    let hi=self.hex4()?;let cp=if (0xd800..=0xdbff).contains(&hi){self.exigir(b'\\')?;self.exigir(b'u')?;let lo=self.hex4()?;if !(0xdc00..=0xdfff).contains(&lo){return Err(FalloT::Json)}0x10000+((hi-0xd800)<<10)+(lo-0xdc00)}else{hi};
                    let ch=char::from_u32(cp).ok_or(FalloT::Json)?;let mut b=[0;4];agregar(&mut v,ch.encode_utf8(&mut b).as_bytes(),max,self.c)?;
                }
            },b=>agregar(&mut v,&[b],max,self.c)?,
        }}String::from_utf8(v).map_err(|_|FalloT::Utf8)
    }
    fn clave_lote(&mut self,permitidas:&[&str],vistas:&mut u8)->RT<usize>{
        self.blanco()?;let k=self.cadena_lote(64)?;let mut ix=None;
        for(i,s)in permitidas.iter().enumerate(){if self.c.igual(&k,s)?{ix=Some(i);}}
        let i=ix.ok_or(FalloT::Esquema)?;let bit=1u8.checked_shl(i as u32).ok_or(FalloT::Entero)?;
        if *vistas&bit!=0{return Err(FalloT::Duplicada)}*vistas|=bit;
        self.blanco()?;self.exigir(b':')?;self.blanco()?;Ok(i)
    }
    fn continuar(&mut self,cierre:u8)->RT<bool>{self.blanco()?;match self.tomar()?{b if b==cierre=>Ok(false),b','=>Ok(true),_=>Err(FalloT::Json)}}
    fn contexto_lote(&mut self)->RT<[Option<String>;5]>{
        self.exigir(b'{')?;let mut vistos=0;let mut campos:[Option<String>;5]=std::array::from_fn(|_|None);
        loop{let i=self.clave_lote(&CLAVES,&mut vistos)?;
            if self.ver()?==Some(b'n'){for b in b"null"{self.exigir(*b)?;}}else{
                let s=self.cadena_lote(32)?;let mut valido=false;for v in VALORES[i]{valido|=self.c.igual(&s,v)?;}if !valido{return Err(FalloT::Esquema)}campos[i]=Some(s);
            }
            if !self.continuar(b'}')?{break}
        }if vistos!=31{return Err(FalloT::Esquema)}Ok(campos)
    }
    fn caso_lote(&mut self,numero:usize)->RT<Caso>{
        self.blanco()?;let inicio=self.i;self.exigir(b'{')?;let mut vistos=0;let(mut id,mut pregunta,mut contexto)=(None,None,None);
        loop{match self.clave_lote(&["id","pregunta","contexto"],&mut vistos)?{
            0=>{let s=self.cadena_lote(5)?;let n=numero.checked_add(1).ok_or(FalloT::Entero)?;let esperado=[b'P',b'3',b'-',b'0'+(n/10)as u8,b'0'+(n%10)as u8];self.c.cobrar(5)?;if s.as_bytes()!=esperado{return Err(FalloT::Correlacion)}id=Some(s);},
            1=>pregunta=Some(self.cadena_lote(8192)?),2=>contexto=Some(self.contexto_lote()?),_=>return Err(FalloT::Esquema),
        }if !self.continuar(b'}')?{break}}
        if vistos!=7{return Err(FalloT::Esquema)}let q=pregunta.ok_or(FalloT::Esquema)?;
        // Se reutiliza el léxico fijado: este paso sólo controla la cota, no deriva ni decide.
        // Cargo conservador previo por buffers y trabajo; no se transfiere al motor A.
        let reserva=(q.len() as u64).checked_add(128*(std::mem::size_of::<Token>() as u64)).ok_or(FalloT::Entero)?;
        let nuevo=self.c.reservado.checked_add(reserva).ok_or(FalloT::Entero)?;if nuevo>self.c.max_memoria{return Err(FalloT::Memoria)}self.c.reservado=nuevo;
        self.c.cobrar((q.len() as u64).checked_mul(8).and_then(|n|n.checked_add(1024)).ok_or(FalloT::Entero)?)?;
        let mut r=Recursos::default();let lex=Lexico::nueva(&q,&mut r,CUPOS).map_err(|e|match e{Error::Tokens=>FalloT::Profundidad,Error::Bytes=>FalloT::Bytes,Error::Reserva=>FalloT::Reserva,Error::Memoria=>FalloT::Memoria,_=>FalloT::Esquema})?;
        if r.capacidad>reserva{return Err(FalloT::Memoria)}
        Ok(Caso{id:id.ok_or(FalloT::Esquema)?,pregunta:q,contexto:contexto.ok_or(FalloT::Esquema)?,inicio,fin:self.i,tokens:lex.tokens.len()})
    }
    fn lote(&mut self)->RT<Lote>{
        self.blanco()?;self.exigir(b'{')?;let mut vistos=0;let mut casos=Vec::new();
        loop{match self.clave_lote(&["version","casos"],&mut vistos)?{
            0=>{let v=self.cadena_lote(32)?;if !self.c.igual(&v,"IE004-P3-A/1")?{return Err(FalloT::Esquema)}},
            1=>{self.exigir(b'[')?;self.c.reservar(&mut casos,24)?;self.blanco()?;if self.ver()?==Some(b']'){return Err(FalloT::Esquema)}
                loop{if casos.len()==24{return Err(FalloT::Esquema)}let caso=self.caso_lote(casos.len())?;self.c.poner(&mut casos,caso,24)?;if !self.continuar(b']')?{break}}
                if casos.len()!=24{return Err(FalloT::Esquema)}
            },_=>return Err(FalloT::Esquema),
        }if !self.continuar(b'}')?{break}}
        self.blanco()?;if vistos!=3||self.i!=self.b.len(){return Err(FalloT::Esquema)}Ok(Lote{casos})
    }
}
fn solicitud_canonica(s:&Caso,c:&mut CuentaT)->RT<SalidaBytes>{
    let mut w=salida(65536,c)?;w.raw("{\"version\":\"IE004-A-SOLICITUD/1\",\"id\":")?;w.str(&s.id)?;w.raw(",\"pregunta\":")?;w.str(&s.pregunta)?;w.raw(",\"contexto\":{")?;
    for i in 0..5{if i>0{w.raw(",")?}w.str(CLAVES[i])?;w.raw(":")?;if let Some(v)=&s.contexto[i]{w.str(v)?}else{w.raw("null")?};}w.raw("}}")?;c.cobrar(w.b.len() as u64)?;Ok(w)
}
fn adaptar(b:&[u8],huella:&str,c:&mut CuentaT)->RT<(Vec<u8>,SalidaBytes)>{
    // Integridad de todos los bytes primero. La huella esperada procede del conductor/custodio.
    c.cobrar(b.len()as u64)?;if !c.igual(sha_str(&sha_texto(b)?)?,huella)?{return Err(FalloT::Correlacion)}
    c.cobrar(b.len()as u64)?;std::str::from_utf8(b).map_err(|_|FalloT::Utf8)?;
    let lote=Decoder{b,i:0,c}.lote()?;
    let mut out=salida(MAX_SALIDA,c)?;let mut trace=salida(65536,c)?;c.cobrar(65536)?; // Cota previa del trabajo de serialización de la traza.
    out.raw("{\"version\":\"IE004-LOTE-EXTRAIDO/1\",\"solicitudes_sha256\":")?;out.str(huella)?;out.raw(",\"solicitudes\":[")?;
    trace.raw("{\"version\":\"IE004-LOTE-TRAZA/1\",\"solicitudes_sha256\":")?;trace.str(huella)?;trace.raw(",\"casos\":[")?;
    for(i,s)in lote.casos.iter().enumerate(){if i>0{out.raw(",")?;trace.raw(",")?}let req=solicitud_canonica(s,c)?;out.raw(std::str::from_utf8(&req.b).map_err(|_|FalloT::Utf8)?)?;
        trace.raw("{\"id\":")?;trace.str(&s.id)?;trace.raw(",\"inicio\":")?;trace.uint(s.inicio as u64)?;trace.raw(",\"fin\":")?;trace.uint(s.fin as u64)?;trace.raw(",\"tokens\":")?;trace.uint(s.tokens as u64)?;
        for(k,bytes)in[("original_caso_sha256",&b[s.inicio..s.fin]),("pregunta_sha256",s.pregunta.as_bytes()),("solicitud_sha256",&req.b)]{c.cobrar(bytes.len()as u64)?;trace.raw(",\"")?;trace.raw(k)?;trace.raw("\":")?;trace.str(sha_str(&sha_texto(bytes)?)?)?;}trace.raw("}")?;
    }
    out.raw("]}\n")?;c.cobrar(out.b.len()as u64*2)?;
    let full=out.b.len().checked_add(48).ok_or(FalloT::Entero)?;let mut frame=Vec::new();c.reservar(&mut frame,full)?;
    frame.extend_from_slice(b"SVLT0001");frame.extend_from_slice(&(out.b.len()as u64).to_le_bytes());frame.extend_from_slice(&out.b);frame.extend_from_slice(&sha256(&out.b)?);
    trace.raw("],\"bytes_lote\":")?;trace.uint(b.len()as u64)?;trace.raw(",\"trabajo_lote\":")?;trace.uint(c.trabajo)?;trace.raw(",\"reserva_acumulada_bytes\":")?;trace.uint(c.reservado)?;trace.raw(",\"a_iniciada\":false,\"v_abierta\":false,\"auxiliar_abierto\":false}\n")?;Ok((frame,trace))
}
fn programa()->RT<()>{
    // No paths de entrada/salida, URLs, notas ni propuestas. El entorno es responsabilidad del conductor.
    let mut args=std::env::args();let _nombre=args.next();let h=args.next().ok_or(FalloT::Esquema)?;if args.next().is_some()||!es_huella(&h){return Err(FalloT::Esquema)}
    let mut c=cuenta_lote();let r=(||{let b=recibir(&mut std::io::stdin().lock(),MAX_LOTE,&mut c)?;
        let (frame,traza)=adaptar(&b,&h,&mut c)?;
        std::io::stderr().lock().write_all(&traza.b).map_err(|_|FalloT::Io)?;
        std::io::stdout().lock().write_all(&frame).map_err(|_|FalloT::Io)
    })();
    if r.is_err(){eprintln!("{{\"version\":\"IE004-LOTE-RECHAZO/1\",\"recibidos\":{},\"trabajo_lote\":{},\"reserva_acumulada_bytes\":{},\"a_iniciada\":false,\"v_abierta\":false,\"auxiliar_abierto\":false}}",c.recibidos,c.trabajo,c.reservado);}
    r
}
