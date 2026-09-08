# Registro de calidad RETP-2026-091 — recepción de G/H y apertura de la fila 7

**Fecha:** 7 de septiembre de 2026  
**Repositorio:** `SV-lenguaje-de-computacion`  
**Base del Lenguaje:** `bc3b22c9e9319e8f191390c8cfe9fa1577904d87`  
**Retorno de Inmunología:** `54fe0d89c9e59065eae2bc8a38f5ec0832ece4b9`  
**Estado:** `RECIBIDO_EN_CANDIDATA_NO_INTEGRADA`

## 1. Objeto

Este registro recibe el retorno acotado G/H de `OP-IMM-001 / Q0 v0` y abre la fila 7 de la secuencia rectora. La recepción no ejecuta Q0, no modifica la Gramática, la representación intermedia ni la realización, y no abre el primer contraste del dominio de Ciberseguridad Inteligente.

## 2. Evidencia recibida

El expediente de Inmunología conserva:

- 44 archivos fuente seleccionados y verificados por longitud, SHA-256 y objeto blob Git;
- 15 requisitos G10, 44 formulaciones LSV y 81 enlaces revisados;
- 27 tipos paramétricos, cuatro salidas terminales y `REQ-IMM-SV-011 = U_NO_DECIDIDO`;
- ocho pares documentales con pérdida confirmada y la reproducción de la campaña F-IF;
- doce comprobaciones SP-01…SP-12 especificadas, pero no ejecutadas de extremo a extremo.

El dictamen recibido es:

```text
SUFICIENCIA_NO_ACREDITADA_PARA_EJECUTAR_Q0
```

Este resultado no afirma que Q0 sea irrepresentable ni imposible de ejecutar en una realización futura.

## 3. Consecuencias para la fila 7

La fila 7 queda abierta para incorporar únicamente los cambios justificados por G/H. El primer objeto es DFL-005: identidad de instancia, ligaduras y suficiencia representacional por operación.

Permanecen vigentes las siguientes restricciones:

- una lista nominal no constituye ligaduras entre parámetro, captura, admisibilidad, transducción, posición y operación;
- los 27 tipos no constituyen 27 transductores;
- la producción observación → `Tri` permanece no habilitada por K1-T;
- la falta de configuración, la no admisión, la insuficiencia representacional y el fallo técnico no producen `0`, `1` ni `U`;
- el fallo técnico no produce una salida clínica;
- ninguna célula, tamaño o asignación se infiere del inventario inmunológico.

Inmunología queda fuera del alcance activo tras la devolución, salvo que aparezca una nueva pregunta constitutiva o una laguna imprescindible cuya resolución corresponda al dominio. La fila 8 corresponderá al contraste por el dominio de Ciberseguridad Inteligente cuando exista una candidata identificada procedente de la fila 7.

## 4. Condición de salida de la fila 7

Antes del segundo contraste de dominio deberá existir:

1. una candidata identificada;
2. correspondencia de cada cambio con una operación, un contrato, una versión y una evidencia;
3. resolución, exclusión o aplazamiento expreso de cada pérdida relevante;
4. comprobaciones positivas y negativas que impidan ocultar obligaciones en campos opacos;
5. conservación explícita de las capacidades todavía no acreditadas.

## 5. Alcance

Este registro y su CSV vinculado documentan la recepción y su efecto sobre la secuencia. No modifican documentos históricos ni atribuyen a la devolución capacidades que el expediente no acredita.

**CSV vinculado:** `docs/calidad/RETP_2026_091_RECEPCION_GH_Y_APERTURA_FILA_7.csv`.
