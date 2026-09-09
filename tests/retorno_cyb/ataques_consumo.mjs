// Ataques al enlace: recalculan integridad para alcanzar la guarda pretendida.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {parseDocumentaryJson as parse} from '../row7_gh/documentary_json.mjs';
import {contractHash} from '../row7_gh/contract_hash.mjs';
import {operationDefinition,consumeBound,directConsumer} from './enlace_consumidor.mjs';
const hash=b=>createHash('sha256').update(b).digest('hex');
const insist=(v,c)=>{if(!v)throw Error(c);};
const report=parse(readFileSync(process.argv[2]));
const rows=report.rows,results=[];
const update=(r,id,bytes)=>{
  const a=r.contract.artifacts.find(a=>a.reference.identifier===id),old=a.reference.sha256;
  a.hex=bytes.toString('hex');a.reference.sha256=hash(bytes);
  const visit=x=>{if(x&&typeof x==='object'){if(x.identifier===id&&x.sha256===old)x.sha256=a.reference.sha256;for(const y of Object.values(x))visit(y);}};
  visit(r.contract);r.contract_sha256=contractHash(r.contract);
};
const changeDefinition=(r,change)=>{
  const a=r.contract.artifacts.find(a=>a.reference.identifier==='OP'),d=parse(Buffer.from(a.hex,'hex'));
  change(d);update(r,'OP',Buffer.from(JSON.stringify(d)));
};
async function attack(name,index,code,change) {
  const control=rows[index],definition=parse(Buffer.from(control.contract.artifacts.find(a=>a.reference.identifier==='OP').hex,'hex'));
  const r=structuredClone(control);change(r);
  // El contrato alterado conserva huella coherente; LIG no decide la regla profesional.
  insist(contractHash(r.contract)===r.contract_sha256,'INTEGRIDAD_DEL_ATAQUE');
  let cause;try{await consumeBound(r.contract,definition,r.recovered_hex);}catch(e){cause=e.message;}
  insist(cause===code,`ATAQUE_${name}_${cause}`);
  let repaired;try{await consumeBound(control.contract,definition,control.recovered_hex);repaired='ADMITIDO';}catch(e){repaired=e.message;}
  insist(repaired===(control.variant==='H'?'INFORMACION_INSUFICIENTE':'ADMITIDO'),'CONTROL');
  results.push({name,cause,control:repaired});
}
await attack('otra_definicion',0,'OPERACION_LIGADA',r=>changeDefinition(r,d=>d.consumer=operationDefinition('regulated:P26','F0').consumer));
await attack('otra_regla',0,'REGLA_LIGADA',r=>{
  const bytes=Buffer.from('globalThis.CYB_CENTINELA_EJECUTADO=true; export const regulated=()=>"1";');
  update(r,'ReglasDocumentales',bytes);changeDefinition(r,d=>d.rules.sha256=hash(bytes));
});
insist(globalThis.CYB_CENTINELA_EJECUTADO===undefined,'CODIGO_ALTERADO_EJECUTADO');
await attack('otro_consumidor',0,'CONSUMIDOR_LIGADO',r=>{
  const bytes=Buffer.from('globalThis.CYB_CENTINELA_EJECUTADO=true; export const consumeDocument=()=>"1";');
  update(r,'ConsumidorDocumental',bytes);changeDefinition(r,d=>d.implementation.sha256=hash(bytes));
});
insist(globalThis.CYB_CENTINELA_EJECUTADO===undefined,'CODIGO_ALTERADO_EJECUTADO');
await attack('carga_otro_selector',0,'SELECTOR_CARGA',r=>{
  const b=Buffer.from('{"parametro":"P26","entrada":{}}');update(r,'FuenteP',b);r.recovered_hex=b.toString('hex');
});
await attack('version_ausente',0,'DEFINICION_VERSION',r=>changeDefinition(r,d=>delete d.version));
await attack('H_ofrecida_completa',79,'OPERACION_LIGADA',r=>changeDefinition(r,d=>d.variant='F0'));
await attack('HS_sin_S',80,'CONSUMO_AMBITO',r=>{r.contract.operations[0].side_information=[];r.contract_sha256=contractHash(r.contract);});
await attack('regla_no_constituida',0,'OPERACION_LIGADA',r=>changeDefinition(r,d=>d.consumer={key:'regulated:P99',kind:'regulated',selector:'P99'}));
// Guarda del consumidor, separada del contrato de referencia anterior.
let cause;try{directConsumer({parametro:'P99',entrada:{}},{...operationDefinition('regulated:P25','F0'),consumer:{key:'regulated:P99',kind:'regulated',selector:'P99'}},{});}catch(e){cause=e.message;}
insist(cause==='REGLA_NO_CONSTITUIDA','SELECTOR_ABIERTO');
results.push({name:'selector_fuera_del_perfil',cause});
console.log(JSON.stringify({scope:'ENLACE_DOCUMENTAL_EXTERNO',attacks:results,untrusted_code_executed:false},null,2));
