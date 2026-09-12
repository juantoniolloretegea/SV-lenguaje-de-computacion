from pathlib import Path
import difflib
r=Path(__file__).resolve().parent
p=r/'candidata/rust/sv_core/src/frontend.rs';s=p.read_text()
def one(a,b):
 global s
 assert s.count(a)==1,(a[:100],s.count(a));s=s.replace(a,b)
one('    pub span: Option<std::ops::Range<usize>>,\n}', '    pub span: Option<std::ops::Range<usize>>,\n    pub foreign_surface: bool,\n}')
one('Self { error, span: None }','Self { error, span: None, foreign_surface: false }')
one('span: Some(source.len()..source.len()) });','span: Some(source.len()..source.len()), foreign_surface: false });')
one(')), span: Some(i..i+ch.len_utf8()) });',')), span: Some(i..i+ch.len_utf8()), foreign_surface: false });')
one('    pos: usize,\n    spans:', '    pos: usize,\n    error_site: std::cell::Cell<Option<(usize, bool)>>,\n    spans:')
one('            pos: 0,\n            source,','            pos: 0,\n            error_site: std::cell::Cell::new(None),\n            source,')
a='''            // EOF tiene posición inequívoca. Los demás fallos sintácticos
            // permanecen sin rango hasta migrar su emisor; no se adivina.
            let span = if matches!(error, FrontendError::UnexpectedEnd) {
                Some(self.source.len()..self.source.len())
            } else { None };
            return Err(FrontendFailure { error, span });'''
b='''            // Sólo el emisor que conoce la posición puede fijarla.
            let site = self.error_site.get();
            let span = if matches!(error, FrontendError::UnexpectedEnd) {
                Some(self.source.len()..self.source.len())
            } else { site.and_then(|(index, _)| self.spans.get(index).cloned()) };
            return Err(FrontendFailure { error, span,
                foreign_surface: site.is_some_and(|(_, foreign)| foreign) });'''
one(a,b)
# Despacho inicial: peek_word no consume el elemento.
a='other => return Err(FrontendError::Unsupported(other.to_owned())),'
i=s.index('    fn parse_declarations(');j=s.index('    fn parse_codomain(',i)
section=s[i:j];assert section.count(a)==1
s=s[:i]+section.replace(a,'other => return Err(self.located_error(FrontendError::Unsupported(other.to_owned()), self.pos, true)),')+s[j:]
one('''    fn take_tri(&mut self) -> Result<Tri, FrontendError> {
        let label = self.take_raw_word()?;''','''    fn take_tri(&mut self) -> Result<Tri, FrontendError> {
        let position = self.pos;
        let label = self.take_raw_word()?;''')
one('_ => Err(FrontendError::InvalidTri(label)),','_ => Err(self.located_error(FrontendError::InvalidTri(label), position, true)),')
# En cada primitiva los errores se producen antes de avanzar, salvo el natural.
start=s.index('    fn peek_word(');end=s.index('    fn at_sym(',start)
section=s[start:end]
section=section.replace('Err(FrontendError::UnexpectedToken(format!("{other:?}")))','Err(self.located_error(FrontendError::UnexpectedToken(format!("{other:?}")), self.pos, false))')
section=section.replace('''Err(FrontendError::UnexpectedToken(format!(
                        "palabra protegida donde se esperaba identificador: {raw}"
                    )))''','''Err(self.located_error(FrontendError::UnexpectedToken(format!(
                        "palabra protegida donde se esperaba identificador: {raw}"
                    )), self.pos, true))''')
section=section.replace('''                self.pos += 1;
                Nat::from_decimal(&value).map_err(|_| FrontendError::InvalidNatural(value))''','''                let position = self.pos;
                self.pos += 1;
                Nat::from_decimal(&value).map_err(|_| self.located_error(FrontendError::InvalidNatural(value), position, false))''')
for msg,foreign in [('esperado {expected}, recibido {got}','true'),('esperado {expected}, recibido {other:?}','false'),('esperado ->, recibido {other:?}','false')]:
 old='Err(FrontendError::UnexpectedToken(format!(\n                "'+msg+'"\n            )))'
 new='Err(self.located_error(FrontendError::UnexpectedToken(format!(\n                "'+msg+'"\n            )), self.pos, '+foreign+'))'
 assert section.count(old)==1,msg;section=section.replace(old,new)
s=s[:start]+section+s[end:]
one('    fn peek_word(&self)', '''    /// Conserva causa y posición en el punto que rechaza, sin interpretar prosa.
    fn located_error(&self, error: FrontendError, position: usize, profile_context: bool) -> FrontendError {
        let foreign = profile_context && matches!(self.tokens.get(position),
            Some(Token::Word(_, STATUS_FOREIGN_CONTEXTUAL | STATUS_FOREIGN_PROTECTED)));
        self.error_site.set(Some((position, foreign)));
        error
    }

    fn peek_word(&self)''')
p.write_text(s)
p=r/'candidata/rust/sv_core/src/compiler_diagnostics.rs';s=p.read_text()
one('COMPILER-DIAGNOSTICS/1','COMPILER-DIAGNOSTICS/2') if False else None
s=s.replace('COMPILER-DIAGNOSTICS/1','COMPILER-DIAGNOSTICS/2')
one('pub enum FrontendCause { UnexpectedEnd,','pub enum FrontendCause { ForeignSurface, UnexpectedEnd,')
one('            Self::Frontend(FrontendCause::UnexpectedEnd) => "CD.UNEXPECTED_END",','            Self::Frontend(FrontendCause::ForeignSurface) => "CD.FOREIGN_SURFACE",\n            Self::Frontend(FrontendCause::UnexpectedEnd) => "CD.UNEXPECTED_END",')
one('            Self::Frontend(FrontendCause::UnexpectedEnd) => (','            Self::Frontend(FrontendCause::ForeignSurface) => ("La grafía no está admitida en esta posición bajo el perfil fuente seleccionado.", "The spelling is not allowed at this position under the selected source profile."),\n            Self::Frontend(FrontendCause::UnexpectedEnd) => (')
one('        let cause = match &e.error {','        let cause = if e.foreign_surface { FrontendCause::ForeignSurface } else { match &e.error {')
one('            FrontendError::InvalidTri(_) => FrontendCause::InvalidTri,\n        };','            FrontendError::InvalidTri(_) => FrontendCause::InvalidTri,\n        } };')
p.write_text(s)
patch=''
for name in ['frontend.rs','compiler_diagnostics.rs']:
 path='rust/sv_core/src/'+name
 patch+=''.join(difflib.unified_diff((r/'base'/path).read_text().splitlines(True),(r/'candidata'/path).read_text().splitlines(True),fromfile='a/'+path,tofile='b/'+path))
(r/'CAMBIO_INCREMENTAL.patch').write_text(patch)
print('Dos archivos modificados; cambio conservado')
