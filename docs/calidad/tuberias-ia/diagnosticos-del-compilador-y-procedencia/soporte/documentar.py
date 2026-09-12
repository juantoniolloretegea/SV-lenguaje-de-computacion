from pathlib import Path
import json,hashlib,shutil,zipfile,datetime,csv,io
r=Path('tmp/diagnosticos-158');o=Path('entregas/diagnosticos-del-compilador-y-procedencia');rec=r/'recepcion-01'
result=json.loads((rec/'RESULTADO.json').read_text());assert result['observaciones']==174
for f in ['RESULTADO.json','IDENTIDAD.json','SECUENCIA.json','OBSERVACIONES.json']:
 p=o/'evidencia'/f;p.parent.mkdir(parents=True,exist_ok=True);shutil.copy(rec/f,p)
# Full process bytes retained in the archive, including initial qualification.
with zipfile.ZipFile(o/'evidencia/REGISTROS_COMPLETOS.zip','w',zipfile.ZIP_DEFLATED) as z:
 for folder in [r/'evidencia',rec]:
  for p in sorted(folder.iterdir()):
   if p.is_file() and p.suffix in ['.json','.stdout','.stderr']:
    z.write(p,folder.name+'/'+p.name)
# Preserve exact scripts, including failed transformation, without inventing metrics.
for name in ['fijar.py','preparar.py','corpus.py','modificar-01.py','modificar.py','ejecutar.py','complemento.py','catalogar.py','empaquetar.py','documentar.py']:
 p=o/'soporte'/name;p.parent.mkdir(parents=True,exist_ok=True);shutil.copy(r/name,p)
proof=[]
for p in sorted((r/'candidata').rglob('*')):
 if p.is_file():
  rel=p.relative_to(r/'candidata');data=p.read_bytes();assert data==(r/'evidencia/fuentes-01'/rel).read_bytes();assert data==(rec/'candidata'/rel).read_bytes()
  proof.append({'ruta':str(rel),'sha256':hashlib.sha256(data).hexdigest()})
