# Contrato candidato para la paridad y las operaciones SV

**Versión 0.1 · S22 · BIS-02 en ejecución · 13 de septiembre de 2026**

## Estatuto

Este documento inicia el diseño contractual autorizado. No define nuevos tipos canónicos, firmas ejecutables ni códigos de diagnóstico. La definición del workflow está cerrada; la realización contractual se somete al banco previo y a la decisión de sede BIS-03.

## Condiciones recibidas

La admisión de un estado celular exige constitución identificada, dimensión conforme, alfabeto ternario, asignación y orden de posiciones preservados. La admisión de soporte exige una declaración competente de tamaños y recursos. Ambas condiciones se verifican separadamente.

La representación matemática y la visual deben referirse a la misma constitución, instancia y estado. Su convenio debe proceder de los fundamentos y de la declaración de representación admitida. Toda transformación con pérdida declara su alcance y no se ofrece como equivalencia total. Una imagen generada y una imagen consumida son hechos distintos que requieren su propia evidencia.

El hecho puede registrarse en prosa, representarse matemáticamente o visualizarse, según el contrato. La decisión formal SV conserva el respaldo algebraico pertinente; una lectura técnica no recibe ese estatuto por estar registrada. La formalización de sucesos pertinentes sigue su horizonte competente. La creación inicial no fabrica un dato de transición anterior; la reevaluación conserva el original y produce la relación declarada con el nuevo estado.

Las operaciones conservan relación semántica previa, participantes, roles, dirección, dominio y codominio. La compuerta, la serie y la supervisión no se reducen a una operación universal. La forma Rust —función, método o macro— conserva esos requisitos y las mismas vías de admisión.

## Decisiones concretas por resolver

| Decisión | Criterio y evidencia exigidos |
| --- | --- |
| frvis: descripción o artefacto | Comparar funcionamiento sin GUI, generación diferida y consumo efectivo. Elegir sin perder paridad ni exigir generación innecesaria. |
| Frame celular y Frame de arquitectura | Declarar una relación tipada que conserve alcance y cierre; no reutilizar el nombre como prueba de equivalencia. |
| Inicialización y reevaluación | Declarar qué constituye el primer estado, qué evidencia corresponde y cuándo existe transición. Preservar el pasado. |
| Tamaños de soporte | Recibir declaración por versión y probar admisión/rechazo. Ningún fixture sintético cierra el inventario de un dominio. |
| Contenedor Rust | Comparar arrays estáticos con almacenamiento encapsulado; incluir construcción, copias, tamaño de código y memoria cuando condicionen la elección. |
| Sede de operaciones | Distinguir obligación nuclear, operación derivada, extensión de dominio y función de presentación. |
| Consejo y decisión | Definir la frontera entre afirmación documental, consulta de estado y decisión formal, incluidos permisos de la tubería. |
| Presupuesto | Constituir límites de célula, composición, texto, imagen y concurrencia antes de afirmar viabilidad. |

## Banco y precedencia

El [banco previo](BANCO_PREVIO_BIS_02_v0_1.json) fija 24 escenarios documentales agrupados en doce pares. Antes de la ejecución, cada escenario necesita fixtures exactos, oráculo independiente, montaje y observador. El resultado esperado provisional se apoya en una obligación identificada y no en copiar el comportamiento de la implementación.

La precondición de cada escenario debe cumplirse para que el control llegue a la comprobación pretendida. Si un caso se rechaza antes, sólo acredita ese rechazo previo. Un control positivo admisible no significa éxito clínico, decisión favorable ni ejecución completa; significa satisfacer la condición concreta del ensayo.

No se cerrará BIS-02 hasta disponer de los contratos necesarios para la realización escogida y del banco comprometido correspondiente. Las operaciones que dependan de un contrato pendiente permanecen sin habilitar.
