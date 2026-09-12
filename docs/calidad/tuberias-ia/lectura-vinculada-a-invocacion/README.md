# Lectura vinculada a invocación y procedencia

**RETP-2026-152 · 12/09/2026 · Candidata documental en Rust; ensayos conformes en el alcance indicado.**

Se ha comprobado una pérdida concreta: presentar una respuesta como perteneciente a otra consulta, incluso cuando ambas consultas producen los mismos bytes. La nueva vista exige la identidad que seleccionó el conductor confiable y conserva el acceso a la petición, al original, al caso del lote, al montaje y a las trazas correspondientes.

## Resultado verificable

| Comprobación | Resultado |
| --- | --- |
| Intercambio de identidad entre invocaciones con respuesta idéntica | Rechazado en ambos sentidos; cada identidad propia se admite |
| Cambio de ordinal, operación o versión | Rechazado |
| Cabecera excesiva; presentación excesiva | Rechazadas por sus límites |
| Respuesta de contenido distinto, aunque el JSON sea válido | Rechazada |
| Obtener vista antes de recibir y comprobar la entrega | Rechazado |
| Cliente externo que fabrica una vista o la conserva tras morir sus bytes | Rechazado por Rust, E0451 y E0515 |
| 24 respuestas públicas y procedencia recuperada | Coinciden con las huellas fijadas antes de esta candidata |
| Regresión del enlace anterior | 12/12 en cada modo |
| Nuevos testigos ejecutados por cliente externo | 9/9 en cada modo, incluido el recorrido de las 24 respuestas |

Rust 1.98.0, depuración y optimización: 17 procesos registrados, 42 tests entre ambos modos, cuatro rechazos previstos de compilación. Ninguna corrección del código después de ejecutar; ningún fallo inesperado. Las 24 respuestas coinciden con el oráculo anterior tanto en la regresión como en la nueva vía. Los testigos de regresión y los nuevos no son dos oráculos independientes: se distinguen del cotejo externo con las huellas anteriores.

[Contrato fijado y testigos](CONTRATO_Y_PLAN.md), [resultado](RESULTADO.json), [comandos y salidas completas](COMANDOS.json), [código del enlace](codigo/lectura.rs) y [cliente de contraste](codigo/tests.rs).

## Qué se ha unido

`ConsultaDeLectura` sólo se obtiene desde la entrega comprobada del enlace custodio. El conductor elige la consulta antes de contrastar la propuesta; no delega esa elección a un identificador recibido del proponente. `LecturaEntregable` conserva referencias inmutables a la consulta y a la presentación comprobada. Su constructor es privado. La operación admitida es la lectura documental experimental `LEER_RECIBO_PUBLICO`.

No se han añadido constructores públicos de autoridad. Una respuesta documental denegada o ambigua conserva su significado original. La operación aquí identificada no autoriza actuaciones del especialista. El permiso ficticio del banco continúa siendo ficticio.

La vista añadida no reserva memoria dinámica; toma préstamos de la custodia. Se mantienen los límites del perfil de presentación y se añaden 64 bytes por cabecera. La preparación, recuperación y pruebas sí conservan sus costes anteriores: esta pieza no acredita memoria residente, latencia de servicio o retención ilimitada. El préstamo evita una referencia colgante dentro del programa seguro; no acredita un proceso o anfitrión hostil.

## Frontera adversarial que permanece

Si dos respuestas son idénticas y alguien presenta sus bytes con la identidad correcta, este cotejo los acepta. Eso prueba correspondencia con la referencia elegida; **no demuestra de qué copia física procedieron los bytes**. Tampoco convierte una etiqueta en prueba independiente de origen. La pertenencia operacional procede del custodio confiable y del recorrido ya recibido en G1, con continuidad intra-proceso. Reinicios, transporte remoto y custodio comprometido siguen fuera del alcance.

Un consumidor que descarte esta vista y entregue directamente bytes ajenos elude la vía candidata. No se afirma imposición global del proceso ni integración productiva en `sv_core`. No hubo IA externa ni observación V; los textos y la política son sintéticos y públicos.

Los fallos nuevos `LimiteCabecera`, `Version`, `Operacion`, `Identidad` y `Presentacion` son variantes internas de esta candidata. No son nuevas claves canónicas del catálogo SV. El fallo de presentación conserva su causa heredada y el de apertura conserva `ErrorEnlace`. No se convierten en U. Su recepción por el catálogo/localización permanece pendiente del cierre acotado de esta campaña.

