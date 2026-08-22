# Acta arquitectónica sobre el estatuto del laboratorio ejecutable, el backend soberano y la doble garantía de la realización SV

**Fecha:** 22/08/2026  
**Estado:** decisión arquitectónica  
**Ámbito:** `SV-lenguaje-de-computacion`

## 1. Objeto

Esta acta fija la separación entre laboratorio ejecutable, realización soberana y acreditación final de garantías en el Lenguaje SV.

Su finalidad es evitar que una materialización de referencia, una batería local de pruebas o la elección de un lenguaje de implementación se interpreten como sustitutos de las garantías arquitectónicas y materiales exigidas por SEC.0.

La presente decisión no modifica la semántica del Lenguaje SV, la gramática, la IR canónica, el catálogo diagnóstico ni los contratos SEC.0-A, SEC.0-D, SEC.0-M, SEC.0-X y SEC.0-T.

## 2. Antecedentes vigentes

Esta acta se interpreta de forma compatible con:

- `MANIFIESTO_DE_ARQUITECTURA_DERECHOS_OBLIGACIONES_GARANTIAS_Y_FUNDAMENTOS_DEL_SISTEMA_VECTORIAL_SV_V1.md`, que sitúa los laboratorios como instrumentos de verificación subordinados al corpus y separa la prueba de laboratorio de la producción;
- `OBJETIVO_RUST_0_BACKEND_SOBERANO.md`, que fija Rust como objetivo principal del backend soberano y mantiene Python como etapa frontal de referencia mientras no se abra la realización soberana;
- los contratos SEC.0-A/D/M/X/T, que determinan las obligaciones de autoridad, fallo cerrado, memoria y continuidad, ejecución material y comprobación adversarial.

Ningún contenido de esta acta rebaja esas obligaciones.

## 3. Estatuto del laboratorio ejecutable

El laboratorio ejecutable no constituye una realización soberana del Lenguaje SV.

Su función admisible queda limitada a producir evidencia reutilizable sobre los contratos, principalmente mediante:

1. casos mínimos que representen consecuencias ejecutables de los contratos;
2. vectores adversariales que puedan volver a ejercerse contra realizaciones posteriores;
3. criterios esperados independientes de la implementación sometida a prueba;
4. regresiones permanentes de defectos ya observados;
5. comprobaciones de que una traducción técnica no altera el significado contractual.

El laboratorio no debe emplearse para simular como garantía material aquello que sólo puede sostener una realización efectiva. En particular, una marca lógica, un objeto inmutable, un contador en memoria, una función de validación o una separación entre clases y módulos no acreditan por sí mismos autoridad material, persistencia resistente a retroceso, consumo único, aislamiento de recursos, independencia física, atestación, raíz de confianza ni mediación completa.

Una ampliación del laboratorio sólo se justifica cuando el artefacto resultante pueda conservar valor probatorio frente al backend soberano o frente al sistema completo. No procede desarrollar en Python una segunda realización de seguridad destinada a ser sustituida posteriormente por la realización soberana.

## 4. Backend soberano

Se mantiene la decisión arquitectónica vigente de utilizar Rust como objetivo principal del backend soberano del Lenguaje SV.

Python conserva su función de etapa frontal y referencia mientras resulte necesario, pero no constituye el destino final de autonomía procesal ni el fundamento material de las garantías SEC.0.

La elección de Rust tampoco constituye por sí sola una garantía de conformidad o seguridad. Rust puede aportar propiedades relevantes de implementación, entre ellas seguridad de memoria, control explícito del entorno de ejecución y artefactos autónomos, pero no demuestra por sí mismo propiedades que dependan del almacenamiento, la administración, la recuperación, la cadena de construcción, las raíces de confianza, la infraestructura, la presentación humana o cualquier otra dependencia material externa al proceso.

## 5. Primera garantía: conformidad de construcción

La primera garantía corresponde a la construcción de la realización conforme a los contratos aplicables.

Para cada garantía material o semántica relevante deberá poder establecerse, dentro del alcance declarado, una correspondencia suficiente entre:

```text
contrato
→ propiedad exigida
→ mecanismo que la impone
→ estado y dependencias necesarias
→ conjunto técnico de confianza
→ modelo de fallos
→ límites conocidos
```

La superación de pruebas unitarias o locales no sustituye esta correspondencia.

