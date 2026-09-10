# Reproducir la recepción 001 de IE-004

Paquete público, sin acceso privado ni API de IA. Se conserva la entrega real de Grok; estas órdenes reproducen el receptor, no vuelven a ejecutar el modelo.

Use Rust 1.98.0 con destino wasm32-wasip1 y Node con WASI preview1. Desde docs/calidad/tuberias-ia/ie004/fuentes:

```bash
mkdir -p ../bin
rustc +1.98.0 --edition=2021 -O -C panic=abort --crate-name receptor_ie004 receptor.rs -o ../bin/receptor
rustc +1.98.0 --edition=2021 -O -C panic=abort --target wasm32-wasip1 -C link-arg=--max-memory=67108864 --crate-name receptor_ie004 receptor.rs -o ../bin/receptor.wasm
node ejecutar.mjs ../bin ../recepcion-001-repro ../recepcion-001/solicitudes-originales.json ../recepcion-001/entrega-original.json ../recepcion-001/ORACULO-ABIERTO.json ../../COMPROMISO-001.json
```

El observador comprueba el compromiso anterior al depósito y produce RECEPCION.json, salidas nativo/WASI, medidas y el testigo de brecha semántica. Resultado custodiado: 23/24 identificaciones y 22/24 cuerpos correctos; fallos L01 y L11. Los resultados funcionales deben coincidir. Los tiempos, metadatos del entorno y, según la plataforma de compilación, hashes del binario nativo pueden variar; no se exige identidad de binarios entre toolchains/plataformas distintas.

El contraste bruto sólo comprueba desigualdad de bytes. Para reproducir LECTURA-FUNCIONAL.json, cada relación además exige que identificacion_correcta y respuesta_correcta sean verdaderas para todos sus miembros. Así L04/L11 no es un contraste funcional correcto: contiene un error de apoyo.

La recepción registrada reutilizó los dos binarios de la ejecución 34501044665/1; no se recompilaron localmente. PROCEDENCIA.json conserva sus identidades y las seis fuentes. El proceso no mide latencia de Grok. Si no hay GNU time se declara CPU/RSS no observables.

Un exit 0 significa que se reprodujo la caracterización; la respuesta puede seguir siendo incorrecta. La brecha semántica conocida debe seguir siendo visible. No se atribuye una auditoría ejecutada a quien sólo lea este paquete.
