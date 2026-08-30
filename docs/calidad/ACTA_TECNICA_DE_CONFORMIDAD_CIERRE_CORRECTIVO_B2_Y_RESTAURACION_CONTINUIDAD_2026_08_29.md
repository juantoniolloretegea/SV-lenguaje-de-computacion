# Acta técnica de conformidad del cierre correctivo B2 y restauración de continuidad

**Fecha:** 29 de agosto de 2026  
**Repositorio:** `SV-lenguaje-de-computacion`  
**Ámbito:** Gramática canónica 0.2 · IR 0.3 · perfiles fuente `SVP-ES` / `SVP-EN` · realización Rust/WebAssembly  
**Estado:** cierre técnico de conformidad

## 1. Objeto

Esta acta cierra el encargo correctivo abierto por `DFL-007`, acredita nuevamente el perímetro de conformidad gramatical de R0 afectado por esa deuda, revalida la continuidad técnica de R1 sobre la base corregida y levanta la suspensión de R2 que dependía exclusivamente de dicho encargo.

El cierre no amplía el alcance de R0 o R1, no completa R2 y no modifica las Garantías I o II.

## 2. Corte material objeto de cierre

La corrección funcional y su distribución estable quedan integradas en el corte de realización:

```text
realización estable = c1acf943a7a44ce81080881e59283de8a2019606
```

El cierre documental y registral posterior queda constituido en un corte distinto:

```text
cierre interno de Calidad = 8248ec5c2c90e39e5b3798205090facc402d2a88
```

Ambos identificadores cumplen funciones distintas: el primero identifica la realización que produce el WebAssembly corregido; el segundo incorpora las reconciliaciones documentales y registrales posteriores. No se exige que una corrección exclusivamente documental altere o reconstruya el WebAssembly ya acreditado.

La realización WebAssembly publicada queda identificada por:

```text
sv_wasm.wasm
bytes   = 378956
SHA-256 = 95c7d1e0313567ef099c6e426a7fcee8ff4a5ac8adb670265f859f1bf03caab3
```

El paquete estático utilizado para el despliegue manual queda identificado por:

```text
SV_LENGUAJE_PRODUCCION_B2_CLOUDFLARE_2026-08-29_FINAL_CONFORMIDAD.zip
bytes   = 167503
SHA-256 = 566200f97bfea86a0b7ce7c4919bac9d5367a67b8cba719eef1c573942d696f5
archivos = 39
```

La distribución contiene una única representación Base64 comprimida del módulo WebAssembly. La reconstrucción Base64 → gzip → WebAssembly produce exactamente la identidad indicada.

## 3. Corrección de los dominios cerrados

La Gramática 0.2 conserva, por herencia de v0.1, los dominios cerrados:

```ebnf
semantic_relation_kind ::= "DeclaredRelation" ;
pattern_kind           ::= "DeclaredPattern" ;
regime_literal         ::= "Simple" | "General" ;
```

La realización Rust anterior aceptaba valores ajenos en esas tres posiciones. En el caso de `Graph.regime`, un literal ajeno permitía evitar la restricción correspondiente al régimen `Simple`.

La corrección integrada impone los tres dominios sobre la identidad canónica común, después de resolver el perfil fuente y antes de admitir la representación resultante. La bienformación conserva además una comprobación defensiva de esos mismos dominios.

La corrección es única para `SVP-ES` y `SVP-EN`; no existe un validador gramatical independiente por idioma.

## 4. Regresión permanente y navegador

El cierre deja pruebas permanentes para los tres defectos en ambos perfiles fuente y para los valores canónicos permitidos.

La ejecución de navegador real correspondiente a la corrección acreditó:

```text
casos válidos del corpus       = 12/12
casos inválidos del corpus     = 67/67
sondas DG-01/02/03 ES+EN       = 6/6
```

Las seis sondas directas exigen rechazo de:

```text
ForeignRelation / equivalente bajo SVP-ES
ForeignPattern  / equivalente bajo SVP-ES
ForeignRegime   / equivalente bajo SVP-ES
```

La comprobación se ejecutó contra el WebAssembly construido desde el mismo código corregido y no se limita a una inspección estática del analizador.

Evidencia reproducible:

