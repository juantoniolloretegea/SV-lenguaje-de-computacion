from pathlib import Path
from datetime import datetime,timezone
import ast,json,hashlib,shutil
R=Path(__file__).resolve().parent;D=R/'entrega';sha=lambda b:hashlib.sha256(b).hexdigest()
assert not (D/'FIJACION_PREVIA.json').exists()
for p in [D/'reproducir.py',*R.glob('*.py')]:ast.parse(p.read_text())
bank=json.loads((D/'contrato-s17/BANCO_OBLIGACIONES.json').read_text());assert len(bank['casos'])==24;assert (D/'codigo/ESPERADO.tsv').read_text()==''.join(f"{x['id']}\t{x['etapa_esperada']}\t{x['causa_esperada']}\n" for x in bank['casos'])
for n,h in bank['fuentes_esperados'].items():b=(D/'esperados'/n).read_bytes();assert len(b)==h['bytes'] and sha(b)==h['sha256']
(D/'README.md').write_text('''# S18 — Recorrido documental conjunto

Apertura previa a compilación. [Campaña y presupuesto](CAMPANA_S18.md) · [Realización Rust](codigo/recorrido.rs) · [Conductor](codigo/contraste.rs) · [Esperados](codigo/ESPERADO.tsv) · [Contrato S17 intacto](contrato-s17/CONTRATO_RECORRIDO_CONJUNTO.md).

24 casos; 144 observaciones normales previstas, cuatro sensibilidades y tres clientes externos. Máximo 29 invocaciones, sin reintentos. Resultado pendiente en esta apertura. [Procedencia](PROCEDENCIA.json), [rectores](RECTORES.json), [fuentes anteriores](FUENTES_S2.json), [fijación previa](FIJACION_PREVIA.json).

Reproducción en Linux x86_64, Rust 1.98.0 fijado:

```sh
python3 reproducir.py --rustc /ruta/absoluta/rustc --salida /ruta/nueva/s18
```

La apertura conservará sus bytes. El acta de resultado o interrupción se incorporará después. No constituye autoridad profesional, integración con proveedor real ni promoción nuclear.
''')
(D/'PREPARACION.json').write_text(json.dumps(dict(version='S18-PREPARACION/1',compilacion_previa=False,ejecucion_funcional_previa=False,alcance='Lectura de interfaces, implementación y cotejo documental de fuentes y esperados. Sin cualificación funcional previa.'),ensure_ascii=False,indent=2)+'\n')
A=D/'administracion';A.mkdir(exist_ok=True)
for n in ['snapshot.py','preparar.py','preparar_registro.py','registrar.py','publicar.py','fijar.py']:shutil.copyfile(R/n,A/n)
f=dict(version='S18-FIJACION/1',fecha_utc=datetime.now(timezone.utc).isoformat(timespec='seconds'),archivos={str(p.relative_to(D)):sha(p.read_bytes()) for p in sorted(D.rglob('*')) if p.is_file()})
(D/'FIJACION_PREVIA.json').write_text(json.dumps(f,ensure_ascii=False,indent=2)+'\n');print('Fijado sin compilar:',sha((D/'FIJACION_PREVIA.json').read_bytes()))
