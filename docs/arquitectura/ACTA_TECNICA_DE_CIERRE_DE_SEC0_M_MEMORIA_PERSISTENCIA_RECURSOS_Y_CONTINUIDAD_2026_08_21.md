# Acta técnica de cierre de SEC.0-M — memoria, persistencia, recursos y continuidad

**Fecha:** 21/08/2026  
**Estado:** cerrado  
**Ámbito:** Lenguaje SV — SEC.0-M

## 1. Objeto del cierre

SEC.0-M se abrió para fijar las condiciones abstractas que deben conservar memoria, persistencia, recursos y continuidad para que una autoridad ya constituida y una comprobación ya gobernada no puedan ser alteradas materialmente mediante retroceso, clonación, bifurcación, vistas derivadas, fallos parciales, agotamiento de recursos o recuperación.

El cierre no materializa infraestructura de ejecución ni selecciona sistema operativo, lenguaje de implementación, motor de almacenamiento, soporte físico o mecanismo concreto de coordinación entre réplicas.

## 2. Documento técnico fijado

Queda incorporado como documento de referencia:

- `CONTRATO_ABSTRACTO_DE_MEMORIA_PERSISTENCIA_RECURSOS_Y_CONTINUIDAD_SEC0_M_2026_08_21.md`.

El contrato distingue estado semántico y estado técnico; define `PDep(d | C)`, `AStore`, `View(AStore)` y `Budget(F | C)`; establece reglas de continuidad, retroceso, clonación, bifurcación, cobertura negativa, ligadura entre comprobación y efecto, fallos parciales, efectos externos, recursos, atención humana, compactación y recuperación; y fija los invariantes M2-01 a M2-29.

## 3. Condiciones estructurales satisfechas

El cierre establece:

1. la presencia material de un objeto no le confiere vigencia semántica;
2. una copia íntegra no constituye por sí sola continuidad legítima;
3. toda decisión reutilizable después de reinicio o recuperación debe conservar dependencias persistentes suficientes;
4. toda decisión sobre vigencia, revocación, consumo, continuidad, cobertura negativa, acumulación o recuperación debe fundarse en una dependencia autoritativa acreditada;
5. una vista derivada que determine una decisión autoritativa pasa a formar parte de las dependencias persistentes y asume las obligaciones correspondientes;
6. una historia localmente válida no determina por sí sola la continuidad vigente;
7. un retroceso material no resucita revocaciones, autorizaciones consumidas ni efectos olvidados;
8. una garantía fuerte contra retroceso requiere una referencia que no pueda retroceder indistinguiblemente junto con el mismo estado;
9. la clonación no multiplica autoridad, identidad de implantación ni autorizaciones de un solo uso;
10. una autoridad consumible o de acumulación única no admite ejercicio automático mientras la unicidad dependa únicamente de estado conjuntamente clonable o retrocedible;
11. la bifurcación y la selección de continuidad deben regirse por una política previamente constituida;
12. `HEAD`, el tiempo local o la respuesta más rápida no seleccionan continuidad por sí solos;
13. la ausencia en un índice sólo acredita ausencia autoritativa cuando su cobertura negativa está acreditada;
14. la comprobación y el efecto deben permanecer ligados a un estado material compatible;
15. un fallo parcial no puede reinterpretarse como éxito o ausencia ni justificar una reejecución ciega;
16. el envío local de una solicitud no acredita por sí solo un efecto externo;
17. toda forma repetible, recursiva, expansiva, abierta a entrada no confiable o capaz de generar actos humanos privilegiados debe disponer de un presupuesto de recursos constituido, comprobable y no tautológico;
18. el agotamiento de recursos no modifica `Tri`, no crea autoridad y no permite omitir comprobaciones;
19. la atención humana se trata como recurso finito también para formas previamente admitidas;
20. la compactación o reducción de historial no puede decidir durante su ejecución qué dependencias dejan de ser relevantes;
21. la recuperación no puede legitimarse únicamente mediante el mismo estado que intenta recuperar;
22. una copia de seguridad sólo recupera el alcance cuyas dependencias pueda acreditar.

## 4. Relación con SEC.0-A y SEC.0-D

SEC.0-M conserva los contratos ya cerrados de SEC.0-A y SEC.0-D.

En particular:

- la materialización de autoridad no crea autoridad nueva;
- una comprobación técnica no crea la autoridad cuya existencia pretende acreditar;
- `D-N` no se identifica con `U` ni con éxito;
- la continuidad material no puede utilizarse para eludir revocaciones, límites de autoridad o requisitos de comprobación;
- el agotamiento de recursos no permite degradar el núcleo de control fijado por SEC.0-D.

## 5. Premisas y límites del cierre

SEC.0-M no demuestra por sí mismo:

- integridad del soporte físico;
- persistencia real de una escritura;
- independencia efectiva de una referencia externa de continuidad;
- monotonicidad física de contadores o relojes;
- aislamiento material de recursos;
- independencia real entre réplicas o testigos;
- integridad del sistema operativo, controladores o compilador;
- fidelidad del observador material;
- propiedades de entrega u orden de la red;
- disponibilidad absoluta;
- ejecución exactamente una vez frente a sistemas externos.

Estas materias permanecen asignadas a SEC.0-X, SEC.0-T o a las fases posteriores que correspondan.

## 6. Continuidad posterior

El cierre de SEC.0-M no modifica gramática, IR, validador ni catálogo diagnóstico.

Tampoco autoriza todavía:

- infraestructura general de ejecución;
- selección de plataforma material;
- sistema operativo concreto;
- base de datos o motor de almacenamiento concreto;
- protocolo de coordinación o consenso;
- tipos nuevos de IR;
- códigos diagnósticos;
- valores numéricos de recursos;
- materialización del modelo estable de usuarios, identidad o distribución.

La fase siguiente deberá abordar las condiciones materiales de ejecución, conjunto técnico de confianza, arranque, atestación e independencia efectiva necesarias para sostener los contratos cerrados hasta este punto.

## 7. Cierre

Las propiedades exigidas para SEC.0-M quedan fijadas en un documento técnico autónomo, con invariantes y límites explícitos de alcance.

SEC.0-M se declara cerrado el 21/08/2026 como **contrato abstracto de memoria, persistencia, recursos y continuidad**.