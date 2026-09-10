# Dictamen focal · perfil IE004-ES-P2/2 · RETP-2026-127

**Destinatario:** Juan Antonio Lloret Egea.
**Fecha:** 10 de septiembre de 2026.
**Encargo:** `perfil-es-p2-2/ENCARGO_REVISION_Y_RESERVA_P3.md`, primera salida (§2).
**Alcance:** revisión focal de D1–D3 y R1–R7 contra /2 y sus contrastes. No es una
búsqueda general de frases.

## Dictamen

```
APTO_PARA_RESERVAR
```

Los tres defectos están corregidos documentalmente y los siete reparos
atendidos. No he hallado defecto bloqueante nuevo. La aptitud es **documental**:
no acredita funcionamiento, aislamiento, viabilidad material ni utilidad del
agente.

La segunda salida (§3) queda condicionada a dos extremos que no dependen de mí y
se detallan en §6: la custodia material y la designación de autoría del oráculo.

---

## 1. Declaración exigida (§1 y §5 del encargo)

| Extremo | Declaración |
|---|---|
| Modelo | Sesión configurada para `claude-opus-5`; el modelo que sirve un turno puede diferir y no puedo acreditarlo desde dentro. |
| Sesión | **La misma sesión continuada** de RETP-126. No es un chat nuevo. Conservo en contexto /1, mi dictamen anterior y sus medidas. |
| Configuración observable | Contenedor efímero en la nube de Anthropic; Linux 6.18, Python 3.11.15; sin credenciales de GitHub; clon público superficial del corte `b9dea6b6469b4b79b0323385cf610438d912f0bf`. |
| Participación previa | Alta. Autor de la nota de giro metodológico que originó P2 y del dictamen sobre /1 al que /2 responde. **No soy independiente del linaje de diseño ni de la devolución que provocó esta versión.** Independiente del implementador (Watson) y del participante (Grok). |
| Escrituras | Ninguna en ningún repositorio. |
| Herramienta auxiliar | `p2v2_parser.py`, escrito por mí: traducción sintáctica de `G01–G25` y del léxico §3. **Papel:** regresión de cobertura. **Límite:** no implementa §5, no es autoridad sobre semántica ni sobre Rust, y no ha sido ejecutado por Watson. Una divergencia entre él y el texto se resuelve a favor del texto. |
| Presupuesto de herramientas (guía §5) | Un proceso, ≈2 s de reloj. No he ejecutado generación exhaustiva ni ampliado longitud. Comprobaciones omitidas deliberadamente: enumeración general de cadenas y cualquier medida de recursos, memoria o tiempo del futuro Rust. |

**Ficheros leídos del corte:** `perfil-es-p2-2/{PERFIL_INTERACCION_ES_IE004_CANDIDATO_2.md,
ENCARGO_REVISION_Y_RESERVA_P3.md, CONTRASTES_PUBLICOS.json, GUIA_RECURSOS_Y_LATENCIA.md}`,
`ACTA_RECEPCION_CLAUDE_Y_CORRECCION_P2_2026_09_10.md`,
`perfil-es-p2-1/COBERTURA_DOCUMENTAL.json`, `ACTA_DISENO_P2_…`,
`diseno-p2/MATRIZ_DISCRIMINANTE.json`, `WORKFLOW_ACOTADO_…`,
`ACTA_RECEPCION_CONTEXTO_IE004_Y_DIAGNOSTICO_CAUSAL_…`,
`fuentes/{contrato.json,base.json,receptor.rs}`. No he entrado en el repositorio privado.

---

## 2. Correcciones de mi propio testigo de /1

Las tres observaciones de la recepción §2 son correctas y las asumo.

1. **«61 sondas» era erróneo.** `p2_probes.py` contiene **60** literales. No existe
   una sonda 61 no identificada: fue un error de recuento mío, no una ejecución
   perdida. Rectifico la cifra de mi dictamen anterior.
2. **`p2_ambig2.py` toma 8 por defecto** mientras mi línea de reproducción indica 6,
   y no lleva plazo, tope de memoria ni de cardinalidad. Correcto.
3. **«Estructura intacta» fue una sobreafirmación mía.** La reducción también
   recortó alternativas estructurales: sustituí `Cabeza`, `SujCab` y `Cortesia`,
   dejando `el registro` como única forma de la alternativa de registro y
   partiendo `Cortesia`. La ausencia de hallazgos en ese subconjunto no acredita
   unicidad general. Retiro esa lectura.

**Exposición adicional que debo declarar.** Además de las 60 entradas del fichero,
ejecuté 18 cadenas sueltas que no quedaron en él y que por tanto no constaban como
quemadas. Quedan expuestas y excluidas de cualquier reserva:

