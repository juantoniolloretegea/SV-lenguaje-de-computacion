fn contar_arboles(m:&Motor,n:u32,est:&mut[u8;256],v:&mut[u64;256])->R<u64>{
 if n==NULO{return Ok(1)}let k=indice(n as u64)?;if k>=256{return Err(Error::Limite)}
 if est[k]==1{return Err(Error::Rango)}if est[k]==2{return Ok(v[k])}est[k]=1;
 let mut f=m.nodos[k].primera;let mut total=0;
 while f!=NULO{let x=m.familias[f as usize];let a=contar_arboles(m,x.a,est,v)?;let b=contar_arboles(m,x.b,est,v)?;total=suma(total,producto(a,b)?)?;f=x.siguiente;}
 est[k]=2;v[k]=total;Ok(total)
}
fn control_expr(e:E,q:&str,esperado:u64){
 let mut g=Gramatica::nueva().unwrap();g.reglas[0]=g.compilar(e,1).unwrap();let t=lexicalizar(q).unwrap();let mut m=Motor::nuevo(&g,&t).unwrap();m.evaluar(g.reglas[0],0).unwrap();let r=m.raiz().unwrap();assert_ne!(r,NULO);let n=contar_arboles(&m,r,&mut[0;256],&mut[0;256]).unwrap();assert_eq!(n,esperado);
 println!("{{\"control\":\"derivaciones\",\"esperadas\":{},\"observadas\":{},\"unidades\":{}}}",esperado,n,m.cuenta.usado);
}
fn main(){
 control_expr(a!(l!("igg"),l!("igg")),"igg",2);
 control_expr(s!(a!(l!("igg"),l!("igg")),a!(l!("iga"),l!("iga"))),"igg iga",4);
 let repeticion=m!(o!(l!("igg")));assert_eq!(verificar_repeticiones(repeticion,&mut[None;25]),Err(Error::Rango));
 let mut g=Gramatica::nueva().unwrap();g.reglas[0]=g.compilar(repeticion,1).unwrap();let t=lexicalizar("").unwrap();let mut m=Motor::nuevo(&g,&t).unwrap();assert_eq!(m.evaluar(g.reglas[0],0),Err(Error::Rango));
 println!("{{\"control\":\"repeticion_anulable\",\"rechazo_constitucion\":true,\"rechazo_ejecucion_si_se_omite_puerta\":true}}");
 let mut g=Gramatica::nueva().unwrap();g.reglas[0]=g.compilar(a!(l!("igg"),l!("igg")),1).unwrap();let t=lexicalizar("igg").unwrap();let mut m=Motor::nuevo(&g,&t).unwrap();m.cuenta.limite=12;
 assert_eq!(m.evaluar(g.reglas[0],0),Err(Error::Limite));assert_ne!(m.raiz().unwrap(),NULO);assert_eq!(m.cuenta.usado,12);assert_eq!(m.estado[m.clave(g.reglas[0],0).unwrap()],1);
 println!("{{\"control\":\"cierre_incompleto\",\"raiz_parcial_presente\":true,\"evaluacion_completa\":false,\"error_presupuesto\":true,\"unidades\":12}}");
}
