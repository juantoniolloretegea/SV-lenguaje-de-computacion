# R1-4 — Permiso y mediación de efectos protegidos

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-4  
**Estado:** abierto

## 1. Objeto

R1-4 materializará la frontera entre el resultado técnico de control cerrado en R1-3 y el compromiso de un efecto protegido.

Su objeto es doble:

1. representar una decisión de permiso ligada a autoridad, forma, efecto y contexto constituidos;
2. asegurar que todo efecto protegido que entre en el alcance de R1-4 atraviese una mediación gobernada suficiente antes de poder comprometerse.

R1-4 no redefine `Tri`, no convierte resultados técnicos en valores ternarios y no atribuye autoridad por el mero hecho de obtener una comprobación favorable.

## 2. Punto de partida

R1-4 se abre después del cierre consolidado de R1-3 sobre:

```text
main previo = d15d93c96095663bcada5c9e567cde7c59345f20
```

con:

```text
R1-2 = CERRADO · INTEGRADO
R1-3 = CERRADO · INTEGRADO
R1-4 = ABIERTO
```

R1-3 entrega resultados técnicos gobernados `D-A`, `D-R` o `D-N`, con requisitos, aplicabilidad, conflicto, cobertura y reutilización ya cerrados en su propio ámbito.

## 3. Separación entre resultado, permiso y ejecución

La apertura fija como invariante:

```text
CheckResult ≠ Permit
Permit ≠ efecto ejecutado
```

Por tanto:

```text
D-A final
↛ Permit por conversión nominal
```

Un resultado `D-A` completo es condición necesaria para una decisión positiva dentro del perfil ordinario de R1-4, pero no es suficiente por sí solo. La decisión deberá comprobar además las ligaduras de autoridad y efecto que correspondan al acto protegido.

Del mismo modo:

```text
D-R → no Permit positivo
D-N → no Permit positivo
error técnico/estructural → no Permit positivo
```

Ninguno de esos estados produce `Tri.U`.

## 4. Identidad material mínima del permiso

Una decisión positiva deberá quedar ligada, como mínimo, a las dimensiones constituidas que resulten causales para el efecto. La primera realización deberá considerar expresamente:

```text
autoridad constituida
+ forma constituida
+ efecto protegido concreto
+ familia de efectos aplicable
+ contexto constitutivo
+ pertenencia al dominio gobernado
+ resultado técnico gobernado de R1-3
```

Si una de estas ligaduras cambia materialmente, un permiso anterior no podrá trasladarse por mera igualdad nominal.

La representación concreta podrá incorporar otras ligaduras ya constituidas cuando resulten necesarias para evitar confusión entre actos materialmente distintos.

## 5. Condición positiva de permiso

Una decisión positiva sólo podrá formarse cuando se acrediten conjuntamente:

1. una autoridad constituida aplicable al efecto;
2. una forma cuya clase, contexto y familia de efectos sean compatibles;
3. el efecto dentro de `E_max` y del dominio gobernado `D_a` de la autoridad;
4. un estado técnico de R1-3 completo y `D-A` para la ligadura material correspondiente;
5. ausencia de una condición de fallo cerrado que invalide la mediación del acto.

No se admite una regla implícita del tipo:

```text
D-A → Permit
```

sin las restantes ligaduras.

## 6. Fallo cerrado de la decisión

La primera realización deberá distinguir entre decisión positiva y ausencia de permiso positivo sin introducir un cuarto valor semántico.

Como mínimo:

```text
D-R → sin permiso positivo
D-N → sin permiso positivo
error de ligadura → sin permiso positivo
error técnico de mediación → efecto no comprometido
```

La ausencia de permiso no constituye por sí sola `D-R`, `D-N` ni `Tri.U`; cada tipo conserva su propio estatuto.

## 7. No fabricación

No deberá existir una API productiva que permita fabricar una decisión positiva a partir de:

- un `CheckResult` aislado;
- una referencia nominal a autoridad;
- una referencia nominal a forma;
- una referencia nominal a efecto;
- un booleano suministrado por el llamador;
- un adaptador o fuente auxiliar;
- la mera posesión de un identificador de permiso.

Los campos que constituyan la decisión deberán ser opacos o privados en la frontera pública ordinaria.

## 8. Mediación del efecto protegido

R1-4 no se limita a calcular una decisión. Debe cerrar la vía entre decisión y compromiso material del efecto.

La propiedad mínima será:

```text
efecto protegido comprometido
⇒ mediación gobernada válida
⇒ permiso aplicable al mismo efecto y ligaduras
```

No debe existir una segunda vía productiva que permita comprometer el mismo efecto protegido sin atravesar esa mediación.

La mediación deberá comprobar la identidad del permiso y del efecto en el punto de compromiso, no sólo en una etapa preparatoria separada.

## 9. Permiso concedido no equivale a efecto ejecutado

Una decisión positiva puede existir y, sin embargo, el efecto no llegar a ejecutarse por una condición material posterior.

Por tanto:

```text
Permit válido
+ fallo material de ejecución
→ efecto no ejecutado
```

sin que ello implique:

```text
Permit → D-R
Permit → Tri.U
Permit → pérdida automática de autoridad
```

La constatación material de ejecución, cuando corresponda, deberá conservarse como hecho distinto del permiso.

