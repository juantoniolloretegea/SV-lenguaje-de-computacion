from pathlib import Path
s=Path('tmp/diagnosticos-158/candidata/rust/sv_core/src')
p=s/'frontend.rs';t=p.read_text()
# Add token spans at emission; no retokenization and no IR modification.
t=t.replace('let tokens = tokenize(source, profile)?;\n    Parser::new(tokens, source, source_file).parse()', 'compile_with_origins(source, source_file, profile).map(|p| p.program).map_err(|e| e.error)')
assert 'Parser::new(tokens, source, source_file).parse()' not in t
start=t.index('fn tokenize<');end=t.index('\nstruct Parser',start)
lex=t[start:end]
lex=lex.replace('Result<Vec<Token<\'_>>, FrontendError>', 'Result<(Vec<Token<\'_>>, Vec<std::ops::Range<usize>>), FrontendFailure>')
# Signature may have explicit lifetime: checked below.
lex=lex.replace('Result<Vec<Token<\'a>>, FrontendError>', 'Result<(Vec<Token<\'a>>, Vec<std::ops::Range<usize>>), FrontendFailure>')
lex=lex.replace('let mut out = Vec::new();','let mut out = Vec::new();\n    let mut spans = Vec::new();')
lex=lex.replace('out.push(Token::Arrow);','out.push(Token::Arrow);\n            spans.push(i..i+2);')
lex=lex.replace('return Err(FrontendError::UnexpectedEnd);','return Err(FrontendFailure { error: FrontendError::UnexpectedEnd, span: Some(source.len()..source.len()) });')
lex=lex.replace('out.push(Token::Text(text.to_owned()));','out.push(Token::Text(text.to_owned()));\n            spans.push(start-1..i+1);')
lex=lex.replace('));\n            continue;\n        }\n\n        let ch', '));\n            spans.push(start..i);\n            continue;\n        }\n\n        let ch')
lex=lex.replace('out.push(Token::Word(&source[start..i], status));','out.push(Token::Word(&source[start..i], status));\n            spans.push(start..i);')
lex=lex.replace('out.push(Token::Sym(ch));','out.push(Token::Sym(ch));\n            spans.push(i..i+1);')
lex=lex.replace('return Err(FrontendError::UnexpectedToken(format!(\n            "carácter léxico no admitido U+{:04X}",\n            ch as u32\n        )));','return Err(FrontendFailure { error: FrontendError::UnexpectedToken(format!(\n            "carácter léxico no admitido U+{:04X}", ch as u32\n        )), span: Some(i..i+ch.len_utf8()) });')
lex=lex.replace('out.push(Token::Eof);\n    Ok(out)','out.push(Token::Eof);\n    spans.push(source.len()..source.len());\n    Ok((out, spans))')
t=t[:start]+lex+t[end:]
insert='''
/// Sidecar de procedencia; nunca se serializa dentro de la IR.
pub(crate) struct ParsedUnit {
    pub program: IrProgram,
    pub objects: Vec<std::ops::Range<usize>>,
    pub operations: Vec<std::ops::Range<usize>>,
}
pub(crate) struct FrontendFailure {
    pub error: FrontendError,
    pub span: Option<std::ops::Range<usize>>,
}
impl From<FrontendError> for FrontendFailure {
    fn from(error: FrontendError) -> Self { Self { error, span: None } }
}
pub(crate) fn compile_with_origins(source: &str, source_file: &str, profile: SourceProfile)
    -> Result<ParsedUnit, FrontendFailure>
{
    let (tokens, spans) = tokenize(source, profile)?;
    Parser::new(tokens, spans, source, source_file).parse()
}
'''
t=t[:start]+insert+t[start:]
t=t.replace('    pos: usize,\n    source:', '    pos: usize,\n    spans: Vec<std::ops::Range<usize>>,\n    object_origins: Vec<std::ops::Range<usize>>,\n    operation_origins: Vec<std::ops::Range<usize>>,\n    source:')
t=t.replace("fn new(tokens: Vec<Token<'a>>, source:","fn new(tokens: Vec<Token<'a>>, spans: Vec<std::ops::Range<usize>>, source:")
t=t.replace('            tokens,\n            pos: 0,','            tokens,\n            spans,\n            object_origins: Vec::new(),\n            operation_origins: Vec::new(),\n            pos: 0,')
a=t.index('    fn parse(mut self)');b=t.index('    fn parse_codomain',a)
old=t[a:b];loop=old[old.index('        while'):old.index('        Ok(construction')]
loop=loop.replace('            match self.peek_word()? {','            let start = self.spans[self.pos].start;\n            let no = self.objects.len();\n            let np = self.operations.len();\n            match self.peek_word()? {')
loop=loop.replace('            }\n        }','            }\n            let span = start..self.spans[self.pos - 1].end;\n            self.object_origins.extend((no..self.objects.len()).map(|_| span.clone()));\n            self.operation_origins.extend((np..self.operations.len()).map(|_| span.clone()));\n        }')
new='''    fn parse(mut self) -> Result<ParsedUnit, FrontendFailure> {
        if let Err(error) = self.parse_declarations() {
            // EOF tiene posición inequívoca. Los demás fallos sintácticos
            // permanecen sin rango hasta migrar su emisor; no se adivina.
            let span = if matches!(error, FrontendError::UnexpectedEnd) {
                Some(self.source.len()..self.source.len())
            } else { None };
            return Err(FrontendFailure { error, span });
        }
        Ok(ParsedUnit {
            program: construction::program(self.source_file, sha256_hex(self.source.as_bytes()), self.objects, self.operations),
            objects: self.object_origins, operations: self.operation_origins,
        })
    }
    fn parse_declarations(&mut self) -> Result<(), FrontendError> {
'''+loop+'''        Ok(())
    }

'''
t=t[:a]+new+t[b:];p.write_text(t)
# Validator: convert only selected emitters, preserve exact legacy text/order.
p=s/'wellformed.rs';t=p.read_text();t='use crate::compiler_diagnostics::{ValidationFailure, Cause, DeclarationId};\n'+t
start=t.index('    fn new(program:');end=t.index('\n    fn object(',start)
t=t[:start]+'''    fn new(program: &'a IrProgram) -> Result<Self, ValidationFailure> {
        let mut by_name = BTreeMap::new();
        let mut locations = BTreeMap::new();
        for (id, name, symbol) in program.objects().iter().enumerate()
            .map(|(i,o)| (DeclarationId::Object(i), o.name(), Symbol::Object(o.kind())))
            .chain(program.operations().iter().enumerate()
                .map(|(i,o)| (DeclarationId::Operation(i), o.name(), Symbol::Operation(o.kind()))))
        {
            if let Some(first) = locations.insert(name, id) {
                return Err(ValidationFailure::known(format!("identificador duplicado: {name}"),
                    Cause::DuplicateIdentifier { name: name.to_owned(), first, second: id }));
            }
            by_name.insert(name, symbol);
        }
        Ok(Self { by_name })
    }
''' + t[end:]
t=t.replace('pub(crate) fn validate_program(program: &IrProgram) -> Result<(), String>', 'pub(crate) fn validate_program(program: &IrProgram) -> Result<(), ValidationFailure>')
t=t.replace("fn validate_object(name: &str, kind: &IrObjectKind, symbols: &Symbols<'_>) -> Result<(), String>","fn validate_object(name: &str, kind: &IrObjectKind, symbols: &Symbols<'_>) -> Result<(), ValidationFailure>")
# Convert all direct Err expressions in these two functions with balanced parentheses,
# including quoted strings. This is a source edit, never a runtime cause parser.
a=t.index('pub(crate) fn validate_program');b=t.index('\nfn validate_operation',a)
part=t[a:b];pos=0
while True:
 i=part.find('Err(',pos)
 if i<0:break
 j=i+4;depth=1;quoted=False;escape=False
 while depth:
  c=part[j]
  if quoted:
   if escape:escape=False
   elif c=='\\':escape=True
   elif c=='"':quoted=False
  elif c=='"':quoted=True
  elif c=='(':depth+=1
  elif c==')':depth-=1
  j+=1
 part=part[:j-1]+'.into()'+part[j-1:];pos=j+7
