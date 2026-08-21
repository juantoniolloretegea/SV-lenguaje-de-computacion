# Acta técnica de cierre de SEC.0-T — pruebas adversariales integrales, trazabilidad y regresión

**Fecha:** 21/08/2026  
**Estado:** cerrado  
**Ámbito:** Lenguaje SV — SEC.0-T

## 1. Objeto del cierre

SEC.0-T fija el contrato abstracto de pruebas aplicable a realizaciones concretas que pretendan acreditar preservación de los contratos SEC.0-A, SEC.0-D, SEC.0-M y SEC.0-X.

El cierre establece cómo deben identificarse el objeto sometido a prueba, la correspondencia entre invariantes y casos, la falsabilidad material, la aplicabilidad de clases de prueba, la independencia del criterio esperado y de la evidencia, la instrumentación, los veredictos, la regresión y las afirmaciones de conformidad.

SEC.0-T no declara conforme ninguna realización concreta por el mero hecho de quedar cerrado.

## 2. Documento técnico fijado

Queda incorporado como documento de referencia:

- `CONTRATO_ABSTRACTO_DE_PRUEBAS_ADVERSARIALES_INTEGRALES_TRAZABILIDAD_Y_REGRESION_SEC0_T_2026_08_21.md`.

El contrato define, entre otros elementos:

- la ligadura de cada ejecución a una versión y artefacto exactos;
- `Targets(t)` y la correspondencia entre caso e invariante;
- `Falsifiable(t,I)` como capacidad material ejercitable y acreditable de discriminar una realización no conforme;
- `Capabilities(SUT,G)` y `ApplicableClass(c | SUT,G)`;
- los veredictos `PASS`, `FAIL`, `NO_EJECUTADO`, `NO_PROBADO` e `INCONCLUSO`;
- las reglas sobre instrumentación perturbadora;
- la independencia del criterio esperado y de la evidencia pública;
- la conservación de escenarios integrales y la reducción de fallos a regresiones;
- la prohibición de relabelado y las mutaciones semánticas obligatorias cuando la distinción correspondiente esté materializada;
- la conformidad acotada al alcance realmente probado.

## 3. Condiciones estructurales satisfechas

El cierre establece que:

1. una prueba sólo cubre un invariante si puede discriminar una violación material dentro de su modelo de fallos;
2. una declaración documental de falsabilidad no basta: debe existir una alteración no conforme especificada, ejercitable y cuyo alcance pueda acreditarse;
3. la aplicabilidad de una clase de prueba deriva de las capacidades y garantías del objeto sometido a prueba;
4. el interesado en el veredicto no puede reducir el perímetro declarando unilateralmente una clase como no realizable;
5. `NO_EJECUTADO`, `NO_PROBADO` e `INCONCLUSO` no constituyen cobertura;
6. un instrumento que puede impedir el fallo no produce por sí solo evidencia transferible a la realización ordinaria;
7. la evidencia pública de conformidad debe estar protegida frente al mismo fallo para el que se invoca;
8. todo fallo confirmado conserva una regresión permanente salvo sustitución por cobertura equivalente acreditada o modificación legítima del contrato;
9. un caso reducido no sustituye al escenario integral que permitió descubrir el fallo;
10. una batería finita no se presenta como demostración universal de ausencia de fallos;
11. SEC.0-T no redefine SEC.0-A, SEC.0-D, SEC.0-M o SEC.0-X para facilitar el resultado de las pruebas.

## 4. Estado de SEC.0

Con este cierre quedan fijados como contratos abstractos:

- SEC.0-A — autoridad, constitución y génesis;
- SEC.0-D — diagnóstico y fallo cerrado;
- SEC.0-M — memoria, persistencia, recursos y continuidad;
- SEC.0-X — ejecución material, conjunto técnico de confianza, arranque, atestación e independencia;
- SEC.0-T — pruebas adversariales integrales, trazabilidad y regresión.

El cierre conjunto de estos contratos no equivale a una declaración de conformidad de la implementación vigente ni a una certificación de plataforma.

## 5. Continuidad hacia realización y pruebas

La etapa posterior debe contrastar realizaciones concretas contra los contratos cerrados.

Cada afirmación de conformidad deberá identificar el objeto exacto sometido a prueba, las capacidades aplicables, los invariantes cubiertos, los veredictos obtenidos, las propiedades pendientes y las limitaciones materiales de la evidencia.

La materialización posterior no queda autorizada a modificar silenciosamente el significado de los contratos para acomodar decisiones de implementación.

## 6. Límites

Este cierre no selecciona ni prescribe:

- sistema operativo;
- procesador;
- lenguaje de implementación;
- mecanismo criptográfico;
- motor de almacenamiento;
- plataforma de ejecución;
- formatos de atestación;
- mecanismos concretos de aislamiento o replicación.

Tampoco modifica por sí mismo la gramática superficial, la IR v0.2, el validador ni el catálogo diagnóstico.

## 7. Cierre

SEC.0-T se declara cerrado el 21/08/2026 como **contrato abstracto de pruebas adversariales integrales, trazabilidad y regresión**.

Con ello queda cerrado el conjunto arquitectónico SEC.0 en su nivel contractual. La conformidad de realizaciones concretas queda sometida a materialización y evidencia ejecutable posterior.
