# Comprobación específica del relevo OP-CYB-001

Fecha: 19 de octubre de 2026. **Criterios de aceptación pendientes de ejecución integrada.** Este documento acompaña [la entrega](ENTREGA.md); no declara resultados nuevos.

Los títulos N-01–N-10 agrupan encargos, pero no sustituyen el significado de cada requisito. Los contrastes siguientes conservan las distinciones de los apartados 5–7 y 12 del [acta con adenda](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/bbac1b44b1d3b845305e9cde492a08221206d631/dominios/ciberseguridad-inteligente/dominio-04-09-26/universos/OP-CYB-001/ACTA_CONTINUIDAD_Y_RELEVO_OP_CYB_001_AL_LENGUAJE_SV_2026_09_09.md). Cada encargo los vinculará al responsable y receptor de la necesidad correspondiente. La [matriz estructurada](MATRIZ.json) contiene los mismos contrastes.

## Nueve antecedentes conservados

| Antecedente | Necesidad que no puede perderse | Comprobación propia |
|---|---|---|
| REQ-CYB-001 | Identidad y evidencia admitida | Original, procedencia, objetos exactos y admisión: RS01, RS04, RS06, RS09. |
| REQ-CYB-002 | Estados independientes por sujeto y uso | Dos sujetos o usos comparten definición o parámetros sin compartir estado: N-01. Se prueba la sustitución indebida y la coexistencia legítima. |
| REQ-CYB-003 | Observación y transducción gobernadas | Separar captura, admisión y Tri: RS06. No suplir la puerta K1-T por etiquetas o cálculos exteriores. |
| REQ-CYB-004 | Historia y vigencia | Cambios intermedios, antecedentes, dependencias y nueva evaluación: RS03, RS09–RS12. |
| REQ-CYB-005 | Comprobación cualificada y sensibilidad | Un criterio debe detectar la alteración pertinente con observador independiente; igualdad de bytes no lo sustituye. Todos los contrastes incluyen sensibilidad. |
| REQ-CYB-006 | Cobertura finita | Conjunto y frontera explícitos, terminación y vacío acreditado: RS07, RS09, RS12. |
| REQ-CYB-007 | Consejo y autoridad separados | Alternativas, impactos, viabilidad, proporcionalidad, permiso y deber: RS02, RS05, RS08, RS11. |
| REQ-CYB-008 | Magnitudes exactas y tipos | Conservar literal original, tipo, unidad, rango y exactitud; no reconstruir exactitud después de convertir con pérdida a coma flotante: N-06/N-08 y RS03. El límite del lector documental no se atribuye a todo Nat. |
| REQ-CYB-009 | Justificación reconstruible y Frame legítimo | Significado, valor semántico, horizonte, consecuencias, destinatario y arquitectura cuando proceda: N-06 y RS08, RS10, RS11. |

## Doce distinciones profesionales

### RS01 — Atribución de acto, sesión y principal

**Sedes de recepción:** N-01. La responsabilidad principal se conserva en la ficha de cada necesidad; esta correspondencia no crea encargos duplicados.

| Comprobación | Criterio |
|---|---|
| Caso admisible | Recuperar actor, principal representado, sesión y acto, con sus relaciones acreditadas. |
| Caso adverso | Conservar el mismo nombre visible y sustituir actor, sesión o principal. |
| Resultado exigido | La atribución cambia o se rechaza por la relación alterada; un resultado técnico no constituye identidad. |
| Sensibilidad | Cambiar una sola relación y observar la causa sin alterar los bytes del resultado. |

### RS02 — Autoridad y delegación limitada

**Sedes de recepción:** N-01, N-09. La responsabilidad principal se conserva en la ficha de cada necesidad; esta correspondencia no crea encargos duplicados.

| Comprobación | Criterio |
|---|---|
| Caso admisible | Una operación dentro de facultad vigente conserva raíz, actor, destinatario, recurso y finalidad. |
| Caso adverso | Usar una referencia íntegra con delegación excedida o revocada; mantener una sesión posterior a la revocación de la cuenta. |
| Resultado exigido | La operación queda impedida cuando corresponde; no se presume que revocar la cuenta cierre toda sesión. |
| Sensibilidad | Mantener el texto y alterar sólo el alcance o vigencia del permiso; observar en el punto del efecto. |

### RS03 — Tiempo, revocación y antecedentes

**Sedes de recepción:** N-10, N-03. La responsabilidad principal se conserva en la ficha de cada necesidad; esta correspondencia no crea encargos duplicados.

| Comprobación | Criterio |
|---|---|
| Caso admisible | Reconstruir emisión, registro y efecto, con intervalos y precisión; conservar revocaciones y cambios intermedios. |
| Caso adverso | Confundir recepción con efecto; eliminar un antecedente; atribuir precedencia a intervalos incompatibles o inciertos. |
| Resultado exigido | Precedencia acreditada e incertidumbre quedan diferenciadas; no hay retroactividad por defecto. |
| Sensibilidad | Cambiar sólo una cota temporal o un antecedente y comprobar la variación exigida por el contrato. |

