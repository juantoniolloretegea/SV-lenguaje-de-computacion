"""Conserva R05 y verifica banco/observaciones. Uso: conservar.py campaña comprobantes."""
from pathlib import Path
import json,sys,hashlib,shutil,datetime
E=Path(__file__).resolve().parent;O=Path(sys.argv[1]);C=Path(sys.argv[2]);D=E/'evidencia';assert not D.exists()
a=json.loads((C/'previo-publico.json').read_text());b=json.loads((C/'previo-laboratorio.json').read_text());assert a['verified'] and b['verified']
for n,h in json.loads((E/'PRECOMPROMISO.json').read_text()).items():assert hashlib.sha256((E/n).read_bytes()).hexdigest()==h,n
for n,x in json.loads((E/'FUENTES.json').read_text())['archivos'].items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==x['sha256'],n
orders=json.loads((O/'ORDENES.json').read_text());assert len(orders)==3 and all(x['exit_code']==0 for x in orders)
assert (O/'00.stdout').read_text().startswith('rustc 1.98.0 ');assert (O/'01.stdout').read_text().startswith('cargo 1.98.0 ')
assert '7 passed; 0 failed;' in (O/'02.stdout').read_text()
js={i:json.loads((O/'mutation'/f'P{i:02d}.json').read_text()) for i in range(1,8)}
for i in [1,2,3,7]:assert js[i]['preservacion']=='CONCORDANCIA_OBSERVADA'
for i in [2,3]:assert js[i]['entrega_previa']['recibo'] is None
assert js[3]['entrega_previa']['primera_guarda']=='D01';assert js[3]['entrega_previa']['despachos_observados'] is None
for i in [4,5]:assert js[i]['panico_observado'] and js[i]['informe_recepcion'] is None and js[i]['intentos_del_recorrido'] is None
assert js[5]['hook_posterior_alcanzado'];assert js[5]['estado_posterior_del_arnes']['vector'][0]=='One'
assert js[6]['preservacion']=='ALTERACION_DETECTADA';assert bytes(js[6]['antes']['contenido'])!=bytes(js[6]['despues']['contenido'])
for i in [6,7]:assert js[i]['inode_antes']!=js[i]['inode_reemplazo']
assert js[7]['entrega_previa']['recibo'] is not None and js[7]['antes']['contenido']==js[7]['despues']['contenido']
D.mkdir();shutil.copytree(O/'mutation',D/'casos')
for p in list(O.glob('*.stdout'))+list(O.glob('*.stderr'))+[O/'ORDENES.json']:shutil.copyfile(p,D/p.name)
(D/'HUELLAS.json').write_text(json.dumps({str(p.relative_to(D)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(D.rglob('*')) if p.is_file()},indent=2)+'\n')
result=dict(version='S26-R05-RESULTADOS/1',fecha_registro_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),precompromiso_lenguaje=a['commit'],precompromiso_laboratorio=b['commit'],campanas=1,sondas_aprobadas=7,sondas_fallidas=0,casos_globales_cerrados=0,oraculos_conservados=True,fuentes_anteriores_conservadas=True,hallazgos=['P02/P03: relectura concordante no convierte rechazo ni ausencia de captura en entrega.','P04/P05: pánico impide informe final; el arnés observa por separado y no rellena contador desconocido con cero.','P06: descriptor abierto observa bytes anteriores y la ruta después contiene otra versión; no hay recibo.','P07: bytes concordantes y recibo pese a objeto de fichero sustituido; igualdad de bytes no acredita identidad de fichero.'],limites=['Copia local Linux, no código productivo integrado.','Pánicos capturados únicamente por arnés; abort, terminación de proceso y alimentación pendientes.','Sustitución tras apertura y antes de lectura, no mutación durante el read.','Inode no se adopta como identidad durable o universal.','Sin BD, recuperación durable, GUI, host resistente ni cobertura clínica acreditados.'])
(E/'RESULTADOS.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
p=E/'README.md';p.write_text(p.read_text()+'''\n## Resultado / RETP-232

Una campaña Rust/Cargo 1.98.0 offline: siete sondas conformes al banco previo, cero fallos de aserción. Los dos mensajes de pánico en stderr corresponden a P04/P05 provocados deliberadamente y capturados sólo por el arnés. No se han corregido ni integrado esos caminos productivos.

P02/P03 conservan lectura posterior concordante sin recibo. P04/P05 no producen MaterialRun: la observación externa posterior no se atribuye al conductor y los intentos desconocidos permanecen null. P05 deja además alteración de archivo; ausencia de retorno no equivale a ausencia de efectos.

P06 demuestra la diferencia entre el objeto ya abierto y una ruta que pasa a designar otro archivo. P07 conserva los bytes y el recibo esperado pese a sustituir el objeto. Es un límite de la afirmación de identidad, no una corrupción semántica demostrada por sí sola. Un éxito de prueba puede ser un contraejemplo confirmado.

[Resultados](RESULTADOS.json), [órdenes](evidencia/ORDENES.json), [stdout](evidencia/02.stdout), [stderr](evidencia/02.stderr), [huellas](evidencia/HUELLAS.json). `evidencia/casos` conserva informes por caso y archivos efectivamente usados/modificados. Precompromiso público: `'''+a['commit']+'''`; espejo: `'''+b['commit']+'''`.

Próximo incremento: recibir explícitamente la terminación sin informe y fijar qué identidad de fuente se exige antes de integrar; después cualificar abort/interrupción de proceso y el montaje completo. No prometer rollback por capturar un pánico. Doce casos globales siguen abiertos y Bis/S24 conservan sus dependencias.
''')
print(json.dumps(result,ensure_ascii=False))
