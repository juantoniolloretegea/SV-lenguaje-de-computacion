from pathlib import Path
import sys,json,hashlib,subprocess
sys.path.insert(0,'bis-c02');from registro import R,B,L,rows,guardar,retp,now
HEAD='b22d03cf369bbe9cab902846d061d5de46f76397';LAB='9da7f6aba6ff72c9a73cd12d24acda4e8e5cc659'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip()==HEAD
assert not subprocess.check_output(['git','status','--porcelain'],cwd=R,text=True)
D=B/'bis-c11';D.mkdir();(D/'entradas').mkdir();(D/'administracion').mkdir()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n')
rectors=json.loads((B/'bis-c10/FUENTES.json').read_text())['fuentes'][:3]
for f in rectors:assert sha(R/f['ruta'])==f['sha256']
texts={'ascii-4096.txt':'a'*4096,'ascii-4097.txt':'a'*4097,'utf8-4096.txt':'á'*2048,'utf8-4097.txt':'á'*2048+'a'}
for n,t in texts.items():(D/'entradas'/n).write_bytes(t.encode('utf-8'))
policy={'version':'BIS-C11-PRESUPUESTO/0.1','estatuto':'Presupuesto sintético de prueba, no máximo universal SV ni perfil de soporte aprobado','unidad_operativa':'peticion_sintetica','limites':{'texto_por_pieza_bytes_utf8':4096,'texto_total_por_peticion_bytes_utf8':8192,'imagen_pixeles_decodificados':65536,'imagen_salida_rgba_bytes':262144,'nodos_composicion':8,'profundidad_composicion_nodos':4,'peticiones_simultaneas':2,'salida_por_peticion_bytes':8192,'llamadas_herramienta_por_peticion':3},'reglas':['Todos los límites son inclusivos; la primera unidad superior excede el presupuesto.','Texto se cuenta por bytes UTF-8 efectivos; no por caracteres, tokens ni tamaño de archivo comprimido.','La suma total de texto incluye todas las piezas recibidas y repetidas; deduplicación semántica no devuelve consumo ya realizado.','RGBA designa exclusivamente cuatro bytes por píxel de salida de este testigo; excluye temporales, stride, decodificador y memoria del proceso.','Profundidad cuenta nodos de la ruta más larga; una raíz aislada vale uno. Un ciclo no admite profundidad finita en este testigo DAG.','Concurrencia cuenta peticiones admitidas aún no finalizadas; reserva y liberación deben ser coherentes frente a carreras.','La cuota de llamadas cuenta intentos despachados, incluidos fallidos y reintentos; no se renueva automáticamente al fallar.','Exceso no permite truncar texto, reducir dimensión de célula o presentar imagen incompleta como fiel.'],'memoria_proceso_max_bytes':None,'tiempo_maximo_ms':None,'motivo_limites_pendientes':'Requieren entorno, medición basal, régimen de interrupción y sede material; no se deducen del tamaño de carga.','tipos_enteros_a_contrastar':['u32','u64','usize_del_destino'],'nota_temporal':'Medir duración no introduce tiempo como primitiva del álgebra. Vigencia de permisos y duración de ejecución tienen contratos distintos.'}
save(D/'PRESUPUESTO_SINTETICO.json',policy)
fixtures={str(p.relative_to(D)):{'bytes':p.stat().st_size,'sha256':sha(p)} for p in sorted((D/'entradas').iterdir())}
save(D/'ENTRADAS_COMPROMETIDAS.json',{'estatuto':'Bytes de prueba y sus identidades; no ensayo de admisión ejecutado','archivos':fixtures})
cs=[]
def add(kind,title,data,expected,seat):
 cs.append({'id':f'C11-{len(cs)+1:02}','clase':kind,'descripcion':title,'entrada_o_estimulo':data,'resultado_esperado_contractual':expected,'sede_a_contrastar':seat,'observador_requerido':'Captura independiente de bytes recibidos, contadores antes/después, reserva, decisión, llamadas y efecto. Medidas del proceso separadas del cálculo de carga; receptor y observador materiales pendientes.','estado':'ESPECIFICADO_NO_EJECUTADO','resultado_observado':None})
