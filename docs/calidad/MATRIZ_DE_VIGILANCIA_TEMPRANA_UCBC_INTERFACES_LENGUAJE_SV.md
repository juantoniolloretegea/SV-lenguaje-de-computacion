# Matriz de vigilancia temprana UCBC — interfaces publicadas y proyectadas ↔ lenguaje SV

## Finalidad

Esta matriz registra, en una sola superficie auditable, la vigilancia temprana coordinada entre el bloque del Programa de interfaces y el repositorio `SV-lenguaje-de-computacion`, con el único objeto de distinguir:

- qué frente o publicación emite una señal relevante para el lenguaje;
- qué plano podría verse afectado;
- si el impacto es solo de vigilancia o si existe contradicción local acreditada;
- y qué tratamiento corresponde en el estado actual.

## Regla de uso

Esta matriz no autoriza por sí sola a modificar la IR, la gramática, el validator, el runner ni el runtime. Su función es ordenar vigilancia coordinada sin convertirla todavía en deuda viva ni en parche funcional.

## Criterio de lectura

La situación mínima de cada fila será una de estas:

- `sin_impacto_inmediato`
- `vigilancia_puertos`
- `vigilancia_admisibilidad`
- `vigilancia_u_honesta`
- `vigilancia_expresion_controlada`
- `contradiccion_local_acreditada`

## Estado

**Estado:** apertura técnica de vigilancia coordinada  
**Fecha de trabajo:** 21/03/2026  
**Base de contraste:** publicaciones visibles del Programa de interfaces + repo fresco de `SV-lenguaje-de-computacion` + repo fresco de `SV-matematica-semantica`

|Frente_o_Publicacion|Estado_de_Fase|Hecho_Publico_Constatado|Plano_del_Lenguaje_Potencialmente_Afectado|Riesgo_Principal|Situacion|Tratamiento_Recomendado|Observaciones|
|---|---|---|---|---|---|---|---|
|Semántica auditada|publicado|Formaliza sucesos, transducción ternaria, clausura trazable e indeterminación estructural `U`|consulta / clausura / trazabilidad|sobreatribución de cierre fuerte fuera de condiciones|sin_impacto_inmediato|mantener como base superior ya asumida|no genera deuda nueva; es marco ya integrado|
|Visión|publicado|Abre captura visual, ternarización y trayectoria sin romper semántica canónica|CaptureSpec / AdmissibilitySpec / trayectorias futuras|confundir carril publicado con obligación inmediata de implementación|vigilancia_puertos|vigilar sin cambio inmediato|referencia útil para futuros carriles, no para parche actual|
|Movilidad|publicado|Fija frontera mínima entre trayectoria interna y exposición externa|trayectoria / exposición / consultas futuras|extrapolación indebida al runtime actual|sin_impacto_inmediato|mantener como doctrina de interfaces|sin contradicción local observable con ABI mínimo|
|Corpus observacional tipado|publicado e integrado|Consta en sede doctrinal con carril propio integrado (`observacional`) y marco explícito de fuente, estructura observable, derivados verificables y objeto consultable|CaptureSpec / AdmissibilitySpec / QueryContext|presión impropia para formalizar artefactos externos como ontología rival|vigilancia_admisibilidad|mantener vigilancia prudente|la integración doctrinal refuerza su peso metodológico, no exige revisión inmediata de IR|
|Olfato|publicado e integrado|Consta en sede doctrinal con carril propio integrado (`olfato`) y formalización de admisibilidad olfativa e indeterminación intermodal|AdmissibilitySpec / `U` / mezclas de canal|degradación de `U` o cierre precipitado ante mezcla|vigilancia_u_honesta|vigilar tratamiento futuro de mezcla y refutación|la integración doctrinal refuerza su papel como precedente metodológico, no como integración material inmediata al lenguaje|
|Oído|proyectado|Se perfila como siguiente carril de interfaz con potencial tratamiento de canal, mezcla y señal temporal sin tiempo fuerte|CaptureSpec / AdmissibilitySpec / referencias de canal|introducir temporalidad soberana o ruido como pseudoestado|vigilancia_puertos|esperar manuscrito y delimitación negativa fuerte|no procede deuda nueva antes de materialización y contraste|
|Tacto|proyectado|Se perfila como carril posterior con potencial tensión entre contacto, frontera y exposición|CaptureSpec / Domain / exposición|confundir tactilidad con ontología nueva o con cierre motor impropio|vigilancia_puertos|esperar manuscrito y delimitación negativa fuerte|sin efecto inmediato sobre IR actual|
|Células especializadas|proyectado|Se perfila como laboratorio controlado de expresión estructural del lenguaje aplicado sobre carriles ya publicados|examples / QuerySpec / QueryContext / legibilidad pública|convertir el laboratorio en presión funcional o normativa sobre N4/Uso|vigilancia_expresion_controlada|permitir solo como laboratorio no normativo y no invasivo|debe poder eliminarse sin dejar rastro normativo en el lenguaje|
|NLP (frente proyectado / activación pendiente)|proyectado|Se perfila una posible cadena auditada de ruido, admisibilidad del canal y reexpresión fiel|QuerySpec / QueryContext / referencias a canal|convertir ruido lingüístico en semántica rival o exigir reforma prematura|vigilancia_puertos|esperar activación y primer dictamen de W-NLP1 SV|sin deuda viva nueva hasta contradicción acreditada|
|UCBC (horizonte coordinador)|proyectado|Se identifica necesidad futura de coordinación transversal de frentes hacia el lenguaje|gobierno técnico / contratos / registros|mezclar coordinación con especificación operativa|vigilancia_puertos|mantener como función coordinadora y no normativa|colofón futuro, no cambio actual de IR|
