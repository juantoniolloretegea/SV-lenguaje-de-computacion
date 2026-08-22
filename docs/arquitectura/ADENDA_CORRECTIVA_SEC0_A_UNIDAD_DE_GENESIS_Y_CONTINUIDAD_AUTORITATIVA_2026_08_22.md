# Adenda correctiva a SEC.0-A — unidad de génesis y continuidad autoritativa

**Fecha:** 22/08/2026  
**Estado:** corrección arquitectónica cerrada  
**Ámbito:** Lenguaje SV — SEC.0-A  
**Documento afectado:** `CONTRATO_ABSTRACTO_DE_AUTORIDAD_CONSTITUCION_Y_GENESIS_SEC0_A_V2_2026_08_21.md`

## 1. Objeto

Esta adenda corrige una ambigüedad del contrato SEC.0-A relativa a T-0. La expresión «primer estado legítimo de autoridad de una instancia» permitía interpretar que el nacimiento técnico de un proceso, réplica, contenedor, máquina virtual, reinicio o identificador de instancia podía constituir una nueva génesis aun cuando ese ejecutor se incorporase a una continuidad autoritativa ya existente.

La corrección fija la unidad de génesis en la **continuidad autoritativa**, no en la identidad efímera del ejecutor material.

No se modifican las restantes clases T-*, las magnitudes de autoridad, el régimen de delegación, las reglas de verificación, las reservas humanas ni los demás invariantes de SEC.0-A salvo en cuanto deban interpretarse de conformidad con esta unidad de génesis.

## 2. Regla corregida de T-0

T-0 constituye exclusivamente el primer estado legítimo de autoridad de una **continuidad autoritativa aún no habitada**.

Una continuidad deja de ser no habitada desde que contiene o admite autoridad, estado autoritativo persistente, una instancia vinculada a esa autoridad, un `AStore` que contenga o fundamente estado autoritativo previamente admitido, dependencias `PDep` ya constituidas relevantes para autoridad o un antecedente legítimo susceptible de recuperación. La mera existencia física de almacenamiento vacío, estructuras sin autoridad admitida o una identidad técnica preparada para el primer arranque no consume por sí sola la génesis.

Por tanto, no crean una nueva T-0 por sí mismos:

- el nacimiento de un proceso;
- un nuevo contenedor o máquina virtual;
- una réplica o copia;
- un `fork`;
- un reinicio;
- un cambio de `instance_id`, `boot_nonce` o identificador equivalente;
- la reubicación o reconstrucción técnica de una realización;
- el montaje, restauración o incorporación de estado autoritativo perteneciente a una continuidad ya habitada.

Si cualquiera de esas operaciones actúa sobre una continuidad ya habitada, T-0 no está disponible. La identidad técnica nueva no altera por sí sola la clase T-* del acto. Si el acto restaura autoridad preexistente, modifica autoridad o altera la constitución, deberá clasificarse como T-R, T-G o T-C según su efecto. Si sólo informa, verifica, habilita o ejerce autoridad ya existente, conserva la clase ordinaria T-I, T-V, T-H o T-E que corresponda.

## 3. Disponibilidad de génesis

La disponibilidad de T-0 depende de la continuidad autoritativa, no del ejecutor material:

```text
T0_disponible(C) ⇔ C es una continuidad autoritativa aún no habitada
```

En consecuencia:

```text
C ya habitada ⇒ T0_disponible(C) = false
```

La creación de una identidad técnica distinta no altera esta condición.

Una continuidad genuinamente nueva puede tener su propia T-0 únicamente cuando no herede, monte, restaure, prolongue ni utilice como fundamento de autoridad estado autoritativo perteneciente a otra continuidad ya habitada.

## 4. Efecto sobre el dominio gobernado `D_a`

La cláusula de SEC.0-A §3.3 según la cual la constitución inicial de `D_a` puede realizarse por T-0 queda restringida a la T-0 definida en esta adenda.

Por tanto, un nuevo proceso, réplica, reinicio o identificador de instancia no puede ampliar, sustituir ni volver a constituir `D_a` invocando una nueva génesis sobre una continuidad ya habitada.

Toda ampliación o modificación posterior de `D_a` continúa requiriendo T-G o T-C conforme al contrato original.

