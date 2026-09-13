# Fuentes, corte y alcance de C07

**S22 · RETP-2026-210 · 13 de septiembre de 2026**

Corte público `f02975d9542905bd837acde07c68906178afcbeb` y laboratorio `709d7ff08c0a386d5e595d18dc9c934117c043f2`. Ambas referencias se comprobaron antes de preparar el incremento. Los tres rectores previamente leídos completos conservan identidad de bytes frente a C06: Pilares, perfiles/contratos/ensamblaje y transición secuencial con sus adendas. La secuencia del workflow continúa en BIS-02.

Se leyó completo el §7 de Fundamentos; se consultaron Frontera A.6–A.10 y D.1–D.5, e IR 0.2 J1.3/J1.4/J2.2/J2.3/J3.2. Se leyó el contraste anterior de composición y biblioteca. Las formas de las fuentes SVP se apoyan en los ejemplos existentes compose_basic y gate_table; se sustituyen sus nombres y parámetros particulares por un montaje sintético explícito, con 16/25 coordenadas.

La inspección Rust comprende las ramas CellSpec/CoupledSpec/Connector y las funciones validate_coupled_state, validate_graph, validate_admissibility_table, validate_gate y la rama Compose de validate_operation en wellformed.rs. Se consultaron los campos de ir.rs y las referencias de operaciones en frontend.rs. No existe parser.rs en esta organización: el analizador está en frontend.rs. Se conservan las deudas reconocidas de procedencia del puente, conflicto General y resultados sin productor; no se atribuye ejecución a la aceptación de estructura.

La previsión de admisión/rechazo de cada fixture deriva de esas guardas y del contrato, y queda fijada antes de ejecutar. No es resultado observado. La inspección de Compose comprueba existencia y tipos de referencias; no interpreta la ley indicada por el texto de constraints. La tabla/gate conserva comprobaciones posicionales, sin demostrar la ligadura semántica por la mera declaración de RGate.

## Evidencia de este incremento

Contrato candidato, dieciocho fuentes SVP, cuatro referencias de montaje, tres oráculos completos de sustitución y nueve filas de compuerta. La verificación auxiliar Python coteja integridad y consistencia de los datos esperados. No ejecuta un parser SVP ni reemplaza la recepción Rust. Ningún fixture C07 se ha compilado o ejecutado en este incremento.

Se conservan todos los expedientes anteriores y sus resultados. El [criterio de aceptación Rust](../bis-c05/CRITERIO_DE_ACEPTACION_RUST_v1.md) permanece vigente. Los adjuntos médicos y cascarones no se modifican ni se importan a los dominios; sus enseñanzas ya están recogidas en el contraste previo, cuya huella se conserva en FUENTES.

BIS-02: dos escenarios originales ejecutados, veintidós pendientes. C07: dieciocho variantes preparadas, cero ejecutadas. Próximo objeto C08. No se cierran DFL-001/006 ni la generalidad sobre un dominio; no se modifica semántica, IR o Rust. S24 sigue pendiente.
