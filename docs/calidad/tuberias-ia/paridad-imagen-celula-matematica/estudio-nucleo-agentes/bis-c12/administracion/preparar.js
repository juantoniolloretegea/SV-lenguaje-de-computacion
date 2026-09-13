
// Preparación documental C12. Ejecutar en functions.exec con load/store disponibles.
// No compila Rust ni interpreta SVP. Datos base recuperados del corte HEAD mediante GitHub.
const B=load("B"),D=B+"/bis-c12",HEAD=load("head"),LAB=load("labhead"),F={};
const put=(p,s)=>F[D+"/"+p]=s;
const js=(p,v)=>put(p,JSON.stringify(v,null,2)+"\n");
const en=[
"-- Testigo sintético C12; sin constitución de dominio.",
"codomain K = { ALFA, BETA, GAMMA };",
"output_semantics Significado {",
' ALFA -> "codomain codominio: alfa";',
' BETA -> "evaluate evaluar: beta";',
' GAMMA -> "U: sin clasificación fuerte";',
"}",
"cellspec Celda { b: 4; codomain: K; semantics: Significado; role: Base; }",
"cellstate Estado { spec: Celda; vector: [Zero,One,U,Zero,One,U,Zero,One,U,Zero,One,U,Zero,One,U,Zero]; }",
"let Evaluacion = evaluate(Estado);",""].join("\n");
const es=[
"-- Testigo sintético C12; sin constitución de dominio.",
"codominio K = { ALFA, BETA, GAMMA };",
"semántica_de_salida Significado {",
' ALFA -> "codomain codominio: alfa";',
' BETA -> "evaluate evaluar: beta";',
' GAMMA -> "U: sin clasificación fuerte";',
"}",
"especificación_de_celda Celda { b: 4; codominio: K; semántica: Significado; rol: Base; }",
"estado_de_celda Estado { especificación: Celda; vector: [Cero,Uno,U,Cero,Uno,U,Cero,Uno,U,Cero,Uno,U,Cero,Uno,U,Cero]; }",
"sea Evaluacion = evaluar(Estado);",""].join("\n");
const entries={
"base.en.svp":en,"base.es.svp":es,
"comentario.es.svp":es.replace("-- Testigo sintético C12; sin constitución de dominio.","-- Comentario distinto; evaluate no es aquí una instrucción."),
"dato_distinto.es.svp":es.replace("codomain codominio: alfa","codominio: alfa"),
"orden_distinto.es.svp":es.replace("[Cero,Uno,U,","[Uno,Cero,U,"),
"perfil_mezclado.es.svp":es.replace("sea Evaluacion","let Evaluacion"),
"dimension_invalida.en.svp":en.replace("b: 4","b: 2"),
"ir_privada.rs":'use sv_core::IrProgram;\nfn main() {\n let _ = IrProgram { source_file: String::new(), source_sha256: String::new(), objects: Vec::new(), operations: Vec::new() };\n}\n',
"constructor_interno.rs":'fn main() {\n let _ = sv_core::ir::construction::program(String::new(), String::new(), Vec::new(), Vec::new());\n}\n',
"documentacion_conforme.txt":"ES: La aceptación acredita las validaciones materializadas; no acredita toda obligación abstracta de la IR.\nEN: Acceptance establishes the implemented validations; it does not establish every abstract IR obligation.\n",
"documentacion_contradictoria.txt":"ES: La aceptación acredita las validaciones materializadas; no acredita toda obligación abstracta de la IR.\nEN: Acceptance establishes every abstract IR obligation.\n"};
for(const [p,s]of Object.entries(entries))put("entradas/"+p,s);
js("ENTRADAS_COMPROMETIDAS.json",{estatuto:"Once entradas literales sin ejecución funcional; identidad Git por archivo en el árbol del commit publicado.",archivos:Object.keys(entries).map(x=>"entradas/"+x)});
js("ORACULO_CANONICO.json",{
version:"BIS-C12-ORACULO/0.1",gramatica:"0.2",ir:"0.3",serializador:"0.1.0",
perfiles:{"base.en.svp":"en","base.es.svp":"es"},
comparacion:"Igualdad completa y ordenada de objects() y operations(), variantes, campos y cardinalidad; versiones cotejadas. Procedencia verificada por separado.",
procedencia_excluida_solo_de_paridad:["source_file","source_sha256"],
anclas_independientes:{objetos_en_orden:["K","Significado","Celda","Estado"],codominio:["ALFA","BETA","GAMMA"],textos:["codomain codominio: alfa","evaluate evaluar: beta","U: sin clasificación fuerte"],b:4,n:16,especificacion:"Celda",semantica:"Significado",rol:"Base",vector:["Zero","One","U","Zero","One","U","Zero","One","U","Zero","One","U","Zero","One","U","Zero"],operacion:{nombre:"Evaluacion",clase:"Evaluate",argumento:"Estado"}},
limite:"Anclas fijadas sin ejecución del sujeto; no serializador alternativo ni oráculo completo de la IR. El conductor Rust pendiente cotejará todos los campos. Un nodo Evaluate no demuestra evaluación productiva ni paridad visual."});
const cases=[];
function add(clase,descripcion,archivos,resultado_esperado_contractual,sede_a_contrastar,estimulo_adicional=""){
cases.push({id:"C12-"+String(cases.length+1).padStart(2,"0"),clase,descripcion,archivos:archivos.map(x=>"entradas/"+x),estimulo_adicional,resultado_esperado_contractual,sede_a_contrastar,estado:"ESPECIFICADO_NO_EJECUTADO",resultado_observado:null});}
add("positivo","Par ES/EN de una misma operación",["base.en.svp","base.es.svp"],"Compilar con perfiles explícitos; comparar objetos, operaciones, versiones y anclas. Verificar procedencias diferentes por separado.","compile_svp_profile");
add("positivo","Comentario cambia sin alterar la operación",["base.es.svp","comentario.es.svp"],"Conservar contenido canónico y verificar que cambian las huellas de fuente.","Compilación y procedencia");
add("positivo","Compatibilidad sin selector",["base.en.svp"],"compile_svp coincide con SourceProfile::En para los mismos bytes y nombre; no autodetecta.","API de compatibilidad");
add("negativo","Perfil explícito incorrecto",["base.es.svp"],"Rechazar fuente ES bajo En; conservar diagnóstico real.","compile_svp_profile","Usar SourceProfile::En");
add("negativo","Mezcla de formas protegidas",["perfil_mezclado.es.svp"],"Rechazar let como producción del perfil Es; no cambiar perfil silenciosamente.","Parser Es");
add("negativo","Etiqueta no constituida",[],'SourceProfile::from_tag("ES") devuelve None; no normalizar ni recurrir a perfil por defecto.',"Selección de perfil","Etiqueta ES en vez de es");
add("positivo","Cadenas y nombres conservados",["base.en.svp","base.es.svp"],"Conservar exactamente tres textos y nombres nominales; palabras citadas no son instrucciones.","Lexer, IR y anclas");
add("negativo","Traducción indebida de una cadena",["base.es.svp","dato_distinto.es.svp"],"Detectar diferencia de datos aunque ambas entradas puedan ser admitidas; no llamarla error de sintaxis.","Comparador");
add("negativo","Permutación de posiciones",["base.es.svp","orden_distinto.es.svp"],"Detectar orden diferente en vector de 16 posiciones; no comparar conjuntos ni ordenar.","Comparación posicional");
add("negativo","Normalización Unicode no autorizada",[],"No equiparar automáticamente grafías distintas; clasificar cada fuente según perfil antes de comparar.","Perfil de identificadores","Materializar identificador con tilde precompuesta y descompuesta; no presumir ambos admitidos");
add("positivo","Ensamblaje por perfiles de unidad",[],"Analizar unidades hasta EOF por separado, reunir IR ordenada y validar globalmente. Identidad de ensamblaje propia.","compile_svp_assembly","Separar declaraciones y estado/operación de las bases: unidad EN y unidad ES; entradas separadas pendientes");
add("negativo","Producción repartida entre archivos",[],"Rechazar unidades incompletas; concatenar texto primero no ejecuta el contrato del ensamblaje.","EOF por unidad","Dividir una producción; entradas pendientes");
add("negativo","Descriptor confundido con aceptación",[],"SourceUnit::new sólo crea descriptor; rechazar referencia global inexistente al compilar el ensamblaje.","SourceUnit y validación global","Unidad con referencia inexistente y otra neutra; materialización pendiente");
add("negativo","Reconstrucción externa de IrProgram",["ir_privada.rs"],"Fallar por campos privados con sv_core enlazado correctamente; dependencia ausente no acredita barrera.","Consumidor Rust compile_fail");
add("negativo","Acceso al constructor interno",["constructor_interno.rs"],"Fallar por privacidad de construction y contrastar control positivo de API pública en igual entorno.","rustc y privacidad");
add("positivo","Documentación bilingüe concordante",["documentacion_conforme.txt"],"Revisión confirma mismo alcance y negación; no altera perfil SVP.","Revisión contractual");
add("negativo","Traducción elimina limitación",["documentacion_contradictoria.txt"],"Detectar afirmación EN más fuerte que ES y contrato; compilar Rust no acredita paridad de prosa.","Revisión de documentación");
add("negativo","Función, método o macro elude guardas",["dimension_invalida.en.svp"],"Cada vía candidata debe conservar rechazo de b=2; requiere ensayo y control positivo propios.","API y expansión","Inventariar vías existentes; no se declara una macro SV implementada");
add("negativo","Doctest falla por causa ajena",[],"No acreditar barrera mediante import inexistente; conservar stderr y control positivo; ejecutar doctests afectados.","cargo test --doc","Mutante con import incorrecto; materialización pendiente");
add("negativo","IA parchea sin conservar contenido",[],"Conservar fuentes, perfiles, servicios, conector, invocación, script, antes/parche/después y resultado real; narración no sustituye evidencia.","Actividad IA y observador","Receptor, herramienta y observador a concretar para ensayo real");
js("BANCO_PREVIO_v0_1.json",{version:"BIS-C12-BANCO/0.1",estatuto:"Especificación previa; no pruebas Rust ni actividad de IA ejecutadas",variantes:20,ejecutadas:0,oraculo:"ORACULO_CANONICO.json",casos:cases});
put("CONTRATO_PERFILES_DOCUMENTACION_CONSTRUCCION_v0_1.md",[
"# Contrato candidato de perfiles, documentación y vías de construcción","",
"**Versión 0.1 · 13 de septiembre de 2026 · S22 · BIS-02/C12 · Juan Antonio Lloret Egea y Watson**","",
"## 1. Objeto y estatuto","",
"C12 desarrolla BIS-O11 y BIS-O05: conservar significado bajo perfiles fuente constituidos, documentar fielmente Rust y evitar vías de construcción que eludan validaciones. Se prepara contrato y banco de contraste. No se amplían gramática, IR, núcleo o biblioteca SV.","",
"El suceso es el hecho; la prosa, la matemática y la imagen son formas de representarlo. Se distingue el hecho de preparar entradas del hecho futuro de ejecutarlas. Un JSON no demuestra por sí solo la conducta del compilador, de una IA o de sus herramientas.","",
"## 2. Ámbitos lingüísticos","",
"| Ámbito | Contrato |","| --- | --- |","| Perfil fuente SVP | Selección en/es y correspondencia de formas constitutivas con gramática canónica. |","| Documentación Rust | Explicaciones ES/EN concordantes sobre las mismas API e invariantes. |","| Presentación al humano | Lengua de interfaz o explicación; no selecciona implícitamente perfil fuente. |","",
"La norma SVP distingue 154 formas canónicas, 11 compartidas y 297 grafías distintas. SourceProfile reconoce En/Es, etiquetas exactas en/es y ABI 0/1. Sin selector, la entrada conserva EN por compatibilidad. No se autoriza autodetección, traducción libre, normalización Unicode, cambios de mayúsculas o ampliación de enumeraciones cerradas.","",
"Comentarios, identificadores nominales y cadenas conservan sus contratos propios. Una forma reservada citada dentro de una cadena es contenido. Traducir formas constitutivas no autoriza traducir datos del dominio. El perfil léxico de identificadores conserva su estatuto separado.","",
"## 3. Paridad y procedencia","",
"El par sintético emplea b=4 y N=16, vector plano ordenado, codominio neutro y una operación Evaluate. Es un testigo de compilación; no constituye dominio, soporte ni carga representativa. No fija tamaño predeterminado para SV.","",
"La prueba futura comparará íntegramente objects() y operations(), variantes, cardinalidad, campos, nombres, referencias, valores y orden. Las anclas previas de ORACULO_CANONICO.json evitan aceptar sin más dos resultados igualmente incorrectos; no sustituyen el cotejo completo ni definen otra IR.","",
"source_file y source_sha256 se verificarán contra el nombre y bytes originales de cada fuente. Sólo se excluyen de la igualdad semántica entre archivos distintos: permanecen en la evidencia. No se borran otros campos, normalizan cadenas ni ordenan posiciones. La coincidencia de Evaluate no acredita evaluación productiva ni paridad visual. K1-T conserva su limitación vigente.","",
"Un comentario puede cambiar la huella sin cambiar significado. En Rust puede afectar líneas o documentación generada; no se promete identidad binaria por equivalencia documental.","",
"## 4. Construcción y aceptación","",
"Las entradas públicas compile_svp, compile_svp_profile y compile_svp_assembly inspeccionadas aplican conformidad de dominios cerrados, bienformación general, de transición/datos y de contexto antes de devolver un programa aceptado. Esta constatación estática no ejecuta cada guarda.","",
"frontend es interno. IrProgram conserva campos privados y getters de lectura. ir::construction y sus constructores son pub(crate), accesibles dentro del núcleo. Esos constructores internos no validan por sí solos: la vía pública aplica controles posteriores. SourceUnit::new crea descriptor de fuente, no una IR aceptada.","",
"El ensamblaje analiza cada unidad con su perfil hasta EOF, reúne objetos y operaciones en orden y valida globalmente. No concatena texto ni permite repartir una producción entre archivos. Su identidad incorpora fuentes, nombres y perfiles; no equivale a la huella de un archivo individual.","",
"Una biblioteca SV puede ofrecer funciones y métodos; una macro, si se justifica, debe expandirse a vías con las mismas obligaciones. La forma de invocación no concede autoridad ni crea una primitiva semántica. Biblioteca, macros y sedes se decidirán en BIS-03 y se materializarán justificadamente en BIS-04. No se afirma su existencia por describirlas.","",
"Las barreras externas requieren consumidores correctamente enlazados con sv_core, controles positivos y diagnóstico de privacidad. compile_fail por dependencia ausente o sintaxis ajena no prueba la restricción pretendida. Las garantías de tipos tampoco sustituyen la validación de contenido recibido en ejecución.","",
"## 5. Documentación ES/EN","",
"Se aplica DOCUMENTACION_RUST_ES_EN_v1: conservar condiciones, negaciones, cuantificadores, unidades, límites, efectos y errores. API, identificadores, versiones y códigos no se traducen arbitrariamente. Un ejemplo ejecutable se escribe una vez con explicación bilingüe y declara perfil si es SVP.","",
"Comentarios ordinarios y documentación ejecutable requieren comprobaciones diferentes. Los doctests afectados deben ejecutarse. Su compilación no prueba equivalencia entre párrafos. El estímulo conforme y el mutante que elimina una reserva permiten revisión documental discriminante, con resultado y alcance propios. No se declara bilingüe todo el código existente.","",
"## 6. Actividad de IA","",
"El agente que selecciona perfil, genera fuente o parchea código debe conservar servicios, conectores, herramientas, invocaciones, scripts exactos, entradas, estados previo y posterior, parche, salidas y errores. El perfil declarado se coteja con la llamada efectiva y el efecto observado. Se mantienen las obligaciones de trazabilidad ya establecidas; una afirmación de equivalencia no es prueba de ejecución.","",
"Esta preparación no ejecuta modelos ni demuestra resistencia a una evasión. Define el contraste posterior con herramientas y efectos reales. La confianza de construcción autoriza avanzar con comprobaciones ordinarias; no abre otra campaña de selección del constructor.","",
"## 7. Banco, incidente de entorno y salida","",
"Veinte escenarios: seis positivos y catorce negativos. Once entradas literales: siete SVP, dos consumidores Rust para rechazo por privacidad y dos estímulos documentales. Unicode, ensamblaje, macros, doctests adversos y actividad real requieren materialización adicional según cada caso. Observados nulos.","",
"La lectura del checkout se completó antes de una desconexión del servidor de ejecución. El intento de crear el script local no obtuvo confirmación y no se ejecutó el preparador Python. La preparación y el cotejo documental efectivos se realizaron mediante JavaScript y el conector GitHub; se conserva el código utilizado. No se atribuye al entorno una ejecución Rust o Python que no realizó. La sincronización del checkout queda pendiente hasta su recuperación.","",
"El cotejo auxiliar comprueba inventario, referencias y cambios explícitos de entradas; no interpreta SVP como sustituto de Rust ni prueba privacidad o guardas. Los ensayos posteriores registrarán cargo/rustc, destino, perfil de compilación, comandos, entradas, salidas y causa discriminante según el criterio C05.","",
"C12 es la última familia numerada preparada. Sigue consolidar cobertura y pendientes C01–C12 en BIS-02 y preparar decisiones de sede BIS-03. Completar familias documentales no cierra BIS-02 ni el workflow. C01 mantiene su evidencia nativa; los otros bancos no se convierten en ejecutados. El original conserva dos escenarios ejecutados y veintidós pendientes. GUI diferida en S24.",""].join("\n"));
put("README.md",[
"# BIS-C12 · Perfiles, documentación y construcción","",
"**S22 · BIS-02 en ejecución · RETP-2026-215**","",
"[Contrato candidato](CONTRATO_PERFILES_DOCUMENTACION_CONSTRUCCION_v0_1.md) · [Banco previo](BANCO_PREVIO_v0_1.json) · [Oráculo](ORACULO_CANONICO.json) · [Entradas](ENTRADAS_COMPROMETIDAS.json) · [Fuentes](FUENTES.json) · [Comprobación](VERIFICACION_DOCUMENTAL.json) · [Código y operaciones](administracion/README.md).","",
"Veinte escenarios, seis positivos y catorce negativos; once entradas literales. **Cero escenarios C12 ejecutados en Rust o en una IA.** Paridad canónica y procedencia se cotejan separadamente. Documentación bilingüe no crea perfiles fuente. Funciones, métodos y posibles macros deben conservar las validaciones.","",
"El entorno local se desconectó tras la lectura; la preparación se realizó directamente mediante JavaScript y GitHub. Sin ejecución del preparador Python ni sincronización final del checkout.","",
"Siguiente: consolidar cobertura y pendientes C01–C12, preparar sedes BIS-03. Dos escenarios originales ejecutados y veintidós pendientes; S24 mantiene GUI diferida.",""].join("\n"));
store("c12files",F);store("c12entries",entries);store("c12cases",cases);