(o/'evidencia/FUENTES_PROBADAS.json').write_text(json.dumps({'primera_cualificacion_y_receptor_identicos':True,'archivos':proof},indent=2)+'\n')
(o/'HALLAZGOS.md').write_text('''# Hallazgos y tratamiento — RETP-158

1. **Pérdida de procedencia:** el parser anterior entregaba sólo IR. Se añade un sidecar generado durante la tokenización y el análisis, con rangos de declaraciones; la IR permanece idéntica. Los rangos son intervalos de bytes semiabiertos y el EOF es len..len de la unidad responsable.
2. **Causa reducida a prosa:** E004 vacío/repetición, E115 y colisiones se estructuran en sus propios emisores. Las API heredadas reciben la misma decisión y el mismo error, descartando únicamente la información adicional.
3. **Fallo del instrumento de edición:** la primera versión de modificar.py buscaba `fn tokenize(`; la función real tiene un parámetro de vida y empieza por `fn tokenize<'a>`. Falló con `ValueError: substring not found`, antes de escribir los archivos de la candidata. Se conserva modificar-01.py y la corrección modificar.py. No fue un fallo de compilación ni se cambió un esperado para ocultarlo. El tiempo de esa llamada no fue incorporado al registro automático de procesos y no se inventa.
4. **Cobertura multifuente insuficiente en la primera batería focal:** las colisiones y EOF estaban cubiertos; faltaba una relación E115 distribuida. Se fijó ESPERADO_RELACIONAL.json antes de ejecutar ese caso adicional. Tres declaraciones de cuatro unidades con igual nombre de archivo se localizan por índice y rol; sus cuatro SHA-256 se cotejan con Python. No se cambió la candidata.
5. **Límites explícitos:** los rangos sintácticos distintos de EOF permanecen pendientes cuando su emisor no da una ubicación inequívoca. Los caracteres léxicos inválidos sí conservan su rango. Los errores de perfil se rechazan y conservan el perfil, pero aún no tienen una subcausa específica de palabra extranjera. Otros validadores se identifican por etapa como CD.UNMIGRATED, sin deducir códigos de su texto.

No se observaron fallos inesperados de la candidata en las dos ejecuciones conservadas. Los rechazos del corpus inválido y los errores de acceso de los clientes negativos son resultados exigidos, no fallos del instrumento.
''')
(o/'CONTRATO_Y_ALCANCE.md').write_text('''# Contrato candidato y alcance — COMPILER-DIAGNOSTICS/1

## Una decisión, dos formas de entrega

`compiler_diagnostics::compile` y `compile_assembly` son entradas detalladas de la candidata. `compile_svp`, `compile_svp_profile` y `compile_svp_assembly` llaman al mismo recorrido y recuperan el error heredado. El parser sigue privado; ningún diagnóstico constituye autoridad ni autoriza ejecución. La IR conserva versión 0.3, estructura y serialización.

`Report` reúne una causa tipada, la etapa, el inventario de unidades originales y los lugares relacionados. Sus campos se construyen dentro del núcleo y se consultan por referencias inmutables. El inventario contiene índice de unidad, nombre, perfil, tamaño en bytes y SHA-256. Los lugares contienen índice, rol y rango opcional. Se distingue el inventario de entradas de las declaraciones efectivamente implicadas; un archivo del inventario no queda acusado por aparecer en él.

## Causas migradas

E004 distingue codominio vacío de miembros repetidos. E115 conserva celda, semántica y codominio cuando existen, y listas separadas de claves repetidas, ausentes y ajenas. La semántica autónoma con duplicados no recibe un codominio inventado. La colisión conserva ambas declaraciones; se mantiene la precedencia anterior, que registra objetos antes que operaciones. El orden de detección no se presenta como orden textual cuando difieren.

Los seis tipos de FrontendError se conservan mediante correspondencia entre variantes, nunca mediante lectura de Debug. Los validadores pendientes reciben CD.UNMIGRATED y su etapa; no se extrae un código SV del texto heredado. Los códigos canónicos acreditados aquí siguen siendo E004 y E115. Las doce claves CD.* pertenecen al contrato local, no amplían el catálogo canónico.

## Procedencia y presentación

Los rangos se obtienen del texto original durante el análisis. Son intervalos [inicio, fin), medidos en bytes desde cero; EOF es [len, len). Una declaración completa puede ser el rango relacionado sin que se publique su texto. No se reconstruye fuente desde IR. Dos unidades con igual nombre se distinguen por índice, perfil y huella.

Los mensajes son plantillas estáticas ES/EN. Una entrada monolingüe recibe su idioma; un ensamblaje mixto dispone de ambos. Con cero unidades no se inventa un perfil: messages() está vacío y el consumidor puede solicitar explícitamente una plantilla. La localización no modifica el juicio.

La presentación ordinaria y Debug de Report no incluyen las fuentes ni literales como SECRETO. Los parámetros tipados y nombres originales siguen disponibles para un receptor autorizado: cualquier interfaz que los represente como HTML/JSON deberá aplicar el escape del formato correspondiente. Esta candidata no añade un serializador público de diagnósticos. legacy() e into_legacy() son accesos explícitos compatibles y pueden contener texto original; no deben confundirse con una salida ya saneada para publicación.

## Qué queda abierto

Migración del resto de emisores y causas de perfiles; rangos sintácticos exactos distintos de EOF; serialización/presentación final de parámetros; recepción de bytes inválidos en adaptadores; integración y medición WASI/navegador. El contrato global DG01–DG14 y DFL-001/011 no se cierran por este subconjunto. Se mantiene DFL-005/006 para admisión profesional y verificadores. La reserva P3 permanece cerrada y P4/P5 conservan sus compuertas.
''')
(o/'ESTADO_PARA_OTRAS_IA.md').write_text('''# Estado de la prueba entre modelos

Juan Antonio trasladará el encargo común cuando esté listo. No se ha contactado ni ejecutado a Claude, Grok, Qwen o DeepSeek. Se conserva su aclaración: Grok dispone de acceso al laboratorio privado; Claude no; el acceso de Qwen/DeepSeek no está confirmado. La entrega común se preparará para lectura en SVcustos, con el mismo material y las mismas instrucciones para todos. No se cambia la visibilidad del laboratorio.

Este paquete contiene código candidato, fuentes, asserts y resultados visibles. Permite **reproducir y auditar** RETP-158. Por su exposición no es una prueba ciega de **resolución independiente**. Para ésta hacen falta un enunciado común y un conjunto de resultados/criterios fijados y custodiados antes de recibir respuestas, con acceso efectivo comprobado para cada participante. La reserva P3 no se utiliza ni se reabre mediante este paquete público.

La evaluación separará corrección respecto del contrato, evidencias recuperables y recursos observados. No exige el mismo programa ni la misma secuencia de operaciones. Una discrepancia se registra en el caso y se coteja con el esperado; el acuerdo entre modelos por sí solo no sustituye ese cotejo. Falta de acceso, falta de ejecución y fallo de resultado son estados distintos.

La traza solicitada incluirá acciones observables, versiones, entradas, scripts, código, conectores utilizados, salidas, errores y medidas efectivas, con una explicación breve de la función de cada acción. Es documentación auditable del trabajo; no se presenta como acceso íntegro a los mecanismos internos privados de un modelo. Los valores no medidos se declaran no disponibles.

**Aviso de entrega a modelos externos: todavía pendiente.** Este corte cierra una prueba nativa de diagnósticos; no acredita aún el cierre conjunto de la adenda de ciberseguridad, las dos fases y todo el catálogo/localización.
''')
(o/'README.md').write_text(f'''# Diagnósticos del compilador y procedencia por unidad

**RETP-158 · 12 de septiembre de 2026 · Candidata experimental.**

E004/E115 y las colisiones conservan ahora causas y referencias a declaraciones originales. EOF mantiene su unidad responsable. La versión anterior y la candidata producen exactamente los mismos resultados en las 120 fuentes canónicas. El cambio está ensayado y conservado como cápsula; no se ha promovido al núcleo productivo.

## Leer en este orden

1. [Plan previo](PLAN_PREVIO.md) e [inventario de emisores previo](INVENTARIO_PREVIO.json).
2. [Contrato y límites](CONTRATO_Y_ALCANCE.md).
3. [Hallazgos, incluida la corrección del instrumento](HALLAZGOS.md).
4. [Catálogo](CATALOGO.json), [español](idiomas/es/diagnosticos.json) e [inglés](idiomas/en/diagnosticos.json).
5. [Resultado del receptor](evidencia/RESULTADO.json), [secuencia](evidencia/SECUENCIA.json), [observaciones](evidencia/OBSERVACIONES.json) y [registros completos](evidencia/REGISTROS_COMPLETOS.zip).
6. [Estado del encargo entre modelos](ESTADO_PARA_OTRAS_IA.md).

## Resultado comprobado

| Comprobación | Resultado |
|---|---|
| Unitarios heredados de RETP-157 | 239/239 en debug y 239/239 en release |
| Corpus canónico | 14 válidos y 106 inválidos; resultado heredado idéntico entre ambas candidatas en ambos modos |
| Casos focales | 29 distintos; tres repeticiones por modo; 174 observaciones en el receptor |
| Procedencia | UTF-8, CRLF, tabulación, EOF, colisiones y E115 entre unidades homónimas |
| Localización | 12 claves locales con mensajes ES/EN; E004/E115 mantienen sus códigos canónicos |
| Fronteras de acceso | Cuatro clientes negativos, seis errores de compilación esperados; cliente público conforme |
| Receptor portátil | {result['procesos']} procesos, comandos, bytes de salida, huellas y medidas conservados |
| Otros proveedores | Ninguna ejecución recibida |

Las capturas de cualificación inicial y del receptor son distintas y están conservadas; no se suman repeticiones como casos nuevos. La cualificación inicial midió tiempo transcurrido; el receptor también registra CPU y máximo acumulado de memoria de hijos. Este último no es memoria por caso. No se midió coste monetario ni inferencia de proveedores.

## Reproducción

Descargue la carpeta completa o [PAQUETE_REPRODUCIBLE.zip](PAQUETE_REPRODUCIBLE.zip). Con Python 3.10 o posterior en Linux y rustc con biblioteca estándar nativa:

```sh
python reproducir.py /ruta/absoluta/rustc /ruta/a/un/directorio/nuevo
```

Se ha comprobado con rustc 1.98.0 (88d9e12ae178fab0fb5cc050a94da85685d449ea), destino x86_64-unknown-linux-gnu. El receptor recupera las cápsulas por SHA-256, ejecuta la base y la candidata, compara el corpus, ejecuta unitarios y casos focales y comprueba catálogo y clientes. No requiere red. Límite: 120 segundos por proceso y 2 MiB por flujo. El presupuesto del receptor no es un presupuesto de razonamiento de otra IA.

[Fuentes candidatas](CANDIDATA_FUENTES.json), [base RETP-157](BASE_FUENTES.json), [corpus](CORPUS_FUENTES.json), [parche](CAMBIO_INCREMENTAL.patch) y scripts están disponibles. El oráculo de las 28 primeras pruebas fue fijado en las assertions de focal.rs antes de ejecutarlo; INDICE_ESPERADOS.json es su índice posterior para el receptor, no un segundo oráculo independiente. El caso relacional adicional tiene su esperado fijado por separado.

## Corte y continuidad

Lenguaje: 4cacf3ec6bd5d0f31206197c7374515a56b34a89. Laboratorio: 1ac32131e2c328e395d828268a5e3517ce43fcca. Rectoras consultadas: pilares; acta de perfiles; acta de transición IMM y sus relevos; workflow acotado V2; contrato diagnóstico 109/110. Se mantiene el paso 6 del plan de integración, distinto de P6 del workflow. Los cimientos conceptuales del frame y de la auditoría del trabajo IA se conservan en sus carpetas troncales.

**Estado global: NO VERDE.** Continúan los emisores restantes, DG global, destinos WASI/navegador, recepción profesional y las compuertas de la campaña. La conformidad de esta candidata nativa no cambia esos estados.
''')
# Registry updates require exact base blob equality.
trees={k:{x['path']:x['sha'] for x in json.loads((r/(k+'-tree.json')).read_text())['tree']} for k in ['lenguaje','laboratorio']}
def blob(p):
 b=p.read_bytes();return hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
