# SEC.0-M — Contrato abstracto de memoria, persistencia, recursos y continuidad

**Fecha:** 21/08/2026  
**Estado:** contrato arquitectónico cerrado  
**Ámbito:** Lenguaje SV — SEC.0-M  
**Antecedentes:** `CONTRATO_ABSTRACTO_DE_AUTORIDAD_CONSTITUCION_Y_GENESIS_SEC0_A_V2_2026_08_21.md` y `CONTRATO_ABSTRACTO_DE_DIAGNOSTICO_Y_FALLO_CERRADO_SEC0_D_2026_08_21.md`  
**Vigencia:** la cláusula de clonación, implantación y génesis de §7 debe leerse conjuntamente con [`ADENDA_CORRECTIVA_SEC0_M_CLONACION_IMPLANTACION_Y_GENESIS_2026_08_22.md`](./ADENDA_CORRECTIVA_SEC0_M_CLONACION_IMPLANTACION_Y_GENESIS_2026_08_22.md), que prevalece ante cualquier lectura incompatible.

## 1. Objeto

SEC.0-M fija las propiedades abstractas que deben conservar memoria, persistencia, recursos y continuidad para que una autoridad ya constituida y una comprobación ya gobernada no puedan ser falsificadas, resucitadas, bifurcadas, agotadas o reinterpretadas por retroceso de estado, clonación, vistas derivadas, fallos parciales, agotamiento de recursos o recuperación.

El contrato no selecciona sistema operativo, lenguaje de implementación, motor de almacenamiento, soporte físico, protocolo de coordinación entre réplicas ni plataforma concreta. Tampoco modifica gramática, IR, validador o catálogo diagnóstico.

## 2. Estado semántico y estado técnico

Se distinguen dos planos.

El **estado semántico** comprende los hechos, autoridades, habilitaciones, continuidades admitidas, resultados diagnósticos aplicables y demás objetos reconocidos por las reglas del SV.

El **estado técnico** comprende las representaciones materiales utilizadas para conservar, transportar, reconstruir, indexar o ejecutar esos objetos: archivos, páginas de almacenamiento, instantáneas, copias de seguridad, memorias intermedias, índices, réplicas, registros, colas, imágenes de proceso y resultados temporales.

Quedan excluidas las inferencias siguientes:

```text
estado técnico presente ⇒ estado semántico vigente
copia íntegra de bytes ⇒ continuidad legítima
historial local válido ⇒ continuidad vigente
índice que apunta a X ⇒ X posee autoridad
marca temporal mayor ⇒ X es la continuidad vigente
```

La materialización puede representar autoridad; no la crea.

## 3. Dependencias persistentes

Para toda decisión protegida `d` y contexto `C` se define abstractamente:

```text
PDep(d | C)
```

como el conjunto suficiente de dependencias persistentes necesarias para volver a justificar `d` después de un reinicio, recuperación o sustitución de proceso.

`PDep(d | C)` puede incluir, cuando proceda, la forma constituida `F`, la autoridad aplicable, el dominio gobernado `D_a`, `Req(F,e | C)`, evidencias admitidas, revocaciones, régimen de gobierno, antecedente de continuidad, resultados de comprobación, contratos de acumulación, datos de vigencia, traza de efectos e información necesaria para recuperación.

Si después de una restauración no puede reconstruirse y acreditarse el conjunto suficiente de dependencias, la decisión no puede reutilizarse como vigente. Una dependencia ausente no se sustituye por un valor por defecto que permita continuar.

## 4. Persistencia autoritativa y estructuras derivadas

Se distingue entre:

```text
AStore
```

como estado persistente del que puede depender una decisión autoritativa, y

```text
View(AStore)
```

como índice, memoria intermedia, resumen, tabla auxiliar o puntero derivado.

Toda decisión sobre vigencia, revocación, consumo, continuidad, cobertura negativa, acumulación o recuperación debe fundarse en un miembro acreditado de `PDep(d | C)`.

Una estructura derivada sólo puede:

1. ayudar a localizar o consultar una fuente autoritativa; o
2. actuar como fuente autoritativa para una decisión, en cuyo caso entra expresamente en `PDep` y asume las obligaciones de `AStore`.

No existe un régimen en el que una estructura se declare no autoritativa y, al mismo tiempo, su respuesta determine por sí sola una decisión autoritativa.

