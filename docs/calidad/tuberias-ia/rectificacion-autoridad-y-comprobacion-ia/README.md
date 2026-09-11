# Autoridad humana y comprobación técnica de la IA

**RETP-2026-148 · 11/09/2026 · Rectificación y contraste acotado.**

## Corrección de la continuación

Juan Antonio precisa que concede autoridad el humano experto de cada dominio constituido; el agente delimita las actuaciones propias de ese dominio o de la parte que consume. Diseñar y probar cómo comprobar las reglas, evitar inferencias no admitidas y resistir manipulaciones es el trabajo técnico encargado a Watson.

**Se retira como bloqueo general la pregunta final de RETP-147 sobre un documento de autoridad.** Fue incorrecto trasladar al autor la solución del mecanismo que estamos construyendo. La aclaración permite continuar el diseño, la comprobación y los ensayos del alcance ya autorizado. No exige otra explicación del frame.

Se mantienen dos hechos técnicos: un recibo no es un permiso, y el enlace profesional a R1 aún necesita una recepción comprobable de sus referentes. Esa recepción pasa a ser una obligación técnica a resolver, no una condición para paralizar el contraste público de fidelidad. La existencia de un dominio o la declaración nominal de ser experto no conceden por sí solas facultades nuevas a una instancia de agente; el encargo humano y la cobertura definida deben quedar comprobablemente relacionados.

Esta rectificación sucede a la lectura bloqueante de [RETP-147](../integracion-adenda-ia-frame-y-catalogo-errores/README.md). Conserva sus fuentes y criterios; no convierte las pruebas preparadas A–L en pruebas ejecutadas ni elimina las puertas de reserva, seguridad material, coste o aceptación final.

## Qué debe comprobar el mecanismo

La comprobación técnica recibe la referencia y las reglas aprobadas para una operación. Para cada propuesta debe poder reconstruir qué datos se recibieron, qué transformación admitida se aplicó y qué se entregó. Una declaración de la IA de que actuó correctamente no constituye esa comprobación.

La realización deberá separar:

1. **Encargo humano y cobertura:** la operación solicitada debe estar definida y autorizada para el agente y el dominio recibidos. La entrada no puede ampliar esas facultades.
2. **Origen y versión:** conservar las fuentes originales y las identidades realmente utilizadas; un hash identifica bytes, no verdad ni autoridad.
3. **Contenido y cobertura:** comprobar las distinciones exigidas por la operación frente a su referencia, incluidas las ausencias relevantes, sujeto, negación, condición e incertidumbre. El comprobador necesita acceso a la evidencia exigida, no sólo a la selección del modelo.
4. **Transformación:** admitir únicamente transformaciones contempladas por el contrato aplicable, conservando qué se retuvo, omitió o sustituyó. El significado de un volcán o de un triángulo procede de su dominio; el núcleo exige el vínculo y las comprobaciones genéricas, sin aprender reglas particulares por analogía.
5. **Entrega:** ligar el objeto realmente presentado a la resolución comprobada y al perfil de presentación aplicable. La validez del transporte no autoriza a rotularlo como fiel.
6. **Fallo y recuperación:** conservar la causa en origen, detener la aceptación afectada y distinguir falta de evidencia, contradicción, rechazo de permiso y fallo técnico. Ninguno se convierte automáticamente en U ni permite completar lo que falta.

Este es el encargo de realización. No se declara implementado por enumerarlo. Rust permite imponer tipos, encapsulación y transiciones; esas propiedades deberán utilizar contratos de suficiencia y autoridad explícitos. No prueban por sí solas la verdad de datos ni invulnerabilidad general.

## Contraste ejecutado sobre el receptor Rust existente

Pregunta experimental: **¿basta aceptar el marco de transporte para acreditar fidelidad, incluso cuando alguien conserva coherencia de longitud y huella?**

Se ejecutó el binario nativo conservado de RETP-142. No hubo una compilación nueva: la ruta histórica del compilador ya no estaba disponible. Se fijó la huella del ejecutable antes de ejecutar y se comprobó que no cambió. El ensayo reproduce la superficie pública `validar` y la comprobación de propuestas `v`, con el original público A01 y V02. No abre modelos ni reserva; no usa datos clínicos reales.

