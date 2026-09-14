# S26 R06 · PROCESO01 · Impedimento instrumental

**Estado: campaña no ejecutada; cero casos de comportamiento completados.**

El banco se publicó antes del intento en Lenguaje [3f306da4e4896a0738ef1bb3a9775a42219437b2](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/3f306da4e4896a0738ef1bb3a9775a42219437b2) y laboratorio `e04e51d2d9f17d9aebb4545e950c789d00fac67e`, con cotejo de ambos árboles completos.

La compilación debug terminó correctamente. La primera invocación del banco terminó con código 1 y diagnóstico `Operation not permitted (os error 1)`. Se escribieron únicamente la apertura P01 y el estado local inicial. La siguiente operación era `UnixListener::bind`; el canal no pudo abrirse y el proceso hijo no llegó a crearse. No se produjo `resultado.json`. El error no es un rechazo del SV, un informe del hijo ni una conformidad del observador.

[EVIDENCIA_IMPEDIMENTO.tar.gz](EVIDENCIA_IMPEDIMENTO.tar.gz), 3595 bytes; SHA-256 `0410cfcae0ce8913fa46633af077e2bef1ad5fa20fc1acaef633466ca0494d2d`. Conserva apertura, archivo inicial, diagnóstico y compilaciones preparatorias. Código y oráculos previos permanecen intactos.

La [variante PROCESO02](../proceso02/README.md) utiliza tuberías anónimas heredadas, conserva las distinciones T03/T04/T08 y exige precompromiso propio antes de ejecutar. No requiere habilitar sockets ni ampliar permisos. T07, integración y casos globales S26 permanecen pendientes.
