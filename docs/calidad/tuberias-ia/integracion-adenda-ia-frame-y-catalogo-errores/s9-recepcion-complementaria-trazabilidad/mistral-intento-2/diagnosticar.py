from pathlib import Path
import json,sys,hashlib
D=Path(__file__).resolve().parent; S=D.parent.parent/'s6-trazabilidad-total'
sys.path.insert(0,str(S/'publico'));from cotejar_entrega import parse
raw=(D/'RESPUESTA_ORIGINAL.txt').read_bytes(); a=parse(raw)
bank=json.loads((S/'publico/BANCO.json').read_text()); ref=json.loads((S/'privado/REFERENCIA_PREVIA.json').read_text())
f={x['id']:x['texto'] for x in bank['fuentes']}; expected={x['id']:x for x in ref['resultados']}
bad=[]; differences=[];good=[]
for c in a['resultados']:
 before=len(differences)+len(bad)
 for x in c['fuentes']:
  actual=x['texto']; canonical=f[x['id']]
  if actual!=canonical:
   assert actual.replace('\\n','\n')==canonical
   bad.append(dict(caso=c['id'],fuente=x['id'],bytes_recibidos=len(actual.encode()),bytes_canonicos=len(canonical.encode()),sha256_recibido=hashlib.sha256(actual.encode()).hexdigest(),sha256_canonico=hashlib.sha256(canonical.encode()).hexdigest(),pares_barra_n=actual.count('\\n'),diagnostico='Los dos caracteres barra invertida y n sustituyen cada LF; doble escape en el JSON recibido. La sustitución diagnóstica produce igualdad exacta; no se modifica la entrega.'))
 for x in c['reglas']:
  if x['texto']!=bank['reglas'][x['id']]:differences.append(dict(caso=c['id'],campo='reglas/'+x['id'],recibido=x['texto'],esperado=bank['reglas'][x['id']]))
 for k in ['decision','causa','contenido','llamadas_politica','caso_texto','fundamento','consecuencia','limites']:
  if c[k]!=expected[c['id']][k]:differences.append(dict(caso=c['id'],campo=k,recibido=c[k],esperado=expected[c['id']][k]))
 if before==len(differences)+len(bad):good.append(c['id'])
assert len(bad)==8 and len(differences)==4
out=dict(citas_totales=32,citas_exactas=24,citas_no_exactas=bad,otras_discrepancias=differences,casos_sin_discrepancias=good,dictamen='NO_CONFORME',limite='Diagnóstico de la representación recibida. No acredita qué componente introdujo los escapes ni la causa interna de los errores. No sustituye el cotejo fijado.')
(D/'DIAGNOSTICO.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(out,ensure_ascii=False,indent=2))
