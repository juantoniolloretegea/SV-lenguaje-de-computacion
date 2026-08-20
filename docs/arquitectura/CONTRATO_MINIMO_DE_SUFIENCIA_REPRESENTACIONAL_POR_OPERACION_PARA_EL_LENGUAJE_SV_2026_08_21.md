# Contrato mínimo de suficiencia representacional por operación para el Lenguaje SV

**Fecha:** 21/08/2026  
**Estado:** contrato técnico de FFL-E  
**Ámbito:** dominios especializados, agentes, consultas e interfaces

## 1. Objeto

Este documento fija el contrato mínimo que deberá respetar el Lenguaje SV cuando una operación se ejecute sobre una representación que conserva sólo una parte de la información de un estado de dominio.

El contrato distingue cuatro cuestiones que no deben confundirse:

- el valor ternario de un parámetro;
- la salida tipada producida por una célula;
- la información conservada por una representación;
- y la posibilidad de ejecutar exactamente una operación a partir de esa representación.

La insuficiencia representacional pertenece al cuarto apartado. No constituye un nuevo valor ternario ni modifica la semántica de `U`.

## 2. Espacio de estados realizables

Para un dominio declarado `D`, se denomina `X_D` al conjunto de estados que pueden constituirse de acuerdo con sus reglas de captura, admisibilidad, ternarización y demás restricciones materiales aplicables.

La pertenencia de un vector a `Tri^n` no demuestra por sí sola que el vector pertenezca a `X_D`.

Toda afirmación fuerte sobre pérdida de recuperabilidad en un dominio especializado deberá utilizar estados cuya pertenencia a `X_D` esté acreditada por evidencia admisible para ese dominio.

## 3. Estratificación declarada

La división de los parámetros en capas, estratos o grupos semánticos no se deduce únicamente de `n = b²`.

Cuando una operación dependa de esa estructura, el dominio deberá declarar al menos:

- la identidad de cada parámetro;
- su posición en la célula;
- su pertenencia al estrato correspondiente;
- y la referencia semántica necesaria para impedir que parámetros no intercambiables sean tratados como equivalentes por una agregación.

La existencia de dos parámetros en el mismo estrato no autoriza a permutarlos salvo que el dominio declare expresamente la simetría o intercambiabilidad pertinente.

## 4. Representaciones declaradas

Una representación de un dominio se modela conceptualmente mediante una aplicación desde el espacio realizable del dominio:

`F_j : X_D -> R_j`

con una especificación equivalente a:

`RepresentationSpec = { domain, output_type, mapping, loss_kind }`

con:

- `domain`: dominio cuyo espacio realizable es `X_D`;
- `output_type`: tipo `R_j` de la salida;
- `mapping`: aplicación determinista `F_j : X_D -> R_j`;
- `loss_kind`: `Injective` o `Lossy`.

La declaración `Lossy` informa de que la representación puede identificar estados distintos. No determina por sí sola qué operaciones dejan de ser recuperables.

Una reducción entre dos niveles no forma parte de `RepresentationSpec`. Las reducciones pertenecen a la cadena que relaciona representaciones ya declaradas.

Si `R_j` es un codominio terminal de una célula, su tipo es el codominio declarado por esa célula. No se identifica por defecto con `Tri`. Cualquier aplicación posterior desde un codominio terminal a `Tri` constituye una transducción separada y debe declararse como tal.

## 5. Cadena de representaciones

Una cadena finita de representaciones se modela conceptualmente como:

`RepresentationChain = { domain, levels, reductions }`

con niveles ordenados

`F_0, F_1, ..., F_m`

y reducciones deterministas

`r_j : R_j -> R_(j+1)`

tales que

`F_(j+1) = r_j ∘ F_j`.

La numeración pertenece a la cadena declarada. No establece un orden universal entre todas las representaciones posibles del sistema.

## 6. Recuperabilidad exacta

Sea `Q : X_D -> Y` una operación determinista con firma declarada.

`Q` es exactamente recuperable desde `F_j` cuando existe una aplicación `q_j : R_j -> Y` con firma compatible tal que

`Q = q_j ∘ F_j`.

