# Acta técnica complementaria de fijación de fuente doctrinal y preservación pre-DSL del aprendizaje trazable

**Fecha:** 16/08/2026  
**Hora (Europe/Madrid):** 06:03  
**Naturaleza:** acta complementaria de gobierno técnico, fijación de fuente y preservación pre-DSL  
**Frente:** Lenguaje SV / aprendizaje trazable / continuidad N3–N4 / preparación de futura traducción al DSL  
**Estado:** cerrada para fijación de fuente; continuidad técnica latente  
**Acta antecedente:** `docs/calidad/ACTA_TECNICA_DE_RECEPCION_DOCTRINAL_Y_PRESERVACION_DE_CONTINUIDAD_DEL_APRENDIZAJE_TRAZABLE_HACIA_IR_N3_N4_2026_08_15.md`  
**Rama técnica reservada:** `traceable-learning-ir-v0.1`  
**Base de verificación:** `VERIFICACION_INTEGRAL` del perímetro afectado y contraste expreso con IR v0.2, gramática superficial v0.1, AUTH preservado, custodia estructural y publicación doctrinal

## 1. Objeto

Esta acta complementa, sin reescribirla, el acta de recepción doctrinal de 15/08/2026. Su objeto es fijar la fuente doctrinal efectiva que deberá gobernar cualquier futura traducción del aprendizaje trazable al Lenguaje SV, preservar las distinciones semánticas que no deben perderse al aproximarse al DSL y cerrar la asimetría registral existente entre la recepción doctrinal ya publicada y los instrumentos activos de Calidad.

No autoriza integración en `main`, no modifica la IR canónica, no modifica la gramática superficial, no altera parser, validator, lowering, runner, Playground ni motor y no convierte la cápsula de referencia en especificación del lenguaje.

## 2. Fuente doctrinal y evidencia editorial fijadas

La fuente doctrinal propia queda identificada por:

- sede: `SV-matematica-semantica/documentos/fundamentos/aprendizaje-trazable-en-inteligencia-artificial/`;
- publicación española: **“Aprendizaje trazable en inteligencia artificial: evolución estructural del conocimiento con frames ternarios y trazas acumulativas”**;
- DOI del preprint español: **`10.21428/39829d0b.bebc607c`**;
- publicación internacional: **“Traceable Learning in Artificial Intelligence: Structural Knowledge Evolution with Ternary Frames and Cumulative Traces”**;
- revista de destino: *Journal of Automated Reasoning*;
- Submission ID: **`6a8347e5-23d3-4f03-87e7-1a8e95e5e594`**;
- versión de envío: **v1.0**;
- estado editorial a esta fecha: **Technical check**;
- cápsula reproducible Code Ocean: versión 1, enviada a publicación y en verificación;
- DOI provisional de Code Ocean: **`10.24433/CO.4645115.v1`**;
- suite ejecutable de referencia: **18 casos finitos**, positivos, negativos y adversariales.

El DOI de Code Ocean se registra con su estatuto actual de **provisional**. Su eventual confirmación sin cambio semántico será un hecho registral y de citación; por sí sola no autorizará ninguna modificación de IR, DSL ni código.

## 3. Relación con el acta de 15/08/2026

El acta antecedente conserva íntegramente su valor histórico como recepción doctrinal y como autorización de la rama latente. No se modifica retrospectivamente.

La expresión editorial «de manera autocontenida» utilizada allí para caracterizar la publicación no forma parte de ningún contrato técnico ni debe propagarse a IR, DSL o documentación futura. La publicación final se trata como una publicación doctrinal autónoma que define los objetos que necesita y cita sus antecedentes; su valor para el lenguaje deriva de sus definiciones y resultados, no de una etiqueta editorial de autocontención.

Esta acta fija además el perímetro final que debe prevalecer sobre resúmenes anteriores cuando una futura unidad traduzca la doctrina al lenguaje.

## 4. Distinciones semánticas que deben sobrevivir a la traducción

No podrá reducirse el aprendizaje trazable a un cambio de vector, a una ampliación de almacenamiento ni a una consulta ternaria aislada. Como mínimo deberán conservarse las siguientes separaciones:

1. **Frame/vector, trayectoria, registro de conocimiento y proyección activa son objetos distintos.**  
   La trayectoria del sistema es append-only; el registro de conocimiento conserva historia estructural; la proyección activa puede crecer, contraerse o recuperar elementos sin borrar la historia.

2. **Evolución, incremento y aprendizaje no son sinónimos.**  
   La pérdida pura puede constituir evolución sin aprendizaje. El aprendizaje exige al menos un incremento históricamente nuevo y sustentado bajo el contrato declarado.

