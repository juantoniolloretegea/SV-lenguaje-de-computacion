use lote::lectura::*;
fn escapar<'a>(q:&'a ConsultaDeLectura<'a>)->LecturaEntregable<'a,'a>{
 let texto=Vec::from(b"{}".as_slice());
 q.comprobar(PropuestaDeLectura{version:VERSION,operacion:OPERACION,identidad:q.identidad(),texto:&texto}).unwrap()
}
fn main(){}
