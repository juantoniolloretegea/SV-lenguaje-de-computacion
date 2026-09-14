#![forbid(unsafe_code)]
use std::{collections::BTreeMap,fs,path::Path};
fn read(p:&Path)->BTreeMap<String,(String,String,String)>{
 let mut out=BTreeMap::new();
 for l in fs::read_to_string(p).unwrap().lines(){
  let x:Vec<_>=l.split('\t').collect();assert_eq!(x.len(),4);
  assert!(out.insert(x[0].into(),(x[1].into(),x[2].into(),x[3].into())).is_none());
 }out
}
fn save(p:&Path,repo:&str,branch:&str,base:&str,commit:&str,m:&BTreeMap<String,(String,String,String)>){
 let mut s=format!("SV-AUX-PUB/1\nrepo\t{repo}\nbranch\t{branch}\nbase\t{base}\ncommit\t{commit}\ncomplete\ttrue\n");
 for(k,(mode,ty,sha))in m{s+=&format!("entry\t{k}\t{mode}\t{ty}\t{sha}\n");}
 assert!(!p.exists());fs::write(p,s).unwrap();
}
fn main(){
 let a:Vec<_>=std::env::args().collect();assert_eq!(a.len(),7);
 let d=Path::new(&a[1]);let tag=&a[2];
 let mut idx=BTreeMap::new();
 for l in fs::read_to_string(d.join(format!("indice-{tag}.tsv"))).unwrap().lines(){
  let(h,p)=l.split_once('\t').unwrap();let f:Vec<_>=h.split(' ').collect();
  assert_eq!(f.len(),3);assert_eq!(f[2],"0");
  assert!(idx.insert(p.to_string(),(f[0].to_string(),"blob".to_string(),f[1].to_string())).is_none());
 }
 let mut lab=read(&d.join(format!("lab-base-{tag}.tsv")));
 for p in fs::read_to_string(d.join(format!("paths-{tag}.txt"))).unwrap().lines(){
  let dest=if let Some(x)=p.strip_prefix("docs/calidad/Inventario-sv/sucesos/"){format!("laboratorio/tareas-watson/sucesos-sv/{x}")}
   else {format!("laboratorio/tareas-watson/{}",p.strip_prefix("docs/calidad/").unwrap())};
  lab.insert(dest,idx.get(p).expect("fichero local ausente").clone());
 }
 for(k,repo,branch,base,commit,m)in [
 ("lang","juantoniolloretegea/SV-lenguaje-de-computacion","main",&a[3],&a[4],idx),
 ("lab","juantoniolloretegea/SV-matematica-semantica-cuaternaria","lab/playground-sv-permanente",&a[5],&a[6],lab)]{
  save(&d.join(format!("{k}-esperado-{tag}.pub")),repo,branch,base,commit,&m);
  let obs=read(&d.join(format!("{k}-observado-{tag}.tsv")));
  save(&d.join(format!("{k}-observado-{tag}.pub")),repo,branch,base,commit,&obs);
  println!("{k}: esperados={} observados={}",m.len(),obs.len());
 }
}
