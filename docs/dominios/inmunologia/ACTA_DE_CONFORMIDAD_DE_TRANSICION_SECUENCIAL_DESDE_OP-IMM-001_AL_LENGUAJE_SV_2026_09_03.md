# Acta de conformidad de transición secuencial desde OP-IMM-001 al Lenguaje SV

> **Continuidad vigente · 06/09/2026:** la [adenda rectora de secuencia (§§12–17)](#adenda-secuencia-20260906) actualiza el recorrido desde PR #61 hasta la consolidación nuclear y su continuación material. Distribuye las obligaciones de los perfiles tecnológicos y conserva los estados históricos de esta acta. Registro RETP-076.

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
| 9 · Regreso al Lenguaje tras CYB | Lenguaje: resolver o delimitar lo refutado y dejar una candidata genérica apta para la puerta siguiente. | PT03/PT04/PT12/PT14: cotejar IMM y CYB sin sumar permisos ni garantías; justificar qué queda en núcleo, contrato, soporte o exclusión. | No se mantiene una generalización refutada ni se particulariza el núcleo para salvar un caso. |
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
