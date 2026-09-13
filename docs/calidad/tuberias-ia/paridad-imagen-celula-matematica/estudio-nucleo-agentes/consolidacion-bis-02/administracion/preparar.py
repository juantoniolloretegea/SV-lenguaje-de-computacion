from pathlib import Path
import sys,json,hashlib,subprocess
sys.path.insert(0,'bis-c02');from registro import R,B,L,rows,guardar,retp,now
HEAD='4fc7a2ceb2ece6d5d69d12e3216eda48e68f6136';LAB='5dc375e7d7af3e1110bb57a9b4e7449eb817dac6'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip()==HEAD
assert not subprocess.check_output(['git','status','--porcelain'],cwd=R,text=True)
D=B/'consolidacion-bis-02';D.mkdir();(D/'administracion').mkdir()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,v):p.write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n')
rectors=json.loads((B/'bis-c11/FUENTES.json').read_text())['fuentes'][:3]
for f in rectors:assert sha(R/f['ruta'])==f['sha256']
original=json.loads((B/'BANCO_PREVIO_BIS_02_v0_1.json').read_text())
limits=[
'Geometría, longitud y orden por sv-native; no soporte, dominio, evaluación ni imagen.',
'Selección de soporte y su imposición pendientes; perfiles {16,25} y {16,25,49} sólo sintéticos.',
'LIG/0.1 existente; enlace específico constitución/instancia/revisión/representación pendiente.',
'Geometría simbólica exacta; sin dibujo, rasterización o consumo acreditados.',
'Descriptores y expectativa de entrega; faltan artefacto visual, instrumentación y uso real.',
'Contratos de lectura, inicialización y reevaluación; faltan productores y custodia entre instantáneas.',
'Fuentes de serie/compuerta y oráculos; no producción de salida, transmisión ni supervisión ejecutadas.',
'Fuentes parciales; falta montaje de representación, inyección técnica y captura independiente.',
'Documentos sintéticos; sin receptor nuevo, modelo interrogado o efecto ensayado.',
'Evidencia sintética; sin receptor completo, justificación formal productiva ni modelo ensayado.',
'Cuotas sintéticas; sin medida de memoria/tiempo ni contención global; C11-13 tiene dos subcasos.',
'Fuentes ES/EN, privacidad y documentación; faltan ensayos propios y entradas adicionales en varios casos.'
]
families=[];sources=[]
for n in range(1,13):
 f=f'C{n:02}';folder=B/f'bis-c{n:02}';bank=folder/('BANCO_COMPROMETIDO.json' if n==1 else 'BANCO_PREVIO_v0_1.json');j=json.loads(bank.read_text());cs=j['casos'];pairs=[x for x in original['casos'] if x['id'].startswith('BIS-'+f+'-')];assert len(pairs)==2
 contract=folder/'README.md' if n==1 else next(folder.glob('CONTRATO*md'))
 families.append({'familia':f,'escenarios_originales':[x['id'] for x in pairs],'obligaciones':pairs[0]['obligaciones'],'contrato':str(contract.relative_to(B)),'banco':str(bank.relative_to(B)),'banco_sha256':sha(bank),'ids_de_fila':[c['id'] for c in cs],'filas':len(cs),'ejecuciones_historicas':13 if n==1 else 0,'ejecuciones_nuevas':0,'estado':'EVIDENCIA_NATIVA_HISTORICA_ACOTADA' if n==1 else 'PREPARADO_NO_EJECUTADO','alcance_y_pendiente':limits[n-1]})
 for p in set([bank,contract,folder/'README.md']):sources.append({'ruta':str(p.relative_to(R)),'sha256':sha(p),'lectura':'Banco e índice de familia; contrato completo C02/C03/C04/C07/C12; finalidad y criterios de salida de los restantes. C01: informe y resultados.'})
