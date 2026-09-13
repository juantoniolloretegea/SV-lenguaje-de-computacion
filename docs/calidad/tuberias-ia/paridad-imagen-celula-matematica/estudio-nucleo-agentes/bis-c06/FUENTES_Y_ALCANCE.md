# Fuentes y alcance de BIS-C06

**S22 · RETP-2026-209 · 13 de septiembre de 2026**

Corte Lenguaje `de6d8809a3c01f98d90775f80d5b05982491ec18`; laboratorio `f2de5c22a807ce325c8f9983b95e312a4245c359`. Referencias de ramas verificadas antes de preparar el incremento. Los tres rectores previamente leídos completos conservan identidad de bytes frente al manifiesto C05. Se conserva el orden y las restricciones de AGENTS, Pilares, perfiles/contratos/ensamblaje y transición secuencial con sus adendas.

Se contrastaron Documento III §§3.2–3.7, 5 y 6 y Documento V §6 desde las copias identificadas en el manifiesto de concordancia anterior. Sus SHA-256 se verifican en [FUENTES.json](FUENTES.json), con repositorio y corte originales. La concordancia del Bis se leyó completa. Las cláusulas IR 0.2 de Frame, TransitionData, Trajectory y J4.1–J5.1 se leen junto con el cierre sucesor IR 0.3, que no impone exhaustividad.

La inspección Rust incluye `frame.rs`, `transition_data_wellformed.rs` y `context_wellformed.rs` completos; los campos pertinentes de `ir.rs`; y las ramas Frame/TransitionData/Trajectory de `wellformed.rs`. Se consultaron las entradas de deuda DFL-003/004/006 y las recepciones posteriores pertinentes: no se presenta la antigua candidata de guardas locales como una capacidad aún ausente en main. El módulo de TransitionData declara explícitamente no ejecutar transición ni decidir causalidad.

S14 y lectura vinculada se reciben con sus resultados anteriores, consultados en C05. No se repitieron sus campañas nativas. La [matriz](MATRIZ_DE_SEDES_Y_LIMITES.json) distingue esas realizaciones de las capacidades algebraicas pendientes. S14 pertenece al expediente de integración 1+3; no es un módulo del núcleo.

## Resultado y límites

Se prepararon un contrato, dieciocho especificaciones y un testigo abstracto de dos instantáneas. El testigo identifica una obligación de enlace entre proyección y custodia; no constituye una vulnerabilidad explotada, un cambio de IR ni una imposibilidad demostrada.

La verificación del incremento es documental y auxiliar Python. No se compila código nuevo, no se ejecutan los escenarios C06, no se habilita inicialización/reevaluación productiva y no se emiten nuevos diagnósticos canónicos. El [criterio Rust](../bis-c05/CRITERIO_DE_ACEPTACION_RUST_v1.md) sigue vigente. Los escenarios originales C06-P/N continúan pendientes, con seis positivos contractuales que requieren materialización y doce negativos por ensayar.

BIS-02 conserva dos escenarios originales ejecutados y veintidós pendientes. Sigue C07, composición tipada entre dimensiones diferentes. S24 continúa pendiente: GUI después de Bis, catálogo y cierre de fase.
