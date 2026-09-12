# S2 · Vigencia causal y cobertura de evidencia

RETP-2026-165 · 12/09/2026 · Responsable: Watson / W-S0.

**Resultado:** contraste conforme en el banco sintético nativo. S2 demuestra que la vigencia cambia la resolución de una consulta inequívoca y que la entrega documental exige conservar la evidencia de esa vigencia. S2 finalizado en este alcance. [Estado y siguiente suceso](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md).

## Resultado causal

La solicitud original A01, «El valor de la IgG.», se recibe con los mismos bytes y contexto LEER/CASO-A/IGG/ACTUAL/VALOR en ambas ejecuciones. Cambia únicamente el booleano confiable de vigencia. Las trazas conservan análisis completo, un único significado, idénticos tokens, ruta y trabajo lógico del analizador.

| Instantánea | Resolución | Contenido | Ruta | Llamadas de política |
| --- | --- | --- | --- | --- |
| Vigencia positiva | DATO | `8.40` | `[1,1,1,1,1]` | 1 |
| Vigencia negativa | PERMISO_REVOCADO | null | `[1,1,1,1,1]` | 1 |

El negativo conserva fuente y alcance null. Es una resolución de política, distinta de una ambigüedad o una avería. [Observación y correspondencia de trazas](OBSERVACION_CAUSAL.json).

El mismo par se comprueba a través del enlace y la cobertura documental: P3-01 y P3-11 conservan sus IDs distintos, pero iguales pregunta y contexto. Un montaje sucesor explícito fija P3-11 con vigencia negativa. El cambio no se atribuye al montaje antiguo. La prueba directa con A01 evita confundir la diferencia de IDs del lote con la causa del resultado.

## Cobertura y conservación

El cuerpo negativo completo con caso y entrada de montaje íntegros se entrega. Omitir la vigencia, aun conservando el caso verdadero y el cuerpo fiel, produce `FaltaVigencia`. Sustituir su cita por la de P3-01 produce `VigenciaDistinta`. El montaje histórico no pasa por el enlace sucesor; omitir el caso, cambiar identidad o alterar una letra del estado dentro de JSON válido conserva causas diferenciadas. El positivo completo se entrega igualmente.

Se obtienen **72 observaciones de doce controles en seis ejecuciones**: tres debug y tres release. Las **42 capturas de cada ejecución** son idénticas entre los seis recorridos. Los diez primeros cuerpos del lote conservan sus esperados anteriores. El recorrido secuencial de once posiciones no representa once consultas distintas ni amplía la cobertura a todo el lote.

Las dos sensibilidades fallan donde se exigía: suprimir la obligación documental se detecta en CI02; desactivar la condición de revocación se detecta en CA02. Son rechazos intencionados de mutantes, no defectos atribuidos a la candidata. No hubo intentos fallidos adicionales ni rectificaciones de esperados en S2.

La cápsula anterior mantiene sus 71 fuentes. La copia ejecutable sustituye sólo el montaje y la fijación de versión/huella del enlace; 69 fuentes permanecen idénticas. La lógica de cobertura de S1 se reutiliza con nuevos rangos y versión. No cambian la resolución semántica, política, custodia, entrega o lectura de la candidata. Los mutantes se conservan aparte como instrumentos de sensibilidad.

## Expediente y reproducción

- [Contrato previo y controles](CONTRATO_S2.md).
- [Banco, cortes, huellas y fuentes consultadas](BANCO_FIJADO.json).
- [Fijación anterior a ejecutar](FIJACION_PREVIA.json).
- [Esperados íntegros](ESPERADOS.json) y [declaración del montaje](DECLARACION_VIGENCIA.json).
- [Resultado](RESULTADO.json), [comandos y retornos](COMANDOS.json), [entorno](ENTORNO.json) y [capturas recuperables](EVIDENCIAS_S2.json).
- [Causas observadas](CAUSAS_OBSERVADAS.json), para su recepción posterior por el catálogo.
- [Manifiesto del expediente](MANIFIESTO.json) y [reproductor ejecutado](reproducir.py).

Desde esta carpeta, con Python 3 en Linux y Rust 1.98.0 disponibles:

```sh
python3 reproducir.py --rustc /ruta/a/rustc --salida /ruta/nueva/s2
```

El destino debe ser nuevo. El reproductor verifica fuentes y esperados, recupera la cápsula, aplica las dos sustituciones declaradas y ejecuta el presupuesto fijado de **23 invocaciones**, con 60 segundos como máximo por proceso. Se registran pared y CPU de cada proceso; RSS individual no disponible. `LG1_FUENTES` identifica el manifiesto previo del expediente, que incluye candidata, instrumentos y esperados; `LG1_BINARIO` identifica el ejecutable de cada recorrido. Las capturas están codificadas en base64 con longitud y SHA-256, sin pérdida de bytes. Las fuentes del compilador y los binarios no se incluyen como parte del expediente.

Esta cualificación fue realizada con el reproductor publicado; no se atribuye una segunda ejecución independiente. El tiempo observado corresponde a esta campaña local, no a una garantía de servicio ni a P5.

## Secuencia restante

| Objeto | Situación y siguiente acción |
| --- | --- |
| C/I | Efecto causal y cobertura de dos piezas acreditados en este banco; insuficiente para cierre universal o revocación profesional |
| S3 · criterio D | Pendiente: precisar el objeto efectivamente presentado y el testigo de pérdida de negación; reutilizar RETP-149/150/152, sin repetir sus pruebas como nuevo resultado |
| Prueba externa común | Recepción pendiente; preparar sobre el recorrido integrado y sus esperados fijados; la reproducción RETP-157 no la sustituye |
| Paso 6 · catálogo y ES/EN | Causas conservadas durante el recorrido; consolidación posterior a la integración de 1+3 |
| Resto de A–L y workflow | Conserva las condiciones y carencias de RETP-162; P3 reservada, P4/P5/P6 y cierre humano integral pendientes |

La instantánea negativa es sintética: no constituye revocación profesional, actualización de una autorización viva, inscripción de dominio o promoción productiva. No se ejecutaron modelos externos, WASI ni navegador. S1 y P3-04 conservan su resultado anterior, sin nueva ejecución ni reinterpretación como prueba de revocación.

Se mantienen `main` en Lenguaje y `lab/playground-sv-permanente` en laboratorio. Sucesos SV, RETP y el espejo documental conservan la continuidad.
