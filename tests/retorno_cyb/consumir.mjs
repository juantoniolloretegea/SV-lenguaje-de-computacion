// Consumidor del banco documental: recibe módulos ya fijados y datos recuperados.
// No consulta fuentes, no crea permisos ni ejecuta una operación profesional SV.
const insist=(condition,code)=>{if(!condition)throw Error(code);};
const keys=(value,wanted)=>value!==null&&typeof value==='object'&&!Array.isArray(value)
  && JSON.stringify(Object.keys(value))===JSON.stringify(wanted);
export function consumeDocument(payload,definition,rules) {
  insist(keys(definition,['schema','profile','version','consumer','variant','rules','implementation','sources_sha256']),'DEFINICION_FORMA');
  insist(definition.schema==='CYB-CONSUMO/0.1'&&definition.profile==='CYB-DOCUMENTAL'&&definition.version==='1','DEFINICION_VERSION');
  insist(['F0','H','HS'].includes(definition.variant),'DEFINICION_VARIANTE');
  if(definition.variant==='H')throw Error('INFORMACION_INSUFICIENTE');
  const c=definition.consumer;
  insist(keys(c,['key','kind','selector']),'SELECTOR_FORMA');
  insist(c.key===`${c.kind}:${c.selector}`,'SELECTOR_IDENTIDAD');
  if(c.kind==='regulated') {
    insist(/^P(2[5-9]|3[0-2])$/.test(c.selector),'REGLA_NO_CONSTITUIDA');
    insist(keys(payload,['parametro','entrada'])&&payload.parametro===c.selector,'SELECTOR_CARGA');
    return rules.regulated(c.selector,payload.entrada);
  }
  if(c.kind==='continuity') {
    insist(['partida','linaje','asignacion','conciliacion','perimetro','vigencia'].includes(c.selector),'REGLA_NO_CONSTITUIDA');
    insist(keys(payload,['clase','datos'])&&payload.clase===c.selector,'SELECTOR_CARGA');
    return rules.continuity(c.selector,payload.datos);
  }
  if(c.kind==='field') {
    insist(payload!==null&&typeof payload==='object'&&!Array.isArray(payload)
      && Object.hasOwn(payload,c.selector),'CAMPO_NO_DISPONIBLE');
    return JSON.stringify(payload[c.selector]);
  }
  throw Error('REGLA_NO_CONSTITUIDA');
}
