from pathlib import Path
import json,hashlib,datetime
r=Path(__file__).resolve().parent
def write(n,x):(r/n).write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
cases=[]
vocab=[
 dict(p='en',rel='semantic_relation',pat='pattern',kind='kind',dr='DeclaredRelation',dp='DeclaredPattern',table='table',arity='arity',cons='constraints',foreign='aridad',let='let',supervise='supervise',target='target',foreign_target='ObjetivoCelda',query='query',by='by',in_='in',foreign_query='VistaEvaluaciónPuntual',protected='codomain',foreign_protected='codominio',adm='admissibility_spec',pid='parameter_id',states='states',rule='rule',ok='Ok',deg='Degraded',no='NotAdmitted',foreign_state='Admitido'),
 dict(p='es',rel='relación_semántica',pat='patrón',kind='clase',dr='RelaciónDeclarada',dp='PatrónDeclarado',table='tabla',arity='aridad',cons='restricciones',foreign='arity',let='sea',supervise='supervisar',target='objetivo',foreign_target='CellTarget',query='consultar',by='por',in_='en',foreign_query='PointEval',protected='codominio',foreign_protected='codomain',adm='especificación_de_admisibilidad',pid='identificador_de_parámetro',states='estados',rule='regla',ok='Admitido',deg='Degradado',no='NoAdmitido',foreign_state='Ok')]
for v in vocab:
 p=v['p'];prefix='-- prólogo é: frame, fuente y fidelidad\r\n\t'
 def add(id,s,cause,marker=None,eof=False):
  s=prefix+s;span=None
  if marker is not None:
   i=s.rindex(marker);span=[len(s[:i].encode()),len(s[:i+len(marker)].encode())]
  if eof:span=[len(s.encode())]*2
  cases.append(dict(id=id+'_'+p,profile=p,source=s,cause=cause,span=span,marker=marker))
 for obj,field,value,kind in [('rel',v['table'],'TabA',v['dr']),('pat',v['arity'],'3',v['dp'])]:
  start=f"{v[obj]} Probe {{ {v['kind']}: {kind}; ";c=v['cons'];f=v['foreign']
  add(obj+'_REPEAT',start+f'{field}: {value}; {field}: {value}; }}','CD.UNEXPECTED_TOKEN',field)
  add(obj+'_ORDER',start+f'{c}: [Local]; {field}: {value}; }}','CD.UNEXPECTED_TOKEN',field)
  add(obj+'_CONSTRAINTS_REPEAT',start+f'{c}: [Local]; {c}: [Local]; }}','CD.UNEXPECTED_TOKEN',c)
  add(obj+'_UNKNOWN',start+'inventado: 3; }','CD.UNSUPPORTED','inventado')
  add(obj+'_FOREIGN',start+f'{f}: 3; }}','CD.FOREIGN_SURFACE',f)
  add(obj+'_COLON_PRECEDENCE',start+f'{f}; 3; }}','CD.UNEXPECTED_TOKEN','; 3'[:1])
  # El único punto y coma posterior al campo sería ambiguo para rindex; se fija el intervalo del separador explícito.
  s=cases[-1]['source'];i=s.index(f+';')+len(f);cases[-1]['span']=[len(s[:i].encode()),len(s[:i+1].encode())]
  add(obj+'_EOF_PRECEDENCE',start+f,'CD.UNEXPECTED_END',eof=True)
 sup=f"{v['let']} X = {v['supervise']}(Meta, {v['target']}: "
 add('SUPERVISE_UNKNOWN',sup+'inventado(Ref));','CD.UNSUPPORTED','inventado')
 add('SUPERVISE_FOREIGN',sup+v['foreign_target']+'(Ref));','CD.FOREIGN_SURFACE',v['foreign_target'])
 add('SUPERVISE_PRECEDENCE',sup+v['foreign_target']+'(Ref);','CD.UNEXPECTED_TOKEN',';')
 query=f"{v['let']} X = {v['query']}(Spec, {v['by']}: AgentX, {v['in_']}: "
 add('QUERY_UNKNOWN',query+'inventado(Ref));','CD.UNSUPPORTED','inventado')
 add('QUERY_FOREIGN',query+v['foreign_query']+'(Ref));','CD.FOREIGN_SURFACE',v['foreign_query'])
 add('QUERY_PRECEDENCE',query+v['foreign_query']+';','CD.UNEXPECTED_TOKEN',';')
 add('LET_PROTECTED',f"{v['let']} X = {v['protected']};",'CD.UNEXPECTED_TOKEN',v['protected'])
 add('LET_FOREIGN',f"{v['let']} X = {v['foreign_protected']};",'CD.FOREIGN_SURFACE',v['foreign_protected'])
 adm=f"{v['adm']} A {{ {v['pid']}: 1; {v['states']}: {{"
 add('STATE_UNKNOWN',adm+'inventado}; }','CD.INVALID_ADMISSIBILITY_STATE','inventado')
 add('STATE_FOREIGN',adm+v['foreign_state']+'}; }','CD.FOREIGN_SURFACE',v['foreign_state'])
 add('STATE_TOKEN',adm+'7}; }','CD.UNEXPECTED_TOKEN','7')
 add('STATE_CARDINALITY_PENDING',adm+f"{v['ok']}, {v['deg']}}}; {v['rule']}: R; }}",'CD.INVALID_ADMISSIBILITY_STATE')
 add('STATE_VALID',adm+f"{v['ok']}, {v['deg']}, {v['no']}}}; {v['rule']}: R; }}",'OK')
