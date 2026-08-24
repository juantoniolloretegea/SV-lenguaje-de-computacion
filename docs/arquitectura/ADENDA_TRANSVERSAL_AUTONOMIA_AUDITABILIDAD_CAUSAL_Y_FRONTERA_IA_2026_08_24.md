# Adenda transversal de autonomía, auditabilidad causal y frontera de IA

**Fecha:** 24/08/2026  
**Ámbito:** `SV-lenguaje-de-computacion`  
**Estado:** decisión arquitectónica complementaria  
**Relación:** complementa `OBJETIVO_RUST_0_BACKEND_SOBERANO.md`, `ESPECIFICACION_ARQUITECTONICA_ENTORNO_EJECUCION_SOBERANO_SV_V0.md` y los contratos SEC.0 aplicables.

## 1. Objeto

Esta adenda fija cuatro condiciones transversales para cualquier realización que pretenda conservar las propiedades del Lenguaje SV:

1. autonomía ejecutiva real del camino soberano;
2. auditabilidad del camino causal material de las garantías reclamadas;
3. frontera entre información externa, causalidad material, autoridad y confianza técnica;
4. confinamiento de los efectos y del consumo material de entradas ya admitidas.

No modifica la gramática, la IR 0.3, `Tri`, `Frame`, C01–C03 ni las clases de transición de SEC.0.

## 2. Autonomía ejecutiva

La autonomía del Lenguaje no se acredita por compilar o probar una biblioteca Rust en un destino nativo.

Para el perfil nativo soberano deberá existir un camino ejecutable suficiente:

```text
programa .svp
→ núcleo Rust soberano
→ análisis léxico y sintáctico
→ AST
→ bienformación
→ descenso a IR
→ operaciones
→ resultado canónico
```

La ejecución del programa no deberá requerir Python, CPython, Pyodide, Node.js, JVM, navegador, herramienta de compilación Rust ni un servicio semántico remoto.

Las dependencias ordinarias del host admitido —sistema operativo, cargador, ABI y bibliotecas materiales declaradas— no constituyen por sí mismas otro lenguaje, pero forman parte del análisis material de la garantía cuando puedan falsificarla.

Quedan excluidas como autonomía final:

- invocación encubierta de Python u otro intérprete;
- inclusión de un runtime ajeno cuya semántica sustituya al núcleo;
- consumo obligatorio de una IR generada previamente por el frontend Python;
- segundo motor semántico lateral;
- descarga posterior de reglas, tablas o componentes que contengan decisiones constitutivas no gobernadas por la misma fuente soberana.

WebAssembly permanece como destino del mismo núcleo, no como sustituto de la autonomía nativa.

## 3. Una semántica y varios adaptadores

Se conserva:

```text
una semántica
una fuente soberana
varios adaptadores materiales
```

Los adaptadores nativo, WebAssembly, interfaz de línea de órdenes, navegador o futuras plataformas pueden introducir dependencias materiales propias. No pueden redefinir:

- `Tri`;
- bienformación;
- C01–C03;
- descenso a IR;
- serialización canónica en su contenido semántico;
- autoridad de clausura;
- reglas constitutivas del mismo programa `.svp`.

## 4. Auditabilidad causal

La auditabilidad se evalúa por garantía concreta y por camino causal, no por la sola inspección del núcleo.

Para una garantía `G` y realización `R`, debe poder identificarse cuando proceda:

```text
fuente
→ construcción
→ artefacto
→ artefacto distribuido
→ artefacto cargado
→ entrada
→ transformación
→ mediación
→ efecto
→ evidencia
```

Todo componente cuyo compromiso pueda hacer falsa `G` dentro del modelo de amenaza declarado pertenece a `TCB(G,R)` o debe quedar excluido mediante evidencia suficiente.

Una capa de transporte, carga, identidad, mediación, actualización, biblioteca dinámica o servicio externo no queda fuera del perímetro por denominarse infraestructura.

## 5. Transparencia y secretos operativos

La seguridad no deberá depender del desconocimiento de los mecanismos del sistema.

Deben poder ser conocidos y auditados, dentro del alcance reclamado:

- algoritmos;
- protocolos;
- transformaciones;
- reglas de autoridad;
- condiciones de admisión;
- tratamiento de errores;
- relaciones de procedencia;
- mecanismos de rotación y revocación;
- dependencias capaces de falsificar una garantía.

