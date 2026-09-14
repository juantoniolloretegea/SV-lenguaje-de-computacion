//! ES: Banco raster acotado. Oráculo geométrico independiente; no interpreta texto.
//! EN: Bounded raster bank. Independent geometric oracle; no text interpretation.
#![forbid(unsafe_code)]
#[path="../bis04-realizacion-i0205-v0_1/proyecto/src/sha.rs"] mod sha;
use std::{fs,io::Read,path::Path,process::Command};
use tiny_skia::Pixmap;
type R<T>=Result<T,Box<dyn std::error::Error>>;
const P:[(f64,f64);16]=[(1000000.,0.),(1847760.,-765366.),(2121321.,-2121321.),(382683.,-923880.),(0.,-1000000.),(-765366.,-1847760.),(-2121321.,-2121321.),(-923880.,-382683.),(-2000000.,0.),(-2771640.,1148049.),(-1414214.,1414214.),(-382683.,923880.),(0.,3000000.),(382683.,923880.),(1414214.,1414214.),(2771640.,1148049.)];
fn read(p:&Path)->R<Vec<u8>> {let mut b=Vec::new();fs::File::open(p)?.take(1_000_001).read_to_end(&mut b)?;if b.len()>1_000_000{return Err("CUOTA".into())}Ok(b)}
fn inspect(b:&[u8])->R<(bool,bool,u64)> {
 if b.len()<24 || &b[..8]!=b"\x89PNG\r\n\x1a\n" || b[16..24]!=[0,0,1,64,0,0,1,104] {return Err("FORMATO_DIMENSION".into())}
 let im=Pixmap::decode_png(b)?;if im.width()!=320||im.height()!=360{return Err("DIMENSION".into())}
 let ink=|x:usize,y:usize|{let p=&im.data()[(y*320+x)*4..][..4];p[3]>0 && p[..3]!=[255,255,255]};
 let mut all=true;
 for i in 0..16 {
  let a=P[i];let z=P[(i+1)%16];
  let x=(((a.0+z.0)/2.+4_000_000.)/25000.).floor() as i32;
  let y=(((a.1+z.1)/2.+4_000_000.)/25000.).floor() as i32;
  all &= (y-2..=y+2).any(|yy|(x-2..=x+2).any(|xx|ink(xx as usize,yy as usize)));
 }
 let mut outside=false;let mut signature=0u64;let mut legend=false;
 for y in 0..360 {for x in 0..320 {
  if ink(x,y) {signature+=((y*320+x) as u64)+1; if y>=318 && y<344 && x>=15 && x<310 {legend=true}
   if y<35||x<15||x>=305||y>=350 {outside=true}
  }
 }}
 Ok((all&&!outside,legend,signature))
}
fn draw(bin:&Path,font:&Path,svg:&Path,png:&Path,out:&Path,id:&str)->R<()>{
 let args=["--skip-system-fonts","--use-font-file",font.to_str().ok_or("RUTA")?,"--font-family","DejaVu Sans","--background","#ffffff",svg.to_str().ok_or("RUTA")?,png.to_str().ok_or("RUTA")?];
 let result=Command::new(bin).args(args).output()?;
 fs::write(out.join(format!("{id}.stdout")),result.stdout)?;
 fs::write(out.join(format!("{id}.stderr")),result.stderr)?;
 fs::write(out.join(format!("{id}.comando.txt")),format!("programa={}\nargv={args:?}\nretorno={:?}\n",bin.display(),result.status.code()))?;
 if !result.status.success(){return Err("RASTERIZADOR".into())}Ok(())
}
fn run()->R<()>{
 let a:Vec<_>=std::env::args().collect();if a.len()!=5{return Err("uso: banco RESVG FONT SVG OUT".into())}
 let bin=Path::new(&a[1]);let font=Path::new(&a[2]);let svg=Path::new(&a[3]);let out=Path::new(&a[4]);fs::create_dir(out)?;
 let input=read(svg)?;if sha::hash(&input)!="e5a9f9bc6df9b4f59a2a0cb908046239fda11ebbc247577b8e8c32b268a8c578" {return Err("SVG".into())}
 if sha::hash(&read(font)?)!="ae7b7855e115a5966d8b1b3f80f254ccc117ec86f9965e202ee2940453837280"{return Err("FUENTE".into())}
 fs::write(out.join("entrada.svg"),&input)?;
 draw(bin,font,&out.join("entrada.svg"),&out.join("R01.png"),out,"R01")?;
 let auth=read(&out.join("R01.png"))?;
 let good=inspect(&auth)?;let mut rows=String::from("caso\tgeometria\tleyenda_presente\tfirma_pixeles\tconforme_al_oraculo_acotado\n");
 let mut conform=true;
 let mut add=|id:&str,v:(bool,bool,u64),expected:(bool,bool)| {let ok=(v.0,v.1)==expected;conform &=ok;rows+=&format!("{id}\t{}\t{}\t{}\t{ok}\n",v.0,v.1,v.2);};
 add("R01",good,(true,true));
 // ES: Fallos sobre el artefacto, con codificación PNG válida.
 // EN: Artifact faults with valid PNG encoding.
 for (id,expected) in [("R02",(true,false)),("R03",(false,true)),("R04",(false,true)),("R05",(false,false))] {
  let mut im=Pixmap::decode_png(&auth)?;let original=im.data().to_vec();
  for y in 0..360 {for x in 0..320 {let i=(y*320+x)*4;
   if (id=="R02"&&y>=310)||(id=="R04"&&x>=200&&y<310)||id=="R05" {im.data_mut()[i..i+4].copy_from_slice(&[255;4]);}
   if id=="R03"&&y<310 {let j=(y*320+319-x)*4;im.data_mut()[i..i+4].copy_from_slice(&original[j..j+4]);}
  }}
  im.save_png(out.join(format!("{id}.png")))?;let b=read(&out.join(format!("{id}.png")))?;add(id,inspect(&b)?,expected);
 }
 // ES: Alteración semántica de leyenda: debe mostrar el límite del observador.
 // EN: Legend meaning alteration: must expose the observer's limitation.
 let text=std::str::from_utf8(&input)?.replace("0: radio 1 | 1: radio 2 | U: radio 3","0: radio 2 | 1: radio 1 | U: radio 3");
 fs::write(out.join("R06.svg"),text)?;
 draw(bin,font,&out.join("R06.svg"),&out.join("R06.png"),out,"R06")?;
 add("R06_limite_leyenda",inspect(&read(&out.join("R06.png"))?)?,(true,true));
 drop(add);
 let substituted=read(&out.join("R05.png"))?;
 let blocked=substituted!=auth;
 let fake=inspect(&substituted)?;
 let false_detected=fake.2!=good.2;
 conform &= blocked && false_detected;
 rows+=&format!("R07_sustitucion\tNA\tNA\tNA\t{blocked}\nR08_consumo_falso\tNA\tNA\t{}\t{false_detected}\n",fake.2);
 fs::write(out.join("OBSERVACIONES.tsv"),rows)?;
 fs::write(out.join("IDENTIDADES.txt"),format!("svg={}\nfont={}\nresvg={}\npng={}\n",sha::hash(&input),sha::hash(&read(font)?),sha::hash(&fs::read(bin)?),sha::hash(&auth)))?;
 println!("ocho sondas; conformidad acotada={conform}; R06 documenta falta de verificacion del significado de leyenda");
 if !conform{return Err("DISCREPANCIA".into())} Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1)}}
