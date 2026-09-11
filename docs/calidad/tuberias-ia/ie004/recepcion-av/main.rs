#![forbid(unsafe_code)]
#![allow(dead_code)] // Se incluyen completas las fuentes fijadas, incluida su exportación histórica.
include!("../semantica-a/gramatica.rs");
include!("../semantica-a/recursos.rs");
include!("../semantica-a/sintaxis.rs");
include!("../semantica-a/semantica.rs");
include!("../semantica-a/servicio.rs");
include!("json_estricto.rs");
include!("sha256.rs");
include!("recepcion_a.rs");
include!("verificar_v.rs");
#[cfg(controles_publicos)]
include!("controles.rs");
#[cfg(not(controles_publicos))]
fn controles()->RT<()>{Err(FalloT::Esquema)}
fn publicar_a(p:ProductoA)->RT<()>{
    // El registro base completo se escribe antes del marco. El conductor valida
    // longitud, huella y fin de proceso antes de publicar el cuerpo al consumidor.
    std::io::stderr().lock().write_all(&p.traza.b).map_err(|_|FalloT::Io)?;
    std::io::stdout().lock().write_all(&p.marco).map_err(|_|FalloT::Io)
}
fn rechazo_recepcion(e:FalloT,c:&CuentaT,vigente:bool)->RT<ProductoA>{
    let body=cuerpo_a(diagnostico("A_TRANSPORTE_RECHAZADO",0),None,vigente,false)?;
    let mut tr=SalidaBytes::nueva(65536)?;tr.raw("{\"version\":\"IE004-A-TRAZA/1\",\"iniciado_a\":false,\"error_transporte\":")?;tr.str(fallo_t_nombre(e))?;tr.raw(",\"recibidos\":")?;tr.uint(c.recibidos)?;tr.raw("}\n")?;
    let marco=marco_a(&body.b)?;Ok(ProductoA{cuerpo:body,traza:tr,marco})
}
fn ejecutar_v<Ra:Read,Rd:Read>(original_input:&mut Ra,input:&mut Rd,lote:&str,cuerpo:&str,max:u64)->RT<SalidaBytes>{
    if !es_huella(lote)||!es_huella(cuerpo)||max>1_000_000{return Err(FalloT::Esquema)}
    let mut c=CuentaT::nueva(true);c.max_trabajo=max;c.reservado=65536+8192; // Instantánea en argv, anexo y metadatos acotados.
    let recibido=recibir(original_input,65536,&mut c);c.instantanea_bytes=c.recibidos;c.recibidos=0;
    let original=match recibido{Ok(b)=>b,Err(e)=>return anexo_v("V_RECHAZADA",Some(e),None,cuerpo,lote,&c,None)};
    let j=match decodificar(&original,&mut c){Ok(j)=>j,Err(e)=>return anexo_v("V_RECHAZADA",Some(e),None,cuerpo,lote,&c,None)};
    let s=match solicitud(&j,&mut c){Ok(s)=>s,Err(e)=>return anexo_v("V_RECHAZADA",Some(e),None,cuerpo,lote,&c,None)};
    if max!=1_000_000&&!s.id.starts_with("PUBLICO-"){return Err(FalloT::Esquema)}
    let b=match recibir(input,262144,&mut c){Ok(b)=>b,Err(e)=>return anexo_v("V_RECHAZADA",Some(e),Some(s.id),cuerpo,lote,&c,None)};
    if b.is_empty(){return anexo_v("V_AUSENTE",None,Some(s.id),cuerpo,lote,&c,None)}
    let check=(||{let v=decodificar(&b,&mut c)?;verificar_propuesta(&v,&s,lote,&mut c)})();
    match check{Ok(proof)=>anexo_v("DERIVACION_SINTACTICA_COMPROBADA",None,Some(s.id),cuerpo,lote,&c,Some(proof)),Err(e)=>anexo_v("V_RECHAZADA",Some(e),Some(s.id),cuerpo,lote,&c,None)}
}
fn programa()->RT<()>{
    let args:Vec<String>=std::env::args().collect();let mode=args.get(1).map(String::as_str).ok_or(FalloT::Esquema)?;
    match mode{
        "a"=>{if args.len()>3||args.get(2).is_some_and(|x|x!="revocado"){return Err(FalloT::Esquema)}let vigente=args.len()==2;let mut c=CuentaT::nueva(false);let p=match recibir(&mut std::io::stdin().lock(),65536,&mut c){Ok(b)=>resolver_a(&b,&mut c,vigente)?,Err(e)=>rechazo_recepcion(e,&c,vigente)?};publicar_a(p)},
        "v"=>{if !(5..=6).contains(&args.len())||args[2].len()>65536{return Err(FalloT::Esquema)}let max=if let Some(s)=args.get(5){s.parse::<u64>().map_err(|_|FalloT::Entero)?}else{1_000_000};let out=ejecutar_v(&mut args[2].as_bytes(),&mut std::io::stdin().lock(),&args[3],&args[4],max)?;std::io::stdout().lock().write_all(&out.b).map_err(|_|FalloT::Io)},
        "validar"=>{let mut c=CuentaT::nueva(false);let b=recibir(&mut std::io::stdin().lock(),4144,&mut c)?;let body=validar_marco(&b)?;let j=decodificar(body,&mut c)?;let obj=objeto(&j,&["version","montaje","perfil","base","politica","fuente_fijada_sha256","vigente","admision","contexto","resolucion"],&mut c)?;if texto(campo(obj,"version",&mut c)?)?!=VERSION_CUERPO{return Err(FalloT::Esquema)}std::io::stdout().lock().write_all(body).map_err(|_|FalloT::Io)},
        "control"=>controles(),
        "tabla"=>tabla_expresiones(),
        _=>Err(FalloT::Esquema)
    }
}
fn main(){if let Err(e)=programa(){eprintln!("SV_AV_ERROR:{}",fallo_t_nombre(e));std::process::exit(2)}}
fn tabla_expresiones()->RT<()>{let g=Gramatica::nueva().map_err(|_|FalloT::Esquema)?;let mut w=SalidaBytes::nueva(131072)?;w.raw("{\"version\":\"IE004-EXPRESIONES/1\",\"perfil\":")?;w.str(PERFIL)?;w.raw(",\"reglas\":[")?;for(i,x)in g.reglas.iter().enumerate(){if i>0{w.raw(",")?}w.uint(*x as u64)?;}w.raw("],\"expresiones\":[")?;for i in 0..g.n{if i>0{w.raw(",")?}w.raw("{\"id\":")?;w.uint(i as u64)?;w.raw(",\"regla\":")?;w.uint(g.dueno[i]as u64)?;let(tipo,a,b,literal)=match g.ops[i]{Op::Vacio=>("VACIO",None,None,None),Op::Lit(l)=>("LITERAL",None,None,Some(l)),Op::Dig=>("DIGITOS",None,None,None),Op::Ref(r)=>("REFERENCIA",Some(g.reglas[r]),None,None),Op::Par(a,b)=>("SECUENCIA",Some(a),Some(b),None),Op::Disy(a,b)=>("DISYUNCION",Some(a),Some(b),None),Op::Alt(a,b)=>("ALTERNATIVA",Some(a),Some(b),None),Op::Rep(a)=>("REPETICION",Some(a),Some(i as u32),None),Op::Negar(a)=>("NEGAR",Some(a),None,None)};w.raw(",\"tipo\":")?;w.str(tipo)?;for(k,v)in[("a",a),("b",b)]{w.raw(",\"")?;w.raw(k)?;w.raw("\":")?;if let Some(x)=v{w.uint(x as u64)?}else{w.raw("null")?};}w.raw(",\"literal\":")?;if let Some(l)=literal{w.str(l)?}else{w.raw("null")?};w.raw("}")?;}w.raw("]}\n")?;std::io::stdout().lock().write_all(&w.b).map_err(|_|FalloT::Io)}
