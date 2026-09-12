from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,base64,shutil
R=Path(__file__).resolve().parent;D=R/'entrega';W=R/'ejecucion';sha=lambda b:hashlib.sha256(b).hexdigest()
def j(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
f=json.loads((D/'FIJACION_PREVIA.json').read_text());unchanged={n:sha((D/n).read_bytes())==h for n,h in f['archivos'].items()};assert all(unchanged.values())
for n in ['COMANDOS.json','ENTORNO.json','BINARIOS.json','RESULTADO.json','INTERRUPCION.json']:
 if (W/n).exists():shutil.copyfile(W/n,D/n)
logs=json.loads((D/'COMANDOS.json').read_text());res=json.loads((D/'RESULTADO.json').read_text()) if (D/'RESULTADO.json').exists() else json.loads((D/'INTERRUPCION.json').read_text());ok=res.get('conforme',False)
capturas=[]
for p in sorted(W.rglob('*')):
 if p.is_file() and any(a.startswith('capturas') for a in p.relative_to(W).parts):
  b=p.read_bytes();capturas.append(dict(ruta=str(p.relative_to(W)),bytes=len(b),sha256=sha(b),base64=base64.b64encode(b).decode()))
j(D/'EVIDENCIAS_S18.json',dict(version='S18-EVIDENCIAS/1',archivos=capturas));j(D/'CUSTODIA_POSTERIOR.json',dict(version='S18-CUSTODIA/1',fijacion_sha256=sha((D/'FIJACION_PREVIA.json').read_bytes()),archivos_previos_intactos=unchanged,total_capturas=len(capturas)))
metrics=dict(version='S18-MEDICIONES/1',invocaciones=len(logs),pared_s=sum(x.get('pared_s',0) for x in logs),cpu_usuario_s=sum(x.get('cpu_usuario_s',0) for x in logs),cpu_sistema_s=sum(x.get('cpu_sistema_s',0) for x in logs),rss_max_hijos_kib=max([x.get('rss_max_hijos_hasta_paso_kib',0) for x in logs] or [0]),rss_individual=None,interpretacion='CPU y pared de invocaciones de esta campaña; RSS es máximo acumulado de hijos, no medición individual; no equivalen a coste de producción o de proveedor.')
j(D/'MEDICIONES.json',metrics)
t=datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z')
if ok:
 body=f'''**Resultado:** conforme dentro del banco S18. Los 24 casos completaron seis ejecuciones debug/release: 144 observaciones normales. Las {res['capturas_identicas_por_ejecucion']} capturas de cada ejecución coinciden byte a byte. Cuatro mutantes detectados en I04/I14/I09/I12. Un cliente externo válido compiló y ejecutó; dos clientes que fabrican las piezas comprobadas fueron rechazados por E0451.

Se ejecutaron exactamente 29 invocaciones, sin reintentos ni modificaciones de los archivos fijados. En esta máquina sumaron {metrics['pared_s']:.6f} s de pared, {metrics['cpu_usuario_s']:.6f} s de CPU de usuario y {metrics['cpu_sistema_s']:.6f} s de CPU de sistema. El máximo acumulado de RSS de hijos fue {metrics['rss_max_hijos_kib']} KiB; no es un RSS individual ni un presupuesto de producción.

## Lo que acredita el recorrido

- La base instalada se consume en producción G1; la reevaluación negativa genera el cuerpo negativo anterior y conserva identidad/parentesco y original positivo.
- La referencia procede de EntregaLiteral G1. La cobertura liga identidad, contexto exacto, presentación del cuerpo, solicitud y base del episodio.
- La recepción retiene los bytes y distingue negativa, truncamiento, exceso y error de comunicación tardío. Ninguno de esos rechazos crea una entrega admitida.
- La escritura requiere EntregaComprobada de constructor privado. El destino existente conserva sus bytes; la sustitución posterior se detecta al recuperar contra G1.
- La escritura parcial controlada conserva los siete bytes emitidos y WriteZero. No se afirma ausencia de efecto después de iniciar la escritura.

## Recepción y siguiente objeto

Se recibe evidencia del enlace documental conjunto que antes sólo existía por piezas. Permite pasar a la recepción en la matriz 1+3 y ordenar sus causas para el catálogo. No equivale al cierre profesional íntegro de los puntos 1 y 3. La recepción deberá mantener las fronteras B/E/K/L, la deuda viva y las puertas P3/P4/P5/P6; no se dará por cumplida una obligación no ejercitada.

Siguiente objeto único: consolidar la matriz de integración con este resultado y el inventario de causas por etapa, para decidir el relevo documental al catálogo sin promover autoridad nuclear ni adelantar agentes.
'''
else:
 body=f'''**Resultado:** campaña interrumpida; no se acredita conformidad conjunta. Motivo registrado: `{res['error']}`. Se conservaron {len(logs)} invocaciones y sus salidas originales. No se corrigieron fuentes ni esperados después de la fijación.

Las operaciones completadas se identifican en COMANDOS.json; un fallo de compilación no es un fallo semántico de los 24 casos ni permite declararlos ejecutados. Los ensayos restantes no se ejecutaron. Siguiente objeto: analizar la interrupción y justificar una nueva fijación antes de modificar o ejecutar de nuevo.
'''
(D/'ACTA_RESULTADO_S18.md').write_text(f'''# Acta de resultado S18 — Recorrido documental conjunto

Fecha: {t}. Responsable: Watson / W-S0. Autorización: continuación expresa tras S17 / RETP-188. La [apertura RETP-189](README.md), el [contrato S17](contrato-s17/CONTRATO_RECORRIDO_CONJUNTO.md), la campaña y la fijación anterior a compilación conservan sus bytes.

{body}
## Custodia y límites

[Procedencia y cortes](PROCEDENCIA.json), [rectores](RECTORES.json), [comandos originales](COMANDOS.json), [capturas recuperables](EVIDENCIAS_S18.json), [custodia posterior](CUSTODIA_POSTERIOR.json) y [mediciones](MEDICIONES.json). El compilador y los binarios quedan identificados; el reproductor y las fuentes permiten repetir en una salida nueva. El cotejo posterior verifica todos los bytes de la fijación previa.

El ensayo es local, con instalador y host confiables y datos artificiales. El documento con instrucciones hostiles fue tratado como dato por este receptor; eso no prueba el comportamiento ni la trazabilidad interna de una IA comercial. No hubo proveedor real, red real, efecto R1, consentimiento profesional, canal secreto declarado, autoridad de destino ni prueba de persistencia tras caída. Tampoco se cualificaron todas las rutas internas de error G1 mediante esta envoltura.

No se modifica semántica 0.2, IR 0.3, gramática, serializador ni núcleo de producción. El resultado se recibe como evidencia de laboratorio. El retorno principal a inmunología y la decisión posterior sobre agentes mantienen su secuencia y sus puertas.
''')
# Código administrativo posterior se incorpora sin modificar el conjunto fijado.
A=D/'administracion-posterior';A.mkdir(exist_ok=True)
for n in ['documentar.py','preparar_cierre.py']:shutil.copyfile(R/n,A/n)
j(D/'MANIFIESTO_POSTERIOR.json',dict(version='S18-MANIFIESTO/1',archivos={str(p.relative_to(D)):dict(bytes=p.stat().st_size,sha256=sha(p.read_bytes())) for p in sorted(D.rglob('*')) if p.is_file() and p.name!='MANIFIESTO_POSTERIOR.json'}))
print('Acta de resultado preparada:',ok,'capturas',len(capturas))
