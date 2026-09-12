# S3 · Presentación y pérdida de negación en destino

RETP-2026-166 · 12/09/2026 · Responsable: Watson / W-S0.

**S3 finalizado en alcance documental nativo.** El recorrido integrado conserva la negación en la presentación aceptada, impide escribir la propuesta que la elimina y detecta una alteración posterior al cotejar los bytes recuperados del archivo. [Sucesos SV y siguiente actividad](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md).

## Qué aporta respecto de los antecedentes

RETP-150 ya rechazaba quitar «no acredita» del alcance de la respuesta. S3 reutiliza ese mecanismo, la lectura vinculada de RETP-152 y la cobertura de S1/S2. El objeto nuevo comprobado es el archivo realmente escrito y recuperado después de pasar por esas capas. Las 73 fuentes recibidas de S2 se conservan intactas; sólo se añade un conductor de ensayo con E/S acotada.

La referencia es P3-01/A01 y su esperado histórico, con el alcance «Registro artificial; no acredita concentración ni condición clínica.». El ataque elimina exactamente «no » delante de «acredita», sin corregir el texto residual ni modificar los demás campos. El JSON adversarial permanece sintácticamente válido.

## Resultado del recorrido

| Caso | Resultado observado |
| --- | --- |
| Presentación literal | Escritura y recuperación exactas; cobertura conforme |
| Separadores JSON permitidos | Se conserva exactamente la presentación espaciada; cobertura conforme |
| Pérdida de negación antes de validar | ContenidoDistinto; archivo no creado |
| Pérdida de negación después de escribir | ContenidoDistinto sobre lo recuperado; referencia y vista originales intactas |
| Omisión de cita de vigencia | FaltaVigencia; archivo no creado |
| Archivo de 16.385 bytes | Limite de lectura |
| Destino preexistente | Io(AlreadyExists); contenido anterior conservado |
| Destino ausente | Io(NotFound) |

La diferencia temporal es esencial: la barrera previa impide escribir por este conductor una propuesta rechazada. En el ataque posterior, el archivo sí llega a estar alterado; el cotejo lo detecta. No se confunde detección con prevención ni se declara resistencia frente a un anfitrión malicioso.

**Ocho controles, seis ejecuciones: 48 observaciones conformes.** Tres ejecuciones debug y tres release, con las **28 capturas de cada recorrido idénticas** entre los seis. La referencia final conserva exactamente los bytes iniciales. Los controles de E/S delimitan únicamente el conductor añadido.

Las dos sensibilidades son detectadas: eliminar la comparación de contenido falla en D03; cotejar el texto previsto en vez del recuperado falla en D04. Los mutantes se preservan como instrumentos y no se incorporan a las fuentes de la candidata. No hubo fallos inesperados ni correcciones durante las 22 invocaciones de cualificación.

## Evidencia y reproducción

- [Contrato fijado antes de ejecutar](CONTRATO_S3.md).
- [Banco y fuentes cotejadas](BANCO_FIJADO.json), [fijación previa](FIJACION_PREVIA.json) y [especímenes exactos](ESPECIMENES.json).
- [Fuentes S2 recibidas sin cambios](FUENTES_S2.json), [conductor de destino](codigo/destino.rs) y [contraste Rust](codigo/contraste.rs).
- [Resultado](RESULTADO.json), [observación del destino](OBSERVACION_DESTINO.json) y [capturas recuperables](EVIDENCIAS_S3.json).
- [Comandos y retornos](COMANDOS.json), [entorno](ENTORNO.json) y [causas observadas](CAUSAS_OBSERVADAS.json).
- [Manifiesto](MANIFIESTO.json) y [reproductor ejecutado](reproducir.py).

En Linux, con Python 3 y Rust 1.98.0, desde esta carpeta:

```sh
python3 reproducir.py --rustc /ruta/a/rustc --salida /ruta/nueva/s3
```

El destino debe ser nuevo. El reproductor verifica la fijación, recupera las fuentes y especímenes, compila y ejecuta el presupuesto cerrado de 22 invocaciones, con 60 segundos como máximo por proceso. La cualificación publicada utilizó este mismo reproductor; no se atribuye otra ejecución independiente. Se conservan CPU y pared de cada proceso; RSS individual no disponible. `LG1_FUENTES` identifica el manifiesto previo del expediente, incluidos instrumentos y esperados; `LG1_BINARIO` identifica cada ejecutable concreto.

Las capturas se conservan por bytes en base64, con longitud y SHA-256. La [preparación anterior a la fijación](PREPARACION.json) registra un error de sintaxis del generador de metadatos, corregido antes de iniciar Rust; no se cuenta como resultado funcional ni produjo ajuste de esperados.

## Alcance y continuación

El destino observado es un archivo leído por el mismo proceso confiable. No se han comprobado pantalla, estilos, recortes, renderizador, comprensión o revisión humana. `flush` no prueba persistencia ante corte eléctrico; una escritura fallida no se presume atómica. Las pruebas no autentican el host ni la identidad entre procesos. Los resultados de esta reducción no cierran D para cualquier representación ni C/I universal, A–L, P4/P5/P6 o una actuación profesional.

**S4 queda pendiente: preparar la prueba externa común del recorrido documental delimitado.** Se fijarán una misma instrucción, fuentes públicamente accesibles, respuestas esperadas ya custodiadas y un formato de recepción comparable. Se distinguirá la ejecución de cada participante del camino o los recursos que declare haber empleado. Este es un banco público conocido, no la reserva inédita P3 ni una prueba ciega. No se han enviado encargos ni recibido resultados externos por S3.

La integración de 1+3 conserva la prioridad. Las causas de este ensayo quedan registradas; el paso 6, consolidación del catálogo y ES/EN, sigue después de la integración. El resto de los criterios y las compuertas del workflow conservan sus pendientes. Se mantienen las ramas `main` del Lenguaje y `lab/playground-sv-permanente` del laboratorio, con expediente y Sucesos SV reflejados en ambas sedes.
