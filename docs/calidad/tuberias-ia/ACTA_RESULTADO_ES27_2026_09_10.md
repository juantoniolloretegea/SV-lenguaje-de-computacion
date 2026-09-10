# ES27: puesto funcionando y entrega a Grok

**RETP-2026-116 · 10/09/2026. Estado: comprobación previa conforme; recepción de Grok pendiente. Aislamiento global: NO VERDE.**

La orden humana exige comprobar funcionamiento antes de optimizar. Se ha construido el puesto con la misma fuente Rust para nativo y WASM/WASI y se ha ejecutado [la campaña 34475250778, intento 1](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/actions/runs/34475250778).

| Comprobación real | Resultado |
| --- | --- |
| «¿Cuál es la capital de España?» | Madrid, en ambos destinos. |
| Casos funcionales | 56 conformes: 15 positivos y 41 negativos. |
| Repetición | Cada positivo, 100 veces por destino; 1500 respuestas por destino, idénticas a sus esperados. |
| Propuestas de IA contradictorias | No sustituyeron la respuesta del motor. Se usaron dobles sintéticos, no nuevas respuestas de Grok. |
| Transporte malformado | 6 controles rechazados en ambos destinos. |
| Observador | Detectó la sustitución deliberada Madrid → Barcelona. |

[Resultado y 28 muestras de proceso](es27-001/RESULTADO.json), [oráculos previos](es27-001/oraculos.json), [salida nativa](es27-001/control-nativo.jsonl) y [salida WASI](es27-001/control-wasi.jsonl). Cinco lotes comparables de 15 preguntas: mediana de proceso nativo 4097742 ns y WASI 52383564 ns; RSS máximo 2128 y 53316 KiB. Incluyen arranque y, en WASI, anfitrión/compilación/instanciación; no son una regresión ni una decisión de optimización.

[Encargo para Grok](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/lab/playground-sv-permanente/laboratorio/tareas-grok/consultas-es27/ENCARGO.md): deposita consultas, el puesto las ejecuta automáticamente y Grok recibe y registra las respuestas. No requiere Rust instalado en su sesión. Todavía no existe una entrega de Grok para ES27.

La realización acota Fase 003 y amplía explícitamente sus formas españolas con G-ES27/1. La base LAB-ES27/1 tiene 27 literales artificiales y permite sólo registros 1 y 7. Madrid procede del ejemplo humano; SIM-007 es ficticio. No reproduce OP-IMM-001, no constituye células ni realiza toda la Fase 003. Integridad adversarial de base, revocación entre decisión y uso e I01–I05 siguen pendientes.

Fuentes fijadas: 9ca6036afe7387431b7591791ddbce74498ff9c9. Lanzador/entrada: b6209141600e2386480d84715623142169112a81; blob del workflow f9606dc5fee6e29c8e4e1451054611f3516ad7ce. Artefacto 10151188295: ZIP recuperado de 1714195 bytes, SHA-256 a3f5452b5868b5b2db2e88672a1212403f819b07e0bec996753dc3a57afcb8b7. Se cotejaron fuentes, ejecutables y resultados; sus identidades completas constan en el JSON. El artefacto de Actions declara caducidad 10/10/2026; las fuentes, oráculos y resultados quedan versionados en repositorio.

Cortes leídos: Lenguaje 094a6ab671395578d1613c8e4137bdbfe83d4718 y laboratorio f373e8f41ae9873ee7682e0c5f0516e32e95188f. Lectura completa de AGENTS, Pilares, perfiles y transición; Fase 003 y RETP-115. Esta recepción actualiza el estado por sucesión y se espeja en laboratorio/Calidad. No modifica producción ni cierra catálogo/localización, fila 9, Qwen o auditoría de Claude.
