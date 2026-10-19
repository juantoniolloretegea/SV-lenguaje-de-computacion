# Revisión adversarial de la admisión institucional

Fecha: 19/10/2026. Revisión técnica delimitada, no auditoría externa ni aceptación de R2 completa.

## Objeciones comprobadas y correcciones

1. **El caso legítimo no atravesaba la mediación.** Reproducido como `MissingHistoricalRule`. Se incorporó la regla de conservación ligada al mandato íntegro; no se eliminó la recomprobación de mediación. El caso legítimo y los adversos pasan después de la corrección.
2. **Una entrada sin firma podía agotar el registro reservado a actuaciones autenticadas.** Se separó un resumen acotado de rechazos sin contenido. Los casos IA24/IA27 envían 128 entradas inválidas y comprueban que sigue siendo posible la actuación legítima. El registro autenticado continúa limitado y su agotamiento no impide revocar.
3. **Podía confundirse un rechazo nuclear de ejecución con un fallo del adaptador.** Las causas se distinguen. La incertidumbre del adaptador no se convierte en refutación o acreditación semántica y no habilita repetición.
4. **La comprobación podía caducar antes del efecto.** Se conserva una segunda muestra temporal inmediatamente antes de la mediación y el ejercicio. IA13 prueba específicamente ese intervalo; IA14 comprueba retroceso del reloj y vencimiento monotónico.
5. **Una firma correcta podía interpretarse como legitimidad autosuficiente.** El contrato y la interfaz declaran expresamente el reconocimiento institucional pendiente y la protección exigida de la configuración. La separación de claves no se presenta como prueba de independencia personal.
6. **El componente opcional podía parecer una activación instalada.** La función está deshabilitada por defecto; no se incorpora a rutas, servicios, datos o credenciales del Gestor. Las pruebas no constituyen una operación profesional.

## Ataques y controles

Se comprobaron firmas falsas, cambio de cada campo del mandato, sustitución de identidad manteniendo su nombre, cambio de contexto, objeto, dominio y operación, reutilización de firma entre mandatos, modificación de evidencia o identificador después de firmar, caducidad, revocación, duplicación concurrente, repetición tras error y pánico del adaptador. La ausencia de evidencia produce estado no verificable; una evidencia diferente se refuta; ninguna provoca efecto.

Nueve alteraciones controladas de las barreras fueron detectadas mediante ejecución: duplicación de admisión, sustitución de alcance, revocación, repetición, igualdad falsa, vigencia constante, omisión de la segunda muestra temporal y omisión de ambas verificaciones de firma. Cada fuente se restituyó y se cotejó por SHA-256. Los resultados positivos se repitieron sobre las fuentes restituidas.

La composición produce las seis clases nucleares y la comprobación específica mediante las funciones existentes. Un cliente externo ejecuta el recorrido permitido sin `for_test`; las pruebas de compilación rechazan el acceso a la evidencia interna y la fabricación directa del estado admitido. El acceso explícito de configuración sigue siendo privilegiado por contrato y requiere recepción material en el soporte: no se declara cerrada esa condición.

## Plataformas y límites de la comprobación

Las pruebas completas del conjunto se ejecutan en Linux. En Windows se comprueban las unidades nucleares, el cliente externo, las fronteras de compilación y la variante optimizada. El intento inicial de todo el conjunto en Windows encontró primero dos bibliotecas de configuraciones distintas en el mismo destino; con un destino aislado apareció la dependencia preexistente de programas Unix en siete pruebas documentales. Esos intentos se conservan y no se cuentan como favorables.

WASI excluye PC06 e IA22, que requieren concurrencia entre hilos, e IA21, que requiere recuperación mediante desenrollado tras un pánico. El primer intento de IA21 produjo una interrupción de WebAssembly; no se oculta ni se presenta como un resultado semántico. La ejecución WASI restante utiliza las mismas fuentes. No se afirma recepción de una instalación en navegador.

Clippy conserva los quince diagnósticos únicos de la base, sin nuevos avisos en los archivos añadidos. Los dos avisos de la candidata, sobre tamaño de una variante y recorrido por índices, fueron corregidos sin supresión general de advertencias. El formato de los archivos nuevos está comprobado. Los antecedentes ajenos no se reformatean.

Las 36 dependencias públicas fijadas en Cargo.lock se consultaron en [OSV](https://google.github.io/osv.dev/post-v1-querybatch/), sin avisos devueltos y sin paginación pendiente; se conserva la respuesta fechada. Se contrastaron además los avisos [RUSTSEC-2022-0093](https://rustsec.org/advisories/RUSTSEC-2022-0093), [RUSTSEC-2024-0344](https://rustsec.org/advisories/RUSTSEC-2024-0344.html) y [RUSTSEC-2021-0100](https://rustsec.org/advisories/RUSTSEC-2021-0100.html): las versiones elegidas están fuera de sus intervalos afectados. Esto no demuestra ausencia de defectos desconocidos ni sustituye una auditoría de sus implementaciones. Sólo se consultaron nombres y versiones públicos; no se remitieron fuentes privadas, documentos o credenciales.

## Dictamen

Resultado favorable para la admisión firmada y su composición intraproceso bajo la configuración institucional reconocida, con los límites descritos en el contrato y en los resultados reproducibles. No es un cierre de la admisión instalada, de (P1 + P3)-Bis, de la adenda completa, de R2 ni de R3. La siguiente recepción debe demostrar la configuración institucional efectiva y su custodia, antes de atribuir esa capacidad a una instalación real; las dependencias de consulta, transducción, cobertura y continuidad permanecen abiertas.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
