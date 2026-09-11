# Causas nacidas en esta frontera — insumo del catálogo

Las causas se producen como variantes Rust en el lugar de comprobación. No se clasifican leyendo texto libre de la IA. Esta tabla conserva propuestas de mensajes; **no constituye cierre del catálogo ni implementación de localización ES/EN**. No asigna nuevos códigos E/DG canónicos.

| Causa tipada | Punto de emisión | Propuesta ES | Propuesta EN |
|---|---|---|---|
| `ContenidoDistinto` | Comparación con el cuerpo del recibo | El contenido propuesto no coincide con el recibo de referencia. | The proposed content does not match the reference receipt. |
| `Integridad` | Validación del marco propuesto | El marco propuesto está incompleto o no conserva su integridad. | The proposed frame is incomplete or fails its integrity check. |
| `Limite` | Tamaño previo a reserva | La propuesta supera el límite de esta entrega. | The proposal exceeds this delivery's size limit. |
| `YaIntentada` | Guarda de intento único | Este recibo ya tiene un intento de entrega registrado. | This receipt already has a recorded delivery attempt. |
| `Custodia(causa)` | Recuperación, pertenencia, estado o reserva | No se ha podido comprobar la entrega; conserve la causa de custodia. | The delivery could not be checked; retain the custody cause. |

`EstadoEntrega::SinIntento`, `Comprobada` y `Rechazada(causa)` son estados distintos. Una propuesta rechazada no se convierte en U, Bottom ni diagnóstico de dominio. Un recibo cuyo cuerpo contiene una negativa o un fallo técnico puede tener una entrega literal comprobada: se acredita la conservación de esa negativa o fallo, no el éxito de la consulta.

La candidata conserva íntegra la variante anidada `Custodia(Fallo)` en su API. El conductor CLI de ensayo resume esa familia como `CUSTODIA`; **esa salida resumida no satisface todavía el diagnóstico estructurado productivo**. Deberá transportar también su causa anidada por el contrato existente, con parámetros tipados y localización revisada. Los contrastes CLI ejecutados en esta matriz afectan a `INTEGRIDAD` y `CONTENIDO_DISTINTO`; no acreditan la localización pendiente.

Antes de cerrar el punto 2: integrar estas causas en el contrato de diagnóstico existente, conservar relaciones y posiciones aplicables, verificar paridad ES/EN sin traducir datos ni identificadores, e incorporar también las causas que resulten de las etapas de fidelidad visual y seguridad. Los mensajes de esta tabla quedan sujetos a esa revisión; no se convierten en autoridad ni permisos.
