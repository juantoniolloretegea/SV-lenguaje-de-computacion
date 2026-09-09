// Observador externo de transporte: entradas estrictas, originales fijados,
// construcción del contrato y recálculo LIG independientes del emisor Rust.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {resolve} from 'node:path';
import {parseDocumentaryJson as parse} from '../row7_gh/documentary_json.mjs';
import {contractHash} from '../row7_gh/contract_hash.mjs';
import {regulated,continuity,regulatedMutants,continuityMutants} from './reglas_documentales.mjs';
const root=new URL('./',import.meta.url),manifestSha='30d0cce81cff67c77fe4cf3a4a5322353fe910ae2ac2694fb972c13a63bf678c';
const hash=b=>createHash('sha256').update(b).digest('hex');
const same=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
const require=(condition,code)=>{if(!condition)throw Error(code);};
const hex=s=>Buffer.from(s,'utf8').toString('hex');
const doc=c=>'parametro' in c?{parametro:c.parametro,entrada:c.entrada}:{clase:c.clase,datos:c.datos};
const consume=c=>'parametro' in c?regulated(c.parametro,c.entrada):continuity(c.clase,c.datos);
function sources() {
  const raw=readFileSync(new URL('fuentes.json',root));require(hash(raw)===manifestSha,'MANIFIESTO_FIJO');const m=parse(raw),all=new Map();
  for(const f of m.files){const b=readFileSync(new URL(f.path,root));require(b.length===f.bytes&&hash(b)===f.sha256,'FUENTE_FIJA');all.set(f.path,parse(b));}
  require(all.size===11,'INVENTARIO_FUENTES');return all;
}
const all=sources(),pr=all.get('originales/v03/testigos.json'),ct=all.get('originales/v04/testigos.json');
const si=all.get('originales/v03/pares_semanticos.json'),pc=all.get('originales/v04/pares.json');
function expectedRows() {
  const rows=[...pr,...ct].map(c=>({id:c.id,variant:'F0',payload:JSON.stringify(doc(c)),side:null,expected:c.esperado,query:null}));
  for(const [version,pairs] of [[3,si],[4,pc]])for(const pair of pairs)for(const state of ['a','b']){
    const c=version===3?pair[state]:ct.find(c=>c.id===pair[state]);require(c,'REFERENTE_PAR');
    const full=version===3?c:doc(c),reduced=version===3?c.technical:pair.proyeccion_tecnica_comun;
    for(const variant of ['F0','H','HS'])rows.push({id:`${pair.id}-${state}`,variant,payload:JSON.stringify(variant==='F0'?full:reduced),side:variant==='HS'?JSON.stringify(full):null,
      expected:version===3?JSON.stringify(c[pair.diferencia]):c.esperado,query:version===3?pair.diferencia:null});
  }
  require(rows.length===186,'INVENTARIO_ESPERADO');return rows;
}
const expected=expectedRows();
const ref=(id,bytes)=>({identifier:id,version:'1',sha256:hash(Buffer.from(bytes))});
function expectedContract(row) {
  const specs=[['ConstitucionD','Constitution'],['AutorD','AuthorityDeclaration'],['Phi','CaptureDefinition'],['Regla','AdmissionDefinition'],['FuenteP','Provenance'],['OP','OperationDefinition']];
  const artifacts=specs.map(([id,kind])=>{const bytes=id==='FuenteP'?row.payload:`testigo declarativo ${id}; versión 1`;return {reference:ref(id,bytes),kind,hex:hex(bytes)};});
  const get=id=>artifacts.find(a=>a.reference.identifier===id).reference;
  const lateral=[];
  if(row.side!==null){const r=ref('Lateral',row.side);artifacts.push({reference:r,kind:'SideInformation',hex:hex(row.side)});lateral.push(r);}
  return {schema:'LIG/0.1',identifier:'CYB-TRANSPORTE-0.1',version:'1',
    program:{source_file:'ligaduras.svp',source_sha256:'accbd7b0d8af928d392a2ad379cd70b7d1d9322925d92a40e0ec69ac2882d9c5',projection_sha256:'6736b7c78adefa3a53b07640e27bb6bd4cdf7e1af01c4f8265754535cb18ba3a'},
    domain:'D',agent:'AG',constitution:get('ConstitucionD'),authority:get('AutorD'),
    instances:[{identifier:'I1',owner:'D',parameter:'P',parameter_id:'1',capture:{object:'Cap',definition:get('Phi')},admission:{object:'Adm',definition:get('Regla')},ternarizer:null,provenance:[get('FuenteP')]}],
    operations:[{identifier:'OP',version:'1',definition:get('OP'),uses:[{identifier:'U1',instance:'I1',destination:null,alias_of:null}],requires_destination:false,sharing:[],input_scope:row.side===null?'BindingsOnly':'BindingsWithSideInformation',side_information:lateral}],artifacts};
}
export function verify(report) {
  require(same(Object.keys(report),['schema','rows'])&&report.schema==='CYB-LIG-TRANSPORTE/0.1','ESQUEMA');
  require(Array.isArray(report.rows)&&report.rows.length===expected.length,'INVENTARIO');
  const outputs=[];
  for(let i=0;i<expected.length;i++){
    const r=report.rows[i],e=expected[i];
    require(same(Object.keys(r),['id','variant','contract_sha256','recovered_hex','contract']),'ESQUEMA_FILA');
    require(r.id===e.id&&r.variant===e.variant,'INVENTARIO');
    require(contractHash(r.contract)===r.contract_sha256,'CONTRATO_RECALCULO');
    require(same(r.contract,expectedContract(e)),'CONTRATO_TESTIGO');
    require(r.recovered_hex===hex(e.side??e.payload),'RECUPERACION');
    const recovered=parse(Buffer.from(r.recovered_hex,'hex'));
    if(r.variant!=='H'){
      const answer=e.query===null?consume(recovered):JSON.stringify(recovered[e.query]);
      require(answer===e.expected,'USO_DOCUMENTAL');outputs.push({id:r.id,variant:r.variant,observed:answer});
    }
  }
  const pairs=[];
  for(let i=78;i<expected.length;i+=6){
    const a=report.rows.slice(i,i+3),b=report.rows.slice(i+3,i+6);
    require(same(a[1].contract,b[1].contract),'H_FILTRA_ESTADO');
    require(!same(a[0].contract,b[0].contract)&&!same(a[2].contract,b[2].contract),'PERDIDA_COMPLETA');
    require(expected[i].expected!==expected[i+3].expected,'PAR_NO_DISCRIMINANTE');
    pairs.push({id:expected[i].id.slice(0,-2),reduced_contract_equal:true,answers_different:true});
  }
  return {scope:'TRANSPORTE_Y_USO_DOCUMENTAL_EXTERNO',contract_hashes_recomputed:186,complete_cases:78,full_pair_recoveries:36,side_pair_recoveries:36,reduced_controls:36,pairs,outputs,
    sv_domain_operation_executed:false,cyb_architecture_constituted:false,material_security_proven:false};
}
function rulesSensitivity() {
  const out=[];
  for(const c of [...pr,...ct])require(consume(doc(c))===c.esperado,'TRANSCRIPCION_DOCUMENTAL');
  for(const [name,cases,judge,mutants] of [['v03',pr,(c,m)=>regulated(c.parametro,c.entrada,m),regulatedMutants],['v04',ct,(c,m)=>continuity(c.clase,c.datos,m),continuityMutants]])for(const m of mutants){
    const ids=cases.filter(c=>judge(c,m)!==c.esperado).map(c=>c.id);require(ids.length>0,'MUTANTE_DOCUMENTAL_SUPERVIVIENTE');out.push({family:name,mutation:m,refuters:ids});
  }
  return out;
}
export function selftest(report) {
  const attacks=[];
  const attack=(name,code,change)=>{
    const forged=structuredClone(report);change(forged);let got=null;try{verify(forged);}catch(e){got=e.message;}require(got===code,`SENSIBILIDAD_${name}_${got}`);attacks.push({name,cause:got});
  };
  attack('huella_F0','CONTRATO_RECALCULO',r=>r.rows[0].contract_sha256='0'.repeat(64));
  attack('huellas_H_simetricas','CONTRATO_RECALCULO',r=>{r.rows[79].contract_sha256=r.rows[82].contract_sha256='0'.repeat(64);});
  attack('fuente_con_huellas_recalculadas','CONTRATO_TESTIGO',r=>{const a=r.rows[0].contract.artifacts.find(a=>a.reference.identifier==='FuenteP');a.hex=hex('{}');a.reference.sha256=hash(Buffer.from('{}'));r.rows[0].contract.instances[0].provenance[0].sha256=a.reference.sha256;r.rows[0].contract_sha256=contractHash(r.rows[0].contract);});
  attack('identificador_filtrado','CONTRATO_TESTIGO',r=>{r.rows[79].contract.identifier='SI01-a';r.rows[79].contract_sha256=contractHash(r.rows[79].contract);});
  attack('recuperacion_sustituida','RECUPERACION',r=>r.rows[0].recovered_hex=hex('{}'));
  attack('fila_duplicada','INVENTARIO',r=>r.rows[1]=r.rows[0]);
  attack('fila_ausente','INVENTARIO',r=>r.rows.pop());
  attack('alcance_falso','ESQUEMA',r=>r.row9_closed=true);
  const raw=JSON.stringify(report);
  for(const [name,token,code] of [['duplicada','{"schema":"falso",'+raw.slice(1),'JSON_CLAVE_REPETIDA'],['decimal',raw.replace('"rows":','"extra":7.5,"rows":'),'JSON_NUMERO_FORMA'],['entero_grande',raw.replace('"rows":','"extra":12345678901234567890,"rows":'),'JSON_NUMERO_RANGO'],['sustituto',raw.replace('"rows":','"extra":"\\ud800","rows":'),'JSON_UNICODE']]){
    let got=null;try{verify(parse(Buffer.from(token)));}catch(e){got=e.message;}require(got===code,`SENSIBILIDAD_${name}`);attacks.push({name,cause:got});
  }
  return attacks;
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)) {
  try {
    const args=process.argv.slice(2);
    if(args.length===1&&args[0]==='--fuentes'){
      console.log(JSON.stringify({sources:11,cases:78,rows:186,rules_sensitivity:rulesSensitivity()},null,2));
    } else {
      require(args.length>=1&&args.length<=2&&(args.length===1||args[1]==='--autoprueba'),'USO');
      const report=parse(readFileSync(args[0]));const result=verify(report);
      result.rules_sensitivity=rulesSensitivity();
      if(args[1]==='--autoprueba')result.observer_sensitivity=selftest(report);
      console.log(JSON.stringify(result,null,2));
    }
  }catch(e){console.error(e.message);process.exitCode=1;}
}