La transparencia no obliga a publicar valores secretos de una implantación concreta.

Pueden permanecer confidenciales, cuando proceda:

- claves privadas;
- credenciales;
- tokens;
- semillas secretas;
- material de recuperación;
- otros valores cuya divulgación comprometa la implantación.

Regla:

```text
secreto operativo ≠ mecanismo secreto
```

Un secreto puede proteger una implantación; no puede convertirse en una premisa semántica invisible.

## 6. Causalidad, autoridad y TCB

Se distinguen tres relaciones:

```text
Causal(c,e)
AutoridadSV(c)
c ∈ TCB(G,R)
```

No son equivalentes.

`Causal(c,e)` indica que el componente `c` interviene materialmente en la producción del efecto `e`.

`AutoridadSV(c)` indica que `c` dispone de una facultad legítimamente constituida para producir una clase de efectos dentro de un alcance.

`c ∈ TCB(G,R)` significa que controlar `c`, dentro del modelo de amenaza de `G`, puede hacer falsa la garantía pese a las restantes premisas declaradas.

Por tanto:

```text
sin autoridad
≠
fuera del camino causal
```

pero también:

```text
causal
≠
miembro automático del TCB de toda garantía
```

La pertenencia al TCB siempre debe expresarse respecto de una garantía y un modelo de amenaza concretos.

## 7. Entradas externas e IA auxiliar

Toda salida de una IA, modelo estadístico o servicio externo entra por defecto como información no autoritativa, salvo regla específica previamente constituida que acredite otro estatuto.

La capacidad predictiva, lingüística, perceptiva o estadística no constituye autoridad SV.

Una salida externa puede ser materialmente causal sin adquirir autoridad. Por ejemplo, puede proponer un candidato, una alerta o una extracción que active una regla previamente constituida.

La información externa no puede por sí misma crear o ampliar:

- autoridad;
- regla de admisión;
- regla de bloqueo;
- regla de transducción;
- `Tri`;
- clausura de `U`.

## 8. Admisión, confianza y efectos posteriores

La admisión de una entrada no constituye por sí sola confianza general sobre su contenido ni autorización para reutilizarla fuera del objeto, ámbito, forma o finalidad gobernados que resulten aplicables.

Por tanto:

```text
admisión
≠ confianza
≠ autoridad
≠ libertad de efecto
```

Una entrada admitida continúa sometida a las reglas de ligadura, requisitos, autoridad, acumulación y contexto que gobiernen el efecto posterior.

Cuando la garantía requiera reconstrucción probatoria, deberá poder determinarse con suficiencia qué objeto entró, qué regla permitió su admisión, a qué objeto o ámbito quedó ligado y qué efecto material produjo. Esta exigencia no obliga a incorporar la totalidad del expediente probatorio en `Frame` ni a imponer el coste forense máximo a toda operación.

## 9. Recursos y disponibilidad bajo entradas admitidas

La admisión tampoco constituye un derecho a consumo material ilimitado.

Una realización que invoque servicios externos, verificaciones, reintentos, colas u otras operaciones capaces de ampliar trabajo deberá gobernar ese consumo dentro del perfil aplicable. Las suboperaciones no pueden ampliar unilateralmente la envolvente de recursos concedida a la operación que las origina.

En particular:

```text
información externa
≠ autoridad adicional
≠ presupuesto computacional adicional
```

Los límites materiales pueden depender del perfil y de la realización. No constituyen semántica universal de `Tri` o `Frame` ni introducen un reloj semántico por utilizar temporizadores, ventanas operativas o mediciones de rendimiento.

El agotamiento de recursos:

- no produce `Tri.U`;
- no autoriza éxito por defecto;
- no permite omitir una condición necesaria y mantener el mismo efecto protegido;
- debe impedir el efecto cuyo cumplimiento ya no pueda acreditarse, sin extender innecesariamente el bloqueo a componentes independientes;
- debe preservar, dentro del alcance declarado, capacidad suficiente para rechazo, detención, diagnóstico y recuperación.

Los propios mecanismos de control, registro y observabilidad forman parte del análisis de recursos cuando puedan contribuir al agotamiento.

## 10. Patrón de propuesta y verificación independiente

Cuando una salida externa proponga un objeto cuya corrección sea material para una garantía, podrá utilizarse el patrón:

```text
proponente externo
→ candidato
→ verificador independiente respecto de la propiedad reclamada
→ admite | rechaza
→ efecto sólo tras admisión
```

