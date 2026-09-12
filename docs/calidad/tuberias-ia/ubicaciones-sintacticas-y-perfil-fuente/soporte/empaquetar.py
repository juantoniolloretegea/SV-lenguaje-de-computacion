from pathlib import Path
import json,hashlib,base64,shutil,zipfile,statistics
r=Path(__file__).resolve().parent;out=r/'entrega/ubicaciones-sintacticas-y-perfil-fuente';out.mkdir(parents=True,exist_ok=True)
def sha(b):return hashlib.sha256(b).hexdigest()
def copy(src,dst):
 p=out/dst;p.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(r/src,p)
for name in ['IDENTIDAD_ENTRADA.json','PLAN_PREVIO.md','COMPROMISO_PREVIO.json','CASOS_ESPERADOS.json','CAMBIO_INCREMENTAL.patch','ejecutar.py','complemento.py','reproducir.py','focal.rs','focal_heredado_v2.rs','catalogo.rs','CATALOGO.json']:
 copy(name,name)
for name in ['preparar.py','modificar.py','empaquetar.py']:copy(name,'soporte/'+name)
for lang in ['es','en']:copy('idiomas/'+lang+'/diagnosticos.json','idiomas/'+lang+'/diagnosticos.json')
for name in ['reproducir.py','focal.rs','corpus.rs','relacional.rs','CATALOGO.json','CORPUS.json','CORPUS_FUENTES.json','catalogo.rs','ESPERADO_RELACIONAL.json']:
 copy('antecedente-158/'+name,'antecedente-158/'+name)
for p in (r/'antecedente-158/clientes').iterdir():copy(str(p.relative_to(r)),str(p.relative_to(r)))
for folder in ['evidencia','complemento']:
 for p in (r/folder).iterdir():
  if p.is_file() and p.suffix in ['.json','.patch']:copy(str(p.relative_to(r)),str(p.relative_to(r)))
copy('antecedente-158/CANDIDATA_FUENTES.json','BASE_FUENTES.json')
files=[]
for p in sorted((r/'candidata').rglob('*')):
 if p.is_file():
  b=p.read_bytes();files.append({'ruta':str(p.relative_to(r/'candidata')),'bytes':len(b),'sha256':sha(b),'base64':base64.b64encode(b).decode()})
(out/'CANDIDATA_FUENTES.json').write_text(json.dumps({'archivos':files},ensure_ascii=False,indent=2)+'\n')
for name in ['frontend.rs','compiler_diagnostics.rs']:copy('candidata/rust/sv_core/src/'+name,'cambios/rust/sv_core/src/'+name)
inventory=[]
p=r/'base/rust/sv_core/src/frontend.rs';fn=None
for n,line in enumerate(p.read_text().splitlines(),1):
 if line.strip().startswith('fn '):fn=line.strip().split('(')[0][3:]
 if 'FrontendError::' in line:inventory.append({'linea_base':n,'funcion':fn,'emisor':line.strip()})
