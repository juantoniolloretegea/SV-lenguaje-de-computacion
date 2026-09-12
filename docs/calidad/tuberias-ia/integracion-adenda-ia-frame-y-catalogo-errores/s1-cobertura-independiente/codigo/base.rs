#![forbid(unsafe_code)]
use lote::{EnlacePublico,lectura::*};
include!("rangos_fijados.rs");
const L:&[u8]=include_bytes!("../fuentes-base/lote-p3/LOTE_SINTETICO_PUBLICO.json");
const M:&[u8]=include_bytes!("../fuentes-base/lote-p3/MONTAJES_PUBLICOS.json");
fn h(k:&str)->[u8;32]{let s=std::env::var(k).unwrap();assert_eq!(s.len(),64);std::array::from_fn(|i|u8::from_str_radix(&s[i*2..i*2+2],16).unwrap())}
fn main(){
 let dir=std::path::PathBuf::from(std::env::var("S1_CAPTURAS").unwrap());
 let mut e=EnlacePublico::preparar(L,M,g1::Realizacion{fuentes_sha256:h("LG1_FUENTES"),binario_sha256:h("LG1_BINARIO")}).unwrap();
 for i in 0..4 {
  e.ejecutar(i).unwrap();let marco=e.recuperar(i,g1::Papel::Marco).unwrap().to_vec();e.comprobar_entrega(i,&marco).unwrap();
  let q=e.consulta_lectura(i).unwrap();let v=q.comprobar(PropuestaDeLectura{version:VERSION,operacion:OPERACION,identidad:q.identidad(),texto:e.cuerpo(i).unwrap()}).unwrap();
  for (name,b) in [("cuerpo",v.original()),("entregado",v.texto()),("caso",v.caso_original()),("solicitud",v.solicitud()),("traza",v.traza_a())] {std::fs::write(dir.join(format!("P3-{:02}.{name}",i+1)),b).unwrap();}
  if let Some((_,a,z))=RANGOS.iter().find(|(p,_,_)|*p==i) {
   let seleccion=[v.caso_original()];assert_eq!(seleccion[0],v.caso_original());
   std::fs::write(dir.join(format!("P3-{:02}.seleccion-caso",i+1)),seleccion[0]).unwrap();
   std::fs::write(dir.join(format!("P3-{:02}.vigencia-omitida",i+1)),&v.montaje_original()[*a..*z]).unwrap();
   println!("P3-{:02}\tcuerpo_entregado=true\tcita_caso_integra=true\tmontaje_en_seleccion=false\tvigente={}",i+1,e.montaje(i).unwrap());
  }
 }
 std::fs::write(dir.join("lote-original.json"),e.originales().0).unwrap();std::fs::write(dir.join("montaje-original.json"),e.originales().1).unwrap();
}
