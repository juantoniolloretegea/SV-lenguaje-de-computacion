# Presentación sin pérdida de contenido: formato acotado desde el recibo

**SV · RETP-2026-150 · 12/09/2026**  
**Estado: perfil candidato de espacios comprobado en Rust nativo. Sin promoción productiva.**

La candidata permite cambiar espacios y saltos de línea fuera de las cadenas de texto, conservando íntegramente los valores, textos, claves y orden. Rechaza quitar una negación, omitir el alcance o sustituir el dato, aunque el documento resultante siga siendo JSON válido. Los 24 originales del lote público permanecen recuperables.

## 1. Problema y alcance

RETP-149 comprobó identidad literal entre el cuerpo de un recibo y una propuesta de entrega. Por su contrato, una presentación con sangría diferente no es una entrega literal idéntica. Eso no es un defecto de aquella frontera: es una operación distinta que necesita una regla explícita.

El contrato de suficiencia representacional, §§4–6 y 10–11, exige declarar qué conserva una transformación y qué información utiliza la recuperación. El acta de perfiles y ensamblaje distingue representación, significado de dominio y soporte. De esos contratos no se deduce automáticamente que cualquier formato o imagen sea equivalente.

Esta continuación introduce **un perfil experimental nuevo y limitado**, `IE004-PRESENTACION-ESPACIOS/1`. La regla admite exclusivamente diferencias en los cuatro separadores JSON —espacio, tabulación, retorno de carro y salto de línea— fuera de cadenas. No incorpora significado médico ni una regla de identificación de volcanes. Tampoco constituye la interfaz profesional.

## 2. Qué se comprueba

El objeto de referencia es una `EntregaLiteral` protegida de la candidata anterior. El conductor conserva la elección del recibo; el texto propuesto no escoge otro recibo ni puede declarar un perfil más permisivo.

La operación comprobada conserva la **secuencia exacta de bytes significativos de JSON**: todos los bytes de cadenas y todos los bytes ajenos a los cuatro separadores permitidos. Por ello se conservan también el orden de claves, el orden de los arrays y la escritura de escapes. No se reserializan valores ni se ordenan claves para hacerlos coincidir.

Ejemplos del alcance:

| Cambio propuesto | Decisión |
|---|---|
| Añadir sangría y saltos entre los elementos | Admitido si el JSON sigue siendo válido |
| `8.40` → `9.40` dentro del dato | Rechazado |
| «no acredita» → «acredita» | Rechazado |
| Eliminar el campo de alcance | Rechazado |
| Añadir o quitar espacios dentro de un texto | Rechazado |
| Cambiar el orden de claves o de un array | Rechazado |
| Reescribir `"a"` como `"\u0061"` | Rechazado en este perfil, aunque otro contrato pudiera admitirlo |
| Convertir `true` en `t r u e` | Rechazado por el reconocimiento sintáctico previo |

La última fila es relevante: **eliminar espacios y comparar sin validar primero la sintaxis sería insuficiente**. Podría recomponer un token que el texto recibido no contiene válidamente. La candidata reconoce primero los dos documentos completos con el lector existente y después compara su contenido significativo.

## 3. Recorrido implementado en seis pasos

1. Se obtiene la entrega literal del recibo conservado por el custodio, conforme a RETP-149.
2. Se limita el cuerpo de referencia a 4096 bytes y la presentación propuesta a 16384 bytes.
3. Se reconoce la sintaxis de ambos textos mediante el lector JSON ya existente, con un presupuesto de comprobación de un millón de unidades. Se mantienen sus límites de profundidad, claves y tipos; no es un reconocedor de todo JSON posible.
4. Se recorren ambos textos sin crear un árbol ni reservar memoria dinámica en el nuevo comprobador. Los espacios dentro de las cadenas y todos sus escapes se conservan.
5. Sólo si coincide todo el contenido exigido se construye `PresentacionComprobada`. La vista conserva texto, referencia original, identidad de la entrega, perfil y trabajo contabilizado. Sus campos son privados y su vida está ligada a la entrega y al texto prestados.
6. El conductor de ensayo emite exclusivamente el texto de esa vista. En los rechazos ensayados no se emite cuerpo por stdout. La campaña conserva entradas, resultados y originales.

La prueba de presentación es una consulta inmutable. No inscribe por sí misma SUCESO ni `sv_core::Frame`, no concede autoridad profesional y no modifica el recibo. No añade retención de intentos de presentación dentro del custodio: los intentos de esta campaña quedan conservados en su evidencia. Una futura obligación de retención en servicio necesitará su enlace correspondiente.

[Comprobador Rust](presentacion.rs) · [Cambio de cinco piezas sobre RETP-149](CAMBIO_CANDIDATO.patch)

## 4. Resultado de la matriz

Una matriz con Rust 1.98.0, debug y release: **47 procesos, sin fallos inesperados ni correcciones causales**.

Por configuración:

- Pasan **52 pruebas anteriores y 10 nuevas**, 62 en total.
- Pasan **11 contrastes del conductor**: dos entregas válidas y nueve rechazos esperados.
- Se comprueban **24 presentaciones del lote público**, conservando los 24 originales y sus huellas esperadas anteriores. Estas posiciones reutilizan diez preguntas distintas; no acreditan 24 preguntas independientes nuevas.
- Compila un cliente externo válido. Se rechazan los intentos externos de fabricar la vista con campos privados y de usarla después de destruir el texto que presta.

