from pathlib import Path
import json,shutil,hashlib
R=Path(__file__).resolve().parent;D=R/'entrega';D.mkdir(exist_ok=True);(D/'codigo').mkdir(exist_ok=True);(D/'especimenes').mkdir(exist_ok=True)
J=lambda p,x:p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n');sha=lambda b:hashlib.sha256(b).hexdigest()
shutil.copyfile(R.parent/'s14-bases/entrega/FUENTES_S2.json',D/'FUENTES_S2.json')
shutil.copyfile(R.parent/'s14-bases/entrega/especimenes/negativo.cuerpo',D/'especimenes/negativo.cuerpo')
x=json.loads((R.parent/'s14-bases/entrega/RECTORES.json').read_text());x['corte']=json.loads((R/'lenguaje-base.json').read_text())['head'];x['objeto_actual']='S15 F: receptor sintético con causas diferenciadas y cobertura S2 intacta.'
tree=json.loads((R/'lenguaje-arbol.json').read_text())
for p,h in x['piezas_sin_cambio_desde_lectura_previa'].items():assert tree[p]==h
J(D/'RECTORES.json',x)
J(D/'PROCEDENCIA.json',dict(version='S15-PROCEDENCIA/1',cortes={k:json.loads((R/(k+'-base.json')).read_text()) for k in ['lenguaje','laboratorio']},fuentes_s2_sha256=sha((D/'FUENTES_S2.json').read_bytes()),esperado_cuerpo='Cuerpo negativo S11/S14 previo, completo y sin regeneración.',novedad='Receptor de protocolo binario de laboratorio y diagnóstico; ninguna modificación de bibliotecas anteriores.'))
