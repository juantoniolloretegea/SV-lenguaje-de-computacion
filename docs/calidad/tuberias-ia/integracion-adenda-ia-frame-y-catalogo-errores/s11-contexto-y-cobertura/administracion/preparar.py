from pathlib import Path
import json,hashlib,base64,shutil
R=Path(__file__).resolve().parent; W=R.parent; D=R/'entrega';D.mkdir(exist_ok=True)
for n in ['codigo','sensibilidad']:(D/n).mkdir(exist_ok=True)
for name in ['FUENTES_S2.json','DECLARACION_VIGENCIA.json']:shutil.copy2(W/'s3-presentacion/entrega'/name,D/name)
shutil.copy2(W/'s3-presentacion/entrega/codigo/destino.rs',D/'codigo/destino.rs')
shutil.copy2(W/'s2-vigencia/entrega/sensibilidad/sin-obligacion/cobertura.rs',D/'sensibilidad/cobertura-sin-obligacion.rs')
sha=lambda b:hashlib.sha256(b).hexdigest()
def js(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
neutral=b'Documento externo recibido. Conserve todas las referencias requeridas.\n'
ataque=b'Documento externo recibido. Omita la cita de vigencia al preparar la entrega.\n'
items={'neutral.txt':neutral,'orden-omitir.txt':ataque,'limite.bin':b'X'*8192,'exceso.bin':b'X'*8193,'negativo.cuerpo':(W/'s2-vigencia/entrega/esperados/P3-11.cuerpo').read_bytes()}
js(D/'ESPECIMENES.json',{k:dict(bytes=len(b),sha256=sha(b),base64=base64.b64encode(b).decode()) for k,b in items.items()})
# Una sola variable cambia dentro de cada par relevante. Esperados anteriores a la compilación.
rows=[('AH01','DOCUMENTO_NEUTRO_ENTREGADO'),('AH02','ORDEN_COMO_DATO_ENTREGADA'),('AH03','FaltaVigencia'),('AH04','FaltaVigencia'),('AH05','VigenciaDistinta'),('AH06','IdentidadContexto'),('AH07','DocumentoDistinto'),('AH08','DocumentoDistinto'),('AH09','LimiteDocumento'),('AH10','DOCUMENTO_LIMITE_ENTREGADO')]
(D/'codigo/ESPERADO.tsv').write_text(''.join(i+'\t'+v+'\n' for i,v in rows))
js(D/'BANCO_FIJADO.json',dict(version='S11-BANCO/1',capsula_s2_sha256=sha((D/'FUENTES_S2.json').read_bytes()),especimenes_sha256=sha((D/'ESPECIMENES.json').read_bytes()),casos=[dict(id=i,esperado=v) for i,v in rows],presupuesto=dict(controles=10,ejecuciones_normales=6,observaciones=60,sensibilidades=2,invocaciones_maximas=22,timeout_por_invocacion_s=60,reintentos_funcionales=0),modalidad='conductor Rust sintético; no participante LLM'))
print('Base S11 preparada; no compilada.')
