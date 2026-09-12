from pathlib import Path
import json,hashlib,difflib
r=Path(__file__).resolve().parent
for n,h in json.loads((r/'COMPROMISO_PREVIO.json').read_text())['archivos'].items():assert hashlib.sha256((r/n).read_bytes()).hexdigest()==h
p=r/'candidata/rust/sv_core/src/frontend.rs';old=p.read_text();s=old
def one(a,b):
 global s
 assert s.count(a)==1,(a,s.count(a));s=s.replace(a,b)
one('error_site: std::cell::Cell<Option<(usize, bool)>>','error_site: std::cell::Cell<Option<(usize, usize, bool)>>')
one('''            } else { site.and_then(|(index, _)| self.spans.get(index).cloned()) };''','''            } else {
                site.and_then(|(first, last, _)| self.spans.get(first).zip(self.spans.get(last))
                    .map(|(a, b)| a.start..b.end))
            };''')
one('foreign_surface: site.is_some_and(|(_, foreign)| foreign)','foreign_surface: site.is_some_and(|(_, _, foreign)| foreign)')
one('''        self.error_site.set(Some((position, foreign)));''','''        self.error_site.set(Some((position, position, foreign)));''')
one('''    fn peek_word(&self) -> Result<&str, FrontendError> {''','''    /// Intervalo inclusivo de tokens conocido por el emisor del conjunto.
    fn located_range_error(&self, error: FrontendError, first: usize, last: usize) -> FrontendError {
        self.error_site.set(Some((first, last, false)));
        error
    }

    fn peek_word(&self) -> Result<&str, FrontendError> {''')
a=s.index('    fn parse_admissibility_spec');b=s.index('    fn parse_ternarizer',a);part=s[a:b]
needle='''        self.word("states")?;
        self.sym(':')?;
        self.sym('{')?;'''
assert part.count(needle)==1
part=part.replace(needle,'''        self.word("states")?;
        self.sym(':')?;
        let states_first = self.pos;
        self.sym('{')?;''')
needle='''        self.sym('}')?;
        self.sym(';')?;
        if states.len() != 3 {
            return Err(FrontendError::InvalidAdmissibilityState(format!(
                "{} estados",
                states.len()
            )));
        }'''
assert part.count(needle)==1
part=part.replace(needle,'''        let states_last = self.pos;
        self.sym('}')?;
        self.sym(';')?;
        if states.len() != 3 {
            return Err(self.located_range_error(FrontendError::InvalidAdmissibilityState(format!(
                "{} estados",
                states.len()
            )), states_first, states_last));
        }''')
s=s[:a]+part+s[b:];p.write_text(s)
(r/'CAMBIO_INCREMENTAL.patch').write_text(''.join(difflib.unified_diff(old.splitlines(True),s.splitlines(True),fromfile='a/rust/sv_core/src/frontend.rs',tofile='b/rust/sv_core/src/frontend.rs')))
print('Intervalo inclusivo de tokens para la colección; un archivo')
