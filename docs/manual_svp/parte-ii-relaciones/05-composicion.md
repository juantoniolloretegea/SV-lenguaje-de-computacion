# **5.** Composición de arquitecturas

[Lectura en la biblioteca](https://documentos-sv.itvia.online/desarrolladores/parte-ii-relaciones/05-composicion.html) · [Enciclopedia temática](https://documentos-sv.itvia.online/enciclopedia/)

[Índice del manual](../README.md) · [Anterior](04-resolucion-u.md) · [Siguiente](06-sucesos-y-consultas.md)

## **5.1.** Declarar las relaciones

Componer exige identificar los elementos y sus relaciones. El siguiente programa describe dos especificaciones acopladas, un conector, una relación, un patrón y un grafo. No necesita estados celulares para expresar esa arquitectura.

```svp
-- compose_basic.svp — Composición con relación y patrón declarados

codomain K3 = { APTO, NO_APTO, INDETERMINADO };
output_semantics Klin { APTO -> "ok"; NO_APTO -> "no"; INDETERMINADO -> "u"; }

cellspec C1 { b: 3; codomain: K3; semantics: Klin; role: Base; }
cellspec C2 { b: 3; codomain: K3; semantics: Klin; role: Base; }

coupledspec CC1 { cell: C1; bridges: [3]; }
coupledspec CC2 { cell: C2; bridges: [3]; }

connector Phi1 {
  source_codomain: K3;
  target_position: 3;
  mapping: { APTO -> Zero; NO_APTO -> One; INDETERMINADO -> U; }
}

semantic_relation R_serie {
  kind: DeclaredRelation;
  constraints: [ProfilaxisRiesgo];
}

pattern P_serie {
  kind: DeclaredPattern;
  arity: 2;
}

graph G1 {
  nodes: [CC1, CC2];
  edges: [edge(source: CC1, target: CC2, position: 3, connector: Phi1)];
  relation: R_serie;
  regime: Simple;
}

let A1 = compose(G1, relations: [R_serie], patterns: [P_serie]);
```

Archivo: [compose_basic.svp](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/tests/conformance/valid/compose_basic.svp).

## **5.2.** Leer desde los elementos hasta la arquitectura

1. `C1` y `C2` definen las células; `CC1` y `CC2` declaran sus posiciones de enlace.
2. `Phi1` identifica el codominio de origen, la posición de destino y el mapa explícito entre valores.
3. `R_serie` y `P_serie` declaran la relación y el patrón utilizados.
4. `G1` relaciona `CC1` con `CC2` mediante una arista que utiliza `Phi1` y la posición 3.
5. `A1` expresa la composición de ese grafo con la relación y el patrón indicados.

El resultado esperado de la comprobación contiene una operación `compose`. Su presencia expresa la composición solicitada; no demuestra una ejecución distribuida, una selección de GPU ni el despliegue de una IA.

## **5.3.** Ejercicio de referencias

Identifique el origen, el destino y el conector de la arista. Explique por qué la aparición de dos células no basta para conocer cómo se relacionan.

<details>
<summary>Ver solución</summary>

Origen: `CC1`; destino: `CC2`; conector: `Phi1`; posición: 3. La arista, el mapa del conector, la relación y el patrón aportan información que no puede deducirse de contar las células.

</details>

Fundamento: [arquitectura y entornos](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/arquitectura-ejecucion.html). El Árbitro–Director y los recursos se estudian en el libro; su configuración no se convierte en una instrucción adicional de SVP.

---

[Índice del manual](../README.md) · [Anterior](04-resolucion-u.md) · [Siguiente](06-sucesos-y-consultas.md)

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
