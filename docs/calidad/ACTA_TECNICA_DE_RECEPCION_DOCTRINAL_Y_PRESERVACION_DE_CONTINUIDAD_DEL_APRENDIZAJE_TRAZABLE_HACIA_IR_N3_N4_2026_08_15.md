# Acta técnica de recepción doctrinal y preservación de continuidad del aprendizaje trazable hacia IR N3/N4

**Fecha:** 15/08/2026  
**Naturaleza:** acta de gobierno técnico, recepción doctrinal y preservación controlada de continuidad  
**Frente:** Lenguaje SV / aprendizaje trazable / continuidad futura de IR N3–N4  
**Estado:** cerrada para recepción; continuidad técnica latente  
**Sede doctrinal de origen:** `SV-matematica-semantica/documentos/fundamentos/`  
**Sede técnica receptora:** `SV-lenguaje-de-computacion`  
**Rama técnica reservada:** `traceable-learning-ir-v0.1`

## 1. Objeto

La presente acta deja constancia de que la publicación de fundamentos actualmente titulada **“Traceable Learning in Artificial Intelligence: Structural Knowledge Evolution with Ternary Frames and Cumulative Traces”** introduce un aparato formal de aprendizaje trazable que, aun perteneciendo doctrinalmente a la sede superior `SV-matematica-semantica`, ejerce presión legítima y directa sobre la continuidad futura del Lenguaje SV.

El propósito de esta acta no es integrar ese aparato en el lenguaje vigente. Su propósito es impedir dos errores simétricos:

1. que una futura unidad ignore la publicación al continuar IR, N3, N4, consultas o DSL;
2. que la publicación sea retroproyectada automáticamente sobre `main` como si ya constituyera sintaxis, IR, validator, runner o capacidad productiva del Lenguaje SV.

## 2. Jerarquía aplicable

Se mantiene íntegramente la fijación de sedes vigente:

- `SV-matematica-semantica` es la sede superior doctrinal y normativa;
- `SV-lenguaje-de-computacion` es la sede operativa y técnica;
- el movimiento descendente legítimo es `doctrina → especificación → lenguaje → pruebas → evidencia`;
- ninguna sede técnica modifica por silencio la doctrina superior.

La publicación de aprendizaje trazable deberá, cuando se publique en el corpus propio, residir en `SV-matematica-semantica/documentos/fundamentos/`. Su presencia en la sede superior no equivale a integración técnica automática.

## 3. Hecho doctrinal nuevo que debe conservar el Lenguaje SV

La publicación formaliza, de manera autocontenida, entre otros elementos:

- asociación explícita entre frame/vector manifestado y traza estructural acumulativa;
- conocimiento manifestado tipado en contenidos, relaciones y rutas;
- ledger cognoscitivo append-only distinto de la proyección activa de conocimiento;
- reproducción ordenada y determinista del ledger;
- episodios finitos anclados a dominio y fundamento declarados;
- frescura histórica que distingue adquisición de recuperación;
- soporte certificado mediante testigos finitos registrados;
- separación entre evolución, incremento y aprendizaje;
- separación formal entre aprendizaje, razonamiento, composición, crecimiento cardinal, verdad y mera extensión de traza;
- veredicto operacional ternario de aprendizaje, con `U` únicamente después de agotamiento declarado y sin confundir consulta incompleta con negación histórica;
- invariancia de la fundación humana frente a secuencias de transiciones máquina admisibles;
- distinción entre reconstrucción de estado y reconstrucción de procedencia.

Estos objetos no se declaran aquí como parte del Lenguaje SV vigente. Se registran como doctrina nueva que deberá ser traducida formalmente antes de cualquier futura incorporación.

## 4. Impacto preliminar sobre la IR vigente

La IR canónica v0.2 sitúa en N3 —Evolución— `Frame`, `TransitionData`, `Trajectory` y `Horizon`, y en N4 —Uso— `Domain`, `Agent`, `QuerySpec`, `QueryContext`, `QueryResult` y `AnalyticView`.

La publicación afecta potencialmente de manera directa a ambos niveles:

### 4.1. Presión futura sobre N3

Deberá estudiarse, sin integración todavía:

- anclaje del ledger cognoscitivo a frames y entradas de trayectoria;
- preservación de historia previa al episodio para distinguir frescura de recuperación;
- identificación explícita de límites ordinales de episodio;
- referencias inmutables a la versión de la fundación bajo la que se constituyó el episodio;
- representación de ejecuciones de razonamiento y composición cuando formen parte de un soporte registrado;
- correspondencia entre trayectoria de sistema y traza cognoscitiva sin convertir ambas en un único objeto.

### 4.2. Presión futura sobre N4

Deberá estudiarse, sin integración todavía:

