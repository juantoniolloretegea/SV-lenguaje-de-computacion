from pathlib import Path
import json,hashlib,zipfile,csv,io,shutil
r=Path(__file__).resolve().parent;out=r/'entrega/ubicaciones-de-rechazos-compuestos'
def sha(b):return hashlib.sha256(b).hexdigest()
for n in ['evidencia/RESULTADO.json','complemento/RESULTADO.json']:
 assert json.loads((r/n).read_text())==json.loads((r/'recepcion-portatil'/n).read_text())
for version in ['base','candidata']:
 for p in (r/version).rglob('*'):
  if p.is_file():assert p.read_bytes()==(r/'recepcion-portatil'/version/p.relative_to(r/version)).read_bytes()
for folder in ['evidencia','complemento']:
 for p in (r/'recepcion-portatil'/folder).iterdir():
  if p.is_file() and p.suffix=='.json':
   dst=out/'recepcion_portatil'/folder/p.name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst)
p=out/'README.md';s=p.read_text();anchor='La reproducción valida el manifiesto SHA-256'
assert 'Se ejecutó la reproducción completa' not in s
s=s.replace(anchor,'Se ejecutó la reproducción completa desde la carpeta de entrega en un directorio nuevo: los 77 archivos por versión y ambos documentos de resultado coinciden exactamente con la cualificación inicial. Se conservan sus [registros separados](recepcion_portatil/), sin sumarlos como nuevas fuentes.\n\n'+anchor);p.write_text(s)
shutil.copyfile(Path(__file__),out/'soporte/cerrar_expediente.py')
entries=[dict(ruta=str(p.relative_to(out)),bytes=p.stat().st_size,sha256=sha(p.read_bytes())) for p in sorted(out.rglob('*')) if p.is_file() and p.name not in ['MANIFIESTO.json','PAQUETE_REPRODUCIBLE.zip']]
(out/'MANIFIESTO.json').write_text(json.dumps(dict(algoritmo='SHA-256',archivos=entries),ensure_ascii=False,indent=2)+'\n')
with zipfile.ZipFile(out/'PAQUETE_REPRODUCIBLE.zip','w',zipfile.ZIP_DEFLATED,compresslevel=9) as z:
 for p in sorted(out.rglob('*')):
  if p.is_file() and p.name!='PAQUETE_REPRODUCIBLE.zip':z.write(p,str(p.relative_to(out)))
with zipfile.ZipFile(out/'PAQUETE_REPRODUCIBLE.zip') as z:
 assert z.testzip() is None
 for item in entries:assert sha(z.read(item['ruta']))==item['sha256']
