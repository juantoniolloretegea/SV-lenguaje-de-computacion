# Dictamen de suficiencia · perfil IE004-ES-P2/1 · RETP-2026-126

**Destinatario:** Juan Antonio Lloret Egea.
**Fecha:** 10 de septiembre de 2026.
**Encargo:** `ENCARGO_REVISION_Y_RESERVA_P3.md`, primera salida (§2).

## Dictamen

```
DEFECTO_DE_ESPECIFICACION
```

La reserva de P3 queda detenida. Tres defectos impiden redactar expectativas
determinadas, y cada uno recae sobre una obligación distinta de la tabla del
§3 del encargo. El testigo público de revisión se aporta en §4; no podrá
reutilizarse como caso reservado.

---

## 1. Declaración exigida por el encargo (§1 y §5)

| Extremo | Declaración |
|---|---|
| Modelo | Sesión configurada para `claude-opus-5`. El modelo que sirve un turno puede diferir de esa configuración y no puedo acreditarlo desde dentro. |
| Sesión | No es una conversación nueva. Sesión larga y continuada con el Director sobre este corpus, con contexto acumulado y una compactación previa. |
| Configuración observable | Contenedor efímero en la nube de Anthropic; Linux 6.18, Python 3.11.15; sin credenciales de GitHub; acceso al corte por clon público superficial del commit fijado. |
| Participación previa **material** | Alta y relevante. En esta misma sesión emití la `NOTA_DE_GIRO_METODOLOGICO_TUBERIAS_IA_Y_MEMO_PARAMETRICO` (10/09/2026), que argumentaba fijar los campos consultables antes que la superficie española y ejercer las prohibiciones §3 de los Pilares contra el banco. El giro que Watson aceptó y que originó P2 recoge ese argumento. **No soy independiente del linaje de diseño de lo que reviso.** |
| Alcance de independencia | Independiente del implementador (Watson) y del participante (Grok): no he escrito el analizador, el corrector ni ninguna entrega de Grok. No independiente respecto del diseño. Conforme al §5 del encargo, si redactase después el oráculo, mi revisión posterior de ese oráculo no contaría como auditoría. |
| Escrituras | Ninguna. No he creado ramas, commits, incidencias, comentarios ni solicitudes de fusión. Trabajo en copia temporal aislada. |

### Corte y ficheros leídos

Corte: `SV-lenguaje-de-computacion@584d5cc58ba12404548592ffc350d6829b12daac`.

| Fichero | bytes | sha256 |
|---|---|---|
| `…/perfil-es-p2-1/PERFIL_INTERACCION_ES_IE004_CANDIDATO_1.md` | 21 505 | `6ba8b9b9…1aa03678` |
| `…/perfil-es-p2-1/ENCARGO_REVISION_Y_RESERVA_P3.md` | 8 854 | `0a37525c…dab09e55` |
| `…/perfil-es-p2-1/COBERTURA_DOCUMENTAL.json` | 66 668 | `e350b7ee…f1c9d542` |

Leídos además, del mismo corte: `perfil-es-p2-1/MANIFIESTO.json`,
`ACTA_DISENO_P2_…`, `ie004/diseno-p2/MATRIZ_DISCRIMINANTE.json`,
`WORKFLOW_ACOTADO_SUBORDINACION_IA_ES_…`,
`ACTA_RECEPCION_CONTEXTO_IE004_Y_DIAGNOSTICO_CAUSAL_…`,
`ie004/fuentes/contrato.json`, `ie004/fuentes/base.json`,
`ie004/fuentes/receptor.rs`.

No leído: el repositorio privado. No hizo falta; los enlaces relativos del
corte bastaron, como el encargo anticipa.

---

## 2. Lo que la revisión acredita a favor del perfil

Estas cifras salen de ejecutar la gramática, no de leerla. Es, hasta donde
consta en `COBERTURA_DOCUMENTAL.json` (`ejecuciones_gramatica: 0`), la
primera ejecución de `G01–G23`.

