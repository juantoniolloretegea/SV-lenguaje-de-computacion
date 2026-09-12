# S15 — Causas de recepción, protocolo y no admisión

Contrato **SV-S15-CAUSAS-F/1**, previo a compilación. Watson / W-S0. Integración 1+3; continuación autorizada tras S14 / RETP-184. Fijación de cortes y rectores en PROCEDENCIA.json y RECTORES.json.

## Objeto, reutilización y frontera

Conservar categorías, causas concretas, etapas, bytes recibidos y paso efectivo por cobertura en un recorrido: flujo sintético → captura acotada → decodificación → cobertura S2 → informe → archivo recuperado → presentación textual. Nunca convertir fallo técnico en U. Sólo un resultado de cobertura válido entrega cuerpo; una lectura entregada de PERMISO_REVOCADO sigue siendo un recibo negativo, no concede permiso y no es negativa del proveedor.

Se reutilizan sin editar las bibliotecas G1, lote y cobertura desde FUENTES_S2.json. El oráculo de cuerpo completo es el negativo previo S11/S14, no uno calculado por el nuevo receptor. El lote fijo necesita preparar sus primeras once posiciones para llegar a P3-11; no son llamadas a IA. Caso y vigencia requeridos e identidad se obtienen de Referencia antes de las propuestas. La no admisión ensayada es **no admisión de lectura documental por cobertura S2**, no una nueva decisión semántica de dominio o del núcleo.

El único añadido es un sobre binario y receptor **de laboratorio**. No existe afirmación de que este formato sea una API, una nueva IR, un perfil de agente o un protocolo profesional. El constructor de estímulos y el decodificador son piezas distintas. Las etiquetas esperadas se fijan en ESPERADO.tsv y las causas reales provienen del lector, decodificador o comprobador existente; no se suministra al receptor una etiqueta de resultado esperado.

## Sobre exacto y precedencia

Cabecera de cinco bytes ASCII SV15 más 0x01. Tipo ASCII N o R. Enteros big-endian. N lleva un campo (longitud u32 y bytes opacos de motivo). R lleva identidad (dos u64), un campo de cuerpo, y caso y vigencia opcionales: bandera 0 ausente, 1 seguida de campo. Cualquier otro tipo/bandera, campo truncado o sobrante se rechaza. El perfil fija VERSION y OPERACION de la lectura existente; no toma instrucciones de los bytes de motivo o cuerpo.

Primero se conserva el flujo hasta EOF, un error explícito o el byte 16385. Máximo conservado 16385 bytes; 16384 es el límite admisible del sobre. Se procesa sólo después de EOF sin error ni exceso. Un error de Read, incluso después de un sobre completo, conserva su ErrorKind y todos los bytes obtenidos y no llega a cobertura. EOF normal con sobre incompleto es Truncado, no Comunicación. No se reintenta Interrupted ni otra causa en este perfil: prevalece el primer error de lectura. Exceso es Limite en recepción; no se vacía ni se reinterpreta como U.

N es una **negativa declarada por el evento sintético**, cuya carga no obtiene autoridad ni se infiere lingüísticamente. R pasa al comprobador S2, que preserva FalloCobertura y sus causas anidadas. Informe tiene campos privados, cuerpo sólo en entrega comprobada, motivo sólo en negativa, y proyección textual con categoría, etapa, causa, presencia de cuerpo y contador real de llamadas a cobertura. Ese contador no es llamadas de política ni llamadas a una IA. El diagnóstico se archiva, relee y presenta íntegro bajo cabecera SV-S15-DIAGNOSTICO/1.

## Banco previo

