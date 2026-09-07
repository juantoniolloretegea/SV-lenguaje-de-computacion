# Acta de conformidad de transición secuencial desde OP-IMM-001 al Lenguaje SV

> **Relevo vigente de la fila 3 · 07/09/2026:** [Domain, §26](#domain-relevo-20260907), RETP-086. Tras su promoción siguen concordancia diagnóstica/deuda/corpus, DFL-010 y condiciones de K1-T. F recibe expresamente las ligaduras de instancia aún no representadas. Los relevos anteriores conservan su fecha y alcance.

> **Continuidad vigente · 06/09/2026:** la [adenda rectora de secuencia (§§12–17)](#adenda-secuencia-20260906) actualiza el recorrido desde PR #61 hasta la consolidación nuclear y su continuación material. Distribuye las obligaciones de los perfiles tecnológicos y conserva los estados históricos de esta acta. Registro RETP-076.

> **Relevo de la fila 1:** [recepción N0-01, §18](#recepcion-n0-01-20260906), RETP-077. Tras su integración corresponde reparar los oráculos; N0-02 viene después. El §17 conserva el estado anterior a esta recepción.

> **Relevo de la fila 2:** [reparación de oráculos, §19](#oraculos-20260906), RETP-078. Tras su promoción corresponde K1 desde N0-02; las divergencias detectadas conservan su deuda.

**Fecha:** 3 de septiembre de 2026  
**Sede:** `SV-lenguaje-de-computacion`  
**Rama de trabajo:** `valoracion-op-imm-001-20260903`  
**Naturaleza:** acta de conformidad arquitectónica y de continuidad operativa  
**Estatuto:** punto de relevo entre el caso director inmunológico y el siguiente frente del Lenguaje  
**Dictamen:** `CONFORME_PARA_TRANSICION_SECUENCIAL`  
**Corrección constitutiva incorporada:** 04-09-2026; exhaustividad corregida, secuencia serial precisada y segundo falsador designado.

## 1. Objeto

Esta acta fija, sin ambigüedad, la decisión de secuencia adoptada después de la valoración y la adversarial de `OP-IMM-001`:

1. Inmunología queda en **pausa controlada**, no cerrada ni descartada;
2. `OP-IMM-001` queda constituido como **primer caso director** del siguiente trabajo del Lenguaje SV;
3. la línea operativa única pasa ahora al Lenguaje de computación;
4. no existe ejecución paralela de ambos frentes;
5. el regreso a Inmunología se producirá cuando el Lenguaje disponga de una candidata estructural que deba contrastarse con el caso director;
6. otros dominios posteriores deberán someter esa candidata a presión antes de universalizar soluciones.

El acta no constituye cierre del dominio de Inmunología, modificación del Lenguaje, apertura del laboratorio ni autorización de R2.

## 2. Cortes de continuidad

| Objeto | Corte | Estatuto en esta transición |
|---|---|---|
| Lenguaje SV soberano | `main@3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d` | base estable leída; no modificada por esta acta |
| Rama documental de trabajo | `valoracion-op-imm-001-20260903@691d79b6246c4e63d3052a48b69606ef232c870c` antes de la corrección de 04-09-2026 | punto documental auditado; la identidad final se verifica de nuevo antes de fusionar |
| Inmunología | `SVperitus-dataset/dominio-inmunologia@3bea6b714be3bd1330e6ca6bbbc228b0eb9c065d` | corte congelado para pausa controlada |
| Playground | identidad ejecutable anterior, no modificada por este expediente | estable e inalterado |

La comprobación pública aplicable al `head 691d79b…` anterior a esta corrección es la ejecución **`Conformidad SVP` `33755720470`** (`run_number = 261`), concluida con `success`. Es una comprobación documental: no acredita nuevas capacidades clínicas ni cierre nuclear. El `head` resultante de la corrección deberá superar de nuevo el flujo antes de cualquier fusión.

## 3. Corpus documental conforme

El punto de relevo está formado por los siguientes documentos, todos en `docs/dominios/inmunologia/`:

1. `VALORACION_TECNICA_Y_ENCAJE_DE_OP-IMM-001_CON_EL_LENGUAJE_SV_2026_09_03.md`;
2. `ADVERSARIAL_DE_CONTINUIDAD_Y_CONFORMIDAD_DE_LA_VALORACION_OP-IMM-001_2026_09_03.md`;
3. `INFORME_DE_SINCRONIZACION_OPERATIVA_ENTRE_LENGUAJE_SV_E_INMUNOLOGIA_OP-IMM-001_2026_09_03.md`;
4. la presente acta de conformidad de transición.

Los tres documentos antecedentes, ya corregidos para reconciliar las familias `REQ-IMM-SV-001..015` y `REQ-IMM-LSV-001..044`, establecen, respectivamente:

- el dictamen `ENCAJA_CON_CAMBIOS`;
- el resultado adversarial corregido `PASA_CON_CORRECCIONES_CONSTITUTIVAS_INCORPORADAS`: el dictamen `ENCAJA_CON_CAMBIOS` se mantiene, pero se retiran la exhaustividad autorreferencial y otras formulaciones auxiliares refutadas;
- el contrato de sincronización que separa autonomía clínica, interfaz externa y actos materiales condicionados.

## 4. Decisión de ingeniería de procesos

No se adopta un modelo paralelo. La capacidad operativa se aplica a un único frente cada vez y los relevos se producen sobre cortes identificados.

La alternativa de cerrar íntegramente Inmunología antes de continuar el Lenguaje se rechaza porque acumularía especificación clínica sobre carencias lingüísticas ya demostradas y aumentaría el retrabajo. También se rechaza desarrollar el Lenguaje en abstracto, porque perdería el primer caso real capaz de revelar insuficiencias y responsabilidades.

Se adopta el siguiente ciclo serial:

1. congelar el caso director en un corte reproducible;
2. cerrar primero las insuficiencias nucleares del Lenguaje que ya son independientes del dominio;
3. diseñar una candidata general utilizando `OP-IMM-001` como prueba directora;
4. regresar a Inmunología para medir conservación, pérdida y responsabilidad;
5. someter la candidata a otros dominios estructuralmente diferentes;
6. universalizar únicamente invariantes que sobrevivan a los contrastes correspondientes.

## 5. Estatuto de OP-IMM-001 como caso director

`OP-IMM-001` aporta una carga estructural concreta: identidad de ejecución, 27 parámetros, procedencias, reglas, configuraciones, valores ternarios, fallos técnicos, resultados por parámetro, trazas, testigos de pérdida, autoridad, salida cerrada y necesidades de persistencia.

Su condición de caso director significa que:

- debe utilizarse para formular ataques, requisitos y oráculos del Lenguaje;
- debe revelar qué información se conserva y cuál se pierde;
- debe impedir que una solución formalmente válida resulte inútil para el dominio;
- debe ser reejecutado o recontrastado cuando exista una candidata pertinente.

No significa que:

- Inmunología se convierta en autoridad universal del Lenguaje;
- sus nombres, estándares o reglas clínicas deban entrar en la gramática;
- sus 27 parámetros determinen por sí solos tipos universales de IR;
- una necesidad exclusiva del primer universo justifique modificar el núcleo;
- el expediente autorice integración, despliegue o uso clínico.

## 6. Invariantes preservados

### 6.1 Geometría

- La célula mínima permanece en `SV(9,3)`.
- No existen células menores.
- Quedan prohibidos relleno, duplicación, fragmentación artificial de identidades y mezcla de agrupaciones para completar nueve posiciones.
- Las cardinalidades G6 `(6,1,3,2,6,9)` son agrupaciones externas, no tamaños de célula.
- `M-MODIFIER-001` continúa como candidata pendiente de constitución semántica; nueve parámetros no constituyen una célula.

### 6.2 Semántica y fallo

- `U` no absorbe errores de esquema, carga, configuración, dependencia o ejecución.
- El fallo técnico produce `EJECUCION_TECNICA_NO_VALIDA`, no una salida del dominio.
- La serialización técnica no equivale a salida clínica canónica.
- Ninguna transformación causalmente relevante puede normalizarse o corregirse de forma silenciosa.

### 6.3 Responsabilidad

- Inmunología conserva finalidad, significado, parámetros, fuentes, reglas y criticidad clínica.
- El Lenguaje conserva formas, tipos, bienformación, semántica formal y operaciones constituidas.
- Motor, infraestructura y organización conservan sus responsabilidades propias.
- No existe herencia automática de soluciones entre universos o dominios.

## 7. Primer frente del Lenguaje tras el relevo

La continuidad del Lenguaje comienza por una revisión de cierre nuclear, no por R2 ni por una extensión inmunológica de la IR.

El primer inventario vinculante incluye:

1. unicidad de `Codomain`;
2. totalidad y unicidad de `OutputSemantics`;
3. rechazo de claves homónimas en la proyección JSON;
4. rechazo de referencias `Horizon.architecture` colgantes;
5. estatuto del ensamblaje de unidades vacías;
6. cualquier dependencia directa entre estas carencias y la suficiencia representacional por operación.

Antes de implementar se deberá:

- reconstruir el estado exacto de especificación, referencia Python, Rust, WebAssembly, pruebas y deuda viva;
- separar defecto normativo, defecto de realización y defecto del comprobador;
- fijar el orden de cierre y sus oráculos negativos;
- comprobar que la corrección no modifica la geometría `SV(9,3)` ni particulariza el núcleo para Inmunología.

## 8. Fronteras de autorización

La presente acta autoriza la **continuidad documental y preparatoria** del frente nuclear del Lenguaje. No autoriza por sí misma:

- abrir o ejecutar el laboratorio;
- modificar gramática, IR, referencia Python, Rust, WASM o producción;
- abrir materialmente R2, R3, R4 o una garantía;
- integrar `OP-IMM-001` en el ejecutable;
- usar datos reales;
- emitir una afirmación de aptitud clínica, seguridad sanitaria o conformidad regulatoria;
- fusionar la rama de trabajo en `main`.

Cada acto material posterior conservará su régimen de autorización, rama, pruebas, adversarial y promoción.

## 9. Condición de regreso a Inmunología

Inmunología permanece en pausa hasta que concurra una de estas condiciones:

1. exista una candidata del Lenguaje que deba contrastarse con `OP-IMM-001`;
2. el frente del Lenguaje demuestre que necesita una precisión constitutiva que sólo el dominio puede resolver;
3. el Director ordene expresamente cambiar el frente operativo.

El regreso no heredará automáticamente una solución. Reabrirá el corte inmunológico declarado, aplicará los requisitos y oráculos conservados y registrará cualquier pérdida sin adaptarla por conveniencia.

## 10. Dominios posteriores

Después de la primera revalidación inmunológica, el segundo falsador designado es el dominio heterogéneo **ciberseguridad inteligente**, constituido como semilla el 04-09-2026 en `SVperitus-dataset`, rama `dominio-ciberseguridad-inteligente`, ruta `dominios/ciberseguridad-inteligente/dominio-04-09-26/`.

Su trabajo sustantivo no comienza ahora. Recibirá el relevo sólo después de que Lenguaje incorpore el retorno inmunológico y publique un corte candidato. Su función será atacar supuestas generalidades mediante infraestructuras operacionales y, cuando aplique, sistemas y modelos de IA versionados y reproducibles. No se fija por esta acta un número suficiente de dominios ni se preseleccionan resultados: dos perfiles permiten contraste, no prueban universalidad absoluta.

## 11. Dictamen de conformidad

La transición es conforme porque:

- existe un corte reproducible de ambos repositorios;
- la valoración cubre explícitamente los 15 requisitos G10 y las 44 solicitudes técnicas, indicando equivalencias, coberturas parciales y ampliaciones transversales;
- la conclusión sobrevivió a una adversarial explícita después de incorporar correcciones constitutivas;
- la pausa de Inmunología no destruye ni clausura su trabajo;
- el Lenguaje recibe un caso director sin subordinarse a una única especialidad;
- el siguiente frente comienza por deuda nuclear ya acreditada;
- R2, laboratorio, producción y uso clínico quedan fuera de este acto.

```text
TRANSICION_SECUENCIAL = CONFORME
FRENTE_ACTIVO_SIGUIENTE = LENGUAJE_DE_COMPUTACION
INMUNOLOGIA = PAUSA_CONTROLADA
CORTE_INMUNOLOGIA = 3bea6b714be3bd1330e6ca6bbbc228b0eb9c065d
OP_IMM_001 = PRIMER_CASO_DIRECTOR
OP_IMM_001 = NO_ESPECIFICACION_UNIVERSAL
PRIMER_TRABAJO_LENGUAJE = REVISION_Y_CIERRE_NUCLEAR
R2_ABIERTA_MATERIALMENTE = NO
LABORATORIO_ABIERTO = NO
LENGUAJE_MODIFICADO_POR_ESTA_ACTA = NO
MAIN_MODIFICADA_POR_ESTA_ACTA = NO
CELULA_MINIMA = SV(9,3)
CELULAS_MENORES = PROHIBIDAS
REGRESO_A_INMUNOLOGIA = TRAS_CANDIDATA_O_NECESIDAD_CONSTITUTIVA
SEGUNDO_FALSADOR = CIBERSEGURIDAD_INTELIGENTE_DIFERIDA
CORRECCIONES_CONSTITUTIVAS_04_09_2026 = INCORPORADAS
CONFORMIDAD_DEL_HEAD_CORREGIDO = OBLIGATORIA_ANTES_DE_FUSION
FUSION_AUTORIZADA_POR_ESTA_ACTA = NO
```

La rama `valoracion-op-imm-001-20260903` queda constituida como punto de partida documental para la continuidad inmediata del Lenguaje SV.


<a id="adenda-secuencia-20260906"></a>

## 12. Adenda rectora de secuencia y obligaciones de perfiles · 06/09/2026

**Registro:** RETP-2026-076. **Naturaleza:** actualización documental de continuidad; subordinada a los Pilares y al acta de perfiles. **Corte leído de main:** 605d900fc535aec4b0010820499b93e44d111f5c. **PR #61 leída:** ad8e8dd30930e35b75bf5f2fad78938d36233b78, abierta en borrador y no fusionada.

Esta adenda actualiza la secuencia de los apartados 4, 7, 9 y 10 sin borrar sus antecedentes. Reúne los relevos que estaban distribuidos entre esta acta, N0, las dos actas privadas de continuidad de R0/R1, el registro experimental 020 y el acta de perfiles RETP-075. Su objeto es que cada continuación sepa qué frente recibe el trabajo, qué debe entregar, qué necesita de los perfiles y qué impide avanzar. No constituye una nueva implementación, cierre de dominio ni selección de plataforma.

La secuencia actualizada se consulta aquí. Las actas históricas y la ficha de laboratorio remiten a esta sede; no mantienen otra secuencia normativa en paralelo. Los estados de los apartados anteriores pertenecen a sus fechas: en particular, su declaración de laboratorio no abierto no describe el laboratorio experimental posteriormente autorizado y documentado.

### 12.1. Fuentes y prelación

| Fuente | Qué aporta y cómo se recibe |
|---|---|
| [Pilares RETP-073](../../calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md) y [arquitectura RETP-072/073](../../calidad/ACTA_TECNICA_DE_ARQUITECTURA_DE_SOFTWARE_NUCLEO_FRONTERA_Y_HOST_SV_2026_09_04.md) | Invariantes, competencias, oráculos y separación núcleo–frontera–host; rigen toda la secuencia. |
| [Acta de perfiles RETP-075, especialmente §§6–12](../../calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md) | Suficiencia antes del cierre, tres contratos y ensamblajes distintos, prueba tecnológica previa y secuencia ya precisada con oráculos, puerta algebraica y frontera operacional. |
| [N0, corte de PR #61](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/ad8e8dd30930e35b75bf5f2fad78938d36233b78/docs/arquitectura/N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md) | Siete familias de hallazgos, K1/K1-T, F, G/H, I/J, K2 y M. Se conserva como documento de una rama candidata; no se presenta como ya integrado en main. |
| Actas privadas V2: [continuidad R0, §§5–8](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/20da3c781286bee1966c22cf1fc7a4755be35b9b/laboratorio-de-infraestructura-SV/registros/024-antecedentes-secuencia/ACTA_PRIVADA_CIERRE_INTEGRAL_R0_SV_2026-09-04_ACTUALIZADA_V2.md) y [continuidad R1, §§7–12](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/20da3c781286bee1966c22cf1fc7a4755be35b9b/laboratorio-de-infraestructura-SV/registros/024-antecedentes-secuencia/ACTA_PRIVADA_CIERRE_R1_Y_CONTINUIDAD_POSTERIOR_SV_2026-09-04_ACTUALIZADA_V2.md) | Antecedentes íntegros del 04/09, custodiados en el laboratorio privado: F-IF con seis testigos y condiciones de retorno a R2. Se actualiza su continuidad, no sus cierres históricos ni sus mediciones. |
| [Registro 020, §§6–9](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/020-PERFIL_TECNOLOGICO_Y_CONTINUIDAD_2026_09_06.md) y [matriz PT01–PT14](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/020-matriz-obligaciones.csv), privados | Inventario tecnológico existente, brechas y siguiente enlace experimental. La distribución por etapas de esta adenda reutiliza esos identificadores; no crea catorce requisitos distintos con nuevos nombres. |
| [Apertura R2](../../arquitectura/ACTA_TECNICA_APERTURA_R2_PERSISTENCIA_Y_CONTINUIDAD_MATERIAL_2026_08_25.md), [contrato R2-0, §§16–20](../../arquitectura/CONTRATO_R2_0_PERSISTENCIA_CONTINUIDAD_Y_RECUPERACION_2026_08_25.md), [entorno soberano, §§24–25](../../../ESPECIFICACION_ARQUITECTONICA_ENTORNO_EJECUCION_SOBERANO_SV_V0.md) y [adenda de alcance temporal](../../arquitectura/ADENDA_DE_ALCANCE_TEMPORAL_A_ESPECIFICACION_ENTORNO_SOBERANO_SV_V0_2026_08_22.md) | Descomposición ya constituida de R2, confianza de plataforma R3 e integración adversarial R4; cada fase conserva alcance, evidencia y habilitación propios. |

Los enlaces privados conservan sus restricciones de acceso. La documentación pública facilita la localización, sin publicar los expedientes privados ni modificar su régimen de custodia.

### 12.2. Nomenclatura que no se puede mezclar

| Identificador | Significado documentado | Consecuencia para el plan |
|---|---|---|
| N0 de la radiografía | Examen previo de objetos, invariantes y oráculos. | N0-01…N0-07 identifican hallazgos; no son siete versiones de IR ni una escalera N0→N1→N2. |
| N0, N1, N2, N3 y N4 de la IR | Definición, Estado, Resultado, Evolución y Uso, según [IR v0.2 §2](../../../IR_CANONICA_BIENFORMACION_SV_v0_2.md), bajo las sustituciones vigentes de [IR v0.3](../../../IR_CANONICA_BIENFORMACION_SV_v0_3.md). | Son niveles de representación, no fases de trabajo. Esta actualización no inventa fases llamadas N1 o N2. |
| K1 y K2 | Cierre intrínseco y decisiones posteriores de frontera, respectivamente. | K2 depende de los contratos contrastados IMM/CYB; no se adelanta por tener ensamblaje ES/EN. |
| F y F-IF | Contrato candidato de dominio y compuerta de interfaces dentro de F. | No son un nuevo cierre de los bloques históricos FFL-A…FFL-E. |
| R0…R4 | Fases del entorno de ejecución soberano. | R0/R1 permanecen cerrados en su alcance; R2 está abierta contractualmente, con realización pendiente; R3/R4 no iniciadas. No se identifican con los niveles N de la IR. |

## 13. Recorrido nuclear y relevos: salida exigida y obligación tecnológica

La tabla fija el orden de dependencias. Las siglas de frente conservan su significado documental. La columna tecnológica distribuye PT01–PT14, cuya matriz fuente describe sus evidencias y brechas; no afirma que estén implementadas. El perfil fuente y el de dominio permanecen presentes junto al soporte tecnológico durante todo el recorrido.

| Orden / frente | Unidad competente y producto de salida | Perfiles y soporte tecnológico que deben entrar en ese punto | Condición de paso |
|---|---|---|---|
| 1 · PR #61 / N0-01 | Lenguaje: recepción del cierre limitado de unicidad de Codomain, radiografía y deuda asociada. | PT01/PT04/PT14: identidad del candidato, diagnósticos, corpus y entorno realmente comprobados; conservación ES/EN y ensamblaje en su alcance. | Revisar la cabeza contra main vigente, reconciliar documentos y RETP sin perder 074 ni 075/076, verificar y promover por el régimen aplicable. Los controles del head antiguo no acreditan una reconciliación futura. |
| 2 · Reparación de oráculos | Lenguaje: comprobadores que detecten la infracción que afirman vigilar. | PT02/PT04/PT13: bytes e identidades cuando se afirme igualdad textual; pares JSON sin ocultar homónimos; distinguir rechazo esperado, fallo interno y señal del proceso; corpus negativo y paridad pertinente. | Demostrar detección con contraejemplos y controles válidos. No ajustar automáticamente el esperado a la salida actual. La reparación del comprobador precede a usarlo para cerrar defectos posteriores. |
| 3 · K1, desde N0-02 | Lenguaje: cierres intrínsecos incrementales y decisiones normativas propias; detalle en §14. | PT04/PT13/PT14: preservación de representación, diagnóstico y una única custodia Rust; requisitos de recursos y toolchain identificados sin mezclar su higiene con el parche semántico. | Cada invariante cierra con norma, realización y oráculos. Las carencias no representables se identifican y remiten a su sede; no se resuelven por analogía. |
| 4 · F | Lenguaje: contrato candidato que distinga identidad, versión, perímetro, operaciones, suficiencia y pérdida; recibe constituciones de dominio. | PT01/PT03/PT08/PT09/PT10/PT11/PT12: declarar qué exige cada operación al soporte, qué deberá representar el Lenguaje y qué impone la frontera. Capacidades, permisos y garantías siguen separados. | La representación requerida queda localizada y es comprobable o se declara su insuficiencia. El contrato no es universal ni convierte PT-SV-LOCAL/0.1-candidata en tipo de IR. |
| 5 · F-IF, dentro de F | Lenguaje sobre paquetes sintéticos constituidos: seis testigos IF-IMM-01…06 y matriz de pérdida por operación, §15. Una necesidad constitutiva vuelve a la unidad IMM conforme al §9. | PT01/PT02/PT03/PT04/PT07/PT10/PT12: identidad de artefacto, procedencia, estado, corrección, transporte y fallo; separar imagen/fichero, interpretación e informe. | No basta transportar un JSON. Cada distinción necesaria debe sobrevivir o generar insuficiencia explícita. No requiere conexión hospitalaria ni datos reales. |
| 6 · G/H, retorno acotado IMM | Inmunología: contrastar la candidata con OP-IMM-001, sus requisitos reconciliados y el contrato completo del perímetro/versionado que constituye. | PT03/PT04/PT07/PT10: el dominio define la información necesaria y el sentido de ausencia, no cobertura y corrección; recibe límites del soporte sin convertirlos en conocimiento clínico. | Dictamen de fidelidad y pérdidas del corte, con aceptación o devolución fundada. No exige completar los 32 universos ni cierra por sí solo todo el dominio de Inmunología. |
| 7 · Regreso al Lenguaje | Lenguaje: incorporar únicamente los cambios justificados por G/H y publicar candidata identificada para el segundo falsador. | PT01/PT03/PT04/PT14: cada cambio queda ligado a operación, contrato, versión y evidencia afectada; soporte y dominio no sustituyen validación lingüística. | Matriz de resolución de pérdidas; ninguna obligación imprescindible se oculta mediante campos opacos o adaptación del caso director. |
| 8 · I/J, segundo falsador CYB | Ciberseguridad inteligente: constituir su primer perímetro y operación falsadora; contrastar la candidata recibida sin heredar contenido IMM. | PT01/PT03/PT05/PT06/PT07/PT08/PT09/PT11/PT12: cuestionar identidad, permisos, repetición, fallos, límites y dependencias desde su dominio técnico. Si hay modelos de IA, son entradas capturadas y versionadas, no autoridad viva de la ejecución reproducible. | Entregar contrato CYB y contraejemplos. El contraste de un dominio de ciberseguridad no equivale a haber aprobado la seguridad material R3/R4 del soporte. |
| 9 · Regreso al Lenguaje tras CYB | Lenguaje: resolver o delimitar lo refutado y dejar una candidata genérica apta para la puerta siguiente. Recibir DFL-009 tras el primer universo CYB: evaluar el servicio nativo, Cloudflare/Workers u otras opciones conforme a la necesidad demostrada (§23). | PT03/PT04/PT12/PT14: cotejar IMM y CYB sin sumar permisos ni garantías; justificar qué queda en núcleo, contrato, soporte o exclusión. | No se mantiene una generalización refutada ni se particulariza el núcleo para salvar un caso. |
| 10 · Puerta algebraica Rust | Lenguaje: especificación y ejecución de las operaciones constituidas incluidas en el alcance, con sus fuentes soberanas, oráculos y correspondencia de resultados. | PT02/PT04/PT13: conteos, T(n), ThresholdOutcome, interpretación terminal tipada y EvalResult coherente, donde estén constituidos; paridad de destinos aplicables. Producción por Ternarizer condicionada a K1-T. | Compilar o serializar evaluate no acredita ejecución algebraica. No se inventan conjuntos de ternarización, reglas de conflicto ni operadores de composición ausentes. Toda operación que se ofrezca como ejecutable debe tener ejecución acreditada; las excluidas no se ofrecen. |
| 11 · Retorno de comprobación IMM/CYB | Contrastar sucesivamente los dos cortes de dominio con la realización algebraica candidata; Lenguaje recibe y registra cada retorno. | PT02/PT03/PT04/PT13: mismas identidades, significado, cobertura y salidas; que cambiar de soporte o perfil fuente no cambie el juicio. | Cada dominio confirma su propio corte o devuelve una pérdida. No se obtiene universalidad absoluta con dos dominios ni se cierran universos no examinados. |
| 12 · K2 | Lenguaje, con ambos contratos disponibles: versión, nombres, procedencia por objeto, orden, forma canónica, unidades vacías y ligaduras nominales aún pendientes. | PT01/PT02/PT03/PT12/PT14: distinguir ensamblaje multifuente, composición de dominios/agentes y ensamblaje tecnológico; identificar versiones y compatibilidad sin ampliación de autoridad. | Las decisiones deben poder representarse, preservarse y comprobarse. COMPOSE_TYPED_AGENTS sigue como decisión separada o exclusión expresa, no alias del ensamblador. |
| 13 · Contrato operacional de frontera | Lenguaje y sede de infraestructura: cerrar contrato de ABI/petición/respuesta, identidad, diagnóstico, recursos, cancelación, correlación, repetición, idempotencia y fallos. | PT01–PT14 en su aplicabilidad: recibir ensayos previos del laboratorio, completar sólo la brecha pertinente y separar declaración formal de imposición material. | Contrato comprobable, realizaciones identificadas y límites explícitos. Las garantías de persistencia o plataforma pendientes se adscriben a R2/R3/R4, sin declararlas satisfechas ni eliminar del contrato lo imprescindible. |
| 14 · Consolidación nuclear en alcance declarado | Lenguaje: acta de cierre que coteje cada operación admitida, semántica, IR, perfil fuente, dominio y soporte exigido con su evidencia. | PT01–PT14: inventario de capacidades incluidas/excluidas, versiones, compatibilidad, pruebas previas, dependencias aún no acreditadas y condición para futuras ampliaciones. | Una brecha imprescindible bloquea el cierre de ese alcance. Una capacidad futura excluida permanece fuera de la oferta. Consolidar el núcleo no acredita el sistema material completo. |

El regreso de una refutación al Lenguaje no reinicia todo el proyecto: identifica la obligación afectada, el cambio necesario y qué oráculos o retornos pierden cobertura. La revalidación se limita a ese impacto y a las puertas exigibles. No se mantienen frentes de dominio en paralelo.

## 14. K1 y K1-T: obligaciones que no esperan una plataforma

Se mantiene el orden intrínseco de N0 §7: N0-01 en su PR; relación CellSpec–OutputSemantics–Codomain (N0-02); ausencia de claves JSON homónimas y estabilidad de proyección (N0-03, dependiente de N0-02); referencia real de Horizon.architecture (N0-04); unicidad de CoupledSpec.bridges bajo BridgeSet; decisión sobre multiplicidad de Horizon.events; mínimo estructural de Domain.parameters y multiplicidad de parameter_id; concordancia de deuda, diagnóstico y corpus.

N0-05 y N0-07 conservan su destino K2. N0-06 conserva su división: las decisiones mínimas pertenecen a K1; la ligadura nominal/numeral, versión, cobertura y campos aún no interpretados requieren contrato y representación propios. No se pretende que una lista, una cadena nominal o una igualdad de cardinalidades resuelva esa ausencia.

K1-T es una dependencia previa de cualquier ruta productiva mediante Ternarizer: cobertura, disjunción y ausencia de solapamiento de B_0, B_1 y B_U conforme a E107/J1.5. Debe cerrarse la obligación o impedirse expresamente esa ruta. No se atribuye ahora un conjunto observacional al dominio ni se declara concluida esta puerta.

El perfil tecnológico recibe aquí lo necesario para conservar identidades y diagnósticos y reproducir los resultados. La higiene de toolchain, dependencias y ejecución local se delimita en un cambio propio cuando proceda. Una actualización de plataforma no debe contaminar la atribución causal de un cierre semántico.

## 15. F-IF y aprovechamiento obligatorio del laboratorio

### 15.1. Los seis testigos existentes

Se conservan los identificadores y familias de la continuidad privada R1 §7; son requisitos de una campaña pendiente, no resultados nuevos de esta adenda:

| Testigo | Familia y distinción imprescindible |
|---|---|
| IF-IMM-01 | Analítica: petición, muestra, observación, unidad, rango/criterio, estado, informe y corrección. |
| IF-IMM-02 | Citometría: fichero instrumental, panel/configuración, controles, observación derivada, informe e interpretación. |
| IF-IMM-03 | Médula ósea: muestra, lugar/procedimiento, procesamiento, informe, anexos e imagen digital cuando exista. |
| IF-IMM-04 | Terapia oncológica: intención, orden, pauta, administración efectiva, modificación/suspensión y resultado. |
| IF-IMM-05 | Ingreso/hospital de día: episodio, encuentro, ubicación, servicio, inicio/fin y variación organizativa. |
| IF-IMM-06 | Historia longitudinal: orden causal, validez temporal, estados, enmiendas, procedencia y relación entre episodios. |

La familia de interfaz no amplía automáticamente el perímetro clínico de OP-IMM-001. Un testigo sólo sirve a una operación consumidora y una constitución declaradas; si falta esa constitución, se devuelve la pregunta al dominio. El banco no acredita tratamiento oncológico, interpretación de imagen ni uso asistencial por disponer de un formato de transporte.

Cada testigo conserva positivo y negativo, fuente/versión/perfil, identidades, W_j, captura φ_j, observación O_j u O_j^⊥, admisibilidad r_j, transducción τ_j, operación Q, información conservada/perdida y oráculo. La [suficiencia representacional por operación](../../arquitectura/CONTRATO_MINIMO_DE_SUFIENCIA_REPRESENTACIONAL_POR_OPERACION_PARA_EL_LENGUAJE_SV_2026_08_21.md) rige el juicio. La falta de observación, el fallo técnico, la no cobertura y U no se intercambian. No se fuerza una extensión de IR por el nombre de un estándar.

### 15.2. Qué se reutiliza y cuándo

| Activo existente | Recepción en esta secuencia | Límite que se conserva |
|---|---|---|
| [Prototipo 0.2 y registro 012, privados](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/012-CORRECCION_VERIFICADA_2026_09_06.md) | F/frontera y preparación posterior de R2: transporte, control, cancelación, recuperación y oráculos ya medidos. | Efecto SQLite sintético; no ejecución de SV/R1 ni cierre R2. |
| [Registro 016, privado](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/016-VIGILANCIA_Y_COSTE_2026_09_06.md) | Oráculos/frontera: separar validez de identidad y medir coste; conservar corrección de la API pública trazada R1. | Compilación real y ataques sintéticos; doble validación no demostró beneficio en el ataque ensayado; no ejercicio de R1. |
| [Registro 018, privado](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/018-COMPARACION_DOTNET_FFI_WASM_2026_09_06.md) | Frontera/PT02/PT08/PT09/PT11/PT13: comparación .NET con FFI nativa y Wasmtime/WASM sobre la misma fuente Rust. | 79 programas, EN monofuente, compilación/proyección; no ensamblaje, álgebra completa, R1 ni protección frente a host comprometido. No se transforma retrospectivamente en corpus de 80 de PR61. |
| [Correspondencia CS01–CS09, privada](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/CORRESPONDENCIA_PROTOCOLO_Y_CONTRATOS_SV_2026_09_06.md) y adenda 016 | Frontera: reutilizar pares y criterios especificados. Enlace acotado disponible para preparar: CS01, CS02, CS04 y CS05; CS03 espera contratos constituidos. | Su ejecución de enlace sigue pendiente. Requiere fixture legítima por API pública trazada y un acople que impida la vía alternativa hacia el efecto. |

La exigencia de prueba previa se aplica antes de promover cada realización tecnológica dentro de su alcance. No impone repetir una campaña porque cambie un enlace o el nombre de una ficha. Cada recepción identifica requisito, perfil y versión, artefacto, configuración, prueba, resultado, coste, alcance y límite. Si cambia una dependencia causalmente relevante se reevalúa la evidencia afectada. El laboratorio no constituye por sí mismo el contrato del dominio ni convierte una prueba local en garantía de plataforma.

La [Documentación de laboratorios](https://juantoniolloretegea.github.io/SVcustos-dataset/laboratorio-de-infraestructura-SV/documentacion/) y la [ficha de perfiles](https://juantoniolloretegea.github.io/SVcustos-dataset/laboratorio-de-infraestructura-SV/documentacion/perfil-tecnologico.html) permiten recorrer los activos. Un enlace experimental disponible no cambia el frente prioritario: tras recibir N0-01 corresponden los oráculos y K1, no una sustitución general del trabajo nuclear por bus o host.

## 16. Después de consolidar el núcleo: R2, R3, R4 y agentes

### 16.1. Retorno a R2 sin borrar su apertura contractual

R2 conserva su apertura y el contrato R2-0 existente. Su reanudación material requiere el corte nuclear consolidado, la salida de K2, perfiles/contratos con identidades suficientes y el régimen de autorización aplicable. Se revisará la descomposición recibida contra los requisitos IMM/CYB y tecnológicos; no se empieza por inventar otro contrato R2-0.

| Corte ya definido | Obligación y recepción tecnológica |
|---|---|
| R2-0 | Reconciliar contrato de persistencia, fronteras de estado y perfiles con el corte recibido. PT01/PT03/PT10/PT14. |
| R2-1 | AStore, PDep, cobertura y retención: identidad del estado y de sus fuentes; una vista no adquiere autoridad. PT01/PT03/PT10. |
| R2-2 | Continuidad vigente, retroceso, clonación y bifurcación: la recuperación sintética 0.2 no demuestra estas garantías. PT08/PT10. |
| R2-3 | Revocación persistente y recuperación gobernada: declarar dependencias y bloquear recuperación circular. PT05/PT07/PT10; independencia material pendiente se remite a R3. |
| R2-4 | Persistencia decisión–efecto, reconciliación y consumo: separar request_id, ExerciseRef, reentrega y nuevo ejercicio; resultado material indeterminado no equivale a U ni habilita reintento. PT05/PT06/PT10. |
| R2-5 | Presupuestos y tiempo sólo cuando sean aplicables: requisitos y fuente explícitos, con medidas en entorno identificado. PT07/PT09. No se introduce reloj semántico universal. |
| R2-6 | Regresión integral, adversarial y cierre del alcance: destinos pertinentes, conservación R0/R1 y límites materiales declarados. PT02/PT04/PT10/PT12/PT14. |

### 16.2. R3 y R4 conservan objeto propio

R3 recibe confianza de plataforma: construcción, artefacto realmente cargado, raíz, actualización, atestación, aislamiento y dependencias capaces de falsificar una garantía. Incorpora PT01/PT08/PT09/PT11/PT14 y los requisitos pendientes remitidos por R2. La comparación FFI/WASM y el inventario de dependencias son antecedentes; no demuestran estas propiedades frente a todos los actores. No se inventa aquí una descomposición R3-0… ni una plataforma obligatoria.

R4 recibe la realización integrada correspondiente a una identidad exacta: fallos compuestos, vías materiales, regresión, reducción causal y pruebas adversariales del sistema. Contrasta PT05–PT12 junto con identidad, diagnósticos, semántica y evolución de PT01–PT04/PT13/PT14. Reunir componentes que pasaron pruebas separadas no demuestra el conjunto. La doble garantía mantiene las condiciones y límites del entorno soberano §25.

### 16.3. Cierre de dominio y constitución de agente

G/H y el posterior retorno de comprobación deben cerrar o devolver **su encargo y contrato en el perímetro declarado**. La clausura integral de todo Inmunología no tiene fecha ni paso obligatorio adicional constituido entre I/J y R2. No se introduce esa dependencia por recuerdo aproximado. Las necesidades clínicas que sólo pueda decidir el dominio vuelven a su unidad competente; un pendiente indispensable para la operación incluida bloquea ese alcance del Lenguaje.

Un agente requiere previamente constitución de dominio recibida y contrato explícito de cobertura, operaciones, permisos y soporte. Su investigación acotada no equivale a aptitud productiva; ésta depende además de persistencia, plataforma e integración aplicables. M y cualquier composición tipada de agentes conservan su decisión propia, posterior a los contratos necesarios o excluida expresamente. La compatibilidad de tres perfiles no crea un agente ni transmite autoridad.

## 17. Punto exacto de reanudación y control de relevo

En el corte de esta adenda, PR #61 continúa pendiente de integración. N0-01 está acreditado en su candidato; el resto de K1, F/F-IF, retornos, puerta algebraica, K2 y consolidación no se dan por cerrados. La actualización documental RETP-075 y la presente RETP-076 no realizan ninguno de esos trabajos.

El siguiente expediente es **recibir y reconciliar PR #61 exclusivamente como N0-01**, conservando RETP-074 en su identidad y las adiciones posteriores de main. La descripción histórica de la PR que la declaraba sincronizada con main@230a205… no acredita sincronización con main@605d900… ni con la integración de esta adenda. Tras su recepción gobernada se reparan los oráculos y se continúa K1 por N0-02. Esta actualización no fusiona PR #61 ni inicia esos parches.

Cada relevo deberá dejar un registro breve con: corte de entrada y salida; frente/operación; perfil fuente, dominio y soporte con versiones; necesidad de semántica/IR; obligación PT aplicable; evidencia reutilizada o nueva; pérdida y límite; decisión de aceptación/devolución y siguiente paso. Una falta de aplicación se justifica; un campo vacío no significa conformidad. Así se alimenta al núcleo antes de cerrarlo, sin añadir una segunda cola de fases tecnológicas ni olvidar sus condiciones materiales.

```text
SECUENCIA = ACTUALIZADA_DOCUMENTALMENTE
PR61_N0_01 = CANDIDATO_PENDIENTE_DE_INTEGRACION
SIGUIENTE_TRAS_RECEPCION_N0_01 = ORACULOS_Y_K1_DESDE_N0_02
N1_N2 = NIVELES_IR_NO_NUEVAS_FASES
G_H = RETORNO_INMUNOLOGICO_ACOTADO
CIERRE_INMUNOLOGIA_INTEGRAL_COMO_PRERREQUISITO = NO
PERFILES_TECNOLOGICOS = OBLIGACIONES_DISTRIBUIDAS_PT01_PT14
PRUEBA_TECNOLOGICA_PREVIA = OBLIGATORIA_PARA_PROMOVER_SU_ALCANCE
NUCLEO = NO_CONSOLIDADO
R2 = ABIERTA_CONTRACTUALMENTE_SIN_CIERRE_MATERIAL
R3_R4 = NO_INICIADOS
REALIZACION_MODIFICADA_POR_ESTA_ADENDA = NO
```

<a id="recepcion-n0-01-20260906"></a>

## 18. Recepción de PR #61 / N0-01 y relevo a oráculos · 06/09/2026

**Registro:** RETP-2026-077. **Expediente de promoción y evidencia del candidato exacto:** [PR #61](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/61), sus [commits](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/61/commits) y [controles](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/61/checks). Este apartado desarrolla exclusivamente la fila 1; conserva la tabla y los antecedentes de §§12–17.

| Campo de relevo | Recepción y límite comprobable |
|---|---|
| Cortes de entrada | Candidata N0-01 `ad8e8dd30930e35b75bf5f2fad78938d36233b78`; main `981159d6428197d1ad1d748649f1d8f690b2f588`; base común `230a205b08f4c54c9c8d9c1c7ad35b2f6ddbbfc4`. Se incorpora main mediante reconciliación, sin reescribir el historial de la rama. |
| Fuentes rectoras leídas | AGENTS, Pilares RETP-073, perfiles RETP-075, esta acta completa con RETP-076, radiografía y acta N0-01, y asientos directamente aplicables. La recepción conserva íntegros 074, 075 y 076, tanto en CSV como en Markdown. |
| Frente y necesidad de semántica/IR | Lenguaje, compilación/validación de `Codomain`: finitud explícita, no vacío y unicidad. La representación ya existe; se impone su invariante con E004. No se añade un tipo de perfil, dominio o soporte a la IR. Gramática efectiva 0.2, IR 0.3 y serializador de referencia 0.1.0 conservan su alcance. |
| Perfil fuente | SVP-EN y SVP-ES vigentes, sin nueva versión ni ampliación. Rust recibe pruebas positivas/negativas y ensamblaje con unidad inválida. La referencia Python y el corpus común ejercen su superficie existente; no se le atribuye ensamblaje bilingüe. |
| Dominio | No aplica contenido de un dominio de conocimiento: el invariante es intrínseco al Lenguaje. IMM y CYB conservan sus relevos posteriores. |
| Soporte y PT aplicables | PT01: identidad de fuentes, candidato y controles. PT04: E004 observable, diferenciando el diagnóstico textual Rust de un diagnóstico estructurado aún pendiente. PT14: corpus, versiones y entorno declarados. Rust 1.98.0 local; los flujos CI identifican sus propios compiladores y destinos nativo, WASI y navegador. No se promueve PT-SV-LOCAL como plataforma productiva. |
| Evidencia recibida y nueva | Se conservan las pruebas N0-01 de la candidata y los registros históricos; se repiten las comprobaciones sobre la reconciliación. La [adenda del acta N0-01](../../arquitectura/ACTA_TECNICA_N0_01_UNICIDAD_DE_CODOMAIN_2026_09_04.md#recepcion-20260906) precisa corpus, identidad literal y alcance diagnóstico. Los cuatro flujos exigibles deben acreditar la nueva cabeza: los resultados del head antiguo no se transfieren. |
| Laboratorio | Se recibe el inventario y sus límites de §15.1. El [registro 018](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/018-COMPARACION_DOTNET_FFI_WASM_2026_09_06.md) conserva sus 79 programas EN; no acredita retrospectivamente los 80 de N0-01, ES ni ensamblaje. Esta recepción no repite esa campaña ni selecciona FFI/.NET/Wasmtime. |
| Decisión y salida | Recepción limitada de unicidad de `Codomain`, radiografía y deuda. Es efectiva en main cuando PR #61 conste fusionada, después de verificar su nueva cabeza y la base vigente. El expediente identifica cabeza, controles y commit de integración; mientras esté sólo en una rama sigue siendo candidata. |
| Siguiente paso | Fila 2: reparación de oráculos, PT02/PT04/PT13, con contraejemplos y controles válidos; después K1 desde N0-02. La recepción no acredita esa reparación ni cierra N0-02, K1-T, F, K2, álgebra, núcleo o R2/R3/R4. |

El éxito del corpus demuestra su alcance observado. Los ejecutores actuales normalizan JSON y algunos aceptan cualquier salida de proceso no nula como rechazo; esas limitaciones permanecen localizadas para la fila 2. Esta recepción comprueba por separado la conservación literal frente al corte de entrada, y para el negativo nuevo exige `rc=1`, E004 y ausencia de IR emitida. No convierte equivalencia normalizada en igualdad de bytes entre emisores ni en cobertura diagnóstica completa.


<a id="oraculos-20260906"></a>

## 19. Reparación de oráculos y relevo a K1 · 06/09/2026

**Registro:** RETP-2026-078. **Corte de entrada:** main `91dc5a3c3b2298ef3fd1b2eefe607f379643e076`, con PR #61 integrada. Este apartado desarrolla exclusivamente la fila 2 y conserva las dependencias de la tabla.

| Campo | Resultado, evidencia y límite |
|---|---|
| Producto | [Comprobadores reparados](../../calidad/ACTA_TECNICA_REPARACION_DE_ORACULOS_2026_09_06.md): pares JSON ordenados sin ocultar homónimos, tipos o precisión; bytes sin conversión de saltos; rechazo con retorno 1, ausencia de IR e identidad diagnóstica por vía. |
| Semántica/IR | No se amplían ni se corrigen en esta fila. La Gramática 0.2, IR 0.3, referencia y núcleo conservan su realización. El observador deja de ocultar pérdidas. |
| Perfiles | PT02/PT04/PT13, con identidad y entorno PT01/PT14. Corpus común EN; controles ES/EN y ensamblaje en sus suites existentes. No aplica contenido de dominio IMM/CYB ni se promueve un soporte tecnológico. |
| Evidencia | 80 casos existentes preservados; 16 pruebas del observador; un control conforme y tres divergencias detectadas. Paridad nativa y destinos WASM por los flujos del candidato exacto. El acta precisa qué igualdades son estructurales y cuáles literales. |
| Laboratorio | Se conserva la distinción validez/identidad de 016 y el alcance de 018. No se repite esa campaña ni se la convierte retrospectivamente en una prueba de los nuevos oráculos. |
| Deuda | `semantics_duplicate` se devuelve a N0-02/N0-03; CRLF y el literal CRLF quedan bajo DFL-008; DFL-001 conserva la concordancia diagnóstica incompleta. Su detección no acredita reparación. |
| Decisión | Reparación efectiva al integrarse su expediente con Conformidad SVP, R0 Rust, R0-8 y R0 WASM correctos en la candidata exacta. La evidencia de sensibilidad se conserva separada de la conformidad. |
| Relevo | Fila 3: K1 desde N0-02, relación total y sin claves repetidas entre CellSpec, OutputSemantics y Codomain. No se anticipan F, K2, álgebra, cierre nuclear o R2/R3/R4. |

<a id="cierre-n0-02-20260906"></a>

## 20. Cierre incremental N0-02 y relevo a N0-03 · 06/09/2026

**Registro:** RETP-2026-079. **Entrada:** main `ed61af2fb80641866356a7138cc87763eab005d9`, con PR #65 integrada. Se desarrolla exclusivamente N0-02 dentro de la fila 3; la tabla y sus dependencias permanecen vigentes.

| Campo | Decisión, evidencia y límite |
|---|---|
| Producto | [Acta N0-02](../../arquitectura/ACTA_TECNICA_N0_02_TOTALIDAD_Y_UNICIDAD_DE_OUTPUT_SEMANTICS_2026_09_06.md): cada `CellSpec` enlaza exactamente una interpretación por miembro de su `Codomain`, sin claves ajenas ni repetidas. |
| Semántica/IR | J-K1 en IR v0.3 §6.2 y E115 en catálogo efectivo. La representación ya contiene la relación; se refuerza la validación sin cambiar Gramática 0.2, esquema IR 0.3 ni serializador 0.1.0. |
| Perfil fuente y dominio | SVP-ES/SVP-EN y ensamblaje comprobados en Rust; Python en su superficie EN. Textos compartidos y orden independiente preservados. No aplica conocimiento IMM/CYB a este invariante intrínseco. |
| Soporte y PT | PT04: rechazo E115 con identidad de la relación y ausencia de IR. PT13: paridad pertinente. PT14: corpus y entorno. PT01/PT02: identidades y conservación literal por emisor frente a la base; no equivalencia literal universal entre emisores. |
| Evidencia | Corpus 85 = 13 válidos + 72 inválidos; cinco pruebas Python y seis Rust N0-02. Doce esperados anteriores intactos; cuatro negativos antes admitidos ahora rechazados en ambas vías. El banco conserva control, cierre relacional y tres divergencias abiertas detectadas. |
| Laboratorio | Se recibe la distinción validez/identidad de 016 y se conserva el alcance de 018, 79 programas EN. No se promueve un nuevo perfil ni se atribuyen retrospectivamente a esos ensayos los 85 casos o el ensamblaje. La prueba previa y el enlace material mantienen sus puertas. |
| Deuda | N0-03: proyección global, con testigo de semántica duplicada sin `CellSpec`. DFL-008: CRLF. DFL-001: concordancia diagnóstica general. El resto de K1/K1-T permanece pendiente. |
| Decisión y salida | Cierre limitado efectivo cuando el expediente enlazado por el acta se integre con Conformidad SVP, R0 Rust, R0-8 y R0 WASM correctos sobre su candidata exacta. Las identidades de cabeza, controles e integración pertenecen a ese expediente. |
| Siguiente paso | Continuar fila 3 por N0-03; no saltar a F, dominios, álgebra, K2, frontera o consolidación. |

<a id="cierre-n0-03-20260906"></a>

## 21. Cierre incremental N0-03 y relevo a N0-04 · 06/09/2026

**Registro:** RETP-2026-080. **Entrada:** main `016b2f4d1f896dcbd4e8d177e8db4e736e2cd571`, con PR #66 integrada. Se continúa la fila 3 dentro de K1, conservando la tabla y sus dependencias.

| Campo | Decisión, evidencia y límite |
|---|---|
| Producto | [Acta N0-03](../../arquitectura/ACTA_TECNICA_N0_03_UNICIDAD_DE_MIEMBROS_Y_ESTABILIDAD_DE_PROYECCION_JSON_2026_09_06.md): unicidad de toda `OutputSemantics`, incluso sin celda, y recorrido JSON sin pérdida de miembros, tipos, orden ni tokens numéricos. |
| Semántica/IR | J-J0 en IR v0.3 §6.3. La estructura existente basta; no se añaden tipos ni importación de IR. E115 conserva identidad y recibe la comprobación no enlazada; su texto general Python se precisa. |
| Perfiles | SVP-ES/SVP-EN y ensamblaje comprobados en Rust; referencia EN Python. No aplica contenido IMM/CYB a este invariante intrínseco. PT04/PT13/PT14 y conservación PT01/PT02 reciben diagnósticos, paridad, corpus e identidades. |
| Evidencia | 88 = 14 válidos + 74 inválidos; N0-03 Python 3/3, Rust 5/5; observador 19/19. Trece esperados intactos y trece salidas por emisor conservadas literalmente frente a la base. Los rechazos previos conservan identidad; sólo cambia el texto general E115 en cuatro casos Python. |
| Sensibilidad | Las cinco fuentes anteriores se conservan. Esquema v3: un control, dos rechazos E115 y dos divergencias CRLF abiertas. No se suman las sondas a conformidad. |
| Laboratorio | Se reutilizan oráculos y distinción validez/identidad de 016; 018 conserva sus 79 programas y condiciones. No se repite ni se amplía retrospectivamente esa campaña, ni se promueve otra plataforma. |
| Decisión y salida | Cierre limitado efectivo al integrar el expediente enlazado con Conformidad SVP, R0 Rust, R0-8 y R0 WASM correctos sobre su candidata exacta; cabeza, árbol, ejecuciones e integración quedan identificados en ese expediente. |
| Deuda y siguiente paso | N0-04: referencia real de `Horizon.architecture` a `CompositionGraph`, según la radiografía. Se conservan DFL-001/008 y el resto de K1/K1-T. No se anticipan F, dominios, álgebra, K2, frontera o consolidación. |

<a id="cierre-n0-04-20260907"></a>

## 22. Cierre incremental N0-04 y relevo a BridgeSet · 07/09/2026

**Registro:** RETP-2026-081. **Entrada:** main `f9aa3ebada0db222bb9d196f94a9b42dac185f97`, PR #67 integrada. Continúa la fila 3; la tabla rectora y sus dependencias se conservan.

| Campo | Decisión, evidencia y límite |
|---|---|
| Producto y fuente | [Acta N0-04](../../arquitectura/ACTA_TECNICA_N0_04_REFERENCIA_REAL_DE_ARQUITECTURA_DEL_HORIZONTE_2026_09_07.md) y J-H0, IR v0.3 §6.4: `ArchitectureId` del horizonte resuelve un `CompositionGraph` declarado y bien formado. La precisión literal de IR v0.2 queda explícita. |
| Realización | Resolutores existentes Python/Rust sobre el programa completo, después de las validaciones previas. La igualdad Agent–Domain–Horizon queda ligada a un referente real. Sin nuevos campos, códigos ni cambios de emisor o versión. |
| Perfiles y soporte | EN Python; ES/EN y ensamblaje mixto Rust, referencias adelantadas y ambos órdenes. PT04/PT13/PT14 y PT01/PT02. No aplica contenido de dominio IMM/CYB ni se promueve un soporte nuevo. |
| Evidencia | 91/91, Python N0-04 4/4 y Rust 5/5; regresiones previas correctas. Trece positivos y 74 negativos conservados literalmente por vía. El positivo histórico `transition_data_events` tenía una referencia colgante: se documentan rechazo de sus bytes originales, corrección declarativa y cambio explícito del esperado. |
| Laboratorio y deuda | 016/018 mantienen sus alcances, con prueba tecnológica previa en su puerta. Cinco sondas de sensibilidad conservadas: control, dos rechazos E115 y dos divergencias CRLF abiertas. DFL-001/008 y resto de K1/K1-T no se cierran. |
| Decisión y salida | Cierre efectivo tras integrar la candidata exacta con los cuatro flujos correctos; identidad de cabeza, árbol, base, ejecuciones y promoción en el expediente enlazado. |
| Siguiente paso | Unicidad de `CoupledSpec.bridges` bajo `BridgeSet`, cotejando su norma; después multiplicidad de `Horizon.events` y mínimo estructural de Domain. N0-05/N0-07 siguen en K2. No se adelantan F, dominios, álgebra, frontera, consolidación ni R2/R3/R4. |

<a id="retirada-python-y-deuda-20260907"></a>
## 23. Retirada Python y evaluación tecnológica diferida · 07/09/2026

**RETP-082; entrada a09b9ef, PR #68 integrada.** La retirada aprobada del compilador Python se documenta en la [adenda de oráculos §9](../../calidad/ACTA_TECNICA_REPARACION_DE_ORACULOS_2026_09_06.md#retirada-python-20260907): conserva corpus, esperados, testigos y exigencia de conformidad de SV/paridad nativa-WASM sobre cada candidata. No convierte la implementación Rust en fuente de autoridad de la DSL.

La [deuda DFL-009](../../calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md#dfl-009--servicio-remoto-de-sv-con-realización-nativa-evaluación-diferida) se recibe **en la fila 9, al retornar del primer universo de Ciberseguridad Inteligente después de I/J**. Se valorará entonces un servicio público de SV con ejecutable nativo identificado, incluido Cloudflare/Workers u otros, reutilizando lo demostrado en el laboratorio. No exige instalación Rust/Cargo en el equipo del visitante ni abre ahora una plataforma; debe comprobarse qué destino ejecuta realmente cada opción.

**Relevo:** sigue la fila 3, K1: unicidad de CoupledSpec.bridges bajo BridgeSet, después las decisiones de Horizon.events y Domain en el orden de §14. La pérdida por campos opcionales repetidos se conserva como DFL-010 dentro de K1. F (fila 4) continúa pendiente de la salida de K1 y de las condiciones expresas de K1-T; la deuda tecnológica diferida no desplaza esas obligaciones.

<a id="relevo-bridgeset-20260907"></a>
## 24. Cierre de BridgeSet y siguiente obligación K1 · 07/09/2026

**RETP-083.** La [radiografía §15](../../arquitectura/N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md#cierre-bridgeset-20260907) recibe el juicio de unicidad J-B0, ya exigido por `BridgeSet`/J1.2. Rechaza posiciones repetidas sin deduplicar ni ordenar; conserva vacío, rango y los 14 esperados anteriores. Se exige conformidad 92/92 y paridad de los 18 testigos ES/EN/ensamblaje en nativo, WASI y navegador antes de promover la candidata. La retirada Python y DFL-009 permanecen en el commit y registro RETP-082 separados.

**Punto vigente después de la promoción:** fila 3, K1, decisión sobre multiplicidad de `Horizon.events`. Continúan luego las obligaciones de `Domain`, concordancia diagnóstica y K1-T en el orden de §14; DFL-010 debe resolverse o delimitarse expresamente antes de F. No se da por habilitada la fila 4. La fila 9 recibe la evaluación tecnológica diferida tras I/J según §23.

<a id="relevo-horizon-events-20260907"></a>
## 25. Tipos del horizonte y relevo al mínimo de Domain · 07/09/2026

**RETP-084; entrada main 16232b6, PR #69 integrada.** La [radiografía §16](../../arquitectura/N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md#horizon-events-20260907) fija J-H1 por cotejo del Documento III y la IR. El horizonte declara tipos: repetir una identidad no registra otra instancia. Se rechaza la repetición sin normalizar; se conservan orden, referencias y recurrencia entre horizontes/datos distintos. El testigo histórico N0-04 afectado queda identificado y sustituido explícitamente.

Conformidad 93/93 y los 20 testigos ES/EN/ensamblaje se exigen en nativo, WASI y navegador antes de promover la candidata; se conservan corpus previo, esperados y bancos anteriores. No se cambia el contrato de TransitionData ni se acredita una nueva plataforma.

**Punto vigente después de la promoción:** fila 3/K1, mínimo estructural de `Domain.parameters` y multiplicidad de `parameter_id` (§14). Continúan concordancia diagnóstica, DFL-010 y condiciones de K1-T; F permanece pendiente. La fila 9 conserva la recepción de DFL-009 tras I/J.

**Recepción documental RETP-085, 07/09/2026:** el [DOI del Documento III](https://doi.org/10.21428/39829d0b.bb86c65d) sustituye el enlace a release1. La [radiografía §16.3](../../arquitectura/N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md#reconciliacion-documento-iii-20260907) conserva el cotejo de la actualización en el repositorio del autor, la corrección notacional y el límite de acceso a la página editorial. El dictamen J-H1 y el relevo K1/Domain se mantienen en ese alcance.

<a id="domain-relevo-20260907"></a>
## 26. Unicidad nominal de Domain y recepción de ligaduras pendientes · 07/09/2026

**RETP-086; entrada:** main `4536bd051cc58bf183b8b9efa5a5f818f18090f9`, PR #71. La [radiografía §17](../../arquitectura/N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md#domain-parameters-20260907) conserva el cotejo de los Documentos IV/V, la norma J-D0, los testigos y los límites. Se rechazan nombres repetidos dentro de `Domain.parameters`, sin deduplicar ni alterar el orden. Se conservan versiones y juicios anteriores.

**Puerta de promoción:** 94 casos de conformidad, tres pruebas de integración y 24 testigos ES/EN/ensamblaje; cuatro flujos correctos sobre la candidata exacta, con ejecución nativa, WASI y navegador. Diez testigos preservan representaciones con ligaduras pendientes; su aceptación no acredita la constitución completa del dominio. PT01/PT02/PT04/PT13/PT14 conservan identidad, diagnóstico, evidencia y entorno. No se amplía el laboratorio histórico.

**Relevo tras promoción:** continúa fila 3/K1 por concordancia diagnóstica/deuda/corpus, DFL-010 y condiciones de K1-T. J-D0 no cierra N0-06 entero: el mínimo de `parameters`, la correspondencia nombre–instancia `(C,j)`–cadena y la multiplicidad numérica quedan en DFL-005, con recepción obligatoria en F y condición bloqueante para operaciones que las requieran. No se deducen correspondencias de posiciones o cardinalidades ni se difiere su suficiencia únicamente a K2. F permanece pendiente de la salida K1/K1-T. DFL-009 conserva la fila 9, retorno del primer universo CYB, para valorar servicio nativo y Cloudflare/Workers u otros. El acceso editorial queda aplazado por decisión humana.
