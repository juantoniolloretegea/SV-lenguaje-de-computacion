from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,base64,shutil
R=Path(__file__).resolve().parent;D=R/'entrega';E=R/'ejecucion';sha=lambda b:hashlib.sha256(b).hexdigest();J=lambda p,x:p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
f=json.loads((D/'FIJACION_PREVIA.json').read_text())
for n,h in f['archivos'].items():assert sha((D/n).read_bytes())==h,n
res=json.loads((E/'RESULTADO.json').read_text());assert res['conforme'];logs=json.loads((E/'COMANDOS.json').read_text());assert len(logs)==17
normal=[x for x in logs if 'ejecución' in x['paso']];assert len(normal)==6
for x in normal:assert x['stdout'].encode()==(D/'codigo/ESPERADO.tsv').read_bytes() and x['exit_code']==0
for n in ['RESULTADO.json','COMANDOS.json','ENTORNO.json','BINARIOS.json']:shutil.copyfile(E/n,D/n)
expected=dict(line.split('\t') for line in (D/'codigo/ESPERADO.tsv').read_text().splitlines());obs=[];captures=[];baseline=None
for mode in ['debug','release']:
 for n in range(1,4):
  c=E/mode/('capturas-'+str(n));curr={p.name:sha(p.read_bytes()) for p in c.iterdir()};assert baseline is None or curr==baseline;baseline=curr
  for id,tag in expected.items():obs.append(dict(caso=id,modo=mode,repeticion=n,etiqueta_esperada=tag,resultado_real=(c/(id+'.resultado-real')).read_text(),evidencia=str(c.relative_to(E))))
  # Cotejo independiente posterior con esperados anteriores: no reconstruye el resultado por reglas nuevas.
  for id,name in [('GJ01','positivo.cuerpo'),('GJ04','negativo.cuerpo'),('GJ05','positivo.cuerpo'),('GJ09','negativo.cuerpo'),('original-final','positivo.cuerpo')]:assert (c/(id+'.cuerpo')).read_bytes()==(D/'especimenes'/name).read_bytes()
  assert (c/'GJ01.base-consumida').read_bytes()==(c/'GJ05.base-consumida').read_bytes()==(c/'original-final.base-consumida').read_bytes()==(D/'especimenes/base-original.bin').read_bytes()
  assert (c/'GJ04.base-consumida').read_bytes()==(D/'especimenes/base-nueva.bin').read_bytes()
  assert (c/'GJ01.solicitud').read_bytes()==(c/'GJ04.solicitud').read_bytes()==(D/'especimenes/solicitud.bin').read_bytes()
for p in sorted(E.rglob('*')):
 if p.is_file() and any(x.startswith('capturas') for x in p.relative_to(E).parts):
  b=p.read_bytes();captures.append(dict(ruta=str(p.relative_to(E)),bytes=len(b),sha256=sha(b),base64=base64.b64encode(b).decode()))
assert len(obs)==72;J(D/'OBSERVACIONES.json',obs);J(D/'EVIDENCIAS_S14.json',dict(version='S14-EVIDENCIAS/1',archivos=captures))
J(D/'MEDICIONES.json',dict(invocaciones=17,pared_campana_s=res['pared_campana_s'],pared_suma_invocaciones_s=sum(x['pared_s'] for x in logs),cpu_usuario_s=sum(x['cpu_usuario_s'] for x in logs),cpu_sistema_s=sum(x['cpu_sistema_s'] for x in logs),ejecuciones_normales=[{k:x[k] for k in ['paso','pared_s','cpu_usuario_s','cpu_sistema_s']} for x in normal],rss_individual=None,tokens=None,coste=None,origen='time.monotonic y diferencias resource.getrusage(RUSAGE_CHILDREN).',limite='Incluye compilación y mutantes; excluye preparación/publicación. No es latencia de modelo ni coste productivo.'))
J(D/'CAUSAS_OBSERVADAS.json',[dict(causa=c,etapa=e,controles=ids,alcance='Variante local del contrato SV-S14-BASES-GJ/1; no código canónico del núcleo ni traducción automática a U.') for c,e,ids in [('BaseDistinta','carga',['GJ02']),('BaseSinCambio','reevaluación',['GJ03']),('BaseDistinta','atribución al original',['GJ06']),('Identidad','atribución al original',['GJ07']),('ReciboDistinto','atribución al original',['GJ08']),('Capacidad','reevaluación',['GJ10']),('FormatoBase','carga',['GJ11']),('LimiteBase','carga',['GJ12'])]])
old=R.parent/'s11-integracion/entrega/MATRIZ_COBERTURA_A_L_RESULTADO_S11.json';x=json.loads(old.read_text());before=json.loads(old.read_text());x['version']='REVISION-COBERTURA-S14-RESULTADO/1';x['corte_s14']=json.loads((D/'PROCEDENCIA.json').read_text())['corte'];x['nota_s14']='Añade evidencia G/J acotada; ningún criterio previo ni resultado S11 se reescribe.'
for c in x['casos']:
 c['resultado_s14']=dict(nuevo_ensayo=c['id'] in ['G','J'],estado='CONFORME_EN_MONTAJE_SINTETICO' if c['id'] in ['G','J'] else 'SIN_CAMBIO_DE_ALCANCE',controles=['GJ01','GJ02','GJ04'] if c['id']=='G' else ['GJ03','GJ04','GJ05','GJ06','GJ07','GJ08','GJ09','GJ10'] if c['id']=='J' else [],cierre_universal=False,limite='Base exterior de vigencia y custodia intraproceso; no componente arbitrario, historia durable o QueryResult nativo.' if c['id'] in ['G','J'] else 'Se conserva el alcance anterior. Los controles de formato/límite S14 no cierran F ni L globales.')