3. **Novedad histórica y recuperación son distintas.**  
   La reactivación de una clase adquirida anteriormente no constituye por sí sola aprendizaje nuevo. Una ruta, relación, argumento o contenido no equivalente y no registrado previamente sí puede constituir incremento nuevo.

4. **Soporte histórico y consulta operacional presente son distintos.**  
   La proposición histórica `Learn_D(E)` sobre un episodio completo no puede ser reescrita porque una vista posterior haya perdido evidencia. Una consulta operacional agotada puede devolver `U` si la base retenida no permite cerrar `LEARN` ni `NO_LEARN`.

5. **La política de soporte es parte de la instancia.**  
   La política finita `ρ_E` y la completitud relativa a ella deben quedar fijadas antes del veredicto. Modificarla después de conocer el resultado constituye otra instancia, no una reinterpretación silenciosa.

6. **Razonamiento, composición y aprendizaje son separables.**  
   Una ejecución de razonamiento o composición puede ocurrir sin aprendizaje; puede participar en el soporte de un incremento; y su esencialidad solo puede afirmarse respecto del registro completo declarado de soportes admisibles.

7. **Aprender no transfiere autoridad.**  
   Las transiciones de máquina pueden modificar conocimiento legítimamente, pero no adquieren por ello autoridad para modificar la especificación rectora definida por autoridad humana. Una revisión humana constituye una operación distinta y crea una nueva versión trazable.

8. **U no es pendiente, error ni valor estadístico.**  
   En consulta operacional de aprendizaje solo comparece después del agotamiento declarado de una consulta admisible; objetos mal formados o fuera de dominio se rechazan antes.

## 5. Custodia y continuidad: perímetro que no debe perderse

El corolario de reconstrucción y su observación asociada obligan a no colapsar cuatro propiedades que pueden divergir bajo pérdida parcial:

- disponibilidad del soporte histórico originario;
- reconstruibilidad del estado de conocimiento;
- reconstruibilidad de la procedencia o genealogía;
- suficiencia del registro retenido para volver a emitir un veredicto operacional definido sobre el episodio histórico.

La pérdida de un testigo histórico puede coexistir con estado reconstruible. La reconstrucción del estado no implica reconstrucción de procedencia. La pérdida de evidencia retenida puede conducir a `U` en una consulta operacional posterior sin negar retrospectivamente el aprendizaje histórico. Un testigo descubierto después no se convierte retroactivamente en el testigo histórico originario.

Estas diferencias son relevantes para la custodia estructural del diseño, del DSL y de los laboratorios y deberán formar parte del control de persistencia, serialización y reconstrucción cuando se abra el frente correspondiente.

## 6. Contraste con la IR canónica v0.2

La lectura de la IR vigente confirma que ya existen puntos de anclaje aprovechables, pero no una traducción completa del nuevo aparato:

- N3 ya contiene `Frame`, `TransitionData`, `Trajectory` y `Horizon`;
- N4 ya contiene `Domain`, `Agent`, `QuerySpec`, `QueryContext`, `QueryResult` y `AnalyticView`;
- `Trajectory` ya es append-only y `Frame` ya es inmutable;
- `QueryResult` ya exige respuesta tipada, justificación y metadatos;
- la IR vigente no posee todavía un objeto canónico específico para el registro histórico de conocimiento, el episodio de aprendizaje, la política de soporte ni el predicado histórico de aprendizaje;
- la ubicación definitiva de esos objetos entre N0–N4 sigue abierta y no se decide por esta acta.

Por tanto, la publicación no exige desechar la IR v0.2, pero tampoco puede proyectarse sobre ella mediante simples alias nominales.

## 7. Contraste con la gramática superficial mínima v0.1

La gramática vigente ya expresa `trajectory`, `domain`, `agent`, `query_spec` y el operador `query`, y sus consultas bajan a objetos tipados de IR. Sin embargo:

- los tipos de consulta vigentes son `PointEvaluation`, `TrajectoryState`, `FrameComparison`, `CoverageState`, `PendingU` y `GlobalCriticality`;
- no existe todavía una consulta superficial de aprendizaje;
- no existe todavía declaración superficial de episodio, registro de conocimiento, política de soporte ni testigo de soporte;
- el `QueryContext` vigente no contiene una variante específica de episodio de aprendizaje;
- la ausencia de esas formas no constituye un error del DSL actual: constituye un punto de diseño futuro que debe cerrarse contra la doctrina y la IR antes de añadir palabras reservadas.