(out/'INVENTARIO_EMISORES_BASE.json').write_text(json.dumps({'archivo':'rust/sv_core/src/frontend.rs','sha256':sha(p.read_bytes()),'menciones_frontend_error':inventory,'nota':'Inventario de menciones; incluye conversiones y pruebas. No equivale a cobertura de todos los emisores alcanzables.'},ensure_ascii=False,indent=2)+'\n')
logs=json.loads((r/'evidencia/PROCESOS.json').read_text());comp=json.loads((r/'complemento/PROCESOS.json').read_text())
metrics={'unidad':'proceso instrumental; incluye compilación y comprobadores','procesos_principales':len(logs),'procesos_complementarios':len(comp),'tiempo_principal_s':sum(x['duracion_ns'] for x in logs)/1e9,'cpu_usuario_principal_s':sum(x['cpu_usuario_s'] for x in logs),'cpu_sistema_principal_s':sum(x['cpu_sistema_s'] for x in logs),'rss_max_acumulado_hijos_principal_kib':max(x['rss_max_acumulado_hijos_kib'] for x in logs),'coste_monetario':None,'coste_inferencia':None,'comparacion_rendimiento':'No constituida: los comprobadores de base y candidata tienen trabajo de verificación distinto.'}
(out/'MEDIDAS.json').write_text(json.dumps(metrics,ensure_ascii=False,indent=2)+'\n')
readme='''# Ubicaciones sintácticas y perfil fuente ES/EN

**RETP-159 · 12 de septiembre de 2026 · Candidata experimental.**

Los errores de las primitivas sintácticas incluidas conservan ahora el intervalo de sus bytes originales. Cuando el punto de rechazo comprueba una forma constitutiva incompatible con el perfil seleccionado, entrega una causa específica y una explicación ES/EN. Los identificadores contextuales válidos, los comentarios y los literales conservan su tratamiento anterior.

## Resultado y evidencia

| Comprobación | Resultado |
|---|---|
| Fuentes focales nuevas | 40: 8 válidas y 32 inválidas |
| Ensamblajes derivados | 64: cada fuente inválida en ambas posiciones, junto a una unidad sana de otro perfil y con el mismo nombre de archivo |
| Repeticiones | Tres por modo; 240 observaciones monofuente y 384 comprobaciones de ensamblaje en la cualificación inicial |
| Corpus canónico | 14 fuentes válidas y 106 inválidas; resultados heredados idénticos entre base y candidata, en debug y release |
| Pruebas unitarias | 239/239 por modo |
| Focales anteriores | 29 por modo; dos esperados de grafía extranjera sucedidos expresamente en la nueva batería |
| Catálogo | 13 causas locales, 26 plantillas ES/EN; códigos SV conservados |
| Sensibilidad | El comprobador detecta la causa ausente en la base y el desplazamiento deliberado del intervalo |
| Emisores compuestos pendientes | Dos controles ES/EN conservan la ausencia explícita de intervalo |
| Fronteras de acceso | Cuatro clientes negativos producen los seis errores de acceso previstos |

[Plan previo](PLAN_PREVIO.md), [casos y esperados](CASOS_ESPERADOS.json) y [compromiso anterior al cambio](COMPROMISO_PREVIO.json). [Resultado principal](evidencia/RESULTADO.json), [observaciones](evidencia/OBSERVACIONES.json), [procesos y salidas](evidencia/PROCESOS.json), [controles complementarios](complemento/RESULTADO.json) y [medidas](MEDIDAS.json).

Las repeticiones no aumentan el número de fuentes distintas. El programa de prueba compara también las API detallada y heredada; las invocaciones internas adicionales no se contabilizan como casos nuevos. El registro de procesos conserva bytes de salida, códigos, comandos, identidad de binarios, tiempo y CPU. La memoria es el máximo acumulado de procesos hijos e incluye el compilador; no es memoria por caso ni el coste incremental del diagnóstico. No se ha constituido una comparación de rendimiento ni medido coste monetario o inferencia.

## Cambio y compatibilidad

La candidata modifica dos archivos de la cápsula RETP-158: [analizador sintáctico](cambios/rust/sv_core/src/frontend.rs) y [diagnósticos](cambios/rust/sv_core/src/compiler_diagnostics.rs). [Comparación incremental](CAMBIO_INCREMENTAL.patch).

El emisor registra la posición que conoce antes de devolver el error. La clasificación de grafía procede del índice léxico existente y del contexto de comprobación. No se interpreta el texto de `Debug` ni el centinela interno para determinar una causa. Las funciones que todavía no aportan esa información mantienen la ausencia de intervalo; no se les asigna por aproximación el último elemento consumido.

`COMPILER-DIAGNOSTICS/2` añade `FrontendCause::ForeignSurface` y `CD.FOREIGN_SURFACE`. Es una sucesión observable de la API diagnóstica experimental: consumidores con correspondencias exhaustivas deberán contemplar la nueva variante. Las API heredadas conservan resultados, errores y precedencia. Los códigos canónicos E004/E115, la IR 0.3 y su proyección permanecen iguales en el corpus comprobado.

La batería original está conservada en [antecedente-158/focal.rs](antecedente-158/focal.rs). La sucesora [focal_heredado_v2.rs](focal_heredado_v2.rs) cambia únicamente los dos esperados extranjeros: de causa genérica y rango ausente a causa específica e intervalo original. Este cambio de contrato se declaró antes de ejecutar la candidata. Los 40 esperados nuevos no se obtuvieron copiando sus salidas.

## Reproducción

Descargue esta carpeta completa o `PAQUETE_REPRODUCIBLE.zip`. Con Linux, Python 3.10 o posterior y rustc con biblioteca estándar nativa:

```sh
python reproducir.py /ruta/absoluta/rustc /ruta/a/directorio/nuevo
```

La reproducción comprueba el manifiesto y las cápsulas SHA-256, recupera 77 archivos por candidata y ejecuta la cualificación y los controles complementarios. No requiere red. Se ha utilizado rustc 1.98.0, destino x86_64-unknown-linux-gnu. El observador conserva los límites de 120 segundos por proceso y 2 MiB por flujo. Python recupera y observa; las decisiones SV se ejecutan en Rust.

## Alcance y continuidad

Cortes de entrada: Lenguaje `03d00131fb0df234101f01c9ca5dfa5f22886ceb`; laboratorio `ce202d9420adb2c6189fce557ad74e2a276c3a0c`, rama `lab/playground-sv-permanente`. Se cotejaron los 49 archivos del antecedente con el árbol publicado. Se consultaron AGENTS, Pilares completos, perfiles y ensamblaje, transición completa y relevos, contrato diagnóstico RETP-109/110, workflow V2 y plan de integración RETP-147. Es un incremento del paso 6; no cierra P6 del workflow ni cambia la secuencia general.

Incluye el despacho inicial y las primitivas `peek_word`, `take_raw_word`, `take_dispatch_word`, `take_word`, `take_text`, `take_nat`, `word`, `sym`, `arrow` y la etiqueta individual de `take_tri`. La modificación del punto de emisión no demuestra por sí sola exhaustividad de todos sus caminos; se conserva el [inventario base](INVENTARIO_EMISORES_BASE.json).

Siguen pendientes los emisores compuestos de campos opcionales y admisibilidad, los validadores restantes, sus subcausas, la presentación y serialización final y DG01–DG14 global. `legacy()` conserva su política de acceso: la conservación del error anterior no lo convierte en texto saneado para publicación. No se acreditan WASI, navegador, recepción profesional ni seguridad material. DFL-001/011 y las demás deudas conservan sus límites. La reserva P3 permanece cerrada y P4/P5 mantienen sus compuertas.

El siguiente objeto es completar el inventario y la migración de los emisores diagnósticos restantes dentro del mismo paso. El encargo común externo continúa pendiente: este paquete expone código, esperados y resultados y permite reproducción, sin constituir una prueba ciega. No se ha ejecutado ningún proveedor externo. No se promueve código al núcleo productivo ni se cambia la visibilidad del laboratorio.

**Estado: CONFORME_EN_ALCANCE_NATIVO; CIERRE_GLOBAL_PENDIENTE.**
'''
(out/'README.md').write_text(readme)
(out/'HALLAZGOS.md').write_text('''# Hallazgos y límites

1. RETP-158 perdía la posición de errores sintácticos distintos de EOF y de caracteres léxicos inválidos. Las primitivas incluidas conservan ahora el intervalo en su punto de rechazo.
2. Las grafías extranjeras de los puntos incluidos compartían causas genéricas. La clasificación léxica y el contexto del emisor permiten distinguirlas sin cambiar admisión ni interpretar el mensaje.
3. La incorporación de una variante pública hace observable la sucesión experimental /1 → /2. Se declara expresamente y se conservan dos versiones de los esperados afectados.
4. Los controles comprueban que una grafía contextual utilizada válidamente como identificador, un literal o un comentario no se convierte en error por su idioma aparente.
5. Los emisores compuestos pendientes conservan rango ausente. Dos controles ES/EN comprueban esa distinción; no se declara cobertura universal.
6. La base sin causa nueva y un mutante con intervalos desplazados terminan con código 101 en el comprobador, por las discrepancias previstas. Son controles de sensibilidad exigidos, conservados por separado; no fallos inesperados de la candidata.

No se observaron fallos inesperados en la cualificación inicial ni en el complemento. Las mediciones instrumentales no acreditan rendimiento relativo ni viabilidad de servicio. La aceptación de este subconjunto no cierra el contrato diagnóstico global.
''')
def manifest():
 entries=[{'ruta':str(p.relative_to(out)),'bytes':p.stat().st_size,'sha256':sha(p.read_bytes())} for p in sorted(out.rglob('*')) if p.is_file() and p.name not in ['MANIFIESTO.json','PAQUETE_REPRODUCIBLE.zip']]
 (out/'MANIFIESTO.json').write_text(json.dumps({'algoritmo':'SHA-256','archivos':entries},ensure_ascii=False,indent=2)+'\n')
manifest()
print(str(out),len(list(out.rglob('*'))),'entradas')
