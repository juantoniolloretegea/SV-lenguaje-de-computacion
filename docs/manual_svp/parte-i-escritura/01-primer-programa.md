# **1.** Preparar y comprobar un programa

[Lectura en la biblioteca](https://documentos-sv.itvia.online/desarrolladores/parte-i-escritura/01-primer-programa.html) · [Enciclopedia temática](https://documentos-sv.itvia.online/enciclopedia/)

[Índice del manual](../README.md) · [Siguiente](02-declaraciones-y-estados.md)

## **1.1.** Qué va a aprender

El objetivo inicial es escribir un archivo SVP, elegir su perfil lingüístico y comprobar su admisión. El resultado permite estudiar las declaraciones y operaciones que reconoce el lenguaje. Escribir en la DSL del SV y desarrollar su implementación en Rust son actividades diferentes.

Puede comenzar en el [Playground del SV](https://lenguaje-sv.itvia.online/) sin instalar herramientas. Las lecciones utilizan los archivos de referencia SVP-EN del repositorio; sus palabras reservadas están en inglés y las explicaciones se ofrecen en español. El perfil SVP-ES dispone de sus propias formas de escritura. El idioma de la página no cambia el perfil del programa.

## **1.2.** Primera comprobación en el navegador

1. Abra el Playground y anote las versiones indicadas en su cabecera.
2. Seleccione **SVP-EN**. Cargue el archivo completo de la [lección de estados celulares](02-declaraciones-y-estados.md).
3. Pulse **Compilar** y lea el diagnóstico antes de interpretar cualquier representación.
4. Si se admite, localice los objetos `K3`, `Klin`, `C1` y `S1`, y la operación `E1`.
5. Conserve el archivo junto con el perfil y las versiones utilizadas para poder repetir la comprobación.

**Admitido** significa que el archivo supera las comprobaciones implementadas por esa distribución. La presencia de `E1` representa una operación solicitada; no acredita que se haya obtenido un dictamen ni una inferencia experta.

## **1.3.** Comprobación nativa del mismo archivo

Con el ejecutable `sv-native` correspondiente a la edición, desde la carpeta donde guardó el archivo:

```text
sv-native --profile en cell_basic.svp
```

En Windows, si el ejecutable está en esa misma carpeta, utilice `.\sv-native.exe --profile en cell_basic.svp`. En Linux, `./sv-native --profile en cell_basic.svp`. Las dos formas indican la ubicación del programa; no cambian el lenguaje.

El ejecutable entrega una representación JSON con objetos y operaciones. Para consultar su identificación:

```text
sv-native manifiesto-sv
```

La [referencia de herramientas del libro](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/herramientas-desarrollo-comprobacion.html) explica la construcción desde fuentes para quien la necesite. Compilar la implementación requiere Rust; utilizar un ejecutable compatible para comprobar SVP no exige escribir Rust.

## **1.4.** Interpretar el resultado

| Resultado del proceso nativo | Significado |
| --- | --- |
| Código de salida `0` y representación JSON | Archivo admitido por el comprobador. |
| Código de salida `1` y diagnóstico | Programa no admitido; revise la causa indicada. |
| Código de salida `2` | Problema de invocación, perfil o lectura del archivo. |

Estos códigos pertenecen al proceso del sistema operativo. La terna celular tiene su propio significado: **0 = Apto, 1 = No apto, U = Indeterminado**. No se traducen unos en otros.

Las comprobaciones de esta edición emplean gramática 0.2, IR 0.3 y proyección 0.1.0. La [identificación de ejemplos](../EDICION.json) permite reproducirlas con la revisión indicada. Si otra distribución ofrece un resultado distinto, conserve ambas identificaciones y examine la diferencia antes de cambiar el programa.

---

[Índice del manual](../README.md) · [Siguiente](02-declaraciones-y-estados.md)

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
