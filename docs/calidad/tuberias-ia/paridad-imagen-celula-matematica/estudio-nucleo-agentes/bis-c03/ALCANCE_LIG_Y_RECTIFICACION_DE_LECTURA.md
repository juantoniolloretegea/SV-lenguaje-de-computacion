# BIS-C03 · Alcance existente de LIG y precisión de la radiografía

**13 de septiembre de 2026 · S22 · RETP-2026-206 · Juan Antonio Lloret Egea y Watson**

## Corte y fuentes rectoras

La revisión corresponde al Lenguaje `d184a8589958d814c902bbfb2ed09589cafdc3bf` y al laboratorio `8040f98321c6b5b780df47c80e7f18b174e7a29f`. Se conserva la lectura rectora completa de Pilares, acta de perfiles y acta secuencial de inmunología, con sus adendas, identificadas en el manifiesto de fuentes. La continuidad verifica identidad de las piezas ya leídas; no convierte los ejemplos históricos en dominios vigentes.

Se han leído completos el contrato de continuidad por operación y el contrato material DFL-005, y el módulo `rust/sv_core/src/bindings.rs`. Se han contrastado las definiciones pertinentes de `ir.rs`, el testigo de dos nodos con CellSpec compartida, las entradas del banco LIG y la sucesión RETP-105 de deuda viva. La inspección de pruebas no equivale a ejecutarlas en este incremento.

## Hallazgo y rectificación de alcance

La matriz histórica de BIS-01 describe referencias nominales de la IR y deja pendientes ligaduras de constitución, instancia y representación. Esa lectura necesita complementarse con la entrada tipada LIG/0.1 existente en el núcleo. No es correcto generalizar la ausencia de ciertos campos en `IrProgram` a la ausencia de toda validación de ligaduras en el Lenguaje.

La [sucesión RETP-105](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/d184a8589958d814c902bbfb2ed09589cafdc3bf/docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md#relevo-retp-105) registra la integración del subconjunto representacional LIG/0.1 mediante PR #85, commit `140d319995a5df685860b348c0fd086ae6130faf`. Los párrafos históricos anteriores que lo presentaban como candidato no describen su estado integrado. La deuda general DFL-005 sigue abierta en sus restantes obligaciones.

| Obligación | Material existente al corte | Límite que recibe BIS-03 |
| --- | --- | --- |
| Constitución y artefactos por operación | Referencias exactas, clase y bytes; expectativa del contrato y del programa | No interpretación completa del documento constitutivo ni autenticación de autoridad. |
| Instancias y destinos | Instancias de parámetros con propietario; usos ordenados, NodeId/posición, alias y compartición declarados | No identifica por sí solo la instancia celular completa ni su continuidad entre revisiones. |
| Especificación compartida | Dos nodos pueden compartir CellSpec y conservar estados referidos separadamente | Compartir tipo o contenido no autoriza a fusionar identidad. |
| Representación específica | El programa conserva estados y referencias arquitectónicas | No hay un tipo específico de LIG que imponga la relación instancia celular/revisión/frvis. |
| Resultado validado | Construcción privada de ValidatedBindings y acceso de lectura | No acredita que cualquier consumidor externo use necesariamente esa entrada ni que consuma el artefacto correcto. |

Este documento complementa BIS-O03/O04 y la lectura de C02, sin reescribir la radiografía ni los asientos previos. La propuesta C02 sobre soporte por versión no se convierte en una capacidad de LIG por la presencia de referencias genéricas: el significado y la imposición de S requieren su contrato.

## Consecuencia para el trabajo

Se evita duplicar el contrato de ligaduras. C03 precisa el enlace todavía no acreditado y registra su banco por separado. No se han añadido ni ejecutado pruebas Rust, ni ejecutado las pruebas LIG existentes, ni alterado su evidencia histórica. Las dieciséis variantes C03 son previas y siguen pendientes. El total de escenarios originales ejecutados de BIS-02 permanece en dos; quedan veintidós pendientes.
