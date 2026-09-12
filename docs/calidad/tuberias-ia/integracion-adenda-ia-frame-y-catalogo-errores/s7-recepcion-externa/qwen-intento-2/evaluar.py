"""Conserva la recepción y ejecuta el cotejador fijado sin reparar la entrega."""
from pathlib import Path
import hashlib,json,re,subprocess,sys,time
from datetime import datetime,timezone
R=Path(__file__).resolve().parent
S=R.parent/'s6-trazabilidad-total'
D=R/'expediente'
def h(b):return hashlib.sha256(b).hexdigest()
def save(name,obj):
 (D/name).write_text(json.dumps(obj,ensure_ascii=False,indent=2)+'\n')
raw=(R.parent/'upload/Pegado text(8).txt').read_bytes()
assert raw==(D/'RESPUESTA_ORIGINAL.txt').read_bytes()
m=re.search(rb'```json\r?\n([\s\S]*?)\r?\n```',raw)
assert m and len(re.findall(rb'```json',raw))==1
block=m[1]
(D/'BLOQUE_JSON_DIAGNOSTICO.json').write_bytes(block)
save('EXTRACCION_DIAGNOSTICA.json',dict(original_sha256=h(raw),inicio_byte=m.start(1),fin_byte_exclusivo=m.end(1),bloque_sha256=h(block),bytes_bloque=len(block),proposito='Diagnóstico acotado del contenido. No sustituye el original ni su dictamen. Se conservan bytes del bloque sin normalización.'))
logs=[]
for src,dst in [('RESPUESTA_ORIGINAL.txt','COTEJO_ORIGINAL.json'),('BLOQUE_JSON_DIAGNOSTICO.json','COTEJO_BLOQUE_DIAGNOSTICO.json')]:
 args=[sys.executable,str(S/'publico/cotejar_entrega.py'),'--respuesta',str(D/src),'--banco',str(S/'publico/BANCO.json'),'--referencia',str(S/'privado/REFERENCIA_PREVIA.json'),'--compromiso',str(S/'publico/COMPROMISO_PREVIO.json'),'--salida',str(D/dst)]
 t=datetime.now(timezone.utc).isoformat();start=time.perf_counter_ns();p=subprocess.run(args,capture_output=True);elapsed=time.perf_counter_ns()-start
 logs.append(dict(inicio_utc=t,comando=args,codigo_salida=p.returncode,stdout=p.stdout.decode(),stderr=p.stderr.decode(),duracion_ns_observador=elapsed,salida=dst,salida_sha256=h((D/dst).read_bytes())))
assert json.loads((D/'COTEJO_ORIGINAL.json').read_text())['errores']==['/:BLOQUE_JSON_INVALIDO']
assert json.loads((D/'COTEJO_BLOQUE_DIAGNOSTICO.json').read_text())['errores']==[]
save('EJECUCIONES_OBSERVADOR.json',dict(responsable='Watson / W-S0',alcance='Cotejo de archivos; no ejecución semántica SV ni medida del participante.',ejecuciones=logs))
ans=json.loads(block);ref=json.loads((S/'privado/REFERENCIA_PREVIA.json').read_bytes())
fields=['decision','causa','contenido','llamadas_politica','caso_texto','fuentes','reglas','fundamento','consecuencia','limites']
save('RESUMEN_CASOS.json',[dict(id=x['id'],decision=x['decision'],causa=x['causa'],obligaciones_cotejadas=fields,resultado='CONFORME_EN_BLOQUE_DIAGNOSTICO') for x in ans['resultados']])
save('RECEPCION.json',dict(suceso='S7',registro='RETP-2026-171',responsable='Watson / W-S0',fecha_registro_utc=datetime.now(timezone.utc).isoformat(),nombre_recibido='Pegado text(8).txt',bytes_original=len(raw),sha256_original=h(raw),participante_declarado=ans['participante'],intento=2,contrato='SV-TRAZABILIDAD-2/1',dictamen_entrega='NO_CONFORME',motivo='Texto añadido tras el bloque JSON, prohibido por el formato fijado.',dictamen_bloque_diagnostico='CONFORME_DOCUMENTAL',casos_conformes_en_bloque=12,casos_exigidos=12,observacion='El texto adicional afirma que no hay texto fuera del bloque, contradicho por el propio archivo. No se atribuye intención.',actividad_participante='Declara lectura documental y ausencia de ejecución propia de Rust, herramientas y medidas; no se recibió registro de ejecución.',tiempo_participante=None,motivo_tiempo_ausente='No se ha aportado medición verificable.',atribucion='Archivo aportado por el usuario como respuesta de Qwen; identidad y versión declaradas, sin certificación del proveedor.',alcance='Incumplimiento de formato y declaración contradictoria; no se observa pérdida de la cadena documental requerida en los doce casos. No acredita auditoría de procesos internos.'))
com=json.loads((S/'publico/COMPROMISO_PREVIO.json').read_bytes())
for name,key in [('cotejar_entrega.py','verificador_sha256'),('BANCO.json','banco_sha256'),('CONTRATO.md','contrato_sha256'),('PRUEBA_COMUN.md','documento_sha256')]:assert h((S/'publico'/name).read_bytes())==com[key]
assert h((S/'privado/REFERENCIA_PREVIA.json').read_bytes())==com['referencia_sha256']
print('Original NO_CONFORME; bloque diagnóstico 12/12 CONFORME_DOCUMENTAL; instrumento íntegro.')
