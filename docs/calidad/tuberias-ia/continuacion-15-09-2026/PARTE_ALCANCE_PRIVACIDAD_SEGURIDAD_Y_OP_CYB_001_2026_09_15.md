# Parte de trabajo y alcance: privacidad, seguridad y relación con OP-CYB-001

**Fecha:** 15 de septiembre de 2026.  
**Seguimiento:** S32.  
**Estado:** en ejecución.

## 1. Qué hace este trabajo

Define las condiciones de privacidad y seguridad que deben conservar las consultas, las evidencias y las salidas del Sistema Vectorial SV, y establece su correspondencia con las obligaciones ya previstas en el primer universo de ciberseguridad inteligente, OP-CYB-001.

El trabajo comprende la identificación de requisitos, su asignación a los componentes responsables de cumplirlos y los criterios que permitirán comprobarlos. Distingue el conocimiento necesario para evaluar una actuación de los mecanismos que protegen el sistema que realiza esa evaluación.

## 2. Qué cubre

| Materia | Cobertura |
|---|---|
| Identidad y autenticación | Diferenciar persona, cuenta, sesión, agente, servicio y principal representado; determinar qué acredita cada comprobación. |
| Autorización | Delimitar operación, recurso, finalidad, destinatario, delegación, vigencia y revocación. |
| Confidencialidad y privacidad | Clasificar datos y derivados; limitar acceso y difusión; determinar vistas, conservación y tratamiento de metadatos. |
| Integridad y autenticidad | Conservar procedencia y transformaciones; detectar sustituciones y omisiones relevantes; distinguir integridad técnica y veracidad. |
| Disponibilidad y continuidad | Identificar dependencias, recursos, recuperación, reintentos y efectos inciertos. |
| Consultas distribuidas | Determinar cuándo conviene consultar en los custodios de origen y qué información puede circular entre ellos. |
| Terceros | Examinar solicitudes, respuestas, registros, retención y accesos de proveedores de API y otros servicios. |
| Verificación | Relacionar requisitos con contratos concretos, casos positivos, casos negativos y evidencia observable. |

Se consideran por separado datos personales, categorías especiales, información confidencial no personal y datos cuya combinación permita identificar o inferir atributos de una persona. Las restricciones también alcanzan explicaciones, enlaces, registros, cachés y copias de respaldo cuando contengan esa información.

La federación no constituye por sí misma anonimización. Una respuesta agregada tampoco acredita el estado de un activo concreto si pierde la identidad o la evidencia que exige su evaluación.

## 3. Hasta dónde llega

OP-CYB-001 mantiene su objeto: fundamentar la evidencia y legitimidad de la corrección de una vulnerabilidad mediante actualización de un activo. Se conservan sus 32 definiciones paramétricas, 17 controles y requisitos existentes.

La correspondencia utiliza especialmente RS01–RS04 para identidad, autoridad y vistas; RS07 para cobertura de flujos; RS09–RS12 para historia, custodia y continuidad; y las condiciones del consejo de la adenda §12. No amplía su catálogo ni convierte el universo en una plataforma general de autenticación, protección de datos o entrenamiento de modelos.

| Resultado alcanzado | Límite de ese resultado |
|---|---|
| Estudio documental de privacidad y marco europeo disponible | No acredita cumplimiento integral de un servicio o tratamiento concreto. |
| Correspondencia identificada con las obligaciones de OP-CYB-001 | No equivale a implementación de controles ni a ejecución del universo. |
| Diez pares de aceptación propuestos en el estudio | Son especificaciones conceptuales; sus pruebas de privacidad no se han ejecutado. |
| Condiciones de incorporación delimitadas | Los contratos concretos de los flujos afectados siguen pendientes. |

Se mantiene la diferencia entre la evidencia original bajo custodia y la vista mínima destinada al consejo. Suprimir toda atribución puede impedir una comprobación legítima; divulgar el original completo puede exceder la finalidad autorizada.

El alcance no incluye nuevos parámetros, otro universo, cambios de la semántica del núcleo, selección de plataforma, habilitación de API, tratamiento de datos personales reales ni ejecución de campañas. La GUI y el punto de retorno del trabajo principal conservan su situación vigente.

## 4. Qué queda pendiente y cuándo corresponde

1. **Durante BIS-03:** completar la correspondencia por flujo entre requisito, dato, relación, componente y prueba. Reutilizar los controles y casos existentes; justificar cualquier necesidad adicional.
2. **Antes de congelar una interfaz afectada o cerrar BIS-03:** constituir su contrato con identidad, finalidad, permisos, destinatarios, conservación, recursos, fallos y criterios de aceptación.
3. **Antes de habilitar el conector o iniciar el tratamiento personal correspondiente, lo que ocurra primero:** determinar las obligaciones jurídicas del caso, realizar la evaluación de impacto cuando proceda y aportar implementación y pruebas de los controles exigibles. Si ya existe tratamiento personal, su evaluación no se difiere.
4. **En la realización que corresponda:** preparar entradas artificiales y resultados esperados, ejecutar pruebas en Rust y registrar el alcance comprobado. Se conserva la prioridad y secuencia de las tareas principales.
5. **Ante cambios relevantes:** revisar las finalidades, datos, proveedores, permisos o riesgos afectados antes de ofrecer la nueva capacidad.

El siguiente trabajo concreto es completar la matriz de correspondencia de los flujos afectados, con referencia a las obligaciones existentes de OP-CYB-001 y a las interfaces del Lenguaje. Este parte no inicia una campaña ni habilita un servicio.

## 5. Condición de cierre del seguimiento

S32 permanecerá abierto mientras falte la correspondencia verificable de los flujos incluidos o la resolución de sus contratos y condiciones de habilitación.

Para cerrar el alcance documental deberán quedar identificados, para cada flujo, el requisito, su componente responsable, la prueba exigida y su situación. La implementación o prueba que corresponda a una fase posterior deberá tener un seguimiento concreto enlazado y una condición de habilitación explícita; no bastará con consignar «pendiente».

Un eventual cierre documental no acreditará seguridad material, anonimato, cumplimiento integral ni cierre de (p1+p3)-Bis.

## 6. Documentos de referencia

- [Estudio de privacidad BIS-03: clasificación, fuentes europeas y aceptación](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8fa67a917d78072582c24af76f3ce1f47d6c079d/docs/calidad/tuberias-ia/continuacion-15-09-2026/ESTUDIO_PRIVACIDAD_BIS03_2026_09_15.md).
- [OP-CYB-001: alcance, obligaciones de soporte y condiciones del consejo, §§3, 6–10 y 12](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/bbac1b44b1d3b845305e9cde492a08221206d631/dominios/ciberseguridad-inteligente/dominio-04-09-26/universos/OP-CYB-001/ACTA_CONTINUIDAD_Y_RELEVO_OP_CYB_001_AL_LENGUAJE_SV_2026_09_09.md).
- [Acta de continuidad del trabajo](ACTA_001_CONTINUIDAD_Y_RUMBO_2026_09_15.md).
- [Registro del suceso S32](../../Inventario-sv/sucesos/SUCESOS_SV.md#s32).
