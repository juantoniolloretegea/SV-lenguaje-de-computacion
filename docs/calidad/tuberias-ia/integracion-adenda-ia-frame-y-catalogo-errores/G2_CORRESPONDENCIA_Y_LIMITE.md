# G2: del recibo a una operación gobernada

**RETP-2026-147 · Delimitación del enlace; no realización productiva.**

## Decisión concreta

Se conserva el recibo público de lectura. No se le añade permiso, célula, SUCESO ni Frame. El enlace productivo a R1 permanece bloqueado por la falta del contrato que admita los referentes de autoridad constituidos. Esa falta está ya documentada en RETP-108 §8.3 y no se resuelve con un nuevo adaptador que fabrique objetos.

## Correspondencia de entradas reales

| Entrada o producto | Fuente y papel actual | Exigencia para actuar |
|---|---|---|
| Lote, montaje, posición, solicitud, cuerpo, traza | `ie004/lote-g1/lib.rs`: `EnlacePublico`, `ejecutar`, `recuperar`, `cuerpo`, `identidad`; custodios G1 | Conservación y pertenencia; no facultad institucional |
| Cinco campos de significado y `vigente: bool` | `ie004/semantica-a/servicio.rs`: `permiso_y_dato` | Política ficticia `FICCION-IE004/1`; no sustituyen requisitos ni autoridad R1 |
| Continuidad protegida | `decision_trace.rs`: `ProtectedDecisionContinuity` | Continuidad de autoridad admitida; no se obtiene del hash de un recibo |
| Forma y efecto | `decide_permit_traced` recibe `FormRef` y `EffectDescriptor` | Referentes constituidos, efecto y contexto del encargo; sin conversión desde `DATO` |
| Resultados de requisitos | `decide_permit_traced` recibe `&[ResolvedRequirementResult]` | Cadenas comprobadas bajo las reglas pertinentes; no resultados elegidos por el auxiliar |
| Permiso trazado | Resultado protegido `TracedPermitDecision::Granted` | Sólo después de decisión R1; no existe permiso negativo fabricado |
| Compromiso y ejercicio | `mediate_traced_permit` y `execute_traced_mediated` | Mismo efecto, continuidad y relación registrada antes de llamar al ejecutor |
| Inscripción y Frame | Operación, destino y productor aplicables | La lectura actual no exige actualización; declarar `TransitionData` no ejecuta transición |

Los paths de Rust se encuentran bajo `rust/sv_core/src/`; los de IE-004 bajo `docs/calidad/tuberias-ia/`. Véase [FUENTES_COTEJADAS.json](FUENTES_COTEJADAS.json) para los blobs del corte.

## Evidencia reutilizada y por qué no se repite

- RETP-145: 24 cuerpos públicos recuperables y 12 testigos del enlace, en debug y release; no inscripción ni permiso.
- RETP-108: dos clientes externos válidos y siete intentos de autoatribución rechazados por su causa Rust. Se comprobaron privacidad de referencias/premisa, diferencia entre referencia documental y autoridad, imposibilidad de convertir `CheckResult` elegido en `TracedPermit` y de sustituir la premisa de génesis por un booleano.
- Las fuentes directamente cotejadas conservan su identidad en la cabeza actual. Esta continuidad permite recibir aquellos resultados en su alcance; no se atribuye una ejecución nueva ni el cierre de un consumidor profesional.

El [contrato CYB §8.3](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e8558ea69aadacc3454cbd82f09a7bf7f4b06648/docs/calidad/CONTRATO_DE_CONSUMO_DOCUMENTAL_CYB_2026_09_09.md#83-alcance-y-siguiente-decisión) formula el requisito ya pendiente: objeto aprobado que fija raíz, titulares, facultades y alcance; procedimiento que admite su evidencia para producir las referencias protegidas y la premisa externa de R1. Su §8.4 acredita barreras, no ese procedimiento.

## Pares que debe satisfacer una futura realización

| Control admitido | Ataque correspondiente | Resultado exigido |
|---|---|---|
| Consulta pública con recibo completo | Misma respuesta ligada a otra invocación | Rechazo de pertenencia; no cambiar la respuesta del dominio |
| Datos documentales conservados sin permiso | Declaración documental ofrecida como autoridad | No producir autoridad ni permiso |
| Solicitud y requisitos bajo autoridad admitida | Booleano o resultado del modelo en lugar de la cadena | No ejecutar efecto; conservar causa |
| Permiso, efecto y compromiso de la misma continuidad | Sustituir efecto, decisión o continuidad | Rechazo en la frontera pertinente |
| Operación constituida de lectura | Exigirle una célula sólo por tener respuesta | Terminar en recibo; ninguna transición inventada |
| Operación constituida que sí exige transición | Presentar validación estructural de Frame como ejecución | No acreditar transición sin productor y evidencia |

Los dos primeros pares reutilizan evidencia indicada. Los demás requieren el montaje gobernado aplicable antes de su ejecución integrada; no se rellenan con una autoridad `for_test` para anunciar un enlace productivo.

## Qué puede avanzar y qué no

Puede continuar el contraste documental y público de pérdidas, la preparación de causas diagnósticas y su localización, conservando el orden del workflow. El vacío de autoridad no invalida una consulta de lectura ni bloquea toda la fila 9.

No puede acreditarse la actuación gobernada completa ni fabricarse su prueba positiva desde el recibo. Para ese salto hace falta recibir el contrato y referentes anteriores, o mantener expresamente fuera de la oferta esa actuación. Esta delimitación no solicita nuevas decisiones sobre el significado humano de frame ni modifica el reparto núcleo/dominio/agente.

La reserva de cualificación y las garantías materiales mantienen sus propias puertas. Aunque se resuelva G2, no se darían por satisfechas automáticamente.
