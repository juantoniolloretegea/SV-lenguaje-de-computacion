# Traceable Learning — mapa de continuidad futura para IR N3/N4

**Versión:** v0.1  
**Fecha:** 15/08/2026  
**Rama:** `traceable-learning-ir-v0.1`  
**Estado:** latente legítimo; documentación prospectiva; sin autorización de integración  
**Fuente doctrinal futura:** publicación de fundamentos “Traceable Learning in Artificial Intelligence: Structural Knowledge Evolution with Ternary Frames and Cumulative Traces”  
**Acta de gobierno:** `docs/calidad/ACTA_TECNICA_DE_RECEPCION_DOCTRINAL_Y_PRESERVACION_DE_CONTINUIDAD_DEL_APRENDIZAJE_TRAZABLE_HACIA_IR_N3_N4_2026_08_15.md`

## 1. Propósito

Este documento conserva el puente de trabajo entre la publicación de fundamentos sobre aprendizaje trazable y una eventual continuidad futura del Lenguaje SV.

No define sintaxis, no modifica la IR canónica vigente y no constituye propuesta de merge. Su objetivo es que, cuando se reabra el frente IR/DSL, el trabajo doctrinal ya publicado no tenga que ser redescubierto ni reinterpretado desde memoria.

## 2. Principio de traducción

La secuencia obligatoria será:

`publicación doctrinal congelada → mapa de obligaciones → diseño IR tipado → juicios de bienformación → validator → lowering/DSL → runner → evidencia`

No se invertirá esa secuencia para hacer que una sintaxis elegida por comodidad determine retrospectivamente la semántica.

## 3. Correspondencia preliminar con la IR v0.2

| Objeto de la publicación | Función semántica | Región IR actualmente más próxima | Estatuto |
|---|---|---|---|
| frame `F_r` + vector `v_r` | manifestación estructural inmutable | N3 `Frame` con referencias a estados/resultados de niveles inferiores | compatible como punto de anclaje; no copiar notación sin revisión |
| traza acumulativa `Γ_{0:r}` | historia estructural append-only | N3 `Trajectory` | afinidad directa; debe preservarse la distinción entre trayectoria de sistema y ledger cognoscitivo |
| ledger cognoscitivo `L_r` | historia inmutable de altas, retiradas, ejecuciones y procedencia cognitivamente relevantes | sin objeto canónico actual específico | candidato futuro N3, subordinado a tipado y regla de enlace con `Trajectory` |
| conocimiento manifestado `(X,R,Λ)` | proyección activa de contenidos, relaciones y rutas | N4 / plano de uso y consulta | candidato de proyección de uso; no debe confundirse con el ledger histórico |
| episodio `E_{i:j}` | unidad finita de comparación bajo dominio y fundación fijados | frontera N3/N4 | candidato a objeto N4 que referencia un segmento N3 y una versión de fundación |
| `Act_D` | fold determinista del ledger hacia conocimiento activo | semántica futura de uso | no integrar hasta cerrar tipos, precondiciones y errores |
| testigo de soporte `W_a` | certificado finito y reconstruible de una incorporación | N4 `Justification` / futura estructura tipada de soporte | afinidad, no identidad |
| política `ρ_E` | frontera finita de soporte admisible y completitud relativa | N4 `QuerySpec/QueryContext` + gobierno de dominio | requiere diseño expreso; nunca inferida a posteriori |
| `Evol_D(E)` | existencia de delta cognoscitivo en el sufijo del ledger | N4 resultado de análisis sobre N3 | candidato futuro de resultado tipado |
| `Inc_D(E)` | incorporaciones frescas y soportadas | N4 resultado estructurado | candidato futuro de resultado tipado, no un escalar |
| `Learn_D(E)` | predicado histórico fuerte de aprendizaje | N4 consulta | candidato futuro de respuesta booleana certificada sobre episodio completo |
| `DecLearn_D(E*)` | `LEARN / NO_LEARN / U` tras consulta operacional agotada | N4 `QueryResult` | encaje fuerte con consulta ternaria; `U` no es error ni pendiente no agotado |
| fundación humana `F_h^D` | dominio, semántica, operadores, cierre y región de validez fijados por autoridad humana | presión transversal N0/N4 | no forzar ubicación antes del diseño; requiere identidad/versionado fuerte |
| `Rev_h` | revisión humana de fundación | control de versiones y gobierno | operación externa a transiciones máquina; no bajar como permiso implícito de self-modification |
| restart code `Q_r` | reconstrucción determinista de estado | futura infraestructura de persistencia | fuera del núcleo mínimo de primera integración salvo necesidad demostrada |

## 4. Obligaciones N3 que deberán resolverse antes de tocar DSL

### N3-A — Ledger cognoscitivo explícito

Debe decidirse si la IR incorpora un objeto específico de ledger o si utiliza una realización canónica sobre `Trajectory`. No se permite que una decisión de almacenamiento borre la separación semántica entre:

- historia del sistema;
- historia cognoscitiva;
- proyección activa de conocimiento.

### N3-B — Anclaje inmutable

Todo registro cognoscitivo que refiera un frame, transición, ejecución o entrada externa deberá hacerlo mediante referencia estable. No se permitirá inserción retrospectiva en un ledger previo.

### N3-C — Historia anterior al episodio

La frescura no puede calcularse únicamente a partir del conocimiento activo al inicio del episodio. La realización deberá conservar o referenciar las clases históricamente adquiridas antes del límite `i`, de modo que una recuperación antigua no se fabrique como incremento nuevo.

### N3-D — Límites de episodio

