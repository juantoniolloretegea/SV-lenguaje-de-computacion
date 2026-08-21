# Acta técnica de cierre de SEC.0-A — autoridad, constitución y génesis

**Fecha:** 21/08/2026  
**Estado:** cerrado  
**Ámbito:** Lenguaje SV — SEC.0-A

## 1. Objeto del cierre

SEC.0-A se abrió para fijar el contrato abstracto que distingue información, evidencia admitida, hecho semántico constituido, autoridad, habilitación y ejercicio; determina las vías legítimas de constitución y transporte de autoridad; y prohíbe que las vías ordinarias de información, verificación, habilitación o ejercicio fabriquen o amplíen autoridad.

El cierre no materializa infraestructura de ejecución ni modifica gramática, IR, validador o catálogo diagnóstico. Su objeto es arquitectónico: establecer propiedades que deberán conservar las fases posteriores.

## 2. Documento técnico fijado

Queda incorporado como documento de referencia:

- `CONTRATO_ABSTRACTO_DE_AUTORIDAD_CONSTITUCION_Y_GENESIS_SEC0_A_V2_2026_08_21.md`.

El contrato define las clases abstractas T-I, T-V, T-H, T-E, T-G, T-C, T-0 y T-R; las magnitudes de autoridad; las ligaduras de contexto; las condiciones de transporte y delegación; la verificación gobernada; la acumulación de efectos; las reservas de decisión humana; las premisas del conjunto técnico de confianza; y los invariantes A2-01 a A2-17.

## 3. Condiciones estructurales satisfechas

El cierre establece las condiciones siguientes:

1. toda autoridad nueva procede de T-0, T-G, T-C o de una T-R que no amplíe una autoridad anterior;
2. T-I, T-V, T-H y T-E no constituyen autoridad;
3. la clase T-* de una forma concreta queda constituida con anterioridad al acto y no puede ser elegida por el ejecutante;
4. una T-E automática sólo puede operar sobre un dominio de autoridad `D_a` gobernado, cuya pertenencia pueda decidirse antes del ejercicio y cuya extensión no pueda ampliarse mediante T-I, T-V, T-H ni T-E;
5. una regla generativa o inductiva no puede ampliar `D_a` por el mero nacimiento ordinario de nuevos objetos; su conjunto portador debe estar previamente gobernado;
6. toda T-E repetible o acumulable debe disponer de un contrato suficiente de acumulación para impedir que la repetición produzca un efecto global fuera de la envolvente autorizada;
7. la verificación, la compatibilidad, la migración y la recuperación no pueden actuar como vías encubiertas de ampliación de autoridad;
8. el ejecutor material no adquiere por ejecutar la autoridad del titular;
9. la firma criptográfica no se considera prueba suficiente de comprensión humana;
10. la separación conceptual de componentes no se considera prueba de aislamiento material.

## 4. Alcance del cierre

SEC.0-A fija un contrato de autoridad. No demuestra por sí mismo:

- integridad del soporte físico, de los microprogramas, del sistema operativo, del compilador o del almacenamiento;
- fidelidad material del visor de firma;
- independencia material o cognitiva de un quórum;
- unicidad global de continuidad entre réplicas o bifurcaciones;
- disponibilidad ante agotamiento de recursos;
- recuperación material de una instalación comprometida.

Estas cuestiones permanecen asignadas a las fases que correspondan y no se consideran resueltas por este cierre.

## 5. Continuidad posterior

El cierre de SEC.0-A no abre automáticamente SEC.0-D. SEC.0-D permanece separado y requerirá decisión expresa para su apertura.

Asimismo, el cierre no autoriza todavía:

- tipos nuevos de IR;
- cambios de gramática;
- códigos diagnósticos nuevos;
- infraestructura general de ejecución;
- selección de una plataforma material;
- materialización del modelo estable de funciones de usuarios, identidad, responsabilidad o distribución reconocida.

## 6. Cierre

Las propiedades exigidas para SEC.0-A quedan fijadas en un documento técnico autónomo, con invariantes verificables y límites explícitos de alcance.

SEC.0-A se declara cerrado el 21/08/2026 como **contrato abstracto de autoridad, constitución y génesis**.
