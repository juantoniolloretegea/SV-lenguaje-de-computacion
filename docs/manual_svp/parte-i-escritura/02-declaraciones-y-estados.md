# **2.** Declaraciones y estados celulares

[Lectura en la biblioteca](https://documentos-sv.itvia.online/desarrolladores/parte-i-escritura/02-declaraciones-y-estados.html) · [Enciclopedia temática](https://documentos-sv.itvia.online/enciclopedia/)

[Índice del manual](../README.md) · [Anterior](01-primer-programa.md) · [Siguiente](../parte-ii-relaciones/03-compuertas.md)

## **2.1.** Del concepto a la escritura

Una especificación describe la constitución de la célula; un estado aporta los valores de una instancia conforme a esa especificación. Lea primero el programa completo y después siga la explicación de sus cinco declaraciones.

```svp
-- cell_basic.svp — Célula simple b=3, n=9
-- Test de conformidad: declaración mínima + evaluación

codomain K3 = { APTO, NO_APTO, INDETERMINADO };

output_semantics Klin {
  APTO -> "Clasificación fuerte favorable";
  NO_APTO -> "Clasificación fuerte desfavorable";
  INDETERMINADO -> "Sin clasificación fuerte";
}

cellspec C1 {
  b: 3;
  codomain: K3;
  semantics: Klin;
  role: Base;
}

cellstate S1 {
  spec: C1;
  vector: [Zero, One, U, Zero, Zero, One, U, Zero, One];
}

let E1 = evaluate(S1);
```

Archivo de referencia: [cell_basic.svp](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/tests/conformance/valid/cell_basic.svp).

## **2.2.** Leer cada declaración

1. `codomain K3` identifica los valores de salida que utiliza el ejemplo. Los nombres `APTO`, `NO_APTO` e `INDETERMINADO` forman parte del codominio declarado.
2. `output_semantics Klin` asocia cada valor con un texto. Esos textos no sustituyen las reglas que determinan un resultado.
3. `cellspec C1` vincula base, codominio, semántica y rol. En esta construcción, `b: 3` corresponde a nueve posiciones.
4. `cellstate S1` refiere a `C1` y aporta nueve valores ordenados. `Zero`, `One` y `U` pertenecen al perfil SVP-EN. El orden identifica las posiciones; contar valores no conserva esa información.
5. `let E1 = evaluate(S1)` declara una operación cuyo resultado previsto es `EvalResult`. La comprobación genera su representación; no entrega por ello una clasificación calculada.

Las llaves delimitan declaraciones, los corchetes delimitan listas y el punto y coma termina los elementos que lo requieren. `--` introduce un comentario hasta el final de la línea. Los identificadores, como `C1`, permiten relacionar declaraciones; cambiar uno exige revisar las referencias que lo utilizan.

## **2.3.** Qué debe observar

Al comprobar el ejemplo, localice cuatro objetos y una operación. La especificación contiene `b: 3` y `n: 9`; el estado conserva los nueve valores y la operación tiene como entrada `S1`.

Fragmento de la representación obtenida:

```json
{"inputs":{"state":"S1"},"name":"E1","result_type":"EvalResult","type":"evaluate"}
```

## **2.4.** Practicar con una modificación acotada

Cambie **únicamente** el primer `Zero` del vector por `One`. Antes de comprobar, formule qué espera observar: deben mantenerse la longitud del vector y sus referencias; debe cambiar el valor de esa posición en la representación. La admisión no permite deducir por sí sola una clasificación final.

Después pruebe el mismo cambio de nombre de `S1` a `EstadoInicial` tanto en su declaración como en `evaluate`. La [lección de ejercicios](../parte-iii-ejercicios/07-ejercicios-y-diagnosticos.md) contiene archivos y soluciones para estas variaciones.

Fundamento: [estados celulares y terna](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/construcciones-y-componentes.html#estados-celulares-y-terna).

---

[Índice del manual](../README.md) · [Anterior](01-primer-programa.md) · [Siguiente](../parte-ii-relaciones/03-compuertas.md)

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