Una garantía no se considera materializada cuando el componente que la aplica puede ser eludido mediante otra vía capaz de producir el mismo efecto protegido.

## 6. Segunda garantía: comprobación adversarial integral

La segunda garantía sólo puede evaluarse cuando exista una realización suficientemente completa para ser sometida a prueba como sistema.

El objeto de esa comprobación no será exclusivamente el código Rust ni un módulo aislado, sino el sistema completo dentro del alcance declarado, incluyendo cuando sean relevantes:

- binarios y bibliotecas ejecutadas;
- entorno de ejecución;
- almacenamiento y persistencia;
- configuración;
- mecanismos de actualización y recuperación;
- credenciales, claves y raíces de confianza;
- interfaces administrativas y de mantenimiento;
- comunicaciones y extremos;
- observadores e instrumentación;
- presentación y autorización humanas;
- dependencias capaces de modificar o evitar un efecto protegido.

La batería adversarial deberá intentar producir los efectos prohibidos por todas las vías materialmente pertinentes y deberá combinar fallos cuando exista interacción causal plausible.

La resistencia local de un componente no permite inferir resistencia del sistema completo.

## 7. Relación entre pruebas locales y garantía final

Las pruebas locales, unitarias, de integración y de regresión siguen siendo obligatorias cuando correspondan. Su función es detectar defectos antes de la comprobación integral y conservar evidencia reproducible de violaciones ya conocidas.

Su estatuto es de condición necesaria, no de sello final.

Por tanto:

```text
fallo local confirmado
⇒ la realización no puede considerarse conforme para el alcance afectado
```

pero no:

```text
todas las pruebas locales superadas
⇒ garantía material integral acreditada
```

La acreditación final exige que las propiedades construidas sobrevivan además a la comprobación adversarial del sistema completo.

## 8. Doble garantía

Se fija como criterio de cierre para una realización soberana la concurrencia de dos garantías diferenciadas:

### Garantía I — construcción conforme

La arquitectura y la realización conservan los contratos aplicables y disponen de mecanismos suficientes para imponerlos dentro del modelo de fallos declarado.

### Garantía II — resistencia adversarial integral

La realización completa ha sido sometida a una batería adversarial capaz de atacar las garantías a través de los componentes y dependencias que materialmente pueden falsificarlas, sin encontrar una violación pendiente dentro del alcance ensayado.

Ninguna de las dos garantías sustituye a la otra.

Una arquitectura correcta que no ha sido atacada integralmente no posee la segunda garantía. Una batería superada sobre una arquitectura cuya correspondencia contractual no está establecida no posee la primera.

## 9. Alcance de la acreditación

La doble garantía no equivale a una afirmación de seguridad absoluta.

Toda acreditación deberá declarar como mínimo:

- versión exacta de la realización;
- contratos y garantías cubiertos;
- modelo de fallos;
- conjunto técnico de confianza relevante;
- dependencias externas;
- escenarios y clases de ataque ejercidos;
- propiedades no probadas;
- límites de instrumentación;
- fallos conocidos pendientes.

La ausencia de fallos observados no demuestra la inexistencia universal de fallos.

## 10. Consecuencias arquitectónicas

A partir de esta decisión:

1. el laboratorio Python no debe crecer como realización paralela de seguridad;
2. deben conservarse únicamente los casos, vectores, criterios y regresiones con valor reutilizable frente a la realización soberana;
3. las propiedades materiales no reproducibles de forma honesta en el laboratorio deben permanecer como obligaciones explícitas para la realización y su comprobación posterior;
4. Rust continúa siendo el objetivo principal del backend soberano, sin atribuirle garantías que dependan de componentes externos al lenguaje;
5. la especificación futura del entorno de ejecución mínimo deberá identificar qué garantías puede imponer el backend y cuáles necesitan mecanismos materiales adicionales;
6. la comprobación final de SEC.0 deberá atacar el sistema completo y no limitarse a una revisión local del backend.

## 11. Cierre

El laboratorio ejecutable sirve para conservar contratos falsables y regresiones reutilizables; no es la realización soberana.

El backend soberano se orienta a Rust; Rust no sustituye la arquitectura material ni sus dependencias.

La conformidad final de una realización del Lenguaje SV requerirá conjuntamente construcción conforme a los contratos y comprobación adversarial integral del sistema completo, siempre dentro de un alcance y un modelo de fallos expresamente declarados.
