# Acta técnica de reapertura y cierre correctivo de SEC.0-A y SEC.0-M — génesis y continuidad autoritativa

**Fecha:** 22/08/2026  
**Estado:** reapertura correctiva cerrada  
**Ámbito:** Lenguaje SV — SEC.0-A / SEC.0-M

## 1. Objeto

Esta acta documenta una reapertura correctiva limitada de SEC.0-A y SEC.0-M motivada por un contraejemplo reproducible posterior a sus cierres del 21/08/2026.

La reapertura no revisa el conjunto de ambos contratos. Se limita a una ambigüedad relativa a la unidad de génesis T-0 y a su interacción con clonación, nueva implantación, reinicio y continuidad autoritativa.

## 2. Hecho técnico constatado

SEC.0-A definía T-0 como la transición que «constituye el primer estado legítimo de autoridad de una instancia».

SEC.0-M establecía, para una copia que pasase a convertirse en una implantación distinta, que debía existir «la transición de constitución o génesis correspondiente conforme a SEC.0-A».

La combinación de ambas cláusulas permitía construir el siguiente camino no deseado:

1. existe una continuidad autoritativa ya habitada;
2. se crea una nueva identidad técnica mediante proceso, réplica, contenedor, máquina virtual, `fork`, reinicio o identificador distinto;
3. esa nueva identidad accede o restaura estado autoritativo previo;
4. se presenta su nacimiento técnico como «primera instancia» y se reclama T-0;
5. mediante esa segunda génesis se intenta crear, sustituir o ampliar autoridad sobre la continuidad existente.

Este camino contradice la finalidad material de SEC.0-A: una operación ordinaria o una nueva identidad técnica no deben fabricar autoridad ni ampliar una autoridad existente.

## 3. Causa de la ambigüedad

El cierre original de SEC.0-A dejó expresamente fuera de alcance la unicidad global de continuidad entre réplicas o bifurcaciones. Esa limitación era legítima y se mantiene.

Sin embargo, la cuestión aquí corregida es anterior y distinta: no se trata de decidir cuál de varias continuidades es vigente, sino de impedir que una identidad técnica nueva convierta una continuidad ya habitada en una nueva génesis.

La expresión «instancia» resultó insuficiente para conservar esa separación.

## 4. Evidencia de reproducibilidad

El contraejemplo es reproducible sin depender de una tecnología concreta:

```text
continuidad C ya habitada
→ nueva identidad técnica I2
→ I2 monta/restaura/continúa estado de C
→ I2 reclama T-0 por ser una "instancia nueva"
→ I2 intenta crear o ampliar autoridad en C
```

La violación existe aunque la nueva identidad se ejecute en otro proceso, contenedor, máquina virtual, host o tras un reinicio.

La identidad local no elimina el antecedente autoritativo de la continuidad.

## 5. Decisión correctiva

Se fija:

> **La unidad de génesis T-0 es la continuidad autoritativa aún no habitada, no la identidad efímera de proceso, réplica, contenedor, máquina virtual, reinicio o instancia.**

En consecuencia:

- una continuidad ya habitada no admite una segunda T-0;
- una nueva identidad técnica sobre `AStore`, `PDep` o autoridad previa no crea génesis;
- restaurar autoridad preexistente corresponde a T-R cuando exista continuidad legítima;
- conceder, delegar, revocar o modificar autoridad corresponde a T-G dentro de su régimen;
- alterar constitución, formas o condiciones constitutivas corresponde a T-C;
- los actos que sólo informen, verifiquen, habiliten o ejerzan autoridad ya existente conservan las clases T-I, T-V, T-H o T-E correspondientes;
- sólo una continuidad genuinamente nueva y no habitada puede disponer de T-0.

La nueva identidad técnica no reclasifica por sí sola ninguna operación: la clase T-* continúa derivándose del efecto real del acto.

## 6. Documentos correctivos

Se incorporan como documentos de referencia:

- `ADENDA_CORRECTIVA_SEC0_A_UNIDAD_DE_GENESIS_Y_CONTINUIDAD_AUTORITATIVA_2026_08_22.md`;
- `ADENDA_CORRECTIVA_SEC0_M_CLONACION_IMPLANTACION_Y_GENESIS_2026_08_22.md`.

Las adendas prevalecen, para esta cuestión concreta, sobre cualquier lectura incompatible de los contratos cerrados el 21/08/2026.

Los documentos originales se conservan como parte de la trazabilidad del cierre anterior.

## 7. Alcance en SEC.0-A

La corrección afecta únicamente a:

1. la cláusula de constitución inicial de `D_a` por T-0;
2. la definición de T-0;
3. la cláusula según la cual el conjunto inicial de formas se establece por T-0;
4. la lectura de A2-02 y A2-13 en cuanto dependan de la unidad de génesis;
5. los casos integrales que distinguen copia técnica, continuidad y autoridad.

No modifica el resto de clases T-*, `E_max`, verificación gobernada, delegación, acumulación, reserva humana ni las demás obligaciones de SEC.0-A.

## 8. Alcance en SEC.0-M

La corrección afecta a la interpretación de la cláusula de clonación e implantación de §7 y a sus consecuencias sobre recuperación y bifurcación.

No modifica las reglas de `AStore`, `PDep`, retroceso, cobertura negativa, presupuestos, tiempo, efectos externos, recuperación no circular ni selección de continuidad.

## 9. Relaciones no modificadas

Esta reapertura no modifica SEC.0-D, SEC.0-X ni SEC.0-T.

Tampoco resuelve:

- qué rama es la continuidad vigente cuando existen varias continuaciones localmente válidas;
- la técnica material para impedir retroceso o clonación;
- la plataforma, almacenamiento, raíz de confianza o mecanismo de atestación concretos;
- la implementación del backend soberano.

Esas cuestiones permanecen en los contratos y fases correspondientes.

## 10. Regresión

El caso de segunda génesis sobre continuidad habitada queda incorporado como regresión permanente y debe mantenerse alineado con `V-A-13` del catálogo adversarial SEC.0.

La corrección no constituye por sí misma cobertura material. Su futura comprobación contra una realización concreta deberá satisfacer SEC.0-T y conservar el estado `NO_PROBADO` mientras no exista una ejecución falsable sobre el `SUT` exacto correspondiente.

## 11. Cierre correctivo

Con las dos adendas incorporadas, SEC.0-A y SEC.0-M vuelven a declararse cerrados en el alcance corregido.

El cierre conserva la limitación original sobre unicidad global de continuidad, pero elimina la posibilidad de utilizar una nueva identidad técnica para reclamar una segunda T-0 sobre una continuidad autoritativa ya habitada.
