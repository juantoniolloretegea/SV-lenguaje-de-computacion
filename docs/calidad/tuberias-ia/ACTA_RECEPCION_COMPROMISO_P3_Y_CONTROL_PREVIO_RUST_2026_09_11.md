# Recepción del compromiso P3 y control previo de capacidad en Rust

**RETP-2026-130 · 11/09/2026.** Responsable: Watson; autoridad y custodio de la reserva: Juan Antonio Lloret Egea.

## Recepción

El titular entrega exclusivamente `COMPROMISO_P3.json` y confirma: «La recepción es recuperable y está separada , Watson.». Se registra esa declaración en el [recibo humano](ie004/recepcion-compromiso-p3/RECIBO_HUMANO_P3.json), separado del [compromiso original](ie004/recepcion-compromiso-p3/COMPROMISO_P3.json), que conserva intactos sus campos de custodia pendiente y reserva candidata. No existe contradicción temporal: el compromiso precede al recibo.

Comprobación directa de los bytes recibidos: **3.193 bytes; SHA-256 `d37246a019332327f54dd45c641944d90e3ae440163d76d0607c5235df63f9f5`**, coincidente con el comunicado. JSON válido en el lector documental estricto, sin claves duplicadas. El perfil referenciado IE004-ES-P2/2 tiene 31.867 bytes y SHA-256 `10c29934f2f8f6bfbb40546f7cf0056d4b53d720c432cacfb9d6b70abefe3d5f`, coincidente con el perfil público sin cambios. [Comprobación](ie004/recepcion-compromiso-p3/VERIFICACION_COMPROMISO.json).

Se acredita **recepción humana declarada, recuperable y separada**, conforme a RETP-128. No se afirma inspección directa del almacén humano, sus permisos o los bytes de los cuatro archivos reservados. Sus huellas sólo están comprometidas; se comprobarán contra los originales en sus hitos. No se solicita su traslado anticipado a Watson ni se publican. La autoría de Claude y su participación en el linaje de diseño conservan la limitación de independencia declarada.

Queda cumplida la condición de custodia que impedía iniciar el corrector. Esto no cualifica el perfil ni habilita aún la captura de Grok: antes siguen siendo necesarias la realización Rust, su congelación, el esquema V y los controles públicos de RETP-128. El oráculo se abrirá después del depósito de esa única captura, conforme al encargo.

## Primera comprobación, acotada al riesgo conocido R10

Se prepara [un instrumento Rust de reconocimiento y contabilidad](ie004/recepcion-compromiso-p3/sonda-rust/README.md) antes del corrector semántico completo. R10 advirtió que la búsqueda por barridos podría agotar el millón de intentos al acercarse a 128 tokens. Se contrasta primero esa condición necesaria para evitar desarrollar encima de una realización insuficiente.

El instrumento transcribe las 25 producciones de /2 y conserva el orden de barridos. Sólo reconoce derivabilidad; no calcula significados, permisos ni respuestas. Su tabla fusiona presencia sintáctica, no interpretaciones semánticas. La contabilidad concreta cobra visitas y particiones, incluidas fallidas y duplicadas; no se anuncia equivalencia cuantitativa con todas las realizaciones posibles de /2. Un fracaso refuta este recorrido y exige localizar su coste; no prueba imposibilidad de Rust, de la gramática o de un recorrido indexado equivalente.

Se fijan antes de ejecutar 72 controles públicos: 48 antecedentes, 18 contrastes y seis fronteras 127/128/129 tokens y 8.191/8.192/8.193 bytes. Las consultas largas repiten la misma restricción ACTUAL; su contexto público exigiría DATO/8.40. La sonda sólo comprueba que el reconocimiento necesario termina. Las primitivas incluyen suma/producto u64 comprobados, conversión a usize, reserva fallible y slices con rangos/fronteras UTF-8 válidos e inválidos. Ningún resultado sintáctico se cuenta como respuesta semántica correcta.

Se prevén Rust 1.98.0 nativo y WASI, debug y release, misma fuente y entradas; un máximo de 30 segundos por ejecución y los presupuestos de preparación/controles ya fijados. Node transcribe datos y conduce la ejecución; no implementa la gramática. Cero ejecuciones Python. Sin nueva enumeración ni consultas a modelos. Se conservará cualquier insuficiencia y no se elevarán cupos ni se ajustarán los esperados después de verla.

## Slices y continuidad

El matiz del titular queda recibido dentro de §2 de /2: propiedad del original, intervalos ligados a su identidad y préstamos temporales de lectura mediante acceso comprobado. No obliga a cambiar el perfil. Evitar copias no concede autoridad semántica ni acredita aislamiento entre procesos o proveedores. El registro conserva bytes e intervalos recuperables, nunca referencias a memoria ya liberada. [Referencia oficial de Rust](https://doc.rust-lang.org/book/ch04-03-slices.html), consultada el 11/09/2026.

Cortes de entrada: Lenguaje `d9e5f39e41876a4668cc114b36e1435d670e3c42`; laboratorio `875128eb0826bd2566affda665f8bd65c50cc7d0`. Se han leído completos AGENTS, Pilares, perfiles/ensamblaje, transición §§1–30 y arquitectura; se han leído Fase 004, workflow RETP-123, perfil /2, guía y encargo/contrato/controles RETP-128. Sus blobs rectores conservan las identidades documentadas en RETP-128. La nota científica RETP-129 permanece como antecedente preparatorio. No se cambia núcleo, DSL/IR, constitución de dominio, política, datos artificiales ni perfil /2.

**Estado al fijar este control:** CUSTODIA_ACREDITADA_POR_RECEPCION_HUMANA_DECLARADA; P3_PRECOMPROBACION_PUBLICA; CORRECTOR_COMPLETO_PENDIENTE; CAPTURA_NO_HABILITADA; AISLAMIENTO_NO_VERDE. Los resultados materiales se incorporarán como adenda identificada; esta preparación no los presume.
