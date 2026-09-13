# Contrato candidato de perfiles, documentación y vías de construcción

**Versión 0.1 · 13 de septiembre de 2026 · S22 · BIS-02/C12 · Juan Antonio Lloret Egea y Watson**

## 1. Objeto y estatuto

C12 desarrolla BIS-O11 y BIS-O05: conservar significado bajo perfiles fuente constituidos, documentar fielmente Rust y evitar vías de construcción que eludan validaciones. Se prepara contrato y banco de contraste. No se amplían gramática, IR, núcleo o biblioteca SV.

El suceso es el hecho; la prosa, la matemática y la imagen son formas de representarlo. Se distingue el hecho de preparar entradas del hecho futuro de ejecutarlas. Un JSON no demuestra por sí solo la conducta del compilador, de una IA o de sus herramientas.

## 2. Ámbitos lingüísticos

| Ámbito | Contrato |
| --- | --- |
| Perfil fuente SVP | Selección en/es y correspondencia de formas constitutivas con gramática canónica. |
| Documentación Rust | Explicaciones ES/EN concordantes sobre las mismas API e invariantes. |
| Presentación al humano | Lengua de interfaz o explicación; no selecciona implícitamente perfil fuente. |

La norma SVP distingue 154 formas canónicas, 11 compartidas y 297 grafías distintas. SourceProfile reconoce En/Es, etiquetas exactas en/es y ABI 0/1. Sin selector, la entrada conserva EN por compatibilidad. No se autoriza autodetección, traducción libre, normalización Unicode, cambios de mayúsculas o ampliación de enumeraciones cerradas.

Comentarios, identificadores nominales y cadenas conservan sus contratos propios. Una forma reservada citada dentro de una cadena es contenido. Traducir formas constitutivas no autoriza traducir datos del dominio. El perfil léxico de identificadores conserva su estatuto separado.

## 3. Paridad y procedencia

El par sintético emplea b=4 y N=16, vector plano ordenado, codominio neutro y una operación Evaluate. Es un testigo de compilación; no constituye dominio, soporte ni carga representativa. No fija tamaño predeterminado para SV.

La prueba futura comparará íntegramente objects() y operations(), variantes, cardinalidad, campos, nombres, referencias, valores y orden. Las anclas previas de ORACULO_CANONICO.json evitan aceptar sin más dos resultados igualmente incorrectos; no sustituyen el cotejo completo ni definen otra IR.

source_file y source_sha256 se verificarán contra el nombre y bytes originales de cada fuente. Sólo se excluyen de la igualdad semántica entre archivos distintos: permanecen en la evidencia. No se borran otros campos, normalizan cadenas ni ordenan posiciones. La coincidencia de Evaluate no acredita evaluación productiva ni paridad visual. K1-T conserva su limitación vigente.

Un comentario puede cambiar la huella sin cambiar significado. En Rust puede afectar líneas o documentación generada; no se promete identidad binaria por equivalencia documental.

## 4. Construcción y aceptación

Las entradas públicas compile_svp, compile_svp_profile y compile_svp_assembly inspeccionadas aplican conformidad de dominios cerrados, bienformación general, de transición/datos y de contexto antes de devolver un programa aceptado. Esta constatación estática no ejecuta cada guarda.

frontend es interno. IrProgram conserva campos privados y getters de lectura. ir::construction y sus constructores son pub(crate), accesibles dentro del núcleo. Esos constructores internos no validan por sí solos: la vía pública aplica controles posteriores. SourceUnit::new crea descriptor de fuente, no una IR aceptada.

El ensamblaje analiza cada unidad con su perfil hasta EOF, reúne objetos y operaciones en orden y valida globalmente. No concatena texto ni permite repartir una producción entre archivos. Su identidad incorpora fuentes, nombres y perfiles; no equivale a la huella de un archivo individual.

Una biblioteca SV puede ofrecer funciones y métodos; una macro, si se justifica, debe expandirse a vías con las mismas obligaciones. La forma de invocación no concede autoridad ni crea una primitiva semántica. Biblioteca, macros y sedes se decidirán en BIS-03 y se materializarán justificadamente en BIS-04. No se afirma su existencia por describirlas.

Las barreras externas requieren consumidores correctamente enlazados con sv_core, controles positivos y diagnóstico de privacidad. compile_fail por dependencia ausente o sintaxis ajena no prueba la restricción pretendida. Las garantías de tipos tampoco sustituyen la validación de contenido recibido en ejecución.

## 5. Documentación ES/EN

Se aplica DOCUMENTACION_RUST_ES_EN_v1: conservar condiciones, negaciones, cuantificadores, unidades, límites, efectos y errores. API, identificadores, versiones y códigos no se traducen arbitrariamente. Un ejemplo ejecutable se escribe una vez con explicación bilingüe y declara perfil si es SVP.

Comentarios ordinarios y documentación ejecutable requieren comprobaciones diferentes. Los doctests afectados deben ejecutarse. Su compilación no prueba equivalencia entre párrafos. El estímulo conforme y el mutante que elimina una reserva permiten revisión documental discriminante, con resultado y alcance propios. No se declara bilingüe todo el código existente.

## 6. Actividad de IA

El agente que selecciona perfil, genera fuente o parchea código debe conservar servicios, conectores, herramientas, invocaciones, scripts exactos, entradas, estados previo y posterior, parche, salidas y errores. El perfil declarado se coteja con la llamada efectiva y el efecto observado. Se mantienen las obligaciones de trazabilidad ya establecidas; una afirmación de equivalencia no es prueba de ejecución.

Esta preparación no ejecuta modelos ni demuestra resistencia a una evasión. Define el contraste posterior con herramientas y efectos reales. La confianza de construcción autoriza avanzar con comprobaciones ordinarias; no abre otra campaña de selección del constructor.

## 7. Banco, incidente de entorno y salida

Veinte escenarios: seis positivos y catorce negativos. Once entradas literales: siete SVP, dos consumidores Rust para rechazo por privacidad y dos estímulos documentales. Unicode, ensamblaje, macros, doctests adversos y actividad real requieren materialización adicional según cada caso. Observados nulos.

La lectura del checkout se completó antes de una desconexión del servidor de ejecución. El intento de crear el script local no obtuvo confirmación y no se ejecutó el preparador Python. La preparación y el cotejo documental efectivos se realizaron mediante JavaScript y el conector GitHub; se conserva el código utilizado. No se atribuye al entorno una ejecución Rust o Python que no realizó. La sincronización del checkout queda pendiente hasta su recuperación.

El cotejo auxiliar comprueba inventario, referencias y cambios explícitos de entradas; no interpreta SVP como sustituto de Rust ni prueba privacidad o guardas. Los ensayos posteriores registrarán cargo/rustc, destino, perfil de compilación, comandos, entradas, salidas y causa discriminante según el criterio C05.

C12 es la última familia numerada preparada. Sigue consolidar cobertura y pendientes C01–C12 en BIS-02 y preparar decisiones de sede BIS-03. Completar familias documentales no cierra BIS-02 ni el workflow. C01 mantiene su evidencia nativa; los otros bancos no se convierten en ejecutados. El original conserva dos escenarios ejecutados y veintidós pendientes. GUI diferida en S24.
