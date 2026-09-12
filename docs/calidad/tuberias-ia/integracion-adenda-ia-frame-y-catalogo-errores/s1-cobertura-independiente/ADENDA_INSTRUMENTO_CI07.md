# Adenda de instrumento CI07 y continuación de la cualificación

12/09/2026. El primer intento se detuvo al observar un rechazo distinto del esperado de CI07. Los seis controles anteriores habían satisfecho sus criterios; no se acredita por ello la cualificación completa. Se conservan la fuente inicial, el manifiesto anterior, las salidas del intento y la propuesta que lo causó.

El espécimen inicial sustituía la llave inicial del objeto por un corchete, dejando el resto del objeto intacto. El contrato heredado de presentación comprueba sintaxis antes de comparar contenido. La observación puntual de ese mismo espécimen produjo `Lectura(Presentacion(Sintaxis(Json)))`; la causa exacta está conservada en `capturas/diagnostico-ci07/CI07.causa-observada`. No hubo entrega de ese cuerpo.

Se corrige exclusivamente el instrumento: CI07 altera una letra del valor textual `PETICION_AMBIGUA`, conservando JSON sintácticamente válido. Se mantiene el esperado previo `ContenidoDistinto`; no se adapta al resultado nuevo. No cambian la candidata de cobertura, las 71 fuentes heredadas, los requisitos de cobertura, las entradas originales, la referencia ni los otros siete controles.

Este ajuste resuelve una discrepancia del espécimen respecto del criterio conforme a RETP-123 P1 y §§3–4. No es una corrección funcional del mecanismo ni una segunda ronda de modelos. La cualificación vuelve a comenzar, conservando por separado el intento interrumpido. Se reutiliza la biblioteca de cobertura de depuración ya compilada, cuya fuente no cambia. Se mantiene el máximo de 30 invocaciones registradas, incluido el intento instrumental sin productor y los dos procesos que recuperan la causa de CI07. Si aparece un nuevo fallo funcional, se detiene el cierre.
