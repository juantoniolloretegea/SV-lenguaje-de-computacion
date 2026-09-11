# Rectificación del esperado del observador, C08

El plan de este contraste escribió JSON_INVALIDO para V04. Fue un error del observador: CONTROLES_PUBLICOS.json del corte 8880a099 ya fija TRUNCADO, y el anexo nativo-release-V04-v.stdout recuperado de la cápsula RETP-137 también dice TRUNCADO. El anexo recién emitido coincide íntegramente en bytes con ese original. No se cambia un esperado SV para acomodarlo a una salida nueva.

Se conservan PLAN_CONTRASTE.json, RESULTADO_SEGUNDO_INTENTO.json, EVIDENCIA_SEGUNDO_INTENTO.json y comprobar-evidencia-correccion-1.mjs. El comprobador reproducible consulta ahora la causa del control previamente fijado. La rectificación vigente de C08 es V_RECHAZADA / TRUNCADO e igualdad literal con su anexo histórico.

Para completar esta ejecución se coteja C08 sobre sus bytes ya obtenidos, sin volver a llamar a A o V. C09, que aún no se había alcanzado, se ejecuta por primera vez con las 16 pruebas existentes de Frame. Ninguna de las dos incidencias corrige Rust ni abre una nueva ronda de modelos. Se conserva íntegro el fallo de ambos intentos del observador.
