from pathlib import Path
import json,hashlib,zipfile,csv,io,shutil
r=Path(__file__).resolve().parent;out=r/'entrega/ubicaciones-sintacticas-y-perfil-fuente'
def sha(b):return hashlib.sha256(b).hexdigest()
for path in ['evidencia/RESULTADO.json','complemento/RESULTADO.json']:
 assert json.loads((r/path).read_text())==json.loads((r/'recepcion-portatil'/path).read_text())
for folder in ['evidencia','complemento']:
 for p in (r/'recepcion-portatil'/folder).iterdir():
  if p.is_file() and p.suffix=='.json':
   dst=out/'recepcion_portatil'/folder/p.name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst)
p=out/'README.md';s=p.read_text();s=s.replace('o `PAQUETE_REPRODUCIBLE.zip`','o [PAQUETE_REPRODUCIBLE.zip](PAQUETE_REPRODUCIBLE.zip)')
s=s.replace('La reproducción comprueba el manifiesto', 'Se ejecutó además la reproducción completa desde el paquete en un directorio nuevo: sus 77 archivos por versión y los dos documentos de resultado coinciden exactamente con la cualificación inicial. Las salidas, tiempos y registros de esa ejecución se conservan por separado en [recepcion_portatil](recepcion_portatil/); no se suman como casos nuevos.\n\nLa reproducción comprueba el manifiesto')
p.write_text(s)
# Conserva el instrumento utilizado para consolidar la entrega.
shutil.copyfile(Path(__file__),out/'soporte/cerrar_expediente.py')
entries=[{'ruta':str(p.relative_to(out)),'bytes':p.stat().st_size,'sha256':sha(p.read_bytes())} for p in sorted(out.rglob('*')) if p.is_file() and p.name not in ['MANIFIESTO.json','PAQUETE_REPRODUCIBLE.zip']]
(out/'MANIFIESTO.json').write_text(json.dumps({'algoritmo':'SHA-256','archivos':entries},ensure_ascii=False,indent=2)+'\n')
with zipfile.ZipFile(out/'PAQUETE_REPRODUCIBLE.zip','w',zipfile.ZIP_DEFLATED,compresslevel=9) as z:
 for p in sorted(out.rglob('*')):
  if p.is_file() and p.name!='PAQUETE_REPRODUCIBLE.zip':z.write(p,str(p.relative_to(out)))
with zipfile.ZipFile(out/'PAQUETE_REPRODUCIBLE.zip') as z:
 assert z.testzip() is None
 for x in entries:assert sha(z.read(x['ruta']))==x['sha256']
