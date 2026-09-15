use leyenda_contenido::{atribucion::{atribuir,Atribucion},plantillas::Mascara,parametros::Parametros};
use std::collections::BTreeSet;
fn main() {
 let p=Parametros::analizar("n_min=1\ntheta=0.5\nepsilon=0.02\nrho=0.1\nr_max=10\na_min=4\ns_px=12.8\ntau_t=1\nd_min=1\ng_min=1\nw_sep_min=1\nw_sep_max=6\npaso_x=1\nmax_rasterizaciones=20000\nmax_pixeles_mascara=8000\n").unwrap();
 let m=|text:&str,s:f64| Mascara{texto:text.into(),origen_x:20,origen_y:334,puntuacion:s,pixeles:BTreeSet::from([(20,334)])};
 let a=m("0:",0.95);let b=m("radio 1",0.80);let c=m("|",0.79);
 let x=atribuir(&[a.clone(),b.clone(),c.clone()],&p);
 let y=atribuir(&[b,c,a],&p);
 println!("Mismo pixel; S=0.95,0.80,0.79; epsilon=0.02");
 println!("orden [0.95,0.80,0.79]: {:?}",x);
 println!("orden [0.80,0.79,0.95]: {:?}",y);
 assert_eq!(x,Atribucion::Exclusiva);assert_eq!(y,Atribucion::Empate);
 println!("Contraejemplo de dependencia del orden reproducido; no es Q1/Q2.");
}