### RS04 — Vistas por finalidad y destinatario

**Sedes de recepción:** N-02, N-09. La responsabilidad principal se conserva en la ficha de cada necesidad; esta correspondencia no crea encargos duplicados.

| Comprobación | Criterio |
|---|---|
| Caso admisible | Recuperar original y vista autorizada, con relación y condición de acceso. |
| Caso adverso | Usar recuperabilidad documental como permiso para difundir; entregar otra vista o a otro destinatario. |
| Resultado exigido | Se conserva el original y se impide la difusión no autorizada antes de ofrecerla. |
| Sensibilidad | Cambiar sólo finalidad o destinatario y comprobar la diferencia de acceso material. |

### RS05 — Plan, razón y permiso

**Sedes de recepción:** N-01, N-05. La responsabilidad principal se conserva en la ficha de cada necesidad; esta correspondencia no crea encargos duplicados.

| Comprobación | Criterio |
|---|---|
| Caso admisible | Distinguir plan, autorización, razones, alternativas, viabilidad, proporcionalidad y consecuencias; aplicar una excepción urgente sólo si su régimen la constituye. |
| Caso adverso | Tratar toda desviación del plan como ilicitud, o invocar urgencia sin su régimen aplicable. |
| Resultado exigido | No se confunden previsión, permiso y excepción; el consejo no fabrica autoridad. |
| Sensibilidad | Mismo hecho con y sin excepción legítimamente constituida; la razón debe reflejar la diferencia. |

### RS06 — Admisión, indeterminación y fallo técnico

**Sedes de recepción:** N-02, N-03, N-06. La responsabilidad principal se conserva en la ficha de cada necesidad; esta correspondencia no crea encargos duplicados.

| Comprobación | Criterio |
|---|---|
| Caso admisible | Admitir evidencia válida de un acto no autorizado cuando el contrato lo permita; producir un valor sólo con evidencia y criterio constituidos. |
| Caso adverso | Confundir instalación fallida con fallo de captura; usar silencio como evidencia negativa; convertir un fallo técnico en Tri.U. |
| Resultado exigido | Se separan no admisión, no aplicabilidad, insuficiencia, valor semántico y fallo instrumental. |
| Sensibilidad | Mantener el acto observado y variar la validez de captura o la evidencia requerida; el diagnóstico debe distinguirlos. |

### RS07 — Cobertura de instancias, permisos y flujos

**Sedes de recepción:** N-01, N-02, N-07, N-09. La responsabilidad principal se conserva en la ficha de cada necesidad; esta correspondencia no crea encargos duplicados.

| Comprobación | Criterio |
|---|---|
| Caso admisible | Un conjunto finito y constituido conserva todas las dependencias exigibles; el vacío tiene justificación independiente. |
| Caso adverso | Omitir un trayecto o dependencia, mezclar ámbitos o declarar vacío lo no comprobado. |
| Resultado exigido | Falta de cobertura diagnosticada; no se presume suficiencia de una selección del propio candidato. |
| Sensibilidad | Retirar sólo una dependencia requerida del conjunto inicialmente válido, conservando las citas restantes. |

### RS08 — Explicación y salidas diferenciadas

**Sedes de recepción:** N-05, N-06, N-08. La responsabilidad principal se conserva en la ficha de cada necesidad; esta correspondencia no crea encargos duplicados.

| Comprobación | Criterio |
|---|---|
| Caso admisible | Conservar resultado técnico, legitimidad, incertidumbre y consecuencias en una explicación fiel, con arquitectura y significado constituidos. |
| Caso adverso | Llamar Frame a un informe sin arquitectura; invertir una negación; deducir legitimidad de un éxito técnico. |
| Resultado exigido | La presentación no cambia el dictamen ni confunde sus dimensiones; no se ofrece una figura sin fundamento como Frame legítimo. |
| Sensibilidad | Mantener el resultado estructurado y alterar sólo la explicación; detectar la discordancia. |

### RS09 — Estado inicial y límite de la historia

**Sedes de recepción:** N-04, N-10. La responsabilidad principal se conserva en la ficha de cada necesidad; esta correspondencia no crea encargos duplicados.

| Comprobación | Criterio |
|---|---|
| Caso admisible | Recuperar por separado referencia aprobada, estado observado y pasado conocido. |
| Caso adverso | Atribuir a una captura posterior el estado anterior, o restaurar una base antigua como vigente. |
| Resultado exigido | El límite histórico permanece explícito; una copia íntegra no acredita vigencia. |
| Sensibilidad | Añadir una captura posterior sin evidencia anterior y comprobar que no amplía el pasado acreditado. |

### RS10 — Antecedentes múltiples y relaciones tipadas

