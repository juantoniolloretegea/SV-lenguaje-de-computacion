# Sedes e interfaz del montaje C02–C05

**S22 · RETP-2026-219 · Juan Antonio Lloret Egea y Watson · 13 de septiembre de 2026.**

Se ha resuelto la decisión documental de sedes para el primer recorrido integrado: **composición experimental en Rust sobre la API pública de `sv_core`**, con receptor y observador separados. No se necesita ampliar semántica o IR para ensayar este montaje; su realización todavía está pendiente.

| Documento | Objeto |
| --- | --- |
| [Decisión](DECISION.md) | Corte, evidencia de código, sedes, almacenamiento, auxiliares, exclusiones y relevo. |
| [Matriz de sedes](DECISION_SEDES.json) | Doce decisiones enlazadas a las doce obligaciones y a casos discriminantes. |
| [Interfaz](INTERFAZ.md) | Recepción acotada, fases de validación, objetos con construcción privada, captura y observación. |
| [Límites del adaptador](LIMITES_ADAPTADOR.json) | Complemento estructural experimental compatible con los canales del compromiso anterior. |
| [Banco instrumental previo](BANCO_ADAPTADOR_PREVIO.json) | 14 casos JSON y 5 de huella SHA, todos sin ejecutar en Rust. |
| [Fuentes](FUENTES.json) | Identidad y alcance efectivo de lectura; no confunde lectura parcial con completa. |
| [Verificación](VERIFICACION_DOCUMENTAL.json) | Compatibilidad estructural, límites literales, hashes y conservación de los 26 casos integrados. |
| [Conservación](CONSERVACION.json) y [manifiesto](MANIFIESTO.json) | Historial anterior, ámbitos de cambio e integridad del expediente. |

La API existente permite consumir IR aceptada y resolver CellSpec/CoupledSpec/CoupledState. La revisión de instancia y su enlace con el descriptor se comprobarán fuera de IR. LIG/0.1 conserva su contrato propio. Se propone un enum privado de arrays por tamaño admitido para el estado del experimento; esa propuesta no prueba coste ni habilita todos los tamaños de un dominio.

La revisión del decodificador anterior encontró dos adaptaciones necesarias: separar las reglas específicas de su antiguo transporte y conservar enteros con signo sin truncarlos a u64. Por eso se compromete el complemento instrumental antes de modificarlo. No se implementa otro parser SVP. SHA se reutilizará como integridad de bytes, sin trasladar el marco de transporte anterior.

**Paso siguiente:** materializar `sv_bis_i0205` y su conductor/observador en laboratorio; fijar las sondas de construcción; compilar con Rust/Cargo 1.98.0 y ejecutar primero auxiliares y después el banco integrado, conservando resultados adversos. BIS-03 queda en ejecución acotada. BIS-02 permanece abierto globalmente; BIS-04 conserva su realización pendiente. Los 26 casos de RETP-218 y los 202 anteriores están intactos y sin nuevas ejecuciones.

Se mantiene la GUI después de Bis, catálogo y cierre de fase. No se cierran imagen/consumo visual, ES/EN integrado, constituciones operativas, RAM/duración ni las deudas del núcleo.

## Reproducción y trazabilidad

`python soporte/verificar.py` repite la comprobación documental desde esta carpeta. Cuenta estructuras y coteja bytes; no ejecuta el decodificador Rust ni interpreta el SVP. Los scripts administrativos archivados conservan las rutas originales del espacio de trabajo y no deben repetirse para duplicar RETP-219. Los auxiliares administrativos de registro/publicación se reutilizan con la identidad registrada desde el expediente RETP-218.

Herramientas usadas: lectura y búsqueda local; scripts Python de documentación, hashes y registros; Git local; conector GitHub para lectura/publicación con base y blobs verificados. No se ha invocado una IA externa ni ejecutado el nuevo montaje. Las versiones `rustc` y `cargo` se comprueban instrumentalmente sin contar ese hecho como caso del banco.
