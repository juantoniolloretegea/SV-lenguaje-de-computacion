# F01/F02 · interfaz disponible, pendiente y continuidad

**14/09/2026. Corte: Lenguaje `1d577e7dffd9bad8d73cd779105c8730301bea0a`; laboratorio `2e2fc695e4e718999f471c7b4fa8520af8138c2f`.**

Se recibe el [resultado LIG](RESULTADOS_F01_F02_LIGADURAS.md), diez casos conformes por perfil de compilación. No se repiten esas campañas ni INTEGRADO01.

## Contraste de la interfaz

| Pieza inspeccionada | Capacidad acreditable | Límite para F01/F02 |
| --- | --- | --- |
| LIG/0.1; `rust/sv_core/src/bindings.rs` | `validate_bindings` conserva y valida usos, instancias, destinos y orden bajo una expectativa explícita. | Recuperar ligaduras no ejecuta una consulta histórica ni selecciona un componente de una ocurrencia de Frame. |
| `rust/sv_core/src/ir.rs` | Representa Frame, Trajectory, QuerySpec y contextos de `query`; conserva referencias. | La representación declarativa no constituye una respuesta material de consulta. |
| `rust/sv_core/src/frame.rs` | Constructor interno que comprueba cierre de Frame; accesores de sus referencias. | Los accesores no forman un servicio de consulta por ocurrencia y posición. |
| `rust/sv_core/src/context_wellformed.rs` y contrato H06/H07 | Comprueban pertenencia y coherencia estructural. | H07 declara expresamente que su aceptación no ejecuta `query` ni completa CQ1–CQ6. |
| Reutilización histórica de requisitos R1 | Reutilización gobernada de resultados cualificados de requisitos. | Objeto y contrato distintos de una consulta de componente celular en un Frame; no se intercambian. |

La inspección se limita a esas sedes y sus exportaciones públicas en `lib.rs`. Concuerda con [R02, C02/C05/C06](../RECEPCION_CONTRACTUAL_R02.md), [LIG/0.1](../../../arquitectura/CONTRATO_MATERIAL_DE_LIGADURAS_DFL_005_2026_09_08.md) y la [nota de historia y consulta presente](../../NOTA_TECNICA_SOBRE_FRAME_HISTORICO_REAPERTURA_Y_CONSULTA_PRESENTE_2026_03_19.md). No se ha identificado en ellas la interfaz material requerida para cerrar F01/F02 completos.

## Decisión y condición de retorno

Se conserva el pendiente en DFL-003/004/005 y R2-0 según su alcance. F01 requiere selección por ocurrencia exacta, dependencias fijadas y respuesta efectiva. F02 requiere operación de resolución por instancia, posición y constitución; ausencia y ambigüedad deberán quedar explícitas. El adaptador de pruebas no suplirá esa operación mediante selección por primera coincidencia ni un resolutor propio.

Esta delimitación no transforma una carencia en U ni cierra S26. El siguiente bloque material disponible en la secuencia actual es S22/RETP-221: fronteras conjuntas del descriptor y la salida del montaje C02–C05. Su [banco previo](../../tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-extension-perfiles-recursos-v0_1/BANCO_FRONTERAS_SALIDA.md) conserva las interfaces existentes, las exclusiones de GUI/BD y el orden Bis → catálogo/cierre de fase → S24.

Este contraste documental no añade ejecuciones SV ni acredita suficiencia clínica.
