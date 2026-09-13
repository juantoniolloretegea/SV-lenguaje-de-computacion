# Contrato candidato de identidad y revisión de la representación celular

**Versión 0.1 · S22 · BIS-02/C03 · 13 de septiembre de 2026 · Juan Antonio Lloret Egea y Watson**

## 1. Finalidad y estatuto

Una representación atribuida a una célula debe conservar la identidad de su constitución, de su instancia y de la revisión representada. La igualdad del vector, del tipo o de los bytes de una imagen no autoriza a intercambiar sus referentes. Este contrato concreta BIS-C03 y prepara la decisión de sede de BIS-03. No constituye una ampliación de la IR, una operación productiva ni una nueva clase canónica de Rust.

El suceso es el hecho; prosa, matemática e imagen son formas de representarlo. Registrar que una representación fue producida o utilizada identifica un hecho técnico; no prueba por sí solo que su contenido describa correctamente otro hecho. La vinculación de identidad es necesaria para la paridad, pero no demuestra la fidelidad geométrica ni la veracidad del contenido.

## 2. Distinciones necesarias

| Referente | Identificación exigida y alcance |
| --- | --- |
| Constitución | Referencia exacta de identidad, versión y huella, ligada al dominio competente. Una huella identifica bytes; la autoridad requiere su propio contrato. |
| Especificación celular | Objeto `CellSpec` dentro de un programa identificado. Expresa estructura; compartirlo no fusiona los nodos que lo utilizan. |
| Nodo arquitectónico | `NodeId`, realizado nominalmente mediante `CoupledSpec`, dentro de la arquitectura y del programa. No se sustituye por el nombre de la `CellSpec`. |
| Instancia de parámetro | Identidad y propietario conservados por `ParameterInstanceBinding`. No equivale automáticamente a una instancia celular completa. |
| Instancia celular | Identidad declarada bajo su constitución y ámbito. Su asociación con nodo, especificación y estado exige un enlace explícito; no se deduce de sus coordenadas. |
| Revisión del estado | Referencia exacta e inmutable dentro de la instancia. Un nuevo identificador de revisión puede conservar el mismo contenido. No acredita por sí solo una transición ni una relación temporal. |
| Representación | Referente anterior, clase de representación, convenio exacto y artefacto identificado. La existencia del vínculo, la producción de bytes y su consumo efectivo tienen evidencias distintas. |

El mínimo algebraico continúa siendo (9,3), sin convertirse en valor predeterminado. El banco utiliza dieciséis coordenadas, con b=4. No declara los tamaños de ningún dominio ni del soporte productivo.

## 3. Capacidad existente que se conserva: LIG/0.1

La [nota de alcance](ALCANCE_LIG_Y_RECTIFICACION_DE_LECTURA.md) documenta una precisión de la radiografía: el núcleo ya dispone de `sv_core::bindings::validate_bindings`. LIG/0.1 recibe `BindingContract` y una expectativa `BindingRequest`, verifica el programa exacto, referencias y bytes, propietario, pertenencia, usos ordenados, destinos por nodo y posición, compartición, alias e información lateral. Su resultado `ValidatedBindings` tiene construcción privada y acceso de lectura.

La constitución y la declaración de autoridad son artefactos con clase y referencia comprobadas. Esa comprobación no interpreta todo su contenido ni autentica a su emisor. La aceptación ordinaria de un programa SVP tampoco acredita por sí sola las ligaduras complementarias. Toda entrada que ofrezca LIG debe atravesar su validación, según su contrato vigente.

BIS-C03 deberá reutilizar esta capacidad donde corresponda. No se crea un segundo esquema de ligaduras por operación ni se reimplementa su huella mediante JSON: LIG/0.1 conserva su codificación binaria canónica y su expectativa exacta. El pequeño registro JSON del banco C03 es un oráculo experimental, no un sustituto serializado de `BindingContract`.

La revisión de una instancia celular y la atribución de su componente visual no son campos tipados específicos de LIG/0.1. Introducir sus nombres en bytes de `Provenance` no demuestra que el núcleo interprete o imponga esas obligaciones. La decisión de representación y sede permanece pendiente.

## 4. Condición de vinculación propuesta

Para una operación que solicite una representación, la expectativa debe proceder de su contexto constituido y conservarse fuera del contenido no confiable presentado. El montaje comprobará el vínculo completo: constitución exacta, programa, arquitectura y nodo cuando correspondan, instancia celular, revisión, estado y componente representados, convenio y artefacto. El contrato particular determina los campos aplicables; la ausencia de un campo obligatorio no se resuelve por inferencia.

La identidad de fuente ya preservada por la IR y `ProgramIdentity` deberá enlazarse con la proyección identificada cuando se incorpore la vía LIG. Las referencias son locales a su programa y ámbito; repetir un nombre en otro dominio o programa no conserva su referente. La aceptación del programa no acredita por sí sola que un registro externo constituya un dominio.

