//! Editor acotado de candidatos documentales; R08 valida el conjunto antes de usarlo.
#[path = "../../../../r08/auxiliar.rs"]
mod auxiliar;
use std::{fs, path::Path};
use sv_bis_i0205::json::{self,J};
type R<T> = Result<T,String>;
fn txt(b:&[u8])->R<&str>{std::str::from_utf8(b).map_err(|e|e.to_string())}
fn field<'a>(j:&'a J,k:&str)->R<&'a str>{j.field(k).text().ok_or_else(||format!("campo: {k}"))}
fn row(v:&[String])->String{v.iter().map(|s|format!("\"{}\"",s.replace('"',"\"\""))).collect::<Vec<_>>().join(",")+"\n"}
fn main_run(root:&Path,input:&Path,out:&Path)->R<()>{
    let j=json::decode(&auxiliar::read(input)?).map_err(|e|e.to_string())?;
    let mut b:Vec<Vec<u8>>=Vec::new();
    for n in auxiliar::FILES {
        let p=if n.starts_with("REGISTRO_"){root.join("docs/calidad").join(n)}else{root.join("docs/calidad/Inventario-sv/sucesos").join(n)};
        b.push(auxiliar::read(&p)?);
    }
    let table=auxiliar::csv(&b[0])?;let header=&table[0];
    let mut target=table.iter().filter(|r|r.first().is_some_and(|s|s=="S26"));
    let mut current=target.next().ok_or("S26 ausente")?.clone();if target.next().is_some(){return Err("S26 duplicado".into());}
    for (k,v) in j.field("suceso").pairs().ok_or("suceso")? {
        let pos=header.iter().position(|x|x==k).ok_or("columna desconocida")?;
        current[pos]=v.text().ok_or("valor no textual")?.into();
    }
    if current[0]!="S26"{return Err("identidad".into());}
    let get=|k:&str|->R<&str>{Ok(&current[header.iter().position(|x|x==k).ok_or("columna")?])};
    // Sólo sustituye la fila física S26. Este perfil editorial rechaza multilinea.
    if current.iter().any(|s|s.contains(['\n','\r'])){return Err("fuera de perfil multilinea".into());}
    let old=txt(&b[0])?;let marker="\nS26,";let start=old.find(marker).ok_or("fila S26 fisica")?+1;
    let end=start+old[start..].find('\n').ok_or("fin fila")?+1;
    let mut a=b.clone();a[0]=[&old[..start],&row(&current),&old[end..]].concat().into_bytes();
    let mut hist=vec![field(&j,"revision")?.into()];hist.extend(current.clone());a[1].extend(row(&hist).as_bytes());
    let md=txt(&b[2])?;let start=md.find("## S26 · ").ok_or("seccion S26")?;
    let end=md[start+10..].find("\n## ").map(|n|start+10+n+1).unwrap_or(md.len());
    let mut section=format!("## S26 · {}\n\n",get("actividad")?);
    for (k,v) in header.iter().zip(&current){if k=="id"||k=="actividad"{continue;}section+=&format!("**{k}:** {}\n\n",if v.is_empty(){"—"}else{v});}
    a[2]=[&md[..start],&section,&md[end..]].concat().into_bytes();
    let retp=field(&j,"retp")?;
    let rt=auxiliar::csv(&b[3])?;let mut rr=Vec::new();
    for k in &rt[0]{rr.push(match k.as_str(){
        "ID"=>format!("RETP-2026-{retp}"),"Frente_Fase"=>"S26".into(),"Resumen_Cambio"=>get("resultado")?.into(),
        "Evidencia"=>get("verificacion")?.into(),"Objecion_Adversarial"=>get("observaciones")?.into(),"Decision"=>get("siguiente_accion")?.into(),"Estado"=>get("estado")?.into(),
        other=>field(j.field("retp_campos"),other)?.into(),
    });}
    a[3].extend(row(&rr).as_bytes());
    let suffix=format!("\n<a id=\"retp-{retp}\"></a>\n\n### RETP-2026-{retp} · S26 · R06 LOCAL01 · Recepción y consumo\n\n{}\n\n{}\n\n{}\n\n{}\n\n[Evidencia]({}).\n",get("resultado")?,get("verificacion")?,get("observaciones")?,get("siguiente_accion")?,get("evidencias")?);
    a[4].extend(suffix.as_bytes());
    fs::create_dir(out).map_err(|e|e.to_string())?;
    for (name,bytes) in auxiliar::FILES.iter().zip(a) {fs::write(out.join(name),bytes).map_err(|e|e.to_string())?;}
    Ok(())
}
fn main(){let a:Vec<_>=std::env::args_os().collect();if a.len()!=4{eprintln!("uso: registrar ROOT ENTRADA_JSON CANDIDATO_NUEVO");std::process::exit(2);}if let Err(e)=main_run(Path::new(&a[1]),Path::new(&a[2]),Path::new(&a[3])){eprintln!("{e}");std::process::exit(1);}}
