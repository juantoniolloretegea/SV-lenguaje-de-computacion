//! Conciliación documental S39/r38; no ejecuta inferencias ni modifica recursos.
//! Uso: control PLAN_JSON DIRECTORIO_SALIDA FUENTE_RS.
//! El plan contiene originales fijados por commit, textos revisados y campos CSV.
//! El informe identifica esas bases y las huellas finales; la recuperación remota
//! se coteja separadamente contra el manifiesto producido.
use std::{collections::BTreeMap,fs,path::{Path,Component}};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
type R<T> = Result<T,Box<dyn std::error::Error>>;
#[derive(Clone)] struct Row{fields:Vec<String>,raw:String}
fn csv(s:&str)->R<Vec<Row>>{
 let b=s.as_bytes();let(mut i,mut start,mut q)=(0,0,false);let mut fs1=vec![];let mut f=vec![];let mut rows=vec![];
 while i<b.len(){let c=b[i];if q{if c==b'"'{if b.get(i+1)==Some(&b'"'){f.push(b'"');i+=1}else{q=false}}else{f.push(c)}}else{match c{
 b'"' if f.is_empty()=>q=true,b','=>fs1.push(String::from_utf8(std::mem::take(&mut f))?),
 b'\r' if b.get(i+1)==Some(&b'\n')=>{},b'\n'=>{fs1.push(String::from_utf8(std::mem::take(&mut f))?);rows.push(Row{fields:std::mem::take(&mut fs1),raw:s[start..=i].into()});start=i+1},_=>f.push(c)}}i+=1}
 if q{return Err("CSV con comillas abiertas".into())}if start<b.len(){fs1.push(String::from_utf8(f)?);rows.push(Row{fields:fs1,raw:s[start..].into()})}Ok(rows)
}
fn render(f:&[String])->String{f.iter().map(|s|format!("\"{}\"",s.replace('"',"\"\""))).collect::<Vec<_>>().join(",")+"\n"}
fn idx(r:&[Row],c:&str)->usize{r[0].fields.iter().position(|s|s==c).expect("Columna")}
fn find<'a>(r:&'a[Row],c:&str,id:&str)->&'a Row{let i=idx(r,c);let v:Vec<_>=r.iter().skip(1).filter(|r|r.fields.get(i).is_some_and(|v|v==id)).collect();assert_eq!(v.len(),1,"Identidad {id}");v[0]}
fn update(s:&str,col:&str,id:&str,p:&Value)->R<String>{
 let mut rows=csv(s)?;let i=idx(&rows,col);let hs=rows[0].fields.clone();let mut n=0;
 for r in rows.iter_mut().skip(1){if r.fields.get(i).is_some_and(|v|v==id){assert_eq!(r.fields.len(),hs.len());n+=1;for(k,v)in p.as_object().unwrap(){r.fields[hs.iter().position(|h|h==k).expect("Campo")]=v.as_str().unwrap().into()}r.raw=render(&r.fields)}}
 assert_eq!(n,1);Ok(rows.iter().map(|r|r.raw.as_str()).collect())
}
fn add(s:&str,f:&[String])->String{format!("{}{}{}",s,if s.ends_with('\n'){""}else{"\n"},render(f))}
fn val<'a>(j:&'a Value,k:&str)->&'a str{j[k].as_str().unwrap()}
fn hash(s:&[u8])->String{format!("{:x}",Sha256::digest(s))}
fn write(root:&Path,p:&str,s:&str)->R<()>{
 let path=Path::new(p);assert!(!path.is_absolute()&&!path.components().any(|c|matches!(c,Component::ParentDir)));
 let dest=root.join(path);let parent=dest.parent().unwrap();fs::create_dir_all(parent)?;assert!(parent.canonicalize()?.starts_with(root.canonicalize()?));assert!(!dest.exists(),"No sobrescribir");fs::write(dest,s)?;Ok(())
}
fn mermaid(s:&str)->Vec<String>{let fence=char::from(96).to_string().repeat(3);s.split(&(fence.clone()+"mermaid")).skip(1).map(|s|s.split(&fence).next().unwrap().into()).collect()}
fn main()->R<()>{
 let a:Vec<String>=std::env::args().collect();assert_eq!(a.len(),4);
 let p:Value=serde_json::from_slice(&fs::read(&a[1])?)?;let out=Path::new(&a[2]);fs::create_dir_all(out)?;
 let(mut docs,mut old,mut paths)=(BTreeMap::<String,String>::new(),BTreeMap::<String,String>::new(),BTreeMap::<String,String>::new());
 for d in p["files"].as_array().unwrap(){let k=val(d,"key").to_owned();let o=d["original"].as_str().unwrap_or("").to_owned();old.insert(k.clone(),o.clone());docs.insert(k.clone(),d["updated"].as_str().unwrap_or(&o).into());paths.insert(k,format!("{}/{}",val(d,"repo"),val(d,"path")));}
 let m=&docs["motor_index"];let at=m.find("| 04–05/10/2026 |").unwrap();let fixed=format!("{}\n{}",m[..at].trim_end_matches(['\r','\n']),&m[at..]);docs.insert("motor_index".into(),fixed);
 docs.insert("sucesoscsv".into(),update(&old["sucesoscsv"],"id","S39",&p["s39_patch"])?);
 let sr=csv(&docs["sucesoscsv"])?;let s39=find(&sr,"id","S39");let hs=&sr[0].fields;
 let mut block=format!("## S39 · {}\n\n",s39.fields[idx(&sr,"actividad")]);
 for(k,v)in hs.iter().zip(&s39.fields){if k!="id"&&k!="actividad"{block.push_str(&format!("**{k}:** {}\n\n",if v.is_empty(){"—"}else{v}));}}
 let mut md=old["sucesosmd"].clone();let start=md.find("## S39 ·").unwrap();let end=start+md[start..].find("## S40 ·").unwrap();md.replace_range(start..end,&block);docs.insert("sucesosmd".into(),md);
 let h=csv(&old["histo"])?;let hi=idx(&h,"id");let ri=idx(&h,"revision");
 assert_eq!(h.iter().skip(1).filter(|r|r.fields.get(hi).is_some_and(|v|v=="S39")).map(|r|r.fields[ri].parse::<u32>().unwrap()).max(),Some(37));
 assert_eq!(&h[0].fields[1..],hs.as_slice());let mut nh=vec!["38".into()];nh.extend(s39.fields.clone());docs.insert("histo".into(),add(&old["histo"],&nh));
 assert!(docs["histo"].starts_with(&old["histo"]));
 let mut t=old["tiquescsv"].clone();
 for k in ["tt14","tt17","tt18"]{let j:Value=serde_json::from_str(&docs[&format!("{k}j")])?;let patch=json!({"estado":j["estado"],"fecha_actualizacion_utc":j["fecha_actualizacion_utc"],"fecha_fin_utc":j["fecha_fin_utc"].as_str().unwrap_or(""),"objeto":j["objeto"],"resultado_actual":j["resultado_actual"],"evidencia":j["evidencias"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).collect::<Vec<_>>().join(" ; ")});t=update(&t,"id",val(&j,"id"),&patch)?}
 let j:Value=serde_json::from_str(&docs["tt19j"])?;let tr=csv(&t)?;let i=idx(&tr,"id");assert!(!tr.iter().any(|r|r.fields.get(i).is_some_and(|s|s=="TT-0019")));
 let nr=tr[0].fields.iter().map(|h|match h.as_str(){"ficha"=>"https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0019.md".into(),"evidencia"=>j["evidencias"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).collect::<Vec<_>>().join(" ; "),_=>j[h].as_str().unwrap_or("").into()}).collect::<Vec<_>>();docs.insert("tiquescsv".into(),add(&t,&nr));
 let rt=csv(&old["retpcsv"])?;let i=idx(&rt,"ID");assert!(!rt.iter().any(|r|r.fields.get(i).is_some_and(|v|v=="RETP-2026-277")));
 let rr=rt[0].fields.iter().map(|h|p["retp_row"][h].as_str().expect("Campo RETP").into()).collect::<Vec<_>>();docs.insert("retpcsv".into(),add(&old["retpcsv"],&rr));assert!(docs["retpcsv"].starts_with(&old["retpcsv"]));
 let ts=csv(&docs["tiquescsv"])?;
 for k in ["tt14","tt17","tt18","tt19"]{let j:Value=serde_json::from_str(&docs[&format!("{k}j")])?;let row=find(&ts,"id",val(&j,"id"));assert_eq!(j["revision_suceso"],38);for c in ["estado","fecha_alta_utc","fecha_actualizacion_utc","fecha_fin_utc","objeto","resultado_actual"]{assert_eq!(row.fields[idx(&ts,c)],j[c].as_str().unwrap_or(""),"Concordancia {k} {c}");}}
 for(k,c,allowed)in [("sucesoscsv","id",vec!["S39"]),("tiquescsv","id",vec!["TT-0014","TT-0017","TT-0018"])]{let prior=csv(&old[k])?;let after=csv(&docs[k])?;let i=idx(&prior,c);for r in prior.iter().skip(1).filter(|r|r.fields.len()>1){let id=&r.fields[i];if !allowed.contains(&id.as_str()){assert_eq!(find(&after,c,id).raw,r.raw,"No alterar {id}")}}}
 let t17:Value=serde_json::from_str(&docs["tt17j"])?;let t18:Value=serde_json::from_str(&docs["tt18j"])?;assert_eq!(t17["estado"],"finalizado");assert_eq!(t17["infraestructura"]["instancia_eliminada_acreditada"],false);assert_eq!(t18["estado"],"finalizado");assert_eq!(t18["infraestructura"]["instancia_eliminada_acreditada"],true);assert_eq!(t18["infraestructura"]["disco_eliminado_acreditado"],true);
 let q:Value=serde_json::from_str(&docs["tt19j"])?;let qs:Value=serde_json::from_str(&docs["qwen_estado"])?;assert_eq!(q["vector_parcial"],json!(["0","0","0","0","NE","NE","NE","NE","NE"]));assert_eq!(q["vector_parcial"],qs["preevaluacion"]["vector_parcial"]);assert!(q["kappa"].is_null()&&q["puntuacion_global"].is_null());assert_eq!(q["aptitud_examen_acreditada"],false);assert_eq!(q["servidor"]["ram_nominal_gb"],256);assert_eq!(q["servidor"]["gpu"],false);
 for k in ["acta001","acta003","acta004","tt14","tt17","tt18"]{let o=old[k].split("\n\n").skip(1).collect::<Vec<_>>().join("\n\n");let o=o.split("\n© 2026 Juan Antonio Lloret Egea.").next().unwrap().trim_end();assert!(docs[k].contains(o),"Antecedente {k}");}
 for(k,s)in &docs{assert_eq!(mermaid(s),mermaid(old.get(k).map(String::as_str).unwrap_or("")),"Diagramas {k}");if paths[k].ends_with(".json"){let _:Value=serde_json::from_str(s)?;}assert!(!s.contains('\u{fffd}'),"UTF8 {k}");}
 let mut manifest=vec![];for(k,s)in &docs{let path=&paths[k];write(out,path,s)?;manifest.push(json!({"ruta":path,"bytes":s.len(),"sha256":hash(s.as_bytes()),"clave_documento":k}));}
 let cp="SV-lenguaje-de-computacion/docs/calidad/tuberias-ia/continuacion-15-09-2026/CONTROL_CONCILIACION_S39_REV38_20261005.rs";let source=fs::read_to_string(&a[3])?;write(out,cp,&source)?;manifest.push(json!({"ruta":cp,"bytes":source.len(),"sha256":hash(source.as_bytes())}));
 let report=json!({"estado":"CONFORME","fecha_corte_documental_utc":p["fecha"],"revision_suceso":38,"bases_consultadas":p["bases"],"alcance":"Concordancia y preservacion documental; no reproduce inferencias, cotejos originales de imagen ni retirada de recursos","comprobaciones":["S39 revision 38 e historial concordantes","Historial previo y otros sucesos y tiques preservados","CSV y JSON de cuatro tiques concordantes","Thinking pendiente de retirada; Safeguard retirado","Qwen cuatro ceros y cinco NE sin puntuacion global","RETP-2026-277 nuevo sin alterar anteriores","Antecedentes y diagramas preservados","JSON validos y huellas de todos los documentos"],"archivos":manifest,"autoria_licencia":p["licencia"]});
 let rp="SV-lenguaje-de-computacion/docs/calidad/tuberias-ia/continuacion-15-09-2026/CONTROL_CONCILIACION_S39_REV38_20261005.json";let data=serde_json::to_string_pretty(&report)?+"\n";write(out,rp,&data)?;manifest.push(json!({"ruta":rp,"bytes":data.len(),"sha256":hash(data.as_bytes())}));
 fs::write(out.parent().unwrap().join("MANIFIESTO.json"),serde_json::to_string_pretty(&json!({"archivos":manifest,"autoria_licencia":p["licencia"]}))?+"\n")?;
 println!("CONFORME: {} archivos; S39 revision 38; cuatro tiques; RETP-2026-277; antecedentes y diagramas preservados.",manifest.len());Ok(())
}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
