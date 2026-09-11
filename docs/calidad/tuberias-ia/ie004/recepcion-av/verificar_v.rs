const VERSION_V:&str="IE004-P3-V/1";
const VERSION_CERT:&str="IE004-DERIVACION-EXP/1";
#[derive(Clone,Copy,Default)]
struct PasoV{expr:u32,regla:u8,alternativa:u8,inicio:usize,fin:usize,hijos:[usize;2],nh:usize}
fn altura_v(h:u8)->RT<u8>{let n=h.checked_add(1).ok_or(FalloT::Entero)?;if n>64{return Err(FalloT::Profundidad)}Ok(n)}
fn enlaces_v(n:usize,mas:usize)->RT<usize>{let total=n.checked_add(mas).ok_or(FalloT::Entero)?;if total>1024{return Err(FalloT::Certificado)}Ok(total)}
fn frontera(lex:&Lexico,q:&str,k:usize)->RT<u64>{if k==0{return Ok(0)}if k==lex.tokens.len(){return Ok(q.len()as u64)}lex.tokens.get(k).map(|t|t.inicio).ok_or(FalloT::Certificado)}
fn posicion(lex:&Lexico,q:&str,x:u64,c:&mut CuentaT)->RT<usize>{
    c.cobrar(1)?;let x_usize=usize::try_from(x).map_err(|_|FalloT::Entero)?;if x_usize>q.len()||!q.is_char_boundary(x_usize){return Err(FalloT::Certificado)}
    for k in 0..=lex.tokens.len(){c.cobrar(1)?;if frontera(lex,q,k)?==x{return Ok(k)}}Err(FalloT::Certificado)
}
fn verificar_propuesta(j:&J,s:&Solicitud<'_>,lote:&str,c:&mut CuentaT)->RT<(usize,usize,u8)>{
    let env=objeto(j,&["version","id","solicitudes_sha256","certificado"],c)?;
    for(key,want)in[("version",VERSION_V),("id",s.id),("solicitudes_sha256",lote)]{let x=texto(campo(env,key,c)?)?;if !c.igual(x,want)?{return Err(FalloT::Correlacion)}}
    let cert=objeto(campo(env,"certificado",c)?,&["version","perfil","fuente_fijada_sha256","pregunta_sha256","raiz","nodos"],c)?;
    c.cobrar(s.pregunta.len()as u64)?;let qsha=sha_texto(s.pregunta.as_bytes())?;
    for(key,want)in[("version",VERSION_CERT),("perfil",PERFIL),("fuente_fijada_sha256",FUENTE_FIJADA),("pregunta_sha256",sha_str(&qsha)?)]{let x=texto(campo(cert,key,c)?)?;if !c.igual(x,want)?{return Err(FalloT::Correlacion)}}
    let raw=lista_v(campo(cert,"nodos",c)?)?;c.cobrar(1)?;if raw.is_empty()||raw.len()>512{return Err(FalloT::Certificado)}
    let root=usize::try_from(numero(campo(cert,"raiz",c)?)?).map_err(|_|FalloT::Entero)?;c.cobrar(1)?;if root!=raw.len()-1{return Err(FalloT::Certificado)}
    // Léxico privado de V; cargo previo conservador cubre sus pasadas sobre bytes.
    let coste=(s.pregunta.len()as u64).checked_mul(8).and_then(|n|n.checked_add(512)).ok_or(FalloT::Entero)?;c.cobrar(coste)?;
    let mut recursos=Recursos::default();let lexico=Lexico::nueva(s.pregunta,&mut recursos,Cupos{trabajo:coste,memoria:65536,estados:0}).map_err(|_|FalloT::Certificado)?;
    c.reservado=c.reservado.checked_add(recursos.capacidad).ok_or(FalloT::Entero)?;if c.reservado>c.max_memoria{return Err(FalloT::Memoria)}
    let g=Gramatica::nueva().map_err(|_|FalloT::Certificado)?;let mut pasos:Vec<PasoV>=Vec::new();c.reservar(&mut pasos,raw.len())?;
    let mut enlaces=0usize;let mut alturas=[0u8;512];let mut alcanzado=[false;512];
    for(i,x)in raw.iter().enumerate(){
        let o=objeto(x,&["regla","expresion","alternativa","inicio","fin","hijos"],c)?;
        let expr=u32::try_from(numero(campo(o,"expresion",c)?)?).map_err(|_|FalloT::Entero)?;
        let regla=u8::try_from(numero(campo(o,"regla",c)?)?).map_err(|_|FalloT::Entero)?;
        let alternativa=u8::try_from(numero(campo(o,"alternativa",c)?)?).map_err(|_|FalloT::Entero)?;
        c.cobrar(3)?;if expr as usize>=g.n||regla!=g.dueno[expr as usize]{return Err(FalloT::Certificado)}
        let inicio=posicion(&lexico,s.pregunta,numero(campo(o,"inicio",c)?)?,c)?;
        let fin=posicion(&lexico,s.pregunta,numero(campo(o,"fin",c)?)?,c)?;c.cobrar(1)?;if inicio>fin{return Err(FalloT::Certificado)}
        let js=lista(campo(o,"hijos",c)?)?;c.cobrar(1)?;if js.len()>2{return Err(FalloT::Certificado)}
        enlaces=enlaces_v(enlaces,js.len())?;
        let mut hijos=[0usize;2];let mut altura=1u8;
        for(k,j)in js.iter().enumerate(){c.cobrar(2)?;let n=usize::try_from(numero(j)?).map_err(|_|FalloT::Entero)?;if n>=i{return Err(FalloT::Certificado)}hijos[k]=n;altura=altura.max(altura_v(alturas[n])?);}
        if altura>64{return Err(FalloT::Profundidad)}alturas[i]=altura;
        let p=PasoV{expr,regla,alternativa,inicio,fin,hijos,nh:js.len()};
        c.cobrar(1)?; // Toda transición de expresión se paga antes de comprobarla.
        let exacto=|child:PasoV,want:u32|child.expr==want&&child.inicio==inicio&&child.fin==fin;
        match g.ops[expr as usize]{
            Op::Vacio=>{c.cobrar(3)?;if p.nh!=0||alternativa!=0||inicio!=fin{return Err(FalloT::Certificado)}},
            Op::Lit(l)=>{c.cobrar(3)?;if p.nh!=0||alternativa!=0||fin-inicio!=l.split(' ').count(){return Err(FalloT::Certificado)}for(k,want)in l.split(' ').enumerate(){let got=lexico.token(inicio+k).map_err(|_|FalloT::Certificado)?;if !c.igual(got,want)?{return Err(FalloT::Certificado)}}},
            Op::Dig=>{c.cobrar(3)?;if p.nh!=0||alternativa!=0||fin!=inicio.checked_add(1).ok_or(FalloT::Entero)?{return Err(FalloT::Certificado)}let t=lexico.token(inicio).map_err(|_|FalloT::Certificado)?;c.cobrar(t.len()as u64+1)?;if t.is_empty()||t.len()>10||!t.bytes().all(|b|b.is_ascii_digit()){return Err(FalloT::Certificado)}},
            Op::Ref(r)=>{c.cobrar(5)?;if p.nh!=1||alternativa!=0||!exacto(pasos[hijos[0]],g.reglas[r]){return Err(FalloT::Certificado)}},
            Op::Negar(e)=>{c.cobrar(5)?;if p.nh!=1||alternativa!=0||!exacto(pasos[hijos[0]],e){return Err(FalloT::Certificado)}},
            Op::Alt(a,b)=>{c.cobrar(5)?;if p.nh!=1||!(1..=2).contains(&alternativa)||!exacto(pasos[hijos[0]],if alternativa==1{a}else{b}){return Err(FalloT::Certificado)}},
            Op::Par(a,b)|Op::Disy(a,b)=>{c.cobrar(8)?;if p.nh!=2||alternativa!=0{return Err(FalloT::Certificado)}let x=pasos[hijos[0]];let y=pasos[hijos[1]];if x.expr!=a||y.expr!=b||x.inicio!=inicio||x.fin!=y.inicio||y.fin!=fin{return Err(FalloT::Certificado)}},
            Op::Rep(e)=>{c.cobrar(9)?;match alternativa{1 if p.nh==0&&inicio==fin=>(),2 if p.nh==2=>{let x=pasos[hijos[0]];let y=pasos[hijos[1]];if x.expr!=e||y.expr!=expr||x.inicio!=inicio||x.fin<=inicio||x.fin!=y.inicio||y.fin!=fin{return Err(FalloT::Certificado)}},_=>return Err(FalloT::Certificado)}}
        }
        if pasos.len()>=pasos.capacity(){return Err(FalloT::Reserva)}pasos.push(p);
    }
    let root_p=pasos[root];c.cobrar(3)?;if root_p.expr!=g.reglas[0]||root_p.inicio!=0||root_p.fin!=lexico.tokens.len(){return Err(FalloT::Certificado)}
    alcanzado[root]=true;for i in(0..pasos.len()).rev(){c.cobrar(1)?;if alcanzado[i]{for &h in &pasos[i].hijos[..pasos[i].nh]{c.cobrar(1)?;alcanzado[h]=true;}}}
    for reached in &alcanzado[..pasos.len()]{c.cobrar(1)?;if !reached{return Err(FalloT::Certificado)}}
    Ok((pasos.len(),enlaces,alturas[root]))
}
fn lista_v(j:&J)->RT<&[J]>{lista(j)}
fn anexo_v(estado:&str,causa:Option<FalloT>,id:Option<&str>,cuerpo:&str,lote:&str,cuenta:&CuentaT,prueba:Option<(usize,usize,u8)>)->RT<SalidaBytes>{
    let mut w=SalidaBytes::nueva(4096)?;w.raw("{\"version\":\"IE004-V-ANEXO/1\",\"estado\":")?;w.str(estado)?;w.raw(",\"causa\":")?;if let Some(e)=causa{w.str(fallo_t_nombre(e))?}else{w.raw("null")?};
    w.raw(",\"id\":")?;if let Some(id)=id{w.str(id)?}else{w.raw("null")?};w.raw(",\"cuerpo_a_sha256\":")?;w.str(cuerpo)?;w.raw(",\"solicitudes_sha256\":")?;w.str(lote)?;
    w.raw(",\"bytes_v_observados\":")?;w.uint(cuenta.recibidos)?;w.raw(",\"bytes_instantanea_a\":")?;w.uint(cuenta.instantanea_bytes)?;w.raw(",\"unidades_v\":")?;w.uint(cuenta.trabajo)?;w.raw(",\"reserva_acumulada_v_bytes\":")?;w.uint(cuenta.reservado)?;
    w.raw(",\"autoridad_sobre_cuerpo_a\":false,\"prueba\":")?;if let Some((n,e,d))=prueba{w.raw("{\"nodos\":")?;w.uint(n as u64)?;w.raw(",\"enlaces\":")?;w.uint(e as u64)?;w.raw(",\"profundidad\":")?;w.uint(d as u64)?;w.raw("}")?;}else{w.raw("null")?;}w.raw("}\n")?;Ok(w)
}