base=Path('tmp/continuacion-157');link='diagnosticos-del-compilador-y-procedencia/README.md'
for k in ['lenguaje','laboratorio']:
 path=('docs/calidad/tuberias-ia' if k=='lenguaje' else 'laboratorio/tareas-watson/tuberias-ia')+'/inicio.md'
 p=base/(k+'-inicio-nuevo.md');assert blob(p)==trees[k][path]
 text=p.read_text();text=text.replace('**Último corte · RETP-157:**','**Corte anterior · RETP-157:**',1)
 text=text.replace('**Estado global: NO VERDE.**','**Estado global: NO VERDE.**\n\n**Último corte · RETP-158:** [diagnósticos del compilador y procedencia por unidad]('+link+'). 239 unitarios por modo, corpus 14/106 idéntico y 29 casos focales nativos; restantes emisores y encargo externo aún pendientes.',1)
 (r/(k+'-inicio-nuevo.md')).write_text(text+'\n\n## RETP-158 · Paso 6: procedencia y causas del compilador\n\n[Resultado y reproducción]('+link+'). E004/E115 y colisiones tipados desde el emisor, sidecar por unidad fuera de IR, EOF original y catálogo ES/EN. Se conservan los fallos del instrumento y las mediciones. No hay promoción productiva ni nueva apertura de P3.\n')