## 5. Efecto sobre el conjunto inicial de formas

La cláusula de SEC.0-A §5 según la cual «el conjunto inicial de formas se establece por T-0» se refiere exclusivamente al conjunto inicial de formas de una continuidad autoritativa aún no habitada.

Una nueva identidad técnica incorporada a una continuidad ya existente no puede declarar otro conjunto «inicial» de formas mediante T-0.

Introducir una forma nueva o modificar materialmente clase, familia de efectos o ligaduras de una forma existente continúa exigiendo T-C; T-G conserva únicamente las facultades que el contrato original le reconoce sobre formas ya constituidas.

## 6. Efecto sobre las vías legítimas de autoridad

La regla de SEC.0-A §6 permanece vigente:

```text
T-0 | T-C | T-G | T-R
```

son las únicas vías abstractas por las que puede llegar a existir autoridad bajo sus condiciones respectivas.

Esta adenda no añade una vía nueva. Restringe T-0 a su único supuesto legítimo y evita que una segunda identidad técnica la reutilice sobre una continuidad ya habitada.

En una continuidad ya habitada:

- restaurar sin ampliar autoridad preexistente corresponde a T-R cuando exista continuidad legítima acreditable;
- conceder, delegar o modificar autoridad dentro del régimen aplicable corresponde a T-G;
- alterar la constitución o introducir condiciones materialmente nuevas corresponde a T-C.

Esta regla no convierte en T-R, T-G o T-C los actos ordinarios que no restauran, crean ni modifican autoridad. T-I, T-V, T-H y T-E conservan exactamente su ámbito original.

## 7. Relación con continuidad y bifurcaciones

Esta corrección **no resuelve por sí sola la unicidad global de continuidad entre réplicas o bifurcaciones**.

Permanece fuera del alcance de SEC.0-A decidir materialmente cuál de varias continuaciones localmente válidas es la vigente. Esa cuestión continúa gobernada por SEC.0-M y SEC.0-X según corresponda.

La corrección fija únicamente una prohibición anterior y necesaria: ninguna rama, réplica, copia o instancia puede crear autoridad nueva declarándose una nueva génesis por el mero hecho de poseer una identidad técnica distinta.

## 8. Invariantes afectados

### A2-02 — Vías legítimas de constitución

Se interpreta desde esta adenda como:

> Toda autoridad nueva procede de T-0 de una continuidad autoritativa aún no habitada, de T-G o T-C válidas, o de una T-R no amplificadora bajo continuidad legítima.

### A2-13 — Historia local no constituye vigencia

Se refuerza con la consecuencia siguiente:

> La creación de una identidad técnica nueva sobre una historia, copia o estado autoritativo previo tampoco constituye una nueva génesis.

Los demás invariantes conservan su redacción y alcance.

## 9. Caso de regresión obligatorio

Se incorpora como contraejemplo permanente el supuesto siguiente:

1. existe una continuidad autoritativa ya habitada con autoridad o estado persistente relevante;
2. se crea otro proceso, réplica, contenedor, máquina virtual, `fork`, reinicio o identificador de instancia;
3. la nueva identidad accede, monta, restaura o continúa el estado previo;
4. intenta declarar T-0 para crear, sustituir o ampliar autoridad, `D_a` o formas constituidas.

Resultado exigido:

```text
T-0 no disponible
```

Si el acto pretende restaurar, crear o modificar autoridad deberá clasificarse como T-R, T-G o T-C según su efecto. Si no altera autoridad, conservará la clase ordinaria que le corresponda. En ningún caso una autoridad fabricada mediante segunda génesis es legítima.

Este caso deberá mantenerse alineado con `tests/sec0/VECTORES_ADVERSARIALES_SEC0_V1.md`, en particular con `V-A-13`.

## 10. Cierre

SEC.0-A permanece cerrado como contrato abstracto de autoridad, constitución y génesis con esta corrección incorporada por adenda.

La expresión «instancia» del contrato de 21/08/2026 no puede utilizarse desde esta adenda para abrir una segunda T-0 sobre una continuidad autoritativa ya habitada.

La unidad soberana de génesis queda fijada en la continuidad autoritativa.
