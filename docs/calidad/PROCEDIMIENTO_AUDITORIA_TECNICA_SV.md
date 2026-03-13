# Procedimiento de Auditoría Técnica del Proyecto SV

## 1. Naturaleza y subordinación

Este documento tiene carácter operativo y subordinado.
No modifica, sustituye ni corrige la autoridad doctrinal vigente del Sistema Vectorial SV.
Su función es regular, de forma proporcionada y revisable, cómo se auditan los hitos técnicos del desarrollo.

## 2. Objeto

Definir un procedimiento mínimo y estable para auditar cambios relevantes del proyecto, preservar la congruencia entre doctrina, especificación, implementación y documentación pública, y evitar tanto la deriva técnica como la burocratización del desarrollo.

## 3. Alcance

Este procedimiento se aplica a hitos técnicos relevantes del proyecto, en particular cuando afecten a:

- especificación operativa del lenguaje;
- parser, lowering, validator, runner, CLI o playground;
- catálogo de errores;
- suite de tests y artefactos de conformidad;
- documentación pública de estado;
- ZIPs operativos de parche;
- cambios de fase o cierres estructurales.

## 4. Principios

La auditoría técnica del proyecto se rige por los siguientes principios:

1. **Primacía doctrinal**  
   La doctrina vigente del Sistema Vectorial SV prevalece sobre cualquier decisión operativa o implementativa.

2. **Proporcionalidad**  
   La intensidad de la auditoría debe ajustarse al impacto real del hito.

3. **No duplicación doctrinal**  
   Ninguna auditoría operativa reabre ni duplica doctrina salvo error material real.

4. **Trazabilidad suficiente**  
   Todo cierre relevante debe poder reconstruirse mediante hechos, fundamento, evidencia, impacto y estado.

5. **No burocratización**  
   La auditoría debe proteger el desarrollo, no bloquearlo.

6. **Adversarial integrada**  
   Toda decisión importante debe someterse a una objeción adversarial explícita antes de su cierre.

## 5. Disparadores de auditoría

Debe abrirse auditoría técnica cuando concurra alguno de los siguientes supuestos:

- creación de un ZIP operativo de parche;
- apertura formal de un parche;
- cierre formal de un parche;
- cambio en IR, frontera, gramática o consenso operativo ya cerrado;
- cambio en validator, runner, catálogo de errores o tests;
- modificación de README o documentación pública de estado;
- aparición de una sorpresa técnica real;
- cierre de fase o paso a nueva fase;
- preparación de salto arquitectónico relevante.

## 6. Niveles de auditoría

### 6.1. Auditoría ligera

Se aplica durante la marcha ordinaria del proyecto.
Su finalidad es comprobar que el hito está delimitado, documentado y respaldado por evidencia suficiente.

### 6.2. Auditoría dura

Se aplica sólo en puntos de control.
Es obligatoria, al menos, en estos casos:

- cierre de parche estructural;
- sorpresa técnica real con impacto de contrato;
- cambio de catálogo de errores;
- cierre de fase;
- paso a nueva fase;
- antes de abrir backend o nuevo nivel arquitectónico relevante.

## 7. Secuencia mínima de auditoría

Toda auditoría deberá seguir, como mínimo, esta secuencia:

### Paso 1 — Delimitación

Precisar:

- objeto exacto;
- alcance;
- qué toca;
- qué no toca;
- riesgo adversarial dominante;
- criterio de cierre.

### Paso 2 — Revisión de base

Comprobar, según proceda:

- doctrina aplicable;
- IR / frontera / gramática;
- estado real del repositorio;
- catálogo de errores;
- validator / runner / tests;
- documentación pública afectada.

### Paso 3 — Clasificación

Separar expresamente:

- error real;
- mejora opcional;
- deuda futura;
- cambio que no procede.

### Paso 4 — Adversarial integrada

Formular la objeción principal contra la decisión propuesta y responderla con base técnica suficiente.

### Paso 5 — Veredicto

Determinar si el hito:

- procede;
- procede con ajuste;
- debe corregirse antes de avanzar;
- debe aplazarse;
- o no debe abrirse.

### Paso 6 — Cierre o traslado

Emitir un cierre acreditado, un cierre con reserva delimitada, un paso correctivo único o un traslado a fase posterior.

## 8. Evidencia admisible

Se consideran fuentes válidas de evidencia, según el caso:

- contenido verificable del repositorio;
- suite de tests y resultados observables;
- ZIP operativo verificado;
- documentación publicada;
- capturas o PDF de confirmación cuando la propagación online o el acceso raw no permitan verificación suficiente.

No se considerarán prueba bastante por sí solos:

- nombres de archivo;
- nombres de rama;
- metadatos aislados;
- inferencias no apoyadas en contenido.

## 9. Regla de no bloqueo

La auditoría no debe convertirse en un proceso permanente de fricción.
Durante la marcha ordinaria se aplicará control ligero.
La revisión dura queda reservada a puntos de control y transiciones relevantes.

## 10. Regla de congruencia

Toda auditoría deberá comprobar que un nivel inferior no corrige silenciosamente a uno superior.

Se preservará la siguiente jerarquía:

1. doctrina y cadena normativa vigente;
2. especificación operativa del lenguaje;
3. implementación de referencia;
4. gobierno técnico del desarrollo.

## 11. Relación con el Registro de Evolución Técnica del Proyecto

Todo hito auditado que altere el estado gobernable del proyecto deberá asentarse en el Registro de Evolución Técnica del Proyecto.

No se registrarán microcambios sin entidad suficiente.

## 12. Salidas posibles

Las salidas formales de una auditoría serán una de estas:

- cierre acreditado;
- cierre con reserva delimitada;
- paso correctivo único;
- reapertura controlada;
- aplazamiento motivado;
- traslado a fase siguiente.

## 13. Regla de estilo documental

Toda redacción pública derivada de auditoría deberá ser:

- formal;
- sobria;
- técnica;
- revisable por terceros;
- y ajena a cocina interna o nomenclaturas privadas.

## 14. Vigencia

Este procedimiento permanecerá vigente mientras no contradiga la autoridad doctrinal superior ni sea sustituido por una versión operativa posterior expresamente documentada.
