# Acta de activación del control de barridos y partes por agente

**Fecha:** 23/03/2026  
**Hora (Europe/Madrid):** 16:59:50  
**Agente:** Agente WLenguaje7 SV

## Objeto

Activar un control de calidad adicional orientado a reducir memoria espuria, inferencia no declarada, lectura parcial del árbol del repositorio verificado y acumulación innecesaria de microregistros dispersos.

## Decisión

Se institucionalizan dos instrumentos nuevos:

1. `REGISTRO_BARRIDOS_DE_ACTIVIDAD_Y_LATENCIA_DEL_REPO.md/.csv`
2. `REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.md/.csv`

## Regla de periodicidad

- Barrido secuencial obligatorio cada **5 microparches** o **1 macrolote equivalente**, lo que ocurra antes.
- El barrido no se apoyará en la fecha bruta del ZIP extraído como criterio suficiente de actividad o inactividad.
- El registro maestro podrá consolidar ciclos cuando proceda, para no inflar innecesariamente la traza principal.

## Regla de base de verificación

Toda actuación deberá declararse según uno de estos valores:

- `INFERIDA`
- `MEMORIA_REFERENCIAL`
- `PARCIAL`
- `VERIFICACION_ACOTADA`
- `VERIFICACION_INTEGRAL`

Las decisiones globales de fase, arquitectura, registro o continuidad del repo exigirán lectura `VERIFICACION_INTEGRAL` del árbol del repositorio verificado.

## Observación

La finalidad de este control no es añadir burocracia ornamental, sino crear una capa mínima de verificabilidad humana para que futuras unidades distingan con claridad entre lectura real, lectura parcial y memoria operativa.
