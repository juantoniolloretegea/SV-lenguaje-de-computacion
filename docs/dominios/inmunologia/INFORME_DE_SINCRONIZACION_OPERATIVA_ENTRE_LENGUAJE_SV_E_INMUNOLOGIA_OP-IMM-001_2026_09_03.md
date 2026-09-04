# Informe de sincronización operativa entre Lenguaje SV e Inmunología para OP-IMM-001

**Fecha:** 3 de septiembre de 2026  
**Emisor:** unidad responsable del Lenguaje de computación SV  
**Destinatario:** dominio de Inmunología  
**Base:** valoración `ENCAJA_CON_CAMBIOS` y adversarial corregida `PASA_CON_CORRECCIONES_CONSTITUTIVAS_INCORPORADAS`  
**Estatuto:** marco de coordinación; no autoriza cambios del Lenguaje, laboratorio, datos reales ni uso clínico  
**Corrección constitutiva incorporada:** 04-09-2026; autonomía separada de concurrencia y secuencia de relevos actualizada.

## 1. Decisión de sincronización

Inmunología conserva capacidad y autonomía para continuar su trabajo constitutivo, pero **no es el frente activo ahora**. No depende de R2 ni de una nueva IR para pensar el dominio; sí respeta la secuencia de un único frente y queda en pausa hasta recibir una candidata del Lenguaje.

La frontera correcta distingue capacidad de agenda:

- `CAPACIDAD_CONSTITUTIVA_DE_INMUNOLOGIA = CONSERVADA`;
- `INMUNOLOGIA = PAUSA_CONTROLADA`;
- `INTEGRACION_EJECUTABLE_CON_LENGUAJE_SV = CONDICIONADA`.

La condición técnica no reduce el espacio de investigación del dominio. El Lenguaje recibe requisitos demostrados y no fija de antemano cuántos universos, raíces, parámetros, reglas, fuentes o consecuencias debe descubrir Inmunología. La pausa sólo ordena cuándo se ejerce esa autonomía.

## 2. Principios de autonomía coordinada

1. Cada universo se constituye y se prueba por sí mismo.
2. La exploración recorre las raíces de `Q0` secuencialmente, sin cuotas, parámetros forzados ni cierre por número prefijado.
3. Los residuos, indeterminaciones y causas de `U` se conservan; no se eliminan para facilitar una geometría.
4. No existe herencia automática de requisitos, reglas, estados ni soluciones desde `OP-IMM-001` hacia otro universo.
5. Una coincidencia entre universos sólo puede proponerse como elemento común cuando se demuestre su equivalencia causal, no por semejanza nominal.
6. Los productos intermedios del dominio no quedan sometidos a una auditoría externa reiterada. La revisión global corresponde al cierre del recorrido; los puntos de sincronización sólo impiden afirmaciones o ejecuciones no autorizadas.
7. El laboratorio mencionado en esta coordinación es un **laboratorio de software**, no un laboratorio clínico.

## 3. Tres carriles de trabajo

| Carril | Inmunología puede hacer | Límite de sincronización |
|---|---|---|
| `A — AUTONOMO` | constituir universos; formular preguntas; agotar raíces; descubrir y tipar parámetros; fijar propiedad, fuentes, reglas, criticidad y consecuencias; documentar causas de `U`; diseñar casos sintéticos; definir perfiles FHIR o terminológicos propios del dominio | no afirmar que el Lenguaje ya los representa ni que existe uso clínico |
| `B — PREPARACION_DE_INTERFAZ` | elaborar manifiestos versionados, resultados tipados por parámetro, mapas de proyección, catálogos cerrados de salida, testigos de pérdida, dependencias y huellas | son contratos candidatos externos; no cambian gramática, IR ni ejecutable |
| `C — ACTO_MATERIAL_CONDICIONADO` | integrar, compilar, persistir, desplegar o modificar Lenguaje/motor sólo tras decisión y autorización correspondientes | incluye Rust, WASM, gramática, IR, diagnósticos, serializador, laboratorio y ejecución de R2/R3/R4 |

Los carriles clasifican actos y responsabilidades; **no autorizan ejecución sustantiva paralela**. A y B podrán avanzar durante un frente inmunológico expresamente abierto, sin convertir C en requisito previo. En el estado actual permanecen en pausa. El carril B debe conservar las necesidades del dominio incluso cuando el Lenguaje vigente no pueda expresarlas.

## 4. Contrato mínimo que ambos lados deben respetar

### 4.1 Geometría