| Caso | Entrada | Observación |
|---|---|---|
| C1 | Marco original | Acepta y devuelve exactamente el cuerpo original |
| C2 | Valor 8.40 sustituido por 9.40, sin actualizar huella | Rechaza: `CORRELACION_INVALIDA`, sin cuerpo |
| C3 | Mismo cambio de valor, con huella recalculada | Acepta y devuelve 9.40: integridad comprobada, fidelidad no comprobada |
| C4 | Se elimina «no» de «no acredita», recalculando longitud y huella | Acepta y devuelve la afirmación alterada: el transporte no detecta la pérdida de significado |
| C5 | Propuesta V02 original | Derivación sintáctica comprobada; autoridad sobre cuerpo A permanece falsa |
| C6 | V02 añade un campo de atribución de autoridad | Rechaza la propuesta por `ESQUEMA_INVALIDO`; el anexo mantiene autoridad falsa y la referencia al cuerpo original |

**C3 y C4 son contraejemplos a “transporte válido implica representación fiel”.** No son dos éxitos de seguridad ni defectos probados contra el contrato limitado del transporte. Tampoco demuestran un ataque que alcance G1, la interfaz profesional, R1 o un efecto: ese recorrido no se ejecutó en esta sonda. C6 prueba el rechazo de ese campo fuera de esquema, no toda posible inyección.

Las seis observaciones coinciden con la caracterización fijada antes del segundo intento. No se suman a P3 ni acreditan los doce casos A–L. La traza y la custodia G1 mantienen sus resultados históricos; no se inyectaron en ellas estos cuerpos alterados.

### Error propio conservado

El primer intento se detuvo en C2: el observador había supuesto el nombre `MARCO_INVALIDO`, inexistente en la enumeración examinada. La fuente previa `validar_marco` devuelve `FalloT::Correlacion` cuando difiere la huella y `fallo_t_nombre` lo presenta como `CORRELACION_INVALIDA`. Se corrigió únicamente ese esperado contra las fuentes, sin tocar entrada o binario. Se conservan script inicial, plan y salidas de ambos intentos. La corrección no es una reparación de Rust ni se oculta como ensayo correcto desde el inicio.

## Consecuencia para el siguiente cambio

El nuevo trabajo técnico debe localizar la aceptación de contenido y la entrega efectiva, y demostrar su correspondencia con una resolución y referencia admitidas. No debe añadir otra comprobación de hash como sustituto de fidelidad ni permitir que el modelo proporcione a la vez la respuesta y su único criterio de aceptación.

Para el recorrido público existente, se reutiliza la resolución determinista A y la custodia G1; la propuesta auxiliar continúa subordinada y no cambia el cuerpo A. Antes de extender la presentación se fijará qué representación consume ese cuerpo, qué transformaciones permite y dónde se comprueba su correspondencia. La comprobación visual profesional conserva su fase: no se abre ahora una interfaz ni se introduce una semántica médica en el núcleo.

Los errores detectados alimentan el catálogo en el punto donde nacen. La localización ES/EN debe conservar causa, procedencia y decisión. El diseño y las pruebas continúan bajo la autorización ya recibida; se acudirá al humano si aparece una decisión de significado o alcance que realmente le corresponda, o para la aceptación final.

## Fuentes, reproducción y límites

Cortes de entrada: Lenguaje `132c323a6241f1917c5e8416d21eb8a66923cce6`; laboratorio `d95b33a208327e66a36023ffba9d1ee4469c727b`. AGENTS y las rectoras completas leídas en el incremento anterior conservan sus identidades; [FUENTES.json](FUENTES.json) identifica las piezas cotejadas. Se releen las entradas de A/V, la validación de marco, la propuesta V y las guardas G1 directamente pertinentes.

[EVIDENCIA.json](EVIDENCIA.json) conserva en base64 las entradas/salidas, planes, resultados, rectificación y primer script, con tamaño y SHA-256. [ejecutar.py](ejecutar.py) es un conductor de pruebas externo; invoca el binario Rust y no compila ni interpreta el Lenguaje SV. El binario no se incluye: su huella y procedencia declarada están en el plan. No se afirma autenticación independiente del anfitrión ni identidad de un binario recompilado.

```text
python ejecutar.py /ruta/receptor-rust /ruta/fixtures-g1 /ruta/salida-nueva
```

La entrada requiere las fixtures públicas `cuerpo-a01.bin`, `marco-a01.bin`, `solicitud-a01.bin`, `propuesta-v02.bin` y `anexo-v02.bin` del corte indicado. Se conservan las cuotas del receptor; seis procesos por intento completo, límite externo de 60 s por proceso y 1 MiB para la salida agregada de la sonda. El primer intento consumió dos procesos; el segundo seis. Ese límite de salida es una comprobación del conductor tras la recepción, no una imposición material al productor. Los tiempos son observaciones del ensayo, no una acreditación de viabilidad P5.

**Estado: bloqueo general rectificado; caracterización pública ejecutada; comprobación integral de fidelidad y seguridad pendiente.**
