from pathlib import Path
import json,hashlib,zipfile,csv,io,shutil
r=Path(__file__).resolve().parent;out=r/'entrega/intervalo-del-conjunto-de-estados'
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
p=out/'README.md';s=p.read_text();anchor='La reproducción comprueba el manifiesto SHA-256'
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

<a id="retp-161"></a>

## RETP-2026-161 · Intervalo del conjunto escrito de estados

12/09/2026. [Candidata, contrato y evidencia reproducible](tuberias-ia/intervalo-del-conjunto-de-estados/README.md). Cortes: Lenguaje 0ef4330b9bb18b09f5a386784d7d3baa559624b2; laboratorio 605de268874621538489554a3f1be0b83f1f4a1f, rama lab/playground-sv-permanente. Se mantienen las ramas existentes y el historial; sin apertura de ramas ni renumeración. Rectoras cotejadas: AGENTS, Pilares, perfiles/ensamblaje y transición/relevos. Continúan el contrato diagnóstico 109/110, workflow V2, plan 147 y la referencia de frame, significado humano, trazabilidad y fidelidad.

El rechazo states.len() != 3 recibe el intervalo completo de la colección escrita, con ambas llaves y contenido interior, excluyendo separador y comentarios posteriores. La condición cuenta etiquetas, incluidas repeticiones, no valores distintos. Se conserva exactamente su punto de emisión después de comprobar cierre y separador, antes de rule. El registro interno admite extremos inclusivos de token; los errores de un elemento mantienen el mismo índice en ambos. Extensión observable de COMPILER-DIAGNOSTICS/2 sin nuevas causas, variantes, mensajes, admisión ni IR. Dos intervalos antes ausentes se suceden expresamente; sus fuentes, IDs, causas y documento original se conservan; otros 92 esperados intactos.

48 fuentes nuevas (12 válidas, 36 inválidas), más 94 heredadas: 142 fuentes y 240 ensamblajes derivados, tres repeticiones por modo. 852 observaciones monofuente y 1.440 comprobaciones de ensamblaje sin inflar fuentes distintas. Incluye longitudes 1/2/4/6, seis permutaciones por perfil, comentarios con llaves, CRLF y precedencia. Huellas, intervalos UTF-8 y unidad/perfil de origen comprobados. Corpus 14/106 y API heredada idénticos; 239 unitarios por modo; 29 focales y relación anteriores; catálogo 13/26 y cuatro clientes negativos/seis errores de acceso conservados. Tres controles detectan ausencia de intervalo, exclusión de cierre y rechazo anticipado.

La primera ejecución se interrumpió con E0786 al consumir la biblioteca optimizada de la base, cuya compilación devolvió cero. Se preservan el artefacto comprimido, sus huellas y todas las salidas del intento. Repetición completa con fuentes, instrumento y opciones idénticos: conforme. La causa raíz de los metadatos corruptos permanece sin determinar; no se acredita una corrección de rustc. Cualificación conforme: 41 procesos; complemento: 10; reproducción completa en directorio nuevo con fuentes y resultados idénticos. El intento interrumpido y sus costes se conservan aparte.

Un archivo de la cápsula modificado y otros 76 idénticos; sin promoción productiva. Medidas incluyen compilación y no acreditan rendimiento relativo, coste por caso, dinero o inferencia. Siguiente objeto acotado: determinar alcanzabilidad y tratamiento de las conversiones defensivas restantes antes de atribuirles ubicación o cobertura. Después continúan validadores y subcausas del paso 6. P6/DG global y DFL-001/011 siguen abiertos; fila 9 y prioridades conservadas, P3 cerrado, P4/P5 con sus compuertas. Encargo externo, destinos y recepción profesional pendientes. No se acredita fidelidad final del conocimiento presentado. Estado: CONFORME_EN_ALCANCE_NATIVO; INCIDENCIA_INSTRUMENTAL_CONSERVADA; CIERRE_GLOBAL_PENDIENTE.
'''
old=(r/'registro.md').read_bytes();assert b'retp-161' not in old;(r/'registro-nuevo.md').write_bytes(old+entry.encode())
row=['RETP-2026-161','12/09/2026','','CANDIDATA_DIAGNOSTICA','Paso 6 / intervalo de colección / ES-EN','Intervalo original de llaves y contenido al rechazar longitud de estados','Rango ausente en rechazo de número de estados escritos distinto de tres','Pilares; perfiles; transición; RETP-109/110; workflow V2; RETP-147/160; frame y fidelidad','Cápsula, esperados y sucesión, inventario, incidente E0786 y reproducción en Calidad/laboratorio','239 unitarios por modo; corpus 14/106 idéntico; 48 nuevas y 94 anteriores; 240 ensamblajes x3x2; sensibilidad 3/3; reproducción portátil','Extensión diagnóstica /2; API heredada e IR conservadas','Incidente de metadatos con causa no determinada; conversiones defensivas; validadores y DG global pendientes','Determinar alcanzabilidad de conversiones defensivas; encargo externo pendiente','CONFORME_EN_ALCANCE_NATIVO_INCIDENCIA_CONSERVADA_CIERRE_GLOBAL_PENDIENTE']
assert len(row)==14
buf=io.StringIO(newline='');csv.writer(buf,lineterminator='\r\n').writerow(row)
old=(r/'registro.csv').read_bytes();assert b'RETP-2026-161' not in old;(r/'registro-nuevo.csv').write_bytes(old+buf.getvalue().encode())
adenda='''

### Recepción RETP-161 · Intervalo del conjunto escrito de estados

[Resultado y límites](tuberias-ia/intervalo-del-conjunto-de-estados/README.md). DFL-001 recibe ubicación de llaves y contenido para el rechazo por número de etiquetas escrito distinto de tres, preservando condición y precedencia. Dos intervalos ausentes se suceden expresamente; otros 92 esperados intactos. 48 fuentes nuevas, 142 totales, 240 ensamblajes derivados, corpus 14/106 idéntico, 239 unitarios por modo y tres controles de sensibilidad. Reproducción portátil conforme; una incidencia E0786 de lectura de la biblioteca base en el primer intento queda preservada con artefacto y causa raíz sin determinar.

Siguen pendientes la alcanzabilidad y tratamiento de conversiones defensivas, los demás validadores y subcausas, presentación/serialización y DG global. DFL-011 y demás deudas no se cierran. Sin promoción productiva, acreditación de fidelidad final, WASI/navegador, recepción profesional ni encargo externo. P3 permanece cerrado y P4/P5 conservan sus compuertas. Se mantienen las ramas existentes y el historial.
'''
(r/'deuda-nueva.md').write_bytes((r/'deuda.md').read_bytes()+adenda.encode())
notice='> **Corte vigente · RETP-161:** [intervalo del conjunto escrito de estados](intervalo-del-conjunto-de-estados/README.md). Ubicación completa y precedencia conservada; 48 fuentes nuevas más 94 anteriores, corpus 14/106 idéntico y reproducción portátil. Incidencia E0786 inicial preservada. Siguiente objeto: alcanzabilidad de conversiones defensivas. Se mantienen las ramas existentes. Referencia conceptual: [Léame primero del frame](frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md). DG global y encargo externo pendientes; las entradas siguientes conservan su corte histórico.\n\n'
for key in ['lenguaje','laboratorio']:(r/(key+'-inicio-nuevo.md')).write_bytes(notice.encode()+(r/(key+'-inicio.md')).read_bytes())
print('Fuentes y resultados portátiles idénticos;',len(entries),'archivos más manifiesto y ZIP;', (out/'PAQUETE_REPRODUCIBLE.zip').stat().st_size,'bytes')
