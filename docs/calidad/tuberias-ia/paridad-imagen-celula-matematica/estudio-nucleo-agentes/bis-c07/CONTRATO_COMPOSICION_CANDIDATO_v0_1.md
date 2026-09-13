# Contrato candidato de composición entre células de dimensiones distintas

**Versión 0.1 · S22 · BIS-02/C07 · 13 de septiembre de 2026 · Juan Antonio Lloret Egea y Watson**

## 1. Fundamento y estatuto

La célula composable conserva vector, codominio, interpretación de salida y rol estructural. Los Fundamentos §7 rechazan una operación binaria universal para cualquier par de células. La Frontera A.6–A.10 y D.2 exige relación semántica previa, distingue serie, compuerta, supervisión y constructor organizativo, y conserva sus diferentes grados de cierre.

C07 concreta un banco sintético transversal con células de 16 y 25 posiciones. No incorpora parámetros de inmunología, neumología o ciberseguridad; los antecedentes médicos conservan su condición de ejemplos. Tampoco acredita por sí solo generalidad sobre un dominio real: el contraste no médico competente sigue siendo exigible antes de esa conclusión.

Los tamaños proceden del montaje de prueba, no de una decisión productiva. Cada vector permanece plano, ordenado y de dimensión propia n=b². La composición no concatena sus coordenadas en una célula de 41 posiciones ni exige rellenar, truncar o redimensionar una de ellas. C02 mantiene la admisión por versión del soporte. El mínimo (9,3) no se utiliza como tamaño predeterminado.

## 2. Operaciones y condiciones

| Familia | Obligación del contrato | Resultado que no se presume |
| --- | --- | --- |
| Serie por puente | Fuente y destino identificados; posición del destino en su BridgeSet; conector total desde el codominio fuente a Tri; relación e interpretación constituidas | Transmisión ejecutada por el mero hecho de declarar una arista |
| Compuerta | Participantes y orden; codominios por posición; tabla total y determinista; salida tipada y relación previa | GateResult.output calculado por admitir gate |
| Dominancia homogénea | Compatibilidad ordinal, semántica y de roles; orden explícito | max/min universal sobre etiquetas o discriminantes |
| Supervisión | Meta-célula, objeto supervisado y efecto del contrato, con sus capacidades reales | Veto efectivo por mostrar una etiqueta de veto |
| Comp | Arquitectura y relaciones organizadas conforme al alcance constituido | Firma cerrada o garantías fuertes no demostradas |

Este banco materializa fuentes para serie y compuerta. No implementa supervisión, criticidad, conflicto general ni Comp completo. La biblioteca puede ofrecer funciones y métodos, pero cada operación deberá imponer sus condiciones y conservar su resultado tipado; las macros no dispensan de esas guardas.

## 3. Serie sintética y oráculos

QA tiene 16 posiciones y QB tiene 25. El montaje directo declara QA→QB y utiliza el puente 17, además de una variante para el puente 25. La cota de la posición se aplica a QB; no se contrasta indebidamente contra las 16 coordenadas de QA. Una relación inversa QB→QA es otra operación, con su conector y posición 1 propios.

Phi17 recibe el codominio KA y declara A0→Zero, A1→One, AU→U. Es una correspondencia sintética explícita, no una ley universal ni una transducción observación→Tri. AU es una etiqueta de KA; su correspondencia con U procede de esta tabla. La salida fuente debe proceder de la evaluación admitida cuando el montaje se ejecute realmente. Los oráculos prueban por separado la sustitución para cada entrada hipotética de KA; no afirman que el vector adjunto haya producido esas tres salidas.

La futura realización debe conservar la fuente, la identidad del conector y la posición, producir el vector de destino según la tabla y preservar las demás coordenadas. `CoupledState` permite comprobar cambios sólo en puentes; esa restricción no demuestra por sí misma que el valor nuevo proceda del conector y salida declarados. La deuda de procedencia se mantiene explícita.

## 4. Compuerta y relación previa

