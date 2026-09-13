# Fuentes y alcance de C10

**13 de septiembre de 2026 · S22 · RETP-2026-213**

Corte Lenguaje `26b0906572e7b9de74ee43bc9f648cc5c2945b25`; laboratorio `13dfebac4feb2b6f9a0ffb5ddd46122262a9d8bc`. Ambas referencias remotas se comprobaron antes de editar. Los tres rectores leídos completos en la continuidad conservan sus huellas. [FUENTES.json](FUENTES.json) precisa las piezas y los alcances.

Se revisaron Documento V §§6.1–6.5 en el corte `b8fd32978292d25adf9b87cf71e409005dce642c`, el contrato S2 y su comprobador de cobertura completos, Query y validate_query, la deuda viva y el relevo S13. La descripción de QueryResult se contrasta con la IR documental; su etiqueta en Rust no prueba una consulta productiva. El antecedente clínico S2 se usa exclusivamente por su obligación de cobertura, sin importar parámetros o semántica médica.

La revisión de código es estática. Los veinte escenarios se especifican con ocho evidencias sintéticas y referencia independiente; no hay compilación nueva, pruebas Rust C10, consulta a modelos ni ejecución profesional. El cotejo auxiliar Python comprueba estructura del banco, sumas del testigo, identidades y preservación del registro; no evalúa automáticamente implicación semántica general ni verdad externa.

La preparación no cierra C10-P/N originales, CQ1–CQ6, BIS-02 o deuda viva. No modifica Rust, semántica, IR, dominio ni catálogo. Continúa C11 y permanece diferida la GUI.