La representación de B no satisface una solicitud de A, aunque A y B compartan la `CellSpec`, la longitud, el vector y todos los bytes del artefacto. Una revisión anterior no satisface una expectativa posterior exacta por mera igualdad de contenido. La palabra «posterior» requiere una relación declarada; los identificadores del banco r1 y r2 no la implementan.

Las referencias de representación se conservan como instantáneas. Actualizar una instancia, una constitución o un convenio genera nuevas referencias conforme a su contrato; no reescribe las anteriores. La continuidad entre versiones, la admisión inicial y la reevaluación requieren sus propias reglas. No se inventa un predecesor para el primer estado ni se transforma un registro técnico en `TransitionData`.

## 5. Banco documental previo y fronteras de su prueba

El [banco](BANCO_PREVIO_v0_1.json) conserva dieciséis entradas literales, sus huellas y salidas esperadas. Su [registro independiente](REGISTRO_INDEPENDIENTE.json) declara A/r1, B/r1, A/r2 y A bajo una segunda versión sintética. Todas usan los mismos bytes matemáticos de dieciséis coordenadas. Así se impide que el contenido actúe como sustituto de la identidad.

El artefacto de prueba es una representación matemática JSON. No es una imagen, un renderizador ni una serialización canónica de la IR. La fuente SVP adjunta deriva del testigo existente de dos nodos que comparten `CellSpec`, ampliado a dieciséis posiciones. Se conserva como entrada aún no ensayada. No contiene una constitución `Domain` ni un contrato LIG completo y no puede presentarse como ensayo de su validación.

La sección `esperado` de cada entrada pertenece al oráculo comprometido del conductor futuro, no a una decisión libre del solicitante productivo. El conductor deberá obtenerla desde su propio banco y comparar con `presentado`; el montaje no debe confiar en una expectativa recibida junto al artefacto de una fuente no confiable. La sustitución simultánea del conductor, del banco y del registro queda fuera de la protección que este ensayo puede acreditar.

Orden propuesto para este montaje: comprobar campos obligatorios; resolver la identidad del registro; contrastar el registro presentado íntegro con la copia independiente; comparar su vínculo con la expectativa; comprobar los bytes entregados contra la huella; emitir el resultado experimental. Esta precedencia no modifica el catálogo ni la precedencia interna de LIG.

Las etiquetas `VINCULO_CONCORDANTE`, `REFERENCIA_INCOMPLETA`, `REFERENCIA_NO_REGISTRADA`, `REGISTRO_NO_CONCORDANTE`, `ATRIBUCION_NO_CONCORDANTE` y `ARTEFACTO_NO_CONCORDANTE` son oráculos del banco. No son códigos nuevos del Lenguaje, permisos ni resultados Tri. Los casos C03-10 y C03-11 deben ejecutarse en ese orden para comprobar que la selección de v2 no persiste indebidamente al solicitar v1.

## 6. Pruebas que todavía deberán realizarse

La comprobación documental no ejecuta los escenarios originales C03-P/N. Antes de su cierre deben decidirse sede y frontera, implementar únicamente lo justificado y contrastar la ligadura material. Cuando una prueba se dirija a una guarda interna de LIG, su expectativa exterior deberá concordar de forma controlada para alcanzar esa guarda; un rechazo de huella exterior no acredita rechazo de propietario, destino o alias.

El observador deberá detectar al menos: aceptar por igualdad de vector; omitir instancia; omitir revisión; resolver automáticamente otra versión; permitir sustituir registro; ignorar bytes distintos. Las pruebas positivas deberán conservar el vínculo completo, además de comprobar el resultado de aceptación. La realización ha de observar la salida real, sin recomponerla a partir de la expectativa.

La paridad de posiciones, símbolos, radios y convenio corresponde a C04. El artefacto efectivamente consumido corresponde a C05. Un resultado favorable del montaje de identidad no cierra ninguno de esos contratos. El frame arquitectónico vigente conserva sus referencias; el significado histórico de `cell_ref` como nodo se documenta y no se renombra silenciosamente.

## 7. Relevo y decisión de sede

BIS-03 recibirá tres hechos separados: ligaduras por operación LIG/0.1 existentes; referencias estructurales de nodos y estados existentes; enlace específico de instancia celular, revisión y representación pendiente de sede e imposición. Deberá determinar si una extensión tipada de la frontera complementaria basta o si una obligación exige modificar la IR o el núcleo. La ruta ofrecida al consumidor tendrá que imponer el control; una función opcional no acreditará esa imposición.

En Rust se distinguirán los datos del contrato, su validación y las operaciones admitidas. Funciones, métodos y, si existe necesidad demostrada, macros son formas de realización; ninguna concede por su nombre estatuto algebraico ni facultades al agente. Arrays o colecciones encapsuladas deben preservar dimensión e identidad. La política documental ES/EN se aplicará al código que se incorpore; no cambia los perfiles fuente de la DSL.

El siguiente objeto de BIS-02 es C04: contrato de paridad posicional y convenio gráfico, leído desde los fundamentos. S22 continúa en ejecución y S24 permanece pendiente. El catálogo y la GUI conservan la secuencia autorizada por el autor.
