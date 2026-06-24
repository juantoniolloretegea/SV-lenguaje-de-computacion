# Inventario, trazabilidad y relaciones editoriales de las publicaciones del Sistema Vectorial SV — V.1

DOI de la colección: [https://doi.org/10.21428/39829d0b.d3a012ba](https://doi.org/10.21428/39829d0b.d3a012ba)  
ISSN: 2695-6411  
Autor: Juan Antonio Lloret Egea  
ORCID: 0000-0002-6634-3351  
Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA)  
Licencia: CC BY-NC-ND 4.0  
Año: 2026  

## Objeto de la publicación V.1

Esta publicación fija la primera versión operativa del inventario de publicaciones del Sistema Vectorial SV. Su función es conservar una referencia única, cronológica y actualizable de publicaciones canónicas, DOI de citación preferente, localización editorial o técnica, colección principal, colecciones relacionadas, dominio trabajado, función material dentro del SV, relaciones editoriales y estado de cada registro.

La publicación V.1 no reemplaza a la colección homónima de PubPub ni a los depósitos técnicos. Actúa como fuente Markdown versionable en GitHub, preparada para copia editorial posterior en PubPub y para sincronización con el archivo CSV situado en `data/inventario_publicaciones_sv.csv`.

## Criterio de lectura

La tabla se ordena de forma cronológica descendente. Cada publicación canónica ocupa una fila. Los espejos editoriales o técnicos se consignan en columnas específicas y no generan una fila nueva salvo que exista una versión materialmente distinta, una release con modificación sustantiva o un depósito autónomo que deba tratarse como publicación diferenciada.

## Tabla matriz del inventario

| Fecha | Título canónico | Título breve | DOI de citación preferente | Enlace principal | Enlaces espejo | Colección principal | Colecciones relacionadas | Dominio SV | Función dentro del SV | Relación con otras publicaciones | Estado editorial | Fuente verificada | Observaciones de custodia |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 2026-06-24 | Inventario, trazabilidad y relaciones editoriales de las publicaciones del Sistema Vectorial SV | Inventario editorial SV | https://doi.org/10.21428/39829d0b.d3a012ba | https://doi.org/10.21428/39829d0b.d3a012ba |  | Inventario, trazabilidad y relaciones editoriales de las publicaciones del Sistema Vectorial SV | Registro de calidad central del SV; SV-lenguaje-de-computacion/docs/calidad/Inventario-sv | Trazabilidad editorial, control de calidad, continuidad documental y relaciones entre publicaciones del Sistema Vectorial SV | Fija el registro maestro de publicaciones canónicas del Sistema Vectorial SV, con DOI de citación preferente, localización editorial o técnica, colección principal, colecciones relacionadas, dominio trabajado, función material dentro del sistema, relaciones editoriales y estado de cada publicación. Su función es impedir pérdida de trazabilidad, duplicación de registros y dependencia de memoria personal o conversacional. | Actúa como publicación matriz V.1 del inventario; se sincroniza con la colección PubPub homónima y con la carpeta de calidad Inventario-sv en SV-lenguaje-de-computacion. | Vigente; publicación matriz V.1; registro maestro inicial | GitHub; PubPub/ITVIA; DOI confirmado | Una publicación canónica equivale a una fila. Los espejos en GitHub, PubPub, Zenodo o HCommons no generan filas nuevas salvo versión materialmente distinta. |

## Control de continuidad

Toda actualización posterior deberá conservar la estructura de columnas, añadir nuevas publicaciones al comienzo de la tabla, actualizar el CSV correspondiente y documentar en `releases/README.md` si la modificación produce una nueva release pública.
