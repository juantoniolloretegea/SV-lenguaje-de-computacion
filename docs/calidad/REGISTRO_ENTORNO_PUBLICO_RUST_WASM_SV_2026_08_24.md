# Registro del entorno público Rust/WebAssembly del Lenguaje SV — 24/08/2026

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

## 1. Objeto

Este registro identifica el entorno público Rust/WebAssembly del Lenguaje SV, su relación con la realización nativa y la referencia Python, la identidad del artefacto desplegado y los límites de la evidencia disponible.

El entorno público se encuentra en:

- <https://lenguaje-sv.itvia.online/>

## 2. Arquitectura material

La ejecución pública sigue esta cadena:

```text
texto .svp
   ↓
interfaz web local
   ↓
sv_wasm
   ↓
sv_core::compile_svp
   ↓
proyección diferencial de R0
```

`sv_core` es el mismo núcleo utilizado por el destino Rust nativo. El adaptador WebAssembly no define una gramática, un analizador ni reglas semánticas independientes.

La capa web sirve HTML, CSS, JavaScript de transporte y el módulo WebAssembly. El JavaScript copia el texto a la memoria del módulo, invoca la ABI exportada y presenta el resultado. La distribución web no participa en la decisión semántica.

## 3. Identidad del corte desplegado

```text
fuente
20a1f95cbf1bdbfb4f16cd39335bd71ca1d1c606

sv_wasm.wasm
bytes   = 337366
SHA-256 = 7b49228624f101dc8d863a2b4d631b7ed8eacb4ee4a29c2459d32f6b63aff5dc
```

Versiones expuestas por el módulo:

```text
Gramática   0.2
IR          0.3
Proyección  0.1.0
```

La proyección 0.1.0 es un observable diferencial compartido por los destinos Rust. No se presenta como sustituto del serializador canónico completo de la referencia Python.

## 4. Comprobación funcional pública

Sobre el dominio institucional se comprobó la carga íntegra del módulo y la ejecución local en navegador.

Caso válido:

```text
archivo: cell_basic.svp
resultado: admitido
```

La salida identifica Gramática 0.2, IR 0.3 y proyección 0.1.0, y conserva el SHA-256 de la fuente del caso.

Contraprueba inválida:

```text
entrada: esto no es SVP
resultado: no admitido
diagnóstico técnico: Frontend(Unsupported("esto"))
```

La inadmisibilidad técnica no produjo `Tri.U`.

## 5. Paridad del corpus comprometido

La batería automatizada asociada al mismo corte enumera:

```text
válidos   = 11
inválidos = 61
total     = 72
```

La paridad acreditada en este corte se formula de manera estricta:

```text
mismo texto .svp
+ aceptación/rechazo alineado en Python · Rust nativo · WebAssembly
+ observable textual de proyección idéntico Rust nativo ↔ WebAssembly
≠ identidad textual Python ↔ equivalence_json
```

Resultados:

- 11/11 casos válidos son admitidos por Python, Rust nativo y WebAssembly de navegador;
- 61/61 casos inválidos son rechazados por las tres vías;
- en los casos válidos, Rust nativo y WebAssembly producen el mismo observable textual de proyección;
- la referencia Python coincide con los oráculos canónicos comprometidos y conserva su función diferencial;
- no se acredita identidad textual bit a bit entre la salida Python y `equivalence_json`;
- WASI conserva la misma admisión/rechazo como evidencia complementaria;
- dos reconstrucciones independientes del mismo corte producen bit a bit los mismos ejecutables nativo, WASI y WebAssembly de navegador.

Evidencia pública:

- [PR #22 — R0: WebAssembly y paridad ejecutada de tres vías](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/22);
- [integración `befc666`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/befc666fabe54ecd541416610bf31ddfe776aa69);
- [R0 WASM paridad de tres vías #11](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/32742397555).

## 6. Distribución pública

El dominio institucional es:

```text
lenguaje-sv.itvia.online
```

La URL auxiliar `workers.dev` está desactivada como acceso público normal.

El entorno no requiere para su funcionamiento ordinario:

- cookies;
- analítica;
- traducción de terceros;
- bibliotecas JavaScript externas;
- servicios remotos de compilación;
- almacenamiento local obligatorio.

Los enlaces documentales externos sólo se solicitan cuando el usuario decide abrirlos.

## 7. Conservación del Playground Python/Pyodide

El entorno Python/Pyodide precedente se conserva como antecedente histórico y no como segundo punto de acceso público vigente:

- [registro histórico del Playground Python/Pyodide](../historico/PLAYGROUND_PYTHON_PYODIDE_2026_08_24.md).

La implementación Python permanece en el repositorio como referencia diferencial y como soporte de los oráculos y diagnósticos de conformidad.

## 8. Límites

Este despliegue no acredita más de lo que prueban sus artefactos y casos:

- no existe paridad exacta acreditada de códigos `E***` ni de textos diagnósticos;
- la ejecución en un navegador real no demuestra compatibilidad universal con todos los motores;
- la proyección diferencial no es el serializador canónico completo;
- no se acredita identidad textual bit a bit entre la salida Python y `equivalence_json`;
- la distribución web no constituye autoridad semántica;
- una entrada no admitida o un fallo técnico no se convierten en `Tri.U`;
- el despliegue no prueba las Garantías I o II;
- R0 permanece abierto hasta su cierre integral expreso;
- R1–R4 no se consideran iniciados por este despliegue.

## 9. Estado de integración

La línea Rust/WebAssembly correspondiente al corte identificado fue integrada en `main` el 24/08/2026 mediante la PR #22 y la confirmación `befc666fabe54ecd541416610bf31ddfe776aa69`.

El despliegue público se había materializado previamente sobre el mismo artefacto WebAssembly. La incorporación a `main` establece la referencia integrada del código; no modifica retrospectivamente la identidad del artefacto desplegado ni amplía el alcance de las pruebas descritas en este registro.

La integración de este acto no cierra por sí sola R0 en su conjunto.
