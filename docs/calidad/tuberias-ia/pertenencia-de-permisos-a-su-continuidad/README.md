# Pertenencia de permisos y compromisos a su continuidad

**RETP-2026-153 · 12/09/2026 · Fallo reproducido y corrección candidata contrastada; pendiente de integración productiva.**

El núcleo examinado aceptaba un permiso y un compromiso de otra continuidad cuando coincidían el ordinal local de decisión y los datos constituidos. La sonda interna reprodujo ambos cruces: el segundo llegó al ejecutor de prueba. La candidata añade pertenencia de instancia y los rechaza antes de mediar o ejecutar.

## Evidencia causal

| Caso | Fuente vigente | Candidata |
| --- | --- | --- |
| Cadena propia de decisión, mediación y ejecución | Acepta; una llamada | Acepta; una llamada |
| Permiso A presentado en continuidad B con traza igual | Acepta; marca B como mediada | `ForeignContinuity`; B no queda mediada |
| Compromiso A presentado en B, ambas previamente mediadas | Ejecuta; una llamada y dos eventos en B | `ForeignContinuity`; cero llamadas y cero eventos de ejercicio en B |
| Identidad agotada | Sin guarda de instancia en esta vía | Rechazo técnico antes de nueva decisión; sin traza añadida |
| Frontera del contador | No aplicable al código previo | No desborda ni reutiliza; agotamiento estable |
| Asignación concurrente | No aplicable al código previo | 128 asignaciones distintas en ocho hilos de la sonda |

[Plan previo](PLAN_PREVIO.md), [fallo reproducido](CONTRASTE_PREVIO.json), [resultados de la candidata](CANDIDATA_RESULTADOS.json), [resultado consolidado](RESULTADO.json) y [parche revisable](CAMBIO_CANDIDATO.patch).

La batería unitaria de `sv_core` pasa **217/217 en depuración y 217/217 en optimización**: 211 pruebas anteriores y seis testigos nuevos por modo. No es toda la conformidad del repositorio ni una campaña WASM. El cliente externo confirma que no se puede mutar el identificador privado ni acceder a la inyección exclusiva de pruebas; se obtienen E0616 y E0599 al compilar contra el artefacto sin `cfg(test)`.

El primer intento falló al compilar por 12 archivos de ejemplo ausentes, citados en 13 inclusiones. Se recuperaron los originales del mismo commit y se cotejaron sus blobs. [ANTECEDENTE.json](ANTECEDENTE.json) conserva esa incidencia de montaje. No se cuenta como fallo funcional ni se cambiaron sus esperados. Después se confirmó el defecto, se preparó una corrección causal y se ejecutó la batería. Los cuatro avisos preexistentes del compilador se conservan en las salidas; no forman parte del cambio.

## Cómo se corrige y cuánto añade

Cada `ProtectedDecisionContinuity` recibe una identidad interna al construirse. Se conserva al mover el objeto. El permiso y el compromiso llevan esa identidad en campos privados; mediación y ejecución la comparan antes de consultar la traza local o llamar al ejecutor. Se conservan `decision:1`, los contenidos públicos de las trazas y las reglas existentes de autoridad, requisitos y efectos. La igualdad textual de una traza deja de bastar para transportar un token entre instancias.

La identidad se asigna mediante un entero atómico con incremento comprobado, sin reloj, entrada del participante ni reutilización. Al agotarse el contador, la continuidad nueva queda sin identidad operativa y no puede emitir decisiones. Es una limitación de esta realización, no una modificación de los naturales o de las células SV. El identificador no es secreto, evidencia humana ni autoridad; conocerlo no permite construir o mutar el token mediante la API pública segura.

[Coste de representación y frontera pública](FRONTERA_PUBLICA_Y_COSTE.json), medidos en esta compilación nativa:

| Tipo | Previo | Candidata | Diferencia |
| --- | ---: | ---: | ---: |
| Continuidad protegida | 248 B | 264 B | +16 B |
| Permiso trazado | 408 B | 416 B | +8 B |
| Compromiso trazado | 408 B | 416 B | +8 B |

