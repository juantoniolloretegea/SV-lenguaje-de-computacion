# **7.** Ejercicios, soluciones y diagnósticos

[Lectura en la biblioteca](https://documentos-sv.itvia.online/desarrolladores/parte-iii-ejercicios/07-ejercicios-y-diagnosticos.html) · [Enciclopedia temática](https://documentos-sv.itvia.online/enciclopedia/)

[Índice del manual](../README.md) · [Anterior](../parte-ii-relaciones/06-sucesos-y-consultas.md) · [Siguiente](08-programas-completos.md)

## **7.1.** Método de comprobación

Conserve el ejemplo original. Escriba una variación con otra denominación, anticipe el resultado y compruebe el archivo completo. Compare el diagnóstico y la representación; no interprete un fallo técnico como un valor de la terna.

| Ejercicio | Archivo | Resultado esperado |
| --- | --- | --- |
| Cambiar la primera posición de `Zero` a `One` | [01-cambiar-posicion.svp](../ejercicios/01-cambiar-posicion.svp) | Admitido; nueve posiciones y primer valor `One`. |
| Renombrar el estado y su referencia | [02-renombrar-estado.svp](../ejercicios/02-renombrar-estado.svp) | Admitido; `EstadoInicial` aparece como objeto y como entrada de `E1`. |
| Escribir el ejemplo celular en SVP-ES | [03-celula-es.svp](../ejercicios/03-celula-es.svp) | Admitido con perfil `es`; conserva objetos y operaciones del ejemplo EN. |

Para la tercera variación:

```text
sv-native --profile es 03-celula-es.svp
```

El cambio de perfil afecta a palabras reservadas, como `cellspec` y `especificación_de_celda`. Los identificadores y textos de salida no se traducen por cambiar el idioma de la interfaz. Las propiedades de procedencia del archivo cambian; la comparación de significado utiliza sus objetos y operaciones.

## **7.2.** Un rechazo deliberado

El archivo [cellstate_vector_length_mismatch.svp](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/tests/conformance/invalid/cellstate_vector_length_mismatch.svp) utiliza una especificación de nueve posiciones con un vector de ocho. Su propósito es comprobar el diagnóstico, y debe mantenerse separado de los ejemplos válidos.

En la revisión de esta edición, el comprobador devuelve código de salida `1` y:

```text
SVP no admitido: InvalidProgram("CellState S1: longitud de vector incompatible")
```

Para corregir el programa es necesario aportar el estado completo conforme a la especificación. Añadir un valor arbitrario únicamente para superar la comprobación no justifica su significado. El ejemplo válido permite comparar la estructura exigida.

## **7.3.** Soluciones y comprobación del aprendizaje

- **Cambio de posición:** únicamente cambia el primer elemento del vector; se conservan la base, los identificadores y el resto de posiciones.
- **Cambio de nombre:** `EstadoInicial` sustituye a `S1` en la declaración y en `evaluate`. Cambiar sólo uno dejaría una referencia sin correspondencia.
- **Perfil español:** `codominio`, `semántica_de_salida`, `especificación_de_celda`, `estado_de_celda` y `evaluar` permiten expresar el mismo ejemplo en SVP-ES. Debe seleccionarse explícitamente ese perfil.

Antes de avanzar, explique con sus propias palabras por qué un programa admitido puede contener una operación todavía no ejecutada y por qué un diagnóstico de archivo no equivale a declarar una célula «No apta».

Fundamento: [conformidad y diagnósticos](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/conformidad-diagnosticos.html). Las huellas y los resultados de los archivos de esta edición se reúnen en [EDICION.json](../EDICION.json).

---

[Índice del manual](../README.md) · [Anterior](../parte-ii-relaciones/06-sucesos-y-consultas.md) · [Siguiente](08-programas-completos.md)

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