La independencia no se presume por utilizar dos componentes o dos modelos distintos.

Debe analizarse si existe una causa común capaz de alinear el error, entre otras:

- mismo artefacto o modelo base;
- mismo proveedor o canal de control;
- misma transformación previa;
- mismo conjunto de datos cuando sea causalmente relevante;
- mismo procedimiento de calibración;
- mismo componente capaz de alterar simultáneamente propuesta y verificación.

Si el verificador sólo comprueba la forma del candidato o vuelve a confiar en la misma inferencia que pretende verificar, no existe independencia suficiente respecto de la corrección material del candidato.

## 11. Estadística, reproducibilidad y ternarización

No se deduce:

```text
salida reproducible
⇒ fundamento no estadístico
```

Un modelo con pesos fijados, semilla fija o ejecución determinista puede seguir produciendo una inferencia cuyo fundamento sea estadístico.

Tampoco se deduce:

```text
score / etiqueta / ranking / vector latente
→ función determinista o Ternarizer
→ Tri

por tanto

fundamento SV suficiente
```

El `Ternarizer` no legitima por sí solo la entrada que recibe. La admisibilidad y la suficiencia del observable deben quedar justificadas por la regla de dominio aplicable.

La coincidencia de varios modelos no transforma por cantidad una inferencia en verdad, autoridad o clausura. Puede utilizarse como señal auxiliar bajo una regla gobernada, sin adquirir por ello estatuto normativo propio.

## 12. Tiempo y estadística como datos de dominio

El tiempo, una magnitud estadística u otra información externa pueden formar parte de un dominio que los declare y constituya expresamente.

No constituyen primitivas universales del Lenguaje ni pueden entrar de forma implícita mediante:

- recencia;
- tiempo de espera agotado;
- orden accidental de llegada;
- memoria de un servicio;
- puntuaciones de confianza;
- frecuencia o mayoría;
- configuración oculta de un proveedor.

El tiempo técnico puede utilizarse para medición, forensia, caducidad operativa o evidencia cuando la regla aplicable lo declare. No se deriva de ello una semántica temporal universal de `Frame`.

## 13. Tutela humana

La presencia de una persona en una interfaz no acredita por sí sola tutela humana efectiva.

Cuando una garantía dependa de decisión o revisión humana deberán quedar suficientemente ligados:

- objeto presentado;
- información material para la decisión;
- persona y función;
- capacidad real de admitir o rechazar;
- acto ejecutado;
- efecto resultante.

Una ratificación meramente formal de una salida automática no convierte a la persona en verificador independiente.

## 14. Alcance de garantías

La presente adenda no afirma seguridad absoluta, disponibilidad absoluta ni viabilidad universal.

Una acreditación deberá continuar expresando, según proceda:

```text
G
R
ThreatModel(G,R)
TCB(G,R)
Evidence(G,R)
FailureLimit(G,R)
perfil
artefacto exacto
límites no probados
```

La ausencia de un fallo observado no demuestra que no existan fallos fuera del alcance ensayado.

## 15. Relación con R0–R4

La distribución de responsabilidades permanece:

```text
R0  → semántica soberana y camino funcional autónomo
R1  → autoridad, mediación y fallo cerrado
R2  → persistencia, continuidad y recuperación
R3  → plataforma, construcción, carga, aislamiento y recursos materiales
R4  → ataque integral del SUT exacto
```

R0 no puede considerarse completo si sólo existe una biblioteca Rust probada pero no un camino `.svp` autónomo suficiente para el perfil declarado.

Las políticas de servicios, colas, reintentos, aislamiento o limitación material no se convierten por ello en primitivas de R0.

R3 no queda probado por el mero hecho de compilar o ejecutar en un sistema operativo concreto.

R4 debe atacar el mismo sistema material cuya garantía se reclama; un modo de auditoría distinto de la realización efectiva no sustituye ese requisito.

No se crea una fase R0-9.

## 16. Estado

Esta adenda fija condiciones arquitectónicas. No declara ya demostradas:

- autonomía completa;
- auditabilidad causal de extremo a extremo;
- disponibilidad bajo una carga o modelo de amenaza concretos;
- viabilidad material;
- Garantía I;
- Garantía II;
- aptitud de una IA concreta;
- una primera versión estable.

Estas propiedades deberán acreditarse en las fases y perfiles correspondientes.