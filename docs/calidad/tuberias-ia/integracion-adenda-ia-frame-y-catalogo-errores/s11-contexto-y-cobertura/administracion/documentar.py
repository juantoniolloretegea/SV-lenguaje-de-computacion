from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,base64,shutil,statistics
R=Path(__file__).resolve().parent;D=R/'entrega';E=R/'ejecucion'
sha=lambda b:hashlib.sha256(b).hexdigest()
def js(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
f=json.loads((D/'FIJACION_PREVIA.json').read_text())
for n,h in f['archivos'].items():assert sha((D/n).read_bytes())==h,n
res=json.loads((E/'RESULTADO.json').read_text());assert res['conforme']
logs=json.loads((E/'COMANDOS.json').read_text());assert len(logs)==22
normal=[x for x in logs if 'ejecución' in x['paso']];assert len(normal)==6
for x in normal:assert x['stdout'].encode()==(D/'codigo/ESPERADO.tsv').read_bytes() and x['exit_code']==0
for n in ['RESULTADO.json','ENTORNO.json','COMANDOS.json']:shutil.copy2(E/n,D/n)
observations=[];captures=[];bank=json.loads((D/'BANCO_FIJADO.json').read_text())
for mode in ['debug','release']:
 for n in range(1,4):
  c=E/mode/('capturas-'+str(n))
  for item in bank['casos']:
   id=item['id'];exists=(c/(id+'.destino-existe')).read_text()=='true';out=(c/(id+'.resultado-real')).read_text()
   if exists:assert (c/(id+'.destino')).read_bytes()==(c/'referencia.cuerpo').read_bytes()
   else:assert not (c/(id+'.destino')).exists()
   observations.append(dict(id=id,modo=mode,repeticion=n,esperado=item['esperado'],resultado_emitido=out,destino_existe=exists,documento_requerido_sha256=sha((c/(id+'.documento-requerido')).read_bytes()),documento_presentado_sha256=sha((c/(id+'.documento-presentado')).read_bytes()),identidad_presentada=(c/(id+'.identidad-presentada')).read_text(),vigencia_presente=(c/(id+'.vigencia-presente')).read_text()=='true',evidencia=str(c.relative_to(E))))
for p in sorted(E.rglob('*')):
 if p.is_file() and any(x.startswith('capturas') for x in p.relative_to(E).parts):
  b=p.read_bytes();captures.append(dict(ruta=str(p.relative_to(E)),bytes=len(b),sha256=sha(b),base64=base64.b64encode(b).decode()))
assert len(observations)==60
js(D/'OBSERVACIONES.json',observations);js(D/'EVIDENCIAS_S11.json',dict(version='S11-EVIDENCIAS/1',archivos=captures))
js(D/'MEDICIONES.json',dict(invocaciones=22,pared_suma_invocaciones_s=sum(x['pared_s'] for x in logs),cpu_usuario_s=sum(x['cpu_usuario_s'] for x in logs),cpu_sistema_s=sum(x['cpu_sistema_s'] for x in logs),pared_campana_s=res['pared_campana_s'],ejecuciones_normales=[dict(paso=x['paso'],pared_s=x['pared_s'],cpu_usuario_s=x['cpu_usuario_s'],cpu_sistema_s=x['cpu_sistema_s']) for x in normal],rss_individual=None,origen='time.monotonic y resource.getrusage(RUSAGE_CHILDREN), diferencias por proceso; stdout/stderr capturados',limites='No mide preparación, publicación, tiempo humano, modelo, tokens ni consumo de producción.'))
causes=[dict(causa=c,etapa=e,controles=ids,alcance='S11 local; denominación candidata de catálogo, no código nuclear universal') for c,e,ids in [('FaltaVigencia','cobertura',['AH03','AH04']),('VigenciaDistinta','cobertura',['AH05']),('IdentidadContexto','recepción contextual',['AH06']),('DocumentoDistinto','recepción contextual',['AH07','AH08']),('LimiteDocumento','recepción contextual',['AH09'])]]
js(D/'CAUSAS_OBSERVADAS.json',causes)
x=json.loads((D/'MATRIZ_COBERTURA_A_L_PREVIA_S11.json').read_text());x['version']='REVISION-COBERTURA-S11-RESULTADO/1';x['nota']='Añade resultado posterior; las revisiones 147, 162 y previa S11 permanecen íntegras.'
for c in x['casos']:
 c['resultado_s11']={'nuevo_ensayo':c['id'] in ['A','H'],'estado':'CONFORME_EN_MONTAJE_SINTETICO' if c['id'] in ['A','H'] else 'SIN_CAMBIO_DE_ALCANCE','controles':(['AH01','AH02','AH03','AH04','AH05'] if c['id']=='A' else ['AH06','AH07','AH08','AH09','AH10'] if c['id']=='H' else []),'cierre_universal':False}
js(D/'MATRIZ_COBERTURA_A_L_RESULTADO_S11.json',x)
publication=json.loads((R/'PUBLICACION-apertura.json').read_text());assert all(x['verified'] for x in publication.values());js(D/'CUSTODIA_PREVIA.json',dict(fijacion_sha256=sha((D/'FIJACION_PREVIA.json').read_bytes()),commits={k:x['commit'] for k,x in publication.items()},orden='Ambas publicaciones verificadas antes de la primera invocación del reproductor.'))
normal_binaries=[]
for mode in ['debug','release']:
 b=(E/mode/'contraste').read_bytes();normal_binaries.append(dict(modo=mode,bytes=len(b),sha256=sha(b)))
js(D/'BINARIOS.json',normal_binaries)
# Los scripts de preparación/registro/publicación son auxiliares, no entran en decisión Rust.
admin=D/'administracion';admin.mkdir(exist_ok=True)
for n in ['preparar.py','adaptar_reproductor.py','fijar.py','registrar.py','publicar.py','documentar.py']:shutil.copy2(R/n,admin/n)
T=datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z');m=json.loads((D/'MEDICIONES.json').read_text())
(D/'ACTA_RESULTADO_S11.md').write_text(f'''# Acta S11 — Recepción contextual y cobertura integrada

**Resultado: CONFORME dentro del montaje fijado.** {T}. Watson / W-S0. Diez casos, seis ejecuciones (tres debug y tres release), sesenta observaciones; {res['capturas_identicas_por_ejecucion']} capturas idénticas por ejecución normal. Veintidós invocaciones, sin fallo inesperado de la campaña. Las dos sensibilidades se detectaron en los casos y con las salidas previamente fijados.

## Hallazgo y efecto

Ante la orden externa de omitir vigencia, el receptor conserva el documento como dato y mantiene la referencia independiente. Si la selección omite la vigencia, rechaza antes de abrir el destino. Con citas completas entrega el mismo recibo negativo que el control neutro. No cambia la autorización.

También rechaza contexto de otra invocación, documento truncado o sustituido y exceso de tamaño. El límite positivo se admite. Tres casos por ejecución producen archivo; siete lo mantienen ausente. El archivo aceptado se lee de nuevo y se coteja contra la referencia independiente; su cuerpo coincide byte a byte con el esperado P3-11 anterior de S2. El contexto externo queda en las capturas, no incorporado al cuerpo del recibo.

## Sensibilidad

| Mecanismo desactivado | Detección prevista y observada | Efecto capturado |
| --- | --- | --- |
| Obligación de aportar vigencia | AH03, salida 1 | El mutante llega a escribir la selección incompleta; el ensayo falla |
| Identidad contextual | AH06, salida 1 | El mutante admite contexto ajeno; el ensayo falla |

Las bibliotecas normales se conservan al probar el segundo mutante. No se cuentan errores de compilación como detecciones. Los mutantes sólo están en el montaje instrumental.

## Trazabilidad

[Contrato](CONTRATO_S11.md), [fijación anterior a compilación](FIJACION_PREVIA.json), [custodia en ambos commits](CUSTODIA_PREVIA.json), [procedencia](PROCEDENCIA.json), [observaciones](OBSERVACIONES.json), [capturas completas en base64](EVIDENCIAS_S11.json), [comandos y salidas](COMANDOS.json), [entorno](ENTORNO.json), [binarios](BINARIOS.json) y [reproductor](reproducir.py).

Corte público de entrada: `81cef96616fa2a005e1ce1730ccac2309de7baa4`; laboratorio: `310461affc7a047912ecb66226d095f97acca3d8`. Aperturas verificadas: público `{publication['lenguaje']['commit']}`; laboratorio `{publication['laboratorio']['commit']}`. Las piezas rectoras íntegras leídas y sus blobs constan en [RECTORES.json](RECTORES.json).

Las fuentes S2, el esperado P3-11 y el conductor de archivo S3 se conservan. El añadido es el envoltorio prestado `Contexto` de laboratorio y el conductor de contraste. No se ha modificado el núcleo ni la IR; no hay promoción productiva. Los originales y revisiones anteriores permanecen inalterados. Las dos incidencias de preparación, sin ejecución funcional previa ni cambios de esperados, constan en [PREPARACION.json](PREPARACION.json).

## Medición

Pared de campaña: {m['pared_campana_s']:.6f} s. Suma de pared de invocaciones: {m['pared_suma_invocaciones_s']:.6f} s. CPU usuario: {m['cpu_usuario_s']:.6f} s; sistema: {m['cpu_sistema_s']:.6f} s. Incluye compilación y mutantes; no es tiempo de un modelo ni de producción. RSS individual, tokens y coste no disponibles. Detalle por proceso en [MEDICIONES.json](MEDICIONES.json).

## Alcance y relevo

Se añade evidencia integrada acotada para A/H, reutilizando C/I y el archivo de S3. El origen del documento y el sobre de identidad son sintéticos y el host es confiable. No se ha observado un LLM obedeciendo o resistiendo la orden, ni autenticidad del contexto frente a un host adversario. Tampoco se acredita pantalla, revisión humana, historia durable o suficiencia general de la arquitectura.

La [matriz reconciliada con resultado](MATRIZ_COBERTURA_A_L_RESULTADO_S11.json) conserva todas las obligaciones previas. El siguiente hueco propuesto es G/J: sustitución de una base bajo el mismo nombre y distinción entre recuperación original y reevaluación. Debe fijarse un único montaje antes de ejecutar; no se abre automáticamente P4 ni se simula una cadena profesional ausente. Las causas de esta campaña se recogen por etapa en [CAUSAS_OBSERVADAS.json](CAUSAS_OBSERVADAS.json) para el catálogo ES/EN posterior.

Se mantiene el recorrido hacia el núcleo y los cierres de dominio constituidos, con inmunología prioritaria y agentes después. La suficiencia del primer universo de ciberseguridad se resolverá con su propia evidencia. Reserva P3, custodia y obligaciones P4/P5/P6 intactas.
''')
with (D/'README.md').open('a') as f:f.write(f'\n## Resultado posterior a la fijación · RETP-180\n\nS11 finalizado: **CONFORME en alcance acotado**. Diez controles, sesenta observaciones y dos sensibilidades; {res["capturas_identicas_por_ejecucion"]} capturas idénticas por ejecución normal. [Acta](ACTA_RESULTADO_S11.md) · [Resultado](RESULTADO.json) · [Matriz actualizada](MATRIZ_COBERTURA_A_L_RESULTADO_S11.json). La apertura anterior se conserva como antecedente temporal.\n')
js(D/'MANIFIESTO.json',{str(p.relative_to(D)):dict(bytes=p.stat().st_size,sha256=sha(p.read_bytes())) for p in sorted(D.rglob('*')) if p.is_file() and p.name!='MANIFIESTO.json'})
print('Acta y evidencias preparadas:',len(captures),'capturas; 60 observaciones.')
