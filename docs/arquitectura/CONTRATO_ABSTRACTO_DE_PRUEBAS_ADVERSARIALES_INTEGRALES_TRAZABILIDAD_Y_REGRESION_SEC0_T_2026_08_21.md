# SEC.0-T — Contrato abstracto de pruebas adversariales integrales, trazabilidad y regresión

**Fecha:** 21/08/2026  
**Estado:** contrato arquitectónico cerrado  
**Ámbito:** Lenguaje SV — SEC.0-T  
**Antecedentes:** SEC.0-A, SEC.0-D, SEC.0-M y SEC.0-X cerrados como contratos abstractos.

## 1. Objeto

SEC.0-T establece las condiciones que debe satisfacer una batería de pruebas destinada a contrastar realizaciones concretas con los contratos SEC.0-A, SEC.0-D, SEC.0-M y SEC.0-X.

El cierre de una prueba no equivale a una demostración universal de seguridad. Sí se establece la implicación contraria: una violación material observada demuestra que la realización sometida a prueba no conserva el contrato afectado dentro del alcance ensayado.

SEC.0-T no modifica gramática, IR, validador ni catálogo diagnóstico, y no selecciona plataforma, sistema operativo, procesador, lenguaje de implementación, almacenamiento ni mecanismo criptográfico.

## 2. Objeto sometido a prueba

Toda ejecución debe identificar de forma suficiente la realización exacta sometida a prueba, denominada `SUT`.

Según el alcance, la identidad debe ligar versión del código, artefacto ejecutable, configuración, dependencias relevantes, definición de garantías, estado inicial, datos de prueba, versión de la batería e instrumentación utilizada.

La evidencia obtenida sobre un objeto no se transfiere automáticamente a otro. Si una diferencia puede afectar causalmente a la garantía dentro del modelo de fallos declarado, la equivalencia entre ambos objetos debe quedar acreditada por evidencia capaz de detectar esa diferencia.

## 3. Correspondencia y falsabilidad

Cada caso `t` debe declarar las propiedades que intenta comprobar:

```text
Targets(t) = {invariantes o garantías}
```

La mera correspondencia nominal no constituye cobertura.

Para que un caso cubra un invariante `I` debe ser falsable respecto de `I`. Se expresa:

```text
Falsifiable(t, I) = true
```

sólo cuando existe una alteración no conforme especificada y materialmente ejercitable —inyección, mutación, artefacto no conforme o condición equivalente— cuyo alcance sobre el objetivo pueda acreditarse y cuyo resultado esperado pueda distinguirse mediante el criterio de la prueba.

Una declaración documental de falsabilidad no basta. Si la alteración no puede ejercerse o no puede acreditarse que alcanzó el objetivo, el caso no cubre `I`.

Cuando una propiedad no admita comprobación dinámica, la evidencia alternativa sólo constituye cobertura si su método puede rechazar o refutar una realización no conforme respecto de la propiedad declarada.

## 4. Aplicabilidad de las clases de prueba

La aplicabilidad de una clase de prueba se deriva de las capacidades y garantías efectivamente presentes en el `SUT`.

Se expresan abstractamente:

```text
Capabilities(SUT, G)
ApplicableClass(c | SUT, G)
```

El ejecutor de la batería no puede reducir el perímetro de conformidad declarando unilateralmente una clase como no realizable.

Si una capacidad existe —por ejemplo ejecución automática, persistencia, recuperación, autoridad consumible, atestación, clonación posible o presentación humana privilegiada— las clases de prueba asociadas son aplicables salvo evidencia suficiente en sentido contrario.

Una afirmación completa de conformidad con A, D, M o X exige cobertura suficiente de todas las propiedades aplicables. Las propiedades `NO_PROBADO`, `NO_EJECUTADO` o `INCONCLUSO` impiden una afirmación completa de conformidad para el alcance afectado.

## 5. Criterio esperado

Toda prueba debe disponer de un criterio esperado `Oracle(t)` que no dependa circularmente del componente sometido al fallo.

