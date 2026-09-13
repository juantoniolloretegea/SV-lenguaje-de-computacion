from pathlib import Path
import json,hashlib,shutil,subprocess
R=Path('manifiesto-sv/checkout');B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';E=B/'bis04-extension-perfiles-recursos-v0_1';D=B/'bis04-realizacion-i0205-v0_1'
def dump(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
(E/'soporte').mkdir(exist_ok=True)
for name in ('preparar.py','conductor.py','reproducir.py','fijar.py'):shutil.copyfile(Path('bis-extension')/name,E/'soporte'/name)
base=subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip();assert base=='aa1229788f0a6625be43c74e29c38023b8193a4c'
sources=json.loads((B/'bis03-sedes-i0205-v0_1/FUENTES.json').read_text())['archivos']
for p,v in sources.items():assert hashlib.sha256((R/p).read_bytes()).hexdigest()==v['sha256'],p
sources.update({str(p.relative_to(R)):{'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in (D/'MANIFIESTO.json',B/'bis-c12/CONTRATO_PERFILES_DOCUMENTACION_CONSTRUCCION_v0_1.md',R/'ADENDA_NORMATIVA_PERFIL_LEXICO_GRAMATICA_SVP_0_2_2026_08_27.md',R/'ESPECIFICACION_NORMATIVA_PERFILES_FUENTE_SVP_ES_EN_v1_2026_08_29.md')})
dump(E/'FUENTES.json',{'corte_publico':base,'corte_laboratorio':'ed5a78fb667401b052e915db38bf17465fe3b2b2','archivos':sources,'lectura':'Pilares, acta de perfiles/ensamblaje, transición secuencial y workflow V2 consultados; hashes de lecturas anteriores revalidados; contrato C12 y normas de perfiles/lexemas leídos.'})
(E/'README.md').write_text('''# Extensión experimental: perfiles, recursos y captura

Estado inicial: banco previo, pendiente de ejecución nativa. Continuación de RETP-220; no cierre global del Bis.

## Alcance fijado antes de ejecutar

26 casos nuevos: 8 de selección/perfil ES/EN, 12 de recursos y 6 de captura. Cinco comparaciones adicionales de IR canónica: dos sobre fuentes integradas y tres sobre fixtures C12 con una operación `evaluate`/`evaluar`. Se exige igualdad de todos los objetos y operaciones en cuatro pares y desigualdad al cambiar un dato textual en el quinto. No se traduce identificadores, datos, comentarios ni nombres de archivo. La procedencia literal (archivo y SHA-256) se comprueba separadamente y debe conservarse.

Los esperados están en [BANCO_PREVIO.json](BANCO_PREVIO.json), [PARIDAD_PREVIA.json](PARIDAD_PREVIA.json) y los `oracle.json` materializados. Todos mantienen `PENDIENTE`/null como instantánea previa; la evidencia irá en archivos separados. El anclaje matemático positivo es el vector y descriptor literal comprometidos en I0205, no una salida generada por el sujeto bajo prueba. La comparación canónica usa Rust `PartialEq` sobre todos los objetos y operaciones, con control de no vacuidad para los fixtures que incluyen evaluación.

Se ensayan límites exactos y excesos de recepción: metadatos, fuente, estado, soporte y total agregado; y recibo de salida 2048/2049 bytes. Los contadores observan bytes realmente devueltos por cada canal, incluido el byte detector de exceso. R01 puede rechazar al recibir (traza vacía) o al acotar el recibo antes de entregar (12 guardas superadas): cada traza se fija expresamente. Se alteran consumidor, canal, transformación, operación y bytes efectivamente recibidos, manteniendo la solicitud admitida.

## Custodia y reproducibilidad

El crate `sv_bis_extension` conduce el experimento y observa su evidencia. [ADMISION_VERIFICADA.tar.gz](ADMISION_VERIFICADA.tar.gz) conserva exactamente el crate de RETP-220, y `sv_core-verificado.tar.gz` conserva su núcleo. No se modifica ninguno. Cada caso contiene su propio registro y contexto de custodia, derivados explícitamente del banco anterior; la solicitud no elige esas autoridades. Los hashes y longitudes de fuentes con espacios o comentario añadido se actualizan en todos sus vínculos antes del ensayo. Variar el cuerpo de soporte genera una nueva referencia exacta por hash, sin ampliar sus tamaños admitidos.

Python prepara, archiva y lanza procesos; no decide admisión ni equivalencia SVP. No hay servicios externos en la ejecución. Herramientas previstas: Cargo/rustc 1.98.0, compilación `--locked --offline`, binario Linux; medición del PID mediante wait4 y reloj monotónico. Código completo en `proyecto/` y administración en `soporte/`. Núcleo y admisión se verifican por SHA-256 antes de compilar.

```sh
. /opt/sv-rust-1.98.0/env.sh
python soporte/reproducir.py /ruta/a/directorio-nuevo
```

## Límites y continuidad

Ejemplos y evidencia acotada de laboratorio. No acreditan garantías generales, consumo visual de IA, bus/host productivo, autoridad externa autenticada, constitución de dominio ni suficiencia total C12. La paridad canónica separa procedencia; no exige igualdad de recibos con perfiles/hashes diferentes. No se ejecutan aquí las 202 filas originales C02–C12 como banco propio ni se alteran sus recuentos. Quedan pendientes límites geométricos/salida agregada, agotamiento de memoria, presupuestos de tiempo/RAM constituidos y el resto del workflow. Bis → catálogo y cierre de fase → análisis e instalación de GUI.

[Fuentes rectoras y corte](FUENTES.json). Los tres rectores de AGENTS.md y workflow V2 mantienen su identidad. Esta extensión no constituye nuevas reglas del lenguaje.
''')
dump(E/'PRECOMPROMISO.json',{str(p.relative_to(E)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(E.rglob('*')) if p.is_file() and p.name!='PRECOMPROMISO.json'})
print('Precompromiso preparado. Sin ejecutar sujeto.')
