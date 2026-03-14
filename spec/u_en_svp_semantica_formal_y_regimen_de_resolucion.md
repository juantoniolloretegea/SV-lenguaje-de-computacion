# U en SVP: semántica formal y régimen de resolución

**Derivación técnica subordinada del Sistema Vectorial SV**  
**Subordinado a la publicación canónica:** *Origen doctrinal, definición y alcance de la U en el Sistema Vectorial SV*  
**URL canónica:** https://www.itvia.online/pub/origen-doctrinal-definicion-y-alcance-de-la-u-en-el-sistema-vectorial-sv/release/1?readingCollection=4ebab177

## 1. Naturaleza de este archivo

Este archivo fija la traducción técnica mínima de la **U** dentro del lenguaje de computación del Sistema Vectorial SV (**SVP**).

No constituye fuente doctrinal originaria ni documento soberano. Su función es operativa y de implementación. En caso de discrepancia, prevalecen:

1. **Fundamentos algebraico-semánticos del Sistema Vectorial SV**
2. La serie **Álgebra de composición intercelular del Sistema Vectorial SV**
3. La publicación canónica **Origen doctrinal, definición y alcance de la U en el Sistema Vectorial SV**
4. La **Frontera normativa del lenguaje SV**
5. La presente derivación técnica

## 2. Núcleo semántico en SVP

En SVP, la **U** conserva su significado doctrinal:

> **U = no lo sé**, entendido como **estado epistémico formal de no determinación actual**.

En consecuencia, dentro del lenguaje:

- `U` **no** es `null`
- `U` **no** es `None`
- `U` **no** es `NaN`
- `U` **no** es un entero
- `U` **no** es un booleano
- `U` **no** es probabilidad
- `U` **no** es estadística
- `U` **no** es inferencia opaca

## 3. Tipo ternario irreductible

El lenguaje debe tratar la terna `{0,1,U}` como un conjunto semántico irreductible.

Representación de referencia:

```text
Tri = { Zero, One, U }
```

Lectura canónica:

- `Zero` = `0` = `Apto`
- `One`  = `1` = `No_Apto`
- `U`    = `U` = `Indeterminado`

No se admiten coerciones implícitas desde ni hacia tipos ajenos al régimen ternario.

## 4. Regla de no degradación

Toda implementación de SVP debe impedir que la U sea degradada por conversión silenciosa.

Quedan prohibidas, como semántica canónica del lenguaje, operaciones del tipo:

```text
U -> 0
U -> 1
U -> false
U -> true
U -> null
```

salvo en contextos de **resolución explícita**, trazable y normativamente permitida.

## 5. Resolución de la U

La U no se elimina por mera coexistencia con otros estados ni por comodidad de implementación. Su resolución exige trazabilidad.

Forma técnica de referencia:

```text
resolve(U, context: Context, mechanism: Mechanism) -> Tri
```

Donde:

- `Context` declara el marco relevante de resolución;
- `Mechanism` declara la vía legítima usada para resolver.

### 5.1. Vías legítimas de resolución

La resolución de la U solo puede apoyarse en una de estas tres familias:

1. **criterio experto**
2. **suceso ulterior relevante**
3. **medición suficiente o medición previamente ruidosa ya depurada**

Si ninguna de estas vías concurre de forma suficiente, la salida correcta sigue siendo `U`.

## 6. Encuentro y resolución

SVP debe distinguir con rigor entre:

- **coexistencia o encuentro** de estados;
- **resolución** efectiva de una U.

La mera coexistencia de `U` con `0`, `1` o `U` **no equivale por sí misma** a resolución.

Toda resolución exige:
- fundamento legítimo,
- política de cierre declarada,
- y trazabilidad verificable.

## 7. Retorno a U

Una clasificación previa en `0` o `1` puede retornar legítimamente a `U` si cae el fundamento que sostenía dicha clasificación.

Por tanto, el lenguaje debe permitir representar, registrar y auditar transiciones del tipo:

```text
0 -> U
1 -> U
```

cuando se invalide:
- la medición,
- la admisibilidad,
- la suficiencia contextual,
- o la validez del mecanismo resolutivo.

## 8. Transducción, composición y consulta

### 8.1. Transducción

Si la cadena de captura o admisibilidad no permite cierre legítimo, la salida debe degradarse conservadoramente a `U`.

### 8.2. Composición

La composición con `U` no se rige por una ley universal abstracta única. Depende de:
- la relación semántica previa;
- la tabla explícita del operador;
- el criterio de cierre del dominio.

Ninguna tabla local puede elevarse a ontología general de la U.

### 8.3. Consulta

Una consulta no puede presentar como fuerte una salida `0` o `1` si la cadena relevante permanece afectada por una `U` no resuelta.

## 9. U legítima y U abusiva

El lenguaje y sus herramientas asociadas deben distinguir entre:

- **U legítima**: emitida por no determinación real y formalmente justificada;
- **U abusiva**: emitida por pereza analítica, oscuridad argumentativa o cierre evitado sin causa legítima.

SVP no debe facilitar la segunda.

## 10. Requisitos mínimos para implementación

Toda implementación compatible con esta derivación deberá cumplir, como mínimo, lo siguiente:

- preservar `Tri` como tipo irreductible;
- impedir coerciones implícitas de la U;
- exigir `Context` y `Mechanism` en toda resolución explícita;
- permitir trazabilidad de permanencia, resolución y retorno a `U`;
- no usar estadística, minería de datos ni inferencia opaca como fundamento primario de decisión;
- mantener subordinación expresa a la doctrina canónica publicada.

## 11. Ubicación de referencia en el ecosistema SV

Tras la publicación canónica en `itvia.online`, la presente derivación técnica vive en el repositorio del lenguaje, dentro de:

```text
SV-lenguaje-de-computacion/spec/
```

Su pieza doctrinal principal y soberana vive en el ecosistema doctrinal del SV, dentro de:

```text
SV-matematica-semantica/especificaciones/
```

## 12. Cláusula final

Este archivo existe para traducir la doctrina canónica de la **U** al plano técnico de SVP sin reducir su densidad semántica ni traicionar su función de honestidad algebraica.

Su misión no es reinterpretar la U, sino impedir que la implementación la empobrezca.
