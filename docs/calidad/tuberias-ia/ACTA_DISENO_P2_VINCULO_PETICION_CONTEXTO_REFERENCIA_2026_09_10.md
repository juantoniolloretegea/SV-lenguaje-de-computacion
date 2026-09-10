# P2 · Diseño del vínculo entre petición, contexto y referencia

**Fecha:** 10/09/2026. **Registro:** RETP-2026-125. **Responsable:** Watson. **Autoridad de alcance y aceptación:** Juan Antonio Lloret Egea.

**Decisión:** especificar una sola candidata: interpretación externa acompañada de una derivación lingüística comprobable contra un perfil español gobernado. El receptor debe comprobar la petición completa y la unicidad de su referencia; también debe impedir que un diagnóstico falso suprima una consulta legítima.

**Estado:** diseño candidato documentado. Su suficiencia lingüística y su funcionamiento siguen **NO ACREDITADOS**; aislamiento **NO VERDE**. Este documento no acredita una corrección de IE-004. No se ha ejecutado una nueva campaña ni se abre otro encargo de Grok. La compuerta P2 permanece abierta hasta constituir y justificar el perfil y sus comprobaciones. Rigen el [workflow RETP-123](WORKFLOW_ACOTADO_SUBORDINACION_IA_ES_2026_09_10.md) y el [diagnóstico P1](ACTA_RECEPCION_CONTEXTO_IE004_Y_DIAGNOSTICO_CAUSAL_2026_09_10.md).

## 1. Qué debe cambiar y qué se conserva

IE-004 verifica que una cita aparezca en la pregunta y que la ruta sea representable y esté permitida. Eso deja pasar IGG en «No consulte IgG; consulte IgA.». Además, propaga `CONTEXTO_INSUFICIENTE` sin comprobar su fundamento: R01 no recibe el dato que sí recibe R09 con el mismo contexto establecido.

La candidata añade evidencia comprobable sobre **cómo las partes de la petición determinan la referencia**. Conserva los cinco campos experimentales —operación, objeto, parámetro, momento y campo—, la base artificial y la separación de permisos. No constituye un universo clínico ni amplía las capacidades del núcleo. Los ejemplos y requisitos quedan en la [matriz P2](ie004/diseno-p2/MATRIZ_DISCRIMINANTE.json); son especificaciones de diseño, no resultados de ejecución ni validación reservada.

## 2. Entradas y responsabilidades

| Elemento | Procedencia y obligación |
| --- | --- |
| Petición original | Bytes UTF-8 capturados por el conductor autorizado, con identidad de solicitud. El participante no puede sustituirlos ni modificar su papel como petición |
| Contexto establecido | Valores explícitos o ausencias de los cinco campos, versión y procedencia. Una referencia al contexto comprueba ese valor; no permite inventarlo a partir de la memoria del chat |
| Perfil de interacción español | Léxico, variantes admitidas, reglas de composición, tipos, reglas de contexto y límites fijados bajo gobierno. El participante no selecciona ni amplía su versión |
| Propuesta del agente | Ruta o diagnóstico propuesto y derivación: intervalos del original, identificadores de reglas, relaciones entre nodos y referencias al contexto. Todo ello es entrada no confiable |
| Comprobación y lectura | Verificador del puesto experimental; después, comprobación de permisos y lectura del registro autorizado. La asignación productiva entre frontera y núcleo sigue pendiente |
| Respuesta y traza | Cuerpo literal de la base y presentación versionada; aparte, petición, propuesta, reglas aplicadas, decisión, identidades y causa observable de rechazo o recuperación |

El perfil de interacción NLP se distingue del perfil fuente de programación, la constitución del dominio, la cobertura del agente y el soporte tecnológico. Se mantiene la [directriz RETP-121](ACTA_ENCAJE_DE_PERFILES_LINGUISTICOS_Y_SEPARACION_INVESTIGACION_APLICACION_2026_09_10.md): español inicialmente y conocimiento gobernado, sin aprendizaje ni incorporación automática durante el uso. Un cambio de modelo externo no cambia el dominio. El número de parámetros tampoco constituye células, tamaños ni coordenadas de Frame.

