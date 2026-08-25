# R2-0 — Contrato de persistencia, continuidad y recuperación

**Fecha:** 25 de agosto de 2026  
**Repositorio:** `SV-lenguaje-de-computacion`  
**Fase:** R2 — persistencia y continuidad material  
**Estado de R2:** abierto  
**Estado de R2-0:** contrato de realización  
**R0:** cerrado  
**R1:** cerrado  
**R3–R4:** no iniciados  
**Garantía I:** `NO_PROBADO`  
**Garantía II:** `NO_PROBADO`

## 1. Objeto

R2-0 fija las condiciones de realización que deberán satisfacer los cortes posteriores de R2 antes de introducir persistencia productiva nueva.

El objeto de R2 no es conservar bytes por sí mismos, sino impedir que una representación técnica restaurada, copiada, resumida, bifurcada o aparentemente reciente pueda sustituir sin acreditación al estado persistente autoritativo o a la continuidad vigente de la que depende una decisión protegida.

R2 parte de las garantías intra-proceso ya cerradas en R0 y R1. No modifica la semántica del Lenguaje SV, la gramática, la IR ni las separaciones entre `Tri`, D-A/D-R/D-N, autoridad, permiso, mediación y ejercicio.

## 2. Corte de partida

R2-0 parte del corte soberano:

```text
main = 23bf24334171b88aa0391613f4de9f58735f56af
```

con el estado:

```text
R0 = CERRADO
R1 = CERRADO
R2 = ABIERTO
R3 = NO INICIADO
R4 = NO INICIADO
```

## 3. Base contractual

R2-0 se interpreta conjuntamente con:

- `ACTA_TECNICA_APERTURA_R2_PERSISTENCIA_Y_CONTINUIDAD_MATERIAL_2026_08_25.md`;
- `CONTRATO_ABSTRACTO_DE_MEMORIA_PERSISTENCIA_RECURSOS_Y_CONTINUIDAD_SEC0_M_2026_08_21.md`;
- `ADENDA_CORRECTIVA_SEC0_M_CLONACION_IMPLANTACION_Y_GENESIS_2026_08_22.md`;
- SEC.0-A y su adenda correctiva sobre la unidad de génesis;
- SEC.0-D, SEC.0-X y SEC.0-T en cuanto fijan fallo cerrado, dependencias materiales, alcance probatorio y criterios de contraste;
- la especificación arquitectónica vigente del entorno de ejecución soberano.

Ninguna regla de R2 podrá rebajar obligaciones ya cerradas por esos contratos.

## 4. Fronteras de estado

R2 deberá conservar como clases no intercambiables:

```text
estado de proceso
≠ estado técnico persistido
≠ estado derivado
≠ estado persistente autoritativo
≠ continuidad vigente
```

No se admitirán inferencias del tipo:

```text
bytes íntegros ⇒ estado vigente
copia completa ⇒ continuidad legítima
historial local válido ⇒ continuidad vigente
índice ⇒ autoridad
último registro ⇒ vigencia
fecha mayor ⇒ autoridad
```

La persistencia representa hechos, autoridad, decisiones y dependencias; no los constituye por el mero acto de almacenarlos.

## 5. `AStore`, vistas y cobertura

R2 materializará la distinción entre un estado persistente autoritativo `AStore` y cualquier estructura derivada utilizada para localizarlo, resumirlo, indexarlo o acelerarlo.

Una vista, caché, índice, resumen, tabla auxiliar o réplica derivada sólo podrá:

1. ayudar a localizar o consultar una fuente autoritativa; o
2. convertirse expresamente en fuente autoritativa para una decisión, en cuyo caso deberá asumir las obligaciones de persistencia y dependencia que correspondan.

No podrá declararse simultáneamente no autoritativa y decidir por sí sola vigencia, revocación, consumo, continuidad o cobertura negativa.

Toda decisión basada en ausencia deberá apoyarse en una fuente cuya cobertura negativa sea suficiente. En particular:

```text
x ∉ View(AStore)
↛
x ∉ AStore
```

salvo que la relación de cobertura aplicable esté acreditada.

## 6. Dependencias persistentes `PDep`

