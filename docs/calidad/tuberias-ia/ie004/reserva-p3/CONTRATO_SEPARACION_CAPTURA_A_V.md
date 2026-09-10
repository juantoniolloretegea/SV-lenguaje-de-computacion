# Captura P3: solicitud confiable A y propuesta externa V

**IE004-P3-TRANSPORTE/1 · RETP-2026-128 · 10/09/2026.** Precisión documental de R11 antes de la reserva; realización y pruebas pendientes. Conserva IE004-ES-P2/2 y su guía de recursos. No declara aislado ningún proceso ni concede facultades por el nombre de un archivo.

## 1. Original confiable y entrada auxiliar

`SOLICITUDES_P3.json` tiene sólo las claves `version` y `casos`. `version` es `IE004-P3-A/1`. `casos` contiene exactamente 24 objetos ordenados, cada uno con sólo `id`, `pregunta` y `contexto`:

- `id`: uno de `P3-01`…`P3-24`, cada uno una sola vez y en ese orden;
- `pregunta`: cadena original UTF-8; límites de 8 192 bytes y 128 tokens del perfil, sin reparación;
- `contexto`: exactamente `operacion`, `objeto`, `parametro`, `momento`, `campo`, con los valores cerrados del contrato IE-004 o null. Sin valores por defecto aportados por el participante.

`ENTRADAS_AUXILIARES_P3.json` tiene sólo `version: IE004-P3-AUX/1` y `casos`, con los mismos 24 identificadores en el mismo orden. Cada objeto tiene únicamente `id` y `nota_externa`, una cadena con el límite vigente del contrato IE-004. Se conserva fuera de A. El participante puede recibir preguntas y notas; recibir una nota no le concede autoridad.

No se aceptan claves duplicadas, adicionales, identificadores repetidos, decodificación inválida ni normalización silenciosa. Las huellas se comprueban sobre bytes antes de cualquier conversión. El corpus confiable procede del custodio y del compromiso recibido antes del corrector. Una copia incluida por Grok no reemplaza esos originales. Base, política, perfil y facultades efectivas se seleccionan desde el montaje confiable fijado, nunca desde campos elegidos por V.

El lote es un formato de custodia. El conductor toma del original confiable una solicitud A cada vez; la interfaz Rust de admisión recibe esa pregunta/contexto y las identidades autorizadas. Los tiempos de preparación del lote se registran aparte y el recorrido completo de solicitud permanece medido según la guía. No se oculta coste obligatorio de admisión dentro de una preparación previa. Los límites de almacenamiento/transporte del lote y su comprobación se congelarán antes de la captura, sin cambiar los límites por solicitud ni permitir cargas no acotadas.

## 2. Depósito de propuestas

Cada propuesta V será un archivo separado bajo la ruta de entrega designada, con nombre determinado por el identificador confiable: `propuestas/P3-01.json`…`propuestas/P3-24.json`. Estos nombres no son paths elegidos desde el contenido del modelo. No se siguen referencias a rutas, URLs o archivos adicionales emitidas por el participante.

El sobre V tendrá `version: IE004-P3-V/1`, `id` y `solicitudes_sha256` para correlación con el lote original, además del certificado cuya representación se congelará con el corrector antes del encargo de captura. La representación interna deberá permitir comprobar reglas/versiones, alternativas, hijos e intervalos originales conforme a P2. Esta precisión fija la separación de entradas, no se presenta como esquema completo de certificados ya implementado. La reserva puede fijar significado y mutaciones paramétricas sin conocer sus codificaciones posteriores.

Ningún dato del sobre V concede permiso, cambia contexto o sustituye el original. Error de correlación, certificado inválido, archivo ausente, duplicado, excesivo o tardío se registra como condición V. Una propuesta completa, incluida su envoltura, queda dentro de los 256 KiB y demás cuotas V de P2. La recepción debe limitar bytes antes de acumularlos y no continuar leyendo un exceso para localizar A.

El conductor determina cuáles son los 24 archivos esperados desde A y puede entregar el cuerpo sin abrir ninguno. Los archivos ajenos al conjunto esperado no se cargan. Las entregas de captura quedan inmutables por commit y no se adjudican a Grok las mutaciones de prueba introducidas después por el comprobador.

## 3. Orden de ejecución obligatorio

1. Verificar y admitir el original A desde custodia confiable, con sus versiones y permisos. Reservar A y su registro base.
2. Resolver A con la realización Rust y entregar el cuerpo canónico completo. A no espera la invocación del modelo, la presencia de V ni su decodificación.
3. Comprobar V sobre la instantánea de A bajo cuenta, almacenamiento, fallos y plazo V separados. La comprobación se anexa y no reabre la respuesta.

Se prohíbe reutilizar como entrada de /2 la trama del receptor antecedente que mezcla pregunta, contexto, permiso, diagnóstico, ruta y apoyos externos. Dividir un objeto ya deserializado no satisface esta separación. Empaquetar archivos juntos para descargarlos tampoco acredita aislamiento: el camino hacia A debe poder funcionar sin descargar, extraer ni procesar el paquete de V.

Los resultados canónicos y los anexos V se guardan separadamente y se correlacionan por identidades comprobadas. Variar V no altera la identidad del original, la resolución, las cuotas o el plazo A. La identidad de los cuerpos puede contrastarse byte a byte; tiempos y estados V no se incorporan a ese cuerpo para fingir determinismo de una nueva llamada al modelo.

## 4. Condiciones de cualificación

Antes de la captura de Grok se congelarán la interfaz concreta y el esquema interno V, el montaje confiable, el decodificador estricto y sus límites, los artefactos Rust nativo/WASI y los controles públicos. La seguridad no se deriva de que los archivos se llamen A/V ni de que compartan un lenguaje.

Los controles deben demostrar que A sirve la misma consulta legítima con V ausente, mínimo válido, al máximo, inválido, excesivo, agotado, mal correlacionado y tardío. Deben comprobar lectura/decodificación efectiva y contadores, no únicamente comparar respuestas finales. Si V obliga a leer antes de admitir A, o puede consumir sus cuotas/plazo o bloquear su entrega, la realización incumple /2 aunque una prueba ordinaria devuelva el dato correcto.

Las cotas de memoria, cancelación y presión concurrente requieren su imposición material en P4; los costes, P5. No hay seguridad universal ni resistencia al host comprometido acreditadas por este contrato. El resultado de un timeout o error técnico no es `Tri.U`.