La afirmación requiere una aplicación de recuperación explícita o una evidencia equivalente que permita verificar la igualdad funcional dentro del alcance declarado.

La mera correlación, aproximación estadística, inferencia probabilística o reconstrucción heurística no constituye recuperabilidad exacta.

## 7. Certificado de límite representacional

Cuando una cadena esté fijada, un certificado de límite representacional para una operación deberá identificar:

`RepresentationFrontierCertificate = { chain, operation, frontier_index, recovery, boundary_witness? }`

El certificado contiene:

- la cadena declarada;
- la operación;
- el mayor nivel desde el que se acredita recuperabilidad exacta;
- la aplicación de recuperación correspondiente;
- y, salvo que se trate del último nivel de la cadena, un testigo de pérdida en el nivel siguiente.

El identificador `RepresentationFrontierCertificate` evita utilizar en la IR la palabra `Resolution` para este objeto, ya que `resolve` y `ResolutionRecord` tienen en el Lenguaje SV una función distinta relacionada con la revisión de `U`.

## 8. Testigo de pérdida

Para un nivel `j < m`, un testigo suficiente de pérdida deberá contener dos estados `x, y` acreditados como realizables en `X_D` y satisfacer:

`F_(j+1)(x) = F_(j+1)(y)`

junto con

`Q(x) != Q(y)`.

El testigo demuestra que `Q` no puede recuperarse exactamente desde `F_(j+1)` dentro del espacio de estados y de la cadena declarados.

Una pareja de vectores sintácticamente válida pero no acreditada como realizable en el dominio no basta para sostener este resultado.

## 9. Relación con `AnalyticView`

La IR v0.2 ya exige que una `AnalyticView` declare si su codificación es inyectiva o con pérdida.

El presente contrato añade una distinción necesaria:

- `Injective` permite reconstruir el objeto de origen dentro de la codificación declarada;
- `Lossy` indica pérdida de información;
- la suficiencia para una operación concreta requiere además una afirmación de recuperabilidad relativa a esa operación.

Por tanto, una codificación con pérdida puede ser legítimamente suficiente para una operación y simultáneamente insuficiente para otra.

La declaración de pérdida y el certificado de suficiencia por operación cumplen funciones distintas y no se sustituyen entre sí.

## 10. Relación con consultas

Cuando una consulta dependa de una operación cuya recuperabilidad se haya certificado respecto de una cadena de representaciones, el contrato futuro de `QuerySpec` deberá poder declarar el requisito de representación aplicable.

Conceptualmente:

`RepresentationRequirement = { operation, chain, certificate }`

Si el certificado fija `frontier_index = l`, los niveles admitidos por ese requisito quedan derivados por la estructura de la cadena:

`accepted_levels = {0, ..., l}`.

`accepted_levels` no es un campo libre que pueda ampliarse por declaración del usuario. La propiedad de segmento inicial se deriva de `F_(j+1) = r_j ∘ F_j` y del certificado correspondiente.

Una consulta sólo podrá atribuirse recuperabilidad exacta si el `QueryContext` suministra una representación admitida por el requisito o información adicional expresamente declarada que forme parte de un nuevo certificado.

La ausencia de una representación suficiente no produce una respuesta ternaria `U`. Impide sostener la ejecución exacta de esa operación desde el contexto disponible.

## 11. Relación con interfaces

Una interfaz no hereda toda la información disponible en el estado de origen. Su compromiso se limita a lo que transmite de forma declarada.

Si una interfaz transmite

`H = phi ∘ F_j`,

la recuperabilidad aguas abajo deberá evaluarse respecto de `H`.

Si el receptor utiliza además información lateral `S`, la afirmación deberá declarar esa dependencia mediante una forma equivalente a

`Q = q(H, S)`.

No es válido atribuir a `H` una recuperabilidad que dependa en realidad de información no transmitida.

Este contrato no modifica `Connector`. Un `Connector` conserva su firma tipada específica. La regla se aplica al contenido informativo disponible a través de cualquier interfaz declarada, incluidos los casos en que un conector sea parte de ella.

## 12. Condición diagnóstica

El Lenguaje SV deberá distinguir entre:

- `U`, como valor ternario legítimo;
- una salida terminal perteneciente al codominio declarado de la célula;
- y la imposibilidad de ejecutar exactamente una operación porque la representación disponible no conserva información suficiente.