# exact source substitutions carry typed values from original branch
part=part.replace('duplicates.into_iter()', 'duplicates.iter().copied()')
old='''Err(format!(
                    "E004 (InvalidCodomain): codomain {name} vacío"
                ).into())'''
new='''Err(ValidationFailure::known(format!(
                    "E004 (InvalidCodomain): codomain {name} vacío"
                ), Cause::EmptyCodomain { name: name.to_owned() }))'''
assert old in part;part=part.replace(old,new)
old='''Err(format!(
                    "E004 (InvalidCodomain): codomain {name} repite: {}",
                    duplicates.iter().copied().collect::<Vec<_>>().join(", ")
                ).into())'''
new='''Err(ValidationFailure::known(format!(
                    "E004 (InvalidCodomain): codomain {name} repite: {}",
                    duplicates.iter().copied().collect::<Vec<_>>().join(", ")
                ), Cause::DuplicateCodomainMembers { name: name.to_owned(), duplicates: duplicates.iter().map(|s| s.to_string()).collect() }))'''
assert old in part;part=part.replace(old,new)
old='''Err(format!(
                    "E115 (InvalidOutputSemantics): OutputSemantics {}: repetidas=[{}]",
                    object.name(),
                    duplicates.iter().copied().collect::<Vec<_>>().join(", "),
                ).into())'''