## 10. Clases de transición

R1-4 no presupone que toda clase T-* sea productiva por el mero hecho de abrir el corte.

T-I, T-V, T-H y T-E continúan sin constituir autoridad por sí mismas.

T-G, T-C y T-R sólo podrán abandonar su estado no productivo cuando una unidad concreta de R1-4 demuestre, para la clase correspondiente:

1. requisitos R1-3 completos;
2. autoridad aplicable;
3. permiso ligado;
4. mediación no eludible del efecto;
5. fallo cerrado ante cualquier discrepancia relevante.

Hasta esa demostración, permanecen no productivas.

## 11. Reutilización y vigencia del permiso

R1-4 no presumirá que un permiso positivo puede reutilizarse indefinidamente.

La primera realización deberá optar por una representación que impida trasladar un permiso fuera de sus ligaduras. Si se admite reutilización, deberá existir una regla expresa de continuidad equivalente en rigor a la aplicada en R1-3 a los resultados históricos.

La mera persistencia del mismo identificador no bastará para conservar aplicabilidad.

No se introduce tiempo como primitiva semántica. Una condición temporal material, si alguna forma la exige, deberá comparecer mediante una mediación situada y explícita fuera de `Frame` y de la IR soberana.

## 12. Autoridad y permiso

El permiso no crea, amplía ni transfiere autoridad.

```text
Permit
≠ Authority
```

Un permiso sólo puede operar dentro de la autoridad ya constituida. No puede ampliar `E_max`, extender `D_a`, cambiar el titular ni modificar el contexto constitutivo.

Si el efecto excede la autoridad aplicable, la decisión positiva debe ser imposible.

## 13. Fuentes auxiliares

Una fuente auxiliar puede aportar información candidata para las comprobaciones gobernadas, pero no puede emitir por sí misma un permiso soberano de R1-4.

En particular, una salida de un sistema estadístico o heurístico:

```text
≠ Authority
≠ Permit
≠ CheckResult válido por mera emisión
```

La indisponibilidad de una fuente auxiliar o de un canal no produce `Tri.U` y no autoriza una continuación silenciosa cuando el perfil aplicable exige esa fuente.

## 14. Recursos y seguridad material

R1-4 no abre todavía el presupuesto de recursos `BudgetΣ` ni la seguridad material de R3.

La primera realización deberá evitar, no obstante, introducir una dependencia necesaria de un servicio externo para la semántica o para la constitución del permiso.

Los límites de recursos, aislamiento de plataforma, gestión de secretos, defensa de red y perfiles auxiliares pertenecen a cortes posteriores y deberán conservar la soberanía de `sv_core`.

## 15. Pruebas mínimas de cierre de R1-4

R1-4 no podrá cerrarse hasta demostrar, como mínimo, que:

1. un `D-A` nominal no fabrica permiso;
2. `D-R`, `D-N` y los errores técnicos no producen permiso positivo;
3. una referencia nominal a autoridad no fabrica autoridad aplicable ni permiso;
4. el efecto debe pertenecer a `E_max` y `D_a`;
5. forma, efecto y contexto deben coincidir con las ligaduras del permiso;
6. un permiso no puede reutilizarse para otro efecto o contexto;
7. modificar una ligadura material invalida la aplicabilidad del permiso previo;
8. un permiso no amplía autoridad;
9. un permiso válido no equivale a efecto efectivamente ejecutado;
10. un fallo material posterior impide el compromiso del efecto sin fabricar `Tri.U`;
11. toda vía productiva de un efecto protegido atraviesa mediación gobernada;
12. no existe una API paralela que comprometa el mismo efecto protegido sin permiso;
13. T-G, T-C o T-R sólo se hacen productivas en la medida expresamente realizada y probada;
14. las regresiones de R0 y R1-0–R1-3 permanecen correctas;
15. nativo y WebAssembly conservan la misma semántica de control aplicable.

## 16. Primera unidad de realización

La primera unidad material de R1-4 deberá ser deliberadamente estrecha. Se limitará a:

1. fijar tipos cerrados para la decisión de permiso y su ligadura material;
2. derivar una decisión a partir de autoridad constituida y resultado técnico R1-3;
3. demostrar por tipos y pruebas negativas que una decisión positiva no puede fabricarse desde valores nominales;
4. no ejecutar todavía T-G, T-C o T-R si la mediación completa de la clase no está materializada en la misma unidad.

La mediación productiva se añadirá sólo después de cerrar esa frontera de decisión.

## 17. Exclusiones y estado

R1-4 no materializa en su apertura:

- persistencia durable;
- continuidad durable entre procesos;
- recuperación durable;
- presupuesto `BudgetΣ`;
- seguridad de plataforma de R3;
- batería adversarial de Garantía II de R4;
- cliente o motor de inteligencia artificial dentro de `sv_core`;
- Garantía I;
- Garantía II;
- R2, R3 o R4.

Estado de apertura:

```text
R0   = CERRADO
R1   = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = CERRADO · INTEGRADO
R1-3 = CERRADO · INTEGRADO
R1-4 = ABIERTO
R2   = NO INICIADO
R3   = NO INICIADO
R4   = NO INICIADO

Garantía I  = NO_PROBADO
Garantía II = NO_PROBADO
```