- La célula mínima es `SV(9,3)`.
- No existen células SV menores.
- Quedan prohibidos el relleno, la duplicación, la fragmentación artificial de identidades y la mezcla de agrupaciones clínicas para completar nueve posiciones.
- Las seis agrupaciones G6 de cardinalidades `(6,1,3,2,6,9)` permanecen como agrupaciones externas.
- `M-MODIFIER-001` es únicamente **candidata** a célula: la cardinalidad nueve no constituye su semántica.

Para constituir esa candidatura deben demostrarse, como mínimo, nueve posiciones inequívocas, particiones ternarias disjuntas y exhaustivas para cada observable admitido, un codominio, una semántica de salida, la operación aplicada y la suficiencia de la representación para esa operación.

### 4.2 Estados y fallos

- `0`, `1` y `U` son valores del dominio formal que deben tener definición y procedencia.
- Un error de esquema, carga, dependencia, configuración o ejecución produce `EJECUCION_TECNICA_NO_VALIDA`; nunca se convierte en `U` ni en una salida clínica.
- La ausencia de regla y la ausencia de configuración se registran por separado de la indeterminación clínica.
- No se normaliza, completa ni corrige silenciosamente una entrada.

### 4.3 Salida y autoridad

- El núcleo no genera prosa clínica libre.
- Toda salida candidata debe pertenecer a un catálogo cerrado, tener versión y conservar las causas que la producen.
- La autoridad clínica, la aprobación sanitaria y la decisión asistencial no pertenecen al Lenguaje.
- La serialización técnica de IR no equivale a una salida clínica canónica.

### 4.4 Procedencia y reproducción

Toda fuente, regla, configuración, terminología y transformación causalmente relevante debe poder identificarse mediante sistema, versión, jurisdicción o ámbito cuando proceda, vigencia y huella verificable. **Toda ejecución válida con identidad completa idéntica debe producir exactamente los mismos bytes de salida canónica. Un fallo técnico no es una salida alternativa: determina que no existe ejecución clínica válida y debe generar exclusivamente el registro técnico estructurado correspondiente.**

## 5. Paquete de entrega de Inmunología por universo

Inmunología conserva libertad para descubrir el contenido. Cuando un universo se presente a sincronización técnica, el paquete debe permitir evaluar pérdida y responsabilidad sin reinterpretarlo:

| Bloque | Contenido mínimo |
|---|---|
| identidad | `Universe_ID`, versión, finalidad y alcance negativo |
| parámetros | IDs estables, definición, propietario único, criticidad y ausencia de duplicados semánticos no declarados |
| procedencia | fuentes, versiones, vigencia, jurisdicción o ámbito, huellas y dependencias |
| admisibilidad | reglas de entrada, estados ausentes y motivos de rechazo |
| ternarización | particiones de `0/1/U`, exhaustividad, disjunción y tratamiento de bordes |
| fallo técnico | catálogo separado de errores de esquema, configuración, dependencia y ejecución |
| operación | función pretendida, entradas, codominio y propiedad de suficiencia que debe conservarse |
| salida | catálogo cerrado, semántica, criticidad, vetos y autoridad humana u organizativa |
| referencias externas | perfiles FHIR, sistemas terminológicos, UID o hashes cuando resulten aplicables |
| corpus sintético | casos positivos, negativos, límites, colisiones y testigos de pérdida |
| necesidad declarada | capacidad disponible, responsabilidad externa o pérdida que exigiría valorar un cambio del Lenguaje |

No se impone una plantilla clínica cerrada ni un número máximo de elementos. El paquete se completa con lo que el universo demuestre, no con casillas artificiales.

## 6. Entrega recíproca del Lenguaje

Cuando reciba un paquete, la unidad del Lenguaje devolverá una clasificación reproducible:

1. `REPRESENTABLE_HOY_DEMOSTRADO`;
2. `REPRESENTABLE_HOY_NO_INTEGRADO`;
3. `RESPONSABILIDAD_EXTERNA_AL_LENGUAJE`;
4. `EXTENSION_COMPATIBLE_NECESARIA`;
5. `FASE_FUTURA_YA_PREVISTA`;
6. `FUERA_DE_ALCANCE_O_NO_AUTORIZADO`.

Cada clasificación debe señalar evidencia, pérdida, propietario, dependencia, momento y siguiente acto. Una necesidad no se trasladará al núcleo si puede permanecer en un perfil externo sin pérdida normativa. Si se propone modificar el Lenguaje, deberá existir un testigo concreto de pérdida y un contraste entre universos que descarte una contingencia exclusiva del primero.