No es admisible que el mismo componente potencialmente comprometido produzca el resultado y constituya la única fuente utilizada para decidir que dicho resultado era correcto.

La independencia del criterio se evalúa respecto de la misma clase de fallo que el caso pretende ensayar. Una segunda copia, ejecución o proceso no constituye independencia si comparte precisamente la causa de compromiso introducida.

## 6. Instrumentación y observación

Toda prueba debe declarar la instrumentación relevante. La instrumentación no se presume transparente.

Si el observador o instrumento puede alterar orden, sincronización, tiempo, recursos, privilegios, persistencia o cualquier otra condición capaz de impedir o transformar el fallo ensayado, el resultado no puede transferirse como `PASS` a la realización ordinaria.

En tal supuesto, el caso será `NO_EJECUTADO` respecto de la garantía de producción o, si procede, podrá atribuir un `PASS` únicamente al objeto efectivamente ensayado, incluida la instrumentación. La transferencia posterior exige una equivalencia acreditada.

## 7. Pruebas integrales y reducción de fallos

La batería debe combinar escenarios largos de extremo a extremo con casos mínimos de regresión.

Cuando un escenario integral descubre un fallo debe conservarse el escenario original, reducirse la violación al caso mínimo que preserve la causa, corregirse la realización, incorporar el caso mínimo como regresión permanente y volver a ejecutar el escenario integral.

El caso reducido no sustituye al escenario integral que permitió descubrir la interacción.

Una regresión sólo puede retirarse por modificación legítima del contrato o por sustitución mediante una cobertura equivalente o superior acreditada.

## 8. Pruebas positivas y negativas

La batería debe comprobar tanto operaciones legítimas que deben avanzar como operaciones prohibidas, refutadas o no verificables que deben quedar bloqueadas.

Las afirmaciones de ausencia —por ejemplo, ausencia de una vía lateral— requieren una cobertura explícita del perímetro examinado. Una enumeración incompleta no demuestra ausencia universal.

## 9. Clases mínimas derivadas de SEC.0-A

Cuando resulten aplicables según las capacidades del `SUT`, deben poder ensayarse al menos: creación de autoridad por información; ampliación indebida de la envolvente máxima de efectos; ampliación del dominio gobernado mediante ejecución ordinaria; reclasificación interesada de transiciones; uso de formas no constituidas; repetición sin contrato de acumulación; delegación fuera de alcance; recuperación que inventa autoridad; y confusión entre capacidad material de ejecución y autoridad semántica.

## 10. Clases mínimas derivadas de SEC.0-D

Deben poder ensayarse, cuando proceda: requisitos vacíos; omisión de obligaciones nucleares; conversión de `D-N` en éxito o en `U`; eliminación de una refutación mediante indisponibilidad; sustitución oportunista del verificador; acreditación propia; cobertura parcial presentada como total; reutilización de comprobaciones caducadas; excepciones o emergencia que omitan el núcleo; y cambio de estado entre comprobación y efecto.

## 11. Clases mínimas derivadas de SEC.0-M

Deben poder ensayarse, cuando proceda: retroceso de revocaciones; restauración de autorizaciones consumidas; clonación y doble consumo; selección de continuidad mediante punteros o índices; índices incompletos; uso de una vista no autoritativa como fuente decisoria; bifurcaciones; copias de seguridad insuficientes; reducción de historial que elimine evidencia; presupuestos ausentes o tautológicos; agotamiento de recursos; saturación de atención humana; efectos externos inciertos; reejecución ciega; retroceso temporal; recuperación circular; y reservas de control sin aislamiento real.

## 12. Clases mínimas derivadas de SEC.0-X

