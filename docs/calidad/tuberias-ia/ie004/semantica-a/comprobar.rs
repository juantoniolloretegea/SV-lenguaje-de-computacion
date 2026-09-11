fn contar_derivaciones(m:&Motor<'_>,n:u32,mem:&mut[Option<u64>;256],d:usize)->R<u64>{
    if d>128||n as usize>=256{return Err(Error::Profundidad)}if let Some(v)=mem[n as usize]{return Ok(v)}
    let mut total=0;let mut f=m.nodos[n as usize].primera;
    while f!=NULO{let x=m.familias[f as usize];let a=if x.a==NULO{1}else{contar_derivaciones(m,x.a,mem,d+1)?};let b=if x.b==NULO{1}else{contar_derivaciones(m,x.b,mem,d+1)?};total=suma(total,producto(a,b)?)?;f=x.siguiente;}mem[n as usize]=Some(total);Ok(total)
}
fn primitivas()->usize{
    let mut n=0;
    macro_rules! comprobar{($v:expr)=>{{assert!($v);n+=1;}}}
    comprobar!(suma(u64::MAX,1)==Err(Error::Overflow));comprobar!(suma(u64::MAX-1,1)==Ok(u64::MAX));
    comprobar!(producto(u64::MAX,2)==Err(Error::Overflow));comprobar!(producto(u64::MAX,1)==Ok(u64::MAX));comprobar!(producto(0,u64::MAX)==Ok(0));
    comprobar!(vista("aá🦀z",1,3)==Ok("á"));comprobar!(vista("aá🦀z",3,7)==Ok("🦀"));
    comprobar!(vista("aá🦀z",2,3)==Err(Error::Utf8));comprobar!(vista("aá🦀z",3,6)==Err(Error::Utf8));
    comprobar!(vista("aá🦀z",7,3)==Err(Error::Rango));comprobar!(vista("aá🦀z",0,u64::MAX)==Err(Error::Rango));
    comprobar!(if usize::BITS==32{indice(u64::MAX)==Err(Error::Conversion)}else{indice(u64::MAX).is_ok()});
    let mut r=Recursos::default();let c=Cupos{trabajo:2,..CUPOS};
    comprobar!(r.cobrar(c,0,2).is_ok());comprobar!(r.cobrar(c,0,1)==Err(Error::Trabajo));comprobar!(r.unidades==2);
    r.unidades=u64::MAX;comprobar!(r.cobrar(Cupos{trabajo:u64::MAX,..CUPOS},0,1)==Err(Error::Overflow));
    let mut r=Recursos::default();let mut v:Vec<u32>=Vec::new();
    comprobar!(r.poner(CUPOS,0,&mut v,10,2).is_ok());comprobar!(r.poner(CUPOS,0,&mut v,20,2).is_ok());
    let anterior=r.reservas;comprobar!(r.poner(CUPOS,0,&mut v,30,2)==Err(Error::Memoria));comprobar!(v==[10,20]&&r.reservas==anterior);
    let mut vacio:Vec<u64>=Vec::new();comprobar!(r.reservar(CUPOS,0,&mut vacio,u64::MAX)==Err(Error::Overflow));comprobar!(vacio.is_empty());
    comprobar!(Contexto{campos:[1,1,3,1,1]}.validar()==Err(Error::Contexto));
    let mut minimos=[None;25];comprobar!(verificar_repeticiones(m!(o!(l!("igg"))),&mut minimos)==Err(Error::Constitucion));
    for(e,q,esperadas)in[(a!(l!("igg"),l!("igg")),"igg",2),(s!(a!(l!("igg"),l!("igg")),a!(l!("iga"),l!("iga"))),"igg iga",4)]{
        let mut g=Gramatica{ops:[Op::Vacio;512],dueno:[0;512],n:0,reglas:[0;25]};g.reglas[0]=g.compilar(e,1).expect("gramática de control");
        let mut m=Motor::nuevo(&g,q,CUPOS).expect("motor de control");m.evaluar(g.reglas[0],0,0).expect("cierre de control");let raiz=m.buscar(g.reglas[0],0,m.lexico.tokens.len()).expect("raíz");
        comprobar!(contar_derivaciones(&m,raiz,&mut[None;256],0)==Ok(esperadas));
    }
    n
}
fn admision(c:&Caso)->R<&str>{
    if c.q.len()>8192{return Err(Error::Bytes)}
    if c.perfil!=PERFIL{return Err(Error::Perfil)}
    if c.base!=BASE||c.politica!=POLITICA{return Err(Error::Version)}
    c.contexto.validar()?;std::str::from_utf8(c.q).map_err(|_|Error::Utf8)
}
fn comprobar_semantico(g:&Gramatica,c:&Caso)->bool{
    let mut motor=None;let mut completo=false;let mut fallo_recursos=None;
    let out=match admision(c){Err(e)=>diagnostico(error_nombre(e),0),Ok(q)=>match Motor::nuevo(g,q,c.cupos){Err((e,r))=>{fallo_recursos=Some(r);diagnostico(error_nombre(e),0)},Ok(mut m)=>{
        let out=match m.resolver(c.contexto,c.vigente){Ok(o)=>{completo=true;o},Err(e)=>diagnostico(error_nombre(e),0)};motor=Some(m);out
    }}};
    let coincide=out.estado==c.esperado&&out.literal==c.literal&&c.ruta.is_none_or(|r|r==out.ruta)&&out.llamadas_politica==c.llamadas&&c.raices.is_none_or(|n|completo&&motor.as_ref().is_some_and(|m|m.raices.len()==n));
    print!("{{\"fase\":\"semantica\",\"id\":");json_texto(c.id);print!(",\"coincide\":{},\"salida\":",coincide);json_salida(out);
    if let Some(m)=motor{m.json_evidencia(completo)}else{let r=fallo_recursos.unwrap_or_default();print!(",\"analisis_completo\":false,\"unidades\":{},\"capacidad_bytes\":{},\"pico_solicitado_bytes\":{},\"reservas\":{:?},\"significados\":null,\"constructor_fallido_o_no_iniciado\":true",r.unidades,r.capacidad,r.pico_solicitado,r.reservas);}
    println!("}}");coincide
}
fn comprobar_sintactico(g:&Gramatica,id:&str,q:&str,esperado:&str)->bool{
    let mut motor=None;
    let estado=match Motor::nuevo(g,q,CUPOS){Err((Error::Bytes,_))=>"Bytes",Err((Error::Tokens,_))=>"Tokens",Err(_)=>"ERROR_TECNICO",Ok(mut m)=>{
        let e=match m.evaluar(g.reglas[0],0,0){Ok(())=>match m.buscar(g.reglas[0],0,m.lexico.tokens.len()){Ok(NULO)=>"NO_DERIVA",Ok(_)=>"DERIVA",Err(_)=>"ERROR_TECNICO"},Err(Error::Trabajo)=>"PRESUPUESTO_AGOTADO",Err(_)=>"ERROR_TECNICO"};motor=Some(m);e
    }};
    let ok=estado==esperado;print!("{{\"fase\":\"sintaxis\",\"id\":");json_texto(id);print!(",\"coincide\":{},\"estado\":",ok);json_texto(estado);if let Some(m)=motor{m.json_evidencia(false)}println!("}}");ok
}
