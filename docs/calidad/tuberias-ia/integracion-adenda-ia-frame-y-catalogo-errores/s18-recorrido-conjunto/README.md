# S18 — Recorrido documental conjunto

Apertura previa a compilación. [Campaña y presupuesto](CAMPANA_S18.md) · [Realización Rust](codigo/recorrido.rs) · [Conductor](codigo/contraste.rs) · [Esperados](codigo/ESPERADO.tsv) · [Contrato S17 intacto](contrato-s17/CONTRATO_RECORRIDO_CONJUNTO.md).

24 casos; 144 observaciones normales previstas, cuatro sensibilidades y tres clientes externos. Máximo 29 invocaciones, sin reintentos. Resultado pendiente en esta apertura. [Procedencia](PROCEDENCIA.json), [rectores](RECTORES.json), [fuentes anteriores](FUENTES_S2.json), [fijación previa](FIJACION_PREVIA.json).

Reproducción en Linux x86_64, Rust 1.98.0 fijado:

```sh
python3 reproducir.py --rustc /ruta/absoluta/rustc --salida /ruta/nueva/s18
```

La apertura conservará sus bytes. El acta de resultado o interrupción se incorporará después. No constituye autoridad profesional, integración con proveedor real ni promoción nuclear.
