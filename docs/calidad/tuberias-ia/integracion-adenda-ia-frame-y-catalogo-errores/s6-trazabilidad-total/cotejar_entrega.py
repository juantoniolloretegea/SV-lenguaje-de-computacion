#!/usr/bin/env python3
"""Cotejo de archivos contra referencia fijada. No ejecuta ni redefine semántica SV."""
import argparse,hashlib,json,re,sys
from pathlib import Path
MAX_BYTES=262144
TOP={'version','banco_sha256','participante','resultados'}
FIELDS={'id','decision','causa','contenido','llamadas_politica','caso_texto','fuentes','reglas','fundamento','consecuencia','limites'}
class Formato(ValueError):pass
def digest(data):return hashlib.sha256(data).hexdigest()
def unique(pairs):
 out={}
 for k,v in pairs:
  if k in out:raise Formato('CLAVE_DUPLICADA:'+k)
  out[k]=v
 return out
def parse(data):
 if len(data)>MAX_BYTES:raise Formato('LIMITE_ARCHIVO')
 text=data.decode('utf-8').strip()
 if text.startswith('```'):
  m=re.fullmatch(r'```json\s*\n([\s\S]*?)\n```',text)
  if not m:raise Formato('BLOQUE_JSON_INVALIDO')
  text=m[1]
 return json.loads(text,object_pairs_hook=unique,parse_constant=lambda x:(_ for _ in ()).throw(Formato('NUMERO_NO_JSON:'+x)))
def index(items,key,path,errors):
 if not isinstance(items,list):errors.append(path+':TIPO_LISTA');return {}
 out={}
 for item in items:
  if not isinstance(item,dict) or not isinstance(item.get(key),str):errors.append(path+':IDENTIDAD_INVALIDA');continue
  v=item[key]
  if v in out:errors.append(path+':IDENTIDAD_DUPLICADA:'+v)
  out[v]=item
 return out
def equal(a,b):
 if type(a) is not type(b):return False
 if isinstance(a,dict):return a.keys()==b.keys() and all(equal(a[k],b[k]) for k in a)
 if isinstance(a,list):return len(a)==len(b) and all(equal(x,y) for x,y in zip(a,b))
 return a==b
def compare(answer,reference,bank):
 errors=[]
 if not isinstance(answer,dict):return ['/:TIPO_OBJETO']
 if set(answer)!=TOP:errors.append('/:CAMPOS')
 for k in ['version','banco_sha256']:
  if not equal(answer.get(k),reference[k]):errors.append('/'+k+':VALOR')
 p=answer.get('participante');pk={'modelo','version_declarada','exposicion_previa_declarada'}
 if not isinstance(p,dict) or set(p)!=pk:errors.append('/participante:CAMPOS')
 else:
  for k in ['modelo','exposicion_previa_declarada']:
   if not isinstance(p[k],str) or not p[k].strip():errors.append('/participante/'+k+':TEXTO_REQUERIDO')
  if p['version_declarada'] is not None and (not isinstance(p['version_declarada'],str) or not p['version_declarada'].strip()):errors.append('/participante/version_declarada:TIPO')
 actual=index(answer.get('resultados'),'id','/resultados',errors);expected={x['id']:x for x in reference['resultados']}
 if actual.keys()!=expected.keys():errors.append('/resultados:COBERTURA_CASOS')
 for id,want in expected.items():
  if id not in actual:continue
  got=actual[id];path='/resultados/'+id
  if set(got)!=FIELDS:errors.append(path+':CAMPOS')
  for k in ['id','decision','causa','contenido','llamadas_politica','caso_texto','fundamento','consecuencia']:
   if not equal(got.get(k),want[k]):errors.append(path+'/'+k+':NO_COINCIDE')
  for k in ['fuentes','reglas']:
   have=index(got.get(k),'id',path+'/'+k,errors);need={x['id']:x for x in want[k]}
   if have.keys()!=need.keys():errors.append(path+'/'+k+':COBERTURA')
   for fid,entry in need.items():
    if fid in have and not equal(have[fid],entry):errors.append(path+'/'+k+'/'+fid+':CITA_NO_EXACTA')
  lim=got.get('limites')
  if not isinstance(lim,list) or any(not isinstance(x,str) for x in lim):errors.append(path+'/limites:TIPO')
  elif len(set(lim))!=len(lim) or set(lim)!=set(want['limites']):errors.append(path+'/limites:COBERTURA')
 return errors