```
No cambie el valor de la IgG.            No traiga el valor de la IgG.
Cambie el valor de la IgG.               Escriba 7 en la IgG.
No quiero el intervalo de referencia.    Del caso-a, el valor de la IgG.
De este caso, el valor de la IgG.        El valor de la IgG de este paciente.
El valor de la IgG del caso-b.           Consulte la unidad, no quiero el valor: la IgG.
No consulte la hemoglobina.              El estado registrado de la IgG.
La cifra registrada de la IgG.           Sólo lea el valor de la IgG, por favor.
El valor actual de IgG o IgM del CASO-A. El valor actual de IgG o IgA del CASO-A.
No consulte IgG o IgM; consulte IgA.     Consulte el valor de IgG o IgM.
```

---

## 3. Verificación focal de D1–D3

### D1 · negación de escritura — **corregido**

`G18` alternativa 2 deriva `"no escriba" Entero "en" Grupo`; `G24` separa el verbo y
`G05`/`G18` lo comparten; la fila de §3 se reescribe en consonancia. §5.1 fija el
patrón `(ESCRIBIR, IGG, VALOR, operando_textual="7")` y prohíbe ampliarlo a
«escribir cualquier valor»; §5.2 añade el cotejo del operando y la regla de que una
exclusión ESCRIBIR no puede coincidir con una ruta positiva LEER. El operando queda
como atributo auxiliar tipado, fuera de los cinco campos y del banco.

Comprobado sintácticamente: C01 y C02 derivan; C02 conserva la lectura positiva.

### D2 · disyunción y unicidad — **corregido**

§5.3 separa por primera vez raíz sintáctica de interpretación semántica: átomo → una
interpretación; `A o B` → **unión**; composición conjuntiva → **producto cartesiano**,
conservando combinaciones contradictorias e incompletas. §5.2 añade que los términos
de `G10` no se unifican entre sí. El criterio de igualdad de significados se amplía
con los valores en conflicto y el operando de escritura. P2-09 queda concretada con
ejemplo público.

Los tres ejemplos de §5.3 se sostienen bajo esas reglas: `IgG o IgM` → dos
interpretaciones → `PETICION_AMBIGUA` antes de política; `IgG o IgG` → unión que
deduplica → ruta IGG; `de IgG de IgA` → una interpretación con dos valores
conjuntivos → `PETICION_CONTRADICTORIA`. Comprobado además que la mezcla
—disyunción y conjunción sobre el mismo campo— produce dos significados distintos y
por tanto `PETICION_AMBIGUA`, no contradicción: la precedencia del §5.3 lo resuelve
sin ambigüedad de lectura.

### D3 · presupuestos — **corregido, y más allá de lo pedido**

§6.1 ordena cuatro pasos con A completo antes y sin esperar al certificado; §6.2 fija
dos cuentas con entradas, arenas y trabajo independientes, sin traspaso de saldo,
arena, caché, excepción ni plazo; §6.3 hace **obligatorio** el orden canónico, no
propuesto. El agotamiento de V detiene V y nunca A; el de A produce fallo técnico sin
dato. §6.1.3 permite comprobar la propuesta **después de fijar y entregar el cuerpo**,
con lo que el vector de denegación que describí deja de existir en el algoritmo
candidato.

§6.1.4 y §6.2 declaran por sí mismos lo que esto no prueba: dos contadores lógicos no
acreditan aislamiento del sistema operativo, y los MiB son cotas candidatas, no
medidas. Correcto, y así debe quedar.

---

## 4. Verificación focal de R1–R7

| # | Estado | Comprobación |
|---|---|---|
| R1 | **Corregido y elevado bien.** `no sólo/solo` deja de derivar; la petición entera da `SOLICITUD_NO_REPRESENTADA` sin rescatar fragmento | C06 y C07 no derivan, como se exige |
| R2 | **Corregido.** `G25` representa `del caso que estamos viendo` y sigue rechazando `de el caso…` | C08 deriva, C09 no |
| R3 | **Corregido.** `G13` admite `"de la" Momento` | C10 deriva con una sola derivación |
| R4 | **Corregido.** §3 fija segmentación izquierda-derecha, posición de separadores, terminal multipalabra y `IgGactual` indivisible | C11, con tabulador, salto y CRLF, deriva |
| R5 | **Declarado deliberado**, sin cambio de comportamiento | C12 deriva |
| R6 | **Corregido.** `G02` admite `Consulta, Negacion: Consulta` y `RecorteObjeto, Consulta` | C13 y C14 derivan |
| R7 | **Explicitado.** La operación bajo negación forma la exclusión sin contaminar la positiva; «No elimine la IgG» → `SIN_SOLICITUD_POSITIVA` | C15 deriva |

### Regresión de cobertura

Reejecuté las **48 preguntas públicas de /1** contra la gramática de /2, porque `G13`
perdió `("de"|"para") Objeto` y `"del" ObjetoExplicito` y podía haber roto casos
históricos con objeto:

```
48 / 48 derivan, todas con derivación única.  0 regresiones.
16 / 16 contrastes de /2 derivan o no derivan según su clase esperada.
```

