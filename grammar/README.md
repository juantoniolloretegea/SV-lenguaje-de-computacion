# Gramática y parser

**Fecha y Versión: V.1 del conjunto**  
**Fecha:** 4 de abril de 2026  
**Versión del conjunto:** V.1 del conjunto  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Titularidad y autoría:** © Juan Antonio Lloret Egea, 2026. Este conjunto se distribuye con atribución explícita de autoría y bajo la licencia indicada, sin autorización para apropiación de la paternidad intelectual del Sistema Vectorial SV.  

---


Relación entre la gramática superficial vigente y el parser de referencia del lenguaje SV.

**Estado:** la superficie vigente ya está activa; esta carpeta permanece como reserva documental para artefactos formales auxiliares aún no materializados aquí.

## Estado real actual

- La gramática superficial vigente del lenguaje se publica en la raíz del repositorio: `GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md`.
- El frontend activo se implementa en `rust/sv_core/src/frontend.rs`; recibe los perfiles fuente SVP-ES/SVP-EN.
- La cadena efectiva de trabajo sigue siendo: gramática superficial vigente → frontend SV → juicios de buena formación → IR canónica v0.3.

## Función de esta carpeta

Esta carpeta no contiene hoy el parser operativo ni sustituye a la gramática vigente publicada en la raíz.

Su función queda reservada para futuros artefactos auxiliares de formalización de superficie, por ejemplo:

- variantes BNF/PEG de apoyo editorial o técnico;
- notas de correspondencia entre producción superficial y parser;
- o materiales auxiliares de auditoría gramatical.

## Regla

Nada de lo que en el futuro se aloje aquí podrá desplazar por sí solo:

- la gramática superficial vigente publicada en la raíz;
- la realización subordinada a la DSL en `rust/sv_core/`;
- ni la subordinación a la IR canónica y a la frontera normativa.

RETP-082 retira el compilador Python. El [estado anterior](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/a09b9efef51f88de29048b9b35e7ac085dc0918f/grammar/README.md) conserva la descripción histórica; las versiones vigentes y su autoridad documental constan en el [índice principal](../README.md).