def instrument(bank_data,reference_data,commitment):
 if digest(bank_data)!=commitment['banco_sha256'] or digest(reference_data)!=commitment['referencia_sha256']:raise Formato('HUELLA_INSTRUMENTO')
 bank=parse(bank_data);ref=parse(reference_data)
 if ref['banco_sha256']!=digest(bank_data):raise Formato('REFERENCIA_OTRO_BANCO')
 expected_ids=['E%02d'%i for i in range(1,13)]
 if sorted(x['id'] for x in bank['casos'])!=expected_ids or sorted(x['id'] for x in ref['resultados'])!=expected_ids:raise Formato('REFERENCIA_CASOS')
 sources={x['id']:x['texto'] for x in bank['fuentes']};rules=bank['reglas']
 for f in bank['fuentes']:
  b=f['texto'].encode();
  if len(b)!=f['bytes'] or digest(b)!=f['sha256']:raise Formato('FUENTE_HUELLA')
 cs={x['id']:x for x in bank['casos']}
 for r in ref['resultados']:
  c=cs[r['id']]
  if set(r)!=FIELDS or r['caso_texto']!=c['enunciado']:raise Formato('REFERENCIA_ESQUEMA')
  if r['fuentes']!=[{'id':x,'texto':sources[x]} for x in c['fuentes']]:raise Formato('REFERENCIA_FUENTES')
  if r['reglas']!=[{'id':x,'texto':rules[x]} for x in c['reglas_a_citar']]:raise Formato('REFERENCIA_REGLAS')
  if r['limites']!=c['limites_a_declarar'] or r['fundamento'] not in bank['fundamentos']:raise Formato('REFERENCIA_TRAZA')
  if r['decision'] not in bank['decisiones'] or r['causa'] not in bank['causas']:raise Formato('REFERENCIA_ETIQUETA')
 if compare(ref,ref,bank):raise Formato('REFERENCIA_INVALIDA')
 return bank,ref
def run(answer_data,bank_data,reference_data,commitment):
 try:bank,ref=instrument(bank_data,reference_data,commitment)
 except (ValueError,KeyError,TypeError,RecursionError,UnicodeError) as e:return {'dictamen':'ERROR_INSTRUMENTO','errores':[str(e)],'admisible_documental':False}
 try:answer=parse(answer_data);errors=compare(answer,ref,bank)
 except (ValueError,KeyError,TypeError,RecursionError,UnicodeError) as e:errors=['/:'+str(e)]
 return {'dictamen':'NO_CONFORME' if errors else 'CONFORME_DOCUMENTAL','errores':errors,'admisible_documental':not errors,'respuesta_sha256':digest(answer_data),'bytes_respuesta':len(answer_data),'casos_exigidos':12,'criterio':'todas las obligaciones; sin compensación','actividad_externa':'no certificada por este cotejo; cualquier afirmación adicional requiere su expediente y revisión antes de admisión general'}
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--respuesta',required=True);ap.add_argument('--banco',required=True);ap.add_argument('--referencia',required=True);ap.add_argument('--compromiso',required=True);ap.add_argument('--salida',required=True);a=ap.parse_args()
 try:
  com=parse(Path(a.compromiso).read_bytes())
  if com['verificador_sha256']!=digest(Path(__file__).read_bytes()):raise Formato('VERIFICADOR_NO_FIJADO')
  ans=Path(a.respuesta).read_bytes();result=run(ans,Path(a.banco).read_bytes(),Path(a.referencia).read_bytes(),com)
 except (OSError,ValueError,KeyError,TypeError,RecursionError) as e:result={'dictamen':'ERROR_INSTRUMENTO','errores':[str(e)],'admisible_documental':False}
 Path(a.salida).write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(result['dictamen']);return 0 if result['dictamen']=='CONFORME_DOCUMENTAL' else 2 if result['dictamen']=='NO_CONFORME' else 3
if __name__=='__main__':sys.exit(main())
