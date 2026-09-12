# S3 · Presentación, negación y objeto recuperado del destino

RETP-2026-166 · 12/09/2026. Contrato anterior a la ejecución. Continuación del criterio D y de la cobertura C/I del recorrido documental.

## Antecedente y diferencia que se ensaya

Cortes: Lenguaje `1430f64869b341ad0795588db425fb69e65e06cb`; laboratorio `84a30056bc9707376b962b58fc6966ffee16b092`. AGENTS y las lecturas íntegras previas de Pilares, perfiles y transición se recotejan sin cambios. `BANCO_FIJADO.json` conserva identidades de estas piezas y de las fuentes funcionales.

RETP-150 ya comprueba la supresión de «no acredita» dentro del alcance del cuerpo y la rechaza mediante `IE004-PRESENTACION-ESPACIOS/1`. También permite separadores JSON fuera de las cadenas. RETP-152 añade pertenencia de la lectura a una invocación. S1/S2 añaden cobertura documental de dos piezas. Esos resultados se reciben como antecedentes; no se vuelven a contar como aportaciones aisladas de S3.

Falta comprobar conjuntamente la selección con cobertura, la vista validada, los bytes escritos al destino y los bytes recuperados de él. S3 fija ese contraste mediante un conductor de archivo nativo. La resolución custodiada puede permanecer intacta mientras el archivo cambia; por ello no basta cotejar de nuevo el texto previsto.

## Referencia y operación

Se reciben sin cambios las 71 fuentes de la copia ejecutable S2 y sus dos archivos de cobertura. P3-01/A01, posición 0, conserva su montaje y esperado anteriores. No se cambia la política, semántica, referencia, API de lectura ni perfil de presentación. Se recibe también la declaración de vigencia de S2 para conservar su procedencia, aunque esta posición permanece positiva.

El cuerpo original incluye «Registro artificial; no acredita concentración ni condición clínica.». El espécimen espaciado sólo introduce LF y espacios fuera de las cadenas. El espécimen adversarial elimina exactamente los bytes de «no » ante «acredita», mantiene JSON válido y conserva los demás campos. Su texto residual se conserva literalmente, sin corregirlo ni atribuirle suficiencia clínica.

La relación de fidelidad sigue siendo la definida por RETP-150, no una equivalencia de significado inventada para este ensayo. La evidencia requerida sigue siendo el caso y la entrada exacta del montaje, elegidos antes de la propuesta.

## Conductor de destino

`destino.rs` es un conductor de ensayo, no una nueva frontera productiva. Sólo recibe una `Entrega` de cobertura construida por las capas previas; escribe exactamente su texto mediante `create_new`, `write_all` y `flush`. No abre el destino cuando la propuesta no ha pasado cobertura. Un error de escritura permanece como error de E/S; no se presume escritura atómica ni se borra una salida parcial automáticamente.

La lectura usa un buffer fijo de 16.385 bytes para detectar el exceso del máximo admitido de 16.384. Sólo los bytes realmente recuperados se presentan de nuevo a la referencia fijada. En el caso válido se permite conservar la presentación con espacios; no se exige que coincida con los separadores originales del recibo. Se comprueba por separado que lo escrito y lo recuperado sean idénticos.

El cotejo utiliza el mismo proceso confiable y las mismas bibliotecas, con una observación separada del archivo; no constituye una implementación independiente ni autenticación entre procesos. `flush` no acredita persistencia ante corte eléctrico. No se prueba un renderizador, una pantalla, estilos, comprensión humana ni un host comprometido.

## Controles fijados

| Control | Entrada o intervención | Resultado exigido |
| --- | --- | --- |
| D01 | Presentación literal y dos citas completas | Escritura y recuperación exactas; cobertura conforme |
| D02 | Presentación con separadores permitidos y dos citas completas | Escritura y recuperación exactas de esa presentación; cobertura conforme |
| D03 | Negación eliminada antes de validar | ContenidoDistinto; destino no creado |
| D04 | Vista completa validada; después de escribir, eliminar negación sólo del archivo | ContenidoDistinto al comprobar lo recuperado; cuerpo y vista originales intactos |
| D05 | Texto fiel, falta cita de vigencia | FaltaVigencia; destino no creado |
| D06 | Archivo de 16.385 bytes | Limite de lectura, sin aceptación como fiel |
| D07 | Destino preexistente | Io(AlreadyExists); bytes anteriores intactos |
| D08 | Archivo ausente | Io(NotFound), separado de pérdida de contenido |

D04 demuestra detección posterior: la alteración se materializa deliberadamente y no se afirma haberla impedido. D03 comprueba la barrera previa del conductor. D06–D08 delimitan las obligaciones de E/S que introduce este conductor; no son una ampliación del catálogo general. Ninguna causa se convierte en U.

Dos sensibilidades: desactivar la comparación de contenido en una copia de presentación debe ser detectado por D03; sustituir la observación recuperada por el texto previo en una copia del conductor debe ser detectado por D04. Las fuentes ordinarias permanecen intactas. No se repiten los controles de constructores o vidas ya acreditados en los expedientes anteriores.

## Presupuesto, conservación y parada

Rust 1.98.0 nativo, debug y release, tres ejecuciones por modo para comparar por bytes los artefactos recuperados. Ocho controles por ejecución. Máximo de 22 invocaciones: una identificación de compilador, ocho compilaciones y seis ejecuciones ordinarias, cuatro compilaciones y una ejecución de la sensibilidad de contenido, una compilación y una ejecución de la sensibilidad de observación. Límite de 60 segundos por proceso.

Fuentes, especímenes, esperados y conductor se fijan antes de ejecutar. Un fallo inesperado detiene la recepción conforme; no se adapta el esperado a la salida ni se amplía el presupuesto. La preparación anterior a la fijación se registra separadamente de la cualificación. Se conservan stdout, stderr, argumentos, retornos, huellas, pared, CPU, propuestas, salidas y causas. RSS individual no disponible. Python sólo organiza y coteja artefactos; Rust resuelve, verifica y escribe/recupera el destino.

## Condición de salida y siguiente objeto

El cierre se limita a correspondencia documental y pérdida de negación a través del archivo del ensayo, con cobertura de las dos piezas. No cierra D para cualquier representación ni C/I universal, A–L, revisión profesional, seguridad del host o P4/P5/P6.

La siguiente actividad será preparar la prueba externa común del recorrido documental delimitado, con una misma instrucción, fuentes accesibles y esperados previamente custodiados. No se enviarán encargos ni se abrirá la reserva por esta cualificación. Los modelos no deciden el oráculo. El catálogo conserva las causas durante el recorrido y se consolida posteriormente. Se mantienen las ramas y los registros Sucesos SV, RETP y el espejo de laboratorio.
