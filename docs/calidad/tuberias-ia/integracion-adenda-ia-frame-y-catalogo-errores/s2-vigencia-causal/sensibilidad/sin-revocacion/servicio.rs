#[derive(Clone,Copy)]
struct Salida { estado:&'static str, literal:Option<&'static str>, ruta:[u8;5], causas:u32, llamadas_politica:u32 }
fn diagnostico(estado:&'static str,causas:u32)->Salida{Salida{estado,literal:None,ruta:[0;5],causas,llamadas_politica:0}}
fn error_nombre(e:Error)->&'static str{match e{
 Error::Overflow=>"A_OVERFLOW",Error::Conversion=>"A_CONVERSION",Error::Trabajo=>"A_PRESUPUESTO_AGOTADO",Error::Memoria=>"A_MEMORIA_LIMITE",
 Error::Estados=>"A_ESTADOS_LIMITE",Error::Reserva=>"A_RESERVA_FALLIDA",Error::Rango=>"A_RANGO_INVALIDO",Error::Utf8=>"A_UTF8_INVALIDO",
 Error::Bytes=>"A_BYTES_LIMITE",Error::Tokens=>"A_TOKENS_LIMITE",Error::Perfil=>"PERFIL_INTERACCION_NO_HABILITADO",
 Error::Version=>"VERSION_NO_HABILITADA",Error::Contexto=>"CONTEXTO_MAL_FORMADO",Error::Constitucion=>"A_CONSTITUCION_INVALIDA",Error::Profundidad=>"A_PROFUNDIDAD_LIMITE"
}}
fn permiso_y_dato(s:Significado,vigente:bool,causas:u32)->Salida{
    // Única puerta de política: su argumento ya representa un significado único, completo y sin impedimento semántico.
    let ruta=s.pos.campos;
    let mut out=Salida{estado:"DATO",literal:None,ruta,causas,llamadas_politica:1};
    if ruta[0]!=1 {out.estado="OPERACION_DENEGADA";return out;}
    if ruta[1]!=1||ruta[2]==4 {out.estado="ACCESO_DENEGADO";return out;}
    if false {out.estado="PERMISO_REVOCADO";return out;}
    let(valor,tri)=match(ruta[2],ruta[3]){(1,1)=>("8.40","1"),(1,2)=>("7.10","0"),(2,1)=>("1.25","U"),(2,2)=>("1.15","1"),_=>{out.estado="REFERENCIA_INVALIDA";return out;}};
    out.literal=match ruta[4]{1=>Some(valor),2=>Some("unidad-simulada"),4=>Some(tri),8=>Some("FICCION-IE004/1"),16=>Some("Registro artificial; no acredita concentración ni condición clínica."),_=>{out.estado="REFERENCIA_INVALIDA";None}};out
}
impl<'a> Motor<'a>{
    fn resolver(&mut self,ctx:Contexto,vigente:bool)->R<Salida>{
        self.evaluar(self.g.reglas[0],0,0)?;
        let raiz=self.buscar(self.g.reglas[0],0,self.lexico.tokens.len())?;
        if raiz==NULO{return Ok(diagnostico("SOLICITUD_NO_REPRESENTADA",0))}
        self.significados(raiz,ctx)?;
        if self.raices.is_empty(){return Err(Error::Constitucion)}
        let mut causas=0;for i in 0..self.raices.len(){self.cobrar(9,std::mem::size_of::<Significado>() as u64)?;causas|=self.raices[i].causas();}
        if self.raices.len()>1{return Ok(diagnostico("PETICION_AMBIGUA",causas|32))}
        for(bit,estado)in[(0,"PETICION_CONTRADICTORIA"),(1,"SIN_SOLICITUD_POSITIVA"),(2,"OPERACION_NO_ADMITIDA"),(3,"FUERA_DE_COBERTURA"),(4,"CONTEXTO_INSUFICIENTE")]{
            if causas&(1<<bit)!=0{return Ok(diagnostico(estado,causas))}
        }
        let sig=self.raices[0];
        if sig.pos.campos.iter().any(|x|x.count_ones()!=1){return Err(Error::Constitucion)}
        self.cobrar(9,5)?;
        Ok(permiso_y_dato(sig,vigente,causas))
    }
}
// Serialización de evidencia posterior al análisis. Streaming: no acumulación de JSON dentro de A.
fn json_texto(s:&str){print!("\"");for c in s.chars(){match c{'"'=>print!("\\\""),'\\'=>print!("\\\\"),'\n'=>print!("\\n"),'\r'=>print!("\\r"),'\t'=>print!("\\t"),c if (c as u32)<32=>print!("\\u{:04x}",c as u32),c=>print!("{}",c)}}print!("\"");}
fn json_restriccion(p:Restriccion){print!("{{\"campos\":{:?},\"pendientes\":{},\"operandos\":[",p.campos,p.pendientes);for i in 0..p.ntextos as usize{if i>0{print!(",")}let t=p.textos[i];json_texto(std::str::from_utf8(&t.bytes[..t.n as usize]).expect("operando ASCII validado"));}print!("]}}");}
fn json_significado(s:Significado){print!("{{\"positiva\":{},\"restricciones\":",s.positiva);json_restriccion(s.pos);print!(",\"exclusiones\":[");for i in 0..s.nneg as usize{if i>0{print!(",")}json_restriccion(s.neg[i]);}print!("],\"fuera\":{},\"modificador_glicosilada\":{}}}",s.fuera,s.modificador_glicosilada);}
fn json_salida(s:Salida){print!("{{\"estado\":");json_texto(s.estado);print!(",\"contenido\":");if let Some(l)=s.literal{json_texto(l)}else{print!("null")};print!(",\"ruta\":{:?},\"causas\":{},\"llamadas_politica\":{},\"perfil\":",s.ruta,s.causas,s.llamadas_politica);json_texto(PERFIL);print!(",\"base\":");json_texto(BASE);print!(",\"politica\":");json_texto(POLITICA);print!(",\"fuente\":");if s.literal.is_some(){json_texto("FICCION-IE004/1")}else{print!("null")};print!(",\"alcance\":");if s.literal.is_some(){json_texto("Registro artificial; no acredita concentración ni condición clínica.")}else{print!("null")};print!("}}");}
impl<'a> Motor<'a>{
    fn json_evidencia(&self,completo:bool){
        print!(",\"original\":");json_texto(self.original);print!(",\"normalizado\":");json_texto(std::str::from_utf8(&self.lexico.normal).expect("UTF8 validado"));
        print!(",\"tokens\":[");for(i,t)in self.lexico.tokens.iter().enumerate(){if i>0{print!(",")}print!("[{},{},{}]",t.inicio,t.fin,t.convertido)}print!("]");
        print!(",\"analisis_completo\":{},\"unidades\":{},\"categorias\":{:?},\"capacidad_bytes\":{},\"pico_solicitado_bytes\":{},\"reservas\":{:?},\"crecimientos_solicitados\":{:?},\"capacidad_anterior_acumulada\":{:?},\"capacidad_nueva_acumulada\":{:?},\"estados_semanticos\":{},\"profundidad_sintactica\":{},\"profundidad_semantica\":{}",completo,self.r.unidades,self.r.categorias,self.r.capacidad,self.r.pico_solicitado,self.r.reservas,self.r.crecimientos,self.r.capacidad_anterior_acumulada,self.r.capacidad_nueva_acumulada,self.r.estados,self.profundidad_sintactica,self.profundidad_semantica);
        print!(",\"significados\":");if completo{print!("[");for(i,s)in self.raices.iter().enumerate(){if i>0{print!(",")}json_significado(*s);}print!("]")}else{print!("null")}
        print!(",\"bosque_sintactico\":{{\"nodos\":[");for(i,n)in self.nodos.iter().enumerate(){if i>0{print!(",")}print!("[{},{},{},{},{},{}]",n.expr,n.inicio,n.fin,n.primera,n.ultima,n.siguiente)}
        print!("],\"familias\":[");for(i,f)in self.familias.iter().enumerate(){if i>0{print!(",")}print!("[{},{},{},{}]",f.rama,f.a,f.b,f.siguiente)}print!("]}}");
        print!(",\"estados_de_evidencia\":[");for(i,s)in self.sem_estados.iter().enumerate(){if i>0{print!(",")}print!("{{\"nodo\":{},\"siguiente\":{},\"significado\":",s.nodo,s.siguiente);json_significado(s.significado);print!("}}");}print!("]");
    }
}