Para toda decisión protegida `d` que pretenda sobrevivir a reinicio, recuperación o sustitución de proceso deberá poder reconstruirse un conjunto suficiente:

```text
PDep(d | C)
```

capaz de volver a justificar su aplicabilidad en el contexto pertinente.

`PDep` podrá incluir, cuando sean materiales para la decisión:

- forma constituida;
- autoridad aplicable;
- `E_max` y `D_a` pertinentes;
- `Req` y sus ligaduras;
- resultados técnicos aplicables;
- cobertura y reglas de reutilización;
- revocaciones;
- contratos de acumulación;
- antecedente de continuidad;
- estado de consumo;
- traza decisión–efecto;
- dependencias de vigencia, tiempo o recuperación cuando correspondan.

La persistencia de un D-A o de un permiso no los independiza de sus dependencias.

Si falta una dependencia necesaria:

```text
no se hereda la acreditación
no se fabrica continuidad
no se fabrica autoridad
no se fabrica Tri.U
```

La operación afectada deberá quedar bloqueada o técnicamente no verificable conforme al contrato aplicable.

## 7. Continuidad vigente, retroceso y bifurcación

R2 conservará la distinción:

```text
ValidLocal(h)
≠
Current(h | I)
```

con:

```text
Current(h | I) ⇒ ValidLocal(h)
```

sin admitir la implicación inversa.

La continuidad vigente no podrá seleccionarse únicamente por:

- `HEAD`;
- mayor contador local;
- fecha o marca temporal mayor;
- último elemento recibido;
- mayor tamaño;
- primer arranque;
- respuesta más rápida;
- identificador técnico nuevo.

Una restauración técnica anterior no podrá, por sí sola, borrar revocaciones posteriores, resucitar autoridad consumida, devolver un permiso agotado, reducir acumulaciones o convertir una rama histórica en continuidad vigente.

Si dos ramas localmente válidas son incompatibles y la política exige unicidad, la imposibilidad de acreditar cuál es vigente bloqueará los efectos dependientes. No se elegirá una rama para preservar disponibilidad.

## 8. Clonación y génesis

La clonación o restauración de estado no multiplica autoridad ni génesis.

Se conserva obligatoriamente:

```text
nuevo proceso / host / contenedor / réplica
+
continuidad autoritativa previa
⇒
T-0 no disponible
```

Cuando una implantación restaure o prolongue autoridad preexistente, la clase aplicable deberá derivarse del efecto real. Una identidad técnica nueva no reclasifica el acto.

## 9. Revocación persistente y recuperación gobernada

R1 dejó T-G, T-C y T-R no productivas. R2 no abre estas clases de forma general.

R2 deberá materializar, de forma acotada por forma y familia de efecto:

1. una vía T-G suficiente para persistir una revocación gobernada y mantenerla eficaz dentro de una continuidad cuya vigencia haya quedado acreditada; una restauración cuya continuidad no pueda acreditarse no podrá presentarse como vigente por omitir esa revocación; y
2. una vía T-R suficiente para restablecer autoridad preexistente bajo continuidad legítima sin crearla, ampliarla ni escoger silenciosamente entre continuidades incompatibles.

La apertura de R2 no habilita por sí sola:

- concesión general de nueva autoridad por T-G;
- delegación general por T-G;
- modificación constitutiva por T-C;
- recuperación sin `Req` propios;
- recuperación fundada únicamente en el mismo estado cuya legitimidad pretende acreditar.

T-C permanecerá no productiva mientras no exista un objeto constitutivo de R2 que exija expresamente su apertura y supere su propio contrato.

## 10. Recuperación no circular

Toda recuperación material deberá partir de una autoridad y una regla de recuperación previamente constituidas.

Un candidato de recuperación deberá poder justificarse respecto de, como mínimo cuando resulten aplicables:

- procedencia;
- continuidad;
- autoridad vigente;
- revocaciones;
- `PDep` suficientes;
- estado de consumo;
- efectos parcialmente ejecutados;
- compatibilidad con el contexto recuperado.

El estado restaurado no podrá ser la única fuente de la prueba que lo acredita frente al mismo fallo considerado.