entry='''

<a id="retp-160"></a>

## RETP-2026-160 · Ubicaciones de rechazos compuestos

12/09/2026. [Candidata, contrato y evidencia reproducible](tuberias-ia/ubicaciones-de-rechazos-compuestos/README.md). Cortes: Lenguaje 76722442d2899300c69d89ab932890a4d7d74c2f; laboratorio afcbd5046b2eec24cb058ce0fedcf580ca4790e8, rama lab/playground-sv-permanente. Se cotejaron las rectoras leídas: AGENTS, Pilares, perfiles y ensamblaje, transición y relevos. Continúan el contrato diagnóstico 109/110, workflow V2 y plan 147, con la referencia conceptual de frame, significado humano, trazabilidad y fidelidad.

Doce puntos de rechazo conservan el índice original antes de consumir el elemento: ocho ramas de campos opcionales de SemanticRelation/Pattern, variantes de supervisión y consulta, palabra protegida en let y etiqueta individual de admisibilidad. La validación conserva su posición y precedencia. La grafía extranjera se clasifica desde el estado léxico y el contexto, sin interpretar prosa. Extensión observable de cobertura de COMPILER-DIAGNOSTICS/2, sin nuevas variantes ni plantillas. Los controles antiguos de rango ausente en campos opcionales quedan sucedidos expresamente; su expediente se conserva.

54 fuentes nuevas (2 válidas, 52 inválidas), más 40 anteriores con esperados intactos: 94 fuentes y 168 ensamblajes derivados, tres repeticiones en debug y release. 564 observaciones monofuente y 1.008 comprobaciones de ensamblaje, sin inflar el número de fuentes distintas. Huellas, intervalos UTF-8, índice y perfil de origen comprobados. Corpus 14/106 y resultados de API heredada idénticos; 239 unitarios por modo; 29 focales anteriores y prueba relacional conservados. Las mismas 13 causas y 26 plantillas; cuatro clientes negativos y seis errores de acceso. Dos controles detectan la ausencia de intervalo en la base y el índice desplazado en un mutante. Cualificación 41 procesos, complemento 7; reproducción completa en directorio nuevo con fuentes y resultados idénticos y registros separados. Sin fallos inesperados.

Sólo cambia frontend.rs de la cápsula experimental; otros 76 archivos idénticos. No se promueve código productivo. Las mediciones incluyen compilación y no acreditan rendimiento relativo, coste por caso, dinero ni inferencia. La cardinalidad incorrecta del conjunto de estados sigue sin intervalo, comprobada en ES/EN; las conversiones defensivas y demás validadores conservan sus pendientes. Siguiente objeto: ubicación del conjunto en cardinalidad, preservando precedencia y diferencia con etiqueta individual. Paso 6 incremental, P6 y DG global abiertos; DFL-001/011 y otras deudas sin cierre. Fila 9 y prioridades conservadas; P3 cerrado, P4/P5 con sus compuertas. Encargo externo, WASI/navegador y recepción profesional pendientes. La precisión diagnóstica no demuestra fidelidad final del conocimiento presentado. Estado: CONFORME_EN_ALCANCE_NATIVO; CIERRE_GLOBAL_PENDIENTE.
'''
old=(r/'registro.md').read_bytes();assert b'retp-160' not in old;(r/'registro-nuevo.md').write_bytes(old+entry.encode())
row=['RETP-2026-160','12/09/2026','','CANDIDATA_DIAGNOSTICA','Paso 6 / rechazos compuestos / fuente ES-EN','Doce emisores conservan posición original y precedencia','Pérdida de ubicación tras consumir campos o variantes','Pilares; perfiles; transición; RETP-109/110; workflow V2; RETP-147/159; frame y fidelidad','Cápsula, cambio, esperados, inventario y reproducción en Calidad/laboratorio','239 unitarios por modo; corpus 14/106 idéntico; 54 fuentes nuevas más 40 anteriores; 168 ensamblajes x3x2; sensibilidad 2/2; reproducción portátil','Extensión diagnóstica /2; API heredada e IR conservadas','Cardinalidad sin intervalo; conversiones defensivas; validadores y DG global; destinos y recepción pendientes','Ubicar el conjunto en error de cardinalidad; encargo externo pendiente','CONFORME_EN_ALCANCE_NATIVO_CIERRE_GLOBAL_PENDIENTE']
assert len(row)==14
buf=io.StringIO(newline='');csv.writer(buf,lineterminator='\r\n').writerow(row)
old=(r/'registro.csv').read_bytes();assert b'RETP-2026-160' not in old;(r/'registro-nuevo.csv').write_bytes(old+buf.getvalue().encode())
adenda='''

### Recepción RETP-160 · Ubicaciones de rechazos compuestos

[Resultado y límites](tuberias-ia/ubicaciones-de-rechazos-compuestos/README.md). DFL-001 recibe ubicación original en doce emisores compuestos, conservando orden de validación y causa desde el contexto léxico. Se comprobaron 54 fuentes nuevas y 40 anteriores, 168 ensamblajes derivados, corpus 14/106 idéntico, 239 unitarios por modo, catálogo intacto, controles de sensibilidad y reproducción portátil. Cambio limitado a un archivo de la cápsula; no promovido al núcleo productivo.

Permanece pendiente la ubicación del conjunto de estados al fallar cardinalidad: conserva ausencia explícita de intervalo en ES/EN. Continúan conversiones defensivas y validadores restantes, subcausas, serialización/presentación y DG global. DFL-011 y otras deudas no se cierran. Tampoco se acredita fidelidad final, WASI/navegador, recepción profesional ni encargo externo. P3 sigue cerrado; P4/P5 conservan sus compuertas.
'''
(r/'deuda-nueva.md').write_bytes((r/'deuda.md').read_bytes()+adenda.encode())
notice='> **Corte vigente · RETP-160:** [ubicaciones de rechazos compuestos](ubicaciones-de-rechazos-compuestos/README.md). Doce emisores con ubicación original y precedencia conservada; 54 fuentes nuevas más 40 anteriores, corpus 14/106 idéntico y reproducción portátil. Siguiente objeto: ubicación del conjunto de estados al fallar cardinalidad. DG global y encargo externo pendientes. Referencia conceptual: [Léame primero del frame](frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md). Las entradas siguientes conservan su corte histórico.\n\n'
for key in ['lenguaje','laboratorio']:(r/(key+'-inicio-nuevo.md')).write_bytes(notice.encode()+(r/(key+'-inicio.md')).read_bytes())
print('Fuentes y resultados portátiles idénticos;',len(entries),'archivos más manifiesto y ZIP;', (out/'PAQUETE_REPRODUCIBLE.zip').stat().st_size,'bytes')
