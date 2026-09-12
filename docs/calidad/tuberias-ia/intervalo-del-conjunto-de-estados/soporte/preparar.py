from pathlib import Path
import json,hashlib,datetime,itertools
r=Path(__file__).resolve().parent
def write(n,x):(r/n).write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
cases=[]
for p,adm,pid,states,rule,ok,deg,no,foreign in [
 ('en','admissibility_spec','parameter_id','states','rule','Ok','Degraded','NotAdmitted','Admitido'),
 ('es','especificación_de_admisibilidad','identificador_de_parámetro','estados','regla','Admitido','Degradado','NoAdmitido','Ok')]:
 prefix='-- prólogo é: origen y fidelidad\r\n\t'
 start=prefix+f'{adm} A {{ {pid}: 1; {states}: '
 end=f'; {rule}: R; }}'
 def add(id,s,cause,fragment=None,eof=False,role='elemento'):
  span=None
  if fragment is not None:
   i=s.rindex(fragment);span=[len(s[:i].encode()),len(s[:i+len(fragment)].encode())]
  if eof:span=[len(s.encode())]*2
  cases.append(dict(id=id+'_'+p,profile=p,source=s,cause=cause,span=span,marker=fragment,alcance_intervalo=role))
 block='{'+ok+'}'
 add('SEMICOLON_PRECEDENCE',start+block+f' {rule}: R; }}','CD.UNEXPECTED_TOKEN',rule)
 for name,labels in [('ONE',[ok]),('TWO',[ok,deg]),('FOUR',[ok,deg,no,ok]),('SIX',[ok,deg,no,ok,deg,no])]:
  block='{'+', '.join(labels)+'}'
  add('COUNT_'+name,start+block+end,'CD.INVALID_ADMISSIBILITY_STATE',block,role='conjunto_escrito')
 block='{'+ok+', -- interno é { }\r\n\t'+deg+'}'
 add('COUNT_COMMENTS',start+block+end,'CD.INVALID_ADMISSIBILITY_STATE',block,role='conjunto_escrito')
 block='{'+ok+'}'
 add('COUNT_BEFORE_EOF',start+block+';','CD.INVALID_ADMISSIBILITY_STATE',block,role='conjunto_escrito')
 add('COUNT_BEFORE_RULE',start+block+'; inventada: R; }','CD.INVALID_ADMISSIBILITY_STATE',block,role='conjunto_escrito')
 add('COUNT_EXCLUDES_TRAILING_COMMENT',start+block+' -- fuera é { }\r\n'+end,'CD.INVALID_ADMISSIBILITY_STATE',block,role='conjunto_escrito')
 add('EMPTY_PRECEDENCE',start+'{}'+end,'CD.UNEXPECTED_TOKEN','}')
 s=cases[-1]['source'];i=s.index('{}')+1;cases[-1]['span']=[len(s[:i].encode()),len(s[:i+1].encode())]
 add('TRAILING_COMMA_PRECEDENCE',start+'{'+ok+',}'+end,'CD.UNEXPECTED_TOKEN','}')
 s=cases[-1]['source'];i=s.index(',}')+1;cases[-1]['span']=[len(s[:i].encode()),len(s[:i+1].encode())]
 add('LABEL_PRECEDENCE',start+'{'+ok+', inventado}'+end,'CD.INVALID_ADMISSIBILITY_STATE','inventado')
 add('FOREIGN_LABEL_PRECEDENCE',start+'{'+foreign+'}'+end,'CD.FOREIGN_SURFACE',foreign)
 add('TOKEN_PRECEDENCE',start+'{7}'+end,'CD.UNEXPECTED_TOKEN','7')
 add('CLOSING_BRACE_PRECEDENCE',start+'{'+ok+'];','CD.UNEXPECTED_TOKEN',']')
 add('LIST_EOF_PRECEDENCE',start+'{'+ok,'CD.UNEXPECTED_END',eof=True)
 add('AFTER_BRACE_EOF_PRECEDENCE',start+'{'+ok+'}','CD.UNEXPECTED_END',eof=True)
 add('LEXICAL_PRECEDENCE',start+'{'+ok+'}'+end+'💣','CD.UNEXPECTED_TOKEN','💣')
 for n,labels in enumerate(itertools.permutations([ok,deg,no]),1):
  add('VALID_ORDER_'+str(n),start+'{'+', '.join(labels)+'}'+end,'OK',role='ninguno')
write('CASOS_NUEVOS.json',{'version':'STATE-LIST-SITES/1','casos':cases})
inherited=json.loads((r/'CASOS_HEREDADOS_160.json').read_text())['casos'];succession=[]
for c in inherited:
 if c['id'].startswith('STATE_CARDINALITY_PENDING_'):
  assert c['span'] is None;s=c['source'];a=s.rindex('{');b=s.index('}',a)+1
  c['span']=[len(s[:a].encode()),len(s[:b].encode())]
  succession.append(dict(id=c['id'],anterior=None,sucesor=c['span'],causa_conservada=c['cause'],fragmento=s[a:b]))
