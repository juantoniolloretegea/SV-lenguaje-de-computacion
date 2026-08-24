# Playground Python/Pyodide del Lenguaje SV — instantánea histórica de 24/08/2026

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

## Objeto

Este directorio conserva la interfaz pública del SVP Playground basada en Python y Pyodide que precedió al entorno público Rust/WebAssembly.

La instantánea HTML se conserva en [`SVP_PLAYGROUND_PYTHON_PYODIDE_2026_08_24.html`](./SVP_PLAYGROUND_PYTHON_PYODIDE_2026_08_24.html). Corresponde a la interfaz que mostraba Gramática 0.2, IR 0.3 y serializador 0.1.0, y que ejecutaba en el navegador la etapa frontal de referencia en Python mediante Pyodide.

## Estatuto

La conservación de esta pieza tiene finalidad histórica y de trazabilidad. No constituye el acceso público principal del Lenguaje SV a partir del relevo hacia Rust/WebAssembly.

La implementación Python permanece en el repositorio como referencia diferencial y como antecedente verificable. La conservación histórica del Playground no crea una segunda semántica ni atribuye vigencia simultánea a dos entornos públicos.

## Entorno público posterior

El acceso público principal se encuentra en:

- <https://lenguaje-sv.itvia.online/>

Ese entorno ejecuta en el navegador el módulo WebAssembly construido sobre `sv_core` y mantiene separada la distribución web de la autoridad semántica del núcleo.
