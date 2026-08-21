# Acta técnica de cierre de SEC.0-D — diagnóstico y fallo cerrado

**Fecha:** 21/08/2026  
**Estado:** cerrado  
**Ámbito:** Lenguaje SV — SEC.0-D

## 1. Objeto del cierre

SEC.0-D se abrió para fijar el contrato abstracto de comprobación aplicable a operaciones sujetas a control y para impedir que la ausencia, insuficiencia o imposibilidad de verificación se conviertan silenciosamente en permiso, autoridad o valor ternario del dominio.

El cierre no materializa infraestructura de ejecución ni modifica gramática, IR, validador o catálogo diagnóstico.

## 2. Documento técnico fijado

Queda incorporado como documento de referencia:

- `CONTRATO_ABSTRACTO_DE_DIAGNOSTICO_Y_FALLO_CERRADO_SEC0_D_2026_08_21.md`.

El contrato define los resultados técnicos D-A, D-R y D-N; el conjunto `Req(F,e | C)`; el núcleo no eludible de control; la regla de fallo cerrado; las condiciones de aplicabilidad y sustitución de verificadores; las reglas de reutilización, cobertura, revocación, excepciones y emergencia; y los invariantes D2-01 a D2-20.

## 3. Condiciones estructurales satisfechas

El cierre establece:

1. el conjunto de formas sujetas a control queda determinado por su constitución y no por una decisión local del ejecutor;
2. `Req(F,e | C)` debe ser no vacío para toda forma sujeta a control;
3. la vaciedad de requisitos no produce permiso;
4. toda forma controlada conserva un núcleo no eludible determinado por su definición constituida;
5. T-E, T-G, T-C y T-R sujetas a control sólo pueden producir su efecto cuando todas las obligaciones exigibles quedan acreditadas;
6. `D-R` y `D-N` bloquean el efecto sin confundirse;
7. `D-N` no se identifica con `U` ni con éxito;
8. un verificador no puede acreditar por sí mismo la autoridad necesaria para legitimar su propio uso;
9. una comprobación histórica sólo puede reutilizarse mientras sus ligaduras de validez permanezcan acreditadas;
10. una refutación vigente no puede eliminarse mediante un resultado posterior no verificable ni mediante una sustitución no gobernada del verificador;
11. las excepciones y formas de emergencia conservan el núcleo de control y no adquieren por esa condición una autoridad, dominio o envolvente mayores;
12. el fallo cerrado se limita al efecto y a sus dependencias constituidas;
13. el diagnóstico debe conservar información suficiente para explicar por qué un efecto no avanzó.

## 4. Relación con SEC.0-A

SEC.0-D conserva el contrato cerrado de SEC.0-A.

En particular:

- una comprobación no crea autoridad;
- la validez de una forma, la autoridad aplicable y la pertenencia a un dominio gobernado deben proceder de estado previamente constituido;
- la imposibilidad de acreditar esas condiciones produce `D-N`;
- no existe conversión silenciosa de una falta de comprobación en T-G, T-C, T-R o T-E válida.

## 5. Premisas y límites del cierre

SEC.0-D no demuestra por sí mismo:

- la indivisibilidad material entre comprobación y ejecución;
- la disponibilidad de evidencias o verificadores;
- la integridad material del verificador;
- el aislamiento de componentes;
- la persistencia o continuidad de los resultados;
- la resistencia a la denegación de servicio;
- la corrección de la realización material que traduzca D-A, D-R y D-N.

Estas materias permanecen asignadas a fases posteriores y no se consideran resueltas por este cierre.

## 6. Continuidad posterior

El cierre de SEC.0-D no modifica gramática, IR, validador ni catálogo diagnóstico.

Tampoco autoriza todavía:

- infraestructura general de ejecución;
- selección de plataforma material;
- mecanismos concretos de persistencia;
- códigos diagnósticos;
- tipos nuevos de IR;
- materialización de verificadores;
- políticas concretas de disponibilidad o recuperación.

La siguiente fase de la secuencia de seguridad requiere decisión expresa para su apertura.

## 7. Cierre

Las propiedades exigidas para SEC.0-D quedan fijadas en un documento técnico autónomo, con invariantes, reglas de paso y límites explícitos de alcance.

SEC.0-D se declara cerrado el 21/08/2026 como **contrato abstracto de diagnóstico y fallo cerrado**.
