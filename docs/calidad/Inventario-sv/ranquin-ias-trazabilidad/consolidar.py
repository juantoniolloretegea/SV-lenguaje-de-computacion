from pathlib import Path
import json,csv,hashlib,sys,importlib.util
W=Path(sys.argv[1]).resolve() if len(sys.argv)>1 else Path(__file__).resolve().parent.parent
R=W/'s10-ranquin';D=R/'expediente';D.mkdir(exist_ok=True)
head=json.loads((R/'lenguaje-base.json').read_text())['head']
tree=json.loads((R/'lenguaje-arbol.json').read_text())
URL='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/'+head+'/'
inputs=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def record(p):
 b=p.read_bytes(); h=hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
 paths=[k for k,v in tree.items() if v==h and k.startswith('docs/calidad/tuberias-ia/')]
 assert paths,str(p)+' no coincide con el corte publicado'
 obj={'archivo_local_relativo':str(p.relative_to(W)),'bytes':len(b),'sha256':sha(b),'git_blob':h,'url':URL+sorted(paths)[0]}
 inputs.append(obj);return obj
def read(p):record(p);return json.loads(p.read_text())
def save(n,o):(D/n).write_text(json.dumps(o,ensure_ascii=False,indent=2)+'\n')
def csvout(n,rows):
 with (D/n).open('w',newline='') as f:
  w=csv.DictWriter(f,fieldnames=list(rows[0]),lineterminator='\n');w.writeheader();w.writerows(rows)
