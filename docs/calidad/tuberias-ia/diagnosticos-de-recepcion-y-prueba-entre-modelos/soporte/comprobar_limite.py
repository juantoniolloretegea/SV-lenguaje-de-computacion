"""Prueba el ejecutor real con límites abreviados; no repite la cualificación SV.
Extrae por AST las mismas funciones del instrumento, sin ejecutar su campaña.
"""
from pathlib import Path
import ast,base64,datetime,hashlib,json,os,selectors,signal,subprocess,sys,time,resource
here=Path(__file__).resolve().parents[1];work=Path(sys.argv[1]).resolve();work.mkdir(parents=True,exist_ok=False)
source=(here/'reproducir.py').read_text();tree=ast.parse(source)
functions=ast.Module(body=[n for n in tree.body if isinstance(n,ast.FunctionDef)],type_ignores=[])
logs=[];events=[];TIMEOUT_S=0.3
exec(compile(functions,str(here/'reproducir.py'),'exec'),globals())
results=[]
for id,code,expected in [
 ('salidas_abiertas','import time; time.sleep(10)','TIEMPO'),
 ('salidas_cerradas','import os,time; os.close(1); os.close(2); time.sleep(10)','TIEMPO'),
 ('exceso_salida','import os,time; os.write(1,b"x"*3000000); time.sleep(10)','SALIDA'),
]:
    try:run(id,[sys.executable,'-c',code])
    except AssertionError:pass
    else:raise AssertionError('el límite no se impuso: '+id)
    row=logs[-1]
    assert row['limite_excedido']==expected and row['codigo']<0,row
    assert row['duracion_ns']<3_000_000_000,row
    assert len(base64.b64decode(row['stdout_base64']))<=2097152
    results.append({'id':id,'limite':row['limite_excedido'],'codigo':row['codigo'],'duracion_ns':row['duracion_ns']})
save('RESULTADO_INSTRUMENTO.json',{'estado':'CONFORME','timeout_ensayo_s':TIMEOUT_S,'sha256_instrumento':hashlib.sha256(source.encode()).hexdigest(),'casos':results,'nota':'presupuesto abreviado del instrumento; no prueba P4 ni P5 del servicio SV'})
print('3 límites del instrumento comprobados')
