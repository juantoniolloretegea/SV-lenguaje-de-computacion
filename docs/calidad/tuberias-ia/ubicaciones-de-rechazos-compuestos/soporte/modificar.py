from pathlib import Path
import json,hashlib,difflib
r=Path(__file__).resolve().parent
for n,h in json.loads((r/'COMPROMISO_PREVIO.json').read_text())['archivos'].items():assert hashlib.sha256((r/n).read_bytes()).hexdigest()==h
p=r/'candidata/rust/sv_core/src/frontend.rs';old=p.read_text();s=old
def one(a,b):
 global s
 assert s.count(a)==1,(a,s.count(a));s=s.replace(a,b)
one('''            let label = self.take_raw_word()?;
            let state = AdmissibilityState::try_from(label.as_str())
                .map_err(|_| FrontendError::InvalidAdmissibilityState(label))?;''','''            let label_position = self.pos;
            let label = self.take_raw_word()?;
            let state = AdmissibilityState::try_from(label.as_str())
                .map_err(|_| self.located_error(FrontendError::InvalidAdmissibilityState(label), label_position, true))?;''')
for start,end,typ,field in [('parse_semantic_relation','parse_pattern','SemanticRelation','table'),('parse_pattern','parse_graph','Pattern','arity')]:
 a=s.index('    fn '+start);b=s.index('    fn '+end,a);part=s[a:b]
 part=part.replace('            let field = self.take_raw_word()?;','            let field_position = self.pos;\n            let field = self.take_raw_word()?;')
 for message in ['campo opcional repetido: '+field,'campo opcional fuera de orden: '+field,'campo opcional repetido: constraints']:
  x='return Err(FrontendError::UnexpectedToken(\n                    format!("'+typ+' {name}: '+message+'")))'
  y='return Err(self.located_error(FrontendError::UnexpectedToken(\n                    format!("'+typ+' {name}: '+message+'")), field_position, true))'
  assert part.count(x)==1;part=part.replace(x,y)
 part=part.replace('return Err(FrontendError::Unsupported(other.to_owned()))','return Err(self.located_error(FrontendError::Unsupported(other.to_owned()), field_position, true))')
 s=s[:a]+part+s[b:]
one('''        let (first_raw, first_status) = self.take_dispatch_word()?;''','''        let first_position = self.pos;
        let (first_raw, first_status) = self.take_dispatch_word()?;''')
one('''                let variant = self.take_raw_word()?;''','''                let variant_position = self.pos;
                let variant = self.take_raw_word()?;''')
one('''                    other => return Err(FrontendError::Unsupported(other.to_owned())),''','''                    other => return Err(self.located_error(FrontendError::Unsupported(other.to_owned()), variant_position, true)),''')
one('''                    return Err(FrontendError::UnexpectedToken(format!(
                        "palabra protegida donde se esperaba identificador: {first_raw}"
                    )));''','''                    return Err(self.located_error(FrontendError::UnexpectedToken(format!(
                        "palabra protegida donde se esperaba identificador: {first_raw}"
                    )), first_position, true));''')
one('''    fn query_context(&mut self) -> Result<IrQueryContext, FrontendError> {
        let variant = self.take_raw_word()?;''','''    fn query_context(&mut self) -> Result<IrQueryContext, FrontendError> {
        let variant_position = self.pos;
        let variant = self.take_raw_word()?;''')
one('''            other => return Err(FrontendError::Unsupported(other.to_owned())),''','''            other => return Err(self.located_error(FrontendError::Unsupported(other.to_owned()), variant_position, true)),''')
assert s.count('self.located_error(')-old.count('self.located_error(')==12
p.write_text(s)
(r/'CAMBIO_INCREMENTAL.patch').write_text(''.join(difflib.unified_diff(old.splitlines(True),s.splitlines(True),fromfile='a/rust/sv_core/src/frontend.rs',tofile='b/rust/sv_core/src/frontend.rs')))
print('12 emisores, un archivo')
