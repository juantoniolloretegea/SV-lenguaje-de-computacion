# C-1.C — Decisión sobre la regularización del contrato diagnóstico del frontend de referencia

## 1. Naturaleza y subordinación

Este documento tiene carácter operativo, técnico y subordinado.
No modifica ni sustituye la autoridad doctrinal vigente del Sistema Vectorial SV, que reside en el repositorio `SV-matematica-semantica` y en la cadena normativa ya fijada.

Su objeto es decidir cómo debe regularizarse la fractura actualmente constatada entre:

- la IR canónica v0.2;
- el catálogo implementado en `src/svp_errors.py`;
- el `validator`;
- el `runner`;
- y la documentación pública afectada.

## 2. Principio rector

El lenguaje SV no puede perder su esencia, su personalidad ni su rumbo.
Si el contrato diagnóstico deriva sin control, el lenguaje deja de recordar qué representa, qué pretende y cuál es su objetivo real.

Por ello, la regularización del catálogo de errores no es una cuestión cosmética ni meramente documental: afecta al eje de identidad técnica del lenguaje y a su capacidad de seguir siendo gobernado por su propia norma.

## 3. Marco de autoridad aplicable

La decisión debe interpretarse conforme a este orden:

1. doctrina y cadena normativa vigente del Sistema Vectorial SV;
2. especificación operativa del lenguaje ya cerrada;
3. implementación de referencia;
4. gobierno técnico del desarrollo.

Además, debe respetarse la **Guía práctica del conocimiento profundo y la crítica de la razón pura**, distinguiendo correctamente entre reflexión no normativa, fundamentos inamovibles, prospectiva, directrices operativas y reglas inamovibles.

## 4. Hecho acreditado

La microauditoría C-1.A y la matriz exhaustiva C-1.B han acreditado que la desalineación entre IR y catálogo implementado ya no es puntual, sino estructural.

Consta:

- divergencia amplia entre la tabla de errores de la IR y `src/svp_errors.py`;
- uso activo, por `validator` y `runner`, de códigos cuyo significado ya no coincide con la IR;
- insuficiencia del documento puente previo;
- y desajuste de parte de la documentación pública respecto del estado real del frontend.

## 5. Vías posibles

### Vía A — Reconducción de la implementación hacia la IR v0.2

#### Descripción
La implementación de referencia recupera la numeración y semántica de errores fijadas por la IR v0.2.

#### Ventajas
- restablece primacía fuerte de la IR;
- elimina la fractura de contrato en su raíz;
- deja un sistema normativamente más limpio;
- reduce ambigüedad futura para backend y tooling.

#### Riesgos
- alto coste de arrastre inmediato;
- necesidad de tocar `svp_errors.py`, `validator`, `runner`, tests y documentación pública;
- posible rotura masiva de la suite vigente;
- riesgo de abrir demasiados frentes a la vez si se ejecuta sin fase intermedia.

#### Juicio
Es una vía doctrinalmente fuerte, pero operativamente agresiva en el punto actual del proyecto.

### Vía B — Catálogo implementativo provisional explícito y controlado

#### Descripción
Se reconoce formalmente que el frontend de referencia usa hoy un catálogo implementativo propio, distinto del de la IR v0.2. Ese catálogo se documenta de forma completa, se deja trazada la deuda de convergencia y se prepara una posterior reconciliación ordenada.

#### Ventajas
- estabiliza el estado real del compilador sin fingir concordancia inexistente;
- permite documentar el contrato efectivo que hoy usa la suite;
- evita una renumeración precipitada y costosa;
- permite seguir avanzando sin perder control.

#### Riesgos
- si se hace mal, puede parecer una claudicación frente a la IR;
- puede consolidar demasiado el desvío si no se deja trazada la convergencia;
- obliga a extremar la claridad documental para no crear una segunda soberanía normativa.

#### Juicio
Es una vía más prudente, siempre que se formule como regularización provisional subordinada, no como independencia de la implementación respecto de la norma.

## 6. Decisión

La decisión correcta en este punto del proyecto es:

> **adoptar la Vía B de forma inmediata, explícita y controlada, manteniendo la Vía A como horizonte de convergencia obligada.**

Dicho de forma más precisa:

- no se declara que la implementación actual haya pasado a ser la nueva autoridad normativa;
- sí se declara que el frontend de referencia trabaja hoy con un catálogo implementativo efectivo que debe ser documentado de manera íntegra y honesta;
- sí se deja fijado que la convergencia futura hacia la norma superior sigue siendo una obligación del proyecto, no una opción olvidable.

## 7. Sentido de la decisión

Esta decisión no se toma por comodidad, sino por disciplina.

La Vía B se adopta porque:

1. la fractura ya es estructural y debe dejar de esconderse;
2. la suite actual ya está verde sobre el contrato implementativo, no sobre la IR;
3. una reconducción total inmediata introduciría un coste y un riesgo de arrastre excesivos;
4. el proyecto necesita primero recuperar gobierno del contrato diagnóstico antes de reordenarlo por completo.

## 8. Efectos inmediatos de la decisión

La adopción de la Vía B implica:

1. documentar de forma completa el catálogo implementativo vigente;
2. corregir afirmaciones públicas que hoy ya no se sostienen materialmente;
3. sustituir el documento puente parcial previo por una regularización exhaustiva;
4. dejar trazada expresamente la deuda de convergencia con la IR v0.2;
5. impedir que la implementación siga derivando silenciosamente.

## 9. Artefactos que deberán derivarse de esta decisión

Sin abrirlos todavía por inercia, esta decisión orienta la creación o revisión de los siguientes artefactos:

- un documento canónico humano del catálogo, cuyo nombre preferente queda fijado como `ERRORES_CANONICOS_SV_v0_2.md`;
- una actualización o sustitución del documento puente de concordancia;
- la revisión de `src/svp_errors.py`;
- la revisión del `README.md` y documentación de tests afectada;
- y, más adelante, una fuente ejecutable derivable del catálogo canónico si el proyecto la precisa.

## 10. Sobre futuros formatos y objetivos

No procede abrir ahora un formato propio `.cat`.
Sí procede dejarlo como posibilidad futura, igual que el backend soberano en Rust o una eventual línea en C.

Ese tipo de elementos debe figurar en un documento de objetivos a futuro y no mezclarse con la regularización inmediata del contrato diagnóstico.

## 11. Regla de continuidad

La presente decisión será aplicable, por analogía metodológica, a otros proyectos del ecosistema `SVcustos / SVperitus` cuando exista tensión entre:

- norma superior;
- contrato implementado;
- documentación pública;
- y gobernanza técnica del desarrollo.

La brújula seguirá siendo siempre el repositorio y su jerarquía, no la memoria conversacional ni la improvisación de una unidad concreta.

## 12. Criterio de cierre de esta decisión

Esta decisión se considerará correctamente ejecutada cuando consten, al menos, estas cuatro cosas:

1. reconocimiento formal del catálogo implementativo efectivo;
2. documentación completa y no parcial de su estado real;
3. corrección de las afirmaciones públicas materialmente inexactas;
4. trazado explícito de la futura convergencia hacia la norma superior.

## 13. Fórmula final

El proyecto no preserva su identidad fingiendo una concordancia inexistente, sino reconociendo con exactitud dónde está, bajo qué autoridad debe corregirse y con qué orden debe hacerlo.

La Vía B se adopta ahora para recuperar control.
La Vía A permanece como horizonte de reconducción normativa.
La autoridad superior no cambia.
La implementación no se emancipa.
El repositorio sigue siendo la brújula.