entry='''

<a id="retp-159"></a>

## RETP-2026-159 · Ubicaciones sintácticas y perfil fuente

12/09/2026. [Candidata, contrato, reproducción y evidencia](tuberias-ia/ubicaciones-sintacticas-y-perfil-fuente/README.md). Cortes de entrada: Lenguaje 03d00131fb0df234101f01c9ca5dfa5f22886ceb; laboratorio ce202d9420adb2c6189fce557ad74e2a276c3a0c, rama lab/playground-sv-permanente. Rectoras consultadas: AGENTS, Pilares, perfiles y ensamblaje, transición completa y relevos, contrato diagnóstico 109/110, workflow V2 y plan de integración 147.

Incremento del paso 6 sobre la cápsula RETP-158: las primitivas sintácticas incluidas registran su intervalo original en el punto de rechazo. La incompatibilidad de grafía con el perfil se conserva desde la clasificación léxica y el contexto de comprobación; no se deduce del mensaje. La sucesión experimental COMPILER-DIAGNOSTICS/2 añade una variante y causa local con ES/EN. Los identificadores contextuales legítimos, comentarios y literales mantienen su tratamiento; las API heredadas, los códigos SV y la IR conservan su contrato.

40 fuentes focales (8 válidas, 32 inválidas), 64 ensamblajes derivados con unidad sana de otro perfil y nombre de archivo compartido, tres repeticiones por modo. Corpus canónico 14/106 con resultados heredados idénticos; 239 unitarios por modo y 29 focales anteriores, con sucesión expresa de dos esperados. 13 causas locales y 26 plantillas; dos controles de sensibilidad detectan causa ausente e intervalo desplazado, y dos controles ES/EN preservan intervalo ausente en emisores todavía pendientes. Cuatro clientes negativos mantienen seis errores de acceso. Cualificación y complemento: 41 y 7 procesos, respectivamente. Reproducción completa desde paquete: fuentes y documentos de resultado idénticos, con registros separados. Sin fallos inesperados de la candidata.

Se conservan comandos, binarios identificados, salidas, tiempo, CPU y máximo acumulado de memoria de procesos hijos. La medición incluye compilación y no acredita consumo por caso, coste incremental, rendimiento relativo ni coste monetario. Restan emisores compuestos y validadores, presentación/serialización final y DG global; WASI/navegador, recepción profesional y compuertas P3/P4/P5 mantienen su estado. DFL-001/011 y las demás deudas no se cierran. Código productivo sin modificar; laboratorio privado; encargo externo pendiente. Siguiente objeto: completar los emisores diagnósticos restantes en el mismo paso. Estado: CONFORME_EN_ALCANCE_NATIVO; CIERRE_GLOBAL_PENDIENTE.
'''
old=(r/'registro.md').read_bytes();assert b'retp-159' not in old;(r/'registro-nuevo.md').write_bytes(old+entry.encode())
row=['RETP-2026-159','12/09/2026','','CANDIDATA_DIAGNOSTICA','Paso 6 / sintaxis / perfil fuente / ES-EN','Intervalos originales en primitivas y causa de grafía incompatible desde el emisor','Pérdida de ubicación sintáctica y causa genérica de perfil','Pilares; perfiles; transición; RETP-109/110; workflow V2; RETP-147/158','Cápsulas, cambio incremental, casos, catálogo y evidencia en Calidad/laboratorio','239 unitarios por modo; corpus 14/106 idéntico; 40 fuentes y 64 ensamblajes x3x2; sensibilidad 2/2; reproducción portátil','Diagnóstico experimental /2; API heredada e IR conservadas','Emisores restantes; DG global; WASI/navegador; recepción profesional pendientes','Completar emisores diagnósticos; encargo externo pendiente','CONFORME_EN_ALCANCE_NATIVO_CIERRE_GLOBAL_PENDIENTE']
assert len(row)==14
buf=io.StringIO(newline='');csv.writer(buf,lineterminator='\r\n').writerow(row)
old=(r/'registro.csv').read_bytes();assert b'RETP-2026-159' not in old;(r/'registro-nuevo.csv').write_bytes(old+buf.getvalue().encode())
adenda='''

### Recepción RETP-159 · Ubicaciones sintácticas y perfil fuente

[Resultado, alcance y límites](tuberias-ia/ubicaciones-sintacticas-y-perfil-fuente/README.md). DFL-001 recibe intervalos originales en primitivas sintácticas y una causa local de grafía incompatible con ES/EN en el punto de rechazo. La sucesión experimental /2 conserva resultados heredados en el corpus 14/106 y en 40 fuentes y 64 ensamblajes focales; 239 unitarios por modo. Se conservan 13 causas locales, 26 plantillas, sensibilidad y reproducción portátil.

La cobertura global sigue abierta: emisores compuestos y validadores restantes, subcausas, serialización/presentación y DG01–DG14. DFL-011 conserva su revisión integral. Las ausencias de WASI/navegador, recepción profesional y ejecuciones externas mantienen su alcance; P3 reservado y P4/P5 conservan sus compuertas. Este incremento no modifica el núcleo productivo ni cierra las restantes deudas.
'''
(r/'deuda-nueva.md').write_bytes((r/'deuda.md').read_bytes()+adenda.encode())
notice='> **Corte vigente · RETP-159:** [ubicaciones sintácticas y perfil fuente ES/EN](ubicaciones-sintacticas-y-perfil-fuente/README.md). Candidata nativa comprobada: 40 fuentes, 64 ensamblajes derivados, corpus 14/106 conservado y reproducción portátil. Emisores restantes, DG global y encargo externo pendientes. Las entradas siguientes conservan su corte histórico.\n\n'
for src,dst in [('inicio.md','lenguaje-inicio-nuevo.md'),('inicio-laboratorio.md','laboratorio-inicio-nuevo.md')]:
 (r/dst).write_bytes(notice.encode()+(r/src).read_bytes())
print('Expediente:',len(entries),'archivos más manifiesto y ZIP;', (out/'PAQUETE_REPRODUCIBLE.zip').stat().st_size,'bytes comprimidos')