- [PR #55 — cierre de dominios gramaticales y regularización de perfiles fuente](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/55);
- [Conformidad SVP — ejecución 33271992372](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/33271992372);
- [R0 Rust — ejecución 33271992363](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/33271992363);
- [R0-8 Baseline nativa — ejecución 33271992371](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/33271992371);
- [R0 WebAssembly, paridad de tres vías y navegador — ejecución 33271992457](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/33271992457).

## 5. Perfiles fuente y perfil léxico

Queda constituida normativamente la separación entre:

```text
perfil léxico de identificadores
≠
perfil fuente SVP-ES / SVP-EN
```

Los perfiles fuente se definen en `ESPECIFICACION_NORMATIVA_PERFILES_FUENTE_SVP_ES_EN_v1_2026_08_29.md` como capa explícita anterior a la Gramática canónica 0.2:

```text
bytes UTF-8
→ perfil léxico común
→ perfil fuente explícito
→ identidad canónica
→ Gramática 0.2
→ IR 0.3
→ semántica única
```

La realización vigente contiene 154 identidades canónicas, 297 grafías distintas y 11 formas compartidas. La selección de perfil es explícita y no existe autodetección ni caída silenciosa entre perfiles.

La canonicalización no traduce identificadores del usuario, cadenas, comentarios, datos ni nombres de archivo y no altera la huella de los bytes originales de la unidad fuente.

## 6. Reconciliaciones documentales exigidas por DFL-007

### 6.1. DD-01 — cierres internos EBNF

La Gramática 0.2 fija expresamente la forma vigente de `connector_decl` y `table_decl`: los bloques internos `mapping` y `table` cierran con `}` sin un punto y coma adicional antes de la llave de cierre de la declaración.

La reconciliación documenta la forma ya adoptada por el corpus canónico y por Rust. No amplía el lenguaje ni modifica las versiones de Gramática, IR o serializador. La Gramática v0.1 permanece intacta como antecedente histórico.

### 6.2. VH-01 — vector adversarial histórico

El archivo anteriormente denominado `tests/adversarial/deep_nested_query_valid.svp` deja de presentarse como programa válido de la gramática actual en el corte documental de cierre.

Su contenido histórico se conserva en:

```text
tests/adversarial/historico/deep_nested_query_legacy_gramatica_0_1.svp
```

con indicación expresa de que corresponde a Gramática 0.1 y no es válido bajo Gramática 0.2. Esta reclasificación es registral y posterior al corte de realización `c1acf943a7a44ce81080881e59283de8a2019606`; el vector no forma parte del módulo WebAssembly ni condiciona su identidad material.

## 7. Revalidación de R1

La corrección de DG-01/02/03 afecta al perímetro de análisis y bienformación compartido, no a las reglas de autoridad, génesis, requisitos, cobertura, resolución de conflictos, reutilización, permiso, mediación, decisión, ejercicio o trazabilidad constituidas en R1.

La ejecución `R0 Rust` sobre la base corregida ejerció el espacio de trabajo Rust completo y obtuvo:

```text
sv_core                     = 210/210
pruebas de dominios cerrados = 5/5
sv_wasm                     = 2/2
doc-tests sv_core           = 17/17
```

Dentro de las 210 pruebas de `sv_core` se ejecutan las comprobaciones de autoridad, génesis, transición, requisitos, aplicabilidad, cobertura, conflicto, reutilización, permisos, mediación, decisiones selladas, efectos protegidos y trazas de ejercicio que materializan el cierre técnico de R1.

No se observa regresión en ese perímetro. En consecuencia, el cierre técnico de R1 queda revalidado contra la base R0 corregida.

## 8. Estado de DFL-007 y continuidad de fases

Se consideran satisfechas las condiciones de cierre del encargo DFL-007:

1. DG-01, DG-02 y DG-03 están corregidos en Rust;
2. las sondas negativas quedan como regresión permanente;
3. el corpus comprometido permanece verde;
4. la referencia diferencial y Rust conservan el contraste previsto;
5. Rust nativo y WebAssembly permanecen alineados;
6. DD-01 queda reconciliada en la Gramática 0.2;
7. VH-01 queda reclasificada con trazabilidad histórica;
8. el perímetro R0 afectado queda nuevamente acreditado;
9. R1 queda revalidado sobre la base corregida.

Por esta acta se satisface además la condición restante de continuidad: se levanta expresamente la suspensión de R2 causada por DFL-007.

El estado resultante es:

```text
R0 = CERRADO, incluido el perímetro correctivo de DFL-007
R1 = CERRADO y revalidado sobre la base corregida
R2 = ABIERTO; levantada la suspensión específica causada por DFL-007
R3 = NO INICIADO
R4 = NO INICIADO

Garantía I  = NO_PROBADO
Garantía II = NO_PROBADO
```

El levantamiento de la suspensión no constituye cierre de R2 ni acredita persistencia o continuidad material aún no demostradas.

## 9. Reconciliación de la publicación B2

La incorporación estable de B2 fue autorizada y ejecutada mientras DFL-007 permanecía abierta. Esa incorporación no cerró la deuda ni habilitó por sí sola R2. La presente acta separa expresamente ambos hechos:

```text
publicación de B2
≠
cierre de DFL-007
≠
habilitación automática de una fase posterior
```

El cierre de DFL-007 se produce únicamente con la corrección, las regresiones permanentes, las reconciliaciones normativas y la revalidación descritas en este documento.

## 10. Comprobación material del despliegue

El paquete identificado en la sección 2 fue cargado en el entorno público `https://lenguaje-sv.itvia.online/`.

La comprobación material posterior al despliegue confirmó el defecto observable que motivó la corrección de navegación: las rutas española e inglesa del Historial Beta se representan como páginas HTML y ya no muestran el código fuente de la página ni remiten a una representación `blob` del repositorio.

Esta comprobación de despliegue no sustituye la verificación independiente final prevista para el mismo corte.

## 11. Límites y deuda restante

Este cierre no materializa `ConflictOperator` ni completa J2.3 para concurrencia en régimen `General`. Esa deuda permanece registrada separadamente y no se confunde con DG-03.

Tampoco acredita:

- serializador canónico Rust completo;
- paridad diagnóstica textual exacta entre todas las realizaciones;
- compatibilidad universal con todos los motores de navegador;
- R3 o R4;
- Garantía I;
- Garantía II.

Las demás deudas vivas mantienen su estatuto propio.

## 12. Registro de evolución

El hito consolidado de este ciclo se registra como [`RETP-2026-071`](./REGISTRO_EVOLUCION_TECNICA_PROYECTO.md) en el registro de evolución técnica y en su CSV maestro. No se crea un asiento independiente para cada corrección de la misma operación material.

## 13. Dictamen

El perímetro correctivo abierto por DFL-007 queda cerrado. Los tres dominios gramaticales vuelven a coincidir con la Gramática canónica 0.2 en la realización Rust y WebAssembly, los dos perfiles fuente convergen sobre una única comprobación canónica, las discrepancias documentales asociadas quedan reconciliadas y R1 conserva sus propiedades técnicas sobre la base corregida.

R2 recupera su estado abierto previo, sin que esta acta anticipe ni acredite su cierre.

La verificación externa independiente del corte final se registrará, en su caso, mediante un acta separada y no forma parte del presente dictamen.
