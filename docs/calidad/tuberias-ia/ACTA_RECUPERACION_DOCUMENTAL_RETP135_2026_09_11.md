# Recuperación documental de RETP-2026-135

**Fecha:** 11 de septiembre de 2026. **Objeto:** completar el depósito interrumpido de los resultados de IE004-SEMANTICA-A/1.

## Evidencia localizada y procedencia

El chat anterior anunció el resultado de la matriz semántica y dejó en curso su depósito espejado. Al revisar los repositorios, Calidad y laboratorio llegaban a RETP-134. En un directorio de trabajo conservado de aquella ejecución se localizaron las fuentes fijadas, cuatro binarios, cuatro salidas originales, las capturas comprimidas, los manifiestos, el acta de resultados y el paquete de publicación RETP-135 pendiente.

Los registros conservados sitúan la compilación inicial en `2026-09-11T06:21:34.121Z` y el fin de la última ejecución en `2026-09-11T06:21:38.563Z`. La verificación de recuperación se realizó a las `2026-09-11T08:01:12.940Z`. Se distinguen así la ejecución original y este depósito documental posterior. Estos archivos no permiten atribuir una causa técnica al corte del chat ni probar la ausencia de cualquier actividad no registrada.

La referencia previa es Lenguaje `5d0f0e5d1cacf69f03cf040b69e1b5bfbf228a75` y laboratorio `4e556e5d072b55b31a2a9b2c9da255602817c999`, rama `lab/playground-sv-permanente`. La lectura rectora y del workflow ya realizada sobre esos cortes sigue siendo aplicable; este acto no cambia sus contratos.

## Comprobación de recuperación

- Las 17 fuentes y datos de `FIJACION_PREVIA.json` conservan sus tamaños y SHA-256. Los 18 archivos prepublicados, incluida la fijación, coinciden con sus objetos Git.
- Los 31 archivos del manifiesto de resultados y los 38 del manifiesto de cierre conservan sus tamaños y SHA-256. También coinciden las huellas de los cuatro binarios locales.
- La descompresión de las dos capturas distintas recupera exactamente las cuatro salidas originales. Cada una contiene cabecera, 61 casos semánticos, 72 casos sintácticos y resumen.
- Se cotejaron los estados y demás campos esperados de cada caso con los corpus fijados antes de la ejecución, además de comprobar el indicador de coincidencia. Los cuerpos de los 133 casos son idénticos entre las cuatro configuraciones, incluidos contadores y capacidades.
- Las cabeceras conservadas registran 26 primitivas. Sus aserciones no se volvieron a ejecutar durante esta recuperación.
- El paquete original de publicación contiene 45 archivos. Se comprobó que sus modificaciones de navegación parten de los objetos Git vigentes y que los registros añaden únicamente RETP-135. El CSV resultante contiene 132 filas de datos, 14 columnas y ningún identificador duplicado.

La verificación consistió en lectura, cálculo de huellas, descompresión y comparación de archivos. No se compiló ni ejecutó de nuevo el banco Rust, no se ejecutó Python y no se accedió a preguntas ni al oráculo de la reserva.

## Resultado y alcance

Las capturas respaldan el [acta original de resultados](ACTA_RESULTADO_SEMANTICA_A_2026_09_11.md): **61/61 controles semánticos y 72/72 regresiones sintácticas** por configuración nativo/WASI, debug/release, con el recuento de **26 primitivas** en cada cabecera.

S26 contiene 128 tokens y registra 621.146 unidades, 1.675.752 bytes de capacidad de buffers, 1.777.770 bytes de pico solicitado calculado y 1.554 estados semánticos. S20 conserva el rechazo de la grafía con acento combinante y S45 admite la tilde precompuesta.

Se preservan los documentos y capturas originales del paquete. Se añade esta acta, su [verificación estructurada](ie004/semantica-a/VERIFICACION_RECUPERACION_RETP135.json) y enlaces de recuperación en navegación y registro Markdown. La afirmación original de una sola matriz se conserva como descripción de aquella unidad; lo comprobado ahora es la integridad y correspondencia de los archivos disponibles, sin reproducción independiente.

Permanecen pendientes la memoria física total, el peor caso universal y las acreditaciones que la ronda pública no cubre. La reserva permanece cerrada. El [siguiente objeto](ie004/semantica-a/SIGUIENTE_OBJETO.md) sigue siendo fijar la correspondencia semántica y la recepción/captura separada de A y V antes de usarla. Este depósito no ejecuta ese siguiente objeto ni reabre catálogo, localización o integración profesional.

## Huellas de referencia

| Elemento original | SHA-256 |
| --- | --- |
| Fijación previa | `e5073224e257bf43e837274b867325a38171363a7521921a25714e3498c7c290` |
| RESULTADO.json | `ed36290ddabff5b4a2b9638ce3d533040f0696ae76741ee541128a1f9dfe5be0` |
| Acta de resultados | `2dca62bffc1d382410b760de2bf38f91ad95b3c720e73a17fd1c8ad1df3ed567` |
| Paquete original publicacion-135.json | `4e7a9688a844e3395170f57bcce3b5d239f32f34537ba491c7fe73459f1f69ce` |
