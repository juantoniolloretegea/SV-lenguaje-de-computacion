# Registro de barridos de actividad y latencia del repo

## Finalidad

Este registro documenta barridos secuenciales del repo fresco orientados a valorar actividad estructural, uso funcional, latencia legítima o sospechosa y necesidad de análisis adicional.

## Regla metodológica

Los barridos no deben tomar como prueba suficiente la fecha de creación o la fecha de modificación del ZIP extraído. Deben valorar, con lectura real del árbol, si la quietud de un fichero:

- es normal y legítima;
- es latencia propia de una pieza canónica o reservada para una fase futura;
- o requiere análisis adicional por aislamiento o ausencia de integración visible.

## Estados recomendados

- `ACTIVO`
- `ESTABLE_LEGITIMO`
- `LATENTE_LEGITIMO`
- `HUERFANO_APARENTE`
- `REQUIERE_ANALISIS`
- `CANDIDATO_A_CONSOLIDACION`

## Regla de entrada

Cuando el barrido no arroje anomalías materiales, basta una entrada global del repositorio. Si aparecen zonas dudosas o artefactos aislados, se añadirán filas específicas por ruta o por grupo homogéneo.
