# S26 R06 · PROCESO02 · Resultado incompleto y corrección de transporte

**Estado: no conforme como campaña. Un caso conforme, un caso discrepante, un caso interrumpido y once sin iniciar. No se ejecutó release.**

Precompromiso: Lenguaje `957bd49c3c23d1e5b7ae5dba8ffadac18e2d9253`; laboratorio `cc882b54f5ffb20b37ad0923733e34ec783a6097`. Ambos árboles completos fueron cotejados antes de la ejecución debug.

P01 devolvió `json_invalido` al recibir la captura, pese a salida 0 del hijo. El control positivo no fue admitido: no acredita una recepción suficiente. P02 cumplió su oráculo de SIGABRT (6), barrera anterior a captura, apertura conservada e informe ausente.

P03 alcanzó de nuevo `json_invalido`. La barrera posterior no llegó a procesarse; la espera final se agotó y el supervisor conservó el diagnóstico `terminacion_final_no_observada`. La campaña terminó con código 1, sin resultado global ni ejecución de P04–P14. La limpieza del proceso hijo no se cuenta como cumplimiento de P03.

La causa localizada es una incompatibilidad del nuevo protocolo: permitía cuerpos de hasta 16384 bytes, mientras que `json::decode` del componente reutilizado rechaza entradas mayores de 8192. Transportar la geometría de 6237 bytes como cadena JSON escapada produjo una pieza superior a esa cuota. No se atribuye el defecto a la admisión ni se amplía el decodificador para ocultarlo.

[EVIDENCIA_INCOMPLETA.tar.gz](EVIDENCIA_INCOMPLETA.tar.gz), 5156 bytes; SHA-256 `34ee0e9980c0df3aba3732ba90d4a143fb9f322a42eb9b6fc85522ae7df3d748`. Conserva observaciones completas de P01/P02, apertura y frontera parcial de P03, destinos locales, diagnóstico y compilación. La implementación anterior guardaba el canal literal tras esperar al hijo: P03 carece de esa copia completa y no se reconstruye retrospectivamente.

[PROCESO03](../proceso03/README.md) divide la captura en fragmentos ordenados, alinea la cuota de cuerpo con el decodificador y guarda el canal en la frontera antes de esperar la terminación final. Código y oráculos de PROCESO02 se conservan. La corrección requiere precompromiso propio y nueva campaña; no convierte estos resultados en conformes. T07, integración y casos globales S26 permanecen pendientes.
