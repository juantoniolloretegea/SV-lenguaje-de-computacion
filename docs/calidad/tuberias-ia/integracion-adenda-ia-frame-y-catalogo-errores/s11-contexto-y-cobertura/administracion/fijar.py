from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,ast,base64
R=Path(__file__).resolve().parent;D=R/'entrega';W=R.parent
sha=lambda b:hashlib.sha256(b).hexdigest()
blob=lambda b:hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
def js(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
ast.parse((D/'reproducir.py').read_text())
tree=json.loads((R/'lenguaje-arbol.json').read_text());provenance=[]
for src,used in [('s3-presentacion/entrega/FUENTES_S2.json','FUENTES_S2.json'),('s3-presentacion/entrega/DECLARACION_VIGENCIA.json','DECLARACION_VIGENCIA.json'),('s3-presentacion/entrega/codigo/destino.rs','codigo/destino.rs'),('s2-vigencia/entrega/sensibilidad/sin-obligacion/cobertura.rs','sensibilidad/cobertura-sin-obligacion.rs'),('s2-vigencia/entrega/ESPERADOS.json',None)]:
 b=(W/src).read_bytes();paths=[p for p,h in tree.items() if h==blob(b)];assert paths,src
 if used:assert (D/used).read_bytes()==b
 else:
  q=json.loads(b)['P3-11.cuerpo'];assert base64.b64decode(q['base64'])==(W/'s2-vigencia/entrega/esperados/P3-11.cuerpo').read_bytes()
 provenance.append(dict(local_origen=src,uso=used or 'ESPECIMENES.json:negativo.cuerpo',bytes=len(b),sha256=sha(b),blob=blob(b),rutas_en_corte=paths))
js(D/'PROCEDENCIA.json',dict(corte=json.loads((R/'lenguaje-base.json').read_text())['head'],archivos=provenance))
js(D/'PREPARACION.json',dict(compilacion_previa=False,ejecucion_funcional_previa=False,incidencias=[{'tipo':'cotejo previo','hecho':'El esperado P3-11.cuerpo no existe como blob separado en Git; el primer cotejo de procedencia se detuvo.','resolucion':'Cotejado el contenedor ESPERADOS.json publicado y sus bytes base64; esperado local idéntico. Sin cambiar bytes ni criterios.'},{'tipo':'entorno','hecho':'La ruta histórica de rustc ya no estaba disponible; no había rustc en PATH.','resolucion':'Recuperado archivo oficial Rust 1.98.0; checksum oficial y hash del binario idéntico a S2/S3 comprobados.','afecta_esperados':False}],recuperacion_compilador=json.loads((R/'toolchain/RECUPERACION.json').read_text()),limites='Inspección y materialización previas. No se ha cualificado el código nuevo.'))
assert not (D/'FIJACION_PREVIA.json').exists()
js(D/'FIJACION_PREVIA.json',dict(version='S11-FIJACION/1',fecha_utc=datetime.now(timezone.utc).isoformat(),archivos={str(p.relative_to(D)):sha(p.read_bytes()) for p in sorted(D.rglob('*')) if p.is_file()}))
(D/'README.md').write_text('''# S11 — Recepción contextual y cobertura integrada

Estado de apertura: fijado antes de compilar. [Contrato](CONTRATO_S11.md), [banco](BANCO_FIJADO.json), [fijación previa](FIJACION_PREVIA.json), [matriz reconciliada](MATRIZ_COBERTURA_A_L_PREVIA_S11.json).

Diez controles y dos sensibilidades; se conservan S2/S3, referencia independiente y archivo final. Se ensaya una recepción contextual mínima de laboratorio. La orden de omitir evidencia se conserva como dato; se comprueba la omisión representada, sin atribuir conducta a un modelo.

Reproducción desde un directorio de salida inexistente:

```sh
python3 reproducir.py --rustc /ruta/absoluta/a/rustc --salida /ruta/nueva/s11
```

El reproductor exige Rust 1.98.0 y el hash de binario previo. Fuente oficial y checksum en PREPARACION.json. Los resultados sólo se acreditarán en el acta posterior a la campaña. No se alteran núcleo, IR, dominios ni paquete externo S4/S6.
''')
print('Fijación anterior a compilación:',sha((D/'FIJACION_PREVIA.json').read_bytes()))
