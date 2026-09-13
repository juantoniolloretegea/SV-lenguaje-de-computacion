# Interfaz elegida del montaje experimental

BIS-03 / I0205 · versión 0.1. Nombres y firmas de diseño para BIS-04; no API existente ni fichero Rust compilado.

## Componentes

| Componente | Recibe | Devuelve | Restricción |
| --- | --- | --- | --- |
| Recepción | Lectores de los cinco canales no confiables | Bytes acotados e inmutables | Cuota efectiva antes de reservar/leer sin límite; suma comprobada. |
| Decodificación | Bytes JSON de un canal | AST documental con orden y lexemas exactos | Rechazo de duplicados, profundidad, valores y miembros; sin interpretar SVP. |
| Compilación | Fuente UTF-8, nombre esperado, perfil explícito | `sv_core::IrProgram` o error existente | `compile_svp_profile`; ningún bypass a frontend/construcción interna. |
| Soporte | IR aceptada y referencia/cuerpo de soporte | Referencia exacta y dimensiones admitidas | Todas las CellSpec; ninguna actualización automática de versión. |
| Identidad/estado | IR, vínculo presentado, registro y contexto | Célula experimental admitida | Resolución tipada de enlaces; vector idéntico al estado independiente. |
| Geometría | Célula admitida, descriptor y convenio | Descriptor validado | M01/M02 antes de M03; preservar los bytes literales. |
| Entrega | Descriptor validado y contexto | Captura local o ausencia/fallo instrumentado | Captor fuera del sujeto; mismo buffer validado como origen del despacho. |
| Observación | Traza, captura, recibo y oráculo independiente | Veredicto de ensayo | No llamar a la aceptación del sujeto para decidir conformidad. |

## Tipos y métodos propuestos

La interfaz conceptual de admisión será:

```rust
pub fn admit(
    request: ReceivedBytes,
    context: &TrustedContext,
    registry: &TrustedRegistry,
) -> Result<AdmittedDelivery, Rejection>;
```

`ReceivedBytes` conserva por separado metadatos de solicitud, fuente SVP, estado matemático, soporte y geometría. La ausencia del soporte es representable; no significa perfil vacío válido. El lector no acepta rutas elegidas libremente por la petición. Los metadatos del caso se materializan exactamente con la convención UTF-8 fijada en RETP-218; el fichero contenedor de los 26 casos no se pasa completo al receptor.

La AST distingue `Null`, `Bool`, lexema entero, texto, lista y pares de objeto ordenados. Al acceder a un campo se distingue ausencia, `null` y valor presente. Cada fase construye sus tipos validados; no se deserializa directamente a `AdmittedDelivery`. Duplicar una clave mediante un escape Unicode sigue siendo duplicarla.

`TrustedContext` y `TrustedRegistry` tienen campos privados. Se construyen desde material de custodia del conductor, con identidad y esquema verificados; su constructor no admite selección de identidad desde datos del emisor. Esa separación es un contrato del laboratorio, no autenticación criptográfica de un origen externo.

`AdmittedDelivery` tendrá acceso público sólo de lectura:

```rust
impl AdmittedDelivery {
    pub fn vector(&self) -> &[sv_core::Tri];
    pub fn descriptor(&self) -> &[u8];
    pub fn identity(&self) -> &CellIdentity;
    pub fn context(&self) -> &DeliveryContext;
}
```

Los miembros, `FixedVector` y el constructor válido serán privados. No se proporciona `Default`, setter, `DerefMut` ni deserialización pública a objetos admitidos. Los nombres de instancia/revisión no se extraen de los bytes geométricos. Las identidades del estado y del descriptor se conservan como objetos distintos.

`Rejection` conservará etapa, etiqueta experimental, causa y evidencia pertinente. `CompileError` se guarda como error de P02; no se reemplaza por una supuesta causa canónica nueva. La traza de etapas indica cuáles se alcanzaron, rechazaron o no se ejecutaron. Para cada caso se compara la primera guarda real con la comprometida.

La captura se construye únicamente al recibir el buffer en el receptor de pruebas. El observador vive en otro módulo/binario de ensayo, con datos que el sujeto no puede consultar por su API. Las inyecciones pertenecen al conductor y se aplican fuera de `admit`. El fallo de entrega D07 tiene un camino explícito posterior al despacho; la ausencia ordinaria usa D01. Un recibo favorable sólo se acredita con la captura y contexto concordantes que ya exige el oráculo.

## Almacenamiento y construcción

Se elige un enum interno de arrays para los tamaños del soporte sintético:

```rust
// Diseño, no implementación integrada.
enum FixedVector {
    N16([sv_core::Tri; 16]),
    N25([sv_core::Tri; 25]),
    N49([sv_core::Tri; 49]),
}
```

Se construye después de soporte y comprobación de longitud, mediante conversión fallible desde el slice de IR. Su presencia no autoriza 49 en v1 ni permite redimensionar una célula admitida. No se devuelve `&mut [Tri]`. Las copias y el tamaño de la enum se medirán en la realización: no se atribuye de antemano menor coste que a un almacenamiento dinámico encapsulado.

Puerta de construcción de BIS-04: compilar y usar la lectura de un objeto realmente admitido; comprobar rechazo de longitud/soporte; intentar desde un consumidor externo construir sus campos o invocar crecimiento/sustitución y exigir error de compilación por privacidad/API, no por un fallo ajeno. Los archivos de esas sondas quedarán fijados antes de ejecutarlas, una vez materializadas las firmas. Esta comprobación de encapsulación no reemplaza los 26 casos de funcionamiento.

## Artefactos de ejecución y reproducción

El montaje conservará Cargo.toml/Cargo.lock, versión de compilador, fuente exacta de `sv_core`, código ES/EN del adaptador, captor y observador, instrucciones de construcción, hashes y orden de ejecución. `cargo build --locked --offline` deberá compilar el proyecto; ningún éxito de Python contará como aceptación Rust.

El banco instrumental J01–J14 se ejecuta sólo sobre la decodificación; no confiere reconocimiento SV a esos JSON. H01–H05 cotejan bytes con huellas independientes. Después se ejecutan I0205-01–26 y la secuencia v1→v2→v1. Para repeticiones se añade identidad de ejecución externa al contexto del caso, sin reescribir el estímulo comprometido. La lista y las cuatro sensibilidades OBS siguen siendo las de RETP-218.

Se distingue error del cargador/conductor, rechazo del sujeto, fallo del captor y discrepancia del observador. Una discrepancia de contrato o banco se documenta y versiona antes de la nueva ejecución; no se ajusta el esperado para hacer pasar el código.
