# Acta 007 · Recepción del acto y comprobación observada

19 de octubre de 2026 · S44 · TT-0028 · RETP-2026-326.

## 1. Objeto y dictamen

Se incorpora al núcleo Rust la realización de RETP-154, revisada sobre `733202084d21f5bffbd082498a4599bad309fc04`, con conservación de la corrección de pertenencia RETP-325. Se comprueban la recepción exacta de un acto previamente admitido, la comparación efectivamente ejecutada y la conservación conjunta del acto, la decisión y el ejercicio.

**Recepción técnica favorable en este alcance delimitado. La aceptación integrada de R2 y de la adenda OP-CYB-001 continúa pendiente.** La prueba presupone una admisión mediante el constructor exclusivo de pruebas existente. No acredita que exista ya un procedimiento institucional efectivo que produzca esa admisión. Tampoco convierte una comparación exacta de datos en autorización o conocimiento profesional.

Esta acta registra la comprobación realizada durante el desarrollo; no se presenta como auditoría externa independiente. La incorporación afecta a las fuentes del núcleo, sin modificación de los servicios instalados del Gestor o del Satélite.

## 2. Correspondencia con sucesos, tiques y actas

| Antecedente | Hecho recibido | Consecuencia para la continuación |
| --- | --- | --- |
| S13 y RETP-182 | Cotejo documental de doce obligaciones y criterios A–L; suficiencia integrada no acreditada | Una declaración de consulta o transducción no equivale a su realización. Se conservan CQ1–CQ6 y las dependencias de semántica e IR. |
| S16 y RETP-187 | Distinción B/E/K/L: facultades, secretos y canales, revisión efectiva, destinatario y mínimo necesario | El éxito de esta recepción no cierra esas cuatro obligaciones en una operación profesional. |
| S26 y recepción R02 | Contratos de identidad, memoria, dependencia, transacción, cobertura y recuperación; F01/F02 pendientes | Comprobar referencias autorizadas, coincidencia temporal, retención y recuperación efectiva antes de declarar continuidad persistente. |
| S38 | Necesidad registrada de aislamiento frente a una IA coexistente | No se declara confinamiento integral, defensa frente a un anfitrión comprometido ni aceptación de R3/R4. |
| S43, TT-0027 y Acta 006 | Norma de código limpio, aislamiento y privacidad; cierre delimitado del Gestor | Se conserva ese cierre y sus obligaciones continuadas. La realización nuclear se sigue mediante S44 y TT-0028, sin atribuirla retrospectivamente al Gestor. |
| Acta 001, §9, y transición rectora, fila 14 | Consolidación nuclear anterior a la reanudación material de R2 | Esta recepción es un avance previo; no altera el orden ni convierte R2-6 en una fase general R6. |

La arquitectura consultada define las fases generales R0–R4 y la descomposición de R2 en R2-0–R2-6. No se ha localizado en esas fuentes una ampliación general R5/R6. Se conserva esta distinción sin renombrar antecedentes ni inferir nuevas etapas a partir de numeraciones de otros documentos.

Fuentes de contraste: [registro de sucesos](../../Inventario-sv/sucesos/SUCESOS_SV.csv), [tiques técnicos](../../Inventario-sv/tiques-tecnicos/TIQUES_TECNICOS.csv), [Acta 001](ACTA_001_CONTINUIDAD_Y_RUMBO_2026_09_15.md), [Acta 006](ACTA_006_CODIGO_LIMPIO_AISLAMIENTO_PRIVACIDAD_2026_10_18.md) y [contrato RETP-154](../recepcion-gobernada-y-comprobacion-observada/CONTRATO_Y_FRONTERA.md). Sus identidades de entrada se conservan junto a los resultados de esta recepción.

## 3. Realización y correcciones

La recepción conserva el acto, el emisor declarado y su versión. Rechaza su sustitución y comprueba el tamaño antes de copiarlos. Al entrar en la constitución inicial consume el preparado, tanto si el plan se acepta como si se rechaza. No interpreta instrucciones contenidas en el documento, no autentica por sí sola al emisor ni crea una premisa de autoridad.

La continuidad recibida utiliza las reglas nucleares ya existentes para decidir, mediar y ejecutar. El acto permanece asociado a esa continuidad. La interfaz no permite sustituirla o extraerla separadamente. Se comprueban la conservación de la referencia causal y el rechazo de permisos y compromisos pertenecientes a otra continuidad, aunque sus contenidos sean iguales. Un fallo posterior al despacho conserva su causa y el estado indeterminado; no habilita la repetición del efecto.

El comprobador calcula la igualdad exacta; no recibe del solicitante un resultado de acreditación. Conserva por separado referencia, observación y contrato. La ausencia de observación no se confunde con una secuencia vacía ni con un valor de la terna del dominio. Sólo admite obligaciones específicas cuyo significado de igualdad haya sido establecido por la configuración competente; rechaza obligaciones nucleares y referencias pertenecientes a otra continuidad.

