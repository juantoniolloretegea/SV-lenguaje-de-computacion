# Actualización transversal de autonomía, auditabilidad y frontera de IA

**Fecha:** 24/08/2026  
**Ámbito:** `SV-lenguaje-de-computacion`  
**Naturaleza:** registro técnico de calidad

## 1. Objeto

Este registro documenta una reconciliación pública limitada a cuatro frentes que ya disponían de antecedentes en la arquitectura del repositorio pero requerían una formulación inequívoca y conjunta:

1. vigencia de C01 frente a formulaciones históricas de la cadena de transducción;
2. criterio de autonomía ejecutiva del backend soberano;
3. auditabilidad causal y estatuto de las entradas procedentes de IA o modelos estadísticos;
4. alcance de efectos y recursos de una entrada una vez admitida.

No se modifica código, gramática, IR, catálogo diagnóstico ni batería de conformidad.

## 2. Hecho constatado — C01 y la Frontera v0

`FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md` conserva históricamente en B.6 una enumeración de admisibilidad con `fallido` y `U`, junto con una proyección de fallo o insuficiencia de admisión a `U`.

La especificación vigente IR 0.3 y C01 separan esos dominios:

```text
AdmissibilityState = {Ok, Degraded, NotAdmitted}
Bottom             ↛ Tri
NotAdmitted        ↛ Tri
fallo técnico      ↛ Tri
```

La adenda de vigencia de 24/08/2026 fija expresamente la precedencia de C01 para el comportamiento actual.

## 3. Hecho constatado — autonomía

El objetivo Rust ya establecía autonomía procesal y exclusión de Python como dependencia final del backend soberano.

La actualización precisa el criterio operativo:

```text
compilación/pruebas nativas de una biblioteca Rust
≠
autonomía del Lenguaje
```

Para declarar completa la autonomía funcional del perfil nativo deberá existir un camino ejecutable `.svp → núcleo Rust soberano → resultado`, sin delegación constitutiva en Python u otro motor semántico.

## 4. Hecho constatado — auditabilidad por garantía

La especificación del entorno soberano y SEC.0 ya definen `TCB(G)` por la capacidad de una dependencia para falsificar la garantía concreta.

La actualización hace explícito que:

```text
componente causal de un efecto
≠
autoridad SV
≠
miembro automático de todo TCB
```

La pertenencia al TCB se decide respecto de `G`, realización y modelo de amenaza.

## 5. Entradas externas e IA

La salida de una IA o modelo estadístico no adquiere autoridad por su origen ni por pasar después por una función tipada.

Cuando la salida proponga un candidato material para una garantía, una vía admisible es la verificación independiente respecto de la propiedad reclamada antes de su constitución.

No se reconoce como fundamento normativo por sí mismo:

- una puntuación;
- una etiqueta discreta;
- un vector latente;
- consenso entre modelos;
- reproducibilidad de una inferencia estadística;
- recencia o memoria del proveedor;
- ratificación humana meramente formal.

## 6. Admisión, efectos y recursos

La actualización hace explícito que superar una frontera de admisión no confiere confianza general, autoridad ni libertad de efecto.

Los efectos posteriores siguen sometidos a las formas, requisitos, ligaduras, autoridad, acumulación y contexto que resulten aplicables.

Tampoco se deriva de la admisión un presupuesto computacional abierto. Cuando una realización pueda inducir llamadas externas, reintentos, colas, verificaciones u otras expansiones de trabajo, éstas deberán permanecer dentro de límites materiales gobernados por el perfil correspondiente.

El agotamiento de recursos no se proyecta sobre `Tri`, no autoriza a omitir una condición necesaria para un efecto protegido y no debe extender el bloqueo más allá de las dependencias realmente afectadas.

La arquitectura debe poder conservar, dentro del alcance declarado, capacidad suficiente para rechazo, detención, diagnóstico y recuperación. Los mecanismos de control y registro también forman parte del análisis de recursos cuando puedan contribuir al agotamiento.

Estas condiciones son arquitectónicas. No declaran ya realizada una garantía de disponibilidad o rendimiento.

## 7. Documentos añadidos

- `docs/arquitectura/ADENDA_DE_VIGENCIA_C01_A_FRONTERA_NORMATIVA_LENGUAJE_SV_2026_08_24.md`;
- `docs/arquitectura/ADENDA_TRANSVERSAL_AUTONOMIA_AUDITABILIDAD_CAUSAL_Y_FRONTERA_IA_2026_08_24.md`.

## 8. Alcance y no efectos

La actualización:

- no reabre C01–C03;
- no crea C04;
- no incorpora TCB, plataforma, identidad, proveedores de IA, colas o políticas de recursos a la IR;
- no selecciona sistema operativo;
- no crea R0-9;
- no acredita Garantía I ni Garantía II;
- no declara completa la autonomía, la disponibilidad, la viabilidad material o la auditabilidad de extremo a extremo;
- no altera el radio del trabajo R0-6 actualmente separado de `main`.

## 9. Estado

Las reglas publicadas en estas adendas son condiciones de arquitectura y de lectura vigente. Su materialización deberá demostrarse en las fases y perfiles correspondientes antes de formular una garantía que dependa de ellas.