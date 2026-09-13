# BIS-C01 · Geometría, longitud y conservación posicional

**Versión 1 · S22 · 13 de septiembre de 2026 · Juan Antonio Lloret Egea y Watson**

## Contrato del ensayo

Se materializan BIS-C01-P y BIS-C01-N del banco previo, mediante trece variantes. El objeto de prueba es la compilación SVP a IR por `sv-native --profile en|es`; no se ejecuta `evaluate`, una IA ni una representación gráfica. La admisión descrita aquí acredita exclusivamente las comprobaciones de geometría y longitud observadas en esta ruta. No acredita constitución de dominio, admisión de soporte ni la totalidad de BIS-O01/O02.

La Frontera normativa v0, A.1–A.2, fija el alfabeto y la longitud `N=b²`, con `b≥3`. Los Pilares y AGENTS impiden rellenar, inferir o reordenar posiciones. La tabla de perfiles SVP recibe sus equivalencias ES/EN. El banco compromete los vectores literales, la salida IR completa esperada y las entradas, identificadas por SHA-256, antes de ejecutar el compilador. Los resultados esperados no se obtienen ejecutando el código examinado. Los textos diagnósticos se fijan para observar la ruta existente, sin conferirles un nuevo código canónico.

Las declaraciones sintéticas `K`, `S`, `C` y `State` tienen función instrumental. El codominio de un solo símbolo se utiliza porque no se ensaya clasificación. No son constituciones de inmunología, ciberseguridad o neumología. Los índices son posiciones originales; no se les atribuyen parámetros de esos dominios.

## Variantes y observadores

- Cinco positivos: nueve, dieciséis, veinticinco, treinta y seis y cuarenta y nueve coordenadas. Nueve es un control del mínimo; el resto del banco trabaja con dimensiones mayores.
- Un positivo adicional permuta expresamente las dos primeras coordenadas en la entrada: la IR debe conservar el nuevo orden declarado. Esto comprueba fidelidad de transporte; no habilita modificar una constitución ya admitida ni acredita detección de una permutación fraudulenta respecto de un contrato externo.
- Un positivo español reproduce el estado de dieciséis coordenadas y su IR canónica.
- Cuatro negativos sitúan la longitud una unidad por debajo y por encima de dieciséis y veinticinco.
- Dos negativos usan `b=2`, uno con cuatro y otro con dieciséis coordenadas. En el segundo coinciden dos defectos: se espera el rechazo previo de `CellSpec`, que no acredita haber alcanzado la comprobación de longitud.

El observador compara código de salida, stderr literal y JSON completo, incluidos identidad de fuente, versión, especificación, dimensión, orden y coordenadas. Para un rechazo exige stdout vacío. La sensibilidad del observador se contrasta alterando cuatro salidas capturadas; se diferencia de los trece ensayos del compilador.

## Reproducción

Desde la raíz de un checkout completo del Lenguaje:

```sh
cargo build --manifest-path rust/Cargo.toml --locked --offline -p sv_native --bin sv-native
node docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis-c01/verificar.mjs rust/target/debug/sv-native /tmp/sv-bis-c01-nueva-ejecucion
```

El directorio de salida debe ser nuevo. El verificador conserva stdout y stderr de cada caso y un informe con la huella del ejecutable y del banco. Node sólo observa procesos y compara resultados; la compilación SVP pertenece al núcleo Rust. Esta herramienta no sustituye la DSL ni añade un intérprete alternativo.

## Cortes, lecturas y límites

Lenguaje: `5ba3fbab751d9e4561dcecf2659e0ee4a54f7b4f`. Laboratorio de entrada: `d2e10fe6d75ba334956de66b7532b0ce3f890f6a`, rama `lab/playground-sv-permanente`. Se reutilizan las lecturas completas de Pilares, Perfiles/Contratos/Ensamblaje y Transición secuencial declaradas en S23: el cotejo Git frente a `8893707154e5fd6e623904a71345268339eb6550` confirma identidad de los dos últimos y sólo la adición del §13 de Pilares, leído en esta revisión. AGENTS, workflow V2, resultado BIS-01 y contrato/banco BIS-02 se consultan de nuevo; Frontera A.1–A.2, frontend, compilación, validación de CellSpec/CellState y CLI se inspeccionan para identificar el montaje.

Una incidencia de preparación fue detectada antes de comprometer el corpus: la primera transcripción del vector de b=6 contenía 37 símbolos. La aserción administrativa de longitud detuvo la preparación; se corrigió la transcripción a 36. No fue una ejecución del Lenguaje ni se presenta como rechazo suyo. Los fixtures comprometidos conservan su identidad después de esa corrección.

El banco no introduce `E003` como protección ejecutable, ni fija tamaño máximo, política de soporte, array Rust, par visual, composición, presupuesto o capacidad de IA. Esas obligaciones mantienen sus contratos y etapas propios.
