# S7 · Recepción de DeepSeek · Segundo intento

RETP-2026-172 · 2026-09-12T15:33:12Z · Responsable: Watson / W-S0.

## Resultado del cotejo fijado

**NO_CONFORME por falta de identidad literal en 26 de 32 citas de fuentes.** El archivo cumple el formato externo: un único bloque JSON sin texto adicional. Las doce decisiones, causas, contenido, llamadas, enunciados, reglas, fundamentos, consecuencias y límites coinciden con la referencia. El único tipo de error emitido es CITA_NO_EXACTA. No hay citas omitidas ni IDs ajenos.

El diagnóstico de las 26 diferencias demuestra que son exclusivamente supresiones de espacios de sangría al inicio de líneas de seis fuentes. F01 y F08 se conservan exactas. Las claves, su orden, valores y todas las cadenas dentro de los textos JSON citados se mantienen. La comprobación léxica conserva las cadenas y elimina sólo separadores externos para localizar diferencias; no se usa como sustituto del criterio de aceptación.

| Fuente | Bytes fijados | Bytes entregados | Espacios suprimidos por aparición |
| --- | ---: | ---: | ---: |
| F02 | 236 | 178 | 58 |
| F03 | 73 | 55 | 18 |
| F04 | 236 | 178 | 58 |
| F05 | 74 | 56 | 18 |
| F06 | 539 | 524 | 15 |
| F07 | 536 | 521 | 15 |


El contrato permite sangría libre en la estructura externa de la respuesta, pero exige recuperar los bytes exactos de cada fuente después de decodificar el campo `texto`. Los espacios alterados pertenecen a ese texto citado y cambian sus huellas. La regla R05 trata la presentación del cuerpo en los supuestos del banco; no sustituye el requisito de copia literal de las fuentes de la respuesta.

## Interpretación del defecto y acotación

Hay conservación del contenido documental y del enlace a las fuentes, pero no fidelidad literal completa. No se deduce ausencia total de trazabilidad, fabricación de fuentes, cambio de significado, intención u opacidad interna. Tampoco se oculta el incumplimiento de integridad byte a byte.

El defecto es localizable y técnicamente acotable mediante un transporte que conserve las fuentes canónicas como archivos o referencias verificadas y evite su reescritura. La ejecución actual no constituye ni valida ese mecanismo futuro. Sustituir aquí las citas por las originales sería reparar la entrega; no se hace ni se produce una respuesta corregida.

La aceptación provisional de Qwen comunicada por la dirección se registra en [su decisión específica](../qwen-intento-2/DECISION_DIRECCION.md), manteniendo el resultado original del verificador. No se presume una decisión humana equivalente sobre esta recepción de DeepSeek. Se conserva el criterio técnico sin introducir una nueva ronda de pruebas ni cambiar el banco fijado.

## Evidencia y procedencia

- [Respuesta íntegra recibida](RESPUESTA_ORIGINAL.txt): 29696 bytes; SHA-256 `062bf4d3edb29cfa8b4b7ebaa47e80b7aa58b71a44675b0beef7d4680299bfc4`.
- [Cotejo original](COTEJO_ORIGINAL.json): 26 rutas de error; salida de proceso 2.
- [Diagnóstico completo de fidelidad](DIAGNOSTICO_FIDELIDAD.json): por fuente y por caso, longitudes y huellas originales/recibidas.
- [Ejecución del observador](EJECUCION_OBSERVADOR.json), [script de evaluación y diagnóstico](evaluar.py), [recepción y límites](RECEPCION.json), [manifiesto](MANIFIESTO.json).

Se verificaron las huellas del banco, referencia, verificador, contrato y documento frente al compromiso previo S6. No se ha modificado el instrumento. El cotejo Python es de archivos, sin ejecutar ni redefinir semántica SV/Rust. Las duraciones capturadas corresponden al observador, no a DeepSeek.

Modelo DeepSeek y versión V3 son declaraciones del archivo, sin certificación del proveedor. Su declaración de exposición se circunscribe al material de su conversación; existe un primer intento atribuido a DeepSeek en S5, pero no se conoce si esta entrega procede de la misma sesión o instancia. No se presume ensayo ciego ni se imputa falsedad a esa declaración por falta de información sobre el contexto.

No aporta registros de ejecución ni métricas propias. No se certifican presencia o ausencia de herramientas, tiempos o esfuerzo. La recepción es el archivo aportado por el usuario; no consta una incidencia de transporte que explique el cambio de sangría. La atribución exacta de ese cambio a un componente anterior no se ha observado.

## Continuidad

S7 en ejecución: segundas entregas recibidas de Qwen y DeepSeek; pendientes Claude y Grok. Qwen aceptado provisionalmente por la dirección. Originales S5 y S7 intactos. Integración 1+3, seguimiento de causas y consolidación del catálogo al final permanecen vigentes.

Cortes: Lenguaje main `964712f6a36c131686e4e0f670090b41c6fa309a`; laboratorio lab/playground-sv-permanente `f453b02725ff468766f394de8698107756e88cd8`. [Piezas rectoras sin cambios desde su lectura](RECTORES.json). Se mantienen las ramas existentes, sin cambios en el núcleo ni el paquete público fijado.