## Continuación exacta para Watson, Claude o Grok

1. **Resultado disponible:** lectura custodiada, presentación limitada y atribución documental comprobadas. Usar la cápsula y el contrato de esta carpeta; no volver a solicitar al autor el significado del frame o quién concede autoridad.
2. **Siguiente objeto único:** precisar y realizar la correspondencia con una operación gobernada real usando las referencias protegidas existentes, o documentar exactamente qué productor falta antes de construir la vía profesional. Este recibo puede aportar evidencia; no produce por sí mismo `ProtectedDecisionContinuity`, `FormRef`, `EffectDescriptor`, `ResolvedRequirementResult` o `Permit`. No fabricar esas referencias con etiquetas, booleanos ni constructores de prueba.
3. **Secuencia conservada:** cerrar el alcance de correspondencia aplicable; después recibir sus errores y localización en el catálogo y continuar la fila 9. Inscripción y Frame sólo donde el contrato los exija. P3 reservado, asociación /2–/3 y P4/P5 siguen sin abrir.
4. **Álgebra ya recibida:** RETP-151 conserva el inventario de soporte nuclear pendiente para la puerta de fila 10. SV no es espacio vectorial; célula plana, ordenada y posicional, n=b², b≥3 y terna fija. No sustituirlo por álgebra lineal, una matriz b×b o una firma universal de composición. U y fallo técnico permanecen separados.
5. **Competencias humanas y de dominio:** el experto constituye el significado y las actuaciones de su dominio. La composición particular y su reparto dominio/agente no se deciden aquí. Los frames son para el especialista de cada dominio; las demos antiguas no constituyen su interfaz vigente.
6. **Autorización conservada:** hay luz verde para trabajo técnico acotado y sus evidencias. Recurrir al autor ante una contradicción real o para aceptación final productiva; no pedirle repetidamente que resuelva la comprobación técnica. Al relevar, verificar el corte vigente y comunicar sólo diferencias que cambien el siguiente trabajo.

Esta pieza conserva el workflow V2 y sucede a RETP-150 únicamente en la atribución de la lectura. G2 profesional, fidelidad visual, seguridad integral, catálogo y cierre del núcleo permanecen abiertos. No se da por consumida la aceptación final humana.

## Fuentes y reproducción

Cortes previos verificados: Lenguaje `ad4d390d6fb1e7134040001b79ba2d433895b786`; laboratorio `a4186826314bf466d50201182b37506d911080e5`. [Cortes, rectores y contratos](CORTES_Y_REFERENCIAS.json). Pilares, perfiles, transición y workflow conservan los bytes previamente leídos. Para este tramo se consultaron el expediente RETP-142, el contrato G1/1 y G2 con la rectificación de RETP-148. No hay cambio doctrinal.

[FUENTES_REPRODUCIBLES.json](FUENTES_REPRODUCIBLES.json) contiene los 71 archivos de la candidata y sus huellas. Incluye los 67 anteriores y cuatro nuevos; entre los anteriores sólo cambia `lote-g1/lib.rs` para publicar el módulo de lectura. Los archivos históricos no necesarios para este ensayo se conservan para mantener la cápsula de procedencia.

Ejecutar `python reproducir.py /ruta/rustc /ruta/nueva-de-resultados` usando el compilador identificado en [COMPILADOR.json](COMPILADOR.json). El directorio debe ser nuevo. El conductor verifica y extrae las fuentes, compila ambas configuraciones y coteja los 24 originales contra las huellas anteriores. Python conduce pruebas y coteja archivos; no participa en la semántica Rust. [CONDUCTOR_EJECUTADO.py](CONDUCTOR_EJECUTADO.py) conserva las rutas de esta ejecución; el reproductor sólo parametriza esas rutas y extrae la cápsula.

Las capturas completas se conservan en [CAPTURAS.json.gz.b64](CAPTURAS.json.gz.b64), con [índice y huellas](CAPTURAS_INDICE.json). Se decodifica Base64, luego gzip y JSON; cada entrada contiene bytes Base64 y su SHA-256. No hay que regenerar el ensayo para leer su evidencia. Los binarios se reconstruyen; los comandos conservan el compilador y [REALIZACIONES.json](REALIZACIONES.json) conserva las huellas efectivas de los binarios y las variables de identificación del conductor.