assert len(succession)==2
write('SUCESION_ESPERADOS.json',{'motivo':'Dos intervalos ausentes del relevo RETP-160 reciben el conjunto escrito completo. Se conservan fuentes, IDs, causas y documento anterior. Los otros 92 esperados permanecen intactos.','cambios':succession})
write('CASOS_ESPERADOS.json',{'version':'STATE-LIST-SITES/1','casos':cases+inherited})
plan='''# Plan previo: intervalo del conjunto escrito de estados

RETP-161. Cortes: Lenguaje 0ef4330b9bb18b09f5a386784d7d3baa559624b2; laboratorio 605de268874621538489554a3f1be0b83f1f4a1f, rama lab/playground-sv-permanente. Se continúa en las ramas existentes, sin apertura, fusión, limpieza ni renumeración de ramas o registros históricos. Base experimental: 77 archivos de RETP-160, expediente cotejado en ambos árboles.

Objeto único del paso 6: dar intervalo al rechazo actual states.len() != 3. Esta comprobación cuenta las etiquetas escritas, incluidas repeticiones; no calcula el número de valores distintos. Se preserva exactamente esa condición. El intervalo comienza en el byte de la llave de apertura y termina inmediatamente después de la llave de cierre, incluidas ambas llaves, espacios y comentarios interiores; excluye comentarios exteriores, separador posterior y campo rule. No se atribuye a una etiqueta individual un error de longitud de la colección escrita.

El emisor conservará los dos índices de token antes de consumir las llaves. El registro interno de localización se extenderá a primero/último, inclusivos; los emisores de un elemento registrarán el mismo índice en ambos extremos. La conversión a bytes utilizará los intervalos léxicos originales. El rechazo permanece después de comprobar la llave de cierre y el punto y coma, antes de rule. EOF, etiquetas inválidas y fallos de separador conservan su precedencia. La tokenización completa y sus rechazos léxicos mantienen prioridad.

Contrato observable: cobertura ampliada de COMPILER-DIAGNOSTICS/2, sin nueva causa, mensaje, variante pública, regla de admisión ni cambio IR. Dos esperados de rango ausente de RETP-160 son sucedidos expresamente, preservando el documento anterior; otros 92 esperados sin cambios. Los nuevos casos y rangos se fijan antes de cambiar la candidata, a partir de fragmentos originales seleccionados, sin leer su salida. Incluyen ES/EN, longitudes 1/2/4/6, seis permutaciones válidas por idioma, CRLF, acentos, comentarios con llaves y errores simultáneos para verificar precedencia. El conjunto vacío y la coma final conservan su rechazo sintáctico previo; no se crea una nueva forma gramatical.

Se comparan decisiones y errores heredados entre base y candidata, monofuente y ensamblaje mixto en ambas posiciones con nombre de archivo repetido. Se comprueban huella SHA-256, bytes y perfil de origen. Corpus canónico 14 válidas/106 inválidas, 239 unitarios por modo, focales y relación previos. Rust nativo debug/release y tres repeticiones focales; Python observa y recupera, nunca actúa como autoridad semántica.

Sensibilidad prevista, con código de salida 101: base sin intervalo detectada en COUNT_ONE_en; mutante que excluye la llave de cierre detectado en COUNT_ONE_en; mutante que adelanta el rechazo antes del separador detectado en SEMICOLON_PRECEDENCE_en. Catálogo 13 causas/26 mensajes intacto; cuatro clientes negativos con seis rechazos de acceso. Límites: 120 segundos por proceso y 2 MiB por flujo, comandos, identidad, salidas, tiempo, CPU y máximo acumulado RSS de hijos. La memoria incluye compilador y no acredita consumo incremental o por caso; no se constituye comparación de rendimiento, dinero ni inferencia. Reproducción sin red en directorio nuevo antes de publicación.

Rectoras consultadas en esta continuidad y cotejadas: AGENTS, Pilares completos, perfiles/ensamblaje, transición y relevos; contrato diagnóstico RETP-109/110, workflow V2, plan RETP-147. Referencia conceptual mantenida: LEAME_PRIMERO del frame, significado humano y fidelidad, adenda visual y trazabilidad/auditoría/reproducción. Localizar fielmente un error no prueba fidelidad del conocimiento presentado.

Se mantiene el paso 6, fila 9 y prioridades del relevo; no se cierra P6, DG global, DFL-001/011 ni otras deudas. Reserva P3 cerrada; P4/P5 con sus compuertas. Fuera: conversiones defensivas restantes y su alcanzabilidad, validadores y subcausas, presentación/serialización, WASI/navegador y recepción profesional. Sin ejecución externa ni promoción al núcleo productivo. El encargo común sigue pendiente.
'''
(r/'PLAN_PREVIO.md').write_text(plan)
write('COMPROMISO_PREVIO.json',{'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'archivos':{n:hashlib.sha256((r/n).read_bytes()).hexdigest() for n in ['PLAN_PREVIO.md','CASOS_NUEVOS.json','CASOS_HEREDADOS_160.json','SUCESION_ESPERADOS.json','CASOS_ESPERADOS.json']}})
print(len(cases),'fuentes nuevas;',len(cases+inherited),'totales')
