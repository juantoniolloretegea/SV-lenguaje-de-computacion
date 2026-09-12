from pathlib import Path
import shutil
r=Path(__file__).resolve().parent;old=r.parent/'continuacion-159'
s=(old/'ejecutar.py').read_text()
s=s.replace("lit(c['cause']),span]", "lit(c['cause']),span,lit(sha(c['source'].encode()))]")
s=s.replace('span:Option<std::ops::Range<usize>>) {','span:Option<std::ops::Range<usize>>, digest:&str) {')
s=s.replace('assert_eq!(report.sources()[0].byte_len,source.len());','''assert_eq!(report.sources()[0].byte_len,source.len());
   assert_eq!(report.sources()[0].sha256,digest,"{id}: huella original");
   assert_eq!(report.sources()[0].profile,p);
   assert_eq!(report.stage(),d::Stage::Frontend);
   if let Some(ref bytes)=span {assert!(source.is_char_boundary(bytes.start) && source.is_char_boundary(bytes.end));}''')
s=s.replace('assert_eq!(a.messages().len(),2);', '''let index=if reverse{1}else{0};
    assert_eq!(a.sources()[index].sha256,digest);assert_eq!(a.sources()[index].profile,p);
    assert_eq!(a.sources()[index].byte_len,source.len());
    assert_eq!(a.messages().len(),2);''')
s=s.replace('for (id,s,p,cause,span) in [ROWS] {check(id,s,p,cause,span);}', 'for (id,s,p,cause,span,digest) in [ROWS] {check(id,s,p,cause,span,digest);}')
s=s.replace("'fuentes_focales':len(cases)","'fuentes_nuevas':54,'fuentes_heredadas_159':40,'fuentes_focales':len(cases)")
(r/'ejecutar.py').write_text(s)
s=(old/'complemento.py').read_text()
start=s.index("source=source.replace('fn main(){'");end=s.index("(r/'catalogo.rs').write_text(source)",start)
s=s[:start]+s[end:]
s=s.replace('ROOT_FOREIGN_en: causa','rel_REPEAT_en: intervalo').replace('ROOT_FOREIGN_en: intervalo','rel_REPEAT_en: intervalo')
s=s.replace("'emisores_compuestos_no_migrados':{'casos':2,'causa':'CD.UNEXPECTED_TOKEN','intervalo':None}","'cardinalidad_no_migrada':{'casos':2,'causa':'CD.INVALID_ADMISSIBILITY_STATE','intervalo':None,'ubicacion_comprobacion':'batería focal principal'}")
s=s.replace("needle='self.error_site.set(Some((position, foreign)));';assert old.count(needle)==1", "needle='            let field_position = self.pos;\\n            let field = self.take_raw_word()?;';assert old.count(needle)==2")
s=s.replace("new=old.replace(needle,'self.error_site.set(Some((position.saturating_add(1), foreign)));')", "new=old.replace(needle,'            let field = self.take_raw_word()?;\\n            let field_position = self.pos;')")
s=s.replace("'base_sin_causa_detectada':True", "'base_sin_intervalo_detectada':True")
(r/'complemento.py').write_text(s)
shutil.copyfile(old/'reproducir.py',r/'reproducir.py')
