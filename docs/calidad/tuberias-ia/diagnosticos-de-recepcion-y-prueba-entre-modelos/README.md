# Diagnósticos de recepción y prueba entre modelos

**RETP-157 · 12/09/2026 · Paso 6 del plan de integración.**

Se ha comprobado una ampliación candidata de recepción y comparación: **239 pruebas unitarias conformes en debug y en release**, incluidas las 231 anteriores; **40 casos focales**, repetidos tres veces en cada configuración; **20 causas locales con mensajes ES/EN**. Los cuatro clientes adversariales producen los seis errores de acceso esperados. Se conservan los 18 procesos ejecutados y las 240 observaciones focales.

El cambio resuelve una pérdida de información del diagnóstico: ahora se sabe qué campo estaba vacío, excedía el límite o no coincidía. Esa causa se conserva donde se detecta. Los métodos anteriores mantienen sus tipos de error y sus decisiones; los métodos detallados añaden fase y causa. El idioma no participa en la aceptación, el rechazo ni el resultado de comprobación.

**Estado: conforme en el alcance nativo ensayado; cierre global pendiente.** La cápsula utiliza premisas sintéticas `for_test`. No se modifica el núcleo productivo ni se acredita recepción profesional, WASI, navegador o ejecución de Qwen, DeepSeek, Grok o Claude. Véase el [estado de cierre y relevo](ESTADO_DE_CIERRE.md).

## Léame primero

1. [Plan previo](PLAN_PREVIO.md): objeto, hipótesis, precedencia, dos fases y presupuesto.
2. [Catálogo de causas](CATALOGO_DE_CAUSAS.md): defecto, emisor, pruebas y localización; [CSV](CATALOGO_LOCALIZADO.csv).
3. [Resultado ejecutado](evidencia/RESULTADO.json), [observaciones](evidencia/OBSERVACIONES.json), [secuencia](evidencia/SECUENCIA.json) y [procesos completos](evidencia/PROCESOS.json).
4. [Encargo para otra IA](ENCARGO_PARA_OTRA_IA.md): instrucciones concretas y archivos de devolución, sin volver a explicar el escenario.
5. [Obligaciones pendientes](ESTADO_DE_CIERRE.md): alcance de la adenda, fases, DG01–DG14 y próxima sede de trabajo.

## Reproducción

Con Python 3 en un sistema POSIX y el compilador Rust identificado, ejecute desde esta carpeta:

```bash
python3 reproducir.py /ruta/al/rustc /ruta/a/un/directorio-nuevo
```

La ejecución no necesita red ni bibliotecas Rust adicionales. El directorio de destino debe ser nuevo; se evita sobrescribir evidencia anterior. El instrumento reconstruye y comprueba la cápsula, guarda los comandos antes de ejecutarlos y conserva las salidas antes de compararlas. Cada proceso tiene un límite de 60 segundos y 2 MiB por flujo de salida. Cualquier límite excedido o divergencia detiene la reproducción.

El compilador usado fue Rust 1.98.0, commit `88d9e12ae178fab0fb5cc050a94da85685d449ea`, host `x86_64-unknown-linux-gnu`. Su identificación completa, su huella y la del instrumento están en [IDENTIDAD](evidencia/IDENTIDAD.json) y PROCESOS. Se han preservado **76 archivos fuente** en [CANDIDATA_FUENTES](CANDIDATA_FUENTES.json), con ruta, SHA-256 y contenido Base64. Los cambios legibles están en [CAMBIO_INCREMENTAL.patch](CAMBIO_INCREMENTAL.patch) y `cambios/`.

La evidencia de la candidata procede de la versión exacta conservada en `soporte/reproducir_01.py`. La entrada actual corrige después una espera sin límite al cerrarse ambos flujos; tres pruebas instrumentales separadas la comprueban. Véase la [nota de la corrección](soporte/NOTA_DEL_INSTRUMENTO.md).

Los programas auxiliares usados para preparar el corte están en `soporte/`. Conservan sus rutas de trabajo originales y documentan cómo se produjo la cápsula; la entrada portátil de reproducción es `reproducir.py`. El manifiesto identifica los archivos del expediente. Excluye su propio contenido y el contenedor ZIP para evitar una referencia circular.

## Qué se ha observado

La fase legítima recibe el acto instalado, obtiene su continuidad, conserva la referencia exacta y acredita la observación idéntica. La fase negativa cambia negación, cobertura, sujeto o valor e incorpora una instrucción ajena; todos esos cuerpos quedan refutados por el comparador de igualdad, que conserva original y observado. Un original correcto vuelve a ser aceptado después de esos contrastes. La observación ausente da `NotVerifiable`, la presente vacía se compara como tal y los excesos de tamaño siguen siendo errores técnicos.

Los mensajes usan texto estático; no incluyen datos del acto, del contrato o de la observación. Sus causas se enlazan con los emisores en [INVENTARIO_EMISORES](INVENTARIO_EMISORES.json). Los diccionarios [ES](idiomas/es/diagnosticos.json) y [EN](idiomas/en/diagnosticos.json) pertenecen al contrato local `RECEPTION-DIAGNOSTICS/1`. No amplían ni renumeran los 51 códigos canónicos del Lenguaje.

## Medición y coste

| Medida observada | Debug | Release |
| --- | --- | --- |
| Compilación de pruebas, segundos de pared | 3,198529 | 10,132007 |
| Regresión unitaria completa, segundos de pared | 0,052637 | 0,045657 |
| Tres procesos focales, segundos de pared | 0,007428 / 0,006609 / 0,006890 | 0,004703 / 0,005317 / 0,005034 |

Son muestras locales con arranque de proceso. PROCESOS conserva nanosegundos, CPU y unidades. La memoria registrada es el **máximo acumulado de los procesos hijos del instrumento**; no puede atribuirse al comparador ni a cada caso. Las duraciones por observación incluyen comprobaciones e instrumentación. No son latencia de inferencia ni prueba de mejora de recursos frente a otros lenguajes o modelos. El coste de proveedores externos no se ha observado.

## Continuidad del trabajo

Este corte desarrolla los apuntes sobre [auditoría del trabajo de la IA](../trazabilidad-auditoria-y-reproduccion-del-trabajo-ia/README.md) y mantiene la referencia [volcán, polígonos y logo](../frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md). Prosigue la [recepción y comprobación RETP-154](../recepcion-gobernada-y-comprobacion-observada/README.md), dentro del plan RETP-147 y su rectificación RETP-148.

La conformidad humana y las luces verdes autorizan continuar el trabajo y conservarlo. La reproducción de este banco ofrece evidencia acotada; la revisión externa y la aceptación final conservan su función. Este encargo no exige que otro modelo copie el procedimiento de Watson: exige que permita comprobar lo que hizo y obtenga el resultado contractual correcto.
