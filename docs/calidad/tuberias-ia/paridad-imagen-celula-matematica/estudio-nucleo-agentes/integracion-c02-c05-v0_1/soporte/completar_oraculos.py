from pathlib import Path
import json,shutil
R=Path('manifiesto-sv/checkout');B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';D=B/'integracion-c02-c05-v0_1'
def write(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
bank=json.loads((D/'BANCO_PREVIO.json').read_text());bank['estado']='COMPROMISO_PREVIO_SIN_EJECUCION';write(D/'BANCO_PREVIO.json',bank)
contexts=json.loads((D/'CONTEXTOS_CONFIABLES.json').read_text());oracles=json.loads((D/'ORACULOS.json').read_text());reg=json.loads((D/'REGISTRO_CONFIABLE.json').read_text())
for c,o in zip(contexts,oracles):
 o['recibo_esperado']=None
 if o['recibo_favorable']:
  o['recibo_esperado']={'version':'BIS-I0205-RECIBO/0.1','caso':c['id'],'resultado':'ENTREGA_DOCUMENTAL_CONCORDANTE','perfil_fuente':c['perfil_fuente'],'fuente_sha256':c['fuente']['sha256'],'soporte':c['soporte'],'dimensiones_ir':c['dimensiones_ir_esperadas'],'vinculo':c['vinculo'],'estado_sha256':c['estado_matematico']['sha256'],'convenio_sha256':reg['convenio']['sha256'],'geometria_sha256':c['geometria']['sha256'],'geometria_bytes':c['geometria']['bytes'],'contexto_entrega':c['entrega']}
write(D/'ORACULOS.json',oracles)
shutil.copyfile('bis-integracion/preparar.py',D/'soporte/preparar_estimulos.py')
shutil.copyfile(__file__,D/'soporte/completar_oraculos.py')
