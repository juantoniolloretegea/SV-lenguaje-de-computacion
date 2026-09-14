# S26-F01/F02: banco previo de referencias y ligaduras

**14 de septiembre de 2026. Unidad: Watson / W-S26. Estado: preparado; sin compilar ni ejecutar.**

Continúa el relevo de [INTEGRADO01](integrado01/RESULTADOS.md). Se fija un incremento limitado a la recepción y al consumo de ligaduras LIG/0.1 mediante su API pública. No implementa un motor de consulta ni atribuye a LIG una consulta histórica que su contrato excluye.

## Fuentes y límite de la realización

Corte leído del Lenguaje: `fe96bea19c14ddbb80c01033b52a7e4761754732`. Laboratorio: `97130b9ff79f199aefcae2ae1ffccef5d98cdd3d`, rama existente `lab/playground-sv-permanente`.

Se han leído completos AGENTS.md, Pilares de diseño, el acta de perfiles/contratos/ensamblaje y el acta de transición desde OP-IMM-001, así como el contrato material LIG/0.1 y el contrato de continuidad de fila 7. Se han examinado la realización pública de ligaduras, Frame y los fixtures y oráculos existentes de `tests/row7_bindings`. Las identidades de entrada constan en [FUENTES_F01_F02_LIGADURAS.json](FUENTES_F01_F02_LIGADURAS.json); el corte completo conserva las dependencias transitivas.

LIG conserva contrato, expectativa, programa, instancias y usos ordenados. La API devuelve esas ligaduras; no resuelve una consulta por Frame histórico ni ejecuta la operación declarada. Los objetos Frame tienen constructores internos; este banco no elude su visibilidad ni introduce un identificador universal de ocurrencia.

Los diez casos ejercitan esa frontera disponible. La diferencia entre las dos instancias I1/I2 del fixture se observa mediante los usos constituidos, aun cuando comparten P y el numeral 1. La igualdad de bytes probada en el caso 01 pertenece a la procedencia versionada; no se presenta como prueba de todas las identidades de ocurrencia SV.

## Banco y oráculos comprometidos

La [fuente Rust](s26_f01_f02_ligaduras.rs) reutiliza únicamente los constructores de fixtures EN y ES existentes. No ejecuta el banco anterior de 53 familias. Cada caso nuevo recorre las dos entradas y conserva el control original antes y después de la mutación. Los resultados esperados son literales y no se deducen de la salida del validador.

| Caso | Estímulo | Oráculo |
| --- | --- | --- |
| 00 | Dos instancias del mismo parámetro y numeral | U1 → I1 → CC1, posición 3; U2 → I2 → CCCompartido, posición 3. Dos instancias conservadas. |
| 01 | Otra versión de FuenteP con los mismos bytes y SHA-256 del contenido | Rechazo ContractIdentity bajo la expectativa A; admisión bajo la expectativa independiente B. A conserva versión 1 y B versión 2 simultáneamente. Mutar copias externas no altera lo recibido. |
| 02 | Registro de instancias en orden I2, I1 | El consumidor sigue recibiendo I1, I2 por el orden de usos. El testigo de elección del primer P resulta distinto del esperado para U1. |
| 03 | Usos en orden U2, U1 | El consumidor recibe I2/CCCompartido/3 antes de I1/CC1/3. |
| 04 | Dos instancias en el mismo nodo con posiciones distintas | U1/I1/CC1/3 y U2/I2/CC1/4 se conservan sin selección por primera coincidencia. |
| 05 | Intercambio I1/I2 bajo expectativa fija | ContractIdentity bajo A; el control con expectativa B admite el intercambio explícito y conserva el orden resultante literal. |
| 06 | Dos destinos iguales sin alias | DestinationCollision, sujeto U2. |
| 07 | Identidad I1 repetida | DuplicateIdentity, sujeto I1. |
| 08 | Uso de I0 inexistente | InstanceMissing, sujeto I0. |
| 09 | Destino obligatorio ausente | DestinationMissing, sujeto U1. |

Los casos 01 y 05 conservan la expectativa A al atacar. Sus controles B fijan otra expectativa expresamente. En 06–09 se calcula la expectativa del candidato defectuoso antes de validar para alcanzar la guarda interior: esto no convierte la salida observada en oráculo. Los errores se comprueban por clase y sujeto, mediante ausencia del objeto validado; no se transforman en Tri.U.