- una consulta tipada de aprendizaje sobre episodio declarado;
- representación de `Evol`, `Inc` y `Learn` como semántica de consulta, no como heurística;
- soporte finito, política de soporte y completitud relativa a una frontera declarada;
- distinción entre proposición histórica de aprendizaje y veredicto operacional `LEARN / NO_LEARN / U`;
- agotamiento declarable y comprobable antes de emitir `U`;
- justificación y procedencia de incrementos, relaciones y rutas;
- versionado de fundamento humano y prohibición de cruce silencioso de versiones dentro de un mismo episodio.

### 4.3. Presión posible sobre N0/N2

La fundación humana declarada combina elementos de especificación y dominio que podrían exigir, en una futura traducción, nuevos objetos o referencias de N0 y nuevos resultados tipados de N2. Esta acta no fija todavía dónde deben residir tales objetos ni altera la ontología actual de niveles.

## 5. Lo que esta acta no autoriza

Queda expresamente **no autorizado** por esta acta:

- modificar `IR_CANONICA_BIENFORMACION_SV_v0_2.md`;
- añadir sintaxis superficial al DSL;
- alterar gramática, AST, parser o lowering;
- modificar `svp_validator.py`, runner, CLI, Playground o backend;
- introducir nuevos códigos de error canónicos;
- convertir el checker de la publicación en componente del Lenguaje SV;
- adoptar automáticamente los nombres `Know`, `Ledger`, `Episode`, `Learn`, `Inc`, `Evol` o cualesquiera otros como nombres finales de IR;
- fusionar la rama técnica reservada en `main` sin una futura autorización arquitectónica expresa.

## 6. Estatuto del checker asociado a la publicación

El checker de referencia de la publicación queda entendido exclusivamente como **realización finita de ejemplos, seguimiento y regresión de las definiciones publicadas**.

No es fuente de validez de los teoremas, no sustituye las demostraciones y no constituye por sí mismo implementación del Lenguaje SV. Sus casos adversariales podrán reutilizarse en el futuro como banco de requisitos o fixtures de traducción, previa revisión de tipado y alcance.

## 7. Decisión de preservación técnica

Se autoriza la creación y conservación de la rama:

`traceable-learning-ir-v0.1`

La rama queda clasificada como:

**`LATENTE_LEGITIMO / VIGILANCIA_DOCTRINAL_ACTIVA`**

Su función será conservar un mapa de impacto y continuidad para cuando el proyecto reabra formalmente IR/N3/N4 y el DSL correspondiente.

La rama no es una implementación alternativa, no compite con `main` y no debe fusionarse automáticamente. Su contenido inicial será documental y de arquitectura prospectiva, sin modificación de código ejecutable.

## 8. Condición de reapertura implementativa

Antes del primer commit que pretenda trasladar materialmente el aprendizaje trazable al Lenguaje SV deberá existir un acta arquitectónica previa que, como mínimo, cierre:

- versión doctrinal exacta de la publicación que se toma como fuente;
- objetos que se incorporan y objetos que permanecen externos;
- ubicación tipada en N0–N4;
- reglas de lowering y bienformación;
- relación exacta entre `Trajectory` y ledger cognoscitivo;
- tratamiento de historia anterior al episodio;
- semántica de soporte y completitud relativa;
- semántica operacional de `U` para consultas de aprendizaje;
- versionado e identidad de la fundación humana;
- preservación de REAL/SIM, precedencia y custodia estructural;
- suite positiva, negativa y adversarial;
- compatibilidad hacia atrás y plan de reversión;
- criterio de cierre para autorizar, en su caso, integración posterior en `main`.

## 9. Dependencia respecto de la publicación externa

Mientras la publicación destinada a *Journal of Automated Reasoning* no esté editorialmente congelada, la rama técnica deberá seguir la **semántica final publicada**, no borradores intermedios ni implementaciones auxiliares.

La futura incorporación del DOI de Code Ocean o modificaciones meramente editoriales no autorizan cambios técnicos por sí mismas. Solo una modificación semántica de la publicación que afecte a las definiciones o resultados aquí enumerados obliga a revisar el mapa de continuidad.

## 10. Cierre

**Recepción doctrinal por el Lenguaje SV:** SÍ.  
**Integración automática en `main`:** NO.  
**Creación de rama de continuidad:** SÍ.  
**Cambio inmediato de IR/DSL/código:** NO.  
**Rango de la publicación:** fundamentos en sede doctrinal superior.  
**Impacto técnico futuro:** principalmente N3/N4, con posible presión posterior sobre N0/N2.  
**Reapertura futura:** condicionada a acta arquitectónica previa.

Se preserva así el principio rector del ecosistema: la doctrina nueva comparece y deja huella técnica, pero su traducción al lenguaje solo se realiza después de tipado, auditoría y autorización explícita.