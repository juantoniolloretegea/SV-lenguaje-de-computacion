#![forbid(unsafe_code)]
use std::io::{self,Read};
const MAX:usize=262144;
const CAMPOS:[&str;5]=["operacion","objeto","parametro","momento","campo"];
struct Lector<'a>{b:&'a[u8],p:usize}
impl<'a> Lector<'a>{
 fn num(&mut self)->Result<usize,&'static str>{if self.p+4>self.b.len(){return Err("TRAMA_INCOMPLETA")};let n=u32::from_le_bytes(self.b[self.p..self.p+4].try_into().unwrap()) as usize;self.p+=4;Ok(n)}
 fn texto(&mut self)->Result<String,&'static str>{let n=self.num()?;if n>8192||n>self.b.len().saturating_sub(self.p){return Err("TEXTO_LIMITE")};let s=std::str::from_utf8(&self.b[self.p..self.p+n]).map_err(|_|"UTF8_INVALIDO")?.to_owned();self.p+=n;Ok(s)}
}
fn j(s:&str)->String{let mut x=String::from("\"");for c in s.chars(){match c{'"'=>x.push_str("\\\""),'\\'=>x.push_str("\\\\"),'\n'=>x.push_str("\\n"),'\r'=>x.push_str("\\r"),'\t'=>x.push_str("\\t"),c if (c as u32)<32=>x.push_str(&format!("\\u{:04x}",c as u32)),c=>x.push(c)}}x.push('"');x}
fn diag(s:&str)->String{format!("{{\"estado\":{},\"contenido\":null,\"base\":\"K-IE004/1\",\"politica\":\"P-IE004/1\"}}",j(s))}
fn resolver(q:&str,ctx:&[String],ruta:&[String],apoyos:&[String],d:&str,vigente:bool)->String{
 if !d.is_empty(){if ["CONTEXTO_INSUFICIENTE","FUERA_DE_COBERTURA","OPERACION_NO_ADMITIDA"].contains(&d)&&ruta.iter().all(|s|s.is_empty())&&apoyos.iter().all(|s|s.is_empty()){return diag(d)}return diag("PROPUESTA_INVALIDA")}
 for i in 0..5{let a=&apoyos[i];if a.starts_with('@'){if a!=&format!("@contexto.{}",CAMPOS[i])||ctx[i].is_empty()||ctx[i]!=ruta[i]{return diag("APOYO_INVALIDO")}}else if a.is_empty()||!q.contains(a){return diag("APOYO_INVALIDO")}}
 if !["LEER","ESCRIBIR"].contains(&ruta[0].as_str())||!["CASO-A","CASO-B"].contains(&ruta[1].as_str())||!["IGG","IGA","IGM"].contains(&ruta[2].as_str())||!["ACTUAL","ANTERIOR"].contains(&ruta[3].as_str())||!["VALOR","UNIDAD","ESTADO","FUENTE","ALCANCE"].contains(&ruta[4].as_str()){return diag("REFERENCIA_INVALIDA")}
 if ruta[0]!="LEER"{return diag("OPERACION_DENEGADA")}
 if ruta[1]!="CASO-A"||ruta[2]=="IGM"{return diag("ACCESO_DENEGADO")}
 // Permiso del conductor, no procedente de la propuesta del modelo.
 if !vigente{return diag("PERMISO_REVOCADO")}
 let (valor,tri)=match (ruta[2].as_str(),ruta[3].as_str()){("IGG","ACTUAL")=>("8.40","1"),("IGG","ANTERIOR")=>("7.10","0"),("IGA","ACTUAL")=>("1.25","U"),("IGA","ANTERIOR")=>("1.15","1"),_=>return diag("REFERENCIA_INVALIDA")};
 let contenido=match ruta[4].as_str(){"VALOR"=>valor,"UNIDAD"=>"unidad-simulada","ESTADO"=>tri,"FUENTE"=>"FICCION-IE004/1","ALCANCE"=>"Registro artificial; no acredita concentración ni condición clínica.",_=>unreachable!()};
 format!("{{\"estado\":\"DATO_EXPERIMENTAL\",\"contenido\":{},\"objeto\":{},\"parametro\":{},\"momento\":{},\"campo\":{},\"fuente\":\"FICCION-IE004/1\",\"alcance\":\"Registro artificial; no acredita concentración ni condición clínica.\",\"base\":\"K-IE004/1\",\"politica\":\"P-IE004/1\"}}",j(contenido),j(&ruta[1]),j(&ruta[2]),j(&ruta[3]),j(&ruta[4]))
}
fn ejecutar(b:&[u8])->Result<Vec<String>,&'static str>{
 let mut l=Lector{b,p:0};let n=l.num()?;if n==0||n>256{return Err("CANTIDAD_INVALIDA")};let mut out=Vec::new();
 for _ in 0..n{let q=l.texto()?;let mut ctx=Vec::new();for _ in 0..5{ctx.push(l.texto()?)}let permiso=l.num()?;if permiso>1{return Err("PERMISO_INVALIDO")};let d=l.texto()?;let mut ruta=Vec::new();let mut apoyos=Vec::new();for _ in 0..5{ruta.push(l.texto()?)}for _ in 0..5{apoyos.push(l.texto()?)}out.push(resolver(&q,&ctx,&ruta,&apoyos,&d,permiso==1));}
 if l.p!=b.len(){return Err("TRAMA_SOBRANTE")};Ok(out)
}
fn main(){let mut b=Vec::new();if io::stdin().take((MAX+1) as u64).read_to_end(&mut b).is_err(){println!("{}",diag("LECTURA_FALLIDA"));std::process::exit(2)}if b.len()>MAX{println!("{}",diag("TRAMA_LIMITE"));std::process::exit(2)}match ejecutar(&b){Ok(v)=>for s in v{println!("{}",s)},Err(e)=>{println!("{}",diag(e));std::process::exit(2)}}}
