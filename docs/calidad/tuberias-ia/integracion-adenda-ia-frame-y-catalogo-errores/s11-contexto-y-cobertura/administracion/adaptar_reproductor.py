from pathlib import Path
W=Path(__file__).resolve().parent.parent;D=W/'s11-integracion/entrega'
s=(W/'s3-presentacion/entrega/reproducir.py').read_text().replace('contraste S3','contraste S11').replace('S3_CAPTURAS','S11_CAPTURAS')
a=s.index("mut=dest/'fuentes-sin-comparacion'")
s=s[:a]+'''# Mutante 1: elimina la obligación de aportar la vigencia; debe caer en AH03.
shutil.copy2(dest/'sensibilidad/cobertura-sin-obligacion.rs',dest/'cobertura/cobertura.rs')
exe,_,_=build(dest/'sin-obligacion',dest/'fuentes-base');run('obligacion desactivada detectada',[exe],code=1,env=env(exe,dest/'sin-obligacion/capturas'),contains='FALLO AH03')
# Mutante 2: omite identidad contextual; bibliotecas normales de debug intactas.
shutil.copy2(dest/'sensibilidad/contexto-sin-identidad.rs',dest/'codigo/contexto.rs')
d=dest/'debug';flags=['--edition=2021','-C','opt-level=0','-C','overflow-checks=yes'];deps=['--extern',f'g1={d}/libg1.rlib','--extern',f'lote={d}/liblote.rlib','--extern',f'cobertura={d}/libcobertura.rlib','-L',f'dependency={d}'];exe=d/'sin-identidad'
run('sin identidad compilacion',[a.rustc,*flags,dest/'codigo/contraste.rs',*deps,'-o',exe]);run('identidad desactivada detectada',[exe],code=1,env=env(exe,d/'capturas-sin-identidad'),contains='FALLO AH06')
assert len(logs)==22
js(dest/'RESULTADO.json',dict(version='S11-RESULTADO/1',conforme=True,controles=10,ejecuciones=6,observaciones=60,capturas_identicas_por_ejecucion=len(captures),invocaciones=22,sensibilidades={'obligacion desactivada':'AH03','identidad contextual desactivada':'AH06'},pared_campana_s=time.monotonic()-start,limite='Conductor sintetico y host confiable; no se prueba resistencia de un LLM ni persistencia hostil del contexto.'))
print('S11 conforme en alcance fijado; presupuesto cerrado.',flush=True)
'''
# Compilador recuperado debe ser exactamente el binario anterior, además de misma versión.
s=s.replace("js(dest/'ENTORNO.json'", "assert sha(Path(a.rustc).read_bytes())=='3690cc576ede140504698405d5d8fa3826aaadbe71699c6c4ed0a565d6f493e2'\njs(dest/'ENTORNO.json'")
(D/'reproducir.py').write_text(s)
# Etiqueta explícita: el destino contiene el recibo; el documento queda conservado en las capturas.
for p in [D/'codigo/contraste.rs',D/'codigo/ESPERADO.tsv',D/'BANCO_FIJADO.json']:
 p.write_text(p.read_text().replace('ORDEN_COMO_DATO_ENTREGADA','ORDEN_CONSERVADA_SIN_AUTORIDAD'))