| Caso | Estímulo | Resultado exigido |
| --- | --- | --- |
| F01 | Sobre R completo, cuerpo negativo previo y cobertura exacta | ENTREGADA; una comprobación, cuerpo íntegro PERMISO_REVOCADO |
| F02 | Sobre N con motivo Servicio no disponible | NEGATIVA_PROVEEDOR, motivo íntegro, cero cobertura/cuerpo |
| F03 | Cabecera alterada | ESQUEMA_INVALIDO / Cabecera |
| F04 | EOF normal tras quitar último byte del sobre R | ESQUEMA_INVALIDO / Truncado |
| F05 | R sin vigencia | NO_ADMISION / FaltaVigencia; una comprobación |
| F06 | R con ordinal distinto | NO_ADMISION / Lectura(Identidad); una comprobación |
| F07 | Error TimedOut antes del primer byte | COMUNICACION / TimedOut; cero cobertura |
| F08 | Sobre R completo seguido de ConnectionReset en vez de EOF | COMUNICACION / ConnectionReset; todos los bytes, cero cobertura |
| F09 | Doce bytes de R seguidos de ConnectionReset | COMUNICACION / ConnectionReset; prefijo íntegro, cero cobertura |
| F10 | N cuyo motivo contiene el cuerpo válido completo | NEGATIVA_PROVEEDOR; nunca entregar ese cuerpo |
| F11 | R con espacio sobrante | ESQUEMA_INVALIDO / Sobrante |
| F12 | R sin caso, con vigencia | NO_ADMISION / FaltaCaso; una comprobación |
| F13 | 16385 espacios | ESQUEMA_INVALIDO / Limite en recepción; conservar 16385 bytes |
| F14 | EOF normal sin bytes | ESQUEMA_INVALIDO / Truncado |

En cada caso: entrada conservada idéntica al estímulo recibido, diagnóstico completo idéntico al esperado preestablecido, archivo recuperado idéntico, presentación textual íntegra; cuerpo sólo F01, motivo sólo F02/F10. Se retiene la referencia antes y después. El flujo entrega fragmentos de como máximo siete bytes para ejercitar la acumulación; fallos se inyectan mediante Read. No es una conexión de red real.

## Sensibilidad y presupuesto cerrado

Seis ejecuciones normales: tres debug, tres release; 14 casos por ejecución, 84 observaciones. Tres mutaciones cfg de la implementación nueva, sin modificar las bibliotecas: (1) proyectar no admisión como U, debe fallar F05; (2) ignorar error de lectura tras recibir bytes, debe fallar F08; (3) admitir negativa como entrega, debe fallar F02. Cada mutante debe compilar y terminar con exit 1 en el caso exacto, conservando la aceptación o pérdida observada; error de compilación no acredita sensibilidad.

Máximo **21 invocaciones**: una identificación de compilador, ocho compilaciones normales, seis ejecuciones normales, tres compilaciones de mutantes y tres ejecuciones de sensibilidad. Rust 1.98.0 Linux x86_64 y binario SHA-256 3690cc576ede140504698405d5d8fa3826aaadbe71699c6c4ed0a565d6f493e2. Tiempo máximo por invocación 60 s. Sin reintentos, sin cambiar esperados o fuentes una vez fijados. Un fallo inesperado detiene y conserva la campaña; se documenta antes de decidir una campaña distinta. Capturas normales deben ser idénticas byte a byte en los seis recorridos.

## Límites y continuidad

Host, referencias, conductor y archivos de diagnóstico confiables. No se acredita origen/autenticación del proveedor, acceso a procesos internos, red real, SDK, host bloqueado, pantalla profesional, recuperación tras reinicio ni ausencia universal de inyección/alucinación. Read que nunca retorna requiere supervisor externo. El límite de memoria de entrada no garantiza resiliencia a OOM del host. No se ensayan exhaustivamente Tipo/Bandera ni todos los ErrorKind. Las causas son variantes de laboratorio, no nuevos códigos canónicos.

La misma captura atraviesa las etapas de este receptor; no se declara construida toda la tubería profesional ni revalidado todo A–L. S13 sigue gobernando suficiencia de gramática0.2/IR0.3; S12 mantiene agentes por valorar después de inmunología. P3/P4/P5/P6 conservan sus reservas. El catálogo recibe causas con sede y evidencia, sin cierre universal ni expansión nuclear por analogía.