El montaje de compuerta usa entradas ordenadas KA/KB y codominio KR. Se fijan las nueve filas del producto cartesiano. La interpretación sintética se documenta junto a ellas; no se infiere a partir de una escala numérica, ni se introduce un orden sobre U. Los oráculos de tabla son resultados esperados para etiquetas de entrada, no resultados ejecutados de evaluate o gate.

La realización inspeccionada comprueba totalidad, unicidad de filas, pertenencia y aridad, y los codominios posicionales de gate. Los campos de AdmissibilityTable/gate no incluyen una referencia tipada directa a SemanticRelation. La mera presencia de RGate en la fuente no prueba su ligadura con T ni la interpretación de sus restricciones. BIS-03 deberá decidir cómo imponer esa obligación aprovechando las ligaduras existentes donde corresponda, sin sustituir LIG/0.1 por el registro JSON del banco.

## 5. Estructura admisible y operación solicitada

Se distinguen dos controles. El compilador comprueba lo que representa y valida su ruta estructural. El consumidor de una operación constituida debe además exigir su contrato exacto, identidad, revisión, dirección y tabla. Un grafo inverso puede ser estructuralmente válido y, sin embargo, no satisfacer la operación directa solicitada.

Los casos C07-16/17 cambian contenido de relación o conector conservando sus nombres y compatibilidades. C07-18 utiliza los mismos bytes que el positivo inverso C07-04, pero se presenta bajo la expectativa independiente de la operación directa. La previsión estática es admisión estructural; el resultado contractual exigido es rechazo por contrato distinto. Esta distinción evita inventar un rechazo del compilador y luego confundirlo con protección existente.

El registro del conductor contiene referencias íntegras a los cuatro montajes positivos. Su comparación futura puede acreditar identidad de esos bytes dentro del ensayo, pero no autoridad, interpretación completa o ejecución algebraica. No debe recibirse como expectativa libre del proponente. No se declara que la igualdad exacta de todo un archivo sea la futura interfaz productiva; es el criterio explícito de este banco.

## 6. Banco y guardas

El banco contiene dieciocho variantes con fuentes SVP literales: cuatro positivas y catorce negativas contractuales. Once negativas prevén rechazo estructural; tres prevén estructura admisible pero contrato distinto. Todos los resultados observados permanecen nulos. Los oráculos de transmisión incluyen tres vectores completos de destino; la compuerta incluye nueve filas completas.

Los negativos estructurales cubren codominio de conector, posición discordante, puente no declarado, actualización fuera de puente, mapping incompleto o duplicado, ciclo, concurrencia Simple, inversión de gate, salida ajena y tabla incompleta. Se especifica la guarda que cada entrada debe alcanzar. Un rechazo previo no acredita una guarda posterior; la fase Rust deberá conservar diagnóstico y orden efectivos.

No se cambia a General para eludir la concurrencia Simple: la falta de ConflictOperator material conserva su deuda. No se convierten rechazos técnicos en U ni se reconstruyen vectores válidos a partir de entradas inválidas. Los tamaños y tablas sintéticos no autorizan extensiones de dominio.

## 7. Sede, realización y cierre pendiente

BIS-03 recibirá: guardas estructurales existentes; enlace de relación exacta e interpretación pendiente de imposición; procedencia material del puente; productor de salida de compuerta; y límites de conflicto/supervisión/Comp. Una necesidad de modificar IR deberá justificarse por una distinción requerida que no pueda conservarse con el contrato disponible. No se amplía el núcleo sólo por comodidad del consumidor.

Antes de ejecutar se fijarán el receptor, el productor correspondiente, versiones, presupuesto y observador independiente. El criterio Rust C05 exige distinguir preparación auxiliar Python, compilación y validación funcional. El observador deberá detectar sustitución de relación, inversión de participantes, mapeo distinto y escritura fuera del puente. Las pruebas materiales de operador no se cierran por admitir el programa y serializar su IR.

C07-P/N originales permanecen pendientes. Continúa C08: conservación de Tri.U válido y separación de fallos técnicos de representación. S22/BIS-02 sigue en ejecución; S24 conserva la GUI para después del Bis, catálogo y cierre de fase.
