# Realización experimental C02–C05 · BIS-04 · 0.1

Juan Antonio Lloret Egea y Watson · S22 · 13 de septiembre de 2026.

Corte de entrada: `2067310b30e5aee76120444f0166b01390a04970`. Realiza las sedes e interfaz de RETP-219 sobre los bancos inmutables de RETP-218 y RETP-219. Estado de este precompromiso: **compilación nativa conseguida; campaña aún sin ejecutar**.

Se incluyen el crate `sv_bis_i0205`, su conductor/receptor, observador separado, Cargo.lock sin dependencias de registro y copia íntegra verificable de sv_core. La copia del núcleo conserva sus fuentes exactas y el manifiesto incorporado; no cambia el workspace productivo.

Antes de ejecutar se fijan ocho sondas externas: siete rechazos de privacidad o API y una lectura que debe compilar. Se fijan también cinco comprobaciones internas de longitud/soporte en un test Rust. Se ejecutarán los 19 instrumentales, 26 integrados, tres invocaciones v1→v2→v1, cuatro sensibilidades del observador y una invocación adicional para lectura de objeto admitido. No son variantes nuevas de los bancos C02–C12 originales.

Los primeros intentos de construcción quedan en evidencias: build-01 no encontró lockfile; se generó offline. build-02 detectó un delimitador sin cerrar en main.rs; se conserva esa fuente. build-03 compiló correctamente. Ningún esperado fue cambiado.

Los tipos admitidos tienen campos privados y vector en arrays; el llamador sólo obtiene slices de lectura. La custodia del contexto la ejerce el conductor; no es autenticación criptográfica ni aislamiento ante un host comprometido. El captor copia el buffer realmente recibido. La falta de captura mantiene efecto desconocido; ningún fallo se convierte en U.

El JSON y SHA adaptan los auxiliares IE004 de recepción-av, eliminando reglas nodos/hijos y el marco SVAC. JSON conserva lexemas enteros, null y ausencia, y aplica límites de RETP-219. SVP se procesa exclusivamente mediante compile_svp_profile. Los comentarios y documentación del código nuevo están en ES/EN.

Fuera de alcance: dominio operativo, consumo visual de IA, perfiles ES/EN integrados, agotamiento real de memoria, viabilidad temporal, cierre global BIS-02/BIS-03, catálogo y GUI. Los rectores de pilares, perfiles/ensamblaje y transición IMM conservan las huellas previamente leídas y revalidadas en FUENTES de RETP-219. Las 202 filas previas C02–C12 permanecen sin ejecutar.