Se añade un contador estático de 8 bytes por instancia cargada del crate. La guarda nueva no añade reservas dinámicas; las estructuras y asignaciones previas siguen existiendo. Estos tamaños Rust no constituyen ABI estable, memoria residente, coste total de servicio ni prueba de latencia. La operación atómica puede reintentar bajo contención: esta prueba no fija un tiempo máximo universal ni abre P4/P5. Sólo se ha contrastado el destino nativo disponible; otros perfiles deben verificar su soporte atómico y su comportamiento antes de adoptar el parche.

## Alcance de seguridad y de autoridad

La sonda usa el montaje sintético interno de R1: génesis y resultados de comprobación de prueba. A partir de él atraviesa las funciones reales de decisión, mediación y ejecución. **No acredita una autoridad profesional, una entrada remota explotable ni una cadena productiva de admisión.** Es un contraejemplo al aislamiento entre instancias del modelo lógico; no se presenta como compromiso de un sistema desplegado.

La protección propuesta es intra-proceso, dentro de la misma realización del crate, bajo Rust seguro y el custodio confiable supuesto. No ofrece autenticación entre procesos, resistencia a manipulación de memoria por host o identidad durable tras reinicios. No cambia la identidad de todos los referentes constituidos ni la procedencia de resultados de requisitos. DFL-005 sigue abierta; no se declara resuelta por añadir este identificador.

`ContinuityExhausted` y `ForeignContinuity` son variantes candidatas de errores de R1, no nuevas claves canónicas SV ni U. La incorporación de variantes puede afectar a consumidores que hagan coincidencia exhaustiva sobre esas enumeraciones; su integración debe incluir catálogo/localización y los consumidores pertinentes. La inyección `simulate_identity_exhaustion` sólo existe bajo `cfg(test)` y no admite autoridad ni crea permisos.

## Qué queda para continuar

La [tabla de productores y contrato de entrada pendiente](PRODUCTORES_Y_LIMITE_DEL_ENLACE.md) concreta dos carencias: admisión externa material de la premisa de autoridad y producción gobernada de `RequirementCheck`. Los productores internos de forma, efecto, resolución y decisión sí están localizados. No se volverá a pedir al autor que defina quién autoriza ni qué significa frame.

Orden de continuación: revisar esta corrección candidata y su incidencia en los diagnósticos; concretar la admisión de referentes y comprobaciones bajo los actos ya constituidos; realizar el enlace aplicable sin fabricar productores. La lectura RETP-152 permanece disponible. El cierre acotado de la campaña alimenta catálogo/localización y la fila 9 conforme al workflow V2. La auditoría RETP-151 conserva las leyes comunes del núcleo y el soporte pendiente para la puerta algebraica. No se decide el reparto particular dominio/agente ni se abren reserva P3, asociación /2–/3, P4/P5 o interfaces profesionales.

La luz verde para trabajo técnico y pruebas está vigente. La aceptación final productiva queda reservada al humano. Las fuentes de `rust/sv_core` en producción no se han modificado: esta carpeta contiene el parche y la evidencia candidata para su revisión.

## Recuperación y reproducción

Cortes examinados: Lenguaje `cf366a339f66ba892a1ce783bb873d8c6f704801`; laboratorio `3d556f378df9306a73f3497e7ee39351640fec36`. [Cortes y 59 archivos Rust originales más 12 fixtures verificados](CORTES_Y_FUENTES.json). Se cotejaron los rectores ya leídos —AGENTS, Pilares, perfiles, transición y workflow— y se consultaron las superficies R1 y el contrato G2. No se altera doctrina, gramática o IR.

Las cápsulas [previa con sonda](PREVIO_FUENTES_CON_SONDA.json) y [candidata](CANDIDATA_FUENTES.json) contienen los bytes completos, rutas relativas y SHA-256. El previo sólo añade los tres testigos al código original; el parche muestra el cambio total frente al commit. Los conductores ejecutados quedan en `conductores/`, con comandos, resultados y huellas de binarios en esta carpeta.

Para reproducir: `python reproducir.py /ruta/rustc /ruta/nueva-de-trabajo`. Usar el compilador identificado en [COMPILADOR.json](COMPILADOR.json). El directorio debe ser nuevo. Se esperan dos fallos de la sonda previa y cero fallos de las 217 pruebas unitarias en cada modo de la candidata. Python sólo prepara archivos, lanza Rust y coteja las salidas. No interpreta SV ni produce autoridad.
