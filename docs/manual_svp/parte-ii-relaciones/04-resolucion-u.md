# **4.** Resolución de U y proyecciones

[Lectura en la biblioteca](https://documentos-sv.itvia.online/desarrolladores/parte-ii-relaciones/04-resolucion-u.html) · [Enciclopedia temática](https://documentos-sv.itvia.online/enciclopedia/)

[Índice del manual](../README.md) · [Anterior](03-compuertas.md) · [Siguiente](05-composicion.md)

## **4.1.** Identificar la indeterminación

La resolución se refiere a una ocurrencia identificada de U y a una especificación que declara contexto y mecanismo. El ejemplo señala la tercera posición de `S1`. Cuente las posiciones desde la primera y compruebe que allí aparece U.

```svp
-- resolve_projection.svp — resolve sobre una U constituida + proyección explícita

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

cellstate S1 {
  spec: C1;
  vector: [Zero, One, U, Zero, Zero, One, U, Zero, One];
}

res_spec RS1 {
  context: ContextoClinico;
  mechanism: RevisionExperto;
  mapping: MapeoResolucion;
}

let RR1 = resolve((S1, 3), with: RS1, context: ContextoClinico, mechanism: RevisionExperto);
let valor_resuelto = RR1.resolved_to;
```

Archivo: [resolve_projection.svp](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/tests/conformance/valid/resolve_projection.svp).

## **4.2.** Seguir las referencias

`RS1` declara los identificadores `ContextoClinico`, `RevisionExperto` y `MapeoResolucion`. En este archivo son referencias nominales; su escritura no aporta por sí sola documentos clínicos, una intervención profesional ni una resolución real.

`RR1` representa la resolución solicitada para `(S1, 3)`. `RR1.resolved_to` expresa la proyección de un campo del resultado previsto. Observe que solicitar esa proyección no autoriza a sustituir U por un valor elegido arbitrariamente.

## **4.3.** Ejercicio de lectura

Explique la diferencia entre el estado original `S1`, la operación `RR1` y `valor_resuelto`. Compruebe después el archivo y localice la operación de resolución y la proyección en su representación.

<details>
<summary>Ver solución</summary>

`S1` conserva el vector de partida. `RR1` identifica la resolución solicitada bajo `RS1`. `valor_resuelto` designa el acceso al campo `resolved_to` del resultado previsto. La admisión conserva estas relaciones; no acredita que el mecanismo haya resuelto materialmente la indeterminación ni que el vector original se haya reescrito.

</details>

Fundamento: [construcciones del lenguaje](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/construcciones-y-componentes.html) y las especificaciones formales recogidas en su referencia.

---

[Índice del manual](../README.md) · [Anterior](03-compuertas.md) · [Siguiente](05-composicion.md)

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
