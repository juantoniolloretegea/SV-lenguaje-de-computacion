# Acta técnica de cierre de SEC.0-X — ejecución material, conjunto técnico de confianza, arranque, atestación e independencia

**Fecha:** 21/08/2026  
**Estado:** cerrado  
**Ámbito:** Lenguaje SV — SEC.0-X

## 1. Objeto del cierre

SEC.0-X se abrió para fijar las condiciones materiales mínimas que debe satisfacer una realización del Lenguaje SV para que los contratos de autoridad, diagnóstico, persistencia y continuidad no puedan ser falseados por componentes no declarados de la propia plataforma.

El cierre no selecciona tecnología concreta ni modifica gramática, IR, validador o catálogo diagnóstico.

## 2. Documento técnico fijado

Queda incorporado como documento de referencia:

- `CONTRATO_ABSTRACTO_DE_EJECUCION_MATERIAL_TCB_ARRANQUE_ATESTACION_INDEPENDENCIA_SEC0_X_2026_08_21.md`.

El contrato establece el conjunto técnico de confianza por garantía, la independencia relativa al modelo de fallos, la raíz de confianza, las reglas de sustitución de raíz, el arranque admitido, la cadena de construcción, la mediación material, la atestación, la persistencia material, el aislamiento de recursos, la presentación humana, la independencia de quórums, los privilegios de administración y la protección de la propia definición de garantía.

## 3. Condiciones estructurales satisfechas

El cierre fija:

1. todo componente capaz de falsificar una garantía pertenece al conjunto técnico de confianza de esa garantía;
2. la separación conceptual no se considera prueba de independencia material;
3. toda independencia se declara respecto de una clase de fallo;
4. las cadenas de confianza poseen raíces no circulares;
5. una raíz comprometida o sospechosa de compromiso no puede ser la única prueba de legitimidad de su sucesora;
6. el arranque correcto no equivale por sí solo a estado de ejecución admitido ni a continuidad semántica;
7. la auditabilidad del código fuente no acredita por sí sola el artefacto cargado;
8. una comprobación sólo permite excluir una herramienta de construcción cuando es independiente frente al mismo fallo;
9. las vías de mantenimiento, actualización, recuperación y administración forman parte del perímetro cuando pueden producir efectos protegidos;
10. la atestación no crea autoridad y su actualidad queda constituida cuando la decisión depende del estado vivo de la plataforma;
11. una respuesta lógica de escritura no acredita una garantía física de persistencia superior a la demostrada;
12. el testigo contra retroceso o doble consumo debe ser independiente del mismo estado que protege frente al fallo considerado;
13. las reservas de recursos sólo se consideran aisladas cuando la plataforma puede imponer materialmente esa separación;
14. la autenticación de un canal no acredita por sí sola el estado del proceso situado detrás del extremo;
15. presentación y firma quedan ligadas a la misma revisión material del objeto autorizado;
16. la pluralidad de firmas, réplicas o servicios no demuestra independencia si comparten una causa de compromiso relevante;
17. los privilegios materiales capaces de falsear una garantía pertenecen a su conjunto técnico de confianza o limitan expresamente la garantía;
18. la definición de una garantía y de sus dependencias es un objeto protegido cuando interviene en decisiones protegidas.

## 4. Relación con SEC.0-A, SEC.0-D y SEC.0-M

SEC.0-X no redefine los contratos anteriores.

En particular:

- la capacidad material no crea autoridad;
- una comprobación no crea la autoridad que pretende acreditar;
- la ausencia de comprobación no se transforma en éxito ni en `U`;
- una copia o historia local no determina por sí sola continuidad vigente;
- un testigo de unicidad no puede depender indistinguiblemente del mismo estado clonable o retrocedible que pretende proteger.

## 5. Límites del cierre

SEC.0-X no demuestra infalibilidad física, ausencia de vulnerabilidades desconocidas, corrección absoluta de sistemas operativos o compiladores, imposibilidad total de extracción de claves, disponibilidad perfecta, independencia universal entre proveedores, comprensión humana ni resistencia a cualquier atacante con acceso físico ilimitado.

Estas materias pertenecen al modelo de fallos y límites de cada realización concreta.

## 6. Continuidad posterior

El cierre de SEC.0-X no selecciona plataforma, sistema operativo, lenguaje de implementación, algoritmo criptográfico, motor de almacenamiento ni tecnología de atestación.

La fase siguiente, SEC.0-T, debe comprobar mediante pruebas integrales que una futura realización conserva conjuntamente los contratos SEC.0-A, SEC.0-D, SEC.0-M y SEC.0-X sin cambiar su significado.

SEC.0-T requiere apertura expresa.

## 7. Cierre

Las propiedades exigidas para SEC.0-X quedan fijadas en un documento técnico autónomo, con invariantes materiales, límites de alcance y dependencias explícitas.

SEC.0-X se declara cerrado el 21/08/2026 como **contrato abstracto de ejecución material, conjunto técnico de confianza, arranque, atestación e independencia**.