record(W/'s6-trazabilidad-total/publico/cotejar_entrega.py')
spec=importlib.util.spec_from_file_location('cotejador',W/'s6-trazabilidad-total/publico/cotejar_entrega.py');mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
allrows=[];detail=[];rank=[]
names=['grok','claude','qwen','deepseek','mistral']
positions=[1,1,3,4,5]
companies=['xAI','Anthropic','Alibaba / Qwen','DeepSeek','Mistral AI']
openmodels=[
 'Grok-1 (Apache-2.0); Grok-2 (xAI Community License). Versiones distintas de Grok 4.6 ensayado.',
 'No se identificaron pesos descargables de Claude en el catálogo oficial revisado.',
 'Qwen3.8-27B (Apache-2.0); Qwen3.8-2.4T-A95B (Qwen3.8-Max License). Variante ensayada no acreditada.',
 'DeepSeek-V4.1-Flash y DeepSeek-V4-Pro-0813 (MIT); R1 y V3.2 disponibles. No equivalen a V3 ensayado.',
 'Mistral-Medium-3.5-128B (MIT modificada); Small 4, Large 3 y Ministral 3 (Apache-2.0). Pesos no ensayados aquí.'
]
publication=[
 'Grok-1: 2024-03-17; Grok-2: subida 2025-08-22 y ficha 2025-08-23.',
 'No aplicable a pesos; Opus 5, servicio: 2026-07-24.',
 'Qwen3.8-27B: 2026-08-14; 2.4T-A95B: 2026-08-12 (anuncios del repositorio).',
 'V4.1-Flash: septiembre de 2026 (día exacto no fijado); V4-Pro-0813: agosto de 2026 (variante y repositorio).',
 'Medium 3.5: repositorio desde 2026-04-29, artículo 2026-05-22; Small 4: 2026-03-16; Large/Ministral 3: 2025-12-02.'
]
updates=[
 'Grok-2: último cambio visible de licencia 2025-11-05; no acredita pesos nuevos. Grok-1 histórico, disponible.',
 'Servicio Opus 5 activo; catálogo y política de retirada vigentes. No hay rama abierta identificada.',
 'Familia activa: lanzamientos de agosto de 2026 y repositorio mantenido; no promesa de actualización de cada peso.',
 'Familia activa: V4.1-Flash con corrección de codificación visible dos días antes de la consulta; no equivale a pesos nuevos.',
 'Medium 3.5: cambio de configuración 2026-07-15; familia con publicaciones de 2026. No equivale a nueva formación de pesos.'
]
abandon=[
 'No se encontró anuncio de abandono en las páginas oficiales revisadas. Actividad escasa de estos pesos no demuestra abandono.',
 'No aplicable a una rama abierta no identificada. Opus 5 figura activo; la retirada de versiones anteriores no es abandono de Claude.',
 'No se encontró anuncio de abandono en las fuentes oficiales revisadas; hay lanzamientos recientes.',
 'No se encontró anuncio de abandono en las fuentes oficiales revisadas; hay lanzamientos recientes.',
 'No se encontró anuncio de abandono en las fuentes oficiales revisadas; hay actividad reciente.'
]
sourceurls=[
 'https://github.com/xai-org/grok-1 ; https://huggingface.co/xai-org/grok-2 ; https://huggingface.co/xai-org/grok-2/commits/main ; https://huggingface.co/xai-org/grok-2/blob/main/LICENSE ; https://x.ai/news',
 'https://platform.claude.com/docs/en/models/overview ; https://www.anthropic.com/news/claude-opus-5 ; https://platform.claude.com/docs/en/about-claude/model-deprecations',
 'https://github.com/QwenLM/Qwen3.8 ; https://huggingface.co/Qwen/Qwen3.8-27B ; https://huggingface.co/Qwen/Qwen3.8-2.4T-A95B ; https://huggingface.co/Qwen/Qwen3.8-2.4T-A95B/blob/main/LICENSE',
 'https://huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash ; https://huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash/commits/main ; https://huggingface.co/deepseek-ai/DeepSeek-V4-Pro-0813 ; https://github.com/deepseek-ai/DeepSeek-R1 ; https://huggingface.co/deepseek-ai/DeepSeek-V3.2',
 'https://huggingface.co/mistralai/Mistral-Medium-3.5-128B ; https://huggingface.co/mistralai/Mistral-Medium-3.5-128B/blob/main/LICENSE ; https://huggingface.co/mistralai/Mistral-Medium-3.5-128B/commits/main ; https://mistral.ai/news/vibe-remote-agents-mistral-medium-3-5/ ; https://mistral.ai/news/mistral-small-4/ ; https://mistral.ai/news/mistral-3/'
]
for i,n in enumerate(names):
 a=W/('s5-recepcion-externa/entrega/'+n if n in ['deepseek','qwen','claude'] else 's5-recepcion-grok/entrega/grok' if n=='grok' else 's8-recepcion-mistral/expediente')
 b=W/('s9-recepcion-mistral/expediente' if n=='mistral' else 's7-recepcion-'+n+'/expediente')
 ev=read(a/'EVALUACION.json');r1=read(a/'RECEPCION.json');evref=record(a/'EVALUACION.json')
 original=next(a.glob('RESPUESTA_ORIGINAL.*')) if n!='mistral' else a/'RESPUESTA_TRANSCRITA.json'
 origref=record(original)
 r2=read(b/'RECEPCION.json');co=read(b/'COTEJO_ORIGINAL.json');coref=record(b/'COTEJO_ORIGINAL.json');rawref=record(b/'RESPUESTA_ORIGINAL.txt')
 assert co['respuesta_sha256']==rawref['sha256']
 parsed=read(b/'BLOQUE_JSON_DIAGNOSTICO.json') if n=='qwen' else mod.parse((b/'RESPUESTA_ORIGINAL.txt').read_bytes())
 diag=read(b/'COTEJO_BLOQUE_DIAGNOSTICO.json') if n=='qwen' else co
 participant=r2['participante_declarado']
 s4cases={c['id']:c for c in ev['casos']}
 s6cases={c['id']:c for c in parsed['resultados']}
 assert len(s4cases)==len(s6cases)==12
 for id,c in s4cases.items():
  allrows.append({'ia':n,'intento':1,'banco':'S4-EXTERNA-DOCUMENTAL/1','caso':id,'dictamen_entrega':ev['dictamen'],'resultado_caso':str(c['resultado_literal'])+'/4; traza '+str(c['trazabilidad'])+'/3','observaciones':c.get('evidencia_evaluacion',c.get('justificacion_evaluador','')),'respuesta_sha256':origref['sha256'],'url_evaluacion':evref['url']})
 for id,c in s6cases.items():
  err=[e for e in diag['errores'] if '/'+id+'/' in e or e.startswith('/resultados/'+id+':')]
  allrows.append({'ia':n,'intento':2,'banco':'SV-TRAZABILIDAD-2/1','caso':id,'dictamen_entrega':co['dictamen'],'resultado_caso':('CONFORME_EN_BLOQUE_DIAGNOSTICO' if n=='qwen' else 'SIN_DISCREPANCIAS') if not err else 'CON_DISCREPANCIAS','observaciones':' ; '.join(err) or ('Original incumple formato; caso cotejado sólo en extracción diagnóstica.' if n=='qwen' else ''),'respuesta_sha256':rawref['sha256'],'url_evaluacion':coref['url']})
 exact=32-sum('/fuentes/' in e for e in diag['errores'])
 detail.append({'ia':n,'S4':{'recepcion':r1,'evaluacion':ev,'original':origref},'S6':{'recepcion':r2,'cotejo_original':co,'cotejo_diagnostico_si_qwen':diag if n=='qwen' else None,'original':rawref,'resultados_recibidos':list(s6cases.values()),'citas_fuentes_exactas':exact},'evaluaciones_inmutables':[evref,coref]})
 rank.append({'puesto_documental':positions[i],'ia':n,'empresa':companies[i],'version_declarada_S6':participant.get('version_declarada'),'dictamen_S6_original':co['dictamen'],'citas_fuentes_exactas_S6':exact,'citas_fuentes_requeridas_S6':32,'S4_resultado_sobre_48':ev['resultado_literal_0_a_48'],'S4_traza_sobre_36':ev['trazabilidad_0_a_36'],'S4_procedimiento_sobre_8':ev['procedimiento_0_a_8'],'S4_entrega_sobre_8':ev['entrega_0_a_8'],'S4_total_literal_no_definitivo':ev['total_literal_0_a_100'],'S4_puntos_reservados':ev['puntos_reservados_por_ambiguedad_publica'],'decision_de_seleccion':['Conformidad documental recibida; selección de implantación pendiente.','Conformidad documental recibida; exposición alta impide comparación independiente.','Aceptación provisional por Dirección; NO_CONFORME original conservado.','Sin conformidad completa; defecto de fidelidad acotado, corrección no validada.','Descartado por Dirección de la selección actual tras segunda entrega NO_CONFORME.'][i],'exposicion_declarada':participant.get('exposicion_previa_declarada'),'tiempo_verificado_s':None,'coste_verificado':None,'modelos_con_pesos_disponibles':openmodels[i],'publicacion':publication[i],'actualizacion':updates[i],'anuncio_abandono':abandon[i],'fuentes_oficiales':sourceurls[i],'evaluacion_S4':evref['url'],'cotejo_S6':coref['url']})