add('positivo','Texto ASCII en el límite',{'archivos':['entradas/ascii-4096.txt']},'Admitir esta guarda de tamaño con4096 bytes; no acredita las guardas semánticas posteriores.','Recepción anterior a copia/parseo')
add('negativo','Texto ASCII una unidad por encima',{'archivos':['entradas/ascii-4097.txt']},'Rechazar exceso4097 sin truncamiento ni creación de salida completa.','Recepción')
add('positivo','UTF-8 en el límite',{'archivos':['entradas/utf8-4096.txt'],'caracteres':2048},'Contabilizar4096 bytes y admitir esta guarda sin alterar tildes.','Recepción y transporte UTF-8')
add('negativo','UTF-8 contado como caracteres',{'archivos':['entradas/utf8-4097.txt'],'caracteres':2049},'Detectar4097 bytes aunque haya2049 caracteres; rechazar exceso.','Contador de bytes')
add('positivo','Texto agregado en el límite',{'archivos':['entradas/ascii-4096.txt','entradas/utf8-4096.txt']},'Admitir agregado8192 con ambas piezas de4096; registrar consumo total.','Presupuesto por petición')
add('negativo','Fragmentación elude límite agregado',{'fragmentos_bytes':[4096,4096,1]},'Rechazar agregado8193 aunque cada pieza respete4096.','Acumulador de recepción')
add('positivo','Imagen decodificada en el límite',{'ancho':256,'alto':256,'canales_salida':4},'Admitir guarda de65536 píxeles y262144 bytes RGBA; no afirmar memoria total ni paridad gráfica.','Prevalidación de dimensiones')
add('negativo','Imagen un píxel por encima',{'ancho':65537,'alto':1,'canales_salida':4},'Rechazar65537 píxeles antes de reservar salida; no generar raster enorme.','Prevalidación de dimensiones')
add('negativo','Producto no representable',{'ancho':4294967295,'alto':4294967295,'canales_salida':4},'No envolver ni truncar el producto: comprobar representabilidad y límites antes de asignar. Si otro límite rechaza antes, no atribuirlo a la guarda de overflow.','Aritmética de tamaños u32/u64/usize')
add('negativo','Imagen comprimida pequeña con expansión excesiva',{'bytes_codificados_declarados':1024,'ancho_decodificado':512,'alto_decodificado':512,'canales_salida':4,'archivo_comprimido':'PENDIENTE'},'No admitir por tamaño comprimido; rechazar262144 píxeles y controlar recursos internos del decodificador.','Decodificación bajo presupuesto')
add('positivo','Arquitectura sintética dentro de cuota',{'nodos':8,'profundidad_nodos':4,'celdas_n':[16,25,16,25,16,25,16,25],'enlaces':[[0,1],[1,2],[2,3],[0,4],[4,5],[5,6],[0,7]]},'Admitir sólo las cuotas de nodos/profundidad, conservar164 posiciones sin relleno. Constitución semántica y enlaces SV pendientes.','Construcción y recorrido de arquitectura')
add('negativo','Un nodo adicional',{'nodos':9,'profundidad_nodos':4},'Rechazar cuota9; no fusionar ni eliminar células para adaptar carga.','Admisión de arquitectura')
add('negativo','Profundidad adicional o ciclo',{'variantes':[{'nodos':5,'enlaces':[[0,1],[1,2],[2,3],[3,4]]},{'nodos':2,'enlaces':[[0,1],[1,0]]}]},'Distinguir profundidad5 de ciclo: ambos no admitidos en este testigo; no recorrer sin cota. Dos subcasos requieren capturas separadas.','Recorrido acotado')
add('positivo','Concurrencia en el límite',{'peticiones_activas':2},'Mantener dos reservas identificadas; ninguna liberación duplicada.','Supervisor de peticiones')
add('negativo','Carrera por tercera petición',{'peticiones_activas':1,'solicitudes_simultaneas_nuevas':2},'Admitir como máximo una nueva reserva; la otra se rechaza según este presupuesto, sin cola ilimitada.','Reserva atómica o serializada')
add('negativo','Salida una unidad excesiva',{'salida_bytes':8193},'Detener producción según contrato y marcar resultado incompleto; no entregar prefijo como salida íntegra.','Serialización y entrega')
add('negativo','Reintentos agotan cuota de herramientas',{'intentos_despachados':3,'solicitud_siguiente':4,'motivo':'reintento tras fallo'},'No despachar cuarto intento ni reiniciar contador por el fallo; conservar efectos previos e incertidumbre técnica.','Adaptador de herramienta y supervisor')
add('negativo','Reserva fallida o memoria no observada',{'simulacion':'fallo de reserva','memoria_proceso_medida':None},'Registrar fallo técnico y medida no disponible; no declarar memoria conforme ni producir Tri.U.','Asignación y observación de proceso')
add('negativo','Agotamiento cancela sólo la narración',{'simulacion':'supervisor solicita cancelación; herramienta continúa','tiempo_ms':None},'Observar terminación real, tareas hijas y efectos; si no se puede confirmar cese, registrar estado no acreditado. No declarar plazo cumplido sin límite fijado.','Cancelación y efecto externo')
add('negativo','Trazabilidad eliminada para aparentar ahorro',{'simulacion':'omitir script o registro de llamada para reducir salida'},'No declarar actividad íntegramente auditada; prever custodia recuperable acotada y referencia fiel o detener la actividad según contrato.','Custodia y presupuesto de evidencia')
assert len(cs)==20
save(D/'BANCO_PREVIO_v0_1.json',{'version':'BIS-C11-BANCO/0.1','estatuto':'Especificaciones sintéticas de recursos; no pruebas Rust ni de IA ejecutadas','presupuesto':{'ruta':'PRESUPUESTO_SINTETICO.json','sha256':sha(D/'PRESUPUESTO_SINTETICO.json')},'variantes':20,'ejecutadas':0,'nota_conteo':'Veinte escenarios; C11-13 contiene dos subcasos aún no materializados. No se equiparan a número de ejecuciones.','casos':cs})
(D/'CONTRATO_RECURSOS_CANDIDATO_v0_1.md').write_text('''# Contrato candidato de presupuestos y límites de recursos

**Versión 0.1 · 13 de septiembre de 2026 · S22 · BIS-02/C11 · Juan Antonio Lloret Egea y Watson**

## 1. Objeto y estatuto

C11 concreta BIS-O10 y BIS-O05 y el §7 del workflow. El soporte debe declarar qué recursos admite, dónde los controla y con qué evidencia. Los presupuestos pertenecen al ensayo o al soporte constituido; no establecen máximos algebraicos universales. Este contrato es candidato documental: no introduce límites productivos en el núcleo ni acredita ejecución nueva en Rust.

N permanece fijo en una célula admitida y cumple N=b², b≥3. Una dimensión válida puede exceder el soporte disponible. Esa no admisión debe expresarse como tal, sin relleno, truncamiento, cambio de dimensión ni conversión a U. No se adopta (9,3) por defecto. Las 3ᴺ posibilidades matemáticas no obligan a enumerarlas o almacenarlas.

## 2. Magnitudes y alcance de cada límite

| Objeto | Magnitud y condición |
| --- | --- |
| Texto | Bytes UTF-8 por pieza y acumulados por petición; distinguir caracteres, tokens y bytes. |
| Imagen | Bytes codificados, dimensiones y píxeles decodificados, salida y temporales del decodificador. |
| Composición | Nodos, enlaces, profundidad, ciclos y coste real del recorrido. |
| Memoria | Carga, capacidad reservada, copias y pico del proceso medidos por separado. |
| Concurrencia | Peticiones activas, reservas globales, tareas hijas y liberación de recursos. |
| Herramientas | Intentos despachados, reintentos, proveedores, entradas, salidas y efectos. |
| Tiempo | Duración, espera e interrupción observadas; reloj y condiciones de medida identificados. |
| Evidencia | Custodia recuperable de artefactos y registros dentro de un presupuesto explícito. |

Un array limita su cardinalidad, pero no acota por sí solo las asignaciones de sus elementos, el número de instancias o toda la ejecución. Un Vec encapsulado puede conservar una longitud semánticamente fija; su capacidad física y sus copias requieren observación propia. No se sustituye masivamente Vec por arrays ni se deduce rendimiento de la seguridad de tipos.

Cada límite declara unidad, ámbito, valor inclusivo, punto de comprobación, contador o medidor, respuesta al exceso y evidencia. Un crecimiento autorizado exige actualizar la reserva antes del consumo. La comprobación debe preceder a la asignación costosa que pretende evitar. Los metadatos de tamaño declarados por la entrada no sustituyen la cuenta de bytes realmente recibidos o la expansión efectiva.

## 3. Sedes inspeccionadas y límites existentes

S11 Contexto comprueba documento.len() frente a8192. Recibe un slice ya existente: esta guarda acota su admisión local, sin demostrar que su transporte o construcción previa no consumieron más memoria.

S3 destino::leer utiliza un búfer fijo de16385 bytes y rechaza al llenarlo; permite observar el primer byte por encima del máximo admitido de16384. Es un conductor de laboratorio y no constituye el transporte productivo de SV.

La CLI sv-native inspeccionada usa fs::read_to_string antes de compile_svp; en esa ruta no se observa un presupuesto de lectura previo. En la ABI wasm32 inspeccionada, resize_buffer aplica Vec::resize a la longitud solicitada; la compilación clona búferes y packed_result comprueba el tamaño de una salida ya producida. No se infiere por ello ausencia de límites en todos los hosts; tampoco se acredita una cuota global por esas guardas locales.

validate_admissibility_table emplea checked_mul al calcular una cardinalidad. Esa guarda evita un producto no representable en ese punto; no constituye un límite de memoria o complejidad para todo el sistema. Una implementación deberá comprobar representabilidad y cuota sin depender de diferencias de overflow entre perfiles debug/release. La anchura de usize del destino no se presume idéntica a la del conductor.

Estas observaciones son estáticas sobre el corte identificado. BIS-03 decidirá sedes y BIS-04 materializará las correcciones justificadas. El presente contrato no altera ABI, política del host, parser, IR ni semántica.

## 4. Presupuesto sintético y testigos

PRESUPUESTO_SINTETICO.json fija4096 bytes por pieza de texto y8192 por petición;65536 píxeles y262144 bytes para una salida RGBA de cuatro bytes por píxel; ocho nodos, profundidad máxima de cuatro nodos, dos peticiones activas,8192 bytes de salida por petición y tres intentos de herramienta despachados. Son valores pequeños elegidos para contrastar fronteras reproducibles; no una recomendación de capacidad profesional.

Cuatro archivos literales permiten contrastar4096/4097 bytes tanto en ASCII como con tildes UTF-8. Los demás casos especifican dimensiones, grafos o secuencias: aún requieren receptor, mutantes, entradas materiales cuando proceda y capturas. El caso de expansión no incluye una imagen comprimida real y el caso de overflow no debe asignar un raster gigante para comprobar el producto.

La composición sintética contiene ocho nodos alternando16 y25 posiciones,164 en total. Es un descriptor de carga: no constituye un dominio, una arquitectura SV admitida ni una tabla de relación. La profundidad se cuenta por nodos; un ciclo se clasifica por separado. El presupuesto de carga no valida compatibilidad algebraica.

Las cuotas de memoria del proceso y duración quedan sin valor porque requieren entorno y mediciones previas. El tamaño del raster o la suma de textos no se presenta como pico de memoria. No se declara cumplimiento de un límite aún no fijado. La medición temporal y la vigencia externa no añaden tiempo como primitiva del núcleo.

## 5. Agentes, llamadas y efecto real

El traslado a una IA comprende todo el recorrido: solicitud, acceso a fuentes, preparación y ejecución de scripts, llamadas a herramientas, reintentos, resultado y entrega. Una cuota de llamadas cuenta los intentos realmente despachados, también los fallidos. Un reintento no renueva permisos ni presupuesto y puede repetir un efecto previo; requiere la política aplicable y el estatuto del resultado anterior.

La reserva de concurrencia debe resistir carreras. La tercera petición se rechaza en este testigo; no se oculta en una cola ilimitada. El fin aparente de una respuesta no prueba que hayan cesado procesos hijos, peticiones remotas o efectos ya comprometidos. La cancelación exige declarar qué puede interrumpirse, quién lo observa y qué queda indeterminado. Sin un supervisor capaz de comprobarlo no se afirma contención material.

Los costes del núcleo, del renderizador, del host y del modelo se registran separadamente. Un dato de consumo comunicado por un proveedor conserva esa procedencia; no se atribuye al observador una medición que no realizó. Los campos ausentes quedan null con motivo; cero sólo significa cero medido.

## 6. Trazabilidad heredada y confianza de construcción

Se hereda la obligación ya fijada de conservar servicios y conectores usados, herramientas y versiones, invocaciones, entradas, scripts exactos, contenido generado, parches con estados previo y posterior, salidas, errores y medidas disponibles. La referencia es el expediente de trazabilidad §§6–7 y el contrato S6 de actividad. C10 añade respaldo de afirmaciones y no reemplaza esas obligaciones.

La autorización vigente permite continuar la construcción y sus verificaciones ordinarias. No se abre otra campaña de selección del constructor ni se exige una confirmación humana por cada operación. La confianza de trabajo y la conservación de evidencia cumplen funciones compatibles. Los resultados externos ya recibidos conservan su estatuto documental y no se reinterpretan como capturas de todas las herramientas de sus participantes.

El presupuesto de salida no autoriza eliminar evidencia para aparentar cumplimiento. Debe existir custodia recuperable y acotada de los artefactos relevantes; la presentación puede enlazarlos. Si la infraestructura impide conservarlos, se registra la limitación y no se declara trazabilidad íntegra. No se incluyen credenciales en la publicación de registros.

## 7. Ensayo pendiente y salida

El banco contiene veinte escenarios: seis positivos y catorce negativos. C11-13 contiene dos subcasos que deberán tener capturas separadas. Todos mantienen observado nulo. Se contrastarán el límite exacto y su primera unidad superior, fragmentación, expansión, overflow, composición, concurrencia, salida, reintentos, reserva fallida, cancelación y custodia.

Antes de ejecutar se fijarán compilador, destinos, perfiles de compilación, receptor, presupuesto, contadores independientes y observador del proceso. Las guardas se ensayarán con entradas positivas y negativas y con sensibilidad deliberada. Si una guarda anterior impide alcanzar otra, sólo se acreditará la que actuó. Las pruebas sintéticas de frontera, la conducta de un modelo concreto y la viabilidad para una carga profesional tendrán alcances separados.

C11-P/N originales continúan pendientes. Esta entrega acredita preparación y cotejo auxiliar de sus datos, no recepción Rust, consumo máximo ni resistencia de una IA. Se conservan dos escenarios originales ejecutados y veintidós pendientes. Continúa C12: perfiles ES/EN, documentación y vías de construcción. Tras BIS-02, BIS-03 decidirá sedes y BIS-04 realizará lo justificado. GUI pendiente en S24.
''')
(D/'README.md').write_text('''# BIS-C11 · Presupuestos y límites de recursos

**S22 · BIS-02 en ejecución · RETP-2026-214**

[Contrato candidato](CONTRATO_RECURSOS_CANDIDATO_v0_1.md) · [Banco previo](BANCO_PREVIO_v0_1.json) · [Presupuesto sintético](PRESUPUESTO_SINTETICO.json) · [Entradas comprometidas](ENTRADAS_COMPROMETIDAS.json) · [Fuentes](FUENTES.json) · [Verificación documental](VERIFICACION_DOCUMENTAL.json) · [Código de preparación y comprobación](administracion/README.md).

Veinte escenarios especificados: seis positivos y catorce negativos; C11-13 contiene dos subcasos. Cuatro entradas textuales literales. **Cero escenarios C11 ejecutados en Rust o en una IA.** Las cuotas son testigos de laboratorio, no máximos universales ni soporte aprobado. Memoria del proceso y duración máxima permanecen por fijar y medir.

Se distingue guarda de tamaño local de protección anterior a asignar; bytes de texto de caracteres; raster de memoria total; declaración de cancelación de cese efectivo. La trazabilidad conserva scripts, parches, herramientas y conectores conforme a las obligaciones ya establecidas.

C01–C10 conservan sus estados. Dos escenarios originales ejecutados y22 pendientes. Continúa C12: perfiles ES/EN, documentación y construcción. S24 mantiene la GUI diferida.
''')
paths=[(f['ruta'],'Lectura rectora completa previa; identidad cotejada frente a C10') for f in rectors]
base='docs/calidad/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/'
for p,scope in [('rust/sv_native/src/main.rs','Lectura completa; entrada fs::read_to_string'),('rust/sv_wasm/src/lib.rs','Cabecera y ABI de búferes, packed_result y sv_compile_svp_json'),('rust/sv_core/src/wellformed.rs','validate_admissibility_table completo'),(base+'s11-contexto-y-cobertura/codigo/contexto.rs','Lectura completa; límite de slice existente'),(base+'s3-presentacion-y-negacion/codigo/destino.rs','Lectura completa; lectura acotada de laboratorio'),(base+'s6-trazabilidad-total/CONTRATO.md','Lectura completa previa; actividad y medición'),('docs/calidad/tuberias-ia/trazabilidad-auditoria-y-reproduccion-del-trabajo-ia/TRAZABILIDAD_AUDITORIA_Y_REPRODUCCION_DEL_TRABAJO_IA_2026_09_12.md','Lectura completa previa; §§6–7 conservación y coste'),('docs/calidad/tuberias-ia/diagnosticos-del-compilador-y-procedencia/ESTADO_PARA_OTRAS_IA.md','Lectura completa previa; conectores y traza')]:paths.append((p,scope))
for p in ['WORKFLOW_P1_P3_BIS_v2.md','BANCO_PREVIO_BIS_02_v0_1.json','bis-c05/CRITERIO_DE_ACEPTACION_RUST_v1.md']:paths.append((str((B/p).relative_to(R)),'Continuidad y obligaciones C11/C12'))
save(D/'FUENTES.json',{'corte_lenguaje':HEAD,'corte_laboratorio':LAB,'fuentes':[{'ruta':p,'sha256':sha(R/p),'lectura':scope} for p,scope in paths],'alcance':'Revisión estática y preparación sintética; sin cambios de núcleo ni ensayos funcionales C11'})
s=json.loads((B/'ESTADO_WORKFLOW.json').read_text());s.update(ultimo_informe='bis-c11/README.md',ultimo_retp='RETP-2026-214',siguiente_accion='Continuar BIS-C12: perfiles ES/EN, documentación y vías de construcción. Completar contratos BIS-02; decidir sedes BIS-03 y realizar y probar lo justificado en Rust.',ampliacion='BIS-C11: presupuestos sintéticos y veinte escenarios de recursos, cuatro entradas textuales; cero ejecutados. Trazabilidad heredada explícita; cuotas globales pendientes de medición.')
s['preparacion_bis_c11']={'contrato':'bis-c11/CONTRATO_RECURSOS_CANDIDATO_v0_1.md','banco':'bis-c11/BANCO_PREVIO_v0_1.json','variantes_especificadas':20,'variantes_ejecutadas':0,'entradas_textuales':4,'corte_revision':HEAD,'alcance':'Cuotas y escenarios de prueba; memoria de proceso y duración aún sin límites constituidos'};save(B/'ESTADO_WORKFLOW.json',s)
p=B/'README.md';t=p.read_text().replace('## Trabajo vigente: S22 · RETP-2026-213','## Trabajo vigente: S22 · RETP-2026-214').replace('Continúa BIS-C11: presupuestos y límites de recursos;','[BIS-C11](bis-c11/README.md) añade presupuesto sintético y veinte escenarios de recursos, cuatro entradas textuales, cero ejecutados. Continúa BIS-C12: perfiles ES/EN, documentación y construcción;');p.write_text(t)
t=L.read_text().replace('**Trabajo vigente: S22 / RETP-2026-213.','**Trabajo vigente: S22 / RETP-2026-214.');a=t.index('**Siguiente objeto material:**');z=t.index('\n\n',a);t=t[:a]+'''**Siguiente objeto material:** continuar BIS-C12: perfiles ES/EN, documentación y vías de construcción. [BIS-C11](../paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis-c11/README.md) fija presupuestos sintéticos, veinte escenarios y cuatro entradas textuales; cero ejecutados. Los límites locales no prueban una cota global de memoria; cancelación declarada no acredita cese efectivo. Se hereda la conservación de scripts, código, parches, servicios, herramientas y conectores; una cuota no permite borrar evidencia. C01 conserva prueba nativa y C02–C11 sus estados; dos escenarios originales ejecutados y22 pendientes. BIS-03 decide sedes y BIS-04 realiza lo justificado.'''+t[z:];L.write_text(t)
f,rs=rows();row=next(x for x in rs if x['id']=='S22');tm=now();row.update(fecha_actualizacion_utc=tm,cortes_de_entrada='Lenguaje '+HEAD+'; laboratorio '+LAB,resultado='BIS-C11 preparado: contrato y presupuesto sintético de recursos; veinte escenarios, seis positivos y catorce negativos, cuatro entradas textuales. Se distingue control local de protección previa a asignación, recursos agregados, concurrencia, llamadas y cese efectivo. Trazabilidad heredada explícita.',verificacion='Cotejo auxiliar Python de bytes UTF-8, inventario, cuotas testigo, huellas e historial. Cero escenarios C11 ejecutados; sin compilación nueva o modelo interrogado. Memoria de proceso y duración máxima pendientes de fijar y medir. Dos escenarios originales ejecutados y veintidós pendientes.',evidencias='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/'+str((D/'README.md').relative_to(R)),referencia_calidad='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-214',siguiente_accion=s['siguiente_accion'],observaciones='Sin cambios de semántica, IR, Rust, dominios o catálogo. Las cuotas de ensayo no constituyen soporte universal. No se inicia otra campaña de selección del constructor. S24 pendiente; BIS-02 sigue abierto.')
guardar(f,rs,row);retp(214,'S22 · BIS-C11: presupuestos y límites de recursos',tm,row,'tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis-c11/README.md','PREPARACION_CONTRACTUAL','Pilares; workflow §7; S3/S11; entrada nativa y ABI WASM; trazabilidad §§6–7','Contrato y testigos previos; medición y control material pendientes')
print('C11 y RETP-214 preparados')