Queda prohibido resolver esta ausencia copiando directamente nombres, campos JSON o decisiones de serialización del checker de la publicación.

## 8. Interacción con AUTH, REAL/SIM y custodia estructural

La futura traducción al DSL deberá contrastarse simultáneamente con tres frentes ya preservados:

### 8.1. AUTH

`sv-auth-v0.2` conserva A.2 r2 y J6 como sustrato técnico latente, no integrado. El aprendizaje trazable no puede utilizarse para eludir esa frontera: un incremento de conocimiento no equivale a una escalada de autoridad, y una revisión humana de la especificación rectora no es una transición ordinaria de aprendizaje de máquina.

### 8.2. REAL/SIM

La previsión de capa IA trazable por agente exige separación entre historia real y ramas simuladas. Una consulta o simulación que produzca una estructura candidata no puede fabricar por sí sola un suceso histórico real ni un aprendizaje histórico real sin el correspondiente proceso constitutivo y registrado.

### 8.3. Custodia estructural

La evolución de parser, validator, lowering, serialización, consulta, laboratorio o interfaz no podrá bypassar la custodia estructural del diseño y del DSL cuando el frente afectado la exija. En particular, la futura persistencia del registro de conocimiento, de los soportes, de la procedencia y de los códigos de reinicio deberá declarar qué propiedad de continuidad preserva realmente.

## 9. Compuerta obligatoria antes de tocar el DSL

Antes del primer commit que modifique `GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md`, el parser o el lowering para incorporar aprendizaje trazable deberá existir una **matriz doctrina ↔ IR v0.2 ↔ cambio mínimo ↔ juicio de bienformación ↔ error observable ↔ lowering ↔ evidencia/prueba**, y una decisión arquitectónica previa deberá cerrar, como mínimo:

- ubicación tipada del registro de conocimiento;
- representación de historia anterior al episodio;
- tipo y límites del episodio;
- identidad/versionado de la especificación rectora humana;
- forma de declarar política de soporte y completitud relativa;
- forma de representar testigos y procedencia;
- diferencia entre `Learn_D(E)` histórico y `DecLearn_D(E*)` operacional;
- condiciones exactas para `U` operacional;
- interacción con `QuerySpec`, `QueryContext`, `QueryResult` y `Justification` existentes;
- interacción con AUTH, REAL/SIM y custodia;
- compatibilidad hacia atrás;
- catálogo de errores y suite positiva, negativa y adversarial;
- decisión expresa sobre si los códigos de reinicio pertenecen al núcleo de la primera integración o quedan en una capa posterior de persistencia.

La primera tarea del próximo frente DSL será cerrar esa matriz. **No será programar ni añadir palabras reservadas.**

## 10. Rama de continuidad

Se mantiene `traceable-learning-ir-v0.1` como rama técnica latente y no fusionable automáticamente. Su mapa inicial queda preservado como historia de recepción. Se autoriza añadir en esa misma rama una versión documental `v0.2` del mapa que incorpore:

- los identificadores finales/provisionales fijados en esta acta;
- el perímetro de custodia y continuidad;
- el contraste real con IR v0.2 y gramática v0.1;
- la compuerta pre-DSL definida arriba.

La existencia de ese mapa `v0.2` no abre por sí sola el frente implementativo ni autoriza merge a `main`.

## 11. Registro y cierre

Esta fijación deberá quedar asentada como `RETP-2026-047` en el registro maestro y acompañada de un barrido `BARR-2026-005` que deje constancia de la lectura pre-DSL y de la ausencia de integración automática.

**Fuente doctrinal fijada:** SÍ.  
**DOI español fijado:** SÍ — `10.21428/39829d0b.bebc607c`.  
**Code Ocean registrado:** SÍ — `10.24433/CO.4645115.v1`, provisional y en verificación.  
**JAR registrado:** SÍ — Submission ID `6a8347e5-23d3-4f03-87e7-1a8e95e5e594`, v1.0, Technical check.  
**IR modificada:** NO.  
**DSL modificado:** NO.  
**Código ejecutable del Lenguaje SV modificado:** NO.  
**Merge de rama latente a `main`:** NO AUTORIZADO.  
**Siguiente paso legítimo:** matriz pre-DSL y acta de reapertura arquitectónica antes de cualquier cambio de sintaxis, IR, parser o lowering.

Se preserva así lo aprovechable sin confundir preservación con integración: el próximo trabajo sobre DSL deberá comenzar desde la doctrina final y desde el estado real de la IR y la gramática, no desde memoria, nombres de implementación ni atajos acumulados.