Si una estructura derivada contradice el estado persistente acreditado, prevalece la fuente autoritativa admitida. Si ésta no puede establecerse, la operación afectada queda técnicamente no verificable conforme a SEC.0-D.

## 5. Historial y continuidad vigente

Se distingue:

```text
ValidLocal(h)
```

—la historia `h` es internamente coherente y verificable— de:

```text
Current(h | I)
```

—`h` pertenece a la continuidad que una implantación `I` reconoce como vigente para una decisión determinada.

Debe cumplirse:

```text
Current(h | I) ⇒ ValidLocal(h)
```

pero no:

```text
ValidLocal(h) ⇒ Current(h | I)
```

Toda transición autoritativa persistente debe quedar ligada a un antecedente de continuidad o a una génesis admitida. La regla que admite una continuación debe estar constituida con anterioridad y no puede depender únicamente de un índice local, de `HEAD`, de la marca temporal mayor, del contador local mayor, del último elemento recibido o de la respuesta más rápida.

## 6. Retroceso de estado

Restaurar una representación técnica anterior no implica que todas sus autoridades, habilitaciones o decisiones sigan vigentes.

Una restauración no puede, por sí sola:

- borrar una revocación posterior;
- hacer reaparecer una autoridad consumida;
- restablecer un testimonio ya utilizado;
- reducir silenciosamente un contador de uso o acumulación;
- olvidar un efecto irreversible ya ejecutado;
- convertir una rama histórica en continuidad vigente.

Si todo el estado empleado para decidir continuidad puede retroceder conjuntamente a una copia antigua autoconsistente, un observador contenido exclusivamente en esa misma copia no puede distinguir necesariamente el estado vigente de una restauración antigua coherente. Por tanto, una garantía fuerte contra retroceso exige una referencia, testimonio o relación de continuidad que no pueda retroceder indistinguiblemente junto con el mismo estado que pretende verificar.

SEC.0-M fija esta exigencia; SEC.0-X deberá determinar cómo puede materializarse.

## 7. Clonación y autoridad consumible

La clonación crea otra representación material del mismo estado técnico, pero no multiplica por sí sola autoridad, titularidad, autorizaciones de un solo uso, continuidad vigente ni identidad de una implantación.

Si una copia debe convertirse en una implantación distinta, deberá existir la transición de constitución o génesis correspondiente conforme a SEC.0-A.

Una autoridad de un solo uso, un contador de consumo o cualquier estado cuya acumulación deba ser única no admite ejercicio automático si su registro de consumo puede clonarse o retroceder indistinguiblemente junto con la réplica que intenta ejercerlo.

Para admitir ejercicio automático debe existir una garantía gobernada que impida o haga no verificable un segundo consumo antes de producir el efecto y que dependa de una referencia, testimonio o mecanismo no indistinguiblemente clonable o retrocedible bajo el mismo fallo.

Una restricción de unicidad confinada al mismo estado conjuntamente clonable o retrocedible no constituye por sí sola esa garantía. Mientras SEC.0-X no materialice una garantía suficiente, no puede presumirse unicidad de consumo.

## 8. Bifurcación

Existe bifurcación cuando dos o más sucesores pretenden continuar desde un antecedente común y pueden ser localmente válidos.

La validez local no determina cuál es la continuidad vigente. La política puede exigir una única continuidad, admitir ramas gobernadas, permitir particiones por ámbitos disjuntos o exigir una transición posterior de reconciliación. La topología admitida y la regla de selección deben estar constituidas antes de utilizarlas para autoridad.

Si la política exige unicidad y no puede acreditarse cuál es la continuación vigente, la continuidad queda no verificable para los efectos que dependan de ella. No se elige una rama únicamente para mantener disponibilidad.

## 9. Cobertura persistente y ausencia

Toda estructura que afirme cubrir un conjunto de objetos autoritativos debe poder demostrar su relación con ese conjunto.

Sin una regla suficiente de cobertura no puede inferirse:

```text
x ∉ J ⇒ x ∉ AStore
```

Las decisiones negativas basadas en ausencia —por ejemplo, la inexistencia de una revocación— requieren una fuente cuya cobertura negativa esté acreditada. Un índice incompleto no demuestra ausencia.

## 10. Ligadura entre comprobación y efecto

