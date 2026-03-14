# SV-lenguaje-de-computacion

## Lenguaje de computación del Sistema Vectorial SV

Lenguaje núcleo semántico del Sistema Vectorial SV: pequeño, tipado, auditable y compilable a un backend robusto.

> [!NOTE]
> ### Acceso al emulador online de SVP
> **SVP Playground — Sistema Vectorial SV**
>
> Herramienta online pública del lenguaje SV para escribir código `.svp` directamente en el navegador, parsearlo y obtener la **IR canónica v0.2 en JSON** o el **error correspondiente con su código específico**.
>
> Su función actual es servir como entorno inmediato de prueba y validación del lenguaje: permite comprobar el lowering `.svp → IR v0.2` sin instalación local, con el parser/lowering de referencia cargado desde el propio repositorio y servido mediante GitHub Pages.
>
> No ejecuta todavía el backend: **parsea, valida y serializa**.
>
> **Acceso directo:**  
> https://juantoniolloretea.github.io/SV-lenguaje-de-computacion/

---

### Qué es este proyecto

Este repositorio contiene la especificación, la gramática y la implementación del lenguaje de computación propio del Sistema Vectorial SV. Su propósito no es competir con lenguajes generalistas, sino preservar de forma nativa lo que otros lenguajes solo preservan por disciplina externa.

### Por qué un lenguaje propio

El Sistema Vectorial SV impone invariantes que los lenguajes convencionales no garantizan por construcción:

- El alfabeto `{0, 1, U}` no son enteros ni booleanos: son estados semánticos.
- `U` no es `null`, no es `None`, no es `NaN`: es honestidad epistémica.
- Una célula `SV(n, b)` no es una lista: es una entidad algebraica con restricciones (`n = b²`, `b ≥ 3`).
- Los operadores lícitos están tipados por la relación semántica previa, no son funciones arbitrarias.
- La trayectoria es inmutable y auditable por construcción.

Un lenguaje propio convierte estas exigencias en **errores de compilación**, no en convenciones de estilo.

### Principio rector

> El SV no necesita un lenguaje propio para existir, pero sí puede justificar un lenguaje propio cuando éste preserve de forma nativa lo que otros lenguajes solo preservan por disciplina externa.

---

### Documentos normativos y técnicos vigentes

Estos son los documentos que definen el lenguaje en su estado actual. Los documentos de la raíz y las especificaciones de `spec/` forman una cadena de subordinación descendente.

| Documento | Archivo | Función |
|-----------|---------|---------|
| **Frontera normativa v0** | [`FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md`](FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md) | Establece qué debe conocer, ejecutar, verificar y prohibir el lenguaje. Cuatro bloques: núcleo ejecutor (A), capa formal tipada (B), interfaz analítica (C), zona prohibida (D). |
| **IR canónica v0.2** | [`IR_CANONICA_BIENFORMACION_SV_v0_2.md`](IR_CANONICA_BIENFORMACION_SV_v0_2.md) | Representación intermedia normalizada, serializable y auditable. Cinco niveles ontológicos (N0–N4), juicios de bienformación, catálogo normativo de errores y lowering disciplinado. |
| **Gramática superficial mínima v0.1** | [`GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md`](GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md) | Superficie declarativa del DSL canónico. Declaraciones nominales, seis operadores con lowering unívoco a la IR, enumeraciones cerradas, prohibiciones sintácticas. |
| **Derivación técnica subordinada sobre la U** | [`spec/u_en_svp_semantica_formal_y_regimen_de_resolucion.md`](spec/u_en_svp_semantica_formal_y_regimen_de_resolucion.md) | Traduce al lenguaje de computación el origen doctrinal, la definición canónica y el régimen formal de resolución de `U`, con subordinación expresa a la publicación canónica y al repositorio padre doctrinal. |

La cadena de compilación que estos documentos definen es:

```text
Archivo .svp → Parser → IR canónica v0.2 (JSON) → Backend Rust + Backend Python (referencia) → WASM (futuro)
```