| Medida | Resultado |
|---|---|
| Preguntas del corpus documental (48) que derivan de `G01` | **48 / 48** |
| De ellas, con **exactamente una** derivación | **48 / 48** |
| Preguntas con más de una derivación | **0** |
| Cadenas generadas desde `Peticion` con vocabulario reducido, longitud ≤ 6 | 32 871 |
| De ellas, con derivaciones ≠ 1 | **0** |
| Sondas dirigidas adicionales | 61, ninguna ambigua |
| Máximo real medido | 80 bytes, 13 tokens no separadores (topes §6: 8 192 y 128) |

Es un resultado sólido y no era el esperable: una gramática escrita a mano con
23 producciones, artículos opcionales, repetición libre de complementos y seis
formas de cuerpo **no presentó ni una sola ambigüedad sintáctica** en lo
explorado. La cobertura del corpus histórico es total. Los topes de longitud
del §6 se confirman holgados; la afirmación de `medicion_documental` es exacta.

También son correctas, y conviene decirlo: la separación entre `Tri` y los
diagnósticos candidatos del puesto (§5.3), la prohibición de convertir avería,
ambigüedad o referencia no disponible en `U` (§5.3), el tratamiento de la
propuesta externa como evidencia no confiable (§2), y el «Alcance de esta
decisión» del §2, que impide leer *ownership* como permiso o soberanía. Ese
párrafo cierra por sí solo la primera mitad de la pregunta 4 del encargo.

---

## 3. Defectos bloqueantes

### D1 · `escriba` se declara denegable y la gramática no lo deriva bajo negación

El §3 clasifica `escriba` y `cambie` como «Escritura reconocida … ESCRIBIR,
**representable para denegación**, sin ejecutor de escritura». El §5.1 confirma
que «No cambie nada» excluye ESCRIBIR y ELIMINAR.

Pero `G05 Orden` enumera `consulte | lea | traiga | dígame | necesito | cambie |
elimine`. **`escriba` no está.** Aparece sólo como terminal suelto en `G04`
alternativa 2 (`"escriba" Entero "en" Grupo`), y `G18 Negacion` se construye
sobre `Orden`, no sobre `Nucleo`. Medido:

```
"No escriba 7 en el valor de la IgG."   → 0 derivaciones
"No cambie el valor de la IgG."         → 1 derivación
"No elimine el valor de la IgG."        → 1 derivación
```

La forma española natural para denegar la escritura con su contenido es
underivable, mientras las otras dos operaciones sí lo son. El §3 promete una
representabilidad que el §4 no entrega.

**Obligación afectada:** «Política y conocimiento · Escritura/lectura
excluida» (§3 del encargo).

### D2 · La disyunción `G10` recibe tres tratamientos incompatibles

`G10 Parametro = AtomoParametro | AtomoParametro "o" AtomoParametro`.

- §5.1: «Las alternativas G10 producen **interpretaciones distintas** tanto en
  positivo como bajo negación».
- §5.2: «Dos valores distintos para el mismo campo producen **contradicción**»
  → orden 1, `PETICION_CONTRADICTORIA`.
- §5.3: `PETICION_AMBIGUA` se decide sobre «**cada raíz completa** … Si hay más
  de un significado». Medido: `"El valor actual de IgG o IgM del CASO-A."`
  produce **una sola raíz**.

Con una raíz, la normalización del §5.3 produce un significado y el desenlace
sería `PETICION_CONTRADICTORIA`; con la lectura del §5.1 serían dos y el
desenlace `PETICION_AMBIGUA`. El perfil no declara cuál. Dos implementaciones
conformes dan diagnósticos distintos sobre la misma cadena.