Deben poder ensayarse, cuando proceda: arranque correcto sobre estado persistente ilegítimo; cadena de construcción alterada; comprobaciones compensatorias con causa de fallo común; sustitución de ejecutables; vías privilegiadas o de mantenimiento que eviten mediación; repetición de atestaciones antiguas; omisión local de frescura; testigos contra retroceso contenidos en la misma copia; falsa independencia de aprobaciones; cambio entre presentación y firma; canal autenticado hacia proceso no admitido; observador y observado bajo la misma causa de compromiso; aislamiento ficticio de recursos; administración material capaz de falsear garantías; rotación de raíz comprometida; y reescritura de `TCB(G)` para ocultar un falsificador.

## 13. Composición de fallos

SEC.0-T exige pruebas de interacción entre A, D, M y X. La resistencia separada a dos fallos no permite inferir resistencia automática a su combinación si existe una interacción causal posible.

No se exige el producto cartesiano de todas las combinaciones. Las omisiones deben justificarse por alcance, independencia o ausencia de interacción material relevante.

## 14. Inyección de fallos

Una prueba sólo cuenta como ejercicio de un fallo cuando existe evidencia de que la inyección alcanzó la dependencia que pretendía afectar.

Se expresa:

```text
Inject(f)
Reach(f, target)
```

Si no puede acreditarse `Reach`, el caso no se registra como superado. Debe quedar `NO_EJECUTADO` o en el estado técnico que corresponda.

## 15. Mutación semántica y prohibición de cambio de significado

La batería debe detectar cambios que conserven apariencia superficial pero alteren el significado contractual.

Cuando el `SUT` materialice las distinciones correspondientes, son obligatorias pruebas adversariales sobre, entre otras, las siguientes mutaciones: `D-N` o `D-R` convertidos en éxito; historia local presentada como continuidad vigente; vista no autoritativa convertida de hecho en fuente decisoria; reducción de `TCB(G)`; forma extraordinaria presentada como ordinaria; evidencia antigua presentada como vigente; revisión intercambiada entre presentación y firma; y resultado material incierto presentado como efecto acreditado.

SEC.0-T prueba los contratos previos; no permite redefinirlos para facilitar la superación de la batería.

## 16. Estado inicial y repetición

Toda prueba debe declarar el estado inicial relevante. Una ejecución no puede heredar accidentalmente autoridad, cachés, verificaciones, claves, contadores, revocaciones, recursos consumidos, ramas, artefactos o sesiones de una prueba anterior.

La preparación entre casos tampoco puede borrar el estado cuya persistencia se pretende comprobar.

En pruebas concurrentes o no deterministas debe declararse la estrategia de repetición y conservarse información suficiente para reconstruir una ejecución concreta. La ausencia de reproducción en un número finito de intentos no demuestra ausencia del fallo.

## 17. Traza de ejecución

Toda ejecución debe conservar información suficiente para relacionar al menos:

```text
TestRun
SUT
TestCase
Targets
ThreatModel
InitialState
InjectedFaults
ReachedFaults
Oracle
Observer
Expected
Observed
Verdict
Artifacts
```

La traza de prueba no sustituye la traza semántica del sistema.

Cuando `TestRun` sostenga una afirmación pública de conformidad, su integridad debe quedar protegida frente a la misma clase de fallo para la que se invoca como evidencia. Una traza reescribible por el mismo `SUT` puede conservar valor de laboratorio, pero no constituye por sí sola evidencia pública independiente frente a dicho fallo.

## 18. Veredictos

Se distinguen al menos:

```text
PASS
FAIL
NO_EJECUTADO
NO_PROBADO
INCONCLUSO
```

`PASS` significa que un caso falsable se ejecutó, el fallo o mutación exigido alcanzó su objetivo y se obtuvo el resultado esperado dentro del alcance declarado.

`FAIL` significa que se observó una violación o resultado incompatible.

`NO_EJECUTADO`, `NO_PROBADO` e `INCONCLUSO` no constituyen cobertura.

Ninguno de estos estados pertenece a `Tri` ni equivale a D-A, D-R o D-N.

Un `FAIL` confirmado no desaparece por la acumulación posterior de ejecuciones satisfactorias.

## 19. Cierre de un fallo

