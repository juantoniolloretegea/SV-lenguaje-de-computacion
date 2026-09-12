from pathlib import Path
import json,hashlib,base64,shutil,zipfile
r=Path(__file__).resolve().parent;out=r/'entrega/ubicaciones-de-rechazos-compuestos';out.mkdir(parents=True,exist_ok=True)
def sha(b):return hashlib.sha256(b).hexdigest()
def copy(src,dst=None):
 p=out/(dst or src);p.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(r/src,p)
for n in ['IDENTIDAD_ENTRADA.json','PLAN_PREVIO.md','COMPROMISO_PREVIO.json','CASOS_ESPERADOS.json','CASOS_NUEVOS.json','CASOS_HEREDADOS_159.json','BASE_FUENTES.json','CAMBIO_INCREMENTAL.patch','ejecutar.py','complemento.py','reproducir.py','focal.rs','focal_heredado_v2.rs','catalogo.rs','CATALOGO.json']:copy(n)
for n in ['preparar.py','modificar.py','preparar_instrumentos.py','empaquetar.py']:copy(n,'soporte/'+n)
for lang in ['es','en']:copy('idiomas/'+lang+'/diagnosticos.json')
for n in ['reproducir.py','focal.rs','corpus.rs','relacional.rs','CATALOGO.json','CORPUS.json','CORPUS_FUENTES.json','catalogo.rs','ESPERADO_RELACIONAL.json']:copy('antecedente-158/'+n)
for p in (r/'antecedente-158/clientes').iterdir():copy(str(p.relative_to(r)))
for folder in ['evidencia','complemento']:
 for p in (r/folder).iterdir():
  if p.is_file() and p.suffix in ['.json','.patch']:copy(str(p.relative_to(r)))
files=[];changes=[]
for p in sorted((r/'candidata').rglob('*')):
 if p.is_file():
  b=p.read_bytes();rel=str(p.relative_to(r/'candidata'));files.append(dict(ruta=rel,bytes=len(b),sha256=sha(b),base64=base64.b64encode(b).decode()))
  if b!=(r/'base'/rel).read_bytes():changes.append(rel)
assert changes==['rust/sv_core/src/frontend.rs'];assert len(files)==77
(out/'CANDIDATA_FUENTES.json').write_text(json.dumps({'archivos':files},ensure_ascii=False,indent=2)+'\n')
copy('candidata/rust/sv_core/src/frontend.rs','cambios/rust/sv_core/src/frontend.rs')
inventory={}
for version in ['base','candidata']:
 p=r/version/'rust/sv_core/src/frontend.rs';mentions=[];fn=None
 for n,line in enumerate(p.read_text().splitlines(),1):
  if line.strip().startswith('fn '):fn=line.strip().split('(')[0][3:]
  if 'FrontendError::' in line:mentions.append(dict(linea=n,funcion=fn,mencion=line.strip()))
 inventory[version]=dict(sha256=sha(p.read_bytes()),menciones=mentions)
