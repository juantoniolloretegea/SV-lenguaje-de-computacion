# Nota técnica de previsión de capa IA trazable por agente especializado

## Estatuto

Esta nota tiene carácter **técnico, arquitectónico y prospectivo restringido**.  
No introduce doctrina nueva, no modifica la jerarquía del ecosistema SV y no abre por sí sola sintaxis, gramática, IR ni backend adicionales.

Su función es dejar **cartografiada** una necesidad futura ya suficientemente visible: la aparición de una **capa de consulta y simulación trazable por agente especializado**, capaz de convivir con el Lenguaje SV sin plegar la doctrina ni degradar la semántica de la terna.

## Hecho constatado

La evolución de `SVperitus-dataset` muestra ya, aunque todavía en régimen piloto, la comparecencia de superficies de uso que no se limitan al cálculo normativo desnudo:

- monitor poligonal del estado;
- trayectoria por SUCESOS;
- separación entre historia real y ramas simuladas;
- apoyo IA al experto humano;
- capas auxiliares de lectura geométrica y de evidencia.

Estas superficies no deben leerse como privilegio exclusivo del agente de seguridad, sino como **patrón operativo emergente** del agente especializado suficientemente maduro.

## Decisión de arquitectura

El lenguaje debe quedar prevenido desde ahora de que, en fases futuras, necesitará servir de base a una capa de:

- consulta trazable por agente;
- simulación condicional separada de la historia real;
- devolución de justificación y metadatos de reconstrucción;
- y soporte de interlocución subordinada con IA.

Esta previsión no obliga todavía a abrir:
- nuevas palabras reservadas;
- nuevos nodos IR;
- nuevas reglas de lowering;
- ni nuevas semánticas de ejecución.

## Límites duros

Toda futura capa IA por agente deberá respetar, al menos, estas condiciones:

1. **Primacía humana.**  
   La decisión final relevante permanece reservada al agente humano.

2. **Subordinación al álgebra y a la doctrina.**  
   La IA no constituye fuente de verdad.

3. **Separación REAL / SIM.**  
   La rama simulada no funda por sí sola nuevo SUCESO real.

4. **Historia append-only.**  
   La trayectoria real no se reescribe retrospectivamente.

5. **Primacía del SUCESO.**  
   El tiempo puede medir u ordenar, pero no sustituye al horizonte de sucesos.

6. **No fabricación de certeza.**  
   La IA no puede cerrar `U` por inferencia opaca, extrapolación continua ni automatismo ilegítimo.

7. **Respuesta trazable.**  
   La salida deberá poder reconstruirse mediante respuesta, justificación y metadatos.

## Consecuencia para el lenguaje

La sede técnica del lenguaje debe reservar, desde ahora, espacio conceptual y documental para un futuro contrato mínimo de:

- consulta por agente;
- simulación hipotética trazable;
- serialización distinguible entre historia real y rama simulada;
- y validación de no mezcla entre ambos regímenes.

Mientras ese contrato no se formalice, cualquier implementación puntual en interfaces o demos deberá leerse como **piloto subordinado** y no como cierre del problema.

## Relación con otras sedes

- `SV-matematica-semantica` fija la autoridad doctrinal superior.
- `SV-lenguaje-de-computacion` deberá absorber, cuando proceda, el contrato técnico mínimo.
- `SVperitus-dataset` actúa hoy como primera manifestación piloto de esa necesidad en los agentes especializados.
- `SVcustos-dataset` podrá intervenir en lo observacional cuando el encaje de captura lo exija, pero no como sede directiva de la capa IA.

## Regla final

La futura capa IA del ecosistema SV no se concibe como soberana ni como repositorio aparte por necesidad inmediata.  
Se concibe como **vector transversal subordinado**, cuya masa futura obliga ya a una previsión arquitectónica seria para evitar reingeniería dolorosa.
