"""Sonda de caracterización: ejecuta el receptor Rust conservado, no compila SV en Python."""
from pathlib import Path
import sys,json,hashlib,subprocess,time
ROOT=Path(__file__).resolve().parent
if len(sys.argv)!=4:raise SystemExit('Uso: ejecutar.py RECEPTOR_RUST DIRECTORIO_FIXTURES SALIDA_NUEVA')
exe=Path(sys.argv[1]).resolve();fixtures=Path(sys.argv[2]).resolve();dest=Path(sys.argv[3]).resolve();dest.mkdir(parents=True,exist_ok=False)
def sha(b):return hashlib.sha256(b).hexdigest()
def wire(b):return b'SVAC0001'+len(b).to_bytes(8,'little')+b+hashlib.sha256(b).digest()
original=(fixtures/'cuerpo-a01.bin').read_bytes();frame=(fixtures/'marco-a01.bin').read_bytes();assert frame[16:-32]==original;assert frame==wire(original)
req=(fixtures/'solicitud-a01.bin').read_bytes();prop=(fixtures/'propuesta-v02.bin').read_bytes();annex=json.loads((fixtures/'anexo-v02.bin').read_bytes());lote=annex['solicitudes_sha256']
assert b'"contenido":"8.40"' in original
changed=original.replace(b'"contenido":"8.40"',b'"contenido":"9.40"');assert changed!=original
assert original.count(b'no acredita')==1
neg=original.replace(b'no acredita',b'acredita')
p=json.loads(prop);p['autoridad_sobre_cuerpo_a']=True;injected=json.dumps(p,separators=(',',':'),ensure_ascii=False).encode()
vargs=['v',req.decode(),lote,sha(original),'1000000']
cases=[
 ('C1',['validar'],frame,0,'CUERPO_ORIGINAL','Control intacto'),
 ('C2',['validar'],frame[:16]+changed+frame[-32:],2,'CORRELACION_INVALIDA','Cambio de dato sin actualizar huella'),
 ('C3',['validar'],wire(changed),0,'CUERPO_CAMBIADO','Cambio de dato con huella recalculada; se caracteriza integridad, no fidelidad'),
 ('C4',['validar'],wire(neg),0,'NEGACION_SUPRIMIDA','Supresión de negación con longitud y huella recalculadas'),
 ('C5',vargs,prop,0,'DERIVACION_SINTACTICA_COMPROBADA','Propuesta V original'),
 ('C6',vargs,injected,0,'V_RECHAZADA_ESQUEMA_INVALIDO','Propuesta V con autoridad añadida por el emisor')]
plan={'registro':'RETP-2026-148','tipo':'CARACTERIZACION_DE_FRONTERA_NO_CUALIFICACION_INTEGRAL','ejecutable_sha256':sha(exe.read_bytes()),'script_sha256':sha(Path(__file__).read_bytes()),'presupuesto':{'procesos':6,'tiempo_maximo_por_proceso_s':60,'salida_total_limite_bytes':1048576,'modelos':0,'reserva':False,'presupuestos_A_V':'sin modificación; V máximo 1000000 unidades'},'advertencias':['C3 y C4 aceptados caracterizan una limitación; no acreditan fidelidad','No se inyectan estas salidas en G1 ni se prueba una vía externa hasta el efecto','El ejecutable procede de un corte anterior; no hay compilación nueva ni prueba de identidad del código cargado independiente del entorno'],'casos':[{'id':i,'args':a,'entrada_sha256':sha(b),'rc_esperado':rc,'observacion_esperada':e,'motivo':m} for i,a,b,rc,e,m in cases]}
for i,a,b,rc,e,m in cases:(dest/(i+'.stdin')).write_bytes(b)
(dest/'PLAN_PREVIO.json').write_text(json.dumps(plan,ensure_ascii=False,indent=2)+'\n')
print('PLAN_FIJADO',sha((dest/'PLAN_PREVIO.json').read_bytes()),flush=True)
results=[];total=0
for i,args,data,rc,expected,m in cases:
 start=time.monotonic();r=subprocess.run([str(exe),*args],input=data,capture_output=True,timeout=60);elapsed=time.monotonic()-start
 (dest/(i+'.stdout')).write_bytes(r.stdout);(dest/(i+'.stderr')).write_bytes(r.stderr);total+=len(r.stdout)+len(r.stderr)
 good=r.returncode==rc and total<=1048576;detail=''
 if i=='C1':good &= r.stdout==original and r.stderr==b''
 elif i=='C2':good &= r.stdout==b'' and r.stderr==b'SV_AV_ERROR:CORRELACION_INVALIDA\n'
 elif i=='C3':good &= r.stdout==changed and json.loads(r.stdout)['resolucion']['contenido']=='9.40';detail='INTEGRIDAD_ACEPTADA_FIDELIDAD_NO_COMPROBADA'
 elif i=='C4':good &= r.stdout==neg and b'no acredita' not in r.stdout;detail='INTEGRIDAD_ACEPTADA_NEGACION_PERDIDA'
 elif i in ('C5','C6'):
  a=json.loads(r.stdout);good &= a['autoridad_sobre_cuerpo_a'] is False and a['cuerpo_a_sha256']==sha(original)
  good &= a['estado']==('DERIVACION_SINTACTICA_COMPROBADA' if i=='C5' else 'V_RECHAZADA')
  good &= a['causa']==(None if i=='C5' else 'ESQUEMA_INVALIDO')
 results.append({'id':i,'rc':r.returncode,'coincide_caracterizacion_previa':bool(good),'detalle':detail,'stdout_sha256':sha(r.stdout),'stderr_sha256':sha(r.stderr),'segundos_observados':elapsed})
 (dest/'RESULTADO.json').write_text(json.dumps({'casos':results,'salida_bytes':total,'estado':'CARACTERIZACION_EN_CURSO'},ensure_ascii=False,indent=2)+'\n')
 print(i,r.returncode,good,detail,flush=True)
 if not good:raise SystemExit('DIVERGENCIA: conservar salidas y diagnosticar sin cambiar esperado')
assert sha(exe.read_bytes())==plan['ejecutable_sha256']
assert (fixtures/'cuerpo-a01.bin').read_bytes()==original
(dest/'RESULTADO.json').write_text(json.dumps({'casos':results,'salida_bytes':total,'estado':'CARACTERIZACION_OBSERVADA_NO_CIERRE_DE_FIDELIDAD','binario_intacto':True,'original_intacto':True},ensure_ascii=False,indent=2)+'\n')
