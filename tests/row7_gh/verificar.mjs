// Observador documental externo. Los esperados proceden del retorno G/H fijado.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {resolve} from 'node:path';

const root = new URL('./', import.meta.url);
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const same = (a,b) => JSON.stringify(a) === JSON.stringify(b);
function requireThat(condition, code) { if (!condition) throw new Error(code); }
function pinned(name, expected) {
  const bytes = readFileSync(new URL(name,root));
  requireThat(hash(bytes) === expected, 'FUENTE_ALTERADA');
  return JSON.parse(bytes);
}
const source = pinned('testigos-gh.json','037944fe4ca28524d7e89403d08e881462f7ec349c21e7d9127ccf4c7c7b1f62');
const inventory = pinned('inventario-gh.json','fad302dc94331f5c4245c0a87c7b47765ee344abb0cd8d56d718d238a7fb2ee6');
const variants = ['F0','H','HS'];
const ids = rows => rows.map(row => row.id);
const jsonBytes = value => Buffer.from(JSON.stringify(value),'utf8');
const rowKey = r => `${r.witness}/${r.state}/${r.variant}`;

// Este control demuestra integridad del inventario, no dicta la suficiencia de una decisión.
export function verifyResolution(resolution) {
  requireThat(resolution.schema === 'GH-LIG-RESOLUCION/0.1' && resolution.contract === 'GH-LIG/0.1', 'MATRIZ_ESQUEMA');
  requireThat(resolution.row7_closed === false && resolution.q0_sufficient === false && resolution.cyb_opened === false && resolution.status === 'CANDIDATA_NO_PROMOVIDA', 'MATRIZ_ALCANCE');
  requireThat(resolution.gh_commit === inventory.source_commit && resolution.source_head === 'd374e1cb373a3dcd141faf482f9501ceb47a7e0b', 'MATRIZ_CORTE');
  for (const category of ['g10','lsv']) requireThat(same(ids(resolution[category]),ids(inventory[category])), 'MATRIZ_INVENTARIO');
  const treatmentIds = ids(resolution.treatments);
  requireThat(new Set(treatmentIds).size === treatmentIds.length, 'MATRIZ_TRATAMIENTOS');
  for (const treatment of resolution.treatments) {
    for (const key of ['id','operation','contract','available','excluded','responsible','stage','reentry_condition','evidence'])
      requireThat(typeof treatment[key] === 'string' && treatment[key].trim().length > 0, 'MATRIZ_TRATAMIENTOS');
  }
  for (const row of [...resolution.g10,...resolution.lsv,...resolution.specified,...resolution.documentary]) {
    requireThat(Array.isArray(row.treatments) && row.treatments.length > 0 && new Set(row.treatments).size === row.treatments.length && row.treatments.every(id => treatmentIds.includes(id)), 'MATRIZ_TRATAMIENTOS');
  }
  for (const [index,row] of resolution.lsv.entries()) {
    requireThat(row.responsible === inventory.lsv[index].responsable && row.source_witness === inventory.lsv[index].testigo, 'MATRIZ_RESPONSABLE');
  }
  requireThat(same(ids(resolution.specified),ids(source.specified)), 'MATRIZ_SP');
  for (const [index,row] of resolution.specified.entries())
    requireThat(row.status === source.specified[index].status && row.owner === source.specified[index].owner, 'MATRIZ_SP_ESTADO');
  requireThat(same(ids(resolution.documentary),ids(source.witnesses)), 'MATRIZ_DOCUMENTAL');
  for (const [index,row] of resolution.documentary.entries()) {
    const witness = source.witnesses[index];
    requireThat(row.operation === witness.operation.id && same(row.paths,witness.operation.paths) && same(row.representations,variants) && row.expected_loss_in_H === true, 'MATRIZ_DOCUMENTAL');
  }
  const families = inventory.families.map(f => ({id:f.id,status:f.estado,queries:f.consultas,limit:f.limite}));
  requireThat(same(resolution.families,families), 'MATRIZ_APLICABILIDAD');
  const edges = inventory.g10.flatMap(g => g.enlaces.map(e => `${g.id}/${e.lsv}`));
  requireThat(edges.length === 81 && new Set(edges).size === 81, 'FUENTE_ENLACES');
  const inverse = inventory.lsv.flatMap(l => l.g10.map(g => `${g}/${l.id}`));
  requireThat(same([...edges].sort(),inverse.sort()), 'FUENTE_ENLACES');
  requireThat(inventory.parameters.length === 27 && new Set(inventory.parameters).size === 27, 'FUENTE_PARAMETROS');
  return {g10:resolution.g10.length,lsv:resolution.lsv.length,links:edges.length,specified:resolution.specified.length,
          documentary:resolution.documentary.length,families:families.length,queries:families.reduce((n,f)=>n+Object.keys(f.queries).length,0),
          clinical_integration_executed:false,decision_sufficiency_automatically_proven:false};
}
function artifactBytes(artifact) {
  requireThat(artifact && typeof artifact.hex === 'string' && /^(?:[0-9a-f]{2})*$/.test(artifact.hex), 'BYTES_FORMATO');
  const bytes = Buffer.from(artifact.hex,'hex');
  requireThat(hash(bytes) === artifact.sha256, 'ARTEFACTO_INTEGRIDAD');
  return bytes;
}
function expectArtifact(actual,id,value,code) {
  requireThat(actual.id === id && actual.version === '1', 'REFERENTE');
  const bytes = artifactBytes(actual);
  requireThat(bytes.equals(jsonBytes(value)), code);
  return JSON.parse(new TextDecoder('utf-8',{fatal:true}).decode(bytes));
}
function ask(packet,operation) {
  return operation.paths.map(path => path.split('.').reduce((value,key) => {
    requireThat(value !== null && typeof value === 'object' && Object.hasOwn(value,key),'RUTA_AUSENTE');
    return value[key];
  },packet));
}
export function verify(report,resolution) {
  const matrix = verifyResolution(resolution);
  requireThat(report.schema === 'GH-LIG-OBSERVACIONES/0.1' && Array.isArray(report.results), 'INFORME_ESQUEMA');
  const expectedKeys = source.witnesses.flatMap(w => w.domain.states.flatMap(s => variants.map(v => `${w.id}/${s.id}/${v}`)));
  requireThat(report.results.length === expectedKeys.length, 'INVENTARIO');
  requireThat(same(report.results.map(rowKey),expectedKeys), 'IDENTIDAD_TESTIGO');
  const results = new Map(report.results.map(r => [rowKey(r),r]));
  let full = 0, sideRecovery = 0, controls = 0, losses = 0;
  for (const witness of source.witnesses) {
    const h = [];
    requireThat(witness.domain.states.length === 2,'TESTIGO_DOMINIO');
    for (const state of witness.domain.states) {
      const reduced = Object.fromEntries(Object.entries(state.packet).filter(([key]) => key !== 'detail'));
      for (const variant of variants) {
        const row = results.get(`${witness.id}/${state.id}/${variant}`);
        requireThat(row.operation === witness.operation.id, 'OPERACION');
        requireThat(/^[0-9a-f]{64}$/.test(row.contract_sha256), 'CONTRATO_HUELLA');
        const withSide = variant === 'HS';
        requireThat(row.scope === (withSide ? 'BindingsWithSideInformation' : 'BindingsOnly'), 'ALCANCE_S');
        const registry = ['ConstitucionD','AutorD','Phi','Regla','FuenteP',witness.operation.id,...(withSide?['S']:[])];
        requireThat(same(row.artifact_ids,registry), 'REGISTRO_ARTEFACTOS');
        expectArtifact(row.definition,witness.operation.id,witness.operation,'DEFINICION');
        const packet = expectArtifact(row.source,'FuenteP',variant==='F0'?state.packet:reduced,'PROCEDENCIA');
        requireThat(Array.isArray(row.side) && row.side.length === (withSide ? 1 : 0), 'INVENTARIO_S');
        if (variant === 'F0') {
          requireThat(same(ask(packet,witness.operation),state.expected),'RECUPERACION_F0'); full++;
        } else if (withSide) {
          const side = expectArtifact(row.side[0],'S',{detail:state.packet.detail},'CONTENIDO_S');
          requireThat(same(ask({...packet,detail:side.detail},witness.operation),state.expected),'RECUPERACION_HS'); sideRecovery++;
        } else {
          requireThat(same(ask(packet,witness.control.operation),witness.control.expected),'CONTROL_H'); controls++;
          let lost = false;
          try { ask(packet,witness.operation); } catch (error) { if (error.message==='RUTA_AUSENTE') lost=true; else throw error; }
          requireThat(lost,'PERDIDA_NO_EJERCIDA');
          h.push({bytes:row.source.hex,contract:row.contract_sha256,expected:state.expected});
        }
      }
    }
    requireThat(h[0].bytes === h[1].bytes && h[0].contract === h[1].contract, 'H_IDENTIDAD');
    requireThat(!same(h[0].expected,h[1].expected), 'TESTIGO_SIN_PERDIDA'); losses++;
  }
  return {verification:'CONFORME_EN_TRANSPORTE_DOCUMENTAL',transports:report.results.length,full_recoveries:full,
          recoveries_with_declared_side_information:sideRecovery,reduced_controls:controls,losses_in_H:losses,matrix,
          sv_query_executed:false,q0_executed:false,clinical_authority_authenticated:false};
}
function replaceBytes(artifact,value) { const bytes=jsonBytes(value); artifact.hex=bytes.toString('hex'); artifact.sha256=hash(bytes); }
export function sensitivity(report,resolution) {
  const select=(r,variant)=>r.results.find(x=>x.variant===variant);
  const attacks = [
    ['MG01_omitir_transporte','INVENTARIO',(r)=>r.results.pop()],
    ['MG02_duplicar_identidad','IDENTIDAD_TESTIGO',(r)=>r.results[1]=structuredClone(r.results[0])],
    ['MG03_procedencia_con_huella_recalculada','PROCEDENCIA',(r)=>replaceBytes(select(r,'F0').source,{inventado:true})],
    ['MG04_detalle_oculto_en_H','PROCEDENCIA',(r)=>replaceBytes(select(r,'H').source,source.witnesses[0].domain.states[0].packet)],
    ['MG05_S_retirada','INVENTARIO_S',(r)=>select(r,'HS').side=[]],
    ['MG06_S_atribuida_a_H','ALCANCE_S',(r)=>select(r,'HS').scope='BindingsOnly'],
    ['MG07_operacion_sustituida','OPERACION',(r)=>r.results[0].operation='OTRA'],
    ['MG08_referente_sustituido','REFERENTE',(r)=>r.results[0].source.id='OTRA'],
    ['MG09_H_filtra_estado','H_IDENTIDAD',(r)=>select(r,'H').contract_sha256='a'.repeat(64)],
    ['MG10_huella_de_artefacto','ARTEFACTO_INTEGRIDAD',(r)=>r.results[0].source.sha256='a'.repeat(64)],
    ['MG11_artefacto_lateral_oculto','REGISTRO_ARTEFACTOS',(r)=>select(r,'H').artifact_ids.push('S')],
    ['MG12_formulacion_omitida','MATRIZ_INVENTARIO',(_r,m)=>m.lsv.pop()],
    ['MG13_SP_cerrada_sin_ejecucion','MATRIZ_SP_ESTADO',(_r,m)=>m.specified[0].status='PASS'],
    ['MG14_fila_cerrada','MATRIZ_ALCANCE',(_r,m)=>m.row7_closed=true],
    ['MG15_S_sustituida_con_huella_recalculada','CONTENIDO_S',(r)=>replaceBytes(select(r,'HS').side[0],{detail:{inventado:true}})],
    ['MG16_definicion_sustituida','DEFINICION',(r)=>replaceBytes(r.results[0].definition,{id:r.results[0].operation,paths:['context']})],
  ];
  verify(JSON.parse(JSON.stringify(report,null,2)),JSON.parse(JSON.stringify(resolution,null,2)));
  const rows=[];
  for (const [id,cause,mutate] of attacks) {
    const r=structuredClone(report),m=structuredClone(resolution); mutate(r,m);
    let actual='SURVIVED';
    try { verify(r,m); } catch (error) { actual=error.message; }
    requireThat(actual===cause,`${id}: esperado ${cause}, recibido ${actual}`);
    rows.push({id,expected:cause,actual});
  }
  return {reserialized_control:'ACCEPTED',directed_mutations:rows.length,survivors:0,semantic_artifact_hashes_recalculated:true,results:rows};
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    const args=process.argv.slice(2);
    const matrix=JSON.parse(readFileSync(new URL('resolucion.json',root)));
    if (args.length===1 && args[0]==='--inventario') {
      console.log(JSON.stringify({verification:'INVENTARIO_CONFORME_NO_EVIDENCIA_DE_TRANSPORTE',matrix:verifyResolution(matrix)},null,2));
    } else {
      requireThat(args.length===1 || (args.length===2 && args[1]==='--autoprueba'),'Uso: verificar.mjs INFORME.json [--autoprueba] | --inventario');
      const report=JSON.parse(readFileSync(args[0]));
      const result=verify(report,matrix);
      if (args[1]==='--autoprueba') result.sensitivity=sensitivity(report,matrix);
      console.log(JSON.stringify(result,null,2));
    }
  } catch (error) { console.error(error.message); process.exitCode=1; }
}