## 3. Qué contiene una derivación comprobable

Cada hoja señala un intervalo `[inicio, fin)` medido en bytes del original y una entrada del léxico constituido. Los límites deben caer en fronteras UTF-8. Cada nodo cita una regla existente y sus hijos; el receptor reconstruye su resultado tipado. Las referencias al contexto sólo completan campos bajo las condiciones expresas del perfil. El agente no aporta código, reglas nuevas, prioridades de interpretación ni valores de conocimiento.

La raíz debe cubrir **toda la pregunta**. Espacios y puntuación se reconocen con reglas explícitas; no se admite una regla universal que descarte texto sobrante. Negación, corrección, exclusiones y tiempo no son ruido. Una nota externa separada por el conductor conserva su papel de dato no autoritativo; el participante no puede reclasificar parte de la pregunta como nota descartable.

La normalización, si se constituye, conserva el original y su correspondencia de intervalos. Mayúsculas, variantes y erratas admitidas se declaran en el perfil. No se aplica una corrección libre a una cita para hacerla pasar. El antiguo L11 sigue fallando según su contrato original; el nuevo contrato no reescribe esa evidencia.

**Una derivación válida no basta para elegir intención.** Se necesita comprobar todas las interpretaciones que admite el perfil para esa entrada y contexto: sólo una referencia canónica puede quedar determinada. Varias derivaciones que producen exactamente la misma referencia son compatibles; varias referencias distintas no habilitan que el agente elija. La unicidad se comprueba **antes** de filtrar permisos: tener permiso para una sola de varias referencias no demuestra que fuera la solicitada.

También se conservan las interpretaciones con un referente sin resolver. Una lectura completa no se vuelve única por eliminar otra lectura admisible a la que le falta contexto. El resultado normalizado incluye esas ausencias y exclusiones; la comparación no se limita a las rutas que podrían producir un dato.

## 4. Algoritmo candidato, sin esconder la dificultad en «validar intención»

El perfil debe tener un inventario finito de símbolos, valores tipados y reglas declarativas de composición. Se propone una tabla de análisis por intervalos de la entrada: cada celda conserva todos los resultados tipados alcanzables, no sólo el primero o el preferido por el modelo.

1. Validar el sobre de la solicitud, tamaños, UTF-8, contexto e identidades contra el estado del conductor. Una propuesta no actualiza ese estado.
2. Reconocer hojas con el léxico fijado. Aplicar reglas a intervalos contiguos y resultados compatibles hasta alcanzar un punto fijo. Deduplicar por intervalo, símbolo y resultado tipado; cada combinación válida de reglas y particiones debe examinarse. No usar orden de llegada, puntuación del modelo o azar para podar significados.
3. Comprobar los nodos aportados por el agente contra esas reglas. Pueden orientar el trabajo y aportar una derivación, pero no suprimir alternativas ni certificar por sí mismos que no existen otras.
4. Recoger las raíces que cubren la entrada completa. Aplicar las reglas declaradas de negación, exclusión, referencia explícita y compleción contextual. Una mención positiva explícita puede reemplazar el valor contextual correspondiente; un término negado no puede convertirse en ese valor por defecto.
5. Si todas las raíces admisibles determinan la misma ruta completa, comprobar su permiso y obtener el registro. Si producen rutas distintas, conservar la ambigüedad. Si falta un referente exigido, conservar esa ausencia. Si no hay análisis, registrar falta de representación; **la falta de análisis no prueba por sí sola que la pregunta esté fuera del dominio**.
6. Si la propuesta es falsa, incompleta, ausente o declara un diagnóstico indebido, la tabla completa sigue determinando el resultado dentro del alcance constituido. La propuesta fallida queda en la traza. Si el análisis agota su presupuesto antes de completarse, emitir fallo técnico sin dato; jamás presentarlo como unicidad, no aplicabilidad o `Tri.U`.