save('RESULTADOS_DETALLADOS.json',{'version':'SV-RANQUIN-RESULTADOS/1','fecha':'2026-09-12','unidad':'Watson / W-S0','alcance':'Consolidación de dictámenes existentes; no nueva ejecución de participantes ni sustitución de originales. Identidades declaradas. Los tiempos del observador no son tiempos del participante.','participantes':detail})
save('RANQUIN.json',rank)
csvout('RANQUIN.csv',rank);csvout('RESULTADOS_POR_CASO.csv',allrows)
unique={o['archivo_local_relativo']:o for o in inputs}
save('EVIDENCIAS_DE_ENTRADA.json',{'corte_publico':head,'archivos':[unique[k] for k in sorted(unique)],'verificacion':'Cada archivo local usado coincide con un blob presente en el árbol del corte público indicado; SHA-256 recalculado. Los cotejos se consolidan sin reinterpretar su verificador.'})
save('FUENTES_OFICIALES.json',{'consulta':'2026-09-12','metodo':'Lectura de páginas oficiales, fichas de pesos, licencias e historiales mediante navegador de consulta web. Las URL pueden evolucionar. No se conserva una captura íntegra ni se atribuye una huella a la página remota.','alcance_inventario':'Selección de modelos y familias relevantes con enlaces al editor; no inventario exhaustivo de todas las variantes y cuantizaciones.','ausencia_anuncio':'No encontrado en las páginas revisadas; no prueba negativa universal ni compromiso de mantenimiento futuro.','proveedores':[{'empresa':companies[i],'publicacion':publication[i],'actualizacion':updates[i],'abandono':abandon[i],'urls':sourceurls[i].split(' ; ')} for i in range(5)]})
assert len(allrows)==120 and len(rank)==5
assert [x['citas_fuentes_exactas_S6'] for x in rank]==[32,32,32,6,24]
for p in D.glob('*.csv'):
 rows=list(csv.reader(p.open(newline='')));assert all(len(r)==len(rows[0]) for r in rows)
print('Consolidados: 5 participantes, 10 entregas, 120 casos; entradas comprobadas contra Git.')
