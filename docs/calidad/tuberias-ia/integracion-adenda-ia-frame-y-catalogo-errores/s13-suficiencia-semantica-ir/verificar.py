"""Cotejo de integridad documental S13; no es un verificador semántico."""
import argparse,hashlib,json
from pathlib import Path
D=Path(__file__).resolve().parent
sha=lambda b:hashlib.sha256(b).hexdigest()
blob=lambda b:hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
def require(ok,message):
 if not ok:raise ValueError(message)
def main():
 ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--raiz-fuentes',required=True,type=Path);a=ap.parse_args()
 sources=json.loads((D/'FUENTES_RECIBIDAS.json').read_text());ps=json.loads((D/'PASAJES_COTEJADOS.json').read_text());matrix=json.loads((D/'MATRIZ_SUFICIENCIA.json').read_text());data={}
 for path,v in sources.items():
  b=(a.raiz_fuentes/path).read_bytes();require(len(b)==v['bytes'] and sha(b)==v['sha256'] and blob(b)==v['blob'],'Fuente distinta: '+path);require(v['corte']==matrix['corte'],'Corte distinto: '+path);data[path]=b
 for key,v in ps.items():
  lines=data[v['fuente']].splitlines(keepends=True);lo,hi=v['linea_inicio'],v['linea_fin'];require(1<=lo<=hi<=len(lines),'Intervalo: '+key);b=b''.join(lines[lo-1:hi]);require(len(b)==v['bytes'] and sha(b)==v['sha256'] and b==v['texto'].encode(),'Pasaje distinto: '+key)
 rows=matrix['obligaciones'];require([r['id'] for r in rows]==['O'+str(i).zfill(2) for i in range(1,13)],'Identidades de obligaciones')
 fields=['obligacion','sede','contrato','representacion_ir','realizacion_rust','evidencia_disponible','control_positivo','contraejemplo_o_control_negativo','perdida_o_limite','tratamiento','pasajes']
 for r in rows:
  require(all(r.get(k) for k in fields),'Campo ausente: '+r['id']);require(all(k in ps for k in r['pasajes']),'Pasaje sin fuente');require(r['ensayo_funcional_nuevo_en_S13'] is False,'Falso nuevo ensayo')
 require(set('ABCDEFGHIJKL')=={c for r in rows for c in r['criterios_A_L']},'Cobertura del inventario A–L')
 for c,ids in matrix['cobertura_criterios'].items():require(ids==[r['id'] for r in rows if c in r['criterios_A_L']],'Mapa distinto '+c)
 prior_path=next(p for p in data if p.endswith('MATRIZ_COBERTURA_A_L_RESULTADO_S11.json'));prior=json.loads(data[prior_path]);require(matrix['criterios_anteriores_preservados']==[c['original_147'] for c in prior['casos']],'Criterios anteriores modificados')
 mf=D/'MANIFIESTO.json'
 if mf.exists():
  for p,v in json.loads(mf.read_text()).items():
   b=(D/p).read_bytes();require(len(b)==v['bytes'] and sha(b)==v['sha256'],'Artefacto distinto: '+p)
 print(json.dumps(dict(resultado='CONFORME_INTEGRIDAD_DOCUMENTAL',fuentes=len(sources),pasajes=len(ps),obligaciones=len(rows),criterios=12,ensayos_funcionales_nuevos=0,manifiesto_comprobado=mf.exists(),limite='No demuestra por sí solo suficiencia semántica ni resultados funcionales.'),ensure_ascii=False))
if __name__=='__main__':main()
