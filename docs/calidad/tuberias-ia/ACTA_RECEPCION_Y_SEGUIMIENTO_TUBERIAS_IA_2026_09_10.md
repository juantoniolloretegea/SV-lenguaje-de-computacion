# Acta de recepción y seguimiento de las tuberías de IA

**Fecha:** 10 de septiembre de 2026. **Registro:** RETP-2026-115.
**Elaboración técnica:** Watson, por encargo de Juan Antonio Lloret Egea.
**Estado global: NO VERDE — aislamiento material y control productivo pendientes.**

Esta acta reúne el resultado recibido y el siguiente ensayo preparado. Se conserva idéntica en el laboratorio y en docs/calidad/tuberias-ia del Lenguaje, conforme a la orden de la Dirección. No constituye un cierre humano ni una integración funcional.

## 1. Cortes y lectura rectora

- Lenguaje leído: main aeb5808242697f4d6f18b49da99f1a73eb4cf49d; árbol 25f3d66daf3fa5ce965b0eccf3cc0fd54b1a703b.
- Laboratorio de entregas: lab/playground-sv-permanente, 67eeceea6cb352d7d9b1b4ef248f2356c0230002.
- Fuente del receptor empleado: ce15fe22e1649f3392ef7af9314b4446e777d3c6; no sv_core productivo.
- [Retorno inmunológico consultado](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/54fe0d89c9e59065eae2bc8a38f5ec0832ece4b9/dominios/inmunologia/marco-tecnico-de-universos-subdominios-y-modulos/01-op-imm-001-informacion-preinmunosupresion-adultos/retorno-gh-2026-09-07/RETORNO_GH_OP-IMM-001_Q0_v0_AL_LENGUAJE_SV_2026-09-07.md): 54fe0d89c9e59065eae2bc8a38f5ec0832ece4b9, §§1–3.2 y límites de Q0.
- Lectura íntegra de AGENTS.md, Pilares, acta de perfiles y ensamblaje, acta de transición secuencial desde OP-IMM-001, arquitectura de núcleo/frontera/host y procedimiento de auditoría. Registros RETP-105–110 recotejados; PR #89 permanece separada con RETP-111–114 en su candidata.
- Autorización de este acto: reflejar resultados en Calidad y continuar la siguiente fase de laboratorio. El último ajuste humano exige una forma y algoritmo deterministas para NLP inicialmente en español.

La recepción identifica el alcance realmente comprobado, no reabre Inmunología ni crea células a partir de sus 27 parámetros.

## 2. Resultado recibido: operaciones-17

[Informe original conservado en laboratorio](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/67eeceea6cb352d7d9b1b4ef248f2356c0230002/laboratorio/tareas-watson/operaciones-17/recepcion-001/RESULTADO.md).

| Magnitud | Resultado observado |
| --- | --- |
| Entrega de Grok | c3b9a471e725e0a230dadc77bbe04ad42047669f |
| Objeto Git de propuestas.json | 34ffce57d74eda043b67882f80c185983b7c64f6 |
| Bytes recuperados y evaluados | 2406 |
| SHA-256 sobre esos bytes | 56840d45ede24480d5e53fc0e23ef79df891d6444e3b761386d05ba47cbba07f |
| Propuestas conformes | 8 de 8 |
| Decisión propuesta por Grok | Tramitar 3; retener 5 |
| Ejecuciones del receptor | 8 nativas y 8 WASI |
| Paridad de salida | 8 pares literalmente idénticos; código 0 |
| Suma de duraciones completas de proceso | Nativo: 22153202 ns; WASI: 424508483 ns |
| Efectos ejecutados | 0 |

[Resultado instrumental espejado, con cada medida](lab035-002/RESULTADO.json). Son exactamente los bytes del resultado del laboratorio. La entrega del participante y sus metadatos de origen se conservan en sus commits originales.

El filtro admitió ocho propuestas correctas. No bloqueó cinco propuestas inválidas: las cinco retenciones fueron decisiones propuestas correctamente por Grok. Esta captura no acredita por sí sola rechazo de una salida maliciosa del modelo. Los controles sintéticos previos del banco conservan su propio estatuto y sus resultados no se atribuyen a Grok.

Las duraciones incluyen el proceso completo; WASI añade su anfitrión, compilación e instanciación. No se deduce una regresión, un coste intrínseco ni una necesidad de refactorización de estas dieciséis medidas. Los checkpoints anteriores se conservan.

## 3. Qué permanece pendiente

No quedan acreditados aislamiento de memoria, confidencialidad, bloqueo de escritura de la base, contención de un proveedor/anfitrión comprometido ni control productivo del núcleo. La nota libre de Grok no fue validada semánticamente por el filtro. Tampoco se verificó una población de sesiones independientes ni determinismo del modelo.

La instrumentación se reservó antes de la captura. No existe una traza completa que permita afirmar desconocimiento absoluto del modelo. La primera tanda abierta conserva su historial y no se convierte retrospectivamente en prueba ciega aprobada.

La publicación de esta recepción no autoriza a Claude, a Grok ni a otra unidad a cerrar el experimento. No se ha instalado ni validado Qwen. El conjunto continúa NO VERDE.

## 4. Siguiente fase preparada

El [protocolo Fase 003](FASE_003_NLP_ES_CONOCIMIENTO_DETERMINISTA.md) contiene una gramática inicial en español, la relación entre parámetro y registro versionado, el algoritmo de consulta y emisión y 18 controles positivos/adversariales: D01–D13 e I01–I05.

La propiedad buscada es identidad literal de respuesta para la misma pregunta, versiones y permiso. La gramática, la selección y el emisor no dependerán de una elección nueva del LLM. Los cambios de conocimiento o autorización serán sucesos explícitos; una revocación no podrá eludirse por exigir repetir una respuesta antigua.

La relación uno a uno y el uso de identificadores se reciben como propuesta de realización. El hash fija bytes; no transforma texto libre en conocimiento matemático ni acredita legitimidad. No se altera la constitución real de un parámetro, sus fuentes, condiciones o criticidad.

**Estado de la Fase 003: algoritmo y pruebas especificados; implementación y ejecución pendientes.** Ninguno de sus 18 controles se presenta como ejecutado. La preparación identifica qué debe ejecutarse después y conserva la exigencia de la misma fuente Rust para nativo/WASM y de pruebas materiales para aislamiento.

## 5. Relevo y alcance del cambio

Se añade esta recepción y el protocolo; se espeja el resultado exacto y se actualizan navegación y RETP CSV/Markdown. Los archivos previos se conservan por sucesión. No cambian código, gramática SV, IR, núcleo, dominios, catálogo, localización ni flujos de ejecución.

La prioridad continúa: control subordinado de IA externa; después catálogo/localización; finalmente fila 9. La candidata diagnóstica #89 conserva sus IDs reservados y su estado. La auditoría pública integral de Claude queda pendiente de la luz verde que le corresponde.

El próximo resultado material deberá probar el algoritmo y los permisos; la presente acta no lo adelanta. El cierre humano y cualquier decisión productiva permanecen en la Dirección.