**Y no es un rincón.** La matriz `P2-09` fija la obligación «Unicidad antes de
permisos» como «Petición que admite dos raíces completas distintas en el
perfil, una permitida y otra no», sin exhibir instancia. No la encontré por
otra vía: en 32 871 cadenas generadas y 48 preguntas del corpus no hay ninguna
ambigüedad sintáctica. **La disyunción `G10` es el único mecanismo del perfil
capaz de producir dos lecturas con permiso distinto** —`IGG o IGM`, con
`P-IE004/1` admitiendo IGG y no IGM— y es justamente el mecanismo cuyo
desenlace queda sin fijar.

**Obligación afectada:** «Ambigüedad y cobertura · Dos lecturas admisibles,
incluso si sólo una tiene permiso» (§3 del encargo). Sin D2 resuelto, esa
obligación no puede cubrirse con un caso reservado ni declararse cubierta.

### D3 · No está declarada la contabilidad entre verificación de la propuesta y análisis independiente

El §6 exige que la propuesta «no puede … consumir el presupuesto reservado al
análisis confiable», y añade que «si un certificado falla y el análisis
independiente puede completarse, se conserva el servicio legítimo». Pero
enumera un único presupuesto —16 384 estados y 1 000 000 de intentos de
aplicación de regla— y la única regla de imputación que fija es la de
duplicados de recepción. **No dice qué parte queda reservada, ni cómo se
imputa el trabajo de comprobar el certificado.**

RETP-125 ya obliga a que el agotamiento produzca fallo técnico sin dato y a que
«el presupuesto no puede seleccionar silenciosamente una interpretación», y esa
doctrina se conserva. Lo que no se conserva es la propiedad que el encargo
pregunta expresamente: *«¿Los límites o el orden de análisis permiten que el
participante cambie el resultado?»* Con la contabilidad sin declarar, una
propuesta grande pero válida —256 KiB, 512 nodos, 1 024 enlaces, profundidad
64— puede agotar el presupuesto compartido y convertir una consulta legítima
en fallo técnico. Es exactamente el fallo que RETP-125 §51 nombra: «un
verificador que sólo rechaza propuestas malas … permite denegar cualquier
consulta legítima».

A esto se añade que el orden canónico del §6 se introduce con «**se propone**»,
no como exigencia.

**Obligación afectada:** «Propuesta adversa» (§3 del encargo). Una mutación
adversa que agote presupuesto no tiene expectativa determinada.

---

## 4. Testigo público de revisión

Conforme al §2 del encargo, el defecto se concreta con material público y
reproducible, que **no podrá reutilizarse como caso reservado de P3**:

| Fichero | Qué es |
|---|---|
| `p2_parser.py` | Implementación literal del léxico §3 y de `G01–G23` §4. Cuenta derivaciones; **no implementa la semántica §5**. |
| `p2_probes.py` | 61 sondas dirigidas con su recuento de derivaciones. |
| `p2_ambig2.py` | Generador de cadenas con vocabulario reducido y verificación con el analizador completo. |

Reproducción: clonar el corte fijado y ejecutar
`python3 p2_parser.py <ruta>/COBERTURA_DOCUMENTAL.json`,
`python3 p2_probes.py` y `python3 p2_ambig2.py 6`.

**Las 61 cadenas de `p2_probes.py` y las 48 del corpus quedan quemadas como
material inédito.** Cualquier reserva posterior debe evitarlas.

**Decisiones que tomé porque el perfil no las fija** (ver R4): los separadores
se descartan entre tokens y no aparecen en ninguna producción. Con otra
decisión, los recuentos de esta tabla podrían variar.

---

## 5. Reparos no bloqueantes

