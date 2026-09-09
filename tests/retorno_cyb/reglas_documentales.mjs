// Transcripción externa de las reglas sintéticas aprobadas, anexos CYB 0.3/0.4.
// No es segunda semántica SV, transductor productivo ni autoridad institucional.
const equal=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
const setEqual=(a,b)=>a.size===b.size && [...a].every(x=>b.has(x));
const interval=x=>{
  if(!Array.isArray(x)||x.length!==2||x.some(v=>typeof v!=='string'||!/^(0|-[1-9][0-9]*|[1-9][0-9]*)$/.test(v)))throw Error('INTERVALO');
  const [a,b]=x.map(BigInt);if(a>b)throw Error('ORDEN');return [a,b];
};
export function regulated(p,d,m=null) {
  try {
    for(const flag of ['admitted','applicable','complete','prior','conflict'])if(flag in d&&typeof d[flag]!=='boolean')throw Error('TIPO');
    if(d.admitted===false)return 'NO_ADMISION';
    if(d.applicable===false)return m==='APLICABILIDAD_COMO_EXITO'?'1':'NO_APLICABLE';
    if(d.conflict)return 'U';
    switch(p) {
      case 'P25':
        if(m==='CUENTA_COMO_SESION'&&d.account)return '1';
        if(!('linked' in d))return 'U';return d.session===d.linked?'1':'0';
      case 'P26':
        if(m==='FIRMA_COMO_COMPETENCIA')return '1';
        if(d.grants.some(x=>equal(x,[d.issuer,d.power])))return '1';return d.complete?'0':'U';
      case 'P27':
        if(!('scope' in d))return 'U';
        if(!Array.isArray(d.scope)||!Array.isArray(d.act)||d.act.length!==4||d.act.some(x=>typeof x!=='string')||d.scope.some(t=>!Array.isArray(t)||t.length!==4||t.some(x=>typeof x!=='string')))throw Error('ALCANCE');
        if(m==='SOLO_NOMBRE_ACCION')return d.scope.some(x=>x[0]===d.act[0])?'1':'0';
        return d.scope.some(x=>equal(x,d.act))?'1':'0';
      case 'P28': {
        if(['window','start','end'].some(k=>!(k in d)))return 'U';
        let [lo,hi]=interval(d.window),[sl,su]=interval(d.start),[el,eu]=interval(d.end);
        if(su>el)throw Error('INICIO_FIN');
        if(m==='DOBLE_PRECISION_INTERMEDIA')[lo,hi,sl,su,el,eu]=[lo,hi,sl,su,el,eu].map(Number);
        if(m==='IGNORAR_INCERTIDUMBRE'){sl=su=(sl+su)/2n;el=eu=(el+eu)/2n;}
        if(sl>=lo&&eu<=hi)return '1';if(su<lo||el>hi)return '0';return 'U';
      }
      case 'P29': {
        const [el,eu]=interval(d.end),rr=d.revocations.map(interval);
        if(m==='CUALQUIER_REVOCACION'&&rr.length)return '1';
        if(rr.some(([,ru])=>ru<=el))return '1';
        if(d.complete&&rr.every(([rl])=>rl>eu))return '0';
        if(m==='SILENCIO_COMO_AUSENCIA'&&!rr.length)return '0';return 'U';
      }
      case 'P30': {
        let [il,iu]=interval(d.issued);const [sl,su]=interval(d.start);
        if(m==='FECHA_ESCRITA_COMO_EMISION'&&'written_date' in d)il=iu=BigInt(d.written_date);
        if(iu<sl)return '1';if(il>=su)return '0';return 'U';
      }
      case 'P31':
        if(m==='CUENTAS_COMO_PERSONAS'&&'account_a' in d)return d.account_a!==d.account_b?'1':'0';
        if(!['principal_a','principal_b'].every(k=>k in d))return 'U';return d.principal_a!==d.principal_b?'1':'0';
      case 'P32':
        if(d.prior===false)return 'NO_ADMISION';
        if(m==='PERMISO_COMO_PLAN'&&d.emergency_permit)return '1';
        if(d.plan.some(x=>equal(x,d.act)))return '1';return d.complete?'0':'U';
      default:throw Error('PARAMETRO');
    }
  } catch {return 'FALLO_TECNICO';}
}
export function continuity(k,d,m=null) {
  if(k==='partida') {
    if(d.origen==='genesis_demostrada'&&d.historia_previa==='desconocida'&&m!=='genesis')return 'AFIRMACION_NO_SOSTENIDA';
    if(!d.cobertura_acreditada||!setEqual(new Set(d.ambito),new Set(d.observados)))return 'NO_CONCLUYENTE';
    if(!(d.instante_observacion<=d.instante_acto&&d.instante_acto<=d.limite_vigencia)&&m!=='tiempo_partida')return 'NO_CONCLUYENTE';
    return 'PARTIDA_ACOTADA';
  }
  if(k==='linaje') {
    for(const [id,hash] of Object.entries(d.referencias)) {if(!(id in d.nodos))return 'NO_CONCLUYENTE';if(d.nodos[id]!==hash)return 'EVIDENCIA_NO_ADMITIDA';}
    const g=new Map(Object.keys(d.nodos).map(id=>[id,[]]));
    for(const [a,b,t,support] of d.relaciones) {
      if(!g.has(a)||!g.has(b))return 'NO_CONCLUYENTE';
      if(!['derivacion','referencia','precedencia_estricta','causa'].includes(t))return 'EVIDENCIA_NO_ADMITIDA';
      if(t==='causa'&&!support&&m!=='causalidad')return 'NO_CONCLUYENTE';
      if(['derivacion','precedencia_estricta','causa'].includes(t)||m==='todo_ciclo')g.get(a).push(b);
    }
    const active=new Set(),seen=new Set();
    const cycle=n=>{if(active.has(n))return true;if(seen.has(n))return false;active.add(n);if(g.get(n).some(cycle))return true;active.delete(n);seen.add(n);return false;};
    return [...g.keys()].some(cycle)?'EVIDENCIA_NO_ADMITIDA':'RELACIONES_CONSERVADAS';
  }
  if(k==='asignacion') {
    if(d.exonera_anterior&&m!=='exoneracion')return 'AFIRMACION_NO_SOSTENIDA';
    if((d.clase_documento!=='designacion'&&m!=='heredar_responsabilidad')||!d.facultad_acreditada)return 'ASIGNACION_NO_ACREDITADA';
    if(d.efecto>d.instante)return 'NO_CONCLUYENTE';
    if(d.aceptacion_exigida||m==='aceptacion_universal'){const a=d.aceptacion;if(!a||a.titular!==d.titular||a.obligacion!==d.obligacion||a.instante>d.instante)return 'NO_CONCLUYENTE';}
    return 'ASIGNACION_ACREDITADA_EN_PERFIL';
  }
  if(k==='conciliacion') {
    if(!d.cobertura_acreditada)return 'NO_CONCLUYENTE';
    const states=new Set(d.inicio),prev=new Map();
    for(const e of d.movimientos) {
      if(prev.has(e.id)){if(!equal(prev.get(e.id),e))return 'EVIDENCIA_NO_ADMITIDA';continue;}prev.set(e.id,e);
      if(e.clase!==d.clase)return 'EVIDENCIA_NO_ADMITIDA';if(e.accion==='efecto_incierto')return 'NO_CONCLUYENTE';
      if(e.accion==='alta'){if(states.has(e.objeto))return 'EVIDENCIA_NO_ADMITIDA';states.add(e.objeto);}
      else if(e.accion==='baja'){if(!states.has(e.objeto))return 'EVIDENCIA_NO_ADMITIDA';states.delete(e.objeto);}
      else return 'EVIDENCIA_NO_ADMITIDA';
    }
    const same=m==='solo_cantidad'?states.size===d.fin.length:setEqual(states,new Set(d.fin));
    return same?'CONCILIACION_COINCIDENTE':'CONCILIACION_DISCORDANTE';
  }
  if(k==='perimetro') {
    if(!d.trayectos.length)return 'NO_CONCLUYENTE';
    const covered=new Set(d.pruebas.filter(p=>p.generacion===d.generacion&&p.valida_hasta>=d.instante&&p.resultado==='favorable').map(p=>p.trayecto));
    const good=m==='un_trayecto'?covered.size>0:d.trayectos.every(t=>covered.has(t));
    return good?'COBERTURA_ACREDITADA_EN_PERFIL':'COBERTURA_INSUFICIENTE';
  }
  if(k==='vigencia') {
    for(const e of d.cambios){if(!d.dependencias.includes(e.objeto)&&m!=='todo_cambio')continue;if(e.efecto===null)return 'NO_CONCLUYENTE';if(e.efecto<=d.instante_acto||m==='retroactividad')return 'REQUIERE_REEVALUACION';}
    return 'SIN_CAMBIO_PERTINENTE_DECLARADO';
  }
  throw Error('CLASE_NO_CONSTITUIDA');
}
export const regulatedMutants=['CUENTA_COMO_SESION','FIRMA_COMO_COMPETENCIA','SOLO_NOMBRE_ACCION','IGNORAR_INCERTIDUMBRE','CUALQUIER_REVOCACION','SILENCIO_COMO_AUSENCIA','FECHA_ESCRITA_COMO_EMISION','CUENTAS_COMO_PERSONAS','PERMISO_COMO_PLAN','APLICABILIDAD_COMO_EXITO','DOBLE_PRECISION_INTERMEDIA'];
export const continuityMutants=['genesis','tiempo_partida','causalidad','todo_ciclo','exoneracion','heredar_responsabilidad','aceptacion_universal','solo_cantidad','un_trayecto','todo_cambio','retroactividad'];