Hay paridad de los 24 textos presentados, de los 24 cuerpos originales y de las salidas de los 11 contrastes entre las dos compilaciones. Las fuentes se fijaron antes de compilar y se cotejaron después. Los dos negativos de compilación por modo son resultados previstos, no defectos del ensayo.

[Resultado](RESULTADO.json) · [Evidencia completa](EVIDENCIA.json) · [Plan fijado](PLAN_FIJADO.json) · [Huellas de ejecutables y bibliotecas](REALIZACIONES.json)

## 5. Qué significa «reconstruir» en este paso

La presentación puede tener separadores diferentes. **No se afirma recuperar sus separadores originales a partir de ella sola.** Se comprueba la conservación de la secuencia de contenido definida por el perfil; el cuerpo original exacto se recupera del custodio y esa dependencia queda explícita.

Esto aplica la distinción del contrato: la información conservada por la representación y la información adicional disponible no son lo mismo. El resultado no es un certificado de suficiencia de dominio ni demuestra realizabilidad de estados médicos. Tampoco permite reconstruir pensamiento interno de una IA: se custodian y verifican operaciones y artefactos observables.

## 6. Límites que continúan abiertos

- **Imagen y comprensión humana:** no se han probado un renderizador, una pantalla, colores, gráficos, ocultación por estilos, recortes o comprensión del inmunólogo. El JSON con sangría es un testigo técnico del cambio de forma; no se propone como su interfaz final.
- **Pérdida de información:** este perfil no admite suprimir campos ni distinciones. Decidir qué pérdidas son suficientes para una operación requiere el contrato y los testigos de esa operación.
- **Fuente y autoridad:** una presentación fiel puede conservar una resolución de origen errónea. Aquí se confía en el productor y en la referencia seleccionada por el conductor. Se mantiene la autoridad del experto humano del dominio constituido y el alcance del agente consumidor.
- **Imposición material:** no se demuestra que todos los consumidores tengan que usar esta API ni que el host no pueda alterar la salida después. La escritura al destinatario no es atómica ni tiene confirmación de lectura.
- **Recursos:** el comprobador nuevo no reserva memoria dinámica, pero utiliza pila y tiempo de CPU. Los conductores y la preparación del ensayo sí reservan buffers. El límite de trabajo es contabilidad, no medida de CPU o RSS. El plazo externo es de 60 segundos por proceso; el volumen de salida del observador se coteja después de capturar. No se acredita aquí P5 ni una cuota acumulada de servicio para llamadas repetidas.
- **Identidad material:** la campaña conserva huellas reales de fuentes y binarios. Los conductores nuevos usan metadatos ficticios explícitos y no los presentan como autenticación del host.
- **Diagnóstico:** las causas Rust permanecen tipadas, incluida la causa anidada del lector. La CLI resume la familia de validación como `SINTAXIS`; esa presentación no cierra el diagnóstico estructurado productivo ni su localización.

## 7. Continuidad, causas y siguiente obligación

Esta es la pieza sucesora acotada de RETP-149 para un cambio de formato. Conserva el workflow V2. La candidata sigue siendo experimental: no modifica el núcleo productivo, la gramática, la IR, los dominios ni los permisos. No se abren reserva, modelos, P4/P5, agentes o interfaz profesional. No se transfiere por esta publicación una obligación nuclear a una biblioteca opcional ni se declara una garantía global porque la API haya pasado pruebas.

Las causas nuevas son `Limite`, `Sintaxis(Fallo)` y `ContenidoDistinto`. `Sintaxis(Fallo)` conserva la variante concreta del lector, incluidos sus límites y trabajo; la integración productiva deberá mantener esa distinción en vez de atribuir todos los casos a un error gramatical. Son insumos pendientes del catálogo, sin asignación de códigos numéricos nuevos ni traducción de los datos. No se convierten en U.

La siguiente obligación sigue siendo la correspondencia de una transformación de información o una representación visual con su operación constituida y el contenido efectivamente recibido. Esta prueba aporta un mecanismo de comparación para una relación declarada, no una autorización para admitir cualquier transformación. La seguridad integral, la localización y la aceptación productiva final continúan pendientes en sus compuertas.

## 8. Fuentes, cortes y reproducción

Lenguaje: `b8f0982f8c2183a71f8a32c12ab55e2e6148591b`, rama `main`. Laboratorio: `196b574f69d456bc31e81ae416d83513b1381730`, rama `lab/playground-sv-permanente`.

Se recuperaron las 63 fuentes de RETP-149 desde su cápsula y se verificaron sus tamaños y huellas. Se cotejaron once referencias rectoras y de continuidad; las lecturas completas anteriores de pilares, perfiles/ensamblaje y conformidad IMM conservan los mismos bytes. Se leyeron íntegramente el workflow V2 y el contrato de suficiencia representacional por operación. La adenda y el expediente previo mantienen sus relaciones; no se reabre quién concede autoridad ni se pide al autor el mecanismo técnico que ha encargado.

[Fuentes cotejadas](FUENTES.json) · [Cápsula de fuentes reproducibles](FUENTES_REPRODUCIBLES.json) · [Procedencia del compilador](COMPILADOR.json) · [Reproducción](REPRODUCIR.md) · [Manifiesto](MANIFIESTO.json)
