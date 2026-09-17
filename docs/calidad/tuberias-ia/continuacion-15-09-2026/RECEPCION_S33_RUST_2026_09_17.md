# Recepción de recuperación instrumental S33 · 17/09/2026

**Seguimiento:** S33, revisión 1, finalizado en el alcance instrumental de esta instancia. **Calidad:** RETP-2026-251. **Unidad:** W-S32. **Corte receptor:** `aa858348615c6823a0fb6abf936770db6737f5e8`.

Se aplica la secuencia de recuperación del Acta 001 §10.3. Se conservan los Pilares de 05/09/2026, el Acta de perfiles de 06/09/2026 y la transición desde OP-IMM-001 con sus adendas; esta recepción no modifica sus decisiones ni el núcleo.

## Evidencia y resultado

El paquete procede del depósito auxiliar privado, commit `d45d8578af36b10b1e3216fd7ac2ff49cd888a47`. Se transfirieron individualmente cinco archivos: manifiesto de canal, suma del manifiesto y distribuciones rustc, rust-std y cargo. Sus huellas coinciden con las referencias del paquete. Los procedimientos se recuperaron del mismo corte; las once entradas de SHA256SUMS resultaron conformes. La comprobación acredita concordancia con las referencias recibidas, sin validación independiente de firmas del distribuidor.

| Actuación | Resultado observado |
|---|---|
| Instalación aislada Linux x86-64 | Inicio 07:33:15 UTC; terminación 0. |
| Identificación de cadena | Rust 1.98.0, LLVM 22.1.8 y Cargo 1.98.0. |
| Programa mínimo | Compilación y ejecución a las 07:33:54 UTC; terminación 0; salida exacta `RUST_1_98_0_MINIMO_OK vertices=4 area=12`. |
| Cotejo documental LEYENDA-CONTENIDO/3 | Compilación y ejecución a las 07:35:00 UTC; terminación 0; dos entradas conformes por tamaño, blob y SHA-256. |

[Informe y registros instrumentales](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/b5ddedbb5f7388cb503e571f91942de352e2b1f5/watson-herramientas/evidencias/receptor-linux-20260917/README.md), fijados al commit `b5ddedbb5f7388cb503e571f91942de352e2b1f5`, con acceso autorizado. La instalación y el programa mínimo conservan órdenes, salidas y retornos. El auxiliar conserva salidas, retornos y huellas; sus órdenes reproducibles se distinguen de una transcripción literal. El auxiliar utiliza la biblioteca estándar y no exige resolver dependencias externas.

## Alcance del cierre

Se acredita recuperación desde la unidad receptora para ejecución Rust nativa y cotejo documental. La transferencia por componentes no acredita la descarga o integridad del ZIP exterior. Cargo se comprobó mediante consulta de versión; no se ensayó un proyecto Cargo con dependencias. Las comprobaciones no cualifican el reconocedor, no representan una célula SV y no constituyen pruebas de privacidad.

La cadena reside en una instalación aislada y se invoca por ruta explícita. Su disponibilidad debe verificarse al reanudar cada sesión; los componentes conservados permiten repetir el procedimiento, pero no garantizan persistencia de una instancia transitoria.

S33 concluye este alcance. S34 conserva su recepción multiplataforma pendiente; la evidencia entregada por el entorno productor no se convierte en reproducción receptora por esta actuación. S22, S26, S32 y BIS-03 mantienen sus estados. H1 conserva las precisiones pendientes H1-01/H1-02. No se alteran mapa histórico, campañas, originales ni secuencia del proyecto.