FFL-E reserva para la tercera situación la clase semántica:

`RepresentationInsufficientForOperation`

sin asignarle todavía un código numérico del catálogo de errores.

La asignación de código deberá realizarse únicamente cuando exista una modificación de especificación o implementación que pueda emitir esta condición de manera observable.

## 13. Relación con diagnósticos existentes

`UndeclaredLossyEncoding` y `RepresentationInsufficientForOperation` no son equivalentes.

La primera condición se refiere a una representación con pérdida utilizada sin declarar esa pérdida. La segunda se refiere a una representación cuya naturaleza puede estar correctamente declarada pero que no conserva las distinciones necesarias para una operación determinada.

Tampoco es equivalente a `StrongConclusionUnderInsufficientCoverage`: una representación puede ser insuficiente aunque todos los datos de entrada estén presentes y no exista ninguna `U`.

## 14. Modelo mínimo previsto para la IR

Sin modificar todavía la IR v0.2, el contrato identifica como candidatos mínimos para una ampliación posterior:

- `ParameterInstance`, con identidad y posición materialmente definidas;
- `Stratification`, como partición declarada y verificable de parámetros cuando el dominio la utilice;
- `RepresentationSpec`;
- `RepresentationChain`;
- `RepresentationRequirement`;
- `RepresentationFrontierCertificate`;
- un testigo tipado de pérdida sobre estados realizables.

Estos objetos pertenecen al ámbito de uso, dominio, consulta y análisis. No requieren ampliar `Tri` ni alterar la estructura de `CellState`.

La salida terminal tipada y su relación con el evaluador de la célula se tratan separadamente en la adenda técnica de FFL-E sobre evaluación y codominios de salida.

## 15. Requisitos de bienformación que deberá preservar una ampliación posterior

Una ampliación de especificación o implementación sólo será válida si garantiza, como mínimo:

1. que toda `RepresentationSpec` declara una aplicación desde `X_D` a su tipo de salida;
2. que todas las representaciones de una cadena pertenecen al mismo dominio declarado;
3. que las reducciones entre niveles son compatibles por tipos;
4. que cada reducción satisface la igualdad declarada `F_(j+1) = r_j ∘ F_j`;
5. que el índice certificado pertenece a la cadena;
6. que la aplicación de recuperación tiene dominio y codominio compatibles con la representación y la operación;
7. que todo testigo negativo utiliza estados acreditados como realizables;
8. que la igualdad de representación y la desigualdad de salida del testigo quedan verificadas o respaldadas por evidencia explícita;
9. que ninguna insuficiencia representacional se degrada a `U`;
10. que una interfaz no recibe atribuciones superiores a la información transmitida;
11. que la información lateral utilizada por una recuperación queda declarada;
12. que la serialización conserva de forma determinista todas las referencias del certificado.

## 16. Prohibiciones

Queda excluido de este contrato:

- inferir automáticamente un límite representacional por el mero índice de una representación;
- tratar una agregación con pérdida como equivalente al estado completo;
- reconstruir etiquetas eliminadas mediante aproximación y presentarlas como recuperación exacta;
- fabricar `U` para representar una insuficiencia de la consulta;
- asumir que dos dominios con el mismo perfil de límites representacionales comparten semántica;
- utilizar prevalencia, probabilidad o rendimiento estadístico como sustituto de la igualdad funcional exigida por el certificado;
- identificar una salida terminal tipada con `Tri` sin una transducción explícita;
- alterar la semántica de `resolve` o `ResolutionRecord` para alojar este mecanismo.

## 17. Compatibilidad con el estado vigente

El contrato no atribuye ejecución nueva a la gramática v0.1, la IR v0.2 ni a la implementación actualmente publicada.

Su efecto inmediato es fijar el alcance técnico de FFL-E: delimita qué deberá conservar una futura ampliación del lenguaje cuando incorpore representaciones, consultas e interfaces capaces de expresar y verificar suficiencia por operación.

La transición a sintaxis, objetos IR ejecutables, validación y pruebas deberá realizarse en una fase posterior y con versión explícita de las especificaciones afectadas.
