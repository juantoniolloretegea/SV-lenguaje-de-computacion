from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,base64,shutil
R=Path(__file__).resolve().parent;D=R/'entrega';E=R/'ejecucion';J=lambda p,x:p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n');sha=lambda b:hashlib.sha256(b).hexdigest()
f=json.loads((D/'FIJACION_PREVIA.json').read_text())
for n,h in f['archivos'].items():assert sha((D/n).read_bytes())==h,n
res=json.loads((E/'RESULTADO.json').read_text());assert res['conforme'];logs=json.loads((E/'COMANDOS.json').read_text());assert len(logs)==21
expected=(D/'codigo/ESPERADO.tsv').read_text().splitlines();normal=[x for x in logs if 'ejecución' in x['paso']];assert len(normal)==6
for x in normal:assert x['stdout'].encode()==(D/'codigo/ESPERADO.tsv').read_bytes() and x['exit_code']==0
for n in ['RESULTADO.json','COMANDOS.json','ENTORNO.json','BINARIOS.json']:shutil.copyfile(E/n,D/n)
obs=[];captures=[];baseline=None
for mode in ['debug','release']:
 for n in range(1,4):
  c=E/mode/('capturas-'+str(n));curr={p.name:sha(p.read_bytes()) for p in c.iterdir()};assert baseline is None or curr==baseline;baseline=curr
  for line in expected:
   id,reg=line.split('\t',1);raw=(reg+'\n').encode();assert (c/(id+'.registro')).read_bytes()==raw
   assert (c/(id+'.presentado')).read_bytes()==b'SV-S15-DIAGNOSTICO/1\n'+raw
   assert (c/(id+'.entrada-recibida')).read_bytes()==(c/(id+'.entrada-prevista')).read_bytes()
   assert (c/(id+'.cuerpo')).exists()==(id=='F01');assert (c/(id+'.motivo')).exists()==(id in ['F02','F10'])
   obs.append(dict(caso=id,modo=mode,repeticion=n,diagnostico=reg,entrada_sha256=sha((c/(id+'.entrada-recibida')).read_bytes()),evidencia=str(c.relative_to(E))))
  assert (c/'F01.cuerpo').read_bytes()==(D/'especimenes/negativo.cuerpo').read_bytes()==(c/'referencia.final').read_bytes()
  assert (c/'F10.motivo').read_bytes()==(D/'especimenes/negativo.cuerpo').read_bytes()
  assert (c/'F02.motivo').read_bytes()==b'Servicio no disponible'
  assert (c/'F08.entrada-recibida').read_bytes()==(c/'F01.entrada-recibida').read_bytes()
  assert (c/'F09.entrada-recibida').read_bytes()==(c/'F01.entrada-recibida').read_bytes()[:12]
  assert len((c/'F13.entrada-recibida').read_bytes())==16385
for p in sorted(E.rglob('*')):
 if p.is_file() and any(x.startswith('capturas') for x in p.relative_to(E).parts):
  b=p.read_bytes();captures.append(dict(ruta=str(p.relative_to(E)),bytes=len(b),sha256=sha(b),base64=base64.b64encode(b).decode()))
assert len(obs)==84;J(D/'OBSERVACIONES.json',obs);J(D/'EVIDENCIAS_S15.json',dict(version='S15-EVIDENCIAS/1',archivos=captures))
J(D/'MEDICIONES.json',dict(invocaciones=21,pared_campana_s=res['pared_campana_s'],pared_suma_invocaciones_s=sum(x['pared_s'] for x in logs),cpu_usuario_s=sum(x['cpu_usuario_s'] for x in logs),cpu_sistema_s=sum(x['cpu_sistema_s'] for x in logs),ejecuciones_normales=[{k:x[k] for k in ['paso','pared_s','cpu_usuario_s','cpu_sistema_s']} for x in normal],rss_individual=None,tokens=None,coste=None,origen='time.monotonic y diferencias resource.getrusage(RUSAGE_CHILDREN).',limite='Incluye compilación y mutantes; excluye preparación/publicación. Sin latencia de proveedor real.'))
causas=[]
for line in expected:
 id,cat,etapa,causa,cuerpo,llamadas=line.split('\t');causas.append(dict(caso=id,categoria=cat,etapa=etapa,causa=causa,cuerpo_entregado=cuerpo=='true',llamadas_cobertura=int(llamadas),alcance='SV-S15-CAUSAS-F/1: receptor sintético; no código canónico ni equivalencia con Tri.'))
