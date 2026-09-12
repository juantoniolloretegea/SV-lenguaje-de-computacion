# Acta S15 — Causas preservadas en recepción y no admisión

**CONFORME dentro de SV-S15-CAUSAS-F/1.** 2026-09-12T19:13:04Z. Watson / W-S0. S15 / RETP-186. Catorce casos en seis ejecuciones (tres debug y tres release): 84 observaciones, 80 capturas idénticas por ejecución. Veintiuna invocaciones y tres controles de sensibilidad detectados en sus casos previstos. Sin reintentos ni cambios de fuentes/esperados después de fijar.

## Resultado material

El receptor acumula fragmentos, conserva los bytes y aplica el orden fijado: terminación de lectura, esquema del sobre, evento de protocolo y comprobación S2. La categoría, etapa, causa concreta, presencia de cuerpo y contador de cobertura llegan sin alteración al registro, su relectura y la presentación textual. Los motivos de negativa se conservan como bytes opacos.

F01 entrega exactamente el recibo anterior PERMISO_REVOCADO después de una comprobación de cobertura: entregar esa lectura documental no equivale a conceder permiso. F02/F10 registran negativa del evento de protocolo y nunca entregan cuerpo, incluso cuando el motivo es el cuerpo documental válido completo. F03/F04/F11/F13/F14 conservan las causas de esquema o límite; F05/F06/F12 conservan FaltaVigencia, Lectura(Identidad) y FaltaCaso del comprobador anterior.

F07/F08/F09 conservan TimedOut o ConnectionReset. F08 recibe los mismos bytes que F01, pero el error posterior impide decodificar/admitir; F09 conserva sus doce bytes parciales. Un EOF normal incompleto permanece Truncado. Ninguna causa se convirtió en U y sólo F01 materializó un cuerpo entregado. Las referencias previas permanecieron intactas.

Las bibliotecas G1, lote y cobertura S2 no se modificaron. El montaje prepara once posiciones del lote fijo por ejecución para obtener P3-11; los catorce casos son variaciones de recepción y no catorce llamadas a un modelo. La no admisión probada es documental, en S2; no se amplió la admisibilidad semántica nuclear.

## Sensibilidad

| Mutación fijada | Resultado observado |
| --- | --- |
| Proyectar no admisión como U | F05 falla con exit 1; el registro pierde la categoría requerida |
| Ignorar error tras recibir bytes | F08 falla con exit 1; aparece una entrega indebida |
| Admitir evento de negativa | F02 falla con exit 1; aparece un cuerpo indebido |

Se conservan los registros y cuerpos indebidos de los mutantes. Las compilaciones tuvieron éxito; no se contaron errores de compilación como sensibilidad. Las advertencias heredadas quedan en stdout/stderr completos; ninguna biblioteca fue editada para ocultarlas.

## Custodia, evidencia y medición

[Contrato](CONTRATO_S15.md), [fijación](FIJACION_PREVIA.json), [custodia previa](CUSTODIA_PREVIA.json), [procedencia](PROCEDENCIA.json), [rectores](RECTORES.json), [fuentes anteriores](FUENTES_S2.json), [receptor](codigo/receptor.rs), [conductor](codigo/contraste.rs), [esperados](codigo/ESPERADO.tsv), [observaciones](OBSERVACIONES.json), [581 capturas recuperables](EVIDENCIAS_S15.json), [comandos](COMANDOS.json), [binarios](BINARIOS.json), [entorno](ENTORNO.json), [mediciones](MEDICIONES.json) y [reproductor](reproducir.py).

Apertura verificada antes de compilar: público `eb789bb7c183ccfe2a68452e7aa487b159758d57`, laboratorio `947067b4dc904f6de12c108c549c25d0f9836de1`. Fijación SHA-256 `39f31f18c539b1968403f29a1e907eb3963357052325cee13a69d97ca6377337`. Todos sus archivos conservan los bytes publicados; esta acta sucede al README de apertura sin reescribirlo.

Pared de campaña: 7.186729 s; suma de pared de invocaciones: 7.079892 s; CPU usuario 7.530232 s; sistema 0.809091 s. Incluye compilación y sensibilidad; no preparación/publicación ni latencia de modelo. RSS individual, tokens y coste no disponibles.

## Alcance y siguiente paso

F recibe evidencia de distinción de causas **en el receptor sintético completo aquí definido**. [Matriz A–L sucesora](MATRIZ_COBERTURA_A_L_RESULTADO_S15.json) y [causas por etapa](CAUSAS_OBSERVADAS.json) conservan el alcance de cada hallazgo. El contraste incluye controles sobre semántica de entrega negativa y fallos después de bytes completos; no se limita a cambiar etiquetas.

No se acreditan proveedor real ni autenticidad de su negativa, SDK, red, pantalla profesional, durabilidad frente a reinicio o host hostil. Flujo, referencias, proceso y archivos son confiables; un Read que no retorna necesita supervisión externa. No se ensayan exhaustivamente Tipo/Bandera, todos los ErrorKind ni memoria agotada. No se pretende conocer intenciones o procesos internos de una IA ni garantizar inmunidad universal a inyección o alucinación. El sobre y las causas S15 son de laboratorio, sin promoción a gramática/IR o códigos canónicos.

**Siguiente:** revisar por sede las obligaciones aún pendientes, especialmente B/E/K/L, y concretar cuál dispone de productor y evidencia ejecutable conforme a S13. El catálogo recibe ahora causas separadas de recepción, protocolo y cobertura; no se declara cerrado antes de acabar las fronteras aplicables. S12 sigue vigente: agentes se valorará después de inmunología; la suficiencia general de CYB no queda decidida aquí. P3/P4/P5/P6 conservan sus puertas y reservas. No hay promoción nuclear por analogía.
