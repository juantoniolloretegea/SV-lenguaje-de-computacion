# Contrato candidato de artefacto consumido, revisión e invocación

**Versión 0.1 · S22 · BIS-02/C05 · 13 de septiembre de 2026 · Juan Antonio Lloret Egea y Watson**

## 1. Objeto y alcance

Toda atribución de uso debe identificar qué representación se utilizó, de qué instancia y revisión procede, quién la recibió y en qué operación e invocación. Una referencia disponible, una huella declarada o una afirmación de la IA no acreditan por sí mismas ese uso. El suceso es el hecho; su registro constituye una representación contrastable del hecho y conserva su alcance de evidencia.

Este contrato prepara BIS-03 y BIS-04. No añade primitivas, campos de IR, permisos ni una API productiva. C03 gobierna identidad y revisión; C04 gobierna paridad. C05 exige enlazar esas obligaciones con el objeto que atraviesa efectivamente la frontera del consumidor. Una coincidencia de bytes no valida autoridad, significado profesional ni comprensión de una imagen.

## 2. Escalones de evidencia

| Hecho que se pretende acreditar | Evidencia necesaria | Límite |
| --- | --- | --- |
| Artefacto disponible | Original recuperable e identidad exacta | No acredita su carga |
| Artefacto cargado | Bytes de la carga observada, ligados a su origen | No acredita entrega al consumidor |
| Entrega al consumidor | Bytes observados en la entrada final y enlace a invocación, operación, canal y destinatario | No acredita procesamiento remoto |
| Consumo por un componente instrumentado | Enlace entre entrada observada y operación real; control sensible a sustitución de la dependencia | No acredita comprensión o fidelidad de una explicación |
| Atribución de resultado | Resultado original ligado a la misma invocación y base | No transforma una explicación plausible en prueba de uso |

El estado de evidencia debe expresar el escalón alcanzado. No se usará una etiqueta general «consumido» para una mera disponibilidad o entrega. Un acuse local y uno emitido por un servicio remoto tienen productores y fronteras de confianza distintos. La captura de pantalla, la revisión humana y el procesamiento interno de un modelo requieren sus propias pruebas; no quedan acreditados por este banco.

## 3. Vínculo y frontera de confianza

La expectativa procede del contexto admitido del montaje y se fija antes de recibir propuestas. Conserva ámbito, invocación, operación, consumidor, canal, instancia, revisión y referencia exacta de representación. Esta última remite al convenio, transformación, estado y vínculo C03; no sustituye los contratos de constitución o LIG/0.1.

La observación procede del punto de captura declarado del montaje confiable. Debe permitir recuperar los bytes realmente pasados al componente. La propuesta no puede elegir otra expectativa ni aportar un booleano que se convierta en observación acreditada. Un formato serializado con todos esos campos sigue siendo una entrada no confiable hasta atravesar su admisión.

El objeto cargado debe conservarse inmutable hasta su uso, o volver a capturarse y comprobarse en la frontera final. Comprobar una ruta y abrirla después no asegura que contenga los mismos bytes. Si un adaptador transforma el objeto, deben conservarse entrada, salida y referencia de la transformación, con su estatuto de pérdida. La huella anterior a la transformación no se atribuye a la salida. C04 deberá resolver su paridad; C05 sólo conserva el enlace material.

Un cambio de revisión puede conservar exactamente los mismos bytes. Se comprueban ambos aspectos por separado. Una identidad de invocación es local a un ámbito: repetirla en otro ámbito no permite reutilizar la evidencia. Recuperar una observación original no se presenta como una ejecución nueva; reinicio, persistencia adversaria y prevención durable de repetición permanecen fuera del montaje local.

## 4. Resultado y orden del montaje candidato

El ensayo documental previsto distinguirá ausencia de observación, procedencia no admitida de ésta, contexto distinto, revisión distinta, representación distinta y bytes distintos. Sólo emitirá `ENTREGA_DOCUMENTAL_CONCORDANTE` si todos esos enlaces se comprueban. Este resultado experimental no significa consumo visual, permiso, Tri ni diagnóstico canónico.

Orden del ensayo: disponibilidad de observación; pertenencia al captor admitido; contexto completo; instancia y revisión; identidad de representación; igualdad exacta de bytes. Los negativos se construyen para alcanzar su guarda: fallar antes no prueba la guarda posterior. Ausencia de observación es `OBSERVACION_NO_DISPONIBLE`; un objeto presente vacío es una secuencia presente y se compara. Un fallo técnico no se convierte en U ni autoriza a inferir ausencia de efectos externos.

Para reclamar consumo material, además de comprobar entrega, el observador debe detectar un componente alterado que conserve los metadatos correctos pero use otra dependencia. Se reutilizará el principio ya ensayado en S14, sin atribuirle cobertura de imágenes o modelos. Las salidas esperadas se fijarán antes del ensayo y no se reconstruirán con el componente probado.

## 5. Banco previo y cierre todavía pendiente

El [banco previo](BANCO_PREVIO_v0_1.json) contiene dieciséis especificaciones: cuatro positivas y doce negativas. Sus objetos observados son estímulos para un futuro montaje controlado; no son capturas de una ejecución realizada. El registro independiente y las instrucciones de alteración pertenecen al conductor confiable, nunca al solicitante productivo.

Se conservan dos descriptores geométricos exactos de C04 como bytes sintéticos. Las referencias A/r1 y A/r2 usan los mismos bytes de identidad; A/r2 con presentación superior-horaria usa el segundo descriptor. Las revisiones son nombres opacos, sin transición temporal implícita. El banco separa así identidad, convenio y contenido sin inventar valores del dominio.

Estos archivos JSON no son imágenes. Los escenarios originales C05-P/N, relativos a imagen S2 y sustitución por imagen S1, conservan su estado pendiente. Su cierre exige materializar las imágenes con originales, identidad y revisión; instrumentar la entrada real; fijar presupuesto y canal; realizar controles positivos, negativos y de sensibilidad. La preparación documental no cuenta como ejecución Rust ni como consumo visual.

Las pruebas adicionales de sensibilidad deberán omitir por separado revisión, ámbito, identidad de invocación, contraste de bytes y enlace entre captura y uso. Cada alteración debe compilar para que su fallo funcional pueda contarse como detección. No se aceptará como detección funcional un mutante que sólo falle al compilar.

## 6. Relación con lo existente y decisión de sede

La [nota de antecedentes](FUENTES_Y_ALCANCE.md) conserva LIG/0.1, lectura vinculada /1 y S14 con sus alcances reales. No se declara ausente toda ligadura o consumo en Rust. La consulta documental y las candidatas de recepción no equivalen a protección productiva de una operación profesional ni a una entrada visual instrumentada.

BIS-03 determinará qué partes reutilizan código existente, qué guardas pertenecen al adaptador y si existe una insuficiencia nuclear demostrada. La prueba de recepción en Rust se rige también por el [criterio transversal](CRITERIO_DE_ACEPTACION_RUST_v1.md). Las funciones y métodos son realizaciones posibles; una macro no exime de las validaciones ni concede estatuto algebraico.

S22 continúa con BIS-C06: lectura técnica, inicialización y reevaluación bajo contratos diferenciados. C05 sigue pendiente de sede, realización y ejecución. El catálogo y la GUI conservan el orden autorizado.