Los límites `i < j` son ordinales, no temporales. Un episodio deberá referir un segmento bien formado de trayectoria y una única versión de fundación. Una revisión de fundación no puede quedar oculta dentro del mismo episodio.

### N3-E — Ejecuciones relevantes

Cuando una ejecución de razonamiento o composición participe en soporte, su ocurrencia concreta deberá poder serializarse y reconstruirse. La mera presencia del operador abstracto no basta para atribución.

## 5. Obligaciones N4

### N4-A — Consulta histórica frente a consulta operacional

La IR de uso deberá distinguir al menos:

- la proposición histórica fuerte `Learn_D(E)` sobre episodio completo y registro de soporte completo;
- la consulta presente `DecLearn_D(E*)`, capaz de devolver `U` solo después de agotamiento declarado de la base accesible.

No se admitirá `U` como sustituto de error de entrada, campo ausente, objeto fuera de dominio o trabajo todavía no agotado.

### N4-B — Soporte y completitud relativa

La política `ρ_E` deberá estar declarada antes del veredicto y formar parte del contexto autorizado de la consulta. Una política modificada después de conocer el resultado constituye otra instancia de consulta/episodio y debe quedar versionada.

### N4-C — Justificación

Un resultado positivo de aprendizaje deberá poder referenciar los incrementos concretos que lo sustentan y, para cada uno, al menos un testigo válido registrado. La ausencia de un testigo bajo una vista incompleta no puede convertirse automáticamente en negación histórica.

### N4-D — Procedencia

La procedencia de contenido, relación, ruta y decisión debe sobrevivir a composición y transducción. El paso por una operación máquina no transforma una decisión humana en derivación propia de la máquina.

### N4-E — Identidad de fundación

La igualdad de identificador no será suficiente si el contenido puede mutar bajo el mismo nombre. Una futura realización deberá utilizar identidad canónica, versión inmutable o digest vinculado al contenido de la fundación.

## 6. Banco mínimo de pruebas que debe sobrevivir a la traducción futura

La futura integración no deberá comenzar copiando el checker de la publicación. Deberá derivar primero la semántica IR y después construir pruebas propias.

Como mínimo deberán reaparecer, adaptados al contrato final:

1. adquisición nueva de objeto soportado;
2. aprendizaje exclusivamente por relación;
3. aprendizaje exclusivamente por ruta;
4. pérdida sin aprendizaje;
5. razonamiento ejecutado sin aprendizaje;
6. adquisición sin razonamiento interno;
7. composición que participa en soporte y produce aprendizaje;
8. composición ejecutada sin aprendizaje;
9. composición que participa pero no es esencial porque existe un testigo alternativo;
10. reapertura de coordenada fuerte a `U` con aprendizaje y contramodelo de pérdida pura con los mismos extremos locales;
11. reexpresión representacional equivalente sin incremento;
12. consulta agotada con registro retenido insuficiente → `U`;
13. intento de reescritura de fundación por máquina → rechazo;
14. recuperación de clase conocida sin segundo incremento;
15. recuperación acompañada de una ruta genuinamente nueva;
16. mutación fresca sin soporte → evolución sin aprendizaje;
17. clase adquirida y retirada antes del episodio, recuperada durante el episodio → no fresca;
18. identidad de fundación alterada bajo el mismo rótulo superficial → rechazo.

La numeración no obliga a conservar los fixtures actuales ni su formato. Conserva las propiedades que deberán ser adversarialmente visibles.

## 7. Preguntas arquitectónicas que permanecen abiertas

No están resueltas por esta rama:

- ubicación definitiva del ledger en N3 o como estructura subordinada especializada;
- relación exacta entre el conocimiento manifestado y el `Frame` completo del sistema;
- forma canónica de equivalencia representacional por tipo;
- catálogo de errores específico;
- serialización de políticas de soporte;
- granularidad de objetos de procedencia;
- representación de fundación humana y revisiones `Rev_h`;
- posible incorporación de restart codes;
- sintaxis superficial del DSL;
- lowering hacia la IR;
- interacción con AUTH preservado en `sv-auth-v0.2`;
- interacción con REAL/SIM, custodia estructural y precedencia global de capas.

Toda respuesta futura deberá apoyarse en la publicación doctrinal final y en una acta de reapertura aprobada.

## 8. Regla de no apropiación de la realización auxiliar

La cápsula/checker que acompaña la publicación es una realización pequeña de ejemplos y regresión. No gobierna esta rama y no constituye especificación del lenguaje.

Si se reutiliza alguna estructura suya, deberá justificarse contra la semántica doctrinal y contra la IR vigente. Ningún JSON, nombre de campo o decisión de implementación del checker adquiere rango canónico por el hecho de existir.

## 9. Punto de continuación cuando se reabra

La primera tarea futura no será programar. Será construir una **matriz doctrina ↔ IR v0.2 ↔ cambio mínimo necesario ↔ juicio de bienformación ↔ error observable ↔ prueba adversarial**.

Solo después de cerrar esa matriz deberá decidirse si procede:

- extender N3;
- extender N4;
- crear objetos nuevos de N0/N2;
- o mantener parte del aparato como capa externa de biblioteca sin modificar la IR nuclear.

## 10. Cierre

Esta rama preserva continuidad, no implementación.

**Doctrina fuente:** superior.  
**IR vigente:** no modificada.  
**DSL vigente:** no modificado.  
**Código ejecutable:** no modificado.  
**Merge a `main`:** no autorizado.  
**Siguiente paso legítimo:** esperar a la versión doctrinal final y, cuando proceda, abrir acta arquitectónica de traducción.
