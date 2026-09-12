# Catálogo de causas y localización ES/EN

Versión `RECEPTION-DIAGNOSTICS/1`. Veinte causas locales de recepción y comparación. Son identidades técnicas del contrato candidato; no códigos canónicos nuevos del Lenguaje SV. La fase se conserva separada (`PrepareReception`, `Receive`, `PrepareCheck`, `Compare`). Los datos de entrada no se interpolan en los mensajes.

## Defectos tratados

| Hallazgo | Origen previo | Tratamiento y evidencia |
| --- | --- | --- |
| Pérdida del campo vacío o excesivo | `reception.rs::bounds`: tres campos agrupados en Empty/TooLarge | Causa específica emitida antes de copiar, con precedencia histórica. RG01–06, PI01–06 y RG13 |
| Pérdida de la declaración discordante | `receive`: emisor/versión agrupados | Se distinguen emisor, versión y ambos; no se consume T-0 por discordancia. RG07–09 |
| Pérdida del campo excesivo del comparador | `prepare`/`run`: contrato/esperado/observado agrupados | Tres causas separadas conservando límite y orden. OC05–07 y OC10 |
| Ausencia de presentación ES/EN de esas causas | Sólo variantes Rust observables | Tabla cerrada, versionada y comprobada contra las 240 observaciones de las seis reproducciones focales |

Estos defectos son de trazabilidad y presentación. Los rechazos históricos de esos casos ya existían; no se presenta la corrección como descubrimiento de una admisión indebida. Los ensayos de esta versión no han producido fallos inesperados de compilación o cualificación. Los cuatro clientes negativos han fallado al compilar precisamente con los seis códigos esperados de privacidad de API, conservados en PROCESOS.

## Fallo adicional del instrumento

Una espera final sin timeout podía eludir el presupuesto si el proceso cerraba ambos flujos antes de terminar. Localizado por inspección tras la campaña, corregido sin cambiar candidata ni esperado y comprobado con tres pruebas instrumentales. Se conservan versión original, versión corregida y evidencia en la [nota del instrumento](soporte/NOTA_DEL_INSTRUMENTO.md). No se crea un código canónico SV para este defecto Python.

## Tabla de localización

Los archivos de consumo son [idiomas/es/diagnosticos.json](idiomas/es/diagnosticos.json) y [idiomas/en/diagnosticos.json](idiomas/en/diagnosticos.json). El [CSV](CATALOGO_LOCALIZADO.csv) y el [inventario JSON](INVENTARIO_EMISORES.json) añaden emisor, líneas y casos de contraste.

| Clave | Español | English |
| --- | --- | --- |
| `RG.EMPTY_ACT` | El acto recibido está vacío. | The received act is empty. |
| `RG.EMPTY_ISSUER` | La declaración de emisor está vacía. | The issuer declaration is empty. |
| `RG.EMPTY_VERSION` | La versión declarada está vacía. | The declared version is empty. |
| `RG.ACT_TOO_LARGE` | El acto supera el límite de bytes. | The act exceeds the byte limit. |
| `RG.ISSUER_TOO_LARGE` | La declaración de emisor supera el límite de bytes. | The issuer declaration exceeds the byte limit. |
| `RG.VERSION_TOO_LARGE` | La versión declarada supera el límite de bytes. | The declared version exceeds the byte limit. |
| `RG.ISSUER_MISMATCH` | El emisor declarado no coincide con el instalado. | The declared issuer does not match the installed issuer. |
| `RG.VERSION_MISMATCH` | La versión declarada no coincide con la instalada. | The declared version does not match the installed version. |
| `RG.DECLARATIONS_MISMATCH` | El emisor y la versión declarados no coinciden con los instalados. | The declared issuer and version do not match the installed declarations. |
| `RG.ACT_MISMATCH` | El acto recibido difiere del acto instalado. | The received act differs from the installed act. |
| `RG.ALREADY_ATTEMPTED` | El intento de génesis de esta preparación ya se ha consumido. | The genesis attempt for this preparation has already been consumed. |
| `RG.GENESIS_REJECTED` | T-0 ha rechazado el plan; se conserva su error tipado. | T-0 rejected the plan; its typed error is retained. |
| `OC.FOREIGN_BINDING` | La obligación o su aplicabilidad no pertenece al vínculo exigido. | The requirement or its applicability does not belong to the required binding. |
| `OC.CORE_REQUIREMENT` | La igualdad de bytes no puede verificar esta obligación nuclear. | Byte equality cannot verify this core requirement. |
| `OC.EMPTY_CONTRACT` | El contrato de comprobación está vacío. | The check contract is empty. |
| `OC.CONTRACT_TOO_LARGE` | El contrato de comprobación supera el límite de bytes. | The check contract exceeds the byte limit. |
| `OC.EXPECTED_TOO_LARGE` | La referencia esperada supera el límite de bytes. | The expected reference exceeds the byte limit. |
| `OC.OBSERVED_TOO_LARGE` | La observación supera el límite de bytes. | The observation exceeds the byte limit. |
| `OC.EXACT_MISMATCH` | La observación difiere de la referencia exacta. | The observation differs from the exact reference. |
| `OC.MISSING_OBSERVATION` | No se dispone de la observación necesaria para comprobar. | The observation required for verification is unavailable. |

## Límites del inventario

El inventario cubre los puntos de fallo propios de recepción y comparación, incluidas las rutas de instalación interna. `RG.GENESIS_REJECTED` conserva el error `GenesisError` original; no traduce exhaustivamente sus causas anidadas. `OC.FOREIGN_BINDING` conserva el alcance conjunto del vínculo exigido; no distingue todas sus subcondiciones. La ausencia de observación y el resultado refutado son resultados de comprobación, no errores técnicos ni valores Tri.U.

El compilador conserva `CompileError::InvalidProgram(String)` y las emisiones textuales E004/E115 en `wellformed.rs`. Son pendientes localizados en [ESTADO_DE_CIERRE](ESTADO_DE_CIERRE.md), no deudas resueltas por esta tabla. El inventario de los 51 códigos canónicos continúa vigente y separado.
