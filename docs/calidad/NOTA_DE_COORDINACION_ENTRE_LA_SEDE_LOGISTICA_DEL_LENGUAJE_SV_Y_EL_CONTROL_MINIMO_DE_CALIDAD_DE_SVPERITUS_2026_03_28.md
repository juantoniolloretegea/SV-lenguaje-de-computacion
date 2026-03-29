# NOTA_DE_COORDINACION_ENTRE_LA_SEDE_LOGISTICA_DEL_LENGUAJE_SV_Y_EL_CONTROL_MINIMO_DE_CALIDAD_DE_SVPERITUS_2026_03_28

**Fecha:** 28/03/2026  
**Estado:** vigente  
**Naturaleza:** nota técnica de coordinación entre sedes  
**Ámbito:** `SV-lenguaje-de-computacion`

## 1. Objeto

La presente nota precisa el modo correcto en que la sede logística del Lenguaje SV debe coordinarse con el control mínimo de calidad abierto en `SVperitus`.

## 2. Criterio de coordinación

La coordinación correcta entre ambas sedes se rige por esta pauta:

1. `SVperitus` conserva la memoria local suficiente para sus agentes, fases, artefactos, deuda y cierres.
2. `SV-lenguaje-de-computacion` conserva la trazabilidad técnica fuerte y el gobierno operativo de los frentes que afecten a SVP o a su carril de implementación.
3. La relación entre ambas sedes es de coherencia, remisión y no contradicción, no de duplicación simétrica.

## 3. Frentes que deben comparecer también en el lenguaje

Deben comparecer también en `docs/calidad/` del lenguaje las piezas de `SVperitus` que afecten a:

- DSL y traducción a SVP;
- validator, lowering, serialización o runner;
- consulta estructural y ABI semántico-diagnóstico;
- o cualquier decisión aplicada que rebase la mera organización local del repositorio.

## 4. Frentes que pueden resolverse localmente en SVperitus

Pueden resolverse de forma local en `SVperitus` los hitos, cierres, deudas y actuaciones que no alteren el carril técnico del lenguaje ni reabran jerarquía doctrinal.

## 5. Finalidad

Esta nota se incorpora para fijar una coordinación estable entre memoria aplicada local y gobierno técnico fuerte, con sobriedad documental y sin duplicación impropia.