SEC.0-D exige que una acreditación siga siendo aplicable en el instante material del efecto. SEC.0-M establece que un permiso referido a una revisión material `r` sólo puede utilizarse si, en el punto de compromiso, `r` sigue siendo la revisión aplicable o una regla gobernada demuestra que los cambios posteriores no afectan las ligaduras utilizadas por `Req(F,e | C)`.

Si esa continuidad no puede acreditarse, corresponde `D-N` y debe repetirse la comprobación sobre un estado aplicable.

SEC.0-M no impone una técnica concreta de coordinación; exige la propiedad de continuidad entre comprobación y efecto.

## 11. Fallos parciales y efectos externos

La ejecución material puede atravesar estados técnicos que no pertenecen a `Tri`. Debe poder distinguirse, según el alcance de la operación, entre intención preparada, efecto acreditado, resultado material no determinado y cierre acreditado sin efecto.

Si un efecto externo pudo ocurrir pero no puede determinarse si ocurrió, la recuperación no puede asumir ni éxito ni ausencia y no puede repetirlo automáticamente. Debe aplicarse una regla constituida de reconciliación, idempotencia, compensación o comprobación externa. Si ninguna permite decidir, el estado permanece técnicamente no verificable.

El hecho de que una solicitud haya sido enviada localmente no acredita que el efecto externo contratado haya ocurrido, salvo que el contrato defina precisamente ese envío como efecto terminal.

SEC.0-M no promete ejecución exactamente una vez frente a sistemas externos que no proporcionen soporte suficiente.

## 12. Persistencia previa y posterior

Para operaciones cuyo resultado no pueda reconstruirse de forma segura después de un fallo, la arquitectura debe conservar antes del efecto información suficiente sobre la operación autorizada, objeto, autoridad, forma, revisión e identidad de operación o equivalente, así como la evidencia necesaria para recuperación.

Después del efecto deberá conservarse, cuando sea técnicamente posible, evidencia suficiente para distinguir éxito, rechazo y resultado material no determinado.

## 13. Recursos y presupuesto

Se consideran, como mínimo, CPU, memoria, entrada y salida, almacenamiento, profundidad, expansión combinatoria, operaciones externas y atención humana.

Para toda forma repetible, recursiva, expansiva, abierta a entrada no confiable o capaz de generar actos humanos privilegiados debe existir una política de consumo constituida:

```text
Budget(F | C)
```

La política debe declarar los recursos relevantes y una cota o criterio de admisión materialmente comprobable. No basta una formulación tautológica como «todos los recursos disponibles» o «hasta que el sistema no pueda continuar».

Si una forma que requiere `Budget(F | C)` carece de él, o el presupuesto no permite decidir si el consumo permanece dentro del régimen autorizado, no admite ejercicio automático.

El agotamiento de recursos no puede producir éxito, `U`, ampliación de autoridad, omisión de comprobaciones ni cambio automático a un régimen más privilegiado.

## 14. Recursos de control

El trabajo ordinario no debe poder consumir sin límite todos los recursos necesarios para rechazar, registrar la causa, conservar evidencia mínima, revocar, detener o recuperar.

No basta declarar una reserva de recursos si depende exactamente del mismo recurso agotable y puede ser consumida por la misma vía. La independencia material de la solución elegida pertenece a SEC.0-X.

SEC.0-M no demuestra disponibilidad absoluta. Exige, al menos, seguridad bajo agotamiento y capacidad de conservar o recuperar las funciones de control en la medida declarada por la arquitectura.

## 15. Atención humana

La atención humana es un recurso finito y atacable. Toda forma capaz de generar actos que requieran una decisión humana privilegiada debe incluir `atencion_humana` entre los recursos de `Budget(F | C)` cuando ese consumo pueda acumularse.

La obligación se aplica también a formas previamente admitidas, agentes especializados y componentes de seguridad. La admisión de una forma no autoriza una cola ilimitada de decisiones privilegiadas.

Reducir la presión humana mediante automatización no transfiere silenciosamente autoridad a la automatización; la delegación continúa gobernada por SEC.0-A.

## 16. Reducción y compactación

Toda forma capaz de reducir, resumir o compactar almacenamiento debe tener constituida con anterioridad una política de retención que determine qué dependencias y propiedades deben conservarse para el alcance pretendido.

La operación de reducción no puede decidir durante su ejecución qué información sigue siendo relevante.