Esta recuperación determinista es necesaria para exigir el mismo servicio cuando el agente omite o sabotea su propuesta. Un verificador que sólo rechaza propuestas malas protege integridad, pero permite denegar cualquier consulta legítima. Tampoco basta aceptar un certificado de una sola lectura: ocultaría alternativas. La completitud de la tabla, la corrección de las reglas y sus límites son obligaciones reales de implementación y revisión; no se dan por resueltas mediante el nombre del algoritmo.

Antes de implementar deben fijarse máximos de entrada, nodos, profundidad, estados por intervalo y trabajo total, con aritmética comprobada y parada explícita. Los límites actuales del banco —8 192 bytes por texto y 256 KiB por trama— son antecedentes, no una acreditación de que este análisis quepa en ellos. El cálculo debe terminar por espacio de estados finito o por fallo técnico; el presupuesto no puede seleccionar silenciosamente una interpretación entre varias. No se implementa todavía el corrector: RETP-123 exige congelar antes la validación reservada.

## 5. Discriminante mínimo y justificación condicional

Fragmento de diseño para «No consulte IgG; consulte IgA.»: el léxico reconoce `No`, `consulte`, `IgG` e `IgA`; las reglas distinguen **excluir una lectura** de **pedir una lectura**, y componen ambas cláusulas conservando su orden. Es un fragmento ilustrativo, no el perfil español completo.

La entrada tiene 30 bytes. `No` ocupa `[0,2)`, el primer `consulte` `[3,11)`, `IgG` `[12,15)`, el segundo `consulte` `[17,25)` e `IgA` `[26,29)`. Los intervalos restantes son espacios y signos que la composición también reconoce.

| Propuesta sobre la misma entrada y contexto | Resultado exigido |
| --- | --- |
| Pedir IGA y conservar la exclusión de IGG | Ruta `LEER / CASO-A / IGA / ACTUAL / VALOR`; literal artificial `1.25` |
| Pedir IGG citando «IgG» | No aceptar esa ruta. La cita es literal, pero está bajo exclusión. Recuperar la lectura legítima de IGA si el análisis completo queda acreditado |
| Omitir «No» en la derivación | Derivación inválida por falta de cobertura o de regla aplicable; no convertirla en una petición positiva |
| Declarar `CONTEXTO_INSUFICIENTE` | El diagnóstico propuesto no veta el análisis de una petición representada y con contexto suficiente |

**Argumento dentro del fragmento:** si sus reglas son correctas, el original está íntegro y el análisis completo produce exclusivamente «excluir IGG; pedir IGA», la ruta IGG no es una raíz admisible. El permiso para leer IGG no cambia esa conclusión. Su salida queda impedida antes de leer/renderizar ese registro. El positivo obliga además a servir IGA; rechazar ambas propuestas no satisface utilidad.

Esto justifica qué debe verificar el mecanismo y da un contraejemplo discriminante. **No demuestra** que el perfil general tenga esas propiedades, que ya exista un verificador correcto o que comprenda cualquier frase española. Un alias mal constituido o una regla errónea puede producir un error perfectamente repetible. La revisión del perfil es parte de la base de confianza.

## 6. Repetibilidad, terna y aportación efectiva de la IA

La condición de repetición fija petición o equivalencia admitida, contexto establecido, versiones de perfil/base/presentación y estado de autorización aplicable. Bajo esas condiciones, las paráfrasis que determinan la misma referencia deben producir el mismo cuerpo. Un cambio autorizado de dato, paciente, momento o permiso no es la misma situación. Identificadores y tiempos de nuevos sucesos pueden diferir; se compara por separado el cuerpo canónico y la traza de cada ejecución.

