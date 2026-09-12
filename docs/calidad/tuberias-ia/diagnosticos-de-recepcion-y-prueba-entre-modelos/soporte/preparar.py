from pathlib import Path
import json,hashlib,base64,shutil,datetime
r=Path('tmp/continuacion-157');o=Path('entregas/diagnosticos-de-recepcion-y-prueba-entre-modelos')
base=json.loads(Path('entregas/recepcion-gobernada-y-comprobacion-observada/CANDIDATA_FUENTES.json').read_text())
for f in base['archivos']:
 data=base64.b64decode(f['base64']);assert hashlib.sha256(data).hexdigest()==f['sha256']
 dest=r/'candidata'/f['ruta'];dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data)
(r/'COMPROMISO_PLAN.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'sha256':hashlib.sha256((o/'PLAN_PREVIO.md').read_bytes()).hexdigest()},indent=2)+'\n')
for p in Path('entregas/recepcion-gobernada-y-comprobacion-observada/clientes').glob('*.rs'):shutil.copyfile(p,o/'clientes'/p.name)
rows=[
 ('EmptyAct','RG.EMPTY_ACT','El acto recibido está vacío.','The received act is empty.'),
 ('EmptyIssuer','RG.EMPTY_ISSUER','La declaración de emisor está vacía.','The issuer declaration is empty.'),
 ('EmptyVersion','RG.EMPTY_VERSION','La versión declarada está vacía.','The declared version is empty.'),
 ('ActTooLarge','RG.ACT_TOO_LARGE','El acto supera el límite de bytes.','The act exceeds the byte limit.'),
 ('IssuerTooLarge','RG.ISSUER_TOO_LARGE','La declaración de emisor supera el límite de bytes.','The issuer declaration exceeds the byte limit.'),
 ('VersionTooLarge','RG.VERSION_TOO_LARGE','La versión declarada supera el límite de bytes.','The declared version exceeds the byte limit.'),
 ('IssuerMismatch','RG.ISSUER_MISMATCH','El emisor declarado no coincide con el instalado.','The declared issuer does not match the installed issuer.'),
 ('VersionMismatch','RG.VERSION_MISMATCH','La versión declarada no coincide con la instalada.','The declared version does not match the installed version.'),
 ('DeclarationsMismatch','RG.DECLARATIONS_MISMATCH','El emisor y la versión declarados no coinciden con los instalados.','The declared issuer and version do not match the installed declarations.'),
 ('ActMismatch','RG.ACT_MISMATCH','El acto recibido difiere del acto instalado.','The received act differs from the installed act.'),
 ('AlreadyAttempted','RG.ALREADY_ATTEMPTED','El intento de génesis de esta preparación ya se ha consumido.','The genesis attempt for this preparation has already been consumed.'),
 ('GenesisRejected','RG.GENESIS_REJECTED','T-0 ha rechazado el plan; se conserva su error tipado.','T-0 rejected the plan; its typed error is retained.'),
 ('ForeignBinding','OC.FOREIGN_BINDING','La obligación o su aplicabilidad no pertenece al vínculo exigido.','The requirement or its applicability does not belong to the required binding.'),
 ('CoreRequirement','OC.CORE_REQUIREMENT','La igualdad de bytes no puede verificar esta obligación nuclear.','Byte equality cannot verify this core requirement.'),
 ('EmptyContract','OC.EMPTY_CONTRACT','El contrato de comprobación está vacío.','The check contract is empty.'),
 ('ContractTooLarge','OC.CONTRACT_TOO_LARGE','El contrato de comprobación supera el límite de bytes.','The check contract exceeds the byte limit.'),
 ('ExpectedTooLarge','OC.EXPECTED_TOO_LARGE','La referencia esperada supera el límite de bytes.','The expected reference exceeds the byte limit.'),
 ('ObservedTooLarge','OC.OBSERVED_TOO_LARGE','La observación supera el límite de bytes.','The observation exceeds the byte limit.'),
 ('ExactMismatch','OC.EXACT_MISMATCH','La observación difiere de la referencia exacta.','The observation differs from the exact reference.'),
 ('MissingObservation','OC.MISSING_OBSERVATION','No se dispone de la observación necesaria para comprobar.','The observation required for verification is unavailable.'),
]
cat={'version':'RECEPTION-DIAGNOSTICS/1','canonicos_sv':False,'filas':[dict(variante=v,clave=k,es=es,en=en) for v,k,es,en in rows]}
(o/'CATALOGO.json').write_text(json.dumps(cat,ensure_ascii=False,indent=2)+'\n')
for lang in ['es','en']:
 (o/'idiomas'/lang/'diagnosticos.json').write_text(json.dumps({'version':cat['version'],'idioma':lang,'mensajes':{k:(es if lang=='es' else en) for v,k,es,en in rows}},ensure_ascii=False,indent=2)+'\n')
source='''//! RETP-157. Identidad diagnóstica de fronteras candidatas; no autoridad SV.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language { Es, En }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticStage { PrepareReception, Receive, PrepareCheck, Compare }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCause {\n'''+''.join('    '+v+',\n' for v,_,_,_ in rows)+'''}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Diagnostic { cause: DiagnosticCause, stage: DiagnosticStage }
impl Diagnostic {
 pub const VERSION: &'static str = "RECEPTION-DIAGNOSTICS/1";
 pub(crate) const fn new(cause: DiagnosticCause, stage: DiagnosticStage) -> Self { Self { cause, stage } }
 pub const fn cause(&self) -> DiagnosticCause { self.cause }
 pub const fn stage(&self) -> DiagnosticStage { self.stage }
 pub const fn code(&self) -> &'static str { match self.cause {\n'''+''.join(' DiagnosticCause::'+v+' => '+json.dumps(k)+',\n' for v,k,_,_ in rows)+''' } }
 // Sólo texto estático: ningún dato recibido se inserta en el diagnóstico.
 // El resultado no se calcula a partir del idioma ni del texto mostrado.
 pub const fn message(&self, language: Language) -> &'static str { match (self.cause, language) {\n'''+''.join(' (DiagnosticCause::'+v+', Language::'+lang+') => '+json.dumps(msg,ensure_ascii=False)+',\n' for v,k,es,en in rows for lang,msg in [('Es',es),('En',en)])+''' } }
}
'''
src=r/'candidata/rust/sv_core/src';(src/'audit_diagnostics.rs').write_text(source)
p=src/'lib.rs';p.write_text(p.read_text().replace('pub mod admissibility;','pub mod audit_diagnostics;\npub mod admissibility;',1))
print('cápsula base comprobada; plan y tablas fijados')