| # | Reparo | Referencia | Evidencia |
|---|---|---|---|
| R1 | `no sólo consulte X` deriva como `Negacion` (`G18`→`G05`, que admite `["sólo"\|"solo"]`) y el §5.1 lo trataría como exclusión de `(LEER, X)`. En español significa lo contrario: pide **más**, no menos. | §5.1 · `G05` · `G18` | `"No sólo consulte la IgG."` → 1 derivación, como negación |
| R2 | `del caso que estamos viendo` **no deriva**; sí deriva `de el caso que estamos viendo`, que es agramatical. `G13` sólo empareja `"del"` con `ObjetoExplicito`. | `G13` · `G16` | `"El valor del caso que estamos viendo."` → 0; `"…de el caso…"` → 1 |
| R3 | `de la anterior` **no deriva**; sí `del anterior` y `la anterior`. `G17 Momento` no admite artículo y `G12 Temporal` sí. Asimetría no declarada dentro del mismo paradigma. | `G12` · `G13` · `G17` | `"El valor de la IgG de la anterior."` → 0 |
| R4 | Los separadores son tokens —el §6 cuenta «128 tokens **no separadores**»— pero ninguna producción del §4 los menciona. Dónde pueden aparecer queda sin declarar; afecta a cualquier caso con salto de línea o espaciado múltiple. | §3 · §4 · §6 | decisión propia obligada, ver §4 |
| R5 | `G01` hace independientes `["¿"]` y `["?" \| "."]`: admite `¿… .` y `… ?` sin apertura. Laxitud no declarada. | `G01` | ambas → 1 derivación |
| R6 | `G02` alternativa 6 exige `Grupo` como primer miembro, no `Consulta`: `"Consulte la unidad, no quiero el valor: la IgG."` → 0, mientras `"El valor, no quiero la IgA: la IgG."` → 1. Y el objeto antepuesto positivo (`"Del CASO-A, el valor actual de IgG."` → 0) sólo existe dentro de la forma negativa completa de `G02` alt. 7, que es la mitad de R15. | `G02` | medido |
| R7 | El desenlace de `no elimine X` es determinable —orden 2 (`SIN_SOLICITUD_POSITIVA`) precede al orden 3 (`OPERACION_NO_ADMITIDA`)— pero exige que el lector reconstruya el razonamiento. Conviene una línea explícita sobre si la marca de operación no admitida sobrevive bajo negación. | §5.1 · §5.3 | — |

---

## 6. La quinta pregunta del §2: qué aporta el agente

El §7 del perfil no la responde y lo declara: «Si el análisis completo hace al
modelo prescindible sin utilidad medible, la arquitectura de colaboración sigue
sin acreditarse». Eso es honesto y no bloquea la reserva.

Lo que la medición añade, como observación y no como dictamen: sobre este
repertorio cerrado, el analizador solo resuelve las 48 preguntas conocidas con
derivación única y no encontré ninguna ambigüedad que exigiera arbitraje. En
ese perímetro, la aportación del modelo no es sólo no acreditada: **no tiene
dónde manifestarse**. Si se quiere acreditarla, tendrá que ser sobre formas que
hoy dan `SOLICITUD_NO_REPRESENTADA` —y entonces la propuesta del agente no
sería una derivación, sino una normalización previa a la cadena, que es otra
figura y exigiría su propio contrato.

---

## 7. Lo que este dictamen no acredita

- No acredita funcionamiento, aislamiento ni suficiencia lingüística: no hay
  corrector, y `I01–I05` siguen pendientes por declaración propia del corpus.
- No acredita que la gramática sea inambigua. Acredita que no hallé ambigüedad
  en 48 preguntas del corpus, 61 sondas dirigidas y 32 871 cadenas generadas
  con vocabulario reducido hasta longitud 6. La generación con vocabulario
  completo y la longitud 7 no terminaron dentro del presupuesto de esta
  revisión.
- No implementa ni comprueba la semántica del §5. Los defectos D1–D3 son de
  especificación, deducidos del texto y confirmados sintácticamente donde
  procedía; no son resultados de un motor semántico.
- No evalúa a Grok, no evalúa a Watson, no califica ninguna entrega y no
  propone universo, prioridad ni plazo.
- No crea reserva. `SOLICITUDES_P3.json`, `ORACULO_P3.json` y
  `COMPROMISO_P3.json` **no existen**: el encargo ordena detener la reserva
  ante defecto bloqueante, y así se ha hecho.