## 7. Disparadores de sincronización

| Hecho observado | Acto adecuado | Lo que no ocurre automáticamente |
|---|---|---|
| cierre constitutivo de un universo | entregar su paquete y conservarlo con corte exacto | no se modifica el Lenguaje |
| aparición de un segundo dominio heterogéneo | comparar causalmente ambos sin herencia; el designado es ciberseguridad inteligente cuando reciba relevo | no se universalizan campos por coincidencia nominal ni se inicia en paralelo |
| pérdida representacional demostrada | decidir si corresponde al dominio, motor, Lenguaje o infraestructura | no se abre R2 ni se cambia IR por inercia |
| necesidad del primer efecto autorizado | contrastar con las fronteras de R1 y la eventual fase pertinente | no se confunde mediación intra-proceso con persistencia |
| propuesta concreta sobre gramática o IR | adversarial inter-universos, especificación, corpus negativo y regresión integral | no se altera producción en el mismo acto |
| intención de usar datos reales | abrir un expediente distinto de privacidad, calidad, riesgo, validación clínica y regulación | la conformidad técnica no autoriza uso clínico |

Estos disparadores son puntos de entrega y decisión, no cuotas temporales ni auditorías de cada producto intermedio.

## 8. Aplicación al próximo retorno de OP-IMM-001

El trabajo inmediato pertenece al Lenguaje: reparación de la PR #60, `N0` y cierre de invariantes intrínsecos. Inmunología permanece congelada. Cuando reciba el relevo y el corte candidato deberá:

- verificar identidades de entrada y salida;
- validar la correspondencia entre `REQ-IMM-SV-001..015` y `REQ-IMM-LSV-001..044`, sin crear una tercera familia;
- instanciar y falsar el contrato candidato de perfil de dominio respecto de `OP-IMM-001 / Q0 v0`, no de toda la Inmunología;
- conservar `REQ-IMM-SV-011 = U_NO_DECIDIDO` salvo evidencia nueva;
- producir testigos positivos, negativos y de pérdida;
- devolver un paquete único y trazable al Lenguaje.

La preparación de interfaz mantendrá las seis agrupaciones de G6 fuera de la geometría celular. Para `M-MODIFIER-001` podrá prepararse una **prueba de constitución**, pero no declararse una célula hasta satisfacer todos los requisitos semánticos de la sección 4.1.

No debe hacerse todavía:

- convertir las agrupaciones de 1, 2, 3 o 6 en células;
- completar posiciones mediante relleno, duplicación o mezcla;
- declarar representados fielmente los 27 parámetros por poder enumerarlos;
- usar `U` como contenedor de fallos técnicos;
- modificar gramática, IR, Rust, WASM, perfiles soberanos o diagnósticos;
- abrir o ejecutar materialmente R2;
- emplear datos reales o producir una salida asistencial.

## 9. Estado coordinado

La sincronización preserva las dos autonomías: Inmunología puede descubrir sin que la geometría del Lenguaje predetermine su clínica; el Lenguaje puede exigir prueba de representación sin apropiarse del contenido clínico.

El punto estable es:

```text
INMUNOLOGIA_CAPACIDAD_CONSTITUTIVA = CONSERVADA
INMUNOLOGIA = PAUSA_CONTROLADA
INMUNOLOGIA_ESPERA_A_R2 = NO
UNIVERSOS_RECORRIDO = SECUENCIAL_SIN_CUOTA_PREFIJADA
HERENCIA_AUTOMATICA_ENTRE_UNIVERSOS = PROHIBIDA
REVISION_GLOBAL = AL_CIERRE_DEL_RECORRIDO
CELULA_MINIMA_SV = SV(9,3)
CELULAS_MENORES = PROHIBIDAS
M_MODIFIER_001 = CANDIDATA_PENDIENTE_DE_CONSTITUCION
OP_IMM_001_INTEGRACION_SV = ENCAJA_CON_CAMBIOS
CAMBIO_INMEDIATO_DEL_LENGUAJE = NO
LABORATORIO_DE_SOFTWARE_ABIERTO = NO
R2_EJECUTADA = NO
USO_CLINICO_O_DATOS_REALES = PROHIBIDO
```

El siguiente paso operativo no pertenece al dominio: corresponde al Lenguaje reparar la PR #60 y cerrar `N0` e invariantes intrínsecos. El próximo paso de Inmunología ocurrirá sólo tras relevo explícito. Cualquier experimento ejecutable, apertura de laboratorio o cambio soberano seguirá requiriendo un acto separado y autorización expresa.