**Sedes de recepción:** N-04, N-10. La responsabilidad principal se conserva en la ficha de cada necesidad; esta correspondencia no crea encargos duplicados.

| Comprobación | Criterio |
|---|---|
| Caso admisible | Conservar extremos, tipos y respaldo de antecedentes múltiples; admitir referencia recíproca legítima. |
| Caso adverso | Convertir referencia recíproca en precedencia estricta, aceptar un ciclo de precedencia imposible o inferir causalidad sin respaldo. |
| Resultado exigido | Referencia, precedencia y causalidad mantienen reglas diferentes. |
| Sensibilidad | Conservar los extremos y cambiar sólo el tipo de relación; detectar el ciclo cuando sea una precedencia estricta. |

### RS11 — Custodia, designación y obligación pendiente

**Sedes de recepción:** N-01, N-05, N-10. La responsabilidad principal se conserva en la ficha de cada necesidad; esta correspondencia no crea encargos duplicados.

| Comprobación | Criterio |
|---|---|
| Caso admisible | Separar poder actuar, recibir pruebas y asumir un deber; acreditar aceptación cuando la exija el régimen. |
| Caso adverso | Transferir custodia y exonerar al obligado por el solo envío o acuse; suponer aceptación cuando sea exigible y falte. |
| Resultado exigido | La obligación pendiente continúa según su régimen, sin exoneración automática ni aceptación universal inventada. |
| Sensibilidad | Eliminar sólo la aceptación requerida y conservar entrega y acuse; el deber no debe darse por extinguido. |

### RS12 — Conciliación, cobertura y reevaluación selectiva

**Sedes de recepción:** N-03, N-04, N-07, N-10. La responsabilidad principal se conserva en la ficha de cada necesidad; esta correspondencia no crea encargos duplicados.

| Comprobación | Criterio |
|---|---|
| Caso admisible | Conservar identidad por clase, generación de la prueba y dependencias temporales pertinentes. |
| Caso adverso | Usar prueba de otra generación; repetir un efecto de resultado incierto; reevaluar sobre dependencias ajenas. |
| Resultado exigido | La conciliación conserva lo incierto; sólo se admiten pruebas pertinentes a la generación y ámbito; no hay repetición silenciosa. |
| Sensibilidad | Cambiar sólo generación o dependencia relevante y distinguirlo de un cambio no pertinente. |

## Doce obligaciones de suficiencia

[S13](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8a99e3de43c6799862b7450854b3751e39ba7e1c/docs/calidad/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/s13-suficiencia-semantica-ir/ACTA_SUFICIENCIA.md) sigue siendo la fuente competente de estas obligaciones y de las pérdidas conocidas. La tabla permite volver desde ellas a la entrega sin dar por ejecutado su contrato.

| Obligación | Alcance | Necesidades receptoras |
|---|---|---|
| O01 | Separar fallo, no admisión y valor semántico U | N-03, N-05 |
| O02 | Producir respuesta, justificación reconstruible y metadatos | N-06 |
| O03 | Exigir cobertura suficiente con acceso independiente | N-02, N-08 |
| O04 | Recibir instrucciones externas como datos sin autoridad | N-02 |
| O05 | Ligar decisión, permiso y efecto sin fabricar autoridad | N-01, N-09 |
| O06 | Conservar identidad de invocación y contexto | N-01, N-04 |
| O07 | Comprobar fidelidad hasta el objeto entregado | N-06, N-08 |
| O08 | Identificar la base realmente utilizada bajo igual nombre | N-08 |
| O09 | Distinguir recuperación original de nueva evaluación | N-04, N-10 |
| O10 | Emitir causas y procedencia sin reparación semántica | N-03, N-06 |
| O11 | Imponer canales, destinatario, mínimo y límites materiales | N-07, N-09 |
| O12 | Ligar revisión y cierre al objeto y alcance comprobados | N-05, N-09 |

## Perfiles y condición de suficiencia

Los perfiles fuente español e inglés son superficies explícitas de la gramática común; el perfil de dominio conserva significado y constitución profesional; el soporte tecnológico declara capacidades y obligaciones materiales. No se infiere que traducir una etiqueta constituya un dominio o realice aislamiento. Sus identidades, versiones, compatibilidad, ligaduras y prueba previa se comprueban conforme al [acta rectora de perfiles](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8a99e3de43c6799862b7450854b3751e39ba7e1c/docs/calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md), incluidas sus obligaciones PT01–PT14.

Antes de seleccionar una sede de representación —IR, contrato externo enlazado u otra composición admitida— se comprobará qué información debe recuperar y aplicar la operación. Ausencia, colisión, incompatibilidad y transformación deben quedar visibles. Si la capacidad no está realizada, no se ofrecerá como disponible.

Las pruebas locales de estos criterios no son exámenes de modelos y no envían contenido a proveedores. Su ejecución posterior requiere el encargo correspondiente; esta entrega deja definidos los productos y controles, sin declarar esas pruebas ejecutadas.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