assert sum(x['filas'] for x in families[1:])==202
save(D/'INVENTARIO_FAMILIAS.json',{'version':'BIS-02-CONSOLIDACION/1','corte':HEAD,'familias':families,'conteos':{'familias':12,'escenarios_originales':24,'originales_ejecutados':2,'originales_pendientes':22,'variantes_nativas_historicas_c01':13,'filas_preparadas_c02_c12':202,'ejecuciones_nuevas':0},'nota':'Filas documentales heterogéneas, con subcasos, expectativas y entradas parcialmente materializadas. No se suman como pruebas ejecutadas ni se calcula porcentaje de seguridad.'})
# Propuestas de examen, no decisiones canónicas ni habilitaciones operativas.
seats=[
('Núcleo: conservar validación de Σ, b, N y posiciones; adaptadores: exigir ruta pública.','C01 acredita su ruta nativa; falta comprobar todas las vías afectadas.','Reutilizar evidencia exacta; ensayar rutas adicionales cuando la realización las incluya.'),
('Dominio constituye tamaños; soporte declara versiones; frontera impone pertenencia sobre IR validada.','Contrato C02 preparado; N fijo no equivale a pertenencia o autorización.','Fijar manifiesto tipado y punto obligatorio; comparar contenedores sólo con tamaños y coste definidos.'),
('Reutilizar LIG/0.1; contrastar enlace complementario de instancia/revisión y consumidor.','No duplicar ligaduras ni identificar por vector igual; falta imposición del vínculo específico.','Fijar expectativa independiente, estado exacto y versión; probar sustitución de identidad.'),
('Representación derivada subordinada al álgebra; frontera enlaza estado/convenio/artefacto.','Frame arquitectónico no acredita pareja celular; descriptor no prueba dibujo.','Decidir descriptor frente a artefacto, relación tipada y condiciones de precisión/recorte.'),
('Núcleo y cada entrada de frontera: distinguir datos, validación y objeto admitido.','Encapsulación y guardas locales presentes; no prueban toda nueva vía.','Inventariar construcción/importación/mutación y verificar que no hay ruta alternativa sin control.'),
('Núcleo para leyes constituidas; biblioteca para operaciones derivadas; custodia y adaptadores por contrato.','Bienformación no produce Evaluate/gate/reevaluación; deudas de productores permanecen.','Delimitar operación por operación: entrada, codominio, rol, dirección, productor y vínculo entre instantáneas.'),
('Tipos y validación nuclear; frontera conserva resultado técnico y procedencia.','Tri.U válido permanece separado de fallo, Bottom, rechazo e imagen pendiente.','Definir retornos diferenciados e inyecciones que alcancen cada guarda sin fabricar U.'),
('Adaptador de entrega, receptor real y observador independiente.','Disponibilidad, entrega y consumo son hechos distintos; capacidad visual no acreditada por nombre.','Instrumentar artefacto exacto, canal e invocación; realizar prueba efectiva de capacidad cuando proceda.'),
('Reutilizar envolventes R1; frontera de documentos y efecto con autoridad previamente constituida.','Documento no concede permiso; rechazo verbal del modelo no prueba bloqueo del efecto.','Separar receptor determinista, conducta de modelo y contención; conservar despacho y efecto observado.'),
('Soporte, entrada y supervisor: cuotas previas, agregación, concurrencia y cancelación.','Cuotas locales no acotan proceso completo; magnitudes globales pendientes.','Fijar presupuesto medible por montaje y contador independiente, con respuesta real al exceso.'),
('Perfil fuente en frontend; documentación ES/EN del Rust afectado; localización en presentación.','Política existente y entradas C12; equivalencia completa aún no ensayada.','Comparar contenido canónico y procedencia separadamente; revisión de prosa y doctests con evidencias distintas.'),
('Cadena de evidencia, consumidor y registros; catálogo receptor al término de BIS-08.','Trazabilidad documental disponible; no acredita por sí sola QueryResult ni CQ1–CQ6.','Vincular obligación/caso/realización/observación/causa y conservar precedencia; no acuñar códigos ahora.')
]
mat=json.loads((B/'MATRIZ_BIS_01.json').read_text());matrix=[]
for i,x in enumerate(mat['filas']):
 fs=[f for f in families if x['id'] in f['obligaciones']]
 matrix.append({'obligacion':x['id'],'denominacion':x['obligacion'],'familias':[f['familia'] for f in fs],'contratos':[f['contrato'] for f in fs],'cobertura_documental':'CARTOGRAFIADA','acreditacion_integral':False,'sede_propuesta_a_contrastar':seats[i][0],'material_y_limite':seats[i][1],'decision_o_evidencia_pendiente':seats[i][2],'estatuto_sede':'PROPUESTA_PARA_BIS_03_NO_DECISION'})
