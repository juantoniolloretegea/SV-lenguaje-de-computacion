//! ES: Inspección previa documental y geométrica; no rasteriza ni prueba píxeles.
//! EN: Documentary and geometric preflight; does not rasterize or test pixels.
#![forbid(unsafe_code)]
#[path = "../bis04-realizacion-i0205-v0_1/proyecto/src/sha.rs"]
mod sha;
use std::{collections::BTreeMap, fs, io::Read, path::Path};
type Point = (i64,i64);
fn edges(p: &[Point]) -> BTreeMap<(Point,Point),usize> {
    let mut m=BTreeMap::new();
    for i in 0..p.len() { let a=p[i]; let b=p[(i+1)%p.len()];
        *m.entry(if a<=b {(a,b)} else {(b,a)}).or_insert(0)+=1;
    } m
}
fn bounded(p:&Path,n:u64)->Result<Vec<u8>,Box<dyn std::error::Error>> {
    let mut b=Vec::new(); fs::File::open(p)?.take(n+1).read_to_end(&mut b)?;
    if b.len() as u64>n {return Err("CUOTA_PREVIA".into());} Ok(b)
}
fn run()->Result<(),Box<dyn std::error::Error>>{
    let args:Vec<_>=std::env::args().collect();
    if args.len()!=4 {return Err("uso: preflight SVG FONT ARCHIVO_RESVG".into());}
    let svg=bounded(Path::new(&args[1]),4096)?;
    if sha::hash(&svg)!="e5a9f9bc6df9b4f59a2a0cb908046239fda11ebbc247577b8e8c32b268a8c578" {return Err("SVG_DISTINTO".into());}
    let s=std::str::from_utf8(&svg)?;
    let body=s.split_once("points=\"").ok_or("PUNTOS")?.1.split_once('"').ok_or("PUNTOS")?.0;
    let points:Vec<Point>=body.split(' ').map(|p|{
        let (x,y)=p.split_once(',').ok_or("PAR")?;
        Ok((x.parse::<i64>()?,y.parse::<i64>()?))
    }).collect::<Result<_,Box<dyn std::error::Error>>>()?;
    if points.len()!=16 {return Err("DIMENSION".into());}
    let mut shifted=points.clone(); shifted.rotate_left(1);
    let mut reversed=points.clone(); reversed.reverse();
    if shifted==points || reversed==points || edges(&shifted)!=edges(&points) || edges(&reversed)!=edges(&points) {return Err("TESTIGO_GEOMETRICO".into());}
    println!("clase\tobjeto\tresultado\tdetalle");
    println!("identidad\tSVG\tCONFORME\t792 bytes; SHA256 {}",sha::hash(&svg));
    println!("estructura\ttipografia\tSIN_FIJAR_EN_SVG\tfont-family ausente: {}",!s.contains("font-family"));
    println!("estructura\torientacion_visible\tNO_DIBUJADA\tP1 y sentido en desc/data-orden; no etiquetas por vertice");
    println!("geometria_exacta\tdesplazamiento_ciclico\tIGUAL_CONJUNTO_DE_ARISTAS\tsecuencia posicional distinta; sin prueba de pixeles");
    println!("geometria_exacta\tinversion\tIGUAL_CONJUNTO_DE_ARISTAS\tsecuencia posicional distinta; sin prueba de pixeles");
    // ES: Límites geométricos racionales, sin redondear a píxeles.
    // EN: Rational geometric bounds without rounding to pixels.
    let xs:Vec<_>=points.iter().map(|p|p.0+4_000_000).collect();
    let ys:Vec<_>=points.iter().map(|p|p.1+4_000_000).collect();
    println!("geometria_exacta\tlimites_vertices_px\tCALCULADOS\tx=[{},{}]/25000 y=[{},{}]/25000",xs.iter().min().unwrap(),xs.iter().max().unwrap(),ys.iter().min().unwrap(),ys.iter().max().unwrap());
    let font=bounded(Path::new(&args[2]),2_000_000)?;
    println!("dependencia\tDejaVuSans.ttf\tPRESENTE\tbytes={} SHA256={}",font.len(),sha::hash(&font));
    match bounded(Path::new(&args[3]),20_000_000) {
        Ok(b)=>{if sha::hash(&b)!="13ed5a2bae7a01156288ecae5bf944cf7d1c572742c19fc68027947a4d87294c" {return Err("ARCHIVO_RESVG_HUELLA".into());}
            println!("dependencia\tresvg-0.48.1.tar.xz\tARCHIVO_COINCIDENTE\tno acredita compilacion ni ejecucion");
        },
        Err(e)=>println!("dependencia\tresvg-0.48.1.tar.xz\tNO_DISPONIBLE\t{e}"),
    }
    println!("campana\trasterizacion\tNO_EJECUTADA\t0 imagenes; 0 casos de pixeles; oraculos por completar");
    Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}