Cuando sean aplicables, la política debe preservar la capacidad de acreditar revocaciones, consumos, estado de acumulación, ligaduras de `Req`, antecedentes de continuidad, testimonios necesarios para recuperación y demás dependencias incluidas en `PDep`.

Una reducción que impida distinguir una autoridad vigente de una revocada, o una autorización consumida de otra no consumida, es inválida para ese alcance.

## 17. Tiempo

Toda decisión que dependa del tiempo debe declarar la fuente temporal entre sus dependencias. SEC.0-M no presume íntegro ningún reloj concreto ni atribuye autoridad a la marca temporal mayor.

La integridad y monotonicidad material de la fuente temporal pertenecen a SEC.0-X.

## 18. Recuperación

La recuperación debe partir de una autoridad de recuperación ya constituida conforme a SEC.0-A. No puede inventarse un responsable o mecanismo de recuperación después del compromiso.

Todo candidato material de recuperación debe poder acreditarse respecto de procedencia, continuidad, vigencia de autoridad, revocaciones, dependencias persistentes, efectos parcialmente ejecutados y compatibilidad con el contexto de recuperación.

No se selecciona un candidato por ser el más reciente según su propio reloj, tener mayor tamaño, haber arrancado primero, estar marcado como `HEAD`, responder antes o contener más operaciones aparentes. La selección requiere una regla de recuperación previamente constituida.

La recuperación es insuficiente si utiliza como única prueba de legitimidad el mismo estado cuya legitimidad intenta restablecer frente al fallo considerado. Debe existir al menos una dependencia de recuperación que no sea indistinguiblemente controlable por el mismo fallo o retroceso relevante.

Una recuperación tampoco puede restaurar una autoridad revocada sólo porque la copia sea anterior a la revocación. Si la vigencia no puede determinarse, los efectos dependientes quedan en `D-N`.

## 19. Copias de seguridad

Debe distinguirse entre copia de seguridad, candidato de recuperación y continuidad vigente.

Una copia puede ser legible y útil para reconstruir datos sin contener evidencia suficiente para restituir autoridad. Sólo puede recuperar el alcance cuyas dependencias persistentes pueda acreditar.

## 20. Observabilidad material

Un registro observado no equivale por sí solo a un hecho material acreditado. SEC.0-M exige identificar las observaciones necesarias para reconstruir fallos, detectar agotamiento, recuperar operaciones inciertas, distinguir retroceso y bifurcación y determinar si una evidencia sigue siendo aplicable.

La integridad material del observador y su independencia respecto del estado observado pertenecen a SEC.0-X.

## 21. Invariantes

