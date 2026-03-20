# CONTRATO_DE_ENGANCHE_DE_INTERFACES_FUTURAS_Y_ABI_SEMANTICO_DIAGNOSTICO_MINIMO

**Fecha:** 20/03/2026  
**Estado:** Contrato operativo subordinado  
**Ámbito:** `SV-lenguaje-de-computacion`

## 1. Naturaleza

El presente documento tiene carácter **operativo, técnico y subordinado**.

No reabre doctrina ni corrige silenciosamente la IR canónica v0.2. Su función es fijar, de manera explícita y auditable, el **contrato mínimo de enganche** que debe permanecer estable para que el núcleo del lenguaje SV pueda avanzar sin comprometer la apertura futura de nuevos carriles de interfaz legítimos.

## 2. Objeto

Materializar el ABI semántico-diagnóstico mínimo que el frontend de referencia deberá preservar al avanzar hacia backend soberano.

Este ABI no enumera sentidos futuros concretos. Fija, en cambio, la **forma canónica de acoplamiento** que cualquier futura interfaz subordinada deberá respetar.

## 3. Principio rector

El núcleo del lenguaje SV podrá avanzar si y solo si preserva simultáneamente:

1. un **núcleo canónico duro**;
2. una **cadena mínima de enganche** explícita;
3. y un **contrato diagnóstico observable** capaz de impedir aperturas arbitrarias.

## 4. Cadena mínima de enganche

Toda futura interfaz legítima deberá poder acoplarse al sistema mediante la siguiente cadena formal mínima:

`CaptureSpec → AdmissibilitySpec → Ternarizer → Domain → Agent → QuerySpec / QueryContext → QueryResult`

La cadena expresa tres exigencias acumulativas:

- que la entrada exógena quede descrita por una captura explícita;
- que la admisibilidad de esa captura quede gobernada por una regla tipada;
- que la transducción a ternario quede formalmente declarada antes de su uso por dominio, agente y consulta.

## 5. Núcleo que debe permanecer estable

A efectos de continuidad hacia backend, se considera núcleo mínimo preservable, al menos, el siguiente bloque:

- gramática superficial vigente;
- IR canónica v0.2 en lo ya materializado;
- contrato diagnóstico observable del frontend de referencia;
- serialización determinista del IR;
- validación explícita de la cadena mínima de enganche en N4/Uso.

## 6. Lo que no forma parte de este contrato mínimo

No forman parte del ABI mínimo, por ahora, los siguientes extremos:

- la enumeración cerrada de futuras interfaces humanas;
- la semántica material exhaustiva de cada futuro carril;
- la interpretación ejecutiva plena de todos los campos opacos de `Domain`;
- el backend soberano en Rust o en otra implementación posterior.

## 7. Regla de apertura legítima de futuros carriles

La apertura futura de un nuevo carril de interfaz será legítima solo si concurren, al menos, estas condiciones:

1. **alcance claro** del carril;
2. **manuscrito serio** y trazable en su sede natural;
3. **soporte verificable** proporcionado a su fase;
4. **delimitación de fase** explícita;
5. **compatibilidad jerárquica** con doctrina, IR y contrato diagnóstico vigentes.

## 8. Traducción implementativa mínima exigible

El frontend de referencia deberá impedir, como mínimo:

- dominios que nombren puertos de captura/admisibilidad/ternarización sin declaración trazable;
- agentes incompatibles con el dominio y la arquitectura que dicen servir;
- consultas cuyo `QuerySpec` no sea coherente con el `QueryContext` ejecutado.

## 9. Regla de prudencia

Este contrato no autoriza a declarar completo el frente básico ni a sobreatribuir al frontend competencias perceptivas no implementadas.

Su efecto es más estrecho: impedir que el núcleo avance de forma ciega o petrificada, y obligarle a conservar desde ahora una forma de acoplamiento futuro limpia, tipada y auditable.

## 10. Cierre

Se fija como ABI semántico-diagnóstico mínimo del frente actual del lenguaje SV la preservación explícita de la cadena de enganche descrita en este documento.

Mientras esta cadena permanezca materializada, validada y documentada, el núcleo podrá avanzar sin comprometer por ello la apertura futura de nuevas interfaces subordinadas.
