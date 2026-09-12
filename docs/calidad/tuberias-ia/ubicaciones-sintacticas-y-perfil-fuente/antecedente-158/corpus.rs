use std::fs;
fn main(){
 let args:Vec<_>=std::env::args().collect();
 for line in fs::read_to_string(&args[1]).unwrap().lines(){
  let (expect,path)=line.split_once('\t').unwrap();let source=fs::read_to_string(path).unwrap();
  let result=sv_core::compile_svp(&source,path);
  assert_eq!(result.is_ok(),expect=="OK","{path}");
  println!("{}\t{:?}",path,result);
 }
}
