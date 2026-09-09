// Verificación del contrato externo. No acepta código distinto del fijado.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {parseDocumentaryJson as parse} from '../row7_gh/documentary_json.mjs';
const root=new URL('./',import.meta.url);
const fixed='0ac31c03a0291f36ec8d90059d1636a4cde3766bcf93bac61421a81bd223dc64';
const hash=b=>createHash('sha256').update(b).digest('hex');
const same=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
const insist=(yes,code)=>{if(!yes)throw Error(code);};
const raw=readFileSync(new URL('consumidores.json',root));
insist(hash(raw)===fixed,'CONSUMIDORES_FIJOS');
export const consumerManifest=parse(raw);
export const ruleBytes=readFileSync(new URL('reglas_documentales.mjs',root));
export const consumerBytes=readFileSync(new URL('consumir.mjs',root));
insist(hash(ruleBytes)===consumerManifest.rules.sha256,'REGLAS_FIJAS');
insist(hash(consumerBytes)===consumerManifest.implementation.sha256,'CONSUMIDOR_FIJO');
// La carga procede de estos bytes ya comprobados, no de una segunda lectura.
const consumer=await import(`data:text/javascript;base64,${consumerBytes.toString('base64')}`);
let rules;
export function operationDefinition(key,variant) {
  const c=consumerManifest.consumers.find(c=>c.key===key);
  insist(c,'REGLA_NO_CONSTITUIDA');
  return {schema:'CYB-CONSUMO/0.1',profile:consumerManifest.profile,version:consumerManifest.version,
    consumer:c,variant,rules:consumerManifest.rules,implementation:consumerManifest.implementation,
    sources_sha256:consumerManifest.sources_sha256};
}
function artifact(contract,reference,code) {
  const matches=contract.artifacts.filter(a=>a.reference.identifier===reference.identifier);
  insist(matches.length===1,code);
  const a=matches[0];insist(same(a.reference,reference),code);
  insist(typeof a.hex==='string'&&/^(?:[0-9a-f]{2})*$/.test(a.hex),code);
  const b=Buffer.from(a.hex,'hex');insist(hash(b)===reference.sha256,code);return b;
}
export async function consumeBound(contract,expectedDefinition,publishedRecovered) {
  insist(contract.operations.length===1,'OPERACION_UNICA');
  const op=contract.operations[0];
  const definition=parse(artifact(contract,op.definition,'DEFINICION_INTEGRIDAD'));
  insist(definition.version==='1','DEFINICION_VERSION');
  insist(same(definition.rules,consumerManifest.rules),'REGLA_LIGADA');
  insist(same(definition.implementation,consumerManifest.implementation),'CONSUMIDOR_LIGADO');
  insist(same(definition,expectedDefinition),'OPERACION_LIGADA');
  const rb=artifact(contract,definition.rules,'REGLA_INTEGRIDAD');
  const cb=artifact(contract,definition.implementation,'CONSUMIDOR_INTEGRIDAD');
  insist(rb.equals(ruleBytes),'REGLA_LIGADA');insist(cb.equals(consumerBytes),'CONSUMIDOR_LIGADO');
  // Comprobar el ámbito antes de recuperar: no se busca S fuera de su contrato.
  const lateral=definition.variant==='HS';
  insist(op.input_scope===(lateral?'BindingsWithSideInformation':'BindingsOnly')
    &&op.side_information.length===(lateral?1:0),'CONSUMO_AMBITO');
  insist(op.uses.length===1&&op.uses[0].destination===null,'CONSUMO_USO');
  const instance=contract.instances.find(i=>i.identifier===op.uses[0].instance);
  insist(instance&&instance.provenance.length===1,'CONSUMO_INSTANCIA');
  const bytes=artifact(contract,lateral?op.side_information[0]:instance.provenance[0],'CONSUMO_FUENTE');
  insist(bytes.toString('hex')===publishedRecovered,'RECUPERACION');
  const data=parse(bytes);
  // No hay invocación de código recibido: sólo el módulo conocido ya verificado.
  rules??=await import(`data:text/javascript;base64,${rb.toString('base64')}`);
  return consumer.consumeDocument(data,definition,rules);
}
export const directConsumer=consumer.consumeDocument;