Todos estos documentos están subordinados a los *Fundamentos algebraico-semánticos del Sistema Vectorial SV* (Release 3), que residen en el repositorio padre [SV-matematica-semantica](https://github.com/juantoniolloretegea/SV-matematica-semantica). En caso de discrepancia, prevalecen los *Fundamentos* y la publicación canónica correspondiente en `itvia.online`.

### Especificaciones activas en `spec/`

La carpeta [`spec/`](spec/) reúne derivaciones técnicas subordinadas, útiles para fijar semántica formal, restricciones de resolución y condiciones de implementación del lenguaje.

En el estado actual del repositorio, su pieza activa principal es:

- [`spec/u_en_svp_semantica_formal_y_regimen_de_resolucion.md`](spec/u_en_svp_semantica_formal_y_regimen_de_resolucion.md)

Esta pieza no redefine la U del sistema. La proyecta técnicamente al lenguaje SVP y la mantiene subordinada a:

1. la publicación canónica en `itvia.online`;
2. el repositorio doctrinal `SV-matematica-semantica`;
3. la frontera normativa, la IR y la gramática del lenguaje.

### Carpeta `beta/`

La carpeta [`beta/`](beta/) reúne materiales de proposición, vigilancia y laboratorio relacionados con el desarrollo del lenguaje de computación del Sistema Vectorial SV.

Su función no es sustituir al núcleo doctrinal del proyecto ni autorizar por sí sola cambios de IR, semántica, gramática o implementación.

En el estado actual, `beta/` se organiza en dos planos internos:

- [`beta/C1_proposiciones/`](beta/C1_proposiciones/) → proposiciones de trabajo y matrices operativas de alcance metodológico.
- [`beta/Lab.SV/`](beta/Lab.SV/) → materiales experimentales o de frontera alojados expresamente sin adopción automática de rango canónico.

Actualmente constan en `beta/` las siguientes piezas activas:

- [`beta/C1_proposiciones/matriz_operativa_completa_clasificador_sv.md`](beta/C1_proposiciones/matriz_operativa_completa_clasificador_sv.md)
- [`beta/Lab.SV/desde_la_terna_0_1_u_hasta_la_nueva_frontera_0_1_u_0_1.md`](beta/Lab.SV/desde_la_terna_0_1_u_hasta_la_nueva_frontera_0_1_u_0_1.md)
- [`beta/README.md`](beta/README.md)

La existencia de piezas en espejo entre `beta/` y `SV-matematica-semantica` no altera la jerarquía del sistema. La prioridad doctrinal corresponde al repositorio `SV-matematica-semantica` y, por encima de él, a la publicación canónica de `itvia.online`.

### Extensión de archivo

Los programas escritos en el lenguaje SV usan la extensión **`.svp`** (Sistema Vectorial Poligonal). El nombre remite a la imagen poligonal cerrada, invariante constitutivo del Sistema Vectorial SV.

### Estado actual

- Parser/lowering de referencia: activo.
- IR canónica v0.2: activa.
- Gramática superficial mínima v0.1: activa.
- Catálogo de errores concordado con la implementación: activo.
- Derivación técnica subordinada de la U: activa.
- Carpeta `beta/` activada con plano de proposiciones y plano de laboratorio experimental alojados bajo control jerárquico.
- Backend soberano en Rust: objetivo fijado, no cerrado.

### Regla de jerarquía

Este repositorio no constituye la autoridad doctrinal suprema del Sistema Vectorial SV. Su función es implementar, verificar y proyectar técnicamente la doctrina. En caso de contradicción:

1. prevalece la publicación canónica en `itvia.online`;
2. prevalece el repositorio padre doctrinal `SV-matematica-semantica`;
3. y solo después rigen los textos técnicos e implementativos de este repositorio.

---

## Licencia

Este repositorio se distribuye bajo la licencia indicada en el archivo [`LICENSE`](LICENSE).
