# Paso 6: causas de recepción, localización y reproducción

Fecha: 12/09/2026. Candidata RETP-157. Responsable de preparación: Watson. Autoridad de alcance y aceptación: Juan Antonio Lloret Egea.

## Corte y objeto

Lenguaje: `1eaee20a87cdff0e2a21c5297b8787a27b5a1102`. Laboratorio: `9ea6053eccd3fa50b37fdab9478e116794fa1aac`. Base material: cápsula candidata RETP-154, 231 pruebas unitarias por configuración. Rectoras cotejadas byte a byte con las lecturas íntegras: AGENTS, Pilares, perfiles/ensamblaje, transición y workflow V2. Se han leído también el plan de integración RETP-147, su rectificación RETP-148, el contrato diagnóstico RETP-109/110 y el catálogo v0.3.

Se ensaya una ampliación aditiva de las dos fronteras candidatas `PreparedGenesisReception` y `PreparedExactCheck`. Los métodos previos conservarán sus errores y decisiones. Los métodos detallados conservarán, en el emisor, la fase y el campo causante. Se localizan sus causas técnicas en ES y EN mediante tablas cerradas. Esas claves locales no se añaden a los 51 códigos canónicos del Lenguaje SV.

Este objeto resuelve una pérdida concreta del paso 6. La migración de todos los emisores del compilador y las compuertas DG01–DG14 siguen siendo obligaciones distintas; esta prueba no las sustituye.

## Hipótesis y fallo previo identificado

RETP-154 agrupa acto, emisor y versión vacíos en `Empty`, sus excesos de tamaño en `TooLarge` y las discordancias de emisor/versión en `DeclarationMismatch`. El comparador agrupa contrato, esperado y observado excesivos en `TooLarge`. La decisión puede ser correcta aunque esa información causal se haya perdido. Ninguna traducción posterior puede recuperarla con certeza.

El cambio hará observable esa distinción conservando la precedencia anterior: intento ya consumido; cualquier vacío, por orden acto/emisor/versión; cualquier exceso, en ese mismo orden; discordancia declarativa; discordancia del acto; resultado de T-0. Si discrepan emisor y versión se conservarán ambos. El comparador conserva su precedencia previa de vínculo, clase, contrato vacío y límites.

## Dos fases y criterios fijados antes de ejecutar

1. **Funcionamiento y trazabilidad:** recepción de un acto sintético bajo premisa de ensayo; custodia exacta; comparación de bytes presente/ausente/vacía; salida bilingüe; ejecución completa de la regresión heredada. La premisa sintética permanece identificada, sin atribuir facultades profesionales.
2. **Contrastes negativos del mismo recorrido:** campos vacíos o excesivos; emisor, versión o acto sustituidos; repetición tras consumo; rechazo T-0; vínculo ajeno; obligación nuclear indebidamente sometida a igualdad; omisión, negación alterada, instrucciones incrustadas y referencia seleccionada por el proponente. Cada resultado se compara con un esperado independiente y explícito. Los textos localizados nunca determinan la decisión.

La prueba de texto malicioso se limita a preservación de bytes y ausencia de incorporación a mensajes técnicos; no acredita aislamiento material, confidencialidad de todo el servicio ni todos los ataques A–L. La igualdad exacta es aquí el contrato explícito de ensayo. No se usa como verificador universal de medicina o de equivalencia semántica.

## Ejecución, presupuesto y conservación

- Nativo, Rust fijado por su salida `--version --verbose` y SHA-256. Configuraciones debug y release; una pasada completa por configuración. Tres repeticiones de la fase focal para comprobar estabilidad del receptor.
- Hasta 60 segundos por proceso y 2 MiB por flujo de salida, con terminación al superar el límite. Registro exacto de comandos, código de salida, stdout/stderr, tiempos UTC y duración monotónica. El límite de bytes del registro no constituye aislamiento del programa.
- Registrar CPU y RSS cuando el instrumental las exponga; el coste de inferencia de proveedores externos no observado se consigna como ausente, nunca como cero. Estas muestras no son la campaña pareada P5 ni permiten proclamar superioridad de rendimiento.
- Conservar fallos de construcción o pruebas con sus fuentes y resultados. Una corrección responde a una causa identificada; no se modifica un esperado para perseguir conformidad. Si falla la cualificación funcional final, se detiene la promoción y queda documentado el defecto.
- Mantener las tres pruebas negativas de API de RETP-154 y el cliente ordinario. No publicar funciones que instalen premisas o desprendan comprobaciones de su evidencia.

## Prueba externa

Se prepara un encargo público reproducible para Qwen, DeepSeek, Grok o Claude. Ninguno se declara ejecutado sin captura real de ese proveedor. Se conservarán sus archivos, herramientas/conectores usados, eventos, resultados y mediciones disponibles. La explicación de decisiones será un resumen atribuido; no se promete acceso al razonamiento interno oculto.

Es una reproducción del banco público, con exposición declarada. No abre la reserva P3, no reinicia sus cuotas y no autoriza rondas automáticas. Una divergencia frente al esperado constituye fallo de la ejecución y versión ensayadas; coincidir entre modelos no sustituye el esperado independiente.
