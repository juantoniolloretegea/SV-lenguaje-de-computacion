"""Cotejo documental S4: comprobaciones mecánicas; no ejecución del participante ni semántica SV."""
from pathlib import Path
import json,hashlib,re,sys
R=Path(sys.argv[1]);S=Path(sys.argv[2])
def sha(b):return hashlib.sha256(b).hexdigest()
def save(n,o):(R/n).write_text(json.dumps(o,ensure_ascii=False,indent=2)+'\n')
a=json.loads((R/'RESPUESTA_TRANSCRITA.json').read_text());com=json.loads((S/'publico/COMPROMISO_PREVIO.json').read_text())
for p,k in [('privado/ORACULO_PREVIO.json','oraculo_sha256'),('publico/FUENTES.json','fuentes_sha256'),('publico/RUBRICA.md','rubrica_sha256'),('publico/PRUEBA_COMUN.md','documento_comun_sha256'),('publico/CASOS.json','casos_sha256')]:assert sha((S/p).read_bytes())==com[k]
f={x['id']:x for x in json.loads((S/'publico/FUENTES.json').read_text())['fuentes']}
for x in f.values():assert sha(x['texto'].encode())==x['sha256'] and len(x['texto'].encode())==x['bytes']
o=json.loads((S/'privado/ORACULO_PREVIO.json').read_text());by={x['id']:x for x in a['resultados']};assert sorted(by)==['E%02d'%i for i in range(1,13)] and len(a['resultados'])==12
hashes=re.findall(r'\b[0-9a-f]{64}\b',by['E09']['evidencia_literal']);assert hashes==[f['F07']['sha256'],f['F06']['sha256']]
line=next(i+1 for i,x in enumerate(f['F06']['texto'].splitlines()) if '"contenido":"8.40"' in x);assert line==11
cases=[]
for c in o['casos']:
 r=by[c['id']];cases.append(dict(id=c['id'],decision_recibida=r['decision'],decision_oraculo=c['esperado']['decision'],causa_recibida=r['causa_o_resolucion'],causa_oraculo=c['esperado']['causa'],fuentes_requeridas=c['fuentes'],fuentes_requeridas_presentes=all(x in r['fuentes'] for x in c['fuentes']),contenido_recibido=r['contenido'],contenido_oraculo=c['esperado']['contenido']))
assert all(x['fuentes_requeridas_presentes'] for x in cases)
save('COTEJO_MECANICO.json',dict(banco='S4-EXTERNA-DOCUMENTAL/1',sha256_transcripcion=sha((R/'RESPUESTA_TRANSCRITA.json').read_bytes()),compromiso=com,casos=cases,huellas_E09_coinciden=True,linea_real_contenido_F06=line,linea_declarada=10,alcance='Datos para revisión S4; las equivalencias y puntuaciones se justifican en EVALUACION.json. No aplica el verificador S6. No certifica lectura remota ni actividad del participante.'))
print('12 IDs y fuentes requeridas presentes; huellas E09 coinciden; E01 fragmento en línea 11, no 10.')