inventory['nota']='Inventario estático de menciones, incluidas conversiones y pruebas; no certifica alcanzabilidad ni cobertura universal.'
inventory['pendientes_sin_rango']=['Cardinalidad de states en parse_admissibility_spec: dos controles focales ES/EN.', 'Conversión defensiva String::from_utf8 en tokenize y Nat::from_decimal en square_nat: sin campaña de alcanzabilidad en este incremento.', 'Conversión genérica From<FrontendError> para fallos sin procedencia.']
(out/'INVENTARIO_EMISORES.json').write_text(json.dumps(inventory,ensure_ascii=False,indent=2)+'\n')
logs=json.loads((r/'evidencia/PROCESOS.json').read_text());comp=json.loads((r/'complemento/PROCESOS.json').read_text())
metrics=dict(unidad='Procesos instrumentales, incluida compilación; no por caso',procesos_principales=len(logs),procesos_complementarios=len(comp),tiempo_principal_s=sum(x['duracion_ns'] for x in logs)/1e9,cpu_usuario_principal_s=sum(x['cpu_usuario_s'] for x in logs),cpu_sistema_principal_s=sum(x['cpu_sistema_s'] for x in logs),rss_max_acumulado_hijos_principal_kib=max(x['rss_max_acumulado_hijos_kib'] for x in logs),coste_monetario=None,coste_inferencia=None,comparacion_rendimiento='No constituida; los comprobadores de base y candidata realizan distinta verificación.')
(out/'MEDIDAS.json').write_text(json.dumps(metrics,ensure_ascii=False,indent=2)+'\n')
readme='''# Ubicaciones de rechazos compuestos

**RETP-160 · 12 de septiembre de 2026 · Candidata experimental.**

Doce puntos del analizador conservan ahora la posición original del elemento que causa el rechazo, aunque se hayan leído otros elementos antes de emitirlo. Esto permite señalar el campo repetido o fuera de orden, la variante no admitida o la etiqueta de admisibilidad incorrecta. La causa de grafía ajena al perfil se conserva desde la clasificación léxica existente.

## Referencia y resultado

Este trabajo continúa el [Léame primero del frame](../frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md), el expediente de significado humano y fidelidad, su adenda visual y el documento de trazabilidad, auditoría y reproducción. Conservar la procedencia de un error ayuda a explicar qué ocurrió. La fidelidad exige además comprobar el conocimiento de referencia y lo efectivamente entregado al humano; esta campaña no la da por demostrada.

| Comprobación | Resultado |
|---|---|
| Fuentes nuevas ES/EN | 54: 2 válidas y 52 inválidas |
| Fuentes heredadas RETP-159 | 40, con sus esperados intactos |
| Total focal | 94 fuentes: 10 válidas y 84 inválidas |
| Ensamblajes derivados | 168; ambas posiciones con una unidad sana del otro perfil y nombre de archivo compartido |
| Repeticiones | Tres por modo: 564 observaciones monofuente y 1.008 comprobaciones de ensamblaje |
| Corpus canónico | 14 válidas y 106 inválidas; resultados heredados idénticos entre base y candidata en debug y release |
| Pruebas unitarias | 239/239 en cada modo |
| Focales RETP-158 | 29 en cada modo, mediante la sucesora explícita ya publicada en RETP-159 |
| Catálogo | Las mismas 13 causas y 26 plantillas ES/EN |
| Sensibilidad | Detectadas la ausencia de intervalo en la base y la posición desplazada deliberadamente |
| Fronteras de acceso | Cuatro clientes negativos conservan los seis errores previstos |

Los casos de precedencia comprueban que un error de separador o fin de fuente, emitido antes, mantiene prioridad sobre una variante que aún no se ha validado. Las fuentes incluyen CRLF, tabulación y texto acentuado. Los intervalos se expresan en bytes UTF-8 y se comprueban contra la fuente y su huella SHA-256. Los ensamblajes preservan el índice y perfil de la unidad responsable incluso al invertir el orden. Las repeticiones y los ensamblajes derivados no se cuentan como nuevas fuentes.

[Plan previo](PLAN_PREVIO.md), [compromiso anterior al cambio](COMPROMISO_PREVIO.json), [fuentes nuevas y esperados](CASOS_NUEVOS.json), [fuentes heredadas](CASOS_HEREDADOS_159.json), [resultado](evidencia/RESULTADO.json), [observaciones](evidencia/OBSERVACIONES.json), [procesos y salidas](evidencia/PROCESOS.json), [sensibilidad y catálogo](complemento/RESULTADO.json), [medidas](MEDIDAS.json).

## Cambio y compatibilidad

Se modifica únicamente [frontend.rs](cambios/rust/sv_core/src/frontend.rs) de la cápsula RETP-159. Los otros 76 archivos coinciden byte por byte. [Cambio incremental](CAMBIO_INCREMENTAL.patch) e [inventario antes/después](INVENTARIO_EMISORES.json).

Incluye ocho ramas de los campos opcionales de SemanticRelation y Pattern: repetición, orden, restricciones repetidas y campo desconocido. Incluye también la variante de supervisión, el contexto de consulta, la palabra protegida en el despacho de let y la etiqueta individual de admisibilidad. El emisor guarda el índice al leer el elemento; la validación permanece en su lugar original. No se interpreta la prosa del error ni un centinela para clasificarlo.

Es una extensión observable de cobertura de `COMPILER-DIAGNOSTICS/2`, sin cambios de variantes públicas ni mensajes. Los emisores incluidos pasan de intervalo ausente a intervalo original; los rechazos de grafía constitutiva extranjera pasan a `CD.FOREIGN_SURFACE`. Los resultados de la API heredada, el orden de rechazo, los códigos SV y la IR se conservan en las comprobaciones ejecutadas. Los antiguos controles de RETP-159 que exigían ausencia de intervalo en campos opcionales quedan sucedidos por esta cobertura; su expediente histórico permanece intacto. Los 40 esperados focales de RETP-159 no se modifican.

## Reproducción

Descargue esta carpeta completa o [PAQUETE_REPRODUCIBLE.zip](PAQUETE_REPRODUCIBLE.zip). Con Linux, Python 3.10 o posterior y rustc con biblioteca estándar nativa:

```sh
python reproducir.py /ruta/absoluta/rustc /ruta/a/directorio/nuevo
```

La reproducción valida el manifiesto SHA-256, recupera los 77 archivos por versión y ejecuta la cualificación y sus controles sin red. Se utilizó rustc 1.98.0 para x86_64-unknown-linux-gnu. El compilador y las decisiones SV se ejecutan en Rust; Python recupera y observa. Los instrumentos preservan 120 segundos por proceso y 2 MiB por flujo.

La cualificación inicial comprende 41 procesos principales y 7 complementarios, con comandos, binarios identificados, códigos y bytes de salida conservados. Los dos fallos de sensibilidad son los rechazos previstos del comprobador y están separados de los resultados de la candidata. No hubo fallos inesperados. Tiempo, CPU y máximo acumulado de memoria de hijos incluyen compilación: no acreditan consumo por caso, mejora de rendimiento, coste incremental, dinero ni inferencia.

## Límites y relevo

Cortes de entrada: Lenguaje `76722442d2899300c69d89ab932890a4d7d74c2f`; laboratorio `afcbd5046b2eec24cb058ce0fedcf580ca4790e8`, rama `lab/playground-sv-permanente`. Se verificó el expediente RETP-159 completo en ambos árboles. Se cotejaron las rectoras leídas: AGENTS, Pilares completos, perfiles y ensamblaje, transición completa y relevos; se consultaron en esta continuidad el contrato diagnóstico RETP-109/110, workflow V2 y plan RETP-147.

La cardinalidad incorrecta de los estados de admisibilidad mantiene rango ausente, comprobado en ambos perfiles: el error afecta al conjunto y no se atribuye artificialmente a una etiqueta. También permanecen sin campaña las conversiones defensivas de texto UTF-8 y del cuadrado natural señaladas en el inventario. Los demás validadores, sus subcausas, la serialización y la presentación final siguen pendientes; este incremento no demuestra cobertura global de todos los caminos.

El siguiente objeto acotado es constituir y comprobar la ubicación del conjunto de estados cuando falla su cardinalidad, incluida la precedencia y la distinción entre etiqueta y conjunto. Después siguen los emisores y validadores pendientes del mismo paso 6. El avance no cierra P6 del workflow ni DG01–DG14 global. DFL-001/011 y las restantes deudas siguen abiertas; la fila 9 y las prioridades del relevo vigente se conservan. La reserva P3 permanece cerrada; P4/P5 mantienen sus compuertas.

El código productivo sigue sin promover esta candidata. No se acreditan WASI, navegador, recepción profesional ni ejecuciones de proveedores externos. El encargo externo común continúa pendiente; el paquete ofrece reproducción pública y no constituye una prueba ciega. La visibilidad del laboratorio se conserva.

**Estado: CONFORME_EN_ALCANCE_NATIVO; CIERRE_GLOBAL_PENDIENTE.**
'''
(out/'README.md').write_text(readme)
entries=[dict(ruta=str(p.relative_to(out)),bytes=p.stat().st_size,sha256=sha(p.read_bytes())) for p in sorted(out.rglob('*')) if p.is_file() and p.name not in ['MANIFIESTO.json','PAQUETE_REPRODUCIBLE.zip']]
(out/'MANIFIESTO.json').write_text(json.dumps(dict(algoritmo='SHA-256',archivos=entries),ensure_ascii=False,indent=2)+'\n')
with zipfile.ZipFile(out/'PAQUETE_REPRODUCIBLE.zip','w',zipfile.ZIP_DEFLATED,compresslevel=9) as z:
 for p in sorted(out.rglob('*')):
  if p.is_file() and p.name!='PAQUETE_REPRODUCIBLE.zip':z.write(p,str(p.relative_to(out)))
print(len(entries),'archivos más manifiesto y ZIP')
