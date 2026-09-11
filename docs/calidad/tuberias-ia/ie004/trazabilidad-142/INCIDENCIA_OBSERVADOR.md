# Incidencia conservada del observador de evidencia

El primer intento recuperó 409 archivos y se detuvo en C02 antes de compilar o ejecutar Rust. El observador suponía que stderr completo era JSON. Las capturas WASI contienen dos líneas de aviso de Node delante de la traza de A.

Se conservan el script inicial, RESULTADO_INICIAL.json y EVIDENCIA_INICIAL.json. La corrección afecta exclusivamente al lector documental: exige el prefijo exacto de dos líneas observado en esta cápsula, registra ese prefijo, conserva la huella del stderr completo y comprueba el JSON restante. No busca arbitrariamente un fragmento que permita aprobar; otros prefijos o texto posterior inválido se rechazan.

No cambian cápsula, fuentes Rust, entrada, esperado ni versiones. El segundo intento repite custodia C01/C02 y ejecuta por primera vez C03–C09. No es una segunda campaña del modelo ni corrección de A/V. La mezcla de canales sigue presente en el banco: esta lectura no la corrige en producción.
