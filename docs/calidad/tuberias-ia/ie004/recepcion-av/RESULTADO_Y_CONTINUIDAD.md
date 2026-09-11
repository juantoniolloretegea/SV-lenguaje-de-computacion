# Resultado público de recepción A/V

**RETP-2026-137 · 11/09/2026 · CONFORME_EN_ALCANCE_PUBLICO.** Preparación inmutable: Calidad `c6ad0c2bf6a027b840f877203cdb3d87dc22eb3c`; laboratorio `141a40d00cb607641bb8645ddfadf0d6ca89da1a`. Una matriz, sin corrección ni reintento de ejecución después de esa fijación. Se comprobaron después las huellas de las fuentes y resultados.

| Configuración | A | V | Relaciones | Reproducciones Rust | Primitivas | Procesos/pipes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Nativo debug | 23/23 | 25/25 | 3/3 | 3 | 22/22 | 9/9 |
| Nativo release | 23/23 | 25/25 | 3/3 | 3 | 22/22 | 9/9 |
| WASI debug | 23/23 | 25/25 | 3/3 | 3 | 22/22 | 9/9 |
| WASI release | 23/23 | 25/25 | 3/3 | 3 | 22/22 | 9/9 |

Son 51 controles distintos por configuración, ejecutados tres veces (153 observaciones), más 22 primitivas y una observación de cada uno de los nueve escenarios de transporte. No son 153 preguntas distintas ni capturas de un modelo. La serie 61/72/26 de RETP-135 conserva su identidad; esta tabla evalúa otro objeto y no reemplaza sus resultados.

## Qué debía suceder y qué sucedió

El cuerpo A debía quedar completo, validado y depositado antes de abrir V. Los eventos de los nueve escenarios lo confirman en las cuatro configuraciones. La propuesta ausente, inválida, excesiva, agotada, mal correlacionada, demorada o sin EOF no cambió ese cuerpo. Su huella común es `189b6f9501ad3a57ed274fc10d1670b7448885f190a2c1f2e8bdb55f92904237`.

La recepción se detuvo al observar el byte 65.537 de A o el 262.145 de V, antes de acumular el exceso. Se rechazaron UTF-8 y JSON inválidos, claves duplicadas/adicionales, enteros no representables, límites de estructura, referencias cíclicas o discordantes y certificados de otra identidad. El marco incompleto o corrupto no fue admitido como cuerpo. Las primitivas ejercieron además las fronteras de profundidad 64 y enlaces 1.024, presupuesto exacto, reserva y fallo de lectura.

La propuesta válida al máximo de 262.144 bytes pasó con 845.002 unidades V. Sus reservas acumuladas fueron 449.779 bytes en nativo y 432.571 en WASI. La reserva de admisión A de la consulta usada en transporte fue 66.748 y 66.364 bytes respectivamente. Esas diferencias corresponden a la representación de los objetos JSON; no son memoria física ni contradicen la identidad del cuerpo. El trabajo y los desenlaces V comparables coincidieron.

La demora solicitada de 50 ms se observó entre 51 y 52 ms. En el control sin EOF, el conductor terminó V por el plazo inyectado de 150 ms, conservando el cuerpo A previamente depositado; V no produjo un anexo propio y el conductor registró ese timeout aparte. No se atribuye un diagnóstico emitido a un proceso que fue terminado. Estos tiempos son observaciones de este banco, no garantías de servicio.

Los 23 cuerpos A coinciden entre las cuatro configuraciones y las tres reproducciones de cada control son idénticas dentro de su configuración. Las paráfrasis y las representaciones JSON equivalentes conservan cuerpo y mantienen distintas sus trazas de procedencia. Siguen sin admitirse la grafía combinante y las entradas fuera de la cobertura lingüística fijada.

## Evidencia y límites

- [Resultado de la matriz](resultados-1/RESULTADO.json), [entorno](resultados-1/ENTORNO.json) y [cortes](resultados-1/CORTES.json).
- [Comprobación posterior de artefactos](COMPROBACION_RESULTADOS.json): 408 archivos de resultado cotejados y ocho binarios locales identificados.
- [Archivo recuperable](CAPTURAS_RECUPERABLES.json.gz.b64) y [sus huellas](ARCHIVO_CAPTURAS.json): contiene 409 archivos, incluido el propio manifiesto original. Guarda todos los stdout/stderr, marcos binarios, cuerpos, anexos y registros de invocación sin pérdida. Los ocho ejecutables quedan identificados y reconstruibles, no incorporados al archivo.
- [Extractor](recuperar-resultados.mjs): `node recuperar-resultados.mjs directorio-nuevo` comprueba huellas y recupera los archivos, sin ejecutar el receptor.

Los controles internos conservan cuerpo, veredicto y huellas de sus trazas; los escenarios de procesos conservan también las trazas A completas. Los avisos de Node/WASI permanecen en stderr y se distinguieron del registro JSON al analizarlo. La comprobación posterior de esos archivos no supuso otra ejecución del banco.

No se modificaron las cinco fuentes semánticas heredadas ni los esperados fijados. No hay nuevo ensayo de Grok, Python o API de proveedor. Este resultado acredita funcionamiento público acotado de la interfaz; no acredita aislamiento material P4, RSS, pila, presión concurrente, utilidad del participante P5 o comprensión universal. La derivación V comprobada no adquiere autoridad normativa.

La reserva queda cerrada y la compatibilidad operativa integral P3 continúa pendiente según la [correspondencia declarada](CORRESPONDENCIA_SEMANTICA_Y_RESERVA.md). El [siguiente objeto](SIGUIENTE_OBJETO.md) es el adaptador estricto del lote y la preparación de la decisión de compatibilidad; no se inicia aquí una captura reservada.
