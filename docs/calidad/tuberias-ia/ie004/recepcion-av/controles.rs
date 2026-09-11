include!("casos_fijados.rs");
fn obtener<'a>(j:&'a J,key:&str)->RT<&'a J>{let J::O(o)=j else{return Err(FalloT::Esquema)};o.iter().find(|(k,_)|k==key).map(|(_,v)|v).ok_or(FalloT::Esquema)}
fn texto_op(j:&J)->RT<Option<&str>>{match j{J::N=>Ok(None),J::S(s)=>Ok(Some(s)),_=>Err(FalloT::Esquema)}}
fn control_a(caso:&CasoA)->RT<(bool,ProductoA,u64)>{
    let mut c=CuentaT::nueva(false);let mut lectura=caso.bytes;let p=match recibir(&mut lectura,65536,&mut c){Ok(b)=>resolver_a(&b,&mut c,caso.vigente)?,Err(e)=>rechazo_recepcion(e,&c,caso.vigente)?};
    let canonical=validar_marco(&p.marco)?;let j=decodificar(canonical,&mut CuentaT::nueva(false))?;let o=obtener(&j,"resolucion")?;
    let mut ok=texto(obtener(o,"estado")?)?==caso.estado&&texto_op(obtener(o,"contenido")?)?==caso.literal&&numero(obtener(o,"llamadas_politica")?)?==caso.llamadas;
    if let Some(ruta)=caso.ruta{let r=lista(obtener(o,"ruta")?)?;ok&=r.len()==5;for(i,x)in r.iter().enumerate(){ok&=numero(x)?==ruta[i]as u64;}}
    if let Some(causa)=caso.causa{let tr=decodificar(&p.traza.b,&mut CuentaT::nueva(false))?;ok&=texto(obtener(&tr,"error_transporte")?)?==causa;}
    if let Some(n)=caso.observados{ok&=c.recibidos==n;}
    Ok((ok,p,c.recibidos))
}
fn primitivas_transporte()->RT<(usize,usize)>{
    let mut n=0usize;let mut correctas=0usize;macro_rules! comprobar{($e:expr)=>{{n+=1;if $e{correctas+=1;}}}}
    comprobar!(sha_str(&sha_texto(b"")?)?=="e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    comprobar!(sha_str(&sha_texto(b"abc")?)?=="ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    let mut cuenta=CuentaT::nueva(true);comprobar!(cuenta.cobrar(1_000_000).is_ok());comprobar!(cuenta.cobrar(1)==Err(FalloT::Trabajo)&&cuenta.trabajo==1_000_000);
    comprobar!(altura_v(63)==Ok(64));comprobar!(altura_v(64)==Err(FalloT::Profundidad));comprobar!(enlaces_v(1023,1)==Ok(1024));comprobar!(enlaces_v(1024,1)==Err(FalloT::Certificado));
    let mut v:Vec<u8>=Vec::new();let mut c=CuentaT::nueva(true);c.max_memoria=16;comprobar!(c.reservar(&mut v,16).is_ok());comprobar!(c.reservar(&mut v,17)==Err(FalloT::Memoria)&&v.capacity()==16);
    let mut c=CuentaT::nueva(false);comprobar!(recibir(&mut [0u8;17].as_slice(),16,&mut c)==Err(FalloT::Bytes)&&c.recibidos==17);
    let j=decodificar(br#""\ud83e\udd80""#,&mut CuentaT::nueva(false))?;comprobar!(texto(&j)?=="🦀");
    comprobar!(matches!(decodificar(br#"{"id":1,"i\u0064":2}"#,&mut CuentaT::nueva(false)),Err(FalloT::Duplicada)));
    comprobar!(matches!(decodificar(b"18446744073709551615",&mut CuentaT::nueva(false)),Ok(J::U(u64::MAX))));
    comprobar!(matches!(decodificar(b"18446744073709551616",&mut CuentaT::nueva(false)),Err(FalloT::Entero)));
    let frame=marco_a(b"{\"completo\":true}\n")?;comprobar!(validar_marco(&frame)==Ok(b"{\"completo\":true}\n".as_slice()));
    comprobar!(validar_marco(&frame[..frame.len()-1])==Err(FalloT::Truncado));
    let mut bad=frame.clone();let last=bad.len()-1;bad[last]^=1;comprobar!(validar_marco(&bad)==Err(FalloT::Correlacion));
    let mut huge=frame.clone();huge[8..16].copy_from_slice(&u64::MAX.to_le_bytes());comprobar!(validar_marco(&huge)==Err(FalloT::Bytes));
    let mut extra=frame.clone();extra.push(0);comprobar!(validar_marco(&extra)==Err(FalloT::Esquema));
    comprobar!(id_permitido("P3-01")&&id_permitido("P3-24")&&!id_permitido("P3-00")&&!id_permitido("P3-25"));
    struct Roto;impl Read for Roto{fn read(&mut self,_:&mut[u8])->std::io::Result<usize>{Err(std::io::Error::other("fallo inyectado del transporte"))}}
    comprobar!(recibir(&mut Roto,16,&mut CuentaT::nueva(false))==Err(FalloT::Io));
    Ok((correctas,n))
}
fn controles()->RT<()>{
    let mut fallos=0;let mut total=0;
    for repeticion in 1..=3{
        for a in CASOS_A{let(ok,p,n)=control_a(a)?;total+=1;if !ok{fallos+=1;}print!("{{\"grupo\":\"A\",\"id\":\"{}\",\"repeticion\":{},\"conforme\":{},\"recibidos\":{},\"cuerpo_sha256\":\"{}\",\"traza_sha256\":\"{}\",\"cuerpo\":",a.id,repeticion,ok,n,sha_str(&sha_texto(&p.cuerpo.b)?)?,sha_str(&sha_texto(&p.traza.b)?)?);std::io::stdout().write_all(&p.cuerpo.b[..p.cuerpo.b.len()-1]).map_err(|_|FalloT::Io)?;println!("}}");}
        for v in CASOS_V{let(_,p,_)=control_a(&CASOS_A[v.original])?;let before=sha256(&p.cuerpo.b)?;let trace_before=sha256(&p.traza.b)?;let bodysha=hex_sha(&before);
            let mut original=CASOS_A[v.original].bytes;let mut propuesta=v.bytes;let annex=ejecutar_v(&mut original,&mut propuesta,LOTE_SHA,sha_str(&bodysha)?,v.cupo)?;
            let j=decodificar(&annex.b,&mut CuentaT::nueva(false))?;let mut ok=texto(obtener(&j,"estado")?)?==v.estado&&texto_op(obtener(&j,"causa")?)?==v.causa&&sha256(&p.cuerpo.b)?==before&&sha256(&p.traza.b)?==trace_before;
            if let Some(n)=v.observados{ok&=numero(obtener(&j,"bytes_v_observados")?)?==n;}total+=1;if !ok{fallos+=1;}
            print!("{{\"grupo\":\"V\",\"id\":\"{}\",\"repeticion\":{},\"conforme\":{},\"cuerpo_a_conservado\":true,\"traza_a_conservada\":true,\"anexo\":",v.id,repeticion,ok);std::io::stdout().write_all(&annex.b[..annex.b.len()-1]).map_err(|_|FalloT::Io)?;println!("}}");
        }
        for(a,b)in[(0,1),(0,16),(0,18)]{let(_,pa,_)=control_a(&CASOS_A[a])?;let(_,pb,_)=control_a(&CASOS_A[b])?;let ok=pa.cuerpo.b==pb.cuerpo.b&&pa.traza.b!=pb.traza.b;total+=1;if !ok{fallos+=1;}println!("{{\"grupo\":\"RELACION\",\"a\":\"{}\",\"b\":\"{}\",\"repeticion\":{},\"conforme\":{}}}",CASOS_A[a].id,CASOS_A[b].id,repeticion,ok);}
    }
    let (ok,n)=primitivas_transporte()?;fallos+=n-ok;
    println!("{{\"resumen\":true,\"controles_por_repeticion\":{},\"repeticiones\":3,\"controles_ejecutados\":{},\"primitivas_correctas\":{},\"primitivas_total\":{},\"fallos\":{}}}",CASOS_A.len()+CASOS_V.len()+3,total,ok,n,fallos);
    if fallos>0{std::process::exit(1)}Ok(())
}