save(D/'MATRIZ_COBERTURA_Y_SEDES.json',{'version':'BIS-02-COBERTURA/1','corte':HEAD,'filas':matrix,'criterio':'Los enlaces proceden de los pares del banco original; cobertura documental no equivale a suficiencia. O03 incorpora la precisión posterior LIG/0.1. No se reescribe la radiografía histórica.'})
gates=[
('G01','Contratos y cobertura C01–C12','CONFORME_DOCUMENTAL','Doce familias y doce obligaciones relacionadas con fuentes y criterios; no doce garantías.'),
('G02','Recorrido e integración de la realización escogida','PENDIENTE','Falta comprometer un montaje común que enlace soporte, identidad, representación y entrega; los bancos aislados no prueban compatibilidad entre contratos.'),
('G03','Datos de prueba y oráculos del montaje escogido','PARCIAL','Hay entradas y oráculos en familias; C06 y otros casos requieren materialización. Todo estímulo exacto y esperado debe quedar fijado antes de ejecutar su prueba.'),
('G04','Tamaños y presupuesto competentes','PENDIENTE','Los perfiles y cuotas sintéticos no constituyen soporte productivo. Magnitudes no constituidas conservan su ausencia y no habilitan uso.'),
('G05','Recepción, observador y sensibilidad','PENDIENTE','Fijar entrada real, captura independiente, precedencia y mutantes discriminantes; identidad de oráculo distinta del sujeto.'),
('G06','Entorno de compilación de la futura realización','NO_DISPONIBLE_EN_COMPROBACION','Checkout sincronizado. rustc/cargo no se resolvieron en PATH ni en /root/.cargo/bin; búsqueda en ubicaciones examinadas sin resultado. No invalida evidencias históricas S23/C01.'),
('G07','Conservación documental y secuencia','CONFORME_DOCUMENTAL','Historial y bancos anteriores intactos; S22 y BIS-02 abiertos, BIS-03 pendiente, S24 pendiente.')
]
save(D/'CONDICIONES_DE_PASO.json',{'version':'BIS-02-PASO/1','criterio_rector':'CONTRATO_CANDIDATO_BIS_02_v0_1.md: contratos necesarios para realización escogida y banco comprometido correspondiente; workflow §3 y §12.','condiciones':[dict(id=a,objeto=b,estado=c,evidencia_o_pendiente=d) for a,b,c,d in gates],'cierre_bis02':False,'apertura_formal_bis03':False,'trabajo_habilitado':'Preparar contrato de integración y compromiso del primer recorrido C02–C05, incorporando guardas transversales; precisar propuestas de sede sin presentarlas como implementadas.','regla':'No se exige ejecutar todas las pruebas para preparar sedes; sí fijar los contratos, entradas y oráculos requeridos antes de cada realización y ejecución. Las pruebas globales continúan en BIS-04/05/06.'})
report='''# Consolidación de cobertura y pendientes de BIS-02

**Versión 1 · 13 de septiembre de 2026 · S22 · RETP-2026-216 · Juan Antonio Lloret Egea y Watson**

## 1. Dictamen

La consolidación de las doce familias C01–C12 queda terminada en alcance documental. BIS-02 continúa en ejecución y BIS-03 permanece pendiente: el contrato general exige los contratos de la realización escogida y su banco comprometido. La existencia de doce carpetas no satisface por sí sola esa condición.

Corte leído del Lenguaje: `'''+HEAD+'''`; laboratorio: `'''+LAB+'''`. Se reutilizan las lecturas rectoras completas de Pilares, perfiles/contratos/ensamblaje y transición secuencial con sus adendas, tras cotejar sus SHA-256 sin cambios. Se han revisado workflow V2, contrato general, matriz histórica, bancos e índices de todas las familias y criterios de salida pertinentes. El manifiesto declara el alcance de lectura por pieza.

## 2. Inventario y significado de los conteos

| Familia | Filas del banco | Ejecuciones históricas | Objeto y límite principal |
| --- | ---: | ---: | --- |
'''
for f in families:report+=f"| [{f['familia']}](../bis-{f['familia'].lower()}/README.md) | {f['filas']} | {f['ejecuciones_historicas']} | {f['alcance_y_pendiente']} |\n"
report+='''
C01 conserva trece variantes nativas conformes y cuatro alteraciones de sensibilidad del observador, en su corte histórico. Las cuatro alteraciones no son otras cuatro compilaciones SVP. C02–C12 contienen 202 filas preparadas sin ejecución funcional. Son especificaciones heterogéneas: algunas tienen entradas literales, otras describen montajes por concretar y C11-13 incluye dos subcasos. No se suman como 215 pruebas de seguridad ni se calcula una tasa de garantía.

El banco original mantiene 24 escenarios: dos ejecutados en el alcance de C01 y veintidós pendientes. Su texto histórico no se reescribe; los estados y resultados posteriores se conservan en los expedientes y en el registro vigente. Este incremento no ejecuta nuevamente C01 ni modifica el estatuto de los demás bancos.

## 3. Cobertura y sedes propuestas

La matriz enlaza cada obligación con sus familias desde los pares del banco original. Las doce obligaciones tienen cobertura documental; ninguna fila se presenta como acreditación integral. Una misma familia puede cubrir varias obligaciones y una obligación puede requerir varios montajes. No se deduce cobertura por contar archivos.

| Obligación | Familias | Sede propuesta y cuestión pendiente |
| --- | --- | --- |
'''
for m in matrix:report+=f"| {m['obligacion']} | {', '.join(m['familias'])} | {m['sede_propuesta_a_contrastar']} {m['decision_o_evidencia_pendiente']} |\n"
report+='''
Las sedes son propuestas para el examen de BIS-03. No constituyen nuevos tipos, firmas, macros, primitivas o códigos de error. LIG/0.1 ya existe y debe reutilizarse en su alcance; no se generaliza su ausencia desde una lectura parcial de IrProgram. Compartir especificación o vector no fusiona instancias. Frame de arquitectura conserva su contrato; el vínculo de la representación celular sigue por precisar.

## 4. Pendientes que condicionan el paso

'''
for a,b,c,d in gates:report+=f"- **{a} · {b}: {c}.** {d}\n"
report+='''
Los contratos individuales fijan precedencias locales. Componerlos exige decidir cómo se enlazan y qué guarda se alcanza primero; no basta concatenar verificadores o aceptar su salida por separado. La expectativa y el oráculo pertenecen al conductor comprometido. El contenido recibido no elige su autoridad, su versión ni la regla con la que se valida.

La ausencia de ejecución no impide preparar decisiones de sede: BIS-04 realiza y BIS-05/06 contrastan recorrido, falsación y coste. Lo que falta para el cierre contractual es concretar el montaje escogido y sus obligaciones de integración, no fingir que ya se han ejecutado todas sus pruebas. Una operación que dependa de una condición pendiente continúa sin habilitar.

## 5. Próximo objeto material

Se prepara como primer recorrido común **recepción de soporte → vínculo de instancia y revisión → representación matemática exacta → entrega de artefacto identificado**, apoyado en C02–C05. Antes de implementarlo se comprometerán una solicitud exacta, registro de soporte y constitución sintéticos custodiados, fuente SVP, selección explícita de perfil, vínculo esperado, transformación, artefacto de salida y recibo observado. Se fijarán rechazos cruzados que detecten versión sustituida, instancia/revisión intercambiada, permutación posicional, transformación distinta y entrega de otro artefacto.

El descriptor exacto es el primer objeto de este recorrido propuesto. Su admisión no cerrará la producción de imagen ni su consumo por una IA. El contrato gráfico material debe fijar precisión y condiciones de presentación antes de esa prueba. C08, C09, C10, C11 y C12 aportan respectivamente estatutos de fallo, autoridad, respaldo, presupuesto e idiomas/construcción. C06 y C07 conservan sus exigencias de productores, composición y continuidad; no se declaran resueltas por la ruta de entrega.

El siguiente incremento deberá convertir esta propuesta en contrato de integración y banco común comprometido, precisar qué controles existentes se reutilizan y aportar el caso discriminante si se pide ampliar IR o núcleo. Se conserva el contraste no médico competente antes de afirmar generalidad. Los ejemplos históricos de inmunología y neumología no se incorporan al dominio vigente.

## 6. Recuperación y entorno

Se recuperó acceso al entorno y se sincronizó el checkout mediante avance rápido desde su copia recuperada hasta el corte C12. El árbol quedó limpio antes de esta consolidación. Esta comprobación sucede al pendiente documentado en C12; no reescribe su historia.

La comprobación de herramientas no encontró cargo ni rustc en PATH ni en la ruta anterior /root/.cargo/bin; la búsqueda acotada tampoco los localizó en /opt, /usr/local, /workspace y /root. Se declara indisponibilidad en este entorno comprobado, sin negar su existencia en otras instalaciones o sus ejecuciones históricas. Antes de ensayar la realización se deberá recuperar y verificar el compilador y sus dependencias. La instalación previa S23 no garantiza persistencia del entorno temporal.

Este incremento usa Python para inventario, huellas y registros, no para compilar ni interpretar SVP. Conserva scripts, comandos, código de publicación, resultados y referencias. No hay nuevas pruebas Rust, de visión o de conducta de modelos. La compilación del código, el rechazo de datos, la detección funcional y la revisión de prosa mantienen evidencias distintas.

## 7. Continuidad

El suceso es el hecho; prosa, matemática e imagen son representaciones. El hecho aquí acreditado es la consolidación documental y su cotejo. No se cierran K1-T, DFL-003/004/005/006, CQ1–CQ6 o la ejecución de productores por añadir una matriz. E003 mantiene su estatuto documentado y no se convierte en guarda ejecutada.

S22 permanece en ejecución. BIS-02 sigue abierto; BIS-03 y etapas posteriores conservan sus estados. El catálogo recibirá causas comprobadas conforme a BIS-08. S24 continúa pendiente: Bis → catálogo y cierre de fase → análisis e instalación de la GUI.
'''
(D/'README.md').write_text(report+'\n[Inventario por familia y caso](INVENTARIO_FAMILIAS.json) · [Matriz de cobertura y sedes](MATRIZ_COBERTURA_Y_SEDES.json) · [Condiciones de paso](CONDICIONES_DE_PASO.json) · [Fuentes](FUENTES.json) · [Verificación](VERIFICACION.json) · [Administración](administracion/README.md).\n')
for p in [B/'WORKFLOW_P1_P3_BIS_v2.md',B/'CONTRATO_CANDIDATO_BIS_02_v0_1.md',B/'BANCO_PREVIO_BIS_02_v0_1.json',B/'MATRIZ_BIS_01.json',B/'bis-c01/evidencia/ejecucion-1/RESULTADO.json',B/'bis-c06/MATRIZ_DE_SEDES_Y_LIMITES.json',B/'bis-c03/ALCANCE_LIG_Y_RECTIFICACION_DE_LECTURA.md']:
 sources.append({'ruta':str(p.relative_to(R)),'sha256':sha(p),'lectura':'Revisión documental de continuidad y condiciones de salida; sin reejecución funcional.'})