Cuando la independencia material necesaria sólo pueda ser suministrada por una raíz, plataforma o mecanismo externo a R2, esa dependencia se declarará y permanecerá `NO_PROBADO` hasta R3. R2 no simulará esa independencia dentro de `sv_core`.

## 11. Persistencia de decisión, compromiso y efecto

R2 deberá prolongar de forma material la cadena cerrada en R1:

```text
D-A
→ Permit
→ MediatedEffectCommitment
→ DispatchCommitted
→ Confirmed | Indeterminate
```

Para operaciones cuyo resultado no pueda reconstruirse con seguridad después de un fallo, deberá existir persistencia suficiente antes del efecto y evidencia posterior suficiente para distinguir, dentro del alcance declarado:

```text
preparado
comprometido para despacho
efecto confirmado
resultado material indeterminado
cierre acreditado sin efecto
```

Si un efecto pudo ocurrir y no puede determinarse si ocurrió:

```text
no se presupone éxito
no se presupone ausencia
no se reintenta automáticamente
no se produce Tri.U
```

La continuación deberá depender de una regla constituida de reconciliación, idempotencia, compensación o comprobación externa. R2 no prometerá ejecución exactamente una vez frente a sistemas que no proporcionen las propiedades necesarias.

## 12. Consumo único

Una afirmación de consumo único resistente a reinicio, restauración, clonación o carrera sólo será admisible cuando el perfil declarado disponga de dependencias suficientes para impedir o hacer no verificable un segundo consumo antes del efecto.

Un contador local clonable o retrocedible junto con el mismo estado protegido no basta.

Cuando la propiedad dependa de una raíz o aislamiento material cuya acreditación pertenezca a R3:

```text
consumo único fuerte = NO_PROBADO
```

hasta que esa dependencia quede acreditada.

La ausencia de esa propiedad no se ocultará mediante una afirmación de ejecución «exactamente una vez».

## 13. Reducción, compactación y retención

Toda reducción o compactación de estado que pueda afectar decisiones autoritativas deberá obedecer una política de retención constituida antes de conocer el resultado que pretende conservar.

La reducción deberá preservar, cuando sean necesarias, las dependencias que permitan distinguir:

- autoridad vigente y revocada;
- autorización disponible y consumida;
- continuidad vigente e historia local;
- requisitos cubiertos y no cubiertos;
- efecto confirmado e indeterminado;
- antecedente suficiente para recuperación.

Una compactación que destruya una dependencia necesaria de `PDep` no será válida para el alcance que pretendía conservar.

## 14. Tiempo y presupuestos

R2 no introduce tiempo como primitiva semántica universal ni abre `BudgetΣ` por defecto.

Una fuente temporal sólo podrá entrar cuando una obligación previamente constituida dependa materialmente de actualidad, expiración, presupuesto o ventana temporal. Esa fuente será una dependencia explícita de la comprobación y no una consulta ambiental oculta del núcleo.

Se conserva:

```text
reloj técnico ≠ autoridad
reloj técnico ≠ continuidad
reloj técnico ≠ Tri
```

`Budget(F | C)` sólo se materializará cuando una forma vigente lo requiera.

Si ningún objeto actual de R2 exige una dependencia temporal o un presupuesto material, R2 podrá cerrar el corte correspondiente mediante evidencia de no aplicabilidad. No se introducirán relojes, contadores ni políticas de consumo por anticipación.

## 15. Separación respecto de R3

R2 puede declarar dependencias materiales cuya verdad no pueda establecer por sí solo. No puede atribuirse su acreditación.

Permanecen fuera de R2, entre otras:

- raíz material de confianza;
- integridad del artefacto cargado;
- cadena completa de construcción, distribución y carga;
- aislamiento de sistema operativo, hipervisor o hardware;
- control material general de red y periféricos;
- atestación de plataforma;
- confidencialidad material completa.

La necesidad de una de estas dependencias no convierte R2 en R3.

## 16. Descomposición de R2

La fase se ordena en los siguientes cortes:

