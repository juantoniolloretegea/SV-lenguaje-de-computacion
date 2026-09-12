from pathlib import Path
from datetime import datetime,timezone
import sys,json,hashlib,subprocess,time
R=Path(__file__).resolve().parent;S=R.parent/'s6-trazabilidad-total';D=R/'expediente'
def save(n,o):(D/n).write_text(json.dumps(o,ensure_ascii=False,indent=2)+'\n')
b=(R.parent/'upload/34f008f4-f093-43be-a563-221e6af86b5e.json').read_bytes();(D/'RESPUESTA_ORIGINAL.txt').write_bytes(b)
cmd=[sys.executable,str(S/'publico/cotejar_entrega.py'),'--respuesta',str(D/'RESPUESTA_ORIGINAL.txt'),'--banco',str(S/'publico/BANCO.json'),'--referencia',str(S/'privado/REFERENCIA_PREVIA.json'),'--compromiso',str(S/'publico/COMPROMISO_PREVIO.json'),'--salida',str(D/'COTEJO_ORIGINAL.json')]
t=datetime.now(timezone.utc).isoformat();start=time.perf_counter_ns();p=subprocess.run(cmd,capture_output=True);dur=time.perf_counter_ns()-start
save('EJECUCION_OBSERVADOR.json',dict(inicio_utc=t,comando=cmd,codigo_salida=p.returncode,stdout=p.stdout.decode(),stderr=p.stderr.decode(),duracion_ns_observador=dur,alcance='Cotejo documental por Watson / W-S0; no mide el tiempo del participante.'))
sys.path.insert(0,str(S/'publico'));import cotejar_entrega as c
com=c.parse((S/'publico/COMPROMISO_PREVIO.json').read_bytes())
for n,k in [('cotejar_entrega.py','verificador_sha256'),('BANCO.json','banco_sha256'),('CONTRATO.md','contrato_sha256'),('PRUEBA_COMUN.md','documento_sha256')]:assert hashlib.sha256((S/'publico'/n).read_bytes()).hexdigest()==com[k]
a=c.parse(b);result=json.loads((D/'COTEJO_ORIGINAL.json').read_text())
save('RECEPCION.json',dict(suceso='S7',registro='RETP-2026-175',responsable='Watson / W-S0',fecha_registro_utc=t,archivo_recibido='34f008f4-f093-43be-a563-221e6af86b5e.json',bytes_original=len(b),sha256_original=hashlib.sha256(b).hexdigest(),participante_declarado=a['participante'],intento=2,dictamen=result['dictamen'],citas_fuentes=sum(len(x['fuentes']) for x in a['resultados']),casos=len(a['resultados']),tiempo_participante=None,actividad_participante='No se adjunta registro de ejecución. No se infieren operaciones usadas o ausentes.',alcance='Conformidad documental de esta entrega; identidad, versión y exposición son declaraciones del participante.'))
print(json.dumps(result,ensure_ascii=False));print((D/'RECEPCION.json').read_text())