write('CASOS_NUEVOS.json',{'version':'COMPOSITE-SITES/1','casos':cases})
inherited=json.loads((r/'CASOS_HEREDADOS_159.json').read_text())['casos']
write('CASOS_ESPERADOS.json',{'version':'COMPOSITE-SITES/1','casos':cases+inherited})
plan='''# Plan previo: ubicación de rechazos compuestos

RETP-160. Cortes: Lenguaje 76722442d2899300c69d89ab932890a4d7d74c2f; laboratorio afcbd5046b2eec24cb058ce0fedcf580ca4790e8, rama lab/playground-sv-permanente. Base: cápsula RETP-159 íntegra, 77 archivos. Se cotejan las rectoras ya leídas y el expediente anterior contra los árboles publicados.

Objeto acotado del paso 6: doce puntos de rechazo. Ocho ramas de campos opcionales de SemanticRelation y Pattern (repetición, orden, repetición de restricciones y campo desconocido); variantes de supervisión y contexto de consulta; palabra protegida en despacho let; etiqueta individual de admisibilidad. Cada emisor conservará el índice antes de consumir el elemento responsable. Se mantiene el orden de evaluación actual: los errores intermedios de separador, referencia o fin de fuente siguen precediendo al rechazo que todavía no se haya emitido. No se adelantan validaciones.

Se fijan fuentes, causas e intervalos UTF-8 antes de modificar la candidata. Cada rango procede del fragmento original seleccionado expresamente. Se incluyen ambos perfiles, CRLF, tabulación, texto acentuado, separadores posteriores y precedencia. Los 40 esperados de RETP-159 se conservan íntegros. Cada inválida se ensambla con una fuente sana del otro perfil en ambas posiciones y con igual nombre de archivo. Se compara la API heredada entre base y candidata, además del corpus canónico 14/106, 239 unitarios por modo y anteriores pruebas focales y relacionales.

Contrato: extensión de cobertura de COMPILER-DIAGNOSTICS/2, sin nuevas variantes ni plantillas. Los emisores incluidos pasan de rango ausente a rango original; las grafías constitutivas extranjeras se clasifican mediante estado léxico y contexto, nunca por mensaje ni centinela. La decisión, el error heredado, su precedencia, IR y códigos SV se preservan. La cardinalidad del conjunto de admisibilidad no se atribuye artificialmente a una etiqueta: permanece sin rango hasta constituir y comprobar una localización del conjunto. Otros validadores y subcausas siguen pendientes.

Controles de sensibilidad previstos: la base RETP-159 debe fallar por intervalo ausente en rel_REPEAT_en; un mutante que guarde el índice de campo después de consumirlo debe fallar por intervalo en el mismo caso. Fallos esperados separados de fallos inesperados, conservados con salidas completas. Catálogo de trece causas y veintiséis mensajes ES/EN idéntico. Cuatro clientes negativos mantienen los seis rechazos de acceso.

Ejecución Rust nativa debug y release, tres repeticiones focales; Python únicamente recupera y observa. Límites instrumentales: 120 segundos por proceso y 2 MiB por flujo. Se conservan identidad, comandos, salidas, códigos, tiempo, CPU y máximo acumulado de memoria de hijos, incluida compilación. No se constituye comparación de rendimiento, dinero ni inferencia. Reproducción sin red en directorio nuevo antes de publicar.

Rectoras consultadas en la continuidad: AGENTS, Pilares completos, perfiles y ensamblaje, transición y relevos, contrato diagnóstico RETP-109/110, workflow V2 y plan RETP-147. Referencia conceptual: LEAME_PRIMERO del frame, documento de significado humano/trazabilidad/fidelidad, adenda visual y documento de trazabilidad/auditoría/reproducción. Un diagnóstico más preciso ayuda a conservar procedencia; no demuestra la fidelidad de la presentación final ni la suficiencia del conocimiento consultado.

Estado global y secuencia conservados: fila 9 abierta bajo las prioridades vigentes, paso 6 incremental sin cierre P6; reserva P3 cerrada, P4/P5 pendientes; DFL-001/011 y restantes deudas abiertas. No se promueve al núcleo productivo, no se ejecutan proveedores externos ni se acredita WASI/navegador o recepción profesional. El encargo externo común sigue pendiente.
'''
(r/'PLAN_PREVIO.md').write_text(plan)
write('COMPROMISO_PREVIO.json',{'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'archivos':{n:hashlib.sha256((r/n).read_bytes()).hexdigest() for n in ['PLAN_PREVIO.md','CASOS_NUEVOS.json','CASOS_HEREDADOS_159.json','CASOS_ESPERADOS.json']}})
print(len(cases),'fuentes nuevas;',len(cases+inherited),'totales')