Las cinco familias en riesgo se conservan por vías distintas: `del CASO-B` y
`del CASO-A` por `G25`/1; `de este caso` y `de ese paciente` por `G25`/2;
`para el caso que estamos viendo` por `G13` con `"para" Objeto`;
`porque estoy revisando este caso` por `G20`.

---

## 5. Reparos no bloqueantes de /2

**R8 · Exhibir la ambigüedad por exclusión irrelevante.** Bajo §5.1 —«G10 se evalúa
mediante la expansión semántica de §5.3 tanto en positivo como bajo negación»— una
petición como una negación disyuntiva acompañada de una consulta positiva sobre un
tercer parámetro produce dos significados que difieren **sólo** en una exclusión que
no toca la ruta servida, y el desenlace es `PETICION_AMBIGUA` sin dato. Es coherente
con §5.3 y está cubierto por «no promete resolver la ambigüedad pragmática de
"no A o B"». Pero es contraintuitivo, ningún contraste lo exhibe, y el atajo natural
de un implementador —podar exclusiones que no afectan a la ruta— es justamente lo que
§5.3 prohíbe. Conviene un contraste público que lo fije antes de que alguien lo
«arregle».

**R9 · «No cambie nada» emite dos patrones desde una sola cláusula.** §5.1 dice que
excluye ESCRIBIR **y** ELIMINAR. Con la maquinaria nueva conviene decir expresamente
que es **una interpretación con dos patrones de exclusión**, y no dos
interpretaciones: bajo la segunda lectura, `"No cambie nada; consulte el valor de la
IgG."` saldría `PETICION_AMBIGUA` en vez de `DATO`. El texto sostiene la primera
lectura —la unión está enunciada sólo para `G10`—, pero es el único punto del perfil
donde una cláusula produce más de un patrón y ahora eso cambia el desenlace.

**R10 · La cota A no está acreditada cerca del techo de entrada.** §6.3 obliga a
barridos repetidos hasta punto fijo contando también los intentos repetidos. Con el
tope declarado de 128 tokens hay 8 256 intervalos, y el producto de intervalos,
alternativas y particiones excede holgadamente 1 000 000 de intentos. §6.4 ya declara
las cotas candidatas y no acreditadas, de modo que **no es un defecto**: es un aviso
para la reserva. Una pregunta próxima al techo tendría desenlace indeterminado entre
`DATO` y fallo técnico de A, así que la reserva debe mantenerse lejos de él —el
máximo histórico son 13 tokens— y una reserva corta **no** debe leerse después como
evidencia de que la cota se sostiene.

**R11 · El transporte de la captura P3 debe separar A de V.** §6.2 declara que una
realización no cumple si el transporte obliga a decodificar la propuesta antes de
admitir A. El antecedente `receptor.rs` hace exactamente eso: una sola trama de
256 KiB con pregunta, contexto, permiso, diagnóstico, ruta y apoyos entrelazados. /2
ya retira autoridad a ese receptor, pero el **formato de captura de P3 se fija más
tarde**; si hereda esa trama, §6.1.1 quedará incumplido en la misma captura que debe
cualificarlo. Conviene fijarlo al redactar ese formato, no después.

---

## 6. Condiciones de la segunda salida

El encargo §3 procede, pero dos extremos no dependen del autor del dictamen:

1. **Custodia (§4).** «La custodia debe ser distinta del repositorio/chat al que
   accede Watson o Grok» y «si no puede garantizar esta custodia, indíquelo: la
   reserva queda `NO_ACREDITADA`». Mi entorno es efímero y no constituye ubicación
   recuperable. **Puedo redactar los tres ficheros y la nota de custodia; no puedo
   garantizar la custodia.** Ésta es material y suya.
2. **Autoría del oráculo (§5).** Si redacto `ORACULO_P3.json`, mi revisión posterior
   de ese mismo oráculo no contará como auditoría independiente de su autoría. Es una
   decisión irreversible sobre quién podrá auditar qué después, y corresponde a la
   Dirección, no a mí.

Hasta que ambos extremos se resuelvan: `RESERVA_NO_CREADA`, y no procede compromiso
público, que el encargo condiciona a custodia material.

---

## 7. Lo que este dictamen no acredita

- No acredita funcionamiento, aislamiento ni suficiencia lingüística. `I01–I05`
  siguen pendientes por declaración del propio corpus.
- No acredita viabilidad material: ninguna cota A/V, arena, plazo o paridad
  nativo/WASI ha sido medida aquí, y no he ejecutado ni compilado Rust.
- No acredita ausencia de ambigüedad en /2. He comprobado 48 preguntas históricas y
  16 contrastes públicos; **no** he repetido la enumeración general, por mandato del
  encargo y de la guía.
- Mi traducción sintáctica no es autoridad sobre §5. Los juicios sobre semántica de
  esta revisión son documentales, deducidos del texto de /2.
- No evalúa a Grok ni a Watson, no califica entregas, no propone universo, prioridad
  ni plazo, y no abre la figura de la normalización previa que /2 declara no abierta.
- No crea reserva ni compromiso.
