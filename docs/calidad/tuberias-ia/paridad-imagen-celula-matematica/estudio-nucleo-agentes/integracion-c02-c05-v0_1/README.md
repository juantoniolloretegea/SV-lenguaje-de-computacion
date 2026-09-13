# Recorrido integrado C02–C05 · compromiso previo 0.1

**S22 · RETP-2026-218 · Juan Antonio Lloret Egea y Watson · 13 de septiembre de 2026.**

Quedan fijados el [contrato de integración](CONTRATO_INTEGRADO.md) y el [banco de 26 casos](BANCO_PREVIO.json): siete positivos y diecinueve negativos **previstos**, con cero ejecuciones Rust de esta integración. Los casos sirven para confrontar una realización futura; su preparación no constituye una garantía de funcionamiento.

El recorrido enlaza soporte tecnológico, identidad de célula/revisión, geometría matemática exacta y entrega documental local. Mantiene separados petición, contexto independiente, estímulo de captura y resultado esperado. A/r1, B/r1 y A/r2 pueden compartir bytes sin compartir identidad.

| Pieza | Contenido |
| --- | --- |
| [Solicitudes](SOLICITUDES.json) | Entradas literales para el sujeto futuro. |
| [Contextos confiables](CONTEXTOS_CONFIABLES.json) y [registro](REGISTRO_CONFIABLE.json) | Identidades, soporte y destinos custodiados fuera de la petición. |
| [Oráculos](ORACULOS.json) | Primera guarda, resultado, recibo y entrega esperados; observaciones vacías. |
| [Planes de inyección](PLANES_DE_INYECCION.json) | Estímulos futuros de captura y sustitución; no evidencia de una entrega ocurrida. |
| [Presupuesto](PRESUPUESTO.json) | Cuotas experimentales, límite geométrico inclusivo 8192 bytes; memoria y tiempo de proceso pendientes. |
| [Fuentes](FUENTES.json) y [manifiesto](MANIFIESTO.json) | Corte de entrada, procedencia y hashes de los archivos. |
| [Verificación documental](VERIFICACION_DOCUMENTAL.json) | Referencias, fórmulas, deltas, longitudes y oráculos comprobados administrativamente. |
| [Control de conservación](CONSERVACION.json) | Registros anteriores intactos, cambio de S22 y secuencia preservados. |

La comprobación documental se puede repetir con `python soporte/verificar.py` desde esta carpeta; sólo lee el expediente y escribe su informe en la salida estándar. **Ese script no compila ni interpreta SVP.** Los scripts administrativos archivados en `soporte/` conservan el contenido utilizado para preparación, registro y publicación; sus rutas corresponden al espacio de trabajo original. No deben ejecutarse para volver a registrar el mismo RETP ni para sobrescribir el banco comprometido.

La geometría positiva se cotejó con fórmulas exactas y el estado previo de C03. La geometría permutada tiene el mismo recuento de símbolos y difiere posicionalmente. Se fijaron escenarios para versión de soporte, CellSpec no seleccionada, cambio de identidad/revisión, pérdida o sustitución de captura, nota imperativa sin autoridad, perfil ausente y límite de tamaño. Se especifican además cuatro sensibilidades del observador, aún sin ejecutar.

**Siguiente paso:** resolver sedes e interfaz de recepción/observación y las condiciones de paso aplicables a BIS-03; después realizar y ensayar este recorrido en Rust. BIS-02 sigue abierto y este incremento no abre formalmente BIS-03. Los 202 casos anteriores C02–C12 siguen sin ejecutar. C01 conserva sus trece variantes históricas; S25 acredita la recuperación instrumental de Rust/Cargo 1.98.0.

El perfil integrado inicial es `en`; ES/EN integrado, imágenes, consumo visual de IA, efectos de agentes externos, RAM y duración conservan sus obligaciones pendientes. Los registros sintéticos no constituyen dominio operativo ni permiso productivo. S24 mantiene: **Bis → catálogo y cierre de fase → análisis e instalación de la GUI**.

## Trazabilidad de esta preparación

Se utilizaron comandos de lectura/búsqueda, scripts Python administrativos, el comprobador documental, Git local y el conector GitHub para lectura y publicación con comprobación de base y blobs. No se ejecutó la nueva ruta SVP, no se invocaron modelos externos ni servicios clínicos. Las herramientas de publicación archivadas muestran los argumentos y procedimientos; las credenciales permanecen en la infraestructura del conector, no en los artefactos.

Durante la preparación se corrigió una ruta de copia inexistente (`constitucion-v1.json`) por el nombre real de C03 (`constitucion-1.json`). El primer intento se detuvo con `FileNotFoundError`; se retiraron únicamente sus dos copias incompletas y se repitió. Los activos originales quedaron intactos. No fue una prueba Rust ni un resultado del banco.
