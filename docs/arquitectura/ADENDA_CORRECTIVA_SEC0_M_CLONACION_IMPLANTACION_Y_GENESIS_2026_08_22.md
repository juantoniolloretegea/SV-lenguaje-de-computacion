# Adenda correctiva a SEC.0-M — clonación, implantación y génesis

**Fecha:** 22/08/2026  
**Estado:** corrección arquitectónica cerrada  
**Ámbito:** Lenguaje SV — SEC.0-M  
**Documento afectado:** `CONTRATO_ABSTRACTO_DE_MEMORIA_PERSISTENCIA_RECURSOS_Y_CONTINUIDAD_SEC0_M_2026_08_21.md`

## 1. Objeto

Esta adenda corrige la lectura posible de SEC.0-M §7 según la cual una copia que pase a convertirse en una implantación distinta podría obtener una nueva génesis por el mero hecho de adquirir identidad técnica propia.

La corrección conserva íntegramente la separación entre estado semántico y estado técnico, las reglas de continuidad, retroceso, bifurcación, consumo, recuperación y persistencia de SEC.0-M. Su único objeto es hacer coherente la cláusula de clonación con la unidad de génesis fijada por la adenda correctiva de SEC.0-A.

## 2. Regla corregida de clonación e implantación

La clonación crea otra representación material del mismo estado técnico, pero no multiplica por sí sola autoridad, titularidad, autorizaciones de un solo uso, continuidad vigente ni génesis.

Cuando una copia, réplica o estado restaurado se convierte en una implantación técnicamente distinta, deben distinguirse dos casos.

### 2.1. Implantación ligada a continuidad autoritativa previa

Si la implantación:

- hereda autoridad o estado autoritativo previo;
- monta o utiliza un `AStore` ya habitado;
- depende de `PDep` pertenecientes a una continuidad anterior;
- restaura un antecedente autoritativo;
- prolonga, replica o bifurca una continuidad existente;
- utiliza como fundamento de legitimidad un estado constituido en una continuidad previa;

entonces no existe una nueva T-0.

La nueva identidad técnica no altera por sí sola la clase T-* de los actos que ejecute. Cuando el acto restaure, conceda, delegue, revoque, modifique o reconstituya autoridad, deberá tratarse según su efecto real:

- T-R, cuando únicamente restablezca autoridad preexistente bajo continuidad legítima;
- T-G, cuando conceda, delegue, revoque o modifique autoridad dentro del régimen aplicable;
- T-C, cuando altere constitución, formas, compatibilidad u otras condiciones constitutivas.

Los actos que sólo informen, verifiquen, habiliten o ejerzan autoridad ya existente conservan las clases T-I, T-V, T-H o T-E correspondientes.

### 2.2. Continuidad genuinamente nueva

Sólo una continuidad autoritativa aún no habitada, que no herede ni utilice como fundamento de autoridad estado autoritativo perteneciente a otra continuidad, puede disponer de su propia T-0 conforme a SEC.0-A corregida.

La mera diferencia de proceso, host, contenedor, máquina virtual, identificador local o imagen de ejecución no demuestra este supuesto.

## 3. Sustitución interpretativa de SEC.0-M §7

La frase original:

> «Si una copia debe convertirse en una implantación distinta, deberá existir la transición de constitución o génesis correspondiente conforme a SEC.0-A»

debe interpretarse desde esta adenda como:

> Si una copia debe convertirse en una implantación distinta, deberá respetar la transición legítima que corresponda conforme a SEC.0-A. Cuando la implantación herede, restaure, monte, prolongue o utilice estado autoritativo previo, T-0 no está disponible. Si el acto restaura o modifica autoridad, deberá ser T-R, T-G o T-C según su efecto; si no altera autoridad, conservará la clase ordinaria que corresponda. Sólo una continuidad autoritativa genuinamente nueva y aún no habitada puede disponer de T-0.

## 4. Relación con `AStore` y `PDep`

La presencia de `AStore` o `PDep` previamente admitidos es material para decidir si una nueva identidad técnica pertenece a una continuidad ya habitada.

No es lícito:

```text
nuevo proceso o nueva implantación
+
AStore/PDep previos
⇒ nueva génesis
```

La relación correcta es:

```text
nuevo proceso o nueva implantación
+
continuidad autoritativa previa
⇒ no hay nueva T-0
```

La clase concreta de cada acto posterior continúa derivándose de su efecto real conforme a SEC.0-A.

## 5. Relación con bifurcación

Esta adenda no decide cuál de dos o más ramas localmente válidas debe reconocerse como continuidad vigente.

Mantiene la regla de SEC.0-M: si la política exige unicidad y no puede acreditarse cuál es la continuación vigente, la continuidad queda no verificable para los efectos dependientes y no se selecciona una rama por conveniencia operativa.

La adenda añade únicamente que ninguna rama puede resolver esa incertidumbre reclamando una nueva T-0 por poseer una identidad técnica distinta.

## 6. Relación con recuperación

La recuperación continúa exigiendo autoridad previamente constituida, regla de selección no circular, procedencia, vigencia y continuidad suficientes.

Un reinicio o restauración que genere una nueva identidad local no transforma la recuperación en génesis.

Si el estado recuperado pertenece a una continuidad ya habitada:

```text
reinicio/restauración ≠ T-0
```

Cuando el acto sea materialmente recuperación de autoridad, conservará las obligaciones de T-R. Si además modifica o reconstituye autoridad o constitución, deberá tratarse como T-G o T-C según el efecto. Un acto que no altere autoridad conserva su clase ordinaria.

## 7. Caso de regresión obligatorio

Se incorpora como caso permanente:

1. una continuidad autoritativa contiene `AStore`, `PDep` o autoridad previamente admitida;
2. el estado se clona, restaura o replica en otra identidad técnica;
3. la nueva implantación intenta declararse de génesis y producir nueva autoridad o ampliar la existente;
4. la única razón invocada para T-0 es que la implantación posee proceso, contenedor, host, máquina virtual o identificador distintos.

Resultado exigido:

```text
T-0 no disponible
```

La identidad técnica nueva no multiplica la génesis ni la autoridad.

## 8. Cierre

SEC.0-M permanece cerrado como contrato abstracto de memoria, persistencia, recursos y continuidad con esta corrección incorporada por adenda.

La conversión de una copia en implantación distinta no constituye por sí sola una nueva continuidad autoritativa ni habilita una segunda T-0.