La revisión adversarial reprodujo dos exposiciones en los diagnósticos de la candidata: identificación del emisor y contenido de las evidencias. Los nuevos diagnósticos omiten esos datos; las consultas explícitas de evidencia conservan su función. También se evita duplicar el contrato y la referencia en cada observación mediante memoria compartida e inmutable. Al completarse una recepción, la evidencia pasa al resultado sin una segunda copia retenida por el preparado.

Los límites son de 65.536 bytes por campo. No son una cuota de memoria del proceso, de tamaño del plan, de número de observaciones o de retención. El error de constitución se devuelve con su causa; el registro histórico de rechazos sigue siendo una obligación del custodio. No se afirma persistencia donde sólo existe conservación en memoria.

## 4. Comprobación adversarial

| Comprobación | Resultado |
| --- | --- |
| Recepción, comprobación y recorrido causal | 25 pruebas específicas favorables; incluyen originales, sustituciones, ausencia, límites, repetición, premisa consumida y modificación posterior de los datos de entrada. |
| Regresión del conjunto en Linux | 404 pruebas favorables, sin fallos ni pruebas ignoradas. |
| Pruebas unitarias del núcleo en Windows | 246 favorables; repetidas con optimización, también favorables. |
| Ejecución en WASI | 245 favorables; se excluye expresamente la prueba PC06 de concurrencia entre hilos. Memoria lineal máxima: 256 MiB; plazo máximo: 30 segundos. No equivale a una prueba en navegador. |
| Confidencialidad de diagnósticos | Dos fallos reproducidos antes de la corrección; ambos controles favorables después. |
| Sensibilidad | Cinco alteraciones deliberadas detectadas mediante pruebas ejecutadas: omisión de coincidencia del acto, omisión del emisor o versión, aceptación de obligación nuclear, acreditación de evidencia ausente y confusión entre igualdad de contenido y pertenencia. Fuentes restituidas por SHA-256. |
| Frontera pública de Rust | Uso permitido compilado y ejecutado. Cinco intentos indebidos rechazados: preparar recepciones o comprobaciones desde fuera, extraer el resultado interno, sustituir la continuidad y fabricar la premisa de prueba. |
| Clippy | Sin errores; 15 diagnósticos distintos heredados. Desaparece uno anterior porque la función correspondiente pasa a utilizarse. No se declara ausencia de advertencias. |

Los resultados, huellas y condiciones de reproducción constan en [RESULTADOS.json](../recepcion-integrada-2026-10-19/recepcion-acto/RESULTADOS.json). Los totales de entornos repetidos no se suman como si fueran casos diferentes. La prueba original de la candidata y las pruebas finales conservan su alcance separado.

## 5. Responsabilidades y condiciones de aceptación restantes

Las responsabilidades siguientes se refieren a funciones del sistema y del procedimiento de aceptación; no presuponen una asignación personal ni una recepción independiente ya realizada.

| Necesidad | Función responsable | Comprobación exigida antes de su aceptación |
| --- | --- | --- |
| Admisión institucional efectiva | Admisión de autoridad y configuración competente | Procedencia reconocida, facultades y alcance del acto; rechazo de ausencia, sustitución y falta de vigencia; premisa obtenida mediante el procedimiento efectivo, no mediante el constructor de pruebas. |
| Producción de todas las comprobaciones obligatorias | Verificación nuclear y composición autorizada del dominio | Requisitos completos, cobertura y procedencia; ninguna igualdad aislada concede permiso ni sustituye verificadores de autoridad. El resultado observado no se extrae todavía para eludir estas condiciones. |
| Consulta y transducción | Desarrollo de semántica e IR bajo los contratos recibidos | Operaciones CQ1–CQ6 y transducción efectivamente realizadas, con pérdida o insuficiencia expresas; una representación nominal no se recibe como ejecución. |
| R2-1 | Persistencia y continuidad | AStore y PDep constituidos, cobertura, retención y recuperación verificables; distinguir dato ausente de consulta sin cobertura suficiente. |
| R2-2 y R2-3 | Continuidad y autoridad | Rechazo de retroceso, clonación y bifurcación no autorizados; revocación conservada durante la recuperación. Una copia de datos no restituye por sí sola autoridad. |
| R2-4 | Ejecución, inscripción y conciliación | Consumo e incertidumbre conservados después de una interrupción real; distinguir aceptación en memoria, escritura y compromiso durable. No repetir un efecto por falta de confirmación. |
| R2-5 y R2-6 | Recursos y aceptación integrada | Límites y tiempo donde estén constituidos; regresión y revisión adversarial integral de la realización persistente exacta. |
| Adenda B/E/K/L | Dominio, revisión competente y entrega autorizada | Facultad válida, aislamiento efectivo, coincidencia entre lo revisado y lo entregado, destinatario autorizado y contenido mínimo. |

La siguiente dependencia es la admisión efectiva y la producción completa de comprobaciones bajo los contratos recibidos. No se crea una aprobación genérica mediante un booleano ni se toma una firma, una huella o una etiqueta como sustituto de autoridad. S44 y TT-0028 permanecen abiertos. R2, la adenda completa y la operación profesional no se dan por terminados.

No se han activado producción, exámenes o cuentas externas, ni se ha enviado contenido a proveedores. Se conservan la terna vigente, el polígono único, la separación de traducción auxiliar y la autoridad de los contratos existentes.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
