# Contrato candidato de presupuestos y límites de recursos

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