assert [{k:v for k,v in c.items() if k!='resultado_s14'} for c in x['casos']]==before['casos'];J(D/'MATRIZ_COBERTURA_A_L_RESULTADO_S14.json',x)
pub=json.loads((R/'PUBLICACION-apertura.json').read_text());assert all(v['verified'] for v in pub.values())
T=datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z');m=json.loads((D/'MEDICIONES.json').read_text())
(D/'ACTA_RESULTADO_S14.md').write_text(f'''# Acta S14 — Base consumida y recuperación frente a reevaluación

**CONFORME dentro del contrato fijado.** {T}. Watson / W-S0. S14 / RETP-184. Doce casos en seis ejecuciones (tres debug y tres release): 72 observaciones normales; {res['capturas_identicas_por_ejecucion']} capturas idénticas por ejecución. Diecisiete invocaciones y tres sensibilidades detectadas en los casos previstos. Sin reintento funcional ni cambio de fuente o esperado posterior a la fijación.

## Resultado material

La misma solicitud A01 produce el cuerpo positivo anterior con la base de vigencia original y el cuerpo negativo anterior al consumir la base nueva. Las bases difieren en un byte y se cargan desde la misma ruta. Ambos cuerpos se cotejan íntegramente con esperados previos y conservan una llamada de política. Las doce observaciones por ejecución incluyen cargas, rechazos y recuperaciones: sólo dos actos nuevos producen recibo G1 en cada ejecución normal; no son doce consultas nuevas.

La sustitución de archivo bajo referencia antigua se rechaza como BaseDistinta. La reevaluación recibe identidad propia y padre original; recuperar después el original devuelve sus mismos cuerpo y base. Presentar la base nueva como original, cambiar identidad o sustituir el cuerpo genera el rechazo correspondiente. El límite de dos actos y la lectura acotada conservan sus causas.

G1 y sus dependencias permanecen intactos. La base exterior de vigencia del experimento no se confunde con K-IE004/1 del recibo. No se ha relajado la fijación de lote/montaje de S2. Esta campaña tampoco vuelve a acreditar la cobertura documental S2/S11 o el transporte S3: reutiliza el custodio previo y añade una frontera local de carga y atribución.

## Sensibilidades

| Mecanismo alterado | Evidencia prevista y observada |
| --- | --- |
| Omitir identidad de carga | GJ02 acepta indebidamente la base sustituida; exit 1 |
| Pasar siempre vigente=true conservando la base nueva | GJ04 produce cuerpo positivo en vez del negativo fijado; exit 1 |
| Omitir sólo la base al atribuir original | GJ06 acepta la atribución de base nueva al cuerpo original; exit 1 |

El segundo control comprueba consumo efectivo: las etiquetas y los bytes conservados no bastan si el productor recibe otro valor. Las capturas de los tres mutantes se conservan, incluidos los cuerpos producidos y las aceptaciones indebidas. No se cuentan errores de compilación como detecciones. La compilación heredada G1 emitió en ambos modos la advertencia de importación `Write` no utilizada; stdout/stderr completos están preservados. No se modificó la biblioteca para quitarla.

## Evidencia y medición

[Contrato](CONTRATO_S14.md), [fijación](FIJACION_PREVIA.json), [custodia previa](CUSTODIA_PREVIA.json), [procedencia](PROCEDENCIA.json), [fuentes G1](FUENTES_S2.json), [código](codigo/bases.rs), [conductor](codigo/contraste.rs), [observaciones](OBSERVACIONES.json), [491 capturas recuperables](EVIDENCIAS_S14.json), [comandos](COMANDOS.json), [binarios](BINARIOS.json), [entorno](ENTORNO.json), [mediciones](MEDICIONES.json) y [reproductor](reproducir.py).

Pared total de campaña: {m['pared_campana_s']:.6f} s. Suma de pared de invocaciones: {m['pared_suma_invocaciones_s']:.6f} s. CPU usuario: {m['cpu_usuario_s']:.6f} s; sistema: {m['cpu_sistema_s']:.6f} s. Incluye compilación y mutantes; no mide preparación/publicación, coste de modelo o latencia profesional. RSS individual, tokens y coste no disponibles.

Aperturas verificadas antes de ejecutar: público `{pub['lenguaje']['commit']}`; laboratorio `{pub['laboratorio']['commit']}`. Fijación SHA-256 `{sha((D/'FIJACION_PREVIA.json').read_bytes())}`. Los rectores previamente leídos permanecen idénticos en el corte de entrada y constan en [RECTORES.json](RECTORES.json). Los artefactos fijados —incluido el README de apertura— conservan sus bytes; esta acta sucede aquel estado pendiente sin reescribirlo.

## Alcance y relevo

G/J recibe evidencia de **base documental de vigencia realmente consumida y separación de actos dentro de un proceso**. El [inventario A–L sucesor](MATRIZ_COBERTURA_A_L_RESULTADO_S14.json) conserva los criterios y resultados anteriores. Formato, capacidad y límite se recogen en [causas por etapa](CAUSAS_OBSERVADAS.json); no se convierten en códigos canónicos por esta campaña.

Las referencias y el conductor son confiables. La comparación de bytes no autentica por sí sola un origen; la captura previa del archivo no prueba ausencia de carrera hostil. No se acreditan sustitución de binario/regla/corpus arbitrarios, restauración/reinicio, persistencia adversaria, canales, pantalla, acto profesional, conducta de LLM o QueryResult/TransitionData ejecutivos del núcleo. Tampoco se prueba aquí la rama SolicitudDistinta ni todos los fallos de E/S que el código puede emitir.

**Siguiente paso:** recibir G/J en el alcance exacto anterior y delimitar la siguiente obligación todavía pendiente del montaje integrado. F conserva la distinción integral entre negativa del proveedor, esquema inválido, no admisión y comunicación fallida; B/E/K/L y las fronteras profesionales/materiales conservan sus productores y puertas. No se declara cerrada toda la integración ni se amplía el núcleo por analogía. La matriz de suficiencia S13 sigue gobernando toda futura promoción.

Se continúa recogiendo causas durante la integración para consolidar después el catálogo aplicable. S12 permanece vigente: la posición de agentes se valorará tras cerrar inmunología. No se decide la suficiencia general del primer universo CYB. P3/P4/P5/P6 conservan reservas y condiciones.
''')
assert len(captures)==491
(D/'RESULTADOS.md').write_text('# Resultado S14\n\n[Acta de resultado](ACTA_RESULTADO_S14.md) · [Matriz A–L](MATRIZ_COBERTURA_A_L_RESULTADO_S14.json) · [Observaciones](OBSERVACIONES.json).\n\nEl README y la fijación conservan el estado de apertura. El acta presente registra el resultado posterior: conforme exclusivamente en el montaje acotado.\n')
J(D/'COMPROBACION_POSTERIOR.json',{'fijacion_intacta':True,'invocaciones':len(logs),'observaciones':len(obs),'capturas':len(captures),'capturas_identicas_por_ejecucion':len(baseline),'cuerpos_completos_contra_esperados_previos':True,'original_y_base_recuperados_intactos':True,'criterios_A_L_y_resultados_S11_intactos':True,'limite':'Cotejo posterior de registros de la misma campaña; no nueva campaña funcional.'})
print('Resultado documental preparado:',len(obs),'observaciones,',len(captures),'capturas.')
