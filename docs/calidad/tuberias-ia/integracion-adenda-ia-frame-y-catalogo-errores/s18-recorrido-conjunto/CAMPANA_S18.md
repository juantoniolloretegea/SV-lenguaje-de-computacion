# S18 — Fijación del recorrido conjunto

Realización de laboratorio del contrato S17-CONTRATO/1, recibido sin cambios. Estado de esta apertura: implementación escrita; compilación y ejecución funcional nuevas todavía no realizadas.

## Cortes y fuentes

PROCEDENCIA.json identifica los cortes main y lab/playground-sv-permanente posteriores a S17, los once archivos recibidos por Git blob y SHA-256 y las referencias exactas. RECTORES.json conserva las piezas rectoras ya leídas y cotejadas sin cambio. FUENTES_S2.json conserva G1 y las fuentes anteriores íntegras; se compila G1 original, no una variante de sensibilidad antigua.

## Adaptación delimitada

`codigo/recorrido.rs` conserva el consumo de la base por G1 y llama a comprobar_entrega antes de obtener EntregaLiteral. La Referencia se obtiene de Historia; la propuesta no puede construirla. La carga de base devuelve causa y bytes leídos cuando se rechaza.

Entrada posee el transporte recibido; Informe toma prestados esos bytes y la referencia. EntregaComprobada mantiene la presentación G1 validada y su referencia durante toda la escritura. Esta separación evita una estructura que posea bytes y simultáneamente se preste a sí misma, y conserva los límites de vida sin unsafe ni constructores públicos de prueba.

El protocolo SV17/1 comprueba contexto, cuerpo, solicitud y base. Los nombres de las causas son locales; la proyección ES/EN procede del enum usado por el registro. No se analiza Debug para decidir. Diagnósticos que no aparecen en el banco detienen la campaña; no se incorporan automáticamente al catálogo canónico.

El escritor del archivo exige la entrega tipada, crea exclusivamente, escribe y hace flush. La recuperación compara contra G1. I22 observa un escritor inyectado que conserva siete bytes y falla; no simula durabilidad ni todos los fallos de un sistema operativo. La atribución original por identidad y las comprobaciones de base/cuerpo se ejercen en sus etapas diferenciadas; no se presenta como una función S14 sin cambios.

## Banco y oráculos

24 casos I01–I24, íntegros en contrato-s17/BANCO_OBLIGACIONES.json. ESPERADO.tsv se deriva únicamente de sus etapas y causas. Los cuerpos positivos/negativos y bases son bytes anteriores a esta realización. El conductor Rust contrasta producción G1, parentesco, referencias, ausencia/presencia de entrega, etapas recorridas, bytes de transporte y archivos. Las etiquetas sólo se imprimen después de comprobar los predicados; un desacuerdo conserva la captura y termina con código 1.

Cada caso conserva referencias, solicitud, base, contexto, cuerpo, marco y traza de los episodios producidos; propuesta, recepción, causa, puertas y estado de archivo cuando existen. No producción, archivo ausente y archivo no observable se distinguen explícitamente. Los bytes del intento se conservan antes de su calificación.

La lectura de G1 no constituye una auditoría de una IA comercial. Los errores inesperados de producción detienen este ensayo; no se presume que esta envoltura haya cualificado todas las rutas de error internas de G1. Tampoco se acredita que la autoridad local del instalador sea una autoridad profesional.

## Presupuesto fijado

| Operación | Invocaciones |
| --- | ---: |
| Identificar compilador | 1 |
| G1, biblioteca sucesora y conductor, debug/release | 6 |
| 24 casos, tres ejecuciones por modo | 6 |
| Cuatro mutantes: biblioteca, conductor y ejecución | 12 |
| Cliente externo válido: compilar y ejecutar | 2 |
| Dos clientes externos inválidos: compilación rechazada E0451 | 2 |
| Total máximo | 29 |

Se prevén 144 observaciones normales. Las seis capturas completas deben coincidir byte a byte. Cada invocación tiene límite de 60 segundos de pared y CPU, 2 GiB de espacio virtual y 32 MiB por archivo. Son límites del supervisor de esta campaña, no requisitos del núcleo ni mediciones de consumo. No hay reintentos. Rust 1.98.0, binario SHA-256 `3690cc576ede140504698405d5d8fa3826aaadbe71699c6c4ed0a565d6f493e2`, Linux x86_64.

Sensibilidades: omitir contexto → I04; forzar consumo positivo → I14; omitir base citada → I09; admitir negativa → I12. No basta que el mutante no compile: su biblioteca y conductor deben compilar y el caso fijado debe detectarlo. Los clientes que fabrican Referencia o EntregaComprobada deben fallar por E0451; cualquier otro defecto no sustituye la evidencia de privacidad.

COMANDOS.json conservará argumentos, entorno fijado, salidas originales en base64, hashes, códigos, pared y CPU. El RSS de hijos es el máximo acumulado hasta cada paso; no se presentará como RSS individual. Si falla una compilación, un predicado, la identidad de capturas o el presupuesto, se conserva INTERRUPCION.json y se detiene sin reparar fuentes ni esperados.

## Puertas que permanecen abiertas

Esta realización sólo puede acreditar su banco. No abre P3/P4/P5/P6, no resuelve B/E/K/L profesionales ni modifica semántica/IR. El cierre 1+3 y el relevo al catálogo deberán valorar el resultado material y esas pérdidas. No se fija un orden de agentes ni se altera el retorno pendiente de inmunología.
