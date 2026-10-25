# **3.** Compuertas y tablas de admisibilidad

[Lectura en la biblioteca](https://documentos-sv.itvia.online/desarrolladores/parte-ii-relaciones/03-compuertas.html) · [Enciclopedia temática](https://documentos-sv.itvia.online/enciclopedia/)

[Índice del manual](../README.md) · [Anterior](../parte-i-escritura/02-declaraciones-y-estados.md) · [Siguiente](04-resolucion-u.md)

## **3.1.** Relacionar dos evaluaciones

Una compuerta utiliza una tabla declarada para relacionar resultados de evaluación. Este ejemplo define dos especificaciones, dos estados y una tabla de dos entradas. Estudie primero sus tipos y después las nueve filas de la tabla.

```svp
-- gate_table.svp — Compuerta con tabla explícita de admisibilidad

codomain K3 = { APTO, NO_APTO, INDETERMINADO };

output_semantics Klin {
  APTO -> "ok";
  NO_APTO -> "no";
  INDETERMINADO -> "u";
}

cellspec C1 {
  b: 3;
  codomain: K3;
  semantics: Klin;
  role: Base;
}

cellspec C2 {
  b: 3;
  codomain: K3;
  semantics: Klin;
  role: Base;
}

cellstate S1 {
  spec: C1;
  vector: [Zero, One, U, Zero, Zero, One, U, Zero, One];
}

cellstate S2 {
  spec: C2;
  vector: [Zero, Zero, Zero, One, Zero, Zero, Zero, One, Zero];
}

admissibility_table T1 {
  input_codomains: [K3, K3];
  output_codomain: K3;
  table: {
    (APTO, APTO) -> APTO;
    (APTO, NO_APTO) -> NO_APTO;
    (APTO, INDETERMINADO) -> INDETERMINADO;
    (NO_APTO, APTO) -> NO_APTO;
    (NO_APTO, NO_APTO) -> NO_APTO;
    (NO_APTO, INDETERMINADO) -> NO_APTO;
    (INDETERMINADO, APTO) -> INDETERMINADO;
    (INDETERMINADO, NO_APTO) -> NO_APTO;
    (INDETERMINADO, INDETERMINADO) -> INDETERMINADO;
  }
}

let E1 = evaluate(S1);
let E2 = evaluate(S2);
let G1 = gate([E1, E2], using: T1);
```

Archivo: [gate_table.svp](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/tests/conformance/valid/gate_table.svp).

## **3.2.** Recorrer el programa

`S1` y `S2` pertenecen a `C1` y `C2`. Ambas especificaciones utilizan `K3`, por lo que las entradas declaradas de `T1` son `[K3, K3]`. La tabla expresa qué salida se asigna a cada pareja de valores del ejemplo; no debe deducirse una tabla alternativa desde los textos de presentación.

`E1` y `E2` representan evaluaciones. `G1` las relaciona mediante `gate([E1, E2], using: T1)`. El comprobador admite el archivo y conserva **dos operaciones `evaluate` y una `gate`**. La tabla contiene nueve parejas, correspondientes a las tres posibilidades de cada entrada.

## **3.3.** Ejercicio de interpretación

Sin ejecutar una inferencia, busque qué declara esta tabla para `(APTO, INDETERMINADO)` y para `(NO_APTO, INDETERMINADO)`. Justifique la respuesta por la fila correspondiente.

<details>
<summary>Ver solución</summary>

Las salidas declaradas son `INDETERMINADO` y `NO_APTO`, respectivamente. La respuesta describe las filas de `T1`; no afirma que los estados del programa hayan producido esas entradas.

</details>

Fundamento: [declaraciones y operaciones](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/construcciones-y-componentes.html#declaraciones-y-operaciones).

---

[Índice del manual](../README.md) · [Anterior](../parte-i-escritura/02-declaraciones-y-estados.md) · [Siguiente](04-resolucion-u.md)

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
