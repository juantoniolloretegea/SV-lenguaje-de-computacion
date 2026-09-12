from pathlib import Path
import json,hashlib,shutil,ast
R=Path(__file__).resolve().parent;W=R.parent;D=R/'entrega';P='docs/calidad/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/';cut=json.loads((R/'lenguaje-base.json').read_text());tree=json.loads((R/'lenguaje-arbol.json').read_text());sha=lambda b:hashlib.sha256(b).hexdigest();blob=lambda b:hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest();sources=[]
def copy(local,remote,dest):
 b=local.read_bytes();assert tree[remote]==blob(b),remote;q=D/dest;q.parent.mkdir(parents=True,exist_ok=True);q.write_bytes(b);sources.append(dict(destino=dest,ruta_origen=remote,corte=cut['head'],blob=blob(b),bytes=len(b),sha256=sha(b),url='https://github.com/'+cut['repo']+'/blob/'+cut['head']+'/'+remote))
for n in ['CONTRATO_RECORRIDO_CONJUNTO.md','BANCO_OBLIGACIONES.json','DIAGNOSTICOS.json']:
 copy(W/'s17-contrato/entrega'/n,P+'s17-contrato-recorrido-conjunto/'+n,'contrato-s17/'+n)
for p in sorted((W/'s17-contrato/entrega/esperados').iterdir()):copy(p,P+'s17-contrato-recorrido-conjunto/esperados/'+p.name,'esperados/'+p.name)
copy(W/'s14-bases/entrega/FUENTES_S2.json',P+'s14-bases-y-reevaluacion/FUENTES_S2.json','FUENTES_S2.json')
x=json.loads((W/'s17-contrato/entrega/RECTORES.json').read_text());x['corte']=cut['head'];x['objeto_actual']='S18: adaptación Rust y campaña del recorrido conjunto bajo S17.'
for p,h in x['piezas_sin_cambio_desde_lectura_previa'].items():assert tree[p]==h
(D/'RECTORES.json').write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
(D/'PROCEDENCIA.json').write_text(json.dumps(dict(version='S18-PROCEDENCIA/1',cortes={k:json.loads((R/(k+'-base.json')).read_text()) for k in ['lenguaje','laboratorio']},fuentes=sources),ensure_ascii=False,indent=2)+'\n')
bank=json.loads((D/'contrato-s17/BANCO_OBLIGACIONES.json').read_text());(D/'codigo/ESPERADO.tsv').write_text(''.join(f"{x['id']}\t{x['etapa_esperada']}\t{x['causa_esperada']}\n" for x in bank['casos']))
print('Fuentes cotejadas:',len(sources),'; esperados derivados del banco S17 intacto, no de la realización.')
