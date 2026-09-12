# Sucesos SV

Registro obligatorio de actividades del proyecto, ordenado mediante la sucesión S0, S1, …, Sn. Se inicia el 12 de septiembre de 2026. Cada identificador corresponde a una actividad delimitada y conserva su identidad durante todo su seguimiento.

## Consulta

- [Sucesos y estado vigente](SUCESOS_SV.md).
- [Registro estructurado en CSV](SUCESOS_SV.csv).
- [Historial de altas y actualizaciones](HISTORIAL_SUCESOS_SV.csv).

La dinámica de avance, ensayo, documentación en laboratorio y publicación de su espejo en Calidad permanece vigente. Este registro es complementario y obligatorio: enlaza cada actividad con su evidencia y sus registros técnicos de calidad en CSV y Markdown. Su numeración es independiente de los identificadores RETP y comienza ahora; los antecedentes conservan sus referencias originales.

## Estados

El campo `estado` sólo admite estos tres valores exactos:

| Estado | Significado |
| --- | --- |
| pendiente | Actividad registrada que aún no se ejecuta o cuya ejecución se ha interrumpido, con motivo y siguiente acción documentados. |
| en ejecución | Actividad iniciada, con responsable, alcance y fecha de inicio identificados. |
| finalizado | Actividad concluida en el alcance declarado, con resultado y evidencia registrados. |

El resultado se describe por separado. Finalizar una actividad no acredita por sí solo un resultado favorable, una validación global ni una autorización de producción. Una interrupción mantiene la fecha de inicio y registra su motivo; cualquier reanudación queda en el historial. Si una actividad termina sin lograr su objetivo, se explicitan ese resultado y los pendientes derivados. Una continuación de un suceso finalizado recibe un identificador nuevo y enlaza el antecedente.

## Alta y actualización

1. Consultar el registro vigente antes de iniciar una actividad. Reutilizar su identificador si el alcance ya está registrado.
2. Asignar al nuevo suceso el entero inmediatamente posterior al mayor identificador publicado. La serie es única: no se reinicia por fecha, responsable, repositorio o rama; no se reutilizan ni se renumeran identificadores publicados.
3. Registrar actividad, alcance, responsable, fechas, dependencias, repositorios y ramas, cortes conocidos, referencia de calidad y siguiente acción. Se publicará el alta antes de ejecutar la actividad; una actividad ya iniciada se declara como tal, sin atribuirle fechas o estados retrospectivos no observados.
4. Actualizar el estado al iniciar, interrumpir, reanudar o concluir, y documentar cualquier cambio de responsable, alcance, resultado o evidencia. Cada actualización añade una instantánea completa al historial con el mismo identificador y una revisión incremental desde 0. Las filas previas del historial se conservan intactas.
5. Mantener una sola fila vigente por suceso en el CSV y su representación concordante en Markdown. El historial y los commits permiten recuperar las versiones anteriores; no se borran sucesos ni se sustituyen antecedentes silenciosamente.
6. Incorporar la evidencia factual y los registros de calidad aplicables; verificar su correspondencia y el espejo antes de dar por concluida una actividad que los requiera.

La asignación se coordina en el registro canónico de Calidad del Lenguaje, rama `main`. Las copias conservan los mismos identificadores y contenidos. Antes de publicar se coteja el commit vigente; si otra incorporación ha ocupado el siguiente identificador, se recibe ese cambio y se asigna el siguiente disponible a la actividad aún no publicada. Un conflicto no se resuelve sobrescribiendo el registro.

## Datos y evidencia

Las fechas utilizan ISO 8601 en UTC, con sufijo `Z`. Las fechas de inicio y fin permanecen vacías hasta que esos hechos ocurran. El identificador de unidad responsable se mantiene estable durante su intervención; cualquier relevo registra el cambio y conserva su autoría anterior. Las dependencias identifican relaciones entre actividades, sin convertirlas en estados adicionales.

El CSV conserva actividad, alcance, responsable, fechas de alta, inicio, actualización y fin, repositorios y ramas, cortes de entrada, dependencias, resultado, verificación, evidencias, referencia de calidad, siguiente acción y observaciones. El historial añade `revision`, consecutiva para cada suceso. Los enlaces a resultados concretos se fijan a commits cuando éstos existen; no se anticipan identificadores de commits futuros. El commit que contiene cada actualización identifica su publicación.

La redacción será técnica, factual, precisa y respetuosa, con alcance y límites explícitos. Las previsiones se distinguen de las actuaciones realizadas y las afirmaciones de resultado remiten a evidencia recuperable. El registro describe actividades observables y evita atribuir comprobaciones que no se hayan efectuado.

## Referencias

- [Léame primero](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md).
- [Registro de evolución técnica — Markdown](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md) y [CSV](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv).
- [Copia de laboratorio](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/tree/lab/playground-sv-permanente/laboratorio/tareas-watson/sucesos-sv).
