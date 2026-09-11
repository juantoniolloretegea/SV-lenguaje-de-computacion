# Controles discriminantes del bosque

RETP-133, fijados después de observar 72/72 y antes de ejecutarlos. Son cuatro controles estructurales de la misma fuente Rust; no preguntas nuevas para la reserva ni cambios del perfil. Se justifican porque los 72 casos no ejercen dos derivaciones sintácticas distintas para una misma raíz.

1. `"igg" | "igg"`: una raíz con dos familias y dos derivaciones, no sólo la primera alternativa.
2. `("igg" | "igg") ("iga" | "iga")`: una raíz con cuatro derivaciones por producto; compartir subnodos no debe perder multiplicidad.
3. `{["igg"]}`: cuerpo anulable rechazado por el comprobador de mínimos antes de usar la repetición. Se comprueba también la defensa de ejecución si se omite deliberadamente esa puerta en el control: error, sin bucle.
4. La alternativa del primer control, con 12 unidades: el primer testigo ya puede estar presente, pero la siguiente consulta agotará la cuenta; la operación completa debe fallar técnicamente sin tratar la raíz parcial como cierre. Las 12 unidades se fijan a partir de los cargos del código, antes de observar el control.

Se ejecutan una vez por nativo/WASI y debug/release, máximo 30 s por proceso, sin ajustar los esperados. Se concatenan la fuente congelada (únicamente se renombra su main para no ejecutar otra vez el corpus) y `controles.rs`; las funciones del motor se conservan byte a byte. El recuento de derivaciones se realiza en Rust sobre el bosque emitido, con sumas/productos comprobados y guardia de ciclo. No se utiliza Python ni un recuento semántico en Node. Estos controles no son prueba de interpretación G10, política ni aislamiento A/V.