p=base/'retp-nuevo.md';assert blob(p)==trees['lenguaje']['docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md']
entry='''

<a id="retp-158"></a>

## RETP-2026-158 · Causas del compilador, procedencia por unidad y ES/EN

12/09/2026. [Plan, contrato, fuentes y evidencia](tuberias-ia/diagnosticos-del-compilador-y-procedencia/README.md). Cortes Lenguaje 4cacf3ec6bd5d0f31206197c7374515a56b34a89 y laboratorio 1ac32131e2c328e395d828268a5e3517ce43fcca. Se continúa, por mandato humano, el paso 6 del plan de integración; no se confunde con P6 del workflow. Pilares, perfiles y transición cotejados; contrato diagnóstico 109/110 aplicado al alcance escogido.

Candidata aditiva sobre RETP-157: sidecar de rangos de bytes de declaraciones originales y metadatos de unidad fuera de IR; causas tipadas E004 vacío/repetición, E115 con listas separadas y colisiones con ambas declaraciones. FrontendError se conserva por variantes; EOF y carácter léxico inválido conservan posición. Los emisores restantes permanecen explícitamente sin migrar. Un recorrido común alimenta la API detallada y las API heredadas. Doce claves locales CD.* con ES/EN; sólo E004/E115 reciben los códigos canónicos ya existentes.

239 unitarios por modo debug/release. Corpus canónico 14 válido/106 inválido: aceptación y payload heredado idénticos entre base y candidata. 29 casos focales, tres repeticiones por modo, 174 observaciones en el receptor. Cuatro clientes negativos conservan seis errores de acceso exigidos; cliente público conforme. Se conservan cualificación y reproducción portátil con salidas, hashes, tiempos y medidas de recursos. Se documenta un fallo del script de edición previo a escribir la candidata, corregido sin cambiar esperados; no hubo fallo inesperado en las ejecuciones de la candidata.

La IR y el núcleo productivo no se modifican. Continúan rangos sintácticos restantes, subcausas de perfil, emisores de otros validadores, serialización diagnóstica, DG01–DG14 global y DFL-001/011; DFL-005/006, WASI/navegador y compuertas P3/P4/P5 permanecen abiertas o cerradas según su estado previo. Ningún proveedor externo ejecutado. SVcustos se mantiene como sede prevista del encargo común; el paquete abierto de reproducción no se confunde con prueba ciega. Estado: DIAGNOSTICOS_COMPILADOR_CANDIDATOS_NATIVOS_CONFORMES_CIERRE_GLOBAL_PENDIENTE.
'''
(r/'retp-nuevo.md').write_text(p.read_text()+entry)
p=base/'retp-nuevo.csv';assert blob(p)==trees['lenguaje']['docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv']
rows=list(csv.reader(io.StringIO(p.read_text())));assert not any(x and x[0]=='RETP-2026-158' for x in rows)
rows.append(['RETP-2026-158','12/09/2026','','CANDIDATA_DIAGNOSTICA','Paso 6 / compilador / procedencia / ES-EN','E004/E115 y colisiones desde el emisor; sidecar original por unidad','Pérdida de procedencia y causas textuales en el compilador','Pilares; perfiles; transición; RETP-109/110; workflow V2; RETP-157','Cápsulas, parche, corpus, catálogo ES/EN, instrumento y evidencia en Calidad/laboratorio','239 unitarios por modo; corpus 14/106 idéntico; 29 casos x3x2; cuatro clientes negativos','Información diagnóstica aditiva; IR y código productivo conservados','Emisores restantes; DG global; WASI/navegador; productor profesional y proveedores pendientes','Completar emisores y preparar encargo común en SVcustos con esperados custodiados','DIAGNOSTICOS_COMPILADOR_CANDIDATOS_NATIVOS_CONFORMES_CIERRE_GLOBAL_PENDIENTE'])
assert all(len(x)==14 for x in rows)
with (r/'retp-nuevo.csv').open('w',newline='') as f:csv.writer(f).writerows(rows)
p=base/'deuda-nueva.md';assert blob(p)==trees['lenguaje']['docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md']
(r/'deuda-nueva.md').write_text(p.read_text()+'''

### Recepción RETP-158 · Compilador y procedencia por unidad

[Resultado y límites](tuberias-ia/diagnosticos-del-compilador-y-procedencia/README.md). DFL-001 recibe causas tipadas E004/E115, colisiones y sidecar de fuentes originales fuera de IR; mensajes locales ES/EN. Corpus 14/106 idéntico, 239 unitarios por modo, 29 casos focales en tres repeticiones por modo. El conjunto no cierra DFL-001: faltan emisores restantes, subcausas de perfiles, rangos sintácticos no EOF, serialización/presentación y DG global. DFL-011 sigue abierta para revisión integral. DFL-005/006 conserva recepción profesional/verificadores pendientes. WASI/navegador, P3 reservado, P4/P5 y ausencia de ejecuciones externas mantienen su estado. No se promueve código productivo ni se hace público el laboratorio.
''')
# Final integrity inventory; ZIP is a transport envelope and excluded from itself.
manifest={'version':'RETP-158','archivos':[{'ruta':str(p.relative_to(o)),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(o.rglob('*')) if p.is_file() and p.name not in ['MANIFIESTO.json','PAQUETE_REPRODUCIBLE.zip']]}
(o/'MANIFIESTO.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n')
with zipfile.ZipFile(o/'PAQUETE_REPRODUCIBLE.zip','w',zipfile.ZIP_DEFLATED) as z:
 for p in sorted(o.rglob('*')):
  if p.is_file() and p.name!='PAQUETE_REPRODUCIBLE.zip':z.write(p,str(p.relative_to(o)))
print('Documentación, registros e integridad preparados:',len(manifest['archivos']),'archivos',flush=True)
