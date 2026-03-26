# Nota de hoja de ruta: célula de verificación formal y generación matemática del SV

**Fecha:** 26/03/2026  
**Naturaleza:** nota de hoja de ruta — familia candidata futura de segundo nivel  
**Frente:** colección Células especializadas del Sistema Vectorial SV  
**Estado:** registrada, no activa, no bloqueante  
**Registro técnico asociado:** véase `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`

---

## 1. Objeto

Dejar constancia, sin abrir frente activo ni establecer prohibición nueva, de una idea de familia candidata futura que ha emergido durante el trabajo de consolidación de la colección *Células especializadas del Sistema Vectorial SV* y que merece registro antes de que el sistema avance hacia backend, NLP o células de dominio específicas.

---

## 2. La idea registrada

Se plantea la posibilidad de que, en el futuro, existan dentro del carril de células especializadas células cuya función no sea operar sobre un dominio externo al SV, sino sobre el propio aparato formal del sistema: células orientadas a verificar coherencia del corpus, generar proposiciones candidatas o extender formalmente la matemática y la semántica del SV.

Esta familia tendría dos escalones distinguibles:

**Escalón 1 — Célula de verificación formal.**  
Dado un conjunto de proposiciones del corpus, comprueba coherencia interna, detecta contraejemplos sobre instancias canónicas (SV(9,3) u otras) y emite dictamen estructural. Opera con el aparato ya disponible. No requiere auto-referencia ni modificación del núcleo. Es una especialización de dominio sobre el dominio del propio SV.

**Escalón 2 — Célula de generación matemática.**  
Produce extensiones candidatas del aparato formal: nuevas proposiciones, nuevas familias de propiedades, nuevas condiciones de equivalencia. Requiere que el sistema pueda representar proposiciones candidatas en la IR, que exista una semántica operacional demostrada y que haya un puente formal entre la doctrina y la ejecución.

La propiedad del SV que hace esta idea no trivial es la U como indeterminación honesta: el sistema puede representar estados de conocimiento parcial sobre sus propias proposiciones, lo que permitiría a una célula razonar sobre qué partes del corpus están casi cerradas (predominio de 1), abiertas (predominio de U) o contradichas (predominio de 0 en el vector de evaluación de una proposición). Eso es territorio matemático genuino, distinto de lo que hace un sistema binario.

---

## 3. Por qué no se abre ahora

Esta familia no pasa en el estado actual el perfil funcional mínimo de §4 del documento marco de células especializadas:

- §4.1: dominio no suficientemente delimitado para el Escalón 2.
- §4.2: interfaz paramétrica no declarada.
- §4.3: criterio de dictamen no fijado para el caso generativo.
- §4.5: relación con el aparato de la familia VII directa pero no formalizada.

Adicionalmente, el Escalón 2 requiere condiciones previas no resueltas:

- el puente semántico entre marco VII y Lenguaje SV (D-08 de la deuda viva);
- la semántica operacional mínima del subconjunto estabilizado (D-07);
- la representación de proposiciones candidatas en la IR (no existe todavía).

El Escalón 1 es más próximo: encaja en la familia de soporte experto (§5.3 del documento marco) y no requiere las condiciones anteriores. Puede evaluarse cuando el carril de células especializadas tenga su primera célula de dominio publicada.

---

## 4. Relación con D.10 de la Frontera Normativa

La Frontera Normativa v0 prohíbe en D.10 que ningún proceso automático modifique la relación semántica previa, la topología del grafo de composición, las tablas de admisibilidad ni las políticas del núcleo.

El Escalón 1 no activa D.10: verifica y emite dictamen sobre el corpus existente, no modifica nada.

El Escalón 2, en su forma más ambiciosa, requeriría que las proposiciones generadas sean candidatas validadas por un humano antes de incorporarse al corpus — no introducción automática en el núcleo. Bajo esa condición, D.10 tampoco se activa. Esta condición no es negociable y debe quedar declarada en cualquier formalización futura de esta familia.

---

## 5. Orden correcto de apertura

1. Resolver D-07 (semántica operacional mínima) y D-08 (puente semántico).
2. Publicar una primera célula de dominio en la colección de células especializadas.
3. Evaluar el Escalón 1 (verificación formal) como siguiente candidata de la familia soporte experto.
4. Solo después, y con D-07 y D-08 resueltos, evaluar el Escalón 2.

---

## 6. Fórmula de cierre

La presente nota no abre deuda viva nueva, no bloquea ningún desarrollo en curso y no autoriza ninguna implementación. Su función es exclusivamente registrar, con trazabilidad, una idea de familia candidata futura para que no se pierda y para que, cuando las condiciones sean las correctas, pueda activarse sin tener que reconstruirla desde cero.
