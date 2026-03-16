# NOTA_DE_ARQUITECTURA_MINIMA_DEL_NUCLEO_ENGANCHABLE_DEL_LENGUAJE_SV

**Fecha:** 16/03/2026  
**Estado:** Nota de arquitectura mínima  
**Ámbito:** `SV-lenguaje-de-computacion`

## 1. Objeto

La presente nota fija un criterio de diseño para el núcleo del lenguaje SV: el núcleo podrá consolidarse y cerrarse sobre sí mismo, pero no hasta el punto de impedir acoplamientos futuros formalmente previstos.

## 2. Principio rector

El núcleo del lenguaje no debe ser:

- blando por indefinición;
- ni cerrado ciegamente por clausura absoluta.

Debe ser, más bien:

**núcleo canónico duro con puertos de extensión previstos.**

## 3. Interpretación procedimental

Este criterio no exige una arquitectura orientada a objetos. Puede implementarse en régimen procedimental o modular, mediante:

- procedimientos;
- módulos;
- bibliotecas;
- contratos de entrada y salida;
- tablas de llamada;
- y puertos de acoplamiento explícitos.

## 4. Núcleo y puertos

Se distingue entre:

### 4.1. Núcleo canónico
Zona donde viven:

- gramática canónica;
- IR vigente;
- reglas del compilador;
- runtime básico;
- semántica ternaria consolidada.

### 4.2. Puertos de extensión previstos
Zonas formalmente previstas para futuros acoplamientos, sin integración automática, por ejemplo:

- puerto de entrada semántica;
- puerto de entrada perceptiva;
- puerto de reevaluación;
- puerto de agentes especializados;
- puertos de bibliotecas o módulos subordinados.

## 5. Relación con los carriles beta

La existencia de líneas beta sobre semántica o visión no obliga por sí sola a abrir el núcleo, pero sí justifica que el núcleo sea diseñado sabiendo desde ahora que podrían existir módulos futuros que requieran acoplamiento.

## 6. Regla de prudencia

El diseño del núcleo deberá preservar dos exigencias simultáneas:

1. capacidad de avance autónomo del lenguaje vigente;
2. y capacidad de enganche futuro sin reconstrucción traumática del sistema.

## 7. Imagen directriz

El modelo adecuado no es el de un núcleo improvisadamente abierto, sino el de una **estación espacial construida desde el momento cero sabiendo que podrá recibir módulos de acoplamiento futuro**, sin dejar por ello de ser estable, habitable y operativa por sí misma.

## 8. Cierre

Se fija, por tanto, como criterio de diseño del Lengua SV, que el núcleo podrá cerrarse sobre sí mismo en su régimen canónico, siempre que preserve puertos de extensión formalmente previstos para procedimientos, módulos y bibliotecas subordinadas.