new='''Err(ValidationFailure::known(format!(
                    "E115 (InvalidOutputSemantics): OutputSemantics {}: repetidas=[{}]",
                    object.name(),
                    duplicates.iter().copied().collect::<Vec<_>>().join(", "),
                ), Cause::OutputSemanticsKeys { cell: None, semantics: object.name().to_owned(), codomain: None,
                    duplicates: duplicates.iter().map(|s| s.to_string()).collect(), missing: Vec::new(), extra: Vec::new() }))'''
assert old in part;part=part.replace(old,new)
old='''Err(format!(
                    "E115 (InvalidOutputSemantics): CellSpec {name}, OutputSemantics {semantics}, Codomain {codomain}: repetidas=[{}]; ausentes=[{}]; ajenas=[{}]",
                    duplicates.iter().copied().collect::<Vec<_>>().join(", "),
                    missing.join(", "),
                    extra.join(", "),
                ).into())'''
new='''Err(ValidationFailure::known(format!(
                    "E115 (InvalidOutputSemantics): CellSpec {name}, OutputSemantics {semantics}, Codomain {codomain}: repetidas=[{}]; ausentes=[{}]; ajenas=[{}]",
                    duplicates.iter().copied().collect::<Vec<_>>().join(", "),
                    missing.join(", "),
                    extra.join(", "),
                ), Cause::OutputSemanticsKeys { cell: Some(name.to_owned()), semantics: semantics.to_owned(), codomain: Some(codomain.to_owned()),
                    duplicates: duplicates.iter().map(|s| s.to_string()).collect(), missing: missing.iter().map(|s| s.to_string()).collect(), extra: extra.iter().map(|s| s.to_string()).collect() }))'''
assert old in part;part=part.replace(old,new)
t=t[:a]+part+t[b:];p.write_text(t)
# Route original entrypoints through same implementation, discarding only sidecar.
p=s/'lib.rs';t=p.read_text().replace('pub mod audit_diagnostics;','pub mod audit_diagnostics;\npub mod compiler_diagnostics;')
a=t.index('    let program = frontend::compile_svp(source, source_file)?;');b=t.index('\n}\n',a)
t=t[:a]+'''    compiler_diagnostics::compile(source, source_file, SourceProfile::En).map_err(|e| e.into_legacy())'''+t[b:]
a=t.index('    let program = frontend::compile_svp_with_profile(source, source_file, profile)?;');b=t.index('\n}\n',a)
t=t[:a]+'''    compiler_diagnostics::compile(source, source_file, profile).map_err(|e| e.into_legacy())'''+t[b:]
a=t.index('    if units.len() < 2 {',t.index('pub fn compile_svp_assembly'));b=t.index('\n}\n',a)
t=t[:a]+'''    compiler_diagnostics::compile_assembly(units).map_err(|e| e.into_legacy())'''+t[b:];p.write_text(t)
print('Frontend, validador y entrada común modificados')
