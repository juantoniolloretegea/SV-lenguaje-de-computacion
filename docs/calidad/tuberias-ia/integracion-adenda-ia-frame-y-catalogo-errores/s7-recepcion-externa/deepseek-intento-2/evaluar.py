"""Recepción original y diagnóstico de fidelidad; no repara ni reclasifica la entrega."""
from pathlib import Path
from datetime import datetime,timezone
import sys,json,hashlib,subprocess,time,re
R=Path(__file__).resolve().parent;S=R.parent/'s6-trazabilidad-total';D=R/'expediente'
def sha(b):return hashlib.sha256(b).hexdigest()
def save(n,o):(D/n).write_text(json.dumps(o,ensure_ascii=False,indent=2)+'\n')
b=(R.parent/'upload/Pegado text(9).txt').read_bytes();(D/'RESPUESTA_ORIGINAL.txt').write_bytes(b)
cmd=[sys.executable,str(S/'publico/cotejar_entrega.py'),'--respuesta',str(D/'RESPUESTA_ORIGINAL.txt'),'--banco',str(S/'publico/BANCO.json'),'--referencia',str(S/'privado/REFERENCIA_PREVIA.json'),'--compromiso',str(S/'publico/COMPROMISO_PREVIO.json'),'--salida',str(D/'COTEJO_ORIGINAL.json')]
t=datetime.now(timezone.utc).isoformat();start=time.perf_counter_ns();p=subprocess.run(cmd,capture_output=True);dur=time.perf_counter_ns()-start
save('EJECUCION_OBSERVADOR.json',dict(inicio_utc=t,comando=cmd,codigo_salida=p.returncode,stdout=p.stdout.decode(),stderr=p.stderr.decode(),duracion_ns_observador=dur,alcance='Cotejo de archivos y diagnóstico por Watson / W-S0; no mide ejecución del participante ni ejecuta semántica SV.'))
sys.path.insert(0,str(S/'publico'));import cotejar_entrega as c
com=c.parse((S/'publico/COMPROMISO_PREVIO.json').read_bytes())
for name,key in [('cotejar_entrega.py','verificador_sha256'),('BANCO.json','banco_sha256'),('CONTRATO.md','contrato_sha256'),('PRUEBA_COMUN.md','documento_sha256')]:assert sha((S/'publico'/name).read_bytes())==com[key]
bank,ref=c.instrument((S/'publico/BANCO.json').read_bytes(),(S/'privado/REFERENCIA_PREVIA.json').read_bytes(),com)
a=c.parse(b);rr={x['id']:x for x in ref['resultados']};diffs=[];cases=[];unique={};total=0
def tokens(text):
 inside=False;escape=False;out=[]
 for ch in text:
  if inside:
   out.append(ch)
   if escape:escape=False
   elif ch=='\\':escape=True
   elif ch=='"':inside=False
  elif ch=='"':inside=True;out.append(ch)
  elif ch not in ' \t\r\n':out.append(ch)
 assert not inside
 return ''.join(out)
for case in a['resultados']:
 expected=rr[case['id']];bad=[]
 for f in case['fuentes']:
  total+=1;want=next(x['texto'] for x in expected['fuentes'] if x['id']==f['id']);got=f['texto']
  if got==want:continue
  assert c.equal(c.parse(got.encode()),c.parse(want.encode()))
  assert tokens(got)==tokens(want)
  # Stronger check: only spaces at the start of each decoded source line differ.
  assert re.sub(r'(?m)^ +','',got)==re.sub(r'(?m)^ +','',want)
  assert len(want)>len(got)
  item=dict(caso=case['id'],fuente=f['id'],bytes_esperados=len(want.encode()),bytes_recibidos=len(got.encode()),sha256_esperado=sha(want.encode()),sha256_recibido=sha(got.encode()),espacios_iniciales_suprimidos=len(want)-len(got),tokens_y_cadenas_identicos=True,solo_sangria_inicial_distinta=True)
  diffs.append(item);bad.append(f['id']);unique[f['id']]={k:v for k,v in item.items() if k!='caso'}
 for k in ['decision','causa','contenido','llamadas_politica','caso_texto','reglas','fundamento','consecuencia','limites']:assert c.equal(case[k],expected[k]),(case['id'],k)
 cases.append(dict(id=case['id'],decision=case['decision'],causa=case['causa'],fuentes_no_exactas=bad,demas_obligaciones='conformes'))
result=json.loads((D/'COTEJO_ORIGINAL.json').read_text());assert p.returncode==2
assert len(result['errores'])==len(diffs)==26
assert all(e.endswith(':CITA_NO_EXACTA') for e in result['errores'])
save('DIAGNOSTICO_FIDELIDAD.json',dict(criterio='Diagnóstico de diferencias, no criterio sustituto de aceptación. No se genera respuesta corregida.',citas_totales=total,citas_no_exactas=len(diffs),fuentes_distintas_afectadas=unique,ocurrencias=diffs,casos=cases,conclusion='Las diferencias detectadas son exclusivamente supresiones de espacios iniciales en las fuentes decodificadas. Valores, cadenas, claves y orden conservados. No se conserva identidad byte a byte de las citas requerida por el contrato.'))
save('RECEPCION.json',dict(suceso='S7',registro='RETP-2026-172',responsable='Watson / W-S0',fecha_registro_utc=datetime.now(timezone.utc).isoformat(),archivo_recibido='Pegado text(9).txt',bytes_original=len(b),sha256_original=sha(b),participante_declarado=a['participante'],intento=2,dictamen=result['dictamen'],actividad_participante='No aporta registro de ejecución ni métricas; no se infieren operaciones usadas o ausentes.',tiempo_participante=None,alcance='Respuestas de los doce casos correctas; fidelidad literal de 26 citas incumplida. No se infiere opacidad interna ni intención.',exposicion='La declaración se limita al material de su conversación. Existe un primer intento atribuido a DeepSeek en S5; no se acredita continuidad de sesión o instancia ni ausencia de exposición previa.'))
print(json.dumps(dict(bytes=len(b),sha256=sha(b),citas_totales=total,citas_distintas=len(diffs),fuentes=list(unique),decision='NO_CONFORME'),ensure_ascii=False))
