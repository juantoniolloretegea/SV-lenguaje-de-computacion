fn fabricar<'a>(consulta:lote::lectura::ConsultaDeLectura<'a>,caso:&'a[u8],vigencia:&'a[u8])->cobertura::Referencia<'a>{
 cobertura::Referencia{consulta,caso,vigencia}
}
fn main(){}
