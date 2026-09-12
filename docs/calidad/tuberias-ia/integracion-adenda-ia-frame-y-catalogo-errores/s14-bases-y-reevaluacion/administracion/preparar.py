from pathlib import Path
import json,hashlib,base64,shutil
R=Path(__file__).resolve().parent;D=R/'entrega';(D/'codigo').mkdir(parents=True,exist_ok=True);(D/'especimenes').mkdir(exist_ok=True)
sha=lambda b:hashlib.sha256(b).hexdigest();blob=lambda b:hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest();J=lambda p,x:p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
tr=json.loads((R/'lenguaje-arbol.json').read_text());old=R.parent/'s11-integracion/entrega';prefix='docs/calidad/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/s11-contexto-y-cobertura/'
for n in ['FUENTES_S2.json','ESPECIMENES.json']:
 b=(old/n).read_bytes();assert blob(b)==tr[prefix+n];shutil.copyfile(old/n,D/n)
rect=json.loads((R.parent/'s13-suficiencia/entrega/RECTORES.json').read_text())
for p,h in rect['piezas_sin_cambio_desde_lectura_previa'].items():assert tr[p]==h,p
rect['corte']=json.loads((R/'lenguaje-base.json').read_text())['head'];rect['objeto_actual']='S14 G/J: bases documentales y recuperación/reevaluación; reutilización G1 sin modificación y sin promover núcleo.';J(D/'RECTORES.json',rect)
caps=json.loads((D/'FUENTES_S2.json').read_text());src={x['ruta']:x for x in caps['archivos']}
for out,p in [('solicitud.bin','fuentes-base/recibo-g1/candidata/fixtures/solicitud-a01.bin'),('positivo.cuerpo','fuentes-base/recibo-g1/candidata/fixtures/cuerpo-a01.bin')]:
 x=src[p];b=base64.b64decode(x['base64']);assert sha(b)==x['sha256'];(D/'especimenes'/out).write_bytes(b)
x=json.loads((old/'ESPECIMENES.json').read_text())['negativo.cuerpo'];b=base64.b64decode(x['base64']);assert sha(b)==x['sha256'];(D/'especimenes/negativo.cuerpo').write_bytes(b)
for n,b in [('base-original.bin',b'SV-S14-VIGENCIA/1\nvigente=1\n'),('base-nueva.bin',b'SV-S14-VIGENCIA/1\nvigente=0\n'),('base-invalida.bin',b'SV-S14-VIGENCIA/1\nvigente=U\n'),('base-excesiva.bin',b' '*65)]: (D/'especimenes'/n).write_bytes(b)
J(D/'PROCEDENCIA.json',{'corte':rect['corte'],'capsula_s2_sha256':sha((D/'FUENTES_S2.json').read_bytes()),'reuso':'G1 y dependencias intactas de FUENTES_S2. No se recompila lote-g1 ni se cambia su guarda de lote/montaje fijados. S2/S3/S11 son antecedentes; el nuevo recorrido no vuelve a acreditar su cobertura ni su archivo protegido.','esperado_positivo':'Cuerpo A01 anterior dentro de cápsula S2. Solicitud A01 exacta de esa misma cápsula.','esperado_negativo':'Cuerpo negativo P3-11 recibido en S11. Misma consulta/contexto admitidos; id de transporte no forma parte del cuerpo esperado. Se coteja el cuerpo entero; no se genera esperado con el ejecutable S14.','variable':'Único byte 1→0 en un campo de vigencia de base documental. Misma ruta de carga, solicitud, productor G1, política y fuente artificial. Esta base exterior de vigencia no reemplaza el campo base K-IE004/1 del cuerpo.','especimenes':{p.name:{'bytes':p.stat().st_size,'sha256':sha(p.read_bytes())} for p in (D/'especimenes').iterdir()}})
print('Fuentes y esperados anteriores recibidos; rectores sin cambios.')