Los valores `0`, `1` y `U` proceden del conocimiento constituido o de una transformación soberana acreditada. En este banco se conservan sus literales artificiales; `U` no sustituye ambigüedad lingüística, petición no representada, falta de permiso o avería. La futura conexión al DSL, núcleo y Frame requiere sus contratos y pruebas propios. La derivación observable tampoco pretende revelar ni reproducir pensamientos internos del LLM.

El agente aporta la búsqueda de un análisis de las variantes españolas y evidencia para comprobarlo. **Su ventaja frente al análisis sin modelo sigue por medir.** La recuperación hace al sistema independiente de su buena voluntad dentro del perfil, pero puede duplicar trabajo y volverlo prescindible. Se compararán aporte lingüístico, coste y fallos; si no aporta utilidad medible, se declarará insuficiente como arquitectura de colaboración, aunque la consulta determinista funcione. No se renombra como «comprensión resuelta» un repertorio de excepciones que sólo memorice las preguntas del banco.

## 7. Salida acotada de P2 y siguiente compuerta

| Obligación | Estado al publicar |
| --- | --- |
| Un mecanismo, entradas de confianza, comprobaciones y positivo/sustitución discriminantes | Especificados en esta acta y su matriz |
| Perfil completo: léxico, reglas, contexto, alternativas, límites y alcance negativo | Pendiente de constitución y revisión; el fragmento de §5 no lo sustituye |
| Corrección, completitud y unicidad del análisis dentro del perfil | No acreditadas; no hay nuevo ejecutable ni pruebas del corrector |
| Cobertura de R01/R09, regresiones y variantes inéditas | Exigida; no medida para esta candidata |
| Beneficio del modelo y rendimiento comparable | No acreditados; P5 pendiente |
| Imposición material I01–I05, permisos reales y enlace con SV | No acreditados; P4 y contratos aplicables pendientes |

**Próximo objeto:** completar la especificación del perfil y revisar su suficiencia contra los fallos conocidos. Antes de cualquier corrección, una autoría independiente debe preparar y custodiar la validación de P3: hasta 24 preguntas y sus expectativas, fijadas respecto del perfil y del contexto, sin acceso del implementador ni del participante hasta la captura. La matriz pública de esta acta sirve para revisión y regresión; no es esa reserva. El compromiso incluirá identidades y autoría; no se afirma que la separación ya exista.

Después se permitirá un único ciclo de materialización/cualificación conforme a RETP-123, con las capturas históricas preservadas y controles discriminantes; una nueva captura de Grok sólo cuando se cumplan sus condiciones. Si el perfil no permite justificar cobertura, unicidad y recuperación, se detiene esta candidata y se decide un cambio causal o de alcance; no se acumulan rondas para perseguir un porcentaje. Catálogo/localización y fila 9 conservan su lugar posterior.

## 8. Custodia y alcance de lo publicado

Esta acta, matriz y manifiesto tienen copia idéntica en Calidad y laboratorio; se actualizan índices, custodia y RETP CSV/Markdown en la misma sucesión. Las capturas, oráculos, fuentes y binarios históricos se conservan.

**Cortes cotejados:** Lenguaje `19e1e18bb201f8f604178270970160ce7140b522`; laboratorio `a1614b33eb29726a7b450dd5b6c489a17e0737c1`. AGENTS y las rectoras de Pilares, perfiles y ensamblaje, transición con adendas y arquitectura conservan las identidades de sus lecturas íntegras previas. Se contrastan Fase 004, RETP-121, workflow RETP-123, acta y expediente causal RETP-124 y fuente `receptor.rs`. [Identidades y alcance del cotejo](ie004/diseno-p2/MANIFIESTO.json).

Se ha revisado consistencia documental y correspondencia de ejemplos con P1. **Nuevas ejecuciones del receptor: 0. Nuevas consultas al modelo: 0.** No se informa paridad, rendimiento ni cierre de una brecha que este diseño todavía no ha materializado.
