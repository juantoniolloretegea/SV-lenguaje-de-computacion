# S26 · Inventario material R01

**Corte inspeccionado:** Lenguaje `8028193c085a7f1a9e063728569d62f380e1b1de`;
laboratorio `c017c1c472ed3e1e85b8b1efe8f5df6e6b126879`.
Continuación de RETP-222. La inspección se limita a los recorridos y archivos
enumerados; no declara auditado todo el repositorio ni todas las plataformas.

## Residencia, movimiento y autoridad

| Recorrido / objeto | Dónde reside y cómo se mueve | Qué puede cambiarlo | Qué significa su aceptación | Sede y brecha material |
| --- | --- | --- | --- | --- |
| CLI `sv-native`: archivo SVP → `String` | `fs::read_to_string` lee la ruta en un buffer propiedad del proceso; el parser recibe `&str`. | El escritor autorizado por el sistema de archivos puede modificar la fuente. La lectura no constituye por sí sola una instantánea atómica frente a escritores concurrentes. | Lectura UTF-8 correcta; todavía no admisión SVP ni commit durable. | `rust/sv_native/src/main.rs`; F01/F05/F06. No se ofrece transacción de fuente y dependencias. |
| Parser → `IrProgram` | Tokenización y análisis en memoria; IR posee `String`, `Vec<IrObject>` y `Vec<IrOperation>`. Consultas públicas devuelven referencias de lectura. | Construcción interna; otra compilación produce otro objeto. Reasignar el objeto seleccionado es distinto de mutar el anterior. | Admisión estructural y relaciones verificadas por el frontend; serialización posterior a stdout. | `frontend.rs`, `ir.rs`; F01/F02/F04. No es registro histórico persistente; stdout no acredita recepción ni durabilidad. |
| Candidato → `Frame` | `Frame::from_candidate` valida referencias y crea un objeto de campos privados en memoria; contiene nombres de referencias, no direcciones persistentes. | Constructores internos antes de aceptación; la API pública inspeccionada no expone mutadores del Frame. | Cierre y pertenencia en el contexto de validación. | `frame.rs`; F02/F05. El frame emitido como `IrObjectKind::Frame` y el tipo `Frame` deben distinguirse; no se presupone que toda ruta IR materialice ese tipo público. |
| LIG → `ValidatedBindings` | El resultado conserva contrato, expectativa y programa propios; registro auxiliar `BTreeMap` presta referencias a artefactos durante validación. | Contrato candidato antes de validarlo; futuras llamadas admiten objetos distintos con expectativas distintas. | Identidades y relaciones exactas dentro de la llamada. | `bindings.rs`; F01/F02/F07. Ese árbol no es BD ni autoridad persistente; rechazo de duplicados local no implica unicidad entre procesos. |
| Custodia experimental → `TrustedContext` / `TrustedRegistry` | El conductor carga archivos separados; los constructores analizan JSON y cotejan referencias/huellas internas. | Quien controle conductor, archivos de custodia o sus argumentos. El nombre `Trusted` no autentica al emisor. | Esquema y consistencia del material recibido bajo la custodia presupuesta del laboratorio. | Bis `lib.rs` / `main.rs`; F01/F03/F08. Sustituir coherentemente toda la base confiable queda fuera de la protección local. |
| Canales → `ReceivedBytes` → `AdmittedDelivery` | Lectura secuencial a `Vec<u8>` privados con cuotas; `admit` consume los buffers, compila, coteja y mueve `geometry` a `descriptor`; copia valores Tri a array fijo 16/25/49. | Los canales antes/durante lectura; tras copia, cambiar el archivo no reescribe automáticamente ese buffer. Una captura posterior es otro objeto. | Concordancia documental de las piezas respecto del contexto fijado. | Bis `ReceivedBytes::read`, `admit`; F01/F05/F12. No atomicidad de varios archivos, BD o durabilidad. Las cuotas son de este montaje. |
| Entrega → `Capture` → `certify` | `receiver` copia bytes a un `Vec` en el mismo proceso. `Capture` tiene campos públicos; `certify` coteja captor, contexto y bytes. | Conductor/código con acceso mutable a Capture; infraestructura fuera de la API también puede intervenir. | Correspondencia con el descriptor en el alcance del captor declarado. | Bis `Capture`, `certify`, conductor; F09/F10/F12. No captura real de pantalla, autenticación externa del captor ni protección contra host comprometido. |
| `Evidence` → archivos JSON | El conductor crea `before` y `after` con `canonical.to_vec()` al inicio; recoge la entrega y escribe resultados con `fs::write`. | El propio conductor; escritor de los archivos de resultados; fallos de escritura/almacenamiento. | Informe del ensayo; `inspect` coteja evidencia suministrada contra oráculos. | Bis `observer.rs` / `main.rs`; F05/F06/F10. Las dos copias iniciales no son dos lecturas del estado después de una mutación; no prueban conservación de una BD. Escritura de informe no constituye commit SV. |
| R1 compromiso → adaptador → traza | `ExecutionContinuity` posee autoridad y `Vec<ExerciseTraceEntry>`; se añade DispatchCommitted, se llama al adaptador y se añade Confirmed o Indeterminate. | `execute_mediated` amplía la traza mediante acceso exclusivo; adaptador determina su respuesta técnica y efecto. | Compromiso lógico local antes de despacho; Confirmed deriva de `Ok(())` del adaptador. | `execution.rs`; F06/F08/F11. No persistencia durable ni atestación independiente del mundo; éxito técnico no equivale a transacción distribuida. |