```text
R2-0  contrato de persistencia y fronteras de estado
R2-1  AStore, PDep, cobertura y retención
R2-2  continuidad vigente, retroceso, clonación y bifurcación
R2-3  revocación persistente y recuperación gobernada
R2-4  persistencia decisión–efecto, reconciliación y consumo
R2-5  presupuestos y tiempo sólo cuando sean aplicables
R2-6  regresión integral, contraste adversarial y cierre de fase
```

La numeración ordena el trabajo. No crea por sí sola tipos, formatos de almacenamiento, procesos, servicios o tecnologías.

## 17. Criterios de cierre de R2

R2 sólo podrá cerrarse si existe evidencia reproducible de que:

1. una representación técnica persistida no adquiere autoridad o vigencia por presencia, copia o integridad de bytes;
2. una vista no autoritativa no puede decidir por sí sola una cuestión autoritativa;
3. una decisión persistida sólo puede reutilizarse cuando sus `PDep` suficientes siguen siendo reconstruibles y aplicables;
4. la ausencia en una vista o índice no se utiliza como ausencia autoritativa sin cobertura acreditada;
5. una continuidad vigente no se selecciona por `HEAD`, fecha, contador, orden de llegada, velocidad o conveniencia;
6. restauración, clonación o nueva identidad técnica no reabren T-0;
7. una revocación gobernada permanece eficaz después de reinicio dentro de una continuidad acreditada; una restauración que no permita acreditar la continuidad vigente no puede presentarse como válida por omitir la revocación;
8. T-R sólo restablece autoridad preexistente bajo continuidad legítima y no crea ni amplía autoridad;
9. una bifurcación incompatible bloquea los efectos dependientes cuando la política exige unicidad y no puede acreditarse la continuidad vigente;
10. un efecto material indeterminado no se reinterpreta como éxito, ausencia, D-A ni `Tri.U`, ni se reintenta automáticamente;
11. una afirmación de consumo único sólo se realiza en perfiles cuyas dependencias permiten sostenerla dentro del modelo de fallos declarado;
12. reducción o compactación no destruyen dependencias necesarias para revocación, consumo, continuidad o recuperación;
13. toda dependencia temporal o de presupuesto material es explícita, aplicable y no ambiental; su ausencia cuando no resulte aplicable no introduce mecanismos artificiales;
14. `D-N`, fallos técnicos, estados de recuperación y deuda de proyecto permanecen fuera de `Tri`;
15. R2 no atribuye como propias garantías de raíz, aislamiento, artefacto cargado o plataforma reservadas a R3;
16. R0 y R1, incluidos sus destinos nativo y WebAssembly, no sufren regresión semántica atribuible a R2.

El cierre de R2 no constituye Garantía I ni Garantía II.

## 18. Estado de transición durante R2

Hasta que cada corte material cierre, se conserva:

```text
T-E = PRODUCTIVA POR VÍA GOBERNADA
T-G = NO PRODUCTIVA salvo las familias que R2 cierre expresamente
T-C = NO PRODUCTIVA
T-R = NO PRODUCTIVA hasta su materialización gobernada en R2-3
```

Ninguna clase se considera abierta por el mero hecho de aparecer en este contrato.

## 19. Continuidad arquitectónica gobernada

La realización de R2 deberá preservar una estructura garantista suficiente para su alcance sin convertir las decisiones actuales de representación, almacenamiento o coordinación en impedimentos frente a arquitecturas posteriores que acrediten una solución superior.

La sustitución futura de una arquitectura no hereda por sí sola autoridad, vigencia, evidencia ni garantías. Toda modificación causalmente relevante seguirá el régimen de gobierno, comprobación y trazabilidad aplicable y el Pliego de Condiciones del Sistema Vectorial SV, DOI `10.21428/39829d0b.bbcac925`.

## 20. Estado resultante de R2-0

La aprobación de este contrato permitirá abrir R2-1. No autoriza todavía código productivo de persistencia ni selecciona una tecnología material.

```text
R0 = CERRADO
R1 = CERRADO
R2 = ABIERTO
R2-0 = CONTRATO DE REALIZACIÓN
R2-1 = NO INICIADO
R3 / R4 = NO INICIADOS
BudgetΣ / IA-SEC = NO ABIERTOS
Garantía I / II = NO_PROBADO
```
