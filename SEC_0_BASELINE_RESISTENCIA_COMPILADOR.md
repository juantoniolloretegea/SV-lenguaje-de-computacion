# SEC_0_BASELINE_RESISTENCIA_COMPILADOR.md

## 1. Objeto y alcance

Este documento fija la baseline inicial de resistencia del compilador de referencia del lenguaje `.svp`.

Su función es exclusivamente técnica, preventiva y transversal. No constituye doctrina del Sistema Vectorial SV, no modifica la frontera normativa del lenguaje, no altera la IR canónica v0.2 y no introduce semántica nueva.

Su alcance actual se limita al compilador de referencia y a sus superficies operativas inmediatas:
- lexer/parser superficial,
- lowering a IR,
- serialización JSON canónica,
- CLI mínima de referencia,
- suite de tests y artefactos de conformidad que sostienen verificabilidad.

## 2. Finalidad de SEC-0

SEC-0 se abre para evitar que el lenguaje derive hacia:
- ambigüedad superficial,
- no determinismo,
- fragilidad ante entradas patológicas,
- inestabilidad diagnóstica,
- divergencia entre contrato público y comportamiento observable,
- erosión de trazabilidad del toolchain.

SEC-0 no es una fase de seguridad ofensiva ni una política de runtime hostil completo. Es una baseline de resistencia del compilador.

## 3. Activos actuales protegidos

En esta fase, los activos a proteger son:

### 3.1. Compilador de referencia
- parseo de superficie,
- lowering a IR,
- serialización canónica,
- emisión diagnóstica,
- CLI declarada públicamente.

### 3.2. Propiedades de confianza
- determinismo del parseo,
- determinismo del lowering,
- estabilidad del JSON canónico,
- estabilidad del error dominante,
- rechazo limpio de entradas inválidas,
- coherencia entre documentación pública y comportamiento real.

### 3.3. Integridad del toolchain inmediato
- suite de conformidad,
- expected JSON,
- smoke tests de CLI,
- artefactos auxiliares del repositorio con valor verificable.

## 4. Amenazas actuales relevantes

Las amenazas actuales relevantes no se formulan como intrusión clásica, sino como sabotabilidad estructural del compilador y de su capa de confianza.

Se consideran amenazas relevantes en esta baseline:

1. **Ambigüedad de parseo**  
   Entradas que puedan tensar el parser hacia interpretaciones inestables o divergentes.

2. **No determinismo de lowering o serialización**  
   El mismo input no debe producir IR distinta ni JSON distinto según entorno o repetición razonable.

3. **Inestabilidad diagnóstica**  
   Un mismo caso inválido no debe fluctuar arbitrariamente entre errores dominantes incompatibles.

4. **Entradas patológicas de degradación**  
   Casos diseñados para tensionar complejidad, profundidad, tamaño o comportamiento del compilador sin aportar valor semántico.

5. **Deriva entre contrato documentado y comportamiento observable**  
   El compilador de referencia, la CLI y la suite no deben declarar un comportamiento que no pueda verificarse materialmente.

6. **Erosión de integridad del repositorio**  
   Duplicados residuales, artefactos no trazados, expected incoherentes o residuos que degraden auditabilidad.

## 5. Amenazas expresamente fuera de alcance en SEC-0A

Quedan fuera de alcance en esta iteración:

- backend ejecutable real,
- WASM,
- sandboxing,
- escape de runtime,
- aislamiento multiusuario,
- supply chain de paquetes,
- ejecución remota de terceros,
- hardening criptográfico de distribución,
- seguridad ofensiva o explotación.

Esas materias pertenecen a una fase posterior, antes de backend/WASM abierto.

## 6. Invariantes de robustez actuales

La baseline SEC-0A fija los siguientes invariantes de robustez del compilador de referencia:

1. **Parseo determinista**  
   La misma entrada debe seguir la misma ruta de parseo dentro del alcance actual del parser de referencia.

2. **Lowering determinista**  
   La misma entrada válida debe producir la misma IR dentro del mismo estado del compilador.

3. **Serialización canónica**  
   La salida JSON debe mantenerse canónica, ordenada y reproducible.

4. **Estabilidad del error dominante**  
   Los casos inválidos ya fijados por la suite deben seguir fallando por el código esperado salvo corrección formal explícita.

5. **Rechazo limpio**  
   La entrada inválida debe fallar con mensaje y código comprensibles, sin estados parciales corruptos ni artefactos silenciosos.

6. **No creación silenciosa de artefactos corruptos**  
   La CLI no debe generar salidas parciales o confusas cuando el proceso falla.

7. **Comportamiento comprensible ante entradas patológicas razonables**  
   El compilador debe seguir siendo verificable y auditable bajo tensión moderada, aun cuando rechace la entrada.

## 7. Regla transversal de revisión adversarial mínima

A partir de SEC-0, ningún parche funcional del lenguaje deberá darse por conceptualmente cerrado sin responder, al menos, a esta pregunta:

**¿El cambio rompe o tensiona parseo determinista, lowering determinista, serialización canónica, estabilidad diagnóstica o rechazo limpio?**

Esta revisión adversarial mínima no sustituye la suite funcional. La acompaña.

## 8. Relación con la suite de tests

SEC-0 no reemplaza la suite de conformidad existente.

La suite ordinaria sigue midiendo:
- conformidad del lowering,
- exactitud del expected JSON,
- exactitud diagnóstica,
- contrato mínimo de CLI.

SEC-0 añade una mirada distinta:
- resistencia,
- robustez,
- estabilidad bajo tensión,
- disciplina de gobierno técnico.

## 9. Implementación prevista tras SEC-0A

La continuación natural de esta baseline será **SEC-0B**, limitada a una primera semilla adversarial pequeña y separada de la conformidad ordinaria.

SEC-0B no deberá convertirse en fuzzing masivo ni en una fase nueva del lenguaje. Su función será únicamente iniciar una batería mínima de entradas frontera del compilador.

## 10. Efectos y no-efectos del documento

### 10.1. Efectos
- fija el marco basal de resistencia del compilador;
- identifica activos y amenazas actuales relevantes;
- establece invariantes mínimos de robustez;
- introduce una disciplina transversal de revisión adversarial mínima.

### 10.2. No-efectos
- no modifica la doctrina base;
- no modifica la IR canónica v0.2;
- no modifica la gramática superficial mínima;
- no introduce semántica nueva;
- no abre aún backend, WASM ni seguridad de runtime.

## 11. Decisión diferida

Cualquier transición desde baseline de resistencia del compilador hacia seguridad central de ejecución queda diferida a un acto posterior, expresamente ligado a la apertura de backend ejecutable, WASM o entrada no confiable de terceros.

Hasta entonces, SEC-0 debe mantenerse como línea transversal, mínima, acumulativa y no invasiva.
