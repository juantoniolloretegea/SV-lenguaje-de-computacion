# Continuidad de trayectoria y representación de sus instantáneas

**BIS-C06 · Cuestión delimitada para BIS-03 · RETP-2026-209**

La IR vigente representa una trayectoria como pares `(frame, transición_saliente_opcional)`. Para una trayectoria de un único frame, el par terminal carece de transición. En la instantánea extendida, ese frame puede tener un enlace saliente a la siguiente evaluación, mientras el nuevo último frame queda sin transición.

El [testigo estructural](TESTIGO_INSTANTANEAS.json) conserva esas dos formas abstractas. Sus F1/F2/nu1 son referencias simbólicas: no constituyen frames ni un operador ejecutados. Es un testigo de representación, no un corpus aceptado por el compilador.

El frame F1 puede conservar sus bytes y, sin embargo, cambiar el par que lo contiene. Por eso deben distinguirse la inmutabilidad del frame, la conservación de una instantánea anterior y el carácter append-only del registro material. La bandera constitutiva `trajectory_is_append_only()` y la aceptación aislada de cada estructura no observan una historia entre dos ejecuciones.

BIS-03 deberá determinar el contrato de extensión y custodia: identidad y recuperación del original, enlace verificable de extensión, integridad de frames y datos anteriores, y correspondencia con la proyección IR. El almacenamiento de nuevas instantáneas o un registro externo de adiciones son alternativas que requieren justificación; este testigo no selecciona ninguna ni modifica J4.2.

No se confunde una actualización permitida de una proyección derivada con permiso para sobrescribir evidencia histórica. Tampoco se declara imposible una realización conforme: se exige que la realización escogida demuestre esa correspondencia. La necesidad de cambiar IR sólo se elevará si existe una obligación que la representación vigente no pueda satisfacer, con su contraejemplo.

Otra frontera relacionada es la reevaluación sin cambio estructural: el Documento III §6.2 la admite bajo régimen auditable pleno, mientras la recepción vigente exige `induced_parameters` no vacío. No se resuelve la correspondencia introduciendo asignaciones ficticias ni declarando toda repetición un cambio. El contrato concreto deberá justificar dato suficiente, operador y marca de régimen antes de ejecutar esa vía.