Un fallo sólo puede declararse corregido cuando se conserva evidencia suficiente de la violación original, se identifica la condición infringida, se modifica legítimamente la realización o el contrato, el caso mínimo deja de fallar por la causa esperada, el escenario integral original deja de fallar y la regresión queda incorporada.

Una corrección que sólo oculta el síntoma no cierra la violación.

## 20. Afirmaciones de conformidad

Toda afirmación de conformidad debe declarar versión exacta, contratos e invariantes cubiertos, casos ejecutados, propiedades no probadas, modelos de fallos, dependencias no ejercitadas, límites de instrumentación y fallos conocidos pendientes.

No se permite inferir:

```text
0 fallos observados
⇒
0 fallos existentes
```

La conformidad es una afirmación de evidencia dentro de un alcance, no una demostración universal de seguridad.

## 21. Invariantes de SEC.0-T

Quedan fijadas las obligaciones siguientes:

1. toda evidencia se liga al objeto exacto probado;
2. no existe transferencia silenciosa de evidencia entre realizaciones distintas;
3. todo caso declara las propiedades que intenta falsificar;
4. la cobertura exige falsabilidad ejercitable y alcance acreditado;
5. ausencia de prueba falsable implica `NO_PROBADO`;
6. el criterio esperado no es circular y su independencia se mide frente al mismo fallo;
7. la instrumentación y sus perturbaciones quedan declaradas;
8. una instrumentación capaz de impedir el fallo no produce evidencia transferible sin equivalencia acreditada;
9. los escenarios integrales se conservan tras reducir un fallo;
10. todo fallo confirmado y corregido deja regresión permanente;
11. la batería contiene pruebas positivas y negativas;
12. la cobertura negativa se limita al perímetro acreditado;
13. las combinaciones de fallos se ensayan cuando exista interacción causal relevante;
14. una inyección sólo cuenta si alcanza el objetivo;
15. una inyección fallida no equivale a `PASS`;
16. el cambio de etiquetas o representaciones no puede alterar el significado contractual;
17. las mutaciones semánticas aplicables son obligatorias;
18. el estado inicial y la preparación entre pruebas son trazables;
19. la falta de reproducción finita no demuestra ausencia del fallo;
20. toda ejecución conserva una traza reproducible;
21. la evidencia pública se protege frente al mismo fallo para el que se invoca;
22. `NO_EJECUTADO`, `NO_PROBADO` e `INCONCLUSO` no cubren;
23. los veredictos de prueba permanecen separados de `Tri` y de D-A/D-R/D-N;
24. un `FAIL` confirmado no se elimina por acumulación estadística de `PASS`;
25. la corrección de un fallo debe ser causal;
26. no existe degradación silenciosa de una garantía para convertir una violación en comportamiento permitido;
27. toda conformidad declara su alcance y sus límites;
28. cero fallos observados no implica cero fallos existentes;
29. SEC.0-T no redefine A, D, M o X;
30. la aplicabilidad deriva de las capacidades y garantías del `SUT`;
31. una afirmación completa de conformidad no admite propiedades aplicables sin cobertura suficiente.

## 22. Límites

SEC.0-T no demuestra ausencia de fallos no ensayados, cobertura universal de hardware o microprogramas, corrección absoluta de la instrumentación, independencia física no materializada, comprensión humana ni seguridad de versiones futuras distintas de la probada.

Tampoco exige que toda propiedad se pruebe dinámicamente. Las propiedades que requieran demostración formal, inspección, revisión de cadena de suministro u otra evidencia deben declarar ese método y su alcance.

## 23. Cierre

SEC.0-T queda cerrado como contrato abstracto de pruebas adversariales integrales, trazabilidad y regresión.

Con este cierre quedan fijados los cinco contratos SEC.0-A, SEC.0-D, SEC.0-M, SEC.0-X y SEC.0-T. El cierre contractual no constituye una declaración de conformidad de la implementación vigente. Toda realización posterior debe aportar evidencia ejecutable y acotada al alcance realmente probado.