save(D/'FUENTES.json',{'corte_lenguaje':HEAD,'corte_laboratorio':LAB,'fuentes':rectors+sources,'alcance':'Consolidación de contratos y evidencias ya identificadas; no nueva inspección integral de todo el Rust.'})
s=json.loads((B/'ESTADO_WORKFLOW.json').read_text());s.update(ultimo_informe='consolidacion-bis-02/README.md',ultimo_retp='RETP-2026-216',siguiente_accion='Comprometer contrato de integración y banco común del primer recorrido C02–C05, con guardas transversales y propuestas de sede; recuperar toolchain antes de ensayos Rust.',ampliacion='Consolidación C01–C12: doce obligaciones cartografiadas; 13 variantes nativas históricas C01 y 202 filas C02–C12 sin ejecutar. BIS-02 abierto por condiciones de integración pendientes.')
s['consolidacion_bis02']={'estado':'finalizado en alcance documental','informe':'consolidacion-bis-02/README.md','cierre_bis02':False,'familias':12,'filas_previas_c02_c12':202,'ejecuciones_nuevas':0,'sincronizacion_checkout':'recuperada y verificada al corte '+HEAD,'toolchain_actual':'cargo/rustc no localizados en PATH ni ubicaciones examinadas; recuperación pendiente antes de ejecución'}
s['preparacion_bis_c12']['sincronizacion_local']='resuelta en RETP-2026-216; expediente histórico C12 conservado'
save(B/'ESTADO_WORKFLOW.json',s)
p=B/'README.md';t=p.read_text().replace('## Trabajo vigente: S22 · RETP-2026-215','## Trabajo vigente: S22 · RETP-2026-216');mark='**Workflow V2:';a=t.index(mark);t=t[:a]+'[Consolidación C01–C12](consolidacion-bis-02/README.md): cobertura, pendientes y propuestas de sede. Trece variantes nativas históricas C01; 202 filas C02–C12 sin ejecución. BIS-02 continúa abierto. Siguiente: contrato de integración y banco común C02–C05.\n\n'+t[a:];p.write_text(t)
t=L.read_text().replace('**Trabajo vigente: S22 / RETP-2026-215.','**Trabajo vigente: S22 / RETP-2026-216.');a=t.index('**Siguiente objeto material:**');z=t.index('\n\n',a);t=t[:a]+'''**Siguiente objeto material:** comprometer contrato de integración y banco común del recorrido C02–C05, incorporando estatutos, autoridad, evidencia, recursos y perfiles. La [consolidación C01–C12](../paridad-imagen-celula-matematica/estudio-nucleo-agentes/consolidacion-bis-02/README.md) fija doce obligaciones y propuestas de sede; no cierra BIS-02 ni abre formalmente BIS-03. C01 conserva trece variantes nativas históricas; C02–C12 suman 202 filas documentales sin ejecutar. Los 24 originales mantienen dos ejecutados y veintidós pendientes. Checkout sincronizado; cargo/rustc no localizados en el entorno recuperado: verificar y recuperar herramientas antes de ensayar Rust. S24 conserva la GUI diferida.'''+t[z:];L.write_text(t)
f,rs=rows();r=next(x for x in rs if x['id']=='S22');tm=now();r.update(fecha_actualizacion_utc=tm,cortes_de_entrada='Lenguaje '+HEAD+'; laboratorio '+LAB,resultado='Consolidación documental C01–C12: doce obligaciones cartografiadas, inventario por caso, límites y propuestas de sede. C01 conserva 13 variantes nativas históricas; C02–C12 contienen 202 filas sin ejecución.',verificacion='Cotejo Python de inventario, huellas, mapeo original y conservación del historial; ninguna nueva ejecución Rust o IA. Checkout recuperado y sincronizado. cargo/rustc no localizados en PATH ni ubicaciones examinadas.',evidencias='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/'+str((D/'README.md').relative_to(R)),referencia_calidad='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-216',siguiente_accion=s['siguiente_accion'],observaciones='Consolidación terminada sólo en alcance documental. BIS-02 abierto; BIS-03 pendiente; 2 escenarios originales ejecutados y 22 pendientes. S24 sin cambios. Sin modificación de Rust, IR, semántica, dominios o catálogo.')
guardar(f,rs,r);retp(216,'S22 · Consolidación de cobertura C01–C12 y preparación de sedes',tm,r,'tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/consolidacion-bis-02/README.md','CONSOLIDACION_DOCUMENTAL','Workflow V2; contrato general BIS-02; doce bancos; precisión LIG/0.1','Trazabilidad y pendientes explícitos; integración común por comprometer')
print('Consolidación preparada: 12 familias; 202 filas previas; cero ejecuciones nuevas.')