J(D/'CAUSAS_OBSERVADAS.json',causas)
old=json.loads((R.parent/'s14-bases/entrega/MATRIZ_COBERTURA_A_L_RESULTADO_S14.json').read_text());x=json.loads(json.dumps(old));x['version']='REVISION-COBERTURA-S15-RESULTADO/1';x['corte_s15']=json.loads((D/'PROCEDENCIA.json').read_text())['cortes'];x['nota_s15']='F añadido dentro del receptor de laboratorio; criterios y resultados anteriores intactos.'
for c in x['casos']:
 c['resultado_s15']=dict(nuevo_ensayo=c['id']=='F',estado='CONFORME_EN_RECEPTOR_SINTETICO' if c['id']=='F' else 'SIN_CAMBIO_DE_ALCANCE',controles=['F'+str(i).zfill(2) for i in range(1,15)] if c['id']=='F' else [],cierre_universal=False,limite='Evento de negativa sintético, errores de Read inyectados, cobertura documental S2 y presentación textual confiable; no SDK/red/pantalla profesionales.' if c['id']=='F' else 'Alcance anterior conservado.')
assert [{k:v for k,v in c.items() if k!='resultado_s15'} for c in x['casos']]==old['casos'];J(D/'MATRIZ_COBERTURA_A_L_RESULTADO_S15.json',x)
pub=json.loads((R/'PUBLICACION-apertura.json').read_text());assert all(v['verified'] for v in pub.values());T=datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z');m=json.loads((D/'MEDICIONES.json').read_text())
(D/'ACTA_RESULTADO_S15.md').write_text(f'''# Acta S15 — Causas preservadas en recepción y no admisión

**CONFORME dentro de SV-S15-CAUSAS-F/1.** {T}. Watson / W-S0. S15 / RETP-186. Catorce casos en seis ejecuciones (tres debug y tres release): 84 observaciones, {res['capturas_identicas_por_ejecucion']} capturas idénticas por ejecución. Veintiuna invocaciones y tres controles de sensibilidad detectados en sus casos previstos. Sin reintentos ni cambios de fuentes/esperados después de fijar.

## Resultado material

El receptor acumula fragmentos, conserva los bytes y aplica el orden fijado: terminación de lectura, esquema del sobre, evento de protocolo y comprobación S2. La categoría, etapa, causa concreta, presencia de cuerpo y contador de cobertura llegan sin alteración al registro, su relectura y la presentación textual. Los motivos de negativa se conservan como bytes opacos.

F01 entrega exactamente el recibo anterior PERMISO_REVOCADO después de una comprobación de cobertura: entregar esa lectura documental no equivale a conceder permiso. F02/F10 registran negativa del evento de protocolo y nunca entregan cuerpo, incluso cuando el motivo es el cuerpo documental válido completo. F03/F04/F11/F13/F14 conservan las causas de esquema o límite; F05/F06/F12 conservan FaltaVigencia, Lectura(Identidad) y FaltaCaso del comprobador anterior.

F07/F08/F09 conservan TimedOut o ConnectionReset. F08 recibe los mismos bytes que F01, pero el error posterior impide decodificar/admitir; F09 conserva sus doce bytes parciales. Un EOF normal incompleto permanece Truncado. Ninguna causa se convirtió en U y sólo F01 materializó un cuerpo entregado. Las referencias previas permanecieron intactas.

Las bibliotecas G1, lote y cobertura S2 no se modificaron. El montaje prepara once posiciones del lote fijo por ejecución para obtener P3-11; los catorce casos son variaciones de recepción y no catorce llamadas a un modelo. La no admisión probada es documental, en S2; no se amplió la admisibilidad semántica nuclear.

## Sensibilidad

| Mutación fijada | Resultado observado |
| --- | --- |
| Proyectar no admisión como U | F05 falla con exit 1; el registro pierde la categoría requerida |
| Ignorar error tras recibir bytes | F08 falla con exit 1; aparece una entrega indebida |
| Admitir evento de negativa | F02 falla con exit 1; aparece un cuerpo indebido |

Se conservan los registros y cuerpos indebidos de los mutantes. Las compilaciones tuvieron éxito; no se contaron errores de compilación como sensibilidad. Las advertencias heredadas quedan en stdout/stderr completos; ninguna biblioteca fue editada para ocultarlas.

## Custodia, evidencia y medición

[Contrato](CONTRATO_S15.md), [fijación](FIJACION_PREVIA.json), [custodia previa](CUSTODIA_PREVIA.json), [procedencia](PROCEDENCIA.json), [rectores](RECTORES.json), [fuentes anteriores](FUENTES_S2.json), [receptor](codigo/receptor.rs), [conductor](codigo/contraste.rs), [esperados](codigo/ESPERADO.tsv), [observaciones](OBSERVACIONES.json), [{len(captures)} capturas recuperables](EVIDENCIAS_S15.json), [comandos](COMANDOS.json), [binarios](BINARIOS.json), [entorno](ENTORNO.json), [mediciones](MEDICIONES.json) y [reproductor](reproducir.py).

Apertura verificada antes de compilar: público `{pub['lenguaje']['commit']}`, laboratorio `{pub['laboratorio']['commit']}`. Fijación SHA-256 `{sha((D/'FIJACION_PREVIA.json').read_bytes())}`. Todos sus archivos conservan los bytes publicados; esta acta sucede al README de apertura sin reescribirlo.

Pared de campaña: {m['pared_campana_s']:.6f} s; suma de pared de invocaciones: {m['pared_suma_invocaciones_s']:.6f} s; CPU usuario {m['cpu_usuario_s']:.6f} s; sistema {m['cpu_sistema_s']:.6f} s. Incluye compilación y sensibilidad; no preparación/publicación ni latencia de modelo. RSS individual, tokens y coste no disponibles.

## Alcance y siguiente paso

F recibe evidencia de distinción de causas **en el receptor sintético completo aquí definido**. [Matriz A–L sucesora](MATRIZ_COBERTURA_A_L_RESULTADO_S15.json) y [causas por etapa](CAUSAS_OBSERVADAS.json) conservan el alcance de cada hallazgo. El contraste incluye controles sobre semántica de entrega negativa y fallos después de bytes completos; no se limita a cambiar etiquetas.

No se acreditan proveedor real ni autenticidad de su negativa, SDK, red, pantalla profesional, durabilidad frente a reinicio o host hostil. Flujo, referencias, proceso y archivos son confiables; un Read que no retorna necesita supervisión externa. No se ensayan exhaustivamente Tipo/Bandera, todos los ErrorKind ni memoria agotada. No se pretende conocer intenciones o procesos internos de una IA ni garantizar inmunidad universal a inyección o alucinación. El sobre y las causas S15 son de laboratorio, sin promoción a gramática/IR o códigos canónicos.

**Siguiente:** revisar por sede las obligaciones aún pendientes, especialmente B/E/K/L, y concretar cuál dispone de productor y evidencia ejecutable conforme a S13. El catálogo recibe ahora causas separadas de recepción, protocolo y cobertura; no se declara cerrado antes de acabar las fronteras aplicables. S12 sigue vigente: agentes se valorará después de inmunología; la suficiencia general de CYB no queda decidida aquí. P3/P4/P5/P6 conservan sus puertas y reservas. No hay promoción nuclear por analogía.
''')
(D/'RESULTADOS.md').write_text('# Resultado S15\n\n[Acta](ACTA_RESULTADO_S15.md) · [Matriz A–L](MATRIZ_COBERTURA_A_L_RESULTADO_S15.json) · [Observaciones](OBSERVACIONES.json).\n\nEl README fijado conserva la apertura. Esta acta registra el resultado posterior exclusivamente en el alcance del receptor sintético.\n')
J(D/'COMPROBACION_POSTERIOR.json',dict(fijacion_intacta=True,observaciones=len(obs),capturas=len(captures),capturas_identicas_por_ejecucion=len(baseline),invocaciones=len(logs),entrada_registro_relectura_presentacion_cotejados=True,cuerpo_completo_contra_esperado_anterior=True,criterios_y_resultados_anteriores_intactos=True,limite='Cotejo posterior de la misma campaña; no nueva ejecución.'))
print('Documentado',len(obs),'observaciones;',len(captures),'capturas.')
