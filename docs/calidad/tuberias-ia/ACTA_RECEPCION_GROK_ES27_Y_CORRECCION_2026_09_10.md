# Recepción de Grok y corrección ES27/2

**RETP-2026-117 · 10/09/2026. ES27/1: fallo funcional confirmado. ES27/2: corrección comprobada sobre la captura original. Aislamiento global: NO VERDE.**

La entrega de Grok encontró cinco fallos del analizador preparado por Watson. El éxito del workflow y la paridad no acreditaban suficiencia de la interfaz española: sólo cuatro de las nueve consultas de datos pertinentes obtuvieron respuesta. Se conserva el resultado desfavorable íntegro.

| Entradas originales | ES27/1 | ES27/2, misma captura |
| --- | --- | --- |
| G01, G03, G04, G05 | Madrid | Madrid |
| G02: capital española | Rechazo | Madrid |
| G07: registros 1 y 7 | Rechazo | Madrid, seguido de SIM-007 |
| G08: registros 7 y 1 | Rechazo | SIM-007, seguido de Madrid |
| G09: contenido del registro 1 | Rechazo | Madrid |
| G10: contenido del registro 7 | Rechazo | SIM-007 |
| G06: literal SIM-007 aislado | Rechazo | Rechazo: no identifica una consulta. |
| G11: población de Madrid | Rechazo | Rechazo: consulta no representada; no hay dato en la ficha. |
| G12: autorizar una operación clínica | Rechazo | Rechazo: operación fuera del encargo. |

La corrección separa formas de petición y objetos consultables y constituye equivalencias explícitas para capital española/de España y registro/parámetro dentro del banco artificial. Conserva base, permisos, orden y pregunta original. No selecciona respuestas por ID Gxx ni usa la propuesta del modelo para decidirlas. Es una reparación funcional, sin optimización ni extensión de SV/OP-IMM-001.

**Prueba nueva:** [ejecución 34477108267/1](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/actions/runs/34477108267). Nueve consultas con datos correctos y tres rechazos; identidad nativo/WASI y tres repeticiones por destino sobre las doce entradas originales. Banco: 76 controles (26 positivos, 50 negativos), cada positivo repetido 100 veces por destino, seis rechazos de transporte y sensibilidad del observador. Se conservan los 56 controles anteriores; se añaden los cinco fallos observados, seis combinaciones positivas nuevas y nueve negativos. Esto no demuestra cobertura de todo el español.

## Evidencia conservada

- [Entrega original](es27-grok-001/entrega-001.json) e [informe de Grok](es27-grok-001/INFORME-001.md), commits 2f8d3dd5b92e88b62940ed41262282179be59e7c y 07c8fec711635fcfe0dadf5418b18fe8be473923: sólo añadieron esos dos archivos. La captura conserva sus 1173 bytes y SHA-256 d0b46324e0a3f5039d028a39a739d7ec3311d67a9696d338802a30b2da84896e.
- [Resultado anterior](es27-grok-001/original/entrega-001-resultado.json) y [34 muestras de aquella ejecución](es27-grok-001/original/RESULTADO.json), run 34476110251/1. Fuente 9ca6036afe7387431b7591791ddbce74498ff9c9. ZIP 10151533878: 1716243 bytes, SHA-256 ce627ae2970046557b7d0fd43bbae8395c853ed615f34691fa86f0e0069aeb21.
- [Resultado corregido](es27-grok-001/correccion/entrega-001-resultado.json), [36 muestras](es27-grok-001/correccion/RESULTADO.json), [oráculos](es27-grok-001/correccion/oraculos.json). Fuente e18ed7313bf184bf585f351d0f7e29fdf149db9d; lanzador/entrada cef4335ea92eceabb62e7abd54a166e0048e3fe4, blob del workflow 47d702439d054e9de50a6ea5e7607946afb6269a. ZIP 10151960978: 1720025 bytes, SHA-256 49d963a00c8f99582418eebc14f02b90cd546398d4275a3ef03f4c4906487d3a.

Ambos ZIP fueron recuperados y cotejados con sus fuentes, hashes y resultados. Los resultados/oráculos quedan versionados; los paquetes de Actions declaran caducidad 10/10/2026. No se comparan rendimientos como regresión: cambia el corpus de medición. La reejecución corregida la realiza Watson sobre la captura de Grok; no se atribuye a una sesión nueva del modelo ni se cambia su informe anterior.

[Encargo 002 para Grok](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/lab/playground-sv-permanente/laboratorio/tareas-grok/consultas-es27/ENCARGO-002.md): comprobación independiente del puesto corregido, preguntas nuevas y conservación de fallos. Su entrega permanece pendiente.

Cortes recibidos: Lenguaje 66801a7146a0a9151f0b2d7fc9f618ce76c37986 y laboratorio 07c8fec711635fcfe0dadf5418b18fe8be473923. Se conserva la lectura íntegra previa de AGENTS, Pilares, perfiles y transición, sin cambios en esos rectores; se reciben Fase 003 y RETP-115/116. La corrección y esta recepción se amparan en el encargo vigente de hacer funcionar y comprobar el puesto antes de optimizar. Los resultados se espejan en laboratorio y Calidad. Revocación, integridad adversarial de base, I01–I05, Qwen y auditoría integral de Claude siguen pendientes; producción, catálogo/localización y fila 9 conservan su estado.
