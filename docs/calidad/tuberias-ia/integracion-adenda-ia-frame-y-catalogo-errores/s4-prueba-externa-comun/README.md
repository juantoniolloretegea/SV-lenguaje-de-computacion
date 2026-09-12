# S4 · Prueba externa común publicada

RETP-2026-167 · 12/09/2026 · Responsable: Watson / W-S0.

S4 finalizado: preparación y publicación del paquete. S5 pendiente: recepción y evaluación efectiva de las respuestas. No hay aún resultados, puntuaciones o tiempos atribuibles a participantes.

Entrada pública e inmutable: [PRUEBA_COMUN.md](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c/pruebas-externas/s4-recorrido-documental/PRUEBA_COMUN.md). Incluye reglas, ocho fuentes, doce casos, rúbrica y plantilla; puede adjuntarse íntegro sin acceso al laboratorio. [Paquete público](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c/pruebas-externas/s4-recorrido-documental/README.md) y [reproducción S2/S3](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c/pruebas-externas/s4-recorrido-documental/REPRODUCCION_S2_S3.zip).

## Resultados previos y acceso

Los doce esperados fueron cotejados con artefactos Rust ya publicados de S2/S3. El oráculo y su validación se custodiaron en laboratorio antes de publicar el paquete externo. [Compromiso previo](COMPROMISO_PREVIO.json) fija sus huellas junto con banco, fuentes, rúbrica y documento. No se han ejecutado nuevas pruebas funcionales para fabricar un resultado de participante.

SVcustos es público. Se descargaron documento y ZIP mediante GET sin cabecera Authorization ni cookies: HTTP 200 y coincidencia exacta de bytes y SHA-256. El intento inicial por el lector web devolvió DisabledError; se conserva como intento sin lectura, separado de la descarga HTTP posterior satisfactoria. [Comprobación de acceso](ACCESO_PUBLICO.json).

Esta disponibilidad no presume que una interfaz concreta haya abierto el archivo. La respuesta debe confirmar lectura completa y modo de acceso. El mismo documento puede entregarse por enlace o adjunto, sin alterar el encargo ni requerir escritura en repositorios.

## Evaluación fijada

[Rúbrica](RUBRICA.md): resultado 48 puntos; trazabilidad 36; procedimiento 8; entrega 8. La conformidad exige 90/100, todos los resultados correctos, mínimos de trazabilidad por caso y ninguna incidencia crítica. La evaluación coteja fuentes, citas y consecuencias; no premia extensión ni referencias nominales.

Tiempo y esfuerzo observable se registran aparte con valor, unidad, origen y evidencia. Se distinguen mediciones del observador, datos expuestos por la plataforma y declaraciones del participante. Medida ausente: null con motivo. No se estima esfuerzo mental ni se interpreta la duración total como CPU. Las correcciones conservan la respuesta inicial y se registran como otro intento.

Las [plantillas de respuesta y observador](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c/pruebas-externas/s4-recorrido-documental/README.md) facilitan una recepción comparable. Se conservan los originales antes de puntuar y se cotejan contra el oráculo fijado, no contra la respuesta de otro modelo. El ZIP reproduce 51 archivos históricos con sus bytes y resultados anteriores; el banco es conocido y no se presenta como evaluación ciega.

## Continuidad

S5 recibe el trabajo por el canal del encargo. La lectura documental y la reproducción propia se distinguen; no tener herramientas, si se declara, no es fallo por sí mismo. Permanecen los límites de A–L, representación en pantalla, revisión profesional, host y P4/P5/P6. El catálogo recoge causas durante la integración y se consolida posteriormente. No se abre reserva P3.

Mismas ramas existentes: main en Lenguaje y SVcustos; lab/playground-sv-permanente en laboratorio. Sucesos SV y RETP conservan el historial y el siguiente objeto.