## Precisiones que cambian la lectura del riesgo

1. **Array no elimina las asignaciones dinámicas del recorrido.** `FixedVector`
   fija la longitud del vector admitido en el montaje; fuente, descriptor, JSON,
   IR, contexto y captura siguen usando colecciones dinámicas. `storage_bytes()`
   devuelve `size_of::<FixedVector>()`; no mide RAM total, capacidad reservada,
   objetos temporales ni RSS. Las mediciones históricas de proceso mantienen su
   propio alcance, sin cuotas de memoria/tiempo constituidas aquí.
2. **Admisión, entrega y confirmación material son fronteras distintas.** Un
   objeto admitido puede seguir existiendo en RAM aunque falle la captura o el
   guardado de un informe. No se puede ascender ese objeto a estado durable por
   llamarlo «validado». R2-0 ya aloja esa obligación.
3. **La procedencia de una observación no se obtiene de una lectura de archivo.**
   Este montaje recibe bytes de fixtures. No ha adquirido una señal física ni
   distinguido dos analíticas reales. Ninguna prueba aquí acredita F03 completo.
4. **La inmutabilidad de API tiene un perímetro.** No acredita memoria física
   incorruptible, exclusión de un host comprometido ni autenticidad de todos los
   archivos. La selección y la copia que consume el siguiente tramo también
   requieren su propia ligadura verificable.
5. **No se ha localizado un commit durable en estos recorridos.** La inspección
   de sus llamadas y manifiestos no muestra motor de BD, WAL, `sync_all` o
   protocolo de recuperación. Esto es una constatación acotada, no una afirmación
   de inexistencia de prototipos de persistencia en otros expedientes.

## Disposición inicial de los doce casos

| Casos de S26 | Avance material posible ahora | Dependencia que impide cerrar el caso completo |
| --- | --- | --- |
| F01 / F05 | Probar cambio de archivo después de recepción y conservación del objeto admitido; rechazar una nueva lectura alterada bajo la expectativa anterior. | Consulta histórica completa, concurrencia durante recepción y frontera de confirmación durable. |
| F02 / F04 | Reutilizar caracterización LIG/Frame/R1 como antecedente; especificar selección exacta y corte de proyección. | API operacional de consulta con dependencias/cobertura suficientes y campañas propias. |
| F03 | Determinar qué identidad aporta el productor y qué acredita la adquisición. | Productor/acquisición real o simulador explícito con evidencia suficiente de origen. |
| F06 / F07 / F08 | Recibir requisitos R2-0 de commit, índice, cobertura y recuperación. | Realización durable y frontera de confianza elegidas y justificadas en su sede; no simularlas con un diccionario. |
| F09 | Reutilizar correlación contextual; fijar respuesta tardía y selección activa. | Consumidor con selección y concurrencia reales; la captura síncrona actual no lo representa. |
| F10 / F12 | Alterar Capture tras admisión y comprobar D06; ausencia de captura y fallo posterior conservan D01/D07. | Pantalla/consumidor material, latencias, recursos y cobertura de efectos fuera del proceso. |
| F11 | Reutilizar consumo y traza R1 como antecedente. | Enlace revisión humana–compromiso–destino, concurrencia y recuperación; no equiparar deduplicación a idempotencia. |

## Primer contraste acotado

[Banco previo R01](r01/BANCO_PREVIO.json): cuatro sondas nuevas sobre el admisor
experimental existente, con fixture positivo I0205-01 copiado literalmente.
Las sondas no cambian semántica, núcleo ni el montaje original. Se comprometen
fuentes y expectativas antes de compilar/ejecutar. La copia de la fuente se altera
con un salto de línea, sin cambiar su contenido SVP semántico: debe conservarse
la identidad anterior en el objeto ya recibido y rechazarse la nueva lectura con
la expectativa anterior. También se distingue captura alterada de ausencia y
fallo posterior. [Código Rust](r01/probe.rs) y [reproductor](r01/reproducir.py).

Estas cuatro sondas son parciales de F01/F05/F10/F12, no cuatro casos globales
cerrados. Las restantes obligaciones conservan su estado pendiente. Los hashes
y rutas de las fuentes inspeccionadas se fijan en `r01/FUENTES.json`.