- **M2-01 — Estado técnico no es estado semántico.** La presencia material de un objeto no le confiere vigencia semántica.
- **M2-02 — Copia no es continuidad.** La copia íntegra de un estado no constituye por sí sola continuidad legítima.
- **M2-03 — Dependencia persistente suficiente.** Toda decisión reutilizable después de reinicio o recuperación conserva dependencias suficientes.
- **M2-04 — Ausencia no se rellena.** Una dependencia ausente no se sustituye por un valor por defecto que permita continuar.
- **M2-05 — Decisión autoritativa fundada en dependencia autoritativa.** Toda decisión material de vigencia, revocación, consumo, continuidad, cobertura negativa, acumulación o recuperación se funda en un miembro acreditado de `PDep`.
- **M2-06 — Validez local no implica vigencia.** Una historia localmente válida no es necesariamente la continuidad vigente.
- **M2-07 — Retroceso no resucita.** Restaurar estado antiguo no revive revocaciones, autorizaciones consumidas ni efectos olvidados.
- **M2-08 — Resistencia al retroceso exige referencia no retrocedible conjuntamente.** Una garantía fuerte contra retroceso no puede depender sólo del mismo estado completamente retrocedible.
- **M2-09 — Clonación no multiplica autoridad.** Un clon no duplica continuidad, titularidad ni autorizaciones de uso único.
- **M2-10 — Bifurcación gobernada.** La política de ramas y selección de continuidad se constituye antes de utilizarse para autoridad.
- **M2-11 — `HEAD` no es autoridad.** Un puntero de conveniencia no determina por sí mismo continuidad vigente.
- **M2-12 — Cobertura negativa acreditada.** La ausencia en un índice sólo demuestra ausencia autoritativa si su cobertura negativa está acreditada.
- **M2-13 — Comprobación ligada al estado de compromiso.** Un permiso no compromete un efecto sobre un estado distinto cuando el cambio pueda alterar sus ligaduras.
- **M2-14 — Fallo parcial no es éxito ni ausencia.** Un resultado material incierto no se reinterpreta sin evidencia suficiente.
- **M2-15 — No reejecución ciega.** Una operación posiblemente ejecutada no se repite automáticamente tras un fallo.
- **M2-16 — Persistencia suficiente antes de efecto irreversible.** La recuperación dispone de información previa suficiente para identificar la operación cuyo resultado pueda quedar incierto.
- **M2-17 — Envío local no acredita efecto externo.** Una solicitud emitida localmente no acredita por sí sola el efecto externo contratado.
- **M2-18 — Agotamiento no cambia semántica.** La falta de recursos técnicos no produce `U`, éxito ni autoridad.
- **M2-19 — Presupuesto obligatorio y no tautológico.** Toda forma que lo requiera posee `Budget(F | C)` constituido y comprobable; si falta o es tautológico, no hay ejercicio automático.
- **M2-20 — Recursos de control preservables.** El trabajo ordinario no consume sin límite todos los recursos necesarios para rechazo, revocación, diagnóstico y recuperación.
- **M2-21 — Fallo cerrado no se amplifica sin dependencia.** La indisponibilidad se propaga sólo por dependencias constituidas.
- **M2-22 — Atención humana acotada.** Toda forma capaz de exigir decisiones humanas privilegiadas gobierna ese consumo dentro de `Budget(F | C)`.
- **M2-23 — Recuperación no autolegitimada.** El estado recuperado no puede ser su única prueba de legitimidad frente al fallo que pretende superar.
- **M2-24 — Recuperación no elimina revocaciones.** Una restauración anterior no presume vigentes autoridades cuya revocación posterior no pueda descartarse.
- **M2-25 — Copia de seguridad no equivale a recuperación válida.** La legibilidad de una copia no acredita suficiencia de continuidad, autoridad ni dependencias.
- **M2-26 — Tiempo declarado.** Toda decisión temporal declara su fuente sin presumir su integridad.
- **M2-27 — Observación no es prueba por sí sola.** Un registro material sólo acredita aquello cuya procedencia e integridad puedan establecerse.
- **M2-28 — Compactación no elige relevancia.** La política de retención se constituye antes de compactar.
- **M2-29 — Unicidad consumible no presumida.** Una autoridad de un solo uso o acumulación única no admite ejercicio automático si el registro de consumo puede clonarse o retroceder indistinguiblemente y no existe una garantía gobernada contra el segundo consumo.

## 22. Premisas y límites

SEC.0-M no demuestra por sí mismo:

- que un dispositivo de almacenamiento no pueda falsear su estado;
- que una escritura declarada persistente haya alcanzado soporte físico estable;
- que una referencia externa de continuidad sea realmente independiente;
- que un contador, reloj o testimonio monotónico no pueda retroceder;
- que dos réplicas no compartan una misma causa de compromiso;
- que el sistema operativo respete aislamiento o límites;
- que el compilador preserve el contrato;
- que el observador de recursos sea íntegro;
- que la red entregue mensajes o preserve orden;
- que exista disponibilidad absoluta;
- que un efecto externo se produzca exactamente una vez;
- que una persona responda siempre o interprete correctamente una consecuencia.

Estas garantías o límites corresponden a SEC.0-X, SEC.0-T o al gobierno posterior según su naturaleza.

## 23. Alcance del cierre

SEC.0-M cierra el contrato abstracto de memoria, persistencia, recursos y continuidad. No materializa todavía:

- almacenamiento físico;
- coordinación concreta entre réplicas;
- mecanismo de consenso;
- sistema de archivos;
- sistema operativo;
- cadena de construcción;
- atestación;
- aislamiento material;
- plataforma de referencia;
- estructuras nuevas de IR;
- sintaxis;
- códigos diagnósticos;
- valores numéricos de recursos;
- infraestructura general de ejecución.

La siguiente fase deberá estudiar las garantías materiales necesarias para que los contratos cerrados de SEC.0-A, SEC.0-D y SEC.0-M no dependan de componentes que puedan falsear precisamente las propiedades que esos contratos presuponen.
