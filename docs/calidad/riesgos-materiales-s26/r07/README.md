# S26 R07 · Falsación de auxiliares heredados

**Banco previo. Ocho sondas especificadas; aún no ejecutadas en esta revisión.**

Entrada: Lenguaje/main `11d441c979c549b7fa459635df014ae987ccde1f`; laboratorio/lab/playground-sv-permanente `3a3a1d6a4101602edb1324012b1d8406125849eb`. Se aplica [Control de auxiliares](../CONTROL_DE_AUXILIARES.md), Pilares, acta de perfiles/contratos/ensamblaje y acta de transición secuencial cotejados en la continuidad anterior. Se conservan R01–R06 y los doce casos globales S26 abiertos.

## Objeto y justificación de herramientas

`observar.rs` es el conductor y contiene los oráculos previos. Se compila con `/opt/sv-rust-1.98.0/bin/rustc`, sin dependencias externas. No requiere Cargo. Las copias de los dos auxiliares se incorporan literalmente al compilar mediante `include_bytes!`; no se modifica su código ni se usan para el registro o publicación reales de esta campaña.

Python se utiliza exclusivamente para ejecutar esos objetos heredados, en procesos hijos con `-I -B`, entorno vaciado y, cuando corresponde, `-O`. La razón frente a Rust es comprobar el comportamiento del código Python real bajo su intérprete: traducirlo previamente cambiaría el objeto. El adaptador GitHub se sustituye explícitamente por un doble local de respuestas fijas y registro de llamadas. El pequeño driver Python sólo invoca el publicador y muestra su retorno. Ambos están visibles como literales en el conductor Rust. No hay SDK, credenciales ni llamadas reales de red en ese doble.

El conductor conserva stdout, stderr, código de salida, directorios y resultados por caso. Lanza argumentos directamente, sin shell. El límite de espera es diez segundos por hijo; agotarlo invalida el oráculo y solicita terminar ese hijo. La cuota de salida de 64 KiB por canal se comprueba después de terminar: no se presenta como límite preventivo de escritura o memoria. No hay cuota de RAM ni contenedor de aislamiento acreditados. El vaciado del entorno no elimina permisos del sistema operativo.

## Oráculos fijados antes de ejecutar

| Caso | Estímulo | Resultado previsto y evidencia |
| --- | --- | --- |
| P01 | Publicación sintética vacía con base correcta; Python normal. | Éxito, una llamada local update_ref, checkpoint verified; control positivo de la ruta. |
| P02 | Misma ruta con base esperada distinta; Python normal. | AssertionError tras primera lectura; sin checkpoint ni llamada local de escritura. |
| P03 | Bytes de P02 bajo `-O`. | Guarda eludida: ruta alcanza update_ref del doble y declara PUBLICACION_VERIFICADA. Es un fallo del auxiliar; no una publicación real. |
| P04 | Checkpoint `verified` anterior con encargo/repo/base distintos y entrada inexistente. | Retorno exitoso anterior sin llamadas al doble ni lectura exigida de entrada; checkpoint literal intacto. |
| P05 | Registro sintético correcto con revisión 1. | Cinco archivos administrativos actualizados; resultado y revisión observados por Rust; control positivo. |
| P06 | Revisión solicitada 99 cuando corresponde 1; Python normal. | AssertionError; cinco archivos permanecen idénticos. |
| P07 | Bytes de P06 bajo `-O`. | Registro aceptado como revisión calculada 1 pese a solicitud 99; guardas eludidas. |
| P08 | Asiento correcto con Markdown de Sucesos ausente. | FileNotFoundError después de cambiar CSV de Sucesos e historial; RETP CSV/MD intactos. Escritura parcial observada. |

P03, P04, P07 y P08 tienen como esperado la reproducción de una debilidad. Cumplir el oráculo no convierte el auxiliar en conforme ni seguro. P08 provoca un error real de archivo ausente; no es una interrupción eléctrica, fallo físico ni terminación forzada durante escritura.

El directorio de campaña debe ser nuevo; si existe, el conductor rechaza reutilizarlo. Los fixtures son sintéticos y no contienen datos clínicos. Las fechas de fixture son datos de prueba, no fechas del expediente.

## Reproducción después del precompromiso

Desde esta carpeta, compile fuera del árbol documental:

```text
/opt/sv-rust-1.98.0/bin/rustc --edition=2021 observar.rs -o /tmp/s26-r07-observar
/tmp/s26-r07-observar /tmp/s26-r07-campana-nueva /opt/codex/runtimes/codex-primary-runtime/dependencies/python/bin/python3
```

Las rutas son ejemplos explícitos; conserve la versión efectiva del intérprete y compilador. No interprete el código de salida del conductor como verificación de seguridad global: sólo indica concordancia de estos ocho oráculos. La fuente, el banco y las huellas se publicarán antes de ejecutar los casos. Los resultados se añadirán en un cambio posterior.

## Límites y continuación

Se examinan guardas Python, reutilización de checkpoint y escritura secuencial sobre archivos reales locales. El transporte GitHub está simulado y no acredita semántica transaccional remota, carreras reales, rollback, durabilidad o resistencia al host. Tampoco protege la RAM del núcleo ni ejecuta los ocho discriminadores del contrato R06.

Tras recibir resultados, priorizar una sustitución acotada en Rust de las funciones de control necesarias, conservando el antecedente y sus fallos. Hasta entonces, las operaciones reales utilizan el conector directo y cotejos explícitos; no los auxiliares examinados. R06 conserva su continuación material. Sin selección de BD, GUI o host y sin cierre de Bis/S24.
