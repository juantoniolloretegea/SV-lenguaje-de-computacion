from pathlib import Path
import ast,json,hashlib,shutil
R=Path(__file__).resolve().parent;D=R/'entrega';A=D/'administracion';A.mkdir(exist_ok=True)
for n in ['snapshot.py','preparar.py','elaborar.py','crear_acta.py','preparar_registro.py','registrar.py','publicar.py','cerrar.py']:
 ast.parse((R/n).read_text());shutil.copyfile(R/n,A/n)
(D/'README.md').write_text('''# S17 — Contrato del recorrido documental conjunto

[Acta y alcance](ACTA_CONTRATO_S17.md) · [Contrato](CONTRATO_RECORRIDO_CONJUNTO.md) · [24 obligaciones](BANCO_OBLIGACIONES.json) · [Diagnósticos locales ES/EN](DIAGNOSTICOS.json).

Contrato documental fijado. Adaptación y ejecución conjunta pendientes. 144 observaciones normales previstas; cero realizadas. No acredita una integración completa, autoridad profesional ni cambios del núcleo.

[Fuentes recuperables](FUENTES_RECIBIDAS.json) · [Interfaces anteriores](INTERFACES_ANTERIORES.json) · [Pasajes exactos](PASAJES_INTERFACES.json) · [Cotejo](COTEJO_DOCUMENTAL.json) · [Rectores](RECTORES.json) · [Cortes](CORTE_Y_ALCANCE.json) · [Manifiesto](MANIFIESTO.json).

Reproducir el cotejo con `python3 verificar.py` desde esta carpeta. Sólo verifica custodia y correspondencia documental. Los scripts de administración conservan la elaboración y publicación en el entorno original; no son un verificador del núcleo SV.
''')
x=dict(version='S17-MANIFIESTO/1',archivos={str(p.relative_to(D)):dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in sorted(D.rglob('*')) if p.is_file() and p.name!='MANIFIESTO.json'});(D/'MANIFIESTO.json').write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
