#![forbid(unsafe_code)]
include!("gramatica.rs");
include!("recursos.rs");
include!("sintaxis.rs");
include!("semantica.rs");
include!("servicio.rs");
include!("casos_semanticos.rs");
include!("regresion.rs");
include!("comprobar.rs");
fn main(){
    let g=Gramatica::nueva().expect("gramática constituida acíclica y repeticiones positivas");
    println!("{{\"instrumento\":\"IE004-SEMANTICA-A/1\",\"perfil\":\"{}\",\"primitivas\":{},\"usize_bits\":{},\"expresiones\":{},\"bytes_significado\":{},\"bytes_estado_sem\":{},\"bytes_nodo\":{},\"bytes_familia\":{},\"bytes_celda\":{},\"bytes_token\":{},\"bytes_gramatica_fija\":{}}}",PERFIL,primitivas(),usize::BITS,g.n,std::mem::size_of::<Significado>(),std::mem::size_of::<EstadoSem>(),std::mem::size_of::<Nodo>(),std::mem::size_of::<Familia>(),std::mem::size_of::<Celda>(),std::mem::size_of::<Token>(),std::mem::size_of::<Gramatica>());
    let mut correctos=0;for c in SEM_CASOS{if comprobar_semantico(&g,c){correctos+=1;}}
    let mut sintaxis=0;for(id,q,esperado)in CASOS{if comprobar_sintactico(&g,id,q,esperado){sintaxis+=1;}}
    println!("{{\"resumen\":true,\"semanticos_correctos\":{},\"semanticos_total\":{},\"sintacticos_correctos\":{},\"sintacticos_total\":{}}}",correctos,SEM_CASOS.len(),sintaxis,CASOS.len());
    if correctos!=SEM_CASOS.len()||sintaxis!=CASOS.len(){std::process::exit(1)}
}