Los controles compartidos de aceptación, ausencia, duplicación y colisión se repiten sólo como discriminadores del nuevo consumo; no se contabilizan como una nueva capacidad ni se sustituyen los resultados anteriores.

## Protocolo de ejecución pendiente

1. Recuperar un entorno con Rust/Cargo 1.98.0 y registrar las versiones efectivas, plataforma y recursos. No reutilizar binarios históricos como si fueran una compilación de esta fuente.
2. Obtener el corte publicado de este precompromiso en la rama existente. Cotejar con el auxiliar Rust disponible el árbol fuente fijado y los archivos del banco contra sus objetos Git, incluidos los recursos incorporados al compilar. Conservar ambos cotejos en la evidencia.
3. Ejecutar desde la raíz del Lenguaje. La copia del laboratorio es un espejo documental; sus rutas relativas no se presentan como una instalación autónoma del compilador.
4. Compilar y ejecutar los dos perfiles con los comandos de abajo. Registrar comandos, fechas, salidas, diagnósticos, retornos y huellas de los ejecutables. El observador de pruebas es Rust; el intérprete de comandos sólo invoca procesos y conserva sus canales.
5. Cotejar de nuevo las fuentes. Un fallo de compilación, una ejecución parcial o un oráculo incumplido se conservan como tales. Cualquier corrección se publica antes de una nueva ejecución; no se cambia el esperado para acomodarlo al resultado.

```sh
cargo +1.98.0 build --manifest-path rust/Cargo.toml --package sv_core --lib --locked --offline --target-dir rust/target/s26-f01-f02
rustc +1.98.0 --edition=2021 --test docs/calidad/riesgos-materiales-s26/r06/s26_f01_f02_ligaduras.rs --extern sv_core=rust/target/s26-f01-f02/debug/libsv_core.rlib -L dependency=rust/target/s26-f01-f02/debug/deps -C opt-level=0 -C debuginfo=2 -o rust/target/s26-f01-f02/s26-f01-f02-debug
rust/target/s26-f01-f02/s26-f01-f02-debug --test-threads=1 --nocapture

cargo +1.98.0 build --manifest-path rust/Cargo.toml --package sv_core --lib --release --locked --offline --target-dir rust/target/s26-f01-f02
rustc +1.98.0 --edition=2021 --test docs/calidad/riesgos-materiales-s26/r06/s26_f01_f02_ligaduras.rs --extern sv_core=rust/target/s26-f01-f02/release/libsv_core.rlib -L dependency=rust/target/s26-f01-f02/release/deps -C opt-level=3 -C debug-assertions=no -C overflow-checks=no -o rust/target/s26-f01-f02/s26-f01-f02-release
rust/target/s26-f01-f02/s26-f01-f02-release --test-threads=1 --nocapture
```

Estos comandos son previstos, no ejecutados. El directorio target es salida efímera de compilación, no una nueva carpeta versionada. El banco es finito: diez casos, dos entradas por caso y dos perfiles; ejecución secuencial, sin red, sin generación aleatoria, sin datos reales y sin efectos externos. No se afirma una cuota de RAM o tiempo acreditada. Un bloqueo técnico interrumpe la campaña con resultado incompleto; no la convierte en éxito.

No se han reejecutado los flujos CI que incluyen observadores Python o Node. Tampoco se modifican sus contratos para hacerlos pasar por una campaña íntegra en Rust.

## Estado y continuación

El entorno comunicó un fallo de inicialización de exec-server; la sesión no dispone de herramientas de ejecución local. La revisión del código y la publicación del banco son preparatorias. **Cero casos nuevos ejecutados; cero nuevos resultados conformes.**

F01 sigue requiriendo consulta y diferenciación de ocurrencias exactas cuando exista su interfaz material constituida. F02 sigue requiriendo resolución del componente por instancia, posición y constitución de célula/Frame en la operación correspondiente. Estas obligaciones no se cierran por comprobar sólo LIG ni mediante un resolutor inventado en el arnés.

Sucesos S26 recibe esta preparación y el impedimento en la revisión 23, con espejo de Calidad y laboratorio. INTEGRADO01 y los demás bancos conservan sus resultados. No se modifica RETP canónica, el núcleo, gramática, IR, contratos, ramas o workflows. S26/Bis permanecen abiertos y S24 conserva su secuencia.
