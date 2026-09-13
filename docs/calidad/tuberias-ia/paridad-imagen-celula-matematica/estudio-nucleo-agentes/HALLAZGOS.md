# Hallazgos de contraste (p1+p3)-Bis

13 de septiembre de 2026. Revisión documental y estática; no revisión clínica ni certificación de entrenamiento. Los identificadores H son locales a este estudio, no códigos de error del catálogo. Los originales permanecen conservados.

## H01. Radios canónicos y adjetivos incompatibles

Referencia rectora: [Fundamentos algebraico-semánticos del Sistema Vectorial SV.md](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/fa3eb727799c322090e4e9126f81238cd198b9cc/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/Fundamentos%20algebraico-sem%C3%A1nticos%20del%20Sistema%20Vectorial%20SV.md), §§2 y 4: ρ(0)=1, ρ(1)=2, ρ(U)=3. En [agentes especializados. (De n = 9 ➔ n = 16.md](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/fa3eb727799c322090e4e9126f81238cd198b9cc/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/agentes%20especializados.%20%28De%20n%20%3D%209%20%E2%9E%94%20n%20%3D%2016.md), §7, se asignan esos mismos números pero se llama «máximo» al radio 2 e «intermedio» al radio 3. La descripción verbal contradice los radios. Los capítulos n=25 y n=36, §7, sí describen 2 como intermedio y 3 como máximo.

Consecuencia: mantener la asignación fundacional y señalar la discrepancia editorial. No invertir radios para acomodar los adjetivos. U exterior no es un orden de gravedad.

## H02. Tres convenciones de orientación requieren correspondencia explícita

Fundamentos §4.1 define θᵢ=2π(i−1)/n: primer eje a la derecha, giro antihorario en coordenadas cartesianas habituales. El ejemplo 2021 y [agentes especializados.md](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/fa3eb727799c322090e4e9126f81238cd198b9cc/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/agentes%20especializados.md), §9, inician arriba y avanzan en sentido horario. [comun/polygons.py](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c/comun/polygons.py) usa 2πk/n−π/2: primer eje abajo y giro antihorario.

Se trata de una diferencia de representación demostrable por las fórmulas, no de una prueba automática de pérdida semántica. Para φ=2π(i−1)/n, el dibujo histórico usa el ángulo π/2−φ y el generador común φ−π/2. Manteniendo el mismo radio e índice, ambas imágenes se relacionan por la transformación (x,y) → (x,−y), reflexión respecto al eje horizontal. Debe declararse esa transformación antes de comparar posiciones. No se ha validado aquí el recorrido completo de renderizado a lectura por IA.

## H03. Codificación histórica de competencia frente a convención canónica

[Desde SVcustos, el 'framework' de intrusión, a SVperitus.md](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/fa3eb727799c322090e4e9126f81238cd198b9cc/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/Desde%20SVcustos%2C%20el%20%27framework%27%20de%20intrusi%C3%B3n%2C%20a%20SVperitus.md), apartado «Fundamento teórico: del parámetro de estado al parámetro de competencia», §1.1, atribuye 1 a cobertura suficiente y 0 a ausencia de cobertura. Fundamentos §2 fija 0=Apto y 1=No_Apto.

Conservar el molde gráfico no resuelve esta inversión. Antes de importar aquel perfil se necesita fijar la proposición que evalúa cada posición y una correspondencia semántica expresa. Este estudio no inventa la traducción ni activa los umbrales de cobertura de aquel diseño.

## H04. El nivel base histórico no usa la misma regla residual

[agentes especializados.md](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/fa3eb727799c322090e4e9126f81238cd198b9cc/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/agentes%20especializados.md), §10, clasifica NORMAL «todo lo demás» tras sus condiciones de intrusión e indeterminación. Fundamentos §5 exige N₀≥T(n) para Apta y deja el resto Indeterminada. Testigo documental: nueve U no cumplen la condición histórica n₁∈{5,6}, por lo que caen en NORMAL; bajo el fundamento quedan Indeterminadas.

No se mezclan estas reglas. La referencia fundacional rige el estudio y la recepción de un antecedente necesita identificar qué regla ejecuta realmente. Este testigo no es una evaluación clínica o de ataques.

## H05. El max del par no es una ley universal ni un máximo de radios

[Células SV en Par  n = 36 + 9 = 45.md](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/fa3eb727799c322090e4e9126f81238cd198b9cc/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/C%C3%A9lulas%20SV%20en%20Par%20%20n%20%3D%2036%20%2B%209%20%3D%2045.md), §7.2, declara el orden N < U < I. El orden radial es ρ(0)<ρ(1)<ρ(U), por lo que no implementa aquel orden de severidad. Fundamentos §7 distingue roles, codominios y operadores tipados; max sólo corresponde cuando su orden y compatibilidad están documentados. El par contiene dos células, no una célula simple SV de 45 coordenadas.

## H06. Configuración y arquitectura construida no coinciden

[agentes/inmunologia/fase_1/config/imm_n25.yaml](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c/agentes/inmunologia/fase_1/config/imm_n25.yaml) declara training.architecture=convnext_tiny y un baseline histórico resnet34. [agentes/inmunologia/fase_1/src/train_resnet.py](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c/agentes/inmunologia/fase_1/src/train_resnet.py), build_model, construye ResNet34; [agentes/inmunologia/fase_1/src/evaluate.py](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c/agentes/inmunologia/fase_1/src/evaluate.py) también la construye. No se observó selección de arquitectura por ese campo en estos scripts.

La configuración por sí sola no acredita qué modelo se entrenó. Antes de reutilizar, deben concordar configuración, constructor y pesos. No se ha modificado ninguno.

## H07. Remapeo adicional de predicciones ya canónicas

El entrenador aplica target_transform de índices de carpetas a índices canónicos. El evaluador aplica remap tanto a las etiquetas del disco como a las predicciones del modelo. Bajo el entrenamiento descrito, estas últimas ya están en el orden canónico. El remapeo adicional intercambia APTO y NO_APTO.

[Script del testigo](testigo_indices.py) · [Resultado ejecutado](TESTIGO_INDICES.json). Se propone un predictor hipotético perfecto sobre tres clases: antes del remapeo adicional acierta 3/3; después, 1/3. Es una demostración del problema de índices, no una métrica de un modelo entrenado. Los índices canónicos 0,1,2 del clasificador no son los símbolos 0,1,U del SV.

## H08. Transformaciones de entrenamiento y fidelidad posicional

train_resnet.py incluye RandomHorizontalFlip y RandomRotation(15). Una clase basada en conteos puede mantenerse aunque una transformación cambie la lectura de posiciones. La validez de estas transformaciones debe evaluarse según la tarea: acertar una clase no acredita conservar la correspondencia entre parámetro y eje para explicación o decisión singular.

## H09. Rutas e importaciones para reutilización autónoma

Los scripts inspeccionados en fase_1/src buscan config respecto a su propio directorio, mientras el YAML recibido está en fase_1/config. Importan common.io_utils y el árbol inspeccionado contiene comun/io_utils.py. Son discrepancias estáticas del material recibido; un entorno externo podría aportar adaptación, pero no se acredita aquí. No se intentó ejecutar ni reparar la tubería histórica.

## Consecuencia del conjunto

Las discrepancias no justifican alterar los fundamentos ni descartar toda la representación. Delimitan las condiciones que deben resolverse antes de promover una realización. La elección futura entre CNN, modelo NLP con visión o varios módulos sigue abierta. El estudio no adopta una IA ni declara resistencia a inyección de instrucciones sin prueba.
