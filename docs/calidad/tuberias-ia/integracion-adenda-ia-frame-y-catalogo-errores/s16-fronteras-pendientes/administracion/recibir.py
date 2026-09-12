from pathlib import Path
import json,hashlib,base64,sys,concurrent.futures
R=Path(__file__).resolve().parent;W=R.parent;D=R/'entrega';D.mkdir(exist_ok=True);L=R/'lectura';L.mkdir(exist_ok=True);sys.path.insert(0,str(W/'s6-trazabilidad-total'));from github_io import call
base=json.loads((R/'lenguaje-base.json').read_text());tree=json.loads((R/'lenguaje-arbol.json').read_text());sha=lambda b:hashlib.sha256(b).hexdigest();blob=lambda b:hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest();P='docs/calidad/tuberias-ia/';I=P+'integracion-adenda-ia-frame-y-catalogo-errores/'
sources={}
for n in ['authority','permission','mediation','execution','decision_trace','ir']:
 p='rust/sv_core/src/'+n+'.rs';sources[p]=W/'s13-suficiencia/lectura'/p
for p in ['IR_CANONICA_BIENFORMACION_SV_v0_3.md','FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md','docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md']:sources[p]=W/'s13-suficiencia/lectura'/p
for s,n in [('pertenencia-de-permisos-a-su-continuidad','README.md'),('recepcion-gobernada-y-comprobacion-observada','CONTRATO_Y_FRONTERA.md'),('integracion-adenda-ia-frame-y-catalogo-errores','G2_CORRESPONDENCIA_Y_LIMITE.md')]:sources[P+s+'/'+n]=W/'revision-162/fuentes'/s/n
sources['docs/calidad/tuberias-ia/WORKFLOW_ACOTADO_SUBORDINACION_IA_ES_V2_2026_09_11.md']=W/'revision-162/fuentes/WORKFLOW_ACOTADO_SUBORDINACION_IA_ES_V2_2026_09_11.md'
for folder,local,n in [('s15-causas-recepcion','s15-causas','MATRIZ_COBERTURA_A_L_RESULTADO_S15.json'),('s15-causas-recepcion','s15-causas','ACTA_RESULTADO_S15.md'),('s15-causas-recepcion','s15-causas','codigo/receptor.rs'),('s14-bases-y-reevaluacion','s14-bases','ACTA_RESULTADO_S14.md'),('s13-suficiencia-semantica-ir','s13-suficiencia','ACTA_SUFICIENCIA.md'),('s12-rectificacion-rumbo','s12-rumbo','ACTA_RECTIFICACION_DE_RUMBO.md')]:sources[I+folder+'/'+n]=W/local/'entrega'/n
sources['docs/calidad/CONTRATO_DE_CONSUMO_DOCUMENTAL_CYB_2026_09_09.md']=None
sources[P+'pertenencia-de-permisos-a-su-continuidad/PRODUCTORES_Y_LIMITE_DEL_ENLACE.md']=None
# Identificación exacta; ninguna reutilización de candidatas o mutantes como main.
def get(item):
 p,local=item;assert p in tree,p
 if local and local.is_file() and blob(local.read_bytes())==tree[p]:b=local.read_bytes();via='copia local cotejada contra blob del corte'
 else:
  z=call('fetch_file',dict(repository_full_name=base['repo'],path=p,ref=base['head']));b=z['content'].encode();assert z['sha']==tree[p];via='GitHub por commit'
 assert blob(b)==tree[p];q=L/p;q.parent.mkdir(parents=True,exist_ok=True);q.write_bytes(b)
 return p,dict(blob=tree[p],bytes=len(b),sha256=sha(b),corte=base['head'],url='https://github.com/'+base['repo']+'/blob/'+base['head']+'/'+p,via=via,base64=base64.b64encode(b).decode())
with concurrent.futures.ThreadPoolExecutor(3) as ex:got=dict(ex.map(get,sources.items()))
(D/'FUENTES_RECIBIDAS.json').write_text(json.dumps(got,ensure_ascii=False,indent=2)+'\n')
x=json.loads((W/'s15-causas/entrega/RECTORES.json').read_text());x['corte']=base['head'];x['objeto_actual']='S16: revisión documental B/E/K/L y fronteras de enlace; sin ejecución nueva o modificación nuclear.'
for p,h in x['piezas_sin_cambio_desde_lectura_previa'].items():assert tree[p]==h
(D/'RECTORES.json').write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
print(len(got),'fuentes cotejadas con corte',base['head'])
