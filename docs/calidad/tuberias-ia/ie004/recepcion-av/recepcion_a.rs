const VERSION_A:&str="IE004-A-SOLICITUD/1";
const VERSION_CUERPO:&str="IE004-A-CUERPO/1";
const MONTAJE:&str="M-IE004-AV/1";
const FUENTE_FIJADA:&str="e5073224e257bf43e837274b867325a38171363a7521921a25714e3498c7c290";
struct Solicitud<'a>{id:&'a str,pregunta:&'a str,contexto:Contexto}
fn id_permitido(s:&str)->bool{
    if let Some(n)=s.strip_prefix("P3-"){return n.len()==2&&n.bytes().all(|b|b.is_ascii_digit())&&n.parse::<u8>().is_ok_and(|n|(1..=24).contains(&n))}
    s.starts_with("PUBLICO-")&&s.len()<=32&&s.len()>8&&s.bytes().all(|b|b.is_ascii_uppercase()||b.is_ascii_digit()||b==b'-')
}
fn solicitud<'a>(j:&'a J,c:&mut CuentaT)->RT<Solicitud<'a>>{
    let o=objeto(j,&["version","id","pregunta","contexto"],c)?;
    let version=texto(campo(o,"version",c)?)?;if !c.igual(version,VERSION_A)?{return Err(FalloT::Esquema)}
    let id=texto(campo(o,"id",c)?)?;c.cobrar(id.len()as u64)?;if !id_permitido(id){return Err(FalloT::Esquema)}
    let q=texto(campo(o,"pregunta",c)?)?;if q.len()>8192{return Err(FalloT::Bytes)}
    let ctx=objeto(campo(o,"contexto",c)?,&["operacion","objeto","parametro","momento","campo"],c)?;
    let keys=["operacion","objeto","parametro","momento","campo"];
    let permitidos:[&[(&str,u8)];5]=[&[("LEER",1),("ESCRIBIR",2)],&[("CASO-A",1),("CASO-B",2)],&[("IGG",1),("IGA",2),("IGM",4)],&[("ACTUAL",1),("ANTERIOR",2)],&[("VALOR",1),("UNIDAD",2),("ESTADO",4),("FUENTE",8),("ALCANCE",16)]];
    let mut campos=[0;5];for i in 0..5{let val=campo(ctx,keys[i],c)?;if matches!(val,J::N){continue}let name=texto(val)?;let mut found=false;for(s,v)in permitidos[i]{if c.igual(name,s)?{campos[i]=*v;found=true;}}if !found{return Err(FalloT::Esquema)}}
    let contexto=Contexto{campos};contexto.validar().map_err(|_|FalloT::Esquema)?;Ok(Solicitud{id,pregunta:q,contexto})
}
fn fallo_t_nombre(e:FalloT)->&'static str{match e{FalloT::Bytes=>"BYTES_LIMITE",FalloT::Utf8=>"UTF8_INVALIDO",FalloT::Json=>"JSON_INVALIDO",FalloT::Duplicada=>"CLAVE_DUPLICADA",FalloT::Esquema=>"ESQUEMA_INVALIDO",FalloT::Entero=>"ENTERO_O_DIMENSION_INVALIDA",FalloT::Profundidad=>"PROFUNDIDAD_LIMITE",FalloT::Memoria=>"MEMORIA_LIMITE",FalloT::Trabajo=>"PRESUPUESTO_AGOTADO",FalloT::Reserva=>"RESERVA_FALLIDA",FalloT::Io=>"ERROR_IO",FalloT::Truncado=>"TRUNCADO",FalloT::Correlacion=>"CORRELACION_INVALIDA",FalloT::Certificado=>"CERTIFICADO_INVALIDO"}}
fn array_u8(w:&mut SalidaBytes,v:&[u8])->RT<()>{w.raw("[")?;for(i,x)in v.iter().enumerate(){if i>0{w.raw(",")?}w.uint(*x as u64)?;}w.raw("]")}
fn array_u64(w:&mut SalidaBytes,v:&[u64])->RT<()>{w.raw("[")?;for(i,x)in v.iter().enumerate(){if i>0{w.raw(",")?}w.uint(*x)?;}w.raw("]")}
fn emitir_salida(w:&mut SalidaBytes,s:Salida)->RT<()>{
    w.raw("{\"estado\":")?;w.str(s.estado)?;w.raw(",\"contenido\":")?;if let Some(l)=s.literal{w.str(l)?}else{w.raw("null")?};w.raw(",\"ruta\":")?;array_u8(w,&s.ruta)?;
    w.raw(",\"causas\":")?;w.uint(s.causas as u64)?;w.raw(",\"llamadas_politica\":")?;w.uint(s.llamadas_politica as u64)?;
    w.raw(",\"fuente\":")?;if s.literal.is_some(){w.str("FICCION-IE004/1")?}else{w.raw("null")?};w.raw(",\"alcance\":")?;
    if s.literal.is_some(){w.str("Registro artificial; no acredita concentración ni condición clínica.")?}else{w.raw("null")?};w.raw("}")
}
fn cuerpo_a(out:Salida,ctx:Option<Contexto>,vigente:bool,admitida:bool)->RT<SalidaBytes>{
    let mut w=SalidaBytes::nueva(4096)?;w.raw("{\"version\":")?;w.str(VERSION_CUERPO)?;w.raw(",\"montaje\":")?;w.str(MONTAJE)?;
    for(k,v)in[("perfil",PERFIL),("base",BASE),("politica",POLITICA),("fuente_fijada_sha256",FUENTE_FIJADA)]{w.raw(",\"")?;w.raw(k)?;w.raw("\":")?;w.str(v)?;}
    w.raw(",\"vigente\":")?;w.boolean(vigente)?;w.raw(",\"admision\":")?;w.str(if admitida{"ADMITIDA"}else{"RECHAZADA"})?;
    w.raw(",\"contexto\":")?;if let Some(c)=ctx{array_u8(&mut w,&c.campos)?}else{w.raw("null")?};w.raw(",\"resolucion\":")?;emitir_salida(&mut w,out)?;w.raw("}\n")?;Ok(w)
}
fn restriccion_bytes(w:&mut SalidaBytes,p:Restriccion)->RT<()>{w.raw("{\"campos\":")?;array_u8(w,&p.campos)?;w.raw(",\"pendientes\":")?;w.uint(p.pendientes as u64)?;w.raw(",\"operandos\":[")?;for i in 0..p.ntextos as usize{if i>0{w.raw(",")?}let t=p.textos[i];w.str(std::str::from_utf8(&t.bytes[..t.n as usize]).map_err(|_|FalloT::Utf8)?)?;}w.raw("]}")}
fn significado_bytes(w:&mut SalidaBytes,s:Significado)->RT<()>{w.raw("{\"positiva\":")?;w.boolean(s.positiva)?;w.raw(",\"restricciones\":")?;restriccion_bytes(w,s.pos)?;w.raw(",\"exclusiones\":[")?;for i in 0..s.nneg as usize{if i>0{w.raw(",")?}restriccion_bytes(w,s.neg[i])?;}w.raw("],\"fuera\":")?;w.boolean(s.fuera)?;w.raw(",\"modificador_glicosilada\":")?;w.boolean(s.modificador_glicosilada)?;w.raw("}")}
struct ProductoA{cuerpo:SalidaBytes,traza:SalidaBytes,marco:Vec<u8>}
fn resolver_a(b:&[u8],c:&mut CuentaT,vigente:bool)->RT<ProductoA>{
    let j=decodificar(b,c);let error_transporte;
    let salida=match &j{Err(e)=>{error_transporte=Some(*e);diagnostico("A_TRANSPORTE_RECHAZADO",0)},Ok(j)=>match solicitud(j,c){
        Err(e)=>{error_transporte=Some(e);diagnostico("A_TRANSPORTE_RECHAZADO",0)},Ok(s)=>{
            let g=Gramatica::nueva().map_err(|_|FalloT::Esquema)?;
            // La gramática deberá vivir con el motor: se resuelve y serializa dentro del préstamo.
            return resolver_admitida(&g,&s,b,c,vigente);
        }
    }};
    // La admisión fallida no inicia gramática, lexer ni política de A.
    let body=cuerpo_a(salida,None,vigente,false)?;let mut trace=SalidaBytes::nueva(65536)?;
    trace.raw("{\"version\":\"IE004-A-TRAZA/1\",\"iniciado_a\":false,\"error_transporte\":")?;trace.str(fallo_t_nombre(error_transporte.ok_or(FalloT::Esquema)?))?;
    trace.raw(",\"recibidos\":")?;trace.uint(c.recibidos)?;trace.raw(",\"original_transporte_sha256\":")?;trace.str(sha_str(&sha_texto(b)?)?)?;trace.raw(",\"cuerpo_sha256\":")?;trace.str(sha_str(&sha_texto(&body.b)?)?)?;trace.raw("}\n")?;
    let marco=marco_a(&body.b)?;Ok(ProductoA{cuerpo:body,traza:trace,marco})
}
fn resolver_admitida(g:&Gramatica,s:&Solicitud<'_>,wire:&[u8],c:&CuentaT,vigente:bool)->RT<ProductoA>{
    let mut completo=false;let mut motor=None;let mut fallo=Recursos::default();
    let out=match Motor::nuevo(g,s.pregunta,CUPOS){Err((e,r))=>{fallo=r;diagnostico(error_nombre(e),0)},Ok(mut m)=>{let o=match m.resolver(s.contexto,vigente){Ok(o)=>{completo=true;o},Err(e)=>diagnostico(error_nombre(e),0)};motor=Some(m);o}};
    let body=cuerpo_a(out,Some(s.contexto),vigente,true)?;
    let n=motor.as_ref().map_or(0,|m|if completo{m.raices.len()}else{0});if n>16384{return Err(FalloT::Memoria)};
    let max=65536usize.checked_add(n.checked_mul(1024).ok_or(FalloT::Entero)?).ok_or(FalloT::Entero)?;
    let mut tr=SalidaBytes::nueva(max)?;tr.raw("{\"version\":\"IE004-A-TRAZA/1\",\"id\":")?;tr.str(s.id)?;
    tr.raw(",\"original\":")?;tr.str(s.pregunta)?;tr.raw(",\"original_transporte_sha256\":")?;tr.str(sha_str(&sha_texto(wire)?)?)?;
    tr.raw(",\"cuerpo_sha256\":")?;tr.str(sha_str(&sha_texto(&body.b)?)?)?;tr.raw(",\"iniciado_a\":true,\"analisis_completo\":")?;tr.boolean(completo)?;
    tr.raw(",\"admision_trabajo\":")?;tr.uint(c.trabajo)?;tr.raw(",\"admision_reserva_acumulada_bytes\":")?;tr.uint(c.reservado)?;
    let r=motor.as_ref().map_or(&fallo,|m|&m.r);tr.raw(",\"unidades_a\":")?;tr.uint(r.unidades)?;tr.raw(",\"categorias_a\":")?;array_u64(&mut tr,&r.categorias)?;
    tr.raw(",\"capacidad_a_bytes\":")?;tr.uint(r.capacidad)?;tr.raw(",\"pico_solicitado_a_bytes\":")?;tr.uint(r.pico_solicitado)?;tr.raw(",\"salida_reservada_bytes\":")?;tr.uint((max+4096+body.b.len()+48)as u64)?;
    tr.raw(",\"resolucion\":")?;emitir_salida(&mut tr,out)?;tr.raw(",\"normalizado\":")?;
    if let Some(m)=&motor{tr.str(std::str::from_utf8(&m.lexico.normal).map_err(|_|FalloT::Utf8)?)?;tr.raw(",\"tokens\":[")?;for(i,t)in m.lexico.tokens.iter().enumerate(){if i>0{tr.raw(",")?}tr.raw("[")?;tr.uint(t.inicio)?;tr.raw(",")?;tr.uint(t.fin)?;tr.raw(",")?;tr.boolean(t.convertido)?;tr.raw("]")?;}tr.raw("]")?;}else{tr.raw("null,\"tokens\":null")?;}
    tr.raw(",\"significados\":")?;if completo{tr.raw("[")?;if let Some(m)=&motor{for(i,x)in m.raices.iter().enumerate(){if i>0{tr.raw(",")?}significado_bytes(&mut tr,*x)?;}}tr.raw("]")?;}else{tr.raw("null")?;}
    tr.raw("}\n")?;let marco=marco_a(&body.b)?;Ok(ProductoA{cuerpo:body,traza:tr,marco})
}
