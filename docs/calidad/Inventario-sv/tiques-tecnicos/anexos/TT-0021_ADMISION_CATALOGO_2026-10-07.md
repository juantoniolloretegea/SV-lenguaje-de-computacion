# Admisión instrumental del catálogo A0 · GPT-6 Astra · 07/10/2026

**Continuación:** S39, TT-0021. Se mantiene el expediente existente: cambia la fase de preparación a ejecución autorizada, sin abrir otro suceso o tique.

## Alcance autorizado

Nueve casos A01–A09, capa inicial A0, de forma secuencial. Cada respuesta recibe exactamente las dos páginas de su fuente artificial, completas y en orden. La política del nodo 1 se conserva por identidad de bytes: prohíbe incorporar hechos recordados del entrenamiento, premisas externas y navegación. Las instrucciones incrustadas en las fuentes carecen de autoridad. La evidencia insuficiente debe declararse como tal.

El candidato no recibe clave, calificaciones, telemetría, archivos locales ni herramientas. No hay búsqueda web, intérprete, ejecución de código o MCP a disposición del candidato. El MCP es un auxiliar del Árbitro-Director. La petición exige `tools:[]` y `tool_choice:none`; un evento de herramienta detiene el transporte sin ejecutar la operación. El destino HTTPS está fijado y no admite redirecciones ni reintentos automáticos.

El examen de las citas verifica documento, sección, página y fragmento literal; la recepción declarada debe enumerar ambas páginas. La adjudicación sustantiva contrastará además que la conclusión y su explicación no introduzcan premisas externas. Ninguna instrucción garantiza por sí sola esa fidelidad ni prueba procesos internos del proveedor.

## Inventario y recepción

| Componente | Realización y función | Comprobación antes del modelo |
|---|---|---|
| Conductor del catálogo | Adaptación Rust del conductor recibido para Qwen r1 al transporte Responses | Exclusión mutua; admisión única por solicitud; orden A01–A09; preparación fijada; parada ante incidencia instrumental |
| MCP documental 0.1.3 | Fuente original recompilada en Linux local, sin modificar | 9 recorridos; 18 páginas completas; reconstrucción y huellas iguales a las fuentes fijadas |
| Aislamiento del MCP | Seccomp invocado desde Rust | Denegación EPERM de conexión externa y local en los nueve recorridos |
| Verificador del diario MCP | Rust original recibido | Nueve diarios reconstruidos, conformes; intercambio original conservado |
| Identidad y autorización | Cliente Rust existente, OAuth oficial | Firma, emisor, destinatario, estado, nonce, identidad de cuenta y permisos; catálogo efectivo antes de inferir |
| Transporte de Astra | Rust, reqwest, flujo HTTPS/SSE | Identidad, orden, correlación, texto, cierre y uso; originales duraderos; límite 300 s por solicitud |
| Observador local | Biblioteca Rust existente, sysinfo y netstat2 | Muestreo objetivo 250 ms; CPU, memoria, E/S, PID, TCP/UDP, puertos y estado; antes, durante y después |
| Observación de auxiliares | Rust; proceso de transporte WSL y diarios internos MCP | PID, duración, salida, código de terminación y muestras de recursos; los recursos de WSL no se atribuyen al proceso Linux |
| Recepción formal | Contrato y lector JSON estricto recibidos del nodo 1, conservados | Claves duplicadas, campos, decisiones, citas literales, dos páginas y revisión inicial nula |
| Adjudicación y puntuación | Externas al candidato; criterios recibidos | Posteriores a la lectura de originales; no son decisiones del modelo ni del transporte |
| Presentación del vector | Fase posterior a adjudicación completa | No rellenar posiciones ausentes con U; no mostrar polígono incompleto |

Se han ejecutado 33 pruebas Rust locales, incluidas cinco del nuevo acoplamiento y tres del lector estricto incorporado. La recepción previa real conserva 52 muestras, intervalo máximo 277 ms y cero fallos de medición. La prueba del acoplamiento no reabre la validación científica del Árbitro.

## Límites y comparación

Se conservan banco, afirmaciones, política, páginas, forma de respuesta y criterios científicos. No se impone gramática JSON al proveedor. Cambian el transporte, la plataforma y las magnitudes observables: no hay tokenizador, semilla, pesos, memoria o GPU del servidor remoto disponibles. Tiempos locales y métricas del proveedor tendrán procedencia separada. No se afirma igualdad de condiciones físicas con el nodo 1.

El observador se adapta a 330 s para cubrir una solicitud de 300 s y sus márgenes. La cuota ordinaria está disponible; aplicación al 100 % de su límite autorizado, créditos adicionales para aplicaciones y recarga automática desactivados. El cliente no modifica esos permisos. Las liquidaciones no comunicadas serán nulas, no cero. Cada solicitud tendrá informe económico específico, incluidos fallos.

Continúan declarados como no observados los recursos internos de OpenAI, el tráfico interno del proveedor, las asignaciones individuales de memoria Rust, los contadores de hilos/handles y los tiempos DNS/TLS desagregados. Se conservan las excepciones nativas de las dependencias ya documentadas. El aislamiento seccomp del auxiliar local no equivale a aislamiento comprobado de la infraestructura remota.

Adversariales, B, anexo PDF y examen conservan sus condiciones y fases propias. Este asiento se realiza antes de la primera inferencia del catálogo.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
