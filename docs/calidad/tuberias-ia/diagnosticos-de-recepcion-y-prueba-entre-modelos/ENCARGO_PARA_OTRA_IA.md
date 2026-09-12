# Encargo de reproducción y revisión independiente

**Para:** una ejecución identificada de Qwen, DeepSeek, Grok o Claude. **Banco:** público RETP-157. **Sin apertura de reserva P3.**

## Texto del encargo

Reproduzca el paquete `diagnosticos-de-recepcion-y-prueba-entre-modelos` en un directorio nuevo. Lea primero README, PLAN_PREVIO, ESPERADOS y el catálogo. Conserve los archivos originales. Declare proveedor, identificación de modelo expuesta por su plataforma, fecha, entorno, instrucciones visibles relevantes, herramientas disponibles y exposición previa al banco. Si algún dato no está disponible, indíquelo como no observado; no lo invente.

La tarea tiene dos fases: comprobar el recorrido legítimo y contrastar sus negativos. Se usa una premisa sintética de ensayo; no se constituye autoridad clínica ni se modifica el núcleo productivo. No transmita datos de pacientes ni use credenciales ajenas.

Puede utilizar un procedimiento distinto para reproducir y revisar. Deben permanecer fijos los bytes de entrada, los contratos, la identidad de la candidata y los resultados esperados. La elegancia del programa o su consumo se valoran aparte. No traduzca ni normalice las entradas para conseguir el resultado. No cambie el esperado después de observar la salida. No ejecute tandas adicionales para perseguir conformidad.

1. Identifique las versiones y compruebe los SHA-256 del manifiesto. Conserve el paquete exacto recibido y su procedencia por commit.
2. Guarde todo script o código antes de ejecutarlo. Si lo modifica, conserve ambas versiones y el cambio. Registre cada comando y cada llamada de herramienta/conector realmente realizados, sus entradas recuperables, salidas, errores y orden. Una herramienta no utilizada se declara como tal; una herramienta no observable se declara como no observada.
3. Ejecute `python3 reproducir.py /ruta/al/rustc /ruta/a/un/directorio-nuevo`. El compilador de referencia es Rust 1.98.0, commit `88d9e12ae178fab0fb5cc050a94da85685d449ea`, destino nativo `x86_64-unknown-linux-gnu`. Si cambia versión o destino, preserve ese hecho; no atribuya automáticamente la diferencia al modelo.
4. Contraste los 40 identificadores de ESPERADOS, sin duplicados ni omisiones, en cada una de las tres repeticiones de debug y release. Compruebe la regresión de 239 pruebas por configuración y los cuatro clientes negativos: sus errores de compilación son el resultado esperado, no un fallo de ejecución del banco.
5. Revise que la referencia del comparador procede de la instalación fijada, que los textos ES/EN no deciden el resultado y que los métodos de instalación siguen siendo internos. Una revisión de fuentes puede señalar defectos aunque la batería pase.
6. Conserve también cualquier interrupción o fallo. Indique el primer caso divergente, esperado, observado, fuente y capa responsable cuando pueda determinarla. Una ejecución incompleta no obtiene conformidad.
7. Entregue su directorio de resultados y los scripts propios utilizados. Añada un informe breve que distinga reproducción ejecutada, revisión documental y extremos no observables. No presente una explicación retrospectiva como registro de operaciones que no se capturaron.

## Archivos mínimos de devolución

| Archivo | Contenido exigido |
| --- | --- |
| `IDENTIDAD_MODELO.json` | Proveedor, modelo observado, UTC, entorno, exposición y campos no observados |
| `INFORME.md` | Resultado por fase, hallazgos, limitaciones y vínculo a la evidencia |
| `SECUENCIA_MODELO.jsonl` | Acciones visibles del participante, en orden; identificador, inicio/fin si están disponibles, herramienta, referencia a entrada, salida y artefactos |
| `scripts/` | Scripts y código propios, con versiones originales y cambios |
| Directorio producido por `reproducir.py` | IDENTIDAD, SECUENCIA, PROCESOS, OBSERVACIONES, RESULTADO y fuentes reconstruidas |
| `MEDICIONES.json` | Duraciones y unidades; método; CPU/memoria cuando se midan; tokens o importe sólo si el proveedor los expone |
| `MANIFIESTO_DEVOLUCION.json` | Ruta relativa, tamaño y SHA-256 de cada archivo devuelto |

La secuencia del modelo es una declaración atribuida y debe distinguirse de la secuencia capturada por el instrumento. El instrumento registra sus propios procesos; no observa las operaciones ocultas de la plataforma ni autentica la identidad comercial del modelo.

Se admite un resumen de las decisiones y de su justificación. No se solicita la cadena interna privada de pensamiento. La parte auditable es el trabajo materializado, su evidencia y su secuencia observable.

## Decisión del receptor

Se compararán primero identidad de entrada, candidata, contrato y esperado; después resultado, causa y mensajes. Si difiere sólo la prosa del informe, no se considera una divergencia funcional. Si difiere un resultado contractual bajo las mismas condiciones, esa ejecución falla. Si faltan datos para comparar condiciones, el extremo queda no acreditado.

La coincidencia de varios modelos no reemplaza el oráculo. Este banco es conocido: su reproducción acredita únicamente ese alcance. Cada participante conserva su propia autoría, instrumental y límites; ninguna ejecución local de Watson se atribuirá a otro proveedor.

**Estado al publicar:** encargo preparado; cero ejecuciones externas recibidas. No hay sesión conectada de esos proveedores en esta intervención. Este documento permite trasladar el encargo sin volver a explicar el escenario.
